//! 要求を後回しにする宣言（docs/ir/core/deferred.md）と、それが検査・集計に与える影響
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

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
    unit_requirement_saying(id, extra, "文。")
}

/// unit_requirement の`文`を statement にしたもの
fn unit_requirement_saying(id: &str, extra: &str, statement: &str) -> String {
    format!(
        "### {id}: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: unit\n{extra}\n{statement}\n"
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

// --- S2: 後回しの要求と後回しのシナリオはテストの無さの検査から外す ---

/// 検査の結果に、その ID を detail にする requirement_without_test と scenario_without_test があるか
fn without_test_details(result: &Value) -> Vec<(String, String)> {
    ["requirement_without_test", "scenario_without_test"]
        .into_iter()
        .flat_map(|kind| {
            findings_of(result, kind)
                .into_iter()
                .map(move |(_, _, detail)| (kind.to_string(), detail))
        })
        .collect()
}

// @kotowari[REQ-core-208, REQ-core-210, REQ-core-085, EX-core-384]
#[test]
fn ex_384_a_requirement_level_declaration_removes_the_missing_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "- deferred: docs/decision/records/r.md#A1\n")
        ),
    );
    let result = check(tmp.path());
    assert!(without_test_details(&result).is_empty(), "{result}");
    assert!(
        findings_of(&result, "source_invalid").is_empty(),
        "{result}"
    );
}

// @kotowari[REQ-core-208, REQ-core-209, REQ-core-137, EX-core-385]
#[test]
fn ex_385_a_document_level_declaration_defers_every_requirement_of_the_document() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n- deferred: docs/decision/records/r.md#A1\n\n## Requirements\n\n{}\n{}\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1\nScenario: 例\n  Given a\n  When b\n  Then c\n```\n",
            unit_requirement("REQ-001", ""),
            unit_requirement("REQ-002", "")
        ),
    );
    let result = check(tmp.path());
    assert!(kinds(&result).is_empty(), "{result}");
}

// @kotowari[REQ-core-208, REQ-core-210, EX-core-389]
#[test]
fn ex_389_an_empty_declaration_still_defers_the_requirement() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "- deferred:\n")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "missing_source"),
        vec![("docs/ir/a.md".to_string(), Some(12), "deferred".to_string())],
        "{result}"
    );
    assert!(without_test_details(&result).is_empty(), "{result}");
}

/// 後回しの要求 REQ-001（unit）と、fields で決めた要求 REQ-002 と、@about が about のシナリオ EX-001
fn deferred_and_other_with_scenario(req_002_fields: &str, about: &str) -> String {
    format!(
        "# 題名\n\n範囲。\n\n## Requirements\n\n{}\n### REQ-002: 名前\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n{req_002_fields}\n文。\n\n## Examples\n\n```gherkin\n@id=EX-001 @about={about} @source=docs/decision/records/r.md#A1\nScenario: 例\n  Given a\n  When b\n  Then c\n```\n",
        unit_requirement("REQ-001", "- deferred: docs/decision/records/r.md#A1\n")
    )
}

// @kotowari[REQ-core-137]
#[test]
fn req_137_a_scenario_about_only_deferred_and_review_requirements_needs_no_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &deferred_and_other_with_scenario(
            "- verification: review\n- how_to_verify: 読む\n",
            "REQ-001,REQ-002",
        ),
    );
    let result = check(tmp.path());
    assert!(without_test_details(&result).is_empty(), "{result}");
}

// @kotowari[REQ-core-137]
#[test]
fn req_137_a_requirement_without_verification_does_not_keep_a_scenario_from_being_deferred() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &deferred_and_other_with_scenario("", "REQ-001,REQ-002"),
    );
    let result = check(tmp.path());
    assert_eq!(
        findings_of(&result, "verification_missing").len(),
        1,
        "{result}"
    );
    assert!(without_test_details(&result).is_empty(), "{result}");
}

// @kotowari[REQ-core-137]
#[test]
fn req_137_the_first_of_two_scenarios_with_one_id_decides_whether_it_is_deferred() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let doc = deferred_and_other_with_scenario("- verification: unit\n", "REQ-001").replace(
        "```\n",
        "\n@id=EX-001 @about=REQ-002 @source=docs/decision/records/r.md#A1\nScenario: 二つ目\n  Given a\n  When b\n  Then c\n```\n",
    );
    write(tmp.path(), "docs/ir/a.md", &doc);
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-002]\n#[test]\nfn t() {}\n",
    );
    let result = check(tmp.path());
    assert_eq!(findings_of(&result, "duplicate_id").len(), 1, "{result}");
    assert!(without_test_details(&result).is_empty(), "{result}");
}

// @kotowari[REQ-core-208, REQ-core-085]
#[test]
fn req_208_the_first_of_two_requirements_with_one_id_decides_whether_it_is_deferred() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "- deferred: docs/decision/records/r.md#A1\n")
        ),
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
    assert_eq!(findings_of(&result, "duplicate_id").len(), 1, "{result}");
    assert!(without_test_details(&result).is_empty(), "{result}");
}

// --- S3: deferred_with_test と depends_on_deferred の注意 ---

/// 題名と範囲と "## Requirements" の6行
const HEAD: &str = "# 題名\n\n範囲。\n\n## Requirements\n\n";
/// 後回しの宣言の行
const DEFER: &str = "- deferred: docs/decision/records/r.md#A1\n";

/// 7行目から始まる後回しの要求 REQ-001（14行目で終わる）
fn deferred_req_001() -> String {
    format!("{HEAD}{}", unit_requirement("REQ-001", DEFER))
}

/// その種類の注意の (path, line, detail)。どれも severity が notice であることも確かめる
fn notices_of(result: &Value, kind: &str) -> Vec<(String, Option<u64>, String)> {
    for f in result["findings"].as_array().unwrap() {
        if f["kind"] == kind {
            assert_eq!(f["severity"], "notice", "{f}");
        }
    }
    findings_of(result, kind)
}

fn at(line: u64, detail: &str) -> (String, Option<u64>, String) {
    ("docs/ir/a.md".to_string(), Some(line), detail.to_string())
}

// @kotowari[REQ-core-211, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-392]
#[test]
fn ex_392_a_mark_on_a_deferred_requirement_is_a_notice_on_its_heading() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", &deferred_req_001());
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-001]\n#[test]\nfn t() {}\n",
    );
    let (code, stdout) = run(tmp.path(), &["check"]);
    let result: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        notices_of(&result, "deferred_with_test"),
        vec![at(7, "REQ-001")],
        "{result}"
    );
    assert_eq!(code, Some(0), "a notice does not change the exit code");
}

// @kotowari[REQ-core-211, TBL-core-019, EX-core-393]
#[test]
fn ex_393_a_mark_on_a_deferred_scenario_is_a_notice_on_its_tag_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{}\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1\nScenario: 例\n  Given a\n  When b\n  Then c\n```\n",
            deferred_req_001()
        ),
    );
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[EX-001]\n#[test]\nfn t() {}\n",
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "deferred_with_test"),
        vec![at(19, "EX-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-211]
#[test]
fn req_211_a_mark_from_a_language_without_a_query_counts() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\ntests:\n  files: [\"tests/**\"]\n",
    );
    write(tmp.path(), "docs/ir/a.md", &deferred_req_001());
    write(tmp.path(), "tests/a.go", "// @kotowari[REQ-001]\n");
    let result = check(tmp.path());
    assert_eq!(result["tests"]["go"]["query"], false, "{result}");
    assert_eq!(
        notices_of(&result, "deferred_with_test"),
        vec![at(7, "REQ-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-211]
#[test]
fn req_211_two_marks_on_one_id_are_one_notice() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", &deferred_req_001());
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-001]\n#[test]\nfn t() {}\n\n// @kotowari[REQ-001, REQ-001]\n#[test]\nfn u() {}\n",
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "deferred_with_test"),
        vec![at(7, "REQ-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-211]
#[test]
fn req_211_the_notice_goes_to_the_first_of_two_requirements_with_one_id() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", &deferred_req_001());
    write(
        tmp.path(),
        "docs/ir/b.md",
        &format!(
            "# 題名\n\n範囲。\n範囲の続き。\n\n## Requirements\n\n{}",
            unit_requirement("REQ-001", "")
        ),
    );
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-001]\n#[test]\nfn t() {}\n",
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "deferred_with_test"),
        vec![at(7, "REQ-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-212, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-394]
#[test]
fn ex_394_a_statement_of_a_requirement_that_is_not_deferred_pointing_at_one_is_a_notice() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{}\n{}",
            deferred_req_001(),
            unit_requirement_saying("REQ-002", "", "`REQ-001` を使う。")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "depends_on_deferred"),
        vec![at(22, "REQ-002 REQ-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-212, REQ-core-137, EX-core-395]
#[test]
fn ex_395_a_scenario_about_deferred_and_other_requirements_is_a_notice_and_needs_a_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &deferred_and_other_with_scenario("- verification: unit\n", "REQ-001,REQ-002"),
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "depends_on_deferred"),
        vec![at(27, "EX-001 REQ-001")],
        "{result}"
    );
    assert_eq!(
        findings_of(&result, "scenario_without_test"),
        vec![at(27, "EX-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-212, EX-core-396]
#[test]
fn ex_396_references_from_deferred_requirements_are_not_notices() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{HEAD}{}\n{}\n{}",
            unit_requirement_saying("REQ-001", DEFER, "`REQ-002` を使う。"),
            unit_requirement("REQ-002", ""),
            unit_requirement_saying("REQ-003", DEFER, "`REQ-001` を使う。")
        ),
    );
    let result = check(tmp.path());
    assert!(
        notices_of(&result, "depends_on_deferred").is_empty(),
        "{result}"
    );
}

// @kotowari[REQ-core-212, EX-core-397]
#[test]
fn ex_397_each_reference_in_a_step_is_one_notice() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{}\n{}\n## Examples\n\n```gherkin\n@id=EX-002 @about=REQ-002 @source=docs/decision/records/r.md#A1\nScenario: 例\n  Given `REQ-001` と `REQ-001`\n  When b\n  Then c\n```\n",
            deferred_req_001(),
            unit_requirement("REQ-002", "")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "depends_on_deferred"),
        vec![at(29, "EX-002 REQ-001"), at(29, "EX-002 REQ-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-212]
#[test]
fn req_212_a_definition_line_pointing_at_a_deferred_requirement_is_a_notice() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{}\n{}",
            deferred_req_001(),
            unit_requirement("REQ-002", "- definition: REQ-001\n")
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "depends_on_deferred"),
        vec![at(21, "REQ-002 REQ-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-212]
#[test]
fn req_212_a_property_statement_pointing_at_a_deferred_requirement_is_a_notice() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{}\n## Properties\n\n### PROP-001: 性質\n\n- source: docs/decision/records/r.md#A1\n\n`REQ-001` が成り立つ。\n",
            deferred_req_001()
        ),
    );
    let result = check(tmp.path());
    assert_eq!(
        notices_of(&result, "depends_on_deferred"),
        vec![at(22, "PROP-001 REQ-001")],
        "{result}"
    );
}

// @kotowari[REQ-core-212]
#[test]
fn req_212_references_to_a_deferred_scenario_and_from_the_flags_are_not_notices() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{}\n{}\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1\nScenario: 例\n  Given a\n  When b\n  Then c\n```\n",
            deferred_req_001(),
            unit_requirement_saying("REQ-002", "", "`EX-001` を見る。")
        ),
    );
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# Flags\n\n### FLAG-001: 穴\n\n- kind: gap\n- related: REQ-001\n- source: docs/decision/records/r.md#A1\n\n本文。\n",
    );
    let result = check(tmp.path());
    assert!(
        notices_of(&result, "depends_on_deferred").is_empty(),
        "{result}"
    );
}

// --- S4: status と list と query の後回し ---

/// 7行目の後回しの要求 REQ-001（名前は "例"）と、それを指すシナリオ EX-001 と、後回しでない要求 REQ-002
fn deferred_with_scenario_and_other() -> String {
    format!(
        "{HEAD}### REQ-001: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: unit\n{DEFER}\n文。\n\n{}\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1\nScenario: 例\n  Given a\n  When b\n  Then c\n```\n",
        unit_requirement("REQ-002", "")
    )
}

// @kotowari[TBL-core-028, REQ-core-165, EX-core-398]
#[test]
fn ex_398_deferred_items_are_counted_and_leave_complete_true() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "{}\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1\nScenario: 例\n  Given a\n  When b\n  Then c\n```\n",
            deferred_req_001()
        ),
    );
    let (code, stdout) = run(tmp.path(), &["status"]);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["requirements"]["deferred"], 1, "{v}");
    assert_eq!(v["requirements"]["with_tests"], 0, "{v}");
    assert_eq!(v["requirements"]["without_tests"], 0, "{v}");
    assert_eq!(v["requirements"]["without_examples"], 0, "{v}");
    assert_eq!(v["scenarios"]["deferred"], 1, "{v}");
    assert_eq!(v["scenarios"]["with_tests"], 0, "{v}");
    assert_eq!(v["scenarios"]["without_tests"], 0, "{v}");
    assert_eq!(v["complete"], true, "{v}");
    assert_eq!(code, Some(0));
}

// @kotowari[TBL-core-028]
#[test]
fn tbl_028_a_deferred_requirement_without_examples_is_counted_in_without_examples() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", &deferred_req_001());
    let (_, stdout) = run(tmp.path(), &["status"]);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["requirements"]["deferred"], 1, "{v}");
    assert_eq!(v["requirements"]["without_examples"], 1, "{v}");
}

// @kotowari[TBL-core-026, REQ-core-155, EX-core-399]
#[test]
fn ex_399_list_marks_deferred_requirements_and_scenarios() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &deferred_with_scenario_and_other(),
    );
    let (_, stdout) = run(tmp.path(), &["list"]);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    let deferred: Vec<(String, Value)> = v["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| {
            (
                item["id"].as_str().unwrap().to_string(),
                item["deferred"].clone(),
            )
        })
        .collect();
    assert_eq!(
        deferred,
        vec![
            ("REQ-001".to_string(), Value::Bool(true)),
            ("REQ-002".to_string(), Value::Bool(false)),
            ("EX-001".to_string(), Value::Bool(true)),
        ],
        "{v}"
    );
    let (_, text) = run(tmp.path(), &["list", "--format", "text"]);
    assert_eq!(
        text.lines().next(),
        Some("REQ-001 unit 例 docs/ir/a.md:7 tests=0 deferred"),
        "{text}"
    );
    assert!(
        text.lines()
            .any(|line| line.starts_with("REQ-002 ") && line.ends_with(" tests=0")),
        "{text}"
    );
}

// @kotowari[TBL-core-027, REQ-core-161]
#[test]
fn tbl_027_query_carries_deferred_in_json_and_on_the_first_text_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &deferred_with_scenario_and_other(),
    );
    let (_, stdout) = run(tmp.path(), &["query", "REQ-001"]);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["items"][0]["deferred"], true, "{v}");
    let (_, stdout) = run(tmp.path(), &["query", "REQ-002"]);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["items"][0]["deferred"], false, "{v}");
    let (_, text) = run(tmp.path(), &["query", "EX-001", "--format", "text"]);
    assert_eq!(
        text.lines().next(),
        Some("EX-001 - 例 docs/ir/a.md:28 tests=0 deferred"),
        "{text}"
    );
}
