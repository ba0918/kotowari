//! "kotowari status" の集計と出力（REQ-core-162〜REQ-core-166、TBL-core-028）
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

/// コマンドを走らせて (終了コード, 標準出力, 標準エラー) を返す
fn run(tmp: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
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

/// "kotowari status" を走らせ、(終了コード, JSON) を返す
fn run_status(tmp: &Path) -> (Option<i32>, serde_json::Value) {
    let (code, stdout, stderr) = run(tmp, &["status"]);
    assert_eq!(stderr, "", "status writes nothing to stderr");
    (code, serde_json::from_str(&stdout).expect("valid JSON"))
}

/// 確認することの行が2つある決定の文書と、その2つを指す印のあるテスト
fn write_ex_258_project(tmp: &Path) {
    make_project(tmp);
    write(
        tmp,
        "docs/decisions/a/a.md",
        "# 題名\n\n範囲。\n\n## 一\n\n状態: 実装済み\n\n### 問題\n\n困っていた。\n\n\
### 決めたこと\n\nこうする。\n\n### 確認すること\n\n- C-a-a-1 一つ目\n- C-a-a-2 二つ目\n",
    );
    write(
        tmp,
        "tests/a.rs",
        "// @kotowari[C-a-a-1, C-a-a-2]\n#[test]\nfn req_001_x() {}\n",
    );
}

// --- REQ-core-162、REQ-core-165: 集計と complete ---

// @kotowari[C-core-status-5, C-core-status-6, C-core-status-9]
#[test]
fn req_162_complete_project_counts_and_exit_zero() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    let (code, v) = run_status(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["tests"]["marks"], 2);
    assert_eq!(v["findings"]["error"], 0);
    assert_eq!(v["findings"]["notice"], 0);
    assert_eq!(v["complete"], true);
}

// --- TBL-core-028: 数え方 ---

// @kotowari[C-core-status-5, C-core-status-9]
#[test]
fn tbl_028_notices_are_counted_apart_from_errors() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    // 指紋の古いガイドの印だけを置き、注意（guide_stale）だけが出る置き場にする
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\nguides:\n  files:\n    - \"guide/*.md\"\n",
    )
    .unwrap();
    write(
        tmp.path(),
        "guide/a.md",
        "# 使い方\n\n<!-- @kotowari[docs/decisions/a/a.md#一:00000000] -->\n",
    );
    let (code, v) = run_status(tmp.path());
    assert_eq!(v["findings"]["error"], 0, "{v}");
    assert_eq!(v["findings"]["notice"], 1, "{v}");
    // 注意は complete を妨げない
    assert_eq!(v["complete"], true, "{v}");
    assert_eq!(code, Some(0), "{v}");
}

// @kotowari[C-core-status-6]
#[test]
fn tbl_028_test_files_match_check() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    let (_, status) = run_status(tmp.path());
    let (_, stdout, _) = run(tmp.path(), &["check"]);
    let check: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(status["tests"]["files"], check["tests"]);
}

// @kotowari[C-core-status-10]
#[test]
fn the_documents_group_counts_the_decision_documents_read_and_their_lines() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    // 2つ目の決定の文書は5行。用語集は話題の文書でないので数えない
    write(
        tmp.path(),
        "docs/decisions/a/b.md",
        "# 題名\n\n範囲。\n\n二行目の段落。\n",
    );
    write(
        tmp.path(),
        "docs/decisions/a/CONTEXT.md",
        "# 用語集\n\n語。\n",
    );
    let first = std::fs::read_to_string(tmp.path().join("docs/decisions/a/a.md")).unwrap();
    let (_, v) = run_status(tmp.path());
    assert_eq!(
        v["documents"],
        serde_json::json!({"files": 2, "lines": first.lines().count() + 5}),
        "{v}"
    );
}

// --- REQ-core-166: 出力の形 ---

// @kotowari[C-core-status-12]
#[test]
fn req_166_json_top_level_has_only_the_groups() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    let (_, stdout, _) = run(tmp.path(), &["status"]);
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let mut names: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(
        names,
        vec![
            "complete",
            "confirmations",
            "decisions",
            "documents",
            "findings",
            "guides",
            "overview",
            "surface",
            "tests",
        ],
    );
    // 群の順は status の表の順で、"complete" が最後
    let order: Vec<usize> = [
        "\"documents\"",
        "\"tests\"",
        "\"guides\"",
        "\"surface\"",
        "\"findings\"",
        "\"complete\"",
    ]
    .iter()
    .map(|key| {
        stdout
            .find(key)
            .unwrap_or_else(|| panic!("no {key} in {stdout}"))
    })
    .collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{stdout}");
}

// @kotowari[C-core-status-1]
#[test]
fn req_162_status_writes_no_finding() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 問題の節の無い決定（check なら誤り）
    write(
        tmp.path(),
        "docs/decisions/a/a.md",
        "# 題名\n\n範囲。\n\n## 一\n\n状態: 実装済み\n\n### 決めたこと\n\nこうする。\n",
    );
    let (code, stdout, stderr) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(1));
    assert_eq!(stderr, "", "status writes no finding to stderr");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let mut names: Vec<&str> = v["findings"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    names.sort_unstable();
    assert_eq!(names, vec!["error", "notice"]);
    for key in ["\"kind\"", "\"detail\"", "\"severity\""] {
        assert!(
            !stdout.contains(key),
            "status writes no finding to stdout: {stdout}"
        );
    }
}

// @kotowari[C-core-status-6, C-core-status-11]
#[test]
fn req_166_text_prints_one_line_per_group() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    let (code, stdout, stderr) = run(tmp.path(), &["status", "--format", "text"]);
    assert_eq!(code, Some(0), "{stderr}");
    let lines: Vec<&str> = stdout.lines().collect();
    let line = |group: &str| {
        lines
            .iter()
            .find(|line| line.split(' ').next() == Some(group))
            .copied()
            .unwrap_or_else(|| panic!("no {group} line: {stdout}"))
    };
    assert!(
        line("documents").starts_with("documents files="),
        "got: {:?}",
        line("documents")
    );
    assert_eq!(line("tests"), "tests marks=2 rs=1");
    assert_eq!(line("guides"), "guides files=0 marks=0");
    assert_eq!(line("overview"), "overview files=0 marks=0");
    assert_eq!(line("surface"), "surface total=0 specified=0 unspecified=0");
    assert_eq!(line("findings"), "findings error=0 notice=0");
    assert_eq!(lines.last().copied(), Some("complete true"));
}
