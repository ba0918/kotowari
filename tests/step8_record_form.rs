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
