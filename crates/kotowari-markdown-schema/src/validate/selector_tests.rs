//! 節と項目の規則が、見出しを名前の一覧か正規表現で選ぶ。

use super::*;
use crate::document::Document;
use crate::schema::parse_schema;

fn validate_src(schema_yaml: &str, doc: &str) -> Vec<Finding> {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    validate(&schema, &document.read_as(schema.reading), false)
}

fn values_of(schema_yaml: &str, doc: &str) -> serde_json::Value {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    crate::extract::extract_values(&schema, &document.read_as(schema.reading))
}

fn summary(findings: &[Finding]) -> Vec<(FindingKind, Option<usize>, Option<&str>)> {
    findings
        .iter()
        .map(|f| (f.kind, f.line, f.node.as_deref()))
        .collect()
}

// @kotowari[C-schema-lang-14]
#[test]
fn a_section_rule_with_a_name_list_selects_a_heading_equal_to_any_of_its_names() {
    let schema = "document:\n  sections:\n    - name: [未決, Open]\n      repeat: { min: 0 }\n      statement: { repeat: { min: 0 } }\n";
    assert!(validate_src(schema, "## 未決\n\n本文。\n").is_empty());
    assert!(validate_src(schema, "## Open\n\n本文。\n").is_empty());
    let other = validate_src(schema, "## 別\n");
    assert_eq!(
        summary(&other),
        vec![(FindingKind::UndeclaredHeading, Some(1), None)]
    );
}

// @kotowari[C-schema-lang-14]
#[test]
fn a_section_rule_with_a_single_name_string_still_selects_that_name() {
    let schema = "document:\n  sections:\n    - name: 理由\n";
    assert!(validate_src(schema, "## 理由\n").is_empty());
}

// @kotowari[C-schema-lang-15]
#[test]
fn an_empty_pattern_rule_after_the_name_rules_checks_every_other_section() {
    let schema = "document:\n  sections:\n    - name: 未決\n      required: false\n    - pattern: \"\"\n      repeat: { min: 0 }\n      statement: { pattern: \"^状態\" }\n";
    let doc = "## 一つ目\n\n状態: 提案\n\n## 二つ目\n\n別の文\n\n## 未決\n";
    let findings = validate_src(schema, doc);
    assert_eq!(
        summary(&findings),
        vec![(FindingKind::StatementPatternMismatch, Some(7), None)]
    );
}

// @kotowari[C-schema-lang-15]
#[test]
fn a_pattern_rule_selects_headings_containing_a_match() {
    let schema = "document:\n  sections:\n    - pattern: \"[0-9]+\"\n      repeat: { min: 0 }\n";
    assert!(validate_src(schema, "## 第2章\n").is_empty());
    assert_eq!(
        summary(&validate_src(schema, "## 序\n")),
        vec![(FindingKind::UndeclaredHeading, Some(1), None)]
    );
}

// @kotowari[C-schema-lang-16]
#[test]
fn the_rule_declared_first_takes_a_heading_two_rules_select() {
    let schema =
        "document:\n  sections:\n    - pattern: \"\"\n      repeat: { min: 0 }\n    - name: 未決\n";
    let findings = validate_src(schema, "## 未決\n");
    assert_eq!(
        summary(&findings),
        vec![(FindingKind::MissingRequiredSection, None, Some("未決"))]
    );
}

// @kotowari[C-schema-cli-30]
#[test]
fn findings_of_a_named_section_rule_hold_the_first_name_of_its_list() {
    let schema = "document:\n  sections:\n    - name: [未決, Open]\n";
    let findings = validate_src(schema, "");
    assert_eq!(
        summary(&findings),
        vec![(FindingKind::MissingRequiredSection, None, Some("未決"))]
    );
}

// @kotowari[C-schema-cli-31]
#[test]
fn findings_of_a_pattern_section_rule_hold_no_node_name_and_name_the_pattern() {
    let schema = "document:\n  sections:\n    - pattern: \"^章\"\n      repeat: { max: 1 }\n";
    let findings = validate_src(schema, "## 章1\n\n## 章2\n");
    assert_eq!(
        summary(&findings),
        vec![(FindingKind::RepeatMaxExceeded, Some(3), None)]
    );
    assert!(findings[0].detail.contains("^章"), "{}", findings[0].detail);
}

// @kotowari[C-schema-extract-45]
#[test]
fn name_on_a_section_extraction_holds_the_heading_text_of_each_section() {
    let schema = "document:\n  sections:\n    - pattern: \"\"\n      repeat: { min: 0 }\n      statement: { repeat: { min: 0 } }\n      extract: { path: decisions, of: { name: name, line: line } }\n";
    let values = values_of(schema, "## 一つ目 `x`\n\n本文。\n\n## 二つ目\n");
    assert_eq!(
        values["decisions"],
        serde_json::json!([{ "name": "一つ目 x", "line": 1 }, { "name": "二つ目", "line": 5 }])
    );
}

// @kotowari[C-schema-lang-15]
#[test]
fn extraction_of_a_rule_takes_only_the_sections_that_rule_selected() {
    let schema = "document:\n  sections:\n    - name: [未決, Open]\n      required: false\n      extract: { path: open, of: { line: line } }\n    - pattern: \"\"\n      repeat: { min: 0 }\n      extract: { path: decisions, of: { line: line } }\n";
    let values = values_of(schema, "## 決定\n\n## Open\n\n## 次\n");
    assert_eq!(values["open"], serde_json::json!({ "line": 3 }));
    assert_eq!(
        values["decisions"],
        serde_json::json!([{ "line": 1 }, { "line": 5 }])
    );
}

// @kotowari[C-schema-lang-4]
#[test]
fn a_section_rule_with_both_or_neither_of_name_and_pattern_is_a_schema_error() {
    for sections in [
        "- name: a\n      pattern: b\n",
        "- required: false\n",
        "- name: []\n",
    ] {
        let schema = format!("document:\n  sections:\n    {sections}");
        assert!(parse_schema(&schema).is_err(), "{schema}");
    }
}

// @kotowari[C-schema-cli-21]
#[test]
fn the_same_name_twice_across_the_section_rules_is_a_schema_error() {
    for sections in ["- name: [a, b]\n    - name: b\n", "- name: [a, a]\n"] {
        let schema = format!("document:\n  sections:\n    {sections}");
        let error = parse_schema(&schema).unwrap_err();
        assert!(error.0.contains("declared twice"), "{error}");
    }
}

const DECISION_ITEMS: &str = "document:\n  sections:\n    - pattern: \"\"\n      repeat: { min: 0 }\n      items:\n        - name: [問題, Problem]\n          statement: { repeat: { min: 0 } }\n        - name: [見送った案, Rejected options]\n          required: false\n          open: true\n";

// @kotowari[C-schema-lang-17]
#[test]
fn without_an_id_rule_an_item_heading_no_rule_selects_is_undeclared_with_its_lines() {
    let doc = "## 決定\n\n### 問題\n\n本文。\n\n### 余計: な見出し\n\n段落。\n\n- 箇条書き\n\n### 別の余計\n";
    let findings = validate_src(DECISION_ITEMS, doc);
    assert_eq!(
        summary(&findings),
        vec![
            (FindingKind::UndeclaredHeading, Some(7), None),
            (FindingKind::UndeclaredLine, Some(9), None),
            (FindingKind::UndeclaredLine, Some(11), None),
            (FindingKind::UndeclaredHeading, Some(13), None),
        ]
    );
}

// @kotowari[C-schema-lang-24]
#[test]
fn a_block_without_a_rule_in_an_item_without_open_is_undeclared_even_beside_an_open_item() {
    let doc = "## 決定\n\n### 問題\n\n1. 順序\n\n### 見送った案\n\n1. 順序\n";
    let findings = validate_src(DECISION_ITEMS, doc);
    assert_eq!(
        summary(&findings),
        vec![(FindingKind::UndeclaredLine, Some(5), None)]
    );
}

// @kotowari[C-schema-cli-30]
#[test]
fn a_missing_item_of_a_name_list_rule_holds_the_first_name_as_the_node_name() {
    let findings = validate_src(DECISION_ITEMS, "## 決定\n\n### Rejected options\n");
    assert_eq!(
        summary(&findings),
        vec![(FindingKind::RepeatMinNotMet, Some(1), Some("問題"))]
    );
}

// @kotowari[C-schema-cli-31]
#[test]
fn findings_of_a_pattern_item_rule_and_of_the_id_rule_hold_no_node_name() {
    let schema = "document:\n  sections:\n    - name: 節\n      items:\n        - pattern: \"^注\"\n          repeat: { max: 1 }\n        - id: \"^A-\"\n";
    let findings = validate_src(schema, "## 節\n\n### 注1\n\n### 注2\n");
    assert_eq!(
        summary(&findings),
        vec![
            (FindingKind::RepeatMaxExceeded, Some(5), None),
            (FindingKind::RepeatMinNotMet, Some(1), None),
        ]
    );
    assert!(findings[0].detail.contains("^注"), "{}", findings[0].detail);
}

// @kotowari[C-schema-doc-23]
#[test]
fn only_the_id_rule_requires_the_id_form_of_its_headings() {
    let schema = "document:\n  sections:\n    - name: 節\n      items:\n        - name: 概要\n        - id: \"^A-[0-9]+$\"\n          repeat: { min: 0 }\n";
    let doc = "## 節\n\n### 概要\n\n### A-1: 良い\n\n### 区切りの無い見出し\n\n### B-1: 合わない\n";
    let findings = validate_src(schema, doc);
    assert_eq!(
        summary(&findings),
        vec![
            (FindingKind::InvalidId, Some(7), None),
            (FindingKind::InvalidId, Some(9), None),
        ]
    );
}

// @kotowari[C-schema-doc-24]
#[test]
fn an_item_selected_by_name_has_the_whole_heading_as_its_name_and_no_id() {
    let schema = "document:\n  sections:\n    - name: 節\n      items:\n        - name: 問題\n          statement: { repeat: { min: 0 } }\n          extract: { path: problem, of: { id: id, name: name } }\n";
    let doc = "## 節\n\n### 問題\n\n本文。\n";
    assert!(validate_src(schema, doc).is_empty());
    assert_eq!(
        values_of(schema, doc)["problem"],
        serde_json::json!({ "name": "問題" })
    );
}

// @kotowari[C-schema-doc-24]
#[test]
fn an_item_selected_by_name_has_its_heading_as_the_heading_of_its_value() {
    let schema = "document:\n  sections:\n    - name: 節\n      items:\n        - name: \"問題: 見出し\"\n          statement: { repeat: { min: 0 } }\n          extract: problem\n";
    let doc = "## 節\n\n### 問題: 見出し\n\n本文。\n";
    assert_eq!(
        values_of(schema, doc)["problem"],
        serde_json::json!("問題: 見出し\n本文。")
    );
}

// @kotowari[C-schema-lang-18]
#[test]
fn items_lists_load_and_the_item_keys_do_not() {
    assert!(parse_schema("document:\n  items:\n    - id: \"^A-\"\n  sections:\n    - name: 節\n      items:\n        - name: 問題\n").is_ok());
    for schema in [
        "document:\n  sections:\n    - name: 節\n      item:\n        id: \"^A-\"\n",
        "document:\n  item:\n    id: \"^A-\"\n",
    ] {
        let error = parse_schema(schema).unwrap_err();
        assert!(error.0.contains("item"), "{error}");
    }
}

// @kotowari[C-schema-lang-4]
#[test]
fn an_item_rule_that_mixes_its_selectors_or_follows_the_id_rule_is_a_schema_error() {
    for items in [
        "- name: a\n          pattern: b\n",
        "- id: \"^A\"\n          name: a\n",
        "- id: \"^A\"\n          pattern: a\n",
        "- id: \"^A\"\n        - name: a\n",
        "- {}\n        - {}\n",
    ] {
        let schema =
            format!("document:\n  sections:\n    - name: 節\n      items:\n        {items}");
        assert!(parse_schema(&schema).is_err(), "{schema}");
    }
}

// @kotowari[C-schema-cli-21]
#[test]
fn the_same_name_twice_across_one_list_of_item_rules_is_a_schema_error() {
    let schema = "document:\n  sections:\n    - name: 節\n      items:\n        - name: [問題, Problem]\n        - name: Problem\n";
    let error = parse_schema(schema).unwrap_err();
    assert!(error.0.contains("declared twice"), "{error}");
}

// @kotowari[C-schema-lang-15]
#[test]
fn extraction_of_each_item_rule_takes_only_the_items_that_rule_selected() {
    let schema = "document:\n  sections:\n    - pattern: \"\"\n      repeat: { min: 0 }\n      items:\n        - name: 問題\n          repeat: { min: 0 }\n          extract: { path: problems, of: { line: line } }\n        - id: \"^C-\"\n          repeat: { min: 0 }\n          extract: { path: rows, of: { id: id, name: name } }\n";
    let doc = "## 一\n\n### 問題\n\n### C-1: 行\n\n## 二\n\n### 問題\n";
    let values = values_of(schema, doc);
    assert_eq!(
        values["problems"],
        serde_json::json!([{ "line": 3 }, { "line": 9 }])
    );
    assert_eq!(
        values["rows"],
        serde_json::json!([{ "id": "C-1", "name": "行" }])
    );
}
