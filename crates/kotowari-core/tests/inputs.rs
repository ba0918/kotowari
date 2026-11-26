#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari_core::SourceText;

#[path = "admission.rs"]
mod admission;
#[path = "test_side_findings.rs"]
mod test_side_findings;

// @kotowari[C-core-libinputs-1, C-core-libinputs-2, C-core-libinputs-3]
#[test]
fn required_groups_are_distinct_from_explicit_empty_groups() {
    use kotowari_core::{InputError, ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InputMissing(_))
    ));
    inputs.decisions = Some(vec![]);
    assert!(ReadModel::build(inputs).is_ok());
}

// @kotowari[C-core-libinputs-11]
#[test]
fn read_models_list_and_query_without_check_only_inputs() {
    use kotowari_core::{ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    inputs.config.guides.files = vec!["guides/**/*.md".into()];
    inputs.decisions = Some(vec![SourceText::new("docs/decisions/memory.md", "# Topic\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nBody.\n\n### Confirmations\n\n- C-m-1 Checked\n").unwrap()]);
    let model = ReadModel::build(inputs).unwrap();
    assert!(!model.list().items().is_empty());
    assert!(model.query("C-m-1").is_ok());
    assert!(model.query("C-m-999").is_err());
}

// @kotowari[C-core-libinputs-1, C-core-libinputs-2]
#[test]
fn enabled_check_groups_must_be_provided_even_when_empty() {
    use kotowari_core::{CheckInputs, InputError, Inspection};
    let mut inputs = CheckInputs::default();
    inputs.read.config.tests.files.clear();
    inputs.read.decisions = Some(vec![]);
    inputs.read.config.guides.files = vec!["guides/**".into()];
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.read.config.surface.unspecified = Some("unspecified.yaml".into());
    for group in 0..3 {
        assert!(matches!(
            Inspection::build(inputs.clone()),
            Err(InputError::InputMissing(_))
        ));
        match group {
            0 => inputs.guides = Some(vec![]),
            1 => inputs.surface = Some(vec![]),
            _ => inputs.unspecified = Some(vec![]),
        }
    }
    assert!(Inspection::build(inputs).is_ok());
}

// @kotowari[C-core-libcrates-5, C-core-libinputs-4, C-core-libinputs-6]
#[test]
fn supplied_analysis_preserves_diagnostics_counts_and_duplicate_suppression() {
    use kotowari_core::{
        CheckInputs, Finding, FindingKind, Inspection, SurfaceAnalysis, TestAnalysis,
    };
    let source = SourceText::new("src/broken.rs", "fn broken( {").unwrap();
    let diagnostic = Finding::new(
        FindingKind::UnparsableFile,
        source.path().into(),
        None,
        source.path().into(),
    );
    let mut inputs = CheckInputs::default();
    inputs.read.decisions = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        line_markers: vec![],
        source: source.clone(),
        has_query: true,
        tests: vec![],
        findings: vec![diagnostic.clone()],
    }]);
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.surface = Some(vec![SurfaceAnalysis {
        source,
        has_query: true,
        surfaces: vec![],
        findings: vec![diagnostic],
    }]);
    let inspection = Inspection::build(inputs).unwrap();
    let result = inspection.check();
    assert_eq!(
        result
            .findings()
            .iter()
            .filter(|finding| finding.kind() == FindingKind::UnparsableFile)
            .count(),
        1
    );
    assert_eq!(result.tests()["rs"].files(), 1);
    assert!(result.tests()["rs"].query());
    assert_eq!(inspection.status().tests().files()["rs"].files(), 1);
    assert!(!inspection.status().complete());
}

// @kotowari[C-core-libinputs-3]
#[test]
fn disabled_groups_are_ignored_including_their_inconsistent_sources() {
    use kotowari_core::{CheckInputs, Inspection, TestAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.config.tests.files.clear();
    inputs.read.decisions = Some(vec![
        SourceText::new("docs/decisions/memory.md", "# Topic\n").unwrap(),
    ]);
    inputs.read.tests = Some(vec![TestAnalysis {
        line_markers: vec![],
        source: SourceText::new("docs/decisions/memory.md", "different").unwrap(),
        has_query: false,
        tests: vec![],
        findings: vec![],
    }]);
    inputs.guides = Some(vec![
        SourceText::new("docs/decisions/memory.md", "different").unwrap(),
    ]);
    let inspection = Inspection::build(inputs).unwrap();
    assert!(inspection.check().tests().is_empty());
}

// @kotowari[C-core-libinputs-5]
#[test]
fn enabled_guide_and_surface_inputs_are_evaluated_without_acquisition() {
    use kotowari_core::{CheckInputs, FindingKind, Inspection, SurfaceAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.config.tests.files.clear();
    inputs.read.decisions = Some(vec![]);
    inputs.read.config.guides.files = vec!["guides/**".into()];
    inputs.guides = Some(vec![
        SourceText::new(
            "guides/memory.md",
            "<!-- @kotowari[docs/decisions/memory.md#name:12345678] -->\n",
        )
        .unwrap(),
    ]);
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.surface = Some(vec![SurfaceAnalysis {
        source: SourceText::new("src/memory.rs", "anything").unwrap(),
        has_query: true,
        surfaces: vec![kotowari_core::surface::Surface {
            kind: "command".into(),
            name: "memory".into(),
            path: "src/memory.rs".into(),
            line: 1,
        }],
        findings: vec![],
    }]);
    let inspection = Inspection::build(inputs).unwrap();
    assert!(
        inspection
            .check()
            .findings()
            .iter()
            .any(|finding| finding.kind() == FindingKind::GuideStale)
    );
    assert!(
        inspection
            .check()
            .findings()
            .iter()
            .any(|finding| finding.kind() == FindingKind::SurfaceWithoutSpec)
    );
}

// @kotowari[C-core-libcrates-5, C-core-libinputs-5]
#[test]
fn non_query_marker_facts_are_consumed_without_source_discovery() {
    use kotowari_core::{CheckInputs, FindingKind, Inspection, TestAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.decisions = Some(vec![SourceText::new("docs/decisions/memory.md", "# Topic\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nBody.\n\n### Confirmations\n\n- C-m-1 Checked\n").unwrap()]);
    inputs.read.tests = Some(vec![TestAnalysis {
        source: SourceText::new("checks/memory.txt", "@kotowari[C-m-1]\n").unwrap(),
        has_query: false,
        tests: vec![],
        findings: vec![],
        line_markers: vec![kotowari_core::tests_discovery::Marker {
            ids: vec!["C-m-1".into()],
            line: 1,
        }],
    }]);
    let inspection = Inspection::build(inputs).unwrap();
    assert!(
        !inspection
            .check()
            .findings()
            .iter()
            .any(|finding| finding.kind() == FindingKind::ConfirmationWithoutTest)
    );
    assert_eq!(inspection.status().tests().marks(), 1);
}

// @kotowari[C-core-libinputs-1, C-core-libinputs-2, C-core-libinputs-13]
#[test]
fn each_enabled_group_is_required_independently_and_empty_is_provided() {
    use kotowari_core::{CheckInputs, InputError, Inspection};
    let mut all = CheckInputs::default();
    all.read.decisions = Some(vec![]);
    all.read.tests = Some(vec![]);
    all.read.config.guides.files = vec!["guides/**".into()];
    all.read.config.surface.files = vec!["src/**".into()];
    all.read.config.surface.rules = vec!["rules.yaml".into()];
    all.read.config.surface.unspecified = Some("unspecified.yaml".into());
    all.guides = Some(vec![]);
    all.surface = Some(vec![]);
    all.unspecified = Some(vec![]);
    assert!(Inspection::build(all.clone()).is_ok());
    for index in 0..5 {
        let mut missing = all.clone();
        match index {
            0 => missing.read.decisions = None,
            1 => missing.read.tests = None,
            2 => missing.guides = None,
            3 => missing.surface = None,
            _ => missing.unspecified = None,
        }
        assert!(matches!(
            Inspection::build(missing),
            Err(InputError::InputMissing(_))
        ));
    }
}

// @kotowari[C-core-libinputs-6, C-core-libpaths-7]
#[test]
fn same_text_shared_between_guides_and_tests_uses_the_overlap_rule() {
    use kotowari_core::{CheckInputs, InputError, Inspection, TestAnalysis};
    let source = SourceText::new("shared.txt", "text").unwrap();
    let mut inputs = CheckInputs::default();
    inputs.read.decisions = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        source: source.clone(),
        has_query: false,
        tests: vec![],
        line_markers: vec![],
        findings: vec![],
    }]);
    inputs.read.config.guides.files = vec!["shared.txt".into()];
    inputs.guides = Some(vec![source]);
    match Inspection::build(inputs) {
        Err(InputError::ConfigError(detail)) => {
            assert!(detail.contains("shared.txt: matched by both"))
        }
        _ => panic!("expected the existing guide/test overlap error"),
    }
}

// @kotowari[C-core-libapi-5]
#[test]
fn invalid_typed_configuration_is_an_execution_failure_not_a_completed_inspection() {
    use kotowari_core::{CheckInputs, InputError, Inspection};
    let mut inputs = CheckInputs::default();
    inputs.read.decisions = Some(vec![]);
    inputs.read.config.tests.files = vec!["[".into()];
    inputs.read.tests = Some(vec![]);
    assert!(matches!(
        Inspection::build(inputs),
        Err(InputError::ConfigError(_))
    ));
}

fn empty_check_inputs() -> kotowari_core::CheckInputs {
    let mut inputs = kotowari_core::CheckInputs::default();
    inputs.read.config.tests.files.clear();
    inputs.read.decisions = Some(vec![SourceText::new(
        "docs/decisions/topic.md",
        "# Topic\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nBody.\n\n### Confirmations\n\n- C-t-1 (by reading) Read it\n",
    )
    .unwrap()]);
    inputs
}

fn group(name: &str, files: usize, marks: usize) -> kotowari_core::FindingGroup {
    use kotowari_core::{Finding, FindingKind};
    kotowari_core::FindingGroup::new(
        name,
        files,
        marks,
        vec![
            Finding::new(
                FindingKind::OverviewLeadMissing,
                ".kotowari/overview/b.md".into(),
                None,
                "b.md".into(),
            ),
            Finding::new(
                FindingKind::GuideStale,
                ".kotowari/overview/a.md".into(),
                Some(9),
                "docs/decisions/topic.md#name 00000000 11111111".into(),
            ),
            Finding::new(
                FindingKind::OverviewPartUnknown,
                ".kotowari/overview/a.md".into(),
                Some(3),
                "chart".into(),
            ),
        ],
    )
}

// @kotowari[C-core-libinputs-7]
#[test]
fn an_additional_group_is_sorted_counted_and_reported_with_its_numbers() {
    use kotowari_core::Inspection;
    let mut inputs = empty_check_inputs();
    inputs.groups = vec![group("overview", 2, 5)];
    let inspection = Inspection::build(inputs).unwrap();
    let check = inspection.check();
    let found: Vec<(&str, Option<usize>, &str)> = check
        .findings()
        .iter()
        .map(|finding| (finding.path(), finding.line(), finding.kind().as_str()))
        .collect();
    assert_eq!(
        found,
        [
            (".kotowari/overview/a.md", Some(3), "overview_part_unknown"),
            (".kotowari/overview/a.md", Some(9), "guide_stale"),
            (".kotowari/overview/b.md", None, "overview_lead_missing"),
        ]
    );
    assert_eq!(check.counts().get("overview_lead_missing"), Some(&1));
    assert_eq!(check.counts().get("guide_stale"), Some(&1));
    let names: Vec<&str> = check.groups().iter().map(|group| group.name()).collect();
    assert_eq!(names, ["overview"]);
    let tally = check.group("overview").expect("group tally");
    assert_eq!(
        (tally.name(), tally.files(), tally.marks()),
        ("overview", 2, 5)
    );
    let status = inspection.status();
    assert_eq!(status.findings().error(), 2);
    assert_eq!(status.findings().notice(), 1);
    assert!(!status.complete());
    let names: Vec<&str> = status.groups().iter().map(|group| group.name()).collect();
    assert_eq!(names, ["overview"]);
    let tally = status.group("overview").expect("status group tally");
    assert_eq!((tally.files(), tally.marks()), (2, 5));
}

// @kotowari[C-core-libinputs-8]
#[test]
fn an_additional_group_is_optional_and_only_added_when_given() {
    use kotowari_core::Inspection;
    let inspection = Inspection::build(empty_check_inputs()).unwrap();
    assert!(inspection.check().findings().is_empty());
    assert!(inspection.check().group("overview").is_none());
    assert!(inspection.check().groups().is_empty());
    assert!(inspection.status().complete());
}

// @kotowari[C-core-libinputs-7]
#[test]
fn two_additional_groups_with_one_name_are_invalid_input() {
    use kotowari_core::{InputError, Inspection};
    let mut inputs = empty_check_inputs();
    inputs.groups = vec![group("overview", 0, 0), group("overview", 1, 0)];
    assert!(matches!(
        Inspection::build(inputs),
        Err(InputError::InvalidInput(_))
    ));
}

// @kotowari[C-core-ovdata-14]
#[test]
fn guide_marks_of_one_text_are_read_with_lines_and_staleness_without_the_overlap_stop() {
    use kotowari_core::{FindingKind, ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files = vec!["notes/**".into()];
    inputs.config.guides.files = vec!["notes/**".into()];
    inputs.decisions = Some(vec![SourceText::new(
        "docs/decisions/topic.md",
        "# Topic\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nBody.\n",
    )
    .unwrap()]);
    inputs.tests = Some(vec![]);
    let model = ReadModel::build(inputs).unwrap();
    let report = model.query("docs/decisions/topic.md#name").unwrap();
    let kotowari_core::ListItem::Decision(decision) = report.items()[0].item() else {
        panic!("the target is a decision");
    };
    let current = decision.fingerprint().to_string();
    let text = format!(
        "# Guide\n\n## Fresh\n<!-- @kotowari[docs/decisions/topic.md#name:{current}] -->\n\n## Old\n<!-- @kotowari[docs/decisions/topic.md#name:00000000] -->\n\n## Broken\n<!-- @kotowari[docs/decisions/topic.md#name] -->\n"
    );
    let reader = model.guide_reader();
    let marks = reader.read("notes/x.md", &text);
    let summary: Vec<(usize, &str, &str, bool)> = marks
        .marks()
        .iter()
        .map(|mark| (mark.line(), mark.id(), mark.fingerprint(), mark.stale()))
        .collect();
    assert_eq!(
        summary,
        [
            (4, "docs/decisions/topic.md#name", current.as_str(), false),
            (7, "docs/decisions/topic.md#name", "00000000", true)
        ]
    );
    let findings: Vec<(&str, Option<usize>, FindingKind)> = marks
        .findings()
        .iter()
        .map(|finding| (finding.path(), finding.line(), finding.kind()))
        .collect();
    assert_eq!(
        findings,
        [
            ("notes/x.md", Some(7), FindingKind::GuideStale),
            ("notes/x.md", Some(10), FindingKind::InvalidMarker),
        ]
    );
}

// @kotowari[C-core-libinputs-14]
#[test]
fn parsing_a_decision_document_returns_its_decisions_and_rows_at_their_lines() {
    let source = SourceText::new(
        "docs/decisions/core/greet.md",
        "# Greetings\n\nDecides how to greet.\n\n## Greet\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo this.\n\n### Confirmations\n\n- C-core-greet-1 A greeting is shown\n- C-core-greet-2 (by reading) The skill says so\n\n## Wave\n\nStatus: proposed\n\n### Problem\n\nTrouble.\n\n### Decision\n\nWave.\n",
    )
    .unwrap();
    let document = kotowari_core::parse_decision_document(&source).unwrap();
    assert_eq!(document.path(), "docs/decisions/core/greet.md");
    assert!(document.findings().is_empty(), "{:?}", document.findings());
    let decisions: Vec<_> = document
        .decisions()
        .iter()
        .map(|d| (d.heading(), d.anchor(), d.line(), d.end_line(), d.state()))
        .collect();
    assert_eq!(
        decisions,
        vec![
            ("Greet", "greet", 5, 21, Some("implemented")),
            ("Wave", "wave", 22, 32, Some("proposed")),
        ]
    );
    let rows: Vec<_> = document.decisions()[0]
        .confirmations()
        .iter()
        .map(|row| (row.id(), row.line(), row.by_reading(), row.name()))
        .collect();
    assert_eq!(
        rows,
        vec![
            ("C-core-greet-1", 19, false, "A greeting is shown"),
            ("C-core-greet-2", 20, true, "The skill says so"),
        ]
    );
    assert!(document.decisions()[1].confirmations().is_empty());
}

// @kotowari[C-core-libinputs-15]
#[test]
fn parsing_a_decision_document_with_form_errors_returns_what_was_read_and_the_findings() {
    let source = SourceText::new(
        "docs/decisions/core/greet.md",
        "# Greetings\n\nDecides how to greet.\n\n## Greet\n\nStatus: done\n\n### Decision\n\nDo this.\n\n### Confirmations\n\n- C-core-greet-1 A greeting is shown\n- C-bad A broken row\n",
    )
    .unwrap();
    let document = kotowari_core::parse_decision_document(&source).unwrap();
    let decision = &document.decisions()[0];
    assert_eq!(
        (decision.heading(), decision.line(), decision.state()),
        ("Greet", 5, None)
    );
    let ids: Vec<_> = decision
        .confirmations()
        .iter()
        .map(|row| (row.id(), row.line()))
        .collect();
    assert_eq!(ids, vec![("C-core-greet-1", 15), ("C-bad", 16)]);
    let findings: Vec<_> = document
        .findings()
        .iter()
        .map(|f| (f.kind().as_str(), f.path(), f.line(), f.detail()))
        .collect();
    assert_eq!(
        findings,
        vec![
            (
                "decision_section_missing",
                "docs/decisions/core/greet.md",
                Some(5),
                "Problem"
            ),
            (
                "decision_state_invalid",
                "docs/decisions/core/greet.md",
                Some(7),
                "done"
            ),
            (
                "invalid_id",
                "docs/decisions/core/greet.md",
                Some(16),
                "C-bad"
            ),
        ]
    );
}
