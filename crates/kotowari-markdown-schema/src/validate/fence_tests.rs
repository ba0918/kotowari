//! 閉じないフェンスは、どのスキーマでも unclosed_codeblock にする。

use super::*;
use crate::document::Document;
use crate::schema::parse_schema;

fn unclosed(schema_yaml: &str, doc: &str, open: bool) -> Vec<(usize, Option<String>)> {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    validate(
        &schema,
        &document.read_as(schema.reading),
        open || schema.open,
    )
    .into_iter()
    .filter(|f| f.kind == FindingKind::UnclosedCodeblock)
    .map(|f| (f.line.unwrap(), f.raw))
    .collect()
}

const OPEN_SECTION: &str = "document:\n  sections:\n    - name: 節\n      open: true\n";

// @kotowari[C-schema-block-39]
#[test]
fn a_fence_opened_indented_in_a_list_item_and_never_closed_is_unclosed_in_both_readings() {
    let doc = "## 節\n\n- 行\n\n  ```rust\n  let x = 1;\n";
    for schema in [
        OPEN_SECTION.to_string(),
        "open: true\ndocument: {}\n".to_string(),
        format!("reading: line\n{OPEN_SECTION}"),
    ] {
        assert_eq!(
            unclosed(&schema, doc, false),
            vec![(5, Some("  ```rust".to_string()))],
            "{schema}"
        );
    }
}

// @kotowari[C-schema-block-39]
#[test]
fn a_fence_opened_on_a_list_marker_line_and_never_closed_is_unclosed_in_both_readings() {
    let doc = "## 節\n\n- ~~~\n  中身\n";
    for schema in [
        OPEN_SECTION.to_string(),
        format!("reading: line\n{OPEN_SECTION}"),
    ] {
        assert_eq!(
            unclosed(&schema, doc, false),
            vec![(3, Some("- ~~~".to_string()))],
            "{schema}"
        );
    }
}

// @kotowari[C-schema-block-38]
#[test]
fn an_unclosed_fence_is_found_in_an_undeclared_section_of_the_open_world_and_without_a_rule() {
    let schema = "document:\n  sections:\n    - name: 節\n";
    let doc = "## 節\n\n## 別\n\n````\nコード\n```\n";
    for open in [false, true] {
        assert_eq!(
            unclosed(schema, doc, open),
            vec![(5, Some("````".to_string()))],
            "open: {open}"
        );
    }
}

// @kotowari[C-schema-block-38]
#[test]
fn an_unclosed_fence_inside_a_block_quote_is_found() {
    let doc = "## 節\n\n> ```\n> 中身\n\n後ろ\n";
    assert_eq!(
        unclosed(OPEN_SECTION, doc, false),
        vec![(3, Some("> ```".to_string()))]
    );
}

// @kotowari[C-schema-block-40]
#[test]
fn closed_fences_get_no_unclosed_codeblock_in_both_readings() {
    let doc = "## 節\n\n```\na\n```\n\n- 行\n\n  ~~~~\n  b\n  ~~~~~\n\n> ```\n> c\n> ```\n";
    for schema in [
        OPEN_SECTION.to_string(),
        format!("reading: line\n{OPEN_SECTION}"),
    ] {
        assert_eq!(unclosed(&schema, doc, false), Vec::new(), "{schema}");
    }
}
