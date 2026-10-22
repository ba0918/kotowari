//! `側`と`一致の記録`の名前の文法（REQ-core-336、REQ-core-337）

use super::*;

/// `一致の記録`の名前の、拡張子の前の部分（REQ-core-337、REQ-core-339）
pub(super) const RECORD: &str = ".i18n.yaml";

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
pub(super) fn split_extension(name: &str) -> (&str, &str) {
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
pub(super) fn split_directory(path: &str) -> (&str, &str) {
    match path.rfind('/') {
        Some(slash) => (&path[..=slash], &path[slash + 1..]),
        None => ("", path),
    }
}

/// ファイル名（ディレクトリを除く）
pub(super) fn file_name(path: &str) -> &str {
    split_directory(path).1
}
