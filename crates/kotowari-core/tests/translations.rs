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

// @kotowari[REQ-core-335, REQ-core-352]
#[test]
fn req_core_335_invalid_tags_stop_even_with_complete_labels() {
    for tag in ["\"\"", "en_us", "\"e n\""] {
        let yaml = format!("languages: [{tag}]\nlabels:\n{}", complete_labels(tag, "X"));
        assert!(
            matches!(Config::parse(&yaml), Err(crate::StopReason::ConfigError(_))),
            "{yaml}"
        );
    }
}

// @kotowari[TBL-core-044]
#[test]
fn tbl_core_044_table_cells_preserve_escaped_pipes_and_source_boundaries() {
    let bordered = "| A | B |\n|---|---|\n| a | b |\n";
    let unbordered = "A | B\n---|---\nx | y\n";
    for kind in [Kind::Topic, Kind::Guide, Kind::OverviewData] {
        assert_eq!(mismatch(kind, bordered, unbordered), None);
        assert_eq!(
            mismatch(kind, bordered, "A | B | C\n---|---|---\nx | y | z\n"),
            Some(("table", Some(1)))
        );
    }
    let glossary = "| Term | Meaning | Source |\n|---|---|---|\n| a | b | r.md#A1 |\n";
    assert_eq!(
        mismatch(
            Kind::Glossary,
            glossary,
            &glossary.replace("| a | b |", "a | b |")
        ),
        None
    );
    assert_eq!(
        mismatch(
            Kind::Glossary,
            glossary,
            &glossary.replace("| r.md", "|r.md")
        ),
        None
    );
    for source in [r"r.md#A1\|", r"r.md#A1\\", r"r.md#A1\\\|"] {
        let with_border =
            format!("| Term | Meaning | Source |\n|---|---|---|\n| a | b | {source}|\n");
        let without_border =
            format!("| Term | Meaning | Source |\n|---|---|---|\n| x | y | {source}\n");
        assert_eq!(
            mismatch(Kind::Glossary, &with_border, &without_border),
            None,
            "{source}"
        );
        assert_eq!(
            mismatch(
                Kind::Glossary,
                &with_border,
                &without_border.replace("A1", "A2")
            ),
            Some(("glossary", Some(3)))
        );
    }
}

// @kotowari[REQ-core-345, TBL-core-044]
#[test]
fn req_core_345_absent_headings_have_no_line_but_changed_headings_have_their_line() {
    let first = "# Guide\n\n## One\n\n### Two\n\n#### Three\n";
    assert_eq!(
        mismatch(Kind::Guide, first, "# Guide\n\n### Two\n\n#### Three\n"),
        Some(("heading", None))
    );
    assert_eq!(
        mismatch(Kind::Guide, first, "# Guide\n\n##### Other\n\n#### Three\n"),
        Some(("heading", Some(3)))
    );
    assert_eq!(
        mismatch(
            Kind::Guide,
            first,
            "# Guide\n\n### Other\n\n### Two\n\n#### Three\n"
        ),
        Some(("heading", Some(3)))
    );
}

// @kotowari[TBL-core-044, TBL-core-045]
#[test]
fn tbl_core_045_overview_sentences_translate_but_keys_values_and_lengths_remain() {
    for (kind, yaml, sentence_fields) in [
        (
            "cards",
            "cards:\n  - title: sentence\n    items: [sentence]\n    refs: [REQ-001]\n",
            &["title", "items"][..],
        ),
        (
            "compare",
            "items:\n  - before: sentence\n    after: sentence\n    why: sentence\n    refs: [REQ-001]\n",
            &["before", "after", "why"][..],
        ),
        (
            "decisions",
            "items:\n  - text: sentence\n    by: sentence\n    refs: [REQ-001]\n",
            &["text", "by"][..],
        ),
        (
            "quiz",
            "items:\n  - q: sentence\n    a: sentence\n    refs: [REQ-001]\n",
            &["q", "a"][..],
        ),
    ] {
        let first = format!("# Overview\n\n```view {kind}\n{yaml}```\n");
        assert_eq!(
            mismatch(
                Kind::OverviewData,
                &first,
                &first.replace("sentence", "translated")
            ),
            None,
            "{kind}"
        );
        for other in [
            first.replace("REQ-001", "REQ-002"),
            first.replace("[REQ-001]", "[REQ-001, REQ-002]"),
        ] {
            assert_eq!(
                mismatch(Kind::OverviewData, &first, &other),
                Some(("part", Some(3))),
                "{kind}"
            );
        }
        for field in sentence_fields {
            let other = first.replace(&format!("{field}:"), "other:");
            assert_eq!(
                mismatch(Kind::OverviewData, &first, &other),
                Some(("part", Some(3))),
                "{kind}.{field}"
            );
        }
        if kind == "cards" {
            assert_eq!(
                mismatch(
                    Kind::OverviewData,
                    &first,
                    &first.replace("[sentence]", "[sentence, translated]")
                ),
                Some(("part", Some(3)))
            );
        }
    }
}

// @kotowari[REQ-core-352]
#[test]
fn req_core_352_unknown_keys_and_a_doubled_number_placeholder_stop() {
    let ja = complete_labels("ja", "J");
    // labels.ja は完全なので、止まる理由はそれぞれの誤りだけ
    assert!(
        Config::parse(&format!(
            "languages: [ja, en]\nlabels:\n{ja}  en:\n    open_items: \"{{n}} left\"\n"
        ))
        .is_ok()
    );
    for labels in [
        format!("labels:\n{ja}    extra: \"x\"\n"),
        format!("labels:\n{ja}  en:\n    unknown: \"x\"\n"),
        format!("labels:\n{ja}  en:\n    open_items: \"{{n}} of {{n}}\"\n"),
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
    // GFM の "\|" はセルの中の文字で、列を分けない
    let sentences = GLOSSARY.replace("| a | m |", "| x | y \\| z |");
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
        .replace("| c | d |", "| e \\| x | f |");
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
            (&CONTENTS.replace("note: N\n", ""), "toc", Some(1)),
            (&CONTENTS.replace("- b", "- c"), "toc", Some(7)),
            (
                &CONTENTS.replace(
                    "    items:\n      - b\n",
                    "    note: x\n    items:\n      - b\n",
                ),
                "toc",
                Some(5),
            ),
            // 名前の項目が無いことは null
            (
                &CONTENTS.replace("      - b\n", "      - b\n      - c\n"),
                "toc",
                Some(8),
            ),
            (&CONTENTS.replace("  - a\n", ""), "toc", None),
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

// @kotowari[REQ-core-344, REQ-core-345, REQ-core-347, TBL-core-044]
#[test]
fn reference_links_preserve_the_order_of_their_resolved_destinations() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    for usage in ["[first][one] [second][two]", "![first][one] ![second][two]"] {
        let first = format!("# Guide\n\n{usage}\n\n[one]: x.md\n[two]: y.md\n");
        let swapped = usage
            .replace("[one]", "[swap]")
            .replace("[two]", "[one]")
            .replace("[swap]", "[two]");
        let other = format!("# Guide\n\n{swapped}\n\n[one]: x.md\n[two]: y.md\n");
        let mut pairs = Pairs::default();
        pairs.insert(pair_of(
            Place::Guide,
            "g/a.md",
            &languages,
            &[("g/a.md", &first), ("g/a.en.md", &other)],
        ));
        let findings = pairs.findings(&config);
        let mismatch: Vec<_> = findings
            .iter()
            .filter(|finding| finding.kind() == "translation_structure_mismatch")
            .collect();
        assert_eq!(mismatch.len(), 1, "{findings:?}");
        assert_eq!(mismatch[0].line(), Some(3));
        assert_eq!(mismatch[0].detail(), "link");
    }
}

// @kotowari[REQ-core-347, REQ-core-348]
#[test]
fn reference_links_and_images_are_checked_at_the_usage_and_definition_lines() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    let mut pairs = Pairs::default();
    pairs.insert(pair_of(
        Place::Guide,
        "g/b.md",
        &languages,
        &[("g/b.md", "# B\n"), ("g/b.en.md", "# B\n")],
    ));
    pairs.insert(pair_of(
        Place::Guide,
        "g/a.md",
        &languages,
        &[
            ("g/a.md", "# A\n"),
            (
                "g/a.en.md",
                "# A\n\n[ONE]\n\n![one][]\n\n[one]: b.md\n[ONE]: b.en.md\n",
            ),
        ],
    ));
    assert_eq!(
        lines_of(&pairs.findings(&config), "link_language_mismatch"),
        [Some(3), Some(5), Some(7)]
    );
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

// @kotowari[REQ-core-346, TBL-core-044]
#[test]
fn req_core_346_a_switcher_at_end_of_file_needs_no_final_newline() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    let mut pairs = Pairs::default();
    pairs.insert(pair_of(
        Place::Guide,
        "g/a.md",
        &languages,
        &[
            ("g/a.md", "# A\n\nJlanguage_name | [English](a.en.md)"),
            ("g/a.en.md", "# A\n\n[Jlanguage_name](a.md) | English"),
        ],
    ));
    let findings = pairs.findings(&config);
    for kind in [
        "translation_switcher_invalid",
        "translation_structure_mismatch",
        "link_language_mismatch",
    ] {
        assert!(lines_of(&findings, kind).is_empty(), "{findings:?}");
    }
}

// @kotowari[REQ-core-346]
#[test]
fn req_core_346_three_language_switchers_do_not_count_as_document_scope() {
    use crate::{CheckInputs, Inspection, SourceText, translations::Place};
    let config = Config::parse(&format!(
        "languages: [ja, en, fr]\nlabels:\n{}{}",
        complete_labels("ja", "J"),
        complete_labels("fr", "F")
    ))
    .unwrap();
    let texts = [
        (
            "docs/ir/a.md",
            "# Topic\n\nJlanguage_name | [English](a.en.md) | [Flanguage_name](a.fr.md)\n\nScope.\n",
        ),
        (
            "docs/ir/a.en.md",
            "# Topic\n\n[Jlanguage_name](a.md) | English | [Flanguage_name](a.fr.md)\n\nScope.\n",
        ),
        (
            "docs/ir/a.fr.md",
            "# Topic\n\n[Jlanguage_name](a.md) | [English](a.en.md) | Flanguage_name\n\nScope.\n",
        ),
    ];
    for omit_scope in [false, true] {
        let mut inputs = CheckInputs::default();
        inputs.read.config = config.clone();
        inputs.read.config.tests.files.clear();
        let first = if omit_scope {
            texts[0].1.replace("Scope.", "")
        } else {
            texts[0].1.to_string()
        };
        inputs.read.ir = Some(vec![SourceText::new(texts[0].0, &first).unwrap()]);
        inputs.read.records = Some(vec![]);
        inputs.read.adr = Some(vec![]);
        inputs
            .translations
            .insert(pair_of(Place::Ir, texts[0].0, &config.languages(), &texts));
        let inspection = Inspection::build(inputs).unwrap();
        let lines = lines_of(inspection.check().findings(), "missing_scope");
        let expected = if omit_scope { vec![None] } else { vec![] };
        assert_eq!(lines, expected, "{:?}", inspection.check().findings());
    }
}

// @kotowari[REQ-core-347, REQ-core-348, REQ-core-349, TBL-core-044]
#[test]
fn req_core_347_inline_images_have_checked_and_compared_destinations() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    let mut pairs = Pairs::default();
    pairs.insert(pair_of(
        Place::Guide,
        "g/b.md",
        &languages,
        &[("g/b.md", "# B\n"), ("g/b.en.md", "# B\n")],
    ));
    pairs.insert(pair_of(
        Place::Guide,
        "g/a.md",
        &languages,
        &[
            ("g/a.md", "# A\n\n![one](b.md)\n"),
            (
                "g/a.en.md",
                "# A\n\n![one](b.md)\n\n![two](../docs/decision/records/r.md)\n",
            ),
        ],
    ));
    let findings = pairs.findings(&config);
    assert_eq!(lines_of(&findings, "link_language_mismatch"), [Some(3)]);
    assert_eq!(lines_of(&findings, "link_to_record"), [Some(5)]);
    assert_eq!(
        lines_of(&findings, "translation_structure_mismatch"),
        [None]
    );
}

// @kotowari[REQ-core-347]
#[test]
fn req_core_347_toc_titles_are_not_scanned_as_markdown_links() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    let mut pairs = Pairs::default();
    let text = "title: \"[r](../docs/decision/records/r.md)\"\nitems: [a]\n";
    pairs.insert(pair_of(
        Place::Toc,
        "g/toc.yaml",
        &languages,
        &[("g/toc.yaml", text), ("g/toc.en.yaml", text)],
    ));
    assert!(lines_of(&pairs.findings(&config), "link_to_record").is_empty());
}

// @kotowari[REQ-core-347, REQ-core-349]
#[test]
fn req_core_347_a_colon_in_a_relative_link_query_is_not_a_uri_scheme() {
    use crate::translations::{Pairs, Place};
    let (config, languages) = ja_en();
    let mut pairs = Pairs::default();
    pairs.insert(pair_of(
        Place::Guide,
        "a.md",
        &languages,
        &[
            ("a.md", "# A\n\n[r](docs/decision/records/r.md?mode=a:b)\n"),
            ("a.en.md", "# A\n"),
        ],
    ));
    let findings = pairs.findings(&config);
    assert_eq!(lines_of(&findings, "link_to_record"), [Some(3)]);
    assert_eq!(
        findings
            .iter()
            .find(|finding| finding.kind() == "link_to_record")
            .unwrap()
            .detail(),
        "docs/decision/records/r.md?mode=a:b"
    );
}
