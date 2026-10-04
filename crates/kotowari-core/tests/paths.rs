use kotowari_core::SourceText;

// @kotowari[REQ-core-322, EX-core-498, EX-core-503]
#[test]
fn logical_paths_are_normalized_without_accessing_the_filesystem() {
    let source = SourceText::new("./memory\\nested//./doc.md/", "# Memory").unwrap();
    assert_eq!(source.path(), "memory/nested/doc.md");
    assert_eq!(source.text(), "# Memory");
    assert_eq!(
        SourceText::new("../outside.md", "text").unwrap().path(),
        "../outside.md"
    );
    assert!(SourceText::new("/absolute.md", "text").is_err());
    assert!(SourceText::new("./", "text").is_err());
}

// @kotowari[REQ-core-322, EX-core-498]
#[test]
fn relative_paths_with_a_colon_remain_valid_on_unix() {
    if cfg!(unix) {
        assert_eq!(
            SourceText::new("a:b/topic.md", "memory").unwrap().path(),
            "a:b/topic.md"
        );
    }
}

// @kotowari[REQ-core-322, EX-core-499, EX-core-502]
#[test]
fn duplicate_normalized_paths_and_cross_group_source_mismatches_are_rejected() {
    use kotowari_core::{InputError, ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    inputs.ir = Some(vec![
        SourceText::new("./docs/ir/memory.md", "one").unwrap(),
        SourceText::new("docs/ir//memory.md", "one").unwrap(),
    ]);
    inputs.records = Some(vec![]);
    inputs.adr = Some(vec![]);
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InvalidInput(_))
    ));
    inputs.ir = Some(vec![SourceText::new("docs/ir/memory.md", "one").unwrap()]);
    inputs.records = Some(vec![SourceText::new("docs/ir/memory.md", "two").unwrap()]);
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InvalidInput(_))
    ));
    inputs.records = Some(vec![SourceText::new("docs/ir/memory.md", "one").unwrap()]);
    assert!(ReadModel::build(inputs).is_ok());
}

// @kotowari[REQ-core-322, EX-core-502]
#[test]
fn inspection_compares_original_test_and_surface_source_texts() {
    use kotowari_core::{CheckInputs, InputError, Inspection, SurfaceAnalysis, TestAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.ir = Some(vec![]);
    inputs.read.records = Some(vec![]);
    inputs.read.adr = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        line_markers: vec![],
        source: SourceText::new("src/memory.rs", "one").unwrap(),
        language: Some("rust".into()),
        has_query: true,
        tests: vec![],
        findings: vec![],
    }]);
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.surface = Some(vec![SurfaceAnalysis {
        source: SourceText::new("src/memory.rs", "two").unwrap(),
        language: Some("rust".into()),
        has_query: true,
        surfaces: vec![],
        findings: vec![],
    }]);
    assert!(matches!(
        Inspection::build(inputs),
        Err(InputError::InvalidInput(_))
    ));
}
