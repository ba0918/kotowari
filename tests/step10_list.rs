//! "kotowari list" の項目の組み立てと出力
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る。`test_globs` が空でなければ "tests.files" に書く
fn make_project(tmp: &Path, test_globs: &[&str]) {
    for dir in [".kotowari", "docs/decisions", "tests"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    let mut config = "decisions:\n  documents: docs/decisions\n".to_string();
    if !test_globs.is_empty() {
        config.push_str("tests:\n  files:\n");
        for glob in test_globs {
            config.push_str(&format!("    - {glob}\n"));
        }
    }
    std::fs::write(tmp.join(".kotowari/config.yaml"), config).unwrap();
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

/// "items" から ID で1件を引く
fn item<'a>(v: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    v["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["id"] == id)
        .unwrap_or_else(|| panic!("no item {id} in {v}"))
}

/// 確認することの行 C-t-a-1 を持つ決定を1つ置いた決定の文書
const DECISION_DOC: &str = "# 題名\n\n範囲。\n\n## 例\n\n状態: 実装済み\n\n### 問題\n\n困っていた。\n\n\
### 決めたこと\n\nこうする。\n\n### 確認すること\n\n- C-t-a-1 確かめる\n";

// --- list の読み取り ---

// --- 項目の形 ---

// @kotowari[C-core-list-4]
#[test]
fn test_in_a_language_without_a_query_has_a_null_name() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.go"]);
    write(tmp.path(), "docs/decisions/t/a.md", DECISION_DOC);
    write(
        tmp.path(),
        "tests/a.go",
        "func x() {\n    // @kotowari[C-t-a-1]\n}\n",
    );
    let v = run_list(tmp.path());
    let tests = item(&v, "C-t-a-1")["tests"].as_array().unwrap().clone();
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

// --- 一覧の順 ---

// --- 出力の形 ---

// @kotowari[C-core-list-8]
#[test]
fn json_top_level_has_only_items() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    let v = run_list(tmp.path());
    assert_eq!(
        v.as_object().unwrap().keys().collect::<Vec<_>>(),
        vec!["items"],
        "the top level is only \"items\": {v}"
    );
}

// --- 対の blob hash ---

/// "languages: [ja, en]" と日本語の`UI の文字`のすべての鍵を書いた設定に、extra を足す
fn make_bilingual_project(tmp: &Path, extra: &str) {
    make_project(tmp, &[]);
    let mut config = std::fs::read_to_string(tmp.join(".kotowari/config.yaml")).unwrap();
    config.push_str("languages: [ja, en]\nlabels:\n  ja:\n");
    for key in [
        "language_name",
        "index_link",
        "stale_mark",
        "outline_stale",
        "superseded",
        "deferred",
        "compare_before",
        "compare_after",
        "compare_why",
        "state_decided",
        "state_planned",
        "state_open",
        "state_dropped",
    ] {
        config.push_str(&format!("    {key}: \"{key}\"\n"));
    }
    for key in ["pages", "stale_sections", "open_items", "planned_items"] {
        config.push_str(&format!("    {key}: \"{key} {{n}}\"\n"));
    }
    config.push_str(extra);
    std::fs::write(tmp.join(".kotowari/config.yaml"), config).unwrap();
}

fn blob(text: &str) -> String {
    kotowari_core::translations::blob_hash(text.as_bytes())
}

// @kotowari[C-core-list-10]
#[test]
fn translations_list_every_pair_with_the_blob_hash_of_each_side() {
    let tmp = TempDir::new().unwrap();
    make_bilingual_project(
        tmp.path(),
        "guides:\n  files:\n    - \"guides/*.md\"\noverview:\n  files:\n    - \".kotowari/overview/*.md\"\n  toc: .kotowari/toc.yaml\n",
    );
    let doc = DECISION_DOC.replace("\n\n### 確認すること\n\n- C-t-a-1 確かめる", "");
    write(tmp.path(), "docs/decisions/t/a.md", &doc);
    write(tmp.path(), "docs/decisions/t/a.en.md", &doc);
    write(tmp.path(), "docs/decisions/t/b.en.md", "x\n");
    write(tmp.path(), "guides/g.md", "a\n");
    write(tmp.path(), ".kotowari/overview/o.md", "# O\n");
    write(tmp.path(), ".kotowari/toc.yaml", "title: T\n");
    let v = run_list(tmp.path());
    let side = |language: &str, path: &str, blob: Option<String>| serde_json::json!({"language": language, "path": path, "blob": blob});
    let entry = |path: &str, sides: Vec<serde_json::Value>| serde_json::json!({"path": path, "sides": sides});
    assert_eq!(
        v["translations"],
        serde_json::json!([
            entry(
                ".kotowari/overview/o.md",
                vec![
                    side("ja", ".kotowari/overview/o.md", Some(blob("# O\n"))),
                    side("en", ".kotowari/overview/o.en.md", None),
                ]
            ),
            entry(
                ".kotowari/toc.yaml",
                vec![
                    side("ja", ".kotowari/toc.yaml", Some(blob("title: T\n"))),
                    side("en", ".kotowari/toc.en.yaml", None),
                ]
            ),
            entry(
                "docs/decisions/t/a.md",
                vec![
                    side("ja", "docs/decisions/t/a.md", Some(blob(&doc))),
                    side("en", "docs/decisions/t/a.en.md", Some(blob(&doc))),
                ]
            ),
            entry(
                "docs/decisions/t/b.md",
                vec![
                    side("ja", "docs/decisions/t/b.md", None),
                    side("en", "docs/decisions/t/b.en.md", Some(blob("x\n"))),
                ]
            ),
            entry(
                "guides/g.md",
                vec![
                    side("ja", "guides/g.md", Some(blob("a\n"))),
                    side("en", "guides/g.en.md", None),
                ]
            ),
        ])
    );
    // "items" は先頭の言語の側だけ
    assert_eq!(v["items"].as_array().unwrap().len(), 1);
    let (code, text, _) = run_list_raw(tmp.path(), &["--format", "text"]);
    assert_eq!(code, Some(0));
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        &lines[lines.len() - 5..],
        [
            format!(".kotowari/overview/o.md ja={} en=-", blob("# O\n")),
            format!(".kotowari/toc.yaml ja={} en=-", blob("title: T\n")),
            format!("docs/decisions/t/a.md ja={} en={}", blob(&doc), blob(&doc)),
            format!("docs/decisions/t/b.md ja=- en={}", blob("x\n")),
            format!("guides/g.md ja={} en=-", blob("a\n")),
        ]
    );
}

// @kotowari[C-core-list-11, C-core-list-15]
#[test]
fn with_two_languages_list_stops_on_the_guide_places_as_check_does() {
    let overlap =
        "tests:\n  files:\n    - \"guides/*.md\"\nguides:\n  files:\n    - \"guides/*.md\"\n";
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    write(tmp.path(), "guides/g.md", "a\n");
    let mut config = std::fs::read_to_string(tmp.path().join(".kotowari/config.yaml")).unwrap();
    config.push_str(overlap);
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), &config).unwrap();
    // 言語が1つなら list はガイドを読まない
    assert_eq!(run_list_raw(tmp.path(), &[]).0, Some(0));
    let tmp = TempDir::new().unwrap();
    make_bilingual_project(tmp.path(), overlap);
    write(tmp.path(), "guides/g.md", "a\n");
    let (code, stdout, stderr) = run_list_raw(tmp.path(), &[]);
    assert_eq!(code, Some(2), "{stdout}");
    let (_, _, check) = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .arg("check")
        .current_dir(tmp.path())
        .output()
        .map(|output| {
            (
                output.status.code(),
                (),
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
        })
        .unwrap();
    assert_eq!(stderr, check);
}

// @kotowari[C-core-list-3, C-core-list-7, C-core-list-9]
#[test]
fn tests_are_one_entry_per_marker_occurrence_in_path_and_line_order_and_a_null_name_is_a_dash() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.rs", "tests/**/*.go"]);
    write(tmp.path(), "docs/decisions/t/a.md", DECISION_DOC);
    write(
        tmp.path(),
        "tests/b.rs",
        "// @kotowari[C-t-a-1, C-t-a-1]\n#[test]\nfn b() {}\n",
    );
    write(
        tmp.path(),
        "tests/a.go",
        "// @kotowari[C-t-a-1]\nfunc A() {}\n",
    );
    let v = run_list(tmp.path());
    let tests = item(&v, "C-t-a-1")["tests"].clone();
    assert_eq!(
        tests,
        serde_json::json!([
            {"path": "tests/a.go", "line": 1, "name": null},
            {"path": "tests/b.rs", "line": 1, "name": "b"},
            {"path": "tests/b.rs", "line": 1, "name": "b"},
        ]),
        "{v}"
    );
    let (code, text, stderr) = run_list_raw(tmp.path(), &["--format", "text"]);
    assert_eq!(code, Some(0), "{stderr}");
    let lines: Vec<&str> = text.lines().collect();
    let row = lines
        .iter()
        .position(|line| line.starts_with("C-t-a-1 "))
        .unwrap();
    assert!(lines[row].ends_with(" tests=3"), "{text}");
    assert_eq!(
        &lines[row + 1..row + 4],
        ["  tests/a.go:1 -", "  tests/b.rs:1 b", "  tests/b.rs:1 b"],
        "{text}"
    );
}
