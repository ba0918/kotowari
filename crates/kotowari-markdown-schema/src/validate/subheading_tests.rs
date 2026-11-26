//! 深さ4以上の見出しを、それを含むノードの行として読み、"subheading" の規則に照らす。

use super::*;
use crate::document::Document;
use crate::schema::parse_schema;

fn validate_src(schema_yaml: &str, doc: &str, open: bool) -> Vec<Finding> {
    let schema = parse_schema(schema_yaml).unwrap();
    let document = Document::parse(doc).unwrap();
    validate(&schema, &document.read_as(schema.reading), open)
}

fn lines_of(findings: &[Finding], kind: FindingKind) -> Vec<usize> {
    findings
        .iter()
        .filter(|f| f.kind == kind)
        .filter_map(|f| f.line)
        .collect()
}

fn item_schema(extra: &str) -> String {
    format!(
        "document:\n  sections:\n    - name: 節\n      items:\n      - id: \"^A-\"\n        bullets:\n          repeat: {{ min: 0 }}\n{extra}"
    )
}

const ITEM_WITH_SUBHEADING: &str = "## 節\n\n### A-1: 名前\n\n- 前\n\n#### 小見出し\n\n- 後\n";

// @kotowari[C-schema-doc-20]
#[test]
fn a_deep_heading_without_a_subheading_rule_is_a_heading_level_mismatch_and_nothing_else() {
    let findings = validate_src(&item_schema(""), ITEM_WITH_SUBHEADING, false);
    assert_eq!(
        lines_of(&findings, FindingKind::HeadingLevelMismatch),
        vec![7]
    );
    assert_eq!(findings.len(), 1, "{findings:?}");
}

// @kotowari[C-schema-doc-20]
#[test]
fn a_deep_heading_in_an_undeclared_section_of_the_open_world_is_a_heading_level_mismatch() {
    let schema = "document:\n  sections:\n    - name: 節\n";
    let doc = "## 節\n\n## 別\n\n本文。\n\n##### 深い\n\n### 項目\n\n#### もっと\n";
    let findings = validate_src(schema, doc, true);
    assert_eq!(
        lines_of(&findings, FindingKind::HeadingLevelMismatch),
        vec![7, 11]
    );
    assert_eq!(findings.len(), 2, "{findings:?}");
}

// @kotowari[C-schema-doc-20]
#[test]
fn a_deep_heading_in_an_undeclared_section_of_the_closed_world_is_a_heading_level_mismatch() {
    let schema = "document:\n  sections:\n    - name: 節\n";
    let doc = "## 節\n\n## 別\n\n#### 深い\n";
    let findings = validate_src(schema, doc, false);
    assert_eq!(
        lines_of(&findings, FindingKind::HeadingLevelMismatch),
        vec![5]
    );
    assert_eq!(
        lines_of(&findings, FindingKind::UndeclaredLine),
        Vec::<usize>::new()
    );
}

// @kotowari[C-schema-doc-21]
#[test]
fn bullets_after_a_deep_heading_stay_in_the_item_and_are_matched_by_its_bullet_rule() {
    let schema = item_schema("          pattern: \"^前$\"\n");
    let findings = validate_src(&schema, ITEM_WITH_SUBHEADING, false);
    assert_eq!(
        lines_of(&findings, FindingKind::BulletPatternMismatch),
        vec![9],
        "{findings:?}"
    );
}

// @kotowari[C-schema-doc-21]
#[test]
fn bullets_after_a_deep_heading_enter_the_item_extraction_and_do_not_cut_its_end() {
    let schema = parse_schema(&format!(
        "{}          extract: rows\n        subheading: {{ repeat: {{ min: 0 }} }}\n        repeat: {{ min: 0 }}\n        extract: {{ path: items, of: {{ end: end }} }}\n",
        item_schema("")
    ))
    .unwrap();
    let doc = format!("{ITEM_WITH_SUBHEADING}\n### A-2: 次\n");
    let document = Document::parse(&doc).unwrap();
    let values = crate::extract::extract_values(&schema, &document);
    assert_eq!(
        values["items"][0],
        serde_json::json!({ "rows": ["- 前", "- 後"], "end": 10 }),
        "{values}"
    );
}

// @kotowari[C-schema-doc-22]
#[test]
fn a_deep_heading_in_an_item_declaring_subheading_is_no_finding() {
    let schema = item_schema("        subheading: { repeat: { min: 0 } }\n");
    let findings = validate_src(&schema, ITEM_WITH_SUBHEADING, false);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-doc-22]
#[test]
fn a_deep_heading_in_the_preamble_and_a_section_declaring_subheading_is_no_finding() {
    let schema = "document:\n  preamble:\n    subheading: { required: false }\n  sections:\n    - name: 節\n      subheading: { required: false }\n";
    let findings = validate_src(schema, "#### 前\n\n## 節\n\n###### 中\n", false);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-schema-lang-26]
#[test]
fn a_subheading_cardinality_counts_the_deep_headings_of_the_item() {
    let schema = item_schema("        subheading: { repeat: { min: 1, max: 1 } }\n");
    let none = validate_src(&schema, "## 節\n\n### A-1: 名前\n\n- 行\n", false);
    let none = only_of(&none, FindingKind::RepeatMinNotMet);
    assert_eq!(none.line, Some(3));
    assert_eq!(none.rule_kind, Some(RuleKind::Subheading));
    let two = validate_src(
        &schema,
        "## 節\n\n### A-1: 名前\n\n#### 1つ目\n\n#### 2つ目\n",
        false,
    );
    let two = only_of(&two, FindingKind::RepeatMaxExceeded);
    assert_eq!(two.line, Some(7));
    assert_eq!(two.rule_kind, Some(RuleKind::Subheading));
}

// @kotowari[C-schema-lang-26]
#[test]
fn a_deep_heading_not_matching_the_subheading_pattern_is_a_subheading_pattern_mismatch() {
    let schema = item_schema("        subheading: { repeat: { min: 0 }, pattern: \"^注\" }\n");
    let doc = "## 節\n\n### A-1: 名前\n\n#### 注記\n\n#### 余談\n";
    let findings = validate_src(&schema, doc, false);
    assert_eq!(
        lines_of(&findings, FindingKind::SubheadingPatternMismatch),
        vec![7],
        "{findings:?}"
    );
    assert_eq!(findings.len(), 1, "{findings:?}");
}

fn only_of(findings: &[Finding], kind: FindingKind) -> &Finding {
    let matching: Vec<&Finding> = findings.iter().filter(|f| f.kind == kind).collect();
    assert_eq!(matching.len(), 1, "{findings:?}");
    matching[0]
}

// @kotowari[C-schema-lang-27]
#[test]
fn a_subheading_rule_with_an_extraction_is_a_schema_error() {
    let schema = item_schema("        subheading: { extract: notes }\n");
    let error = parse_schema(&schema).unwrap_err();
    assert!(error.0.contains("extract"), "{error}");
}

// @kotowari[C-schema-lang-4]
#[test]
fn a_subheading_rule_whose_lower_bound_exceeds_its_upper_bound_is_a_schema_error() {
    let schema = item_schema("        subheading: { repeat: { min: 2, max: 1 } }\n");
    assert!(parse_schema(&schema).is_err());
}
