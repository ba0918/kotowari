//! テストの発見と印の結び付け（REQ-core-071〜REQ-core-088）

use crate::deferred;
use crate::ir::{IrDocument, Item, is_valid_id};
pub use crate::test_markers::{InvalidMarkers, Marker, MarkerIds, parse_markers_in_line};
use crate::{Finding, FindingKind};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

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
    /// テストに結び付く印
    pub marker_ids: MarkerIds,
    /// テストに結び付く位置にある、中身が空または閉じ括弧のない印
    pub invalid_markers: InvalidMarkers,
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

/// テストの発見の結果
pub struct DiscoveredTests {
    /// TBL-core-021: 読んだテストのファイルの拡張子ごとの数
    pub tally: BTreeMap<String, crate::TestFileTally>,
    /// TBL-core-026: 印の出現ごとの (ID, テストのファイル, 行, テストの名前)
    pub markers: Vec<TestMarker>,
    /// 読んだ`テストのファイル`の`基準のディレクトリ`からの相対パス。バイト順（REQ-core-199 の重なりの判定）
    pub files: Vec<String>,
}

pub fn check_entries<'a>(
    analysis: impl IntoIterator<Item = (&'a str, &'a crate::TestAnalysis)>,
    docs: &[IrDocument],
    known_ids: &BTreeSet<String>,
    ir_path: &str,
    findings: &mut Vec<Finding>,
) -> DiscoveredTests {
    let mut all_tests = Vec::new();
    let mut markers = Vec::new();
    let mut tally = BTreeMap::new();
    let mut files = Vec::new();
    for (path, file) in analysis {
        let extension = Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        tally
            .entry(extension.to_owned())
            .or_insert(crate::TestFileTally {
                files: 0,
                query: file.has_query,
            })
            .files += 1;
        files.push(path.to_owned());
        findings.extend(file.findings.iter().cloned());
        if file.has_query {
            record_test_markers(&file.tests, path, known_ids, &mut markers, findings);
            all_tests.extend(file.tests.iter().cloned());
        } else {
            let lines = crate::ir::split_lines(file.source.text());
            for marker in &file.line_markers {
                if marker.ids.is_empty() {
                    findings.push(Finding::new(
                        FindingKind::InvalidMarker,
                        path.to_owned(),
                        Some(marker.line),
                        lines
                            .get(marker.line.saturating_sub(1))
                            .copied()
                            .unwrap_or("")
                            .to_owned(),
                    ));
                } else {
                    for id in &marker.ids {
                        markers.push(TestMarker {
                            id: id.clone(),
                            path: path.to_owned(),
                            line: marker.line,
                            name: None,
                        });
                        if !known_ids.contains(id) {
                            findings.push(Finding::new(
                                FindingKind::UnresolvedReference,
                                path.to_owned(),
                                Some(marker.line),
                                id.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }
    files.sort();
    check_missing_tests(docs, ir_path, &markers, &all_tests, findings);
    DiscoveredTests {
        tally,
        markers,
        files,
    }
}

/// `問い合わせのある言語`のファイルで見つけた`テスト`の印を積み、印を検査する
fn record_test_markers(
    tests: &[DiscoveredTest],
    rel_path: &str,
    known_ids: &BTreeSet<String>,
    markers: &mut Vec<TestMarker>,
    findings: &mut Vec<Finding>,
) {
    for test in tests {
        // 印の検証
        check_test_markers(test, rel_path, known_ids, findings);
        for (id, marker_line) in &test.marker_ids {
            markers.push(TestMarker {
                id: id.clone(),
                path: rel_path.to_string(),
                line: *marker_line,
                name: test.name.clone(),
            });
        }
        // REQ-core-072: テストに結び付く空・不正な印
        for (line_num, raw) in &test.invalid_markers {
            findings.push(Finding::new(
                FindingKind::InvalidMarker,
                rel_path.to_string(),
                Some(*line_num),
                raw.clone(),
            ));
        }
    }
}

/// 集めた印から、テストのない具体例・テストのない要求・印の無いテストを検査する
fn check_missing_tests(
    docs: &[IrDocument],
    ir_path: &str,
    markers: &[TestMarker],
    all_tests: &[DiscoveredTest],
    findings: &mut Vec<Finding>,
) {
    let scenarios = collect_scenarios(docs, ir_path);
    let coverage = TestCoverage::new(markers, &scenarios);
    let requirements = deferred::Requirements::new(docs);

    // REQ-core-137: テストのない具体例
    for (id, scenario) in &scenarios {
        if scenario.needs_test && !coverage.is_marked(id) {
            findings.push(Finding::new(
                FindingKind::ScenarioWithoutTest,
                scenario.path.clone(),
                Some(scenario.line),
                id.clone(),
            ));
        }
    }

    // REQ-core-085: テストのない要求。`後回し`の要求には出さない
    for doc in docs {
        for item in &doc.items {
            if let Item::Requirement {
                id,
                verification: Some(v),
                ..
            } = item
                && v != "review"
                && is_valid_id(id)
                && !requirements.is_deferred(id)
                && !coverage.has_test(id)
            {
                let path = crate::join_display_path(ir_path, &doc.relative_path);
                findings.push(Finding::new(
                    FindingKind::RequirementWithoutTest,
                    path,
                    Some(item.item_line()),
                    id.clone(),
                ));
            }
        }
    }

    // REQ-core-086: 印の無いテスト
    for test in all_tests {
        if test.marker_ids.is_empty() {
            // 名前が null なら節の最初の行の文字を detail にする
            let detail = test
                .name
                .clone()
                .unwrap_or_else(|| test.first_line_text.clone());
            findings.push(Finding::new(
                FindingKind::TestWithoutId,
                test.file_path.clone(),
                Some(test.line),
                detail,
            ));
        }
    }
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
            findings.push(Finding::new(
                FindingKind::UnresolvedReference,
                file_path.to_string(),
                Some(*marker_line),
                id.clone(),
            ));
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
        TestCoverage {
            marked,
            covered_by_scenario,
        }
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
    /// REQ-core-137 の適用条件を満たすか（"@about" に、検証が "unit"・"property"・"proof" で
    /// `後回し`でない要求がある）
    pub needs_test: bool,
    /// `後回しのシナリオ`か
    pub deferred: bool,
}

/// 具体例の ID から "@about" と場所を引く表を作る。
/// 同じ ID の`シナリオ`が2か所以上にあるときは REQ-core-032 の1つ目（文書はパスのバイト順、
/// 同じ文書では行の小さい方）を使う。docs も items もその順に並んでいる。
pub fn collect_scenarios(docs: &[IrDocument], ir_path: &str) -> BTreeMap<String, ScenarioCoverage> {
    let requirements = deferred::Requirements::new(docs);

    let mut scenarios: BTreeMap<String, ScenarioCoverage> = BTreeMap::new();
    for doc in docs {
        for item in &doc.items {
            if let Item::Scenario {
                id: Some(id),
                line,
                tag_line,
                about,
                ..
            } = item
            {
                scenarios
                    .entry(id.clone())
                    .or_insert_with(|| ScenarioCoverage {
                        about: about.clone(),
                        path: crate::join_display_path(ir_path, &doc.relative_path),
                        line: tag_line.unwrap_or(*line),
                        needs_test: requirements.needs_test(about),
                        deferred: requirements.is_deferred_scenario(about),
                    });
            }
        }
    }
    scenarios
}
