//! テストの発見と印の結び付け（REQ-core-071〜REQ-core-088）

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
    known_ids: &BTreeSet<String>,
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
            let lines = crate::split_lines(file.source.text());
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
    check_unmarked_tests(&all_tests, findings);
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

/// 印の無いテストに test_without_id を出す
fn check_unmarked_tests(all_tests: &[DiscoveredTest], findings: &mut Vec<Finding>) {
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
