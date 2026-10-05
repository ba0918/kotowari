//! `対`の見分け方、`一致の記録`、`対`の`指摘`（docs/ir/core/translation-pairs.md、
//! docs/ir/core/translation-structure.md）
//!
//! ファイルを探して読むのは呼び出し側で、ここは名前と読んだ中身だけから決める。

use crate::{Finding, FindingKind};
use sha1::{Digest, Sha1};
use std::sync::Arc;

/// `一致の記録`の名前の、拡張子の前の部分（REQ-core-337、REQ-core-339）
const RECORD: &str = ".i18n.yaml";

/// REQ-core-340: "blob "、バイト数の10進、1バイトの 0、バイト列をつないだものの SHA-1 を16進の小文字で
pub fn blob_hash(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("blob {}\0", bytes.len()).as_bytes());
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// ファイル名を、最後の "." の前と、その "." からの拡張子に分ける（REQ-core-337）
fn split_extension(name: &str) -> (&str, &str) {
    match name.rfind('.') {
        Some(dot) => (&name[..dot], &name[dot..]),
        None => (name, ""),
    }
}

/// `対`にする置き場で読んだファイルの名前の見分け（REQ-core-337）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Classified {
    /// `先頭の言語`の`側`
    First,
    /// `先頭の言語`でない言語の`側`。first はその`対`の`先頭の言語`の`側`のファイル名
    Side { language: String, first: String },
    /// "<幹>.i18n.yaml"。`側`として読まない
    Record,
}

/// ファイル名（ディレクトリを除く）を`言語の一覧`で見分ける（REQ-core-337）
pub fn classify(name: &str, languages: &[String]) -> Classified {
    if name.ends_with(RECORD) {
        return Classified::Record;
    }
    let (rest, extension) = split_extension(name);
    if let Some((stem, tag)) = rest.rsplit_once('.')
        && languages.iter().skip(1).any(|language| language == tag)
    {
        return Classified::Side {
            language: tag.to_string(),
            first: format!("{stem}{extension}"),
        };
    }
    Classified::First
}

/// `先頭の言語`の`側`のパスから、その言語の`側`のパス。`先頭の言語`ならそのまま
pub fn side_path(first: &str, language: &str, languages: &[String]) -> String {
    if languages.first().map(String::as_str) == Some(language) {
        return first.to_string();
    }
    let (directory, name) = split_directory(first);
    let (stem, extension) = split_extension(name);
    format!("{directory}{stem}.{language}{extension}")
}

/// `先頭の言語`の`側`のパスから、`一致の記録`のパス（REQ-core-339）
pub fn record_path(first: &str) -> String {
    let (directory, name) = split_directory(first);
    let (stem, _) = split_extension(name);
    format!("{directory}{stem}{RECORD}")
}

/// パスを、最後の "/" までのディレクトリとファイル名に分ける
fn split_directory(path: &str) -> (&str, &str) {
    match path.rfind('/') {
        Some(slash) => (&path[..=slash], &path[slash + 1..]),
        None => ("", path),
    }
}

/// ファイル名（ディレクトリを除く）
fn file_name(path: &str) -> &str {
    split_directory(path).1
}

/// `対`を読んだ置き場
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    Ir,
    Guide,
    OverviewData,
    Toc,
}

/// ある`側`の読んだ中身
#[derive(Debug, Clone)]
pub struct SideText {
    blob: String,
    text: Arc<str>,
}

impl SideText {
    /// bytes はファイルのバイト列そのもの、text はそれを読んだ文字（BOM を除いたもの）
    pub fn new(bytes: &[u8], text: impl Into<String>) -> Self {
        Self {
            blob: blob_hash(bytes),
            text: Arc::from(text.into()),
        }
    }
    pub fn blob(&self) -> &str {
        &self.blob
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// `対`の1つの言語の`側`
#[derive(Debug, Clone)]
pub struct Side {
    language: String,
    path: String,
    content: Option<SideText>,
}

impl Side {
    /// path は`基準のディレクトリ`からの相対パス。ファイルが無ければ content は None
    pub fn new(
        language: impl Into<String>,
        path: impl Into<String>,
        content: Option<SideText>,
    ) -> Self {
        Self {
            language: language.into(),
            path: path.into(),
            content,
        }
    }
    pub fn language(&self) -> &str {
        &self.language
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn content(&self) -> Option<&SideText> {
        self.content.as_ref()
    }
}

/// 1つの`対`。sides は`言語の一覧`の順で、最初が`先頭の言語`の`側`
#[derive(Debug, Clone)]
pub struct Pair {
    place: Place,
    sides: Vec<Side>,
    /// `一致の記録`のバイト列。無ければ None
    record: Option<Vec<u8>>,
}

impl Pair {
    /// first は`先頭の言語`の`側`の`基準のディレクトリ`からの相対パス。read はパスからそのファイルの
    /// 中身を返し、ファイルが無ければ None を返す
    pub fn read<E>(
        place: Place,
        first: &str,
        languages: &[String],
        mut read: impl FnMut(&str) -> Result<Option<SideText>, E>,
        record: Option<Vec<u8>>,
    ) -> Result<Self, E> {
        let sides = languages
            .iter()
            .map(|language| {
                let path = side_path(first, language, languages);
                let content = read(&path)?;
                Ok(Side::new(language.as_str(), path, content))
            })
            .collect::<Result<_, E>>()?;
        Ok(Self {
            place,
            sides,
            record,
        })
    }
    pub fn place(&self) -> Place {
        self.place
    }
    /// `先頭の言語`の`側`のパス
    pub fn path(&self) -> &str {
        &self.sides[0].path
    }
    pub fn sides(&self) -> &[Side] {
        &self.sides
    }
    /// `先頭の言語`の`側`
    pub fn first(&self) -> &Side {
        &self.sides[0]
    }
    /// `先頭の言語`でない言語の`側`
    pub fn others(&self) -> &[Side] {
        &self.sides[1..]
    }
}

/// `一致の記録`を読んだもの。誤りなら TBL-core-008 の detail
fn read_record(pair: &Pair) -> Result<std::collections::BTreeMap<String, String>, &'static str> {
    let Some(bytes) = &pair.record else {
        return Err("missing");
    };
    let text = std::str::from_utf8(bytes).map_err(|_| "yaml")?;
    let value: serde_json::Value = if text.trim().is_empty() {
        serde_json::Value::Null
    } else {
        serde_saphyr::from_str(crate::strip_bom(text)).map_err(|_| "yaml")?
    };
    let serde_json::Value::Object(map) = value else {
        return Err("keys");
    };
    let mut record = std::collections::BTreeMap::new();
    for (key, value) in map {
        let serde_json::Value::String(value) = value else {
            return Err("keys");
        };
        record.insert(key, value);
    }
    let names: std::collections::BTreeSet<&str> = pair
        .sides
        .iter()
        .map(|side| file_name(&side.path))
        .collect();
    if !record.keys().map(String::as_str).eq(names.iter().copied()) {
        return Err("keys");
    }
    let hex = |value: &String| {
        value.len() == 40
            && value
                .bytes()
                .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
    };
    if !record.values().all(hex) {
        return Err("value");
    }
    Ok(record)
}

/// 欠けた側、`一致の記録`、古い側の`指摘`（REQ-core-338、REQ-core-339、REQ-core-341）
fn pair_findings(pair: &Pair, findings: &mut Vec<Finding>) {
    let missing = |path: &str, detail: &str| {
        Finding::new(
            FindingKind::TranslationMissing,
            path.to_string(),
            None,
            detail.to_string(),
        )
    };
    let first = pair.first();
    if first.content.is_none() {
        for side in pair.others().iter().filter(|side| side.content.is_some()) {
            findings.push(missing(&side.path, &first.path));
        }
        return;
    }
    for side in pair.others().iter().filter(|side| side.content.is_none()) {
        findings.push(missing(&first.path, &side.path));
    }
    match read_record(pair) {
        Err(detail) => findings.push(Finding::new(
            FindingKind::TranslationRecordInvalid,
            record_path(&first.path),
            None,
            detail.to_string(),
        )),
        Ok(record) => {
            for side in &pair.sides {
                let Some(content) = &side.content else {
                    continue;
                };
                let recorded = &record[file_name(&side.path)];
                if *recorded != content.blob {
                    findings.push(Finding::new(
                        FindingKind::TranslationStale,
                        side.path.clone(),
                        None,
                        format!("{recorded} {}", content.blob),
                    ));
                }
            }
        }
    }
}

/// 読んだ`対`の並び。同じ`先頭の言語`の`側`の`対`は1つにする（REQ-core-337）
#[derive(Debug, Clone, Default)]
pub struct Pairs(std::collections::BTreeMap<String, Pair>);

impl Pairs {
    /// 加える。同じパスの`対`が既にあれば加えない
    pub fn insert(&mut self, pair: Pair) {
        self.0.entry(pair.path().to_string()).or_insert(pair);
    }
    pub fn contains(&self, first: &str) -> bool {
        self.0.contains_key(first)
    }
    pub fn get(&self, first: &str) -> Option<&Pair> {
        self.0.get(first)
    }
    /// `先頭の言語`の`側`のパスのバイト順
    pub fn iter(&self) -> impl Iterator<Item = &Pair> {
        self.0.values()
    }
    pub fn extend(&mut self, other: Pairs) {
        for pair in other.0.into_values() {
            self.insert(pair);
        }
    }
    /// すべての`対`の`指摘`
    pub(crate) fn findings(&self) -> Vec<Finding> {
        let mut findings = Vec::new();
        for pair in self.iter() {
            pair_findings(pair, &mut findings);
        }
        findings
    }
}
