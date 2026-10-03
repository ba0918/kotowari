use kotowari_core::{SourceText, config::Config};
use kotowari_source_analysis::Analyzer;

// @kotowari[REQ-core-308, REQ-core-315, EX-core-481, EX-core-496, EX-core-497]
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
