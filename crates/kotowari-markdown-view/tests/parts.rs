use kotowari_markdown_view::{
    Block, Document, PART_KINDS, Part, Reference, ReferenceState, RenderInput, Section, TocGroup,
    part_schema, render,
};
use serde_json::{Value, json};

#[path = "common/ui.rs"]
mod ui;

/// TBL-view-001 の種類の名前
const KINDS: [&str; 8] = [
    "lead",
    "flow",
    "steps",
    "cards",
    "status",
    "compare",
    "decisions",
    "quiz",
];

/// 種類ごとの、スキーマに合う部品の例（REQ-view-014）
fn examples() -> Vec<(&'static str, Value)> {
    vec![
        (
            "lead",
            json!({"conclusion": "結論", "points": ["要点1", "要点2"]}),
        ),
        ("lead", json!({"conclusion": "要点の無い結論"})),
        (
            "flow",
            json!({"columns": [
                [{"title": "入力", "body": "元データ", "tone": "accent"}],
                [{"title": "検査"}, {"title": "描画", "body": "view", "tone": "good"}]
            ]}),
        ),
        (
            "steps",
            json!({"items": [
                {"title": "読む", "body": "元データを読む", "refs": ["REQ-x-001"]},
                {"title": "書く"}
            ], "width": "half"}),
        ),
        (
            "cards",
            json!({"cards": [
                {"title": "build", "items": ["書く", "消す"], "tone": "warn"},
                {"title": "serve", "items": []}
            ]}),
        ),
        (
            "status",
            json!({"items": [
                {"state": "decided", "text": "build を作る", "refs": ["docs/x.md#A1"]},
                {"state": "planned", "text": "serve"},
                {"state": "open", "text": "hot reload"},
                {"state": "dropped", "text": "render"}
            ]}),
        ),
        (
            "compare",
            json!({"items": [
                {"before": "毎回 HTML を書く", "after": "元データから描く", "why": "見た目を揃える", "refs": []}
            ]}),
        ),
        (
            "decisions",
            json!({"roots": [
                {"ref": "docs/x.md#A1", "text": "根の判断", "by": "利用者", "children": [
                    {"ref": "docs/x.md#A2", "text": "下の判断", "by": "LLM", "children": [
                        {"ref": "docs/x.md#A3", "text": "さらに下", "by": "LLM"}
                    ]}
                ]}
            ]}),
        ),
        (
            "quiz",
            json!({"items": [{"q": "どこに書く?", "a": "cache の下", "refs": ["REQ-x-001"]}]}),
        ),
    ]
}

fn validator(kind: &str) -> jsonschema::Validator {
    let schema: Value = serde_json::from_str(part_schema(kind).expect(kind)).expect(kind);
    jsonschema::validator_for(&schema).expect(kind)
}

fn lead() -> Part {
    Part {
        kind: "lead".into(),
        value: json!({"conclusion": "結論"}),
    }
}

fn page_with(blocks: Vec<Block>, references: Vec<Reference>) -> String {
    let input = RenderInput {
        documents: vec![Document {
            name: "a".into(),
            title: "題名".into(),
            lead: lead(),
            preamble: vec![],
            sections: vec![Section {
                heading: "節".into(),
                stale: false,
                blocks,
            }],
        }],
        references,
        toc: TocGroup::default(),
        ..ui::japanese()
    };
    render(&input)
        .into_iter()
        .find(|page| page.name == "a.html")
        .unwrap()
        .content
}

fn part(kind: &str, value: Value) -> Block {
    Block::Part(Part {
        kind: kind.into(),
        value,
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

/// 値から "width" を除く
fn full(block: &Block) -> Block {
    match block {
        Block::Part(part) => {
            let mut value = part.value.clone();
            if let Some(map) = value.as_object_mut() {
                map.remove("width");
            }
            Block::Part(Part {
                kind: part.kind.clone(),
                value,
            })
        }
        other => other.clone(),
    }
}

fn draw(kind: &str, value: Value) -> String {
    let text = page_with(vec![part(kind, value)], vec![]);
    let start = text.find("<section").unwrap();
    text[start..].to_string()
}

// @kotowari[EX-view-007, REQ-view-012, REQ-view-011]
#[test]
fn ex_view_007_schemas_exist_for_the_eight_kinds_and_not_for_others() {
    assert_eq!(PART_KINDS, KINDS);
    for kind in KINDS {
        let schema = part_schema(kind).unwrap_or_else(|| panic!("{kind}"));
        let value: Value = serde_json::from_str(schema).unwrap();
        assert_eq!(value["type"], "object", "{kind}");
    }
    assert_eq!(part_schema("chart"), None);
    assert_eq!(part_schema(""), None);
}

/// 型が object の部分スキーマのうち "additionalProperties": false を宣言しないものの場所
fn open_objects(schema: &Value, at: &str, found: &mut Vec<String>) {
    match schema {
        Value::Object(map) => {
            let is_object = map.get("type") == Some(&json!("object"));
            if is_object && map.get("additionalProperties") != Some(&json!(false)) {
                found.push(if at.is_empty() {
                    "(root)".into()
                } else {
                    at.into()
                });
            }
            for (key, child) in map {
                open_objects(child, &format!("{at}/{key}"), found);
            }
        }
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                open_objects(child, &format!("{at}/{index}"), found);
            }
        }
        _ => {}
    }
}

// @kotowari[REQ-view-013]
#[test]
fn req_view_013_every_object_in_every_part_schema_is_closed() {
    for kind in KINDS {
        let schema: Value = serde_json::from_str(part_schema(kind).unwrap()).unwrap();
        let mut found = vec![];
        open_objects(&schema, "", &mut found);
        assert!(found.is_empty(), "{kind}: {found:?}");
    }
}

// @kotowari[EX-view-009, REQ-view-013]
#[test]
fn ex_view_009_an_object_without_additional_properties_is_caught() {
    let schema = json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {"items": {"type": "array", "items": {"$ref": "#/$defs/item"}}},
        "$defs": {"item": {"type": "object", "properties": {"x": {"type": "string"}}}}
    });
    let mut found = vec![];
    open_objects(&schema, "", &mut found);
    assert_eq!(found, ["/$defs/item"]);
}

// @kotowari[REQ-view-014, EX-view-009]
#[test]
fn req_view_014_every_kind_has_examples_that_fit_their_schema_and_render() {
    let examples = examples();
    for kind in KINDS {
        assert!(
            examples.iter().any(|(example, _)| *example == kind),
            "no example for {kind}"
        );
    }
    for (kind, value) in examples {
        let validator = validator(kind);
        let errors: Vec<String> = validator
            .iter_errors(&value)
            .map(|error| format!("{} {:?}", error.instance_path(), error.kind()))
            .collect();
        assert!(errors.is_empty(), "{kind}: {errors:?}");
        let drawn = page_with(vec![part(kind, value)], vec![]);
        assert_ne!(drawn, page_with(vec![], vec![]), "{kind}");
    }
}

// @kotowari[REQ-view-015]
#[test]
fn req_view_015_every_schema_accepts_only_half_as_the_width() {
    for (kind, value) in examples() {
        let validator = validator(kind);
        let mut half = value.clone();
        half["width"] = json!("half");
        assert!(validator.is_valid(&half), "{kind}");
        for other in [json!("full"), json!(""), json!(2)] {
            let mut wide = value.clone();
            wide["width"] = other;
            assert!(!validator.is_valid(&wide), "{kind}");
        }
    }
}

// @kotowari[EX-view-008, REQ-view-015]
#[test]
fn ex_view_008_two_following_half_parts_sit_side_by_side() {
    let cards = |title: &str| {
        part(
            "cards",
            json!({"cards": [{"title": title, "items": []}], "width": "half"}),
        )
    };
    let blocks = vec![
        cards("左のカード"),
        cards("右のカード"),
        part(
            "status",
            json!({"items": [{"state": "decided", "text": "全幅の状態"}]}),
        ),
    ];
    let text = page_with(blocks.clone(), vec![]);
    let wide = page_with(blocks.iter().map(full).collect(), vec![]);
    // 幅いっぱいに描いた場合と比べ、2つの cards を囲む所だけが変わり、status は変わらない
    // 組にすると、幅いっぱいの描き方に横並びの囲みが加わる
    assert!(text.len() > wide.len());
    let (start, end) = changed(&text, &wide).expect("a row");
    let at = |needle: &str| text.find(needle).unwrap();
    assert!(start < at("左のカード") && at("右のカード") < end);
    assert!(end <= at("全幅の状態"));
}

// @kotowari[REQ-view-015]
#[test]
fn req_view_015_a_half_part_left_over_and_others_are_drawn_full_width() {
    let half = |title: &str| {
        part(
            "steps",
            json!({"items": [{"title": title}], "width": "half"}),
        )
    };
    let blocks = vec![
        half("一"),
        half("二"),
        half("三"),
        Block::Markdown("間の文".into()),
        half("四"),
    ];
    let text = page_with(blocks.clone(), vec![]);
    let wide = page_with(blocks.iter().map(full).collect(), vec![]);
    // 幅いっぱいに描いた場合と比べて変わるのは一と二を組にする所だけで、三から後は同じ
    assert!(text.len() > wide.len());
    let (start, end) = changed(&text, &wide).expect("a row");
    let at = |needle: &str| text.find(needle).unwrap();
    assert!(start < at("一") && at("二") < end);
    assert!(end <= at("三"));
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_lead_draws_the_conclusion_and_the_points() {
    let drawn = draw(
        "lead",
        json!({"conclusion": "結論の文", "points": ["点A", "点B"]}),
    );
    assert!(drawn.contains("結論の文"));
    assert!(drawn.find("点A").unwrap() < drawn.find("点B").unwrap());
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_flow_draws_columns_of_boxes_on_a_grid_from_left_to_right() {
    let drawn = draw(
        "flow",
        json!({"columns": [[{"title": "左", "tone": "accent"}], [{"title": "右", "body": "説明"}]]}),
    );
    assert!(drawn.find("左").unwrap() < drawn.find("右").unwrap());
    assert!(drawn.contains("説明"));
    let plain = draw(
        "flow",
        json!({"columns": [[{"title": "左"}], [{"title": "右", "body": "説明"}]]}),
    );
    assert_ne!(drawn, plain, "the tone changes how the box is drawn");
    assert!(
        !drawn.contains("style="),
        "positions come from the grid, not from the data"
    );
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_steps_are_a_numbered_sequence() {
    let drawn = draw(
        "steps",
        json!({"items": [{"title": "最初", "body": "本文"}, {"title": "次"}]}),
    );
    assert!(drawn.contains("<ol"));
    assert!(drawn.find("最初").unwrap() < drawn.find("次").unwrap());
    assert!(drawn.contains("本文"));
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_cards_draw_a_title_and_a_list_of_items() {
    let drawn = draw(
        "cards",
        json!({"cards": [{"title": "カード", "items": ["項目1", "項目2"], "tone": "bad"}]}),
    );
    assert!(drawn.contains("カード"));
    assert!(drawn.contains("<li>項目1</li>") && drawn.contains("<li>項目2</li>"));
    let plain = draw(
        "cards",
        json!({"cards": [{"title": "カード", "items": ["項目1", "項目2"]}]}),
    );
    assert_ne!(drawn, plain, "the tone changes how the card is drawn");
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_status_draws_a_label_for_each_of_the_four_states() {
    let drawn = draw(
        "status",
        json!({"items": [
            {"state": "decided", "text": "t1"}, {"state": "planned", "text": "t2"},
            {"state": "open", "text": "t3"}, {"state": "dropped", "text": "t4"}
        ]}),
    );
    let mut last = 0;
    for (state, text) in [
        ("決定", "t1"),
        ("予定", "t2"),
        ("未決", "t3"),
        ("取り下げ", "t4"),
    ] {
        let label = drawn[last..].find(state).map(|at| last + at);
        let label = label.unwrap_or_else(|| panic!("{state}"));
        let body = drawn[label..].find(text).map(|at| label + at);
        last = body.unwrap_or_else(|| panic!("{text}"));
    }
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_compare_strikes_through_the_before() {
    let drawn = draw(
        "compare",
        json!({"items": [{"before": "前の形", "after": "後の形", "why": "理由"}]}),
    );
    assert!(drawn.contains("<del>前の形</del>"));
    assert!(drawn.contains("後の形") && drawn.contains("理由"));
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_decisions_draw_a_tree_with_who_decided() {
    let drawn = draw(
        "decisions",
        json!({"roots": [{"ref": "r1", "text": "根", "by": "利用者", "children": [
            {"ref": "r2", "text": "枝", "by": "LLM"}
        ]}]}),
    );
    let root = drawn.find("根").unwrap();
    let nested = drawn.find("<ul").unwrap();
    assert!(root < drawn.find("枝").unwrap());
    assert!(drawn[nested..].contains("<ul"));
    assert!(root < drawn.find("利用者").unwrap());
    assert!(drawn.find("枝").unwrap() < drawn.find("LLM").unwrap());
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_quiz_hides_the_answer_until_chosen() {
    let drawn = draw("quiz", json!({"items": [{"q": "問い", "a": "答え"}]}));
    let details = drawn.find("<details").unwrap();
    let summary = drawn.find("<summary>問い</summary>").unwrap();
    assert!(details < summary && summary < drawn.find("答え").unwrap());
}

// @kotowari[REQ-view-003]
#[test]
fn req_view_003_a_value_that_does_not_fit_is_drawn_without_reporting() {
    let text = page_with(
        vec![
            part("steps", json!({"items": "not a list", "unknown": 1})),
            part("chart", json!({"x": 1})),
            part("quiz", json!(null)),
        ],
        vec![Reference {
            key: "k".into(),
            label: "k".into(),
            body: Some(String::new()),
            state: ReferenceState::Current,
        }],
    );
    assert!(text.contains("</html>"));
}

// @kotowari[EX-view-006, REQ-view-010, REQ-view-003]
#[test]
fn ex_view_006_pages_with_every_kind_load_nothing_from_outside() {
    let blocks = examples()
        .into_iter()
        .map(|(kind, value)| part(kind, value))
        .collect();
    let references = vec![Reference {
        key: "REQ-x-001".into(),
        label: "REQ-x-001".into(),
        body: Some("本文".into()),
        state: ReferenceState::Current,
    }];
    let input = RenderInput {
        documents: vec![Document {
            name: "a".into(),
            title: "題名".into(),
            lead: lead(),
            preamble: vec![],
            sections: vec![Section {
                heading: "節".into(),
                stale: true,
                blocks,
            }],
        }],
        references,
        toc: TocGroup::default(),
        ..ui::japanese()
    };
    for page in render(&input) {
        let text = page.content.to_lowercase();
        for scheme in ["http://", "https://"] {
            for attribute in ["src=\"", "href=\""] {
                assert!(
                    !text.contains(&format!("{attribute}{scheme}")),
                    "{}: {attribute}{scheme}",
                    page.name
                );
            }
        }
        assert!(!text.contains("@import"), "{}", page.name);
        assert!(!text.contains("url("), "{}", page.name);
    }
}

// @kotowari[REQ-view-010]
#[test]
fn req_view_010_markdown_text_cannot_make_the_page_load_images_scripts_or_styles() {
    let text = page_with(
        vec![Block::Markdown(
            "![図](https://example.com/a.png)\n\n<img src=\"https://example.com/b.png\">\n\n<link rel=\"stylesheet\" href=\"https://example.com/c.css\">\n\n<style>@import url(https://example.com/d.css);</style>\n"
                .into(),
        )],
        vec![],
    );
    let start = text.find("<main").unwrap();
    let main = &text[start..];
    for tag in ["<img", "<script", "<link", "<style", "<iframe"] {
        assert!(!main.contains(tag), "{tag}");
    }
    assert!(!main.contains("src=\""));
}

// @kotowari[REQ-view-003, REQ-view-011, REQ-view-007]
#[test]
fn req_view_003_the_characters_of_field_values_are_drawn_as_text_not_markup() {
    let text = "前置き q<q q>q z&z q\"q q'q";
    let drawn = page_with(
        vec![
            part("steps", json!({"items": [{"title": text}]})),
            // スキーマに合わない値も検査せずに描く。属性の中に置かれる値も文字のまま描く
            part(
                "cards",
                json!({"cards": [{"title": "c", "items": [], "tone": "x\" onclick=\"y"}]}),
            ),
        ],
        vec![],
    );
    assert!(drawn.contains("前置き"));
    for raw in ["q<q", "q>q", "z&z", "q\"q", "q'q", "x\" onclick"] {
        assert!(!drawn.contains(raw), "{raw} is written as markup");
    }
}

// @kotowari[REQ-view-011, TBL-view-001]
#[test]
fn tbl_view_001_the_four_status_badges_are_drawn_distinguishably() {
    // 札と、UI の文字の札の表示
    let states = [
        ("decided", "決定"),
        ("planned", "予定"),
        ("open", "未決"),
        ("dropped", "取り下げ"),
    ];
    // 札の文字を除いても残る違いが、見た目の違いである
    let drawn: Vec<String> = states
        .iter()
        .map(|(state, label)| {
            draw("status", json!({"items": [{"state": state, "text": "文"}]})).replace(label, "")
        })
        .collect();
    let first = drawn[0].as_bytes();
    let prefix = (0..first.len())
        .take_while(|&at| {
            drawn
                .iter()
                .all(|other| other.as_bytes().get(at) == first.get(at))
        })
        .count();
    let suffix = (0..first.len() - prefix)
        .take_while(|&back| {
            drawn.iter().all(|other| {
                let other = other.as_bytes();
                other.len() > prefix + back
                    && other[other.len() - 1 - back] == first[first.len() - 1 - back]
            })
        })
        .count();
    let differences: Vec<&str> = drawn
        .iter()
        .map(|text| &text[prefix..text.len() - suffix])
        .collect();
    let style = render(&RenderInput::default())
        .into_iter()
        .find(|page| page.name == "style.css")
        .unwrap()
        .content;
    for (index, difference) in differences.iter().enumerate() {
        assert!(!difference.is_empty(), "{}", states[index].0);
        // 違いはページが使う共通のスタイルが見た目を与えるものである
        assert!(
            style.contains(difference),
            "{}: {difference}",
            states[index].0
        );
        for other in &differences[index + 1..] {
            assert_ne!(difference, other);
        }
    }
}
