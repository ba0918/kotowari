//! 文書の行の区切り

/// 行を "\n"、"\r\n"、単独の "\r" で分割する。"\r\n" は1つの行の終わりに数える（TBL-core-010）
pub fn split_lines(content: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut rest = content;
    while let Some(pos) = rest.find(['\n', '\r']) {
        lines.push(&rest[..pos]);
        let ending = if rest[pos..].starts_with("\r\n") {
            2
        } else {
            1
        };
        rest = &rest[pos + ending..];
    }
    // 最後の行（改行なし）
    if !rest.is_empty() {
        lines.push(rest);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    // @kotowari[C-core-document-7]
    #[test]
    fn a_crlf_ends_one_line_and_a_leading_lf_gives_an_empty_first_line() {
        assert_eq!(split_lines("a\r\nb"), vec!["a", "b"]);
        assert_eq!(split_lines("\na"), vec!["", "a"]);
        assert_eq!(split_lines("a\rb"), vec!["a", "b"]);
    }
}
