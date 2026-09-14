//! 用語、曖昧語、文書名の参照の検査（REQ-063〜REQ-070, REQ-104）

use crate::ir::{is_valid_id, IrDocument, Item};
use crate::{Finding, FindingKind};
use std::collections::BTreeSet;

/// 用語集から用語の集合を作る
pub fn collect_glossary_terms(docs: &[IrDocument]) -> Option<BTreeSet<String>> {
    let mut has_glossary = false;
    let mut terms = BTreeSet::new();

    for doc in docs {
        if doc.filename == "CONTEXT.md" {
            has_glossary = true;
            for item in &doc.items {
                if let Item::GlossaryTerm { term, .. } = item {
                    terms.insert(term.clone());
                }
            }
        }
    }

    if has_glossary {
        Some(terms)
    } else {
        None
    }
}

/// 対象の行からバッククォートで囲んだ語を検査する（REQ-064, REQ-065, REQ-116）
pub fn check_unknown_terms(
    text: &str,
    line: usize,
    glossary: &Option<BTreeSet<String>>,
    _known_ids: &BTreeSet<String>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    // REQ-116: バッククォートが奇数の行は unclosed_backtick
    // 二重引用符の外のバッククォートだけを数える
    if crate::has_odd_backticks_outside_quotes(text) {
        findings.push(Finding::new(FindingKind::UnclosedBacktick, path.to_string(), Some(line), text.to_string()));
        return;
    }
    for content in crate::extract_backtick_contents_outside_quotes(text) {
        // REQ-064: 前後の空白を除く
        let trimmed = content.trim();
        // REQ-064: 中身が空の囲み
        if trimmed.is_empty() {
            findings.push(Finding::new(FindingKind::UnknownTerm, path.to_string(), Some(line), "``".to_string()));
            continue;
        }
        // ID なら参照チェック（ir モジュールで済み）、用語チェックはしない
        if is_valid_id(trimmed) {
            continue;
        }
        // 用語集にあるか
        let is_known = match glossary {
            Some(terms) => terms.contains(trimmed),
            None => false, // 用語集がない → すべて unknown
        };
        if !is_known {
            findings.push(Finding::new(FindingKind::UnknownTerm, path.to_string(), Some(line), trimmed.to_string()));
        }
    }
}

/// 曖昧語の検査（REQ-066, REQ-067）
/// 行の左から最長一致で重ならない形で数える（A142）
pub fn check_vague_words(
    text: &str,
    line: usize,
    vague_words: &[String],
    path: &str,
    findings: &mut Vec<Finding>,
) {
    if vague_words.is_empty() {
        return;
    }
    let chars: Vec<char> = text.chars().collect();
    let mut pos = 0;
    while pos < chars.len() {
        // 現在位置から始まる最長の曖昧語を探す
        let mut best_word: Option<&String> = None;
        let mut best_len: usize = 0;
        let remaining: String = chars[pos..].iter().collect();
        for word in vague_words {
            if word.is_empty() {
                continue;
            }
            if remaining.starts_with(word.as_str()) {
                let wlen = word.chars().count();
                if wlen > best_len {
                    best_word = Some(word);
                    best_len = wlen;
                }
            }
        }
        if let Some(word) = best_word {
            findings.push(Finding::new(FindingKind::VagueWord, path.to_string(), Some(line), word.clone()));
            pos += best_len;
        } else {
            pos += 1;
        }
    }
}

/// 文書名の参照の検査（REQ-069, REQ-070, TBL-014）
pub fn check_document_references(
    docs: &[IrDocument],
    ir_path: &str,
    ir_filenames: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    // 句読点の集合
    let punctuation: &[char] = &[
        ',', '.', ':', ';', '(', ')', '"', '\'',
        '、', '。', '，', '．', '（', '）', '「', '」', '『', '』', '\u{201C}', '\u{201D}',
    ];

    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.filename);
        let lines = crate::ir::split_lines_for_doc_ref(&doc.filename, &doc.raw_content);

        let mut current_fence: Option<crate::ir::CodeFence> = None;
        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            if let Some(ref fence) = current_fence {
                if crate::ir::is_closing_fence(line, fence) {
                    current_fence = None;
                }
                continue;
            }
            if let Some(fence) = crate::ir::parse_opening_fence(line) {
                current_fence = Some(fence);
                continue;
            }

            // 二重引用符の中を除外するため、引用符の外の部分だけ検査
            let parts = crate::split_outside_quotes(line);
            for part in &parts {
                find_doc_refs(part, line_num, &path, ir_filenames, punctuation, findings);
            }
        }
    }
}

/// 文書名の参照を見つける
fn find_doc_refs(
    text: &str,
    line: usize,
    path: &str,
    ir_filenames: &BTreeSet<String>,
    punctuation: &[char],
    findings: &mut Vec<Finding>,
) {
    // 英小文字と数字とハイフンの並びに ".md" が続くものを探す
    let bytes = text.as_bytes();
    let len = text.len();
    let mut i = 0;

    while i < len {
        // ".md" を探す
        if let Some(md_pos) = text[i..].find(".md") {
            let md_abs = i + md_pos;

            // TBL-014: ".md" の後が英数字、"_"、"-" でないこと
            let after_md = md_abs + 3;
            if after_md < len {
                let next_byte = bytes[after_md];
                if next_byte.is_ascii_alphanumeric() || next_byte == b'_' || next_byte == b'-' {
                    i = md_abs + 1;
                    continue;
                }
            }

            // ".md" の前の英小文字と数字とハイフンの並びを逆に辿る
            let mut name_start = md_abs;
            while name_start > 0 {
                let prev = bytes[name_start - 1];
                if prev.is_ascii_lowercase() || prev.is_ascii_digit() || prev == b'-' {
                    name_start -= 1;
                } else {
                    break;
                }
            }

            if name_start < md_abs {
                let doc_name = &text[name_start..md_abs + 3];

                // 直前が行頭、空白、句読点のいずれか
                let boundary_ok = if name_start == 0 {
                    true
                } else {
                    let prev_char = text[..name_start].chars().next_back().unwrap();
                    prev_char.is_whitespace()
                        || punctuation.contains(&prev_char)
                };

                if boundary_ok {
                    // IR の置き場にその文書があるか
                    if !ir_filenames.contains(doc_name) {
                        findings.push(Finding::new(FindingKind::MissingDocument, path.to_string(), Some(line), doc_name.to_string()));
                    }
                }
            }

            i = md_abs + 3;
        } else {
            break;
        }
    }
}

/// IR 文書の対象の行（TBL-013）で用語と曖昧語を検査する
pub fn check_terms_and_vague_words(
    docs: &[IrDocument],
    glossary: &Option<BTreeSet<String>>,
    known_ids: &BTreeSet<String>,
    vague_words: &[String],
    ir_path: &str,
    findings: &mut Vec<Finding>,
) {
    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.filename);
        for item in &doc.items {
            match item {
                Item::Requirement { statements, .. } => {
                    for (line, text) in statements {
                        check_unknown_terms(text, *line, glossary, known_ids, &path, findings);
                        check_vague_words(text, *line, vague_words, &path, findings);
                    }
                }
                Item::Property { statements, .. } => {
                    for (line, text) in statements {
                        check_unknown_terms(text, *line, glossary, known_ids, &path, findings);
                        check_vague_words(text, *line, vague_words, &path, findings);
                    }
                }
                Item::Scenario { steps, .. } => {
                    for (line, text) in steps {
                        check_unknown_terms(text, *line, glossary, known_ids, &path, findings);
                        check_vague_words(text, *line, vague_words, &path, findings);
                    }
                }
                // 用語集の意味の列、問題の記録の本文は対象外
                _ => {}
            }
        }
    }
}
