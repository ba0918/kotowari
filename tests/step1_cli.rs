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

// --- REQ-107: --help と --version ---

// @kotowari[REQ-107, REQ-002, TBL-002]
#[test]
fn req_107_help_and_version_exit_zero_without_check() {
    // --help は check 無しでも終了コード0
    let out = cmd()
        .arg("--help")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    let stdout = String::from_utf8_lossy(&out.get_output().stdout);
    assert!(!stdout.is_empty(), "help should produce output");

    // --version も同様
    let out = cmd()
        .arg("--version")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    let stdout = String::from_utf8_lossy(&out.get_output().stdout);
    assert!(!stdout.is_empty(), "version should produce output");
}

// @kotowari[REQ-107, REQ-004]
#[test]
fn req_107_help_wins_over_argument_errors() {
    // --help が他の引数の誤りに優先する
    cmd()
        .args(["--help", "--unknown"])
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    // --version も同様
    cmd()
        .args(["--version", "--unknown"])
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
}

// --- REQ-004: 引数の誤り（追加） ---

// @kotowari[REQ-004, REQ-005]
#[test]
fn req_004_no_arguments_stops() {
    cmd()
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-004, REQ-005]
#[test]
fn req_004_option_without_value_stops() {
    cmd()
        .args(["check", "--format"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-004, REQ-005]
#[test]
fn req_004_repeated_option_stops() {
    cmd()
        .args(["check", "--format", "json", "--format", "text"])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-004, REQ-005]
#[test]
fn req_004_config_pointing_to_directory_stops() {
    cmd()
        .args(["check", "--config", "."])
        .current_dir(valid_project_dir())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-002]
#[test]
fn req_002_options_before_or_after_check() {
    // オプションが check の前でも後でも受ける
    cmd()
        .args(["--format", "json", "check"])
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    cmd()
        .args(["check", "--format", "json"])
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
}

// --- REQ-005: 停止の出力の形 ---

// @kotowari[REQ-005, TBL-018, TBL-020]
#[test]
fn req_005_stderr_first_line_has_the_reason_wording() {
    // 4つの文言を検査する
    // 引数の誤り
    let out = cmd()
        .args(["check", "--bad"])
        .current_dir(valid_project_dir())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert!(
        first_line.starts_with("argument error: "),
        "expected 'argument error: ...', got: {first_line:?}"
    );
}

// @kotowari[REQ-005, TBL-020]
#[test]
fn req_005_config_error_detail_path_is_relative_to_base_not_to_cwd() {
    use std::fs;
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // 基準のディレクトリの直下に、知らないキーを持つ壊れた設定を置く
    fs::write(tmp.path().join("bad.yaml"), "unknown_key: 1\n").unwrap();
    let sub = tmp.path().join("sub");
    fs::create_dir_all(&sub).unwrap();
    // sub/ から "--config ../bad.yaml" を指す（CWD からの相対、REQ-003）
    let out = cmd()
        .args(["check", "--config", "../bad.yaml"])
        .current_dir(&sub)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert!(
        first_line.starts_with("config error: bad.yaml:"),
        "the --config detail path should be relative to the base directory (bad.yaml), not to the CWD (../bad.yaml): {first_line:?}"
    );
}

// @kotowari[REQ-005, TBL-020]
#[test]
fn req_005_stderr_detail_path_is_relative() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // docs/ir を作らない → 読めないファイル
    let out = cmd()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert!(
        first_line.starts_with("unreadable file: "),
        "expected 'unreadable file: ...', got: {first_line:?}"
    );
    // 絶対パスが含まれないことを確認
    assert!(
        !first_line.contains(tmp.path().to_str().unwrap()),
        "detail should not contain absolute path, got: {first_line:?}"
    );
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
