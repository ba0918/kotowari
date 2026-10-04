use kotowari_markdown_view::{
    Block, Document, Page, Part, Reference, ReferenceState, RenderInput, Section, render,
};
use serde_json::json;

fn lead(conclusion: &str) -> Part {
    Part {
        kind: "lead".into(),
        value: json!({"conclusion": conclusion, "points": ["要点"]}),
    }
}

fn document(name: &str, title: &str, sections: Vec<Section>) -> Document {
    Document {
        name: name.into(),
        title: title.into(),
        lead: lead(&format!("{title}の結論")),
        sections,
    }
}

fn section(heading: &str, blocks: Vec<Block>) -> Section {
    Section {
        heading: heading.into(),
        stale: false,
        blocks,
    }
}

fn page<'a>(pages: &'a [Page], name: &str) -> &'a str {
    &pages
        .iter()
        .find(|page| page.name == name)
        .unwrap_or_else(|| panic!("no page {name}"))
        .content
}

fn position(text: &str, needle: &str) -> usize {
    text.find(needle)
        .unwrap_or_else(|| panic!("{needle} not in page"))
}

// @kotowari[EX-view-001, REQ-view-002, REQ-view-005]
#[test]
fn ex_view_001_two_documents_give_four_pages_and_an_ordered_index() {
    let input = RenderInput {
        documents: vec![
            document("guides", "ガイドの印", vec![]),
            document("changes", "変更照合", vec![]),
        ],
        references: vec![],
    };
    let pages = render(&input);
    let names: Vec<&str> = pages.iter().map(|page| page.name.as_str()).collect();
    assert_eq!(
        names,
        ["changes.html", "guides.html", "index.html", "style.css"]
    );
    let index = page(&pages, "index.html");
    assert!(position(index, "変更照合") < position(index, "ガイドの印"));
    assert!(position(index, "変更照合の結論") < position(index, "ガイドの印の結論"));
    assert!(index.contains("href=\"changes.html\""));
    assert!(index.contains("href=\"guides.html\""));
}

// @kotowari[REQ-view-002]
#[test]
fn req_view_002_pages_refer_to_the_shared_style_and_to_each_other_relatively() {
    let input = RenderInput {
        documents: vec![document("changes", "変更照合", vec![])],
        references: vec![],
    };
    let pages = render(&input);
    for name in ["index.html", "changes.html"] {
        assert!(page(&pages, name).contains("href=\"style.css\""), "{name}");
    }
    assert!(page(&pages, "changes.html").contains("href=\"index.html\""));
    assert!(!page(&pages, "style.css").is_empty());
}

// @kotowari[REQ-view-002]
#[test]
fn req_view_002_no_documents_still_give_the_index_and_the_style() {
    let pages = render(&RenderInput::default());
    let names: Vec<&str> = pages.iter().map(|page| page.name.as_str()).collect();
    assert_eq!(names, ["index.html", "style.css"]);
}

// @kotowari[REQ-view-001]
#[test]
fn req_view_001_the_input_carries_documents_sections_and_the_reference_table() {
    let input = RenderInput {
        documents: vec![document(
            "a",
            "題名A",
            vec![section(
                "節の見出し",
                vec![Block::Markdown("段落の文".into())],
            )],
        )],
        references: vec![Reference {
            key: "REQ-x-001".into(),
            label: "REQ-x-001".into(),
            body: "要求の文".into(),
            state: ReferenceState::Current,
        }],
    };
    let text = page(&render(&input), "a.html").to_string();
    for needle in ["題名A", "題名Aの結論", "節の見出し", "段落の文"] {
        assert!(text.contains(needle), "{needle}");
    }
}

// @kotowari[REQ-view-006]
#[test]
fn req_view_006_the_lead_follows_the_title_and_sections_keep_their_order() {
    let input = RenderInput {
        documents: vec![document(
            "a",
            "題名A",
            vec![
                section("二番目ではない最初の節", vec![]),
                section("後の節", vec![]),
            ],
        )],
        references: vec![],
    };
    let text = page(&render(&input), "a.html").to_string();
    let title = position(&text, "<h1>題名A</h1>");
    let lead = position(&text, "題名Aの結論");
    let first = position(&text, "二番目ではない最初の節");
    let second = position(&text, "後の節");
    assert!(title < lead && lead < first && first < second);
}

fn one_section_page(blocks: Vec<Block>) -> String {
    let input = RenderInput {
        documents: vec![document("a", "題名", vec![section("節", blocks)])],
        references: vec![],
    };
    page(&render(&input), "a.html").to_string()
}

// @kotowari[EX-view-003, REQ-view-007]
#[test]
fn ex_view_003_raw_html_is_text_and_comments_are_dropped() {
    let text = one_section_page(vec![Block::Markdown(
        "前の文\n\n<script>x</script>\n\n<!-- @kotowari[REQ-core-001:00000000] -->\n\n後の文 <!-- inline --> 続き\n"
            .into(),
    )]);
    assert!(!text.contains("<script>"));
    assert!(text.contains("&lt;script&gt;"));
    assert!(!text.contains("@kotowari["));
    assert!(!text.contains("inline"));
    assert!(text.contains("後の文"));
}

// @kotowari[REQ-view-007]
#[test]
fn req_view_007_markdown_text_follows_commonmark_and_gfm_tables() {
    let text = one_section_page(vec![Block::Markdown(
        "### 小見出し\n\n- 項目\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n`code` と **強調**\n".into(),
    )]);
    for needle in [
        "<h3>小見出し</h3>",
        "<li>項目</li>",
        "<table>",
        "<td>2</td>",
        "<code>code</code>",
        "<strong>強調</strong>",
    ] {
        assert!(text.contains(needle), "{needle}");
    }
}
