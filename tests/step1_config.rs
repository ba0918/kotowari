#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn valid_project_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/valid-project")
        .leak()
}

/// テスト用のプロジェクトを一時ディレクトリに作る
fn make_project(tmp: &Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/decisions")).unwrap();
    fs::write(tmp.join(".kotowari/config.yaml"), "").unwrap();
}

// --- UTF-8 でない設定で停止 ---

// @kotowari[C-core-clienv-7]
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

// --- 基準のディレクトリの決め方 ---

// @kotowari[C-core-base-1]
#[test]
fn req_009_base_is_the_dir_holding_dot_kotowari() {
    let tmp = TempDir::new().unwrap();
    // /tmp/xxx/.kotowari/ と /tmp/xxx/docs/decisions/ を作り、
    // /tmp/xxx/sub/ から起動する
    make_project(tmp.path());
    let sub = tmp.path().join("sub");
    fs::create_dir_all(&sub).unwrap();
    // sub/ にはサブコマンドが通る→上の .kotowari/ が基準になる
    cmd().arg("check").current_dir(&sub).assert().code(0);
}

// @kotowari[C-core-base-2]
#[test]
fn req_009_falls_back_to_cwd() {
    // .kotowari/ が上にもない → CWD が基準になる
    // CWD に決定の文書の置き場を直接作る
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    // .kotowari/ なし → CWD が基準 → 既定の設定ファイルも無い → 既定の値
    cmd().arg("check").current_dir(tmp.path()).assert().code(0);
}

// @kotowari[C-core-base-1]
#[test]
fn req_009_base_from_a_subdirectory_is_the_ancestor_with_dot_kotowari() {
    // 下のディレクトリで起動しても、指摘のパスが上のディレクトリからの相対になる
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let sub = tmp.path().join("src");
    fs::create_dir_all(&sub).unwrap();
    fs::write(tmp.path().join("docs/decisions/a.md"), "no title\n").unwrap();

    let output = cmd().arg("check").current_dir(&sub).output().unwrap();
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let paths: Vec<&str> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["path"].as_str().unwrap())
        .collect();
    assert_eq!(
        paths,
        vec!["docs/decisions/a.md"],
        "the base is the ancestor holding .kotowari, so the path is relative to it: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

// --- 設定の値は基準のディレクトリからの相対 ---

// @kotowari[C-core-base-5]
#[test]
fn req_010_config_values_are_relative_to_base() {
    let tmp = TempDir::new().unwrap();
    // base: /tmp/xxx/
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("my-decisions")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: my-decisions\n",
    )
    .unwrap();
    // サブディレクトリから起動しても基準で解決する
    let sub = tmp.path().join("sub");
    fs::create_dir_all(&sub).unwrap();
    cmd().arg("check").current_dir(&sub).assert().code(0);
}

// --- 設定のパスは基準を変えない ---

// @kotowari[C-core-base-4]
#[test]
fn prop_001_config_path_does_not_move_the_base() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 別の場所に設定ファイルを置く
    let other_dir = tmp.path().join("other");
    fs::create_dir_all(&other_dir).unwrap();
    fs::write(other_dir.join("my-config.yaml"), "").unwrap();
    // --config で別の場所の設定を指しても、基準は .kotowari/ のある場所
    cmd()
        .args(["check", "--config", "other/my-config.yaml"])
        .current_dir(tmp.path())
        .assert()
        .code(0);
}

// --- 設定のパスの基準 ---

// @kotowari[C-core-config-2]
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

// @kotowari[C-core-config-2, C-core-list-1, C-core-query-1]
#[test]
fn req_003_list_query_and_status_read_the_config_given_by_config() {
    // 既定の場所の設定は決定の文書の置き場を指さず、--config の設定だけが spec/ を指す。
    // 読んだ設定が渡した方なら、どのコマンドも spec/ の項目を読む
    let tmp = TempDir::new().unwrap();
    for dir in [".kotowari", "spec", "docs/decisions"] {
        fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: missing\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("other.yaml"),
        "decisions:\n  documents: spec\ntests:\n  files: []\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("spec/a.md"),
        "# A\n\nScope.\n\n## D\n\nStatus: implemented\n\n### Problem\n\nP.\n\n### Decision\n\nD.\n\n### Confirmations\n\n- C-a-1 (by reading) R\n",
    )
    .unwrap();
    for (args, read) in [
        (
            &["list", "--format", "text"][..],
            "C-a-1 reading R spec/a.md:19",
        ),
        (
            &["query", "C-a-1", "--format", "text"],
            "C-a-1 reading R spec/a.md:19",
        ),
        (
            &["status", "--format", "json"],
            "\"documents\":{\"files\":1,",
        ),
    ] {
        let output = cmd()
            .args(args)
            .args(["--config", "other.yaml"])
            .current_dir(tmp.path())
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_ne!(output.status.code(), Some(2), "{args:?}: {stderr}");
        assert!(stdout.contains(read), "{args:?}: {stdout}");
    }
}

// --- 既定の設定ファイルの場所 ---

// @kotowari[C-core-config-1]
#[test]
fn req_011_reads_dot_kotowari_config_by_default() {
    // 既に valid-project fixture にあるので、そのまま通る
    cmd()
        .arg("check")
        .current_dir(valid_project_dir())
        .assert()
        .code(0);
}

// --- 設定ファイルが無いときは既定の値 ---

// @kotowari[C-core-config-4]
#[test]
fn req_012_missing_config_uses_defaults() {
    let tmp = TempDir::new().unwrap();
    // .kotowari/ ディレクトリはあるが config.yaml はない
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    cmd().arg("check").current_dir(tmp.path()).assert().code(0);
}

// --- キーと既定の値 ---

// @kotowari[C-core-config-6]
#[test]
fn req_013_defaults_match_the_table() {
    let cfg = kotowari_core::config::Config::default();
    assert_eq!(cfg.decisions.documents, "docs/decisions");
    assert_eq!(cfg.tests.files, vec!["src/**/*.rs", "tests/**/*.rs"]);
    assert!(cfg.tests.rust.attributes.is_empty());
    assert!(cfg.tests.rust.macros.is_empty());
    assert_eq!(
        cfg.vague_words,
        vec!["適切に", "必要に応じて", "通常は", "など"]
    );
}

// --- 設定の誤り ---

// @kotowari[C-core-config-12]
#[test]
fn req_014_unknown_key_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join(".kotowari/config.yaml"), "limit: 50\n").unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_wrong_type_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files: 42\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[C-core-config-25]
#[test]
fn req_014_empty_vague_word_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "vague_words:\n  - \"\"\n  - 適切に\n",
    )
    .unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[C-core-config-10]
#[test]
#[cfg(unix)]
fn req_018_unreadable_dir_stops() {
    use std::os::unix::fs::PermissionsExt;
    if std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
    {
        return;
    }
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let place = tmp.path().join("docs/decisions");
    fs::set_permissions(&place, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    fs::set_permissions(&place, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "unreadable directory should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
}

// --- 一覧は既定を置き換える ---

// @kotowari[C-core-config-19]
#[test]
fn req_015_list_replaces_default() {
    let yaml = "tests:\n  files:\n    - \"my/**/*.rs\"\n";
    let cfg = kotowari_core::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg.tests.files, vec!["my/**/*.rs"]);
}

// --- 空の一覧 ---

// @kotowari[C-core-config-20]
#[test]
fn req_016_empty_list_means_none() {
    let yaml = "vague_words: []\n";
    let cfg = kotowari_core::config::Config::parse(yaml).unwrap();
    assert!(cfg.vague_words.is_empty());
}

// --- 入れ子のキー ---

// @kotowari[C-core-config-21]
#[test]
fn req_017_nested_keys() {
    let yaml =
        "decisions:\n  documents: my-decisions\ntests:\n  rust:\n    attributes: [kani::proof]\n";
    let cfg = kotowari_core::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg.decisions.documents, "my-decisions");
    assert_eq!(cfg.tests.rust.attributes, vec!["kani::proof"]);
}

// --- 置き場が無いとき停止 ---

// @kotowari[C-core-clienv-7, C-core-clienv-12, C-core-cli-14, C-core-config-10]
#[test]
fn req_018_missing_decisions_dir_stops_with_its_relative_path_and_the_os_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::remove_dir_all(tmp.path().join("docs/decisions")).unwrap();
    // OS の誤りの文は環境で変わるので、同じ置き場を読んだときの文をここで作る
    let os_error = fs::read_dir(tmp.path().join("docs/decisions")).unwrap_err();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // 詳細は設定の値のままの相対パスで、絶対パスを含まない
    let expected = format!("unreadable file: docs/decisions: {os_error}");
    assert_eq!(stderr.lines().next(), Some(expected.as_str()));
}

// --- Step 2: 設定の誤りの拡張、パスの正規化 ---

// @kotowari[C-core-config-12]
#[test]
fn req_014_unparsable_yaml_stops() {
    let yaml = "tests: [invalid yaml\n";
    let result = kotowari_core::config::Config::parse(yaml);
    assert!(result.is_err(), "unparsable YAML should stop");
}

// @kotowari[C-core-config-13]
#[test]
fn req_014_duplicate_key_stops() {
    let yaml = "vague_words: []\nvague_words: [など]\n";
    let result = kotowari_core::config::Config::parse(yaml);
    assert!(result.is_err(), "duplicate key should stop");
}

// @kotowari[C-core-config-13]
#[test]
fn ex_core_383_a_duplicate_key_stops_with_one_line_naming_the_key() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "vague_words: []\nvague_words: []\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("config error: "), "{stderr}");
    assert!(stderr.contains("duplicate key: vague_words"), "{stderr}");
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_value_stops() {
    // "vague_words:" だけの行は null
    let yaml = "vague_words:\n";
    let result = kotowari_core::config::Config::parse(yaml);
    assert!(result.is_err(), "null value should stop");
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_tests_files_stops() {
    let yaml = "tests:\n  files:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null tests.files should stop"
    );
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_tests_rust_attributes_stops() {
    let yaml = "tests:\n  rust:\n    attributes:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null tests.rust.attributes should stop"
    );
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_tests_rust_macros_stops() {
    let yaml = "tests:\n  rust:\n    macros:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null tests.rust.macros should stop"
    );
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_vague_words_key_stops() {
    let yaml = "vague_words:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null vague_words should stop"
    );
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_decisions_stops() {
    let yaml = "decisions:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null decisions should stop"
    );
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_tests_stops() {
    let yaml = "tests:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null tests should stop"
    );
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_tests_rust_stops() {
    let yaml = "tests:\n  rust:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null tests.rust should stop"
    );
}

// @kotowari[C-core-config-26]
#[test]
fn req_014_windows_drive_letter_like_path_is_not_absolute() {
    // 絶対パスの判定は先頭の "/" だけ。"a:b" をドライブ文字として拒まない
    let yaml = "decisions:\n  documents: a:b\n";
    let result = kotowari_core::config::Config::parse(yaml);
    assert!(
        result.is_ok(),
        "a value like 'a:b' must not be rejected as an absolute path: {:?}",
        result.err()
    );
}

// @kotowari[C-core-config-15]
#[test]
fn req_014_absolute_path_stops() {
    let yaml = "decisions:\n  documents: /absolute/path\n";
    let result = kotowari_core::config::Config::parse(yaml);
    assert!(result.is_err(), "absolute path should stop");
}

// @kotowari[C-core-config-17]
#[test]
fn req_014_duplicate_vague_word_stops() {
    let yaml = "vague_words:\n  - \"foo\"\n  - \"foo\"\n";
    let result = kotowari_core::config::Config::parse(yaml);
    assert!(result.is_err(), "duplicate vague word should stop");
}

// @kotowari[C-core-config-16]
#[test]
fn req_014_invalid_glob_stops() {
    let yaml = "tests:\n  files:\n    - \"[invalid\"\n";
    let result = kotowari_core::config::Config::parse(yaml);
    assert!(result.is_err(), "invalid glob should stop");
}

// @kotowari[C-core-config-7]
#[test]
fn req_014_overview_files_is_read_as_a_glob_list() {
    let cfg = kotowari_core::config::Config::parse(
        "overview:\n  files:\n    - \".kotowari/overview/*.md\"\n  toc: .kotowari/overview/toc.yaml\n",
    )
    .expect("overview.files should parse");
    assert_eq!(
        cfg.overview.map(|overview| overview.files),
        Some(vec![".kotowari/overview/*.md".to_string()])
    );
}

// @kotowari[C-core-config-7]
#[test]
fn tbl_core_004_overview_toc_is_read_as_one_path() {
    let cfg = kotowari_core::config::Config::parse(
        "overview:\n  files: [\"o/*.md\"]\n  toc: o/toc.yaml\n",
    )
    .expect("overview.toc should parse");
    assert_eq!(
        cfg.overview.map(|overview| overview.toc),
        Some("o/toc.yaml".into())
    );
}

// @kotowari[C-core-config-18]
#[test]
fn req_014_overview_without_toc_stops_with_a_config_error() {
    let result = kotowari_core::config::Config::parse("overview:\n  files: [\"o/*.md\"]\n");
    match result {
        Err(kotowari_core::StopReason::ConfigError(detail)) => {
            assert!(detail.contains("overview.toc"), "{detail}");
        }
        other => panic!("expected a config error, got {other:?}"),
    }
}

// @kotowari[C-core-config-14, C-core-config-15]
#[test]
fn req_014_a_null_absolute_or_non_string_overview_toc_stops() {
    for toc in ["", " /abs/toc.yaml", " [a, b]", " {a: 1}"] {
        let yaml = format!("overview:\n  files: [\"o/*.md\"]\n  toc:{toc}\n");
        assert!(
            matches!(
                kotowari_core::config::Config::parse(&yaml),
                Err(kotowari_core::StopReason::ConfigError(_))
            ),
            "{yaml:?} should stop"
        );
    }
}

// @kotowari[C-core-config-6]
#[test]
fn tbl_core_004_without_the_overview_key_no_overview_data_is_configured() {
    assert_eq!(kotowari_core::config::Config::default().overview, None);
    let cfg = kotowari_core::config::Config::parse("tests:\n  files: []\n").unwrap();
    assert_eq!(cfg.overview, None);
}

// @kotowari[C-core-config-16]
#[test]
fn req_014_invalid_overview_glob_stops() {
    let result = kotowari_core::config::Config::parse(
        "overview:\n  files:\n    - \"[invalid\"\n  toc: toc.yaml\n",
    );
    assert!(
        result.is_err(),
        "invalid glob in overview.files should stop"
    );
}

// @kotowari[C-core-config-18, C-core-config-14]
#[test]
fn req_014_overview_without_files_or_with_null_stops() {
    for yaml in [
        "overview:\n",
        "overview: {}\n",
        "overview:\n  files:\n",
        "overview:\n  files: []\n  toc: toc.yaml\n  extra: 1\n",
        "overview:\n  toc: toc.yaml\n",
    ] {
        assert!(
            kotowari_core::config::Config::parse(yaml).is_err(),
            "{yaml:?} should stop"
        );
    }
}

// @kotowari[C-core-config-16]
#[test]
fn req_014_an_invalid_overview_glob_stops_check_with_a_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "overview:\n  files:\n    - \"[invalid\"\n  toc: toc.yaml\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("config error: "));
}

// @kotowari[C-core-config-5]
#[test]
fn req_012_empty_config_uses_defaults() {
    let yaml = "";
    let cfg = kotowari_core::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg, kotowari_core::config::Config::default());
}

// @kotowari[C-core-config-5]
#[test]
fn req_012_comment_only_config_uses_defaults() {
    let yaml = "# comment only\n";
    let cfg = kotowari_core::config::Config::parse(yaml).unwrap();
    assert_eq!(cfg, kotowari_core::config::Config::default());
}

// @kotowari[C-core-base-7]
#[test]
fn req_110_trailing_slash_in_config_is_normalized_in_path() {
    assert_eq!(kotowari_core::normalize_path("docs/ir/"), "docs/ir");
    assert_eq!(kotowari_core::normalize_path("./docs/ir/"), "docs/ir");
}

// @kotowari[C-core-base-7]
#[test]
fn req_110_dot_segments_are_folded() {
    assert_eq!(kotowari_core::normalize_path("./docs/./ir"), "docs/ir");
    assert_eq!(kotowari_core::normalize_path("docs//ir"), "docs/ir");
    assert_eq!(kotowari_core::normalize_path("docs\\ir"), "docs/ir");
}

// @kotowari[C-core-base-7]
#[test]
fn req_110_dot_alone_normalizes_to_empty_place() {
    // "." や "./" だけの置き場は空になる（呼び出し元が文書名だけの path を作る）
    assert_eq!(kotowari_core::normalize_path("."), "");
    assert_eq!(kotowari_core::normalize_path("./"), "");
}

// @kotowari[C-core-base-6]
#[test]
fn req_110_ir_dot_produces_bare_filename_path() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: .\n",
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
        "documents: '.' should produce a bare filename path, not './a.md'"
    );
}

// @kotowari[C-core-config-10]
#[test]
fn req_018_place_that_is_a_file_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 決定の文書の置き場をファイルに替える
    fs::remove_dir_all(tmp.path().join("docs/decisions")).unwrap();
    fs::write(tmp.path().join("docs/decisions"), "not a directory").unwrap();
    cmd()
        .arg("check")
        .current_dir(tmp.path())
        .assert()
        .code(2)
        .stdout("");
}

// @kotowari[C-core-base-3]
#[test]
fn tbl_003_kotowari_file_is_ignored_in_search() {
    let tmp = TempDir::new().unwrap();
    // .kotowari をファイルとして作成し、親ディレクトリに .kotowari/ を作る
    let child = tmp.path().join("child");
    fs::create_dir_all(&child).unwrap();
    fs::write(child.join(".kotowari"), "this is a file, not a dir").unwrap();
    // 親に .kotowari/ ディレクトリを作る
    fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    fs::write(tmp.path().join(".kotowari/config.yaml"), "").unwrap();
    // child から走らせると、child/.kotowari はファイルなので無視して
    // 親の .kotowari/ を見つけるべき
    cmd().arg("check").current_dir(&child).assert().code(0);
}

// @kotowari[C-core-clienv-5, C-core-clienv-12, C-core-clienv-7]
#[test]
#[cfg(unix)]
fn req_018_unreadable_subdirectory_of_the_documents_place_stops() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let dir = tmp.path().join("docs/decisions/sub/deep");
    fs::create_dir_all(&dir).unwrap();
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o000)).unwrap();
    let os_error = fs::read_dir(&dir).unwrap_err().to_string();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains(&format!(
        "unreadable file: docs/decisions/sub/deep: {os_error}"
    )));
}

// @kotowari[C-core-base-6, C-core-base-7, C-core-output-7]
#[test]
fn req_110_path_carries_the_subdirectory() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: ./docs//decisions/./\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("docs/decisions/sub/deep")).unwrap();
    fs::write(tmp.path().join("docs/decisions/sub/deep/a.md"), "").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["decisions"]["documents"], 1);
    assert_eq!(
        result["findings"][0]["path"],
        "docs/decisions/sub/deep/a.md"
    );
}

// --- 直下の kotowari.toml を読まない ---

// @kotowari[C-core-config-3]
#[test]
fn req_020_kotowari_toml_beside_the_base_is_not_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let before = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        before.status.code(),
        Some(0),
        "the base project should check cleanly"
    );

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

    // 設定ファイルが無く既定に落ちる配置でも、kotowari.toml は既定の代わりに読まれない
    fs::remove_file(tmp.path().join(".kotowari/config.yaml")).unwrap();
    fs::remove_file(tmp.path().join("kotowari.toml")).unwrap();
    let default_before = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        default_before.status.code(),
        Some(0),
        "defaults should check cleanly"
    );
    fs::write(tmp.path().join("kotowari.toml"), "ir = \"elsewhere\"\n").unwrap();
    let default_after = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        default_after.status.code(),
        Some(0),
        "kotowari.toml should not replace the defaults either: {}",
        String::from_utf8_lossy(&default_after.stderr)
    );
    assert_eq!(default_after.stdout, default_before.stdout);
}

// --- スキーマのファイルを読まない ---

// @kotowari[C-core-formc-1]
#[test]
fn req_090_schema_file_is_not_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let before = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        before.status.code(),
        Some(0),
        "the base project should check cleanly"
    );

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

// --- 外部の mdschema を使わない ---

// @kotowari[C-core-formc-2]
#[test]
fn req_091_check_runs_the_same_with_an_empty_path() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
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
    assert_eq!(
        restricted.stdout, normal.stdout,
        "stdout should not depend on PATH"
    );
    assert_eq!(
        restricted.stderr, normal.stderr,
        "stderr should not depend on PATH"
    );
}

// --- 状態を保存しない ---

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

// @kotowari[C-core-scope-1]
#[test]
fn req_102_check_writes_nothing_under_home_tmpdir_or_base() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
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

// --- キーと既定の値 ---

/// 表の鍵をすべて書いた設定（"overview" と "labels" は組み合わせの条件があるので除く）
const ALL_KEYS: &str = "decisions:
  documents: docs/decisions
tests:
  files:
    - \"tests/**/*.rs\"
  rust:
    attributes: []
    macros: []
  rules: []
mutants:
  equivalents: docs/equivalents.yaml
vague_words:
  - \"適切に\"
";

// @kotowari[C-core-config-12]
#[test]
fn req_014_only_the_config_keys_of_the_table_are_accepted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());

    // 表の鍵を書いた設定は通る
    fs::write(tmp.path().join(".kotowari/config.yaml"), ALL_KEYS).unwrap();
    let accepted = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        accepted.status.code(),
        Some(0),
        "the keys of the table should be accepted: {}",
        String::from_utf8_lossy(&accepted.stderr)
    );

    // 表に無い鍵は、どの階層に書いても設定の誤りで停止する
    let variants = [
        (
            "top level",
            ALL_KEYS.replace(
                "decisions:\n",
                "queries:\n  python: python.scm\ndecisions:\n",
            ),
        ),
        (
            "decisions",
            ALL_KEYS.replace(
                "  documents: docs/decisions\n",
                "  documents: docs/decisions\n  queries: python.scm\n",
            ),
        ),
        (
            "tests",
            ALL_KEYS.replace("  rust:\n", "  queries:\n    python: python.scm\n  rust:\n"),
        ),
        (
            "tests.rust",
            ALL_KEYS.replace(
                "    macros: []\n",
                "    macros: []\n    queries: python.scm\n",
            ),
        ),
        (
            "mutants",
            ALL_KEYS.replace(
                "  equivalents: docs/equivalents.yaml\n",
                "  equivalents: docs/equivalents.yaml\n  queries: 3\n",
            ),
        ),
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

// --- 同梱の問い合わせは外せない ---

// @kotowari[C-core-qrules-2]
#[test]
fn req_121_bundled_query_stays_when_rules_are_added() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // Rust の同梱の "#[test]" と同じ言語に、別のものに当たるルールを足す
    fs::create_dir_all(tmp.path().join("rules")).unwrap();
    fs::write(
        tmp.path().join("rules/bench.yml"),
        "id: bench\nlanguage: rust\nrule:\n  kind: function_item\n  has:\n    field: name\n    regex: ^bench_\n    pattern: $NAME\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.rs\"\n  rules:\n    - rules/bench.yml\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/a.rs"),
        "#[test]\nfn t() {}\n\nfn bench_x() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    let details: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == "test_without_id")
        .map(|f| f["detail"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(details, vec!["t", "bench_x"], "{v}");
}

// @kotowari[C-core-config-6, C-core-config-7, C-core-qrules-3]
#[test]
fn tbl_004_tests_rules_is_read_and_defaults_to_empty() {
    let cfg = kotowari_core::config::Config::parse("tests:\n  rules:\n    - ./rules/a.yml\n")
        .expect("the key should be read");
    // 設定の値のパスは正規化する
    assert_eq!(cfg.tests.rules, vec!["rules/a.yml".to_string()]);
    assert!(
        kotowari_core::config::Config::default()
            .tests
            .rules
            .is_empty()
    );
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_null_tests_rules_stops() {
    let yaml = "tests:\n  rules:\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "null tests.rules should stop"
    );
}

// @kotowari[C-core-config-15]
#[test]
fn req_014_absolute_tests_rules_stops() {
    let yaml = "tests:\n  rules:\n    - /abs/rule.yml\n";
    assert!(
        kotowari_core::config::Config::parse(yaml).is_err(),
        "an absolute path should stop"
    );
}

// --- 等価の一覧を指す鍵 ---

// @kotowari[C-core-config-6, C-core-config-7]
#[test]
fn tbl_004_mutants_equivalents_is_read_and_has_no_default() {
    let cfg =
        kotowari_core::config::Config::parse("mutants:\n  equivalents: ./docs/equivalents.yaml\n")
            .expect("the key should be read");
    // 設定の値のパスは正規化する
    assert_eq!(
        cfg.mutants.equivalents.as_deref(),
        Some("docs/equivalents.yaml")
    );
    // 既定は無い（鍵が無ければ等価の一覧は0件）
    assert_eq!(
        kotowari_core::config::Config::default().mutants.equivalents,
        None
    );
}

// @kotowari[C-core-config-15]
#[test]
fn req_014_absolute_mutants_equivalents_stops() {
    let result =
        kotowari_core::config::Config::parse("mutants:\n  equivalents: /abs/equivalents.yaml\n");
    assert!(result.is_err(), "an absolute path should stop");
}

// @kotowari[C-core-config-14]
#[test]
fn req_014_mutants_equivalents_that_is_not_a_string_stops() {
    let result = kotowari_core::config::Config::parse("mutants:\n  equivalents:\n    - a\n");
    assert!(result.is_err(), "a value that is not a string should stop");
}

// --- 言語の一覧と UI の文字の誤り ---

/// 設定の本文を書いて check を走らせる
fn check_with_config(config: &str) -> std::process::Output {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join(".kotowari/config.yaml"), config).unwrap();
    cmd().arg("check").current_dir(tmp.path()).output().unwrap()
}

/// 直した設定では止まらず、誤った設定では設定の誤りで止まることを確かめる
fn assert_config_error(valid: &str, invalid: &str) {
    let output = check_with_config(valid);
    assert_eq!(output.status.code(), Some(0), "{valid}: {output:?}");
    let output = check_with_config(invalid);
    assert_eq!(output.status.code(), Some(2), "{invalid}");
    assert!(output.stdout.is_empty(), "{invalid}");
    assert!(
        String::from_utf8_lossy(&output.stderr).starts_with("config error: "),
        "{invalid}"
    );
}

/// "labels.<tag>" の17の鍵のうち、skip を除いたもの
fn labels_without(tag: &str, skip: &str) -> String {
    let keys = [
        ("language_name", "日本語"),
        ("index_link", "一覧"),
        ("pages", "{n} ページ"),
        ("stale_sections", "見直していない節 {n}"),
        ("open_items", "未決 {n}"),
        ("planned_items", "予定 {n}"),
        ("stale_mark", "見直していない節"),
        ("outline_stale", "見直していない"),
        ("superseded", "（置き換え済み）"),
        ("deferred", "（後回し）"),
        ("compare_before", "前"),
        ("compare_after", "後"),
        ("compare_why", "理由"),
        ("state_decided", "決定"),
        ("state_planned", "予定"),
        ("state_open", "未決"),
        ("state_dropped", "取り下げ"),
    ];
    let mut out = format!("labels:\n  {tag}:\n");
    for (key, value) in keys {
        if key != skip {
            out.push_str(&format!("    {key}: \"{value}\"\n"));
        }
    }
    out
}

// @kotowari[C-core-pairs-3]
#[test]
fn ex_core_513_a_language_tag_with_an_upper_case_letter_stops() {
    assert_config_error(
        &format!("languages: [ja, en]\n{}", labels_without("ja", "")),
        &format!("languages: [ja, EN]\n{}", labels_without("ja", "")),
    );
}

// @kotowari[C-core-ovlang-8]
#[test]
fn ex_core_537_labels_of_a_language_other_than_english_missing_a_key_stops() {
    assert_config_error(
        &format!("languages: [ja]\n{}", labels_without("ja", "")),
        &format!("languages: [ja]\n{}", labels_without("ja", "stale_mark")),
    );
}

// @kotowari[C-core-ovlang-10]
#[test]
fn ex_core_538_a_counted_text_without_the_number_placeholder_stops() {
    assert_config_error(
        "labels:\n  en:\n    pages: \"{n} pages\"\n",
        "labels:\n  en:\n    pages: \"pages\"\n",
    );
}

// @kotowari[C-core-ovlang-12]
#[test]
fn ex_core_539_labels_of_a_language_not_in_the_list_stops() {
    assert_config_error(
        &format!("languages: [ja]\n{}", labels_without("ja", "")),
        &labels_without("ja", ""),
    );
}

// @kotowari[C-core-config-15]
#[test]
fn ex_core_544_a_glob_element_starting_with_a_slash_stops_as_a_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files: ['/tests/**/*.rs']\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("config error: "), "{stderr}");
}

// @kotowari[C-core-config-15]
#[test]
fn req_014_every_files_key_rejects_a_glob_element_starting_with_a_slash() {
    for (key, yaml) in [
        ("tests.files", "tests:\n  files: ['/a/*.rs']\n"),
        ("guides.files", "guides:\n  files: ['/a/*.md']\n"),
        (
            "surface.files",
            "surface:\n  files: ['/a/*.rs']\n  rules: [rules]\n",
        ),
        (
            "overview.files",
            "overview:\n  files: ['/a/*.md']\n  toc: toc.yaml\n",
        ),
    ] {
        let result = kotowari_core::config::Config::parse(yaml);
        match result {
            Err(kotowari_core::StopReason::ConfigError(detail)) => assert!(
                detail.contains(&format!("absolute path not allowed for {key}: /a/")),
                "{key}: {detail}"
            ),
            other => panic!("{key}: {other:?}"),
        }
    }
}

// --- 決定の文書の置き場の鍵と、無くした古い形の鍵 ---

/// ".kotowari/" と既定の決定の文書の置き場 "docs/decisions" だけを持つプロジェクト
fn make_decisions_project(tmp: &Path, config: &str) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/decisions")).unwrap();
    fs::write(tmp.join(".kotowari/config.yaml"), config).unwrap();
}

// @kotowari[C-core-config-8]
#[test]
fn without_the_documents_key_check_reads_the_decision_documents_under_docs_decisions() {
    let tmp = TempDir::new().unwrap();
    make_decisions_project(tmp.path(), "tests:\n  files: []\n");
    fs::write(tmp.path().join("docs/decisions/a.md"), "No title\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(v["findings"][0]["kind"], "missing_title", "{v}");
    assert_eq!(v["findings"][0]["path"], "docs/decisions/a.md", "{v}");
}

// @kotowari[C-core-config-9]
#[test]
fn the_location_keys_of_the_old_form_stop_with_a_config_error() {
    for key in [
        "ir: docs/ir\n",
        "decisions:\n  records: docs/decision/records\n",
        "decisions:\n  adr: docs/decision/adr\n",
    ] {
        let tmp = TempDir::new().unwrap();
        make_decisions_project(tmp.path(), key);
        let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
        assert_eq!(output.status.code(), Some(2), "{key}: {output:?}");
        assert!(output.stdout.is_empty(), "{key}");
        assert!(
            String::from_utf8_lossy(&output.stderr).starts_with("config error: "),
            "{key}: {output:?}"
        );
    }
}

// @kotowari[C-core-config-27]
#[test]
fn the_limit_keys_stop_with_a_config_error() {
    for key in ["limits:\n  lines: 5\n", "limits:\n  requirements: 7\n"] {
        let tmp = TempDir::new().unwrap();
        make_decisions_project(tmp.path(), key);
        let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
        assert_eq!(output.status.code(), Some(2), "{key}: {output:?}");
        assert!(output.stdout.is_empty(), "{key}");
        assert!(
            String::from_utf8_lossy(&output.stderr).starts_with("config error: "),
            "{key}: {output:?}"
        );
    }
}

// @kotowari[C-core-base-8]
#[test]
fn a_parent_step_in_the_documents_place_is_folded_and_one_with_nothing_to_fold_is_kept() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path().join("base");
    make_decisions_project(&base, "decisions:\n  documents: docs/x/../decisions\n");
    fs::write(base.join("docs/decisions/a.md"), "No title\n").unwrap();
    let output = cmd().arg("check").current_dir(&base).output().unwrap();
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(v["findings"][0]["path"], "docs/decisions/a.md", "{v}");

    fs::create_dir_all(tmp.path().join("out")).unwrap();
    fs::write(tmp.path().join("out/a.md"), "No title\n").unwrap();
    fs::write(
        base.join(".kotowari/config.yaml"),
        "decisions:\n  documents: ../out\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(&base).output().unwrap();
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(v["findings"][0]["path"], "../out/a.md", "{v}");
}

// @kotowari[C-core-input-1]
#[test]
fn req_111_bom_in_config_records_adr_and_tests_is_skipped_end_to_end() {
    let tmp = TempDir::new().unwrap();
    let bom = "\u{feff}";
    make_decisions_project(
        tmp.path(),
        &format!("{bom}decisions:\n  documents: docs/decisions\n"),
    );
    fs::write(
        tmp.path().join("docs/decisions/a.md"),
        format!("{bom}# Title\n\nScope.\n\n## Greet\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nGreet.\n\n### Confirmations\n\n- C-a-1 Shown\n"),
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/t.rs"),
        format!("{bom}// @kotowari[C-a-1]\n#[test]\nfn t() {{}}\n"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "BOM in every file kind must be skipped: {output:?}"
    );
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(v["findings"].as_array().unwrap().is_empty(), "{v}");
}
