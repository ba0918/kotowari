//! 面の検査（docs/decisions/core/surface.md、surface-unspecified.md）と、その設定の鍵 "surface"
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。`surface` の行をそのまま設定に足す
fn make_project(tmp: &Path, surface: &str) {
    for dir in [".kotowari", "docs/decisions"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        format!(
            "decisions:\n  documents: docs/decisions\ntests:\n  files:\n    - \"tests/**/*.rs\"\n{surface}"
        ),
    )
    .unwrap();
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

// @kotowari[C-core-surface-9]
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

// @kotowari[C-core-surface-9]
#[test]
fn ex_core_416_files_without_rules_stop() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "surface:\n  files:\n    - \"src/**/*.rs\"\n");
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stdout}{stderr}");
    assert!(stderr.starts_with("config error: "), "{stderr}");
}

// @kotowari[C-core-surface-9]
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

// @kotowari[C-core-surface-9]
#[test]
fn req_225_a_key_combination_error_stops_every_command_that_reads_the_config() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "surface:\n  files:\n    - \"src/**/*.rs\"\n");
    write(tmp.path(), "results.json", "{}");
    for args in [
        &["list"][..],
        &["query", "C-core-cli-1"],
        &["status"],
        &["mutants", "--tool", "cargo-mutants", "results.json"],
    ] {
        let (code, _, stderr) = run(tmp.path(), args);
        assert_eq!(code, Some(2), "{args:?}: {stderr}");
        assert!(stderr.starts_with("config error: "), "{args:?}: {stderr}");
    }
}

// --- S2: 面の取り出し ---

/// "surface.files" を "src/**" に、"surface.rules" を "rules/surface.yml" にする設定の行
const SURFACE_SRC: &str =
    "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"rules/surface.yml\"\n";

/// 文字列のリテラルのうち "--" で始まるものを種類 "flag" の`面`にする`面の規則`
const FLAG_RULE: &str =
    "id: flag\nlanguage: rust\nrule:\n  kind: string_literal\n  regex: '^\"--'\n  pattern: $NAME\n";

// @kotowari[C-core-surface-5]
#[test]
fn req_224_surface_rules_are_not_added_to_the_test_queries() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"tests/**\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: func\nlanguage: rust\nrule:\n  kind: function_item\n  has:\n    field: name\n    pattern: $NAME\n",
    );
    write(tmp.path(), "tests/a.rs", "fn helper() {}\n");
    let (_, stdout, stderr) = run(tmp.path(), &["check"]);
    let v: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    assert!(
        v["counts"].get("test_without_id").is_none(),
        "a surface rule counted a test: {v}"
    );
}

// @kotowari[C-core-surface-8]
#[test]
fn req_224_a_non_utf8_surface_file_in_a_rule_language_stops_as_a_test_file_does() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/data.rs"), [0xff, 0xfe, 0x00]).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("non-UTF-8 file: src/data.rs"),
        "{stderr}"
    );
}

// @kotowari[C-core-surface-7]
#[test]
fn req_236_a_non_utf8_surface_file_not_in_a_rule_language_is_not_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/logo.png"), [0xff, 0xfe, 0x00]).unwrap();
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
}

// @kotowari[C-core-clienv-7, C-core-config-11]
#[cfg(unix)]
#[test]
fn req_018_a_dangling_symlink_in_the_surface_walk_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::os::unix::fs::symlink("missing.rs", tmp.path().join("src/link.rs")).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("unreadable file: src/link.rs"),
        "{stderr}"
    );
}

// @kotowari[C-core-surface-10]
#[test]
fn ex_core_417_a_missing_rule_file_stops_check_but_not_list() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"rules/missing.yml\"\n",
    );
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    let (code, _, stderr) = run(tmp.path(), &["list"]);
    assert_eq!(code, Some(0), "{stderr}");
}

// @kotowari[C-core-clienv-9, C-core-surface-10]
#[test]
fn req_225_a_broken_rule_file_stops_check_and_status_naming_the_rule_file() {
    let broken: [(&str, &str); 4] = [
        ("rules/surface.yml", "id: [unclosed\n"),
        (
            "rules/surface.yml",
            "id: x\nlanguage: cobol\nrule:\n  pattern: $NAME\n",
        ),
        ("rules/surface.yml", "id: x\nlanguage: rust\nrule: {}\n"),
        ("rules/surface.yml/inner", "not a file\n"),
    ];
    for (path, content) in broken {
        let tmp = TempDir::new().unwrap();
        make_project(tmp.path(), SURFACE_SRC);
        write(tmp.path(), path, content);
        for command in ["check", "status"] {
            let (code, _, stderr) = run(tmp.path(), &[command]);
            assert_eq!(code, Some(2), "{command} {content}: {stderr}");
            assert!(
                stderr.starts_with("config error: ") && stderr.contains("rules/surface.yml"),
                "{command} {content}: {stderr}"
            );
        }
    }
}

// @kotowari[C-core-surface-10]
#[test]
fn req_225_the_same_rule_path_twice_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"rules/surface.yml\"\n    - \"rules/surface.yml\"\n",
    );
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("config error: ") && stderr.contains("rules/surface.yml"),
        "{stderr}"
    );
}

// @kotowari[C-core-surface-6]
#[test]
fn req_236_a_file_already_unparsable_as_a_test_file_is_not_reported_twice() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"tests/**/*.rs\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "tests/bad.rs", "fn f( {\n");
    let (_, stdout, stderr) = run(tmp.path(), &["check"]);
    let v: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    assert_eq!(v["counts"]["unparsable_file"], 1, "{v}");
}

// --- S3: IR にあるかの判定と surface_without_spec ---

/// "surface.files" を "src/**/*.rs" に、"surface.rules" を "rules/surface.yml" にする設定の行
const SURFACE_RS: &str =
    "surface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"rules/surface.yml\"\n";

/// "src/cli.rs" の3行目に "--format"、12行目と30行目に "--verbose" の文字列のリテラルを置く
fn cli_rs() -> String {
    let mut lines = vec!["let _ = 0;".to_string(); 31];
    lines[0] = "fn main() {".to_string();
    lines[2] = "    let _ = \"--format\";".to_string();
    lines[11] = "    let _ = \"--verbose\";".to_string();
    lines[29] = "    let _ = \"--verbose\";".to_string();
    lines[30] = "}".to_string();
    lines.join("\n") + "\n"
}

/// 「決めたこと」が `decision` の決定を1つ持つ決定の文書。`rest` はそのあとに足す
fn decision_doc(decision: &str, rest: &str) -> String {
    format!(
        "# 題名\n\n範囲。\n\n## 決定\n\n状態: 実装済み\n\n### 問題\n\n困っていた。\n\n### 決めたこと\n\n{decision}\n{rest}"
    )
}

/// "--format" を`要求`の`文`に二重引用符で書いた IR と、面の規則と "src/cli.rs"
fn ex_407_project(tmp: &Path) {
    make_project(tmp, SURFACE_RS);
    write(tmp, "rules/surface.yml", FLAG_RULE);
    write(tmp, "src/cli.rs", &cli_rs());
}

/// check --format json の結果
fn check_json(tmp: &Path) -> (Option<i32>, serde_json::Value) {
    let (code, stdout, stderr) = run(tmp, &["check", "--format", "json"]);
    let v = serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    (code, v)
}

/// その種類の指摘の (path, line, detail) の並び
fn findings_of(v: &serde_json::Value, kind: &str) -> Vec<(String, serde_json::Value, String)> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .map(|f| {
            (
                f["path"].as_str().unwrap().to_string(),
                f["line"].clone(),
                f["detail"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

/// surface_without_spec の detail の並び
fn without_spec(v: &serde_json::Value) -> Vec<String> {
    findings_of(v, "surface_without_spec")
        .into_iter()
        .map(|(_, _, detail)| detail)
        .collect()
}

// @kotowari[C-core-findings-6, C-core-surface-15, C-core-surface-16]
#[test]
fn ex_core_408_a_surface_absent_from_the_ir_is_an_error_at_its_first_place() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        &decision_doc("\"--format\" を受ける。", ""),
    );
    let (code, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_without_spec"),
        vec![(
            "src/cli.rs".to_string(),
            serde_json::json!(12),
            "flag --verbose".to_string()
        )],
        "{v}"
    );
    assert_eq!(v["findings"][0]["severity"], "error", "{v}");
    assert_eq!(code, Some(1));
}

// @kotowari[C-core-surface-16]
#[test]
fn req_227_the_first_place_is_by_path_bytes_then_line() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    let mut a = ["let _ = 0;"; 9];
    a[8] = "fn a() { let _ = \"--verbose\"; }";
    write(tmp.path(), "src/a.rs", &(a.join("\n") + "\n"));
    write(
        tmp.path(),
        "src/b.rs",
        "fn b() { let _ = \"--verbose\"; }\n",
    );
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        &decision_doc("\"--format\" を受ける。", ""),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_without_spec"),
        vec![(
            "src/a.rs".to_string(),
            serde_json::json!(9),
            "flag --verbose".to_string()
        )],
        "{v}"
    );
}

// @kotowari[C-core-surface-12]
#[test]
fn ex_core_410_a_name_that_is_only_part_of_the_quoted_content_is_not_in_the_ir() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        &decision_doc(
            "\"--format\" と \"--verbose true\" と \" --verbose\" を受ける。",
            "",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[C-core-surface-11]
#[test]
fn req_226_double_quotes_pair_from_the_left() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    // 左から対にすると "x" の後の引用符と "--verbose" の前の引用符が対になる
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        &decision_doc("\"--format\" と x\" \"--verbose\" を受ける。", ""),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[C-core-surface-17]
#[test]
fn req_227_no_surface_is_extracted_when_surface_rules_is_empty() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "src/cli.rs", &cli_rs());
    let (code, v) = check_json(tmp.path());
    assert!(without_spec(&v).is_empty(), "{v}");
    assert_eq!(code, Some(0), "{v}");
}

// --- S4: 未記載の面の一覧 ---

/// 場面（"--format" だけが決定の文書にある）に、"surface.unspecified" と一覧の中身を足す
fn ex_408_project_with_list(tmp: &Path, list: &str) {
    make_project(
        tmp,
        &format!("{SURFACE_RS}  unspecified: \"docs/surface.yaml\"\n"),
    );
    write(tmp, "rules/surface.yml", FLAG_RULE);
    write(tmp, "src/cli.rs", &cli_rs());
    write(
        tmp,
        "docs/decisions/a/a.md",
        &decision_doc("\"--format\" を受ける。", ""),
    );
    write(tmp, "docs/surface.yaml", list);
}

/// 一覧の1件
fn entry(kind: &str, name: &str, why: &str) -> String {
    format!("- kind: \"{kind}\"\n  name: \"{name}\"\n  why: \"{why}\"\n")
}

// @kotowari[C-core-unspec-1]
#[test]
fn req_232_a_matching_entry_removes_the_error() {
    let tmp = TempDir::new().unwrap();
    // 同じ内容の1件が2つあっても検査しない
    let one = entry("flag", "--verbose", "まだ決めていない");
    ex_408_project_with_list(tmp.path(), &format!("{one}{one}"));
    let (code, v) = check_json(tmp.path());
    assert!(v["findings"].as_array().unwrap().is_empty(), "{v}");
    assert_eq!(code, Some(0));
}

// @kotowari[C-core-unspec-2]
#[test]
fn ex_core_420_an_entry_of_another_kind_does_not_match() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("subcommand", "--verbose", "理由"));
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[C-core-unspec-2, C-core-unspec-6]
#[test]
fn req_232_names_are_compared_without_trimming() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--verbose ", "理由"));
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[C-core-forder-4, C-core-unspec-3, C-core-unspec-7]
#[test]
fn ex_core_421_an_entry_whose_reason_is_blank_is_invalid_and_removes_nothing() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--verbose", " \t "));
    let (code, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_unspecified_invalid"),
        vec![(
            "docs/surface.yaml".to_string(),
            serde_json::Value::Null,
            "flag --verbose".to_string()
        )],
        "{v}"
    );
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
    assert!(
        findings_of(&v, "surface_unspecified_stale").is_empty(),
        "{v}"
    );
    assert_eq!(code, Some(1));
}

// @kotowari[C-core-unspec-3, C-core-unspec-7]
#[test]
fn req_233_each_malformed_entry_is_invalid_with_the_written_kind_and_name() {
    let tmp = TempDir::new().unwrap();
    let list = [
        "- \"not a mapping\"\n",
        "- kind: \"flag\"\n  name: \"--a\"\n",
        "- kind: \"flag\"\n  name: \"--b\"\n  why: \"理由\"\n  extra: \"x\"\n",
        "- kind: \"flag\"\n  name: 3\n  why: \"理由\"\n",
        "- name: \"--c\"\n  why: \"理由\"\n",
        "- kind: \"flag\"\n  name: \"--d\"\n  why: 1\n",
    ]
    .concat();
    ex_408_project_with_list(tmp.path(), &list);
    let (_, v) = check_json(tmp.path());
    let mut details: Vec<String> = findings_of(&v, "surface_unspecified_invalid")
        .into_iter()
        .map(|(_, _, detail)| detail)
        .collect();
    details.sort();
    assert_eq!(
        details,
        vec![" ", " --c", "flag ", "flag --a", "flag --b", "flag --d"],
        "{v}"
    );
    assert!(
        findings_of(&v, "surface_unspecified_stale").is_empty(),
        "{v}"
    );
}

// @kotowari[C-core-forder-4, C-core-findings-1, C-core-findings-2, C-core-unspec-4]
#[test]
fn ex_core_422_an_entry_for_a_surface_gone_from_the_code_is_a_notice() {
    let tmp = TempDir::new().unwrap();
    let list = [
        entry("flag", "--verbose", "理由"),
        entry("flag", "--old", "理由"),
    ]
    .concat();
    ex_408_project_with_list(tmp.path(), &list);
    let (code, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_unspecified_stale"),
        vec![(
            "docs/surface.yaml".to_string(),
            serde_json::Value::Null,
            "flag --old".to_string()
        )],
        "{v}"
    );
    let stale = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["kind"] == "surface_unspecified_stale")
        .unwrap();
    assert_eq!(stale["severity"], "notice");
    assert_eq!(code, Some(0), "a notice does not change the exit code: {v}");
}

// @kotowari[C-core-unspec-4]
#[test]
fn ex_core_423_an_entry_for_a_surface_now_in_the_ir_is_a_notice() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--format", "理由"));
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_unspecified_stale"),
        vec![(
            "docs/surface.yaml".to_string(),
            serde_json::Value::Null,
            "flag --format".to_string()
        )],
        "{v}"
    );
}

// @kotowari[C-core-clienv-7, C-core-unspec-5, C-core-unspec-8]
#[test]
fn ex_core_424_a_missing_list_file_stops_as_unreadable() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "");
    std::fs::remove_file(tmp.path().join("docs/surface.yaml")).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(stderr.starts_with("unreadable file: "), "{stderr}");
}

// @kotowari[C-core-clienv-9, C-core-unspec-5, C-core-unspec-8]
#[test]
fn ex_core_425_a_list_that_is_not_a_sequence_stops_naming_the_list() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "kind: flag\n");
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("config error: docs/surface.yaml"),
        "{stderr}"
    );
}

// @kotowari[C-core-clienv-9, C-core-clienv-12, C-core-clienv-7, C-core-unspec-5, C-core-unspec-8]
#[test]
fn req_231_a_list_that_is_not_yaml_or_not_utf8_stops_check_and_status() {
    let cases: [(&[u8], &str); 2] = [
        (b"- [unclosed\n", "config error: docs/surface.yaml"),
        (&[0xff, 0xfe], "non-UTF-8 file: docs/surface.yaml"),
    ];
    for (content, wording) in cases {
        let tmp = TempDir::new().unwrap();
        ex_408_project_with_list(tmp.path(), "");
        std::fs::write(tmp.path().join("docs/surface.yaml"), content).unwrap();
        for command in ["check", "status"] {
            let (code, _, stderr) = run(tmp.path(), &[command]);
            assert_eq!(code, Some(2), "{command}: {stderr}");
            assert!(stderr.starts_with(wording), "{command}: {stderr}");
        }
    }
}

// @kotowari[C-core-unspec-5]
#[test]
fn req_231_a_blank_list_has_no_entries() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "# まだ無い\n\n");
    let (code, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
    assert_eq!(code, Some(1));
}

// @kotowari[C-core-list-16, C-core-query-10]
#[test]
fn req_229_list_and_query_do_not_read_the_surface_list_or_files() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "");
    std::fs::remove_file(tmp.path().join("docs/surface.yaml")).unwrap();
    std::fs::write(tmp.path().join("src/data.bin"), [0xff, 0xfe]).unwrap();
    for args in [&["list"][..], &["query", "docs/decisions/a/a.md#決定"]] {
        let (code, _, stderr) = run(tmp.path(), args);
        assert_eq!(code, Some(0), "{args:?}: {stderr}");
    }
}

// @kotowari[C-core-surface-21]
#[test]
fn req_229_mutants_does_not_read_the_surface_list_rules_or_files() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "");
    std::fs::remove_file(tmp.path().join("docs/surface.yaml")).unwrap();
    std::fs::write(tmp.path().join("rules/surface.yml"), [0xff, 0xfe]).unwrap();
    std::fs::write(tmp.path().join("src/data.bin"), [0xff, 0xfe]).unwrap();
    write(
        tmp.path(),
        "results.json",
        r#"{"outcomes":[{"scenario":"Baseline","summary":"Success"}]}"#,
    );
    let (code, _, stderr) = run(
        tmp.path(),
        &["mutants", "--tool", "cargo-mutants", "results.json"],
    );
    assert_eq!(code, Some(0), "{stderr}");
}

// --- S5: check の出力と status ---

/// "--format" を`要求`の`文`に書き、"--verbose" を`未記載の面の一覧`に載せる
fn ex_407_ir(tmp: &Path) {
    write(
        tmp,
        "docs/decisions/a/a.md",
        &decision_doc("\"--format\" を受ける。", ""),
    );
}

// @kotowari[C-core-output-3, C-core-surface-1, C-core-surface-11, C-core-surface-18]
#[test]
fn ex_core_407_a_surface_quoted_in_a_requirement_is_not_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    let mut lines = ["let _ = 0;"; 4];
    lines[0] = "fn main() {";
    lines[2] = "    let _ = \"--format\";";
    lines[3] = "}";
    write(tmp.path(), "src/cli.rs", &(lines.join("\n") + "\n"));
    ex_407_ir(tmp.path());
    let (code, v) = check_json(tmp.path());
    assert!(without_spec(&v).is_empty(), "{v}");
    assert_eq!(v["surface"], serde_json::json!({"unspecified": 0}), "{v}");
    assert_eq!(code, Some(0), "{v}");
}

// @kotowari[C-core-surface-18, C-core-surface-19]
#[test]
fn ex_core_414_the_count_line_is_printed_even_with_no_findings() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(
        tmp.path(),
        "src/cli.rs",
        "fn main() {\n    let _ = \"--format\";\n}\n",
    );
    ex_407_ir(tmp.path());
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "text"]);
    assert_eq!(stdout, "surface: unspecified=0\n", "{stderr}");
    assert_eq!(code, Some(0));
}

// @kotowari[C-core-surface-17, C-core-surface-20]
#[test]
fn ex_core_413_a_project_without_surface_rules_sees_nothing() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "src/cli.rs", &cli_rs());
    ex_407_ir(tmp.path());
    let (_, stdout, _) = run(tmp.path(), &["check", "--format", "text"]);
    assert!(!stdout.contains("surface"), "{stdout}");
    let (_, v) = check_json(tmp.path());
    assert!(v.get("surface").is_none(), "{v}");
}

// @kotowari[C-core-unspec-1, C-core-surface-18]
#[test]
fn ex_core_419_a_listed_surface_is_not_an_error_and_is_counted() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--verbose", "まだ決めていない"));
    let (code, v) = check_json(tmp.path());
    assert!(without_spec(&v).is_empty(), "{v}");
    assert_eq!(v["surface"]["unspecified"], 1, "{v}");
    assert_eq!(code, Some(0), "{v}");
}

// @kotowari[C-core-unspec-4, C-core-surface-18]
#[test]
fn ex_core_423_a_listed_surface_now_in_the_ir_is_not_counted() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--verbose", "理由"));
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        &decision_doc("\"--format\" と \"--verbose\" を受ける。", ""),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_unspecified_stale")
            .into_iter()
            .map(|(_, _, detail)| detail)
            .collect::<Vec<_>>(),
        vec!["flag --verbose"],
        "{v}"
    );
    assert_eq!(v["surface"]["unspecified"], 0, "{v}");
}

/// 面が4つ（"--format" と "--config" は IR に、"--verbose" は一覧に、"--tool" はどこにも無い）のプロジェクト。
/// "--config" は2か所にあっても種類と名前の組で1つに数える
fn four_surfaces(tmp: &Path) {
    ex_408_project_with_list(tmp, &entry("flag", "--verbose", "理由"));
    write(
        tmp,
        "src/more.rs",
        "fn m() {\n    let _ = [\"--config\", \"--tool\", \"--config\"];\n}\n",
    );
    write(
        tmp,
        "docs/decisions/a/a.md",
        &decision_doc("\"--format\" と \"--config\" を受ける。", ""),
    );
}

// @kotowari[C-core-surface-19]
#[test]
fn req_228_the_count_line_comes_after_the_findings() {
    let tmp = TempDir::new().unwrap();
    four_surfaces(tmp.path());
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "text"]);
    assert_eq!(
        stdout, "src/more.rs:2 [error] surface_without_spec flag --tool\nsurface: unspecified=1\n",
        "{stderr}"
    );
    assert_eq!(code, Some(1));
}

// @kotowari[C-core-status-2, C-core-status-5, C-core-status-8, C-core-status-11]
#[test]
fn req_229_status_counts_surfaces_and_a_surface_error_makes_it_incomplete() {
    let tmp = TempDir::new().unwrap();
    four_surfaces(tmp.path());
    let (code, stdout, stderr) = run(tmp.path(), &["status"]);
    let v: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    assert_eq!(
        v["surface"],
        serde_json::json!({"total": 4, "specified": 2, "unspecified": 1}),
        "{v}"
    );
    assert_eq!(v["findings"]["error"], 1, "{v}");
    assert_eq!(v["complete"], false, "{v}");
    assert_eq!(code, Some(1));
    let (_, text, _) = run(tmp.path(), &["status", "--format", "text"]);
    let lines: Vec<&str> = text.lines().collect();
    let guides = lines.iter().position(|l| l.starts_with("guides ")).unwrap();
    assert_eq!(
        lines[guides + 2],
        "surface total=4 specified=2 unspecified=1",
        "{text}"
    );
}

// @kotowari[C-core-status-8]
#[test]
fn tbl_028_status_surface_counts_are_zero_without_surface_rules() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    ex_407_ir(tmp.path());
    let (_, stdout, stderr) = run(tmp.path(), &["status"]);
    let v: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    assert_eq!(
        v["surface"],
        serde_json::json!({"total": 0, "specified": 0, "unspecified": 0}),
        "{v}"
    );
}

// @kotowari[C-core-surface-16]
#[test]
fn req_227_the_first_place_is_the_smallest_line_even_when_rules_share_an_id() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    // 同じ id の規則が2つあり、先の規則は後の行にだけ当たる
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: flag\nlanguage: rust\nrule:\n  kind: string_literal\n  pattern: $NAME\n  inside:\n    kind: const_item\n    stopBy: end\n---\nid: flag\nlanguage: rust\nrule:\n  kind: string_literal\n  pattern: $NAME\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() { let _ = \"--verbose\"; }\nconst C: &str = \"--verbose\";\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_without_spec"),
        vec![(
            "src/cli.rs".to_string(),
            serde_json::json!(1),
            "flag --verbose".to_string()
        )],
        "{v}"
    );
}

// @kotowari[C-core-surface-6]
#[test]
fn ex_core_427_check_reports_the_unparsable_surface_file_beside_a_test_file_one() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "src/bad.rs", "fn f( {\n");
    // 別のパスのテストのファイルにも unparsable_file が出ている
    write(tmp.path(), "tests/broken.rs", "fn g( {\n");
    let (_, v) = check_json(tmp.path());
    let paths: Vec<String> = findings_of(&v, "unparsable_file")
        .into_iter()
        .map(|(path, _, _)| path)
        .collect();
    assert_eq!(paths, vec!["src/bad.rs", "tests/broken.rs"], "{v}");
}

// --- 決定の文書に引用された面 ---

/// 面の規則と "src/cli.rs"。IR は無い。
/// `sections` は決定1つの (節の名前, 中身) の並び
fn decision_project(tmp: &Path, sections: &[(&str, &str)]) {
    make_project(tmp, SURFACE_RS);
    write(tmp, "rules/surface.yml", FLAG_RULE);
    write(tmp, "src/cli.rs", &cli_rs());
    let mut text = "# 題名\n\n範囲。\n\n## フラグを受ける\n\n状態: 提案\n\n".to_string();
    for (name, body) in sections {
        text.push_str(&format!("### {name}\n\n{body}\n\n"));
    }
    write(tmp, "docs/decisions/flags.md", &text);
}

// @kotowari[C-core-surface-11]
#[test]
fn req_226_names_quoted_in_what_a_decision_decided_and_its_rows_count() {
    let tmp = TempDir::new().unwrap();
    decision_project(
        tmp.path(),
        &[
            ("問題", "困っていた。"),
            ("決めたこと", "\"--format\" を受ける。"),
            ("確認すること", "- C-flags-1 `--verbose` で詳しく出る"),
        ],
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), Vec::<String>::new(), "{v}");
}

// @kotowari[C-core-surface-13]
#[test]
fn req_226_names_quoted_only_in_the_problem_rejected_options_or_open_items_do_not_count() {
    let tmp = TempDir::new().unwrap();
    decision_project(
        tmp.path(),
        &[
            ("問題", "\"--verbose\" が無かった。"),
            ("決めたこと", "\"--format\" を受ける。"),
            ("見送った案", "- \"--verbose\" を受ける。要らない。"),
        ],
    );
    let path = tmp.path().join("docs/decisions/flags.md");
    let text =
        std::fs::read_to_string(&path).unwrap() + "## 未決\n\n- \"--verbose\" を受けるか。\n";
    std::fs::write(path, text).unwrap();
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[C-core-surface-12, C-core-surface-14]
#[test]
fn req_226_a_decision_quotes_a_name_only_as_the_whole_content_of_a_cell_outside_code() {
    let tmp = TempDir::new().unwrap();
    decision_project(
        tmp.path(),
        &[
            ("問題", "困っていた。"),
            (
                "決めたこと",
                "| 鍵 | 値 |\n|---|---|\n| \"--format | x\" | \"--verbose true\" |\n\n```\n\"--verbose\"\n```",
            ),
        ],
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        without_spec(&v),
        vec!["flag --format", "flag --verbose"],
        "{v}"
    );
}

// @kotowari[C-core-status-8, C-core-surface-11]
#[test]
fn req_226_a_name_quoted_in_a_decision_is_counted_as_specified_by_status() {
    let tmp = TempDir::new().unwrap();
    decision_project(
        tmp.path(),
        &[
            ("問題", "困っていた。"),
            ("決めたこと", "\"--format\" を受ける。"),
        ],
    );
    write(
        tmp.path(),
        "docs/surface.yaml",
        &entry("flag", "--format", "古い"),
    );
    let config = tmp.path().join(".kotowari/config.yaml");
    let text = std::fs::read_to_string(&config)
        .unwrap()
        .replace("  rules:", "  unspecified: \"docs/surface.yaml\"\n  rules:");
    std::fs::write(config, text).unwrap();
    let (_, stdout, stderr) = run(tmp.path(), &["status"]);
    let v: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    assert_eq!(
        v["surface"],
        serde_json::json!({"total": 2, "specified": 1, "unspecified": 0}),
        "{v}"
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(
        findings_of(&v, "surface_unspecified_stale")
            .into_iter()
            .map(|(_, _, detail)| detail)
            .collect::<Vec<_>>(),
        vec!["flag --format"],
        "{v}"
    );
}

// @kotowari[C-core-surface-11]
#[test]
fn req_226_a_name_in_a_decision_cell_counts_even_after_a_stray_quote_in_another_cell() {
    let tmp = TempDir::new().unwrap();
    decision_project(
        tmp.path(),
        &[
            ("問題", "困っていた。"),
            (
                "決めたこと",
                "| 鍵 | 値 |\n|---|---|\n| x\" | \"--format\" |\n| `--verbose` | y |",
            ),
        ],
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), Vec::<String>::new(), "{v}");
}

// @kotowari[C-core-surface-11, C-core-surface-14]
#[test]
fn req_226_a_code_block_inside_a_row_or_a_quote_of_a_decision_is_not_read() {
    let tmp = TempDir::new().unwrap();
    decision_project(
        tmp.path(),
        &[
            ("問題", "困っていた。"),
            (
                "決めたこと",
                "> \"--format\" を受ける。\n>\n> ```\n> \"--verbose\"\n> ```",
            ),
            (
                "確認すること",
                "- C-flags-1 次を受ける\n\n  ```\n  \"--verbose\"\n  ```",
            ),
        ],
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}
