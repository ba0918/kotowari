//! "kotowari mutants" が結果のファイルを読み、指摘と集計を出すところの検査。

use assert_cmd::Command;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// TBL-core-024 の鍵だけを持つ変異の1件
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

/// 一時ディレクトリにファイルを書く（親のディレクトリも作る）
fn write(dir: &Path, name: &str, body: &str) {
    let path = dir.join(name);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, body).unwrap();
}

/// 結果のファイルを "outcomes.json" に置いた一時ディレクトリ
fn project(results: &str) -> TempDir {
    let tmp = TempDir::new().unwrap();
    write(tmp.path(), "outcomes.json", results);
    tmp
}

/// 標準出力の JSON
fn json_of(output: &std::process::Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "stdout should be one JSON: {e}\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
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

/// 結果の誤りで停止し、詳細が結果のファイルの相対パスで始まることを見る（TBL-core-018、TBL-core-020）
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

// --- REQ-core-144: 結果の誤り ---

// @kotowari[REQ-core-144, EX-core-207]
#[test]
fn req_144_unknown_outcome_value_stops() {
    assert_results_error(&outcomes(&[
        BASELINE_SUCCESS.to_string(),
        mutant_at("src/a.rs", 3, "replace f with ()", "Flaky"),
    ]));
}

// @kotowari[REQ-core-144, EX-core-208]
#[test]
fn req_144_failed_baseline_stops() {
    assert_results_error(&outcomes(&[
        r#"{"scenario":"Baseline","summary":"Failure"}"#.to_string(),
        mutant_at("src/a.rs", 3, "replace f with ()", "CaughtMutant"),
    ]));
}

// @kotowari[REQ-core-144, EX-core-222]
#[test]
fn req_144_broken_json_stops() {
    assert_results_error("{");
}

// @kotowari[REQ-core-144, TBL-core-024, EX-core-223]
#[test]
fn req_144_wrong_key_type_stops() {
    // "span.start.line" が数ではなく文字列
    assert_results_error(&outcomes(&[
        r#"{"scenario":{"Mutant":{"file":"src/a.rs","name":"src/a.rs:3:5: replace f with ()","span":{"start":{"line":"3","column":5}}}},"summary":"MissedMutant"}"#.to_string(),
    ]));
}

// @kotowari[REQ-core-144, TBL-core-024, EX-core-224]
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

// @kotowari[REQ-core-144, EX-core-225]
#[test]
fn req_144_line_zero_stops() {
    assert_results_error(&outcomes(&[mutant_at(
        "src/a.rs",
        0,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[REQ-core-144, EX-core-226]
#[test]
fn req_144_path_outside_the_base_stops() {
    assert_results_error(&outcomes(&[mutant_at(
        "../x/src/a.rs",
        3,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[REQ-core-144, EX-core-227]
#[test]
fn req_144_results_without_baseline_and_with_unknown_keys_are_read() {
    // 基準の実行が無く、TBL-core-024 に挙げていない鍵 "extra" を持つ1件
    let entry = r#"{"scenario":{"Mutant":{"file":"src/a.rs","name":"src/a.rs:3:5: replace f with ()","span":{"start":{"line":3,"column":5}},"extra":1}},"summary":"CaughtMutant","extra":1}"#;
    let tmp = project(&outcomes(&[entry.to_string()]));
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        first_stderr_line(&output)
    );
    assert_eq!(json_of(&output)["mutants"]["caught"], 1);
}

// --- REQ-core-139、REQ-core-140: 見逃しと時間切れの指摘 ---

// @kotowari[REQ-core-138, REQ-core-139, TBL-core-024, EX-core-204]
#[test]
fn req_139_survived_mutant_is_an_error() {
    // 等価の一覧の鍵は設定に無い
    let tmp = project(&outcomes(&[
        BASELINE_SUCCESS.to_string(),
        mutant_at("src/a.rs", 3, "replace f with ()", "MissedMutant"),
    ]));
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    let findings = v["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0]["kind"], "mutant_survived");
    assert_eq!(findings[0]["severity"], "error");
    assert_eq!(findings[0]["path"], "src/a.rs");
    assert_eq!(findings[0]["line"], 3);
    assert_eq!(findings[0]["detail"], "replace f with ()");
    assert_eq!(output.status.code(), Some(1));
}

// @kotowari[REQ-core-139, REQ-core-145, EX-core-205]
#[test]
fn req_139_caught_and_unviable_mutants_yield_nothing() {
    let tmp = project(&outcomes(&[
        mutant_at("src/a.rs", 3, "replace f with ()", "CaughtMutant"),
        mutant_at("src/a.rs", 4, "replace g with ()", "Unviable"),
    ]));
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    assert!(
        v["findings"].as_array().unwrap().is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(v["mutants"]["caught"], 1);
    assert_eq!(v["mutants"]["unviable"], 1);
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[REQ-core-140, EX-core-206]
#[test]
fn req_140_timeout_is_a_notice_even_when_listed() {
    // EX-core-206 の Given: その変異に合う形の正しい1件と、その文面を持つソースを置く
    // （SRC_A、CHANGE、valid_entry、project_with_list はこのファイルの後ろで定義している）
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "Timeout")]),
        &valid_entry(),
    );
    write(tmp.path(), "src/a.rs", SRC_A);
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    let findings = v["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0]["kind"], "mutant_timeout");
    assert_eq!(findings[0]["severity"], "notice");
    assert_eq!(findings[0]["path"], "src/a.rs");
    assert_eq!(findings[0]["line"], 3);
    // PROP-core-005: "timeout" は mutant_timeout の指摘の数に等しい。
    // 一覧に載っていても "equivalent" には数えない（REQ-core-140: 一覧との一致を見ない）
    assert_eq!(v["mutants"]["timeout"], 1);
    assert_eq!(v["mutants"]["equivalent"], 0);
    // 一覧の1件の文面は今のソースにあるので古くない（REQ-core-142）
    assert!(findings_of(&v, "equivalent_stale").is_empty(), "{findings:?}");
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[REQ-core-139, PROP-core-005, EX-core-229]
#[test]
fn req_139_duplicate_mutants_yield_one_finding_each() {
    let one = mutant_at("src/a.rs", 3, "replace f with ()", "MissedMutant");
    let tmp = project(&outcomes(&[one.clone(), one]));
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    assert_eq!(v["findings"].as_array().unwrap().len(), 2);
    assert_eq!(v["counts"]["mutant_survived"], 2);
    assert_eq!(v["mutants"]["survived"], 2);
}

// --- REQ-core-145、REQ-core-146: 集計 ---

// @kotowari[REQ-core-145, REQ-core-146, PROP-core-005, EX-core-209]
#[test]
fn req_145_no_mutants_exits_zero_with_all_zero_counts() {
    let tmp = project(&outcomes(&[BASELINE_SUCCESS.to_string()]));
    let output = run_in(tmp.path(), &["--format", "text"]);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "mutants: caught=0 survived=0 timeout=0 unviable=0 equivalent=0\n"
    );
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[REQ-core-146, EX-core-230]
#[test]
fn req_146_summary_is_the_last_line_after_findings() {
    let tmp = project(&outcomes(&[mutant_at(
        "src/a.rs",
        3,
        "replace f with ()",
        "MissedMutant",
    )]));
    let output = run_in(tmp.path(), &["--format", "text"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(
        lines.first().copied(),
        Some("src/a.rs:3 [error] mutant_survived replace f with ()")
    );
    assert_eq!(
        lines.last().copied(),
        Some("mutants: caught=0 survived=1 timeout=0 unviable=0 equivalent=0")
    );
}

// --- REQ-core-147: 読む範囲 ---

// @kotowari[REQ-core-147, EX-core-210]
#[test]
fn req_147_missing_ir_directory_does_not_stop_mutants() {
    let tmp = project(&outcomes(&[mutant_at(
        "src/a.rs",
        3,
        "replace f with ()",
        "CaughtMutant",
    )]));
    // 設定の "ir" の指す先が無い
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        "ir: docs/no-such-place\n",
    );
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        first_stderr_line(&output)
    );
}

// @kotowari[REQ-core-147, TBL-core-025, EX-core-231]
#[test]
fn req_147_unmarked_test_is_not_reported_and_json_has_three_keys() {
    let tmp = project(&outcomes(&[mutant_at(
        "src/a.rs",
        3,
        "replace f with ()",
        "CaughtMutant",
    )]));
    // 印の無いテストの関数
    write(tmp.path(), "tests/x.rs", "#[test]\nfn t() {}\n");
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    assert!(
        v["findings"].as_array().unwrap().is_empty(),
        "{:?}",
        v["findings"]
    );
    // REQ-core-147 が定めるのは鍵の集合（3つだけ）で、鍵の順ではない。
    // 並べてから比べるのは、JSON の鍵の順が serde_json の preserve_order の
    // 有無で変わり、同じ workspace の別のクレートの都合で入れ替わるため
    let mut keys: Vec<&str> = v.as_object().unwrap().keys().map(|k| k.as_str()).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["counts", "findings", "mutants"]);
}

// --- REQ-core-143、REQ-core-148: 等価の一覧 ---

/// EX-core-211 の場面のソース。3行目が "    if a == b {"
const SRC_A: &str = "fn f() {\n    let x = 1;\n    if a == b {\n    }\n}\n";
/// EX-core-211 の場面の変更の説明
const CHANGE: &str = "replace == with != in f";

/// 等価の一覧の1件（値はすべて引用符でくくる）
fn entry(fields: &[(&str, &str)]) -> String {
    let mut yaml = String::new();
    for (i, (key, value)) in fields.iter().enumerate() {
        yaml.push_str(if i == 0 { "- " } else { "  " });
        yaml.push_str(&format!("{key}: {value:?}\n"));
    }
    yaml
}

/// EX-core-211 の形の正しい1件
fn valid_entry() -> String {
    entry(&[
        ("file", "src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "equivalent"),
        ("why", "the branch cannot be reached"),
    ])
}

/// 設定で等価の一覧を指した置き場
fn project_with_list(results: &str, list: &str) -> TempDir {
    let tmp = project(results);
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        "mutants:\n  equivalents: docs/equivalents.yaml\n",
    );
    write(tmp.path(), "docs/equivalents.yaml", list);
    tmp
}

/// その種類の指摘だけを取り出す
fn findings_of<'a>(v: &'a serde_json::Value, kind: &str) -> Vec<&'a serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .collect()
}

/// EX-core-211 の場面（3行目の見逃しが1件、ソースあり）で一覧を読ませる
fn run_with_list_output(list: &str) -> std::process::Output {
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "MissedMutant")]),
        list,
    );
    write(tmp.path(), "src/a.rs", SRC_A);
    run_in(tmp.path(), &[])
}

fn run_with_list(list: &str) -> serde_json::Value {
    json_of(&run_with_list_output(list))
}

// @kotowari[REQ-core-143, EX-core-215]
#[test]
fn req_143_blank_why_is_invalid_and_suppresses_nothing() {
    let v = run_with_list(&entry(&[
        ("file", "src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "equivalent"),
        ("why", "   "),
    ]));
    let invalid = findings_of(&v, "equivalent_invalid");
    assert_eq!(invalid.len(), 1, "{:?}", v["findings"]);
    assert_eq!(invalid[0]["severity"], "error");
    assert_eq!(invalid[0]["path"], "docs/equivalents.yaml");
    assert!(invalid[0]["line"].is_null());
    assert_eq!(invalid[0]["detail"], "src/a.rs: replace == with != in f");
    // 形の誤った1件は見逃しを外さない
    let survived = findings_of(&v, "mutant_survived");
    assert_eq!(survived.len(), 1);
    assert_eq!(survived[0]["line"], 3);
    // A53: 形の誤った1件に equivalent_stale は出さない
    assert!(findings_of(&v, "equivalent_stale").is_empty());
}

// @kotowari[REQ-core-143, EX-core-216]
#[test]
fn req_143_class_other_than_equivalent_is_invalid() {
    let v = run_with_list(&entry(&[
        ("file", "src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "untested"),
        ("why", "not checked yet"),
    ]));
    assert_eq!(findings_of(&v, "equivalent_invalid").len(), 1);
}

// @kotowari[REQ-core-143, EX-core-236]
#[test]
fn req_143_entry_outside_the_base_is_invalid() {
    let v = run_with_list(&entry(&[
        ("file", "../x/src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "equivalent"),
        ("why", "outside"),
    ]));
    assert_eq!(findings_of(&v, "equivalent_invalid").len(), 1);
}

// @kotowari[REQ-core-143, EX-core-220]
#[test]
fn req_143_entry_without_file_has_an_empty_detail_prefix() {
    let v = run_with_list(&entry(&[
        ("change", "replace f with ()"),
        ("text", "if a == b {"),
        ("class", "equivalent"),
        ("why", "no file key"),
    ]));
    let invalid = findings_of(&v, "equivalent_invalid");
    assert_eq!(invalid.len(), 1, "{:?}", v["findings"]);
    assert_eq!(invalid[0]["detail"], ": replace f with ()");
}

// @kotowari[REQ-core-143, EX-core-237]
#[test]
fn req_143_duplicate_invalid_entries_yield_one_finding_each() {
    let one = entry(&[
        ("file", "src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "equivalent"),
        ("why", ""),
    ]);
    let v = run_with_list(&format!("{one}{one}"));
    assert_eq!(findings_of(&v, "equivalent_invalid").len(), 2);
}

// @kotowari[REQ-core-148, EX-core-217]
#[test]
fn req_148_missing_list_file_stops() {
    let tmp = project(&outcomes(&[mutant_at(
        "src/a.rs",
        3,
        CHANGE,
        "CaughtMutant",
    )]));
    // 鍵はあるが指す先のファイルは無い
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        "mutants:\n  equivalents: docs/equivalents.yaml\n",
    );
    let output = run_in(tmp.path(), &[]);
    assert_eq!(output.status.code(), Some(2));
    let first_line = first_stderr_line(&output);
    assert!(
        first_line.starts_with("unreadable file: docs/equivalents.yaml"),
        "got: {first_line:?}"
    );
}

// @kotowari[REQ-core-148, EX-core-238]
#[test]
fn req_148_empty_list_file_is_zero_entries() {
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "CaughtMutant")]),
        "",
    );
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        first_stderr_line(&output)
    );
}

// @kotowari[REQ-core-148, EX-core-239]
#[test]
fn req_148_list_that_is_not_a_sequence_stops_with_the_list_path() {
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "CaughtMutant")]),
        "file: src/a.rs\n",
    );
    let output = run_in(tmp.path(), &[]);
    assert_eq!(output.status.code(), Some(2));
    let first_line = first_stderr_line(&output);
    assert!(
        first_line.starts_with("config error: docs/equivalents.yaml"),
        "got: {first_line:?}"
    );
}

// @kotowari[REQ-core-148, EX-core-221]
#[test]
fn req_148_check_ignores_a_missing_list_file() {
    let tmp = TempDir::new().unwrap();
    for dir in ["docs/ir", "docs/decision/records", "docs/decision/adr"] {
        std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\nmutants:\n  equivalents: docs/equivalents.yaml\n",
    );
    let output = cmd()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_ne!(
        output.status.code(),
        Some(2),
        "check should not read the list: {}",
        first_stderr_line(&output)
    );
}

// --- REQ-core-141、REQ-core-142: 一覧との一致と、文面の無くなった1件 ---

// @kotowari[REQ-core-139, REQ-core-141, REQ-core-145, EX-core-211]
#[test]
fn req_141_listed_survivor_is_counted_as_equivalent() {
    let output = run_with_list_output(&valid_entry());
    let v = json_of(&output);
    assert!(
        findings_of(&v, "mutant_survived").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(v["mutants"]["survived"], 0);
    assert_eq!(v["mutants"]["equivalent"], 1);
    // 文面が今のソースにある1件は古くない（REQ-core-142）
    assert!(
        findings_of(&v, "equivalent_stale").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[REQ-core-141, EX-core-212]
#[test]
fn req_141_moved_line_still_matches() {
    // "    if a == b {" が7行目に動き、見逃しの行も 7
    let moved = "fn f() {\n    let a = 1;\n    let b = 2;\n    let c = 3;\n    let d = 4;\n    let e = 5;\n    if a == b {\n    }\n}\n";
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 7, CHANGE, "MissedMutant")]),
        &valid_entry(),
    );
    write(tmp.path(), "src/a.rs", moved);
    let v = json_of(&run_in(tmp.path(), &[]));
    assert!(
        findings_of(&v, "mutant_survived").is_empty(),
        "{:?}",
        v["findings"]
    );
}

// @kotowari[REQ-core-141, REQ-core-142, EX-core-213]
#[test]
fn req_141_rewritten_line_no_longer_matches_and_entry_goes_stale() {
    // 3行目が "    if a == c {" に変わり、"if a == b {" の行はどこにも無い
    let rewritten = "fn f() {\n    let x = 1;\n    if a == c {\n    }\n}\n";
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "MissedMutant")]),
        &valid_entry(),
    );
    write(tmp.path(), "src/a.rs", rewritten);
    let v = json_of(&run_in(tmp.path(), &[]));
    let survived = findings_of(&v, "mutant_survived");
    assert_eq!(survived.len(), 1, "{:?}", v["findings"]);
    assert_eq!(survived[0]["path"], "src/a.rs");
    assert_eq!(survived[0]["line"], 3);
    let stale = findings_of(&v, "equivalent_stale");
    assert_eq!(stale.len(), 1, "{:?}", v["findings"]);
    assert_eq!(stale[0]["severity"], "notice");
    assert_eq!(stale[0]["path"], "docs/equivalents.yaml");
    assert!(stale[0]["line"].is_null());
    assert_eq!(stale[0]["detail"], "src/a.rs: replace == with != in f");
}

// @kotowari[REQ-core-141, EX-core-214]
#[test]
fn req_141_line_beyond_the_file_does_not_match() {
    // "src/a.rs" は5行で、見逃しは9行目
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 9, CHANGE, "MissedMutant")]),
        &valid_entry(),
    );
    write(tmp.path(), "src/a.rs", SRC_A);
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    let survived = findings_of(&v, "mutant_survived");
    assert_eq!(survived.len(), 1, "{:?}", v["findings"]);
    assert_eq!(survived[0]["line"], 9);
    assert_eq!(output.status.code(), Some(1));
}

// @kotowari[REQ-core-141, EX-core-232]
#[test]
fn req_141_one_entry_matches_every_line_with_the_same_text() {
    // 3行目と8行目がどちらも "    if a == b {"
    let twice = "fn f() {\n    let x = 1;\n    if a == b {\n    }\n    let y = 2;\n    let z = 3;\n    let w = 4;\n    if a == b {\n    }\n}\n";
    let tmp = project_with_list(
        &outcomes(&[
            mutant_at("src/a.rs", 3, CHANGE, "MissedMutant"),
            mutant_at("src/a.rs", 8, CHANGE, "MissedMutant"),
        ]),
        &valid_entry(),
    );
    write(tmp.path(), "src/a.rs", twice);
    let v = json_of(&run_in(tmp.path(), &[]));
    assert!(
        findings_of(&v, "mutant_survived").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(v["mutants"]["equivalent"], 2);
}

// @kotowari[REQ-core-141, EX-core-233]
#[test]
fn req_141_entry_for_another_file_does_not_match() {
    // "src/b.rs" にも同じ文面の行が同じ行番号であるが、"file" が違うので一致しない
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "MissedMutant")]),
        &entry(&[
            ("file", "src/b.rs"),
            ("change", CHANGE),
            ("text", "if a == b {"),
            ("class", "equivalent"),
            ("why", "another file"),
        ]),
    );
    write(tmp.path(), "src/a.rs", SRC_A);
    write(tmp.path(), "src/b.rs", SRC_A);
    let v = json_of(&run_in(tmp.path(), &[]));
    let survived = findings_of(&v, "mutant_survived");
    assert_eq!(survived.len(), 1, "{:?}", v["findings"]);
    assert_eq!(survived[0]["path"], "src/a.rs");
    assert_eq!(survived[0]["line"], 3);
}

// @kotowari[REQ-core-141, EX-core-234]
#[test]
fn req_141_path_spelling_tab_indent_and_crlf_still_match() {
    // 一覧の "file" は "./src/a.rs"、ソースの字下げはタブで行の終わりは "\r\n"
    let tabbed = "fn f() {\r\n\tlet x = 1;\r\n\tif a == b {\r\n\t}\r\n}\r\n";
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "MissedMutant")]),
        &entry(&[
            ("file", "./src/a.rs"),
            ("change", CHANGE),
            ("text", "if a == b {"),
            ("class", "equivalent"),
            ("why", "the branch cannot be reached"),
        ]),
    );
    write(tmp.path(), "src/a.rs", tabbed);
    let v = json_of(&run_in(tmp.path(), &[]));
    assert!(
        findings_of(&v, "mutant_survived").is_empty(),
        "{:?}",
        v["findings"]
    );
}

// @kotowari[REQ-core-111, REQ-core-141]
#[test]
fn req_111_source_with_a_bom_still_matches_on_its_first_line() {
    // REQ-core-111: 読むファイルの先頭の BOM は読み飛ばす。REQ-core-147 はソースを mutants が読むファイルに挙げる。
    // 読み飛ばさないと1行目の文面に BOM が残り、一覧の1件と一致しない
    const CHANGE_ON_LINE_ONE: &str = "replace f with ()";
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 1, CHANGE_ON_LINE_ONE, "MissedMutant")]),
        &entry(&[
            ("file", "src/a.rs"),
            ("change", CHANGE_ON_LINE_ONE),
            ("text", "fn f() {"),
            ("class", "equivalent"),
            ("why", "the body has no observable effect"),
        ]),
    );
    write(tmp.path(), "src/a.rs", &format!("\u{FEFF}{SRC_A}"));
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    assert!(
        findings_of(&v, "mutant_survived").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert!(
        findings_of(&v, "equivalent_stale").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(v["mutants"]["equivalent"], 1);
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[REQ-core-141, REQ-core-142, EX-core-235]
#[test]
fn req_141_non_utf8_source_does_not_stop() {
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "MissedMutant")]),
        &valid_entry(),
    );
    std::fs::create_dir_all(tmp.path().join("src")).unwrap();
    std::fs::write(tmp.path().join("src/a.rs"), b"\xff\xfe").unwrap();
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    assert_eq!(findings_of(&v, "mutant_survived").len(), 1);
    assert_eq!(findings_of(&v, "equivalent_stale").len(), 1);
    assert_eq!(output.status.code(), Some(1));
}

// @kotowari[REQ-core-142, EX-core-243]
#[test]
fn req_142_stale_entry_detail_keeps_the_written_path() {
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "CaughtMutant")]),
        &entry(&[
            ("file", "./src/a.rs"),
            ("change", "replace f with ()"),
            ("text", "no such line"),
            ("class", "equivalent"),
            ("why", "written with a leading dot"),
        ]),
    );
    write(tmp.path(), "src/a.rs", SRC_A);
    let v = json_of(&run_in(tmp.path(), &[]));
    let stale = findings_of(&v, "equivalent_stale");
    assert_eq!(stale.len(), 1, "{:?}", v["findings"]);
    assert_eq!(stale[0]["detail"], "./src/a.rs: replace f with ()");
}

// @kotowari[REQ-core-144, REQ-core-139]
#[test]
fn req_144_line_one_is_read() {
    // 1行目の変異は「行が1未満」ではないので停止しない
    let tmp = project(&outcomes(&[mutant_at(
        "src/a.rs",
        1,
        "replace f with ()",
        "MissedMutant",
    )]));
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "stderr: {}",
        first_stderr_line(&output)
    );
    let v = json_of(&output);
    let survived = findings_of(&v, "mutant_survived");
    assert_eq!(survived.len(), 1, "{:?}", v["findings"]);
    assert_eq!(survived[0]["line"], 1);
}

// @kotowari[REQ-core-148]
#[test]
fn req_148_list_of_comments_only_is_zero_entries() {
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "CaughtMutant")]),
        "# 今は1件も無い\n# あとで足す\n",
    );
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        first_stderr_line(&output)
    );
}

// @kotowari[REQ-core-148]
#[test]
fn req_148_blank_lines_between_comments_are_still_empty() {
    // REQ-core-148 の「空（0バイトか注釈だけ）」は空行を内容に数えない
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "CaughtMutant")]),
        "# 今は1件も無い\n\n# あとで足す\n",
    );
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        first_stderr_line(&output)
    );
}

// @kotowari[REQ-core-143]
#[test]
fn req_143_entry_with_a_key_outside_the_five_is_invalid() {
    // 鍵は5つだが "why" が無く、代わりに知らない鍵 "note" がある
    let v = run_with_list(&entry(&[
        ("file", "src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "equivalent"),
        ("note", "the reason belongs in why"),
    ]));
    assert_eq!(findings_of(&v, "equivalent_invalid").len(), 1);
    // 形の誤った1件は見逃しを外さない
    assert_eq!(findings_of(&v, "mutant_survived").len(), 1);
}
