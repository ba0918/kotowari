//! HTML の文字の逃がし方とページの骨格

/// 本文と属性の値に置く文字を逃がす
pub(crate) fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// ページの名前を相対リンクの href にする。予約されていない文字のほかはパーセント符号にする
pub(crate) fn href(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for byte in name.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

/// 共通のスタイルを相対パスで参照するページの骨格（REQ-view-002）
pub(crate) fn shell(title: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html lang=\"ja\">\n<head>\n<meta charset=\"utf-8\">\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
<title>{}</title>\n<link rel=\"stylesheet\" href=\"style.css\">\n</head>\n<body>\n{body}</body>\n</html>\n",
        escape(title)
    )
}
