//! 要求を後回しにする宣言（docs/ir/core/deferred.md）と、それが検査・集計に与える影響

use serde_json::Value;
use std::path::Path;
use tempfile::TempDir;

/// 出典 "docs/decision/records/r.md#A1" が解決する置き場と設定を作る
fn make_project(tmp: &Path) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "tests",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    write(
        tmp,
        ".kotowari/config.yaml",
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    );
    write(
        tmp,
        "docs/decision/records/r.md",
        "# 記録\n\n## Agreements\n\n- A1 合意\n",
    );
}

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// コマンドを走らせて (終了コード, 標準出力) を返す。標準エラーは空であるはず
fn run(tmp: &Path, args: &[&str]) -> (Option<i32>, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(args)
        .current_dir(tmp)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "", "no stop is expected");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
    )
}

/// "kotowari check" の JSON
fn check(tmp: &Path) -> Value {
    let (_, stdout) = run(tmp, &["check"]);
    serde_json::from_str(&stdout).expect("valid JSON")
}

/// その種類の指摘の (path, line, detail)
fn findings_of(result: &Value, kind: &str) -> Vec<(String, Option<u64>, String)> {
    result["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .map(|f| {
            (
                f["path"].as_str().unwrap().to_string(),
                f["line"].as_u64(),
                f["detail"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

/// 指摘の種類の並び
fn kinds(result: &Value) -> Vec<String> {
    result["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["kind"].as_str().unwrap().to_string())
        .collect()
}

/// 検証が "unit" の要求の見出しと行。extra は "- verification:" の行の直後に入る
fn unit_requirement(id: &str, extra: &str) -> String {
    format!(
        "### {id}: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: unit\n{extra}\n文。\n"
    )
}

// --- S1: "- deferred:" の行の読み取りと出典の検査 ---

// @kotowari[REQ-core-209, EX-core-386]
#[test]
fn ex_386_a_document_level_declaration_alone_is_not_a_scope() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n- deferred: docs/decision/records/r.md#A1\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "missing_scope"),
        vec![("docs/ir/a.md".to_string(), None, "a.md".to_string())],
        "{result}"
    );
    assert!(
        findings_of(&result, "unknown_field").is_empty(),
        "the declaration itself is a known line: {result}"
    );
}

// @kotowari[REQ-core-209, TBL-core-008, EX-core-387]
#[test]
fn ex_387_a_second_document_level_declaration_is_a_duplicate_field() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n- deferred: docs/decision/records/r.md#A1\n- deferred: docs/decision/records/r.md#A1\n範囲の続き。\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "duplicate_field"),
        vec![("docs/ir/a.md".to_string(), Some(5), "deferred".to_string())],
        "{result}"
    );
    assert!(findings_of(&result, "unknown_field").is_empty(), "{result}");
}

// @kotowari[REQ-core-209, EX-core-388]
#[test]
fn ex_388_a_declaration_in_the_glossary_is_still_an_unknown_field() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/CONTEXT.md",
        "# Glossary\n\n- deferred: docs/decision/records/r.md#A1\n\n| Term | Meaning | Source |\n|---|---|---|\n| 語 | 意味 | docs/decision/records/r.md#A1 |\n",
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "unknown_field"),
        vec![(
            "docs/ir/CONTEXT.md".to_string(),
            Some(3),
            "- deferred: docs/decision/records/r.md#A1".to_string()
        )],
        "{result}"
    );
}

// @kotowari[REQ-core-209]
#[test]
fn req_209_a_declaration_in_the_flags_is_still_an_unknown_field() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# Flags\n\n- deferred: docs/decision/records/r.md#A1\n\n### FLAG-001: 穴\n\n- kind: gap\n- related: REQ-001\n- source: docs/decision/records/r.md#A1\n\n本文。\n",
    );
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "unknown_field"),
        vec![(
            "docs/ir/FLAGS.md".to_string(),
            Some(3),
            "- deferred: docs/decision/records/r.md#A1".to_string()
        )],
        "{result}"
    );
}

// @kotowari[REQ-core-210, REQ-core-115, REQ-core-046, TBL-core-011, EX-core-390]
#[test]
fn ex_390_a_declaration_whose_decision_is_missing_is_a_source_invalid_on_its_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            unit_requirement(
                "REQ-001",
                "- deferred: docs/decision/records/r.md#A1, docs/decision/records/r.md#A9\n"
            )
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "source_invalid"),
        vec![(
            "docs/ir/a.md".to_string(),
            Some(12),
            "docs/decision/records/r.md#A9".to_string()
        )],
        "only the second value fails, on the deferred line: {result}"
    );
    assert!(findings_of(&result, "unknown_field").is_empty(), "{result}");
}

// @kotowari[REQ-core-208, EX-core-391]
#[test]
fn ex_391_form_errors_and_duplicate_ids_remain_under_a_declaration() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n- deferred: docs/decision/records/r.md#A1\n\n## Requirements\n\n### REQ-001: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n\n文。\n",
    );
    write(
        tmp.path(),
        "docs/ir/b.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "verification_missing"),
        vec![("docs/ir/a.md".to_string(), Some(8), "REQ-001".to_string())],
        "{result}"
    );
    assert_eq!(
        findings_of(&result, "duplicate_id"),
        vec![("docs/ir/b.md".to_string(), Some(7), "REQ-001".to_string())],
        "{result}"
    );
    assert!(findings_of(&result, "unknown_field").is_empty(), "{result}");
}

// @kotowari[REQ-core-208]
#[test]
fn req_208_both_declarations_on_one_requirement_raise_nothing() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n- deferred: docs/decision/records/r.md#A1\n\n## Requirements\n\n### REQ-001: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: 読んで確かめる\n- deferred: docs/decision/records/r.md#A1\n\n文。\n",
    );
    let result = check(tmp.path());
    assert!(kinds(&result).is_empty(), "{result}");
}

// @kotowari[REQ-core-209, REQ-core-210]
#[test]
fn req_209_a_declaration_in_a_document_without_requirements_is_still_checked() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n- deferred: docs/decision/records/r.md#A9\n- deferred: docs/decision/records/r.md#A1\n",
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "source_invalid"),
        vec![(
            "docs/ir/a.md".to_string(),
            Some(4),
            "docs/decision/records/r.md#A9".to_string()
        )],
        "only the first line's value is read: {result}"
    );
    assert_eq!(
        findings_of(&result, "duplicate_field"),
        vec![("docs/ir/a.md".to_string(), Some(5), "deferred".to_string())],
        "{result}"
    );
    assert_eq!(
        kinds(&result).len(),
        2,
        "nothing about the missing requirements: {result}"
    );
}

// @kotowari[REQ-core-210, TBL-core-008, TBL-core-019]
#[test]
fn req_210_an_empty_declaration_is_a_missing_source_on_its_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n- deferred:\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "- deferred:\n")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "missing_source"),
        vec![
            ("docs/ir/a.md".to_string(), Some(4), "deferred".to_string()),
            ("docs/ir/a.md".to_string(), Some(13), "deferred".to_string()),
        ],
        "{result}"
    );
}
