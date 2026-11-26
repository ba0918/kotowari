//! ノードが自分の中身として宣言する "open: true" の検査。

use super::*;
use crate::document::Document;
use crate::schema::parse_schema;

fn validate_src(schema_yaml: &str, doc: &str) -> Vec<Finding> {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    validate(&schema, &document.read_as(schema.reading), false)
}

fn lines_of(findings: &[Finding], kind: FindingKind) -> Vec<usize> {
    findings
        .iter()
        .filter(|f| f.kind == kind)
        .filter_map(|f| f.line)
        .collect()
}

/// 規則の無い段落、順序付きリスト、表、コードブロックを持つ項目。
const FREE_ITEM_DOC: &str = "## 節\n\n### A-1: 名前\n\n段落。\n\n1. 順序\n\n| a | b |\n|---|---|\n| 1 | 2 |\n\n```\ncode\n```\n";

fn item_schema(open: &str) -> String {
    format!("document:\n  sections:\n    - name: 節\n      item:\n        id: \"^A-\"\n{open}")
}

// @kotowari[C-schema-lang-20]
#[test]
fn an_item_declaring_open_accepts_blocks_that_have_no_rule() {
    let findings = validate_src(&item_schema("        open: true\n"), FREE_ITEM_DOC);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-lang-20]
#[test]
fn the_same_item_without_open_makes_each_block_without_a_rule_an_undeclared_line() {
    let findings = validate_src(&item_schema(""), FREE_ITEM_DOC);
    assert_eq!(
        lines_of(&findings, FindingKind::UndeclaredLine),
        vec![5, 7, 9, 13],
        "{findings:?}"
    );
}

// @kotowari[C-schema-lang-21]
#[test]
fn a_bullet_rule_on_an_open_item_keeps_its_pattern() {
    let schema = item_schema(
        "        open: true\n        bullets:\n          repeat: { min: 0 }\n          pattern: \"^ok\"\n",
    );
    let doc = "## 節\n\n### A-1: 名前\n\n- ok 1\n- ng 2\n\n段落。\n";
    let findings = validate_src(&schema, doc);
    assert_eq!(
        lines_of(&findings, FindingKind::BulletPatternMismatch),
        vec![6],
        "{findings:?}"
    );
    assert_eq!(findings.len(), 1, "{findings:?}");
}

// @kotowari[C-schema-lang-22]
#[test]
fn children_declaring_open_accept_a_child_list_a_table_and_a_code_block() {
    let schema = "document:\n  sections:\n    - name: 節\n      bullets:\n        repeat: { min: 0 }\n        children: { open: true }\n";
    let doc = "## 節\n\n- 行\n\n  - 子\n\n  | a | b |\n  |---|---|\n  | 1 | 2 |\n\n  ```\n  code\n  ```\n";
    let findings = validate_src(schema, doc);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-lang-22]
#[test]
fn children_without_open_make_the_same_child_blocks_undeclared_lines() {
    let schema = "document:\n  sections:\n    - name: 節\n      bullets:\n        repeat: { min: 0 }\n        children: {}\n";
    let doc = "## 節\n\n- 行\n\n  - 子\n\n  | a | b |\n  |---|---|\n  | 1 | 2 |\n\n  ```\n  code\n  ```\n";
    let findings = validate_src(schema, doc);
    assert_eq!(
        lines_of(&findings, FindingKind::UndeclaredLine),
        vec![5, 7, 11],
        "{findings:?}"
    );
}

// @kotowari[C-schema-lang-23]
#[test]
fn a_deep_heading_in_an_open_item_is_still_a_heading_level_mismatch() {
    let doc = "## 節\n\n### A-1: 名前\n\n#### 小見出し\n\n段落。\n";
    let findings = validate_src(&item_schema("        open: true\n"), doc);
    assert_eq!(
        lines_of(&findings, FindingKind::HeadingLevelMismatch),
        vec![5],
        "{findings:?}"
    );
}

// @kotowari[C-schema-lang-20]
#[test]
fn a_section_and_a_preamble_declaring_open_accept_blocks_that_have_no_rule() {
    let schema =
        "document:\n  preamble:\n    open: true\n  sections:\n    - name: 節\n      open: true\n";
    let doc = "前置きの段落。\n\n- 箇条書き\n\n## 節\n\n段落。\n\n1. 順序\n";
    let findings = validate_src(schema, doc);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-lang-21]
#[test]
fn open_on_a_node_does_not_lift_the_lower_bound_of_its_rules() {
    let schema = "document:\n  sections:\n    - name: 節\n      open: true\n      statement: { repeat: { min: 1 } }\n";
    let doc = "## 節\n\n- 箇条書きだけ\n";
    let findings = validate_src(schema, doc);
    assert_eq!(
        lines_of(&findings, FindingKind::RepeatMinNotMet),
        vec![1],
        "{findings:?}"
    );
}
