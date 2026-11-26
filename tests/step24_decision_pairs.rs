//! 決定の文書の多言語の対（docs/decisions/core/languages-and-guides.md の「決定の文書も、多言語の対で持てる」）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use std::path::Path;
use tempfile::TempDir;

/// 日本語の`UI の文字`の17の鍵
const JAPANESE: [(&str, &str); 17] = [
    ("language_name", "日本語"),
    ("index_link", "一覧"),
    ("pages", "{n} ページ"),
    ("stale_sections", "見直していない節 {n}"),
    ("open_items", "未決 {n}"),
    ("planned_items", "予定 {n}"),
    ("stale_mark", "IR が変わった後、まだ見直していない節"),
    ("outline_stale", "見直していない"),
    ("superseded", "（置き換え済み）"),
    ("deferred", "（後回し）"),
    ("compare_before", "前"),
    ("compare_after", "後"),
    ("compare_why", "理由"),
    ("state_decided", "決定"),
    ("state_planned", "予定"),
    ("state_open", "未決"),
    ("state_dropped", "取り下げ"),
];

/// 英語を先頭に日本語の対を持つ、決定の文書の置き場のある設定を作る
fn make_project(tmp: &Path) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    let mut config = String::from(
        "tests:\n  files:\n    - \"tests/**/*.rs\"\ndecisions:\n  documents: docs/decisions\n\
         languages: [en, ja]\nlabels:\n  ja:\n",
    );
    for (key, text) in JAPANESE {
        config.push_str(&format!("    {key}: \"{text}\"\n"));
    }
    write(tmp, ".kotowari/config.yaml", &config);
}

fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// 英語の側。rows は確認することの行
fn english(name: &str, state: &str, rows: &[&str]) -> String {
    let rows: String = rows.iter().map(|row| format!("- {row}\n")).collect();
    format!(
        "# Greetings\n\nEnglish | [日本語]({name}.ja.md)\n\nDecides how to greet.\n\n\
         ## Greet\n\nStatus: {state}\n\n### Problem\n\nTrouble.\n\n### Decision\n\nDo this.\n\n\
         ### Confirmations\n\n{rows}"
    )
}

/// 日本語の側。rows は確認することの行
fn japanese(name: &str, state: &str, rows: &[&str]) -> String {
    let rows: String = rows.iter().map(|row| format!("- {row}\n")).collect();
    format!(
        "# 挨拶\n\n[English]({name}.md) | 日本語\n\n挨拶の仕方を決める。\n\n\
         ## 挨拶する\n\n状態: {state}\n\n### 問題\n\n困っていた。\n\n### 決めたこと\n\nこうする。\n\n\
         ### 確認すること\n\n{rows}"
    )
}

/// 両側を書き、今の中身で一致の記録を書く
fn write_pair(tmp: &Path, dir: &str, name: &str, first: &str, side: &str) {
    write(tmp, &format!("{dir}/{name}.md"), first);
    write(tmp, &format!("{dir}/{name}.ja.md"), side);
    agree(tmp, dir, name);
}

/// 両側の今の中身で一致の記録を書く
fn agree(tmp: &Path, dir: &str, name: &str) {
    let hash =
        |rel: &str| kotowari_core::translations::blob_hash(&std::fs::read(tmp.join(rel)).unwrap());
    write(
        tmp,
        &format!("{dir}/{name}.i18n.yaml"),
        &format!(
            "{name}.md: {}\n{name}.ja.md: {}\n",
            hash(&format!("{dir}/{name}.md")),
            hash(&format!("{dir}/{name}.ja.md"))
        ),
    );
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
    line: Option<u64>,
    detail: &str,
) -> (String, String, Option<u64>, String) {
    (kind.to_string(), path.to_string(), line, detail.to_string())
}

const DIR: &str = "docs/decisions/greet";

// @kotowari[C-core-lang-1]
#[test]
fn a_pair_with_both_sides_and_a_matching_record_raises_nothing() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "proposed", &["C-greet-1 A greeting is shown"]),
        &japanese("greet", "提案", &["C-greet-1 挨拶が出る"]),
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![], "{value}");
    assert_eq!(
        value["decisions"],
        serde_json::json!({"documents": 1, "decisions": 1, "confirmations": 1})
    );
}

// @kotowari[C-core-lang-1]
#[test]
fn a_decision_document_without_its_japanese_side_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet/greet.md",
        &english("greet", "proposed", &["C-greet-1 A greeting is shown"]),
    );
    let value = check(tmp.path());
    assert!(
        findings(&value).contains(&finding(
            "translation_missing",
            "docs/decisions/greet/greet.md",
            None,
            "docs/decisions/greet/greet.ja.md"
        )),
        "{value}"
    );
}

// @kotowari[C-core-lang-1, C-core-lang-3]
#[test]
fn a_japanese_side_without_its_first_side_is_an_error_and_is_not_read_as_a_topic() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet/greet.ja.md",
        &japanese("greet", "提案", &["C-greet-1 挨拶が出る"]),
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![finding(
            "translation_missing",
            "docs/decisions/greet/greet.ja.md",
            None,
            "docs/decisions/greet/greet.md"
        )],
        "{value}"
    );
    assert_eq!(value["decisions"]["documents"], 0);
}

// @kotowari[C-core-lang-2]
#[test]
fn changing_one_side_without_the_record_is_an_error_on_that_side() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "proposed", &["C-greet-1 A greeting is shown"]),
        &japanese("greet", "提案", &["C-greet-1 挨拶が出る"]),
    );
    write(
        tmp.path(),
        "docs/decisions/greet/greet.ja.md",
        &japanese("greet", "提案", &["C-greet-1 挨拶の言葉が出る"]),
    );
    let kinds: Vec<_> = findings(&check(tmp.path()))
        .into_iter()
        .map(|(kind, path, _, _)| (kind, path))
        .collect();
    assert_eq!(
        kinds,
        vec![(
            "translation_stale".to_string(),
            "docs/decisions/greet/greet.ja.md".to_string()
        )]
    );
}

// @kotowari[C-core-lang-3]
#[test]
fn ids_and_marks_are_matched_against_the_first_language_side_only() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "implemented", &["C-greet-1 A greeting is shown"]),
        &japanese("greet", "実装済み", &["C-greet-1 挨拶が出る"]),
    );
    // 日本語の側にだけある ID を指す印は、どの文書にも無い ID を指す
    write(
        tmp.path(),
        "tests/a.rs",
        "// @kotowari[C-greet-1]\n#[test]\nfn one() {}\n\n// @kotowari[C-greet-9]\n#[test]\nfn nine() {}\n",
    );
    let value = check(tmp.path());
    let kinds: Vec<_> = findings(&value)
        .into_iter()
        .map(|(kind, _, _, detail)| (kind, detail))
        .collect();
    assert_eq!(kinds.len(), 1, "{value}");
    assert_eq!(kinds[0].0, "unresolved_reference", "{value}");
    assert!(kinds[0].1.contains("C-greet-9"), "{value}");
    let (_, stdout, _) = run(tmp.path(), &["query", "C-greet-1"]);
    let query: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(query["items"].as_array().unwrap().len(), 1, "{query}");
    assert_eq!(query["items"][0]["path"], "docs/decisions/greet/greet.md");
    assert_eq!(query["items"][0]["name"], "A greeting is shown");
}

// @kotowari[C-core-lang-3]
#[test]
fn a_row_only_in_the_japanese_side_of_an_implemented_decision_needs_no_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "implemented", &[]),
        &japanese("greet", "実装済み", &["C-greet-1 挨拶が出る"]),
    );
    let kinds: Vec<_> = findings(&check(tmp.path()))
        .into_iter()
        .map(|(kind, _, _, _)| kind)
        .collect();
    assert_eq!(kinds, vec!["translation_structure_mismatch".to_string()]);
}

/// 日本語の側だけを差し替えて記録を書き直し、骨組みの食い違いの (行, 部分) を返す
fn mismatch(english_side: &str, japanese_side: &str) -> Vec<(Option<u64>, String)> {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(tmp.path(), DIR, "greet", english_side, japanese_side);
    findings(&check(tmp.path()))
        .into_iter()
        .map(|(kind, path, line, detail)| {
            assert_eq!(kind, "translation_structure_mismatch");
            assert_eq!(path, "docs/decisions/greet/greet.ja.md");
            (line, detail)
        })
        .collect()
}

// @kotowari[C-core-lang-4]
#[test]
fn a_different_state_on_the_japanese_side_is_a_structure_mismatch() {
    assert_eq!(
        mismatch(
            &english("greet", "proposed", &["C-greet-1 Shown"]),
            &japanese("greet", "実装済み", &["C-greet-1 出る"]),
        ),
        vec![(Some(9), "state".to_string())]
    );
}

// @kotowari[C-core-lang-4]
#[test]
fn a_different_confirmation_id_or_reading_flag_is_a_structure_mismatch() {
    assert_eq!(
        mismatch(
            &english("greet", "proposed", &["C-greet-1 Shown"]),
            &japanese("greet", "提案", &["C-greet-2 出る"]),
        ),
        vec![(Some(21), "confirmation".to_string())]
    );
    assert_eq!(
        mismatch(
            &english("greet", "proposed", &["C-greet-1 (by reading) Shown"]),
            &japanese("greet", "提案", &["C-greet-1 出る"]),
        ),
        vec![(Some(21), "confirmation".to_string())]
    );
}

// @kotowari[C-core-lang-4]
#[test]
fn a_missing_or_different_heading_on_the_japanese_side_is_a_structure_mismatch() {
    let first = english("greet", "proposed", &["C-greet-1 Shown"]);
    let without_problem =
        japanese("greet", "提案", &["C-greet-1 出る"]).replace("### 問題\n\n困っていた。\n\n", "");
    assert_eq!(
        mismatch(&first, &without_problem),
        vec![(None, "heading".to_string())]
    );
    let other_section =
        japanese("greet", "提案", &["C-greet-1 出る"]).replace("### 問題", "### 背景");
    assert_eq!(
        mismatch(&first, &other_section),
        vec![(Some(11), "heading".to_string())]
    );
}

// @kotowari[C-core-lang-4]
#[test]
fn different_words_in_the_same_skeleton_are_no_mismatch() {
    assert_eq!(
        mismatch(
            &english("greet", "implemented", &["C-greet-1 (by reading) Shown"]),
            &japanese("greet", "実装済み", &["C-greet-1 （読んで確認）出る"]).replace(
                "こうする。",
                "こうする。表も書く。\n\n| a | b |\n|---|---|\n| 1 | 2 |"
            ),
        ),
        vec![]
    );
}

// @kotowari[C-core-lang-1]
#[test]
fn a_decision_document_without_its_switcher_line_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "proposed", &[]).replace("English | [日本語](greet.ja.md)\n\n", ""),
        &japanese("greet", "提案", &[]),
    );
    assert_eq!(
        findings(&check(tmp.path())),
        vec![finding(
            "translation_switcher_invalid",
            "docs/decisions/greet/greet.md",
            Some(3),
            "English | [日本語](greet.ja.md)"
        )]
    );
}

// @kotowari[C-core-docs-8]
#[test]
fn the_switcher_line_is_not_the_topic_paragraph() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "proposed", &[]).replace("Decides how to greet.\n\n", ""),
        &japanese("greet", "提案", &[]).replace("挨拶の仕方を決める。\n\n", ""),
    );
    assert_eq!(
        findings(&check(tmp.path())),
        vec![finding(
            "topic_lead_missing",
            "docs/decisions/greet/greet.md",
            Some(1),
            "Greetings"
        )]
    );
}

// @kotowari[C-core-docs-11]
#[test]
fn links_in_the_japanese_side_are_checked_against_the_japanese_headings() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    let link =
        |text: &str, target: &str| text.replace("こうする。", &format!("[こう]({target})する。"));
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "proposed", &[]),
        &link(&japanese("greet", "提案", &[]), "#挨拶する"),
    );
    assert_eq!(findings(&check(tmp.path())), vec![]);
    write(
        tmp.path(),
        "docs/decisions/greet/greet.ja.md",
        &link(&japanese("greet", "提案", &[]), "#greet"),
    );
    agree(tmp.path(), DIR, "greet");
    assert_eq!(
        findings(&check(tmp.path())),
        vec![finding(
            "link_target_missing",
            "docs/decisions/greet/greet.ja.md",
            Some(17),
            "#greet"
        )]
    );
    write(
        tmp.path(),
        "docs/decisions/greet/greet.ja.md",
        &link(&japanese("greet", "提案", &[]), "greet.md#greet"),
    );
    agree(tmp.path(), DIR, "greet");
    assert_eq!(
        findings(&check(tmp.path())),
        vec![finding(
            "link_language_mismatch",
            "docs/decisions/greet/greet.ja.md",
            Some(17),
            "greet.md#greet"
        )]
    );
}

// @kotowari[C-core-docs-20]
#[test]
fn a_glossary_is_paired_but_its_content_is_not_compared() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write(
        tmp.path(),
        "docs/decisions/greet/CONTEXT.md",
        "# Glossary\n\nEnglish | [日本語](CONTEXT.ja.md)\n",
    );
    let value = check(tmp.path());
    assert_eq!(
        findings(&value),
        vec![
            finding(
                "translation_record_invalid",
                "docs/decisions/greet/CONTEXT.i18n.yaml",
                None,
                "missing"
            ),
            finding(
                "translation_missing",
                "docs/decisions/greet/CONTEXT.md",
                None,
                "docs/decisions/greet/CONTEXT.ja.md"
            ),
        ]
    );
    write_pair(
        tmp.path(),
        DIR,
        "CONTEXT",
        "# Glossary\n\nEnglish | [日本語](CONTEXT.ja.md)\n\n| Term | Meaning |\n|---|---|\n| greeting | words |\n",
        "# 用語集\n\n[English](CONTEXT.md) | 日本語\n\n## 挨拶\n\n会ったときの言葉。[無い](nowhere.md)\n",
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![], "{value}");
    assert_eq!(value["decisions"]["documents"], 0);
}

// @kotowari[C-core-lang-1]
#[test]
fn list_shows_the_decision_document_pairs() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "proposed", &[]),
        &japanese("greet", "提案", &[]),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["list"]);
    assert_eq!(code, Some(0), "{stderr}");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let paths: Vec<_> = value["translations"]
        .as_array()
        .unwrap_or_else(|| panic!("{value}"))
        .iter()
        .map(|pair| pair["path"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(paths, vec!["docs/decisions/greet/greet.md".to_string()]);
}

// @kotowari[C-core-docs-1]
#[test]
fn an_unclosed_code_block_on_the_japanese_side_is_an_error_on_that_side() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &english("greet", "proposed", &["C-greet-1 Shown"]),
        &format!(
            "{}\n```text\n",
            japanese("greet", "提案", &["C-greet-1 出る"])
        ),
    );
    let (code, stdout, _) = run(tmp.path(), &["check"]);
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        findings(&value),
        vec![finding(
            "unclosed_code_block",
            "docs/decisions/greet/greet.ja.md",
            Some(23),
            "```text"
        )]
    );
    assert_eq!(code, Some(1));
}

// @kotowari[C-core-docs-1]
#[test]
fn a_pair_whose_lines_end_in_a_lone_carriage_return_reads_like_one_ending_in_line_feeds() {
    let read = |ending: &str| {
        let tmp = TempDir::new().unwrap();
        make_project(tmp.path());
        write_pair(
            tmp.path(),
            DIR,
            "greet",
            &english("greet", "implemented", &["C-greet-1 Shown"]).replace('\n', ending),
            &japanese("greet", "実装済み", &["C-greet-1 出る"]).replace('\n', ending),
        );
        let (_, stdout, _) = run(
            tmp.path(),
            &["query", "docs/decisions/greet/greet.md#greet"],
        );
        let decision: serde_json::Value = serde_json::from_str(&stdout).unwrap();
        (findings(&check(tmp.path())), decision)
    };
    let (findings_lf, decision_lf) = read("\n");
    assert_eq!(
        findings_lf,
        vec![finding(
            "confirmation_without_test",
            "docs/decisions/greet/greet.md",
            Some(21),
            "C-greet-1"
        )]
    );
    assert_eq!(read("\r"), (findings_lf, decision_lf));
}

// @kotowari[C-core-docs-1]
#[test]
fn a_second_title_on_both_sides_is_an_error_on_each_side() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &format!(
            "{}\n# More\n",
            english("greet", "proposed", &["C-greet-1 Shown"])
        ),
        &format!(
            "{}\n# 続き\n",
            japanese("greet", "提案", &["C-greet-1 出る"])
        ),
    );
    assert_eq!(
        findings(&check(tmp.path())),
        vec![
            finding(
                "multiple_titles",
                "docs/decisions/greet/greet.ja.md",
                None,
                "続き"
            ),
            finding(
                "multiple_titles",
                "docs/decisions/greet/greet.md",
                None,
                "More"
            ),
        ]
    );
}

// @kotowari[C-core-docs-1]
#[test]
fn a_leading_byte_order_mark_on_either_side_is_skipped() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path());
    write_pair(
        tmp.path(),
        DIR,
        "greet",
        &format!(
            "\u{feff}{}",
            english("greet", "proposed", &["C-greet-1 Shown"])
        ),
        &format!("\u{feff}{}", japanese("greet", "提案", &["C-greet-1 出る"])),
    );
    let value = check(tmp.path());
    assert_eq!(findings(&value), vec![], "{value}");
    assert_eq!(value["decisions"]["decisions"], 1);
}
