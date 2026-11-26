//! "kotowari query" の1件の組み立てと出力
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る
fn make_project(tmp: &Path) {
    for dir in [".kotowari", "docs/decisions", "tests"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\n",
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

// --- query の読み取り ---

// @kotowari[C-core-query-3]
#[test]
fn req_156_duplicate_ids_are_all_listed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    for name in ["a.md", "b.md"] {
        write(
            tmp.path(),
            &format!("docs/decisions/{name}"),
            "# 題名\n\n範囲。\n\n## 例\n\n状態: 実装済み\n\n### 問題\n\n困っていた。\n\n\
### 決めたこと\n\nこうする。\n\n### 確認すること\n\n- C-t-1 確かめる\n",
        );
    }
    let v = run_query(tmp.path(), "C-t-1");
    let paths: Vec<&str> = v["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["path"].as_str().unwrap())
        .collect();
    assert_eq!(paths, vec!["docs/decisions/a.md", "docs/decisions/b.md"]);
}

// --- 無い ID ---

// @kotowari[C-core-clienv-7, C-core-query-7]
#[test]
fn req_157_unknown_id_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let (code, stdout, stderr) = run_query_raw(tmp.path(), &["C-t-999"]);
    assert_eq!(code, Some(2));
    assert_eq!(stdout, "");
    assert_eq!(
        stderr.lines().next().unwrap_or(""),
        "argument error: unknown id: C-t-999"
    );
}

// --- body と referenced_by ---

// --- 逆引きの並び ---

// --- 出力の形 ---

// @kotowari[C-core-query-11]
#[test]
fn req_161_json_top_level_has_only_items() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/a.md",
        "# 題名\n\n範囲。\n\n## 例\n\n状態: 実装済み\n\n### 問題\n\n困っていた。\n\n### 決めたこと\n\nこうする。\n",
    );
    let v = run_query(tmp.path(), "docs/decisions/a.md#例");
    let names: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
    assert_eq!(names, vec!["items"]);
}
