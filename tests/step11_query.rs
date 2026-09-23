//! "kotowari query" の1件の組み立てと出力（REQ-core-156〜REQ-core-161、TBL-core-027）

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る
fn make_project(tmp: &Path) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "tests",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n",
    )
    .unwrap();
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

/// "kotowari query" を走らせて (終了コード, 標準出力, 標準エラー) を返す
fn run_query_raw(tmp: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("query")
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

/// "kotowari query <id>" を走らせ、終了コード 0 を確かめて JSON を返す
fn run_query(tmp: &Path, id: &str) -> serde_json::Value {
    let (code, stdout, stderr) = run_query_raw(tmp, &[id]);
    assert_eq!(code, Some(0), "query should exit 0: {stderr}");
    serde_json::from_str(&stdout).expect("valid JSON")
}

/// "items" がちょうど1件であることを確かめ、その1件を返す
fn only_item(v: &serde_json::Value) -> &serde_json::Value {
    let items = v["items"].as_array().unwrap();
    assert_eq!(items.len(), 1, "one item: {v}");
    &items[0]
}

/// "referenced_by" の1件を "ID 種類 via パス:行" の形の文字にする
fn references(item: &serde_json::Value) -> Vec<String> {
    item["referenced_by"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| {
            let mut names: Vec<&str> = r.as_object().unwrap().keys().map(String::as_str).collect();
            names.sort_unstable();
            // TBL-core-027: 逆引きの1件は id、kind、path、line、via を持つ
            assert_eq!(names, vec!["id", "kind", "line", "path", "via"], "{r}");
            format!(
                "{} {} {} {}:{}",
                r["id"].as_str().unwrap(),
                r["kind"].as_str().unwrap(),
                r["via"].as_str().unwrap(),
                r["path"].as_str().unwrap(),
                r["line"]
            )
        })
        .collect()
}

/// "body" の行を並べる
fn body(item: &serde_json::Value) -> Vec<String> {
    item["body"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l.as_str().unwrap().to_string())
        .collect()
}

/// 要求の見出しと行を作る
fn requirement(id: &str, name: &str, verification: &str) -> String {
    format!(
        "### {id}: {name}\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: {verification}\n\n文である。\n\n"
    )
}

/// EX-core-250、EX-core-254、EX-core-255 が使う文書。
/// "REQ-001" の見出しが 7 行目、"@id" のタグが 19 行目、"Scenario:" が 20 行目、
/// ステップが 21 行目から 23 行目にある
const EX_250_DOCUMENT: &str = "\
# 題名

範囲。

## Requirements

### REQ-001: 例

- kind: ubiquitous
- source: docs/decision/records/records.md#A1
- verification: unit

文。


## Examples

```gherkin
@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1
Scenario: 例のシナリオ
  Given 何か
  When 何かする
  Then こうなる
```
";

// --- REQ-core-156: query の読み取り ---

// @kotowari[TBL-core-027, EX-core-287]
#[test]
fn tbl_027_an_item_citing_an_id_twice_is_one_reference() {
    // 1つの項目が同じ ID を同じ行の種類で2回指しても、逆引きは1件
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let doc = format!(
        "# 題名\n\n範囲。\n\n## Requirements\n\n### REQ-001: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n`REQ-002` と `REQ-002` を使う。\n\n{}",
        requirement("REQ-002", "指される", "unit")
    );
    write(tmp.path(), "docs/ir/a.md", &doc);
    let v = run_query(tmp.path(), "REQ-002");
    assert_eq!(
        references(only_item(&v)),
        vec!["REQ-001 requirement text docs/ir/a.md:7".to_string()]
    );
}

// @kotowari[REQ-core-156, REQ-core-159, TBL-core-027, EX-core-250]
#[test]
fn req_156_item_has_body_and_referenced_by() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", EX_250_DOCUMENT);
    let v = run_query(tmp.path(), "REQ-001");
    let req = only_item(&v);
    assert_eq!(req["id"], "REQ-001");
    assert_eq!(req["path"], "docs/ir/a.md");
    assert_eq!(req["line"], 7);
    assert_eq!(
        body(req),
        vec![
            "- kind: ubiquitous",
            "- source: docs/decision/records/records.md#A1",
            "- verification: unit",
            "",
            "文。",
        ],
    );
    assert_eq!(references(req), vec!["EX-001 scenario about docs/ir/a.md:20"]);
    // TBL-core-027: list の1件の鍵に "body" と "referenced_by" が増えた形
    let mut names: Vec<&str> = req.as_object().unwrap().keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        vec![
            "body",
            "definition",
            "examples",
            "how_to_verify",
            "id",
            "kind",
            "line",
            "name",
            "path",
            "referenced_by",
            "sources",
            "tests",
            "type",
            "verification",
        ],
    );
}

// @kotowari[REQ-core-156, EX-core-253]
#[test]
fn req_156_duplicate_ids_are_all_listed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    for name in ["a.md", "b.md"] {
        write(
            tmp.path(),
            &format!("docs/ir/{name}"),
            &format!("# 題名\n\n範囲。\n\n## Requirements\n\n{}", requirement("REQ-001", "例", "unit")),
        );
    }
    let v = run_query(tmp.path(), "REQ-001");
    let paths: Vec<&str> = v["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["path"].as_str().unwrap())
        .collect();
    assert_eq!(paths, vec!["docs/ir/a.md", "docs/ir/b.md"]);
}

// @kotowari[REQ-core-156]
#[test]
fn req_156_items_are_listed_despite_ir_errors_and_exit_zero() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // "- verification:" の行の無い要求（check なら verification_missing）
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n### REQ-002: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n\n文である。\n",
    );
    let (code, stdout, stderr) = run_query_raw(tmp.path(), &["REQ-002"]);
    assert_eq!(code, Some(0), "an IR error does not change the exit code: {stderr}");
    assert_eq!(stderr, "", "query writes no finding to stderr");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert!(only_item(&v)["verification"].is_null());
    assert!(v.get("findings").is_none(), "query writes no finding to stdout: {v}");
}

// --- REQ-core-157: 無い ID ---

// @kotowari[REQ-core-157, TBL-core-020, EX-core-251]
#[test]
fn req_157_unknown_id_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!("# 題名\n\n範囲。\n\n## Requirements\n\n{}", requirement("REQ-001", "例", "unit")),
    );
    let (code, stdout, stderr) = run_query_raw(tmp.path(), &["REQ-999"]);
    assert_eq!(code, Some(2));
    assert_eq!(stdout, "");
    assert_eq!(
        stderr.lines().next().unwrap_or(""),
        "argument error: unknown id: REQ-999"
    );
}

// --- TBL-core-027: body と referenced_by ---

// @kotowari[TBL-core-027, EX-core-255]
#[test]
fn tbl_027_scenario_body_starts_at_the_tag_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", EX_250_DOCUMENT);
    let v = run_query(tmp.path(), "EX-001");
    let scenario = only_item(&v);
    assert_eq!(scenario["line"], 20);
    // 19 行目のタグの行から 23 行目の最後のステップまで
    assert_eq!(
        body(scenario),
        vec![
            "@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1",
            "Scenario: 例のシナリオ",
            "  Given 何か",
            "  When 何かする",
            "  Then こうなる",
        ],
    );
    assert!(references(scenario).is_empty(), "{scenario}");
}

// @kotowari[TBL-core-027, REQ-core-054, EX-core-257]
#[test]
fn tbl_027_definition_and_text_references_are_listed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n\
### REQ-001: 定義する側\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n- definition: TBL-001\n\n文である。\n\n\
### REQ-002: 文で指す側\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n`TBL-001` を使う。\n\n\
## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/records.md#A1\n\n| A |\n|---|\n| 1 |\n",
    );
    let v = run_query(tmp.path(), "TBL-001");
    assert_eq!(
        references(only_item(&v)),
        vec![
            "REQ-001 requirement definition docs/ir/a.md:7",
            "REQ-002 requirement text docs/ir/a.md:16",
        ],
    );
}

// @kotowari[TBL-core-027, REQ-core-054]
#[test]
fn tbl_027_text_reference_is_only_the_backticked_id() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n\
### REQ-001: 地の文で書く側\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\nTBL-001 をバッククォート無しで書く。\n\n\
### REQ-002: 引用符の中で書く側\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n\"`TBL-001`\" と引用符の中に書く。\n\n\
## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/records.md#A1\n\n| A |\n|---|\n| 1 |\n",
    );
    let v = run_query(tmp.path(), "TBL-001");
    assert!(references(only_item(&v)).is_empty(), "{v}");
}

// @kotowari[TBL-core-027]
#[test]
fn tbl_027_body_stops_before_the_next_heading() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 次の見出しが直後にある要求と、本文が1行だけの要求
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n### REQ-001: 本文の無い要求\n### REQ-002: 本文が1行の要求\n- kind: ubiquitous\n## Examples\n",
    );
    assert!(body(only_item(&run_query(tmp.path(), "REQ-001"))).is_empty());
    assert_eq!(
        body(only_item(&run_query(tmp.path(), "REQ-002"))),
        vec!["- kind: ubiquitous"],
    );
}

// --- REQ-core-160: 逆引きの並び ---

// @kotowari[REQ-core-160]
#[test]
fn req_160_referenced_by_is_ordered_by_path_then_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 後から読む文書が先に参照を持ち、同じ文書の中は行の小さい方が先になる
    write(
        tmp.path(),
        "docs/ir/b.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n\
### REQ-003: 三\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n- definition: TBL-001\n\n文である。\n",
    );
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n\
### REQ-002: 二\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n- definition: TBL-001\n\n文である。\n\n\
### REQ-001: 一\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n- definition: TBL-001\n\n文である。\n\n\
## Decision tables\n\n### TBL-001: 表\n\n- source: docs/decision/records/records.md#A1\n\n| A |\n|---|\n| 1 |\n",
    );
    let v = run_query(tmp.path(), "TBL-001");
    assert_eq!(
        references(only_item(&v)),
        vec![
            "REQ-002 requirement definition docs/ir/a.md:7",
            "REQ-001 requirement definition docs/ir/a.md:16",
            "REQ-003 requirement definition docs/ir/b.md:7",
        ],
    );
}

// --- REQ-core-161: 出力の形 ---

// @kotowari[REQ-core-161]
#[test]
fn req_161_json_top_level_has_only_items() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", EX_250_DOCUMENT);
    let v = run_query(tmp.path(), "REQ-001");
    let names: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(names, vec!["items"]);
}

// @kotowari[REQ-core-161, REQ-core-155, EX-core-254]
#[test]
fn req_161_text_prints_body_and_referenced_by_lines() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(tmp.path(), "docs/ir/a.md", EX_250_DOCUMENT);
    // 印を3行目に置く（EX-core-254 は印の行が 3）
    write(
        tmp.path(),
        "tests/a.rs",
        "\n\n// @kotowari[REQ-001]\n#[test]\nfn req_001_x() {}\n",
    );
    let (code, stdout, stderr) = run_query_raw(tmp.path(), &["--format", "text", "REQ-001"]);
    assert_eq!(code, Some(0), "query should exit 0: {stderr}");
    assert_eq!(
        stdout,
        "REQ-001 unit 例 docs/ir/a.md:7 tests=1\n\
         \x20 tests/a.rs:3 req_001_x\n\
         \x20 - kind: ubiquitous\n\
         \x20 - source: docs/decision/records/records.md#A1\n\
         \x20 - verification: unit\n\
         \x20 \n\
         \x20 文。\n\
         \x20 <- EX-001 about docs/ir/a.md:20\n",
    );
}

// @kotowari[EX-core-277]
#[test]
fn ex_core_277_body_ends_before_the_next_heading_and_drops_trailing_blank_lines() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}{}## Properties\n\n### PROP-001: 性質\n\n- source: docs/decision/records/records.md#A1\n\n性質の文。\n\n",
            requirement("REQ-001", "一", "unit"),
            requirement("REQ-002", "二", "unit"),
        ),
    );
    let fields = vec![
        "- kind: ubiquitous",
        "- source: docs/decision/records/records.md#A1",
        "- verification: unit",
        "",
        "文である。",
    ];
    assert_eq!(body(only_item(&run_query(tmp.path(), "REQ-001"))), fields);
    assert_eq!(body(only_item(&run_query(tmp.path(), "REQ-002"))), fields);
    assert_eq!(
        body(only_item(&run_query(tmp.path(), "PROP-001"))),
        vec!["- source: docs/decision/records/records.md#A1", "", "性質の文。"],
    );
}

// @kotowari[EX-core-280]
#[test]
fn ex_core_280_a_heading_shaped_line_inside_a_code_block_does_not_end_the_body() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n### REQ-001: 一\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n文である。\n\n```\n## 例\n```\n\n### REQ-002: 二\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n文である。\n",
    );
    let body = body(only_item(&run_query(tmp.path(), "REQ-001")));
    assert_eq!(body.last().map(String::as_str), Some("```"), "{body:?}");
    assert!(body.iter().any(|line| line == "## 例"), "{body:?}");
}
