//! ガイドの設定の鍵 "guides.files"（docs/decisions/core/guides.md）

use kotowari_core::config::Config;

// @kotowari[C-core-config-6]
#[test]
fn tbl_004_guides_files_defaults_to_an_empty_list() {
    assert!(Config::default().guides.files.is_empty());
    let cfg = Config::parse("decisions:\n  documents: docs/decisions\n").unwrap();
    assert!(
        cfg.guides.files.is_empty(),
        "without the key there are no guides"
    );
}

// @kotowari[C-core-config-7]
#[test]
fn tbl_004_guides_files_reads_a_list_of_globs() {
    let cfg = Config::parse("guides:\n  files:\n    - \"guides/**/*.md\"\n    - \"README.md\"\n")
        .unwrap();
    assert_eq!(cfg.guides.files, vec!["guides/**/*.md", "README.md"]);
}
