use kotowari_markdown_schema::{
    Document, Schema, ValidationOptions, extract_partial, extract_validated,
};
fn main() {
    let schema =
        Schema::parse("document:\n  title:\n    pattern: '^Expected$'\n    extract: title\n")
            .unwrap();
    let good = Document::parse("# Expected\n").unwrap();
    assert!(extract_validated(&schema, &good, ValidationOptions::default()).is_ok());
    let bad = Document::parse("# Other\n").unwrap();
    assert!(extract_validated(&schema, &bad, ValidationOptions::default()).is_err());
    let partial = extract_partial(&schema, &bad, ValidationOptions::default());
    assert!(!partial.findings().is_empty());
    assert_eq!(partial.values()["title"], "Other");
}
