//! テストの発見と印の結び付け（REQ-071〜REQ-088）

use crate::config::Config;
use crate::ir::{is_valid_id, IrDocument, Item};
use crate::Finding;
use globset::{Glob, GlobSetBuilder};
use std::collections::BTreeSet;
use std::path::Path;
use walkdir::WalkDir;

/// 発見されたテスト
#[derive(Debug, Clone)]
pub struct DiscoveredTest {
    pub name: String,
    pub file_path: String,
    pub line: usize,
    pub marker_ids: BTreeSet<String>,
    /// テストに結び付く位置にある、中身が空または閉じ括弧のない印
    pub invalid_markers: Vec<(usize, String)>,
}

/// 印の解析結果
#[derive(Debug, Clone)]
pub struct Marker {
    pub ids: Vec<String>,
    pub line: usize,
    pub raw: String,
}

/// @kotowari[...] 印を1行から抽出する
pub fn parse_markers_in_line(line: &str, line_num: usize) -> Vec<Marker> {
    let mut markers = Vec::new();
    let mut search_start = 0;

    while let Some(start) = line[search_start..].find("@kotowari[") {
        let abs_start = search_start + start;
        let content_start = abs_start + "@kotowari[".len();

        if let Some(close) = line[content_start..].find(']') {
            let content = &line[content_start..content_start + close];
            let ids: Vec<String> = content
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let raw = line[abs_start..content_start + close + 1].to_string();
            markers.push(Marker {
                ids,
                line: line_num,
                raw,
            });
            search_start = content_start + close + 1;
        } else {
            // 閉じ括弧がない
            let raw = line[abs_start..].to_string();
            markers.push(Marker {
                ids: vec![],
                line: line_num,
                raw,
            });
            break;
        }
    }

    markers
}

/// テストのファイルを glob で収集する
pub fn collect_test_files(
    base: &Path,
    config: &Config,
) -> Vec<(String, String)> {
    let mut builder = GlobSetBuilder::new();
    for pattern in &config.tests.files {
        if let Ok(g) = Glob::new(pattern) {
            builder.add(g);
        }
    }
    let globset = match builder.build() {
        Ok(g) => g,
        Err(_) => return vec![],
    };

    let mut files = Vec::new();

    for entry in WalkDir::new(base)
        .into_iter()
        .filter_entry(|e| {
            // 隠しディレクトリを除外（REQ-019）。ルートは除外しない
            if e.file_type().is_dir() && e.depth() > 0 {
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.')
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let rel = entry
                .path()
                .strip_prefix(base)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .to_string();
            // Windows パス区切りを / に
            let rel = rel.replace('\\', "/");
            if globset.is_match(&rel) {
                files.push((rel, entry.path().to_string_lossy().to_string()));
            }
        }
    }

    files.sort();
    files
}

/// Rust ファイルのテストを tree-sitter で発見する
pub fn discover_rust_tests(
    content: &str,
    file_rel: &str,
    config: &Config,
) -> Result<Vec<DiscoveredTest>, String> {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_rust::LANGUAGE;
    parser
        .set_language(&language.into())
        .map_err(|e| format!("failed to set language: {e}"))?;

    let tree = parser
        .parse(content, None)
        .ok_or_else(|| format!("failed to parse {file_rel}"))?;

    if tree.root_node().has_error() {
        return Err(format!("syntax error in {file_rel}"));
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut tests = Vec::new();

    discover_tests_in_node(
        tree.root_node(),
        content,
        &lines,
        file_rel,
        config,
        &mut tests,
        false,
    );

    Ok(tests)
}

fn discover_tests_in_node(
    node: tree_sitter::Node,
    source: &str,
    lines: &[&str],
    file_rel: &str,
    config: &Config,
    tests: &mut Vec<DiscoveredTest>,
    in_macro: bool,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "function_item" => {
                check_function(child, source, lines, file_rel, config, tests);
            }
            "macro_invocation" => {
                // マクロ名をチェック
                if let Some(macro_node) = child.child_by_field_name("macro") {
                    let macro_name = &source[macro_node.byte_range()];
                    // 末尾のセグメントで比較
                    let last_segment = macro_name.rsplit("::").next().unwrap_or(macro_name);

                    if config.tests.rust.macros.iter().any(|m| {
                        let m_last = m.rsplit("::").next().unwrap_or(m);
                        m_last == last_segment
                    }) {
                        // マクロの中身を Rust として再パース
                        // token_tree を子ノードから直接探す
                        let body_node = {
                            let mut cursor2 = child.walk();
                            child.children(&mut cursor2)
                                .find(|c| c.kind() == "token_tree")
                        };
                        if let Some(body) = body_node {
                            let body_text = &source[body.byte_range()];
                            // token_tree の中身（{ ... } の中）を取得
                            let inner = body_text
                                .strip_prefix('{')
                                .and_then(|s| s.strip_suffix('}'))
                                .unwrap_or(body_text);

                            let byte_offset = body.start_byte()
                                + if body_text.starts_with('{') { 1 } else { 0 };
                            let line_offset = source[..byte_offset].matches('\n').count();

                            // 再パース
                            let mut inner_parser = tree_sitter::Parser::new();
                            let language = tree_sitter_rust::LANGUAGE;
                            inner_parser.set_language(&language.into()).ok();

                            if let Some(inner_tree) = inner_parser.parse(inner, None) {
                                let inner_lines: Vec<&str> = inner.lines().collect();
                                // マクロ内の関数を検出
                                let mut inner_tests = Vec::new();
                                discover_macro_functions(
                                    inner_tree.root_node(),
                                    inner,
                                    &inner_lines,
                                    file_rel,
                                    config,
                                    &mut inner_tests,
                                    line_offset,
                                );
                                tests.extend(inner_tests);
                            }
                        }
                    }
                }
            }
            _ => {
                // 再帰
                discover_tests_in_node(child, source, lines, file_rel, config, tests, in_macro);
            }
        }
    }
}

fn discover_macro_functions(
    node: tree_sitter::Node,
    inner_source: &str,
    inner_lines: &[&str],
    file_rel: &str,
    config: &Config,
    tests: &mut Vec<DiscoveredTest>,
    line_offset: usize,
) {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "function_item" {
            // マクロ内の関数はすべてテストと数える
            if let Some(name_node) = child.child_by_field_name("name") {
                let name = &inner_source[name_node.byte_range()];
                let inner_line = child.start_position().row;
                let actual_line = line_offset + inner_line + 1;

                // inner の関数前のコメントの印を集める
                let all_ids = collect_markers_before_line(inner_lines, inner_line);

                tests.push(DiscoveredTest {
                    name: name.to_string(),
                    file_path: file_rel.to_string(),
                    line: actual_line,
                    marker_ids: all_ids,
                    invalid_markers: Vec::new(),
                });
            }
        } else {
            discover_macro_functions(
                child,
                inner_source,
                inner_lines,
                file_rel,
                config,
                tests,
                line_offset,
            );
        }
    }
}

fn check_function(
    node: tree_sitter::Node,
    source: &str,
    lines: &[&str],
    file_rel: &str,
    config: &Config,
    tests: &mut Vec<DiscoveredTest>,
) {
    let has_test_attr = has_attribute(node, source, "#[test]")
        || config.tests.rust.attributes.iter().any(|attr| {
            has_configured_attribute(node, source, attr)
        });

    if !has_test_attr {
        return;
    }

    let name = match node.child_by_field_name("name") {
        Some(n) => source[n.byte_range()].to_string(),
        None => return,
    };

    let func_line = node.start_position().row; // 0-indexed
    let actual_line = func_line + 1;

    // 前の兄弟ノード（コメント、属性）から印を集める
    let (mut all_ids, mut all_invalid) = collect_markers_from_siblings(node, source, lines);

    // 関数本体の先頭のコメントの印を集める
    let (body_ids, body_invalid) = collect_body_start_markers(node, source);
    all_ids.extend(body_ids);
    all_invalid.extend(body_invalid);

    tests.push(DiscoveredTest {
        name,
        file_path: file_rel.to_string(),
        line: actual_line,
        marker_ids: all_ids,
        invalid_markers: all_invalid,
    });
}

/// 関数が #[test] 属性を持つか
fn has_attribute(node: tree_sitter::Node, source: &str, attr_text: &str) -> bool {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "attribute_item" || child.kind() == "attribute" {
            let text = &source[child.byte_range()];
            if text.trim() == attr_text {
                return true;
            }
        }
    }

    // 前の兄弟ノードの属性も見る
    let mut prev = node.prev_sibling();
    while let Some(p) = prev {
        if p.kind() == "attribute_item" {
            let text = &source[p.byte_range()];
            if text.trim() == attr_text {
                return true;
            }
        } else if p.kind() != "line_comment" && p.kind() != "block_comment" {
            break;
        }
        prev = p.prev_sibling();
    }

    false
}

/// 設定の属性を持つか（パスの完全一致、引数は無視）
fn has_configured_attribute(node: tree_sitter::Node, source: &str, attr_path: &str) -> bool {
    let mut prev = node.prev_sibling();
    while let Some(p) = prev {
        if p.kind() == "attribute_item" {
            let text = &source[p.byte_range()];
            // #[path] or #[path(args)] の形
            let inner = text
                .trim()
                .strip_prefix("#[")
                .and_then(|s| s.strip_suffix(']'));
            if let Some(inner) = inner {
                let path_part = if let Some(paren) = inner.find('(') {
                    &inner[..paren]
                } else {
                    inner
                };
                if path_part.trim() == attr_path {
                    return true;
                }
            }
        } else if p.kind() != "line_comment" && p.kind() != "block_comment" {
            break;
        }
        prev = p.prev_sibling();
    }
    false
}

/// 関数の前の兄弟ノード（属性、コメント）から印を集める
/// 返り値: (正常な印の ID 集合, 空・不正な印の (行, 行の文字) のリスト)
fn collect_markers_from_siblings(
    node: tree_sitter::Node,
    source: &str,
    _lines: &[&str],
) -> (BTreeSet<String>, Vec<(usize, String)>) {
    let mut ids = BTreeSet::new();
    let mut invalid = Vec::new();
    let mut prev = node.prev_sibling();

    while let Some(p) = prev {
        match p.kind() {
            "line_comment" | "block_comment" => {
                let text = &source[p.byte_range()];
                let line_num = p.start_position().row + 1;
                for marker in parse_markers_in_line(text, line_num) {
                    if marker.ids.is_empty() {
                        invalid.push((line_num, text.to_string()));
                    } else {
                        for id in &marker.ids {
                            ids.insert(id.clone());
                        }
                    }
                }
            }
            "attribute_item" => {
                // 属性は飛ばして前のコメントも見る
            }
            _ => {
                // 空行チェック: 前のノードと現在のノードの間に空行があれば止める
                break;
            }
        }

        // 前のノードとさらに前のノードの間に空行があるか
        let prev_prev = p.prev_sibling();
        if let Some(pp) = prev_prev {
            let gap_start = pp.end_position().row;
            let gap_end = p.start_position().row;
            if gap_end > gap_start + 1 {
                // 空行がある → ここまで
                break;
            }
        }

        prev = p.prev_sibling();
    }

    (ids, invalid)
}

/// 指定行（0-indexed）の前のコメント塊から印を集める
/// コメント（// か /*）と属性（#[）の行だけ遡り、空行またはそれ以外の行で切れる
fn collect_markers_before_line(lines: &[&str], target_line: usize) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    if target_line == 0 {
        return ids;
    }

    let mut line_idx = target_line.saturating_sub(1);
    loop {
        if line_idx >= lines.len() {
            break;
        }
        let line = lines[line_idx].trim();
        if line.is_empty() {
            break;
        }

        // コメントと属性の行だけ遡る。それ以外（関数定義など）で停止する
        if !line.starts_with("//") && !line.starts_with("/*") && !line.starts_with("#[") {
            break;
        }

        for marker in parse_markers_in_line(line, line_idx + 1) {
            for id in &marker.ids {
                ids.insert(id.clone());
            }
        }

        if line_idx == 0 {
            break;
        }
        line_idx -= 1;
    }

    ids
}

/// 関数本体の先頭のコメントから印を集める
/// 返り値: (正常な印の ID 集合, 空・不正な印の (行, 行の文字) のリスト)
fn collect_body_start_markers(node: tree_sitter::Node, source: &str) -> (BTreeSet<String>, Vec<(usize, String)>) {
    let mut ids = BTreeSet::new();
    let mut invalid = Vec::new();

    if let Some(body) = node.child_by_field_name("body") {
        let mut cursor = body.walk();
        let mut past_open_brace = false;
        for child in body.children(&mut cursor) {
            if child.kind() == "{" {
                past_open_brace = true;
                continue;
            }
            if child.kind() == "}" {
                continue;
            }
            if !past_open_brace {
                continue;
            }
            if child.kind() == "line_comment" || child.kind() == "block_comment" {
                let text = &source[child.byte_range()];
                let line_num = child.start_position().row + 1;
                for marker in parse_markers_in_line(text, line_num) {
                    if marker.ids.is_empty() {
                        invalid.push((line_num, text.to_string()));
                    } else {
                        for id in &marker.ids {
                            ids.insert(id.clone());
                        }
                    }
                }
            } else {
                // コメント以外が来たら止める（本体の途中の印は無視）
                break;
            }
        }
    }

    (ids, invalid)
}

/// テストの発見と印のチェックの全体
pub fn discover_and_check(
    base: &Path,
    config: &Config,
    docs: &[IrDocument],
    known_ids: &BTreeSet<String>,
    ir_path: &str,
    findings: &mut Vec<Finding>,
) -> Result<(), crate::StopReason> {
    let test_files = collect_test_files(base, config);
    let mut all_tests: Vec<DiscoveredTest> = Vec::new();
    let mut all_marker_ids: BTreeSet<String> = BTreeSet::new();

    for (rel_path, abs_path) in &test_files {
        let bytes = match std::fs::read(abs_path) {
            Ok(b) => b,
            Err(e) => {
                return Err(crate::StopReason::UnreadableFile(
                    format!("{abs_path}: {e}"),
                ));
            }
        };
        let content = match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => {
                return Err(crate::StopReason::NonUtf8File(rel_path.clone()));
            }
        };

        let ext = std::path::Path::new(rel_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if ext == "rs" {
            // 問い合わせのある言語（Rust）
            match discover_rust_tests(&content, rel_path, config) {
                Ok(tests) => {
                    for test in &tests {
                        // 印の検証
                        check_test_markers(test, rel_path, known_ids, findings);
                        for id in &test.marker_ids {
                            all_marker_ids.insert(id.clone());
                        }
                        // REQ-072: テストに結び付く空・不正な印
                        for (line_num, raw) in &test.invalid_markers {
                            findings.push(Finding {
                                kind: "invalid_marker".to_string(),
                                severity: "error".to_string(),
                                path: rel_path.clone(),
                                line: Some(*line_num),
                                detail: raw.clone(),
                            });
                        }
                    }
                    all_tests.extend(tests);
                }
                Err(_) => {
                    findings.push(Finding {
                        kind: "unparsable_file".to_string(),
                        severity: "error".to_string(),
                        path: rel_path.clone(),
                        line: None,
                        detail: rel_path.clone(),
                    });
                }
            }
        } else {
            // 問い合わせの無い言語: 印を拾い、検査もする（REQ-076, REQ-087, REQ-072, REQ-054）
            for (idx, line) in content.lines().enumerate() {
                let line_num = idx + 1;
                for marker in parse_markers_in_line(line, line_num) {
                    if marker.ids.is_empty() {
                        // REQ-072: 空の印、または閉じ括弧のない印
                        findings.push(Finding {
                            kind: "invalid_marker".to_string(),
                            severity: "error".to_string(),
                            path: rel_path.clone(),
                            line: Some(line_num),
                            detail: line.to_string(),
                        });
                    } else {
                        for id in &marker.ids {
                            all_marker_ids.insert(id.clone());
                            // REQ-054: 存在しない ID への参照
                            if !known_ids.contains(id) {
                                findings.push(Finding {
                                    kind: "unresolved_reference".to_string(),
                                    severity: "error".to_string(),
                                    path: rel_path.clone(),
                                    line: Some(line_num),
                                    detail: id.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // REQ-085: テストのない要求
    for doc in docs {
        for item in &doc.items {
            if let Item::Requirement {
                id, verification, ..
            } = item
            {
                if let Some(v) = verification {
                    if v != "review" && is_valid_id(id) && !all_marker_ids.contains(id) {
                        let path = format!("{}/{}", ir_path, doc.filename);
                        findings.push(Finding {
                            kind: "requirement_without_test".to_string(),
                            severity: "error".to_string(),
                            path,
                            line: Some(item.item_line()),
                            detail: id.clone(),
                        });
                    }
                }
            }
        }
    }

    // REQ-086: 印の無いテスト
    for test in &all_tests {
        if test.marker_ids.is_empty() {
            findings.push(Finding {
                kind: "test_without_id".to_string(),
                severity: "error".to_string(),
                path: test.file_path.clone(),
                line: Some(test.line),
                detail: test.name.clone(),
            });
        }
    }

    Ok(())
}

/// テストの印を検証する
fn check_test_markers(
    test: &DiscoveredTest,
    file_path: &str,
    known_ids: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    // invalid_marker のチェックは行単位で行うべきだが、
    // ここではテストに結び付いた印のチェックを行う
    // REQ-054 の unresolved_reference は印からも出る
    for id in &test.marker_ids {
        if !id.is_empty() && !known_ids.contains(id) {
            findings.push(Finding {
                kind: "unresolved_reference".to_string(),
                severity: "error".to_string(),
                path: file_path.to_string(),
                line: Some(test.line),
                detail: id.clone(),
            });
        }
    }
}

