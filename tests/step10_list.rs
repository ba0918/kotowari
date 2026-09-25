//! "kotowari list" の項目の組み立てと出力（REQ-core-151〜REQ-core-155、TBL-core-026）

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。`test_globs` が空でなければ "tests.files" に書く
fn make_project(tmp: &Path, test_globs: &[&str]) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "tests",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    let mut config =
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n"
            .to_string();
    if !test_globs.is_empty() {
        config.push_str("tests:\n  files:\n");
        for glob in test_globs {
            config.push_str(&format!("    - {glob}\n"));
        }
    }
    std::fs::write(tmp.join(".kotowari/config.yaml"), config).unwrap();
    std::fs::write(
        tmp.join("docs/decision/records/records.md"),
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    )
    .unwrap();
}

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// "kotowari list" を走らせて (終了コード, 標準出力, 標準エラー) を返す
fn run_list_raw(tmp: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("list")
        .args(args)
        .current_dir(tmp)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

/// "kotowari list" を走らせ、終了コード 0 を確かめて JSON を返す
fn run_list(tmp: &Path) -> serde_json::Value {
    let (code, stdout, stderr) = run_list_raw(tmp, &[]);
    assert_eq!(code, Some(0), "list should exit 0: {stderr}");
    serde_json::from_str(&stdout).expect("valid JSON")
}

/// "kotowari list --format text" を走らせ、終了コード 0 を確かめて標準出力を返す
fn run_list_text(tmp: &Path) -> String {
    let (code, stdout, stderr) = run_list_raw(tmp, &["--format", "text"]);
    assert_eq!(code, Some(0), "list should exit 0: {stderr}");
    stdout
}

/// "items" から ID で1件を引く
fn item<'a>(v: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    v["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["id"] == id)
        .unwrap_or_else(|| panic!("no item {id} in {v}"))
}

/// 1件の鍵の名前を並べる
fn keys(item: &serde_json::Value) -> Vec<String> {
    let mut names: Vec<String> = item.as_object().unwrap().keys().cloned().collect();
    names.sort();
    names
}

/// 要求の見出しと行を作る
fn requirement(id: &str, name: &str, verification: &str) -> String {
    format!(
        "### {id}: {name}\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: {verification}\n\n文である。\n\n"
    )
}

// --- REQ-core-151: list の読み取り ---

// @kotowari[REQ-core-151, REQ-core-153, TBL-core-026, EX-core-245]
#[test]
fn req_151_requirement_with_a_marked_test_is_listed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "例", "unit")
        ),
    );
    write(
        tmp.path(),
        "tests/a.rs",
        // 印を3行目に置く（EX-core-245 と EX-core-248 は印の行が 3）
        "\n\n// @kotowari[REQ-001]\n#[test]\nfn req_001_x() {}\n",
    );
    let v = run_list(tmp.path());
    let req = item(&v, "REQ-001");
    assert_eq!(req["kind"], "requirement");
    assert_eq!(req["name"], "例");
    assert_eq!(req["verification"], "unit");
    assert_eq!(req["path"], "docs/ir/a.md");
    assert_eq!(req["line"], 7);
    let tests = req["tests"].as_array().unwrap();
    assert_eq!(tests.len(), 1, "one marked test: {req}");
    assert_eq!(tests[0]["path"], "tests/a.rs");
    assert_eq!(tests[0]["line"], 3);
    assert_eq!(tests[0]["name"], "req_001_x");
}

// @kotowari[REQ-core-151, EX-core-246]
#[test]
fn req_151_items_are_listed_despite_ir_errors_and_exit_zero() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    // "- verification:" の行の無い要求（check なら verification_missing と requirement_without_test）
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n### REQ-002: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n\n文である。\n",
    );
    let (code, stdout, stderr) = run_list_raw(tmp.path(), &[]);
    assert_eq!(
        code,
        Some(0),
        "an IR error does not change the exit code: {stderr}"
    );
    assert_eq!(stderr, "", "list writes no finding to stderr");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let req = item(&v, "REQ-002");
    assert!(
        req["verification"].is_null(),
        "the missing line is null: {req}"
    );
    assert!(
        v.get("findings").is_none(),
        "list writes no finding to stdout: {v}"
    );
}

// --- REQ-core-153: 項目の形 ---

// @kotowari[REQ-core-153, TBL-core-026, EX-core-247]
#[test]
fn req_153_test_in_a_language_without_a_query_has_a_null_name() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.go"]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "例", "unit")
        ),
    );
    write(
        tmp.path(),
        "tests/a.go",
        "func x() {\n    // @kotowari[REQ-001]\n}\n",
    );
    let v = run_list(tmp.path());
    let tests = item(&v, "REQ-001")["tests"].as_array().unwrap().clone();
    assert_eq!(
        tests.len(),
        1,
        "the marker of a language without a query is listed: {v}"
    );
    assert_eq!(tests[0]["path"], "tests/a.go");
    assert_eq!(tests[0]["line"], 2);
    assert!(
        tests[0]["name"].is_null(),
        "no test name without a query: {tests:?}"
    );
}

// @kotowari[REQ-core-153, TBL-core-026]
#[test]
fn req_153_table_property_scenario_and_flag_carry_their_keys() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        concat!(
            "# 題名\n\n範囲。\n\n",
            "## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/records.md#A1\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n",
            "## Properties\n\n### PROP-001: 性\n\n- source: docs/decision/records/records.md#A1\n\n文である。\n\n",
            "## Examples\n\n```gherkin\n",
            "@id=EX-001 @about=TBL-001 @source=docs/decision/records/records.md#A1\n",
            "Scenario:   名前に空白がある  \n  Given 前提\n```\n",
        ),
    );
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# 題名\n\n範囲。\n\n## Flags\n\n### FLAG-001: 記録\n\n- kind: gap\n- related: TBL-001\n- source: docs/decision/records/records.md#A1\n\n本文である。\n",
    );
    let v = run_list(tmp.path());

    let common = [
        "id",
        "kind",
        "line",
        "name",
        "path",
        "sources",
        "tests",
        "fingerprint",
        "deferred",
    ];
    let with = |extra: &[&str]| {
        let mut names: Vec<String> = common
            .iter()
            .chain(extra.iter())
            .map(|s| s.to_string())
            .collect();
        names.sort();
        names
    };

    let table = item(&v, "TBL-001");
    assert_eq!(table["kind"], "table");
    assert_eq!(
        keys(table),
        with(&["examples"]),
        "a table has no type: {table}"
    );

    let property = item(&v, "PROP-001");
    assert_eq!(property["kind"], "property");
    assert_eq!(keys(property), with(&["examples"]));

    let scenario = item(&v, "EX-001");
    assert_eq!(scenario["kind"], "scenario");
    assert_eq!(
        keys(scenario),
        with(&[]),
        "a scenario has no examples: {scenario}"
    );
    // TBL-core-026: "Scenario:" の後の文字から前後の半角空白とタブを除いたもの
    assert_eq!(scenario["name"], "名前に空白がある");
    // TBL-core-026: "Scenario:" の行（26 行目のタグの行ではない）
    assert_eq!(scenario["line"], 27);

    let flag = item(&v, "FLAG-001");
    assert_eq!(flag["kind"], "flag");
    assert_eq!(keys(flag), with(&["type", "relations"]));
    assert_eq!(flag["type"], "gap");
    assert_eq!(flag["relations"], serde_json::json!(["TBL-001"]));

    // 要求の鍵は5つ多い
    write(
        tmp.path(),
        "docs/ir/b.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "例", "review")
        ),
    );
    let v = run_list(tmp.path());
    assert_eq!(
        keys(item(&v, "REQ-001")),
        with(&[
            "type",
            "verification",
            "definition",
            "examples",
            "how_to_verify"
        ])
    );
}

// @kotowari[TBL-core-026, EX-core-288]
#[test]
fn tbl_026_only_the_first_scenario_of_a_duplicated_id_counts_as_an_example() {
    // 同じ ID のシナリオが2つあると、REQ-core-032 の1つ目だけを具体例に数える
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        concat!(
            "# 題名\n\n範囲。\n\n",
            "## Requirements\n\n### REQ-001: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n\n文である。\n\n",
            "### REQ-002: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n\n文である。\n\n",
            "## Examples\n\n```gherkin\n",
            "@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\n",
            "Scenario: 1つ目\n  Given 前提\n\n",
            "@id=EX-001 @about=REQ-002 @source=docs/decision/records/records.md#A1\n",
            "Scenario: 2つ目\n  Given 前提\n```\n",
        ),
    );
    let v = run_list(tmp.path());
    assert_eq!(
        item(&v, "REQ-001")["examples"],
        serde_json::json!(["EX-001"])
    );
    assert_eq!(item(&v, "REQ-002")["examples"], serde_json::json!([]));
}

// @kotowari[REQ-core-153, TBL-core-026]
#[test]
fn req_153_examples_are_the_scenarios_about_the_item() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        concat!(
            "# 題名\n\n範囲。\n\n",
            "## Requirements\n\n### REQ-001: 例\n\n- kind: algorithm\n- source: docs/decision/records/records.md#A1\n- verification: review\n- definition: TBL-001\n\n",
            "## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/records.md#A1\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n",
            "## Properties\n\n### PROP-001: 性\n\n- source: docs/decision/records/records.md#A1\n\n文である。\n\n",
            "## Examples\n\n```gherkin\n",
            "@id=EX-002 @about=REQ-001,PROP-001 @source=docs/decision/records/records.md#A1\n",
            "Scenario: 2つ目\n  Given 前提\n\n",
            "@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\n",
            "Scenario: 1つ目\n  Given 前提\n```\n",
        ),
    );
    let v = run_list(tmp.path());
    // 自分を "@about" に挙げるシナリオの ID の並び
    assert_eq!(
        item(&v, "REQ-001")["examples"],
        serde_json::json!(["EX-001", "EX-002"])
    );
    assert_eq!(
        item(&v, "PROP-001")["examples"],
        serde_json::json!(["EX-002"])
    );
    // 誰も挙げない決定表は空の並び（鍵ごと消えない）
    assert_eq!(item(&v, "TBL-001")["examples"], serde_json::json!([]));
    assert_eq!(
        item(&v, "REQ-001")["definition"],
        serde_json::json!(["TBL-001"])
    );
}

// @kotowari[REQ-core-153, TBL-core-026, TBL-core-011]
#[test]
fn req_153_how_to_verify_is_the_value_of_the_line_or_null() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        concat!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n",
            "### REQ-001: 行がある\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n- how_to_verify: 手で動かして見る\n\n文である。\n\n",
            "### REQ-002: 行が無い\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n\n文である。\n\n",
            "### REQ-003: 値が空\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: review\n- how_to_verify:\n\n文である。\n",
        ),
    );
    let v = run_list(tmp.path());
    assert_eq!(item(&v, "REQ-001")["how_to_verify"], "手で動かして見る");
    assert!(item(&v, "REQ-002")["how_to_verify"].is_null());
    // REQ-core-098: 値が空の行は行が在るものとして扱う
    assert_eq!(
        item(&v, "REQ-003")["how_to_verify"],
        "",
        "a blank value is a line that is there: {v}"
    );
}

// @kotowari[REQ-core-153, TBL-core-026]
#[test]
fn req_153_tests_have_one_entry_per_marker_occurrence() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "例", "unit")
        ),
    );
    // 同じテストの2つの行に同じ ID の印がある
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-001]\n// @kotowari[REQ-001]\n#[test]\nfn req_001_x() {}\n",
    );
    let v = run_list(tmp.path());
    let tests = item(&v, "REQ-001")["tests"].as_array().unwrap().clone();
    assert_eq!(tests.len(), 2, "one entry per marker occurrence: {tests:?}");
    assert_eq!(tests[0]["line"], 1);
    assert_eq!(tests[1]["line"], 2);
    assert!(tests.iter().all(|t| t["name"] == "req_001_x"));
}

// --- REQ-core-154: 一覧の順 ---

// @kotowari[REQ-core-154]
#[test]
fn req_154_items_and_tests_are_ordered_by_path_then_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    // 同じ節の中で、要求の見出しの下に gherkin のブロックを置く。
    // 読み取りはシナリオを先に組み立てるので、並べ替えが無いと行の順にならない
    write(
        tmp.path(),
        "docs/ir/b.md",
        concat!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n",
            "### REQ-002: 後の要求\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n文である。\n\n",
            "```gherkin\n@id=EX-001 @about=REQ-002 @source=docs/decision/records/records.md#A1\n",
            "Scenario: 後のシナリオ\n  Given 前提\n```\n",
        ),
    );
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "先の要求", "unit")
        ),
    );
    // 印は前の兄弟を後ろから辿って集めるので、並べ替えが無いと行の順にならない
    write(
        tmp.path(),
        "tests/b.rs",
        "// @kotowari[REQ-001]\n// @kotowari[REQ-001]\n#[test]\nfn req_001_y() {}\n",
    );
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[REQ-001, EX-001]\n#[test]\nfn req_001_x() {}\n",
    );
    let v = run_list(tmp.path());

    let order: Vec<(String, u64)> = v["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| {
            (
                i["path"].as_str().unwrap().to_string(),
                i["line"].as_u64().unwrap(),
            )
        })
        .collect();
    let mut sorted = order.clone();
    sorted.sort();
    assert_eq!(
        order, sorted,
        "items are ordered by path then line: {order:?}"
    );
    assert_eq!(
        order[0].0, "docs/ir/a.md",
        "the first document comes first: {order:?}"
    );

    let tests: Vec<(String, u64)> = item(&v, "REQ-001")["tests"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| {
            (
                t["path"].as_str().unwrap().to_string(),
                t["line"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        tests,
        vec![
            ("tests/a.rs".to_string(), 1),
            ("tests/b.rs".to_string(), 1),
            ("tests/b.rs".to_string(), 2),
        ],
        "the tests are ordered by path then line"
    );
}

// --- REQ-core-155: 出力の形 ---

// @kotowari[REQ-core-155]
#[test]
fn req_155_json_top_level_has_only_items() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "例", "review")
        ),
    );
    let v = run_list(tmp.path());
    assert_eq!(
        v.as_object().unwrap().keys().collect::<Vec<_>>(),
        vec!["items"],
        "the top level is only \"items\": {v}"
    );
}

// @kotowari[REQ-core-155, EX-core-248]
#[test]
fn req_155_text_prints_one_line_per_item_and_indented_test_lines() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "例", "unit")
        ),
    );
    write(
        tmp.path(),
        "tests/a.rs",
        // 印を3行目に置く（EX-core-245 と EX-core-248 は印の行が 3）
        "\n\n// @kotowari[REQ-001]\n#[test]\nfn req_001_x() {}\n",
    );
    assert_eq!(
        run_list_text(tmp.path()),
        "REQ-001 unit 例 docs/ir/a.md:7 tests=1\n  tests/a.rs:3 req_001_x\n"
    );
}

// @kotowari[REQ-core-155]
#[test]
fn req_155_text_writes_dash_for_a_null_test_name() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.go"]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "例", "unit")
        ),
    );
    write(
        tmp.path(),
        "tests/a.go",
        "func x() {\n    // @kotowari[REQ-001]\n}\n",
    );
    assert_eq!(
        run_list_text(tmp.path()),
        "REQ-001 unit 例 docs/ir/a.md:7 tests=1\n  tests/a.go:2 -\n"
    );
}

// @kotowari[REQ-core-155]
#[test]
fn req_155_text_writes_dash_for_the_verification_of_a_non_requirement() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        concat!(
            "# 題名\n\n範囲。\n\n",
            "## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/records.md#A1\n\n| a | b |\n|---|---|\n| 1 | 2 |\n",
        ),
    );
    // 要求以外の "検証" の欄は "-"。印のあるテストが無ければ tests=0 で続く行は無い
    assert_eq!(
        run_list_text(tmp.path()),
        "TBL-001 - 表 docs/ir/a.md:7 tests=0\n"
    );
}

// @kotowari[REQ-core-155]
#[test]
fn req_155_text_writes_dash_for_a_requirement_without_a_verification_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(
        tmp.path(),
        "docs/ir/a.md",
        concat!(
            "# 題名\n\n範囲。\n\n",
            "## Requirements\n\n### REQ-001: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n\n文。\n",
        ),
    );
    // "- verification:" の行の無い要求の "検証" の欄も "-"
    assert_eq!(
        run_list_text(tmp.path()),
        "REQ-001 - 例 docs/ir/a.md:7 tests=0\n"
    );
}
