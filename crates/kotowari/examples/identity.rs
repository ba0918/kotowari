fn main() {
    let source: kotowari::SourceText =
        kotowari_core::SourceText::new("memory.md", "# Memory\n").unwrap();
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
