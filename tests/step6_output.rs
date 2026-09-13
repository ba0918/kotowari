use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn make_project(tmp: &std::path::Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/brainstorm")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\n",
    )
    .unwrap();
    fs::write(
        tmp.join("docs/decision/brainstorm/records.md"),
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    )
    .unwrap();
}

fn parse_json(output: &std::process::Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("valid JSON")
}

// --- REQ-021: 出力の形の値 ---

// @kotowari[REQ-021]
#[test]
fn req_021_format_values_are_json_and_text() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // --format json
    let output = cmd()
        .args(["check", "--format", "json"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str::<serde_json::Value>(&stdout).expect("should be valid JSON");

    // --format text
    let output2 = cmd()
        .args(["check", "--format", "text"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output2.status.code(), Some(0));
}

// --- REQ-022: JSON を1つ出す ---

// @kotowari[REQ-022]
#[test]
fn req_022_json_is_one_document() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let output = cmd()
        .args(["check", "--format", "json"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // 1つのJSONとしてパースできる
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(v.is_object());
}

// --- REQ-023: JSON の中身 ---

// @kotowari[REQ-023, TBL-005, TBL-006]
#[test]
fn req_023_files_counts_all_docs_and_lines_sums_them() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# A\n\nScope.\n\n## 要求\n\n### REQ-001: R\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nStmt.\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("docs/ir/CONTEXT.md"),
        "# 用語集\n\n| 用語 | 意味 | 出典 |\n|---|---|---|\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    // files は用語集を含む
    assert_eq!(v["files"], 2, "files should count all docs including glossary");
    // lines は合計
    assert!(v["lines"].as_u64().unwrap() > 0, "lines should be positive");
    // findings は配列
    assert!(v["findings"].is_array());
    // counts はオブジェクト
    assert!(v["counts"].is_object());
}

// @kotowari[REQ-023, TBL-006]
#[test]
fn req_023_finding_keys_match_the_table() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 題名なしの文書を作って指摘を出す
    fs::write(tmp.path().join("docs/ir/a.md"), "no title\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let findings = v["findings"].as_array().unwrap();
    assert!(!findings.is_empty());
    for f in findings {
        assert!(f["kind"].is_string(), "kind should be string");
        assert!(f["severity"].is_string(), "severity should be string");
        assert!(f["path"].is_string(), "path should be string");
        // line は number か null
        assert!(
            f["line"].is_null() || f["line"].is_number(),
            "line should be null or number"
        );
        assert!(f["detail"].is_string(), "detail should be string");
    }
}

// --- PROP-002: counts と findings の一致 ---

// @kotowari[PROP-002]
#[test]
fn prop_002_counts_match_findings() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "no title\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let findings = v["findings"].as_array().unwrap();
    let counts = v["counts"].as_object().unwrap();

    // counts の各種類の値は findings の中のその種類の数に等しい
    let mut finding_counts = std::collections::HashMap::new();
    for f in findings {
        let kind = f["kind"].as_str().unwrap();
        *finding_counts.entry(kind.to_string()).or_insert(0u64) += 1;
    }

    for (kind, count) in counts {
        let count_val = count.as_u64().unwrap();
        let finding_count = finding_counts.get(kind).copied().unwrap_or(0);
        assert_eq!(
            count_val, finding_count,
            "counts[{kind}]={count_val} but findings has {finding_count}"
        );
    }

    // findings に1件も無い種類は counts に無い
    for (kind, count) in &finding_counts {
        assert!(
            counts.contains_key(kind),
            "kind {kind} has {count} findings but is not in counts"
        );
    }
}

// --- REQ-025: 文字の出力 ---

// @kotowari[REQ-025]
#[test]
fn req_025_text_has_one_line_per_finding_with_bracketed_severity() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(tmp.path().join("docs/ir/a.md"), "no title\n").unwrap();
    let output = cmd()
        .args(["check", "--format", "text"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty(), "should have text output");
    for line in &lines {
        // [error] か [warning] を含む
        assert!(
            line.contains("[error]") || line.contains("[warning]"),
            "line should have bracketed severity: {line}"
        );
    }
}

// --- REQ-026: 行の無い指摘の文字の出力 ---

// @kotowari[REQ-026]
#[test]
fn req_026_null_line_prints_dash() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // missing_title は line が null
    fs::write(tmp.path().join("docs/ir/a.md"), "no title\n").unwrap();
    let output = cmd()
        .args(["check", "--format", "text"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    // "パス:- [error] missing_title ..." の形
    assert!(
        stdout.contains(":- [error]") || stdout.contains(":- [warning]"),
        "null line should print as '-': {stdout}"
    );
}

// --- REQ-007: 終了コード ---

// @kotowari[REQ-007, TBL-002]
#[test]
fn req_007_exit_code_one_on_error_and_zero_on_warning_only() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());

    // 誤りなし → 終了コード 0
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "no errors should exit 0");

    // 誤りあり → 終了コード 1
    fs::write(tmp.path().join("docs/ir/a.md"), "no title\n").unwrap();
    let output2 = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output2.status.code(), Some(1), "errors should exit 1");

    // 停止 → 終了コード 2
    let output3 = cmd()
        .args(["check", "--unknown"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output3.status.code(), Some(2), "stop should exit 2");
}
