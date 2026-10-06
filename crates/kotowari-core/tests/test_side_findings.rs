#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
//! テスト側の指摘の範囲（docs/ir/core/test-side-findings.md）

use kotowari_core::{
    CheckInputs, Finding, FindingGroup, FindingKind, Inspection, SourceText, SurfaceAnalysis,
    TestAnalysis,
};

const TEST_ONLY: &str = "tests/a.rs";
const SURFACE_ONLY: &str = "src/lib.rs";
const TEST_AND_SURFACE: &str = "src/both.rs";

fn test_file(source: &SourceText) -> TestAnalysis {
    TestAnalysis {
        source: source.clone(),
        language: Some("rust".into()),
        has_query: true,
        tests: vec![],
        line_markers: vec![],
        findings: vec![],
    }
}

fn surface_file(source: &SourceText) -> SurfaceAnalysis {
    SurfaceAnalysis {
        source: source.clone(),
        language: Some("rust".into()),
        has_query: true,
        surfaces: vec![],
        findings: vec![],
    }
}

/// テストのファイルだけ、面のファイルだけ、両方のファイルを1つずつ読み、findings を加えた検査
fn inspect_with(findings: Vec<Finding>) -> Inspection {
    let test_only = SourceText::new(TEST_ONLY, "fn a() {}").unwrap();
    let surface_only = SourceText::new(SURFACE_ONLY, "fn b() {}").unwrap();
    let both = SourceText::new(TEST_AND_SURFACE, "fn c() {}").unwrap();
    let mut inputs = CheckInputs::default();
    inputs.read.ir = Some(vec![]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.tests = Some(vec![test_file(&test_only), test_file(&both)]);
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.surface = Some(vec![surface_file(&surface_only), surface_file(&both)]);
    inputs.groups = vec![FindingGroup::new("given", 0, 0, findings)];
    Inspection::build(inputs).unwrap()
}

// @kotowari[TBL-core-047]
#[test]
fn tbl_core_047_each_row_decides_whether_an_error_is_a_test_side_finding() {
    let cases = [
        (
            FindingKind::RequirementWithoutTest,
            "docs/ir/topic.md",
            true,
        ),
        (FindingKind::ScenarioWithoutTest, "docs/ir/topic.md", true),
        (FindingKind::TestWithoutId, "docs/ir/topic.md", true),
        (FindingKind::InvalidMarker, TEST_ONLY, true),
        (FindingKind::InvalidMarker, "docs/guides/a.md", false),
        (FindingKind::UnresolvedReference, TEST_ONLY, true),
        (FindingKind::UnresolvedReference, "docs/guides/a.md", false),
        (FindingKind::UnparsableFile, TEST_ONLY, true),
        (FindingKind::UnparsableFile, TEST_AND_SURFACE, false),
        (FindingKind::UnparsableFile, SURFACE_ONLY, false),
        // 表に無い種類は、テストのファイルから出てもテスト側の指摘でない
        (FindingKind::UnknownTerm, TEST_ONLY, false),
    ];
    for (kind, path, expected) in cases {
        let inspection = inspect_with(vec![Finding::new(
            kind,
            path.into(),
            Some(1),
            "detail".into(),
        )]);
        let check = inspection.check();
        let finding = check
            .findings()
            .iter()
            .find(|finding| finding.kind() == kind && finding.path() == path)
            .unwrap();
        assert_eq!(
            check.is_test_side_finding(finding),
            expected,
            "{kind} on {path}"
        );
    }
}
