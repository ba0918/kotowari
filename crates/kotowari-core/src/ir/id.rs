//! `ID` の文法（TBL-core-004、REQ-core-043）

/// 項目の ID の種別
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdPrefix {
    Req,
    Tbl,
    Prop,
    Ex,
    Flag,
}

/// 見出しの`ID`がその`項目`の種類の形に合うか。合わない見出しはスキーマの側が
/// unknown_heading にしているので、`項目`にしない（REQ-core-043、REQ-core-114）
pub(super) fn is_item_id(id: &str, prefix: IdPrefix) -> bool {
    id_prefix(id) == Some(prefix) && is_valid_id(id)
}

/// ID の接頭辞を判定する
pub(super) fn id_prefix(id: &str) -> Option<IdPrefix> {
    if id.starts_with("REQ-") {
        Some(IdPrefix::Req)
    } else if id.starts_with("TBL-") {
        Some(IdPrefix::Tbl)
    } else if id.starts_with("PROP-") {
        Some(IdPrefix::Prop)
    } else if id.starts_with("EX-") {
        Some(IdPrefix::Ex)
    } else if id.starts_with("FLAG-") {
        Some(IdPrefix::Flag)
    } else {
        None
    }
}

/// ID を接頭辞の後ろの「省いてよい名前」と「数字」に分ける。形に合わなければ None（REQ-core-124）
pub(super) fn split_id(s: &str) -> Option<(Option<&str>, &str)> {
    let prefix = id_prefix(s)?;
    let rest = match prefix {
        IdPrefix::Req => &s[4..],
        IdPrefix::Tbl => &s[4..],
        IdPrefix::Prop => &s[5..],
        IdPrefix::Ex => &s[3..],
        IdPrefix::Flag => &s[5..],
    };
    // 最後の "-" より後ろを数字、その前を名前とする
    let (name, digits) = match rest.rsplit_once('-') {
        Some((name, digits)) => (Some(name), digits),
        None => (None, rest),
    };
    let digits_ok = digits.len() >= 3
        && (digits.len() == 3 || !digits.starts_with('0'))
        && digits.chars().all(|c| c.is_ascii_digit());
    if !digits_ok {
        return None;
    }
    if name.is_some_and(|name| !is_valid_id_name(name)) {
        return None;
    }
    Some((name, digits))
}

/// ID の名前の形（小文字の英字で始まり、2文字目からは小文字の英数字と "-"。REQ-core-124）
pub(super) fn is_valid_id_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// ID の形式を検証する（省いてよい名前、数字は3桁以上、4桁以上では先頭の0を認めない）
pub fn is_valid_id(s: &str) -> bool {
    split_id(s).is_some()
}

/// ID の名前を返す。名前が無ければ None（REQ-core-124、REQ-core-167）
pub fn id_name(s: &str) -> Option<&str> {
    split_id(s).and_then(|(name, _)| name)
}
