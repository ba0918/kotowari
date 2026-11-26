//! 決定の文書を読む check（docs/decisions/core/decision-documents.md、docs/decisions/core/confirmations.md）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 古い形の置き場と、決定の文書の置き場を指す設定を作る。`documents` が None なら鍵を書かない
fn make_project(tmp: &Path, documents: Option<&str>) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    let decisions = documents.map_or(String::new(), |place| {
        format!("decisions:\n  documents: {place}\n")
    });
    write(
        tmp,
        ".kotowari/config.yaml",
        &format!("tests:\n  files:\n    - \"tests/**/*.rs\"\n{decisions}"),
    );
}

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// 1つの決定。`state` は状態の行の値、`sections` は (節の名前, 中身) の並び
fn decision(heading: &str, state: &str, sections: &[(&str, &str)]) -> String {
    let mut text = format!("## {heading}\n\n状態: {state}\n\n");
    for (name, body) in sections {
        text.push_str(&format!("### {name}\n\n{body}\n\n"));
    }
    text
}

/// 題名と話題の段落のあとに決定を並べた文書
fn topic(decisions: &[String]) -> String {
    format!("# 挨拶\n\n挨拶の仕方を決める。\n\n{}", decisions.concat())
}

/// 問題と決めたことだけを持つ決定
fn bare(heading: &str, state: &str) -> String {
    decision(
        heading,
        state,
        &[("問題", "困っていた。"), ("決めたこと", "こうする。")],
    )
}

/// 問題・決めたこと・確認することを持つ決定。`rows` は確認することの行
fn confirmed(heading: &str, state: &str, rows: &[&str]) -> String {
    let rows: String = rows.iter().map(|row| format!("- {row}\n")).collect();
    decision(
        heading,
        state,
        &[
            ("問題", "困っていた。"),
            ("決めたこと", "こうする。"),
            ("確認すること", rows.trim_end()),
        ],
    )
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

/// check の JSON
fn check(tmp: &Path) -> serde_json::Value {
    let (_, stdout, stderr) = run(tmp, &["check"]);
    serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"))
}

/// 指摘の (kind, path, line, detail) の一覧
fn findings(value: &serde_json::Value) -> Vec<(String, String, Option<u64>, String)> {
    value["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["kind"].as_str().unwrap().to_string(),
                f["path"].as_str().unwrap().to_string(),
                f["line"].as_u64(),
                f["detail"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

fn finding(
    kind: &str,
    path: &str,
    line: u64,
    detail: &str,
) -> (String, String, Option<u64>, String) {
    (
        kind.to_string(),
        path.to_string(),
        Some(line),
        detail.to_string(),
    )
}

// @kotowari[C-core-docs-1]
#[test]
fn check_counts_the_documents_decisions_and_confirmations_it_read() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet/greet.md",
        &topic(&[
            confirmed(
                "挨拶する",
                "提案",
                &["C-greet-1 挨拶が出る", "C-greet-2 名前が入る"],
            ),
            bare("別れる", "提案"),
        ]),
    );
    write(
        tmp.path(),
        "docs/decisions/greet/farewell.md",
        &topic(&[confirmed("手を振る", "提案", &["C-farewell-1 手が動く"])]),
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
    assert_eq!(
        value["decisions"],
        serde_json::json!({"documents": 2, "decisions": 3, "confirmations": 3})
    );
}

// @kotowari[C-core-docs-15]
#[test]
fn check_reads_the_decision_documents_from_the_configured_place() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("specs"));
    write(
        tmp.path(),
        "specs/greet.md",
        &topic(&[bare("挨拶する", "決定")]),
    );
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[bare("別れる", "決定")]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "decision_state_invalid",
            "specs/greet.md",
            7,
            "決定"
        )]
    );
    assert_eq!(value["decisions"]["documents"], 1);
}

// @kotowari[C-core-docs-15]
#[test]
fn check_without_the_place_in_the_configuration_reads_no_decision_documents() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), None);
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[bare("挨拶する", "決定")]),
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
    assert!(value.get("decisions").is_none(), "{value}");
}

// @kotowari[C-core-docs-15]
#[test]
fn check_stops_when_the_configured_place_does_not_exist() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("specs"));
    let (code, _, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(stderr.contains("specs"), "{stderr}");
}

// @kotowari[C-core-docs-5]
#[test]
fn a_state_that_is_neither_proposed_nor_implemented_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[
            bare("挨拶する", "提案"),
            bare("別れる", "実装済み"),
            bare("手を振る", "決定"),
        ]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "decision_state_invalid",
            "docs/decisions/greet.md",
            31,
            "決定"
        )]
    );
}

// @kotowari[C-core-docs-5]
#[test]
fn a_decision_without_a_state_line_is_an_error_at_its_heading() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        "# 挨拶\n\n挨拶の仕方を決める。\n\n## 挨拶する\n\n### 問題\n\n困っていた。\n\n### 決めたこと\n\nこうする。\n",
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "decision_state_invalid",
            "docs/decisions/greet.md",
            5,
            ""
        )]
    );
}

// @kotowari[C-core-docs-6]
#[test]
fn a_decision_without_the_problem_section_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[decision(
            "挨拶する",
            "提案",
            &[("決めたこと", "こうする。")],
        )]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "decision_section_missing",
            "docs/decisions/greet.md",
            5,
            "問題"
        )]
    );
}

// @kotowari[C-core-docs-6]
#[test]
fn a_decision_without_the_decided_section_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[decision("挨拶する", "提案", &[("問題", "困っていた。")])]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "decision_section_missing",
            "docs/decisions/greet.md",
            5,
            "決めたこと"
        )]
    );
}

// @kotowari[C-core-docs-7]
#[test]
fn a_decision_without_rejected_options_or_confirmations_is_not_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[bare("挨拶する", "実装済み")]),
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
    assert_eq!(value["decisions"]["decisions"], 1);
}

// @kotowari[C-core-docs-8]
#[test]
fn a_document_without_the_topic_paragraph_after_its_title_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &format!("# 挨拶\n\n{}", bare("挨拶する", "提案")),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "topic_lead_missing",
            "docs/decisions/greet.md",
            1,
            "挨拶"
        )]
    );
}

// @kotowari[C-core-docs-14]
#[test]
fn check_does_not_read_the_frozen_tree_under_the_place() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[bare("挨拶する", "提案")]),
    );
    write(
        tmp.path(),
        "docs/frozen/old.md",
        &topic(&[confirmed("古い決定", "決定", &["C-old-1 古い行"])]),
    );
    write(
        tmp.path(),
        "tests/old.rs",
        "// @kotowari[C-old-1]\n#[test]\nfn old() {}\n",
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "unresolved_reference",
            "tests/old.rs",
            1,
            "C-old-1"
        )]
    );
    assert_eq!(value["decisions"]["documents"], 1);
}

/// `ids` を指す印を付けたテストを1つ書く
fn marked_test(tmp: &Path, rel: &str, ids: &[&str]) {
    let marks: String = ids
        .iter()
        .map(|id| format!("// @kotowari[{id}]\n"))
        .collect();
    write(tmp, rel, &format!("{marks}#[test]\nfn marked() {{}}\n"));
}

// @kotowari[C-core-confirm-8]
#[test]
fn a_mark_pointing_at_a_proposed_decision_gives_neither_an_error_nor_a_notice() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"])]),
    );
    marked_test(tmp.path(), "tests/greet.rs", &["C-greet-1"]);
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
}

// @kotowari[C-core-confirm-1]
#[test]
fn a_confirmation_of_an_implemented_decision_without_a_test_is_an_error_on_its_row() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "実装済み",
            &["C-greet-1 挨拶が出る", "C-greet-2 名前が入る"],
        )]),
    );
    marked_test(tmp.path(), "tests/greet.rs", &["C-greet-1"]);
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "confirmation_without_test",
            "docs/decisions/greet.md",
            20,
            "C-greet-2"
        )]
    );
}

// @kotowari[C-core-confirm-1]
#[test]
fn a_confirmation_without_a_test_does_not_stop_check_with_allow_test_findings() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "実装済み", &["C-greet-1 挨拶が出る"])]),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["check", "--allow-test-findings"]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    let (code, stdout, stderr) = run(tmp.path(), &["check"]);
    assert_eq!(code, Some(1), "{stdout}{stderr}");
}

// @kotowari[C-core-confirm-2]
#[test]
fn an_implemented_decision_whose_confirmations_all_have_tests_gives_no_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "実装済み",
            &["C-greet-1 挨拶が出る", "C-greet-2 名前が入る"],
        )]),
    );
    marked_test(tmp.path(), "tests/greet.rs", &["C-greet-1", "C-greet-2"]);
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
}

// @kotowari[C-core-confirm-7]
#[test]
fn a_confirmation_of_a_proposed_decision_without_a_test_is_not_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[
            confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"]),
            confirmed("別れる", "実装済み", &["C-greet-2 手を振る"]),
        ]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "confirmation_without_test",
            "docs/decisions/greet.md",
            35,
            "C-greet-2"
        )]
    );
}

// @kotowari[C-core-confirm-6]
#[test]
fn a_row_confirmed_by_reading_needs_no_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "実装済み",
            &[
                "C-greet-1 （読んで確認）skill に書いてある",
                "C-greet-2 名前が入る",
            ],
        )]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "confirmation_without_test",
            "docs/decisions/greet.md",
            20,
            "C-greet-2"
        )]
    );
}

// @kotowari[C-core-confirm-3]
#[test]
fn two_confirmations_with_the_same_id_are_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"])]),
    );
    write(
        tmp.path(),
        "docs/decisions/wave.md",
        &topic(&[confirmed("手を振る", "提案", &["C-greet-1 手が動く"])]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "duplicate_id",
            "docs/decisions/wave.md",
            19,
            "C-greet-1"
        )]
    );
}

// @kotowari[C-core-confirm-3]
#[test]
fn a_confirmation_row_without_a_well_formed_id_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "提案",
            &["C-greet-01 挨拶が出る", "名前が入る", "C-Greet-2 大文字"],
        )]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![
            finding("invalid_id", "docs/decisions/greet.md", 19, "C-greet-01"),
            finding("invalid_id", "docs/decisions/greet.md", 20, "名前が入る"),
            finding("invalid_id", "docs/decisions/greet.md", 21, "C-Greet-2"),
        ]
    );
}

// @kotowari[C-core-confirm-4]
#[test]
fn a_mark_pointing_at_an_id_in_no_document_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"])]),
    );
    marked_test(tmp.path(), "tests/greet.rs", &["C-greet-1", "C-greet-9"]);
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "unresolved_reference",
            "tests/greet.rs",
            2,
            "C-greet-9"
        )]
    );
}

/// status の "complete"
fn complete(tmp: &Path) -> bool {
    let (_, stdout, stderr) = run(tmp, &["status"]);
    let value: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    value["complete"].as_bool().unwrap()
}

// @kotowari[C-core-docs-12]
#[test]
fn an_item_in_the_open_section_makes_status_incomplete() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &format!(
            "{}## 未決\n\n- 夜の挨拶をどうするか。利用者が次の壁打ちで決める\n",
            topic(&[bare("挨拶する", "提案")])
        ),
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
    assert_eq!(value["decisions"]["decisions"], 1);
    assert!(!complete(tmp.path()));
}

// @kotowari[C-core-docs-13]
#[test]
fn no_errors_and_no_open_items_make_status_complete() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &format!("{}## 未決\n\n", topic(&[bare("挨拶する", "提案")])),
    );
    assert!(complete(tmp.path()));
    write(
        tmp.path(),
        "docs/decisions/wave.md",
        &topic(&[bare("手を振る", "決定")]),
    );
    assert!(!complete(tmp.path()));
}

// @kotowari[C-core-docs-11]
#[test]
fn a_link_to_a_document_that_does_not_exist_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet/greet.md",
        "# 挨拶\n\n挨拶の仕方を決める。[別れ](farewell.md)と[外](../../ir/x.md)と[Web](https://example.com/a.md)を見る。\n",
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "link_target_missing",
            "docs/decisions/greet/greet.md",
            3,
            "farewell.md"
        )]
    );
}

// @kotowari[C-core-docs-11]
#[test]
fn a_link_to_a_heading_that_does_not_exist_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &format!(
            "# 挨拶\n\n挨拶の仕方を決める。[無い](#無い見出し)と[別](wave.md#手を振らない)を見る。\n\n{}",
            bare("挨拶する", "提案")
        ),
    );
    write(
        tmp.path(),
        "docs/decisions/wave.md",
        &topic(&[bare("手を振る", "提案")]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![
            finding(
                "link_target_missing",
                "docs/decisions/greet.md",
                3,
                "#無い見出し"
            ),
            finding(
                "link_target_missing",
                "docs/decisions/greet.md",
                3,
                "wave.md#手を振らない"
            ),
        ]
    );
}

// @kotowari[C-core-docs-11]
#[test]
fn links_to_headings_follow_the_github_anchor_rule() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    let links = [
        "#挨拶は朝昼晩に分ける",
        "#決定の文書は-docsdecisions-に置き古い形は凍結する",
        "#use-the-config_file",
        "#挨拶は朝昼晩に分ける-1",
        "wave.md#手を振る",
        "../decisions/wave.md",
        "#%E6%8C%A8%E6%8B%B6%E3%81%AF%E6%9C%9D%E6%98%BC%E6%99%A9%E3%81%AB%E5%88%86%E3%81%91%E3%82%8B",
    ];
    let lead: String = links
        .iter()
        .map(|target| format!("[リンク]({target})"))
        .collect();
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &format!(
            "# 挨拶\n\n挨拶の仕方を決める。{lead}\n\n{}{}{}{}",
            bare("挨拶は、朝・昼・晩に分ける", "提案"),
            bare(
                "決定の文書は docs/decisions に置き、古い形は凍結する",
                "提案"
            ),
            bare("Use the `config_file`!", "提案"),
            bare("挨拶は、朝・昼・晩に分ける", "提案"),
        ),
    );
    write(
        tmp.path(),
        "docs/decisions/wave.md",
        &topic(&[bare("手を振る", "提案")]),
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
}

// @kotowari[C-core-docs-11]
#[test]
fn a_link_to_a_file_that_is_not_markdown_is_not_checked() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(tmp.path(), "docs/decisions/greet/diagram.png", "");
    write(
        tmp.path(),
        "docs/decisions/greet/greet.md",
        "# 挨拶\n\n挨拶の仕方を決める。![図](diagram.png)\n",
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![]);
}

// @kotowari[C-core-docs-8]
#[test]
fn a_document_without_a_title_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &format!("挨拶の仕方を決める。\n\n{}", bare("挨拶する", "提案")),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![(
            "missing_title".to_string(),
            "docs/decisions/greet.md".to_string(),
            None,
            String::new()
        )]
    );
}

/// 確認することの ID を指すテストを2つ持つテストのファイルを書く
fn two_marked_tests(tmp: &Path, rel: &str, id: &str) {
    write(
        tmp,
        rel,
        &format!(
            "// @kotowari[{id}]\n#[test]\nfn first() {{}}\n\n// @kotowari[{id}]\n#[test]\nfn second() {{}}\n"
        ),
    );
}

/// query の JSON
fn query(tmp: &Path, id: &str) -> serde_json::Value {
    let (code, stdout, stderr) = run(tmp, &["query", id]);
    assert_eq!(code, Some(0), "{stdout}{stderr}");
    serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"))
}

// @kotowari[C-core-confirm-5]
#[test]
fn query_of_a_confirmation_id_returns_the_file_line_and_name_of_each_test_pointing_at_it() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "実装済み",
            &["C-greet-1 挨拶が出る", "C-greet-2 名前が `name` で入る"],
        )]),
    );
    two_marked_tests(tmp.path(), "tests/greet.rs", "C-greet-2");
    marked_test(tmp.path(), "tests/other.rs", &["C-greet-1"]);
    let value = query(tmp.path(), "C-greet-2");
    assert_eq!(
        value,
        serde_json::json!({"items": [{
            "id": "C-greet-2",
            "kind": "confirmation",
            "name": "名前が name で入る",
            "path": "docs/decisions/greet.md",
            "line": 20,
            "decision": "挨拶する",
            "state": "implemented",
            "by_reading": false,
            "tests": [
                {"path": "tests/greet.rs", "line": 1, "name": "first"},
                {"path": "tests/greet.rs", "line": 5, "name": "second"},
            ],
            "body": ["- C-greet-2 名前が `name` で入る"],
            "referenced_by": [],
        }]})
    );
}

// @kotowari[C-core-confirm-5]
#[test]
fn query_of_a_confirmation_id_prints_its_tests_in_text() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "提案",
            &["C-greet-1 （読んで確認）skill に挨拶が書いてある"],
        )]),
    );
    two_marked_tests(tmp.path(), "tests/greet.rs", "C-greet-1");
    let (code, stdout, stderr) = run(tmp.path(), &["query", "C-greet-1", "--format", "text"]);
    assert_eq!(code, Some(0), "{stderr}");
    assert_eq!(
        stdout,
        "C-greet-1 reading skill に挨拶が書いてある docs/decisions/greet.md:19 tests=2\n\
         \x20 tests/greet.rs:1 first\n\
         \x20 tests/greet.rs:5 second\n\
         \x20 - C-greet-1 （読んで確認）skill に挨拶が書いてある\n\
         \x20 decision proposed 挨拶する\n"
    );
}

// @kotowari[C-core-confirm-5]
#[test]
fn query_of_a_confirmation_id_without_tests_returns_an_empty_list_of_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"])]),
    );
    let value = query(tmp.path(), "C-greet-1");
    assert_eq!(value["items"][0]["tests"], serde_json::json!([]));
}

// @kotowari[C-core-confirm-5]
#[test]
fn query_of_a_confirmation_id_in_no_document_stops_as_an_unknown_id() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"])]),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["query", "C-greet-9"]);
    assert_eq!(code, Some(2), "{stdout}");
    assert_eq!(stdout, "");
    assert_eq!(
        stderr.lines().next().unwrap_or(""),
        "argument error: unknown id: C-greet-9"
    );
}

// @kotowari[C-core-confirm-5]
#[test]
fn list_includes_the_confirmation_rows_in_path_and_line_order() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "提案",
            &["C-greet-1 挨拶が出る", "C-greet-2 名前が入る"],
        )]),
    );
    marked_test(tmp.path(), "tests/greet.rs", &["C-greet-2"]);
    let (code, stdout, stderr) = run(tmp.path(), &["list", "--format", "text"]);
    assert_eq!(code, Some(0), "{stderr}");
    assert_eq!(
        stdout,
        "docs/decisions/greet.md#挨拶する - 挨拶する docs/decisions/greet.md:5 tests=0\n\
         C-greet-1 - 挨拶が出る docs/decisions/greet.md:19 tests=0\n\
         C-greet-2 - 名前が入る docs/decisions/greet.md:20 tests=1\n\
         \x20 tests/greet.rs:1 marked\n"
    );
}

// @kotowari[C-core-confirm-5]
#[test]
fn query_of_a_wrapped_confirmation_row_joins_its_name_and_keeps_its_lines_as_the_body() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "提案",
            &["C-greet-1 挨拶が\n  二行で出る", "C-greet-2 名前が入る"],
        )]),
    );
    let value = query(tmp.path(), "C-greet-1");
    assert_eq!(value["items"][0]["name"], "挨拶が 二行で出る");
    assert_eq!(
        value["items"][0]["body"],
        serde_json::json!(["- C-greet-1 挨拶が", "  二行で出る"])
    );
}

/// status を走らせて (JSON, text の出力) を返す
fn status(tmp: &Path) -> (serde_json::Value, String) {
    let (_, stdout, stderr) = run(tmp, &["status"]);
    let value = serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}{stderr}"));
    let (_, text, _) = run(tmp, &["status", "--format", "text"]);
    (value, text)
}

// @kotowari[C-core-confirm-9]
#[test]
fn status_counts_the_proposed_and_the_implemented_decisions_apart() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[
            bare("挨拶する", "提案"),
            bare("別れる", "実装済み"),
            bare("手を振る", "提案"),
        ]),
    );
    write(
        tmp.path(),
        "docs/decisions/wave.md",
        &topic(&[bare("会釈する", "提案"), bare("黙る", "決定")]),
    );
    let (value, text) = status(tmp.path());
    assert_eq!(
        value["decisions"],
        serde_json::json!({"proposed": 3, "implemented": 1})
    );
    assert!(
        text.lines()
            .any(|line| line == "decisions proposed=3 implemented=1"),
        "{text}"
    );
}

// @kotowari[C-core-confirm-9]
#[test]
fn status_without_the_place_in_the_configuration_shows_no_decision_counts() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), None);
    let (value, text) = status(tmp.path());
    assert!(value.get("decisions").is_none(), "{value}");
    assert!(
        !text.lines().any(|line| line.starts_with("decisions ")),
        "{text}"
    );
}

// @kotowari[C-core-confirm-5]
#[test]
fn list_sorts_the_confirmation_rows_with_the_ir_items_by_path() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/ir/a.md",
        "# 題名\n\n範囲。\n\n## Requirements\n\n### REQ-001: 例\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\n文である。\n",
    );
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"])]),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["list"]);
    assert_eq!(code, Some(0), "{stderr}");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let ids: Vec<&str> = value["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["id"].as_str().unwrap())
        .collect();
    assert_eq!(
        ids,
        vec!["docs/decisions/greet.md#挨拶する", "C-greet-1", "REQ-001"]
    );
}

// @kotowari[C-core-confirm-5]
#[test]
fn a_row_of_a_decision_with_a_wrong_state_line_has_no_state() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed("挨拶する", "決定", &["C-greet-1 挨拶が出る"])]),
    );
    let value = query(tmp.path(), "C-greet-1");
    assert_eq!(value["items"][0]["state"], serde_json::Value::Null);
    let (_, text, _) = run(tmp.path(), &["query", "C-greet-1", "--format", "text"]);
    assert_eq!(text.lines().last(), Some("  decision - 挨拶する"), "{text}");
}

// @kotowari[C-core-confirm-5]
#[test]
fn list_leaves_out_a_row_without_a_well_formed_id() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[confirmed(
            "挨拶する",
            "提案",
            &["挨拶が出る", "C-greet-2 名前が入る"],
        )]),
    );
    let (_, stdout, _) = run(tmp.path(), &["list", "--format", "text"]);
    assert_eq!(
        stdout,
        "docs/decisions/greet.md#挨拶する - 挨拶する docs/decisions/greet.md:5 tests=0\n\
         C-greet-2 - 名前が入る docs/decisions/greet.md:20 tests=0\n"
    );
}

// --- ガイドの印が決定を指す（docs/decisions/core/languages-and-guides.md） ---

/// 決定の文書の置き場と、"guides/**/*.md" をガイドとして読む設定を作る
fn make_guided_project(tmp: &Path) {
    make_project(tmp, Some("docs/decisions"));
    let config = tmp.join(".kotowari/config.yaml");
    let mut text = std::fs::read_to_string(&config).unwrap();
    text.push_str("guides:\n  files:\n    - \"guides/**/*.md\"\n");
    std::fs::write(config, text).unwrap();
}

/// 挨拶の決定1つの文書。`decided` は決めたこと、`rows` は確認することの中身
fn greeting(state: &str, problem: &str, decided: &str, rows: &str) -> String {
    topic(&[decision(
        "挨拶する",
        state,
        &[
            ("問題", problem),
            ("決めたこと", decided),
            ("確認すること", rows),
        ],
    )])
}

/// `greeting` の既定の中身で書いた文書の、決めたことと確認することの指紋
const GREETING_FINGERPRINT: &str = "0c16c2c1";

/// "guides/a.md" の3行目に、`entries` を並べたガイドの印を置く
fn write_guide(tmp: &Path, entries: &str) {
    write(
        tmp,
        "guides/a.md",
        &format!("# ガイド\n\n<!-- @kotowari[{entries}] -->\n\n挨拶の仕方。\n"),
    );
}

/// "guides/a.md" への指摘の (種類, 行, detail)
fn guide_findings(value: &serde_json::Value) -> Vec<(String, u64, String)> {
    findings(value)
        .into_iter()
        .filter(|(_, path, _, _)| path == "guides/a.md")
        .map(|(kind, _, line, detail)| (kind, line.unwrap_or(0), detail))
        .collect()
}

/// 既定の挨拶の文書と、それを今の指紋で指すガイド
fn greeting_project(tmp: &Path, document: &str) {
    make_guided_project(tmp);
    write(tmp, "docs/decisions/greet.md", document);
    write_guide(
        tmp,
        &format!("docs/decisions/greet.md#挨拶する:{GREETING_FINGERPRINT}"),
    );
}

// @kotowari[C-core-lang-5]
#[test]
fn a_guide_mark_with_the_current_fingerprint_of_a_decision_is_not_stale() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    let value = check(tmp.path());
    assert_eq!(guide_findings(&value), vec![], "{value}");
    assert_eq!(value["guides"]["marks"], 1, "{value}");
}

// @kotowari[C-core-lang-5]
#[test]
fn changing_what_was_decided_makes_the_guide_mark_stale() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &greeting(
            "提案",
            "困っていた。",
            "ああする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    let value = check(tmp.path());
    let found = guide_findings(&value);
    assert_eq!(found.len(), 1, "{value}");
    assert_eq!(found[0].0, "guide_stale");
    let detail = &found[0].2;
    let current = detail
        .strip_prefix(&format!(
            "docs/decisions/greet.md#挨拶する {GREETING_FINGERPRINT} "
        ))
        .unwrap_or_else(|| panic!("{detail}"));
    assert!(
        current.len() == 8 && current != GREETING_FINGERPRINT,
        "{detail}"
    );
}

// @kotowari[C-core-lang-5]
#[test]
fn changing_a_row_to_confirm_makes_the_guide_mark_stale() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が二度出る",
        ),
    );
    let value = check(tmp.path());
    assert_eq!(
        guide_findings(&value)
            .iter()
            .map(|(kind, _, _)| kind.as_str())
            .collect::<Vec<_>>(),
        vec!["guide_stale"],
        "{value}"
    );
}

// @kotowari[C-core-lang-5]
#[test]
fn a_heading_below_what_was_decided_stays_inside_the_fingerprint() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &greeting(
            "提案",
            "困っていた。",
            "こうする。\n\n#### 例\n\n朝に言う。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    let value = check(tmp.path());
    assert_eq!(guide_findings(&value).len(), 1, "{value}");
}

// @kotowari[C-core-lang-6]
#[test]
fn changing_only_the_state_does_not_make_the_guide_mark_stale() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &greeting(
            "実装済み",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    let value = check(tmp.path());
    assert_eq!(guide_findings(&value), vec![], "{value}");
}

// @kotowari[C-core-lang-6]
#[test]
fn changing_only_the_problem_does_not_make_the_guide_mark_stale() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &greeting(
            "提案",
            "ずっと困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    let value = check(tmp.path());
    assert_eq!(guide_findings(&value), vec![], "{value}");
}

// @kotowari[C-core-lang-6]
#[test]
fn adding_rejected_options_does_not_make_the_guide_mark_stale() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &topic(&[decision(
            "挨拶する",
            "提案",
            &[
                ("問題", "困っていた。"),
                ("決めたこと", "こうする。"),
                ("見送った案", "- 黙る。失礼になる。"),
                ("確認すること", "- C-greet-1 挨拶が出る"),
            ],
        )]),
    );
    let value = check(tmp.path());
    assert_eq!(guide_findings(&value), vec![], "{value}");
}

// @kotowari[C-core-lang-5]
#[test]
fn a_guide_mark_pointing_at_no_decision_is_stale_with_no_current_fingerprint() {
    let tmp = TempDir::new().unwrap();
    make_guided_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    // 無い見出し、決定でない見出し（題名と節）、無い文書
    write_guide(
        tmp.path(),
        "docs/decisions/greet.md#別れる:0c16c2c1, docs/decisions/greet.md#挨拶:0c16c2c1, \
         docs/decisions/greet.md#問題:0c16c2c1, docs/decisions/none.md#挨拶する:0c16c2c1",
    );
    let value = check(tmp.path());
    assert_eq!(
        guide_findings(&value)
            .into_iter()
            .map(|(_, _, detail)| detail)
            .collect::<Vec<_>>(),
        vec![
            "docs/decisions/greet.md#別れる 0c16c2c1 -",
            "docs/decisions/greet.md#問題 0c16c2c1 -",
            "docs/decisions/greet.md#挨拶 0c16c2c1 -",
            "docs/decisions/none.md#挨拶する 0c16c2c1 -",
        ],
        "{value}"
    );
    assert!(
        findings(&value)
            .iter()
            .all(|(kind, _, _, _)| kind != "unresolved_reference"),
        "{value}"
    );
}

// @kotowari[C-core-lang-5]
#[test]
fn a_repeated_heading_is_pointed_at_with_the_numbered_anchor() {
    let tmp = TempDir::new().unwrap();
    make_guided_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &topic(&[
            bare("挨拶する", "提案"),
            confirmed("挨拶する", "提案", &["C-greet-1 挨拶が出る"]),
        ]),
    );
    write_guide(
        tmp.path(),
        &format!("docs/decisions/greet.md#挨拶する-1:{GREETING_FINGERPRINT}"),
    );
    let value = check(tmp.path());
    assert_eq!(guide_findings(&value), vec![], "{value}");
}

// @kotowari[C-core-lang-5]
#[test]
fn a_guide_mark_holds_marks_of_the_old_form_and_of_decisions_together() {
    let tmp = TempDir::new().unwrap();
    greeting_project(
        tmp.path(),
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    write_guide(
        tmp.path(),
        &format!("REQ-009:51b1f3da, docs/decisions/greet.md#挨拶する:{GREETING_FINGERPRINT}"),
    );
    let value = check(tmp.path());
    assert_eq!(
        guide_findings(&value),
        vec![(
            "guide_stale".to_string(),
            3,
            "REQ-009 51b1f3da -".to_string()
        )],
        "{value}"
    );
    assert_eq!(value["guides"]["marks"], 2, "{value}");
}

// @kotowari[C-core-lang-5]
#[test]
fn a_decision_entry_without_an_anchor_or_with_a_bad_fingerprint_is_an_invalid_mark() {
    let tmp = TempDir::new().unwrap();
    make_guided_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    write(
        tmp.path(),
        "guides/a.md",
        "# ガイド\n\n<!-- @kotowari[docs/decisions/greet.md#:0c16c2c1] -->\n\n\
         <!-- @kotowari[#挨拶する:0c16c2c1] -->\n\n\
         <!-- @kotowari[docs/decisions/greet.md#挨拶する:0C16C2C1] -->\n",
    );
    let value = check(tmp.path());
    assert_eq!(
        guide_findings(&value)
            .into_iter()
            .map(|(kind, line, _)| (kind, line))
            .collect::<Vec<_>>(),
        vec![
            ("invalid_marker".to_string(), 3),
            ("invalid_marker".to_string(), 5),
            ("invalid_marker".to_string(), 7),
        ],
        "{value}"
    );
    assert_eq!(value["guides"]["marks"], 0, "{value}");
}

// @kotowari[C-core-lang-5]
#[test]
fn query_of_a_decision_returns_the_fingerprint_a_guide_mark_takes() {
    let tmp = TempDir::new().unwrap();
    make_guided_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    let value = query(tmp.path(), "docs/decisions/greet.md#挨拶する");
    assert_eq!(
        value,
        serde_json::json!({"items": [{
            "id": "docs/decisions/greet.md#挨拶する",
            "kind": "decision",
            "name": "挨拶する",
            "path": "docs/decisions/greet.md",
            "line": 5,
            "state": "proposed",
            "fingerprint": GREETING_FINGERPRINT,
            "body": ["### 決めたこと", "", "こうする。", "### 確認すること", "", "- C-greet-1 挨拶が出る"],
            "referenced_by": [],
        }]})
    );
}

// @kotowari[C-core-lang-5]
#[test]
fn list_includes_each_decision_with_its_fingerprint() {
    let tmp = TempDir::new().unwrap();
    make_guided_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["list", "--format", "text"]);
    assert_eq!(code, Some(0), "{stderr}");
    assert_eq!(
        stdout,
        "docs/decisions/greet.md#挨拶する - 挨拶する docs/decisions/greet.md:5 tests=0\n\
         C-greet-1 - 挨拶が出る docs/decisions/greet.md:19 tests=0\n"
    );
}

// @kotowari[C-core-lang-5]
#[test]
fn a_decision_entry_outside_a_markdown_path_or_with_a_second_hash_or_a_space_is_an_invalid_mark() {
    let tmp = TempDir::new().unwrap();
    make_guided_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &greeting(
            "提案",
            "困っていた。",
            "こうする。",
            "- C-greet-1 挨拶が出る",
        ),
    );
    write(
        tmp.path(),
        "guides/a.md",
        "# ガイド\n\n<!-- @kotowari[docs/decisions/greet.txt#挨拶する:0c16c2c1] -->\n\n\
         <!-- @kotowari[docs/decisions/greet.md#挨拶#する:0c16c2c1] -->\n\n\
         <!-- @kotowari[docs/decisions/greet.md#挨拶 する:0c16c2c1] -->\n",
    );
    let value = check(tmp.path());
    assert_eq!(
        guide_findings(&value)
            .into_iter()
            .map(|(kind, line, _)| (kind, line))
            .collect::<Vec<_>>(),
        vec![
            ("invalid_marker".to_string(), 3),
            ("invalid_marker".to_string(), 5),
            ("invalid_marker".to_string(), 7),
        ],
        "{value}"
    );
}

/// 英語の形の語で書いた、題名と話題の段落のあとに決定を並べた文書
fn english_topic(decisions: &[&str]) -> String {
    format!(
        "# Greetings\n\nDecides how to greet.\n\n{}",
        decisions.concat()
    )
}

// @kotowari[C-core-docs-1]
// @kotowari[C-core-confirm-6]
#[test]
fn a_document_written_with_the_english_keywords_is_read_like_the_japanese_one() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &format!(
            "{}## Open\n\n- How to greet at night. The user decides it at the next brainstorm\n",
            english_topic(&[
                "## Greet\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo this.\n\n\
                 ### Rejected options\n\n- Not this. Why.\n\n### Confirmations\n\n\
                 - C-greet-1 (by reading) The skill says so\n- C-greet-2 A greeting is shown\n\n",
                "## Leave\n\nStatus: proposed\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo that.\n\n",
            ])
        ),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "confirmation_without_test",
            "docs/decisions/greet.md",
            24,
            "C-greet-2"
        )]
    );
    assert_eq!(
        value["decisions"],
        serde_json::json!({"documents": 1, "decisions": 2, "confirmations": 2})
    );
    let (status, _) = status(tmp.path());
    assert_eq!(
        status["decisions"],
        serde_json::json!({"proposed": 1, "implemented": 1})
    );
    assert_eq!(status["complete"], false);
}

// @kotowari[C-core-confirm-5]
#[test]
fn list_gives_the_same_state_values_for_english_and_japanese_documents() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/en.md",
        &english_topic(&[
            "## Greet\n\nStatus: proposed\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo this.\n\n\
             ### Confirmations\n\n- C-en-1 (by reading) The skill says so\n\n",
        ]),
    );
    write(
        tmp.path(),
        "docs/decisions/ja.md",
        &topic(&[confirmed(
            "挨拶する",
            "提案",
            &["C-ja-1 （読んで確認）skill に書いてある"],
        )]),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["list"]);
    assert_eq!(code, Some(0), "{stderr}");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let rows: Vec<_> = value["items"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|item| item["kind"] == "confirmation")
        .map(|item| {
            (
                item["id"].as_str().unwrap().to_string(),
                item["state"].clone(),
                item["by_reading"].clone(),
                item["name"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(
        rows,
        vec![
            (
                "C-en-1".to_string(),
                serde_json::json!("proposed"),
                serde_json::json!(true),
                "The skill says so".to_string()
            ),
            (
                "C-ja-1".to_string(),
                serde_json::json!("proposed"),
                serde_json::json!(true),
                "skill に書いてある".to_string()
            ),
        ]
    );
}

// @kotowari[C-core-docs-6]
#[test]
fn a_missing_section_is_named_in_the_language_of_the_state_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &english_topic(&["## Greet\n\nStatus: proposed\n\n### Decision\n\nDo this.\n\n"]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "decision_section_missing",
            "docs/decisions/greet.md",
            5,
            "Problem"
        )]
    );
}

// @kotowari[C-core-docs-5]
#[test]
fn an_english_state_line_with_another_value_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some("docs/decisions"));
    write(
        tmp.path(),
        "docs/decisions/greet.md",
        &english_topic(&[
            "## Greet\n\nStatus: decided\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo this.\n\n",
        ]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "decision_state_invalid",
            "docs/decisions/greet.md",
            7,
            "decided"
        )]
    );
}
