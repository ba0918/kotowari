use kotowari_core::{SourceText, ir};

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
    inputs.read.config.surface.unspecified = Some("unspecified.yaml".into());
    inputs.read.config.changes = Some(kotowari_core::config::ChangesConfig {
        files: vec![],
        records: vec![],
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
            .filter(|finding| finding.kind == FindingKind::UnparsableFile)
            .count(),
        1
    );
    assert_eq!(result.tests()["rs"].files, 1);
    assert!(result.tests()["rs"].query);
    assert_eq!(inspection.status().tests().files["rs"].files, 1);
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
            .any(|finding| finding.kind == FindingKind::GuideStale)
    );
    assert!(
        inspection
            .check()
            .findings()
            .iter()
            .any(|finding| finding.kind == FindingKind::SurfaceWithoutSpec)
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
            .any(|finding| finding.kind == FindingKind::RequirementWithoutTest)
    );
    assert_eq!(inspection.status().tests().marks, 1);
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
    for (index, expected) in [
        "IR",
        "records",
        "ADR",
        "test information",
        "guides",
        "surface analysis",
        "unspecified surfaces",
        "change records",
    ]
    .iter()
    .enumerate()
    {
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
        match Inspection::build(missing) {
            Err(InputError::InputMissing(group)) => assert_eq!(group, *expected),
            _ => panic!("expected missing {expected}"),
        }
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
