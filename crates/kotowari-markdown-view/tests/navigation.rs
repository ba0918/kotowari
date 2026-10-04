use kotowari_markdown_view::{
    Block, Document, Page, Part, RenderInput, Section, TocGroup, TocItem, render,
};
use serde_json::json;

fn lead(conclusion: &str) -> Part {
    Part {
        kind: "lead".into(),
        value: json!({"conclusion": conclusion}),
    }
}

fn status(states: &[&str]) -> Part {
    let items: Vec<_> = states
        .iter()
        .map(|state| json!({"state": state, "text": "文"}))
        .collect();
    Part {
        kind: "status".into(),
        value: json!({"items": items}),
    }
}

fn section(stale: bool, blocks: Vec<Block>) -> Section {
    Section {
        heading: "節".into(),
        stale,
        blocks,
    }
}

fn document(name: &str, sections: Vec<Section>) -> Document {
    Document {
        name: name.into(),
        title: format!("題名{name}"),
        lead: lead(&format!("{name}の結論")),
        preamble: vec![],
        sections,
    }
}

fn name(name: &str) -> TocItem {
    TocItem::Document(name.into())
}

fn group(title: &str, items: Vec<TocItem>) -> TocItem {
    TocItem::Group(TocGroup {
        title: title.into(),
        note: None,
        items,
    })
}

fn input(documents: Vec<Document>, items: Vec<TocItem>) -> RenderInput {
    RenderInput {
        documents,
        references: vec![],
        toc: TocGroup {
            title: "kotowari".into(),
            note: None,
            items,
        },
    }
}

fn page<'a>(pages: &'a [Page], name: &str) -> &'a str {
    &pages
        .iter()
        .find(|page| page.name == name)
        .unwrap_or_else(|| panic!("no page {name}"))
        .content
}

fn index(input: &RenderInput) -> String {
    page(&render(input), "index.html").to_string()
}

/// 一覧の中の、その文書の項目。リンクから項目の終わりまで
fn entry<'a>(index: &'a str, name: &str) -> &'a str {
    let start = index
        .find(&format!("href=\"{name}.html\""))
        .unwrap_or_else(|| panic!("no entry {name}"));
    let end = start + index[start..].find("</li>").expect("entry end");
    &index[start..end]
}

/// 一覧の中の、題名がそれである目次の群の見出し
fn heading<'a>(index: &'a str, title: &str) -> &'a str {
    let start = index
        .find(&format!(">{title}<"))
        .unwrap_or_else(|| panic!("no heading {title}"));
    let end = start + index[start..].find("</summary>").expect("heading end");
    &index[start..end]
}

/// 一覧の見出し（目次そのものの題名と数）
fn top(index: &str) -> &str {
    let start = index.find("<h1").expect("title");
    let end = start + index[start..].find("</header>").expect("header end");
    &index[start..end]
}

// @kotowari[EX-view-011, REQ-view-016]
#[test]
fn ex_view_011_only_nonzero_state_counts_are_added_to_the_card() {
    let a = document(
        "a",
        vec![
            section(true, vec![Block::Part(status(&["未決", "決定"]))]),
            section(true, vec![]),
            section(false, vec![]),
        ],
    );
    let index = index(&input(vec![a], vec![name("a")]));
    let card = entry(&index, "a");
    assert!(card.contains("見直していない節 2"), "{card}");
    assert!(card.contains("未決 1"), "{card}");
    assert!(!index.contains("予定"));
}

// @kotowari[REQ-view-016]
#[test]
fn req_view_016_labels_are_counted_in_the_preamble_and_in_every_section_but_not_the_lead() {
    let mut a = document(
        "a",
        vec![
            section(false, vec![Block::Part(status(&["予定", "未決"]))]),
            section(
                false,
                vec![
                    Block::Markdown("未決 予定".into()),
                    Block::Part(status(&["予定"])),
                ],
            ),
        ],
    );
    a.preamble = vec![status(&["予定"])];
    a.lead = lead("結論");
    let index = index(&input(vec![a], vec![name("a")]));
    let card = entry(&index, "a");
    assert!(card.contains("予定 3"), "{card}");
    assert!(card.contains("未決 1"), "{card}");
    assert!(!card.contains("見直していない節"), "{card}");
}

// @kotowari[REQ-view-016]
#[test]
fn req_view_016_a_document_with_nothing_to_review_has_no_counts() {
    let a = document(
        "a",
        vec![section(false, vec![Block::Part(status(&["決定"]))])],
    );
    let index = index(&input(vec![a], vec![name("a")]));
    let card = entry(&index, "a");
    for word in ["見直していない節", "未決", "予定"] {
        assert!(!card.contains(word), "{word}: {card}");
    }
}

// @kotowari[EX-view-012, REQ-view-017]
#[test]
fn ex_view_012_group_counts_include_every_descendant_document() {
    let b = document(
        "b",
        vec![section(false, vec![Block::Part(status(&["未決", "未決"]))])],
    );
    let documents = vec![document("a", vec![]), b, document("c", vec![])];
    let index = index(&input(
        documents,
        vec![group(
            "テスト",
            vec![name("a"), group("変異テスト", vec![name("b"), name("c")])],
        )],
    ));
    let outer = heading(&index, "テスト");
    assert!(
        outer.contains("3 ページ") && outer.contains("未決 2"),
        "{outer}"
    );
    let inner = heading(&index, "変異テスト");
    assert!(
        inner.contains("2 ページ") && inner.contains("未決 2"),
        "{inner}"
    );
    assert!(!index.contains("見直していない節"));
}

// @kotowari[REQ-view-017]
#[test]
fn req_view_017_the_outermost_heading_counts_pages_and_sums_but_not_planned() {
    let a = document(
        "a",
        vec![
            section(true, vec![Block::Part(status(&["未決", "予定"]))]),
            section(true, vec![]),
        ],
    );
    let b = document("b", vec![section(true, vec![])]);
    let index = index(&input(
        vec![a, b],
        vec![group("群", vec![name("a")]), name("b")],
    ));
    let head = top(&index);
    for needle in ["kotowari", "2 ページ", "見直していない節 3", "未決 1"] {
        assert!(head.contains(needle), "{needle}: {head}");
    }
    assert!(!head.contains("予定"), "{head}");
    let inner = heading(&index, "群");
    assert!(inner.contains("1 ページ") && inner.contains("見直していない節 2"));
    assert!(!inner.contains("予定"), "{inner}");
}

// @kotowari[REQ-view-017]
#[test]
fn req_view_017_a_group_without_states_shows_only_its_page_count() {
    let index = index(&input(
        vec![document("a", vec![])],
        vec![group("群", vec![name("a")])],
    ));
    let inner = heading(&index, "群");
    assert!(inner.contains("1 ページ"), "{inner}");
    for word in ["見直していない節", "未決"] {
        assert!(!inner.contains(word), "{word}: {inner}");
    }
}

// @kotowari[EX-view-013, REQ-view-018]
#[test]
fn ex_view_013_groups_are_open_foldable_elements_and_no_page_has_a_script() {
    let input = input(
        vec![document("a", vec![]), document("b", vec![])],
        vec![group("外", vec![name("a"), group("内", vec![name("b")])])],
    );
    let pages = render(&input);
    let index = page(&pages, "index.html");
    // 目次そのものは畳めず、入れ子の2つの群だけが開いた状態の畳める要素になる
    assert_eq!(index.matches("<details").count(), 2);
    assert_eq!(index.matches("<details open").count(), 2);
    assert_eq!(index.matches("<summary").count(), 2);
    assert!(!top(index).contains("<summary"));
    for page in &pages {
        assert!(!page.content.contains("<script"), "{}", page.name);
    }
}

// @kotowari[REQ-view-021, REQ-view-002]
#[test]
fn req_view_021_mismatched_contents_are_drawn_without_reporting() {
    let input = input(
        vec![document("a", vec![]), document("c", vec![])],
        vec![name("a"), name("z"), name("a")],
    );
    let pages = render(&input);
    let names: Vec<&str> = pages.iter().map(|page| page.name.as_str()).collect();
    assert_eq!(names, ["a.html", "c.html", "index.html", "style.css"]);
    let index = page(&pages, "index.html");
    assert_eq!(index.matches("題名a").count(), 2);
    assert_eq!(index.matches("href=\"a.html\"").count(), 2);
    assert!(!index.contains("題名c") && !index.contains("c.html"));
    assert!(!index.contains("z.html") && !index.contains(">z<"));
    assert!(top(index).contains("2 ページ"));
}

// @kotowari[REQ-view-021, REQ-view-017]
#[test]
fn req_view_021_names_without_a_document_are_not_counted_in_a_group() {
    let index = index(&input(
        vec![document("a", vec![])],
        vec![group("群", vec![name("z"), name("a"), name("y")])],
    ));
    assert!(heading(&index, "群").contains("1 ページ"));
    assert!(top(&index).contains("1 ページ"));
}
