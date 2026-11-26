//! 対の読み方、一致の記録、骨組み、切り替えの行、リンク（docs/decisions/core/translation-pairs.md、
//! docs/decisions/core/translation-structure.md）
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use serde_json::Value;
use std::path::Path;
use tempfile::TempDir;

/// 17の鍵と、日本語の`UI の文字`
const JAPANESE: [(&str, &str); 17] = [
    ("language_name", "日本語"),
    ("index_link", "一覧"),
    ("pages", "{n} ページ"),
    ("stale_sections", "見直していない節 {n}"),
    ("open_items", "未決 {n}"),
    ("planned_items", "予定 {n}"),
    ("stale_mark", "決定が変わった後、まだ見直していない節"),
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

/// 置き場と設定を作る。extra は設定に足す行
fn make_project(tmp: &Path, languages: Option<&[&str]>, extra: &str) {
    for dir in [".kotowari", "docs/decisions"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    let documents = match extra.contains("decisions:") {
        true => "",
        false => "decisions:\n  documents: docs/decisions\n",
    };
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        format!(
            "{documents}tests:\n  files:\n    - \"tests/**/*.rs\"\n{}{extra}",
            language_config(languages)
        ),
    )
    .unwrap();
    if extra.contains("decisions:") {
        decision_sides(tmp, languages);
    }
}

/// 全体像が扱う決定の文書 docs/decisions/a.md を、言語ごとの`側`と一致の記録とともに書く。
/// 決めたことは日本語の側が「文。」、ほかの側が「Text.」
fn decision_sides(tmp: &Path, languages: Option<&[&str]>) {
    let tags = languages.unwrap_or(&["en"]);
    let file = |index: usize| match index {
        0 => "a.md".to_string(),
        _ => format!("a.{}.md", tags[index]),
    };
    let name = |tag: &str| if tag == "en" { "English" } else { "日本語" };
    let mut record = String::new();
    for (index, tag) in tags.iter().enumerate() {
        let switcher: Vec<String> = tags
            .iter()
            .enumerate()
            .map(|(other, other_tag)| match other == index {
                true => name(other_tag).to_string(),
                false => format!("[{}]({})", name(other_tag), file(other)),
            })
            .collect();
        let switcher = match languages {
            Some(_) => format!("{}\n\n", switcher.join(" | ")),
            None => String::new(),
        };
        let body = if *tag == "ja" {
            "挨拶を決める。\n\n## 挨拶する\n\n状態: 提案\n\n### 問題\n\n困っていた。\n\n### 決めたこと\n\n文。\n\n### 確認すること\n\n- C-a-1 出る\n"
        } else {
            "Decides greetings.\n\n## Greet\n\nStatus: proposed\n\n### Problem\n\nTrouble.\n\n### Decision\n\nText.\n\n### Confirmations\n\n- C-a-1 Shown\n"
        };
        let text = format!("# A\n\n{switcher}{body}");
        record.push_str(&format!(
            "{}: {}\n",
            file(index),
            kotowari_core::translations::blob_hash(text.as_bytes())
        ));
        write(tmp, &format!("docs/decisions/{}", file(index)), &text);
    }
    if languages.is_some() {
        write(tmp, "docs/decisions/a.i18n.yaml", &record);
    }
}

const JA_EN: Option<&[&str]> = Some(&["ja", "en"]);
const GUIDES: &str = "guides:\n  files:\n    - \"guides/**/*.md\"\n";

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// 決定 "Name" を1つ持つ決定の文書。決めたことは `statement`
fn decision_topic(statement: &str) -> String {
    format!(
        "# A\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\n{statement}\n"
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

/// ガイドの対と一致の記録
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

// @kotowari[C-core-pairs-1]
#[test]
fn with_one_language_a_suffixed_file_is_a_document_of_its_own() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), None, "");
    write(tmp.path(), "docs/decisions/a.md", &decision_topic("Body."));
    write(
        tmp.path(),
        "docs/decisions/a.en.md",
        &decision_topic("Body."),
    );
    let report = check(tmp.path());
    assert!(
        findings(&report, "translation_missing").is_empty(),
        "{report}"
    );
    assert_eq!(report["decisions"]["documents"], 2);
    let (_, list) = run(tmp.path(), &["list"]);
    let list: Value = serde_json::from_str(&list).unwrap();
    let paths: Vec<&str> = list["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item["path"].as_str().unwrap())
        .collect();
    assert!(paths.contains(&"docs/decisions/a.en.md"), "{list}");
    assert!(list.get("translations").is_none(), "{list}");
}

// @kotowari[C-core-pairs-4]
#[test]
fn a_missing_english_side_is_an_error_on_the_first_side() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "docs/decisions/a.md", &decision_topic("文。"));
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_missing"),
        [pair("docs/decisions/a.md", "docs/decisions/a.en.md")]
    );
    let missing = findings(&report, "translation_missing");
    assert!(missing[0].1.is_null(), "{report}");
}

// @kotowari[C-core-pairs-5]
#[test]
fn a_missing_first_side_is_an_error_on_the_side_that_exists() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(
        tmp.path(),
        "docs/decisions/b.en.md",
        &decision_topic("Text."),
    );
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_missing"),
        [pair("docs/decisions/b.en.md", "docs/decisions/b.md")]
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

// @kotowari[C-core-pairs-6]
#[test]
fn a_suffix_of_the_first_language_or_an_unlisted_one_is_part_of_the_name() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(
        tmp.path(),
        "docs/decisions/c.ja.md",
        &decision_topic("文。"),
    );
    write(
        tmp.path(),
        "docs/decisions/c.fr.md",
        &decision_topic("文。"),
    );
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_missing"),
        [
            pair("docs/decisions/c.fr.md", "docs/decisions/c.fr.en.md"),
            pair("docs/decisions/c.ja.md", "docs/decisions/c.ja.en.md"),
        ]
    );
}

// @kotowari[C-core-pairs-7]
#[test]
fn a_record_matching_the_files_raises_nothing() {
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

// @kotowari[C-core-pairs-11]
#[test]
fn changing_one_side_makes_it_stale() {
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

// @kotowari[C-core-pairs-8]
#[test]
fn record_keys_other_than_the_side_names_are_invalid() {
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

// @kotowari[C-core-pairs-8]
#[test]
fn a_missing_record_is_invalid() {
    let tmp = TempDir::new().unwrap();
    guide_pair_with_record(tmp.path(), None);
    let report = check(tmp.path());
    assert_eq!(
        paths_and_details(&report, "translation_record_invalid"),
        [pair("guides/a.i18n.yaml", "missing")]
    );
}

// @kotowari[C-core-pairs-8]
#[test]
fn an_unreadable_record_or_a_malformed_value_is_named_in_order() {
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

// @kotowari[C-core-pairs-12]
#[test]
fn a_stale_guide_mark_is_reported_on_every_side() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, GUIDES);
    write(tmp.path(), "docs/decisions/a.md", &decision_topic("文。"));
    write(
        tmp.path(),
        "docs/decisions/a.en.md",
        &decision_topic("Text."),
    );
    let guide = "# G\n\n<!-- @kotowari[docs/decisions/a.md#name:00000000] -->\n";
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

const OVERVIEW: &str = "decisions:\n  documents: docs/decisions\n\
    overview:\n  files:\n    - \".kotowari/overview/*.md\"\n  toc: .kotowari/toc.yaml\n";

/// 正しい全体像の元データ
fn overview_data(title: &str) -> String {
    format!(
        "---\ndocuments:\n  - docs/decisions/a.md\n---\n\n# {title}\n\n```view lead\nconclusion: {title}\n```\n\n## {title}\n\n{title}\n"
    )
}

/// 決定の文書の対と、全体像の元データの対と目次の対
fn overview_pairs(tmp: &Path) {
    make_project(tmp, JA_EN, OVERVIEW);
    write(tmp, ".kotowari/overview/a.md", &overview_data("題名"));
    write(tmp, ".kotowari/overview/a.en.md", &overview_data("Title"));
    write(tmp, ".kotowari/toc.yaml", "title: 目次\nitems:\n  - a\n");
    write(
        tmp,
        ".kotowari/toc.en.yaml",
        "title: Contents\nitems:\n  - a\n",
    );
}

// @kotowari[C-core-ovdata-4]
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

/// 言語の一覧を JA_EN にし、設定に extra を足して、各コマンドの (終了コード, 標準エラーの1行目) を返す
fn stops(tmp: &Path, extra: &str, commands: &[&[&str]]) -> Vec<(Option<i32>, String)> {
    write(
        tmp,
        ".kotowari/config.yaml",
        &format!("{}{extra}", language_config(JA_EN)),
    );
    commands
        .iter()
        .map(|args| {
            let output = assert_cmd::Command::cargo_bin("kotowari")
                .unwrap()
                .args(*args)
                .current_dir(tmp)
                .output()
                .unwrap();
            let stderr = String::from_utf8_lossy(&output.stderr);
            (
                output.status.code(),
                stderr.lines().next().unwrap_or("").to_string(),
            )
        })
        .collect()
}

const CHECK_STATUS_BUILD: [&[&str]; 3] = [&["check"], &["status"], &["overview", "build"]];

// @kotowari[C-core-guides-9, C-core-guides-24, C-core-ovdata-21]
#[test]
fn a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "shared/a.en.md", &overview_data("Title"));
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: 目次\nitems:\n  - a\n",
    );
    let stopped = stops(
        tmp.path(),
        "tests:\n  files: ['shared/**']\nguides:\n  files: ['shared/**']\noverview:\n  files: ['shared/**']\n  toc: .kotowari/toc.yaml\n",
        &CHECK_STATUS_BUILD,
    );
    let expected = (
        Some(2),
        "config error: shared/a.en.md: matched by both guides.files and tests.files".to_string(),
    );
    assert_eq!(stopped, [expected.clone(), expected.clone(), expected]);
}

// @kotowari[C-core-ovdata-3, C-core-ovdata-22]
#[test]
fn a_side_without_its_first_side_matched_by_overview_and_guides_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "shared/a.en.md", &overview_data("Title"));
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: 目次\nitems:\n  - a\n",
    );
    let stopped = stops(
        tmp.path(),
        "guides:\n  files: ['shared/**']\noverview:\n  files: ['shared/**']\n  toc: .kotowari/toc.yaml\n",
        &CHECK_STATUS_BUILD,
    );
    let expected = (
        Some(2),
        "config error: shared/a.en.md: matched by both overview.files and guides.files".to_string(),
    );
    assert_eq!(stopped, [expected.clone(), expected.clone(), expected]);
}

// @kotowari[C-core-guides-9, C-core-guides-23]
#[test]
fn a_side_found_by_name_counts_as_a_guide_in_the_overlap() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), JA_EN, "");
    write(tmp.path(), "guides/a.md", "a\n");
    write(tmp.path(), "guides/a.en.md", "a\n");
    let stopped = stops(
        tmp.path(),
        "tests:\n  files: ['guides/*.en.md']\nguides:\n  files: ['guides/a.md']\n",
        &[&["check"]],
    );
    assert_eq!(
        stopped,
        [(
            Some(2),
            "config error: guides/a.en.md: matched by both guides.files and tests.files"
                .to_string()
        )]
    );
}

// @kotowari[C-core-ovlang-2, C-core-ovlang-4]
#[test]
fn three_languages_keep_their_own_html_language_and_contents() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some(&["en", "ja", "fr"]), OVERVIEW);
    for (suffix, title) in [("", "Contents"), (".ja", "目次"), (".fr", "Sommaire")] {
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

// @kotowari[C-core-ovcmd-5]
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

// @kotowari[C-core-ovcmd-7]
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
            "{}tests:\n  files: []\ndecisions:\n  documents: docs/decisions\noverview:\n  files: ['notes/*.md']\n  toc: .kotowari/toc.yaml\n",
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

// @kotowari[C-core-pairs-13]
#[test]
fn the_english_overview_side_is_not_another_page() {
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

// @kotowari[C-core-pairs-13]
#[test]
fn overview_and_contents_forms_are_checked_on_every_side() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    write(
        tmp.path(),
        ".kotowari/overview/a.en.md",
        "---\ndocuments:\n  - docs/decisions/a.md\n---\n\n# Title\n\n## Title\n\nText.\n",
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
    // 目次の空の群は目次の形の検査で、目次のどの側にも出る
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

/// status の部品を持つ全体像の元データ
fn overview_with_status(title: &str, state: &str, text: &str) -> String {
    format!(
        "---\ndocuments:\n  - docs/decisions/a.md\n---\n\n# {title}\n\n```view lead\nconclusion: {title}\n```\n\n## {title}\n\n```view status\nitems:\n  - state: {state}\n    text: {text}\n```\n"
    )
}

// @kotowari[C-core-pairstruct-3]
#[test]
fn a_part_field_that_is_not_a_sentence_must_match() {
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

// @kotowari[C-core-pairstruct-4]
#[test]
fn contents_that_differ_only_in_titles_and_notes_match() {
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

// @kotowari[C-core-pairstruct-6]
#[test]
fn correct_switcher_lines_raise_nothing() {
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

// @kotowari[C-core-pairstruct-7]
#[test]
fn a_missing_switcher_line_reports_the_line_it_should_be() {
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

// @kotowari[C-core-pairstruct-8]
#[test]
fn overview_data_has_no_switcher_line() {
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

/// 決定の文書の対と、英語のガイド
fn guide_linking(tmp: &Path, english: &str) {
    make_project(tmp, JA_EN, GUIDES);
    write(tmp, "docs/decisions/a.md", &decision_topic("文。"));
    write(tmp, "docs/decisions/a.en.md", &decision_topic("Text."));
    write(
        tmp,
        "guides/g.md",
        "# G\n\n日本語 | [English](g.en.md)\n\n本文。\n",
    );
    write(tmp, "guides/g.en.md", english);
}

// @kotowari[C-core-pairstruct-14]
#[test]
fn an_english_guide_linking_the_japanese_decision_document_is_an_error() {
    let tmp = TempDir::new().unwrap();
    guide_linking(
        tmp.path(),
        "# G\n\n[日本語](g.md) | English\n\nSee [a](../docs/decisions/a.md#name).\n",
    );
    let report = check(tmp.path());
    assert_eq!(
        findings(&report, "link_language_mismatch"),
        [(
            "guides/g.en.md".to_string(),
            Value::from(5),
            "../docs/decisions/a.md#name".to_string()
        )]
    );
}

// @kotowari[C-core-pairstruct-12]
#[test]
fn links_to_the_same_language_outside_or_inside_the_page_are_not_errors() {
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

/// 参照を持つ steps の部品を持つ全体像の元データ
fn overview_with_refs(title: &str, refs: &str) -> String {
    format!(
        "---\ndocuments:\n  - docs/decisions/a.md\n---\n\n# {title}\n\n```view lead\nconclusion: {title}\n```\n\n## {title}\n\n```view steps\nitems:\n  - title: {title}\n    refs: [{refs}]\n```\n"
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

// @kotowari[C-core-ovlang-6]
#[test]
fn english_is_the_built_in_text_with_the_labels_on_top() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        None,
        &format!("{OVERVIEW}labels:\n  en:\n    pages: \"{{n}} docs\"\n"),
    );
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

// @kotowari[C-core-ovlang-1, C-core-ovlang-5]
#[test]
fn english_pages_are_written_under_en_and_link_each_other() {
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

// @kotowari[C-core-ovlang-3, C-core-ovlang-7]
#[test]
fn one_language_writes_no_language_directory() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), Some(&["ja"]), OVERVIEW);
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

// @kotowari[C-core-ovlang-11]
#[test]
fn english_pages_take_reference_bodies_from_the_english_side_of_the_decision_document() {
    let tmp = TempDir::new().unwrap();
    overview_pairs(tmp.path());
    let refs = "\"docs/decisions/a.md#挨拶する\", C-a-1";
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
    for (page, own, other) in [
        (
            &english,
            ["Greet", "Text.", "C-a-1 Shown"],
            ["挨拶する", "文。", "出る"],
        ),
        (
            &root,
            ["挨拶する", "文。", "C-a-1 出る"],
            ["Greet", "Text.", "Shown"],
        ),
    ] {
        for text in own {
            assert!(page.contains(text), "{text}: {page}");
        }
        for text in other {
            assert!(!page.contains(text), "{text}: {page}");
        }
    }
}

// @kotowari[C-core-ovcmd-26]
#[test]
fn a_missing_side_stops_the_build_without_writing() {
    let tmp = TempDir::new().unwrap();
    // 目次の骨組みが食い違うと止まる
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

// @kotowari[C-core-pairstruct-5]
#[test]
fn overview_links_to_the_matching_guide_sides_do_not_stop_the_build() {
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
