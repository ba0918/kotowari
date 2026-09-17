//! "kotowari mutants" が結果のファイルを読み、指摘と集計を出すところの検査。

use assert_cmd::Command;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// TBL-024 の鍵だけを持つ変異の1件
fn mutant(file: &str, line: u64, column: u64, name: &str, summary: &str) -> String {
    format!(
        r#"{{"scenario":{{"Mutant":{{"file":{file:?},"name":{name:?},"span":{{"start":{{"line":{line},"column":{column}}}}}}}}},"summary":{summary:?}}}"#
    )
}

/// ファイルと行と列から名前を組み立てた、結果が `summary` の変異の1件
fn mutant_at(file: &str, line: u64, change: &str, summary: &str) -> String {
    mutant(file, line, 5, &format!("{file}:{line}:5: {change}"), summary)
}

/// 結果のファイルの中身
fn outcomes(entries: &[String]) -> String {
    format!(r#"{{"outcomes":[{}]}}"#, entries.join(","))
}

/// 成功した基準の実行の1件
const BASELINE_SUCCESS: &str = r#"{"scenario":"Baseline","summary":"Success"}"#;

/// 結果のファイルを "outcomes.json" に置いた一時ディレクトリ
fn project(results: &str) -> TempDir {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("outcomes.json"), results).unwrap();
    tmp
}

/// その置き場で "kotowari mutants" を走らせる
fn run_in(dir: &Path, args: &[&str]) -> std::process::Output {
    let mut command = cmd();
    command.args(["mutants", "--tool", "cargo-mutants"]);
    command.args(args);
    command.arg("outcomes.json");
    command.current_dir(dir).output().unwrap()
}

/// 標準エラーの1行目
fn first_stderr_line(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .next()
        .unwrap_or("")
        .to_string()
}

/// 結果の誤りで停止し、詳細が結果のファイルの相対パスで始まることを見る（TBL-018、TBL-020）
fn assert_results_error(results: &str) {
    let tmp = project(results);
    let output = run_in(tmp.path(), &[]);
    assert_eq!(output.status.code(), Some(2), "should stop");
    assert!(
        String::from_utf8_lossy(&output.stdout).is_empty(),
        "a stop writes nothing to stdout"
    );
    let first_line = first_stderr_line(&output);
    assert!(
        first_line.starts_with("results error: outcomes.json: "),
        "expected 'results error: outcomes.json: ...', got: {first_line:?}"
    );
}

// --- REQ-144: 結果の誤り ---

// @kotowari[REQ-144, EX-207]
#[test]
fn req_144_unknown_outcome_value_stops() {
    assert_results_error(&outcomes(&[
        BASELINE_SUCCESS.to_string(),
        mutant_at("src/a.rs", 3, "replace f with ()", "Flaky"),
    ]));
}

// @kotowari[REQ-144, EX-208]
#[test]
fn req_144_failed_baseline_stops() {
    assert_results_error(&outcomes(&[
        r#"{"scenario":"Baseline","summary":"Failure"}"#.to_string(),
        mutant_at("src/a.rs", 3, "replace f with ()", "CaughtMutant"),
    ]));
}

// @kotowari[REQ-144, EX-222]
#[test]
fn req_144_broken_json_stops() {
    assert_results_error("{");
}

// @kotowari[REQ-144, TBL-024, EX-223]
#[test]
fn req_144_wrong_key_type_stops() {
    // "span.start.line" が数ではなく文字列
    assert_results_error(&outcomes(&[
        r#"{"scenario":{"Mutant":{"file":"src/a.rs","name":"src/a.rs:3:5: replace f with ()","span":{"start":{"line":"3","column":5}}}},"summary":"MissedMutant"}"#.to_string(),
    ]));
}

// @kotowari[REQ-144, TBL-024, EX-224]
#[test]
fn req_144_name_without_the_location_prefix_stops() {
    assert_results_error(&outcomes(&[mutant(
        "src/a.rs",
        3,
        5,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[REQ-144, EX-225]
#[test]
fn req_144_line_zero_stops() {
    assert_results_error(&outcomes(&[mutant_at(
        "src/a.rs",
        0,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[REQ-144, EX-226]
#[test]
fn req_144_path_outside_the_base_stops() {
    assert_results_error(&outcomes(&[mutant_at(
        "../x/src/a.rs",
        3,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[REQ-144, EX-227]
#[test]
fn req_144_results_without_baseline_and_with_unknown_keys_are_read() {
    // 基準の実行が無く、TBL-024 に挙げていない鍵 "extra" を持つ1件
    let entry = r#"{"scenario":{"Mutant":{"file":"src/a.rs","name":"src/a.rs:3:5: replace f with ()","span":{"start":{"line":3,"column":5}},"extra":1}},"summary":"CaughtMutant","extra":1}"#;
    let tmp = project(&outcomes(&[entry.to_string()]));
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        first_stderr_line(&output)
    );
}
