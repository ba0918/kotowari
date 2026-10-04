use kotowari_core::{SourceText, ir};

#[path = "admission.rs"]
mod admission;

// @kotowari[REQ-core-314, REQ-core-043, EX-core-290]
#[test]
fn partial_parsing_retains_a_nameless_requirement_and_its_heading_diagnostic() {
    let source = SourceText::new("docs/ir/topic.md", "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001:\n\n- kind: ubiquitous\n- verification: unit\n\nBody.\n").unwrap();
    let document = ir::parse(&source, Default::default()).unwrap();
    assert_eq!(document.items().len(), 1);
    assert_eq!(document.items()[0].id(), Some("REQ-001"));
    assert_eq!(document.items()[0].line(), 7);
    assert!(
        document
            .findings()
            .iter()
            .any(|finding| finding.kind().as_str() == "unknown_heading"
                && finding.line() == Some(7)
                && finding.detail() == "### REQ-001:")
    );
}

// @kotowari[REQ-core-315, REQ-core-317, EX-core-488, EX-core-489]
#[test]
fn required_groups_are_distinct_from_explicit_empty_groups() {
    use kotowari_core::{InputError, ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InputMissing(_))
    ));
    inputs.ir = Some(vec![]);
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InputMissing(_))
    ));
    inputs.records = Some(vec![]);
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InputMissing(_))
    ));
    inputs.adr = Some(vec![]);
    assert!(ReadModel::build(inputs).is_ok());
}

// @kotowari[REQ-core-317, EX-core-491, REQ-core-312]
#[test]
fn read_models_list_and_query_without_check_only_inputs() {
    use kotowari_core::{ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    inputs.config.guides.files = vec!["guides/**/*.md".into()];
    inputs.ir = Some(vec![SourceText::new("docs/ir/memory.md", "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- verification: unit\n\nBody.\n").unwrap()]);
    inputs.records = Some(vec![]);
    inputs.adr = Some(vec![]);
    let model = ReadModel::build(inputs).unwrap();
    assert!(!model.list().items().is_empty());
    assert!(model.query("REQ-001").is_ok());
    assert!(model.query("REQ-999").is_err());
}

// @kotowari[REQ-core-315, EX-core-488, EX-core-489]
#[test]
fn enabled_check_groups_must_be_provided_even_when_empty() {
    use kotowari_core::{CheckInputs, InputError, Inspection};
    let mut inputs = CheckInputs::default();
    inputs.read.config.tests.files.clear();
    inputs.read.ir = Some(vec![]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.config.guides.files = vec!["guides/**".into()];
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.read.config.surface.unspecified = Some("unspecified.yaml".into());
    inputs.read.config.changes = Some(kotowari_core::config::ChangesConfig {
        files: vec!["src/**".into()],
        records: vec!["changes/**".into()],
        exclude: vec![],
    });
    for group in 0..4 {
        assert!(matches!(
            Inspection::build(inputs.clone()),
            Err(InputError::InputMissing(_))
        ));
        match group {
            0 => inputs.guides = Some(vec![]),
            1 => inputs.surface = Some(vec![]),
            2 => inputs.unspecified = Some(vec![]),
            _ => inputs.changes = Some(vec![]),
        }
    }
    assert!(Inspection::build(inputs).is_ok());
}

// @kotowari[REQ-core-308, REQ-core-315, EX-core-481, EX-core-496, EX-core-497]
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
    inputs.read.ir = Some(vec![]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        line_markers: vec![],
        source: source.clone(),
        language: Some("rust".into()),
        has_query: true,
        tests: vec![],
        findings: vec![diagnostic.clone()],
    }]);
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.surface = Some(vec![SurfaceAnalysis {
        source,
        language: Some("rust".into()),
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

// @kotowari[REQ-core-315, EX-core-488, EX-core-489]
#[test]
fn disabled_groups_are_ignored_including_their_inconsistent_sources() {
    use kotowari_core::{CheckInputs, Inspection, TestAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.config.tests.files.clear();
    inputs.read.ir = Some(vec![
        SourceText::new("docs/ir/memory.md", "# Topic\n").unwrap(),
    ]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        line_markers: vec![],
        source: SourceText::new("docs/ir/memory.md", "different").unwrap(),
        language: None,
        has_query: false,
        tests: vec![],
        findings: vec![],
    }]);
    inputs.guides = Some(vec![
        SourceText::new("docs/ir/memory.md", "different").unwrap(),
    ]);
    let inspection = Inspection::build(inputs).unwrap();
    assert!(inspection.check().tests().is_empty());
}

// @kotowari[REQ-core-315, EX-core-488, EX-core-489, EX-core-497]
#[test]
fn enabled_guide_and_surface_inputs_are_evaluated_without_acquisition() {
    use kotowari_core::{CheckInputs, FindingKind, Inspection, SurfaceAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.config.tests.files.clear();
    inputs.read.ir = Some(vec![]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.config.guides.files = vec!["guides/**".into()];
    inputs.guides = Some(vec![
        SourceText::new("guides/memory.md", "<!-- @kotowari[REQ-001:12345678] -->\n").unwrap(),
    ]);
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.surface = Some(vec![SurfaceAnalysis {
        source: SourceText::new("src/memory.rs", "anything").unwrap(),
        language: Some("rust".into()),
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

// @kotowari[REQ-core-314, REQ-core-317, EX-core-487]
#[test]
fn partial_ir_retains_invalid_and_missing_ids_at_the_original_lines() {
    let source = SourceText::new("docs/ir/memory.md", "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-bad: Invalid\n\nBody.\n\n### : Missing\n\nBody.\n").unwrap();
    let document = ir::parse(&source, Default::default()).unwrap();
    assert_eq!(document.items().len(), 2);
    assert_eq!(document.items()[0].id(), Some("REQ-bad"));
    assert_eq!(document.items()[0].line(), 7);
    assert_eq!(document.items()[0].end_line(), Some(10));
    assert_eq!(document.items()[1].id(), None);
    assert_eq!(document.items()[1].line(), 11);
    assert!(!document.findings().is_empty());
}

// @kotowari[REQ-core-314, REQ-core-322, EX-core-487, EX-core-498]
#[test]
fn nested_ir_paths_preserve_the_document_kind_of_the_filename() {
    let source = SourceText::new(
        "docs/ir/topic/FLAGS.md",
        "# Flags\n\nScope.\n\n### FLAG-001: Open\n\n- kind: gap\n\nBody.\n",
    )
    .unwrap();
    let document = ir::parse(&source, Default::default()).unwrap();
    assert_eq!(document.items().len(), 1);
    assert_eq!(document.items()[0].id(), Some("FLAG-001"));
    assert_eq!(document.path(), "docs/ir/topic/FLAGS.md");
}

// @kotowari[REQ-core-314, REQ-core-317, REQ-core-322, REQ-core-054, EX-core-487, EX-core-498]
#[test]
fn parsed_documents_retain_nested_paths_and_definition_and_statement_references() {
    let text = "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001: Selection\n\n- kind: algorithm\n- verification: unit\n- definition: TBL-001\n\nUses `REQ-002`.\n";
    let full = ir::parse_document("nested/topic.md", text).unwrap();
    assert_eq!(full.filename(), "topic.md");
    assert_eq!(full.relative_path(), "nested/topic.md");
    let source = SourceText::new("docs/ir/nested/topic.md", text).unwrap();
    let partial = ir::parse(&source, Default::default()).unwrap();
    assert_eq!(partial.path(), "docs/ir/nested/topic.md");
    let references = partial.items()[0].references();
    assert_eq!(
        references
            .iter()
            .map(|reference| reference.id)
            .collect::<Vec<_>>(),
        ["TBL-001", "REQ-002"]
    );
    assert!(matches!(references[0].via, ir::Via::Definition));
    assert!(matches!(references[1].via, ir::Via::Text));
    assert_eq!(references[0].finding_line, 11);
    assert_eq!(references[1].finding_line, 13);
}

// @kotowari[REQ-core-308, REQ-core-315, EX-core-481, EX-core-496]
#[test]
fn non_query_marker_facts_are_consumed_without_source_discovery() {
    use kotowari_core::{CheckInputs, FindingKind, Inspection, TestAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.ir = Some(vec![SourceText::new("docs/ir/memory.md", "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- verification: unit\n\nBody.\n").unwrap()]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        source: SourceText::new("checks/memory.txt", "@kotowari[REQ-001]\n").unwrap(),
        language: None,
        has_query: false,
        tests: vec![],
        findings: vec![],
        line_markers: vec![kotowari_core::tests_discovery::Marker {
            ids: vec!["REQ-001".into()],
            line: 1,
        }],
    }]);
    let inspection = Inspection::build(inputs).unwrap();
    assert!(
        !inspection
            .check()
            .findings()
            .iter()
            .any(|finding| finding.kind() == FindingKind::RequirementWithoutTest)
    );
    assert_eq!(inspection.status().tests().marks(), 1);
}

// @kotowari[REQ-core-315, EX-core-488, EX-core-489]
#[test]
fn each_enabled_group_is_required_independently_and_empty_is_provided() {
    use kotowari_core::{CheckInputs, InputError, Inspection};
    let mut all = CheckInputs::default();
    all.read.ir = Some(vec![]);
    all.read.records = Some(vec![]);
    all.read.adr = Some(vec![]);
    all.read.tests = Some(vec![]);
    all.read.config.guides.files = vec!["guides/**".into()];
    all.read.config.surface.files = vec!["src/**".into()];
    all.read.config.surface.rules = vec!["rules.yaml".into()];
    all.read.config.surface.unspecified = Some("unspecified.yaml".into());
    all.read.config.changes = Some(kotowari_core::config::ChangesConfig {
        files: vec!["src/**".into()],
        records: vec!["changes/**".into()],
        exclude: vec![],
    });
    all.guides = Some(vec![]);
    all.surface = Some(vec![]);
    all.unspecified = Some(vec![]);
    all.changes = Some(vec![]);
    assert!(Inspection::build(all.clone()).is_ok());
    for index in 0..8 {
        let mut missing = all.clone();
        match index {
            0 => missing.read.ir = None,
            1 => missing.read.records = None,
            2 => missing.read.adr = None,
            3 => missing.read.tests = None,
            4 => missing.guides = None,
            5 => missing.surface = None,
            6 => missing.unspecified = None,
            _ => missing.changes = None,
        }
        assert!(matches!(
            Inspection::build(missing),
            Err(InputError::InputMissing(_))
        ));
    }
}

// @kotowari[REQ-core-315, REQ-core-322, EX-core-488, EX-core-489, EX-core-499]
#[test]
fn same_text_shared_between_guides_and_tests_uses_the_overlap_rule() {
    use kotowari_core::{CheckInputs, InputError, Inspection, TestAnalysis};
    let source = SourceText::new("shared.txt", "text").unwrap();
    let mut inputs = CheckInputs::default();
    inputs.read.ir = Some(vec![]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        source: source.clone(),
        language: None,
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

// @kotowari[REQ-core-312, EX-core-484]
#[test]
fn invalid_typed_configuration_is_an_execution_failure_not_a_completed_inspection() {
    use kotowari_core::{CheckInputs, InputError, Inspection};
    let mut inputs = CheckInputs::default();
    inputs.read.ir = Some(vec![]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
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
    inputs.read.ir = Some(vec![SourceText::new(
        "docs/ir/topic.md",
        "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: read\n\nBody.\n",
    )
    .unwrap()]);
    inputs.read.records = Some(vec![
        SourceText::new(
            "docs/decision/records/r.md",
            "# R\n\n## Context\n\nc\n\n## Agreements\n\n- A1 decided\n  - why: because\n",
        )
        .unwrap(),
    ]);
    inputs.read.adr = Some(vec![]);
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
                "REQ-001 00000000 11111111".into(),
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

// @kotowari[REQ-core-315, TBL-core-042, REQ-core-290]
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

// @kotowari[REQ-core-315, TBL-core-042]
#[test]
fn an_additional_group_is_optional_and_only_added_when_given() {
    use kotowari_core::Inspection;
    let inspection = Inspection::build(empty_check_inputs()).unwrap();
    assert!(inspection.check().findings().is_empty());
    assert!(inspection.check().group("overview").is_none());
    assert!(inspection.check().groups().is_empty());
    assert!(inspection.status().complete());
}

// @kotowari[REQ-core-315, TBL-core-042]
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

// @kotowari[REQ-core-286]
#[test]
fn guide_marks_of_one_text_are_read_with_lines_and_staleness_without_the_overlap_stop() {
    use kotowari_core::{FindingKind, ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files = vec!["notes/**".into()];
    inputs.config.guides.files = vec!["notes/**".into()];
    inputs.ir = Some(vec![SourceText::new(
        "docs/ir/topic.md",
        "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- verification: review\n- how_to_verify: read\n\nBody.\n",
    )
    .unwrap()]);
    inputs.records = Some(vec![]);
    inputs.adr = Some(vec![]);
    inputs.tests = Some(vec![]);
    let model = ReadModel::build(inputs).unwrap();
    let report = model.query("REQ-001").unwrap();
    let kotowari_core::ListItem::Requirement(requirement) = report.items()[0].item() else {
        panic!("REQ-001 is a requirement");
    };
    let current = requirement.fingerprint().to_string();
    let text = format!(
        "# Guide\n\n## Fresh\n<!-- @kotowari[REQ-001:{current}] -->\n\n## Old\n<!-- @kotowari[REQ-001:00000000] -->\n\n## Broken\n<!-- @kotowari[REQ-001] -->\n"
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
            (4, "REQ-001", current.as_str(), false),
            (7, "REQ-001", "00000000", true)
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

fn model_with_records() -> kotowari_core::ReadModel {
    use kotowari_core::{ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    inputs.ir = Some(vec![SourceText::new(
        "docs/ir/core/topic.md",
        "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-core-001: Name\n\n- kind: ubiquitous\n- source: docs/decision/records/2026-01-01-x.md#A1\n- verification: review\n- how_to_verify: read\n\nBody.\n",
    )
    .unwrap()]);
    inputs.records = Some(vec![
        SourceText::new(
            "docs/decision/records/2026-01-01-x.md",
            "# X\n\n## Context\n\nc\n\n## Agreements\n\n- A1 古い決定の文\n  - why: w\n  - superseded_by: [A2](#A2)\n- A2 新しい決定\n  - why: w\n  - superseded_by:\n",
        )
        .unwrap(),
        SourceText::new(
            "docs/decision/records/notes.md",
            "# Notes\n\n## 背景\n\n一行目\n\n二行目\n\n## 次\n\n別の節\n",
        )
        .unwrap(),
    ]);
    inputs.adr = Some(vec![
        SourceText::new(
            "docs/decision/adr/0001-a.md",
            "# ADR\n\n## Decision\n\n決めたこと\n```\n## 中\n```\n",
        )
        .unwrap(),
    ]);
    ReadModel::build(inputs).unwrap()
}

// @kotowari[REQ-core-291, TBL-core-039]
#[test]
fn a_source_resolves_to_the_decision_line_or_the_heading_section_it_points_at() {
    use kotowari_core::sources::SourceTarget;
    let model = model_with_records();
    let Some(SourceTarget::Decision {
        path,
        number,
        text,
        superseded,
    }) = model.source_target("docs/decision/records/2026-01-01-x.md#A1")
    else {
        panic!("A1 is a decision");
    };
    assert_eq!(
        (path.as_str(), number.as_str(), text.as_str(), superseded),
        (
            "docs/decision/records/2026-01-01-x.md",
            "A1",
            "古い決定の文",
            true
        )
    );
    let Some(SourceTarget::Decision { superseded, .. }) =
        model.source_target("docs/decision/records/2026-01-01-x.md#A2")
    else {
        panic!("A2 is a decision");
    };
    assert!(!superseded, "an empty superseded_by line counts as absent");
    let Some(SourceTarget::Heading { heading, lines, .. }) =
        model.source_target("docs/decision/records/notes.md#背景")
    else {
        panic!("a heading of a file that is not a decision record");
    };
    assert_eq!(
        (heading.as_str(), lines),
        ("背景", vec!["一行目".to_string(), "二行目".to_string()])
    );
    let Some(SourceTarget::Heading { lines, .. }) =
        model.source_target("docs/decision/adr/0001-a.md#Decision")
    else {
        panic!("an ADR heading");
    };
    assert_eq!(lines, ["決めたこと", "```", "## 中", "```"]);
    for missing in [
        "docs/decision/records/2026-01-01-x.md#A9",
        "docs/decision/adr/0001-a.md#中",
        "docs/other.md#A1",
        "REQ-core-001",
    ] {
        assert!(model.source_target(missing).is_none(), "{missing}");
    }
}

// @kotowari[REQ-core-284]
#[test]
fn the_read_documents_expose_their_place_relative_paths_and_kinds() {
    let model = model_with_records();
    let documents: Vec<(&str, kotowari_core::DocKind)> = model
        .documents()
        .iter()
        .map(|document| (document.relative_path(), document.kind()))
        .collect();
    assert_eq!(
        documents,
        [("core/topic.md", kotowari_core::DocKind::Topic)]
    );
}
