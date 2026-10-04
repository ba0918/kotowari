use kotowari_markdown_view::{
    Block, Document, Page, Part, Reference, ReferenceState, RenderInput, Section, TocGroup,
    TocItem, render,
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
        preamble: vec![],
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

// @kotowari[EX-view-001, REQ-view-002]
#[test]
fn ex_view_001_two_documents_give_four_pages() {
    let input = RenderInput {
        documents: vec![
            document("guides", "ガイドの印", vec![]),
            document("changes", "変更照合", vec![]),
        ],
        references: vec![],
        toc: contents(&["guides", "changes"]),
    };
    let pages = render(&input);
    let names: Vec<&str> = pages.iter().map(|page| page.name.as_str()).collect();
    assert_eq!(
        names,
        ["changes.html", "guides.html", "index.html", "style.css"]
    );
}

/// 文書の名前を1段に並べた目次
fn contents(names: &[&str]) -> TocGroup {
    TocGroup {
        title: "目次".into(),
        note: None,
        items: names
            .iter()
            .map(|name| TocItem::Document((*name).into()))
            .collect(),
    }
}

// @kotowari[EX-view-010, REQ-view-005]
#[test]
fn ex_view_010_the_index_follows_the_contents_order_and_nesting() {
    let input = RenderInput {
        documents: vec![
            document("a", "題名A", vec![]),
            document("b", "題名B", vec![]),
        ],
        references: vec![],
        toc: TocGroup {
            title: "kotowari".into(),
            note: None,
            items: vec![
                TocItem::Group(TocGroup {
                    title: "テスト".into(),
                    note: Some("テストとの対応".into()),
                    items: vec![TocItem::Document("b".into())],
                }),
                TocItem::Document("a".into()),
            ],
        },
    };
    let pages = render(&input);
    let index = page(&pages, "index.html");
    assert!(index.contains("<h1>kotowari</h1>"));
    // "テスト" の群は、その見出しから群の終わりまでに b を持ち、a はその後にある
    let group = position(index, ">テスト<");
    let end = group + position(&index[group..], "</details>");
    let b = position(index, "題名B");
    let b_conclusion = position(index, "題名Bの結論");
    let note = position(index, "テストとの対応");
    assert!(group < note && note < b && b < b_conclusion && b_conclusion < end);
    assert!(end < position(index, "題名A") && end < position(index, "題名Aの結論"));
    assert!(index.contains("<a href=\"b.html\"><span class=\"title\">題名B</span></a>"));
    assert!(index.contains("<a href=\"a.html\"><span class=\"title\">題名A</span></a>"));
}

// @kotowari[REQ-view-005]
#[test]
fn req_view_005_the_note_of_the_contents_is_drawn_under_the_index_heading() {
    let mut toc = contents(&["a"]);
    toc.note = Some("製品の地図".into());
    let input = RenderInput {
        documents: vec![document("a", "題名A", vec![])],
        references: vec![],
        toc,
    };
    let index = page(&render(&input), "index.html").to_string();
    let heading = position(&index, "<h1>目次</h1>");
    let note = position(&index, "製品の地図");
    assert!(heading < note && note < position(&index, "題名A"));
}

// @kotowari[REQ-view-002]
#[test]
fn req_view_002_pages_refer_to_the_shared_style_and_to_each_other_relatively() {
    let input = RenderInput {
        documents: vec![document("changes", "変更照合", vec![])],
        references: vec![],
        toc: contents(&["changes"]),
    };
    let pages = render(&input);
    assert!(page(&pages, "index.html").contains("href=\"changes.html\""));
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
fn req_view_001_the_input_carries_documents_sections_the_reference_table_and_the_contents() {
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
        toc: TocGroup {
            title: "目次の題名".into(),
            note: Some("目次の説明".into()),
            items: vec![TocItem::Group(TocGroup {
                title: "群の題名".into(),
                note: None,
                items: vec![TocItem::Document("a".into())],
            })],
        },
    };
    let pages = render(&input);
    let text = page(&pages, "a.html");
    for needle in ["題名A", "題名Aの結論", "節の見出し", "段落の文"] {
        assert!(text.contains(needle), "{needle}");
    }
    let index = page(&pages, "index.html");
    for needle in ["目次の題名", "目次の説明", "群の題名", "題名A"] {
        assert!(index.contains(needle), "{needle}");
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
        toc: TocGroup::default(),
    };
    let text = page(&render(&input), "a.html").to_string();
    let title = position(&text, "<h1>題名A</h1>");
    let lead = position(&text, "題名Aの結論");
    let first = position(&text, "二番目ではない最初の節");
    let second = position(&text, "後の節");
    assert!(title < lead && lead < first && first < second);
}

// @kotowari[REQ-view-006, REQ-view-001]
#[test]
fn req_view_006_preamble_parts_follow_the_lead_in_order_before_the_sections() {
    let steps = |title: &str| Part {
        kind: "steps".into(),
        value: json!({"items": [{"title": title}]}),
    };
    let mut a = document("a", "題名A", vec![section("最初の節", vec![])]);
    a.preamble = vec![steps("冒頭の一つ目"), steps("冒頭の二つ目")];
    let input = RenderInput {
        documents: vec![a],
        references: vec![],
        toc: TocGroup::default(),
    };
    let text = page(&render(&input), "a.html").to_string();
    let lead = position(&text, "題名Aの結論");
    let first = position(&text, "冒頭の一つ目");
    let second = position(&text, "冒頭の二つ目");
    let section = position(&text, "最初の節");
    assert!(lead < first && first < second && second < section);
}

fn one_section_page(blocks: Vec<Block>) -> String {
    let input = RenderInput {
        documents: vec![document("a", "題名", vec![section("節", blocks)])],
        references: vec![],
        toc: TocGroup::default(),
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
fn req_view_007_text_next_to_a_comment_is_not_drawn_as_code() {
    for source in [
        "<!-- c -->text\n",
        "  <!-- c -->  text\n",
        "<!-- a --> <!-- b -->\ttext\n",
        "<!--\nc\n-->     text\n",
        "> <!-- c -->    text\n",
        "- <!-- c -->     text\n",
    ] {
        let body = one_section_page(vec![Block::Markdown(source.into())]);
        assert!(body.contains("text"), "{source:?}");
        assert!(!body.contains("<pre"), "{source:?}");
        assert!(!body.contains("&lt;!--"), "{source:?}");
    }
}

/// 空白の並びを1つにまとめたページ。HTML では空白の並びは1つの空白と同じに描かれる
fn collapsed(page: &str) -> String {
    page.split_whitespace().collect::<Vec<_>>().join(" ")
}

// @kotowari[REQ-view-007]
#[test]
fn req_view_007_a_comment_spanning_lines_inside_a_paragraph_keeps_one_paragraph() {
    let with = one_section_page(vec![Block::Markdown("before <!--\nc\n--> AFTER\n".into())]);
    let without = one_section_page(vec![Block::Markdown("before AFTER\n".into())]);
    assert_eq!(collapsed(&with), collapsed(&without));
}

// @kotowari[REQ-view-007]
#[test]
fn req_view_007_a_comment_line_keeps_the_blocks_around_it_apart() {
    for (source, expected) in [
        ("para1\n<!-- c -->\npara2\n", "para1\n\npara2\n"),
        ("p\n<!-- c -->\n---\n", "p\n\n---\n"),
        ("> q\n<!-- c -->\n> r\n", "> q\n\n> r\n"),
    ] {
        let with = one_section_page(vec![Block::Markdown(source.into())]);
        let without = one_section_page(vec![Block::Markdown(expected.into())]);
        assert_eq!(collapsed(&with), collapsed(&without), "{source:?}");
    }
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

fn reference(key: &str, label: &str, body: &str, state: ReferenceState) -> Reference {
    Reference {
        key: key.into(),
        label: label.into(),
        body: body.into(),
        state,
    }
}

fn steps_with_refs(refs: &[&str]) -> Block {
    Block::Part(Part {
        kind: "steps".into(),
        value: json!({"items": [{"title": "段階", "refs": refs}]}),
    })
}

/// 2つのページが食い違う範囲の、left の中のバイトの位置。同じなら None
fn changed(left: &str, right: &str) -> Option<(usize, usize)> {
    let (a, b) = (left.as_bytes(), right.as_bytes());
    let prefix = a.iter().zip(b).take_while(|(x, y)| x == y).count();
    if prefix == a.len() && prefix == b.len() {
        return None;
    }
    let suffix = a[prefix..]
        .iter()
        .rev()
        .zip(b[prefix..].iter().rev())
        .take_while(|(x, y)| x == y)
        .count();
    Some((prefix, a.len() - suffix))
}

/// 参照を refs に持つ steps の部品1つの節を持つページ。参照の状態だけを変えて描き比べる
fn page_with_references(refs: &[&str], references: Vec<Reference>) -> String {
    let input = RenderInput {
        documents: vec![document(
            "a",
            "題名",
            vec![section("節", vec![steps_with_refs(refs)])],
        )],
        references,
        toc: TocGroup::default(),
    };
    page(&render(&input), "a.html").to_string()
}

// @kotowari[EX-view-004, REQ-view-008]
#[test]
fn ex_view_004_a_superseded_reference_opens_its_body_in_place_with_a_mark() {
    let with = |state| {
        page_with_references(
            &["docs/x.md#A1"],
            vec![reference("docs/x.md#A1", "x A1", "古い決定", state)],
        )
    };
    let text = with(ReferenceState::Superseded);
    let label = position(&text, "x A1");
    let body = position(&text, "古い決定");
    assert!(label < body);
    // 状態だけを変えると、参照の描き方のうち本文より前だけが変わる。それが置き換え済みの印である
    let (start, end) = changed(&text, &with(ReferenceState::Current)).expect("a mark");
    assert!(position(&text, "段階") < start && end <= body);
    assert!(!text.contains("href=\"docs/x.md"));
}

// @kotowari[REQ-view-008]
#[test]
fn req_view_008_deferred_references_are_marked_and_no_reference_links_out() {
    let refs = ["REQ-x-001", "REQ-x-002", "missing"];
    let with = |state| {
        page_with_references(
            &refs,
            vec![
                reference("REQ-x-001", "REQ-x-001", "後回しの要求", state),
                reference(
                    "REQ-x-002",
                    "REQ-x-002",
                    "今の要求",
                    ReferenceState::Current,
                ),
            ],
        )
    };
    let text = with(ReferenceState::Deferred);
    assert!(text.contains("後回しの要求") && text.contains("今の要求"));
    // 後回しの印は、その参照の本文より前に付き、置き換え済みの印とは別のものである
    let (start, end) = changed(&text, &with(ReferenceState::Current)).expect("a mark");
    assert!(position(&text, "段階") < start && end <= position(&text, "後回しの要求"));
    assert_ne!(text, with(ReferenceState::Superseded));
    // 参照はページの外へのリンクにならない。リンクの数は参照の無い部品のページと同じ
    let plain = page_with_references(&[], vec![]);
    assert_eq!(
        text.matches("href=").count(),
        plain.matches("href=").count()
    );
}

// @kotowari[EX-view-005, REQ-view-009]
#[test]
fn ex_view_005_only_the_stale_section_carries_the_mark() {
    let with = |stale: bool| {
        let mut a = section("節A", vec![]);
        a.stale = stale;
        let input = RenderInput {
            documents: vec![document("a", "題名", vec![a, section("節B", vec![])])],
            references: vec![],
            toc: TocGroup::default(),
        };
        page(&render(&input), "a.html").to_string()
    };
    let text = with(true);
    // 古いとしたことで変わるのは、節 A の見出しと節 B の見出しの間だけである
    let (start, end) = changed(&text, &with(false)).expect("a mark");
    assert!(position(&text, "節A") < start && end <= position(&text, "節B"));
}
