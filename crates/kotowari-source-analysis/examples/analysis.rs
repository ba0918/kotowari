use kotowari_source_analysis::{Analyzer, SourceText};
fn main() {
    let analyzer = Analyzer::new(Default::default(), vec![], vec![]).unwrap();
    let source = SourceText::new("memory.txt", "@kotowari[REQ-001]\n").unwrap();
    let result = analyzer.tests(source).unwrap();
    assert_eq!(result.line_markers.len(), 1);
    assert_eq!(result.source.text(), "@kotowari[REQ-001]\n");
}
