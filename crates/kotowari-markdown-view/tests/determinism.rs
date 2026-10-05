use kotowari_markdown_view::{
    Block, Document, Part, Reference, ReferenceState, RenderInput, Section, TocGroup, TocItem,
    render,
};
use proptest::prelude::*;
use serde_json::json;

#[path = "common/ui.rs"]
mod ui;

fn state() -> impl Strategy<Value = ReferenceState> {
    prop_oneof![
        Just(ReferenceState::Current),
        Just(ReferenceState::Superseded),
        Just(ReferenceState::Deferred),
    ]
}

fn reference() -> impl Strategy<Value = Reference> {
    (
        "[a-zA-Z0-9#./-]{1,12}",
        ".{0,12}",
        prop::option::of(".{0,24}"),
        state(),
    )
        .prop_map(|(key, label, body, state)| Reference {
            key,
            label,
            body,
            state,
        })
}

fn part() -> impl Strategy<Value = Part> {
    (
        0usize..9,
        ".{0,16}",
        prop::collection::vec("[a-zA-Z0-9#./-]{1,12}", 0..3),
        any::<bool>(),
    )
        .prop_map(|(kind, text, refs, half)| {
            let mut value = match kind {
                0 => json!({"conclusion": text, "points": [text]}),
                1 => json!({"columns": [[{"title": text, "tone": "good"}], [{"title": text}]]}),
                2 => json!({"items": [{"title": text, "refs": refs}]}),
                3 => json!({"cards": [{"title": text, "items": [text]}]}),
                4 => json!({"items": [{"state": "open", "text": text, "refs": refs}]}),
                5 => json!({"items": [{"before": text, "after": text, "why": text, "refs": refs}]}),
                6 => json!({"roots": [{"ref": refs.first().cloned().unwrap_or_default(), "text": text, "by": "LLM"}]}),
                7 => json!({"items": [{"q": text, "a": text, "refs": refs}]}),
                _ => json!({"unknown": text}),
            };
            if half {
                value["width"] = json!("half");
            }
            let kinds = [
                "lead",
                "flow",
                "steps",
                "cards",
                "status",
                "compare",
                "decisions",
                "quiz",
                "chart",
            ];
            Part {
                kind: kinds[kind].into(),
                value,
            }
        })
}

fn block() -> impl Strategy<Value = Block> {
    prop_oneof![
        ".{0,40}".prop_map(Block::Markdown),
        part().prop_map(Block::Part),
    ]
}

fn section() -> impl Strategy<Value = Section> {
    (
        ".{0,12}",
        any::<bool>(),
        prop::collection::vec(block(), 0..5),
    )
        .prop_map(|(heading, stale, blocks)| Section {
            heading,
            stale,
            blocks,
        })
}

fn document() -> impl Strategy<Value = Document> {
    (
        "[a-z0-9-]{1,8}",
        ".{0,12}",
        part(),
        prop::collection::vec(part(), 0..3),
        prop::collection::vec(section(), 0..4),
    )
        .prop_map(|(name, title, lead, preamble, sections)| Document {
            name,
            title,
            lead,
            preamble,
            sections,
        })
}

fn group(items: impl Strategy<Value = Vec<TocItem>>) -> impl Strategy<Value = TocGroup> {
    (".{0,12}", prop::option::of(".{0,12}"), items).prop_map(|(title, note, items)| TocGroup {
        title,
        note,
        items,
    })
}

/// 目次。名前は文書の名前と同じ形で作るので、文書のある名前も無い名前も重なった名前も出る
fn toc() -> impl Strategy<Value = TocGroup> {
    let name = "[a-z0-9-]{1,8}".prop_map(TocItem::Document);
    let item = name.prop_recursive(3, 12, 4, |inner| {
        group(prop::collection::vec(inner, 0..4)).prop_map(TocItem::Group)
    });
    group(prop::collection::vec(item, 0..5))
}

fn input() -> impl Strategy<Value = RenderInput> {
    (
        prop::collection::vec(document(), 0..4),
        prop::collection::vec(reference(), 0..5),
        toc(),
    )
        .prop_map(|(documents, references, toc)| RenderInput {
            documents,
            references,
            toc,
            ..ui::japanese()
        })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    // @kotowari[EX-view-002, REQ-view-004]
    #[test]
    fn ex_view_002_the_same_input_renders_to_the_same_bytes(input in input()) {
        let first = render(&input);
        let second = render(&input.clone());
        prop_assert_eq!(first.len(), second.len());
        for (left, right) in first.iter().zip(&second) {
            prop_assert_eq!(&left.name, &right.name);
            prop_assert_eq!(left.content.as_bytes(), right.content.as_bytes());
        }
    }
}
