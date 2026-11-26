//! "kotowari status" の集計と出力（REQ-core-162〜REQ-core-166、TBL-core-028）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 置き場と設定を作る
fn make_project(tmp: &Path) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "docs/decisions",
        "tests",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n  documents: docs/decisions\n",
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

/// 要求の見出しと行を作る
fn requirement(id: &str, name: &str, fields: &str) -> String {
    format!(
        "### {id}: {name}\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n{fields}\n文である。\n\n"
    )
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

// @kotowari[REQ-core-162, REQ-core-165, TBL-core-028, EX-core-258, C-core-status-5, C-core-status-6, C-core-status-9]
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

// @kotowari[REQ-core-165, REQ-core-098, EX-core-259]
#[test]
fn req_165_review_requirement_without_how_to_verify_is_not_complete() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-002", "二", "- verification: review\n")
        ),
    );
    let (code, v) = run_status(tmp.path());
    assert_eq!(code, Some(1), "{v}");
    assert!(v["findings"]["error"].as_u64().unwrap() >= 1, "{v}");
    assert_eq!(v["requirements"]["review_without_how_to_verify"], 1);
    assert_eq!(v["complete"], false);
}

// @kotowari[REQ-core-165, EX-core-260]
#[test]
fn req_165_flag_makes_it_not_complete() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement(
                "REQ-001",
                "一",
                "- verification: review\n- how_to_verify: 人が読む\n"
            )
        ),
    );
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# 問題の記録\n\nなし。\n\n## Flags\n\n### FLAG-001: 抜け\n\n- kind: gap\n- related: REQ-001\n- source: docs/decision/records/records.md#A1\n\n本文。\n",
    );
    let (code, v) = run_status(tmp.path());
    assert_eq!(v["findings"]["error"], 0, "check reports nothing here: {v}");
    assert_eq!(v["items"]["flag"], 1);
    assert_eq!(v["complete"], false);
    assert_eq!(code, Some(1), "{v}");
}

// --- TBL-core-028: 数え方 ---

// @kotowari[TBL-core-028, REQ-core-085, EX-core-263]
#[test]
fn tbl_028_requirement_covered_through_a_scenario_counts_as_with_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}## Examples\n\n\
```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\nScenario: 例\n  Given 何か\n```\n",
            requirement("REQ-001", "一", "- verification: unit\n")
        ),
    );
    // 具体例の ID だけの印
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[EX-001]\n#[test]\nfn ex_001_x() {}\n",
    );
    let (_, v) = run_status(tmp.path());
    assert_eq!(v["requirements"]["with_tests"], 1, "{v}");
    assert_eq!(v["requirements"]["without_tests"], 0, "{v}");
}

// @kotowari[TBL-core-028]
#[test]
fn tbl_028_review_requirements_are_not_in_with_or_without_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement(
                "REQ-001",
                "一",
                "- verification: review\n- how_to_verify: 人が読む\n"
            )
        ),
    );
    let (_, v) = run_status(tmp.path());
    assert_eq!(v["requirements"]["review"], 1);
    assert_eq!(v["requirements"]["with_tests"], 0, "{v}");
    assert_eq!(v["requirements"]["without_tests"], 0, "{v}");
}

// @kotowari[TBL-core-028]
#[test]
fn tbl_028_requirement_without_a_verification_line_is_counted_by_marks_only() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}",
            requirement("REQ-001", "一", "")
        ),
    );
    let (_, v) = run_status(tmp.path());
    // "- verification:" の行の無い要求は検証の値のどれにも数えない
    for key in ["unit", "property", "proof", "review"] {
        assert_eq!(v["requirements"][key], 0, "{key}: {v}");
    }
    // review でないので、印の有無でどちらかに数える
    assert_eq!(v["requirements"]["with_tests"], 0, "{v}");
    assert_eq!(v["requirements"]["without_tests"], 1, "{v}");
}

// @kotowari[TBL-core-028]
#[test]
fn tbl_028_items_are_counted_by_kind() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}## Decision tables\n\n\
### TBL-001: 表\n\n- source: docs/decision/records/records.md#A1\n\n| A |\n|---|\n| 1 |\n\n\
### TBL-002: もう1つの表\n\n- source: docs/decision/records/records.md#A1\n\n| A |\n|---|\n| 1 |\n\n\
## Properties\n\n### PROP-001: 性\n\n- source: docs/decision/records/records.md#A1\n\n文である。\n\n## Examples\n\n\
```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\nScenario: 例\n  Given 何か\n```\n",
            requirement(
                "REQ-001",
                "一",
                "- verification: review\n- how_to_verify: 人が読む\n"
            )
        ),
    );
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# 問題の記録\n\nなし。\n\n## Flags\n\n### FLAG-001: 抜け\n\n- kind: gap\n- related: REQ-001\n- source: docs/decision/records/records.md#A1\n\n本文。\n",
    );
    let (_, v) = run_status(tmp.path());
    // 決定表だけ2つにして、種類ごとの数が入れ替わらないことも見る
    for (key, count) in [
        ("requirement", 1),
        ("table", 2),
        ("property", 1),
        ("scenario", 1),
        ("flag", 1),
    ] {
        assert_eq!(v["items"][key], count, "{key}: {v}");
    }
    // 印の無い具体例はテストの無い側に数える
    assert_eq!(v["scenarios"]["with_tests"], 0, "{v}");
    assert_eq!(v["scenarios"]["without_tests"], 1, "{v}");
}

// @kotowari[TBL-core-028]
#[test]
fn tbl_028_requirements_are_counted_by_verification_value() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## Requirements\n\n{}{}{}{}",
            requirement("REQ-001", "一", "- verification: unit\n"),
            requirement("REQ-002", "二", "- verification: property\n"),
            requirement("REQ-003", "三", "- verification: proof\n"),
            requirement(
                "REQ-004",
                "四",
                "- verification: review\n- how_to_verify: 人が読む\n"
            ),
        ),
    );
    let (_, v) = run_status(tmp.path());
    for key in ["unit", "property", "proof", "review"] {
        assert_eq!(v["requirements"][key], 1, "{key}: {v}");
    }
    // review の3件以外は分母に入り、印が無いのでテストの無い側に数える
    assert_eq!(v["requirements"]["without_tests"], 3, "{v}");
}

// @kotowari[TBL-core-028, REQ-core-165, C-core-status-5, C-core-status-9]
#[test]
fn tbl_028_notices_are_counted_apart_from_errors() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    // 指紋の古いガイドの印だけを置き、注意（guide_stale）だけが出る置き場にする
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\n  documents: docs/decisions\nguides:\n  files:\n    - \"guide/*.md\"\n",
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

// @kotowari[TBL-core-028, TBL-core-021, C-core-status-6]
#[test]
fn tbl_028_documents_and_test_files_match_check() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    let (_, status) = run_status(tmp.path());
    let (_, stdout, _) = run(tmp.path(), &["check"]);
    let check: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    assert_eq!(status["documents"]["files"], check["files"]);
    assert_eq!(status["documents"]["lines"], check["lines"]);
    assert_eq!(status["tests"]["files"], check["tests"]);
}

// --- REQ-core-166: 出力の形 ---

// @kotowari[REQ-core-164, REQ-core-166, TBL-core-028, C-core-status-12]
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
            "items",
            "overview",
            "requirements",
            "scenarios",
            "surface",
            "tests",
        ],
    );
    // 群の順は TBL-core-028 の表の順で、"complete" が最後
    let order: Vec<usize> = [
        "\"documents\"",
        "\"items\"",
        "\"requirements\"",
        "\"scenarios\"",
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

// @kotowari[REQ-core-162, C-core-status-1]
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

// @kotowari[REQ-core-166, TBL-core-028, EX-core-261, C-core-status-6, C-core-status-11]
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
