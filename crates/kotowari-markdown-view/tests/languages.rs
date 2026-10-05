//! ページの言語、UI の文字、ほかの言語へのリンク、本文の無い参照（docs/ir/view/languages.md）

use kotowari_markdown_view::{
    Block, Document, OtherLanguage, Page, Part, Reference, ReferenceState, RenderInput, Section,
    TocGroup, TocItem, render,
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[expect(
    dead_code,
    reason = "this file reads only the links of the shared page helpers"
)]
mod common;

/// TBL-view-002 の鍵と、数を入れるか
const KEYS: [(&str, bool); 17] = [
    ("language_name", false),
    ("index_link", false),
    ("pages", true),
    ("stale_sections", true),
    ("open_items", true),
    ("planned_items", true),
    ("stale_mark", false),
    ("outline_stale", false),
    ("superseded", false),
    ("deferred", false),
    ("compare_before", false),
    ("compare_after", false),
    ("compare_why", false),
    ("state_decided", false),
    ("state_planned", false),
    ("state_open", false),
    ("state_dropped", false),
];

/// すべての鍵の値を "X" と鍵の名前（数を入れる鍵では末尾に " {n}"）にした UI の文字
fn marked_ui() -> BTreeMap<String, String> {
    KEYS.iter()
        .map(|(key, counted)| {
            let count = if *counted { " {n}" } else { "" };
            (key.to_string(), format!("X{key}{count}"))
        })
        .collect()
}

fn part(kind: &str, value: Value) -> Part {
    Part {
        kind: kind.into(),
        value,
    }
}

fn lead() -> Part {
    part("lead", json!({"conclusion": "結論", "points": ["要点"]}))
}

fn document(name: &str, sections: Vec<Section>) -> Document {
    Document {
        name: name.into(),
        title: format!("題名{name}"),
        lead: lead(),
        preamble: vec![],
        sections,
    }
}

fn section(stale: bool, blocks: Vec<Block>) -> Section {
    Section {
        heading: "節".into(),
        stale,
        blocks,
    }
}

fn input(documents: Vec<Document>, ui: BTreeMap<String, String>) -> RenderInput {
    let items = documents
        .iter()
        .map(|document| TocItem::Document(document.name.clone()))
        .collect();
    RenderInput {
        documents,
        references: vec![],
        toc: TocGroup {
            title: "目次".into(),
            note: None,
            items,
        },
        language: "xx".into(),
        ui,
        others: vec![],
    }
}

fn page<'a>(pages: &'a [Page], name: &str) -> &'a str {
    &pages
        .iter()
        .find(|page| page.name == name)
        .unwrap_or_else(|| panic!("no page {name}"))
        .content
}

fn html_pages(pages: &[Page]) -> impl Iterator<Item = &Page> {
    pages.iter().filter(|page| page.name.ends_with(".html"))
}

/// "html" の要素の lang の属性の値
fn lang(html: &str) -> &str {
    let start = html.find("<html").expect("an html element");
    let tag = &html[start..start + html[start..].find('>').expect("tag end")];
    let value = &tag[tag.find("lang=\"").expect("a lang attribute") + 6..];
    &value[..value.find('"').expect("attribute end")]
}

// @kotowari[REQ-view-026, REQ-view-027, EX-view-018]
#[test]
fn ex_view_018_the_page_language_and_its_text_follow_the_input() {
    let mut ui = marked_ui();
    ui.insert("stale_mark".into(), "Pas encore relu".into());
    let mut input = input(vec![document("a", vec![section(true, vec![])])], ui);
    input.language = "fr".into();
    let pages = render(&input);
    for page in html_pages(&pages) {
        assert_eq!(lang(&page.content), "fr", "{}", page.name);
    }
    // 古い節の見出しは、アウトラインの項目の後に描かれる最後の "節" で、印はその後にある
    let a = page(&pages, "a.html");
    let heading = a.rfind("節").expect("the section heading");
    let mark = a.find("Pas encore relu").expect("the stale mark");
    assert!(mark > heading, "{a}");
}

/// 要素の印の外の文字の塊。文字の参照は戻す
fn text_nodes(html: &str) -> Vec<String> {
    let body = &html[html.find("<body>").expect("body")..];
    let mut nodes = Vec::new();
    let mut current = String::new();
    let mut in_tag = false;
    for c in body.chars() {
        match c {
            '<' => {
                in_tag = true;
                nodes.push(std::mem::take(&mut current));
            }
            '>' => in_tag = false,
            _ if !in_tag => current.push(c),
            _ => {}
        }
    }
    nodes
        .into_iter()
        .map(|node| {
            node.replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'")
                .replace("&amp;", "&")
                .trim()
                .to_string()
        })
        .filter(|node| !node.is_empty())
        .collect()
}

/// 値の中のすべての文字列
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "serde_json::Value is a foreign enum: the remaining JSON kinds are deliberately handled alike"
)]
fn strings(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(text) => out.push(text.clone()),
        Value::Array(items) => items.iter().for_each(|item| strings(item, out)),
        Value::Object(map) => map.values().for_each(|item| strings(item, out)),
        _ => {}
    }
}

// @kotowari[REQ-view-027, EX-view-019, TBL-view-002]
#[test]
fn ex_view_019_every_text_the_view_writes_comes_from_the_ui_text() {
    let refs = json!(["REQ-x-001", "docs/x.md#A1"]);
    let parts = vec![
        part(
            "flow",
            json!({"columns": [[{"title": "箱", "body": "箱の文"}]]}),
        ),
        part(
            "steps",
            json!({"items": [{"title": "段階", "body": "段階の文", "refs": refs}]}),
        ),
        part(
            "cards",
            json!({"cards": [{"title": "札", "items": ["札の項目"]}]}),
        ),
        part(
            "status",
            json!({"items": [
                {"state": "decided", "text": "決めた"}, {"state": "planned", "text": "予定の文"},
                {"state": "open", "text": "未決の文"}, {"state": "dropped", "text": "やめた"}
            ]}),
        ),
        part(
            "compare",
            json!({"items": [{"before": "前の形", "after": "後の形", "why": "わけ"}]}),
        ),
        part(
            "decisions",
            json!({"roots": [{"ref": "docs/x.md#A1", "text": "判断", "by": "利用者"}]}),
        ),
        part("quiz", json!({"items": [{"q": "問い", "a": "答え"}]})),
    ];
    let mut blocks: Vec<Block> = parts.into_iter().map(Block::Part).collect();
    blocks.push(Block::Markdown("本文の段落".into()));
    let mut input = input(
        vec![
            document("a", vec![section(true, blocks)]),
            document("b", vec![]),
        ],
        marked_ui(),
    );
    input.toc.items = vec![TocItem::Group(TocGroup {
        title: "群".into(),
        note: Some("群の説明".into()),
        items: vec![TocItem::Document("a".into())],
    })];
    input.references = vec![
        Reference {
            key: "REQ-x-001".into(),
            label: "REQ-x-001".into(),
            body: Some("後回しの要求".into()),
            state: ReferenceState::Deferred,
        },
        Reference {
            key: "docs/x.md#A1".into(),
            label: "x A1".into(),
            body: Some("古い決定".into()),
            state: ReferenceState::Superseded,
        },
    ];
    input.others = vec![OtherLanguage {
        name: "Autre".into(),
        place: "fr/".into(),
    }];
    let mut allowed = Vec::new();
    for document in &input.documents {
        allowed.push(document.title.clone());
        let parts = std::iter::once(&document.lead)
            .chain(&document.preamble)
            .chain(document.sections.iter().flat_map(|section| {
                section.blocks.iter().filter_map(|block| match block {
                    Block::Part(part) => Some(part),
                    Block::Markdown(_) => None,
                })
            }));
        for part in parts {
            strings(&part.value, &mut allowed);
        }
        for section in &document.sections {
            allowed.push(section.heading.clone());
            for block in &section.blocks {
                if let Block::Markdown(text) = block {
                    allowed.push(text.clone());
                }
            }
        }
    }
    for reference in &input.references {
        allowed.push(reference.label.clone());
        allowed.extend(reference.body.clone());
    }
    allowed.extend([
        "目次".to_string(),
        "群".into(),
        "群の説明".into(),
        "Autre".into(),
    ]);
    let ui: Vec<String> = input.ui.values().cloned().collect();
    let from_ui = |node: &str| {
        ui.iter().any(|text| match text.split_once("{n}") {
            Some((before, after)) => node
                .strip_prefix(before)
                .and_then(|rest| rest.strip_suffix(after))
                .is_some_and(|number| {
                    !number.is_empty() && number.bytes().all(|b| b.is_ascii_digit())
                }),
            None => node == text,
        })
    };
    let pages = render(&input);
    for page in html_pages(&pages) {
        for node in text_nodes(&page.content) {
            assert!(
                allowed.iter().any(|text| text.contains(&node)) || from_ui(&node),
                "{}: {node:?} comes from neither the input nor the UI text",
                page.name
            );
        }
    }
    // UI の文字のうち描く所のある鍵は、どれかのページに描かれる
    let all: String = html_pages(&pages)
        .map(|page| page.content.as_str())
        .collect();
    for key in [
        "index_link",
        "stale_mark",
        "outline_stale",
        "superseded",
        "deferred",
        "compare_before",
        "compare_after",
        "compare_why",
        "state_decided",
        "state_planned",
        "state_open",
        "state_dropped",
        "pages",
        "stale_sections",
        "open_items",
        "planned_items",
    ] {
        assert!(all.contains(&format!("X{key}")), "{key}");
    }
}

// @kotowari[REQ-view-028, EX-view-020]
#[test]
fn ex_view_020_the_number_goes_where_the_text_puts_it() {
    let mut ui = marked_ui();
    ui.insert("pages".into(), "全{n}件".into());
    let mut input = input(
        vec![
            document("a", vec![]),
            document("b", vec![]),
            document("c", vec![]),
        ],
        ui,
    );
    input.toc.items = vec![TocItem::Group(TocGroup {
        title: "群".into(),
        note: None,
        items: ["a", "b", "c"]
            .into_iter()
            .map(|name| TocItem::Document(name.into()))
            .collect(),
    })];
    let pages = render(&input);
    let index = page(&pages, "index.html");
    let group = &index[index.find("群").unwrap()..];
    let group = &group[..group.find("</summary>").expect("the group heading")];
    assert!(group.contains("全3件"), "{group}");
}

/// ほかの言語が1件の入力と、空の入力で描いたページ
fn with_and_without_others() -> (Vec<Page>, Vec<Page>) {
    let mut input = input(vec![document("a", vec![])], marked_ui());
    let without = render(&input);
    input.others = vec![OtherLanguage {
        name: "English".into(),
        place: "en/".into(),
    }];
    (render(&input), without)
}

// @kotowari[REQ-view-029, EX-view-021]
#[test]
fn ex_view_021_every_page_links_the_same_page_in_the_other_language() {
    let (pages, _) = with_and_without_others();
    for (name, target) in [("a.html", "en/a.html"), ("index.html", "en/index.html")] {
        assert!(
            common::links(page(&pages, name)).contains(&(target.into(), "English".into())),
            "{name}"
        );
    }
    for page in &pages {
        assert!(!page.content.contains("<script"), "{}", page.name);
    }
}

// @kotowari[REQ-view-029, EX-view-022]
#[test]
fn ex_view_022_without_other_languages_there_is_no_language_link() {
    let (with, without) = with_and_without_others();
    for (name, target) in [("a.html", "en/a.html"), ("index.html", "en/index.html")] {
        let mut expected = common::links(page(&with, name));
        expected.retain(|link| *link != (target.to_string(), "English".to_string()));
        assert_eq!(common::links(page(&without, name)), expected, "{name}");
    }
}

// @kotowari[REQ-view-030, EX-view-023]
#[test]
fn ex_view_023_a_reference_without_a_body_is_its_label_and_opens_nothing() {
    let steps = part(
        "steps",
        json!({"items": [{"title": "段階", "refs": ["docs/x.md#A1"]}]}),
    );
    let mut input = input(
        vec![document(
            "a",
            vec![section(false, vec![Block::Part(steps)])],
        )],
        marked_ui(),
    );
    input.references = vec![Reference {
        key: "docs/x.md#A1".into(),
        label: "x A1".into(),
        body: None,
        state: ReferenceState::Current,
    }];
    let pages = render(&input);
    let a = page(&pages, "a.html");
    assert!(a.contains("x A1"), "{a}");
    assert!(!a.contains("<details"), "{a}");
}

// @kotowari[REQ-view-008, REQ-view-030]
#[test]
fn a_reference_missing_from_the_table_is_its_key_as_text_and_opens_nothing() {
    // 表に無い参照は開く本文を持たないので、本文の無い参照と同じく選んでも何も開かない
    let steps = part(
        "steps",
        json!({"items": [{"title": "段階", "refs": ["docs/x.md#A1"]}]}),
    );
    let input = input(
        vec![document(
            "a",
            vec![section(false, vec![Block::Part(steps)])],
        )],
        marked_ui(),
    );
    let pages = render(&input);
    let a = page(&pages, "a.html");
    assert!(
        a.contains("<span class=\"ref ref-plain\">docs/x.md#A1</span>"),
        "{a}"
    );
    assert!(!a.contains("<details"), "{a}");
}
