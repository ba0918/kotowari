//! テストの発見と印の結び付け（REQ-core-071〜REQ-core-088）

use crate::comment_block::LineMap;
use crate::config::Config;
use crate::ir::{IrDocument, Item, is_valid_id};
use crate::test_queries::{ParsedFile, TestQueries, language_of};
use crate::{Finding, FindingKind};
use ast_grep_core::Node;
use ast_grep_core::tree_sitter::{LanguageExt, StrDoc};
use ast_grep_language::SupportLang;
use globset::{Glob, GlobSetBuilder};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use walkdir::WalkDir;

/// 発見されたテスト
#[derive(Debug, Clone)]
pub struct DiscoveredTest {
    /// `テスト`の名前。`問い合わせ`が "$NAME" を捕まえなければ None（REQ-core-180）
    pub name: Option<String>,
    /// `テスト`の節の最初の行の全体の文字から前後の空白を除いたもの
    pub first_line_text: String,
    pub file_path: String,
    /// `テスト`の節の最初の行
    pub line: usize,
    /// 印の出現ごとの (ID, 印のある行)。A152: 同じ ID の印が複数あっても出現ごとに数える
    pub marker_ids: Vec<(String, usize)>,
    /// テストに結び付く位置にある、中身が空または閉じ括弧のない印
    pub invalid_markers: Vec<(usize, String)>,
}

/// `印`の1つの出現。`ID` ごとに1件で、同じ行の同じ `ID` の2つ目も1件（TBL-core-026 の "tests"）
#[derive(Debug, Clone)]
pub struct TestMarker {
    /// 印に書かれた `ID`
    pub id: String,
    /// `テストのファイル`の`基準のディレクトリ`からの相対パス
    pub path: String,
    /// 印のある行
    pub line: usize,
    /// `テスト`の名前。`問い合わせの無い言語`と、`問い合わせ`が "$NAME" を捕まえない`テスト`では無い
    pub name: Option<String>,
}

/// 印の解析結果
#[derive(Debug, Clone)]
pub struct Marker {
    pub ids: Vec<String>,
    pub line: usize,
}

/// @kotowari[...] 印を1行から抽出する
pub fn parse_markers_in_line(line: &str, line_num: usize) -> Vec<Marker> {
    let mut markers = Vec::new();
    let mut search_start = 0;

    while let Some(start) = line[search_start..].find("@kotowari[") {
        let abs_start = search_start + start;
        let content_start = abs_start + "@kotowari[".len();

        if let Some(close) = line[content_start..].find(']') {
            // "]" の位置。"]" から "@kotowari[" は始まらないので、次の探索はここから始める
            let close_pos = content_start + close;
            let content = &line[content_start..close_pos];
            let ids: Vec<String> = content
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            markers.push(Marker { ids, line: line_num });
            search_start = close_pos;
        } else {
            // 閉じ括弧がない
            markers.push(Marker { ids: vec![], line: line_num });
            break;
        }
    }

    markers
}

/// テストのファイルを glob で収集する
pub fn collect_test_files(
    base: &Path,
    config: &Config,
) -> Result<Vec<(String, String)>, crate::StopReason> {
    let mut builder = GlobSetBuilder::new();
    for pattern in &config.tests.files {
        // glob の構文は Config::parse で検証済み
        let g = Glob::new(pattern).map_err(|e| {
            crate::StopReason::ConfigError(format!("invalid glob: {pattern}: {e}"))
        })?;
        builder.add(g);
    }
    let globset = builder.build().map_err(|e| {
        crate::StopReason::ConfigError(format!("glob build error: {e}"))
    })?;

    let mut files = Vec::new();

    for entry in WalkDir::new(base)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // 隠しディレクトリを除外（REQ-core-019）。ルートは除外しない
            // ディレクトリのシンボリックリンクは辿らない（REQ-core-079, A102）
            if e.depth() > 0 {
                let ft = e.file_type();
                if ft.is_symlink() {
                    // シンボリックリンク: ファイルなら含める、ディレクトリなら除外
                    // filter_entry ではディレクトリかどうかで判定
                    // WalkDir は follow_links(false) なのでシンボリックリンクは展開されない
                    // ここで辿って判定する
                    if let Ok(meta) = std::fs::metadata(e.path()) {
                        if meta.is_dir() {
                            return false; // ディレクトリリンクは辿らない
                        }
                    }
                    return true; // ファイルリンクは含める
                }
                if ft.is_dir() {
                    let name = e.file_name().to_string_lossy();
                    return !name.starts_with('.');
                }
            }
            true
        })
    {
        // REQ-core-018（A96）: 走査でディレクトリが読めなければ停止する
        let entry = entry.map_err(|e| {
            let where_ = e.path()
                .map(|p| p.strip_prefix(base).unwrap_or(p))
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            crate::StopReason::UnreadableFile(format!("{where_}: {e}"))
        })?;
        // ファイルまたはファイルのシンボリックリンク
        let is_file = if entry.file_type().is_symlink() {
            // A146: 先の無いシンボリックリンクは読めないファイルとして停止する
            std::fs::metadata(entry.path())
                .map(|m| m.is_file())
                .map_err(|e| {
                    let rel = entry.path().strip_prefix(base).unwrap_or(entry.path())
                        .to_string_lossy().replace('\\', "/");
                    crate::StopReason::UnreadableFile(format!("{rel}: {e}"))
                })?
        } else {
            entry.file_type().is_file()
        };
        if is_file {
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
    Ok(files)
}

/// Rust ファイルのテストを、同梱の問い合わせと設定の "tests.rust" で発見する
pub fn discover_rust_tests(
    content: &str,
    file_rel: &str,
    config: &Config,
) -> Result<Vec<DiscoveredTest>, String> {
    let queries = TestQueries::new(config).map_err(|e| format!("{e:?}"))?;
    discover_tests(content, file_rel, SupportLang::Rust, &queries, config)
}

/// `問い合わせのある言語`のファイルのテストを発見し、`直前のコメントの塊`の印を結び付ける。
/// 構文の誤りが1つでもあれば Err（REQ-core-083）
pub fn discover_tests(
    content: &str,
    file_rel: &str,
    lang: SupportLang,
    queries: &TestQueries,
    config: &Config,
) -> Result<Vec<DiscoveredTest>, String> {
    let parsed =
        ParsedFile::parse(content, lang).ok_or_else(|| format!("syntax error in {file_rel}"))?;
    let root = parsed.root();
    let lines = LineMap::new(content, &root, lang);
    let mut tests = Vec::new();
    for test in parsed.find_tests(queries, lang, file_rel) {
        let first_line = test.node.start_pos().line();
        let (marker_ids, invalid_markers) = lines.markers_before(first_line);
        tests.push(DiscoveredTest {
            name: test.name,
            first_line_text: line_text(content, first_line),
            file_path: file_rel.to_string(),
            line: first_line + 1,
            marker_ids,
            invalid_markers,
        });
    }
    if lang == SupportLang::Rust {
        discover_macro_tests(&root, content, file_rel, config, &mut tests);
    }
    tests.sort_by_key(|t| t.line);
    Ok(tests)
}

/// 行の全体の文字から前後の空白を除いたもの（REQ-core-086 の名前が null のときの detail）
fn line_text(content: &str, line: usize) -> String {
    content.lines().nth(line).unwrap_or("").trim().to_string()
}

/// "tests.rust.macros" のマクロを探し、中身を Rust の項目として読み直す（TBL-core-017）
fn discover_macro_tests(
    node: &RustNode<'_>,
    source: &str,
    file_rel: &str,
    config: &Config,
    tests: &mut Vec<DiscoveredTest>,
) {
    for child in node.children() {
        match child.kind().as_ref() {
            "macro_invocation" => {
                if is_configured_macro(&child, config) {
                    reparse_macro_body(&child, source, file_rel, tests);
                }
            }
            _ => discover_macro_tests(&child, source, file_rel, config, tests),
        }
    }
}

type RustNode<'r> = Node<'r, StrDoc<SupportLang>>;

/// マクロの名前の末尾の要素が設定のどれかと一致するか
fn is_configured_macro(node: &RustNode<'_>, config: &Config) -> bool {
    let Some(macro_node) = node.field("macro") else {
        return false;
    };
    let macro_name = macro_node.text();
    let last_segment = macro_name.rsplit("::").next().unwrap_or(&macro_name);
    config.tests.rust.macros.iter().any(|m| {
        let m_last = m.rsplit("::").next().unwrap_or(m);
        m_last == last_segment
    })
}

fn reparse_macro_body(
    node: &RustNode<'_>,
    source: &str,
    file_rel: &str,
    tests: &mut Vec<DiscoveredTest>,
) {
    let Some(body) = node.children().find(|c| c.kind() == "token_tree") else {
        return;
    };
    let body_text = body.text();
    // token_tree の中身（{ ... } の中）
    let inner = body_text
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .unwrap_or(&body_text);
    let byte_offset = body.range().start + if body_text.starts_with('{') { 1 } else { 0 };
    let line_offset = source[..byte_offset].matches('\n').count();

    // REQ-core-083: 読み直したときの構文の誤りは unparsable_file にせず、読めた関数を数える
    let inner_root = SupportLang::Rust.ast_grep(inner);
    let inner_lines = LineMap::new(inner, &inner_root.root(), SupportLang::Rust);
    collect_macro_functions(
        &inner_root.root(),
        inner,
        &inner_lines,
        file_rel,
        line_offset,
        tests,
    );
}

/// マクロの中の最上位の関数ごとに数える（入れ子の関数は数えない）
fn collect_macro_functions(
    node: &RustNode<'_>,
    inner: &str,
    inner_lines: &LineMap<'_>,
    file_rel: &str,
    line_offset: usize,
    tests: &mut Vec<DiscoveredTest>,
) {
    for child in node.children() {
        if child.kind() != "function_item" {
            collect_macro_functions(&child, inner, inner_lines, file_rel, line_offset, tests);
            continue;
        }
        let Some(name_node) = child.field("name") else {
            continue;
        };
        let first_line = child.start_pos().line();
        // A26: マクロの外と同じ行の規則で結び付ける
        let (ids, invalid) = inner_lines.markers_before(first_line);
        tests.push(DiscoveredTest {
            // A38: 名前は関数の名前
            name: Some(name_node.text().to_string()),
            first_line_text: line_text(inner, first_line),
            file_path: file_rel.to_string(),
            line: line_offset + first_line + 1,
            marker_ids: ids
                .into_iter()
                .map(|(id, ln)| (id, line_offset + ln))
                .collect(),
            invalid_markers: invalid
                .into_iter()
                .map(|(ln, raw)| (line_offset + ln, raw))
                .collect(),
        });
    }
}

/// 単独の "\r" を "\n" に置き換える。TBL-core-010 は単独の "\r" も行の終わりに数えるが、
/// `str::lines()` も tree-sitter の行も "\n" でしか行を分けない。どちらも1バイトなので、
/// 置き換えてもバイトの位置は変わらない
fn lone_cr_to_lf(content: &str) -> String {
    let mut out = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' && chars.peek() != Some(&'\n') {
            out.push('\n');
        } else {
            out.push(c);
        }
    }
    out
}

/// テストの発見と印のチェックの全体
pub fn discover_and_check(
    base: &Path,
    config: &Config,
    docs: &[IrDocument],
    known_ids: &BTreeSet<String>,
    ir_path: &str,
    findings: &mut Vec<Finding>,
) -> Result<(BTreeMap<String, crate::TestFileTally>, Vec<TestMarker>), crate::StopReason> {
    let test_files = collect_test_files(base, config)?;
    let queries = TestQueries::load(base, config)?;
    let mut all_tests: Vec<DiscoveredTest> = Vec::new();
    // REQ-core-153: list の "tests" の元。check は使わない
    let mut markers: Vec<TestMarker> = Vec::new();
    // TBL-core-021: 読んだテストのファイルを拡張子ごとに数える
    let mut tally: BTreeMap<String, crate::TestFileTally> = BTreeMap::new();

    for (rel_path, abs_path) in &test_files {
        let content = lone_cr_to_lf(&crate::read_utf8_file(
            std::path::Path::new(abs_path),
            rel_path,
        )?);

        let ext = std::path::Path::new(rel_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        // REQ-core-081: 拡張子から言語を決め、その言語にルールがあれば問い合わせのある言語
        let lang = language_of(rel_path).filter(|lang| queries.has_query(*lang));
        let query = lang.is_some();

        tally
            .entry(ext.to_string())
            .or_insert(crate::TestFileTally { files: 0, query })
            .files += 1;

        if let Some(lang) = lang {
            match discover_tests(&content, rel_path, lang, &queries, config) {
                Ok(tests) => {
                    for test in &tests {
                        // 印の検証
                        check_test_markers(test, rel_path, known_ids, findings);
                        for (id, marker_line) in &test.marker_ids {
                            markers.push(TestMarker {
                                id: id.clone(),
                                path: rel_path.clone(),
                                line: *marker_line,
                                name: test.name.clone(),
                            });
                        }
                        // REQ-core-072: テストに結び付く空・不正な印
                        for (line_num, raw) in &test.invalid_markers {
                            findings.push(Finding::new(FindingKind::InvalidMarker, rel_path.clone(), Some(*line_num), raw.clone()));
                        }
                    }
                    all_tests.extend(tests);
                }
                Err(_) => {
                    findings.push(Finding::new(FindingKind::UnparsableFile, rel_path.clone(), None, rel_path.clone()));
                }
            }
        } else {
            // 問い合わせの無い言語: 印を拾い、検査もする（REQ-core-076, REQ-core-087, REQ-core-072, REQ-core-054）
            for (idx, line) in content.lines().enumerate() {
                let line_num = idx + 1;
                for marker in parse_markers_in_line(line, line_num) {
                    if marker.ids.is_empty() {
                        // REQ-core-072: 空の印、または閉じ括弧のない印
                        findings.push(Finding::new(FindingKind::InvalidMarker, rel_path.clone(), Some(line_num), line.to_string()));
                    } else {
                        for id in &marker.ids {
                            markers.push(TestMarker {
                                id: id.clone(),
                                path: rel_path.clone(),
                                line: line_num,
                                name: None,
                            });
                            // REQ-core-054: 存在しない ID への参照
                            if !known_ids.contains(id) {
                                findings.push(Finding::new(FindingKind::UnresolvedReference, rel_path.clone(), Some(line_num), id.clone()));
                            }
                        }
                    }
                }
            }
        }
    }

    let scenarios = collect_scenarios(docs, ir_path);
    let coverage = TestCoverage::new(&markers, &scenarios);

    // REQ-core-137: テストのない具体例
    for (id, scenario) in &scenarios {
        if scenario.needs_test && !coverage.is_marked(id) {
            findings.push(Finding::new(FindingKind::ScenarioWithoutTest, scenario.path.clone(), Some(scenario.line), id.clone()));
        }
    }

    // REQ-core-085: テストのない要求
    for doc in docs {
        for item in &doc.items {
            if let Item::Requirement {
                id, verification, ..
            } = item
            {
                if let Some(v) = verification {
                    if v != "review" && is_valid_id(id) && !coverage.has_test(id) {
                        let path = crate::join_display_path(ir_path, &doc.relative_path);
                        findings.push(Finding::new(FindingKind::RequirementWithoutTest, path, Some(item.item_line()), id.clone()));
                    }
                }
            }
        }
    }

    // REQ-core-086: 印の無いテスト
    for test in &all_tests {
        if test.marker_ids.is_empty() {
            // 名前が null なら節の最初の行の文字を detail にする
            let detail = test.name.clone().unwrap_or_else(|| test.first_line_text.clone());
            findings.push(Finding::new(FindingKind::TestWithoutId, test.file_path.clone(), Some(test.line), detail));
        }
    }

    Ok((tally, markers))
}

/// テストの印を検証する
fn check_test_markers(
    test: &DiscoveredTest,
    file_path: &str,
    known_ids: &BTreeSet<String>,
    findings: &mut Vec<Finding>,
) {
    // REQ-core-054, REQ-core-118: unresolved_reference の line は印のある行
    for (id, marker_line) in &test.marker_ids {
        if !id.is_empty() && !known_ids.contains(id) {
            findings.push(Finding::new(FindingKind::UnresolvedReference, file_path.to_string(), Some(*marker_line), id.clone()));
        }
    }
}


/// REQ-core-085: `ID` に結び付く`テスト`があるかの判定。
/// check の requirement_without_test と status の with_tests はこの同じ判定を使う
pub struct TestCoverage {
    /// `印`に現れた `ID`
    marked: BTreeSet<String>,
    /// `印`のある`シナリオ`が "@about" に挙げている `ID`
    covered_by_scenario: BTreeSet<String>,
}

impl TestCoverage {
    pub fn new(markers: &[TestMarker], scenarios: &BTreeMap<String, ScenarioCoverage>) -> Self {
        let marked: BTreeSet<String> = markers.iter().map(|m| m.id.clone()).collect();
        let covered_by_scenario = marked
            .iter()
            .filter_map(|id| scenarios.get(id))
            .flat_map(|scenario| scenario.about.iter().cloned())
            .collect();
        TestCoverage { marked, covered_by_scenario }
    }

    /// その `ID` を`印`に含む`テスト`があるか
    pub fn is_marked(&self, id: &str) -> bool {
        self.marked.contains(id)
    }

    /// その `ID` を`印`に含む`テスト`があるか、その `ID` を "@about" に持つ`シナリオ`の
    /// `ID` を`印`に含む`テスト`があるか
    pub fn has_test(&self, id: &str) -> bool {
        self.is_marked(id) || self.covered_by_scenario.contains(id)
    }
}

/// 具体例の ID から引く、その`シナリオ`の "@about" と場所（REQ-core-137、REQ-core-085）
#[derive(Debug, Clone)]
pub struct ScenarioCoverage {
    /// "@about" に挙がった ID
    pub about: Vec<String>,
    /// 指摘のパス（IR の置き場からの相対）
    pub path: String,
    /// TBL-core-019: タグの行（無ければ "Scenario:" の行）
    pub line: usize,
    /// REQ-core-137 の適用条件を満たすか（"@about" に、検証が "unit"・"property"・"proof" の要求がある）
    pub needs_test: bool,
}

/// 具体例の ID から "@about" と場所を引く表を作る。
/// 同じ ID の`シナリオ`が2か所以上にあるときは REQ-core-032 の1つ目（文書はパスのバイト順、
/// 同じ文書では行の小さい方）を使う。docs も items もその順に並んでいる。
pub fn collect_scenarios(docs: &[IrDocument], ir_path: &str) -> BTreeMap<String, ScenarioCoverage> {
    // 要求の ID から "- verification:" の値を引く（行が無ければ None）
    let mut verifications: BTreeMap<&str, Option<&str>> = BTreeMap::new();
    for doc in docs {
        for item in &doc.items {
            if let Item::Requirement { id, verification, .. } = item {
                verifications.entry(id).or_insert_with(|| verification.as_deref());
            }
        }
    }

    let mut scenarios: BTreeMap<String, ScenarioCoverage> = BTreeMap::new();
    for doc in docs {
        for item in &doc.items {
            if let Item::Scenario { id: Some(id), line, tag_line, about, .. } = item {
                // 要求として解決できない "@about"、"- verification:" の行の無い要求、
                // 検証の値が4つ以外の要求、検証が "review" の要求は数えない
                let needs_test = about.iter().any(|a| {
                    matches!(verifications.get(a.as_str()), Some(Some(v))
                        if crate::ir::VERIFICATION_VALUES.contains(v) && *v != "review")
                });
                scenarios.entry(id.clone()).or_insert_with(|| ScenarioCoverage {
                    about: about.clone(),
                    path: crate::join_display_path(ir_path, &doc.relative_path),
                    line: tag_line.unwrap_or(*line),
                    needs_test,
                });
            }
        }
    }
    scenarios
}
