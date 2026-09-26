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
fn req_224_a_non_utf8_surface_file_in_a_rule_language_stops_as_a_test_file_does() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/data.rs"), [0xff, 0xfe, 0x00]).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(
        stderr.starts_with("non-UTF-8 file: src/data.rs"),
        "{stderr}"
    );
}

// @kotowari[REQ-core-236, REQ-core-224]
#[test]
fn req_236_a_non_utf8_surface_file_not_in_a_rule_language_is_not_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "docs/ir/a.md", "# A\n\n範囲。\n");
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/logo.png"), [0xff, 0xfe, 0x00]).unwrap();
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
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

/// review の`要求`1つの節。`文`は `statement`
fn review_requirement(id: &str, statement: &str) -> String {
    format!(
        "### {id}: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: 読んで確かめる\n\n{statement}\n\n"
    )
}

/// 要求の節とほかの節から IR の文書を作る
fn ir_doc(requirements: &str, rest: &str) -> String {
    format!("# 題名\n\n範囲。\n\n## Requirements\n\n{requirements}{rest}")
}

/// EX-core-407 の場面: "--format" を`要求`の`文`に二重引用符で書いた IR と、面の規則と "src/cli.rs"
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

// @kotowari[EX-core-408, EX-core-409, REQ-core-227, TBL-core-019, TBL-core-006, TBL-core-008]
#[test]
fn ex_core_408_a_surface_absent_from_the_ir_is_an_error_at_its_first_place() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement("REQ-001", "\"--format\" を受ける。"),
            "",
        ),
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

// @kotowari[REQ-core-227]
#[test]
fn req_227_the_first_place_is_by_path_bytes_then_line() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    let mut a = vec!["let _ = 0;"; 9];
    a[8] = "fn a() { let _ = \"--verbose\"; }";
    write(tmp.path(), "src/a.rs", &(a.join("\n") + "\n"));
    write(
        tmp.path(),
        "src/b.rs",
        "fn b() { let _ = \"--verbose\"; }\n",
    );
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement("REQ-001", "\"--format\" を受ける。"),
            "",
        ),
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

// @kotowari[EX-core-410, REQ-core-226]
#[test]
fn ex_core_410_a_name_that_is_only_part_of_the_quoted_content_is_not_in_the_ir() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement(
                "REQ-001",
                "\"--format\" と \"--verbose true\" と \" --verbose\" を受ける。",
            ),
            "",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[REQ-core-226]
#[test]
fn req_226_double_quotes_pair_from_the_left() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    // 左から対にすると "x" の後の引用符と "--verbose" の前の引用符が対になる
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement("REQ-001", "\"--format\" と x\" \"--verbose\" を受ける。"),
            "",
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[EX-core-411, REQ-core-226]
#[test]
fn ex_core_411_table_cells_deferred_statements_and_steps_count() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: command\nlanguage: rust\nrule:\n  kind: string_literal\n  pattern: $NAME\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "const C: [&str; 4] = [\"list\", \"query\", \"plan\", \"head\"];\n",
    );
    let deferred = "### REQ-002: 後回し\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: unit\n- deferred: docs/decision/records/r.md#A1\n\n\"query\" を受ける。\n\n";
    let table = "## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/r.md#A1\n\n| \"head\" | 中身 |\n|---|---|\n| \"list\" | 一覧 |\n\n";
    let example = "## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001\nScenario: 場面\n  When \"plan\" を実行する\n  Then 終わる\n```\n";
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(
            &format!("{}{deferred}", review_requirement("REQ-001", "文。")),
            &format!("{table}{example}"),
        ),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), Vec::<String>::new(), "{v}");
}

// @kotowari[EX-core-412, REQ-core-226]
#[test]
fn ex_core_412_a_glossary_term_in_backticks_counts() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: command\nlanguage: rust\nrule:\n  kind: string_literal\n  pattern: $NAME\n",
    );
    write(tmp.path(), "src/cli.rs", "const S: &str = \"status\";\n");
    write(
        tmp.path(),
        "docs/ir/CONTEXT.md",
        "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n| status | 集計 | docs/decision/records/r.md#A1 |\n",
    );
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&review_requirement("REQ-001", "`status` を出す。"), ""),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), Vec::<String>::new(), "{v}");
    assert!(findings_of(&v, "unknown_term").is_empty(), "{v}");
}

// @kotowari[EX-core-429, REQ-core-226]
#[test]
fn ex_core_429_property_statements_and_how_to_verify_lines_do_not_count() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    let requirement = "### REQ-001: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: \"--verbose\" を確かめる\n\n\"--format\" を受ける。\n\n";
    let property = "## Properties\n\n### PROP-001: 性質\n\n- source: docs/decision/records/r.md#A1\n\n\"--verbose\" が常に成り立つ。\n\n";
    write(tmp.path(), "docs/ir/a.md", &ir_doc(requirement, property));
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[REQ-core-226]
#[test]
fn req_226_scope_lines_flags_and_the_glossary_do_not_count() {
    let tmp = TempDir::new().unwrap();
    ex_407_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n\"--verbose\" を扱う。\n\n## Requirements\n\n{}",
            review_requirement("REQ-001", "\"--format\" を受ける。")
        ),
    );
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# Flags\n\n### FLAG-001: 問題\n\n- kind: gap\n- source: docs/decision/records/r.md#A1\n\n\"--verbose\" が決まっていない。\n",
    );
    write(
        tmp.path(),
        "docs/ir/CONTEXT.md",
        "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n| 冗長 | \"--verbose\" の出力 | docs/decision/records/r.md#A1 |\n",
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[REQ-core-227]
#[test]
fn req_227_no_surface_is_extracted_when_surface_rules_is_empty() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "src/cli.rs", &cli_rs());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&review_requirement("REQ-001", "文。"), ""),
    );
    let (code, v) = check_json(tmp.path());
    assert!(without_spec(&v).is_empty(), "{v}");
    assert_eq!(code, Some(0), "{v}");
}

// --- S4: 未記載の面の一覧 ---

/// EX-core-408 の場面（"--format" だけが IR にある）に、"surface.unspecified" と一覧の中身を足す
fn ex_408_project_with_list(tmp: &Path, list: &str) {
    make_project(
        tmp,
        &format!("{SURFACE_RS}  unspecified: \"docs/surface.yaml\"\n"),
    );
    write(tmp, "rules/surface.yml", FLAG_RULE);
    write(tmp, "src/cli.rs", &cli_rs());
    write(
        tmp,
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement("REQ-001", "\"--format\" を受ける。"),
            "",
        ),
    );
    write(tmp, "docs/surface.yaml", list);
}

/// 一覧の1件
fn entry(kind: &str, name: &str, why: &str) -> String {
    format!("- kind: \"{kind}\"\n  name: \"{name}\"\n  why: \"{why}\"\n")
}

// @kotowari[REQ-core-232]
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

// @kotowari[EX-core-420, REQ-core-232]
#[test]
fn ex_core_420_an_entry_of_another_kind_does_not_match() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("subcommand", "--verbose", "理由"));
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[REQ-core-232]
#[test]
fn req_232_names_are_compared_without_trimming() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--verbose ", "理由"));
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
}

// @kotowari[EX-core-421, REQ-core-233, REQ-core-027, TBL-core-006, TBL-core-008]
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

// @kotowari[REQ-core-233]
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

// @kotowari[EX-core-422, REQ-core-234, REQ-core-027, TBL-core-009]
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

// @kotowari[EX-core-423, REQ-core-234]
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

// @kotowari[EX-core-424, REQ-core-231, TBL-core-001]
#[test]
fn ex_core_424_a_missing_list_file_stops_as_unreadable() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "");
    std::fs::remove_file(tmp.path().join("docs/surface.yaml")).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(stderr.starts_with("unreadable file: "), "{stderr}");
}

// @kotowari[EX-core-425, REQ-core-231, TBL-core-020]
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

// @kotowari[REQ-core-231, TBL-core-001, TBL-core-020]
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

// @kotowari[REQ-core-231]
#[test]
fn req_231_a_blank_list_has_no_entries() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "# まだ無い\n\n");
    let (code, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), vec!["flag --verbose"], "{v}");
    assert_eq!(code, Some(1));
}

// @kotowari[REQ-core-229, REQ-core-152, REQ-core-158]
#[test]
fn req_229_list_and_query_do_not_read_the_surface_list_or_files() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), "");
    std::fs::remove_file(tmp.path().join("docs/surface.yaml")).unwrap();
    std::fs::write(tmp.path().join("src/data.bin"), [0xff, 0xfe]).unwrap();
    for args in [&["list"][..], &["query", "REQ-001"]] {
        let (code, _, stderr) = run(tmp.path(), args);
        assert_eq!(code, Some(0), "{args:?}: {stderr}");
    }
}

// @kotowari[REQ-core-229]
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

/// EX-core-407 の場面: "--format" を`要求`の`文`に書き、"--verbose" を`未記載の面の一覧`に載せる
fn ex_407_ir(tmp: &Path) {
    write(
        tmp,
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement("REQ-001", "\"--format\" を受ける。"),
            "",
        ),
    );
}

// @kotowari[EX-core-407, REQ-core-223, REQ-core-226, REQ-core-228, TBL-core-005]
#[test]
fn ex_core_407_a_surface_quoted_in_a_requirement_is_not_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    let mut lines = vec!["let _ = 0;"; 4];
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

// @kotowari[EX-core-414, REQ-core-228]
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

// @kotowari[EX-core-413, REQ-core-227, REQ-core-228]
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

// @kotowari[EX-core-419, REQ-core-232, REQ-core-228]
#[test]
fn ex_core_419_a_listed_surface_is_not_an_error_and_is_counted() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--verbose", "まだ決めていない"));
    let (code, v) = check_json(tmp.path());
    assert!(without_spec(&v).is_empty(), "{v}");
    assert_eq!(v["surface"]["unspecified"], 1, "{v}");
    assert_eq!(code, Some(0), "{v}");
}

// @kotowari[EX-core-423, REQ-core-228]
#[test]
fn ex_core_423_a_listed_surface_now_in_the_ir_is_not_counted() {
    let tmp = TempDir::new().unwrap();
    ex_408_project_with_list(tmp.path(), &entry("flag", "--verbose", "理由"));
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement("REQ-001", "\"--format\" と \"--verbose\" を受ける。"),
            "",
        ),
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
        "docs/ir/a.md",
        &ir_doc(
            &review_requirement("REQ-001", "\"--format\" と `--config` を受ける。"),
            "",
        ),
    );
    write(
        tmp,
        "docs/ir/CONTEXT.md",
        "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n| --config | 設定 | docs/decision/records/r.md#A1 |\n",
    );
}

// @kotowari[REQ-core-228, TBL-core-005]
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

// @kotowari[REQ-core-229, REQ-core-162, TBL-core-028]
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
        lines[guides + 1],
        "surface total=4 specified=2 unspecified=1",
        "{text}"
    );
}

// @kotowari[TBL-core-028, REQ-core-162]
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

// @kotowari[REQ-core-227]
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
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&review_requirement("REQ-001", "文。"), ""),
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

// @kotowari[EX-core-427, REQ-core-236]
#[test]
fn ex_core_427_check_reports_the_unparsable_surface_file_beside_a_test_file_one() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "src/bad.rs", "fn f( {\n");
    // 別のパスのテストのファイルにも unparsable_file が出ている
    write(tmp.path(), "tests/broken.rs", "fn g( {\n");
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&review_requirement("REQ-001", "文。"), ""),
    );
    let (_, v) = check_json(tmp.path());
    let paths: Vec<String> = findings_of(&v, "unparsable_file")
        .into_iter()
        .map(|(path, _, _)| path)
        .collect();
    assert_eq!(paths, vec!["src/bad.rs", "tests/broken.rs"], "{v}");
}

// @kotowari[REQ-core-226]
#[test]
fn req_226_a_backticked_term_in_a_decision_table_cell_counts() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_RS);
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: command\nlanguage: rust\nrule:\n  kind: string_literal\n  pattern: $NAME\n",
    );
    write(tmp.path(), "src/cli.rs", "const S: &str = \"status\";\n");
    write(
        tmp.path(),
        "docs/ir/CONTEXT.md",
        "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n| status | 集計 | docs/decision/records/r.md#A1 |\n",
    );
    let table = "## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/r.md#A1\n\n| コマンド | 中身 |\n|---|---|\n| `status` | 集計 |\n\n";
    write(
        tmp.path(),
        "docs/ir/a.md",
        &ir_doc(&review_requirement("REQ-001", "文。"), table),
    );
    let (_, v) = check_json(tmp.path());
    assert_eq!(without_spec(&v), Vec::<String>::new(), "{v}");
    assert!(findings_of(&v, "unknown_term").is_empty(), "{v}");
}
