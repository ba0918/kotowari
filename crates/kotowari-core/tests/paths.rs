use kotowari_core::SourceText;

// @kotowari[C-core-libpaths-1, C-core-libpaths-2, C-core-libpaths-5]
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

// @kotowari[C-core-libpaths-5]
#[test]
fn relative_paths_with_a_colon_remain_valid_on_unix() {
    if cfg!(unix) {
        assert_eq!(
            SourceText::new("a:b/topic.md", "memory").unwrap().path(),
            "a:b/topic.md"
        );
    }
}

// @kotowari[C-core-libpaths-6, C-core-libpaths-7]
#[test]
fn duplicate_normalized_paths_and_cross_group_source_mismatches_are_rejected() {
    use kotowari_core::{InputError, ReadInputs, ReadModel};
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    inputs.decisions = Some(vec![
        SourceText::new("./docs/decisions/memory.md", "one").unwrap(),
        SourceText::new("docs/decisions//memory.md", "one").unwrap(),
    ]);
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InvalidInput(_))
    ));
    let test = |text: &str| kotowari_core::TestAnalysis {
        line_markers: vec![],
        source: SourceText::new("docs/decisions/memory.md", text).unwrap(),
        has_query: false,
        tests: vec![],
        findings: vec![],
    };
    inputs.config.tests.files = vec!["docs/**".into()];
    inputs.decisions = Some(vec![
        SourceText::new("docs/decisions/memory.md", "one").unwrap(),
    ]);
    inputs.tests = Some(vec![test("two")]);
    assert!(matches!(
        ReadModel::build(inputs.clone()),
        Err(InputError::InvalidInput(_))
    ));
    inputs.tests = Some(vec![test("one")]);
    assert!(ReadModel::build(inputs).is_ok());
}

// @kotowari[C-core-libpaths-8]
#[test]
fn inspection_compares_original_test_and_surface_source_texts() {
    use kotowari_core::{CheckInputs, InputError, Inspection, SurfaceAnalysis, TestAnalysis};
    let mut inputs = CheckInputs::default();
    inputs.read.decisions = Some(vec![]);
    inputs.read.tests = Some(vec![TestAnalysis {
        line_markers: vec![],
        source: SourceText::new("src/memory.rs", "one").unwrap(),
        has_query: true,
        tests: vec![],
        findings: vec![],
    }]);
    inputs.read.config.surface.rules = vec!["rules.yaml".into()];
    inputs.read.config.surface.files = vec!["src/**".into()];
    inputs.surface = Some(vec![SurfaceAnalysis {
        source: SourceText::new("src/memory.rs", "two").unwrap(),
        has_query: true,
        surfaces: vec![],
        findings: vec![],
    }]);
    assert!(matches!(
        Inspection::build(inputs),
        Err(InputError::InvalidInput(_))
    ));
}
