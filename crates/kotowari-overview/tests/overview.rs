#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari_core::{Finding, NativeSourceText, ReadModel, RepositoryReadPreparation, SourceText};
use kotowari_markdown_view::ReferenceState;
use kotowari_overview::{Overview, inspect};

/// 決定の文書。実装済みの決定と、提案の決定を持つ
const CLI_DECISIONS: &str = "# CLI\n\nDecides the commands.\n\n\
## Run commands\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nRun them.\n\n\
### Confirmations\n\n- C-core-cli-1 A command runs\n  and ends\n\n\
## Later\n\nStatus: proposed\n\n### Problem\n\nTrouble.\n\n### Decision\n\nLater.\n\n\
### Confirmations\n\n- C-core-cli-2 Later runs\n";

const OTHER_DECISIONS: &str = "# Other\n\nDecides the rest.\n\n## Rest\n\nStatus: implemented\n\n\
### Problem\n\nTrouble.\n\n### Decision\n\nRest.\n\n### Confirmations\n\n- C-core-other-1 Rest\n";

fn model() -> ReadModel {
    let mut config = kotowari_core::config::Config::default();
    config.tests.files.clear();
    let native = |path: &str, text: &str| NativeSourceText::new(path, path, text);
    let mut preparation = RepositoryReadPreparation::new(config).unwrap();
    for (path, text) in [
        ("docs/decisions/core/cli.md", CLI_DECISIONS),
        ("docs/decisions/core/other.md", OTHER_DECISIONS),
        ("docs/decisions/core/CONTEXT.md", "# Glossary\n"),
    ] {
        preparation
            .push_decision_document(native(path, text))
            .unwrap();
    }
    preparation.finish(Vec::new()).unwrap()
}

fn fingerprint(model: &ReadModel, id: &str) -> String {
    let report = model.query(id).unwrap();
    match report.items()[0].item() {
        kotowari_core::ListItem::Decision(item) => item.fingerprint().to_string(),
        kotowari_core::ListItem::Confirmation(_) => panic!("{id} is a decision"),
    }
}

const TOC: &str = ".kotowari/toc.yaml";

/// 元データのすべての名前を1回ずつ並べた目次で検査する
fn run(model: &ReadModel, files: &[(&str, &str)]) -> Overview {
    let mut names: Vec<&str> = files
        .iter()
        .map(|(path, _)| {
            let name = path.rsplit('/').next().unwrap();
            name.strip_suffix(".md").unwrap_or(name)
        })
        .collect();
    names.sort();
    names.dedup();
    let items: Vec<String> = names
        .iter()
        .map(|name| serde_json::to_string(name).unwrap())
        .collect();
    run_with_toc(
        model,
        files,
        &format!("title: 目次\nitems: [{}]\n", items.join(", ")),
    )
}

fn run_with_toc(model: &ReadModel, files: &[(&str, &str)], toc: &str) -> Overview {
    let sources: Vec<SourceText> = files
        .iter()
        .map(|(path, text)| SourceText::new(*path, *text).unwrap())
        .collect();
    inspect(model, &sources, &SourceText::new(TOC, toc).unwrap())
}

const FRONT: &str = "---\ndocuments:\n  - docs/decisions/core/cli.md\n---\n";
const LEAD: &str = "```view lead\nconclusion: 結論\npoints: [要点]\n```\n";

/// frontmatter、題名、lead に続けて body を置いた元データ
fn data(title: &str, body: &str) -> String {
    format!("{FRONT}\n# {title}\n\n{LEAD}\n{body}")
}

fn found(overview: &Overview) -> Vec<(String, Option<usize>, String, String)> {
    overview
        .findings()
        .iter()
        .map(|finding: &Finding| {
            (
                finding.path().to_string(),
                finding.line(),
                finding.kind().as_str().to_string(),
                finding.detail().to_string(),
            )
        })
        .collect()
}

fn of_kind(overview: &Overview, kind: &str) -> Vec<(String, Option<usize>, String, String)> {
    found(overview)
        .into_iter()
        .filter(|(_, _, found, _)| found == kind)
        .collect()
}

const A: &str = ".kotowari/overview/a.md";
const B: &str = ".kotowari/overview/b.md";

fn line_of(text: &str, needle: &str) -> usize {
    text.lines().position(|line| line == needle).unwrap() + 1
}

// @kotowari[C-core-ovdata-1]
#[test]
fn valid_overview_data_has_no_findings_and_counts_its_files() {
    let model = model();
    let text = data("変更照合", "## 流れ\n\n文。\n");
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(found(&overview), []);
    assert_eq!((overview.files(), overview.marks()), (1, 0));
}

// @kotowari[C-core-ovdata-8, C-core-ovdata-9]
#[test]
fn ex_core_466_an_unknown_part_and_a_part_with_an_unknown_key_are_errors() {
    let model = model();
    let text = data(
        "a",
        "## 節\n\n```view chart\nx: 1\n```\n\n```view cards\ncards:\n  - title: t\n    items: []\n    color: red\n```\n",
    );
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(
        found(&overview),
        [
            (
                A.into(),
                Some(line_of(&text, "```view chart")),
                "overview_part_unknown".into(),
                "chart".into()
            ),
            (
                A.into(),
                Some(line_of(&text, "```view cards")),
                "overview_part_invalid".into(),
                "cards /cards/0/color".into()
            ),
        ]
    );
}

// @kotowari[C-core-ovdata-9]
#[test]
fn req_core_282_places_name_missing_keys_the_whole_value_and_unreadable_yaml_once_each() {
    let model = model();
    let text = data(
        "a",
        "## 節\n\n```view steps\nitems:\n  - body: b\n```\n\n```view quiz\n- 1\n```\n\n```view status\nitems: [\n```\n\n```view lead\n```\n",
    );
    let overview = run(&model, &[(A, &text)]);
    let details: Vec<String> = of_kind(&overview, "overview_part_invalid")
        .into_iter()
        .map(|(_, _, _, detail)| detail)
        .collect();
    assert_eq!(
        details,
        [
            "steps /items/0/title",
            "quiz (root)",
            "status (yaml)",
            "lead (root)"
        ]
    );
}

// @kotowari[C-core-ovdata-9]
#[test]
fn req_core_282_each_unknown_key_and_each_value_that_does_not_fit_has_its_own_place() {
    let model = model();
    let text = data(
        "a",
        "## 節\n\n```view lead\nconclusion: c\nunknown: 1\nunknown2: 2\n```\n\n```view status\nitems:\n  - state: 済み\n    text: t\n```\n",
    );
    let overview = run(&model, &[(A, &text)]);
    let details: Vec<String> = of_kind(&overview, "overview_part_invalid")
        .into_iter()
        .map(|(_, _, _, detail)| detail)
        .collect();
    assert_eq!(
        details,
        ["lead /unknown", "lead /unknown2", "status /items/0/state"]
    );
}

// @kotowari[C-core-forder-4, C-core-ovdata-11]
#[test]
fn ex_core_467_a_first_part_that_is_not_lead_is_an_error_without_a_line() {
    let model = model();
    let text =
        format!("{FRONT}\n# x\n\n```view steps\nitems:\n  - title: t\n```\n\n## 節\n\n文。\n");
    let overview = run(&model, &[(".kotowari/overview/x.md", &text)]);
    assert_eq!(
        of_kind(&overview, "overview_lead_missing"),
        [(
            ".kotowari/overview/x.md".into(),
            None,
            "overview_lead_missing".into(),
            "x.md".into()
        )]
    );
}

// @kotowari[C-core-ovdata-11]
#[test]
fn req_core_283_comments_and_blank_lines_before_the_lead_are_skipped() {
    let model = model();
    let ok = format!("{FRONT}\n# x\n\n<!-- note -->\n\n<!-- another -->\n{LEAD}\n## 節\n\n文。\n");
    assert_eq!(found(&run(&model, &[(A, &ok)])), []);
    let heading_first = format!("{FRONT}\n# x\n\n## 節\n\n{LEAD}");
    assert_eq!(
        of_kind(
            &run(&model, &[(A, &heading_first)]),
            "overview_lead_missing"
        )
        .len(),
        1
    );
}

// @kotowari[C-core-forder-4, C-core-lang-7, C-core-ovdata-12, C-core-ovdata-23]
#[test]
fn ex_core_468_a_missing_decision_document_and_one_shared_by_two_overviews_are_errors() {
    let model = model();
    let a = format!(
        "---\ndocuments:\n  - docs/decisions/core/cli.md\n  - docs/decisions/core/none.md\n---\n\n# a\n\n{LEAD}"
    );
    let b = format!("---\ndocuments:\n  - docs/decisions/core/cli.md\n---\n\n# b\n\n{LEAD}");
    let overview = run(&model, &[(A, &a), (B, &b)]);
    assert_eq!(
        found(&overview),
        [
            (
                A.into(),
                None,
                "overview_document_missing".into(),
                "docs/decisions/core/none.md".into()
            ),
            (
                B.into(),
                None,
                "overview_document_shared".into(),
                "docs/decisions/core/cli.md".into()
            ),
        ]
    );
}

// @kotowari[C-core-ovdata-5, C-core-ovdata-23]
#[test]
fn req_core_284_a_valid_documents_list_is_checked_beside_other_frontmatter_violations() {
    let model = model();
    let text = format!(
        "---\ndocuments:\n  - docs/decisions/core/cli.md\n  - docs/decisions/core/none.md\ntitle: x\n---\n\n# a\n\n{LEAD}"
    );
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(of_kind(&overview, "overview_form_invalid").len(), 1);
    assert_eq!(
        of_kind(&overview, "overview_document_missing"),
        [(
            A.into(),
            None,
            "overview_document_missing".into(),
            "docs/decisions/core/none.md".into()
        )]
    );
}

// @kotowari[C-core-ovdata-23]
#[test]
fn req_core_284_a_glossary_does_not_count_as_a_decision_document() {
    let model = model();
    let text = format!(
        "---\ndocuments:\n  - docs/decisions/core/CONTEXT.md\n  - docs/decisions/core/other.md\n---\n\n# a\n\n{LEAD}"
    );
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(
        found(&overview),
        [(
            A.into(),
            None,
            "overview_document_missing".into(),
            "docs/decisions/core/CONTEXT.md".into()
        )]
    );
}

// @kotowari[C-core-ovdata-13, C-core-lang-8]
#[test]
fn ex_core_469_unresolved_references_are_errors_one_per_reference() {
    let model = model();
    let text = data(
        "a",
        "## 節\n\n```view steps\nitems:\n  - title: t\n    refs: [C-core-cli-1, \"docs/decisions/core/cli.md#run-commands\", C-core-cli-999, \"docs/decisions/core/cli.md#none\", foo]\n```\n",
    );
    let overview = run(&model, &[(A, &text)]);
    let line = Some(line_of(&text, "```view steps"));
    assert_eq!(
        found(&overview),
        [
            (
                A.into(),
                line,
                "overview_ref_unresolved".into(),
                "C-core-cli-999".into()
            ),
            (
                A.into(),
                line,
                "overview_ref_unresolved".into(),
                "docs/decisions/core/cli.md#none".into()
            ),
            (
                A.into(),
                line,
                "overview_ref_unresolved".into(),
                "foo".into()
            ),
        ]
    );
}

// @kotowari[C-core-ovdata-13]
#[test]
fn req_core_285_ref_fields_at_any_depth_are_references_and_invalid_parts_are_not_read() {
    let model = model();
    let text = data(
        "a",
        "## 節\n\n```view decisions\nroots:\n  - ref: nope-1\n    text: t\n    by: b\n    children:\n      - ref: nope-2\n        text: t\n        by: b\n```\n\n```view steps\nitems:\n  - refs: [nope-3]\n```\n",
    );
    let details: Vec<String> = of_kind(&run(&model, &[(A, &text)]), "overview_ref_unresolved")
        .into_iter()
        .map(|(_, _, _, detail)| detail)
        .collect();
    assert_eq!(details, ["nope-1", "nope-2"]);
}

// @kotowari[C-core-ovdata-14]
#[test]
fn ex_core_470_a_stale_guide_mark_in_overview_data_is_a_guide_stale_notice() {
    let model = model();
    let current = fingerprint(&model, "docs/decisions/core/cli.md#run-commands");
    let text = data(
        "a",
        &format!(
            "## 新しい節\n<!-- @kotowari[docs/decisions/core/cli.md#run-commands:{current}] -->\n\n文。\n\n## 古い節\n<!-- @kotowari[docs/decisions/core/cli.md#run-commands:00000000] -->\n\n文。\n"
        ),
    );
    let overview = run(&model, &[(A, &text)]);
    let stale = of_kind(&overview, "guide_stale");
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0].0, A);
    assert_eq!(overview.marks(), 2);
}

// @kotowari[C-core-ovdata-15]
#[test]
fn req_core_286_a_malformed_guide_mark_in_overview_data_is_an_invalid_marker() {
    let model = model();
    let text = data(
        "a",
        "## 節\n<!-- @kotowari[docs/decisions/core/cli.md#run-commands] -->\n",
    );
    let overview = run(&model, &[(A, &text)]);
    let invalid = of_kind(&overview, "invalid_marker");
    assert_eq!(invalid.len(), 1);
    assert_eq!(invalid[0].0, A);
}

// @kotowari[C-core-ovdata-5]
#[test]
fn ex_core_471_an_unknown_frontmatter_key_is_a_form_error() {
    let model = model();
    let text =
        format!("---\ndocuments:\n  - docs/decisions/core/cli.md\ntitle: x\n---\n\n# a\n\n{LEAD}");
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(
        of_kind(&overview, "overview_form_invalid"),
        [(
            A.into(),
            None,
            "overview_form_invalid".into(),
            "frontmatter".into()
        )]
    );
}

// @kotowari[C-core-forder-4, C-core-ovdata-5, C-core-ovdata-23]
#[test]
fn tbl_core_038_frontmatter_must_hold_only_a_nonempty_documents_list_of_strings() {
    let model = model();
    for front in [
        "",
        "---\n- a\n---\n",
        "---\ndocuments: [\n---\n",
        "---\nother: 1\n---\n",
        "---\ndocuments: []\n---\n",
        "---\ndocuments: docs/decisions/core/cli.md\n---\n",
        "---\ndocuments:\n  - 1\n---\n",
        // 古い鍵 "ir" は知らない鍵で、一覧も無い（C-core-ovdata-23）
        "---\nir:\n  - docs/decisions/core/cli.md\n---\n",
    ] {
        let text = format!("{front}\n# a\n\n{LEAD}");
        let forms = of_kind(&run(&model, &[(A, &text)]), "overview_form_invalid");
        assert!(!forms.is_empty(), "{front:?}");
        assert!(
            forms
                .iter()
                .all(|(_, line, _, detail)| line.is_none() && detail == "frontmatter"),
            "{front:?}: {forms:?}"
        );
    }
}

// @kotowari[C-core-ovdata-6]
#[test]
fn tbl_core_038_the_title_is_exactly_one_heading_of_depth_one() {
    let model = model();
    let none = format!("{FRONT}\n{LEAD}");
    assert!(!of_kind(&run(&model, &[(A, &none)]), "overview_form_invalid").is_empty());
    let two = data("a", "## 節\n\n# second\n");
    let forms = of_kind(&run(&model, &[(A, &two)]), "overview_form_invalid");
    assert_eq!(forms.len(), 1);
    assert_eq!(forms[0].1, Some(line_of(&two, "# second")));
    assert_eq!(forms[0].3, "multiple_titles");
}

// @kotowari[C-core-ovdata-6]
#[test]
fn tbl_core_038_before_the_first_section_only_parts_and_comments_are_allowed() {
    let model = model();
    let text =
        format!("{FRONT}\n# a\n\n{LEAD}\n文がある。\n\n```rust\nfn x() {{}}\n```\n\n## 節\n");
    let forms = of_kind(&run(&model, &[(A, &text)]), "overview_form_invalid");
    let lines: Vec<Option<usize>> = forms.iter().map(|(_, line, _, _)| *line).collect();
    assert_eq!(
        lines,
        [
            Some(line_of(&text, "文がある。")),
            Some(line_of(&text, "```rust"))
        ]
    );
    let more_parts = format!(
        "{FRONT}\n# a\n\n{LEAD}\n<!-- c -->\n```view steps\nitems:\n  - title: t\n```\n\n## 節\n"
    );
    assert_eq!(found(&run(&model, &[(A, &more_parts)])), []);
}

// @kotowari[C-core-ovdata-6]
#[test]
fn tbl_core_038_a_section_takes_text_lists_tables_code_comments_and_subheadings() {
    let model = model();
    let text = data(
        "a",
        "## 節\n\n文。\n\n- 箇条\n  - 入れ子\n\n1. 番号\n2. 番号\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n```sh\necho\n```\n\n<!-- comment -->\n\n### 小見出し\n\n文。\n",
    );
    assert_eq!(found(&run(&model, &[(A, &text)])), []);
    let deep = data("a", "## 節\n\n#### 深い\n");
    let forms = of_kind(&run(&model, &[(A, &deep)]), "overview_form_invalid");
    assert_eq!(forms.len(), 1);
    assert_eq!(forms[0].1, Some(line_of(&deep, "#### 深い")));
}

// @kotowari[C-core-ovdata-7, C-core-ovdata-8]
#[test]
fn tbl_core_038_a_part_is_a_fence_whose_info_string_is_view_and_a_kind() {
    let model = model();
    let text = data(
        "a",
        "## 節\n\n```view   chart\nx: 1\n```\n\n```viewer chart\nx: 1\n```\n\n```yaml\nview: chart\n```\n",
    );
    let unknown = of_kind(&run(&model, &[(A, &text)]), "overview_part_unknown");
    assert_eq!(unknown.len(), 1);
    assert_eq!(unknown[0].3, "chart");
}

// @kotowari[C-core-forder-4, C-core-ovcmd-24]
#[test]
fn req_core_305_duplicated_names_and_index_and_style_are_errors() {
    let model = model();
    let one = |document: &str, title: &str| {
        format!("---\ndocuments:\n  - {document}\n---\n\n# {title}\n\n{LEAD}")
    };
    let a1 = one("docs/decisions/core/cli.md", "a1");
    let a2 = one("docs/decisions/core/other.md", "a2");
    let overview = run(
        &model,
        &[
            (".kotowari/overview/a.md", &a1),
            (".kotowari/overview/sub/a.md", &a2),
            (
                ".kotowari/overview/index.md",
                &one("docs/decisions/core/none1.md", "i"),
            ),
            (
                ".kotowari/overview/style.md",
                &one("docs/decisions/core/none2.md", "s"),
            ),
        ],
    );
    assert_eq!(
        of_kind(&overview, "overview_name_conflict"),
        [
            (
                ".kotowari/overview/index.md".into(),
                None,
                "overview_name_conflict".into(),
                "index".into()
            ),
            (
                ".kotowari/overview/style.md".into(),
                None,
                "overview_name_conflict".into(),
                "style".into()
            ),
            (
                ".kotowari/overview/sub/a.md".into(),
                None,
                "overview_name_conflict".into(),
                "a".into()
            ),
        ]
    );
}

fn reference<'a>(overview: &'a Overview, key: &str) -> &'a kotowari_markdown_view::Reference {
    overview
        .render_input()
        .references
        .iter()
        .find(|reference| reference.key == key)
        .unwrap_or_else(|| panic!("no reference {key}"))
}

fn with_refs(refs: &str) -> String {
    data(
        "a",
        &format!("## 節\n\n```view steps\nitems:\n  - title: t\n    refs: [{refs}]\n```\n"),
    )
}

// @kotowari[C-core-ovcmd-29]
#[test]
fn references_to_proposed_decisions_and_their_rows_are_deferred_and_the_rest_current() {
    let model = model();
    let text = with_refs(
        "\"docs/decisions/core/cli.md#run-commands\", C-core-cli-1, \"docs/decisions/core/cli.md#later\", C-core-cli-2",
    );
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(found(&overview), []);
    let states: Vec<(&str, ReferenceState)> = overview
        .render_input()
        .references
        .iter()
        .map(|reference| (reference.key.as_str(), reference.state))
        .collect();
    assert_eq!(
        states,
        [
            ("C-core-cli-1", ReferenceState::Current),
            ("C-core-cli-2", ReferenceState::Deferred),
            ("docs/decisions/core/cli.md#later", ReferenceState::Deferred),
            (
                "docs/decisions/core/cli.md#run-commands",
                ReferenceState::Current
            ),
        ]
    );
}

// @kotowari[C-core-ovcmd-14]
#[test]
fn a_reference_to_a_decision_or_a_row_carries_the_contents_of_its_target() {
    let model = model();
    let text = with_refs("\"docs/decisions/core/cli.md#run-commands\", C-core-cli-1");
    let overview = run(&model, &[(A, &text)]);
    let decision = reference(&overview, "docs/decisions/core/cli.md#run-commands");
    assert_eq!(decision.label, "Run commands");
    assert_eq!(
        decision.body.as_deref(),
        Some(
            "## Run commands\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nRun them.\n\n\
             ### Confirmations\n\n- C-core-cli-1 A command runs\n  and ends"
        )
    );
    let row = reference(&overview, "C-core-cli-1");
    assert_eq!(row.label, "C-core-cli-1");
    assert_eq!(
        row.body.as_deref(),
        Some("- C-core-cli-1 A command runs\n  and ends")
    );
}

// @kotowari[C-core-lang-9, C-core-ovcmd-15]
#[test]
fn a_section_is_stale_when_the_decision_its_mark_points_at_changes() {
    let model = model();
    let target = "docs/decisions/core/cli.md#run-commands";
    let listed = model.list();
    let current = listed
        .items()
        .iter()
        .find_map(|item| {
            let kotowari_core::ListItem::Decision(decision) = item else {
                return None;
            };
            (decision.id() == target).then(|| decision.fingerprint().to_string())
        })
        .unwrap();
    let text = data(
        "a",
        &format!(
            "## 今の節\n<!-- @kotowari[{target}:{current}] -->\n\n文。\n\n## 古い節\n<!-- @kotowari[{target}:00000000] -->\n\n文。\n"
        ),
    );
    let overview = run(&model, &[(A, &text)]);
    let sections: Vec<(&str, bool)> = overview.render_input().documents[0]
        .sections
        .iter()
        .map(|section| (section.heading.as_str(), section.stale))
        .collect();
    assert_eq!(sections, [("今の節", false), ("古い節", true)]);
}

// @kotowari[C-core-ovcmd-13]
#[test]
fn req_core_291_one_entry_per_distinct_reference_across_all_overviews() {
    let model = model();
    let a = with_refs("C-core-cli-1, C-core-cli-1");
    let b = "---\ndocuments:\n  - docs/decisions/core/other.md\n---\n\n# b\n\n```view quiz\nitems:\n  - q: q\n    a: a\n    refs: [C-core-cli-1, C-core-other-1]\n```\n".to_string();
    let overview = run(&model, &[(A, &a), (B, &b)]);
    let keys: Vec<&str> = overview
        .render_input()
        .references
        .iter()
        .map(|reference| reference.key.as_str())
        .collect();
    assert_eq!(keys, ["C-core-cli-1", "C-core-other-1"]);
}

// @kotowari[C-core-ovcmd-15]
#[test]
fn req_core_292_a_section_is_stale_only_when_a_mark_in_it_is_stale() {
    let model = model();
    let current = fingerprint(&model, "docs/decisions/core/cli.md#run-commands");
    let text = format!(
        "{FRONT}\n# a\n\n<!-- @kotowari[docs/decisions/core/cli.md#run-commands:00000000] -->\n{LEAD}\n## 新しい節 <!-- note -->\n<!-- @kotowari[docs/decisions/core/cli.md#run-commands:{current}] -->\n\n文。\n\n## 古い節\n<!-- @kotowari[docs/decisions/core/cli.md#run-commands:00000000] -->\n\n文。\n\n## 印の無い節\n\n文。\n"
    );
    let overview = run(&model, &[(A, &text)]);
    let document = &overview.render_input().documents[0];
    let sections: Vec<(&str, bool)> = document
        .sections
        .iter()
        .map(|section| (section.heading.as_str(), section.stale))
        .collect();
    assert_eq!(
        sections,
        [("新しい節", false), ("古い節", true), ("印の無い節", false)]
    );
}

// @kotowari[C-core-ovcmd-15]
#[test]
fn req_core_292_only_html_comments_are_removed_from_heading_text() {
    // 型引数のような山括弧は行の中の HTML として読まれるが、消すのはコメントだけ
    let model = model();
    let text = data(
        "Option<T> の扱い",
        "## Vec<String> の扱い <!-- note -->\n\n文。\n",
    );
    let overview = run(&model, &[(A, &text)]);
    let document = &overview.render_input().documents[0];
    assert_eq!(document.title, "Option<T> の扱い");
    assert_eq!(document.sections[0].heading, "Vec<String> の扱い");
}

// @kotowari[C-view-rendering-1]
#[test]
fn valid_data_renders_into_the_pages_the_view_returns_for_its_input() {
    let model = model();
    let a = data(
        "一つ目",
        "## 節\n\n文。\n\n```view steps\nitems:\n  - title: 段階\n    refs: [C-core-cli-1]\n```\n\n後の文。\n",
    );
    let b = format!("---\ndocuments:\n  - docs/decisions/core/other.md\n---\n\n# 二つ目\n\n{LEAD}");
    let overview = run(&model, &[(A, &a), (B, &b)]);
    let pages = overview.pages().expect("no errors");
    let names: Vec<&str> = pages.iter().map(|page| page.name.as_str()).collect();
    assert_eq!(names, ["a.html", "b.html", "index.html", "style.css"]);
    assert_eq!(
        pages,
        kotowari_markdown_view::render(overview.render_input())
    );
    let document = &overview.render_input().documents[0];
    assert_eq!(
        (document.name.as_str(), document.title.as_str()),
        ("a", "一つ目")
    );
    assert_eq!(document.lead.kind, "lead");
    let blocks = &document.sections[0].blocks;
    assert!(
        matches!(&blocks[0], kotowari_markdown_view::Block::Markdown(text) if text.trim() == "文。")
    );
    assert!(
        matches!(&blocks[1], kotowari_markdown_view::Block::Part(part) if part.kind == "steps")
    );
    assert!(
        matches!(&blocks[2], kotowari_markdown_view::Block::Markdown(text) if text.trim() == "後の文。")
    );
}

// @kotowari[C-view-rendering-10]
#[test]
fn parts_after_the_lead_and_before_the_first_section_reach_the_page_in_order() {
    let model = model();
    let text = data(
        "a",
        "<!-- c -->\n```view steps\nitems:\n  - title: 冒頭の一つ目\n```\n\n```view cards\ncards:\n  - title: 冒頭の二つ目\n    items: []\n```\n\n## 最初の節の見出し\n\n文。\n",
    );
    let overview = run(&model, &[(A, &text)]);
    let pages = overview.pages().expect("no errors");
    let page = &pages
        .iter()
        .find(|page| page.name == "a.html")
        .unwrap()
        .content;
    let at = |needle: &str| page.find(needle).unwrap_or_else(|| panic!("{needle}"));
    // アウトラインの項目は、ページの中の節へのリンクで、lead の前にある。節の見出しはリンク先の場所の後にある
    let link = at("href=\"#");
    let place = &page[link + "href=\"#".len()..];
    let id = &place[..place.find('"').expect("link end")];
    let section = at(&format!("id=\"{id}\""));
    let heading = section + page[section..].find("最初の節の見出し").expect("heading");
    assert!(link < at("結論"));
    assert!(at("結論") < at("冒頭の一つ目"));
    assert!(at("冒頭の一つ目") < at("冒頭の二つ目"));
    assert!(at("冒頭の二つ目") < heading);
}

// @kotowari[C-core-ovcmd-8]
#[test]
fn data_with_errors_gives_no_pages_and_the_error_count() {
    let model = model();
    let text = format!("{FRONT}\n# a\n\n## 節\n");
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(overview.pages(), Err(1));
    assert_eq!(overview.errors(), 1);
}

// @kotowari[C-core-ovout-1]
#[test]
fn the_findings_and_numbers_become_the_overview_group() {
    let model = model();
    let current = fingerprint(&model, "docs/decisions/core/cli.md#run-commands");
    let text = data(
        "a",
        &format!(
            "## 節\n<!-- @kotowari[docs/decisions/core/cli.md#run-commands:{current}, docs/decisions/core/other.md#rest:00000000] -->\n"
        ),
    );
    let overview = run(&model, &[(A, &text), (B, "no frontmatter\n")]);
    let errors = overview.errors();
    let group = overview.into_group();
    assert_eq!(group.tally().name(), "overview");
    assert_eq!((group.tally().files(), group.tally().marks()), (2, 2));
    assert_eq!(
        group
            .findings()
            .iter()
            .filter(|finding| finding.severity() == "error")
            .count(),
        errors
    );
}

// @kotowari[C-core-ovdata-11]
#[test]
fn req_core_283_a_second_title_before_the_lead_is_the_first_block_after_the_title() {
    let model = model();
    let text = format!("{FRONT}\n# a\n\n# b\n\n{LEAD}\n## 節\n\n文。\n");
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(of_kind(&overview, "overview_lead_missing").len(), 1);
}

// @kotowari[C-core-ovdata-12]
#[test]
fn req_core_284_one_file_listing_a_document_twice_does_not_share_it() {
    let model = model();
    let text = format!(
        "---\ndocuments:\n  - docs/decisions/core/cli.md\n  - docs/decisions/core/cli.md\n---\n\n# a\n\n{LEAD}"
    );
    let overview = run(&model, &[(A, &text)]);
    assert_eq!(of_kind(&overview, "overview_document_shared"), []);
}

// @kotowari[C-core-ovout-1]
#[test]
fn req_core_288_the_number_of_files_counts_every_overview_data_file() {
    let model = model();
    let a = data("a", "## 節\n\n文。\n");
    let b = format!("---\ndocuments:\n  - docs/decisions/core/other.md\n---\n\n# b\n\n{LEAD}");
    assert_eq!(run(&model, &[]).files(), 0);
    assert_eq!(run(&model, &[(A, &a), (B, &b)]).files(), 2);
}

/// 名前の元データを、ほかに指摘の出ない形で並べる
fn named(names: &[&'static str]) -> Vec<(String, String)> {
    names
        .iter()
        .map(|name| {
            (
                format!(".kotowari/overview/{name}.md"),
                data(&format!("題名{name}"), ""),
            )
        })
        .collect()
}

fn run_named(names: &[&'static str], toc: &str) -> Overview {
    let files = named(names);
    let files: Vec<(&str, &str)> = files
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    // 元データは別の IR の文書を持たないので、2つ目以降は overview_document_shared になる。目次の指摘だけを見る
    run_with_toc(&model(), &files, toc)
}

/// 目次の指摘だけの、種類と detail
fn toc_findings(overview: &Overview) -> Vec<(String, String)> {
    found(overview)
        .into_iter()
        .filter(|(_, _, kind, _)| kind.starts_with("overview_toc_"))
        .map(|(path, line, kind, detail)| {
            assert_eq!(path, TOC);
            assert_eq!(line, None, "{kind} {detail}");
            (kind, detail)
        })
        .collect()
}

fn pairs(expected: &[(&str, &str)]) -> Vec<(String, String)> {
    expected
        .iter()
        .map(|(kind, detail)| (kind.to_string(), detail.to_string()))
        .collect()
}

// @kotowari[C-core-forder-4, C-core-ovtoc-7, C-core-ovtoc-12]
#[test]
fn ex_core_507_an_unknown_key_in_a_group_is_a_form_error_and_stops_the_comparison() {
    let overview = run_named(
        &["a"],
        "title: 目次\nitems:\n  - title: 群\n    items: [x]\n    color: red\n",
    );
    assert_eq!(
        toc_findings(&overview),
        pairs(&[("overview_toc_invalid", "/items/0/color")])
    );
}

// @kotowari[C-core-forder-4, C-core-ovtoc-7]
#[test]
fn tbl_core_043_each_form_violation_names_its_place() {
    for (toc, places) in [
        ("items: [a]\n", vec!["/title"]),
        ("title: 目次\n", vec!["/items"]),
        ("title: ''\nitems: [a]\n", vec!["/title"]),
        (
            "title: 目次\nnote: \"一行目\\n二行目\"\nitems: [a]\n",
            vec!["/note"],
        ),
        ("title: 目次\nitems: [a, '']\n", vec!["/items/1"]),
        ("title: 目次\nitems: a\n", vec!["/items"]),
        ("title: 目次\nitems: [a, 3]\n", vec!["/items/1"]),
        (
            "title: 目次\nitems: [a, {items: [b]}]\n",
            vec!["/items/1/title"],
        ),
        ("title: [目次\nitems: [a\n", vec!["(yaml)"]),
        ("- a\n- b\n", vec!["(root)"]),
        ("", vec!["(root)"]),
        (
            "title: 目次\nitems: [a]\nextra: 1\nmore: 2\n",
            vec!["/extra", "/more"],
        ),
    ] {
        let overview = run_named(&["a"], toc);
        let mut found: Vec<String> = toc_findings(&overview)
            .into_iter()
            .map(|(kind, detail)| {
                assert_eq!(kind, "overview_toc_invalid", "{toc:?}");
                detail
            })
            .collect();
        found.sort();
        assert_eq!(found, places, "{toc:?}");
    }
}

// @kotowari[C-core-ovtoc-8]
#[test]
fn tbl_core_043_a_well_formed_toc_with_a_note_and_nested_groups_has_no_finding() {
    let overview = run_named(
        &["a", "b"],
        "title: 目次\nnote: 一行の説明\nitems:\n  - a\n  - title: 群\n    note: ''\n    items: [b]\n",
    );
    assert_eq!(toc_findings(&overview), pairs(&[]));
}

// @kotowari[C-core-forder-4, C-core-ovtoc-9]
#[test]
fn ex_core_508_missing_unknown_and_repeated_names_are_errors() {
    let overview = run_named(&["a", "b", "c"], "title: 目次\nitems: [a, z, a]\n");
    let mut findings = toc_findings(&overview);
    findings.sort();
    assert_eq!(
        findings,
        pairs(&[
            ("overview_toc_page_duplicate", "/items/2"),
            ("overview_toc_page_missing", "b"),
            ("overview_toc_page_missing", "c"),
            ("overview_toc_page_unknown", "/items/1"),
        ])
    );
}

// @kotowari[C-core-ovtoc-9]
#[test]
fn req_core_329_an_unknown_name_repeated_is_only_unknown_each_time() {
    let overview = run_named(
        &["a"],
        "title: 目次\nitems: [a, z, {title: 群, items: [z]}]\n",
    );
    let mut findings = toc_findings(&overview);
    findings.sort();
    assert_eq!(
        findings,
        pairs(&[
            ("overview_toc_page_unknown", "/items/1"),
            ("overview_toc_page_unknown", "/items/2/items/0"),
        ])
    );
}

// @kotowari[C-core-ovtoc-9]
#[test]
fn req_core_329_the_second_place_is_found_depth_first_in_written_order() {
    let overview = run_named(
        &["a"],
        "title: 目次\nitems:\n  - title: 前\n    items: [{title: 内, items: [a]}]\n  - a\n",
    );
    assert_eq!(
        toc_findings(&overview),
        pairs(&[("overview_toc_page_duplicate", "/items/1")])
    );
}

// @kotowari[C-core-forder-4, C-core-ovtoc-10]
#[test]
fn ex_core_509_a_group_without_items_is_an_error() {
    let overview = run_named(&["a"], "title: 目次\nitems: [a, {title: 空, items: []}]\n");
    assert_eq!(
        toc_findings(&overview),
        pairs(&[("overview_toc_group_empty", "/items/1")])
    );
}

// @kotowari[C-core-ovtoc-10]
#[test]
fn req_core_330_an_empty_outermost_group_is_reported_at_the_root() {
    let overview = run_named(&["a"], "title: 目次\nitems: []\n");
    let mut findings = toc_findings(&overview);
    findings.sort();
    assert_eq!(
        findings,
        pairs(&[
            ("overview_toc_group_empty", "(root)"),
            ("overview_toc_page_missing", "a"),
        ])
    );
}

// @kotowari[C-core-ovtoc-13]
#[test]
fn req_core_332_the_render_input_carries_the_toc_as_written() {
    use kotowari_markdown_view::{TocGroup, TocItem};
    let overview = run_named(
        &["a", "b", "c"],
        "title: 目次\nnote: 説明\nitems:\n  - c\n  - title: 群\n    items: [b]\n  - a\n",
    );
    assert_eq!(
        overview.render_input().toc,
        TocGroup {
            title: "目次".into(),
            note: Some("説明".into()),
            items: vec![
                TocItem::Document("c".into()),
                TocItem::Group(TocGroup {
                    title: "群".into(),
                    note: None,
                    items: vec![TocItem::Document("b".into())],
                }),
                TocItem::Document("a".into()),
            ],
        }
    );
}

// @kotowari[C-core-ovtoc-11]
#[test]
fn req_core_294_toc_errors_count_as_errors_and_block_the_pages() {
    let overview = run_named(&["a"], "title: 目次\nitems: [z]\n");
    assert!(overview.errors() >= 2);
    assert_eq!(overview.pages(), Err(overview.errors()));
}
