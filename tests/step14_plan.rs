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
