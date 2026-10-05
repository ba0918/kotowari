#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
fn main() {
    let source: kotowari::SourceText =
        kotowari_core::SourceText::new("memory.md", "# Memory\n").unwrap();
    #[expect(
        clippy::needless_pass_by_value,
        reason = "the example shows that the re-exported type is accepted where the core type is taken by value"
    )]
    fn core_consumer(source: kotowari_core::SourceText) {
        assert_eq!(source.path(), "memory.md");
    }
    core_consumer(source);
    let inputs: kotowari::ReadInputs = kotowari_core::ReadInputs::default();
    assert!(matches!(
        kotowari::ReadModel::build(inputs),
        Err(kotowari_core::InputError::InputMissing(_))
    ));
}
