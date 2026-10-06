//! 全体像の元データの読み込みと検査を check、status、list、query から見る
//! （docs/ir/core/overview-data.md、docs/ir/core/overview-output.md）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

const IR: &str = "# CLI\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: read\n\nBody.\n";

/// 誤りの無い IR と判断の記録と、`config` を設定に持つプロジェクト
fn make_project(tmp: &Path, config: &str) {
    for dir in [".kotowari", "docs/decision/adr", "tests"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    write(tmp, "docs/ir/cli.md", IR);
    write(
        tmp,
        "docs/decision/records/r.md",
        "# R\n\n## Context\n\nc\n\n## Agreements\n\n- A1 決めた\n  - why: w\n",
    );
    write(
        tmp,
        ".kotowari/config.yaml",
        &format!("tests:\n  files:\n    - \"tests/**/*.rs\"\n{config}"),
    );
}

fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

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

fn json(stdout: &str) -> serde_json::Value {
    serde_json::from_str(stdout).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

const OVERVIEW: &str =
    "overview:\n  files:\n    - \".kotowari/overview/*.md\"\n  toc: .kotowari/toc.yaml\n";

/// 正しい全体像の元データ
fn valid(title: &str) -> String {
    format!(
        "---\nir:\n  - docs/ir/cli.md\n---\n\n# {title}\n\n```view lead\nconclusion: 結論\n```\n\n## 節\n\n文。\n"
    )
}

/// 冒頭が lead でない全体像の元データ
fn without_lead(ir: &str) -> String {
    format!("---\nir:\n  - {ir}\n---\n\n# 題名\n\n## 節\n\n文。\n")
}

fn kinds_on(value: &serde_json::Value, path: &str) -> Vec<String> {
    value["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["path"] == path)
        .map(|finding| finding["kind"].as_str().unwrap().to_string())
        .collect()
}

// @kotowari[EX-core-464, REQ-core-278, REQ-core-019, REQ-core-288]
#[test]
fn ex_core_464_overview_data_in_a_named_hidden_directory_is_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(
        tmp.path(),
        ".kotowari/overview/mutants.md",
        &valid("変異テスト"),
    );
    write_toc(tmp.path(), &["mutants"]);
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "json"]);
    assert_eq!(code, Some(0), "{stderr}{stdout}");
    let value = json(&stdout);
    assert_eq!(
        value["overview"],
        serde_json::json!({"files": 1, "marks": 0})
    );
}

// @kotowari[REQ-core-019, REQ-core-278]
#[test]
fn req_core_019_a_broad_glob_does_not_enter_hidden_directories_it_does_not_name() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files:\n    - \"**/*.md\"\n  toc: .kotowari/toc.yaml\n",
    );
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &without_lead("docs/ir/cli.md"),
    );
    write_toc(tmp.path(), &["cli", "r"]);
    let (_, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    let value = json(&stdout);
    assert!(kinds_on(&value, ".kotowari/overview/a.md").is_empty());
    // 当たるのは docs/ の下の IR と判断の記録の2つだけ
    assert_eq!(value["overview"]["files"], 2);

    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files:\n    - \".kotowari/overview/**/*.md\"\n  toc: .kotowari/toc.yaml\n",
    );
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    write(tmp.path(), ".kotowari/overview/sub/b.md", &valid("b"));
    write(
        tmp.path(),
        ".kotowari/overview/.hidden/c.md",
        &without_lead("docs/ir/cli.md"),
    );
    write_toc(tmp.path(), &["a", "b"]);
    let (_, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    let value = json(&stdout);
    assert!(kinds_on(&value, ".kotowari/overview/.hidden/c.md").is_empty());
    assert_eq!(value["overview"]["files"], 2);
}

// @kotowari[REQ-core-019]
#[test]
fn req_core_019_a_brace_across_a_slash_in_overview_files_does_not_stop_the_check() {
    // 波括弧の中の "/" で切った前置きは glob にならない。設定の glob は有効なので、
    // 検査は止まらずに元データを読む
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files:\n    - \"{docs/.overview,docs/overview}/*.md\"\n  toc: .kotowari/toc.yaml\n",
    );
    write(tmp.path(), "docs/overview/b.md", &valid("b"));
    write_toc(tmp.path(), &["b"]);
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "json"]);
    assert_eq!(code, Some(0), "{stderr}{stdout}");
    assert_eq!(json(&stdout)["overview"]["files"], 1);
}

// @kotowari[EX-core-545, REQ-core-019]
#[test]
fn ex_core_545_a_hidden_directory_named_only_inside_braces_is_not_entered() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files:\n    - \"{.overview,other}/*.md\"\n  toc: .kotowari/toc.yaml\n",
    );
    write(
        tmp.path(),
        ".overview/a.md",
        &without_lead("docs/ir/cli.md"),
    );
    // 同じ中身の元データは、波括弧の中の隠しディレクトリでない置き場では読まれて誤りになる
    write(tmp.path(), "other/b.md", &without_lead("docs/ir/cli.md"));
    write_toc(tmp.path(), &["b"]);
    let (_, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    let value = json(&stdout);
    assert!(!kinds_on(&value, "other/b.md").is_empty(), "{stdout}");
    assert!(kinds_on(&value, ".overview/a.md").is_empty(), "{stdout}");
    assert_eq!(value["overview"]["files"], 1);
}

// @kotowari[REQ-core-278]
#[test]
fn req_core_278_only_files_with_a_lowercase_md_extension_are_read() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files:\n    - \"notes/*\"\n  toc: .kotowari/toc.yaml\n",
    );
    write(tmp.path(), "notes/a.md", &valid("a"));
    write(tmp.path(), "notes/b.MD", "broken");
    write(tmp.path(), "notes/c.txt", "broken");
    write(tmp.path(), "notes/d.md.bak", "broken");
    write_toc(tmp.path(), &["a"]);
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    assert_eq!(code, Some(0), "{stdout}");
    assert_eq!(json(&stdout)["overview"]["files"], 1);
}

// @kotowari[REQ-core-278, TBL-core-001]
#[test]
fn req_core_278_overview_data_that_is_not_utf8_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    std::fs::create_dir_all(tmp.path().join(".kotowari/overview")).unwrap();
    std::fs::write(tmp.path().join(".kotowari/overview/a.md"), [0xff, 0xfe]).unwrap();
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert!(
        stderr.starts_with("non-UTF-8 file: .kotowari/overview/a.md"),
        "{stderr}"
    );
}

// @kotowari[REQ-core-018, REQ-core-278, TBL-core-001]
#[cfg(unix)]
#[test]
fn req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    std::fs::create_dir_all(tmp.path().join(".kotowari/overview")).unwrap();
    std::os::unix::fs::symlink(
        tmp.path().join("missing.md"),
        tmp.path().join(".kotowari/overview/a.md"),
    )
    .unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2));
    assert!(stderr.starts_with("unreadable file: "), "{stderr}");
}

// @kotowari[EX-core-465, REQ-core-280, TBL-core-020]
#[test]
fn ex_core_465_overview_data_matched_by_guides_is_a_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['docs/a.md']\n  toc: .kotowari/toc.yaml\nguides:\n  files: ['docs/a.md']\n",
    );
    write(tmp.path(), "docs/a.md", &valid("a"));
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert_eq!(
        stderr.lines().next(),
        Some("config error: docs/a.md: matched by both overview.files and guides.files")
    );
}

// @kotowari[REQ-core-280, TBL-core-020]
#[test]
fn req_core_280_the_first_overlap_in_byte_order_is_reported_and_tests_overlap_too() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['tests/**']\n  toc: .kotowari/toc.yaml\n",
    );
    write(tmp.path(), "tests/b.md", &valid("b"));
    write(tmp.path(), "tests/a.md", &valid("a"));
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files: ['tests/**']\noverview:\n  files: ['tests/**']\n  toc: .kotowari/toc.yaml\n",
    )
    .unwrap();
    let (code, _, stderr) = run(tmp.path(), &["status"]);
    assert_eq!(code, Some(2));
    assert_eq!(
        stderr.lines().next(),
        Some("config error: tests/a.md: matched by both overview.files and tests.files")
    );
}

// @kotowari[REQ-core-280, REQ-core-199]
#[test]
fn req_core_280_the_guides_and_tests_overlap_is_judged_first_and_guides_win_when_all_three_match() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), "tests/a.md", &valid("a"));
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files: ['tests/**']\nguides:\n  files: ['tests/**']\noverview:\n  files: ['tests/**']\n  toc: .kotowari/toc.yaml\n",
    )
    .unwrap();
    let (_, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(
        stderr.lines().next(),
        Some("config error: tests/a.md: matched by both guides.files and tests.files")
    );
}

// @kotowari[EX-core-472, REQ-core-288, REQ-core-290, REQ-core-027]
#[test]
fn ex_core_472_errors_in_overview_data_join_findings_counts_and_the_exit_code() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    write(
        tmp.path(),
        ".kotowari/overview/b.md",
        &without_lead("docs/ir/other.md"),
    );
    write_toc(tmp.path(), &["a", "b"]);
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    assert_eq!(code, Some(1));
    let value = json(&stdout);
    assert_eq!(value["overview"]["files"], 2);
    let lead: Vec<&serde_json::Value> = value["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == "overview_lead_missing")
        .collect();
    assert_eq!(lead.len(), 1);
    assert_eq!(lead[0]["path"], ".kotowari/overview/b.md");
    assert_eq!(lead[0]["line"], serde_json::Value::Null);
    assert_eq!(lead[0]["detail"], "b.md");
    assert_eq!(value["counts"]["overview_lead_missing"], 1);
    assert_eq!(value["counts"]["overview_ir_missing"], 1);
}

// @kotowari[REQ-core-290, REQ-core-024]
#[test]
fn req_core_290_overview_findings_are_sorted_with_the_other_findings() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['docs/a.md', 'docs/z.md']\n  toc: .kotowari/toc.yaml\n",
    );
    write(tmp.path(), "docs/a.md", &without_lead("docs/ir/cli.md"));
    // docs/ir/ の IR の誤りより docs/a.md が先、docs/z.md は後に並ぶ
    write(tmp.path(), "docs/ir/bad.md", "no title\n");
    write(tmp.path(), "docs/z.md", &without_lead("docs/ir/none.md"));
    write_toc(tmp.path(), &["a", "z"]);
    let (_, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    let paths: Vec<String> = json(&stdout)["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|finding| finding["path"].as_str().unwrap().to_string())
        .collect();
    let mut sorted = paths.clone();
    sorted.sort();
    assert_eq!(paths, sorted);
    assert_eq!(paths.first().map(String::as_str), Some("docs/a.md"));
    assert_eq!(paths.last().map(String::as_str), Some("docs/z.md"));
}

// @kotowari[REQ-core-288]
#[test]
fn req_core_288_the_text_format_does_not_show_the_overview_group() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    write_toc(tmp.path(), &["a"]);
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "text"]);
    assert_eq!(code, Some(0));
    assert!(!stdout.contains("overview"), "{stdout}");
}

// @kotowari[EX-core-463, REQ-core-279, REQ-core-288, TBL-core-005]
#[test]
fn ex_core_463_without_the_overview_key_check_reads_no_overview_data() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(
        tmp.path(),
        ".kotowari/overview/x.md",
        "# broken\n\nno lead\n",
    );
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    assert_eq!(code, Some(0));
    let value = json(&stdout);
    assert_eq!(
        value["overview"],
        serde_json::json!({"files": 0, "marks": 0})
    );
    assert!(kinds_on(&value, ".kotowari/overview/x.md").is_empty());
}

// @kotowari[EX-core-473, REQ-core-289, REQ-core-290, REQ-core-162, TBL-core-028]
#[test]
fn ex_core_473_an_error_in_overview_data_keeps_status_from_complete() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &without_lead("docs/ir/cli.md"),
    );
    write_toc(tmp.path(), &["a"]);
    let (code, stdout, _) = run(tmp.path(), &["status", "--format", "json"]);
    assert_eq!(code, Some(1));
    let value = json(&stdout);
    assert_eq!(
        value["overview"],
        serde_json::json!({"files": 1, "marks": 0})
    );
    assert_eq!(value["complete"], false);
    assert_eq!(value["findings"]["error"], 1);
}

// @kotowari[REQ-core-289, REQ-core-166, TBL-core-028]
#[test]
fn req_core_289_status_text_shows_the_overview_group_after_guides() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    write_toc(tmp.path(), &["a"]);
    let (code, stdout, _) = run(tmp.path(), &["status", "--format", "text"]);
    assert_eq!(code, Some(0), "{stdout}");
    let lines: Vec<&str> = stdout.lines().collect();
    let guides = lines
        .iter()
        .position(|line| line.starts_with("guides "))
        .unwrap();
    assert_eq!(lines[guides + 1], "overview files=1 marks=0");
    assert!(lines[guides + 2].starts_with("surface "));
}

// @kotowari[REQ-core-152, REQ-core-158]
#[test]
fn req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['docs/notes/**']\n  toc: .kotowari/toc.yaml\nguides:\n  files: ['docs/notes/**']\n",
    );
    std::fs::create_dir_all(tmp.path().join("docs/notes")).unwrap();
    std::fs::write(tmp.path().join("docs/notes/a.md"), [0xff]).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["list"]);
    assert_eq!(code, Some(0), "{stderr}");
    let (code, _, stderr) = run(tmp.path(), &["query", "REQ-001"]);
    assert_eq!(code, Some(0), "{stderr}");
}

const TOC: &str = ".kotowari/toc.yaml";

/// 名前を書かれた順に1段に並べた目次を書く
fn write_toc(tmp: &Path, names: &[&str]) {
    write(
        tmp,
        TOC,
        &format!("title: 目次\nitems: [{}]\n", names.join(", ")),
    );
}

// @kotowari[REQ-core-325, REQ-core-328, REQ-core-329, REQ-core-288]
#[test]
fn req_core_325_check_reads_the_toc_and_reports_its_errors_on_the_toc_file() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    write_toc(tmp.path(), &["z"]);
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--format", "json"]);
    assert_eq!(code, Some(1), "{stderr}");
    let value = json(&stdout);
    let mut kinds = kinds_on(&value, TOC);
    kinds.sort();
    assert_eq!(
        kinds,
        ["overview_toc_page_missing", "overview_toc_page_unknown"]
    );
    // 目次は元データの数に入らない
    assert_eq!(
        value["overview"],
        serde_json::json!({"files": 1, "marks": 0})
    );
}

// @kotowari[REQ-core-325]
#[test]
fn req_core_325_status_stops_when_the_toc_file_is_missing() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    for command in ["check", "status"] {
        let (code, stdout, stderr) = run(tmp.path(), &[command]);
        assert_eq!(code, Some(2), "{command}");
        assert!(stdout.is_empty(), "{command}");
        assert!(
            stderr.starts_with(&format!("unreadable file: {TOC}: ")),
            "{command}: {stderr}"
        );
    }
}

// @kotowari[REQ-core-325, TBL-core-001]
#[test]
fn req_core_325_a_toc_that_is_not_utf8_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    std::fs::write(tmp.path().join(TOC), [0xff, 0xfe]).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2));
    assert_eq!(
        stderr.lines().next(),
        Some(format!("non-UTF-8 file: {TOC}").as_str())
    );
}

// @kotowari[EX-core-506, REQ-core-326, TBL-core-001]
#[test]
fn ex_core_506_a_toc_matched_by_the_overview_glob_is_a_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['.kotowari/overview/*.md']\n  toc: .kotowari/overview/toc.md\n",
    );
    write(tmp.path(), ".kotowari/overview/a.md", &valid("a"));
    write(
        tmp.path(),
        ".kotowari/overview/toc.md",
        "title: 目次\nitems: [a]\n",
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert_eq!(
        stderr.lines().next(),
        Some(
            "config error: .kotowari/overview/toc.md: matched by both overview.toc and overview.files"
        )
    );
}

// @kotowari[REQ-core-326]
#[test]
fn req_core_326_the_toc_is_compared_with_the_files_each_walk_reads() {
    // overview.files は ".md" だけを読むので、同じ glob に当たる YAML の目次は重ならない
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['notes/*']\n  toc: notes/toc.yaml\n",
    );
    write(tmp.path(), "notes/a.md", &valid("a"));
    write(tmp.path(), "notes/toc.yaml", "title: 目次\nitems: [a]\n");
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(0), "{stderr}");
    // ガイドとテストの走査はファイルを拡張子で選ばないので、目次に当たる
    for (config, key) in [
        (
            "guides:\n  files: ['notes/*.yaml']\noverview:\n  files: ['notes/*.md']\n  toc: notes/toc.yaml\n",
            "guides.files",
        ),
        (
            "tests:\n  files: ['notes/*.yaml']\noverview:\n  files: ['notes/*.md']\n  toc: notes/toc.yaml\n",
            "tests.files",
        ),
    ] {
        write(tmp.path(), ".kotowari/config.yaml", config);
        let (code, _, stderr) = run(tmp.path(), &["status"]);
        assert_eq!(code, Some(2), "{config}: {stderr}");
        assert_eq!(
            stderr.lines().next(),
            Some(
                format!("config error: notes/toc.yaml: matched by both overview.toc and {key}")
                    .as_str()
            ),
            "{config}"
        );
    }
}

// @kotowari[REQ-core-326, REQ-core-280]
#[test]
fn req_core_326_the_overview_data_overlap_is_judged_before_the_toc_overlap() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['notes/*.md']\n  toc: notes/toc.md\nguides:\n  files: ['notes/*.md']\n",
    );
    write(tmp.path(), "notes/a.md", &valid("a"));
    write(tmp.path(), "notes/toc.md", "title: 目次\nitems: [a]\n");
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2));
    assert_eq!(
        stderr.lines().next(),
        Some("config error: notes/a.md: matched by both overview.files and guides.files")
    );
}

// @kotowari[EX-core-511]
#[test]
fn ex_core_511_an_unreadable_yaml_part_points_at_the_line_of_the_yaml_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    let lines = [
        "---",
        "ir:",
        "  - docs/ir/cli.md",
        "---",
        "# 題名",
        "```view lead",
        "conclusion: 結論",
        "```",
        "## 節",
        "```view cards",
        "cards:",
        "  - title: a",
        "    items: [x",
        "```",
    ];
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &format!("{}\n", lines.join("\n")),
    );
    write_toc(tmp.path(), &["a"]);
    let (code, stdout, _) = run(tmp.path(), &["check", "--format", "json"]);
    assert_eq!(code, Some(1));
    let value = json(&stdout);
    let invalid: Vec<&serde_json::Value> = value["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == "overview_part_invalid")
        .collect();
    assert_eq!(invalid.len(), 1, "{stdout}");
    assert_eq!(invalid[0]["path"], ".kotowari/overview/a.md");
    assert_eq!(invalid[0]["line"], 13);
    assert_eq!(invalid[0]["detail"], "cards (yaml)");
}
