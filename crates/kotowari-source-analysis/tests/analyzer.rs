use kotowari_core::{SourceText, config::Config};
use kotowari_source_analysis::Analyzer;

// @kotowari[C-core-libcrates-6]
#[test]
fn source_analysis_retains_source_facts_and_diagnostics_without_acquisition() {
    let analyzer = Analyzer::new(Config::default(), vec![], vec![]).unwrap();
    let source = SourceText::new(
        "nonexistent/test.rs",
        "// @kotowari[REQ-001]\n#[test]\nfn memory() {}\n",
    )
    .unwrap();
    let result = analyzer.tests(source.clone()).unwrap();
    assert_eq!(result.source.text(), source.text());
    assert_eq!(result.source.path(), "nonexistent/test.rs");
    assert!(result.has_query);
    assert_eq!(result.tests.len(), 1);
    assert_eq!(result.tests[0].name.as_deref(), Some("memory"));
    assert_eq!(result.tests[0].marker_ids[0].0, "REQ-001");
    let broken = analyzer
        .tests(SourceText::new("nonexistent/broken.rs", "fn broken( {").unwrap())
        .unwrap();
    assert!(broken.tests.is_empty());
    assert_eq!(broken.findings.len(), 1);
    let text = analyzer
        .tests(SourceText::new("nonexistent/file.txt", "@kotowari[REQ-001]\n").unwrap())
        .unwrap();
    assert!(!text.has_query);
    assert_eq!(text.line_markers.len(), 1);
}

// @kotowari[C-core-libpaths-10]
#[test]
fn cached_analysis_of_another_revision_is_rejected_at_native_admission() {
    use kotowari_core::{NativeSourceText, NativeTestAnalysis, ReadModel, RepositoryReadInputs};
    let config = Config::default();
    let analyzer = Analyzer::new(config.clone(), vec![], vec![]).unwrap();
    let acquired = "// acquired revision\n";
    let admit = |analyzed: &str| {
        let analysis = analyzer
            .tests(SourceText::new("tests/sample.rs", analyzed).unwrap())
            .unwrap();
        ReadModel::build_repository(RepositoryReadInputs {
            config: config.clone(),
            decisions: vec![NativeSourceText::new(
                "tests/sample.rs",
                "tests/sample.rs",
                acquired,
            )],
            tests: vec![NativeTestAnalysis {
                source: NativeSourceText::new("tests/sample.rs", "tests/sample.rs", acquired),
                analysis,
            }],
        })
    };
    assert!(admit("#[test]\nfn cached_revision() {}\n").is_err());
    assert!(admit(acquired).is_ok());
}

// @kotowari[C-core-surface-15]
#[test]
fn public_surface_inspection_reports_uncovered_analyzer_facts() {
    use kotowari_core::{FindingKind, surface};
    let rule = SourceText::new(
        "rules/commands.yaml",
        "id: command\nlanguage: rust\nrule:\n  pattern: 'fn $NAME() { $$$ }'\n",
    )
    .unwrap();
    let analyzer = Analyzer::new(Config::default(), vec![], vec![rule]).unwrap();
    let facts = analyzer
        .surfaces(SourceText::new("src/commands.rs", "fn known() {}\nfn missing() {}\n").unwrap())
        .unwrap();
    let document = SourceText::new("docs/decisions/topic.md", "# Commands\n\nScope.\n\n## Known\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nThe command \"known\".\n").unwrap();
    let mut findings = vec![];
    let tally = surface::check_analysis(&[facts], &[document], &[], &mut findings).unwrap();
    assert_eq!(tally.total(), 2);
    assert_eq!(tally.specified(), 1);
    assert_eq!(tally.unspecified(), 0);
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].kind(), FindingKind::SurfaceWithoutSpec);
    assert_eq!(findings[0].path(), "src/commands.rs");
    assert!(findings[0].detail().contains("missing"));
}

// @kotowari[C-core-libapi-5]
#[test]
fn malformed_public_rules_explain_the_analysis_failure() {
    let error = Analyzer::new(
        Config::default(),
        vec![
            SourceText::new("rules/broken.yaml", "id: broken\nlanguage: rust\nrule: [\n").unwrap(),
        ],
        vec![],
    )
    .err()
    .unwrap();
    assert!(!error.to_string().trim().is_empty());
    assert!(error.to_string().contains("rules/broken.yaml"));
}
