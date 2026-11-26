//! 名前付きキャプチャ（of の鍵の値の group）と、箇条書きの end。

use super::*;
use crate::document::Document;
use crate::schema::parse_schema;
use serde_json::json;

fn values(schema_yaml: &str, doc: &str) -> Value {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    extract_values(&schema, &document.read_as(schema.reading))
}

fn bullet_schema(pattern: &str, of: &str) -> String {
    format!(
        "document:\n  sections:\n    - name: 節\n      bullets:\n        repeat: {{ min: 0 }}\n        pattern: \"{pattern}\"\n        children: {{ open: true }}\n        extract: {{ path: rows, of: {of} }}\n"
    )
}

// @kotowari[C-schema-extract-48]
#[test]
fn group_values_in_of_hold_the_text_each_capture_of_the_bullet_pattern_captured() {
    let schema = bullet_schema(
        "^(?P<code>[A-Z]-[0-9]+) (?P<text>.*)$",
        "{ code: { group: code }, text: { group: text }, line: line }",
    );
    let v = values(&schema, "## 節\n\n- A-1 一つ目\n- B-22 二つ目の文\n");
    assert_eq!(
        v["rows"],
        json!([
            { "code": "A-1", "text": "一つ目", "line": 3 },
            { "code": "B-22", "text": "二つ目の文", "line": 4 }
        ])
    );
}

// @kotowari[C-schema-extract-49]
#[test]
fn a_capture_that_took_no_part_in_the_match_leaves_its_key_out() {
    let schema = bullet_schema(
        "^(?P<flag>!)?(?P<text>.*)$",
        "{ flag: { group: flag }, text: { group: text } }",
    );
    let v = values(&schema, "## 節\n\n- !急ぎ\n- 普通\n");
    assert_eq!(
        v["rows"],
        json!([{ "flag": "!", "text": "急ぎ" }, { "text": "普通" }])
    );
}

// @kotowari[C-schema-extract-50]
#[test]
fn a_capture_of_a_wrapped_bullet_takes_the_lead_paragraph_joined_with_one_space() {
    let schema = bullet_schema("^(?P<text>.*)$", "{ text: { group: text } }");
    let v = values(&schema, "## 節\n\n- 一行目の\n  二行目\n");
    assert_eq!(v["rows"], json!([{ "text": "一行目の 二行目" }]));
}

// @kotowari[C-schema-extract-46]
#[test]
fn end_of_a_bullet_is_the_last_line_of_its_list_item_with_continuations_and_children() {
    let schema = bullet_schema("", "{ line: line, end: end }");
    let doc =
        "## 節\n\n- 一つ目\n\n  続きの段落\n\n  - 子\n    - 孫\n- 二つ目\n\n次の段落ではない\n";
    let v = values(&schema, doc);
    assert_eq!(
        v["rows"],
        json!([{ "line": 3, "end": 8 }, { "line": 9, "end": 9 }])
    );
}

// @kotowari[C-schema-extract-46]
#[test]
fn end_of_a_bullet_when_reading_by_line_is_the_last_line_of_its_children() {
    let schema = format!(
        "reading: line\n{}",
        bullet_schema("", "{ line: line, end: end }")
    );
    let doc = "## 節\n\n- 一つ目\n  - 子\n  ```\n  code\n  ```\n- 二つ目\n";
    let v = values(&schema, doc);
    assert_eq!(
        v["rows"],
        json!([{ "line": 3, "end": 7 }, { "line": 8, "end": 8 }])
    );
}

// @kotowari[C-schema-extract-51]
#[test]
fn a_group_value_in_of_of_the_title_holds_the_capture_and_value_keeps_the_heading() {
    let schema = "document:\n  title:\n    pattern: \"^ADR-(?<id>\\\\d{4}):\"\n    extract: { path: title, value: text, of: { id: { group: id }, line: line } }\n";
    let v = values(schema, "# ADR-0042: 話\n");
    assert_eq!(
        v["title"],
        json!({ "text": "ADR-0042: 話", "id": "0042", "line": 1 })
    );
}

// @kotowari[C-schema-extract-52]
#[test]
fn schemas_that_misuse_group_stop() {
    let cases = [
        // path と並べた group
        "document:\n  title:\n    pattern: \"(?<id>x)\"\n    extract: { path: id, group: id }\n",
        // 題名と箇条書きのほかのノードの of の group
        "document:\n  preamble:\n    statement:\n      pattern: \"(?<id>x)\"\n      extract: { path: s, of: { id: { group: id } } }\n",
        "document:\n  sections:\n    - name: 節\n      extract: { path: s, of: { id: { group: id } } }\n",
        "document:\n  sections:\n    - name: 節\n      items:\n        - extract: { path: s, of: { id: { group: id } } }\n",
        // 正規表現の無いノード
        "document:\n  title:\n    extract: { path: t, of: { id: { group: id } } }\n",
        "document:\n  preamble:\n    bullets:\n      extract: { path: b, of: { id: { group: id } } }\n",
        // その名前付きキャプチャの無い正規表現
        "document:\n  title:\n    pattern: \"(?<other>x)\"\n    extract: { path: t, of: { id: { group: id } } }\n",
        "document:\n  preamble:\n    bullets:\n      pattern: \"(?<other>x)\"\n      extract: { path: b, of: { id: { group: id } } }\n",
        // group のほかの鍵を持つ写像
        "document:\n  title:\n    pattern: \"(?<id>x)\"\n    extract: { path: t, of: { id: { group: id, other: 1 } } }\n",
    ];
    for schema in cases {
        assert!(parse_schema(schema).is_err(), "{schema}");
    }
}

// @kotowari[C-schema-extract-47]
#[test]
fn schemas_that_declare_a_word_on_a_node_that_does_not_accept_it_stop() {
    let cases = [
        // id は項目だけ
        (
            "document:\n  sections:\n    - name: 節\n      extract: { path: s, of: { id: id } }\n",
            "\"id\"",
        ),
        // name は項目と節だけ
        (
            "document:\n  preamble:\n    bullets:\n      extract: { path: b, of: { name: name } }\n",
            "\"name\"",
        ),
        // end は項目、節、箇条書きだけ
        (
            "document:\n  preamble:\n    table:\n      extract: { path: t, of: { end: end } }\n",
            "\"end\"",
        ),
    ];
    for (schema, word) in cases {
        let error = parse_schema(schema).unwrap_err();
        assert!(error.0.contains(word), "{schema}: {error}");
    }
    assert!(
        parse_schema(
            "document:\n  preamble:\n    bullets:\n      extract: { path: b, of: { end: end } }\n"
        )
        .is_ok()
    );
}
