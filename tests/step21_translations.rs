//! 対の読み方、一致の記録、骨組み、切り替えの行、リンク（docs/ir/core/translation-pairs.md、
//! docs/ir/core/translation-structure.md）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

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

// @kotowari[REQ-core-280, REQ-core-337]
#[test]
fn translated_overview_files_cannot_also_be_test_files() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        &format!(
            "{}tests:\n  files: ['overview/*.en.md']\noverview:\n  files: ['overview/*.md']\n  toc: .kotowari/toc.yaml\n",
            language_config(JA_EN)
        ),
    );
    write(tmp.path(), "overview/a.md", &overview_data("題名"));
    write(tmp.path(), "overview/a.en.md", &overview_data("Title"));
    for args in [vec!["check"], vec!["status"], vec!["overview", "build"]] {
        let output = assert_cmd::Command::cargo_bin("kotowari")
            .unwrap()
            .args(args)
            .current_dir(tmp.path())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert_eq!(
            String::from_utf8_lossy(&output.stderr).lines().next(),
            Some("config error: overview/a.en.md: matched by both overview.files and tests.files"),
            "{output:?}"
        );
    }
    assert!(!tmp.path().join(".kotowari/cache/overview").exists());
}

// @kotowari[REQ-core-288, REQ-core-293, REQ-core-353, REQ-core-355]
#[test]
fn three_languages_keep_their_own_html_language_and_contents() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some(&["en", "ja", "fr"]), OVERVIEW);
    for (suffix, title) in [("", "Contents"), (".ja", "目次"), (".fr", "Sommaire")] {
        write(
            tmp.path(),
            &format!("docs/ir/a{suffix}.md"),
            &topic("REQ-001", "Body."),
        );
        write(
            tmp.path(),
            &format!(".kotowari/overview/a{suffix}.md"),
            &overview_data(title),
        );
        write(
            tmp.path(),
            &format!(".kotowari/toc{suffix}.yaml"),
            &format!("title: {title}\nitems:\n  - a\n"),
        );
    }
    assert_eq!(check(tmp.path())["overview"]["files"], 3);
    let (code, output) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{output}");
    for (place, language, title) in [
        ("", "en", "Contents"),
        ("ja/", "ja", "目次"),
        ("fr/", "fr", "Sommaire"),
    ] {
        let index = built(tmp.path(), &format!("{place}index.html"));
        assert!(
            index.contains(&format!("<html lang=\"{language}\">")),
            "{index}"
        );
        assert!(index.contains(&format!("<h1>{title}</h1>")), "{index}");
        assert!(
            links(&index).iter().any(|(path, _)| path == "a.html"),
            "{index}"
        );
    }
    assert!(
        links(&built(tmp.path(), "fr/a.html"))
            .iter()
            .any(|(path, _)| path == "../ja/a.html")
    );
    let (code, output) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{output}");
    let report: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(report["unchanged"], 9);
    assert_eq!(report["written"], serde_json::json!([]));
}

// @kotowari[REQ-core-296, REQ-core-324, REQ-core-353]
#[cfg(unix)]
#[test]
fn a_linked_language_directory_stops_before_any_page_is_written() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    let outside = TempDir::new().unwrap();
    write(tmp.path(), ".kotowari/cache/overview/a.html", "old");
    write(outside.path(), "keep", "outside");
    std::os::unix::fs::symlink(
        outside.path(),
        tmp.path().join(".kotowari/cache/overview/en"),
    )
    .unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(["overview", "build"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .starts_with("cache error: .kotowari/cache/overview/en"),
        "{output:?}"
    );
    assert_eq!(built(tmp.path(), "a.html"), "old");
    assert!(
        !tmp.path()
            .join(".kotowari/cache/overview/index.html")
            .exists()
    );
    assert_eq!(
        std::fs::read_to_string(outside.path().join("keep")).unwrap(),
        "outside"
    );
    assert_eq!(std::fs::read_dir(outside.path()).unwrap().count(), 1);
}

// @kotowari[REQ-core-324, REQ-core-353]
#[cfg(unix)]
#[test]
fn an_inaccessible_language_cache_reports_a_cache_error() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        &format!(
            "{}tests:\n  files: []\noverview:\n  files: ['notes/*.md']\n  toc: .kotowari/toc.yaml\n",
            language_config(JA_EN)
        ),
    );
    write(tmp.path(), "notes/a.md", &overview_data("題名"));
    write(tmp.path(), "notes/a.en.md", &overview_data("Title"));
    let cache = tmp.path().join(".kotowari/cache/overview");
    std::fs::create_dir_all(cache.join("en")).unwrap();
    std::fs::set_permissions(&cache, std::fs::Permissions::from_mode(0o000)).unwrap();
    let denied = std::fs::symlink_metadata(cache.join("en")).is_err();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(["overview", "build"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    std::fs::set_permissions(&cache, std::fs::Permissions::from_mode(0o700)).unwrap();
    if denied {
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .starts_with("cache error: .kotowari/cache/overview/en"),
            "{output:?}"
        );
        assert!(!cache.join("index.html").exists());
    }
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

/// 要求と具体例を持つ話題ごとの文書。"- source:" は9行目にある
fn topic_with_example(source: &str, statement: &str, step: &str) -> String {
    format!(
        "# A\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- source: {source}\n- verification: review\n- how_to_verify: read\n\n{statement}\n\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001 @source=docs/decision/records/r.md#A1\nScenario: {step}\n  Given {step}\n```\n"
    )
}

const RECORD_A1: &str = "docs/decision/records/r.md#A1";

// @kotowari[REQ-core-344, EX-core-526]
#[test]
fn ex_core_526_sides_that_differ_only_in_sentences_match() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(
        tmp.path(),
        "docs/ir/a.md",
        &topic_with_example(RECORD_A1, "文。", "例"),
    );
    write(
        tmp.path(),
        "docs/ir/a.en.md",
        &topic_with_example(RECORD_A1, "Text.", "example"),
    );
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_structure_mismatch").is_empty(),
        "{report}"
    );
}

// @kotowari[REQ-core-345, EX-core-527]
#[test]
fn ex_core_527_a_different_source_is_reported_on_its_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(
        tmp.path(),
        "docs/ir/a.md",
        &topic_with_example(RECORD_A1, "文。", "例"),
    );
    write(
        tmp.path(),
        "docs/ir/a.en.md",
        &topic_with_example("docs/decision/records/r.md#A2", "Text.", "example"),
    );
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "translation_structure_mismatch"),
        [(
            "docs/ir/a.en.md".to_string(),
            Value::from(9),
            "field".to_string()
        )]
    );
}

/// status の部品を持つ全体像の元データ
fn overview_with_status(title: &str, state: &str, text: &str) -> String {
    format!(
        "---\nir:\n  - docs/ir/a.md\n---\n\n# {title}\n\n```view lead\nconclusion: {title}\n```\n\n## {title}\n\n```view status\nitems:\n  - state: {state}\n    text: {text}\n```\n"
    )
}

// @kotowari[TBL-core-045, EX-core-528]
#[test]
fn ex_core_528_a_part_field_that_is_not_a_sentence_must_match() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &overview_with_status("題名", "open", "未決の文"),
    );
    write(
        tmp.path(),
        ".kotowari/overview/a.en.md",
        &overview_with_status("Title", "open", "Open text"),
    );
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_structure_mismatch").is_empty(),
        "{report}"
    );
    write(
        tmp.path(),
        ".kotowari/overview/a.en.md",
        &overview_with_status("Title", "decided", "Decided text"),
    );
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_structure_mismatch"),
        [pair(".kotowari/overview/a.en.md", "part")]
    );
}

// @kotowari[EX-core-529]
#[test]
fn ex_core_529_contents_that_differ_only_in_titles_and_notes_match() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: 目次\nnote: 説明\nitems:\n  - title: 群\n    items:\n      - a\n",
    );
    write(
        tmp.path(),
        ".kotowari/toc.en.yaml",
        "title: Contents\nnote: About\nitems:\n  - title: Group\n    items:\n      - a\n",
    );
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_structure_mismatch").is_empty(),
        "{report}"
    );
    // 名前の項目の入れ子が違えば食い違う
    write(
        tmp.path(),
        ".kotowari/toc.en.yaml",
        "title: Contents\nnote: About\nitems:\n  - a\n",
    );
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_structure_mismatch"),
        [pair(".kotowari/toc.en.yaml", "toc")]
    );
}

// @kotowari[REQ-core-346, EX-core-530]
#[test]
fn ex_core_530_correct_switcher_lines_raise_nothing() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, GUIDES);
    write(
        tmp.path(),
        "guides/a.md",
        "# A\n\n日本語 | [English](a.en.md)\n\n本文。\n",
    );
    write(
        tmp.path(),
        "guides/a.en.md",
        "# A\n\n[日本語](a.md) | English\n\nText.\n",
    );
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_switcher_invalid").is_empty(),
        "{report}"
    );
}

// @kotowari[EX-core-531]
#[test]
fn ex_core_531_a_missing_switcher_line_reports_the_line_it_should_be() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, GUIDES);
    write(tmp.path(), "guides/b.md", "# B\n\n本文。\n");
    write(
        tmp.path(),
        "guides/b.en.md",
        "# B\n\n[日本語](b.md) | English\n",
    );
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "translation_switcher_invalid"),
        [(
            "guides/b.md".to_string(),
            Value::from(3),
            "日本語 | [English](b.en.md)".to_string()
        )]
    );
}

// @kotowari[EX-core-532]
#[test]
fn ex_core_532_overview_data_has_no_switcher_line() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_switcher_invalid")
            .iter()
            .all(|(path, _, _)| !path.starts_with(".kotowari/")),
        "{report}"
    );
}

// @kotowari[REQ-core-346]
#[test]
fn req_core_346_a_switcher_line_is_not_the_scope_of_an_ir_document() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    let with = |switcher: &str, scope: &str| {
        topic("REQ-001", "文。").replace("# A\n\nScope.\n", &format!("# A\n\n{switcher}\n{scope}"))
    };
    write(
        tmp.path(),
        "docs/ir/a.md",
        &with("日本語 | [English](a.en.md)", "\n範囲。\n"),
    );
    write(
        tmp.path(),
        "docs/ir/a.en.md",
        &with("[日本語](a.md) | English", "\nScope.\n"),
    );
    let report = check(tmp.path());
    assert!(findings(&report, "missing_scope").is_empty(), "{report}");
    assert!(
        findings(&report, "translation_switcher_invalid").is_empty(),
        "{report}"
    );
    // 切り替えの行だけでは文書が扱う範囲にならない
    write(
        tmp.path(),
        "docs/ir/a.md",
        &with("日本語 | [English](a.en.md)", ""),
    );
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "missing_scope"),
        [pair("docs/ir/a.md", "a.md")]
    );
}

/// EX-core-533 の IR の対と、英語のガイド
fn guide_linking(tmp: &Path, english: &str) {
    make_project(tmp, JA_EN, GUIDES);
    write(tmp, "docs/ir/a.md", &topic("REQ-001", "文。"));
    write(tmp, "docs/ir/a.en.md", &topic("REQ-001", "Text."));
    write(
        tmp,
        "guides/g.md",
        "# G\n\n日本語 | [English](g.en.md)\n\n本文。\n",
    );
    write(tmp, "guides/g.en.md", english);
}

// @kotowari[REQ-core-347, REQ-core-348, EX-core-533]
#[test]
fn ex_core_533_an_english_guide_linking_the_japanese_ir_is_an_error() {
    let tmp = TempDir::new().unwrap();
    guide_linking(
        tmp.path(),
        "# G\n\n[日本語](g.md) | English\n\nSee [a](../docs/ir/a.md#REQ-001).\n",
    );
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "link_language_mismatch"),
        [(
            "guides/g.en.md".to_string(),
            Value::from(5),
            "../docs/ir/a.md#REQ-001".to_string()
        )]
    );
}

// @kotowari[EX-core-534]
#[test]
fn ex_core_534_links_to_the_same_language_outside_or_inside_the_page_are_not_errors() {
    let tmp = TempDir::new().unwrap();
    guide_linking(
        tmp.path(),
        "# G\n\n[日本語](g.md) | English\n\nSee [a](../docs/ir/a.en.md#other), [b](https://example.com/a.md) and [c](#top).\n",
    );
    let report = check(tmp.path());
    assert!(
        findings(&report, "link_language_mismatch").is_empty(),
        "{report}"
    );
}

// @kotowari[REQ-core-349, EX-core-535]
#[test]
fn ex_core_535_a_link_to_a_decision_record_is_an_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, GUIDES);
    write(
        tmp.path(),
        "guides/g.md",
        "# G\n\n[r](../docs/decision/records/r.md#A1)\n",
    );
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "link_to_record")
            .into_iter()
            .map(|(path, line, _)| (path, line))
            .collect::<Vec<_>>(),
        [("guides/g.md".to_string(), Value::from(3))]
    );
}

/// 参照を持つ steps の部品を持つ全体像の元データ
fn overview_with_refs(title: &str, refs: &str) -> String {
    format!(
        "---\nir:\n  - docs/ir/a.md\n---\n\n# {title}\n\n```view lead\nconclusion: {title}\n```\n\n## {title}\n\n```view steps\nitems:\n  - title: {title}\n    refs: [{refs}]\n```\n"
    )
}

/// 全体像のページ。build が書いたファイルの中身
fn built(tmp: &Path, name: &str) -> String {
    std::fs::read_to_string(tmp.join(".kotowari/cache/overview").join(name))
        .unwrap_or_else(|error| panic!("{name}: {error}"))
}

/// HTML の中のリンクの (リンク先, 文字)
fn links(html: &str) -> Vec<(String, String)> {
    html.split("<a ")
        .skip(1)
        .map(|link| {
            let (tag, rest) = link.split_once('>').unwrap();
            let href = tag
                .split_once("href=\"")
                .and_then(|(_, value)| value.split_once('"'))
                .map(|(value, _)| value.to_string())
                .unwrap_or_default();
            let text = &rest[..rest.find("</a>").unwrap()];
            (href, text.to_string())
        })
        .collect()
}

// @kotowari[REQ-core-351, EX-core-536]
#[test]
fn ex_core_536_english_is_the_built_in_text_with_the_labels_on_top() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        None,
        &format!("{OVERVIEW}labels:\n  en:\n    pages: \"{{n}} docs\"\n"),
    );
    write(tmp.path(), "docs/ir/a.md", &topic("REQ-001", "Text."));
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &overview_with_status("Title", "open", "Open text"),
    );
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: Contents\nitems:\n  - a\n",
    );
    let (code, stdout) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{stdout}");
    let index = built(tmp.path(), "index.html");
    assert!(index.contains("1 docs"), "{index}");
    assert!(!index.contains("1 pages"), "{index}");
    assert!(index.contains("1 open"), "{index}");
    assert!(built(tmp.path(), "a.html").contains(">Open<"));
}

// @kotowari[REQ-core-353, REQ-core-355, EX-core-540]
#[test]
fn ex_core_540_english_pages_are_written_under_en_and_link_each_other() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    let (code, stdout) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{stdout}");
    let root = built(tmp.path(), "a.html");
    let english = built(tmp.path(), "en/a.html");
    assert!(
        links(&root).contains(&("en/a.html".into(), "English".into())),
        "{root}"
    );
    assert!(
        links(&english).contains(&("../a.html".into(), "日本語".into())),
        "{english}"
    );
    assert!(
        english.contains("Title") && !english.contains("題名"),
        "{english}"
    );
    assert!(root.contains("題名"), "{root}");
    assert!(
        links(&built(tmp.path(), "en/index.html"))
            .contains(&("../index.html".into(), "日本語".into()))
    );
}

// @kotowari[EX-core-541]
#[test]
fn ex_core_541_one_language_writes_no_language_directory() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some(&["ja"]), OVERVIEW);
    write(tmp.path(), "docs/ir/a.md", &topic("REQ-001", "文。"));
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &overview_data("題名"),
    );
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: 目次\nitems:\n  - a\n",
    );
    let (code, stdout) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{stdout}");
    assert!(!tmp.path().join(".kotowari/cache/overview/ja").exists());
    let index = built(tmp.path(), "index.html");
    // 一覧は日本語の UI の文字で描かれる
    assert!(index.contains("1 ページ"), "{index}");
}

// @kotowari[REQ-core-354, EX-core-542]
#[test]
fn ex_core_542_english_pages_take_ir_bodies_from_the_english_side_and_no_record_bodies() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    let refs = "REQ-001, docs/decision/records/r.md#A1";
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &overview_with_refs("題名", refs),
    );
    write(
        tmp.path(),
        ".kotowari/overview/a.en.md",
        &overview_with_refs("Title", refs),
    );
    let (code, stdout) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{stdout}");
    let root = built(tmp.path(), "a.html");
    let english = built(tmp.path(), "en/a.html");
    assert!(
        english.contains("Text.") && !english.contains("文。"),
        "{english}"
    );
    assert!(
        english.contains("r A1") && !english.contains("Agreement"),
        "{english}"
    );
    assert!(root.contains("文。") && !root.contains("Text."), "{root}");
    assert!(
        root.contains("r A1") && root.contains("Agreement"),
        "{root}"
    );
}

// @kotowari[REQ-core-294]
#[test]
fn req_core_294_a_missing_side_stops_the_build_without_writing() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    std::fs::remove_file(tmp.path().join("docs/ir/a.en.md")).unwrap();
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(["overview", "build"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("errors in overview data; run kotowari check"),
        "{output:?}"
    );
    assert!(!tmp.path().join(".kotowari/cache/overview").exists());
    // 骨組みの食い違いでも止まる
    overview_pairs(tmp.path());
    write(
        tmp.path(),
        ".kotowari/toc.en.yaml",
        "title: Contents\nitems:\n  - title: G\n    items:\n      - a\n",
    );
    let (code, _) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(2));
    assert!(!tmp.path().join(".kotowari/cache/overview").exists());
}

// @kotowari[REQ-core-336, EX-core-517]
#[test]
fn req_core_336_a_guide_glob_over_the_record_places_does_not_pair_the_records() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        JA_EN,
        "guides:\n  files:\n    - \"docs/**/*.md\"\n",
    );
    write(
        tmp.path(),
        "docs/decision/adr/0001-a.md",
        "# ADR\n\nText.\n",
    );
    write(
        tmp.path(),
        "docs/guides/g.md",
        "# G\n\n日本語 | [English](g.en.md)\n\n本文。\n",
    );
    write(
        tmp.path(),
        "docs/guides/g.en.md",
        "# G\n\n[日本語](g.md) | English\n\nText.\n",
    );
    let report = check(tmp.path());
    let on_records: Vec<&Value> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| {
            let kind = finding["kind"].as_str().unwrap();
            (kind.starts_with("translation_") || kind.starts_with("link_"))
                && finding["path"]
                    .as_str()
                    .unwrap()
                    .starts_with("docs/decision/")
        })
        .collect();
    assert!(on_records.is_empty(), "{report}");
    let (_, list) = run(tmp.path(), &["list"]);
    let list: Value = serde_json::from_str(&list).unwrap();
    let paired: Vec<&str> = list["translations"]
        .as_array()
        .unwrap_or_else(|| panic!("{list}"))
        .iter()
        .map(|translation| translation["path"].as_str().unwrap())
        .collect();
    assert_eq!(paired, ["docs/guides/g.md"], "{list}");
}

// @kotowari[REQ-core-294, TBL-core-044]
#[test]
fn req_core_294_overview_links_to_the_matching_guide_sides_do_not_stop_the_build() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    let config = std::fs::read_to_string(tmp.path().join(".kotowari/config.yaml")).unwrap();
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        &format!("{config}{GUIDES}"),
    );
    let linking = |title: &str, guide: &str| {
        format!("{}\nSee [g](../../guides/{guide}).\n", overview_data(title))
    };
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &linking("題名", "g.md"),
    );
    write(
        tmp.path(),
        ".kotowari/overview/a.en.md",
        &linking("Title", "g.en.md"),
    );
    write(
        tmp.path(),
        "guides/g.md",
        "# G\n\n日本語 | [English](g.en.md)\n\n本文。\n",
    );
    write(
        tmp.path(),
        "guides/g.en.md",
        "# G\n\n[日本語](g.md) | English\n\nText.\n",
    );
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_structure_mismatch").is_empty(),
        "{report}"
    );
    let (code, stdout) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{stdout}");
    assert!(
        tmp.path()
            .join(".kotowari/cache/overview/en/a.html")
            .is_file()
    );
}
