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
        Ok(Extract {
            path,
            value,
            of: of.unwrap_or_default(),
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
    validate_schema(&schema)?;
    Ok(schema)
}

fn validate_schema(schema: &Schema) -> Result<(), SchemaError> {
    if let Some(title) = &schema.document.title {
        validate_title(title)?;
    }
    if let Some(preamble) = &schema.document.preamble {
        validate_preamble(preamble)?;
    }
    reject_duplicate_names(
        schema.document.sections.iter().map(|s| s.name.as_str()),
        "section",
    )?;
    for section in &schema.document.sections {
        validate_section(section)?;
    }
    if let Some(item) = &schema.document.item {
        validate_item(item)?;
    }
    reject_colliding_root_paths(&schema.document)?;
    reject_root_paths_on_the_type_key(schema)
}

/// 要素オブジェクトの外、文書の値の根に置く配置パスの衝突を停止にする（TBL-schema-009）。
/// 題名・前置部・節・項目と、それらの直下のノードの配置パスはすべて同じ置き場に並ぶ。
fn reject_colliding_root_paths(document: &Document) -> Result<(), SchemaError> {
    let mut paths: Vec<String> = Vec::new();
    collect_root_paths(document, &mut paths);
    let keys: Vec<&str> = paths.iter().map(String::as_str).collect();
    reject_colliding_keys(&keys, "document")
}

/// 文書の値の根に置く配置パスを集める。題名・前置部・節・項目と、それらの直下のノードと、
/// 箇条書きの子フィールドの配置パスが同じ置き場に並ぶ
fn collect_root_paths(document: &Document, paths: &mut Vec<String>) {
    let mut push = |extract: Option<&Extract>| {
        if let Some(extract) = extract {
            paths.push(extract.path().to_string());
        }
    };
    push(document.title.as_ref().and_then(|t| t.extract.as_ref()));
    if let Some(preamble) = &document.preamble {
        for field in &preamble.fields {
            push(field.extract.as_ref());
        }
        push(preamble.statement.as_ref().and_then(|s| s.extract.as_ref()));
        push(preamble.bullets.as_ref().and_then(|b| b.extract.as_ref()));
        push(preamble.table.as_ref().and_then(|t| t.extract.as_ref()));
        push(preamble.codeblock.as_ref().and_then(|c| c.extract.as_ref()));
    }
    for section in &document.sections {
        push(section.extract.as_ref());
        for field in &section.fields {
            push(field.extract.as_ref());
        }
        push(section.statement.as_ref().and_then(|s| s.extract.as_ref()));
        push(section.bullets.as_ref().and_then(|b| b.extract.as_ref()));
        push(section.table.as_ref().and_then(|t| t.extract.as_ref()));
        push(section.codeblock.as_ref().and_then(|c| c.extract.as_ref()));
        push(section.item.as_ref().and_then(|i| i.extract.as_ref()));
    }
    push(document.item.as_ref().and_then(|i| i.extract.as_ref()));
    // 箇条書きの子フィールドも、その箇条書きの置き場（根）に値を出す
    if let Some(bullets) = document.preamble.as_ref().and_then(|p| p.bullets.as_ref()) {
        collect_children_extract_paths(bullets, paths);
    }
    for section in &document.sections {
        if let Some(bullets) = &section.bullets {
            collect_children_extract_paths(bullets, paths);
        }
    }
}

/// "ast --schema" は根の "type" に "name" を置くので、"name" を宣言した`スキーマ`で根の
/// 配置パスが "type" の鍵を使うと "name" が消える。衝突と同じく停止にする
/// （TBL-schema-009、2026-09-24-review2-gaps の A2）
fn reject_root_paths_on_the_type_key(schema: &Schema) -> Result<(), SchemaError> {
    if schema.name.is_none() {
        return Ok(());
    }
    let mut paths: Vec<String> = Vec::new();
    collect_root_paths(&schema.document, &mut paths);
    match paths
        .iter()
        .find(|path| path.as_str() == "type" || path.starts_with("type."))
    {
        Some(path) => Err(SchemaError(format!(
            "document has the key \"{path}\", which conflicts with \"type\" that ast --schema sets from \"name\""
        ))),
        None => Ok(()),
    }
}

fn validate_title(title: &Title) -> Result<(), SchemaError> {
    reject_of_words(title.extract.as_ref(), "title", OUTSIDE_ITEM_OF)?;
    reject_duplicate_element_keys(title.extract.as_ref(), "title", &[])?;
    let Some(group) = title.extract.as_ref().and_then(Extract::group) else {
        return Ok(());
    };
    // `group` を宣言した抽出は pattern の名前付きキャプチャを取る。pattern が無い、
    // または pattern が指定の名前付きキャプチャを含まない題名は schema_invalid（REQ-schema-048）
    let Some(pattern) = &title.pattern else {
        return Err(SchemaError(
            "title capture extract requires a pattern".into(),
        ));
    };
    if !pattern.has_capture_group(group) {
        return Err(SchemaError(format!(
            "title pattern does not contain a named capture group \"{group}\""
        )));
    }
    Ok(())
}

fn validate_preamble(preamble: &Preamble) -> Result<(), SchemaError> {
    validate_fields(&preamble.fields)?;
    validate_statement(preamble.statement.as_ref())?;
    validate_bullets(preamble.bullets.as_ref(), false)?;
    validate_table(preamble.table.as_ref())?;
    validate_codeblock(preamble.codeblock.as_ref())?;
    Ok(())
}

fn validate_section(section: &Section) -> Result<(), SchemaError> {
    if let Some(repeat) = &section.repeat {
        repeat.validate()?;
    }
    reject_capture_extract(section.extract.as_ref(), "section")?;
    reject_of_words(section.extract.as_ref(), "section", SECTION_OF)?;
    reject_duplicate_element_keys(section.extract.as_ref(), "section", &[])?;
    validate_fields(&section.fields)?;
    validate_statement(section.statement.as_ref())?;
    validate_bullets(section.bullets.as_ref(), false)?;
    validate_table(section.table.as_ref())?;
    validate_codeblock(section.codeblock.as_ref())?;
    if let Some(item) = &section.item {
        validate_item(item)?;
    }
    Ok(())
}

fn validate_item(item: &Item) -> Result<(), SchemaError> {
    if let Some(repeat) = &item.repeat {
        repeat.validate()?;
    }
    reject_capture_extract(item.extract.as_ref(), "item")?;
    // 項目の内部に extract を宣言するなら、項目自身も extract を持つ必要がある。
    // 内部の配置パスは項目オブジェクトの中の相対パスなので、置き場が要る（REQ-schema-039、REQ-schema-047）
    let internal = item_internal_extract_paths(item);
    if item.extract.is_none() && !internal.is_empty() {
        return Err(SchemaError(
            "item internals declare extract but the item itself does not".into(),
        ));
    }
    // 内側の配置パスも項目オブジェクトの中の鍵なので、重複の判定に含める（TBL-schema-009）
    reject_duplicate_element_keys(item.extract.as_ref(), "item", &internal)?;
    validate_table(item.table.as_ref())?;
    validate_codeblock(item.codeblock.as_ref())?;
    validate_fields(&item.fields)?;
    validate_statement(item.statement.as_ref())?;
    validate_bullets(item.bullets.as_ref(), false)?;
    Ok(())
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

/// 表の規則を検査する。
fn validate_table(table: Option<&Table>) -> Result<(), SchemaError> {
    if let Some(table) = table {
        if let Some(repeat) = &table.repeat {
            repeat.validate()?;
        }
        // どの表を選ぶかはヘッダで決めるので、ヘッダの無い select は停止にする（TBL-schema-009）
        if table.select.is_some() && table.header.is_none() {
            return Err(SchemaError("table select requires a header".into()));
        }
        if let Some(header) = &table.header {
            reject_duplicate_names(header.iter().map(String::as_str), "table header column")?;
        }
        reject_capture_extract(table.extract.as_ref(), "table")?;
        reject_of_words(table.extract.as_ref(), "table", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(table.extract.as_ref(), "table", &[])?;
    }
    Ok(())
}

/// 1つの要素オブジェクトの中で鍵が重複するスキーマを停止にする（TBL-schema-009）。鍵は
/// `value`、`of` の鍵、そして内側のノードの配置パス（`internal` で渡す）である。
/// 判定は配置パス全体で行い、`a.b` と `a.c` は別の鍵、`a` と `a.b` は重複とする。
fn reject_duplicate_element_keys(
    extract: Option<&Extract>,
    node: &str,
    internal: &[String],
) -> Result<(), SchemaError> {
    let mut keys: Vec<&str> = Vec::new();
    if let Some(extract) = extract {
        keys.extend(extract.value());
        keys.extend(extract.of().iter().map(|(key, _)| key.as_str()));
    }
    keys.extend(internal.iter().map(String::as_str));
    reject_colliding_keys(&keys, &format!("{node} element object"))
}

/// 同じ置き場の配置パスのうち、同じパスか一方が他方の手前の段にあたるものを停止にする
/// （TBL-schema-009）。`a.b` と `a.c` は別の鍵、`a` と `a.b` は衝突とする。
fn reject_colliding_keys(keys: &[&str], place: &str) -> Result<(), SchemaError> {
    for (index, key) in keys.iter().enumerate() {
        for other in &keys[index + 1..] {
            if key == other || is_ancestor_path(key, other) || is_ancestor_path(other, key) {
                return Err(SchemaError(format!(
                    "{place} has the key \"{key}\" twice (conflicts with \"{other}\")"
                )));
            }
        }
    }
    Ok(())
}

/// `outer` が `inner` の親の配置パスか。`a` は `a.b` の親で、`a` と `ab` は無関係。
fn is_ancestor_path(outer: &str, inner: &str) -> bool {
    inner
        .strip_prefix(outer)
        .is_some_and(|rest| rest.starts_with('.'))
}

/// コードブロックの規則を検査する。
fn validate_codeblock(codeblock: Option<&CodeBlock>) -> Result<(), SchemaError> {
    if let Some(codeblock) = codeblock {
        if let Some(repeat) = &codeblock.repeat {
            repeat.validate()?;
        }
        reject_capture_extract(codeblock.extract.as_ref(), "codeblock")?;
        reject_of_words(codeblock.extract.as_ref(), "codeblock", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(codeblock.extract.as_ref(), "codeblock", &[])?;
    }
    Ok(())
}

/// 同じ置き場で同じ名前を2度宣言したスキーマを停止にする（TBL-schema-009、
/// 2026-09-24-review4-gaps の A3、A5）。検査と抽出で効く宣言が食い違うか、値が黙って消える
fn reject_duplicate_names<'a>(
    names: impl Iterator<Item = &'a str>,
    what: &str,
) -> Result<(), SchemaError> {
    let mut seen = std::collections::BTreeSet::new();
    for name in names {
        if !seen.insert(name) {
            return Err(SchemaError(format!("{what} \"{name}\" is declared twice")));
        }
    }
    Ok(())
}

fn validate_fields(fields: &[Field]) -> Result<(), SchemaError> {
    reject_duplicate_names(fields.iter().map(|f| f.name.as_str()), "field")?;
    for field in fields {
        if let Some(repeat) = &field.repeat {
            repeat.validate()?;
        }
        if let Some(when) = &field.when {
            when.validate()?;
        }
        reject_capture_extract(field.extract.as_ref(), "field")?;
        reject_of_words(field.extract.as_ref(), "field", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(field.extract.as_ref(), "field", &[])?;
    }
    Ok(())
}

fn validate_statement(statement: Option<&Statement>) -> Result<(), SchemaError> {
    if let Some(statement) = statement {
        if let Some(repeat) = &statement.repeat {
            repeat.validate()?;
        }
        if let Some(when) = &statement.when {
            when.validate()?;
        }
        reject_capture_extract(statement.extract.as_ref(), "statement")?;
        reject_of_words(statement.extract.as_ref(), "statement", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(statement.extract.as_ref(), "statement", &[])?;
    }
    Ok(())
}

fn validate_bullets(bullets: Option<&Bullets>, in_children: bool) -> Result<(), SchemaError> {
    if let Some(bullets) = bullets {
        if in_children && bullets.extract.is_some() {
            return Err(SchemaError(
                "children bullets cannot declare extract".into(),
            ));
        }
        if let Some(repeat) = &bullets.repeat {
            repeat.validate()?;
        }
        if let Some(when) = &bullets.when {
            when.validate()?;
        }
        if !in_children {
            reject_capture_extract(bullets.extract.as_ref(), "bullets")?;
        }
        reject_of_words(bullets.extract.as_ref(), "bullets", OUTSIDE_ITEM_OF)?;
        reject_duplicate_element_keys(bullets.extract.as_ref(), "bullets", &[])?;
        if let Some(children) = &bullets.children {
            validate_fields(&children.fields)?;
            validate_bullets(children.bullets.as_deref(), true)?;
        }
    }
    Ok(())
}

/// 題名以外のノードには名前付きキャプチャ（`group`）の抽出を宣言できない（TBL-schema-008）。
/// 宣言すると schema_invalid の停止になる。
fn reject_capture_extract(extract: Option<&Extract>, node: &str) -> Result<(), SchemaError> {
    if extract.and_then(Extract::group).is_some() {
        return Err(SchemaError(format!(
            "{node} extract cannot use the named-group capture form"
        )));
    }
    Ok(())
}

/// 項目と節の外のノードが `of` に宣言できる語。`id` と `name` は項目だけ、`end` は項目と節だけ（REQ-schema-048）。
const OUTSIDE_ITEM_OF: &[OfKind] = &[OfKind::Line, OfKind::Raw];
/// 節が `of` に宣言できる語（REQ-schema-048）。
const SECTION_OF: &[OfKind] = &[OfKind::Line, OfKind::Raw, OfKind::End];

/// ノードが受け付けない `of` の語を schema_invalid の停止にし、受け付ける語の一覧を示す（REQ-schema-065）。
fn reject_of_words(
    extract: Option<&Extract>,
    node: &str,
    accepted: &[OfKind],
) -> Result<(), SchemaError> {
    let Some(extract) = extract else {
        return Ok(());
    };
    for (_, kind) in extract.of() {
        if !accepted.contains(kind) {
            let words: Vec<String> = accepted
                .iter()
                .map(|k| format!("\"{}\"", k.word()))
                .collect();
            return Err(SchemaError(format!(
                "{node} extract cannot use \"{}\" in of; accepted: {}",
                kind.word(),
                words.join(", ")
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADR_LIKE: &str = r#"
name: adr
open: false
document:
  title:
    pattern: "^ADR-(?<id>\\d{4}):"
    extract: { path: id, group: id }
  preamble:
    fields:
      - name: 状態
        enum: [承認済み, 却下]
        extract: status
      - name: 日付
        pattern: "^\\d{4}-\\d{2}-\\d{2}"
        separator: ","
    statement:
      required: false
    bullets:
      pattern: "^[^-]"
  sections:
    - name: 状況
      extract: sections.context
      statement:
        required: false
      bullets:
        repeat: { min: 0 }
        extract: sections.reasons
      table:
        header: [用語, 意味]
      codeblock:
        lang: gherkin
        lines: ["^@", "^Scenario:"]
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0, max: 5 }
        fields:
          - name: 種類
            enum: [algorithm, ubiquitous]
          - name: 定義
            when: { field: 種類, eq: algorithm }
"#;

    // @kotowari[REQ-schema-016, REQ-schema-017]
    #[test]
    fn loads_a_schema_with_all_rule_kinds() {
        let schema = parse_schema(ADR_LIKE).unwrap();
        assert_eq!(schema.name.as_deref(), Some("adr"));
        assert!(!schema.open);
        assert!(schema.document.title.is_some());
        let preamble = schema.document.preamble.as_ref().unwrap();
        assert_eq!(preamble.fields.len(), 2);
        assert_eq!(preamble.fields[0].name, "状態");
        assert!(preamble.statement.is_some());
        assert!(preamble.bullets.is_some());
        let section = &schema.document.sections[0];
        assert_eq!(section.name, "状況");
        assert!(section.table.is_some());
        assert!(section.codeblock.is_some());
        let item = section.item.as_ref().unwrap();
        assert_eq!(item.id.as_ref().map(|p| p.source()), Some("REQ-\\d{3,}"));
        assert_eq!(item.repeat.as_ref().unwrap().max, Some(5));
        assert_eq!(
            item.fields[1].when.as_ref().unwrap().eq.as_deref(),
            Some("algorithm")
        );
    }

    // @kotowari[REQ-schema-029]
    #[test]
    fn csv_is_a_shorthand_for_comma_separator() {
        let yaml = r#"
document:
  preamble:
    fields:
      - name: タグ
        csv: true
"#;
        let schema = parse_schema(yaml).unwrap();
        let field = &schema.document.preamble.unwrap().fields[0];
        assert_eq!(field.effective_separator(), Some(","));
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn extract_accepts_the_shorthand_and_the_named_group() {
        // 略記（`extract: <名前>`）は `{ path: <名前> }` と同じに読む（REQ-schema-048）
        let yaml = r#"
document:
  title:
    pattern: "^ADR-(?<id>\\d{4})"
    extract: { path: id, group: id }
  preamble:
    fields:
      - name: 状態
        extract: status
"#;
        let schema = parse_schema(yaml).unwrap();
        let title = schema.document.title.unwrap();
        let title_extract = title.extract.as_ref().unwrap();
        assert_eq!(title_extract.path(), "id");
        assert_eq!(title_extract.group(), Some("id"));
        let preamble = schema.document.preamble.unwrap();
        let field_extract = preamble.fields[0].extract.as_ref().unwrap();
        assert_eq!(field_extract.path(), "status");
        assert_eq!(field_extract.value(), None);
        assert!(field_extract.of().is_empty());
        assert_eq!(field_extract.group(), None);
    }

    // @kotowari[REQ-schema-018, EX-schema-008]
    #[test]
    fn unknown_key_is_an_error() {
        let yaml = "document:\n  bogus: 1\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-018]
    #[test]
    fn type_violation_is_an_error() {
        let yaml = "open: not-a-bool\ndocument:\n  title:\n    pattern: x\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn min_greater_than_max_is_an_error() {
        let yaml = "document:\n  sections:\n    - name: x\n      repeat: { min: 3, max: 1 }\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn negative_repeat_is_an_error() {
        let yaml = "document:\n  sections:\n    - name: x\n      repeat: { min: -1 }\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn non_integer_repeat_is_an_error() {
        let yaml = "document:\n  sections:\n    - name: x\n      repeat: { min: 1.5 }\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-020]
    #[test]
    fn when_with_both_operators_is_an_error() {
        let yaml = r#"
document:
  preamble:
    fields:
      - name: 定義
        when: { field: 種類, eq: a, ne: b }
"#;
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-020]
    #[test]
    fn when_with_no_operator_is_an_error() {
        let yaml = r#"
document:
  preamble:
    fields:
      - name: 定義
        when: { field: 種類 }
"#;
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-016]
    #[test]
    fn invalid_regex_pattern_is_an_error() {
        let yaml = "document:\n  title:\n    pattern: \"(\"\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-016, REQ-schema-025]
    #[test]
    fn invalid_item_id_regex_is_an_error() {
        let yaml = "document:\n  sections:\n    - name: x\n      item:\n        id: \"(\"\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn capture_extract_outside_title_is_schema_invalid() {
        // `group` の宣言（名前付きキャプチャ）は題名にだけ使える。題名以外のノードで
        // 宣言したときは schema_invalid の停止になる（TBL-schema-008）。
        let cases: &[(&str, &str)] = &[
            (
                "field",
                "document:\n  preamble:\n    fields:\n      - name: 状態\n        extract: { path: s, group: x }\n",
            ),
            (
                "statement",
                "document:\n  preamble:\n    statement:\n      extract: { path: s, group: x }\n",
            ),
            (
                "bullets",
                "document:\n  preamble:\n    bullets:\n      extract: { path: b, group: x }\n",
            ),
            (
                "section",
                "document:\n  sections:\n    - name: 状況\n      extract: { path: sec, group: x }\n",
            ),
            (
                "item",
                "document:\n  sections:\n    - name: 要求\n      item:\n        extract: { path: item, group: x }\n",
            ),
            (
                "table",
                "document:\n  sections:\n    - name: 用語集\n      table:\n        header: [a]\n        extract: { path: t, group: x }\n",
            ),
            (
                "codeblock",
                "document:\n  sections:\n    - name: コード\n      codeblock:\n        extract: { path: c, group: x }\n",
            ),
        ];
        for (node, yaml) in cases {
            assert!(
                parse_schema(yaml).is_err(),
                "{node} に Capture 形の extract を宣言したのに schema_invalid にならない"
            );
        }
    }

    // @kotowari[REQ-schema-039]
    #[test]
    fn item_internal_extract_without_item_extract_is_schema_invalid() {
        let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        fields:
          - name: 種類
            extract: kind
"#;
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-039]
    #[test]
    fn item_child_field_extract_without_item_extract_is_schema_invalid() {
        // item の bullets.children.fields も「項目の内部のフィールド行」なので、
        // extract を宣言すると schema_invalid（REQ-schema-039）。
        let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        bullets:
          repeat: { min: 0 }
          children:
            fields:
              - name: superseded_by
                extract: superseded_by
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "item の bullets.children.fields に extract を宣言すると schema_invalid（REQ-schema-039）"
        );
    }

    // @kotowari[REQ-schema-039]
    #[test]
    fn item_deep_child_field_extract_without_item_extract_is_schema_invalid() {
        // 再帰的な入れ子（item の bullets.children.bullets.children.fields）も
        // 項目の内部のフィールド行なので schema_invalid（REQ-schema-039）。
        let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        bullets:
          repeat: { min: 0 }
          children:
            bullets:
              repeat: { min: 0 }
              children:
                fields:
                  - name: 補足
                    extract: note
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "item の入れ子の子フィールドに extract を宣言すると schema_invalid（REQ-schema-039）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn capture_extract_without_pattern_is_schema_invalid() {
        // `group` を宣言した抽出は pattern の名前付きキャプチャを取る。pattern が無い
        // 題名は schema_invalid の停止になる（REQ-schema-048）。
        let yaml = r#"
document:
  title:
    extract: { path: id, group: id }
"#;
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn capture_extract_with_pattern_missing_the_group_is_schema_invalid() {
        // pattern が指定の名前付きキャプチャを含まない題名も schema_invalid（TBL-schema-008）。
        let yaml = r#"
document:
  title:
    pattern: "^ADR-(?<other>\\d{4}):"
    extract: { path: id, group: id }
"#;
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-016]
    #[test]
    fn not_yaml_is_an_error() {
        assert!(parse_schema("not: [valid: yaml").is_err());
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn bullets_with_children_loads() {
        let yaml = r#"
document:
  sections:
    - name: 決定
      bullets:
        repeat: { min: 0 }
        extract: decisions
        children:
          fields:
            - name: superseded_by
              extract: superseded_by
          bullets:
            repeat: { min: 0 }
            children:
              fields:
                - name: 補足
"#;
        let schema = parse_schema(yaml).unwrap();
        let bullets = schema.document.sections[0].bullets.as_ref().unwrap();
        let children = bullets.children.as_ref().unwrap();
        assert_eq!(children.fields[0].name, "superseded_by");
        let child_bullets = children.bullets.as_ref().unwrap();
        assert!(
            child_bullets.children.as_ref().unwrap().fields[0].name == "補足",
            "children.bullets が再帰的に children を持てる（任意の深さ）"
        );
    }

    // @kotowari[REQ-schema-031]
    #[test]
    fn item_bullets_can_have_children() {
        let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        bullets:
          repeat: { min: 0 }
          children:
            fields:
              - name: superseded_by
"#;
        let schema = parse_schema(yaml).unwrap();
        let bullets = schema.document.sections[0]
            .item
            .as_ref()
            .unwrap()
            .bullets
            .as_ref()
            .unwrap();
        assert!(bullets.children.is_some());
    }

    // @kotowari[REQ-schema-018]
    #[test]
    fn unknown_key_under_bullets_is_an_error() {
        let yaml = "document:\n  sections:\n    - name: 理由\n      bullets:\n        bogus: 1\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-018]
    #[test]
    fn type_violation_under_children_is_an_error() {
        let yaml = "document:\n  sections:\n    - name: 理由\n      bullets:\n        children: not-a-map\n";
        assert!(parse_schema(yaml).is_err());
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn children_bullets_with_extract_is_schema_invalid() {
        let yaml = r#"
document:
  sections:
    - name: 決定
      bullets:
        children:
          bullets:
            extract: nested
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "children.bullets に extract を宣言すると schema_invalid（TBL-schema-008）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn deep_children_bullets_with_extract_is_schema_invalid() {
        let yaml = r#"
document:
  sections:
    - name: 決定
      bullets:
        children:
          bullets:
            children:
              bullets:
                extract: nested
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "再帰的な children.bullets にも extract を宣言できない（TBL-schema-008）"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-036]
    #[test]
    fn child_field_can_have_extract() {
        let yaml = r#"
document:
  sections:
    - name: 決定
      bullets:
        children:
          fields:
            - name: superseded_by
              extract: superseded_by
"#;
        assert!(
            parse_schema(yaml).is_ok(),
            "子フィールドは自分の extract を持つことができる（REQ-schema-036）"
        );
    }

    // @kotowari[REQ-schema-039, EX-schema-014]
    #[test]
    fn item_table_extract_without_item_extract_is_rejected() {
        let yaml = r#"
document:
  sections:
    - name: 決定表
      item:
        table:
          header: [用語, 意味]
          extract: glossary
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "項目が extract を持たないまま内部の表に宣言すると停止する（REQ-schema-039）"
        );
    }

    // @kotowari[REQ-schema-039]
    #[test]
    fn item_codeblock_extract_without_item_extract_is_rejected() {
        let yaml = r#"
document:
  sections:
    - name: 具体例
      item:
        codeblock:
          lang: gherkin
          extract: scenarios
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "項目が extract を持たないまま内部のコードブロックに宣言すると停止する（REQ-schema-039）"
        );
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn repeat_with_equal_min_and_max_is_accepted() {
        let yaml = r#"
document:
  sections:
    - name: 決定表
      item:
        table:
          repeat: { min: 1, max: 1 }
"#;
        assert!(
            parse_schema(yaml).is_ok(),
            "下限と上限が等しい範囲はちょうどその個数で、形に反しない"
        );
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn item_table_invalid_repeat_is_rejected() {
        let yaml = r#"
document:
  sections:
    - name: 決定表
      item:
        table:
          header: [用語, 意味]
          repeat: { min: 2, max: 1 }
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "項目の表の repeat も min > max なら停止する（REQ-schema-019）"
        );
    }

    // @kotowari[REQ-schema-019]
    #[test]
    fn item_codeblock_invalid_repeat_is_rejected() {
        let yaml = r#"
document:
  sections:
    - name: 具体例
      item:
        codeblock:
          lang: gherkin
          repeat: { min: 2, max: 1 }
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "項目のコードブロックの repeat も min > max なら停止する（REQ-schema-019）"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn of_id_outside_an_item_is_rejected() {
        let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract: { path: id, of: { id: id } }
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "of: id は項目にだけ宣言できる（REQ-schema-048）"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn unknown_of_value_is_rejected() {
        let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract: { path: x, of: { x: column } }
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "of の知らない語は停止する（REQ-schema-048）"
        );
    }

    // @kotowari[REQ-schema-039, REQ-schema-047]
    #[test]
    fn item_internal_extract_with_item_extract_loads() {
        let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        extract:
          path: requirements
          of: { id: id }
        fields:
          - name: 種類
            extract: kind
        table:
          extract: rows
"#;
        assert!(
            parse_schema(yaml).is_ok(),
            "項目が extract を持てば内部にも宣言できる（REQ-schema-039）"
        );
    }

    // @kotowari[REQ-schema-035, REQ-schema-048]
    #[test]
    fn nested_extract_declaration_loads() {
        // 抽出の宣言は path・value・of・group を持つ1つの入れ子である（REQ-schema-048）
        let yaml = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [用語, 意味]
        extract: { path: glossary, value: cells, of: { line: line } }
"#;
        let schema = parse_schema(yaml).unwrap();
        let extract = schema.document.sections[0]
            .table
            .as_ref()
            .unwrap()
            .extract
            .as_ref()
            .unwrap();
        assert_eq!(extract.path(), "glossary");
        assert_eq!(extract.value(), Some("cells"));
        assert_eq!(
            extract.of(),
            [("line".to_string(), OfKind::Line)],
            "of は鍵から導かれる値の語への対応として読む（REQ-schema-048）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn legacy_sequence_extract_is_schema_invalid() {
        // 書式の並びは受けない（REQ-schema-048）
        let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract:
            - status
            - { path: status_line, of: { line: line } }
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "旧来の書式の並びは schema_invalid の停止になる（REQ-schema-048）"
        );
    }

    // @kotowari[REQ-schema-035]
    #[test]
    fn extract_without_a_path_is_schema_invalid() {
        let yaml = r#"
document:
  sections:
    - name: 記録
      fields:
        - name: 状態
          extract: { of: { status_line: line } }
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "配置パスの無い宣言は schema_invalid の停止になる（REQ-schema-048）"
        );
    }

    // @kotowari[REQ-schema-042, REQ-schema-048]
    #[test]
    fn duplicate_key_between_an_inner_node_and_a_derived_value_is_schema_invalid() {
        // 項目のオブジェクトの中で、内側のフィールド行の配置パスと外側の
        // 導かれる値の鍵が重なるスキーマは、文書を読まずに停止する（TBL-schema-009）
        let yaml = r#"
document:
  sections:
    - name: 要求
      item:
        extract: { path: requirements, of: { line: line } }
        fields:
          - name: 状態
            extract: line
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "要素オブジェクトの中で鍵が重複したら停止する（TBL-schema-009）"
        );
    }

    // @kotowari[REQ-schema-042, REQ-schema-048]
    #[test]
    fn duplicate_key_between_value_and_a_derived_value_is_schema_invalid() {
        let yaml = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [a, b]
        extract: { path: glossary, value: cells, of: { cells: line } }
"#;
        assert!(
            parse_schema(yaml).is_err(),
            "要素の値の鍵と導かれる値の鍵が重複したら停止する（TBL-schema-009）"
        );
    }

    // @kotowari[REQ-schema-042, REQ-schema-048]
    #[test]
    fn a_parent_key_is_a_duplicate_but_a_sibling_key_is_not() {
        // 判定は配置パス全体で行う。a と a.b は親子で重複、a.b と a.c は別の鍵（TBL-schema-009）
        let parent_and_child = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [x]
        extract: { path: glossary, value: a, of: { "a.b": line } }
"#;
        let siblings = r#"
document:
  sections:
    - name: 用語集
      table:
        header: [x]
        extract: { path: glossary, value: a.b, of: { "a.c": line } }
"#;
        assert!(
            parse_schema(parent_and_child).is_err(),
            "a と a.b は片方が他方の親なので重複（TBL-schema-009）"
        );
        assert!(
            parse_schema(siblings).is_ok(),
            "a.b と a.c は入れ子を共有するだけの別の鍵（TBL-schema-009）"
        );
    }
}
