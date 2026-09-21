//! 用語、曖昧語、文書名の参照の検査（REQ-core-063〜REQ-core-070, REQ-core-104）

use crate::ir::{is_valid_id, IrDocument, Item};
use crate::{Finding, FindingKind};
use std::collections::BTreeSet;

/// 用語集から用語の集合を作る
pub fn collect_glossary_terms(docs: &[IrDocument], directory: &str) -> Option<BTreeSet<String>> {
    let mut has_glossary = false;
    let mut terms = BTreeSet::new();

    for doc in docs {
        if doc.is_glossary_in_chain(directory) {
            has_glossary = true;
            let duplicate_rows = doc.duplicate_glossary_rows(docs);
            for item in &doc.items {
                if duplicate_rows.contains(&item.item_line()) {
                    continue;
                }
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

/// 対象の行からバッククォートで囲んだ語を検査する（REQ-core-064, REQ-core-065, REQ-core-116）
pub fn check_unknown_terms(
    text: &str,
    line: usize,
    glossary: &Option<BTreeSet<String>>,
    _known_ids: &BTreeSet<String>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    // REQ-core-116: バッククォートが奇数の行は unclosed_backtick
    // 二重引用符の外のバッククォートだけを数える
    if crate::has_odd_backticks_outside_quotes(text) {
        findings.push(Finding::new(FindingKind::UnclosedBacktick, path.to_string(), Some(line), text.to_string()));
        return;
    }
    for content in crate::extract_backtick_contents_outside_quotes(text) {
        // REQ-core-064: 前後の空白を除く
        let trimmed = content.trim();
        // REQ-core-064: 中身が空の囲み
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

/// 曖昧語の検査（REQ-core-066, REQ-core-067）
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
    while chars.get(pos).is_some() {
        // 現在位置から始まる最長の曖昧語を探す
        let remaining: String = chars[pos..].iter().collect();
        let best_word = vague_words
            .iter()
            .filter(|w| !w.is_empty() && remaining.starts_with(w.as_str()))
            .max_by_key(|w| w.chars().count());
        if let Some(word) = best_word {
            findings.push(Finding::new(FindingKind::VagueWord, path.to_string(), Some(line), word.clone()));
            pos += word.chars().count();
        } else {
            pos += 1;
        }
    }
}

/// 文書名の参照の検査（REQ-core-069, REQ-core-070, TBL-core-014）
pub fn check_document_references(
    docs: &[IrDocument],
    ir_path: &str,
    ir_paths: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.relative_path);
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
                find_doc_refs(part, line_num, &path, &doc.directory, ir_paths, findings);
            }
        }
    }
}

/// 文書名の参照を見つける
fn find_doc_refs(
    text: &str,
    line: usize,
    path: &str,
    directory: &str,
    ir_paths: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    let bytes = text.as_bytes();
    let len = text.len();
    let mut i = 0;

    // ".md" を探す
    while let Some(md_pos) = text.get(i..).and_then(|s| s.find(".md")) {
        let md_abs = i + md_pos;

        // TBL-core-014: 出典の印やパスの途中は参照にならない
        let after_md = md_abs + 3;
        if after_md < len {
            let next_byte = bytes[after_md];
            if next_byte.is_ascii_alphanumeric() || next_byte == b'_' || next_byte == b'-' || next_byte == b'#' || next_byte == b'/' {
                i = md_abs + 1;
                continue;
            }
        }

        // 語の途中から拾い直さないため、要素内のドットも含めて辿る
        let mut name_start = md_abs;
        while name_start > 0 {
            let prev = bytes[name_start - 1];
            if prev.is_ascii_lowercase() || prev.is_ascii_digit() || prev == b'-' || prev == b'/' || prev == b'.' {
                name_start -= 1;
            } else {
                break;
            }
        }

        let doc_name = &text[name_start..md_abs + 3];

        let boundary_ok = if name_start == 0 {
            true
        } else {
            let prev_char = text[..name_start].chars().next_back().unwrap();
            !prev_char.is_ascii_alphanumeric()
                && !matches!(prev_char, '_' | '-' | '/' | '.' | '`')
        };

        let mut elements = text[name_start..md_abs].rsplit('/');
        let name_ok = elements.next().is_some_and(is_reference_name);
        let directories_ok = elements.all(|element| {
            element == "." || element == ".." || is_reference_name(element)
        });

        if boundary_ok && name_ok && directories_ok {
            let target = if doc_name.contains('/') {
                doc_name.to_string()
            } else {
                crate::join_display_path(directory, doc_name)
            };
            if !ir_paths.contains(&target) {
                findings.push(Finding::new(FindingKind::MissingDocument, path.to_string(), Some(line), doc_name.to_string()));
            }
        }

        i = md_abs + 3;
    }
}

fn is_reference_name(element: &str) -> bool {
    !element.is_empty()
        && element.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// IR 文書の対象の行（TBL-core-013）で用語と曖昧語を検査する
pub fn check_terms_and_vague_words(
    docs: &[IrDocument],
    known_ids: &BTreeSet<String>,
    vague_words: &[String],
    ir_path: &str,
    findings: &mut Vec<Finding>,
) {
    for doc in docs {
        let glossary = collect_glossary_terms(docs, &doc.directory);
        let path = crate::join_display_path(ir_path, &doc.relative_path);
        for item in &doc.items {
            match item {
                Item::Requirement { statements, .. } => {
                    for (line, text) in statements {
                        check_unknown_terms(text, *line, &glossary, known_ids, &path, findings);
                        check_vague_words(text, *line, vague_words, &path, findings);
                    }
                }
                Item::Property { statements, .. } => {
                    for (line, text) in statements {
                        check_unknown_terms(text, *line, &glossary, known_ids, &path, findings);
                        check_vague_words(text, *line, vague_words, &path, findings);
                    }
                }
                Item::Scenario { steps, .. } => {
                    for (line, text) in steps {
                        check_unknown_terms(text, *line, &glossary, known_ids, &path, findings);
                        check_vague_words(text, *line, vague_words, &path, findings);
                    }
                }
                // 用語集の意味の列、問題の記録の本文は対象外
                _ => {}
            }
        }
    }
}
