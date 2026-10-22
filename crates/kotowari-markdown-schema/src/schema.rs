//! スキーマ YAML のモデルと読み込み。

use crate::document::Block;
use regex::Regex;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer};

/// スキーマに書かれた正規表現。読み込み時に一度だけコンパイルし、以後は再利用する。
#[derive(Debug, Clone)]
pub struct Pattern {
    source: String,
    compiled: Regex,
}

impl Pattern {
    /// 文字列に一致するか。
    pub fn is_match(&self, text: &str) -> bool {
        self.compiled.is_match(text)
    }

    /// 名前付きキャプチャを含む最初の一致を取る。
    pub(crate) fn captures<'h>(&self, text: &'h str) -> Option<regex::Captures<'h>> {
        self.compiled.captures(text)
    }

    /// スキーマに書かれた正規表現の文字列。
    pub fn source(&self) -> &str {
        &self.source
    }

    /// 指定の名前の名前付きキャプチャを含むか。Capture 形の抽出が使う（REQ-schema-048）。
    pub fn has_capture_group(&self, name: &str) -> bool {
        self.compiled.capture_names().flatten().any(|n| n == name)
    }
}

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let source = String::deserialize(deserializer)?;
        let compiled = Regex::new(&source).map_err(D::Error::custom)?;
        Ok(Pattern { source, compiled })
    }
}

/// スキーマを読めなかった理由。TBL-schema-009 の「スキーマが形に合わない」に相当する。
#[derive(Debug)]
pub struct SchemaError(pub String);

impl std::fmt::Display for SchemaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SchemaError {}

/// スキーマ言語の最上位（REQ-schema-016）。
///
/// Construction and mutation cannot bypass semantic validation.
///
/// ```compile_fail
/// use kotowari_markdown_schema::Schema;
/// let schema = Schema { name: None, open: false, reading: Default::default(), document: todo!() };
/// ```
///
/// ```compile_fail
/// use kotowari_markdown_schema::Schema;
/// let mut schema = Schema::parse("document: {}").unwrap();
/// schema.open = true;
/// ```
///
/// ```compile_fail
/// use kotowari_markdown_schema::Schema;
/// let schema: Schema = serde_saphyr::from_str("document: {}").unwrap();
/// ```
#[derive(Debug)]
pub struct Schema {
    /// 型の名前。`ast --schema` の `type` に使う
    pub(crate) name: Option<String>,
    /// 閉じた世界を緩めるか（REQ-schema-002）
    pub(crate) open: bool,
    /// 見出しの下の行の読み方（REQ-schema-060）。書かないときは段落で読む
    pub(crate) reading: Reading,
    /// 文書の構造の木
    pub(crate) document: Document,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSchema {
    name: Option<String>,
    #[serde(default)]
    open: bool,
    #[serde(default)]
    reading: Reading,
    document: Document,
}

impl Schema {
    pub fn parse(yaml: &str) -> Result<Self, SchemaError> {
        parse_schema(yaml)
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn is_open(&self) -> bool {
        self.open
    }
}

/// 見出しと前置部の下の行の読み方（REQ-schema-060、TBL-schema-011）。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reading {
    /// CommonMark の段落として読む
    #[default]
    Paragraph,
    /// 1行ずつ読み分ける
    Line,
}

/// `document` の下の規則種別（TBL-schema-004）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    pub title: Option<Title>,
    pub preamble: Option<Preamble>,
    #[serde(default)]
    pub sections: Vec<Section>,
    /// 節を挟まずに文書の直下に置く項目（REQ-schema-016、REQ-schema-061）
    pub item: Option<Item>,
}

/// 題名（TBL-schema-004、REQ-schema-022）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Title {
    pub pattern: Option<Pattern>,
    pub extract: Option<Extract>,
}

/// 前置部（TBL-schema-004、REQ-schema-023）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Preamble {
    #[serde(default)]
    pub fields: Vec<Field>,
    pub statement: Option<Statement>,
    pub bullets: Option<Bullets>,
    pub table: Option<Table>,
    pub codeblock: Option<CodeBlock>,
    /// フィールド行の並び順を強制する（REQ-schema-041）
    #[serde(default)]
    pub ordered: bool,
}

/// 節（TBL-schema-004、REQ-schema-024）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// 見出しの文字列
    pub name: String,
    pub required: Option<bool>,
    pub repeat: Option<Repeat>,
    pub statement: Option<Statement>,
    #[serde(default)]
    pub fields: Vec<Field>,
    /// フィールド行の並び順を強制する（REQ-schema-041）
    #[serde(default)]
    pub ordered: bool,
    pub bullets: Option<Bullets>,
    pub table: Option<Table>,
    pub codeblock: Option<CodeBlock>,
    pub item: Option<Item>,
    pub extract: Option<Extract>,
}

/// 項目（TBL-schema-004、REQ-schema-025）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    /// 見出しの ID 部分の正規表現
    pub id: Option<Pattern>,
    #[serde(default)]
    pub fields: Vec<Field>,
    /// フィールド行の並び順を強制する（REQ-schema-041）
    #[serde(default)]
    pub ordered: bool,
    pub statement: Option<Statement>,
    pub bullets: Option<Bullets>,
    pub table: Option<Table>,
    pub codeblock: Option<CodeBlock>,
    pub required: Option<bool>,
    pub repeat: Option<Repeat>,
    pub extract: Option<Extract>,
}

/// フィールド行（TBL-schema-004、REQ-schema-029）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub name: String,
    pub required: Option<bool>,
    pub repeat: Option<Repeat>,
    pub pattern: Option<Pattern>,
    #[serde(rename = "enum")]
    pub r#enum: Option<Vec<String>>,
    pub separator: Option<String>,
    /// `csv: true` は `separator: ","` の省略形
    pub csv: Option<bool>,
    pub when: Option<When>,
    pub extract: Option<Extract>,
}

impl Field {
    /// separator と csv を解決した区切り文字。どちらも無ければ None。
    pub fn effective_separator(&self) -> Option<&str> {
        match (&self.separator, self.csv) {
            (Some(sep), _) => Some(sep),
            (None, Some(true)) => Some(","),
            (None, _) => None,
        }
    }
}

/// スキーマが宣言したフィールド行の名前と一致するか。TBL-schema-007 のフィールド行判定。
pub(crate) fn is_declared_field(fields: &[Field], name: &str) -> bool {
    fields.iter().any(|f| f.name == name)
}

/// 文（TBL-schema-004、REQ-schema-032）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Statement {
    pub required: Option<bool>,
    pub repeat: Option<Repeat>,
    pub pattern: Option<Pattern>,
    #[serde(rename = "enum")]
    pub r#enum: Option<Vec<String>>,
    pub when: Option<When>,
    pub extract: Option<Extract>,
}

/// 箇条書きの子の規則（REQ-schema-031）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Children {
    #[serde(default)]
    pub fields: Vec<Field>,
    /// 再帰的に `children` を持つことができる（任意の深さ）
    pub bullets: Option<Box<Bullets>>,
}

/// 箇条書き（TBL-schema-004、TBL-schema-007）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bullets {
    pub required: Option<bool>,
    pub repeat: Option<Repeat>,
    pub pattern: Option<Pattern>,
    pub when: Option<When>,
    pub extract: Option<Extract>,
    /// 子の規則。子のフィールド行と箇条書きを宣言する
    pub children: Option<Children>,
}

/// 表（TBL-schema-004、REQ-schema-033）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    /// ヘッダのセル列。宣言しないときはヘッダと列数を検査しない（REQ-schema-033）
    pub header: Option<Vec<String>>,
    /// 同じノードの中の表のうち、どれをこの規則の表にするか（REQ-schema-059）。
    /// 書かないときはすべての表がこの規則の表になる
    pub select: Option<Select>,
    pub required: Option<bool>,
    pub repeat: Option<Repeat>,
    pub extract: Option<Extract>,
}

/// 表の規則の `select`（REQ-schema-059）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Select {
    /// ヘッダが宣言と合う最初の表だけ
    First,
}

impl Table {
    /// `blocks` の表のうち、この規則の表にするものの開始行。`select` を書かない
    /// 規則はすべての表を受けるので None を返す（REQ-schema-059）。
    pub(crate) fn selected_line<'a>(
        &self,
        blocks: impl IntoIterator<Item = &'a Block>,
    ) -> Selection {
        let (Some(Select::First), Some(expected)) = (self.select, &self.header) else {
            return Selection::All;
        };
        Selection::Only(blocks.into_iter().find_map(|block| match block {
            Block::Table { header, line, .. } if header == expected => Some(*line),
            Block::Field { .. }
            | Block::Bullet { .. }
            | Block::OrderedList { .. }
            | Block::Statement { .. }
            | Block::Table { .. }
            | Block::Code { .. }
            | Block::Other { .. } => None,
        }))
    }
}

/// 表の規則がどの表を受けるか。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Selection {
    /// すべての表
    All,
    /// この開始行の表だけ。None ならどの表も受けない
    Only(Option<usize>),
}

impl Selection {
    /// 開始行 `line` の表を受けるか。
    pub(crate) fn takes(self, line: usize) -> bool {
        match self {
            Selection::All => true,
            Selection::Only(selected) => selected == Some(line),
        }
    }
}

/// コードブロック（TBL-schema-004、REQ-schema-034）。
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CodeBlock {
    pub lang: Option<String>,
    /// ブロック内の行ごとの規則。各行がパターンのいずれかに一致すること
    pub lines: Option<Vec<Pattern>>,
    pub required: Option<bool>,
    pub repeat: Option<Repeat>,
    pub extract: Option<Extract>,
}

/// 出現回数（REQ-schema-019、TBL-schema-005）。
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repeat {
    pub min: Option<u64>,
    pub max: Option<u64>,
}

impl Repeat {
    fn validate(&self) -> Result<(), SchemaError> {
        if let (Some(min), Some(max)) = (self.min, self.max)
            && min > max
        {
            return Err(SchemaError("repeat min is greater than max".into()));
        }
        Ok(())
    }
}

/// 条件付き規則（REQ-schema-020）。
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct When {
    /// 参照するフィールド行の名前。同じノードの下に限定
    pub field: String,
    pub eq: Option<String>,
    pub ne: Option<String>,
}

impl When {
    fn validate(&self) -> Result<(), SchemaError> {
        let operators = self.eq.is_some() as u8 + self.ne.is_some() as u8;
        if operators != 1 {
            return Err(SchemaError("when must have exactly one of eq or ne".into()));
        }
        Ok(())
    }
}

/// `of` が選ぶ、ノードから導かれる値の種類（REQ-schema-048）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OfKind {
    /// ノードまたは要素が現れた行番号（1始まりの数値）
    Line,
    /// 項目の見出しの ID 部分
    Id,
    /// 項目の見出しの名前部分
    Name,
    /// 生の行。字下げと末尾の空白を含み、組み立て直さない
    Raw,
    /// 項目と節の最後の行（1始まりの数値。REQ-schema-062）
    End,
}

impl OfKind {
    /// スキーマに書く語。
    fn word(self) -> &'static str {
        match self {
            OfKind::Line => "line",
            OfKind::Id => "id",
            OfKind::Name => "name",
            OfKind::Raw => "raw",
            OfKind::End => "end",
        }
    }
}

/// 1つのノードが宣言した抽出規則。YAML では `配置パス`だけの略記か、
/// `path`・`value`・`of`・`group` を持つ入れ子の写像を受ける（REQ-schema-048）。
#[derive(Debug, Clone)]
pub struct Extract {
    /// 出力の置き場。必ず1つ
    path: String,
    /// 要素の値を置く、要素オブジェクトの中の鍵
    value: Option<String>,
    /// 要素ごとの導かれる値。鍵は要素オブジェクトの中の配置パス。
    /// 宣言された順に持つ（出力の鍵の順がこの順になる）
    of: Vec<(String, OfKind)>,
    /// 題名の正規表現の名前付きキャプチャの名前
    group: Option<String>,
}

impl Extract {
    /// 出力の置き場。
    pub fn path(&self) -> &str {
        &self.path
    }

    /// 要素の値を置く、要素オブジェクトの中の鍵。
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// 要素ごとの導かれる値。宣言された順。
    pub fn of(&self) -> &[(String, OfKind)] {
        &self.of
    }

    /// 題名の名前付きキャプチャの名前。
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }

    /// 導かれる値を1つでも宣言しているか。
    pub fn has_of(&self) -> bool {
        !self.of.is_empty()
    }
}

impl<'de> Deserialize<'de> for Extract {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(ExtractVisitor)
    }
}

struct ExtractVisitor;

impl<'de> serde::de::Visitor<'de> for ExtractVisitor {
    type Value = Extract;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("an extract path, or a mapping with \"path\"")
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Extract, E> {
        check_placement_path(v).map_err(E::custom)?;
        Ok(Extract {
            path: v.to_string(),
            value: None,
            of: Vec::new(),
            group: None,
        })
    }

    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<Extract, M::Error> {
        let mut path: Option<String> = None;
        let mut value: Option<String> = None;
        let mut of: Option<Vec<(String, OfKind)>> = None;
        let mut group: Option<String> = None;
        while let Some(key) = map.next_key::<String>()? {
            let duplicate = |k: &str| M::Error::custom(format!("extract has \"{k}\" twice"));
            match key.as_str() {
                "path" => {
                    if path.is_some() {
                        return Err(duplicate("path"));
                    }
                    path = Some(map.next_value()?);
                }
                "value" => {
                    if value.is_some() {
                        return Err(duplicate("value"));
                    }
                    value = Some(map.next_value()?);
                }
                "of" => {
                    if of.is_some() {
                        return Err(duplicate("of"));
                    }
                    of = Some(map.next_value::<OfEntries>()?.0);
                }
                "group" => {
                    if group.is_some() {
                        return Err(duplicate("group"));
                    }
                    group = Some(map.next_value()?);
                }
                other => {
                    return Err(M::Error::custom(format!("unknown extract key \"{other}\"")));
                }
            }
        }
        // 配置パスは必ず1つ。無い宣言は schema_invalid の停止になる（REQ-schema-048）
        let Some(path) = path else {
            return Err(M::Error::custom("extract requires \"path\""));
        };
        check_placement_path(&path).map_err(M::Error::custom)?;
        // value と of の鍵も要素オブジェクトの中の配置パス（REQ-schema-048、TBL-schema-009）
        let of = of.unwrap_or_default();
        for relative in value.iter().chain(of.iter().map(|(key, _)| key)) {
            check_placement_path(relative).map_err(M::Error::custom)?;
        }
        Ok(Extract {
            path,
            value,
            of,
            group,
        })
    }
}

/// 配置パスのドットで区切った名前のどれかが空なら誤りにする（REQ-schema-036、
/// 2026-09-24-review4-gaps の A5）。空の名前は空文字列の JSON の鍵になってしまう
fn check_placement_path(path: &str) -> Result<(), String> {
    if path.split('.').any(str::is_empty) {
        return Err(format!("placement path \"{path}\" has an empty name"));
    }
    Ok(())
}

/// `of` の `{ 鍵: 語 }` の対応。宣言された順を保つ。同じ鍵を2度書いても
/// 黙って上書きせず、そのまま持って TBL-schema-009 の鍵の重複の判定に回す。
struct OfEntries(Vec<(String, OfKind)>);

impl<'de> Deserialize<'de> for OfEntries {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        // deserialize_map だと serde-saphyr は写像でない値を Visitor に渡さず、期待した型を説明に出さない
        d.deserialize_any(OfEntriesVisitor)
    }
}

struct OfEntriesVisitor;

impl<'de> serde::de::Visitor<'de> for OfEntriesVisitor {
    type Value = OfEntries;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("a mapping of placement keys to derived values")
    }

    fn visit_map<M: serde::de::MapAccess<'de>>(self, mut map: M) -> Result<OfEntries, M::Error> {
        let mut entries = Vec::new();
        while let Some((key, kind)) = map.next_entry::<String, OfKind>()? {
            entries.push((key, kind));
        }
        Ok(OfEntries(entries))
    }
}

/// スキーマ YAML を型付きのモデルに読み、形の違反を `SchemaError` にする。
pub fn parse_schema(yaml: &str) -> Result<Schema, SchemaError> {
    // 型や語が違うときに問題の欄の名前を説明に入れるため、読みながら欄の道筋を控える（REQ-schema-065）。
    // 停止は説明の1行目だけを出すので、欄の名前は複数行になる serde-saphyr の説明より前に置く
    let mut track = serde_path_to_error::Track::new();
    let parsed = serde_saphyr::with_deserializer_from_str(yaml, |de| {
        RawSchema::deserialize(serde_path_to_error::Deserializer::new(de, &mut track))
    });
    let schema = parsed.map_err(|e| match track.path().to_string().as_str() {
        "." => SchemaError(format!("{e}")),
        field => SchemaError(format!("in {field}: {e}")),
    })?;
    let schema = Schema {
        name: schema.name,
        open: schema.open,
        reading: schema.reading,
        document: schema.document,
    };
    check::validate_schema(&schema)?;
    Ok(schema)
}

/// 項目の内部のノードが1つでも `extract` を宣言しているか。宣言していれば
/// 項目の抽出はオブジェクトの形になる（REQ-schema-047）。
pub(crate) fn item_internals_declare_extract(item: &Item) -> bool {
    !item_internal_extract_paths(item).is_empty()
}

/// 項目の内部のノードが宣言した配置パス。項目オブジェクトの中の相対パスで、
/// TBL-schema-009 の鍵の重複の判定にも使う。
fn item_internal_extract_paths(item: &Item) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let mut push = |extract: Option<&Extract>| {
        if let Some(extract) = extract {
            paths.push(extract.path().to_string());
        }
    };
    for field in &item.fields {
        push(field.extract.as_ref());
    }
    push(item.statement.as_ref().and_then(|s| s.extract.as_ref()));
    push(item.bullets.as_ref().and_then(|b| b.extract.as_ref()));
    push(item.table.as_ref().and_then(|t| t.extract.as_ref()));
    push(item.codeblock.as_ref().and_then(|c| c.extract.as_ref()));
    if let Some(bullets) = &item.bullets {
        collect_children_extract_paths(bullets, &mut paths);
    }
    paths
}

/// 箇条書きの子フィールドが宣言した配置パスを、任意の深さから集める。
fn collect_children_extract_paths(bullets: &Bullets, paths: &mut Vec<String>) {
    let Some(children) = &bullets.children else {
        return;
    };
    for field in &children.fields {
        if let Some(extract) = &field.extract {
            paths.push(extract.path().to_string());
        }
    }
    if let Some(child_bullets) = children.bullets.as_deref() {
        collect_children_extract_paths(child_bullets, paths);
    }
}

mod check;
#[cfg(test)]
mod tests;
