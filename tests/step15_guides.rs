//! ガイドの印（REQ-core-198〜REQ-core-206、TBL-core-036）と、その設定の鍵 "guides.files"

use kotowari_core::config::Config;

// --- REQ-core-014、TBL-core-004: "guides.files" の鍵 ---

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

// @kotowari[REQ-core-014]
#[test]
fn req_014_an_unreadable_guides_glob_is_a_config_error() {
    let tmp = tempfile::TempDir::new().unwrap();
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "guides:\n  files:\n    - \"[invalid\"\n",
    )
    .unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("config error: "), "{stderr}");
    // 知らない鍵としてではなく、glob として読めない要素として止まる
    assert!(
        stderr.contains("invalid glob pattern: [invalid"),
        "{stderr}"
    );
}
