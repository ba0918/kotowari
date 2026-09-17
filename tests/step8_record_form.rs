//! 判断の記録の形の検査（REQ-129〜REQ-135、TBL-022、REQ-132、TBL-023）

use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// テスト用のプロジェクトを作る（設定と、Context を持たない記録と、記録でない Markdown）
fn make_project_with_records(tmp: &std::path::Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/records")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/records/records.md"),
        "# 判断の記録\n\n## Agreements\n\n- A1 最初の合意\n- A2 二番目の合意\n\n## Prohibitions\n\n- P1 禁止事項\n\n## Delegated\n\n## Rejected\n\n- R1 却下\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/records/ir-form.md"),
        "# IR の形の契約\n\n## 文書\n\n文書の形。\n\n## 項目\n\n項目の形。\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/adr/0001-test.md"),
        "# ADR 0001: テスト\n\n## 状況\n\n状況の説明。\n\n## 決定\n\n決定。\n",
    )
    .unwrap();
}

/// 記録を1つ足す
fn write_record(tmp: &std::path::Path, name: &str, content: &str) {
    fs::write(tmp.join("docs/decision/records").join(name), content).unwrap();
}

fn parse_json(output: &std::process::Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("should be valid JSON")
}

fn findings_by_kind(v: &serde_json::Value, kind: &str) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .cloned()
        .collect()
}

fn check(tmp: &std::path::Path) -> serde_json::Value {
    let output = cmd().arg("check").current_dir(tmp).output().unwrap();
    parse_json(&output)
}

// --- REQ-130 / TBL-022: 必須の補足の行 ---

// @kotowari[REQ-130, TBL-022]
#[test]
fn req_130_context_record_without_why_is_record_field_missing() {
    // EX-101
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "x.md",
        "# 記録 x\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n",
    );
    let v = check(tmp.path());
    let found = findings_by_kind(&v, "record_field_missing");
    assert_eq!(found.len(), 1, "one finding for the missing why: {:?}", found);
    assert_eq!(found[0]["path"], "docs/decision/records/x.md");
    assert_eq!(found[0]["line"], 9);
    assert_eq!(found[0]["detail"], "why");
    assert_eq!(found[0]["severity"], "error");
}

// @kotowari[REQ-133, TBL-022]
#[test]
fn req_133_not_recorded_passes_and_blank_value_is_missing() {
    // EX-103
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "y.md",
        "# 記録 y\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 最初の合意\n- why: not recorded\n- A2 二番目の合意\n- why:   \n",
    );
    let v = check(tmp.path());
    let found = findings_by_kind(&v, "record_field_missing");
    assert_eq!(
        found.len(),
        1,
        "\"not recorded\" counts as present and a blank value counts as absent: {:?}",
        found
    );
    assert_eq!(found[0]["line"], 11);
    assert_eq!(found[0]["detail"], "why");
}

// @kotowari[REQ-130, TBL-022]
#[test]
fn req_130_superseded_line_without_superseded_by_is_missing() {
    // EX-105
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "z.md",
        "# 記録 z\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n\n## Superseded\n\n- A3 置き換えられた決定\n- why: x\n",
    );
    let v = check(tmp.path());
    let found = findings_by_kind(&v, "record_field_missing");
    assert_eq!(found.len(), 1, "the Superseded line needs superseded_by: {:?}", found);
    assert_eq!(found[0]["line"], 14);
    assert_eq!(found[0]["detail"], "superseded_by");
}

// --- REQ-131 / TBL-022: 知らない名前の補足の行 ---

// @kotowari[REQ-131, TBL-022]
#[test]
fn req_131_unknown_field_name_is_record_field_unknown() {
    // EX-104
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "u.md",
        "# 記録 u\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n- reason: x\n",
    );
    let v = check(tmp.path());
    let unknown = findings_by_kind(&v, "record_field_unknown");
    assert_eq!(unknown.len(), 1, "\"reason\" is not one of the six names: {:?}", unknown);
    assert_eq!(unknown[0]["line"], 11);
    assert_eq!(unknown[0]["detail"], "reason");
    assert_eq!(unknown[0]["severity"], "error");
    let missing = findings_by_kind(&v, "record_field_missing");
    assert!(missing.is_empty(), "why is present, so nothing is missing: {:?}", missing);
}

// @kotowari[REQ-131, REQ-133, TBL-019]
#[test]
fn req_131_unknown_name_on_two_lines_yields_two_findings() {
    // 同じ知らない名前が2行あれば2件。値が空の行も名前の検査を受ける（REQ-133）
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "t.md",
        "# 記録 t\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n- reason: x\n- reason:\n",
    );
    let v = check(tmp.path());
    let unknown = findings_by_kind(&v, "record_field_unknown");
    let lines: Vec<i64> = unknown.iter().map(|f| f["line"].as_i64().unwrap()).collect();
    assert_eq!(lines, vec![11, 12], "one finding per line, blank value included: {:?}", unknown);
    assert!(unknown.iter().all(|f| f["detail"] == "reason"));
}

// --- REQ-129 / REQ-134 / REQ-135: 読まない行と、検査を受けない記録 ---

// @kotowari[REQ-129]
#[test]
fn req_129_record_without_context_is_not_checked() {
    // EX-102
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "c.md",
        "# 記録 c\n\n## Agreements\n\n- A1 ある合意\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "a record without \"## Context\" is not checked: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-135]
#[test]
fn req_135_lines_outside_the_table_sections_and_orphans_are_not_read() {
    // EX-110
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "v.md",
        concat!(
            "# 記録 v\n\n## Context\n\n背景。\n\n",
            "## Revisions\n\n- A21 は A5 を置き換える\n\n",
            "## Agreements\n\n- why: x\n- A1 ある合意\n- why: x\n- (i) 入れ子でない箇条\n- why : x\n（なし）\n",
        ),
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "lines outside the table's sections, orphan field lines and other shapes are not read: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-134]
#[test]
fn req_134_duplicate_field_names_pass() {
    // EX-111
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "w.md",
        "# 記録 w\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n- why: x\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "the number of field lines with the same name is not checked: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-135]
#[test]
fn req_135_numbered_line_inside_code_block_is_not_read() {
    // EX-113
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "cb.md",
        "# 記録 cb\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n\n```text\n- A9 コードブロックの中\n```\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "a numbered line inside a code block is not read: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-135]
#[test]
fn req_135_unclosed_code_block_runs_to_the_end_of_the_file() {
    // A45: 閉じられずに文書が終わるコードブロックは文書の終わりまでが中で、指摘は出さない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "uc.md",
        "# 記録 uc\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n- why: x\n\n```text\n- A9 x\n- A8 x\n",
    );
    let v = check(tmp.path());
    let for_file: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["path"] == "docs/decision/records/uc.md")
        .collect();
    assert!(for_file.is_empty(), "no finding for an unclosed code block: {:?}", for_file);
}

// @kotowari[REQ-130]
#[test]
fn req_130_unindented_field_line_belongs_to_the_decision() {
    // EX-114: 字下げ無しの補足の行も直前の番号の行に付く。空行を挟んでもよい
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "ui.md",
        "# 記録 ui\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 ある合意\n\n- why: x\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty(),
        "an unindented why after a blank line still belongs to A1: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-130, REQ-133]
#[test]
fn req_130_decision_line_with_colon_is_not_a_field() {
    // EX-115: 本文にコロンを含む決定の行は番号の行で、補足の行と見ない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "co.md",
        "# 記録 co\n\n## Context\n\n背景。\n\n## Agreements\n\n- A1 定義を機械的にする: 節にある行\n- why: x\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "the decision line is a numbered line, not a field line: {:?}",
        v["findings"]
    );
}

// @kotowari[REQ-129]
#[test]
fn req_129_file_without_decision_sections_is_not_a_record() {
    // A41: 決定の節の見出しを持たないファイルは判断の記録でない
    let tmp = TempDir::new().unwrap();
    make_project_with_records(tmp.path());
    write_record(
        tmp.path(),
        "notrec.md",
        "# 記録でない文書\n\n## Context\n\n背景。\n\n## Superseded\n\n- A3 置き換えられた決定\n",
    );
    let v = check(tmp.path());
    assert!(
        findings_by_kind(&v, "record_field_missing").is_empty()
            && findings_by_kind(&v, "record_field_unknown").is_empty(),
        "a file without a decision section is not a record: {:?}",
        v["findings"]
    );
}
