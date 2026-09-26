//! 面の検査（docs/ir/core/surface.md、surface-unspecified.md）と、その設定の鍵 "surface"

use kotowari_core::config::Config;
use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。`surface` の行をそのまま設定に足す
fn make_project(tmp: &Path, surface: &str) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        format!("tests:\n  files:\n    - \"tests/**/*.rs\"\n{surface}"),
    )
    .unwrap();
    write(
        tmp,
        "docs/decision/records/r.md",
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    );
}

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// コマンドを走らせて (終了コード, 標準出力, 標準エラー) を返す
fn run(tmp: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(args)
        .current_dir(tmp)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

// --- S1: 面の設定の鍵と停止 ---

// @kotowari[TBL-core-004]
#[test]
fn tbl_004_surface_keys_default_to_empty_lists_and_no_list_file() {
    let cfg = Config::default();
    assert!(cfg.surface.files.is_empty());
    assert!(cfg.surface.rules.is_empty());
    assert_eq!(cfg.surface.unspecified, None);
}

// @kotowari[TBL-core-004, REQ-core-224]
#[test]
fn tbl_004_surface_keys_are_read_and_their_paths_normalized() {
    let cfg = Config::parse(
        "surface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"./rules/surface.yml\"\n  unspecified: \"docs//surface.yaml\"\n",
    )
    .unwrap();
    assert_eq!(cfg.surface.files, vec!["src/**/*.rs"]);
    assert_eq!(cfg.surface.rules, vec!["rules/surface.yml"]);
    assert_eq!(
        cfg.surface.unspecified.as_deref(),
        Some("docs/surface.yaml")
    );
}

// @kotowari[REQ-core-014]
#[test]
fn req_014_an_invalid_surface_files_glob_stops() {
    let result = Config::parse(
        "surface:\n  files:\n    - \"[invalid\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    assert!(result.is_err(), "invalid glob in surface.files should stop");
}

// @kotowari[REQ-core-014]
#[test]
fn req_014_null_or_absolute_surface_values_stop() {
    for yaml in [
        "surface:\n",
        "surface:\n  files:\n  rules:\n    - \"r.yml\"\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"r.yml\"\n  unspecified:\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"/r.yml\"\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"r.yml\"\n  unspecified: \"/s.yaml\"\n",
    ] {
        assert!(Config::parse(yaml).is_err(), "should stop: {yaml}");
    }
}

// @kotowari[EX-core-415, REQ-core-225]
#[test]
fn ex_core_415_rules_without_files_stop_with_a_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stdout}{stderr}");
    assert!(stderr.starts_with("config error: "), "{stderr}");
}

// @kotowari[EX-core-416, REQ-core-225]
#[test]
fn ex_core_416_files_without_rules_stop() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "surface:\n  files:\n    - \"src/**/*.rs\"\n");
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stdout}{stderr}");
    assert!(stderr.starts_with("config error: "), "{stderr}");
}

// @kotowari[EX-core-428, REQ-core-225]
#[test]
fn ex_core_428_a_list_key_without_rules_stops_even_list() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  unspecified: \"docs/surface.yaml\"\n",
    );
    let (code, stdout, stderr) = run(tmp.path(), &["list"]);
    assert_eq!(code, Some(2), "{stdout}{stderr}");
    assert!(stderr.starts_with("config error: "), "{stderr}");
}

// @kotowari[REQ-core-225]
#[test]
fn req_225_a_key_combination_error_stops_every_command_that_reads_the_config() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "surface:\n  files:\n    - \"src/**/*.rs\"\n");
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    write(tmp.path(), "results.json", "{}");
    for args in [
        &["list"][..],
        &["query", "REQ-core-001"],
        &["status"],
        &["mutants", "--tool", "cargo-mutants", "results.json"],
    ] {
        let (code, _, stderr) = run(tmp.path(), args);
        assert_eq!(code, Some(2), "{args:?}: {stderr}");
        assert!(stderr.starts_with("config error: "), "{args:?}: {stderr}");
    }
}
