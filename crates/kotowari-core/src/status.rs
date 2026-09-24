//! "kotowari status" の集計（REQ-core-162、REQ-core-164、REQ-core-165、TBL-core-028）

use crate::guides::GuideTally;
use crate::ir::IrDocument;
use crate::list::{self, ListItem};
use crate::tests_discovery::{TestCoverage, TestMarker, collect_scenarios};
use crate::{Finding, TestFileTally};
use serde::Serialize;
use std::collections::BTreeMap;

/// "kotowari status" の出力の最上位。TBL-core-028 の群を表の順に持つ（REQ-core-166）
#[derive(Debug, Serialize)]
pub struct StatusResult {
    pub documents: Documents,
    pub items: Items,
    pub requirements: Requirements,
    pub scenarios: Scenarios,
    pub tests: Tests,
    /// "kotowari check" の "guides" と同じ（TBL-core-005）
    pub guides: GuideTally,
    pub findings: Findings,
    /// REQ-core-165: `誤り`が0件で、かつ`問題の記録`の`項目`が0件のときだけ true
    pub complete: bool,
}

/// 読んだ `IR` の文書（TBL-core-028。"kotowari check" の "files" と "lines" と同じ）
#[derive(Debug, Default, Serialize)]
pub struct Documents {
    pub files: usize,
    pub lines: usize,
}

/// `ID` を持つ`項目`と`シナリオ`の種類ごとの数（TBL-core-028）
#[derive(Debug, Default, Serialize)]
pub struct Items {
    pub requirement: usize,
    pub table: usize,
    pub property: usize,
    pub scenario: usize,
    pub flag: usize,
}

/// `要求`の数（TBL-core-028）
#[derive(Debug, Default, Serialize)]
pub struct Requirements {
    pub unit: usize,
    pub property: usize,
    pub proof: usize,
    pub review: usize,
    pub with_tests: usize,
    pub without_tests: usize,
    pub review_with_how_to_verify: usize,
    pub review_without_how_to_verify: usize,
    pub without_examples: usize,
}

/// `シナリオ`の数（TBL-core-028）
#[derive(Debug, Default, Serialize)]
pub struct Scenarios {
    pub with_tests: usize,
    pub without_tests: usize,
}

/// `印`と読んだ`テストのファイル`（TBL-core-028）
#[derive(Debug, Serialize)]
pub struct Tests {
    /// `印`の出現の数。1つの`印`に `ID` が複数あれば `ID` ごとに1つ
    pub marks: usize,
    /// TBL-core-021: "kotowari check" の "tests" と同じ
    pub files: BTreeMap<String, TestFileTally>,
}

/// "kotowari check" の`指摘`の数（TBL-core-028）
#[derive(Debug, Default, Serialize)]
pub struct Findings {
    pub error: usize,
    pub notice: usize,
}

/// REQ-core-166: TBL-core-028 の群ごとに "群名 鍵=値 鍵=値" の1行を、表の順に出す。
/// 鍵の語は JSON と同じで、値の間は半角空白1つ、桁揃えの空白は入れない
pub fn print_text(result: &StatusResult) {
    let documents = &result.documents;
    println!(
        "documents files={} lines={}",
        documents.files, documents.lines
    );
    let items = &result.items;
    println!(
        "items requirement={} table={} property={} scenario={} flag={}",
        items.requirement, items.table, items.property, items.scenario, items.flag
    );
    let requirements = &result.requirements;
    println!(
        "requirements unit={} property={} proof={} review={} with_tests={} without_tests={} \
review_with_how_to_verify={} review_without_how_to_verify={} without_examples={}",
        requirements.unit,
        requirements.property,
        requirements.proof,
        requirements.review,
        requirements.with_tests,
        requirements.without_tests,
        requirements.review_with_how_to_verify,
        requirements.review_without_how_to_verify,
        requirements.without_examples
    );
    let scenarios = &result.scenarios;
    println!(
        "scenarios with_tests={} without_tests={}",
        scenarios.with_tests, scenarios.without_tests
    );
    // TBL-core-028: "text" では読んだテストのファイルを拡張子ごとに数える
    let mut tests = format!("tests marks={}", result.tests.marks);
    for (extension, tally) in &result.tests.files {
        tests.push_str(&format!(" {extension}={}", tally.files));
    }
    println!("{tests}");
    println!(
        "guides files={} marks={}",
        result.guides.files, result.guides.marks
    );
    let findings = &result.findings;
    println!(
        "findings error={} notice={}",
        findings.error, findings.notice
    );
    println!("complete {}", result.complete);
}

/// check と同じ読み取りと検査の結果から TBL-core-028 の集計を作る（REQ-core-162、REQ-core-164）
pub fn build(
    docs: &[IrDocument],
    ir_path: &str,
    markers: &[TestMarker],
    tally: BTreeMap<String, TestFileTally>,
    guides: GuideTally,
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
                if coverage.is_marked(&scenario.id) {
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
    } else if coverage.has_test(&requirement.id) {
        // REQ-core-085: 検証が review でない要求だけを分母にする
        counts.with_tests += 1;
    } else {
        counts.without_tests += 1;
    }

    if requirement.examples.is_empty() {
        counts.without_examples += 1;
    }
}
