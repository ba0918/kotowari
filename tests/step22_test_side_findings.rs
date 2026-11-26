//! テスト側の指摘を終了コードに数えない check（docs/decisions/core/test-side-findings.md）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。`extra` の行をそのまま設定に足す
fn make_project(tmp: &Path, extra: &str) {
    for dir in [".kotowari", "docs/decisions"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    write(
        tmp,
        ".kotowari/config.yaml",
        &format!(
            "decisions:\n  documents: docs/decisions\ntests:\n  files:\n    - \"tests/**/*.rs\"\n{extra}"
        ),
    );
}

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// テストの指していない確認することの行を1つ持つ決定の文書。ほかの`誤り`は無い
const UNTESTED_DOC: &str = "# Greeting\n\nScope.\n\n## Greet\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n\
### Decision\n\nGreet.\n\n### Confirmations\n\n- C-greet-greet-1 A greeting is shown\n";

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

// @kotowari[C-core-testside-1, C-core-testside-2, C-core-testside-4]
#[test]
fn ex_core_547_a_specification_ahead_of_its_tests_passes_with_the_option() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "docs/decisions/greet/greet.md", UNTESTED_DOC);
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    assert_eq!(
        findings(&stdout),
        vec![(
            "confirmation_without_test".to_string(),
            "error".to_string(),
            "docs/decisions/greet/greet.md".to_string()
        )]
    );
}

// @kotowari[C-core-testside-1]
#[test]
fn ex_core_548_the_same_state_stops_without_the_option() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "docs/decisions/greet/greet.md", UNTESTED_DOC);
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
}

/// テストの無い確認することの行と、問題の節の無い決定
fn untested_with_a_form_error(tmp: &Path) {
    make_project(tmp, "");
    write(
        tmp,
        "docs/decisions/greet/greet.md",
        &format!("{UNTESTED_DOC}\n## Wave\n\nStatus: implemented\n\n### Decision\n\nWave.\n"),
    );
}

// @kotowari[C-core-testside-6]
#[test]
fn ex_core_549_an_error_in_the_ir_stops_even_with_the_option() {
    let tmp = TempDir::new().unwrap();
    untested_with_a_form_error(tmp.path());
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
    assert!(
        findings(&stdout)
            .iter()
            .any(|(kind, _, _)| kind == "decision_section_missing"),
        "{stdout}"
    );
}

// @kotowari[C-core-testside-8]
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

// @kotowari[C-core-testside-8]
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

// @kotowari[C-core-testside-5]
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

// @kotowari[C-core-testside-7]
#[test]
fn ex_core_551_a_malformed_guide_mark_stops_even_with_the_option() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "guides:\n  files:\n    - \"guides/**/*.md\"\n");
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

// @kotowari[C-core-testside-2]
#[test]
fn req_core_357_the_option_leaves_the_output_unchanged() {
    let tmp = TempDir::new().unwrap();
    untested_with_a_form_error(tmp.path());
    for format in ["json", "text"] {
        let (code, without, stderr) = run(tmp.path(), &["check", "--format", format]);
        assert_eq!(code, Some(1), "{without}{stderr}");
        let (code, with, stderr) = run(
            tmp.path(),
            &["check", "--format", format, "--allow-test-findings"],
        );
        assert_eq!(code, Some(1), "{with}{stderr}");
        assert!(without.contains("confirmation_without_test"), "{without}");
        assert!(without.contains("decision_section_missing"), "{without}");
        assert_eq!(with, without, "{format}");
    }
}
