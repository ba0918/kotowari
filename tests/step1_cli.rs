use assert_cmd::Command;
use std::path::Path;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn valid_project_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/valid-project").leak()
}

// --- REQ-001: コマンドは1つ ---

// @kotowari[REQ-001]
#[test]
fn req_001_only_check_subcommand() {
    // "kotowari" だけ（サブコマンドなし）は停止する
    cmd().current_dir(valid_project_dir()).assert().code(2);
    // "kotowari check" は通る
    cmd()
        .arg("check")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
}

// --- REQ-002: 受けるオプション ---

// @kotowari[REQ-002]
#[test]
fn req_002_only_format_and_config_options() {
    // --format json は通る
    cmd()
        .args(["check", "--format", "json"])
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
}

// --- REQ-004: 引数の誤り ---

// @kotowari[REQ-004, REQ-005, REQ-007]
#[test]
fn req_004_unknown_option_stops() {
    let output = cmd()
        .args(["check", "--verbose"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("")
        .get_output()
        .clone();
    // REQ-005: stderr に理由がある
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.is_empty(), "stderr should have a reason");
}

// @kotowari[REQ-004, REQ-005]
#[test]
fn req_004_positional_argument_stops() {
    cmd()
        .args(["check", "extra"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-004]
#[test]
fn req_004_unknown_format_value_stops() {
    cmd()
        .args(["check", "--format", "xml"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-004]
#[test]
fn req_004_missing_config_file_stops() {
    cmd()
        .args(["check", "--config", "nonexistent.yaml"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
}

// --- REQ-005: 停止の出力（別のテストで既にカバー） ---

// @kotowari[REQ-005]
#[test]
fn req_005_stop_writes_nothing_to_stdout_and_reason_to_stderr() {
    let assert = cmd()
        .args(["check", "--unknown"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(!stderr.is_empty());
}

// --- REQ-007: 終了コード ---

// @kotowari[REQ-007, TBL-002]
#[test]
fn req_007_exit_codes_zero_two() {
    // 成功: 終了コード 0
    cmd()
        .arg("check")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    // 停止: 終了コード 2
    cmd()
        .args(["check", "--bad"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2);
}

// --- REQ-021: 既定は json ---

// @kotowari[REQ-021]
#[test]
fn req_021_default_format_is_json() {
    let assert = cmd()
        .arg("check")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("should be valid JSON");
    assert_eq!(v["files"], 0);
    assert!(v["findings"].as_array().unwrap().is_empty());
}
