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
        "C-greet-1 - 挨拶が出る docs/decisions/greet.md:19 tests=0\n\
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
