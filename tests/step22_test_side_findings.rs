//! テスト側の指摘を終了コードに数えない check（docs/ir/core/test-side-findings.md）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。`extra` の行をそのまま設定に足す
fn make_project(tmp: &Path, extra: &str) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    write(
        tmp,
        ".kotowari/config.yaml",
        &format!("tests:\n  files:\n    - \"tests/**/*.rs\"\n{extra}"),
    );
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

/// 1つの`要求`だけを持つ IR の文書。`文`は `statement`
fn ir_with_requirement(verification: &str, statement: &str) -> String {
    format!(
        "# Greeting\n\nScope.\n\n## Requirements\n\n### REQ-greet-001: Greet\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: {verification}\n\n{statement}\n"
    )
}

/// テストの無い unit の`要求`。ほかの`誤り`は無い
const UNTESTED: &str = "The system will greet.";

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

/// 指摘の (kind, severity, path) の一覧
fn findings(stdout: &str) -> Vec<(String, String, String)> {
    let v: serde_json::Value =
        serde_json::from_str(stdout).unwrap_or_else(|e| panic!("{e}: {stdout}"));
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["kind"].as_str().unwrap().to_string(),
                f["severity"].as_str().unwrap().to_string(),
                f["path"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

// @kotowari[EX-core-547]
#[test]
fn ex_core_547_a_specification_ahead_of_its_tests_passes_with_the_option() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "docs/ir/greet/greet.md",
        &ir_with_requirement("unit", UNTESTED),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    assert_eq!(
        findings(&stdout),
        vec![(
            "requirement_without_test".to_string(),
            "error".to_string(),
            "docs/ir/greet/greet.md".to_string()
        )]
    );
}

// @kotowari[EX-core-548]
#[test]
fn ex_core_548_the_same_state_stops_without_the_option() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "docs/ir/greet/greet.md",
        &ir_with_requirement("unit", UNTESTED),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
}

/// テストの無い`要求`と、用語集に無い語を逆引用符で囲んだ IR の行
fn untested_with_unknown_term(tmp: &Path) {
    make_project(tmp, "");
    write(
        tmp,
        "docs/ir/greet/greet.md",
        &ir_with_requirement("unit", "The system will greet the `visitor`."),
    );
}

// @kotowari[EX-core-549]
#[test]
fn ex_core_549_an_error_in_the_ir_stops_even_with_the_option() {
    let tmp = TempDir::new().unwrap();
    untested_with_unknown_term(tmp.path());
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
    assert!(
        findings(&stdout)
            .iter()
            .any(|(kind, _, _)| kind == "unknown_term"),
        "{stdout}"
    );
}

// @kotowari[EX-core-550]
#[test]
fn ex_core_550_an_unreadable_test_file_that_is_also_a_surface_file_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "    - \"src/**/*.rs\"\nsurface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: flag\nlanguage: rust\nrule:\n  kind: string_literal\n  regex: '^\"--'\n  pattern: $NAME\n",
    );
    write(tmp.path(), "src/lib.rs", "fn f( {\n");
    write(
        tmp.path(),
        "docs/ir/greet/greet.md",
        &ir_with_requirement("review\n- how_to_verify: read it", UNTESTED),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
    assert!(
        findings(&stdout).contains(&(
            "unparsable_file".to_string(),
            "error".to_string(),
            "src/lib.rs".to_string()
        )),
        "{stdout}"
    );
}

// @kotowari[TBL-core-047]
#[test]
fn tbl_core_047_an_unreadable_test_file_matched_by_surface_files_in_no_rule_language_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "    - \"src/**/*.py\"\nsurface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: flag\nlanguage: rust\nrule:\n  kind: string_literal\n  regex: '^\"--'\n  pattern: $NAME\n",
    );
    write(tmp.path(), "src/notes.py", "def f(:\n");
    write(
        tmp.path(),
        "docs/ir/greet/greet.md",
        &ir_with_requirement("review\n- how_to_verify: read it", UNTESTED),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
    assert!(
        findings(&stdout).contains(&(
            "unparsable_file".to_string(),
            "error".to_string(),
            "src/notes.py".to_string()
        )),
        "{stdout}"
    );
}

// @kotowari[TBL-core-047]
#[test]
fn tbl_core_047_errors_only_in_test_files_pass_with_the_option() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-greet-999]\n#[test]\nfn a() {}\n\n// @kotowari[]\n#[test]\nfn b() {}\n",
    );
    write(tmp.path(), "tests/b.rs", "fn f( {\n");
    write(
        tmp.path(),
        "docs/ir/greet/greet.md",
        &ir_with_requirement("review\n- how_to_verify: read it", UNTESTED),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    let found = findings(&stdout);
    for (kind, path) in [
        ("unresolved_reference", "tests/a.rs"),
        ("invalid_marker", "tests/a.rs"),
        ("unparsable_file", "tests/b.rs"),
    ] {
        assert!(
            found.contains(&(kind.to_string(), "error".to_string(), path.to_string())),
            "{stdout}"
        );
    }
}

// @kotowari[EX-core-551]
#[test]
fn ex_core_551_a_malformed_guide_mark_stops_even_with_the_option() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "guides:\n  files:\n    - \"guides/**/*.md\"\n");
    write(
        tmp.path(),
        "docs/ir/greet/greet.md",
        &ir_with_requirement("review\n- how_to_verify: read it", UNTESTED),
    );
    write(
        tmp.path(),
        "guides/a.md",
        "# Guide\n\n<!-- @kotowari[] -->\n",
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
    assert_eq!(
        findings(&stdout),
        vec![(
            "invalid_marker".to_string(),
            "error".to_string(),
            "guides/a.md".to_string()
        )]
    );
}

// @kotowari[REQ-core-357]
#[test]
fn req_core_357_the_option_leaves_the_output_unchanged() {
    let tmp = TempDir::new().unwrap();
    untested_with_unknown_term(tmp.path());
    for format in ["json", "text"] {
        let (code, without, stderr) = run(tmp.path(), &["check", "--format", format]);
        assert_eq!(code, Some(1), "{without}{stderr}");
        let (code, with, stderr) = run(
            tmp.path(),
            &["check", "--format", format, "--allow-test-findings"],
        );
        assert_eq!(code, Some(1), "{with}{stderr}");
        assert!(without.contains("requirement_without_test"), "{without}");
        assert!(without.contains("unknown_term"), "{without}");
        assert_eq!(with, without, "{format}");
    }
}
