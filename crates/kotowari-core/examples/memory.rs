#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari_core::{CheckInputs, InputError, Inspection, ReadInputs, ReadModel, SourceText};
fn main() {
    let source = SourceText::new("docs/decisions/memory.md", "# Memory\n").unwrap();
    let parsed = kotowari_core::parse_decision_document(&source).unwrap();
    assert!(!parsed.findings().is_empty());
    assert!(matches!(
        ReadModel::build(ReadInputs::default()),
        Err(InputError::InputMissing(_))
    ));
    let read = ReadInputs {
        decisions: Some(vec![source]),
        tests: Some(vec![]),
        ..Default::default()
    };
    let model = ReadModel::build(read.clone()).unwrap();
    assert!(model.list().items().is_empty());
    let inspection = Inspection::build(CheckInputs {
        read,
        ..Default::default()
    })
    .unwrap();
    assert!(!inspection.check().findings().is_empty());
}
