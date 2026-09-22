use assert_cmd::Command;
use std::path::Path;

const T_SCHEMA: &str = r#"
document:
  title:
    pattern: "^T-\\d+:"
  sections:
    - name: 状況
      statement:
        required: false
"#;

fn write_file(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(&path, content).unwrap();
    path
}

fn mds() -> Command {
    Command::cargo_bin("mds").unwrap()
}

// @kotowari[REQ-schema-005]
#[test]
fn version_prints_mds_version_to_stdout_and_exits_zero() {
    let output = mds().arg("--version").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("mds "), "stdout: {stdout}");
}

// @kotowari[REQ-schema-005, REQ-schema-040]
#[test]
fn ast_outputs_mdast_json_for_the_adr_fixture() {
    let mut cmd = Command::cargo_bin("mds").unwrap();
    let output = cmd.args(["ast", "fixtures/adr/0001.md"]).output().unwrap();
    assert!(output.status.success());
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["type"], "root");
    let types: Vec<&str> = json["children"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n["type"].as_str().unwrap())
        .collect();
    assert!(types.contains(&"heading"));
    assert!(types.contains(&"list"));
    assert!(types.contains(&"paragraph"));
    assert!(json.get("position").is_none());
    assert!(json["children"][0].get("position").is_none());
}

// @kotowari[REQ-schema-009, REQ-schema-042, REQ-schema-043]
#[test]
fn ast_with_format_text_stops_with_argument_error() {
    let mut cmd = Command::cargo_bin("mds").unwrap();
    cmd.args(["ast", "fixtures/adr/0001.md", "--format", "text"]);
    let output = cmd.output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("mds: argument_error:"), "stderr: {stderr}");
}

// @kotowari[REQ-schema-009, REQ-schema-042]
#[test]
fn unknown_flag_stops_with_argument_error() {
    let output = mds()
        .args(["check", "fixtures/adr/0001.md", "--bogus"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("mds: argument_error:"), "stderr: {stderr}");
}

// @kotowari[REQ-schema-036]
#[test]
fn ast_schema_outputs_typed_tree() {
    let output = mds()
        .args(["ast", "fixtures/adr/0001.md", "--schema"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["type"], "adr");
    assert_eq!(json["id"], "ADR-0001");
    assert_eq!(
        json["sections"]["reasons"],
        serde_json::json!(["- 理由その1", "- 理由その2"])
    );
}

// @kotowari[REQ-schema-009, REQ-schema-014, REQ-schema-042]
#[test]
fn ast_schema_without_schema_stops() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write_file(dir.path(), "doc.md", "# 題名\n");
    let output = mds()
        .args(["ast", doc.to_str().unwrap(), "--schema"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_not_found"));
}

// @kotowari[REQ-schema-006, EX-schema-003]
#[test]
fn check_clean_document_exits_zero_with_no_output() {
    let output = mds()
        .args(["check", "fixtures/adr/0001.md"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
}

// @kotowari[REQ-schema-007]
#[test]
fn check_json_outputs_empty_files_array_for_clean_file() {
    let output = mds()
        .args(["check", "fixtures/adr/0001.md", "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json, serde_json::json!({ "files": [] }));
}

// @kotowari[REQ-schema-007, REQ-schema-010]
#[test]
fn check_json_outputs_empty_files_array_for_clean_directory() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n本文。\n",
    );
    let output = mds()
        .args(["check", dir.path().to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json, serde_json::json!({ "files": [] }));
}

// @kotowari[REQ-schema-006, REQ-schema-007]
#[test]
fn check_document_with_findings_exits_one_with_text() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n本文。\n\n## 補足\n\n本文。\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&format!("{}:10: undeclared_heading:", doc.display())));
}

// @kotowari[REQ-schema-007, REQ-schema-008]
#[test]
fn check_json_output_has_files_with_findings() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n本文。\n\n## 補足\n\n本文。\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let files = json["files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    let finding = &files[0]["findings"][0];
    assert_eq!(finding["kind"], "undeclared_heading");
    assert_eq!(finding["severity"], "error");
    assert_eq!(finding["path"], doc.to_str().unwrap());
    assert_eq!(finding["line"], 10);
    assert!(finding.get("detail").is_some());
}

// @kotowari[REQ-schema-007]
#[test]
fn check_format_defaults_to_text() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n本文。\n\n## 補足\n\n本文。\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&format!("{}:10: undeclared_heading:", doc.display())));
    assert!(!stdout.trim_start().starts_with('{'));
}

// @kotowari[REQ-schema-002, EX-schema-001]
#[test]
fn check_open_relaxes_undeclared_headings() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n本文。\n\n## 補足\n\n本文。\n",
    );
    let closed = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(closed.status.code(), Some(1));
    let opened = mds()
        .args(["check", doc.to_str().unwrap(), "--open"])
        .output()
        .unwrap();
    assert_eq!(opened.status.code(), Some(0));
    assert!(opened.stdout.is_empty());
}

// @kotowari[REQ-schema-003, REQ-schema-031]
#[test]
fn check_undeclared_child_under_declared_field_exits_one() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "schema.yaml",
        "document:\n  title:\n    pattern: \"^T-\\\\d+:\"\n  sections:\n    - name: 状況\n      fields:\n        - name: 状態\n      statement:\n        required: false\n",
    );
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n- 状態: 承認済み\n  - 子箇条書き\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("undeclared_line"),
        "宣言済みフィールド行の下の子リストは undeclared_line になる: {stdout}"
    );
}

// @kotowari[REQ-schema-021]
#[test]
fn check_child_bullet_when_uses_sibling_field_exits_one() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "schema.yaml",
        "document:\n  title:\n    pattern: \"^T-\\\\d+:\"\n  sections:\n    - name: 決定\n      bullets:\n        repeat: { min: 0 }\n        children:\n          fields:\n            - name: 種類\n          bullets:\n            repeat: { min: 0 }\n            pattern: \"^子\"\n            when: { field: 種類, eq: algorithm }\n",
    );
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 決定\n\n- 親\n  - 種類: algorithm\n  - 違反\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("bullet_pattern_mismatch"),
        "children.bullets の when が兄弟の children.fields を参照して pattern が効く: {stdout}"
    );
}

// @kotowari[REQ-schema-039]
#[test]
fn check_item_child_field_extract_stops_with_schema_invalid() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "schema.yaml",
        "document:\n  title:\n    pattern: \"^T-\\\\d+:\"\n  sections:\n    - name: 要求\n      item:\n        bullets:\n          repeat: { min: 0 }\n          children:\n            fields:\n              - name: superseded_by\n                extract: superseded_by\n",
    );
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 要求\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("schema_invalid"),
        "item の bullets.children.fields の extract は schema_invalid で停止する: {stderr}"
    );
}

// @kotowari[REQ-schema-008]
#[test]
fn check_skips_bom_and_counts_crlf_lines() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "\u{FEFF}---\r\n$schema: ./schema.yaml\r\n---\r\n# T-1: 例\r\n\r\n## 状況\r\n\r\n## 補足",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&format!("{}:8: undeclared_heading:", doc.display())));
}

// @kotowari[REQ-schema-008, REQ-schema-022]
#[test]
fn check_missing_title_has_no_line_number() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n## 状況\n\n本文。\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(&format!("{}: missing_title:", doc.display())));
    assert!(!stdout.contains(":1: missing_title:"));
}

// @kotowari[REQ-schema-009, REQ-schema-011, REQ-schema-042]
#[test]
fn check_missing_schema_stops_with_schema_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write_file(dir.path(), "doc.md", "# 題名\n");
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_not_found"));
}

// @kotowari[REQ-schema-009, REQ-schema-011, REQ-schema-042]
#[test]
fn check_unresolvable_schema_stops_with_schema_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./nope.yaml\n---\n# 題名\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_not_found"));
}

// @kotowari[REQ-schema-009, REQ-schema-042]
#[test]
fn check_unreadable_file_stops() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("missing.md");
    let output = mds()
        .args(["check", missing.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unreadable_file"));
}

// @kotowari[REQ-schema-009, REQ-schema-014, EX-schema-004, REQ-schema-042]
#[test]
fn check_broken_frontmatter_stops_with_frontmatter_invalid() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write_file(dir.path(), "doc.md", "---\n$schema: [\n---\n# 題名\n");
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("frontmatter_invalid"));
}

// @kotowari[REQ-schema-009, REQ-schema-014]
#[test]
fn values_with_broken_frontmatter_stops_with_frontmatter_invalid() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write_file(dir.path(), "doc.md", "---\n$schema: [\n---\n# 題名\n");
    let output = mds()
        .args(["values", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("frontmatter_invalid"));
}

// @kotowari[REQ-schema-009, REQ-schema-014]
#[test]
fn ast_schema_with_broken_frontmatter_stops_with_frontmatter_invalid() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write_file(dir.path(), "doc.md", "---\n$schema: [\n---\n# 題名\n");
    let output = mds()
        .args(["ast", doc.to_str().unwrap(), "--schema"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("frontmatter_invalid"));
}

// @kotowari[REQ-schema-007]
#[test]
fn values_text_defaults_to_indented_text() {
    let output = mds()
        .args(["values", "fixtures/adr/0001.md"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id: ADR-0001\n"));
    assert!(stdout.contains("status: 承認済み\n"));
    assert!(stdout.contains("sections:\n  context: 背景の段落。\n"));
    assert!(stdout.contains("  reasons:\n    1. - 理由その1\n    2. - 理由その2\n"));
}

// @kotowari[REQ-schema-007]
#[test]
fn values_text_indents_multiline_value_two_deeper_than_the_key() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "schema.yaml",
        "document:\n  sections:\n    - name: 状況\n      extract: body\n      statement:\n        required: false\n",
    );
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n## 状況\n\nこれは\n続きの文\n",
    );
    let output = mds()
        .args(["values", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("body: これは\n  続きの文"),
        "stdout: {stdout}"
    );
}

// @kotowari[REQ-schema-036, EX-schema-013]
#[test]
fn values_json_is_nested_by_path() {
    let output = mds()
        .args(["values", "fixtures/adr/0001.md", "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["id"], "ADR-0001");
    assert_eq!(json["title"], "ADR-0001: テストの印は @kotowari[ID, ...]");
    assert_eq!(json["status"], "承認済み");
    assert_eq!(json["date"], "2026-09-16");
    assert_eq!(json["sections"]["context"], "背景の段落。");
    assert_eq!(json["sections"]["decision"], "判断の内容。");
    assert_eq!(
        json["sections"]["reasons"],
        serde_json::json!(["- 理由その1", "- 理由その2"])
    );
}

// @kotowari[REQ-schema-009, REQ-schema-014, REQ-schema-042]
#[test]
fn values_without_schema_stops_with_schema_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write_file(dir.path(), "doc.md", "# 題名\n");
    let output = mds()
        .args(["values", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_not_found"));
}

const DIR_SCHEMA: &str = r#"
document:
  title:
    pattern: "^T-\\d+:"
  sections:
    - name: 状況
      statement:
        required: false
"#;

fn good_doc() -> &'static str {
    "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n本文。\n"
}

fn bad_doc() -> &'static str {
    "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n本文。\n\n## 補足\n\n本文。\n"
}

// @kotowari[REQ-schema-010]
#[test]
fn check_directory_reports_all_failing_documents() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", DIR_SCHEMA);
    write_file(dir.path(), "good.md", good_doc());
    write_file(dir.path(), "bad.md", bad_doc());
    write_file(dir.path(), "noschema.md", "# スキーマなし\n");
    let output = mds()
        .args(["check", dir.path().to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let files = json["files"].as_array().unwrap();
    assert_eq!(files.len(), 1);
    assert!(files[0]["path"].as_str().unwrap().ends_with("bad.md"));
    assert_eq!(files[0]["findings"][0]["kind"], "undeclared_heading");
}

// @kotowari[REQ-schema-010, REQ-schema-044]
#[test]
fn check_directory_skips_hidden_directories() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", DIR_SCHEMA);
    write_file(dir.path(), "good.md", good_doc());
    write_file(dir.path(), ".hidden/bad.md", bad_doc());
    let output = mds()
        .args(["check", dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
}

// @kotowari[REQ-schema-010, REQ-schema-044]
#[test]
fn check_directory_does_not_follow_symlinks() {
    let outside = tempfile::tempdir().unwrap();
    let external = write_file(outside.path(), "external.md", bad_doc());
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", DIR_SCHEMA);
    write_file(dir.path(), "good.md", good_doc());
    std::os::unix::fs::symlink(&external, dir.path().join("link.md")).unwrap();
    let output = mds()
        .args(["check", dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
}

// @kotowari[REQ-schema-043]
#[test]
fn stop_does_not_carry_the_referenced_file_contents() {
    // スキーマとして読めないファイルを `$schema` に指定すると、パーサの
    // メッセージが参照先の中身を引用する。`$schema` は文書側が自由に書ける
    // ので、そのまま流すと読める任意のファイルの断片が標準エラーへ出る
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "not-a-schema.txt",
        "FIRST_LINE=alpha\nSECOND_LINE=bravo\nTHIRD_LINE=charlie\n",
    );
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./not-a-schema.txt\n---\n# T-1: x\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr.lines().count(), 1, "停止は1行で出す: {stderr}");
    assert!(
        !stderr.contains("SECOND_LINE") && !stderr.contains("THIRD_LINE"),
        "参照先ファイルの中身が標準エラーに出ている: {stderr}"
    );
}

// @kotowari[REQ-schema-010, REQ-schema-009, REQ-schema-042]
#[test]
fn check_directory_stops_on_invalid_schema() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", "document: [\n");
    write_file(dir.path(), "doc.md", good_doc());
    let output = mds()
        .args(["check", dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_invalid"));
}

// @kotowari[REQ-schema-010, REQ-schema-009, REQ-schema-042, REQ-schema-043, REQ-schema-053]
#[test]
fn check_directory_stops_on_frontmatter_invalid_and_outputs_no_findings() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", DIR_SCHEMA);
    write_file(dir.path(), "good.md", good_doc());
    write_file(
        dir.path(),
        "bad.md",
        "---\n$schema:\n---\n# T-1: 例\n\n## 補足\n\n本文。\n",
    );
    let output = mds()
        .args(["check", dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("frontmatter_invalid"));
    assert!(output.stdout.is_empty(), "停止のときは指摘を出力しない");
}

// @kotowari[REQ-schema-010, REQ-schema-009, REQ-schema-042, REQ-schema-043]
#[test]
fn check_directory_stops_on_schema_not_found_and_outputs_no_findings() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", DIR_SCHEMA);
    write_file(dir.path(), "good.md", good_doc());
    write_file(
        dir.path(),
        "bad.md",
        "---\n$schema: ./missing.yaml\n---\n# T-1: 例\n\n## 補足\n\n本文。\n",
    );
    let output = mds()
        .args(["check", dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("schema_not_found"));
    assert!(output.stdout.is_empty(), "停止のときは指摘を出力しない");
}

// @kotowari[REQ-schema-010, REQ-schema-009, REQ-schema-042]
#[test]
fn check_directory_stops_on_unreadable_file() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let path = write_file(dir.path(), "secret.md", good_doc());
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = mds()
        .args(["check", dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unreadable_file"));
}

// @kotowari[REQ-schema-010]
#[test]
fn check_fixtures_directory_passes() {
    let output = mds().args(["check", "fixtures"]).output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty());
}

// @kotowari[REQ-schema-010, REQ-schema-035]
#[test]
fn ir_fixture_passes_check_and_extracts_values() {
    let checked = mds()
        .args(["check", "fixtures/ir/kanji.md"])
        .output()
        .unwrap();
    assert_eq!(checked.status.code(), Some(0));

    let values = mds()
        .args(["values", "fixtures/ir/kanji.md"])
        .output()
        .unwrap();
    assert_eq!(values.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&values.stdout);
    assert!(stdout.contains("title: 印の仕様"));
    assert!(stdout.contains("scope: この文書は、印の構文と意味を正規化した仕様を扱う。"));
    assert!(stdout.contains("用語: 印"));
    assert!(stdout.contains("Scenario: 印を書く"));
}

// @kotowari[REQ-schema-035, REQ-schema-031]
#[test]
fn decision_record_fixture_passes_check_and_extracts_values() {
    // 判断の記録（決定の行 `- 決定1 ...` + 子の `superseded_by`）の書式を、
    // children で宣言したスキーマで端から端まで検査・抽出する。
    let checked = mds()
        .args(["check", "fixtures/decision/records.md"])
        .output()
        .unwrap();
    assert_eq!(checked.status.code(), Some(0), "check が通る");

    let values = mds()
        .args(["values", "fixtures/decision/records.md", "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(values.status.code(), Some(0));
    let json: serde_json::Value = serde_json::from_slice(&values.stdout).unwrap();
    assert_eq!(json["title"], "判断の記録");
    // 決定の行の抽出要素は元の行の文字列
    let decisions = json["decisions"].as_array().unwrap();
    assert_eq!(decisions.len(), 2);
    assert_eq!(
        decisions[0],
        serde_json::json!(
            "- 決定1 これは実在しないダミーの決定である。判断の記録の書式を検査するためにだけ存在する"
        )
    );
    assert_eq!(
        decisions[1],
        serde_json::json!(
            "- 決定2 これは実在しないダミーの決定である。既定の置き場所をこの書式で表す"
        )
    );
    // 子フィールド superseded_by は自身の extract で別の配置パスに値が出る
    assert_eq!(
        json["superseded_by"],
        "[example-log.md#S1](./example-log.md#S1)（新しい記録は書かず、既存は残す）"
    );
}

// @kotowari[REQ-schema-009, REQ-schema-043]
#[test]
fn no_subcommand_stops_with_a_usage_hint_not_the_about_text() {
    let output = mds().output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("Validate Markdown documents against"),
        "about の文をそのまま停止の説明にしない: {stderr}"
    );
    assert!(stderr.contains("--help"), "使い方への案内を出す: {stderr}");
}

const NODE_SCHEMA: &str = r#"name: t
document:
  sections:
    - name: 要求
      item:
        id: "REQ-\\d+"
        repeat: { min: 0 }
        fields:
          - name: 種類
"#;

// @kotowari[REQ-schema-008]
#[test]
fn check_json_finding_carries_the_node_name_and_the_raw_line() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", NODE_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n## 要求\n\n### REQ-001: `名前`\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let finding = &json["files"][0]["findings"][0];
    assert_eq!(finding["kind"], "missing_required_field");
    assert_eq!(finding["line"], 6);
    assert_eq!(finding["node"], "種類");
    assert_eq!(finding["text"], "### REQ-001: `名前`");
}

// @kotowari[REQ-schema-055]
#[test]
fn check_json_undeclared_line_has_rule_kind() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", T_SCHEMA);
    let doc = write_file(
        dir.path(),
        "doc.md",
        "---\n$schema: ./schema.yaml\n---\n# T-1: 例\n\n## 状況\n\n- 種類: ubiquitous\n",
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let finding = &json["files"][0]["findings"][0];
    assert_eq!(finding["kind"], "undeclared_line");
    assert_eq!(finding["rule_kind"], "field");
}

/// `mds values --format json` の出力を読む。`schema` は文書の隣に置く。
fn values_json(schema: &str, doc_body: &str) -> serde_json::Value {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", schema);
    let doc = write_file(
        dir.path(),
        "doc.md",
        &format!("---\n$schema: ./schema.yaml\n---\n{doc_body}"),
    );
    let output = mds()
        .args(["values", doc.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

/// `mds values` を走らせて終了コードと標準エラー・標準出力を返す。
fn values_run(schema: &str, doc_body: &str) -> (Option<i32>, String, String) {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", schema);
    let doc = write_file(
        dir.path(),
        "doc.md",
        &format!("---\n$schema: ./schema.yaml\n---\n{doc_body}"),
    );
    let output = mds()
        .args(["values", doc.to_str().unwrap()])
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// `mds check --format json` の findings を読む。
fn check_findings(schema: &str, doc_body: &str) -> Vec<serde_json::Value> {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", schema);
    let doc = write_file(
        dir.path(),
        "doc.md",
        &format!("---\n$schema: ./schema.yaml\n---\n{doc_body}"),
    );
    let output = mds()
        .args(["check", doc.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    json["files"][0]["findings"].as_array().unwrap().clone()
}

// @kotowari[EX-schema-018]
#[test]
fn values_json_keys_table_rows_by_declared_header_or_by_column_position() {
    let schema = r#"
document:
  sections:
    - name: 宣言あり
      table:
        header: [用語, 意味]
        extract: { path: declared, value: cells }
    - name: 宣言なし
      table:
        extract: { path: positional, value: cells }
"#;
    let doc = "## 宣言あり\n\n|  | 意味 |\n|---|---|\n| 印 | しるし |\n\n## 宣言なし\n\n| a | a |\n|---|---|\n| 1 | 2 |\n";
    let v = values_json(schema, doc);
    assert_eq!(
        v["declared"],
        serde_json::json!([{ "cells": { "用語": "印", "意味": "しるし" } }]),
        "header を宣言した表の行は宣言した名前を鍵にしたオブジェクトになる"
    );
    assert_eq!(
        v["positional"],
        serde_json::json!([{ "cells": ["1", "2"] }]),
        "header を宣言しない表の行は列の位置の配列になる"
    );
}

// @kotowari[EX-schema-019]
#[test]
fn values_json_nests_repeated_table_rows_per_table_with_the_data_row_lines() {
    let schema = r#"
document:
  sections:
    - name: 決定表
      table:
        repeat: { min: 0 }
        header: [a, b]
        extract: { path: tables, value: cells, of: { line: line } }
"#;
    let doc = "## 決定表\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n| 5 | 6 |\n\n| a | b |\n|---|---|\n| 7 | 8 |\n| 9 | 10 |\n| 11 | 12 |\n";
    let v = values_json(schema, doc);
    assert_eq!(
        v["tables"],
        serde_json::json!([
            [
                { "cells": { "a": "1", "b": "2" }, "line": 8 },
                { "cells": { "a": "3", "b": "4" }, "line": 9 },
                { "cells": { "a": "5", "b": "6" }, "line": 10 }
            ],
            [
                { "cells": { "a": "7", "b": "8" }, "line": 14 },
                { "cells": { "a": "9", "b": "10" }, "line": 15 },
                { "cells": { "a": "11", "b": "12" }, "line": 16 }
            ]
        ]),
        "行ごとの行番号はデータ行を指し、配置パスの直下は表ごとの段になる"
    );
}

// @kotowari[EX-schema-020]
#[test]
fn values_json_splits_a_statement_with_derived_values_into_one_element_per_line() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "^REQ-[0-9]+$"
        extract: { path: requirements, of: { id: id } }
        fields:
          - name: 種類
            extract: kind
        statement:
          extract: { path: text, value: value, of: { line: line, raw: raw } }
"#;
    let doc = "## 要求\n\n### REQ-001: 例\n\n- 種類: ubiquitous\n\n  一覧の行の継続段落。\n\n1行目\n  字下げの2行目  \n3行目\n\n続く段落\n";
    let v = values_json(schema, doc);
    assert_eq!(
        v["requirements"]["text"],
        serde_json::json!([
            { "value": "1行目", "line": 12, "raw": "1行目" },
            { "value": "字下げの2行目", "line": 13, "raw": "  字下げの2行目  " },
            { "value": "3行目", "line": 14, "raw": "3行目" },
            { "value": "続く段落", "line": 16, "raw": "続く段落" }
        ]),
        "行の数と同じ数の要素が出て、値は前後の空白を取り除き、生の行は字下げと末尾の空白を残す。一覧の行の継続段落は入らない"
    );
}

// @kotowari[EX-schema-021]
#[test]
fn values_json_keeps_shorthand_extractions_unsplit() {
    let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        extract: text
      table:
        required: false
        header: [a, b]
        extract: rows
"#;
    let doc = "## 記録\n\n1行目\n2行目\n\n| a | b |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |\n";
    let v = values_json(schema, doc);
    assert_eq!(
        v["text"],
        serde_json::json!("1行目\n2行目"),
        "略記の文は1つの文字列になり、要素ごとのオブジェクトを作らない"
    );
    assert_eq!(
        v["rows"],
        serde_json::json!([
            { "a": "1", "b": "2" },
            { "a": "3", "b": "4" }
        ]),
        "略記の表は行の並びになり、要素ごとのオブジェクトを作らない"
    );
}

// @kotowari[EX-schema-022]
#[test]
fn values_without_a_path_in_the_extract_declaration_stops() {
    let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        extract:
          value: text
"#;
    let (code, _stdout, stderr) = values_run(schema, "## 記録\n\n本文。\n");
    assert_eq!(code, Some(2), "stderr: {stderr}");
}

// @kotowari[EX-schema-023]
#[test]
fn values_json_omits_the_value_key_when_the_extract_declares_only_derived_values() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "^REQ-[0-9]+$"
        extract: { path: requirements, of: { id: id, line: line } }
        fields:
          - name: 種類
            extract: kind
"#;
    let doc = "## 要求\n\n### REQ-001: 例\n\n- 種類: ubiquitous\n";
    let v = values_json(schema, doc);
    assert_eq!(
        v["requirements"],
        serde_json::json!({ "id": "REQ-001", "line": 6, "kind": "ubiquitous" }),
        "value を書かない要素は導かれる値の鍵と内側のノードの配置パスだけを持つ"
    );
}

// @kotowari[EX-schema-024]
#[test]
fn values_with_an_unaccepted_derived_word_stops() {
    let schema = r#"
document:
  sections:
    - name: 記録
      statement:
        extract:
          path: lines
          value: text
          of: { column: column }
"#;
    let (code, _stdout, stderr) = values_run(schema, "## 記録\n\n本文。\n");
    assert_eq!(code, Some(2), "stderr: {stderr}");
}

// @kotowari[EX-schema-025]
#[test]
fn values_json_gives_the_raw_line_for_the_title_and_for_table_rows() {
    let schema = r#"
document:
  title:
    pattern: "^T-[0-9]+:"
    extract:
      path: title
      value: value
      of: { line: line, raw: raw }
  sections:
    - name: 用語集
      table:
        header: [用語, 意味]
        extract:
          path: glossary
          value: cells
          of: { line: line, raw: raw }
"#;
    let doc = "# T-1:  題名  \n\n## 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 | しるし |\n";
    let v = values_json(schema, doc);
    assert_eq!(
        v["title"]["raw"],
        serde_json::json!("# T-1:  題名  "),
        "題名の生の行は見出しの行をそのまま出す"
    );
    assert_eq!(
        v["glossary"][0]["raw"],
        serde_json::json!("| 印 | しるし |"),
        "表の行の生の行はそのデータ行をそのまま出す"
    );
}

const FINDING_POSITION_SCHEMA: &str = r#"
document:
  sections:
    - name: 要求
      repeat: { min: 0 }
      item:
        id: "^REQ-[0-9]+$"
        repeat: { min: 0 }
        fields:
          - name: 種類
            enum: [ubiquitous]
    - name: 用語集
      repeat: { min: 0 }
      table:
        header: [用語, 意味]
"#;

// @kotowari[EX-schema-026]
#[test]
fn check_json_finding_line_points_at_the_node_or_at_the_one_that_should_contain_it() {
    let doc = "## 要求\n\n### REQ-001: 欠落\n\n### REQ-002: 形\n\n- 種類: bogus\n\n## 用語集\n\n| 用語 | 意味 |\n|---|---|\n| 印 |\n";
    let findings = check_findings(FINDING_POSITION_SCHEMA, doc);
    let by_kind = |kind: &str| -> serde_json::Value {
        findings
            .iter()
            .find(|f| f["kind"] == kind)
            .unwrap_or_else(|| panic!("{kind} が無い: {findings:?}"))
            .clone()
    };
    assert_eq!(
        by_kind("missing_required_field")["line"],
        6,
        "欠落の指摘の行はその項目の見出しの行"
    );
    assert_eq!(
        by_kind("field_enum_invalid")["line"],
        10,
        "形に合わない値のフィールド行の指摘はそのフィールド行"
    );
    assert_eq!(
        by_kind("table_header_mismatch")["line"],
        16,
        "列の足りない表の指摘はその行"
    );
}

const NODE_NAME_SCHEMA: &str = r#"
document:
  title:
    pattern: "^T-[0-9]+:"
  sections:
    - name: 要求
      item:
        id: "^REQ-[0-9]+$"
        fields:
          - name: 種類
            enum: [ubiquitous]
"#;

// @kotowari[EX-schema-027]
#[test]
fn check_json_finding_carries_the_declared_name_only_where_the_node_has_one() {
    let doc = "# 合わない題名\n\n## 要求\n\n### REQ-001: 欠落\n";
    let findings = check_findings(NODE_NAME_SCHEMA, doc);
    let field = findings
        .iter()
        .find(|f| f["kind"] == "missing_required_field")
        .unwrap();
    assert_eq!(
        field["node"], "種類",
        "フィールド行の指摘は宣言した名前を持つ"
    );
    let title = findings
        .iter()
        .find(|f| f["kind"] == "title_pattern_mismatch")
        .unwrap();
    assert!(
        title.get("node").is_none(),
        "題名の指摘はノードの名前を持たない: {title}"
    );
}

// @kotowari[EX-schema-028]
#[test]
fn check_json_invalid_id_finding_carries_the_heading_line_verbatim() {
    let doc = "# T-1: 例\n\n## 要求\n\n###  BAD-1:  `名前`  \n";
    let findings = check_findings(NODE_NAME_SCHEMA, doc);
    let invalid = findings
        .iter()
        .find(|f| f["kind"] == "invalid_id")
        .unwrap_or_else(|| panic!("invalid_id が無い: {findings:?}"));
    assert_eq!(
        invalid["text"], "###  BAD-1:  `名前`  ",
        "生の行は文書のその行と一文字も違わない"
    );
}

// @kotowari[EX-schema-029]
#[test]
fn values_with_colliding_element_keys_stops_without_reporting_the_document() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        id: "^REQ-[0-9]+$"
        extract: { path: requirements, of: { id: id } }
        fields:
          - name: 種類
            extract: id
"#;
    let (code, stdout, stderr) =
        values_run(schema, "## 要求\n\n### REQ-001: 例\n\n- 種類: ubiquitous\n");
    assert_eq!(code, Some(2), "stderr: {stderr}");
    assert!(
        stderr.contains("schema_invalid"),
        "標準エラーはスキーマが形に合わないことを知らせる: {stderr}"
    );
    assert!(stdout.is_empty(), "文書から読んだ値は出さない: {stdout}");
}

// @kotowari[EX-schema-030]
#[test]
fn values_with_paths_sharing_only_a_level_above_the_dot_does_not_stop() {
    let sibling = r#"
document:
  sections:
    - name: 要求
      item:
        id: "^REQ-[0-9]+$"
        extract: { path: requirements, of: { id: id } }
        fields:
          - name: 甲
            extract: a.b
          - name: 乙
            extract: a.c
"#;
    let nested = r#"
document:
  sections:
    - name: 要求
      item:
        id: "^REQ-[0-9]+$"
        extract: { path: requirements, of: { id: id } }
        fields:
          - name: 甲
            extract: a
          - name: 乙
            extract: a.b
"#;
    let doc = "## 要求\n\n### REQ-001: 例\n\n- 甲: 1\n- 乙: 2\n";
    let (code, _stdout, stderr) = values_run(sibling, doc);
    assert_eq!(code, Some(0), "\"a.b\" と \"a.c\" は重複でない: {stderr}");
    let (code, _stdout, stderr) = values_run(nested, doc);
    assert_eq!(code, Some(2), "\"a\" と \"a.c\" は重複で停止する: {stderr}");
}

/// スキーマと本文を一時ディレクトリに置き、`mds <subcommand> <doc> --format json` を走らせる。
/// 終了コードと標準出力の JSON（読めなければ Null）と標準エラーを返す。
fn mds_json(
    schema: &str,
    doc_body: &str,
    subcommand: &str,
) -> (Option<i32>, serde_json::Value, String) {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "schema.yaml", schema);
    let doc = write_file(
        dir.path(),
        "doc.md",
        &format!("---\n$schema: ./schema.yaml\n---\n{doc_body}"),
    );
    let output = mds()
        .args([subcommand, doc.to_str().unwrap(), "--format", "json"])
        .output()
        .unwrap();
    let json = serde_json::from_slice(&output.stdout).unwrap_or(serde_json::Value::Null);
    (
        output.status.code(),
        json,
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// `mds check --format json` の全ファイルの指摘を1つの並びにする。
fn all_findings(check_json: &serde_json::Value) -> Vec<serde_json::Value> {
    check_json["files"]
        .as_array()
        .map(|files| {
            files
                .iter()
                .flat_map(|f| f["findings"].as_array().cloned().unwrap_or_default())
                .collect()
        })
        .unwrap_or_default()
}

// @kotowari[EX-schema-040]
#[test]
fn ex_schema_040_schema_without_reading_reads_by_paragraph() {
    let rules = r#"
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: { path: items, of: { id: id } }
        statement:
          repeat: { max: 1 }
          extract: text
"#;
    let without = format!("document:{rules}");
    let paragraph = format!("reading: paragraph\ndocument:{rules}");
    let doc = "## 要求\n\n### REQ-1: 例\n\n一行目\n二行目\n";

    let (code_a, check_a, stderr_a) = mds_json(&without, doc, "check");
    let (code_b, check_b, stderr_b) = mds_json(&paragraph, doc, "check");
    assert_eq!(code_a, Some(0), "stderr: {stderr_a}");
    assert_eq!(code_b, Some(0), "stderr: {stderr_b}");
    assert!(
        all_findings(&check_a).is_empty(),
        "2行の段落は1つの文: {check_a}"
    );
    assert_eq!(check_a, check_b);

    let (code_a, values_a, stderr_a) = mds_json(&without, doc, "values");
    let (code_b, values_b, stderr_b) = mds_json(&paragraph, doc, "values");
    assert_eq!(code_a, Some(0), "stderr: {stderr_a}");
    assert_eq!(code_b, Some(0), "stderr: {stderr_b}");
    assert_eq!(values_a, values_b);
    assert_eq!(
        values_a["items"][0]["text"],
        serde_json::json!(["一行目\n二行目"]),
        "2行の段落は1つの文の要素"
    );
}

// @kotowari[EX-schema-041]
#[test]
fn ex_schema_041_unknown_reading_value_stops() {
    let schema = "reading: word\ndocument:\n  sections:\n    - name: 要求\n";
    let (code, _json, stderr) = mds_json(schema, "## 要求\n", "check");
    assert_eq!(code, Some(2), "stderr: {stderr}");
    assert!(stderr.contains("schema_invalid"), "stderr: {stderr}");
}

// @kotowari[EX-schema-035]
#[test]
fn ex_schema_035_paragraph_reading_absorbs_following_lines_and_ignores_quotes() {
    let schema = r#"
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        fields:
          - name: 種類
        bullets:
          required: false
"#;
    let doc = "## 要求\n\n### REQ-1: 例\n\n- 種類: ubiquitous\n続く行\n\n  字下げした継続段落\n\n> 引用の行\n";
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(0), "stderr: {stderr} json: {json}");
    assert!(all_findings(&json).is_empty(), "{json}");
}

/// 種別が`文`の undeclared_line の指摘が指す行番号。
fn undeclared_statement_lines(findings: &[serde_json::Value]) -> Vec<u64> {
    findings
        .iter()
        .filter(|f| f["kind"] == "undeclared_line" && f["rule_kind"] == "statement")
        .map(|f| f["line"].as_u64().unwrap())
        .collect()
}

// @kotowari[EX-schema-031]
#[test]
fn ex_schema_031_line_reading_keeps_the_line_after_a_field_line_as_a_statement() {
    let schema = r#"
reading: line
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: { path: items, of: { id: id } }
        fields:
          - name: 種類
            extract: kind
        statement:
          extract: text
"#;
    let doc = "## 要求\n\n### REQ-1: 例\n\n- 種類: ubiquitous\n次の行\n";
    let (code, json, stderr) = mds_json(schema, doc, "values");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    assert_eq!(json["items"][0]["kind"], "ubiquitous", "{json}");
    assert_eq!(json["items"][0]["text"], "次の行", "{json}");
}

// @kotowari[EX-schema-032]
#[test]
fn ex_schema_032_line_reading_counts_indented_quote_html_and_image_lines_as_statements() {
    let schema = r#"
reading: line
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        bullets:
          required: false
"#;
    // 行番号: frontmatter が1〜3行目、"## 要求" が4行目
    let doc = "## 要求\n\n### REQ-1: 例\n\n- 一覧の行\n\n  字下げした行\n\n> 引用の行\n\n<div>HTML の行</div>\n\n![画像](img.png)\n";
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(1), "stderr: {stderr}");
    let findings = all_findings(&json);
    assert_eq!(
        undeclared_statement_lines(&findings),
        vec![10, 12, 14, 16],
        "{json}"
    );
}

// @kotowari[EX-schema-033]
#[test]
fn ex_schema_033_line_reading_makes_no_setext_heading_or_indented_code_block() {
    let schema = r#"
reading: line
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        extract: { path: items, of: { id: id } }
        statement:
          repeat: { min: 0 }
          extract: { path: lines, value: text, of: { line: line } }
"#;
    // 行番号: "見出しでない文" が8行目
    let doc = "## 要求\n\n### REQ-1: 例\n\n見出しでない文\n---\n\n    字下げした行\n";
    let (code, json, stderr) = mds_json(schema, doc, "values");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    assert_eq!(
        json["items"][0]["lines"],
        serde_json::json!([
            { "text": "見出しでない文", "line": 8 },
            { "text": "---", "line": 9 },
            { "text": "字下げした行", "line": 11 },
        ]),
        "{json}"
    );
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    assert!(all_findings(&json).is_empty(), "{json}");
}

// @kotowari[EX-schema-034]
#[test]
fn ex_schema_034_pipe_line_without_delimiter_is_a_statement_in_both_readings() {
    for reading in ["paragraph", "line"] {
        let schema = format!(
            "reading: {reading}\ndocument:\n  sections:\n    - name: 要求\n      item:\n        repeat: {{ min: 0 }}\n"
        );
        let doc = "## 要求\n\n### REQ-1: 例\n\n| a | b |\n";
        let (code, json, stderr) = mds_json(&schema, doc, "check");
        assert_eq!(code, Some(1), "{reading}: stderr: {stderr}");
        let findings = all_findings(&json);
        assert_eq!(
            undeclared_statement_lines(&findings),
            vec![8],
            "{reading}: {json}"
        );
        assert_eq!(findings.len(), 1, "{reading}: {json}");
    }
}

// @kotowari[EX-schema-039]
#[test]
fn ex_schema_039_line_reading_keeps_an_indented_list_line_a_list_line() {
    let schema = r#"
reading: line
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
        fields:
          - name: 種類
"#;
    // 行番号: "  - b" が9行目
    let doc = "## 要求\n\n### REQ-1: 例\n\n- 種類: ubiquitous\n  - b\n";
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(1), "stderr: {stderr}");
    let findings = all_findings(&json);
    assert!(
        findings.iter().any(|f| f["line"] == 9
            && f["kind"] == "undeclared_line"
            && f["rule_kind"] == "bullets"),
        "{json}"
    );
    assert!(undeclared_statement_lines(&findings).is_empty(), "{json}");
}

// @kotowari[TBL-schema-011]
#[test]
fn tbl_schema_011_line_reading_reads_hash_only_hash_word_and_equals_lines_as_statements() {
    let schema = r#"
reading: line
document:
  sections:
    - name: 要求
      item:
        repeat: { min: 0 }
"#;
    // 行番号: "#" が8行目
    let doc = "## 要求\n\n### REQ-1: 例\n\n#\n#foo\n文の行\n===\n";
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(1), "stderr: {stderr}");
    let findings = all_findings(&json);
    assert_eq!(
        undeclared_statement_lines(&findings),
        vec![8, 9, 10, 11],
        "{json}"
    );
    assert_eq!(findings.len(), 4, "見出しの指摘は出ない: {json}");
}

// @kotowari[EX-schema-036]
#[test]
fn ex_schema_036_select_first_uses_only_the_first_table_whose_header_matches() {
    let schema = r#"
document:
  preamble:
    table:
      header: [a, b]
      select: first
      extract: rows
"#;
    // 行番号: 1つ目の表が4行目、2つ目が8行目、3つ目が12行目
    let doc = "| x | y |\n|---|---|\n| 0 | 0 |\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n| a | b |\n|---|---|\n| 3 | 4 |\n";
    let (code, json, stderr) = mds_json(schema, doc, "values");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    assert_eq!(
        json["rows"],
        serde_json::json!([{ "a": "1", "b": "2" }]),
        "{json}"
    );
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(1), "stderr: {stderr}");
    let findings = all_findings(&json);
    let undeclared_tables: Vec<u64> = findings
        .iter()
        .filter(|f| f["kind"] == "undeclared_line" && f["rule_kind"] == "table")
        .map(|f| f["line"].as_u64().unwrap())
        .collect();
    assert_eq!(undeclared_tables, vec![4, 12], "{json}");
    assert!(
        !findings
            .iter()
            .any(|f| f["kind"] == "table_header_mismatch"),
        "{json}"
    );
    assert_eq!(findings.len(), 2, "{json}");
}

// @kotowari[EX-schema-037, TBL-schema-009]
#[test]
fn ex_schema_037_table_rule_without_select_reports_a_mismatched_header_and_select_needs_header() {
    let header_only = "document:\n  preamble:\n    table:\n      header: [a, b]\n";
    let select_only = "document:\n  preamble:\n    table:\n      select: first\n";
    let doc = "| x | y |\n|---|---|\n| 0 | 0 |\n";
    let (code, json, stderr) = mds_json(header_only, doc, "check");
    assert_eq!(code, Some(1), "stderr: {stderr}");
    assert!(
        all_findings(&json)
            .iter()
            .any(|f| f["kind"] == "table_header_mismatch"),
        "{json}"
    );
    let (code, _json, stderr) = mds_json(select_only, doc, "check");
    assert_eq!(code, Some(2), "stderr: {stderr}");
    assert!(stderr.contains("schema_invalid"), "stderr: {stderr}");
}

// @kotowari[EX-schema-038]
#[test]
fn ex_schema_038_extra_cells_are_dropped_and_missing_cells_are_reported() {
    let schema = r#"
document:
  preamble:
    table:
      header: [a, b, c]
      extract: rows
"#;
    // 行番号: 4つのセルの行が6行目、2つのセルの行が7行目
    let doc = "| a | b | c |\n|---|---|---|\n| 1 | 2 | 3 | 4 |\n| 5 | 6 |\n";
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(1), "stderr: {stderr}");
    let findings = all_findings(&json);
    assert_eq!(findings.len(), 1, "{json}");
    assert_eq!(findings[0]["line"], 7, "{json}");
    let (code, json, stderr) = mds_json(schema, doc, "values");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    assert_eq!(
        json["rows"][0],
        serde_json::json!({ "a": "1", "b": "2", "c": "3" }),
        "{json}"
    );
}

// @kotowari[EX-schema-050]
#[test]
fn ex_schema_050_select_other_than_first_stops() {
    let schema = "document:\n  preamble:\n    table:\n      header: [a, b]\n      select: last\n";
    let (code, _json, stderr) = mds_json(schema, "| a | b |\n|---|---|\n| 1 | 2 |\n", "check");
    assert_eq!(code, Some(2), "stderr: {stderr}");
    assert!(stderr.contains("schema_invalid"), "stderr: {stderr}");
}

// @kotowari[EX-schema-043]
#[test]
fn ex_schema_043_document_item_reads_items_before_the_first_section() {
    let schema = r#"
document:
  preamble:
    statement:
      extract: scope
  item:
    id: "^FLAG-\\d+$"
    repeat: { min: 0 }
    extract: { path: flags, of: { id: id } }
    statement:
      extract: text
  sections:
    - name: 節
      item:
        id: "^FLAG-\\d+$"
        repeat: { min: 0 }
        extract: { path: section_flags, of: { id: id } }
        statement:
          extract: text
"#;
    let doc = "前置部の文。\n\n### FLAG-1: 直下\n\n直下の文。\n\n## 節\n\n### FLAG-2: 節の下\n\n節の文。\n";
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(0), "stderr: {stderr} {json}");
    assert!(all_findings(&json).is_empty(), "{json}");
    let (code, json, stderr) = mds_json(schema, doc, "values");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    assert_eq!(json["scope"], "前置部の文。", "{json}");
    assert_eq!(
        json["flags"],
        serde_json::json!([{ "id": "FLAG-1", "text": "直下の文。" }]),
        "{json}"
    );
    assert_eq!(
        json["section_flags"],
        serde_json::json!([{ "id": "FLAG-2", "text": "節の文。" }]),
        "{json}"
    );
}

// @kotowari[EX-schema-044]
#[test]
fn ex_schema_044_without_document_item_a_level_three_heading_before_sections_is_reported() {
    let schema = r#"
document:
  sections:
    - name: 節
      item:
        repeat: { min: 0 }
"#;
    // 行番号: 節の前の見出しが4行目
    let doc = "### X-1: 節の前\n\n## 節\n\n### X-2: 節の下\n";
    let (code, json, stderr) = mds_json(schema, doc, "check");
    assert_eq!(code, Some(1), "stderr: {stderr}");
    assert!(
        all_findings(&json)
            .iter()
            .any(|f| f["line"] == 4 && f["kind"] == "undeclared_heading"),
        "{json}"
    );
}

// @kotowari[EX-schema-045]
#[test]
fn ex_schema_045_end_is_the_line_before_the_next_heading_as_deep_or_shallower() {
    let schema = r#"
document:
  sections:
    - name: 一
      extract: { path: one, of: { line: line, end: end } }
      item:
        repeat: { min: 0 }
        extract: { path: items, of: { line: line, end: end } }
        statement:
          required: false
    - name: 二
      extract: { path: two, of: { line: line, end: end } }
      statement:
        required: false
"#;
    // 行番号: "## 一" が4行目、"### A-2" が10行目、"## 二" が14行目、最後の行が16行目
    let doc = "## 一\n\n### A-1: a\n\n文。\n\n### A-2: b\n\n文。\n\n## 二\n\n最後の文。\n";
    let (code, json, stderr) = mds_json(schema, doc, "values");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    assert_eq!(
        json["items"],
        serde_json::json!([{ "line": 6, "end": 9 }, { "line": 10, "end": 13 }]),
        "{json}"
    );
    assert_eq!(
        json["one"],
        serde_json::json!({ "line": 4, "end": 13 }),
        "{json}"
    );
    assert_eq!(
        json["two"],
        serde_json::json!({ "line": 14, "end": 16 }),
        "{json}"
    );
}

// @kotowari[EX-schema-046]
#[test]
fn ex_schema_046_end_outside_items_and_sections_stops() {
    let schema = r#"
document:
  preamble:
    table:
      extract: { path: rows, of: { end: end } }
"#;
    let (code, _json, stderr) = mds_json(schema, "| a |\n|---|\n| 1 |\n", "values");
    assert_eq!(code, Some(2), "stderr: {stderr}");
    assert!(stderr.contains("schema_invalid"), "stderr: {stderr}");
}
