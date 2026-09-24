//! "kotowari plan" が計画書を1つ読み、同梱のスキーマで形を検査すること（docs/ir/core/plan.md）

use assert_cmd::Command;
use kotowari_core::{Cli, parse_args};
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

/// 標準エラーの1行目
fn first_stderr_line(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr)
        .lines()
        .next()
        .unwrap_or("")
        .to_string()
}

/// 引数の誤りで停止することを見る
fn assert_argument_error(list: &[&str], dir: &Path) {
    let output = cmd().args(list).current_dir(dir).output().unwrap();
    assert_eq!(output.status.code(), Some(2), "{list:?} should stop");
    let first_line = first_stderr_line(&output);
    assert!(
        first_line.starts_with("argument error: "),
        "{list:?}: got {first_line:?}"
    );
    // plan はコマンドなので、知らないコマンドとしての停止ではない（REQ-core-001）
    assert!(
        !first_line.starts_with("argument error: unknown command"),
        "{list:?}: plan should be a command: {first_line:?}"
    );
}

// --- REQ-core-190: plan の引数 ---

// @kotowari[REQ-core-190, EX-core-357]
#[test]
fn ex_core_357_plan_without_a_plan_file_stops() {
    let tmp = TempDir::new().unwrap();
    assert_argument_error(&["plan"], tmp.path());
}

// @kotowari[REQ-core-190, EX-core-342]
#[test]
fn ex_core_342_two_plan_files_stop() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    std::fs::write(tmp.path().join("b.md"), "# b\n").unwrap();
    assert_argument_error(&["plan", "a.md", "b.md"], tmp.path());
}

// @kotowari[REQ-core-190, EX-core-343]
#[test]
fn ex_core_343_config_on_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: docs/ir\n").unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    assert_argument_error(
        &["plan", "--config", ".kotowari/config.yaml", "a.md"],
        tmp.path(),
    );
}

// @kotowari[REQ-core-190]
#[test]
fn req_core_190_config_pointing_to_a_directory_on_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    assert_argument_error(&["plan", "--config", ".kotowari", "a.md"], tmp.path());
}

// @kotowari[REQ-core-002]
#[test]
fn req_core_002_plan_takes_the_format_before_or_after_the_path() {
    for list in [
        ["plan", "--format", "text", "a.md"],
        ["plan", "a.md", "--format", "text"],
        ["--format", "text", "plan", "a.md"],
    ] {
        let parsed = parse_args(&args(&list));
        assert!(
            matches!(parsed, Ok(Cli::Plan { ref path, .. }) if path == Path::new("a.md")),
            "{list:?}: {parsed:?}"
        );
    }
}

// @kotowari[REQ-core-004]
#[test]
fn req_core_004_tool_on_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.md"), "# a\n").unwrap();
    assert_argument_error(&["plan", "--tool", "cargo-mutants", "a.md"], tmp.path());
}

// --- REQ-core-191〜REQ-core-197: 計画書を読み、指摘を invalid_plan で出す ---

/// 形の揃った計画書
const VALID_PLAN: &str = include_str!("../fixtures/plans/valid.md");

/// 欄 "Done when" の行が無いステップを持つ計画書
fn plan_without_done_when() -> String {
    let plan: String = VALID_PLAN
        .lines()
        .filter(|line| !line.starts_with("- Done when:"))
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(plan, VALID_PLAN, "the fixture should have a Done when line");
    plan
}

/// "## Out of scope" の前に中身の無い節 "## Notes" を足した計画書と、その見出しの行
fn plan_with_unknown_section() -> (String, usize) {
    let plan = VALID_PLAN.replace("## Out of scope", "## Notes\n\n## Out of scope");
    let line = plan.lines().position(|l| l == "## Notes").unwrap() + 1;
    (plan, line)
}

/// 基準のディレクトリ（`.kotowari/` の無い一時ディレクトリ）の "docs/plans/a.md" に計画書を置く
fn project_with_plan(content: &str) -> TempDir {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/plans")).unwrap();
    std::fs::write(tmp.path().join("docs/plans/a.md"), content).unwrap();
    tmp
}

/// "kotowari plan" を実行し、終了コードと標準出力を返す
fn run_plan(dir: &Path, list: &[&str]) -> (Option<i32>, String) {
    let output = cmd().args(list).current_dir(dir).output().unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
    )
}

/// JSON の出力の "findings"
fn findings_of(stdout: &str) -> Vec<serde_json::Value> {
    let json: serde_json::Value = serde_json::from_str(stdout).unwrap();
    json["findings"].as_array().unwrap().clone()
}

// @kotowari[REQ-core-192, REQ-core-193, EX-core-333]
#[test]
fn ex_core_333_a_step_missing_a_field_is_an_invalid_plan_error() {
    let tmp = project_with_plan(&plan_without_done_when());
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert!(!findings.is_empty());
    for f in &findings {
        assert_eq!(f["kind"], "invalid_plan", "{f}");
        assert_eq!(f["severity"], "error", "{f}");
        assert_eq!(f["path"], "docs/plans/a.md", "{f}");
        // detail はスキーマの側の種類に ": " と詳細が続く
        let detail = f["detail"].as_str().unwrap();
        let (kind, rest) = detail
            .split_once(": ")
            .unwrap_or_else(|| panic!("{detail:?}"));
        assert!(
            !kind.is_empty() && kind.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
            "{detail:?}"
        );
        assert!(!rest.is_empty(), "{detail:?}");
    }
}

// @kotowari[REQ-core-192, REQ-core-193]
#[test]
fn req_core_192_a_valid_plan_has_no_findings() {
    let tmp = project_with_plan(VALID_PLAN);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
    assert!(findings_of(&stdout).is_empty(), "{stdout}");
}

// @kotowari[REQ-core-193]
#[test]
fn req_core_193_a_finding_without_a_line_has_a_null_line() {
    // 題名が無い計画書。題名の欠けはスキーマの側が行を持たずに出す
    let plan: String = VALID_PLAN
        .lines()
        .filter(|line| !line.starts_with("# "))
        .map(|line| format!("{line}\n"))
        .collect();
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert!(
        findings
            .iter()
            .any(|f| f["kind"] == "invalid_plan" && f["line"].is_null()),
        "{stdout}"
    );
}

// @kotowari[REQ-core-193]
#[test]
fn req_core_193_the_line_is_the_line_the_schema_side_reported() {
    // 宣言していない節を、スキーマの側はその見出しの行で指す
    let (plan, line) = plan_with_unknown_section();
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert_eq!(findings.len(), 1, "{stdout}");
    assert_eq!(findings[0]["line"], line, "{stdout}");
}

// @kotowari[REQ-core-025, REQ-core-193]
#[test]
fn req_core_025_plan_text_output_has_one_line_per_finding() {
    let (plan, line) = plan_with_unknown_section();
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md", "--format", "text"]);
    assert_eq!(code, Some(1), "{stdout}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "{stdout}");
    let prefix = format!("docs/plans/a.md:{line} [error] invalid_plan ");
    assert!(lines[0].starts_with(&prefix), "{stdout}");
}

// @kotowari[REQ-core-197, EX-core-344]
#[test]
fn ex_core_344_a_missing_plan_stops_as_an_unreadable_file() {
    let tmp = TempDir::new().unwrap();
    let output = cmd()
        .args(["plan", "docs/plans/none.md"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let first_line = first_stderr_line(&output);
    assert!(
        first_line.starts_with("unreadable file: "),
        "{first_line:?}"
    );
}

// @kotowari[REQ-core-197]
#[test]
fn req_core_197_a_directory_given_as_the_plan_stops_as_an_unreadable_file() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/plans")).unwrap();
    let output = cmd()
        .args(["plan", "docs/plans"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let first_line = first_stderr_line(&output);
    assert!(
        first_line.starts_with("unreadable file: "),
        "{first_line:?}"
    );
}

// @kotowari[REQ-core-197, EX-core-358]
#[test]
fn ex_core_358_a_non_utf8_plan_stops() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/plans")).unwrap();
    let mut bytes = VALID_PLAN.as_bytes().to_vec();
    bytes.extend_from_slice(&[0xff, 0xfe, b'\n']);
    std::fs::write(tmp.path().join("docs/plans/a.md"), bytes).unwrap();
    let output = cmd()
        .args(["plan", "docs/plans/a.md"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let first_line = first_stderr_line(&output);
    assert!(first_line.starts_with("non-UTF-8 file: "), "{first_line:?}");
}

// @kotowari[REQ-core-191, EX-core-345]
#[test]
fn ex_core_345_a_schema_named_in_the_frontmatter_is_not_used() {
    let plan = format!("---\n$schema: missing.yaml\n---\n{VALID_PLAN}");
    let tmp = project_with_plan(&plan);
    assert!(!tmp.path().join("docs/plans/missing.yaml").exists());
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
}

// @kotowari[REQ-core-191, EX-core-361]
#[test]
fn ex_core_361_a_broken_frontmatter_is_skipped_unread() {
    let plan = format!("---\n: : [unclosed\n---\n{VALID_PLAN}");
    let tmp = project_with_plan(&plan);
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
}

// @kotowari[REQ-core-194, TBL-core-005, EX-core-346]
#[test]
fn ex_core_346_plan_json_has_only_findings_and_counts() {
    let tmp = project_with_plan(&plan_without_done_when());
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md", "--format", "json"]);
    assert_eq!(code, Some(1), "{stdout}");
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let mut keys: Vec<&String> = json.as_object().unwrap().keys().collect();
    keys.sort();
    assert_eq!(keys, ["counts", "findings"], "{stdout}");
    let invalid = json["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == "invalid_plan")
        .count();
    assert!(invalid > 0);
    assert_eq!(json["counts"]["invalid_plan"], invalid, "{stdout}");
}

// @kotowari[REQ-core-194, TBL-core-006]
#[test]
fn req_core_194_plan_finding_keys_are_those_of_check() {
    let tmp = project_with_plan(&plan_without_done_when());
    let (_, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    for f in findings_of(&stdout) {
        let mut keys: Vec<&String> = f.as_object().unwrap().keys().collect();
        keys.sort();
        assert_eq!(keys, ["detail", "kind", "line", "path", "severity"], "{f}");
    }
}

// @kotowari[REQ-core-196, EX-core-359]
#[test]
fn ex_core_359_a_broken_config_does_not_stop_plan() {
    let tmp = project_with_plan(VALID_PLAN);
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/config.yaml"), "ir: [unclosed\n").unwrap();
    let (code, stdout) = run_plan(tmp.path(), &["plan", "docs/plans/a.md"]);
    assert_eq!(code, Some(0), "{stdout}");
}

// @kotowari[REQ-core-193, TBL-core-006, EX-core-360]
#[test]
fn ex_core_360_a_plan_outside_the_base_has_a_parent_path() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("work/.kotowari")).unwrap();
    std::fs::create_dir_all(tmp.path().join("plans")).unwrap();
    std::fs::write(tmp.path().join("plans/a.md"), plan_without_done_when()).unwrap();
    let (code, stdout) = run_plan(&tmp.path().join("work"), &["plan", "../plans/a.md"]);
    assert_eq!(code, Some(1), "{stdout}");
    let findings = findings_of(&stdout);
    assert!(!findings.is_empty());
    for f in &findings {
        assert_eq!(f["path"], "../plans/a.md", "{f}");
    }
}

// @kotowari[REQ-core-193, TBL-core-006]
#[test]
fn req_core_193_the_path_is_normalised_from_the_base() {
    // カレントディレクトリが基準の下でも、"./" を付けても、path は基準からの相対に正規化する
    let tmp = project_with_plan(&plan_without_done_when());
    std::fs::create_dir_all(tmp.path().join(".kotowari")).unwrap();
    let (_, stdout) = run_plan(&tmp.path().join("docs"), &["plan", "./plans/../plans/a.md"]);
    let findings = findings_of(&stdout);
    assert!(!findings.is_empty(), "{stdout}");
    for f in &findings {
        assert_eq!(f["path"], "docs/plans/a.md", "{f}");
    }
}

// --- REQ-core-192: 計画書の形 ---

/// 計画書を "docs/plans/a.md" に置いて "kotowari plan docs/plans/a.md" を実行し、終了コードと標準出力を返す
fn plan_outcome(plan: &str) -> (Option<i32>, String) {
    let tmp = project_with_plan(plan);
    run_plan(tmp.path(), &["plan", "docs/plans/a.md"])
}

/// 終了コードが1で、invalid_plan の誤りが出ることを見る
fn assert_invalid_plan(plan: &str) {
    let (code, stdout) = plan_outcome(plan);
    assert_eq!(code, Some(1), "{plan}\n{stdout}");
    assert!(
        findings_of(&stdout)
            .iter()
            .any(|f| f["kind"] == "invalid_plan" && f["severity"] == "error"),
        "{stdout}"
    );
}

/// 終了コードが0で、指摘が出ないことを見る
fn assert_valid_plan(plan: &str) {
    let (code, stdout) = plan_outcome(plan);
    assert_eq!(code, Some(0), "{plan}\n{stdout}");
    assert!(findings_of(&stdout).is_empty(), "{stdout}");
}

/// `from` をちょうど1か所だけ `to` に置き換える
fn replace_once(plan: &str, from: &str, to: &str) -> String {
    assert_eq!(plan.matches(from).count(), 1, "{from:?} should appear once");
    plan.replacen(from, to, 1)
}

/// ステップ "### S1: 入力を読む" の見出しと8つの欄
const STEP_S1: &str = "### S1: 入力を読む

- Purpose: 入力のファイルを読む
- Specification: `docs/ir/core/a.md#REQ-core-001`
- Prerequisites: なし
- May change: `src/lib.rs`
- Done when: 入力のファイルを読んだ結果が返る
- Shown by: test — REQ-core-001
- Left to the implementer: なし
- Stop and hand back if: 入力の形が仕様から決まらないとき
";

// @kotowari[REQ-core-192, REQ-core-193, EX-core-332]
#[test]
fn ex_core_332_a_well_formed_plan_has_no_findings() {
    assert!(VALID_PLAN.contains(STEP_S1));
    assert_valid_plan(VALID_PLAN);
}

// @kotowari[REQ-core-192, EX-core-334]
#[test]
fn ex_core_334_shown_by_not_starting_with_one_of_the_four_words_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "- Shown by: test — REQ-core-001",
        "- Shown by: manual — 目で見る",
    ));
}

// @kotowari[REQ-core-192, EX-core-335]
#[test]
fn ex_core_335_shown_by_word_followed_by_more_letters_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "- Shown by: test — REQ-core-001",
        "- Shown by: tests — 名前",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_each_of_the_four_words_may_start_shown_by_alone_or_before_a_space() {
    for value in [
        "test",
        "check — 3つのコマンド",
        "artifact — README.md",
        "external\t実機",
    ] {
        assert_valid_plan(&replace_once(
            VALID_PLAN,
            "- Shown by: test — REQ-core-001",
            &format!("- Shown by: {value}"),
        ));
    }
}

// @kotowari[REQ-core-192, EX-core-336]
#[test]
fn ex_core_336_a_plan_without_a_step_is_an_error() {
    assert_invalid_plan(&replace_once(VALID_PLAN, STEP_S1, ""));
}

// @kotowari[REQ-core-192, EX-core-337]
#[test]
fn ex_core_337_the_test_command_section_may_be_absent() {
    assert_valid_plan(&replace_once(
        VALID_PLAN,
        "## Test command\n\n`cargo test`\n\n",
        "",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_two_test_command_sections_are_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "## Out of scope",
        "## Test command\n\n`cargo nextest run`\n\n## Out of scope",
    ));
}

// @kotowari[REQ-core-192, EX-core-338]
#[test]
fn ex_core_338_two_goal_sections_are_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "## Out of scope",
        "## Goal\n\nもう1つのゴール。\n\n## Out of scope",
    ));
}

// @kotowari[REQ-core-192, EX-core-339]
#[test]
fn ex_core_339_a_plan_wide_section_may_hold_a_table() {
    assert_valid_plan(&replace_once(
        VALID_PLAN,
        "S1 が REQ-core-001 を確かめる。\n",
        "| ステップ | 確かめる要求 |\n|---|---|\n| S1 | REQ-core-001 |\n",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_a_plan_wide_section_may_hold_a_code_block() {
    assert_valid_plan(&replace_once(
        VALID_PLAN,
        "`cargo test`\n",
        "```sh\nCARGO_BUILD_JOBS=4 cargo test\n1. これはコードの中の行\n```\n",
    ));
}

// @kotowari[REQ-core-192, EX-core-340]
#[test]
fn ex_core_340_prose_right_after_a_field_line_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "- Done when: 入力のファイルを読んだ結果が返る\n",
        "- Done when: 入力のファイルを読んだ結果が返る\n続けて書いた地の文\n",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_an_indented_line_after_a_blank_line_under_a_step_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "- Stop and hand back if: 入力の形が仕様から決まらないとき\n",
        "- Stop and hand back if: 入力の形が仕様から決まらないとき\n\n    字下げした続き\n",
    ));
}

// @kotowari[REQ-core-192, EX-core-341]
#[test]
fn ex_core_341_section_order_and_gaps_in_step_numbers_are_not_checked() {
    // "## Goal" を "## Out of scope" の後へ動かし、ステップを S1 と S3 にする
    let goal = "## Goal\n\n利用者が入力のファイルを渡すと、結果が標準出力に出る。\n\n";
    let moved = replace_once(VALID_PLAN, goal, "");
    let moved = replace_once(&moved, "## Steps", &format!("{goal}## Steps"));
    let s3 = STEP_S1.replace("### S1: 入力を読む", "### S3: 結果を出す");
    let plan = format!("{moved}\n{s3}");
    assert_valid_plan(&plan);
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_two_steps_with_the_same_number_are_not_checked() {
    assert_valid_plan(&format!("{VALID_PLAN}\n{STEP_S1}"));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_the_name_after_the_colon_of_a_step_heading_is_not_checked() {
    assert_valid_plan(&replace_once(VALID_PLAN, "### S1: 入力を読む", "### S12:"));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_a_step_heading_without_a_colon_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "### S1: 入力を読む",
        "### S1 入力を読む",
    ));
}

// @kotowari[REQ-core-192, EX-core-347]
#[test]
fn ex_core_347_a_missing_required_section_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "## Stop conditions\n\n- 入力の形が仕様から決まらないとき\n\n",
        "",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_every_required_section_is_required() {
    for name in [
        "Goal",
        "Specification",
        "Approach and why",
        "Scope of change",
        "Step order and prerequisites",
        "Verification map",
        "Left to the implementer",
        "Stop conditions",
        "Out of scope",
        "Steps",
    ] {
        // 見出しだけを別の名前の節に変えると、その節が欠けて知らない節が1つ増える。
        // 欠けたことだけを見るため、見出しを消して中身を前の節に寄せる
        let heading = format!("## {name}\n");
        let plan = replace_once(VALID_PLAN, &heading, "");
        let (code, stdout) = plan_outcome(&plan);
        assert_eq!(code, Some(1), "without {name}: {stdout}");
        assert!(
            findings_of(&stdout)
                .iter()
                .any(|f| f["line"].is_null() && f["detail"].as_str().unwrap().contains(name)),
            "without {name}: {stdout}"
        );
    }
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_two_titles_are_an_error() {
    assert_invalid_plan(&format!("{VALID_PLAN}\n# もう1つの題名\n"));
}

// @kotowari[REQ-core-192, EX-core-348]
#[test]
fn ex_core_348_an_unknown_section_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "## Out of scope",
        "## Notes\n\nメモ。\n\n## Out of scope",
    ));
}

// @kotowari[REQ-core-192, EX-core-349]
#[test]
fn ex_core_349_fields_out_of_order_are_an_error() {
    let plan = replace_once(VALID_PLAN, "- Purpose: 入力のファイルを読む\n", "");
    let plan = replace_once(
        &plan,
        "- Done when: 入力のファイルを読んだ結果が返る\n",
        "- Done when: 入力のファイルを読んだ結果が返る\n- Purpose: 入力のファイルを読む\n",
    );
    // Done when の行が Purpose の行より前にある
    assert!(plan.find("- Done when:").unwrap() < plan.find("- Purpose:").unwrap());
    assert_invalid_plan(&plan);
}

// @kotowari[REQ-core-192, EX-core-350]
#[test]
fn ex_core_350_a_field_twice_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "- Purpose: 入力のファイルを読む\n",
        "- Purpose: 入力のファイルを読む\n- Purpose: もう1つの目的\n",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_every_step_field_is_required() {
    for name in [
        "Purpose",
        "Specification",
        "Prerequisites",
        "May change",
        "Done when",
        "Shown by",
        "Left to the implementer",
        "Stop and hand back if",
    ] {
        let prefix = format!("- {name}: ");
        let plan: String = VALID_PLAN
            .lines()
            .filter(|line| !line.starts_with(&prefix))
            .map(|line| format!("{line}\n"))
            .collect();
        assert_ne!(plan, VALID_PLAN, "{name}");
        let (code, stdout) = plan_outcome(&plan);
        assert_eq!(code, Some(1), "without {name}: {stdout}");
    }
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_an_unknown_field_under_a_step_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "- Stop and hand back if: 入力の形が仕様から決まらないとき\n",
        "- Stop and hand back if: 入力の形が仕様から決まらないとき\n- Notes: メモ\n",
    ));
}

// @kotowari[REQ-core-192, EX-core-351]
#[test]
fn ex_core_351_a_line_before_the_first_step_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "## Steps\n\n",
        "## Steps\n\nステップの前の地の文\n\n",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_a_list_before_the_first_step_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "## Steps\n\n",
        "## Steps\n\n- 前置き\n\n",
    ));
}

// @kotowari[REQ-core-192, EX-core-352]
#[test]
fn ex_core_352_a_step_heading_not_of_s_and_a_number_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "### S1: 入力を読む",
        "### Step 1: 入力を読む",
    ));
}

// @kotowari[REQ-core-192, EX-core-353]
#[test]
fn ex_core_353_a_numbered_list_in_a_plan_wide_section_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "S1 だけ。\n",
        "S1 だけ。\n\n1. S1 を先に行う\n",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_a_deeper_heading_in_a_plan_wide_section_is_an_error() {
    for heading in ["### 小見出し", "#### 深い見出し"] {
        assert_invalid_plan(&replace_once(
            VALID_PLAN,
            "関数の名前。\n",
            &format!("関数の名前。\n\n{heading}\n"),
        ));
    }
}

// @kotowari[REQ-core-192, EX-core-354]
#[test]
fn ex_core_354_a_plan_wide_section_may_hold_a_child_list() {
    assert_valid_plan(&replace_once(
        VALID_PLAN,
        "- `src/lib.rs`\n",
        "- `src/lib.rs`\n  - 引数の解析\n  - 実行\n",
    ));
}

// @kotowari[REQ-core-192, EX-core-355]
#[test]
fn ex_core_355_a_line_between_the_title_and_the_first_section_is_an_error() {
    assert_invalid_plan(&replace_once(
        VALID_PLAN,
        "# 計画: 入力を読んで結果を出す\n",
        "# 計画: 入力を読んで結果を出す\n\n要約の地の文\n",
    ));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_a_line_before_the_title_is_not_checked() {
    assert_valid_plan(&format!("前の行\n\n{VALID_PLAN}"));
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_a_line_before_the_title_does_not_hide_a_line_after_the_title() {
    // 題名は3行目、題名と最初の節の間の地の文は5行目
    let plan = format!(
        "前の行\n\n{}",
        replace_once(
            VALID_PLAN,
            "# 計画: 入力を読んで結果を出す\n",
            "# 計画: 入力を読んで結果を出す\n\n要約の地の文\n",
        )
    );
    let (code, stdout) = plan_outcome(&plan);
    assert_eq!(code, Some(1), "{stdout}");
    let lines: Vec<_> = findings_of(&stdout)
        .iter()
        .map(|f| f["line"].clone())
        .collect();
    assert_eq!(lines, vec![serde_json::json!(5)], "{stdout}");
}

// @kotowari[REQ-core-192, EX-core-356]
#[test]
fn ex_core_356_field_lines_may_use_an_asterisk() {
    let plan = STEP_S1
        .lines()
        .filter(|line| line.starts_with("- "))
        .fold(VALID_PLAN.to_string(), |plan, line| {
            plan.replace(line, &format!("*{}", &line[1..]))
        });
    assert!(!plan.contains("\n- Purpose:") && plan.contains("\n* Stop and hand back if:"));
    assert_valid_plan(&plan);
}

// @kotowari[REQ-core-192]
#[test]
fn req_core_192_field_lines_may_use_a_plus() {
    let plan = STEP_S1
        .lines()
        .filter(|line| line.starts_with("- "))
        .fold(VALID_PLAN.to_string(), |plan, line| {
            plan.replace(line, &format!("+{}", &line[1..]))
        });
    assert!(plan.contains("\n+ Purpose:"));
    assert_valid_plan(&plan);
}
