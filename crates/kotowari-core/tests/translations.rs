//! 言語の一覧、UI の文字、対の読み方（docs/decisions/core/translation-pairs.md、docs/decisions/core/overview-languages.md）

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

// @kotowari[C-core-ovlang-6, C-core-ovlang-7]
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

// @kotowari[C-core-pairs-2]
#[test]
fn req_core_334_the_language_list_is_the_languages_key_and_english_when_absent_or_empty() {
    let ja = complete_labels("ja", "J");
    let config = Config::parse(&format!("languages: [ja, en]\nlabels:\n{ja}")).unwrap();
    assert_eq!(config.languages(), ["ja", "en"]);
    assert_eq!(
        Config::parse("tests:\n  files: []\n").unwrap().languages(),
        ["en"]
    );
    assert_eq!(
        Config::parse("languages: []\n").unwrap().languages(),
        ["en"]
    );
    // validated は読んだ値をそのまま保つ
    assert_eq!(config.validated().unwrap(), config);
}

// @kotowari[C-core-pairs-3]
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

// @kotowari[C-core-pairs-3]
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

// @kotowari[C-core-pairstruct-2]
#[test]
fn tbl_core_044_table_cells_preserve_escaped_pipes_and_source_boundaries() {
    let bordered = "| A | B |\n|---|---|\n| a | b |\n";
    let unbordered = "A | B\n---|---\nx | y\n";
    for kind in [Kind::Guide, Kind::OverviewData] {
        assert_eq!(mismatch(kind, bordered, unbordered), None);
        assert_eq!(
            mismatch(kind, bordered, "A | B | C\n---|---|---\nx | y | z\n"),
            Some(("table", Some(1)))
        );
    }
}

// @kotowari[C-core-pairstruct-2]
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

// @kotowari[C-core-pairstruct-3]
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

// @kotowari[C-core-ovlang-9, C-core-ovlang-10, C-core-ovlang-13]
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

// @kotowari[C-core-pairs-9]
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

const GUIDE: &str = "# Guide\n\nText.\n\n<!-- @kotowari[docs/decisions/a.md#use:0123abcd] -->\n\n## Use\n\n```sh\nkotowari check\n```\n\n| a | b |\n|---|---|\n| c | d |\n";

// @kotowari[C-core-pairstruct-1, C-core-pairstruct-2]
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

const OVERVIEW_DATA: &str = "---\ndocuments:\n  - docs/decisions/a.md\n---\n\n# Title\n\n```view lead\nconclusion: c\npoints:\n  - p\n```\n\n<!-- @kotowari[docs/decisions/a.md#use:0123abcd] -->\n\n## Section\n\n```view steps\nitems:\n  - title: t\n    body: b\n    refs: [REQ-001]\n```\n\n| a |\n|---|\n| b |\n";

// @kotowari[C-core-pairstruct-3]
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
                &OVERVIEW_DATA.replace("docs/decisions/a.md", "docs/decisions/b.md"),
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

// @kotowari[C-core-pairstruct-4]
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

// @kotowari[C-core-pairstruct-5]
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

// @kotowari[C-core-pairstruct-11]
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

// @kotowari[C-core-forder-4]
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
    // d.md の3行目は切り替えの行と同じでないので、そのリンクも検査する
    assert_eq!(
        lines_of(&findings, "link_language_mismatch"),
        [Some(3), Some(7)]
    );
}

// @kotowari[C-core-pairstruct-5]
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

// @kotowari[C-core-pairstruct-9]
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

// @kotowari[C-core-pairstruct-6]
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

// @kotowari[C-core-pairstruct-5, C-core-pairstruct-11]
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
    assert_eq!(
        lines_of(&findings, "translation_structure_mismatch"),
        [None]
    );
}

/// 決定の文書の1つの決定。state は状態の行、row は確認することの1行
fn decision_document(state: &str, section: &str, row: &str) -> String {
    format!(
        "# T\n\nLead.\n\n## D\n\n{state}\n\n### {section}\n\nText.\n\n### Confirmations\n\n- {row}\n"
    )
}

// @kotowari[C-core-lang-4]
#[test]
fn a_decision_skeleton_compares_headings_states_and_rows_but_not_words() {
    let first = decision_document("Status: proposed", "Problem", "C-a-1 Shown");
    assert_eq!(
        mismatch(
            Kind::Decision,
            &first,
            &decision_document("状態: 提案", "問題", "C-a-1 出る")
                .replace("### Confirmations", "### 確認すること")
        ),
        None
    );
    assert_eq!(
        mismatch(
            Kind::Decision,
            &first,
            &decision_document("Status: implemented", "Problem", "C-a-1 Shown")
        ),
        Some(("state", Some(7)))
    );
    assert_eq!(
        mismatch(
            Kind::Decision,
            &first,
            &decision_document("Status: proposed", "Background", "C-a-1 Shown")
        ),
        Some(("heading", Some(9)))
    );
    assert_eq!(
        mismatch(
            Kind::Decision,
            &first,
            &decision_document("Status: proposed", "Problem", "C-a-1 (by reading) Shown")
        ),
        Some(("confirmation", Some(15)))
    );
    assert_eq!(mismatch(Kind::DecisionGlossary, &first, "# Other\n"), None);
}
