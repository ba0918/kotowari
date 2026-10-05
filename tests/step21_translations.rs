//! 対の読み方、一致の記録、骨組み、切り替えの行、リンク（docs/ir/core/translation-pairs.md、
//! docs/ir/core/translation-structure.md）

use serde_json::Value;
use std::path::Path;
use tempfile::TempDir;

/// TBL-core-046 の17の鍵と、日本語の`UI の文字`
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

/// "languages" と、英語でない言語の完全な "labels"。languages が None なら鍵を書かない
fn language_config(languages: Option<&[&str]>) -> String {
    let Some(languages) = languages else {
        return String::new();
    };
    let mut out = format!("languages: [{}]\n", languages.join(", "));
    let others: Vec<&&str> = languages.iter().filter(|tag| **tag != "en").collect();
    if !others.is_empty() {
        out.push_str("labels:\n");
        for tag in others {
            out.push_str(&format!("  {tag}:\n"));
            for (key, text) in JAPANESE {
                out.push_str(&format!("    {key}: \"{text}\"\n"));
            }
        }
    }
    out
}

/// 置き場と設定と判断の記録を作る。extra は設定に足す行
fn make_project(tmp: &Path, languages: Option<&[&str]>, extra: &str) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        format!(
            "tests:\n  files:\n    - \"tests/**/*.rs\"\n{}{extra}",
            language_config(languages)
        ),
    )
    .unwrap();
    write(
        tmp,
        "docs/decision/records/r.md",
        "# Records\n\n## Context\n\nc\n\n## Agreements\n\n- A1 Agreement\n  - why: because\n",
    );
}

const JA_EN: Option<&[&str]> = Some(&["ja", "en"]);
const GUIDES: &str = "guides:\n  files:\n    - \"guides/**/*.md\"\n";

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// 要求を1つ持つ話題ごとの文書
fn topic(id: &str, statement: &str) -> String {
    format!(
        "# A\n\nScope.\n\n## Requirements\n\n### {id}: Name\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: read\n\n{statement}\n"
    )
}

/// コマンドを走らせて (終了コード, 標準出力)
fn run(tmp: &Path, args: &[&str]) -> (Option<i32>, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(args)
        .current_dir(tmp)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
    )
}

/// "kotowari check --format json" の出力
fn check(tmp: &Path) -> Value {
    let (_, stdout) = run(tmp, &["check", "--format", "json"]);
    serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

/// その種類の指摘を (path, line, detail) で集める
fn findings(report: &Value, kind: &str) -> Vec<(String, Value, String)> {
    report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == kind)
        .map(|finding| {
            (
                finding["path"].as_str().unwrap().to_string(),
                finding["line"].clone(),
                finding["detail"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

/// その種類の指摘の (path, detail)
fn paths_and_details(report: &Value, kind: &str) -> Vec<(String, String)> {
    findings(report, kind)
        .into_iter()
        .map(|(path, _, detail)| (path, detail))
        .collect()
}

fn pair(path: &str, detail: &str) -> (String, String) {
    (path.to_string(), detail.to_string())
}

const A_HASH: &str = "78981922613b2afb6025042ff6bd878ac1994e85";
const EMPTY_HASH: &str = "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391";

/// EX-core-518 のガイドの対と一致の記録
fn guide_pair_with_record(tmp: &Path, record: Option<&str>) {
    make_project(tmp, JA_EN, GUIDES);
    write(tmp, "guides/a.md", "a\n");
    write(tmp, "guides/a.en.md", "a\n");
    if let Some(record) = record {
        write(tmp, "guides/a.i18n.yaml", record);
    }
}

fn both_hashes() -> String {
    format!("a.md: {A_HASH}\na.en.md: {A_HASH}\n")
}

// @kotowari[REQ-core-334, EX-core-512]
#[test]
fn ex_core_512_with_one_language_a_suffixed_file_is_a_document_of_its_own() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), None, "");
    write(tmp.path(), "docs/ir/a.md", &topic("REQ-001", "Body."));
    write(tmp.path(), "docs/ir/a.en.md", &topic("REQ-002", "Body."));
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_missing").is_empty(),
        "{report}"
    );
    assert_eq!(report["files"], 2);
    let (_, list) = run(tmp.path(), &["list"]);
    let list: Value = serde_json::from_str(&list).unwrap();
    let paths: Vec<&str> = list["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["path"].as_str().unwrap())
        .collect();
    assert!(paths.contains(&"docs/ir/a.en.md"), "{list}");
    assert!(list.get("translations").is_none(), "{list}");
}

// @kotowari[REQ-core-336, REQ-core-337, REQ-core-338, EX-core-514]
#[test]
fn ex_core_514_a_missing_english_side_is_an_error_on_the_first_side() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "docs/ir/a.md", &topic("REQ-001", "文。"));
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_missing"),
        [pair("docs/ir/a.md", "docs/ir/a.en.md")]
    );
    let missing = findings(&report, "translation_missing");
    assert!(missing[0].1.is_null(), "{report}");
}

// @kotowari[REQ-core-338, EX-core-515]
#[test]
fn ex_core_515_a_missing_first_side_is_an_error_on_the_side_that_exists() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "docs/ir/b.en.md", &topic("REQ-001", "Text."));
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_missing"),
        [pair("docs/ir/b.en.md", "docs/ir/b.md")]
    );
    // その側はほかの検査で読まない
    assert!(
        report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .all(|finding| finding["kind"] == "translation_missing"),
        "{report}"
    );
}

// @kotowari[REQ-core-337, EX-core-516]
#[test]
fn ex_core_516_a_suffix_of_the_first_language_or_an_unlisted_one_is_part_of_the_name() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "docs/ir/c.ja.md", &topic("REQ-001", "文。"));
    write(tmp.path(), "docs/ir/c.fr.md", &topic("REQ-002", "文。"));
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_missing"),
        [
            pair("docs/ir/c.fr.md", "docs/ir/c.fr.en.md"),
            pair("docs/ir/c.ja.md", "docs/ir/c.ja.en.md"),
        ]
    );
}

// @kotowari[REQ-core-336, EX-core-517]
#[test]
fn ex_core_517_decision_records_are_not_paired() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_missing")
            .iter()
            .all(|(path, _, _)| !path.starts_with("docs/decision/")),
        "{report}"
    );
}

// @kotowari[REQ-core-339, REQ-core-340, REQ-core-341, EX-core-518]
#[test]
fn ex_core_518_a_record_matching_the_files_raises_nothing() {
    let tmp = TempDir::new().unwrap();
    guide_pair_with_record(tmp.path(), Some(&both_hashes()));
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_stale").is_empty(),
        "{report}"
    );
    assert!(
        findings(&report, "translation_record_invalid").is_empty(),
        "{report}"
    );
    assert!(
        findings(&report, "translation_missing").is_empty(),
        "{report}"
    );
}

// @kotowari[REQ-core-341, EX-core-519]
#[test]
fn ex_core_519_changing_one_side_makes_it_stale() {
    let tmp = TempDir::new().unwrap();
    guide_pair_with_record(tmp.path(), Some(&both_hashes()));
    write(tmp.path(), "guides/a.md", "");
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "translation_stale"),
        [(
            "guides/a.md".to_string(),
            Value::Null,
            format!("{A_HASH} {EMPTY_HASH}")
        )]
    );
}

// @kotowari[REQ-core-339, EX-core-520]
#[test]
fn ex_core_520_record_keys_other_than_the_side_names_are_invalid() {
    let tmp = TempDir::new().unwrap();
    guide_pair_with_record(
        tmp.path(),
        Some(&format!("a.md: {A_HASH}\na.zh.md: {A_HASH}\n")),
    );
    write(tmp.path(), "guides/a.md", "");
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "translation_record_invalid"),
        [(
            "guides/a.i18n.yaml".to_string(),
            Value::Null,
            "keys".to_string()
        )]
    );
    assert!(
        findings(&report, "translation_stale").is_empty(),
        "{report}"
    );
}

// @kotowari[REQ-core-339, EX-core-521]
#[test]
fn ex_core_521_a_missing_record_is_invalid() {
    let tmp = TempDir::new().unwrap();
    guide_pair_with_record(tmp.path(), None);
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_record_invalid"),
        [pair("guides/a.i18n.yaml", "missing")]
    );
}

// @kotowari[REQ-core-339]
#[test]
fn req_core_339_an_unreadable_record_or_a_malformed_value_is_named_in_order() {
    for (record, detail) in [
        ("a.md: [\n", "yaml"),
        ("- a.md\n", "keys"),
        (&*format!("a.md: {A_HASH}\na.en.md: 1\n"), "keys"),
        (&*format!("a.md: {A_HASH}\na.en.md: ABC\n"), "value"),
        (
            &*format!("a.md: {A_HASH}\na.en.md: {}\n", A_HASH.to_uppercase()),
            "value",
        ),
    ] {
        let tmp = TempDir::new().unwrap();
        guide_pair_with_record(tmp.path(), Some(record));
        let report = check(tmp.path());
        assert_eq!(
            paths_and_details(&report, "translation_record_invalid"),
            [pair("guides/a.i18n.yaml", detail)],
            "{record}"
        );
    }
}

// @kotowari[REQ-core-337]
#[test]
fn req_core_337_a_pair_read_from_two_places_is_reported_once_and_sides_are_found_by_name() {
    let tmp = TempDir::new().unwrap();
    // ガイドの glob は先頭の言語の側にだけ当たり、IR の置き場にも当たる
    make_project(
        tmp.path(),
        JA_EN,
        "guides:\n  files:\n    - \"guides/a.md\"\n    - \"docs/ir/*.md\"\n",
    );
    write(tmp.path(), "docs/ir/a.md", &topic("REQ-001", "文。"));
    write(tmp.path(), "guides/a.md", "a\n");
    write(tmp.path(), "guides/a.en.md", "a\n");
    write(tmp.path(), "guides/a.i18n.yaml", &both_hashes());
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_missing"),
        [pair("docs/ir/a.md", "docs/ir/a.en.md")]
    );
    assert_eq!(
        paths_and_details(&report, "translation_record_invalid"),
        [pair("docs/ir/a.i18n.yaml", "missing")]
    );
    // glob に当たらない "guides/a.en.md" も側として読まれ、hash が照らされる
    write(tmp.path(), "guides/a.en.md", "b\n");
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "translation_stale")
            .into_iter()
            .map(|(path, _, _)| path)
            .collect::<Vec<_>>(),
        ["guides/a.en.md"]
    );
}

/// 用語 "term" を持つか持たない用語集
fn glossary(with_term: bool) -> String {
    let mut text = "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n".to_string();
    if with_term {
        text.push_str("| term | meaning | docs/decision/records/r.md#A1 |\n");
    }
    text
}

/// EX-core-522 の文書。英語の側だけが用語 "term" を使う
fn ir_pair_with_term(tmp: &Path, term_in_english: bool) {
    make_project(tmp, JA_EN, "");
    write(tmp, "docs/ir/a.md", &topic("REQ-001", "文。"));
    write(tmp, "docs/ir/a.en.md", &topic("REQ-001", "Uses `term`."));
    write(tmp, "docs/ir/CONTEXT.md", &glossary(!term_in_english));
    write(tmp, "docs/ir/CONTEXT.en.md", &glossary(term_in_english));
}

// @kotowari[REQ-core-342, EX-core-522]
#[test]
fn ex_core_522_the_english_side_is_not_counted_and_takes_terms_from_the_english_glossary() {
    let tmp = TempDir::new().unwrap();
    ir_pair_with_term(tmp.path(), true);
    let report = check(tmp.path());
    assert!(findings(&report, "duplicate_id").is_empty(), "{report}");
    assert!(findings(&report, "unknown_term").is_empty(), "{report}");
    let (_, list) = run(tmp.path(), &["list"]);
    let list: Value = serde_json::from_str(&list).unwrap();
    let items: Vec<(&str, &str)> = list["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| (item["id"].as_str().unwrap(), item["path"].as_str().unwrap()))
        .collect();
    assert_eq!(items, [("REQ-001", "docs/ir/a.md")]);
    // "files" と "lines" にはすべての側を数える
    let lines = |name: &str| {
        std::fs::read_to_string(tmp.path().join("docs/ir").join(name))
            .unwrap()
            .lines()
            .count()
    };
    let total: usize = ["a.md", "a.en.md", "CONTEXT.md", "CONTEXT.en.md"]
        .into_iter()
        .map(lines)
        .sum();
    assert_eq!(report["files"], 4);
    assert_eq!(report["lines"], total);
    let (_, status) = run(tmp.path(), &["status", "--format", "json"]);
    let status: Value = serde_json::from_str(&status).unwrap();
    assert_eq!(status["documents"]["files"], 4);
    assert_eq!(status["documents"]["lines"], total);
}

// @kotowari[EX-core-523]
#[test]
fn ex_core_523_a_term_only_in_the_japanese_glossary_is_unknown_on_the_english_side() {
    let tmp = TempDir::new().unwrap();
    ir_pair_with_term(tmp.path(), false);
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "unknown_term"),
        [pair("docs/ir/a.en.md", "term")]
    );
}

// @kotowari[REQ-core-342]
#[test]
fn req_core_342_text_and_glossary_checks_run_on_the_english_side_but_item_checks_do_not() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "docs/ir/a.md", &topic("REQ-001", "文。"));
    // 閉じないバッククォートと曖昧語は英語の側にも出る。ID の参照（unresolved_reference）は出ない
    write(
        tmp.path(),
        "docs/ir/a.en.md",
        &topic("REQ-001", "Uses `term and など and `REQ-999`."),
    );
    write(tmp.path(), "docs/ir/CONTEXT.md", &glossary(false));
    // 英語の用語集の題名と崩れた行
    write(
        tmp.path(),
        "docs/ir/CONTEXT.en.md",
        "# Terms\n\n| Term | Meaning | Source |\n|---|---|---|\n| broken |\n",
    );
    let report = check(tmp.path());
    let on = |kind: &str| {
        findings(&report, kind)
            .into_iter()
            .map(|(path, _, _)| path)
            .collect::<Vec<_>>()
    };
    assert_eq!(on("unclosed_backtick"), ["docs/ir/a.en.md"]);
    assert_eq!(on("vague_word"), ["docs/ir/a.en.md"]);
    assert_eq!(on("glossary_title_invalid"), ["docs/ir/CONTEXT.en.md"]);
    assert_eq!(on("invalid_glossary_row"), ["docs/ir/CONTEXT.en.md"]);
    assert!(on("unresolved_reference").is_empty(), "{report}");
}

// @kotowari[REQ-core-343, EX-core-524]
#[test]
fn ex_core_524_a_stale_guide_mark_is_reported_on_every_side() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, GUIDES);
    write(tmp.path(), "docs/ir/a.md", &topic("REQ-001", "文。"));
    write(tmp.path(), "docs/ir/a.en.md", &topic("REQ-001", "Text."));
    let guide = "# G\n\n<!-- @kotowari[REQ-001:00000000] -->\n";
    write(tmp.path(), "guides/a.md", guide);
    write(tmp.path(), "guides/a.en.md", guide);
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "guide_stale")
            .into_iter()
            .map(|(path, _, _)| path)
            .collect::<Vec<_>>(),
        ["guides/a.en.md", "guides/a.md"]
    );
}

const OVERVIEW: &str =
    "overview:\n  files:\n    - \".kotowari/overview/*.md\"\n  toc: .kotowari/toc.yaml\n";

/// 正しい全体像の元データ
fn overview_data(title: &str) -> String {
    format!(
        "---\nir:\n  - docs/ir/a.md\n---\n\n# {title}\n\n```view lead\nconclusion: {title}\n```\n\n## {title}\n\n{title}\n"
    )
}

/// IR の対と、全体像の元データの対と目次の対
fn overview_pairs(tmp: &Path) {
    make_project(tmp, JA_EN, OVERVIEW);
    write(tmp, "docs/ir/a.md", &topic("REQ-001", "文。"));
    write(tmp, "docs/ir/a.en.md", &topic("REQ-001", "Text."));
    write(tmp, ".kotowari/overview/a.md", &overview_data("題名"));
    write(tmp, ".kotowari/overview/a.en.md", &overview_data("Title"));
    write(tmp, ".kotowari/toc.yaml", "title: 目次\nitems:\n  - a\n");
    write(
        tmp,
        ".kotowari/toc.en.yaml",
        "title: Contents\nitems:\n  - a\n",
    );
}

// @kotowari[EX-core-525]
#[test]
fn ex_core_525_the_english_overview_side_is_not_another_page() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    let report = check(tmp.path());
    for kind in [
        "overview_name_conflict",
        "overview_toc_page_missing",
        "overview_ir_shared",
    ] {
        assert!(findings(&report, kind).is_empty(), "{kind}: {report}");
    }
}

// @kotowari[REQ-core-343]
#[test]
fn req_core_343_overview_and_contents_forms_are_checked_on_every_side() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    write(
        tmp.path(),
        ".kotowari/overview/a.en.md",
        "---\nir:\n  - docs/ir/a.md\n---\n\n# Title\n\n## Title\n\nText.\n",
    );
    write(tmp.path(), ".kotowari/toc.en.yaml", "items: []\n");
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "overview_lead_missing"),
        [pair(".kotowari/overview/a.en.md", "a.en.md")]
    );
    assert!(
        findings(&report, "overview_toc_invalid")
            .iter()
            .all(|(path, _, _)| path == ".kotowari/toc.en.yaml"),
        "{report}"
    );
    assert!(
        !findings(&report, "overview_toc_invalid").is_empty(),
        "{report}"
    );
    // 目次の空の群は目次の形の検査（REQ-core-330）で、目次のどの側にも出る
    write(
        tmp.path(),
        ".kotowari/toc.en.yaml",
        "title: Contents\nitems: []\n",
    );
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "overview_toc_group_empty"),
        [pair(".kotowari/toc.en.yaml", "(root)")]
    );
}
