fn main() {
    let config: kotowari::Config = kotowari_core::config::Config::default();
    let inputs = kotowari_core::ReadInputs {
        config,
        ..Default::default()
    };
    assert_eq!(inputs.config.ir, "docs/ir");
    assert!(matches!(
        kotowari::ReadModel::build(inputs),
        Err(kotowari_core::InputError::InputMissing(_))
    ));
}
