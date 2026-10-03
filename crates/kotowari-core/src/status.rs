//! "kotowari status" の集計（REQ-core-162、REQ-core-164、REQ-core-165、TBL-core-028）

use crate::guides::GuideTally;
use crate::ir::IrDocument;
use crate::list::{self, ListItem};
use crate::surface::SurfaceTally;
use crate::tests_discovery::{TestCoverage, TestMarker, collect_scenarios};
use crate::{Finding, TestFileTally};
use serde::Serialize;
use std::collections::BTreeMap;

/// "kotowari status" の出力の最上位。TBL-core-028 の群を表の順に持つ（REQ-core-166）
#[derive(Debug, Clone, Serialize)]
pub struct StatusResult {
    pub documents: Documents,
    pub items: Items,
    pub requirements: Requirements,
    pub scenarios: Scenarios,
    pub tests: Tests,
    /// "kotowari check" の "guides" と同じ（TBL-core-005）
    pub guides: GuideTally,
    /// `面`の数。"surface.rules" が空の一覧なら3つとも 0（REQ-core-229）
    pub surface: SurfaceTally,
    pub findings: Findings,
    /// REQ-core-165: `誤り`が0件で、かつ`問題の記録`の`項目`が0件のときだけ true
    pub complete: bool,
}

/// 読んだ `IR` の文書（TBL-core-028。"kotowari check" の "files" と "lines" と同じ）
#[derive(Debug, Clone, Default, Serialize)]
pub struct Documents {
    pub(crate) files: usize,
    pub(crate) lines: usize,
}
impl Documents {
    readonly!(copy files: usize, lines: usize);
}

/// `ID` を持つ`項目`と`シナリオ`の種類ごとの数（TBL-core-028）
#[derive(Debug, Clone, Default, Serialize)]
pub struct Items {
    pub(crate) requirement: usize,
    pub(crate) table: usize,
    pub(crate) property: usize,
    pub(crate) scenario: usize,
    pub(crate) flag: usize,
}
impl Items {
    readonly!(copy requirement: usize, table: usize, property: usize, scenario: usize, flag: usize);
}

/// `要求`の数（TBL-core-028）
#[derive(Debug, Clone, Default, Serialize)]
pub struct Requirements {
    pub(crate) unit: usize,
    pub(crate) property: usize,
    pub(crate) proof: usize,
    pub(crate) review: usize,
    pub(crate) with_tests: usize,
    pub(crate) without_tests: usize,
    pub(crate) review_with_how_to_verify: usize,
    pub(crate) review_without_how_to_verify: usize,
    pub(crate) without_examples: usize,
    /// `後回し`の`要求`の数。`項目`の出現ごとに数える
    pub(crate) deferred: usize,
}
impl Requirements {
    readonly!(copy unit: usize, property: usize, proof: usize, review: usize, with_tests: usize, without_tests: usize, review_with_how_to_verify: usize, review_without_how_to_verify: usize, without_examples: usize, deferred: usize);
}

/// `シナリオ`の数（TBL-core-028）
#[derive(Debug, Clone, Default, Serialize)]
pub struct Scenarios {
    pub(crate) with_tests: usize,
    pub(crate) without_tests: usize,
    /// `後回しのシナリオ`の数
    pub(crate) deferred: usize,
}
impl Scenarios {
    readonly!(copy with_tests: usize, without_tests: usize, deferred: usize);
}

/// `印`と読んだ`テストのファイル`（TBL-core-028）
#[derive(Debug, Clone, Serialize)]
pub struct Tests {
    /// `印`の出現の数。1つの`印`に `ID` が複数あれば `ID` ごとに1つ
    pub(crate) marks: usize,
    /// TBL-core-021: "kotowari check" の "tests" と同じ
    pub(crate) files: BTreeMap<String, TestFileTally>,
}
impl Tests {
    readonly!(copy marks: usize);
    readonly!(borrow files: BTreeMap<String, TestFileTally>);
}

/// "kotowari check" の`指摘`の数（TBL-core-028）
#[derive(Debug, Clone, Default, Serialize)]
pub struct Findings {
    pub(crate) error: usize,
    pub(crate) notice: usize,
}
impl Findings {
    readonly!(copy error: usize, notice: usize);
}

/// check と同じ読み取りと検査の結果から TBL-core-028 の集計を作る（REQ-core-162、REQ-core-164）
pub fn build(
    docs: &[IrDocument],
    ir_path: &str,
    markers: &[TestMarker],
    tally: BTreeMap<String, TestFileTally>,
    guides: GuideTally,
    surface: SurfaceTally,
    findings: &[Finding],
) -> StatusResult {
    // 数える母集団は "kotowari list" の "items" と同じ（`ID` を持つ項目とシナリオ）
    let listed = list::build(docs, ir_path, markers);
    let coverage = TestCoverage::new(markers, &collect_scenarios(docs, ir_path));

    let mut items = Items::default();
    let mut requirements = Requirements::default();
    let mut scenarios = Scenarios::default();

    for item in &listed.items {
        match item {
            ListItem::Requirement(requirement) => {
                items.requirement += 1;
                count_requirement(requirement, &coverage, &mut requirements);
            }
            ListItem::WithExamples(example) if example.kind == "table" => items.table += 1,
            ListItem::WithExamples(_) => items.property += 1,
            ListItem::Scenario(scenario) => {
                items.scenario += 1;
                if scenario.deferred {
                    scenarios.deferred += 1;
                } else if coverage.is_marked(&scenario.id) {
                    scenarios.with_tests += 1;
                } else {
                    scenarios.without_tests += 1;
                }
            }
            ListItem::Flag(_) => items.flag += 1,
        }
    }

    let mut counts = Findings::default();
    for finding in findings {
        if finding.severity == "error" {
            counts.error += 1;
        } else {
            counts.notice += 1;
        }
    }

    StatusResult {
        documents: Documents {
            files: docs.len(),
            lines: docs.iter().map(|doc| doc.line_count).sum(),
        },
        // REQ-core-165: 誤りが0件で、かつ問題の記録の項目が0件のときだけ complete
        complete: counts.error == 0 && items.flag == 0,
        items,
        requirements,
        scenarios,
        tests: Tests {
            marks: markers.len(),
            files: tally,
        },
        guides,
        surface,
        findings: counts,
    }
}

/// TBL-core-028: 1つの`要求`を検証の値、テストの有無、確かめ方の有無、具体例の有無で数える
fn count_requirement(
    requirement: &list::RequirementItem,
    coverage: &TestCoverage,
    counts: &mut Requirements,
) {
    // "- verification:" の行の無い要求はどれにも数えない
    match requirement.verification.as_deref() {
        Some("unit") => counts.unit += 1,
        Some("property") => counts.property += 1,
        Some("proof") => counts.proof += 1,
        Some("review") => counts.review += 1,
        _ => {}
    }

    if requirement.verification.as_deref() == Some("review") {
        // REQ-core-098: 値が空の行は無い行として扱う（解析の時点で落ちている）
        if requirement.how_to_verify.is_none() {
            counts.review_without_how_to_verify += 1;
        } else {
            counts.review_with_how_to_verify += 1;
        }
    } else if !requirement.deferred {
        // TBL-core-028: with_tests と without_tests は`後回し`でない要求だけを数える
        // REQ-core-085: 検証が review でない要求だけを分母にする
        if coverage.has_test(&requirement.id) {
            counts.with_tests += 1;
        } else {
            counts.without_tests += 1;
        }
    }

    if requirement.examples.is_empty() {
        counts.without_examples += 1;
    }
    if requirement.deferred {
        counts.deferred += 1;
    }
}
