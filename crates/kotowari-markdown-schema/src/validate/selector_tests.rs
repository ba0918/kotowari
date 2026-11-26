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
