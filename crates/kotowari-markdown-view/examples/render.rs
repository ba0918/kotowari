use kotowari_markdown_view::{
    Block, Document, PART_KINDS, Part, Reference, ReferenceState, RenderInput, Section,
    part_schema, render,
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
            body: "The body opened in place".into(),
            state: ReferenceState::Current,
        }],
    };
    let pages = render(&input);
    let names: Vec<&str> = pages.iter().map(|page| page.name.as_str()).collect();
    assert_eq!(names, ["example.html", "index.html", "style.css"]);
    println!("{}", names.join("\n"));
}
