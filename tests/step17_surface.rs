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

// --- S2: 面の取り出し ---

/// "surface.files" を "src/**" に、"surface.rules" を "rules/surface.yml" にする設定の行
const SURFACE_SRC: &str =
    "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"rules/surface.yml\"\n";

/// 文字列のリテラルのうち "--" で始まるものを種類 "flag" の`面`にする`面の規則`
const FLAG_RULE: &str =
    "id: flag\nlanguage: rust\nrule:\n  kind: string_literal\n  regex: '^\"--'\n  pattern: $NAME\n";

/// 設定を読んで面を取り出す。(面の (種類, 名前, パス, 行) の並び, 指摘の (種類, パス) の並び)
fn extract(tmp: &Path) -> (Vec<(String, String, String, usize)>, Vec<(String, String)>) {
    let text = std::fs::read_to_string(tmp.join(".kotowari/config.yaml")).unwrap();
    let cfg = Config::parse(&text).unwrap();
    let mut findings = Vec::new();
    let surfaces =
        kotowari_core::surface::extract(tmp, &cfg, &mut findings).unwrap_or_else(|e| panic!("{e}"));
    (
        surfaces
            .into_iter()
            .map(|s| (s.kind, s.name, s.path, s.line))
            .collect(),
        findings
            .into_iter()
            .map(|f| (f.kind.to_string(), f.path))
            .collect(),
    )
}

fn surface(kind: &str, name: &str, path: &str, line: usize) -> (String, String, String, usize) {
    (kind.to_string(), name.to_string(), path.to_string(), line)
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_a_matched_node_is_a_surface_named_by_name_with_one_quote_pair_removed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = \"--format\";\n    let b = \"x\";\n    let c = r#\"\"--raw\"\"#;\n}\n",
    );
    let (surfaces, findings) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![surface("flag", "--format", "src/cli.rs", 2)],
        "only the literal starting with \"--\" matches"
    );
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_only_one_pair_of_quotes_is_removed_and_other_forms_stay() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    // 識別子を名前にする規則と、raw の文字列のリテラルを名前にする規則
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: ident\nlanguage: rust\nrule:\n  kind: identifier\n  inside:\n    kind: let_declaration\n  pattern: $NAME\n---\nid: raw\nlanguage: rust\nrule:\n  kind: raw_string_literal\n  pattern: $NAME\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = r#\"'q'\"#;\n}\n",
    );
    let (surfaces, _) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![
            surface("ident", "a", "src/cli.rs", 2),
            surface("raw", "r#\"'q'\"#", "src/cli.rs", 2),
        ]
    );
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_a_match_without_name_is_not_a_surface() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: nameless\nlanguage: rust\nrule:\n  kind: string_literal\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = \"--format\";\n}\n",
    );
    let (surfaces, _) = extract(tmp.path());
    assert!(surfaces.is_empty(), "{surfaces:?}");
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_language_is_matched_without_case_and_accepts_aliases() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: flag\nlanguage: Rust\nrule:\n  kind: string_literal\n  pattern: $NAME\n---\nid: route\nlanguage: ts\nrule:\n  kind: string\n  pattern: $NAME\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = \"--format\";\n}\n",
    );
    write(tmp.path(), "src/app.ts", "const r = '/users';\n");
    let (surfaces, _) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![
            surface("route", "/users", "src/app.ts", 1),
            surface("flag", "--format", "src/cli.rs", 2),
        ]
    );
}

// @kotowari[EX-core-418, REQ-core-223]
#[test]
fn ex_core_418_a_rule_is_not_applied_to_a_file_of_another_language() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "src/a.ts", "const a = \"--format\";\n");
    let (surfaces, findings) = extract(tmp.path());
    assert!(surfaces.is_empty(), "{surfaces:?}");
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[REQ-core-224]
#[test]
fn req_224_rule_files_and_ignores_are_applied_and_severity_off_still_applies() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(
        tmp.path(),
        "rules/surface.yml",
        &format!(
            "{FLAG_RULE}severity: off\nfiles:\n  - \"src/cli/**\"\nignores:\n  - \"src/cli/skip.rs\"\n"
        ),
    );
    let body = "fn f() {\n    let a = \"--format\";\n}\n";
    write(tmp.path(), "src/cli/main.rs", body);
    write(tmp.path(), "src/cli/skip.rs", body);
    write(tmp.path(), "src/other.rs", body);
    let (surfaces, _) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![surface("flag", "--format", "src/cli/main.rs", 2)]
    );
}

// @kotowari[REQ-core-224]
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
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    let (_, stdout, stderr) = run(tmp.path(), &["check"]);
    let v: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    assert!(
        v["counts"].get("test_without_id").is_none(),
        "a surface rule counted a test: {v}"
    );
}

// @kotowari[REQ-core-224]
#[test]
fn req_224_a_non_utf8_surface_file_stops_as_a_test_file_does() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/data.bin"), [0xff, 0xfe, 0x00]).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("non-UTF-8 file: src/data.bin"),
        "{stderr}"
    );
}

// @kotowari[REQ-core-018, TBL-core-001]
#[cfg(unix)]
#[test]
fn req_018_a_dangling_symlink_in_the_surface_walk_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::os::unix::fs::symlink("missing.rs", tmp.path().join("src/link.rs")).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("unreadable file: src/link.rs"),
        "{stderr}"
    );
}

// @kotowari[EX-core-417, REQ-core-225, REQ-core-229]
#[test]
fn ex_core_417_a_missing_rule_file_stops_check_but_not_list() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"rules/missing.yml\"\n",
    );
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    let (code, _, stderr) = run(tmp.path(), &["list"]);
    assert_eq!(code, Some(0), "{stderr}");
}

// @kotowari[REQ-core-225, TBL-core-020]
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
        write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
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

// @kotowari[REQ-core-225]
#[test]
fn req_225_the_same_rule_path_twice_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"rules/surface.yml\"\n    - \"rules/surface.yml\"\n",
    );
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("config error: ") && stderr.contains("rules/surface.yml"),
        "{stderr}"
    );
}

// @kotowari[EX-core-426, REQ-core-236]
#[test]
fn ex_core_426_a_file_not_in_a_rule_language_is_not_parsed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "src/notes.py", "def (:\n");
    let (_, findings) = extract(tmp.path());
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[EX-core-427, REQ-core-236]
#[test]
fn ex_core_427_a_syntax_error_in_a_rule_language_file_is_unparsable() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(
        tmp.path(),
        "src/bad.rs",
        "fn f( {\n    let a = \"--format\";\n",
    );
    let (surfaces, findings) = extract(tmp.path());
    assert!(surfaces.is_empty(), "{surfaces:?}");
    assert_eq!(
        findings,
        vec![("unparsable_file".to_string(), "src/bad.rs".to_string())]
    );
}

// @kotowari[REQ-core-236]
#[test]
fn req_236_a_file_already_unparsable_as_a_test_file_is_not_reported_twice() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"tests/**/*.rs\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "tests/bad.rs", "fn f( {\n");
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    let (_, stdout, stderr) = run(tmp.path(), &["check"]);
    let v: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    assert_eq!(v["counts"]["unparsable_file"], 1, "{v}");
}
