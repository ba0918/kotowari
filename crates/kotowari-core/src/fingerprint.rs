//! `項目`と`シナリオ`の本文と`指紋`（TBL-core-027 の "body"、REQ-core-203）。
//! "kotowari query" の "body"、"kotowari list" の "fingerprint"、`ガイドの印`の照合が同じ切り出しを使う

use crate::ir::{IrDocument, Item};
use sha2::{Digest, Sha256};

/// REQ-core-203: `指紋`の元の行を "\n" でつないだ UTF-8 のバイト列の SHA-256 を、16進の小文字で書いた先頭の8文字。
/// `項目`は本文から "- source:" の行を除いた並び（除いた後で空の行を除き直さない）、
/// `シナリオ`はステップの行だけの並び。行はどれも行の終わりの文字を含まない。
/// 文書単位の "- deferred:" の行を持つ文書の`要求`では、その1つ目の行を並びの先頭に加える。
/// lines は doc の行
pub fn fingerprint_of(doc: &IrDocument, item: &Item, lines: &[&str]) -> String {
    let mut fingerprinted = Vec::new();
    if let (Item::Requirement { .. }, Some(deferral)) = (item, &doc.deferred) {
        fingerprinted.push(lines[deferral.line - 1].to_string());
    }
    fingerprinted.extend(fingerprint_lines(item, lines));
    let joined = fingerprinted.join("\n");
    let digest = Sha256::digest(joined.as_bytes());
    digest.iter().take(4).map(|b| format!("{b:02x}")).collect()
}

fn fingerprint_lines(item: &Item, lines: &[&str]) -> Vec<String> {
    match item {
        Item::Scenario { steps, .. } => steps.iter().map(|(_, step)| step.clone()).collect(),
        _ => {
            let source_line = source_line_of(item);
            body_lines(item, lines)
                .into_iter()
                .filter(|(number, _)| Some(*number) != source_line)
                .map(|(_, line)| line.to_string())
                .collect()
        }
    }
}

/// スキーマの側が返した "- source:" の行（1始まり）。行の文字の形では見分けない
fn source_line_of(item: &Item) -> Option<usize> {
    match item {
        Item::Requirement { source_line, .. }
        | Item::DecisionTable { source_line, .. }
        | Item::Property { source_line, .. }
        | Item::FlagEntry { source_line, .. } => *source_line,
        Item::Scenario { .. } | Item::GlossaryTerm { .. } => None,
    }
}

/// TBL-core-027: 本文の行。`項目`は見出しの次の行から、スキーマの側が返すその`項目`の最後の行まで
/// （REQ-core-169、REQ-core-170）。`シナリオ`は "@id" のタグの行から最後のステップの行まで。
/// 先頭と末尾の空の行は含めない
pub fn body_of(item: &Item, lines: &[&str]) -> Vec<String> {
    body_lines(item, lines)
        .into_iter()
        .map(|(_, line)| line.to_string())
        .collect()
}

/// 本文の行を、1始まりの行番号と組にして並べる（`body_of` と同じ範囲）
fn body_lines<'a>(item: &Item, lines: &[&'a str]) -> Vec<(usize, &'a str)> {
    // 1始まりの行の範囲（両端を含む）
    let (mut first, mut last) = match item {
        Item::Scenario {
            line,
            tag_line,
            steps,
            ..
        } => (
            tag_line.unwrap_or(*line),
            steps.last().map_or(*line, |(step_line, _)| *step_line),
        ),
        _ => (
            item.item_line() + 1,
            item.end_line().unwrap_or(item.item_line()).min(lines.len()),
        ),
    };
    let is_blank = |number: usize| lines[number - 1].is_empty();
    while first <= last && is_blank(first) {
        first += 1;
    }
    while first <= last && is_blank(last) {
        last -= 1;
    }
    (first..=last).map(|n| (n, lines[n - 1])).collect()
}
