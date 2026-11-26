//! "kotowari mutants" が結果のファイルを読み、指摘と集計を出すところの検査。
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use assert_cmd::Command;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// 鍵だけを持つ変異の1件
fn mutant(file: &str, line: u64, column: u64, name: &str, summary: &str) -> String {
    format!(
        r#"{{"scenario":{{"Mutant":{{"file":{file:?},"name":{name:?},"span":{{"start":{{"line":{line},"column":{column}}}}}}}}},"summary":{summary:?}}}"#
    )
}

/// ファイルと行と列から名前を組み立てた、結果が `summary` の変異の1件
fn mutant_at(file: &str, line: u64, change: &str, summary: &str) -> String {
    mutant(
        file,
        line,
        5,
        &format!("{file}:{line}:5: {change}"),
        summary,
    )
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

/// 結果の誤りで停止し、詳細が結果のファイルの相対パスで始まることを見る
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

// --- 結果の誤り ---

// @kotowari[C-core-mutin-5]
#[test]
fn unknown_outcome_value_stops() {
    assert_results_error(&outcomes(&[
        BASELINE_SUCCESS.to_string(),
        mutant_at("src/a.rs", 3, "replace f with ()", "Flaky"),
    ]));
}

// @kotowari[C-core-mutin-5]
#[test]
fn failed_baseline_stops() {
    assert_results_error(&outcomes(&[
        r#"{"scenario":"Baseline","summary":"Failure"}"#.to_string(),
        mutant_at("src/a.rs", 3, "replace f with ()", "CaughtMutant"),
    ]));
}

// @kotowari[C-core-mutin-5]
#[test]
fn broken_json_stops() {
    assert_results_error("{");
}

// @kotowari[C-core-mutin-5]
#[test]
fn wrong_key_type_stops() {
    // "span.start.line" が数ではなく文字列
    assert_results_error(&outcomes(&[
        r#"{"scenario":{"Mutant":{"file":"src/a.rs","name":"src/a.rs:3:5: replace f with ()","span":{"start":{"line":"3","column":5}}}},"summary":"MissedMutant"}"#.to_string(),
    ]));
}

// @kotowari[C-core-mutin-5]
#[test]
fn name_without_the_location_prefix_stops() {
    assert_results_error(&outcomes(&[mutant(
        "src/a.rs",
        3,
        5,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[C-core-mutin-5]
#[test]
fn line_zero_stops() {
    assert_results_error(&outcomes(&[mutant_at(
        "src/a.rs",
        0,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[C-core-mutin-6]
#[test]
fn path_outside_the_base_stops() {
    assert_results_error(&outcomes(&[mutant_at(
        "../x/src/a.rs",
        3,
        "replace f with ()",
        "MissedMutant",
    )]));
}

// @kotowari[C-core-mutin-3]
#[test]
fn results_without_baseline_and_with_unknown_keys_are_read() {
    // 基準の実行が無く、読まない鍵 "extra" を持つ1件
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

// --- 見逃しと時間切れの指摘 ---

// @kotowari[C-core-equiv-1, C-core-mutin-1, C-core-mutin-2, C-core-mutants-3]
#[test]
fn survived_mutant_is_an_error() {
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

// @kotowari[C-core-mutin-4]
#[test]
fn c_core_mutin_4_a_result_file_written_with_a_leading_dot_is_output_normalized() {
    let tmp = project(&outcomes(&[
        BASELINE_SUCCESS.to_string(),
        mutant_at("./src/a.rs", 3, "replace f with ()", "MissedMutant"),
    ]));
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    let findings = v["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0]["kind"], "mutant_survived");
    assert_eq!(findings[0]["path"], "src/a.rs");
}

// @kotowari[C-core-mutin-2, C-core-mutants-4]
#[test]
fn caught_and_unviable_mutants_yield_nothing() {
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

// @kotowari[C-core-equiv-13, C-core-mutin-2, C-core-mutants-5, C-core-mutants-8]
#[test]
fn timeout_is_a_notice_even_when_listed() {
    // その変異に合う形の正しい1件と、その文面を持つソースを置く
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
    // "timeout" は mutant_timeout の指摘の数に等しい。
    // 一覧に載っていても "equivalent" には数えない（一覧との一致を見ない）
    assert_eq!(v["mutants"]["timeout"], 1);
    assert_eq!(v["mutants"]["equivalent"], 0);
    // 一覧の1件の文面は今のソースにあるので古くない
    assert!(
        findings_of(&v, "equivalent_stale").is_empty(),
        "{findings:?}"
    );
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[C-core-mutants-6, C-core-mutants-8]
#[test]
fn duplicate_mutants_yield_one_finding_each() {
    let one = mutant_at("src/a.rs", 3, "replace f with ()", "MissedMutant");
    let tmp = project(&outcomes(&[one.clone(), one]));
    let output = run_in(tmp.path(), &[]);
    let v = json_of(&output);
    assert_eq!(v["findings"].as_array().unwrap().len(), 2);
    assert_eq!(v["counts"]["mutant_survived"], 2);
    assert_eq!(v["mutants"]["survived"], 2);
}

// --- 集計 ---

// @kotowari[C-core-mutants-9, C-core-mutants-10]
#[test]
fn no_mutants_exits_zero_with_all_zero_counts() {
    let tmp = project(&outcomes(&[BASELINE_SUCCESS.to_string()]));
    let output = run_in(tmp.path(), &["--format", "text"]);
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "mutants: caught=0 survived=0 timeout=0 unviable=0 equivalent=0\n"
    );
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[C-core-mutants-9]
#[test]
fn summary_is_the_last_line_after_findings() {
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

// --- 読む範囲 ---

// @kotowari[C-core-mutants-1]
#[test]
fn missing_decisions_directory_does_not_stop_mutants() {
    let tmp = project(&outcomes(&[mutant_at(
        "src/a.rs",
        3,
        "replace f with ()",
        "CaughtMutant",
    )]));
    // 設定の決定の文書の置き場の指す先が無い
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        "decisions:\n  documents: docs/no-such-place\n",
    );
    let output = run_in(tmp.path(), &[]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        first_stderr_line(&output)
    );
}

// @kotowari[C-core-mutants-2, C-core-mutants-7]
#[test]
fn unmarked_test_is_not_reported_and_json_has_three_keys() {
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
    // 決めてあるのは鍵の集合（3つだけ）で、鍵の順ではない。
    // 並べてから比べるのは、JSON の鍵の順が serde_json の preserve_order の
    // 有無で変わり、同じ workspace の別のクレートの都合で入れ替わるため
    let mut keys: Vec<&str> = v.as_object().unwrap().keys().map(|k| k.as_str()).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["counts", "findings", "mutants"]);
}

// --- 等価の一覧 ---

/// 3行目が "    if a == b {"
const SRC_A: &str = "fn f() {\n    let x = 1;\n    if a == b {\n    }\n}\n";
/// 場面の変更の説明
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

/// 形の正しい1件
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

/// 場面（3行目の見逃しが1件、ソースあり）で一覧を読ませる
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

// @kotowari[C-core-equiv-14, C-core-equiv-15, C-core-equiv-16]
#[test]
fn blank_why_is_invalid_and_suppresses_nothing() {
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
    // 形の誤った1件に equivalent_stale は出さない
    assert!(findings_of(&v, "equivalent_stale").is_empty());
}

// @kotowari[C-core-equiv-14]
#[test]
fn class_other_than_equivalent_is_invalid() {
    let v = run_with_list(&entry(&[
        ("file", "src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "untested"),
        ("why", "not checked yet"),
    ]));
    assert_eq!(findings_of(&v, "equivalent_invalid").len(), 1);
}

// @kotowari[C-core-equiv-14]
#[test]
fn entry_outside_the_base_is_invalid() {
    let v = run_with_list(&entry(&[
        ("file", "../x/src/a.rs"),
        ("change", CHANGE),
        ("text", "if a == b {"),
        ("class", "equivalent"),
        ("why", "outside"),
    ]));
    assert_eq!(findings_of(&v, "equivalent_invalid").len(), 1);
}

// @kotowari[C-core-equiv-16]
#[test]
fn entry_without_file_has_an_empty_detail_prefix() {
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

// @kotowari[C-core-equiv-17]
#[test]
fn duplicate_invalid_entries_yield_one_finding_each() {
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

// @kotowari[C-core-equiv-2]
#[test]
fn missing_list_file_stops() {
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

// @kotowari[C-core-equiv-1]
#[test]
fn empty_list_file_is_zero_entries() {
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

// @kotowari[C-core-equiv-3]
#[test]
fn list_that_is_not_a_sequence_stops_with_the_list_path() {
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

// @kotowari[C-core-equiv-4]
#[test]
fn check_ignores_a_missing_list_file() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decisions")).unwrap();
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        "mutants:\n  equivalents: docs/equivalents.yaml\n",
    );
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_ne!(
        output.status.code(),
        Some(2),
        "check should not read the list: {}",
        first_stderr_line(&output)
    );
}

// --- 一覧との一致と、文面の無くなった1件 ---

// @kotowari[C-core-equiv-5, C-core-mutants-8]
#[test]
fn listed_survivor_is_counted_as_equivalent() {
    let output = run_with_list_output(&valid_entry());
    let v = json_of(&output);
    assert!(
        findings_of(&v, "mutant_survived").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(v["mutants"]["survived"], 0);
    assert_eq!(v["mutants"]["equivalent"], 1);
    // 文面が今のソースにある1件は古くない
    assert!(
        findings_of(&v, "equivalent_stale").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(output.status.code(), Some(0));
}

// @kotowari[C-core-equiv-6]
#[test]
fn moved_line_still_matches() {
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

// @kotowari[C-core-equiv-6, C-core-equiv-11]
#[test]
fn rewritten_line_no_longer_matches_and_entry_goes_stale() {
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

// @kotowari[C-core-equiv-10]
#[test]
fn line_beyond_the_file_does_not_match() {
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

// @kotowari[C-core-equiv-9]
#[test]
fn one_entry_matches_every_line_with_the_same_text() {
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

// @kotowari[C-core-equiv-7]
#[test]
fn entry_for_another_file_does_not_match() {
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

// @kotowari[C-core-equiv-8]
#[test]
fn path_spelling_tab_indent_and_crlf_still_match() {
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

// @kotowari[C-core-equiv-8]
#[test]
fn lone_cr_ends_a_line_of_the_source() {
    // 行の区切りは単独の "\r" も行の終わりに数える。"\n" だけで分けると3行目が見つからない
    let lone_cr = "fn f() {\r    let x = 1;\r    if a == b {\r    }\r}\r";
    let tmp = project_with_list(
        &outcomes(&[mutant_at("src/a.rs", 3, CHANGE, "MissedMutant")]),
        &valid_entry(),
    );
    write(tmp.path(), "src/a.rs", lone_cr);
    let v = json_of(&run_in(tmp.path(), &[]));
    assert!(
        findings_of(&v, "mutant_survived").is_empty(),
        "{:?}",
        v["findings"]
    );
    assert_eq!(v["mutants"]["equivalent"], 1);
}

// @kotowari[C-core-equiv-5, C-core-input-1]
#[test]
fn source_with_a_bom_still_matches_on_its_first_line() {
    // 読むファイルの先頭の BOM は読み飛ばす。ソースは mutants が読むファイルである。
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

// @kotowari[C-core-equiv-10, C-core-equiv-12]
#[test]
fn non_utf8_source_does_not_stop() {
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

// @kotowari[C-core-equiv-11]
#[test]
fn stale_entry_detail_keeps_the_written_path() {
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

// @kotowari[C-core-mutin-5, C-core-mutants-3]
#[test]
fn line_one_is_read() {
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

// @kotowari[C-core-equiv-1]
#[test]
fn list_of_comments_only_is_zero_entries() {
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

// @kotowari[C-core-equiv-1]
#[test]
fn blank_lines_between_comments_are_still_empty() {
    // 「空（0バイトか注釈だけ）」は空行を内容に数えない
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

// @kotowari[C-core-equiv-14, C-core-equiv-15]
#[test]
fn entry_with_a_key_outside_the_five_is_invalid() {
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
