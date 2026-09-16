use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn valid_project_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/valid-project").leak()
}

/// テスト用のプロジェクトを一時ディレクトリに作る
fn make_project(tmp: &Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
}

// --- REQ-006: UTF-8 でない設定で停止 ---

// @kotowari[REQ-006, TBL-001]
#[test]
fn req_006_non_utf8_config_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 非 UTF-8 のバイト列を設定ファイルに書く
    fs::write(tmp.path().join(".kotowari/config.yaml"), b"\xff\xfe").unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// --- REQ-009: 基準のディレクトリの決め方 ---

// @kotowari[REQ-009, TBL-003]
#[test]
fn req_009_base_is_the_dir_holding_dot_kotowari() {
    let tmp = TempDir::new().unwrap();
    // /tmp/xxx/.kotowari/ と /tmp/xxx/docs/ir/ を作り、
    // /tmp/xxx/sub/ から起動する
    make_project(tmp.path());
    let sub = tmp.path().join("sub");
    fs::create_dir_all(&sub).unwrap();
    // sub/ にはサブコマンドが通る→上の .kotowari/ が基準になる
    cmd()
        .arg("check")
        .current_dir(&sub)
        .assert()
        .code(0);
}

// @kotowari[REQ-009, TBL-003]
#[test]
fn req_009_falls_back_to_cwd() {
    // .kotowari/ が上にもない → CWD が基準になる
    // CWD に docs/ir 等を直接作る
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    // .kotowari/ なし → CWD が基準 → 既定の設定ファイルも無い → REQ-012 で既定値
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(0);
}

// --- REQ-010: 設定の値は基準のディレクトリからの相対 ---

// @kotowari[REQ-010]
#[test]
fn req_010_config_values_are_relative_to_base() {
    let tmp = TempDir::new().unwrap();
    // base: /tmp/xxx/
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("my-ir")).unwrap();
    fs::create_dir_all(tmp.path().join("my-records")).unwrap();
    fs::create_dir_all(tmp.path().join("my-adr")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: my-ir\ndecisions:\n  records: my-records\n  adr: my-adr\n",
    )
    .unwrap();
    // サブディレクトリから起動しても基準で解決する
    let sub = tmp.path().join("sub");
    fs::create_dir_all(&sub).unwrap();
    cmd()
        .arg("check")
        .current_dir(&sub)
        .assert()
        .code(0);
}

// --- PROP-001: 設定のパスは基準を変えない ---

// @kotowari[PROP-001]
#[test]
fn prop_001_config_path_does_not_move_the_base() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 別の場所に設定ファイルを置く
    let other_dir = tmp.path().join("other");
    fs::create_dir_all(&other_dir).unwrap();
    fs::write(
        other_dir.join("my-config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    // --config で別の場所の設定を指しても、基準は .kotowari/ のある場所
    cmd()
        .args(["check", "--config", "other/my-config.yaml"])
        .current_dir(tmp.path())
        .assert()
        .code(0);
}

// --- REQ-003: 設定のパスの基準 ---

// @kotowari[REQ-003]
#[test]
fn req_003_config_path_is_relative_to_cwd() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // CWD からの相対パスで設定ファイルを指す
    cmd()
        .args(["check", "--config", ".kotowari/config.yaml"])
        .current_dir(tmp.path())
        .assert()
        .code(0);
}

// --- REQ-011: 既定の設定ファイルの場所 ---

// @kotowari[REQ-011]
#[test]
fn req_011_reads_dot_kotowari_config_by_default() {
    // 既に valid-project fixture にあるので、そのまま通る
    cmd()
        .arg("check")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
}

// --- REQ-012: 設定ファイルが無いときは既定の値 ---

// @kotowari[REQ-012]
#[test]
fn req_012_missing_config_uses_defaults() {
    let tmp = TempDir::new().unwrap();
    // .kotowari/ ディレクトリはあるが config.yaml はない
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(0);
}

// --- REQ-013: キーと既定の値 ---

// @kotowari[REQ-013, TBL-004]
#[test]
fn req_013_defaults_match_the_table() {
    let cfg = kotowari::config::Config::default();
    assert_eq!(cfg.ir, "docs/ir");
    assert_eq!(cfg.decisions.records, "docs/decision/brainstorm");
    assert_eq!(cfg.decisions.adr, "docs/decision/adr");
    assert_eq!(cfg.tests.files, vec!["src/**/*.rs", "tests/**/*.rs"]);
    assert!(cfg.tests.rust.attributes.is_empty());
    assert!(cfg.tests.rust.macros.is_empty());
    assert_eq!(cfg.limits.lines.get(), 200);
    assert_eq!(cfg.limits.requirements.get(), 10);
    assert_eq!(
        cfg.vague_words,
        vec!["適切に", "必要に応じて", "通常は", "など"]
    );
}

// @kotowari[REQ-013, REQ-014]
#[test]
fn req_013_explicit_limits_values_are_used_as_given() {
    let yaml = "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\nlimits:\n  lines: 5\n  requirements: 7\n";
    let cfg = kotowari::config::Config::parse(yaml).expect("explicit limits should parse");
    assert_eq!(cfg.limits.lines.get(), 5, "lines should be the given value, not the default or a hardcoded one");
    assert_eq!(cfg.limits.requirements.get(), 7, "requirements should be the given value, not the default or a hardcoded one");
}

// --- REQ-014: 設定の誤り ---

// @kotowari[REQ-014]
#[test]
fn req_014_unknown_key_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\nlimit: 50\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-014]
#[test]
fn req_014_wrong_type_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: 42\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-014]
#[test]
fn req_014_negative_limit_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\nlimits:\n  lines: -5\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-014]
#[test]
fn req_014_zero_limit_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\nlimits:\n  lines: 0\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-014]
#[test]
fn req_014_empty_vague_word_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\nvague_words:\n  - \"\"\n  - 適切に\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-018]
#[test]
#[cfg(unix)]
fn req_018_unreadable_dir_stops() {
    use std::os::unix::fs::PermissionsExt;
    if std::process::Command::new("id").arg("-u").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
    {
        return;
    }
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let ir_dir = tmp.path().join("docs/ir");
    fs::set_permissions(&ir_dir, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    fs::set_permissions(&ir_dir, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "unreadable directory should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
}

// @kotowari[REQ-018]
#[test]
#[cfg(unix)]
fn req_018_unreadable_records_dir_stops() {
    use std::os::unix::fs::PermissionsExt;
    if std::process::Command::new("id").arg("-u").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
    {
        return;
    }
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let records_dir = tmp.path().join("docs/decision/brainstorm");
    fs::set_permissions(&records_dir, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    fs::set_permissions(&records_dir, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "unreadable records directory should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
}

// @kotowari[REQ-018]
#[test]
#[cfg(unix)]
fn req_018_unreadable_adr_dir_stops() {
    use std::os::unix::fs::PermissionsExt;
    if std::process::Command::new("id").arg("-u").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
    {
        return;
    }
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let adr_dir = tmp.path().join("docs/decision/adr");
    fs::set_permissions(&adr_dir, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    fs::set_permissions(&adr_dir, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "unreadable adr directory should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
}

// --- REQ-015: 一覧は既定を置き換える ---

// @kotowari[REQ-015]
#[test]
fn req_015_list_replaces_default() {
    let yaml = "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"my/**/*.rs\"\n";
    let cfg = kotowari::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg.tests.files, vec!["my/**/*.rs"]);
}

// --- REQ-016: 空の一覧 ---

// @kotowari[REQ-016]
#[test]
fn req_016_empty_list_means_none() {
    let yaml = "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\nvague_words: []\n";
    let cfg = kotowari::config::Config::parse(yaml).unwrap();
    assert!(cfg.vague_words.is_empty());
}

// --- REQ-017: 入れ子のキー ---

// @kotowari[REQ-017]
#[test]
fn req_017_nested_keys() {
    let yaml = "ir: docs/ir\ndecisions:\n  records: my-records\n  adr: my-adr\n";
    let cfg = kotowari::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg.decisions.records, "my-records");
    assert_eq!(cfg.decisions.adr, "my-adr");
}

// --- REQ-018: 置き場が無いとき停止 ---

// @kotowari[REQ-018, TBL-001]
#[test]
fn req_018_missing_ir_dir_stops() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    // docs/ir を作らない
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-005, REQ-018, TBL-018, TBL-020]
#[test]
fn req_005_stderr_carries_the_stop_reason_text() {
    // A137 で改めた: TBL-020 の形（相対パスと OS の誤りの文）
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::remove_dir_all(tmp.path().join("docs/ir")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    let first_line = stderr.lines().next().unwrap_or("");
    // TBL-018: 「unreadable file」の文言で始まる
    assert!(
        first_line.starts_with("unreadable file: "),
        "stderr should start with TBL-018 wording, got: {first_line:?}"
    );
    // TBL-020: 相対パスを含む（設定の ir の値）
    assert!(
        first_line.contains("docs/ir"),
        "stderr should contain the relative path, got: {first_line:?}"
    );
}

// @kotowari[REQ-018, TBL-001]
#[test]
fn req_018_missing_records_dir_stops() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    // docs/decision/brainstorm を作らない
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[REQ-018, TBL-001]
#[test]
fn req_018_missing_adr_dir_stops() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    // docs/decision/adr を作らない
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// --- REQ-019: glob の読み方 ---

// @kotowari[REQ-019]
#[test]
fn req_019_glob_is_recursive_and_skips_hidden_dirs() {
    // この機能はテストの発見（Step 4）で完全に検査するので、
    // ここでは設定の glob が受理されることだけ確かめる
    let yaml = "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"src/**/*.rs\"\n";
    let cfg = kotowari::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg.tests.files, vec!["src/**/*.rs"]);
}

// --- Step 2: 設定の誤りの拡張、パスの正規化 ---

// @kotowari[REQ-014]
#[test]
fn req_014_unparsable_yaml_stops() {
    let yaml = "ir: [invalid yaml\n";
    let result = kotowari::config::Config::parse(yaml);
    assert!(result.is_err(), "unparsable YAML should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_duplicate_key_stops() {
    let yaml = "ir: docs/ir\nir: other\n";
    let result = kotowari::config::Config::parse(yaml);
    assert!(result.is_err(), "duplicate key should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_value_stops() {
    // "ir:" だけの行は null
    let yaml = "ir:\n";
    let result = kotowari::config::Config::parse(yaml);
    assert!(result.is_err(), "null value should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_tests_files_stops() {
    let yaml = "tests:\n  files:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null tests.files should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_tests_rust_attributes_stops() {
    let yaml = "tests:\n  rust:\n    attributes:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null tests.rust.attributes should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_tests_rust_macros_stops() {
    let yaml = "tests:\n  rust:\n    macros:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null tests.rust.macros should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_vague_words_key_stops() {
    let yaml = "vague_words:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null vague_words should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_decisions_stops() {
    let yaml = "decisions:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null decisions should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_tests_stops() {
    let yaml = "tests:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null tests should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_tests_rust_stops() {
    let yaml = "tests:\n  rust:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null tests.rust should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_null_limits_stops() {
    let yaml = "limits:\n";
    assert!(kotowari::config::Config::parse(yaml).is_err(), "null limits should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_windows_drive_letter_like_path_is_not_absolute() {
    // A161: 絶対パスの判定は先頭の "/" だけ。"a:b" をドライブ文字として拒まない
    let yaml = "ir: a:b\n";
    let result = kotowari::config::Config::parse(yaml);
    assert!(
        result.is_ok(),
        "a value like 'a:b' must not be rejected as an absolute path: {:?}",
        result.err()
    );
}

// @kotowari[REQ-014]
#[test]
fn req_014_absolute_path_stops() {
    let yaml = "ir: /absolute/path\n";
    let result = kotowari::config::Config::parse(yaml);
    assert!(result.is_err(), "absolute path should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_duplicate_vague_word_stops() {
    let yaml = "vague_words:\n  - \"foo\"\n  - \"foo\"\n";
    let result = kotowari::config::Config::parse(yaml);
    assert!(result.is_err(), "duplicate vague word should stop");
}

// @kotowari[REQ-014]
#[test]
fn req_014_invalid_glob_stops() {
    let yaml = "tests:\n  files:\n    - \"[invalid\"\n";
    let result = kotowari::config::Config::parse(yaml);
    assert!(result.is_err(), "invalid glob should stop");
}

// @kotowari[REQ-012]
#[test]
fn req_012_empty_config_uses_defaults() {
    let yaml = "";
    let cfg = kotowari::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg.ir, "docs/ir");
}

// @kotowari[REQ-012]
#[test]
fn req_012_comment_only_config_uses_defaults() {
    let yaml = "# comment only\n";
    let cfg = kotowari::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg.ir, "docs/ir");
}

// @kotowari[REQ-110]
#[test]
fn req_110_trailing_slash_in_config_is_normalized_in_path() {
    assert_eq!(kotowari::normalize_path("docs/ir/"), "docs/ir");
    assert_eq!(kotowari::normalize_path("./docs/ir/"), "docs/ir");
}

// @kotowari[REQ-110]
#[test]
fn req_110_dot_segments_are_folded() {
    assert_eq!(kotowari::normalize_path("./docs/./ir"), "docs/ir");
    assert_eq!(kotowari::normalize_path("docs//ir"), "docs/ir");
    assert_eq!(kotowari::normalize_path("docs\\ir"), "docs/ir");
}

// @kotowari[REQ-110]
#[test]
fn req_110_dot_alone_normalizes_to_empty_place() {
    // "." や "./" だけの置き場は空になる（呼び出し元が文書名だけの path を作る）
    assert_eq!(kotowari::normalize_path("."), "");
    assert_eq!(kotowari::normalize_path("./"), "");
}

// @kotowari[REQ-110]
#[test]
fn req_110_ir_dot_produces_bare_filename_path() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: .\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    fs::write(tmp.path().join("a.md"), "No title\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let findings = v["findings"].as_array().unwrap();
    let mt = findings
        .iter()
        .find(|f| f["kind"] == "missing_title")
        .expect("missing_title should be reported");
    assert_eq!(
        mt["path"], "a.md",
        "ir: '.' should produce a bare filename path, not './a.md'"
    );
}

// @kotowari[REQ-018]
#[test]
fn req_018_place_that_is_a_file_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // ir をファイルに替える
    fs::remove_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::write(tmp.path().join("docs/ir"), "not a directory").unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[TBL-003]
#[test]
fn tbl_003_kotowari_file_is_ignored_in_search() {
    let tmp = TempDir::new().unwrap();
    // .kotowari をファイルとして作成し、親ディレクトリに .kotowari/ を作る
    let child = tmp.path().join("child");
    fs::create_dir_all(&child).unwrap();
    fs::write(child.join(".kotowari"), "this is a file, not a dir").unwrap();
    // 親に .kotowari/ ディレクトリを作る
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    ).unwrap();
    // child から走らせると、child/.kotowari はファイルなので無視して
    // 親の .kotowari/ を見つけるべき
    cmd()
        .arg("check")
        .current_dir(&child)
        .assert()
        .code(0);
}

// @kotowari[REQ-018, TBL-001, TBL-020]
#[test]
#[cfg(unix)]
fn req_018_unreadable_ir_subdirectory_stops() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let dir = tmp.path().join("docs/ir/sub/deep");
    fs::create_dir_all(&dir).unwrap();
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o000)).unwrap();
    let os_error = fs::read_dir(&dir).unwrap_err().to_string();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains(&format!("unreadable file: docs/ir/sub/deep: {os_error}")));
}

// @kotowari[REQ-110, TBL-006]
#[test]
fn req_110_path_carries_the_subdirectory() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: ./docs//ir/./\n").unwrap();
    fs::create_dir_all(tmp.path().join("docs/ir/sub/deep")).unwrap();
    fs::write(tmp.path().join("docs/ir/sub/deep/a.md"), "").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["files"], 1);
    assert_eq!(result["findings"][0]["path"], "docs/ir/sub/deep/a.md");
}

// --- REQ-020: 直下の kotowari.toml を読まない ---

// @kotowari[REQ-020]
#[test]
fn req_020_kotowari_toml_beside_the_base_is_not_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "# Title\n\nScope.\n").unwrap();
    let before = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(before.status.code(), Some(0), "the base project should check cleanly");

    // 読まれれば置き場が "elsewhere" になり、置き場が無いことを理由に停止する値
    fs::write(tmp.path().join("kotowari.toml"), "ir = \"elsewhere\"\n").unwrap();
    let after = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        after.status.code(),
        Some(0),
        "kotowari.toml should not be read as a configuration file: {}",
        String::from_utf8_lossy(&after.stderr)
    );
    assert_eq!(
        after.stdout, before.stdout,
        "the result should not change when kotowari.toml is present"
    );
}

// --- REQ-090: スキーマのファイルを読まない ---

// @kotowari[REQ-090]
#[test]
fn req_090_schema_file_is_not_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "# Title\n\nScope.\n").unwrap();
    let before = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(before.status.code(), Some(0), "the base project should check cleanly");

    // 読まれれば YAML として読めないことで停止する中身にする
    fs::write(
        tmp.path().join(".kotowari/schema.yaml"),
        "document: [unclosed\n  - : :\n",
    )
    .unwrap();
    let after = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        after.status.code(),
        Some(0),
        "a schema file should not be read: {}",
        String::from_utf8_lossy(&after.stderr)
    );
    assert_eq!(
        after.stdout, before.stdout,
        "the result should not change when a schema file is present"
    );
}

// --- REQ-091: 外部の mdschema を使わない ---

// @kotowari[REQ-091]
#[test]
fn req_091_check_runs_the_same_with_an_empty_path() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "# Title\n\nScope.\n").unwrap();
    let normal = cmd().arg("check").current_dir(tmp.path()).output().unwrap();

    // PATH を空のディレクトリだけに向ける。前段で外部のプロセスを起動していれば結果が変わる
    let empty = TempDir::new().unwrap();
    let restricted = cmd()
        .arg("check")
        .env("PATH", empty.path())
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(
        restricted.status.code(),
        normal.status.code(),
        "exit code should not depend on PATH: {}",
        String::from_utf8_lossy(&restricted.stderr)
    );
    assert_eq!(restricted.stdout, normal.stdout, "stdout should not depend on PATH");
    assert_eq!(restricted.stderr, normal.stderr, "stderr should not depend on PATH");
}

// --- REQ-102: 状態を保存しない ---

/// ディレクトリの全エントリを、相対パスと（ファイルなら）中身のバイト列で写し取る
fn snapshot(root: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, Option<Vec<u8>>)>) {
        let mut paths: Vec<std::path::PathBuf> = fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        paths.sort();
        for path in paths {
            let rel = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .to_string();
            if path.is_dir() {
                out.push((rel, None));
                walk(root, &path, out);
            } else {
                out.push((rel, Some(fs::read(&path).unwrap())));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out
}

// @kotowari[REQ-102]
#[test]
fn req_102_check_writes_nothing_under_home_tmpdir_or_base() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "# Title\n\nScope.\n").unwrap();
    let home = TempDir::new().unwrap();

    let home_before = snapshot(home.path());
    let base_before = snapshot(tmp.path());

    let output = cmd()
        .arg("check")
        .env("HOME", home.path())
        .env("TMPDIR", home.path())
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "check should run: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert_eq!(
        snapshot(home.path()),
        home_before,
        "nothing should be written under HOME or TMPDIR"
    );
    assert_eq!(
        snapshot(tmp.path()),
        base_before,
        "nothing should be written under the base directory"
    );
}

// --- REQ-121: 設定で問い合わせを足せない ---

/// TBL-004 の9個のキーをすべて書いた設定
const ALL_NINE_KEYS: &str = "ir: docs/ir
decisions:
  records: docs/decision/brainstorm
  adr: docs/decision/adr
tests:
  files:
    - \"tests/**/*.rs\"
  rust:
    attributes: []
    macros: []
limits:
  lines: 200
  requirements: 10
vague_words:
  - \"適切に\"
";

// @kotowari[REQ-121, REQ-014]
#[test]
fn req_121_only_the_nine_config_keys_are_accepted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());

    // TBL-004 の9個をすべて書いた設定は通る
    fs::write(tmp.path().join(".kotowari/config.yaml"), ALL_NINE_KEYS).unwrap();
    let accepted = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        accepted.status.code(),
        Some(0),
        "the nine keys of TBL-004 should be accepted: {}",
        String::from_utf8_lossy(&accepted.stderr)
    );

    // 問い合わせを足す鍵は、どの階層に書いても設定の誤りで停止する
    let variants = [
        ("top level", ALL_NINE_KEYS.replace("ir: docs/ir\n", "ir: docs/ir\nqueries:\n  python: python.scm\n")),
        ("decisions", ALL_NINE_KEYS.replace("  adr: docs/decision/adr\n", "  adr: docs/decision/adr\n  queries: python.scm\n")),
        ("tests", ALL_NINE_KEYS.replace("  rust:\n", "  queries:\n    python: python.scm\n  rust:\n")),
        ("tests.rust", ALL_NINE_KEYS.replace("    macros: []\n", "    macros: []\n    queries: python.scm\n")),
        ("limits", ALL_NINE_KEYS.replace("  requirements: 10\n", "  requirements: 10\n  queries: 3\n")),
    ];
    for (level, yaml) in variants {
        fs::write(tmp.path().join(".kotowari/config.yaml"), &yaml).unwrap();
        let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
        assert_eq!(
            output.status.code(),
            Some(2),
            "an unknown key at {level} should stop: {yaml}"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        let first = stderr.lines().next().unwrap_or("");
        assert!(
            first.starts_with("config error"),
            "an unknown key at {level} should stop as a config error, got: {first}"
        );
    }
}
