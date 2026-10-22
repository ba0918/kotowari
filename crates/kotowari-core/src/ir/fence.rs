//! `コードブロック`の囲みを追う

use super::*;

/// 閉じない`コードブロック`の開始の行と、その行の文字（REQ-core-112）
pub(super) fn unclosed_code_block(content: &str) -> Option<(usize, String)> {
    let mut open = None;
    for (idx, (line, part)) in fenced_lines(content).into_iter().enumerate() {
        match part {
            Fenced::Opening => open = Some((idx + 1, line)),
            Fenced::Closing => open = None,
            Fenced::Outside | Fenced::Inside => {}
        }
    }
    open.map(|(line, raw)| (line, raw.to_string()))
}

/// コードブロックの囲みの情報
#[derive(Debug, Clone)]
pub struct CodeFence {
    /// 囲みの文字（'`' or '~'）
    pub fence_char: char,
    /// 囲みの長さ（3以上）
    pub fence_len: usize,
    /// 言語（あれば）
    pub lang: String,
}

/// 行がコードブロックの囲みのどこにあるか
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Fenced {
    Outside,
    /// 開きの囲みの行
    Opening,
    Inside,
    /// 閉じの囲みの行
    Closing,
}

/// 文書の行（TBL-core-010 の区切り）と、それぞれがコードブロックの囲みのどこにあるか。
/// 閉じずに文書が終われば、開きより後はすべて囲みの中
pub(crate) fn fenced_lines(content: &str) -> Vec<(&str, Fenced)> {
    let mut open: Option<CodeFence> = None;
    split_lines(content)
        .into_iter()
        .map(|line| {
            let part = match &open {
                Some(fence) if is_closing_fence(line, fence) => Fenced::Closing,
                Some(_) => Fenced::Inside,
                None => match parse_opening_fence(line) {
                    Some(fence) => {
                        open = Some(fence);
                        Fenced::Opening
                    }
                    None => Fenced::Outside,
                },
            };
            if part == Fenced::Closing {
                open = None;
            }
            (line, part)
        })
        .collect()
}

/// 行がコードブロックの開始の囲みかどうかを判定する。
/// 3つ以上の ` か ~ で始まる行が該当する。
pub fn parse_opening_fence(line: &str) -> Option<CodeFence> {
    let bytes = line.as_bytes();
    if bytes.len() < 3 {
        return None;
    }
    let fence_char = bytes[0] as char;
    if fence_char != '`' && fence_char != '~' {
        return None;
    }
    let fence_len = bytes.iter().take_while(|&&b| b == bytes[0]).count();
    if fence_len < 3 {
        return None;
    }
    let lang = line[fence_len..].trim().to_string();
    Some(CodeFence {
        fence_char,
        fence_len,
        lang,
    })
}

/// 行が開いている囲みを閉じるかどうかを判定する。
pub fn is_closing_fence(line: &str, opening: &CodeFence) -> bool {
    let bytes = line.as_bytes();
    if bytes.is_empty() || bytes[0] as char != opening.fence_char {
        return false;
    }
    let fence_len = bytes.iter().take_while(|&&b| b == bytes[0]).count();
    if fence_len < opening.fence_len {
        return false;
    }
    // 閉じる囲みの後は空白だけ
    line[fence_len..].trim().is_empty()
}
