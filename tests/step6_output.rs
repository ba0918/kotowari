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
        // [error] か [notice] を含む
        assert!(
            line.contains("[error]") || line.contains("[notice]"),
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
        stdout.contains(":- [error]") || stdout.contains(":- [notice]"),
        "null line should print as '-': {stdout}"
    );
}

// --- REQ-007: 終了コード ---

// @kotowari[REQ-007, TBL-002]
#[test]
fn req_007_exit_code_one_on_error_and_zero_on_notice_only() {
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

    // 注意だけ（201行で too_many_lines）→ 終了コード 0 のまま
    let long = format!("# Title\n\nScope.\n{}", "\n".repeat(198));
    fs::write(tmp.path().join("docs/ir/a.md"), long).unwrap();
    let output_notice = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let stdout = String::from_utf8_lossy(&output_notice.stdout);
    assert!(stdout.contains("too_many_lines"), "notice should be present: {stdout}");
    assert!(stdout.contains("\"notice\""), "severity should be notice: {stdout}");
    assert_eq!(output_notice.status.code(), Some(0), "notice only should exit 0");

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

// --- REQ-128, TBL-021, PROP-004: 読んだテストのファイルの申告 ---

/// "tests.files" の glob を指定したプロジェクトを作る
fn make_project_with_test_globs(tmp: &std::path::Path, globs: &[&str]) {
    make_project(tmp);
    let list: String = globs.iter().map(|g| format!("    - \"{g}\"\n")).collect();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        format!(
            "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n{list}"
        ),
    )
    .unwrap();
}

/// glob に当たる場所へファイルを書く
fn write_test_file(tmp: &std::path::Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

// @kotowari[REQ-128, TBL-021]
#[test]
fn req_128_tests_key_lists_files_per_extension_with_query_flag() {
    // EX-035: ".rs" が2つと ".py" が1つ
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["lib/**/*"]);
    write_test_file(tmp.path(), "lib/a.rs", "pub fn a() {}\n");
    write_test_file(tmp.path(), "lib/b.rs", "pub fn b() {}\n");
    write_test_file(tmp.path(), "lib/c.py", "print(1)\n");
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    assert_eq!(v["tests"]["rs"]["files"], 2, "two .rs files: {v}");
    assert_eq!(v["tests"]["rs"]["query"], true, "rs has a query: {v}");
    assert_eq!(v["tests"]["py"]["files"], 1, "one .py file: {v}");
    assert_eq!(v["tests"]["py"]["query"], false, "py has no query: {v}");
    assert_eq!(
        v["tests"].as_object().unwrap().len(),
        2,
        "only the extensions that were read: {v}"
    );
}

// @kotowari[REQ-128, TBL-021]
#[test]
fn req_128_tests_is_an_empty_object_without_test_files() {
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["nothing/**/*.rs"]);
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    assert!(
        v["tests"].is_object(),
        "tests should be an object even with no test files: {v}"
    );
    assert!(
        v["tests"].as_object().unwrap().is_empty(),
        "tests should be empty with no test files: {v}"
    );
}

// @kotowari[REQ-128]
#[test]
fn req_128_text_format_does_not_print_tests() {
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["lib/**/*"]);
    fs::write(tmp.path().join("docs/ir/a.md"), "# Title\n\nScope.\n").unwrap();
    write_test_file(tmp.path(), "lib/a.py", "print(1)\n");

    // JSON には出る
    let json_output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&json_output);
    assert_eq!(v["tests"]["py"]["files"], 1, "json should report the file: {v}");

    // text は指摘の行だけなので、指摘が無ければ何も出ない
    let text_output = cmd()
        .args(["check", "--format", "text"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&text_output.stdout);
    assert_eq!(stdout, "", "text format should print findings only: {stdout}");
}

// @kotowari[TBL-021]
#[test]
fn tbl_021_extension_is_after_the_last_dot_and_dotless_names_share_the_empty_key() {
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["lib/**/*"]);
    write_test_file(tmp.path(), "lib/a.test.rs", "pub fn a() {}\n");
    write_test_file(tmp.path(), "lib/.rs", "x\n");
    write_test_file(tmp.path(), "lib/run", "x\n");
    write_test_file(tmp.path(), "lib/foo.", "x\n");
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    assert_eq!(v["tests"]["rs"]["files"], 1, "a.test.rs has the rs key: {v}");
    assert_eq!(
        v["tests"][""]["files"], 3,
        ".rs, run and foo. share the empty key: {v}"
    );
    assert_eq!(v["tests"][""]["query"], false, "the empty key has no query: {v}");
    assert_eq!(v["tests"].as_object().unwrap().len(), 2, "two keys: {v}");
}

// @kotowari[TBL-021]
#[test]
fn tbl_021_uppercase_extension_is_a_separate_key_without_query() {
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["lib/**/*"]);
    write_test_file(tmp.path(), "lib/a.RS", "x\n");
    write_test_file(tmp.path(), "lib/b.rs", "pub fn b() {}\n");
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    assert_eq!(v["tests"]["RS"]["files"], 1, "RS is its own key: {v}");
    assert_eq!(v["tests"]["RS"]["query"], false, "RS has no query: {v}");
    assert_eq!(v["tests"]["rs"]["files"], 1, "rs is separate: {v}");
    assert_eq!(v["tests"]["rs"]["query"], true, "rs has a query: {v}");
}

// @kotowari[TBL-021]
#[test]
fn tbl_021_unparsable_file_is_counted() {
    // EX-038: tree-sitter で読めないファイルも "files" に数える
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["lib/**/*.rs"]);
    write_test_file(tmp.path(), "lib/ok.rs", "pub fn ok() {}\n");
    write_test_file(tmp.path(), "lib/broken.rs", "pub fn broken( {\n");
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let unparsable: Vec<_> = v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == "unparsable_file")
        .collect();
    assert_eq!(unparsable.len(), 1, "one unparsable file: {v}");
    assert_eq!(v["tests"]["rs"]["files"], 2, "the unparsable file is counted: {v}");
}

// @kotowari[TBL-021]
#[test]
#[cfg(unix)]
fn tbl_021_same_path_counts_once_and_symlink_counts_apart() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    // 同じファイルが2つの glob に当たる
    make_project_with_test_globs(tmp.path(), &["lib/**/*.rs", "lib/*.rs"]);
    write_test_file(tmp.path(), "lib/a.rs", "pub fn a() {}\n");
    // 実体とリンクは別のパス
    symlink(tmp.path().join("lib/a.rs"), tmp.path().join("lib/link.rs")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    assert_eq!(
        v["tests"]["rs"]["files"], 2,
        "one path counts once even with two globs, and the symlink counts apart: {v}"
    );
}

// @kotowari[TBL-021]
#[test]
fn tbl_021_keys_are_in_byte_order() {
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["lib/**/*"]);
    write_test_file(tmp.path(), "lib/a.rs", "pub fn a() {}\n");
    write_test_file(tmp.path(), "lib/a.py", "print(1)\n");
    write_test_file(tmp.path(), "lib/a.RS", "x\n");
    write_test_file(tmp.path(), "lib/run", "x\n");
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let tests_part = &stdout[stdout.find("\"tests\":").expect("tests key")..];
    let mut positions = Vec::new();
    for key in ["\"\":", "\"RS\":", "\"py\":", "\"rs\":"] {
        positions.push(tests_part.find(key).unwrap_or_else(|| panic!("{key} in {tests_part}")));
    }
    let mut sorted = positions.clone();
    sorted.sort_unstable();
    assert_eq!(
        positions, sorted,
        "keys should appear in byte order (\"\", \"RS\", \"py\", \"rs\"): {tests_part}"
    );
}

// @kotowari[TBL-021]
#[test]
#[cfg(unix)]
fn tbl_021_excluded_entries_are_not_counted() {
    use std::os::unix::net::UnixListener;
    let tmp = TempDir::new().unwrap();
    make_project_with_test_globs(tmp.path(), &["lib/**/*.rs"]);
    write_test_file(tmp.path(), "lib/a.rs", "pub fn a() {}\n");
    // glob に当たるがディレクトリでも通常のファイルでもないものは読まない（除外）
    let _socket = UnixListener::bind(tmp.path().join("lib/socket.rs")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    assert_eq!(
        v["tests"]["rs"]["files"], 1,
        "the socket is excluded and not counted: {v}"
    );
}

// @kotowari[PROP-004]
#[test]
fn prop_004_files_sum_equals_the_number_of_read_test_files() {
    use proptest::prelude::*;

    // 1件ごとに CLI を起動するので試行回数を絞る
    let config = proptest::test_runner::Config {
        cases: 8,
        ..Default::default()
    };
    proptest!(config, |(exts in proptest::collection::vec(0..4usize, 1..8usize))| {
        let names = ["rs", "py", "txt", ""];
        let tmp = TempDir::new().unwrap();
        make_project_with_test_globs(tmp.path(), &["lib/**/*"]);
        for (i, e) in exts.iter().enumerate() {
            let ext = names[*e];
            let rel = if ext.is_empty() {
                format!("lib/f{i}")
            } else {
                format!("lib/f{i}.{ext}")
            };
            write_test_file(tmp.path(), &rel, "pub fn f() {}\n");
        }
        let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
        let v = parse_json(&output);
        let sum: u64 = v["tests"]
            .as_object()
            .expect("tests object")
            .values()
            .map(|e| e["files"].as_u64().expect("files number"))
            .sum();
        prop_assert_eq!(sum, exts.len() as u64, "sum of files should equal the number of test files: {}", v);
    });
}
