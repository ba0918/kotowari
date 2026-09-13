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

fn make_ir_with_req(tmp: &std::path::Path, req_id: &str, verification: &str) {
    fs::write(
        tmp.join("docs/ir/a.md"),
        format!(
            "# Title\n\nScope.\n\n## 要求\n\n### {req_id}: Test\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: {verification}\n\nStatement.\n"
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

// --- REQ-079: テストのファイル ---

// @kotowari[REQ-079]
#[test]
fn req_079_reads_files_matching_the_globs() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n#[test]\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(rwt.is_empty(), "REQ-001 should have test coverage: {:?}", rwt);
}

// --- REQ-080: tree-sitter で読む ---

// @kotowari[REQ-080]
#[test]
fn req_080_uses_tree_sitter_with_bundled_rust_query() {
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
    assert!(twi.iter().any(|f| f["detail"] == "my_test"), "should find test via tree-sitter: {:?}", twi);
}

// --- REQ-081: .rs だけが問い合わせのある言語 ---

// @kotowari[REQ-081]
#[test]
fn req_081_only_rs_maps_to_rust() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // .py ファイルにテストっぽいものを書いても test_without_id は出ない
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.py"),
        "def test_something():\n    pass\n",
    )
    .unwrap();
    // ただし .py ファイルは glob に当たらないので tests.files に追加
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"tests/**/*.py\"\n    - \"tests/**/*.rs\"\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // .py ファイルからは test_without_id は出ない
    assert!(!twi.iter().any(|f| f["detail"] == "test_something"), ".py should not detect tests: {:?}", twi);
}

// --- REQ-082: Rust のテスト ---

// @kotowari[REQ-082, TBL-017]
#[test]
fn req_082_test_attribute_is_always_counted() {
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
    assert!(twi.iter().any(|f| f["detail"] == "counted_test"), "should count #[test]: {:?}", twi);
}

// @kotowari[REQ-082, TBL-017]
#[test]
fn req_082_configured_attribute_matches_path_with_arguments() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  rust:\n    attributes:\n      - kani::proof\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::write(
        tmp.path().join("src/lib.rs"),
        "// @kotowari[REQ-001]\n#[kani::proof(unwind = 3)]\nfn my_proof() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(rwt.is_empty(), "kani::proof should count as test: {:?}", rwt);
}

// @kotowari[REQ-082, TBL-017]
#[test]
fn req_082_macro_body_functions_are_counted_by_last_segment() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  rust:\n    macros:\n      - proptest\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "proptest::proptest! {\n    // @kotowari[REQ-001]\n    #[test]\n    fn my_prop_test(x in 0..100u32) {\n        assert!(x < 101);\n    }\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    // proptest マクロ内の関数がテストとして数えられ、REQ-001 に結び付く
    assert!(rwt.is_empty(), "proptest function should cover REQ-001: {:?}", rwt);
}

// --- REQ-083: 読めないファイル ---

// @kotowari[REQ-083]
#[test]
fn req_083_unparsable_file_is_skipped() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(tmp.path().join("tests/broken.rs"), "this is not valid rust {{{{").unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    // unparsable_file は停止しない（終了コード 2 にならない）
    assert_ne!(output.status.code(), Some(2), "should not stop");
    // unparsable_file の指摘が出る
    let uf = findings_by_kind(&v, "unparsable_file");
    assert!(
        uf.iter().any(|f| f["detail"].as_str().unwrap().contains("broken.rs")),
        "should report unparsable_file for syntax error: {:?}",
        uf
    );
}

// --- REQ-071: 印の構文 ---

// @kotowari[REQ-071, TBL-015]
#[test]
fn req_071_marker_syntax_allows_spaces_around_commas() {
    let markers = kotowari::tests_discovery::parse_markers_in_line(
        "// @kotowari[REQ-001 , TBL-002]",
        1,
    );
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].ids, vec!["REQ-001", "TBL-002"]);
}

// --- REQ-072: 形の誤った印 ---

// @kotowari[REQ-072]
#[test]
fn req_072_invalid_marker() {
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

// @kotowari[REQ-072, TBL-008]
#[test]
fn req_072_invalid_marker_detail_is_line_text() {
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

// @kotowari[REQ-072, TBL-016]
#[test]
fn req_072_invalid_marker_outside_test_is_ignored() {
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

// @kotowari[REQ-072, REQ-073]
#[test]
fn req_072_invalid_marker_second_on_line_is_detected() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 1行に正常な印と空の印 → 2つ目も invalid_marker として検出される
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001] @kotowari[]\n#[test]\nfn two_markers_test() {}\n",
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

// --- REQ-073: 1行に複数の印 ---

// @kotowari[REQ-073]
#[test]
fn req_073_several_markers_on_one_line() {
    let markers = kotowari::tests_discovery::parse_markers_in_line(
        "// @kotowari[REQ-001] @kotowari[TBL-002]",
        1,
    );
    assert_eq!(markers.len(), 2);
}

// --- REQ-074: コメント記号を見ない ---

// @kotowari[REQ-074]
#[test]
fn req_074_marker_anywhere_in_the_line_regardless_of_comment_syntax() {
    let markers = kotowari::tests_discovery::parse_markers_in_line(
        "/* @kotowari[REQ-001] */",
        1,
    );
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].ids, vec!["REQ-001"]);
}

// --- REQ-075: 印の結び付け ---

// @kotowari[REQ-075, TBL-016]
#[test]
fn req_075_marker_before_attributes_binds() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n#[test]\nfn before_attr_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(rwt.is_empty(), "marker before attr should bind: {:?}", rwt);
    assert!(twi.is_empty(), "test should have marker: {:?}", twi);
}

// @kotowari[REQ-075, TBL-016]
#[test]
fn req_075_marker_at_body_start_binds() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn body_start_test() {\n    // @kotowari[REQ-001]\n    assert!(true);\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(rwt.is_empty(), "marker at body start should bind: {:?}", rwt);
    assert!(twi.is_empty(), "test should have marker: {:?}", twi);
}

// @kotowari[REQ-075, TBL-016]
#[test]
fn req_075_both_places_merge_ids() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 2つの要求
    fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\n\nScope.\n\n## 要求\n\n### REQ-001: A\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nStmt.\n\n### REQ-002: B\n\n- 種類: ubiquitous\n- 出典: docs/decision/brainstorm/records.md#A1\n- 検証: unit\n\nStmt.\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n#[test]\nfn merged_test() {\n    // @kotowari[REQ-002]\n    assert!(true);\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(rwt.is_empty(), "both markers should merge: {:?}", rwt);
}

// @kotowari[REQ-075, TBL-016]
#[test]
fn req_075_marker_in_body_middle_is_ignored() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn mid_body_test() {\n    let x = 1;\n    // @kotowari[REQ-001]\n    assert!(x == 1);\n}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // 本体の途中の印は無視される
    assert!(twi.iter().any(|f| f["detail"] == "mid_body_test"), "marker in body middle should be ignored: {:?}", twi);
}

// --- REQ-076: 問い合わせの無い言語の印 ---

// @kotowari[REQ-076]
#[test]
fn req_076_unknown_language_scans_raw_text() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"tests/**/*.py\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.py"),
        "# @kotowari[REQ-001]\ndef test_something():\n    pass\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    // .py の印は requirement_without_test を消す
    assert!(rwt.is_empty(), "unknown lang markers should count for coverage: {:?}", rwt);
}

// --- REQ-077: 存在しない ID だけを指す印 ---

// @kotowari[REQ-077]
#[test]
fn req_077_unresolved_only_marker_still_counts() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-999]\n#[test]\nfn unresolved_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // 印があるので test_without_id にはならない
    assert!(!twi.iter().any(|f| f["detail"] == "unresolved_test"), "unresolved marker should still count: {:?}", twi);
}

// @kotowari[REQ-054, REQ-077]
#[test]
fn req_054_marker_to_known_id_is_not_unresolved() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n#[test]\nfn test_a() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(ur.is_empty(), "a marker to an existing ID must not be unresolved: {:?}", ur);
}

// --- REQ-078: review の要求を指す印 ---

// @kotowari[REQ-078]
#[test]
fn req_078_marker_to_review_requirement_is_not_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "review");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // review の要求にテストがなくても requirement_without_test にならない
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(rwt.is_empty(), "review requirements don't need tests: {:?}", rwt);
}

// --- REQ-054: 印から存在しない ID ---

// @kotowari[REQ-054]
#[test]
fn req_054_marker_to_unknown_id_is_unresolved() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-999]\n#[test]\nfn unresolved_marker_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(ur.iter().any(|f| f["detail"] == "REQ-999"), "should report unresolved marker: {:?}", ur);
}

// --- REQ-085: テストのない要求 ---

// @kotowari[REQ-085]
#[test]
fn req_085_fixture_has_no_uncovered_requirement() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n#[test]\nfn covered_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(rwt.is_empty(), "all requirements covered: {:?}", rwt);
}

// --- REQ-086: 印の無いテスト ---

// @kotowari[REQ-086]
#[test]
fn req_086_fixture_has_no_unmarked_test_and_one_after_removal() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 印のあるテスト
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n#[test]\nfn marked_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(twi.is_empty(), "marked test should not be test_without_id: {:?}", twi);

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

// --- REQ-087: 問い合わせの無い言語 ---

// @kotowari[REQ-087]
#[test]
fn req_087_unknown_language_only_feeds_coverage() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"tests/**/*.py\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.py"),
        "# @kotowari[REQ-001]\ndef test_a():\n    pass\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    // 問い合わせの無い言語からは test_without_id は出ない
    assert!(twi.is_empty(), "unknown lang should not produce test_without_id: {:?}", twi);
}

// --- REQ-072, REQ-054: 問い合わせの無い言語の印の検査 ---

// @kotowari[REQ-072, REQ-054]
#[test]
fn req_072_non_query_language_checks_invalid_marker() {
    // .py ファイルの空の印 → invalid_marker が出る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"tests/**/*.py\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.py"),
        "# @kotowari[]\ndef test_a():\n    pass\n",
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

// @kotowari[REQ-054]
#[test]
fn req_054_non_query_language_checks_unresolved_reference() {
    // .py ファイルの存在しない ID の印 → unresolved_reference が出る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"tests/**/*.py\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.py"),
        "# @kotowari[REQ-999]\ndef test_a():\n    pass\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    assert!(
        ur.iter().any(|f| f["detail"] == "REQ-999"),
        "unresolved_reference should fire for non-query language: {:?}",
        ur
    );
}

// --- REQ-077: 印の中の ID の形でない要素 ---

// @kotowari[REQ-077]
#[test]
fn req_077_malformed_id_in_marker_is_unresolved_reference_rs() {
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

// @kotowari[REQ-077]
#[test]
fn req_077_malformed_id_in_marker_is_unresolved_reference_non_rs() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  files:\n    - \"tests/**/*.py\"\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.py"),
        "# @kotowari[REQ001]\ndef test_a():\n    pass\n",
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

// --- REQ-088: IR に文書が無いとき ---

// @kotowari[REQ-088]
#[test]
fn req_088_empty_ir_still_checks_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // IR 文書なし
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "#[test]\nfn unmarked_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(twi.iter().any(|f| f["detail"] == "unmarked_test"), "should check tests even with empty IR: {:?}", twi);
    // requirement_without_test は出ない（IR に要求がない）
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(rwt.is_empty());
    // 終了コード 1（誤りあり）
    assert_eq!(output.status.code(), Some(1));
}

// --- TBL-016: 空行を挟んだ印は結び付かない ---

// @kotowari[REQ-075, TBL-016]
#[test]
fn tbl_016_blank_line_between_marker_and_test_breaks_binding() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    // 印と #[test] の間に空行がある → 結び付かない
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n\n#[test]\nfn test_a() {}\n",
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

// @kotowari[REQ-075, TBL-016]
#[test]
fn tbl_016_macro_function_boundary_breaks_marker_binding() {
    // マクロ内で @kotowari[REQ-001] → fn a() → fn b()（空行なし）
    // b には印が無いので test_without_id が出る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  rust:\n    macros:\n      - my_macro\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "my_macro! {\n    // @kotowari[REQ-001]\n    fn a() {}\n    fn b() {}\n}\n",
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

// --- TBL-001: UTF-8 でないテストファイルで停止 ---

// @kotowari[REQ-006, TBL-001]
#[test]
#[cfg(unix)]
fn tbl_001_unreadable_test_file_stops() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    let test_path = tmp.path().join("tests/unreadable.rs");
    fs::write(&test_path, b"#[test]\nfn t() {}\n").unwrap();
    fs::set_permissions(&test_path, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    // root ではパーミッションが効かないのでスキップ
    if std::process::Command::new("id").arg("-u").output()
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

// @kotowari[REQ-006, TBL-001]
#[test]
fn tbl_001_non_utf8_test_file_stops() {
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
}

// --- REQ-079: シンボリックリンクを辿らない ---

// @kotowari[REQ-079]
#[test]
#[cfg(unix)]
fn req_079_symlink_is_not_followed() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::create_dir_all(tmp.path().join("elsewhere")).unwrap();
    fs::write(
        tmp.path().join("elsewhere/linked_test.rs"),
        "// @kotowari[REQ-001]\n#[test]\nfn linked_test() {}\n",
    )
    .unwrap();
    symlink(
        tmp.path().join("elsewhere/linked_test.rs"),
        tmp.path().join("tests/linked_test.rs"),
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let twi = findings_by_kind(&v, "test_without_id");
    assert!(
        !twi.iter().any(|f| f["detail"] == "linked_test"),
        "シンボリックリンク先のテストは走査されないはず: {:?}",
        twi
    );
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(
        rwt.iter().any(|f| f["detail"] == "REQ-001"),
        "シンボリックリンクが辿られないので REQ-001 はテスト無しのはず: {:?}",
        rwt
    );
}

// --- REQ-019: glob は再帰し、隠しディレクトリを含めない ---

// @kotowari[REQ-019]
#[test]
fn req_019_hidden_directory_is_excluded_and_subdirectory_is_included() {
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

// @kotowari[REQ-082, TBL-017]
#[test]
fn req_082_test_attribute_after_comment_is_recognized() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    make_ir_with_req(tmp.path(), "REQ-001", "unit");
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "// @kotowari[REQ-001]\n#[test]\n// intermediate comment\nfn after_comment_test() {}\n",
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
    // 認識されていれば REQ-001 は印で結び付いているので requirement_without_test は出ない
    let rwt = findings_by_kind(&v, "requirement_without_test");
    assert!(
        !rwt.iter().any(|f| f["detail"] == "REQ-001"),
        "the test after the comment should bind REQ-001: {:?}",
        rwt
    );
}

// --- has_configured_attribute: カスタム属性を持たない関数はテストにならない ---

// @kotowari[REQ-082, TBL-017]
#[test]
fn req_082_function_without_configured_attribute_not_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/brainstorm\n  adr: docs/decision/adr\ntests:\n  rust:\n    attributes:\n      - kani::proof\n",
    )
    .unwrap();
    fs::create_dir_all(tmp.path().join("src")).unwrap();
    fs::write(
        tmp.path().join("src/lib.rs"),
        "fn helper_function() {}\n",
    )
    .unwrap();
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

// @kotowari[REQ-071, TBL-015]
#[test]
fn req_071_marker_line_number_is_reported() {
    let markers = kotowari::tests_discovery::parse_markers_in_line(
        "// @kotowari[REQ-001]",
        42,
    );
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].line, 42, "marker line should match the given line number");
}

// @kotowari[REQ-054]
#[test]
fn req_054_marker_unresolved_reference_reports_fn_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    fs::create_dir_all(tmp.path().join("tests")).unwrap();
    fs::write(
        tmp.path().join("tests/test_a.rs"),
        "\n// @kotowari[REQ-999]\n#[test]\nfn marker_line_test() {}\n",
    )
    .unwrap();
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    let v = parse_json(&output);
    let ur = findings_by_kind(&v, "unresolved_reference");
    let req999 = ur.iter().find(|f| f["detail"] == "REQ-999");
    assert!(req999.is_some(), "should find unresolved REQ-999: {:?}", ur);
    assert_eq!(
        req999.unwrap()["line"], 4,
        "unresolved_reference line should be the fn line (4)"
    );
}
