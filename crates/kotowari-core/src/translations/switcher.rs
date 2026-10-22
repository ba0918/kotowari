//! `切り替えの行`（REQ-core-346）

use super::*;

/// `題名`の行と、`題名`の後の最初の空でない行（`題名`が無ければファイルの最初の空でない行）
pub(super) fn switcher_candidate(text: &str) -> (Option<usize>, Option<(usize, &str)>) {
    // 先頭の BOM は読み飛ばす（REQ-core-111）。BOM は1行目の中にあるので行の番号は変わらない
    let lines = crate::ir::split_lines(crate::strip_bom(text));
    let title = lines.iter().position(|line| line.starts_with("# "));
    let from = title.map_or(0, |title| title + 1);
    let candidate = lines
        .iter()
        .enumerate()
        .skip(from)
        .find(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| (index + 1, *line));
    (title.map(|title| title + 1), candidate)
}

/// `切り替えの行`と同じ行を空にした文。行の番号は変わらない。言語が1つなら文のまま（REQ-core-346）
pub(crate) fn without_switcher(
    config: &crate::config::Config,
    first: &str,
    language: &str,
    text: &str,
) -> Option<String> {
    if config.languages().len() < 2 {
        return None;
    }
    let expected = Context::new(&Pairs::default(), config).switcher(first, language);
    switcher_line(text, &expected).map(|line| blank_line(text, line))
}

/// あるべき`切り替えの行`と同じ行。無ければ None
pub(crate) fn switcher_line(text: &str, expected: &str) -> Option<usize> {
    match switcher_candidate(text).1 {
        Some((line, found)) if found == expected => Some(line),
        _ => None,
    }
}

/// その行を空にした文。行の番号は変わらない
pub(crate) fn blank_line(text: &str, line: usize) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    let mut number = 1;
    loop {
        let end = rest.find(['\n', '\r']).unwrap_or(rest.len());
        if number != line {
            out.push_str(&rest[..end]);
        }
        let ending = if rest[end..].starts_with("\r\n") {
            2
        } else {
            (end < rest.len()) as usize
        };
        out.push_str(&rest[end..end + ending]);
        rest = &rest[end + ending..];
        number += 1;
        if rest.is_empty() {
            return out;
        }
    }
}
