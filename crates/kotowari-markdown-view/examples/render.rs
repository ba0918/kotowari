use kotowari_markdown_view::{
    Block, Document, PART_KINDS, Part, Reference, ReferenceState, RenderInput, Section, TocGroup,
    TocItem, part_schema, render,
};
fn main() {
    assert!(PART_KINDS.iter().all(|kind| part_schema(kind).is_some()));
    let input = RenderInput {
        documents: vec![Document {
            name: "example".into(),
            title: "Example".into(),
            lead: Part {
                kind: "lead".into(),
                value: serde_json::json!({"conclusion": "Pages are rendered in memory"}),
            },
            preamble: vec![],
            sections: vec![Section {
                heading: "Flow".into(),
                stale: false,
                blocks: vec![
                    Block::Markdown("Text before the part.".into()),
                    Block::Part(Part {
                        kind: "steps".into(),
                        value: serde_json::json!({"items": [{"title": "Render", "refs": ["R1"]}]}),
                    }),
                ],
            }],
        }],
        references: vec![Reference {
            key: "R1".into(),
            label: "R1".into(),
            body: Some("The body opened in place".into()),
            state: ReferenceState::Current,
        }],
        toc: TocGroup {
            title: "Example".into(),
            note: Some("One group holding the one document".into()),
            items: vec![TocItem::Document("example".into())],
        },
        language: "en".into(),
        // view は文字を中に持たないので、描く文字はすべて呼び出し側が渡す
        ui: [
            ("pages", "{n} pages"),
            ("stale_sections", "{n} sections to review"),
            ("open_items", "{n} open"),
            ("planned_items", "{n} planned"),
            ("index_link", "Overview"),
        ]
        .into_iter()
        .map(|(key, text)| (key.to_string(), text.to_string()))
        .collect(),
        others: vec![],
    };
    let pages = render(&input);
    let names: Vec<&str> = pages.iter().map(|page| page.name.as_str()).collect();
    assert_eq!(names, ["example.html", "index.html", "style.css"]);
    println!("{}", names.join("\n"));
}
