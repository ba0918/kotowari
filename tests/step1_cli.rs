use assert_cmd::Command;
use std::path::Path;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn valid_project_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/valid-project").leak()
}

// --- REQ-001: コマンドは5つ ---

// @kotowari[REQ-001]
#[test]
fn req_001_five_commands_only() {
    // "kotowari check" は通る
    cmd()
        .arg("check")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    // "kotowari list" も2つ目のコマンドとして通る
    cmd()
        .arg("list")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    // "kotowari mutants" も3つ目のコマンドとして通る
    let tmp = dir_with_results(&["outcomes.json"]);
    cmd()
        .args(["mutants", "--tool", "cargo-mutants", "outcomes.json"])
        .current_dir(tmp.path())
        .assert()
        .code(0);
    // "kotowari query" は4つ目、"kotowari status" は5つ目のコマンドとして通る
    let project = dir_with_one_requirement();
    cmd()
        .args(["query", "REQ-001"])
        .current_dir(project.path())
        .assert()
        .code(0);
    cmd()
        .arg("status")
        .current_dir(project.path())
        .assert()
        .code(0);
    // ほかの語はコマンドにならない
    cmd()
        .arg("mutate")
        .current_dir(valid_project_dir())
        .assert()
        .code(2);
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

// REQ-008: 受けるオプションの集合を固定するのはこのテスト（否定側）
// @kotowari[REQ-004, REQ-005, REQ-007, REQ-008, EX-001]
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
    // 作らないコマンドの名前も、コマンドの後ろでは位置引数の誤り
    cmd()
        .args(["check", "render"])
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

// @kotowari[REQ-004]
#[test]
fn req_004_unknown_command_before_check_has_the_unknown_command_wording() {
    // "check" より前の未知の位置引数は "unknown command: ..."（"unexpected argument: ..." ではない）
    let output = cmd()
        .args(["foo"])
        .current_dir(valid_project_dir())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert_eq!(
        first_line, "argument error: unknown command: foo",
        "got: {first_line:?}"
    );
}

// @kotowari[REQ-004, TBL-020, EX-219]
#[test]
fn req_004_no_arguments_names_all_five_commands() {
    let output = cmd().current_dir(valid_project_dir()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert_eq!(
        first_line, "argument error: expected command: check, list, mutants, query or status",
        "got: {first_line:?}"
    );
}

// @kotowari[REQ-004, TBL-020, EX-241]
#[test]
fn req_004_options_without_a_command_names_all_five_commands() {
    let output = cmd()
        .args(["--format", "text"])
        .current_dir(valid_project_dir())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert_eq!(
        first_line, "argument error: expected command: check, list, mutants, query or status",
        "got: {first_line:?}"
    );
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
    fs::create_dir_all(tmp.path().join("docs/decision/records")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
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
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
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

// @kotowari[REQ-005, TBL-020]
#[test]
fn req_005_config_outside_the_base_is_shown_relative_with_parent_segments() {
    // A164: 基準の外にある設定ファイルの詳細は "../" を含む基準からの相対パス
    use std::fs;
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    let base = tmp.path().join("proj");
    fs::create_dir_all(base.join(".kotowari")).unwrap();
    fs::create_dir_all(base.join("docs/ir")).unwrap();
    fs::create_dir_all(base.join("docs/decision/records")).unwrap();
    fs::create_dir_all(base.join("docs/decision/adr")).unwrap();
    fs::write(
        base.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // 基準の1つ上に壊れた設定を置き、基準の下の sub/ から指す
    fs::write(tmp.path().join("bad.yaml"), "unknown_key: 1\n").unwrap();
    let sub = base.join("sub");
    fs::create_dir_all(&sub).unwrap();
    let out = cmd()
        .args(["check", "--config", "../../bad.yaml"])
        .current_dir(&sub)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert!(
        first_line.starts_with("config error: ../bad.yaml:"),
        "the detail should be relative to the base (../bad.yaml), not to the CWD: {first_line:?}"
    );
}


// --- REQ-002、REQ-144: "kotowari mutants" の引数が結果のファイルに届く ---

/// 捕まえた変異が1件だけの結果のファイル
const ONE_CAUGHT_RESULT: &str = r#"{"outcomes":[
  {"scenario":{"Mutant":{"file":"src/a.rs","name":"src/a.rs:3:5: replace f with ()",
   "span":{"start":{"line":3,"column":5}}}},"summary":"CaughtMutant"}
]}"#;

// @kotowari[REQ-002, EX-244]
#[test]
fn req_002_mutants_options_can_come_before_the_command_and_after_the_path() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("outcomes.json"), ONE_CAUGHT_RESULT).unwrap();
    let output = cmd()
        .args([
            "--tool",
            "cargo-mutants",
            "mutants",
            "outcomes.json",
            "--format",
            "text",
        ])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(0),
        "the options may sit before the command and after the path: {stderr}"
    );
}

// @kotowari[REQ-144]
#[test]
fn req_144_missing_result_file_is_an_unreadable_file() {
    let output = cmd()
        .args(["mutants", "--tool", "cargo-mutants", "no-such-file.json"])
        .current_dir(valid_project_dir())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert!(
        first_line.starts_with("unreadable file: "),
        "expected 'unreadable file: ...', got: {first_line:?}"
    );
}

// --- REQ-149: mutants の引数 ---

/// 読める結果のファイルを置いた一時ディレクトリを作る
fn dir_with_results(names: &[&str]) -> tempfile::TempDir {
    let tmp = tempfile::TempDir::new().unwrap();
    for name in names {
        std::fs::write(tmp.path().join(name), ONE_CAUGHT_RESULT).unwrap();
    }
    tmp
}

/// 引数の誤りで停止することを見る
fn assert_argument_error(args: &[&str], dir: &Path) {
    let output = cmd().args(args).current_dir(dir).output().unwrap();
    assert_eq!(output.status.code(), Some(2), "{args:?} should stop");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    assert!(
        first_line.starts_with("argument error: "),
        "{args:?} should stop as an argument error, got: {first_line:?}"
    );
}

// @kotowari[REQ-149, EX-218]
#[test]
fn req_149_mutants_without_tool_is_an_argument_error() {
    let tmp = dir_with_results(&["outcomes.json"]);
    assert_argument_error(&["mutants", "outcomes.json"], tmp.path());
}

// @kotowari[REQ-149, EX-240]
#[test]
fn req_149_unknown_tool_is_an_argument_error() {
    let tmp = dir_with_results(&["a.json"]);
    assert_argument_error(&["mutants", "--tool", "stryker", "a.json"], tmp.path());
}

// @kotowari[REQ-149, EX-242]
#[test]
fn req_149_two_result_paths_is_an_argument_error() {
    let tmp = dir_with_results(&["a.json", "b.json"]);
    assert_argument_error(
        &["mutants", "--tool", "cargo-mutants", "a.json", "b.json"],
        tmp.path(),
    );
}

// @kotowari[REQ-149]
#[test]
fn req_149_mutants_without_a_result_path_is_an_argument_error() {
    let tmp = dir_with_results(&[]);
    assert_argument_error(&["mutants", "--tool", "cargo-mutants"], tmp.path());
}

// @kotowari[REQ-004]
#[test]
fn req_004_tool_on_check_is_an_argument_error() {
    assert_argument_error(
        &["check", "--tool", "cargo-mutants"],
        valid_project_dir(),
    );
}

// --- REQ-152: list の停止 ---

// @kotowari[REQ-152, REQ-004]
#[test]
fn req_152_tool_on_list_is_an_argument_error() {
    assert_argument_error(&["list", "--tool", "cargo-mutants"], valid_project_dir());
}

// @kotowari[REQ-152, REQ-004]
#[test]
fn req_152_positional_after_list_is_an_argument_error() {
    assert_argument_error(&["list", "extra"], valid_project_dir());
}

// @kotowari[REQ-002]
#[test]
fn req_002_list_options_can_come_before_or_after_the_command() {
    // オプションが list の前でも後でも受ける
    cmd()
        .args(["--format", "json", "list"])
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
    cmd()
        .args(["list", "--format", "json"])
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
}

// @kotowari[REQ-152, EX-249]
#[test]
fn req_152_unreadable_config_stops_like_check() {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    // YAML として読めない設定
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: [unclosed\n").unwrap();

    let first_line_of = |command: &str| {
        let output = cmd()
            .arg(command)
            .current_dir(tmp.path())
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{command} should stop with exit code 2"
        );
        String::from_utf8_lossy(&output.stderr)
            .lines()
            .next()
            .unwrap_or("")
            .to_string()
    };
    let check_line = first_line_of("check");
    let list_line = first_line_of("list");
    assert!(
        check_line.starts_with("config error: "),
        "check should stop with a config error, got: {check_line:?}"
    );
    assert_eq!(
        list_line, check_line,
        "list should stop with the same wording as check"
    );
}

// --- REQ-008: 作らないコマンド ---

// @kotowari[REQ-008]
#[test]
fn req_008_render_is_an_argument_error() {
    // "render" はコマンドにならない
    assert_argument_error(&["render"], valid_project_dir());
}

// --- REQ-157、REQ-163、REQ-004: query と status の引数 ---

// @kotowari[REQ-157, EX-252]
#[test]
fn req_157_two_positional_arguments_stop() {
    assert_argument_error(&["query", "REQ-001", "REQ-002"], valid_project_dir());
}

// @kotowari[REQ-157]
#[test]
fn req_157_query_with_no_positional_stops() {
    assert_argument_error(&["query"], valid_project_dir());
}

// @kotowari[REQ-157, REQ-124]
#[test]
fn req_157_positional_that_is_not_an_id_stops() {
    // ID の形でない語と、数字が足りない ID
    assert_argument_error(&["query", "foo"], valid_project_dir());
    assert_argument_error(&["query", "REQ-01"], valid_project_dir());
}

// @kotowari[REQ-004]
#[test]
fn req_004_positional_after_status_stops() {
    assert_argument_error(&["status", "extra"], valid_project_dir());
}

// @kotowari[REQ-004]
#[test]
fn req_004_tool_on_query_or_status_stops() {
    assert_argument_error(
        &["query", "--tool", "cargo-mutants", "REQ-001"],
        valid_project_dir(),
    );
    assert_argument_error(&["status", "--tool", "cargo-mutants"], valid_project_dir());
}

// @kotowari[REQ-002]
#[test]
fn req_002_query_and_status_options_before_or_after_the_command() {
    // オプションが query と status の前でも後でも受ける
    let project = dir_with_one_requirement();
    for args in [
        vec!["--format", "json", "query", "REQ-001"],
        vec!["query", "REQ-001", "--format", "json"],
        vec!["--format", "json", "status"],
        vec!["status", "--format", "json"],
    ] {
        cmd()
            .args(&args)
            .current_dir(project.path())
            .assert()
            .code(0);
    }
}

// @kotowari[REQ-158, EX-256]
#[test]
fn req_158_unreadable_config_stops_like_check() {
    assert_stops_like_check(&["query", "REQ-001"]);
}

// @kotowari[REQ-163, EX-262]
#[test]
fn req_163_unreadable_config_stops_like_check() {
    assert_stops_like_check(&["status"]);
}

/// YAML として読めない設定で、そのコマンドが "kotowari check" と同じ1行目で停止することを見る
fn assert_stops_like_check(args: &[&str]) {
    use tempfile::TempDir;
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: [unclosed\n").unwrap();

    let first_line_of = |args: &[&str]| {
        let output = cmd().args(args).current_dir(tmp.path()).output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "{args:?} should stop with exit code 2"
        );
        String::from_utf8_lossy(&output.stderr)
            .lines()
            .next()
            .unwrap_or("")
            .to_string()
    };
    let check_line = first_line_of(&["check"]);
    assert!(
        check_line.starts_with("config error: "),
        "check should stop with a config error, got: {check_line:?}"
    );
    assert_eq!(
        first_line_of(args),
        check_line,
        "{args:?} should stop with the same wording as check"
    );
}

/// 項目が1つあり、check の指摘が0件の置き場を作る
fn dir_with_one_requirement() -> tempfile::TempDir {
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
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("docs/decision/records/records.md"),
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    )
    .unwrap();
    // 検証が review で確かめ方のある要求なら、テストが無くても check の指摘は出ない
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# 題名\n\n範囲。\n\n## 要求\n\n### REQ-001: 例\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n- 検証: review\n- 確かめ方: 人が読む\n\n文である。\n",
    )
    .unwrap();
    tmp
}
