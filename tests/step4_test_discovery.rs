#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use assert_cmd::Command;
use std::fs;
use tempfile::TempDir;

// --- 関数本体の先頭のコメントの印は無視する ---

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn make_project(tmp: &std::path::Path) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/decisions")).unwrap();
    fs::write(
        tmp.join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\n",
    )
    .unwrap();
}

/// `path` に、確認することの行 `rows` を持つ決定を1つ置いた決定の文書を書く
fn make_decision_with_rows(tmp: &std::path::Path, path: &str, rows: &[&str]) {
    let rows: String = rows
        .iter()
        .map(|row| format!("- {row} Checked\n"))
        .collect();
    let path = tmp.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!(
            "# Title\n\nScope.\n\n## Decision\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo it.\n\n### Confirmations\n\n{rows}"
        ),
    )
    .unwrap();
}

fn parse_json(output: &std::process::Output) -> serde_json::Value {
    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&stdout).expect("valid JSON")
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

// --- テストのファイル ---

// @kotowari[C-core-discover-1]
#[test]
fn reads_files_matching_the_globs() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert!(
        rwt.is_empty(),
        "C-t-a-1 should have test coverage: {:?}",
        rwt
    );
}

// --- tree-sitter で読む ---

// @kotowari[C-core-discover-4]
#[test]
fn uses_tree_sitter_with_bundled_rust_query() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn my_test() { assert!(true); }\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "my_test"),
        "should find test via tree-sitter: {:?}",
        twi
    );
}

// --- 拡張子と言語の対応 ---

// @kotowari[C-core-discover-7]
#[test]
fn extension_decides_the_language_case_sensitively() {
    // 表の拡張子だけが言語を決め、大文字小文字を区別する。".go" は言語が決まっても
    // 問い合わせが無いので false、".PY" と ".Rs" は言語が決まらないので false
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*\"\n",
    )
    .unwrap();
    for name in ["a.rs", "a.go", "A.PY", "a.Rs"] {
        write_test_file(tmp.path(), name, "x\n");
    }
    let v = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    assert_eq!(v["tests"]["rs"]["query"], true, "{v}");
    for ext in ["go", "PY", "Rs"] {
        assert_eq!(v["tests"][ext]["query"], false, "{ext}: {v}");
    }
}

// @kotowari[C-core-discover-6]
#[test]
fn file_of_a_language_without_a_query_is_not_parsed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    write_test_file(tmp.path(), "a.go", "func TestA(t *testing.T) {\n");
    let v = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    assert!(findings_by_kind(&v, "unparsable_file").is_empty(), "{v}");
}

// @kotowari[C-core-discover-7]
#[test]
fn upper_case_extension_decides_no_language() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*\"\n",
    )
    .unwrap();
    write_test_file(tmp.path(), "A.PY", "def test_x():\n    pass\n");
    let v = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    assert!(findings_by_kind(&v, "test_without_id").is_empty(), "{v}");
}

// @kotowari[C-core-document-7, C-core-document-8, C-core-markers-7]
#[test]
fn lone_cr_ends_a_line_of_a_test_file() {
    // 行の区切りは単独の "\r" も行の終わりに数える。"\n" だけで数えると、
    // "\r" の後の印が前の行に報告される
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n    - \"tests/**/*.rs\"\n",
    )
    .unwrap();
    fs::write(
        tmp.path().join("tests/test_a.go"),
        "first\r// @kotowari[C-t-a-999]\r",
    )
    .unwrap();
    fs::write(
        tmp.path().join("tests/test_b.rs"),
        "#[test]\r/* first\r@kotowari[C-t-a-998] */\rfn t() {}\r",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let lines: Vec<(String, u64)> = findings_by_kind(&v, "unresolved_reference")
        .iter()
        .map(|f| {
            (
                f["path"].as_str().unwrap().to_string(),
                f["line"].as_u64().unwrap(),
            )
        })
        .collect();
    assert!(
        lines.contains(&("tests/test_a.go".to_string(), 2)),
        "{lines:?}"
    );
    assert!(
        lines.contains(&("tests/test_b.rs".to_string(), 3)),
        "{lines:?}"
    );
}

// --- Rust のテスト ---

// @kotowari[C-core-queries-4]
#[test]
fn test_attribute_is_always_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn counted_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "counted_test"),
        "should count #[test]: {:?}",
        twi
    );
}

// @kotowari[C-core-queries-5]
#[test]
fn configured_attribute_matches_path_with_arguments() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    attributes:\n      - kani::proof\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::write(
        tmp.path().join("src/lib.rs"),
        "// @kotowari[C-t-a-1]\n#[kani::proof(unwind = 3)]\nfn my_proof() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert!(
        rwt.is_empty(),
        "kani::proof should count as test: {:?}",
        rwt
    );
}

// @kotowari[C-core-queries-7]
#[test]
fn macro_body_functions_are_counted_by_last_segment() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    macros:\n      - proptest\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "proptest::proptest! {\n    // @kotowari[C-t-a-1]\n    #[test]\n    fn my_prop_test(x in 0..100u32) {\n        assert!(x < 101);\n    }\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    // proptest マクロ内の関数がテストとして数えられ、確認することの行に結び付く
    assert!(
        rwt.is_empty(),
        "proptest function should cover C-t-a-1: {:?}",
        rwt
    );
}

// @kotowari[C-core-queries-7]
#[test]
fn two_functions_in_a_path_macro_are_both_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    macros:\n      - proptest\n",
    )
    .unwrap();
    write_test_file(
        tmp.path(),
        "test_a.rs",
        "proptest::proptest! {\n    #[test]\n    fn first_prop(x in 0..100u32) {\n        assert!(x < 101);\n    }\n\n    #[test]\n    fn second_prop(y in 0..100u32) {\n        assert!(y < 101);\n    }\n}\n",
    );
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    // 印が無いので、数えられたテストの数だけ test_without_id が出る
    let twi = findings_by_kind(&result, "test_without_id");
    assert_eq!(
        twi.len(),
        2,
        "both functions in the path-qualified macro count as tests: {:?}",
        result
    );
    let names: Vec<&str> = twi.iter().map(|f| f["detail"].as_str().unwrap()).collect();
    assert_eq!(names, vec!["first_prop", "second_prop"]);
}

// --- 読めないファイル ---

// @kotowari[C-core-discover-8]
#[test]
fn unparsable_file_is_skipped() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/broken.rs"),
        "this is not valid rust {{{{",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    // unparsable_file は停止しない（終了コード 2 にならない）
    assert_ne!(output.status.code(), Some(2), "should not stop");
    // unparsable_file の指摘が出る
    let uf = findings_by_kind(&v, "unparsable_file");
    assert!(
        uf.iter()
            .any(|f| f["detail"].as_str().unwrap().contains("broken.rs")),
        "should report unparsable_file for syntax error: {:?}",
        uf
    );
}

// --- 形の誤った印 ---

// @kotowari[C-core-markers-4]
#[test]
fn invalid_marker() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 空の印
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[]\n#[test]\nfn empty_marker_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(!im.is_empty(), "should find invalid marker: {:?}", im);
}

// @kotowari[C-core-findings-3, C-core-markers-6]
#[test]
fn invalid_marker_detail_is_line_text() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 空の印 → detail は印の部分文字列ではなく行の文字
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[]\n#[test]\nfn empty_marker_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(!im.is_empty(), "should find invalid marker");
    assert_eq!(
        im[0]["detail"].as_str().unwrap(),
        "// @kotowari[]",
        "detail should be the full line text, not just the marker substring"
    );
}

// @kotowari[C-core-markers-9]
#[test]
fn invalid_marker_outside_test_is_ignored() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // テスト関数の外にある空の印 → invalid_marker は出ない
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[]\n\nfn not_a_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(
        im.is_empty(),
        "invalid_marker should not fire for markers outside tests: {:?}",
        im
    );
}

// @kotowari[C-core-markers-4]
#[test]
fn invalid_marker_second_on_line_is_detected() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 1行に正常な印と空の印 → 2つ目も invalid_marker として検出される
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1] @kotowari[]\n#[test]\nfn two_markers_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(
        !im.is_empty(),
        "should detect invalid marker as second on line: {:?}",
        im
    );
}

// --- 1行に複数の印 ---

// @kotowari[C-core-markers-2]
#[test]
fn both_markers_on_one_line_bind_to_the_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    write_test_file(
        tmp.path(),
        "test_a.rs",
        "// @kotowari[C-t-a-1] @kotowari[TBL-002]\n#[test]\nfn req_001_two_markers() {}\n",
    );
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    // 確認することの行に結び付いた: テストの無い行として挙がらない
    assert!(
        findings_by_kind(&result, "confirmation_without_test").is_empty(),
        "the first marker binds C-t-a-1 to the test: {:?}",
        result
    );
    // 知らない ID にも結び付いた: 仕様に無い ID として印の行から挙がる
    let unresolved = findings_by_kind(&result, "unresolved_reference");
    assert_eq!(
        unresolved.len(),
        1,
        "the second marker binds TBL-002 to the test: {:?}",
        result
    );
    assert_eq!(unresolved[0]["detail"], "TBL-002");
    assert_eq!(unresolved[0]["path"], "tests/test_a.rs");
    assert_eq!(unresolved[0]["line"], 1);
    assert!(
        findings_by_kind(&result, "test_without_id").is_empty(),
        "the test is marked: {:?}",
        result
    );
}

// --- 印の結び付け ---

// @kotowari[C-core-markers-14]
#[test]
fn marker_before_attributes_binds() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\nfn before_attr_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(rwt.is_empty(), "marker before attr should bind: {:?}", rwt);
    assert!(twi.is_empty(), "test should have marker: {:?}", twi);
}

// @kotowari[C-core-markers-8]
#[test]
fn marker_at_body_start_does_not_bind() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn body_start_test() {\n    // @kotowari[C-t-a-1]\n    assert!(true);\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(twi.len(), 1, "{v}");
    assert_eq!(twi[0]["detail"], "body_start_test");
    assert_eq!(twi[0]["line"], 2);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert_eq!(
        rwt.len(),
        1,
        "the marker at the body start covers nothing: {v}"
    );
}

// @kotowari[C-core-markers-8]
#[test]
fn body_start_marker_does_not_add_to_the_marks_before() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 2つの確認することの行
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1", "C-t-a-2"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\nfn merged_test() {\n    // @kotowari[C-t-a-2]\n    assert!(true);\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert_eq!(
        rwt.len(),
        1,
        "only the marker before the test binds: {:?}",
        rwt
    );
    assert_eq!(rwt[0]["detail"], "C-t-a-2");
}

// @kotowari[C-core-markers-9]
#[test]
fn marker_in_body_middle_is_ignored() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn mid_body_test() {\n    let x = 1;\n    // @kotowari[C-t-a-1]\n    assert!(x == 1);\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // 本体の途中の印は無視される
    assert!(
        twi.iter().any(|f| f["detail"] == "mid_body_test"),
        "marker in body middle should be ignored: {:?}",
        twi
    );
}

// @kotowari[C-core-markers-18]
#[test]
fn mark_in_an_attribute_body_is_not_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[doc = \"@kotowari[C-t-a-999]\"]\n#[test]\nfn t() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    assert!(
        findings_by_kind(&v, "unresolved_reference").is_empty(),
        "{v}"
    );
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(twi.len(), 1, "{v}");
    assert_eq!(twi[0]["detail"], "t");
}

// @kotowari[C-core-queries-6]
#[test]
fn test_inside_a_function_body_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "fn helper() {\n    #[test]\n    fn inner() {}\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(twi.len(), 1, "{v}");
    assert_eq!(twi[0]["detail"], "inner");
}

// @kotowari[C-core-queries-7]
#[test]
fn function_in_a_mod_inside_a_macro_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    macros:\n      - proptest\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "proptest! {\n    mod nested {\n        fn inside_module() {}\n    }\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(twi.len(), 1, "{v}");
    assert_eq!(twi[0]["detail"], "inside_module");
}

// @kotowari[C-core-markers-11]
#[test]
fn marks_bind_only_to_the_first_test_on_the_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n#[test] fn a() {} #[test] fn b() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(twi.len(), 1, "{v}");
    assert_eq!(twi[0]["detail"], "b");
    let output = cmd().arg("list").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let item = v["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["id"] == "C-t-a-1")
        .unwrap();
    let names: Vec<_> = item["tests"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].clone())
        .collect();
    assert_eq!(names, vec!["a"], "{item}");
}

// @kotowari[C-core-markers-11, C-core-markers-12, C-core-queries-7]
#[test]
fn marks_bind_only_to_the_first_macro_function_on_the_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    macros:\n      - my_macro\n",
    )
    .unwrap();
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "my_macro! {\n    // @kotowari[C-t-a-1]\n    fn a() {} fn b() {}\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(twi.len(), 1, "{v}");
    assert_eq!(twi[0]["detail"], "b");
}

// @kotowari[C-core-markers-11, C-core-markers-12, C-core-queries-7]
#[test]
fn macro_function_before_a_normal_test_on_the_line_takes_the_marks() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    macros:\n      - my_macro\n",
    )
    .unwrap();
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // マクロの中の関数が同じ行の普通のテストより前に始まるので、印はマクロの中の関数に付く
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\nmy_macro! { #[allow(dead_code)] fn b() {} } #[test] fn a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(twi.len(), 1, "{v}");
    assert_eq!(twi[0]["detail"], "a");
}

// --- 問い合わせの無い言語の印 ---

// @kotowari[C-core-markers-19]
#[test]
fn unknown_language_scans_raw_text() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.go"),
        "// @kotowari[C-t-a-1]\nfunc Test_something(t *testing.T) {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    // .go の印は confirmation_without_test を消す
    assert!(
        rwt.is_empty(),
        "unknown lang markers should count for coverage: {:?}",
        rwt
    );
}

// @kotowari[C-core-markers-19]
#[test]
fn unknown_language_marker_line_is_one_indexed_from_its_own_line() {
    // 問い合わせの無い言語では、印の行は印のある行そのもの（先頭からの行番号）であり、0 ではない
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.go"),
        "// leading\n// @kotowari[C-t-a-999]\nfunc Test_something(t *testing.T) {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    let req999 = ur.iter().find(|f| f["detail"] == "C-t-a-999");
    assert!(
        req999.is_some(),
        "should find unresolved C-t-a-999: {:?}",
        ur
    );
    assert_eq!(
        req999.unwrap()["line"],
        2,
        "the marker line should be its own physical line (2), not the index into the file: {:?}",
        ur
    );
}

// --- 存在しない ID だけを指す印 ---

// @kotowari[C-core-markers-21]
#[test]
fn unresolved_only_marker_still_counts() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-999]\n#[test]\nfn unresolved_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // 印があるので test_without_id にはならない
    assert!(
        !twi.iter().any(|f| f["detail"] == "unresolved_test"),
        "unresolved marker should still count: {:?}",
        twi
    );
}

// @kotowari[C-core-confirm-4]
#[test]
fn marker_to_known_id_is_not_unresolved() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(
        ur.is_empty(),
        "a marker to an existing ID must not be unresolved: {:?}",
        ur
    );
}

// --- review の要求を指す印 ---

// --- 印から存在しない ID ---

// @kotowari[C-core-confirm-4]
#[test]
fn marker_to_unknown_id_is_unresolved() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-999]\n#[test]\nfn unresolved_marker_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f["detail"] == "C-t-a-999"),
        "should report unresolved marker: {:?}",
        ur
    );
}

// --- テストのない要求 ---

// --- 印の無いテスト ---

// @kotowari[C-core-cover-1]
#[test]
fn fixture_has_no_unmarked_test_and_one_after_removal() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 印のあるテスト
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\nfn marked_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.is_empty(),
        "marked test should not be test_without_id: {:?}",
        twi
    );

    // 印を外す
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn marked_test() {}\n",
    )
    .unwrap();
    let output2 = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v2 = parse_json(&output2);
    let twi2 = findings_by_kind(&v2, "test_without_id");
    assert_eq!(twi2.len(), 1, "should have one test_without_id: {:?}", twi2);
}

// --- 問い合わせの無い言語 ---

// @kotowari[C-core-cover-4]
#[test]
fn unknown_language_only_feeds_coverage() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.go"),
        "// @kotowari[C-t-a-1]\nfunc Test_a(t *testing.T) {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // 問い合わせの無い言語からは test_without_id は出ない
    assert!(
        twi.is_empty(),
        "unknown lang should not produce test_without_id: {:?}",
        twi
    );
}

// --- 問い合わせの無い言語の印の検査 ---

// @kotowari[C-core-markers-20]
#[test]
fn non_query_language_checks_invalid_marker() {
    // .go ファイルの空の印 → invalid_marker が出る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.go"),
        "// @kotowari[]\nfunc Test_a(t *testing.T) {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(
        !im.is_empty(),
        "invalid_marker should fire for non-query language: {:?}",
        im
    );
}

// @kotowari[C-core-confirm-4]
#[test]
fn non_query_language_checks_unresolved_reference() {
    // .go ファイルの存在しない ID の印 → unresolved_reference が出る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.go"),
        "// @kotowari[C-t-a-999]\nfunc Test_a(t *testing.T) {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f["detail"] == "C-t-a-999"),
        "unresolved_reference should fire for non-query language: {:?}",
        ur
    );
}

// --- 印の中の ID の形でない要素 ---

// @kotowari[C-core-confirm-4]
#[test]
fn malformed_id_in_marker_is_unresolved_reference_rs() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ001]\n#[test]\nfn malformed_id_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f["detail"] == "REQ001"),
        "ID の形でない要素 REQ001 に unresolved_reference が出るはず: {:?}",
        ur
    );
}

// @kotowari[C-core-confirm-4]
#[test]
fn malformed_id_in_marker_is_unresolved_reference_non_rs() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.go"),
        "// @kotowari[REQ001]\nfunc Test_a(t *testing.T) {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f["detail"] == "REQ001"),
        "問い合わせの無い言語でも ID の形でない要素 REQ001 に unresolved_reference が出るはず: {:?}",
        ur
    );
}

// --- 決定の文書が無いとき ---

// @kotowari[C-core-cover-5]
#[test]
fn no_decision_documents_still_checks_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 決定の文書なし
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn unmarked_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert_eq!(
        twi.len(),
        1,
        "should check tests even with empty IR: {:?}",
        twi
    );
    assert_eq!(twi[0]["detail"], "unmarked_test");
    // confirmation_without_test は出ない（確認することの行が無い）
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert!(rwt.is_empty());
    // 終了コード 1（誤りあり）
    assert_eq!(output.status.code(), Some(1));
}

// --- 行を持たないテストの宣言 ---

/// 行 C-t-a-1 を持つ実装済みの決定と、`tests/test_a.<ext>` に `test_source` を置いて check の JSON を返す
fn check_with_test(test_ext: &str, test_source: &str) -> (serde_json::Value, Option<i32>) {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    if test_ext != "rs" {
        fs::write(
            tmp.path().join(".kotowari/config.yaml"),
            format!("tests:\n  files:\n    - \"tests/**/*.{test_ext}\"\n"),
        )
        .unwrap();
    }
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join(format!("tests/test_a.{test_ext}")),
        test_source,
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    (parse_json(&output), output.status.code())
}

fn kinds_and_details(v: &serde_json::Value) -> Vec<(String, String)> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["kind"].as_str().unwrap().to_owned(),
                f["detail"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

// @kotowari[C-core-cover-6]
#[test]
fn c_core_cover_6_declared_test_gets_no_test_without_id_nor_unresolved_reference() {
    let (v, _) = check_with_test(
        "rs",
        "// @kotowari[C-t-a-1]\n#[test]\nfn marked() {}\n\n// @kotowari[detail]\n#[test]\nfn declared() {}\n",
    );
    assert_eq!(kinds_and_details(&v), Vec::<(String, String)>::new());
}

// @kotowari[C-core-cover-6]
#[test]
fn c_core_cover_6_declaration_beside_a_row_id_in_one_mark() {
    let (v, code) = check_with_test(
        "rs",
        "// @kotowari[C-t-a-1, detail]\n#[test]\nfn marked_and_declared() {}\n",
    );
    // 行の ID は今までどおり数え、宣言は何も出さない
    assert_eq!(kinds_and_details(&v), Vec::<(String, String)>::new());
    assert_eq!(code, Some(0));
}

// @kotowari[C-core-cover-7]
#[test]
fn c_core_cover_7_declaration_does_not_satisfy_a_row() {
    let (v, _) = check_with_test("rs", "// @kotowari[detail]\n#[test]\nfn declared() {}\n");
    assert_eq!(
        kinds_and_details(&v),
        vec![("confirmation_without_test".to_owned(), "C-t-a-1".to_owned())]
    );
}

// @kotowari[detail]
#[test]
fn declaration_in_a_language_without_a_query_counts_for_nothing() {
    let (v, _) = check_with_test("go", "// @kotowari[detail]\nfunc Test_a(t *testing.T) {}\n");
    // invalid_marker も unresolved_reference も出さず、行も満たさない
    assert_eq!(
        kinds_and_details(&v),
        vec![("confirmation_without_test".to_owned(), "C-t-a-1".to_owned())]
    );
}

// @kotowari[detail]
#[test]
fn declaration_word_is_matched_exactly() {
    let (v, _) = check_with_test(
        "rs",
        "// @kotowari[C-t-a-1, Detail]\n#[test]\nfn capitalised() {}\n",
    );
    assert_eq!(
        kinds_and_details(&v),
        vec![("unresolved_reference".to_owned(), "Detail".to_owned())]
    );
}

// @kotowari[detail]
#[test]
fn declaration_is_not_counted_in_status_marks() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1, detail]\n#[test]\nfn a() {}\n\n// @kotowari[detail]\n#[test]\nfn b() {}\n",
    )
    .unwrap();
    let output = cmd()
        .arg("status")
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let v = parse_json(&output);
    assert_eq!(v["tests"]["marks"], 1, "{v}");
}

// --- 空行を挟んだ印は結び付かない ---

// @kotowari[C-core-markers-13]
#[test]
fn blank_line_between_marker_and_test_breaks_binding() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 印と #[test] の間に空行がある → 結び付かない
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n\n#[test]\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "test_a"),
        "blank line should break marker binding: {:?}",
        twi
    );
}

// @kotowari[C-core-markers-13]
#[test]
fn blank_line_between_marker_and_function_breaks_binding() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 属性の後の印と関数の間に空行がある → 結び付かない
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\n// @kotowari[C-t-a-1]\n\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "test_a"),
        "blank line should break marker binding: {:?}",
        twi
    );
}

// @kotowari[C-core-markers-12]
#[test]
fn macro_function_boundary_breaks_marker_binding() {
    // マクロ内で印 → fn a() → fn b()（空行なし）
    // b には印が無いので test_without_id が出る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    macros:\n      - my_macro\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "my_macro! {\n    // @kotowari[C-t-a-1]\n    fn a() {}\n    fn b() {}\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "b"),
        "fn b should be test_without_id because marker belongs to fn a: {:?}",
        twi
    );
    // fn a は印を持つので test_without_id にはならない
    assert!(
        !twi.iter().any(|f| f["detail"] == "a"),
        "fn a should NOT be test_without_id: {:?}",
        twi
    );
}

// --- UTF-8 でないテストファイルで停止 ---

// @kotowari[C-core-clienv-7]
#[test]
#[cfg(unix)]
fn unreadable_test_file_stops() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    let test_path = tmp.path().join("tests/unreadable.rs");
    fs::write(&test_path, b"#[test]\nfn t() {}\n").unwrap();
    fs::set_permissions(&test_path, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    // root ではパーミッションが効かないのでスキップ
    if std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
    {
        return;
    }
    assert_eq!(
        output.status.code(),
        Some(2),
        "unreadable test file should stop with exit code 2"
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
}

// @kotowari[C-core-clienv-7]
#[test]
fn non_utf8_test_file_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 非 UTF-8 バイト列
    fs::write(tmp.path().join("tests/bad.rs"), b"\xff\xfe").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    // 終了コード 2（停止）
    assert_eq!(
        output.status.code(),
        Some(2),
        "non-UTF-8 test file should stop with exit code 2"
    );
    // 標準出力は空
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
    // 停止の詳細に絶対パスが含まれない
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains(tmp.path().to_str().unwrap()),
        "stderr should not contain absolute path, got: {stderr:?}"
    );
}

// --- ファイルのシンボリックリンクは読む ---

// @kotowari[C-core-discover-2, C-core-discover-3]
#[test]
#[cfg(unix)]
fn file_symlink_is_read() {
    // ファイルのシンボリックリンクは読む（ディレクトリのリンクは辿らない）
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::create_dir_all(tmp.path().join("elsewhere")).unwrap();
    fs::write(
        tmp.path().join("elsewhere/linked_test.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\nfn linked_test() {}\n",
    )
    .unwrap();
    // ファイルへのシンボリックリンク → 読まれる
    symlink(
        tmp.path().join("elsewhere/linked_test.rs"),
        tmp.path().join("tests/linked_test.rs"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert!(
        rwt.is_empty(),
        "ファイルのシンボリックリンクのテストは読まれるので C-t-a-1 はカバーされるはず: {:?}",
        rwt
    );
    // ディレクトリのシンボリックリンクは辿らない
    fs::create_dir_all(tmp.path().join("linked_dir_target")).unwrap();
    fs::write(
        tmp.path().join("linked_dir_target/another_test.rs"),
        "#[test]\nfn another_test() {}\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        tmp.path().join("linked_dir_target"),
        tmp.path().join("tests/linked_dir"),
    )
    .unwrap();
    let output2 = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v2 = parse_json(&output2);
    let twi = findings_by_kind(&v2, "test_without_id");
    assert!(
        !twi.iter().any(|f| f["detail"] == "another_test"),
        "ディレクトリのシンボリックリンクの先のテストは走査されないはず: {:?}",
        twi
    );
}

// @kotowari[C-core-clienv-7, C-core-config-11, C-core-discover-17]
#[test]
#[cfg(unix)]
fn unreadable_directory_under_tests_stops() {
    use std::os::unix::fs::PermissionsExt;
    if std::process::Command::new("id")
        .arg("-u")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
    {
        return;
    }
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    let sub = tmp.path().join("tests/sub");
    fs::create_dir_all(&sub).unwrap();
    fs::write(
        sub.join("hidden_test.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\nfn hidden_test() {}\n",
    )
    .unwrap();
    fs::set_permissions(&sub, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    fs::set_permissions(&sub, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "an unreadable directory in the test walk should stop: {:?}",
        output
    );
    assert!(output.stdout.is_empty(), "stdout should be empty on stop");
}

// --- glob は再帰し、隠しディレクトリを含めない ---

// @kotowari[C-core-config-22]
#[test]
fn hidden_directory_is_excluded_and_subdirectory_is_included() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 再帰が効くことの確認: tests/sub/deep.rs に印の無いテストを置く
    fs::create_dir_all(tmp.path().join("tests/sub")).unwrap();
    fs::write(
        tmp.path().join("tests/sub/deep.rs"),
        "#[test]\nfn deep_test() {}\n",
    )
    .unwrap();
    // 隠しディレクトリの除外の確認: tests/.hidden/hidden.rs にテストを置く
    fs::create_dir_all(tmp.path().join("tests/.hidden")).unwrap();
    fs::write(
        tmp.path().join("tests/.hidden/hidden.rs"),
        "#[test]\nfn hidden_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // tests/sub/deep.rs のテストは再帰で発見される
    assert!(
        twi.iter().any(|f| f["detail"] == "deep_test"),
        "subdirectory test should be found via recursive glob: {:?}",
        twi
    );
    // tests/.hidden/ のテストは除外される
    assert!(
        !twi.iter().any(|f| f["detail"] == "hidden_test"),
        "hidden directory test should be excluded: {:?}",
        twi
    );
}

// --- has_attribute: #[test] の前にコメント行がある関数 ---

// @kotowari[C-core-queries-4]
#[test]
fn test_attribute_after_comment_is_recognized() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1]\n#[test]\n// intermediate comment\nfn after_comment_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        !twi.iter().any(|f| f["detail"] == "after_comment_test"),
        "function with comment between #[test] and fn should be recognized as test: {:?}",
        twi
    );
    // 認識されていれば確認することの行は印で結び付いているので confirmation_without_test は出ない
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert!(
        !rwt.iter().any(|f| f["detail"] == "C-t-a-1"),
        "the test after the comment should bind C-t-a-1: {:?}",
        rwt
    );
}

// --- has_configured_attribute: カスタム属性を持たない関数はテストにならない ---

// @kotowari[C-core-queries-5]
#[test]
fn function_without_configured_attribute_not_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    attributes:\n      - kani::proof\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::write(tmp.path().join("src/lib.rs"), "fn helper_function() {}\n").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        !twi.iter().any(|f| f["detail"] == "helper_function"),
        "function without configured attribute should not be counted as test: {:?}",
        twi
    );
}

// --- collect_markers の行番号 ---

// @kotowari[C-core-markers-7]
#[test]
fn unresolved_reference_line_is_the_marker_line() {
    // unresolved_reference の line は印のある行
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "\n// @kotowari[C-t-a-999]\n#[test]\nfn marker_line_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    let req999 = ur.iter().find(|f| f["detail"] == "C-t-a-999");
    assert!(
        req999.is_some(),
        "should find unresolved C-t-a-999: {:?}",
        ur
    );
    assert_eq!(
        req999.unwrap()["line"],
        2,
        "unresolved_reference line should be the marker line (2), not the fn line"
    );
}

// --- Step 6: テストの数え方と印 ---

// @kotowari[C-core-queries-4]
#[test]
fn attribute_path_ending_in_test_is_counted() {
    // "#[ test ]" や "#[core::prelude::v1::test]" も数える
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[  test  ]\nfn spaced_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "spaced_test"),
        "spaced #[ test ] should be counted: {:?}",
        twi
    );
}

// @kotowari[C-core-queries-7]
#[test]
fn nested_function_in_macro_is_not_counted() {
    // マクロの中の入れ子の関数は数えない（最上位だけ）
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  rust:\n    macros:\n      - my_macro\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "my_macro! {\n    fn outer() {\n        fn inner() {}\n    }\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "outer"),
        "outer function should be counted: {:?}",
        twi
    );
    assert!(
        !twi.iter().any(|f| f["detail"] == "inner"),
        "inner function should not be counted: {:?}",
        twi
    );
}

// @kotowari[C-core-markers-8, C-core-markers-12]
#[test]
fn macro_function_body_marker_does_not_bind() {
    // マクロの中の関数でも、本体の先頭のコメントの印は結び付かない
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\ntests:\n  rust:\n    macros:\n      - my_macro\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "my_macro! {\n    fn body_marker() {\n        // @kotowari[C-t-a-1]\n        assert!(true);\n    }\n}\n",
    ).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert_eq!(
        rwt.len(),
        1,
        "body marker in macro function should not bind: {:?}",
        rwt
    );
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "body_marker"),
        "{:?}",
        twi
    );
}

// @kotowari[C-core-markers-12]
#[test]
fn macro_function_block_comment_marker_binds() {
    // マクロの中の関数でも、通常の関数と同じ規則で複数行のブロックコメントの印が結び付く
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\ntests:\n  rust:\n    macros:\n      - my_macro\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "my_macro! {\n    /* @kotowari[C-t-a-1]\n    */\n    fn block_comment_marker() {}\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert!(
        rwt.is_empty(),
        "a multi-line block comment marker right before a macro function should bind, just like a normal function: {:?}",
        rwt
    );
    // 対: 同じ形で印を外すと confirmation_without_test が出る（印が結び付いたから空だった、の裏付け）
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "my_macro! {\n    /* no marker here\n    */\n    fn block_comment_marker() {}\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "confirmation_without_test");
    assert_eq!(
        rwt.len(),
        1,
        "without the marker C-t-a-1 must be reported: {:?}",
        rwt
    );
}

// @kotowari[C-core-confirm-4]
#[test]
fn duplicate_marker_id_in_same_test_reports_per_occurrence() {
    // 同じテストに同じ ID を指す印が複数あるとき、unresolved_reference は出現ごとに1件
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-999]\n// @kotowari[C-t-a-999]\n#[test]\nfn duplicate_marker_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    let matching: Vec<_> = ur.iter().filter(|f| f["detail"] == "C-t-a-999").collect();
    assert_eq!(
        matching.len(),
        2,
        "each occurrence of the same unresolved ID pointing at one test should produce its own finding: {:?}",
        ur
    );
    let mut lines: Vec<u64> = matching
        .iter()
        .map(|f| f["line"].as_u64().unwrap())
        .collect();
    lines.sort();
    assert_eq!(lines, vec![1, 2]);
}

// @kotowari[C-core-markers-5]
#[test]
fn marker_spanning_lines_is_invalid() {
    // 行をまたぐ印は invalid_marker
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1\n// ]\n#[test]\nfn spanning_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(
        !im.is_empty(),
        "spanning marker should produce invalid_marker: {:?}",
        im
    );
}

// @kotowari[C-core-markers-6]
#[test]
fn detail_is_the_raw_line() {
    // invalid_marker の detail は生の行
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[C-t-a-1\n#[test]\nfn spanning_detail_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let im = findings_by_kind(&v, "invalid_marker");
    assert!(!im.is_empty(), "should have invalid_marker");
    assert_eq!(
        im[0]["detail"], "// @kotowari[C-t-a-1",
        "detail should be the raw line text"
    );
}

// @kotowari[C-core-discover-7]
#[test]
fn uppercase_extension_has_no_query() {
    // ".RS" は問い合わせの無い言語（大文字小文字を区別）
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.RS\"\n    - \"tests/**/*.rs\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.RS"),
        "#[test]\nfn uppercase_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // .RS ファイルは問い合わせの無い言語なので test_without_id は出ない
    assert!(
        !twi.iter().any(|f| f["detail"] == "uppercase_test"),
        ".RS should not produce test_without_id: {:?}",
        twi
    );
}

// @kotowari[C-core-config-23]
#[test]
fn hidden_file_matched_by_glob_is_read() {
    // ".foo.rs" は glob が当てれば読まれる
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/.hidden_test.rs"),
        "#[test]\nfn hidden_file_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        twi.iter().any(|f| f["detail"] == "hidden_file_test"),
        "hidden file matched by glob should be read: {:?}",
        twi
    );
}

// @kotowari[C-core-config-11, C-core-discover-14]
#[test]
#[cfg(unix)]
fn broken_symlink_in_tests_stops() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    symlink(
        tmp.path().join("nowhere.rs"),
        tmp.path().join("tests/broken.rs"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "a broken symlink in the test walk must stop: {:?}",
        output
    );
    assert!(output.stdout.is_empty());
}

// @kotowari[C-core-config-11, C-core-discover-15]
#[test]
#[cfg(unix)]
fn broken_symlink_outside_glob_stops() {
    // 走査は基準のディレクトリ全体（隠しディレクトリを除く）を歩いてから glob で選ぶので、
    // glob に当たらない場所（既定は src/**/*.rs, tests/**/*.rs）の先の無いシンボリックリンクでも停止する
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    symlink(tmp.path().join("nowhere.txt"), tmp.path().join("notes.txt")).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(
        output.status.code(),
        Some(2),
        "a broken symlink outside every configured glob must still stop the walk: {:?}",
        output
    );
    assert!(output.stdout.is_empty());
}

// @kotowari[C-core-base-6, C-core-document-1, C-core-output-7]
#[test]
fn confirmation_without_test_path_carries_the_subdirectory() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(
        tmp.path(),
        "docs/decisions/network/dns/timeout.md",
        &["C-network-timeout-1"],
    );
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let missing = findings_by_kind(&result, "confirmation_without_test");
    assert_eq!(missing.len(), 1, "{:?}", missing);
    assert_eq!(missing[0]["path"], "docs/decisions/network/dns/timeout.md");
    assert_eq!(missing[0]["detail"], "C-network-timeout-1");
}

// --- テストのない具体例 ---

fn write_test_file(tmp: &std::path::Path, name: &str, content: &str) {
    fs::create_dir_all(tmp.join("tests")).unwrap();
    fs::write(tmp.join("tests").join(name), content).unwrap();
}

// @kotowari[C-core-markers-1]
#[test]
fn one_marker_may_name_several_scenarios() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1", "C-t-a-2"]);
    // 1つの印が2つの ID を挙げ、もう1本が同じ C-t-a-1 をもう一度挙げる
    write_test_file(
        tmp.path(),
        "test_a.rs",
        "// @kotowari[C-t-a-1, C-t-a-2]\n#[test]\nfn req_001_both() {}\n\n// @kotowari[C-t-a-1]\n#[test]\nfn req_001_again() {}\n",
    );
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let cwt = findings_by_kind(&result, "confirmation_without_test");
    assert!(
        cwt.is_empty(),
        "one marker may name several rows, and the count of markers is not read: {:?}",
        result
    );
}

// @kotowari[C-core-cover-3]
#[test]
fn marker_in_a_file_without_query_feeds_scenario_coverage() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_decision_with_rows(tmp.path(), "docs/decisions/t/a.md", &["C-t-a-1"]);
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files:\n    - \"tests/**/*.go\"\n",
    )
    .unwrap();
    // 印は問い合わせの無い言語のファイルにだけある
    write_test_file(
        tmp.path(),
        "test_a.go",
        "// @kotowari[C-t-a-1]\nfunc Test_a(t *testing.T) {}\n",
    );
    let result = parse_json(&cmd().arg("check").current_dir(tmp.path()).output().unwrap());
    let cwt = findings_by_kind(&result, "confirmation_without_test");
    assert!(
        cwt.is_empty(),
        "the marker in the file of a language without a query counts for the row: {:?}",
        result
    );
}

// --- 具体例の印は要求の分も満たす ---

// --- ir-references.md の具体例 ---

// --- ID の名前（2026-09-22-id-namespace.md）---
