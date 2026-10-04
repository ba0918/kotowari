//! ガイドの印（REQ-core-198〜REQ-core-206、TBL-core-036）と、その設定の鍵 "guides.files"

use kotowari_core::config::Config;

// @kotowari[TBL-core-004]
#[test]
fn tbl_004_guides_files_defaults_to_an_empty_list() {
    assert!(Config::default().guides.files.is_empty());
    let cfg = Config::parse("ir: docs/ir\n").unwrap();
    assert!(
        cfg.guides.files.is_empty(),
        "without the key there are no guides"
    );
}

// @kotowari[TBL-core-004]
#[test]
fn tbl_004_guides_files_reads_a_list_of_globs() {
    let cfg = Config::parse("guides:\n  files:\n    - \"guides/**/*.md\"\n    - \"README.md\"\n")
        .unwrap();
    assert_eq!(cfg.guides.files, vec!["guides/**/*.md", "README.md"]);
}
