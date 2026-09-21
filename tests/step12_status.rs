//! "kotowari status" の集計と出力（REQ-core-162〜REQ-core-166、TBL-core-028）

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
    format!("### {id}: {name}\n\n- 種類: ubiquitous\n- 出典: docs/decision/records/records.md#A1\n{fields}\n文である。\n\n")
}

/// EX-core-258 と EX-core-261 が使う文書とテスト。
/// 検証が unit の要求、確かめ方のある review の要求、その具体例、印のあるテストがある
fn write_ex_258_project(tmp: &Path) {
    make_project(tmp);
    write(
        tmp,
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## 要求\n\n{}{}## 具体例\n\n\
```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\nScenario: 例\n  Given 何か\n```\n",
            requirement("REQ-001", "一", "- 検証: unit\n"),
            requirement("REQ-002", "二", "- 検証: review\n- 確かめ方: 人が読む\n"),
        ),
    );
    write(
        tmp,
        "tests/a.rs",
        "// @kotowari[REQ-001, EX-001]\n#[test]\nfn req_001_x() {}\n",
    );
}

// --- REQ-core-162、REQ-core-165: 集計と complete ---

// @kotowari[REQ-core-162, REQ-core-165, TBL-core-028, EX-core-258]
#[test]
fn req_162_complete_project_counts_and_exit_zero() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    let (code, v) = run_status(tmp.path());
    assert_eq!(code, Some(0), "{v}");
    assert_eq!(v["requirements"]["unit"], 1);
    assert_eq!(v["requirements"]["review"], 1);
    assert_eq!(v["requirements"]["with_tests"], 1);
    assert_eq!(v["requirements"]["without_tests"], 0);
    assert_eq!(v["requirements"]["review_with_how_to_verify"], 1);
    assert_eq!(v["requirements"]["review_without_how_to_verify"], 0);
    assert_eq!(v["requirements"]["without_examples"], 1);
    assert_eq!(v["scenarios"]["with_tests"], 1);
    assert_eq!(v["scenarios"]["without_tests"], 0);
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
            "# 題名\n\n範囲。\n\n## 要求\n\n{}",
            requirement("REQ-002", "二", "- 検証: review\n")
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
            "# 題名\n\n範囲。\n\n## 要求\n\n{}",
            requirement("REQ-001", "一", "- 検証: review\n- 確かめ方: 人が読む\n")
        ),
    );
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# 問題の記録\n\nなし。\n\n### FLAG-001: 抜け\n\n- 種類: gap\n- 関係: REQ-001\n- 出典: docs/decision/records/records.md#A1\n\n本文。\n",
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
            "# 題名\n\n範囲。\n\n## 要求\n\n{}## 具体例\n\n\
```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\nScenario: 例\n  Given 何か\n```\n",
            requirement("REQ-001", "一", "- 検証: unit\n")
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
            "# 題名\n\n範囲。\n\n## 要求\n\n{}",
            requirement("REQ-001", "一", "- 検証: review\n- 確かめ方: 人が読む\n")
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
        &format!("# 題名\n\n範囲。\n\n## 要求\n\n{}", requirement("REQ-001", "一", "")),
    );
    let (_, v) = run_status(tmp.path());
    // "- 検証:" の行の無い要求は検証の値のどれにも数えない
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
            "# 題名\n\n範囲。\n\n## 要求\n\n{}## 決定表\n\n\
### TBL-001: 表\n\n- 出典: docs/decision/records/records.md#A1\n\n| A |\n|---|\n| 1 |\n\n\
### TBL-002: もう1つの表\n\n- 出典: docs/decision/records/records.md#A1\n\n| A |\n|---|\n| 1 |\n\n\
## 性質\n\n### PROP-001: 性\n\n- 出典: docs/decision/records/records.md#A1\n\n文である。\n\n## 具体例\n\n\
```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1\nScenario: 例\n  Given 何か\n```\n",
            requirement("REQ-001", "一", "- 検証: review\n- 確かめ方: 人が読む\n")
        ),
    );
    write(
        tmp.path(),
        "docs/ir/FLAGS.md",
        "# 問題の記録\n\nなし。\n\n### FLAG-001: 抜け\n\n- 種類: gap\n- 関係: REQ-001\n- 出典: docs/decision/records/records.md#A1\n\n本文。\n",
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
            "# 題名\n\n範囲。\n\n## 要求\n\n{}{}{}{}",
            requirement("REQ-001", "一", "- 検証: unit\n"),
            requirement("REQ-002", "二", "- 検証: property\n"),
            requirement("REQ-003", "三", "- 検証: proof\n"),
            requirement("REQ-004", "四", "- 検証: review\n- 確かめ方: 人が読む\n"),
        ),
    );
    let (_, v) = run_status(tmp.path());
    for key in ["unit", "property", "proof", "review"] {
        assert_eq!(v["requirements"][key], 1, "{key}: {v}");
    }
    // review の3件以外は分母に入り、印が無いのでテストの無い側に数える
    assert_eq!(v["requirements"]["without_tests"], 3, "{v}");
}

// @kotowari[TBL-core-028, REQ-core-165]
#[test]
fn tbl_028_notices_are_counted_apart_from_errors() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // 行数の上限を1にして、注意（too_many_lines）だけが出る置き場にする
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "ir: docs/ir\ndecisions:\n  records: docs/decision/records\n  adr: docs/decision/adr\nlimits:\n  lines: 1\n",
    )
    .unwrap();
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!(
            "# 題名\n\n範囲。\n\n## 要求\n\n{}",
            requirement("REQ-001", "一", "- 検証: review\n- 確かめ方: 人が読む\n")
        ),
    );
    let (code, v) = run_status(tmp.path());
    assert_eq!(v["findings"]["error"], 0, "{v}");
    assert_eq!(v["findings"]["notice"], 1, "{v}");
    // 注意は complete を妨げない（REQ-core-165 は誤りと問題の記録だけを見る）
    assert_eq!(v["complete"], true, "{v}");
    assert_eq!(code, Some(0), "{v}");
}

// @kotowari[TBL-core-028, TBL-core-021]
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

// @kotowari[REQ-core-164, REQ-core-166, TBL-core-028]
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
            "documents",
            "findings",
            "items",
            "requirements",
            "scenarios",
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
        "\"findings\"",
        "\"complete\"",
    ]
    .iter()
    .map(|key| stdout.find(key).unwrap_or_else(|| panic!("no {key} in {stdout}")))
    .collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{stdout}");
}

// @kotowari[REQ-core-162]
#[test]
fn req_162_status_writes_no_finding() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    // "- 検証:" の行の無い要求（check なら verification_missing）
    write(
        tmp.path(),
        "docs/ir/a.md",
        &format!("# 題名\n\n範囲。\n\n## 要求\n\n{}", requirement("REQ-001", "一", "")),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(1));
    assert_eq!(stderr, "", "status writes no finding to stderr");
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let mut names: Vec<&str> = v["findings"].as_object().unwrap().keys().map(String::as_str).collect();
    names.sort_unstable();
    assert_eq!(names, vec!["error", "notice"]);
    for key in ["\"kind\"", "\"detail\"", "\"severity\""] {
        assert!(!stdout.contains(key), "status writes no finding to stdout: {stdout}");
    }
}

// @kotowari[REQ-core-166, TBL-core-028, EX-core-261]
#[test]
fn req_166_text_prints_one_line_per_group() {
    let tmp = TempDir::new().unwrap();
    write_ex_258_project(tmp.path());
    let (code, stdout, stderr) = run(tmp.path(), &["status", "--format", "text"]);
    assert_eq!(code, Some(0), "{stderr}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 7, "one line per group: {stdout}");
    assert!(
        lines[0].starts_with("documents files=1 lines="),
        "got: {:?}",
        lines[0]
    );
    assert_eq!(lines[1], "items requirement=2 table=0 property=0 scenario=1 flag=0");
    assert_eq!(
        lines[2],
        "requirements unit=1 property=0 proof=0 review=1 with_tests=1 without_tests=0 review_with_how_to_verify=1 review_without_how_to_verify=0 without_examples=1"
    );
    assert_eq!(lines[3], "scenarios with_tests=1 without_tests=0");
    assert_eq!(lines[4], "tests marks=2 rs=1");
    assert_eq!(lines[5], "findings error=0 notice=0");
    assert_eq!(lines[6], "complete true");
}
