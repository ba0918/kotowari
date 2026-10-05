//! 言語の一覧、UI の文字、対の読み方（docs/ir/core/translation-pairs.md、docs/ir/core/overview-languages.md）

use crate::config::Config;

/// TBL-core-046 の鍵と英語の文字
const ENGLISH: [(&str, &str); 17] = [
    ("language_name", "English"),
    ("index_link", "Overview"),
    ("pages", "{n} pages"),
    ("stale_sections", "{n} sections to review"),
    ("open_items", "{n} open"),
    ("planned_items", "{n} planned"),
    ("stale_mark", "Not reviewed since the IR changed"),
    ("outline_stale", "not reviewed"),
    ("superseded", "(superseded)"),
    ("deferred", "(deferred)"),
    ("compare_before", "Before"),
    ("compare_after", "After"),
    ("compare_why", "Why"),
    ("state_decided", "Decided"),
    ("state_planned", "Planned"),
    ("state_open", "Open"),
    ("state_dropped", "Dropped"),
];

/// 17の鍵すべてに、鍵の名前に prefix を付けた値（数を入れる鍵は " {n}" を足す）を持つ labels の1言語分
fn complete_labels(tag: &str, prefix: &str) -> String {
    let mut out = format!("  {tag}:\n");
    for (key, english) in ENGLISH {
        let count = if english.contains("{n}") { " {n}" } else { "" };
        out.push_str(&format!("    {key}: \"{prefix}{key}{count}\"\n"));
    }
    out
}

// @kotowari[REQ-core-351, TBL-core-046]
#[test]
fn req_core_351_english_is_the_built_in_text_with_overrides_and_others_are_the_labels() {
    let yaml = format!(
        "languages: [ja, en]\nlabels:\n  en:\n    pages: \"{{n}} docs\"\n{}",
        complete_labels("ja", "J")
    );
    let config = Config::parse(&yaml).unwrap();
    let english = config.ui_text("en");
    for (key, text) in ENGLISH {
        let expected = if key == "pages" { "{n} docs" } else { text };
        assert_eq!(english.get(key), Some(expected), "{key}");
    }
    let japanese = config.ui_text("ja");
    for (key, text) in ENGLISH {
        let count = if text.contains("{n}") { " {n}" } else { "" };
        assert_eq!(japanese.get(key), Some(format!("J{key}{count}").as_str()));
    }
    assert_eq!(japanese.get("unknown"), None);
}

// @kotowari[REQ-core-334]
#[test]
fn req_core_334_the_language_list_is_the_languages_key_and_english_when_absent_or_empty() {
    let ja = complete_labels("ja", "J");
    let config = Config::parse(&format!("languages: [ja, en]\nlabels:\n{ja}")).unwrap();
    assert_eq!(config.languages(), ["ja", "en"]);
    assert_eq!(Config::parse("ir: docs/ir\n").unwrap().languages(), ["en"]);
    assert_eq!(
        Config::parse("languages: []\n").unwrap().languages(),
        ["en"]
    );
    // validated は読んだ値をそのまま保つ
    assert_eq!(config.validated().unwrap(), config);
}

// @kotowari[REQ-core-335]
#[test]
fn req_core_335_an_empty_malformed_or_repeated_tag_stops() {
    let ja = complete_labels("ja", "J");
    for list in [
        "[ja, \"\"]",
        "[ja, en_us]",
        "[ja, \"e n\"]",
        "[ja, ja]",
        "[en, en]",
    ] {
        assert!(
            matches!(
                Config::parse(&format!("languages: {list}\nlabels:\n{ja}")),
                Err(crate::StopReason::ConfigError(_))
            ),
            "{list} should stop"
        );
    }
    assert!(
        Config::parse(&format!(
            "languages: [ja, zh-hant, en2]\nlabels:\n{ja}{}{}",
            complete_labels("zh-hant", "Z"),
            complete_labels("en2", "E")
        ))
        .is_ok()
    );
}

// @kotowari[REQ-core-352]
#[test]
fn req_core_352_unknown_keys_and_a_doubled_number_placeholder_stop() {
    let ja = complete_labels("ja", "J");
    for labels in [
        format!("labels:\n{ja}    extra: \"x\"\n"),
        "labels:\n  en:\n    unknown: \"x\"\n".to_string(),
        "labels:\n  en:\n    open_items: \"{n} of {n}\"\n".to_string(),
    ] {
        assert!(
            matches!(
                Config::parse(&format!("languages: [ja, en]\n{labels}")),
                Err(crate::StopReason::ConfigError(_))
            ),
            "{labels} should stop"
        );
    }
}

// @kotowari[REQ-core-340]
#[test]
fn req_core_340_the_blob_hash_is_the_git_blob_hash() {
    assert_eq!(
        crate::translations::blob_hash(b"a\n"),
        "78981922613b2afb6025042ff6bd878ac1994e85"
    );
    assert_eq!(
        crate::translations::blob_hash(b""),
        "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"
    );
}

use crate::skeleton::{Kind, mismatch};

/// 文だけを変えた側は一致し、部分ごとに1か所を変えた側はその部分の名前と行で食い違う
fn assert_parts(
    kind: Kind,
    first: &str,
    sentences: &str,
    variants: &[(&str, &str, Option<usize>)],
) {
    assert_eq!(mismatch(kind, first, first), None);
    assert_eq!(mismatch(kind, first, sentences), None, "{sentences}");
    for (other, part, line) in variants {
        assert_eq!(
            mismatch(kind, first, other),
            Some((*part, *line)),
            "{other}"
        );
    }
}

const TOPIC: &str = "# T\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- source: r.md#A1\n- verification: review\n- how_to_verify: read\n\nBody.\n\n## Decision tables\n\n### TBL-001: Table\n\n- source: r.md#A1\n\n| a | b |\n|---|---|\n| REQ-001 | x |\n\n```text\ncode\n```\n\n## Examples\n\n```gherkin\n@id=EX-001 @about=REQ-001\nScenario: s\n  Given g\n  Then t\n```\n";

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_topic_documents_compare_headings_fields_tables_gherkin_and_code() {
    let sentences = TOPIC
        .replace("# T\n", "# Title\n")
        .replace("Scope.", "Another scope.")
        .replace("REQ-001: Name", "REQ-001: Other name")
        .replace("Body.", "Other body.")
        .replace("| x |", "| y |")
        .replace("Scenario: s", "Scenario: other")
        .replace("Given g", "Given other");
    assert_parts(
        Kind::Topic,
        TOPIC,
        &sentences,
        &[
            (
                &TOPIC.replace("### TBL-001", "### TBL-002"),
                "heading",
                Some(18),
            ),
            (
                &TOPIC.replace("## Examples", "### Examples"),
                "heading",
                Some(30),
            ),
            (
                &TOPIC.replace("- kind: ubiquitous", "- kind: event_driven"),
                "field",
                Some(9),
            ),
            (&TOPIC.replace("- how_to_verify: read\n", ""), "field", None),
            (
                &TOPIC.replace("| REQ-001 | x |", "| REQ-002 | x |"),
                "table",
                Some(24),
            ),
            (
                &TOPIC.replace("| REQ-001 | x |", "| REQ-001 | x | z |"),
                "table",
                Some(24),
            ),
            (
                &TOPIC.replace("@about=REQ-001\n", "@about=REQ-002\n"),
                "gherkin",
                Some(33),
            ),
            (&TOPIC.replace("  Then t", "  And t"), "gherkin", Some(36)),
            (&TOPIC.replace("\ncode\n", "\nother\n"), "code", Some(26)),
        ],
    );
}

const GLOSSARY: &str = "# Glossary\n\n| Term | Meaning | Source |\n|---|---|---|\n| a | m | r.md#A1 |\n| b | n | r.md#A2 |\n";

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_glossaries_compare_the_rows_and_their_sources() {
    let sentences = GLOSSARY.replace("| a | m |", "| x | y |");
    assert_parts(
        Kind::Glossary,
        GLOSSARY,
        &sentences,
        &[
            (&GLOSSARY.replace("r.md#A2", "r.md#A3"), "glossary", Some(6)),
            (
                &GLOSSARY.replace("| b | n | r.md#A2 |\n", ""),
                "glossary",
                None,
            ),
        ],
    );
}

const FLAGS: &str = "# Flags\n\nScope.\n\n## Flags\n\n### FLAG-001: Name\n\n- kind: gap\n- related: REQ-001\n- source: r.md#A1\n\nBody.\n";

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_flag_records_compare_ids_and_fields() {
    let sentences = FLAGS
        .replace("Body.", "Other.")
        .replace(": Name", ": Other");
    assert_parts(
        Kind::Flags,
        FLAGS,
        &sentences,
        &[
            (&FLAGS.replace("FLAG-001", "FLAG-002"), "flag", Some(7)),
            (&FLAGS.replace("REQ-001", "REQ-002"), "flag", Some(10)),
        ],
    );
}

const GUIDE: &str = "# Guide\n\nText.\n\n<!-- @kotowari[REQ-001:0123abcd] -->\n\n## Use\n\n```sh\nkotowari check\n```\n\n| a | b |\n|---|---|\n| c | d |\n";

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_guides_compare_headings_marks_code_and_tables() {
    let sentences = GUIDE
        .replace("Text.", "Other text.")
        .replace("## Use", "## Usage")
        .replace("| c | d |", "| e | f |");
    assert_parts(
        Kind::Guide,
        GUIDE,
        &sentences,
        &[
            (&GUIDE.replace("## Use", "### Use"), "heading", Some(7)),
            (&GUIDE.replace("0123abcd", "0123abce"), "mark", Some(5)),
            (
                &GUIDE.replace("kotowari check", "kotowari list"),
                "code",
                Some(9),
            ),
            (
                &GUIDE.replace("| c | d |", "| c | d | e |"),
                "table",
                Some(15),
            ),
            (&GUIDE.replace("| c | d |\n", ""), "table", None),
        ],
    );
}

const OVERVIEW_DATA: &str = "---\nir:\n  - docs/ir/a.md\n---\n\n# Title\n\n```view lead\nconclusion: c\npoints:\n  - p\n```\n\n<!-- @kotowari[REQ-001:0123abcd] -->\n\n## Section\n\n```view steps\nitems:\n  - title: t\n    body: b\n    refs: [REQ-001]\n```\n\n| a |\n|---|\n| b |\n";

// @kotowari[TBL-core-044, TBL-core-045]
#[test]
fn tbl_core_044_overview_data_compares_frontmatter_headings_parts_marks_and_tables() {
    let sentences = OVERVIEW_DATA
        .replace("# Title\n", "# Other\n")
        .replace("conclusion: c", "conclusion: other")
        .replace("  - p", "  - q")
        .replace("## Section", "## Other")
        .replace("title: t", "title: u")
        .replace("body: b", "body: d")
        .replace("| b |", "| c |");
    assert_parts(
        Kind::OverviewData,
        OVERVIEW_DATA,
        &sentences,
        &[
            (
                &OVERVIEW_DATA.replace("docs/ir/a.md", "docs/ir/b.md"),
                "frontmatter",
                Some(1),
            ),
            (
                &OVERVIEW_DATA.replace("## Section", "### Section"),
                "heading",
                Some(16),
            ),
            (
                &OVERVIEW_DATA.replace("  - p\n", "  - p\n  - q\n"),
                "part",
                Some(8),
            ),
            (
                &OVERVIEW_DATA.replace("[REQ-001]", "[REQ-002]"),
                "part",
                Some(18),
            ),
            (
                &OVERVIEW_DATA.replace("view steps", "view cards"),
                "part",
                Some(18),
            ),
            (
                &OVERVIEW_DATA.replace("0123abcd", "0123abce"),
                "mark",
                Some(14),
            ),
            (
                &OVERVIEW_DATA.replace("| b |", "| b | c |"),
                "table",
                Some(27),
            ),
        ],
    );
}

const CONTENTS: &str = "title: T\nnote: N\nitems:\n  - a\n  - title: G\n    items:\n      - b\n";

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_contents_compare_nesting_names_and_whether_a_note_is_present() {
    let sentences = CONTENTS
        .replace("title: T", "title: U")
        .replace("note: N", "note: M");
    assert_parts(
        Kind::Toc,
        CONTENTS,
        &sentences,
        &[
            (&CONTENTS.replace("note: N\n", ""), "toc", None),
            (&CONTENTS.replace("- b", "- c"), "toc", None),
            (
                &CONTENTS.replace(
                    "    items:\n      - b\n",
                    "    note: x\n    items:\n      - b\n",
                ),
                "toc",
                None,
            ),
        ],
    );
}

/// texts のパスと中身から`対`を作る。`一致の記録`は無い
fn pair_of(
    place: crate::translations::Place,
    first: &str,
    languages: &[String],
    texts: &[(&str, &str)],
) -> crate::translations::Pair {
    crate::translations::Pair::read::<()>(
        place,
        first,
        languages,
        |path| {
            Ok(texts
                .iter()
                .find(|(name, _)| *name == path)
                .map(|(_, text)| crate::translations::SideText::new(text.as_bytes(), *text)))
        },
        None,
    )
    .unwrap()
}

fn ja_en() -> (Config, Vec<String>) {
    let config = Config::parse(&format!(
        "languages: [ja, en]\nlabels:\n{}",
        complete_labels("ja", "J")
    ))
    .unwrap();
    let languages = config.languages();
    (config, languages)
}

/// 種類ごとの "line"
fn lines_of(findings: &[crate::Finding], kind: &str) -> Vec<Option<usize>> {
    findings
        .iter()
        .filter(|finding| finding.kind() == kind)
        .map(crate::Finding::line)
        .collect()
}

// @kotowari[REQ-core-027, TBL-core-019]
#[test]
fn req_core_027_lines_of_the_pair_findings() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    let mut pairs = Pairs::default();
    // 欠けた側と一致の記録: null
    pairs.insert(pair_of(
        Place::Guide,
        "g/a.md",
        &languages,
        &[("g/a.md", "# A\n")],
    ));
    // 切り替えの行: その行、題名の行、null
    pairs.insert(pair_of(
        Place::Guide,
        "g/b.md",
        &languages,
        &[("g/b.md", "# B\n\nText.\n"), ("g/b.en.md", "# B\n")],
    ));
    pairs.insert(pair_of(
        Place::Guide,
        "g/c.md",
        &languages,
        &[("g/c.md", "")],
    ));
    // 骨組みとリンク: 食い違った要素とリンクの行
    pairs.insert(pair_of(
        Place::Guide,
        "g/d.md",
        &languages,
        &[
            ("g/d.md", "# D\n\nJdlanguage_name | [English](d.en.md)\n\n## X\n\n[r](../docs/decision/records/r.md)\n"),
            ("g/d.en.md", "# D\n\n[Jlanguage_name](d.md) | English\n\n### X\n\n[a](a.md)\n"),
        ],
    ));
    let findings = pairs.findings(&config);
    assert_eq!(lines_of(&findings, "translation_missing"), [None, None]);
    assert!(
        lines_of(&findings, "translation_record_invalid")
            .iter()
            .all(Option::is_none)
    );
    assert_eq!(
        lines_of(&findings, "translation_switcher_invalid"),
        [Some(1), Some(3), Some(1), None, Some(3)]
    );
    assert_eq!(
        lines_of(&findings, "translation_structure_mismatch"),
        [Some(5)]
    );
    assert_eq!(lines_of(&findings, "link_to_record"), [Some(7)]);
    // d.md の3行目は切り替えの行と同じでないので、そのリンクも検査する
    assert_eq!(
        lines_of(&findings, "link_language_mismatch"),
        [Some(3), Some(7)]
    );
}

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_link_parts_compare_destinations_read_as_the_first_language_side() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    for place in [Place::Guide, Place::OverviewData] {
        let mut pairs = Pairs::default();
        let first = "# A\n\n[x](b.md#one) [y](https://example.com)\n";
        pairs.insert(pair_of(
            place,
            "g/b.md",
            &languages,
            &[("g/b.md", "# B\n"), ("g/b.en.md", "# B\n")],
        ));
        for (other, mismatch) in [
            ("# A\n\n[x](b.en.md#two) [y](https://example.org)\n", false),
            ("# A\n\n[x](c.md)\n", true),
        ] {
            let mut pairs = pairs.clone();
            pairs.insert(pair_of(
                place,
                "g/a.md",
                &languages,
                &[("g/a.md", first), ("g/a.en.md", other)],
            ));
            let found: Vec<_> = pairs
                .findings(&config)
                .into_iter()
                .filter(|finding| {
                    finding.kind() == "translation_structure_mismatch"
                        && finding.path() == "g/a.en.md"
                })
                .map(|finding| finding.detail().to_string())
                .collect();
            let expected: &[&str] = if mismatch { &["link"] } else { &[] };
            assert_eq!(found, expected, "{place:?} {other}");
        }
    }
}

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_the_switcher_line_is_not_in_the_skeleton() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    let mut pairs = Pairs::default();
    // 英語の側の切り替えの行のリンクは、先頭の言語の側のリンクと数が合わなくても食い違わない
    pairs.insert(pair_of(
        Place::Guide,
        "g/a.md",
        &languages,
        &[
            (
                "g/a.md",
                "# A\n\nJlanguage_name | [English](a.en.md)\n\n[x](x.md)\n",
            ),
            (
                "g/a.en.md",
                "# A\n\n[Jlanguage_name](a.md) | English\n\n[x](x.md)\n",
            ),
        ],
    ));
    let findings = pairs.findings(&config);
    assert!(
        lines_of(&findings, "translation_structure_mismatch").is_empty(),
        "{findings:?}"
    );
    assert!(
        lines_of(&findings, "link_language_mismatch").is_empty(),
        "{findings:?}"
    );
    assert!(
        lines_of(&findings, "translation_switcher_invalid").is_empty(),
        "{findings:?}"
    );
}
