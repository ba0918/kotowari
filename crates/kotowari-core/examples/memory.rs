#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari_core::{
    CheckInputs, InputError, Inspection, IrOptions, ReadInputs, ReadModel, SourceText,
};
fn main() {
    let source = SourceText::new("docs/ir/memory.md", "# Memory\n").unwrap();
    let parsed = kotowari_core::ir::parse(&source, IrOptions::default()).unwrap();
    assert!(!parsed.findings().is_empty());
    assert!(matches!(
        ReadModel::build(ReadInputs::default()),
        Err(InputError::InputMissing(_))
    ));
    let read = ReadInputs {
        ir: Some(vec![source]),
        records: Some(vec![]),
        adr: Some(vec![]),
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
