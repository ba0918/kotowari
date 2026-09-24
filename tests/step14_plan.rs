//! "kotowari plan" が計画書を1つ読み、同梱のスキーマで形を検査すること（docs/ir/core/plan.md）

use assert_cmd::Command;
use kotowari_core::{Cli, parse_args};
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

/// 標準エラーの1行目
fn first_stderr_line(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .next()
        .unwrap_or("")
        .to_string()
}

/// 引数の誤りで停止することを見る
fn assert_argument_error(list: &[&str], dir: &Path) {
    let output = cmd().args(list).current_dir(dir).output().unwrap();
    assert_eq!(output.status.code(), Some(2), "{list:?} should stop");
    let first_line = first_stderr_line(&output);
    assert!(
        first_line.starts_with("argument error: "),
        "{list:?}: got {first_line:?}"
    );
    // plan はコマンドなので、知らないコマンドとしての停止ではない（REQ-core-001）
    assert!(
        !first_line.starts_with("argument error: unknown command"),
        "{list:?}: plan should be a command: {first_line:?}"
    );
}

// --- REQ-core-190: plan の引数 ---

// @kotowari[REQ-core-190, EX-core-357]
#[test]
fn ex_core_357_plan_without_a_plan_file_stops() {
    let tmp = TempDir::new().unwrap();
    assert_argument_error(&["plan"], tmp.path());
}

// @kotowari[REQ-core-190, EX-core-342]
#[test]
fn ex_core_342_two_plan_files_stop() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    std::fs::write(tmp.path().join("b.md"), "# b\n").unwrap();
    assert_argument_error(&["plan", "a.md", "b.md"], tmp.path());
}

// @kotowari[REQ-core-190, EX-core-343]
#[test]
fn ex_core_343_config_on_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: docs/ir\n").unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    assert_argument_error(
        &["plan", "--config", ".kotowari/config.yaml", "a.md"],
        tmp.path(),
    );
}

// @kotowari[REQ-core-190]
#[test]
fn req_core_190_config_pointing_to_a_directory_on_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    assert_argument_error(&["plan", "--config", ".kotowari", "a.md"], tmp.path());
}

// @kotowari[REQ-core-002]
#[test]
fn req_core_002_plan_takes_the_format_before_or_after_the_path() {
    for list in [
        ["plan", "--format", "text", "a.md"],
        ["plan", "a.md", "--format", "text"],
        ["--format", "text", "plan", "a.md"],
    ] {
        let parsed = parse_args(&args(&list));
        assert!(
            matches!(parsed, Ok(Cli::Plan { ref path, .. }) if path == Path::new("a.md")),
            "{list:?}: {parsed:?}"
        );
    }
}

// @kotowari[REQ-core-004]
#[test]
fn req_core_004_tool_on_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    assert_argument_error(&["plan", "--tool", "cargo-mutants", "a.md"], tmp.path());
}

// --- REQ-core-191〜REQ-core-197: 計画書を読み、指摘を invalid_plan で出す ---

/// 形の揃った計画書
const VALID_PLAN: &str = include_str!("../fixtures/plans/valid.md");

/// 欄 "Done when" の行が無いステップを持つ計画書
fn plan_without_done_when() -> String {
    let plan: String = VALID_PLAN
        .lines()
        .filter(|line| !line.starts_with("- Done when:"))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(plan, VALID_PLAN, "the fixture should have a Done when line");
    plan
}

/// "## Out of scope" の前に中身の無い節 "## Notes" を足した計画書と、その見出しの行
fn plan_with_unknown_section() -> (String, usize) {
    let plan = VALID_PLAN.replace("## Out of scope", "## Notes\n\n## Out of scope");
    let line = plan.lines().position(|l| l == "## Notes").unwrap() + 1;
    (plan, line)
}

/// 基準のディレクトリ（`.kotowari/` の無い一時ディレクトリ）の "docs/plans/a.md" に計画書を置く
fn project_with_plan(content: &str) -> TempDir {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/plans")).unwrap();
    std::fs::write(tmp.path().join("docs/plans/a.md"), content).unwrap();
    tmp
}

/// "kotowari plan" を実行し、終了コードと標準出力を返す
fn run_plan(dir: &Path, list: &[&str]) -> (Option<i32>, String) {
    let output = cmd().args(list).current_dir(dir).output().unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
    )
}

/// JSON の出力の "findings"
fn findings_of(stdout: &str) -> Vec<serde_json::Value> {
    let json: serde_json::Value = serde_json::from_str(stdout).unwrap();
    json["findings"].as_array().unwrap().clone()
}

// @kotowari[REQ-core-192, REQ-core-193, EX-core-333]
#[test]
fn ex_core_333_a_step_missing_a_field_is_an_invalid_plan_error() {
    let tmp = project_with_plan(&plan_without_done_when());
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert!(!findings.is_empty());
    for f in &findings {
        assert_eq!(f["kind"], "invalid_plan", "{f}");
        assert_eq!(f["severity"], "error", "{f}");
        assert_eq!(f["path"], "docs/plans/a.md", "{f}");
        // detail はスキーマの側の種類に ": " と詳細が続く
        let detail = f["detail"].as_str().unwrap();
        let (kind, rest) = detail.split_once(": ").unwrap_or_else(|| panic!("{detail:?}"));
        assert!(
            !kind.is_empty() && kind.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
            "{detail:?}"
        );
        assert!(!rest.is_empty(), "{detail:?}");
    }
}

// @kotowari[REQ-core-192, REQ-core-193]
#[test]
fn req_core_192_a_valid_plan_has_no_findings() {
    let tmp = project_with_plan(VALID_PLAN);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
    assert!(findings_of(&stdout).is_empty(), "{stdout}");
}

// @kotowari[REQ-core-193]
#[test]
fn req_core_193_a_finding_without_a_line_has_a_null_line() {
    // 題名が無い計画書。題名の欠けはスキーマの側が行を持たずに出す
    let plan: String = VALID_PLAN
        .lines()
        .filter(|line| !line.starts_with("# "))
        .map(|line| format!("{line}\n"))
        .collect();
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert!(
        findings
            .iter()
            .any(|f| f["kind"] == "invalid_plan" && f["line"].is_null()),
        "{stdout}"
    );
}

// @kotowari[REQ-core-193]
#[test]
fn req_core_193_the_line_is_the_line_the_schema_side_reported() {
    // 宣言していない節を、スキーマの側はその見出しの行で指す
    let (plan, line) = plan_with_unknown_section();
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert_eq!(findings.len(), 1, "{stdout}");
    assert_eq!(findings[0]["line"], line, "{stdout}");
}

// @kotowari[REQ-core-025, REQ-core-193]
#[test]
fn req_core_025_plan_text_output_has_one_line_per_finding() {
    let (plan, line) = plan_with_unknown_section();
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md", "--format", "text"]);
    assert_eq!(code, Some(1), "{stdout}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "{stdout}");
    let prefix = format!("docs/plans/a.md:{line} [error] invalid_plan ");
    assert!(lines[0].starts_with(&prefix), "{stdout}");
}

// @kotowari[REQ-core-197, EX-core-344]
#[test]
fn ex_core_344_a_missing_plan_stops_as_an_unreadable_file() {
    let tmp = TempDir::new().unwrap();
    let output = cmd()
        .args(["plan", "docs/plans/none.md"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let first_line = first_stderr_line(&output);
    assert!(first_line.starts_with("unreadable file: "), "{first_line:?}");
}

// @kotowari[REQ-core-197]
#[test]
fn req_core_197_a_directory_given_as_the_plan_stops_as_an_unreadable_file() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/plans")).unwrap();
    let output = cmd()
        .args(["plan", "docs/plans"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let first_line = first_stderr_line(&output);
    assert!(first_line.starts_with("unreadable file: "), "{first_line:?}");
}

// @kotowari[REQ-core-197, EX-core-358]
#[test]
fn ex_core_358_a_non_utf8_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/plans")).unwrap();
    let mut bytes = VALID_PLAN.as_bytes().to_vec();
    bytes.extend_from_slice(&[0xff, 0xfe, b'\n']);
    std::fs::write(tmp.path().join("docs/plans/a.md"), bytes).unwrap();
    let output = cmd()
        .args(["plan", "docs/plans/a.md"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let first_line = first_stderr_line(&output);
    assert!(first_line.starts_with("non-UTF-8 file: "), "{first_line:?}");
}

// @kotowari[REQ-core-191, EX-core-345]
#[test]
fn ex_core_345_a_schema_named_in_the_frontmatter_is_not_used() {
    let plan = format!("---\n$schema: missing.yaml\n---\n{VALID_PLAN}");
    let tmp = project_with_plan(&plan);
    assert!(!tmp.path().join("docs/plans/missing.yaml").exists());
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
}

// @kotowari[REQ-core-191, EX-core-361]
#[test]
fn ex_core_361_a_broken_frontmatter_is_skipped_unread() {
    let plan = format!("---\n: : [unclosed\n---\n{VALID_PLAN}");
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
}

// @kotowari[REQ-core-194, TBL-core-005, EX-core-346]
#[test]
fn ex_core_346_plan_json_has_only_findings_and_counts() {
    let tmp = project_with_plan(&plan_without_done_when());
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md", "--format", "json"]);
    assert_eq!(code, Some(1), "{stdout}");
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let mut keys: Vec<&String> = json.as_object().unwrap().keys().collect();
    keys.sort();
    assert_eq!(keys, ["counts", "findings"], "{stdout}");
    let invalid = json["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == "invalid_plan")
        .count();
    assert!(invalid > 0);
    assert_eq!(json["counts"]["invalid_plan"], invalid, "{stdout}");
}

// @kotowari[REQ-core-194, TBL-core-006]
#[test]
fn req_core_194_plan_finding_keys_are_those_of_check() {
    let tmp = project_with_plan(&plan_without_done_when());
    let (_, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    for f in findings_of(&stdout) {
        let mut keys: Vec<&String> = f.as_object().unwrap().keys().collect();
        keys.sort();
        assert_eq!(keys, ["detail", "kind", "line", "path", "severity"], "{f}");
    }
}

// @kotowari[REQ-core-196, EX-core-359]
#[test]
fn ex_core_359_a_broken_config_does_not_stop_plan() {
    let tmp = project_with_plan(VALID_PLAN);
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: [unclosed\n").unwrap();
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
}

// @kotowari[REQ-core-193, TBL-core-006, EX-core-360]
#[test]
fn ex_core_360_a_plan_outside_the_base_has_a_parent_path() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("work/.kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("plans")).unwrap();
    std::fs::write(tmp.path().join("plans/a.md"), plan_without_done_when()).unwrap();
    let (code, stdout) = run_plan(&tmp.path().join("work"), &["plan", "../plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert!(!findings.is_empty());
    for f in &findings {
        assert_eq!(f["path"], "../plans/a.md", "{f}");
    }
}

// @kotowari[REQ-core-193, TBL-core-006]
#[test]
fn req_core_193_the_path_is_normalised_from_the_base() {
    // カレントディレクトリが基準の下でも、"./" を付けても、path は基準からの相対に正規化する
    let tmp = project_with_plan(&plan_without_done_when());
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    let (_, stdout) = run_plan(&tmp.path().join("docs"), &["plan", "./plans/../plans/a.md"]);
    let findings = findings_of(&stdout);
    assert!(!findings.is_empty(), "{stdout}");
    for f in &findings {
        assert_eq!(f["path"], "docs/plans/a.md", "{f}");
    }
}
