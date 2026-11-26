//! 段落で読むときの引用、HTML のブロック、水平線と、指摘にしないブロック。

use super::*;
use crate::document::Document;
use crate::schema::parse_schema;

fn validate_src(schema_yaml: &str, doc: &str) -> Vec<Finding> {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    validate(&schema, &document.read_as(schema.reading), schema.open)
}

fn undeclared(findings: &[Finding]) -> Vec<(usize, Option<RuleKind>)> {
    findings
        .iter()
        .filter(|f| f.kind == FindingKind::UndeclaredLine)
        .map(|f| (f.line.unwrap(), f.rule_kind))
        .collect()
}

const SECTION: &str =
    "document:\n  sections:\n    - name: 節\n      statement: { repeat: { min: 0 } }\n";
const QUOTE_HTML_BREAK: &str = "## 節\n\n> 引用\n> 続き\n\n<div>\nHTML\n</div>\n\n---\n\n本文。\n";

// @kotowari[C-schema-block-34, C-schema-closed-6]
#[test]
fn quotes_html_blocks_and_thematic_breaks_in_a_section_without_open_are_undeclared_lines() {
    let findings = validate_src(SECTION, QUOTE_HTML_BREAK);
    assert_eq!(
        undeclared(&findings),
        vec![
            (3, Some(RuleKind::BlockQuote)),
            (6, Some(RuleKind::Html)),
            (10, Some(RuleKind::ThematicBreak)),
        ],
        "{findings:?}"
    );
}

// @kotowari[C-schema-block-35]
#[test]
fn image_only_paragraphs_and_link_reference_definitions_get_no_finding() {
    let doc = "## 節\n\n![alt](img.png)\n\n[ref]: https://example.com\n\n本文。\n";
    let findings = validate_src(SECTION, doc);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-block-41]
#[test]
fn a_block_of_only_html_comments_gets_no_finding() {
    let doc = "## 節\n\n<!-- 書き手への注記 -->\n\n<!-- 1つ目 --> <!-- 2つ目 -->\n<!--\n複数行\n-->\n\n本文。\n";
    let findings = validate_src(SECTION, doc);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-block-41]
#[test]
fn an_html_block_with_content_besides_comments_is_an_undeclared_line() {
    let doc = "## 節\n\n<!-- 注記 --><b>x</b>\n\n本文。\n";
    let findings = validate_src(SECTION, doc);
    assert_eq!(
        undeclared(&findings),
        vec![(3, Some(RuleKind::Html))],
        "{findings:?}"
    );
}

// @kotowari[C-schema-block-36]
#[test]
fn quotes_html_blocks_and_thematic_breaks_in_a_section_declaring_open_get_no_finding() {
    let schema = "document:\n  sections:\n    - name: 節\n      open: true\n";
    let findings = validate_src(schema, QUOTE_HTML_BREAK);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-block-37]
#[test]
fn top_level_open_does_not_let_a_quote_through_in_a_declared_section() {
    let schema = format!("open: true\n{SECTION}");
    let findings = validate_src(&schema, "## 節\n\n> 引用\n");
    assert_eq!(
        undeclared(&findings),
        vec![(3, Some(RuleKind::BlockQuote))],
        "{findings:?}"
    );
}

// @kotowari[C-schema-block-34]
#[test]
fn a_quote_under_a_bullet_whose_children_do_not_declare_open_is_an_undeclared_line() {
    let schema = "document:\n  sections:\n    - name: 節\n      bullets:\n        repeat: { min: 0 }\n        children: {}\n";
    let findings = validate_src(schema, "## 節\n\n- 行\n\n  > 引用\n");
    assert_eq!(
        undeclared(&findings),
        vec![(5, Some(RuleKind::BlockQuote))],
        "{findings:?}"
    );
}

// @kotowari[C-schema-block-34, C-schema-closed-2]
#[test]
fn a_quote_inside_an_undeclared_section_is_allowed_in_the_open_world() {
    let schema = format!("open: true\n{SECTION}");
    let findings = validate_src(&schema, "## 節\n\n本文。\n\n## 別\n\n> 引用\n");
    assert!(findings.is_empty(), "{findings:?}");
}
