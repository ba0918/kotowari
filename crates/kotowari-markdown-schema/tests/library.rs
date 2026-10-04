//! ライブラリの入口の契約。依存するクレートが実際に辿る道筋を、公開 API だけで通す。
//! ファイルの読み書きと URL の取得は呼び出し側の責務で、ここでは行わない。

use std::path::{Path, PathBuf};

use kotowari_markdown_schema::{Document, Schema, ValidationOptions};
fn extract_values(schema: &Schema, document: &Document) -> serde_json::Value {
    kotowari_markdown_schema::extract_partial(schema, document, ValidationOptions::default())
        .values()
        .clone()
}
use kotowari_markdown_schema::finding::FindingKind;
use kotowari_markdown_schema::frontmatter::{
    ResolvedSchema, SchemaRef, frontmatter_schema, resolve_schema,
};
fn parse_schema(yaml: &str) -> Result<Schema, kotowari_markdown_schema::SchemaError> {
    Schema::parse(yaml)
}
fn validate(
    schema: &Schema,
    document: &Document,
    relax: bool,
) -> Vec<kotowari_markdown_schema::finding::Finding> {
    kotowari_markdown_schema::validate(schema, document, ValidationOptions { relax })
}

const SCHEMA: &str = r#"
name: ir
open: false
document:
  title:
    pattern: "^.+"
    extract: title
  preamble:
    statement:
      required: true
      extract: scope
  sections:
    - name: 要求
      item:
        id: "REQ-\\d{3,}"
        repeat: { min: 0 }
        extract:
          path: requirements
          of: { id: id, line: line }
        fields:
          - name: 種類
            enum: [ubiquitous, algorithm]
            extract: kind
        statement:
          required: true
          extract: text
"#;

const DOCUMENT: &str = "---\n$schema: ../.kotowari/schemas/ir.yaml\n---\n# 題名\n\nこの文書が扱う範囲。\n\n## 要求\n\n### REQ-001: 名前\n\n- 種類: ubiquitous\n\n本文。\n";

// @kotowari[REQ-schema-068, REQ-schema-069, EX-schema-085]
#[test]
fn parsed_values_can_be_reused_for_validated_extraction() {
    use kotowari_markdown_schema::{Schema, ValidationOptions, extract_validated};
    let schema = Schema::parse(SCHEMA).unwrap();
    let document: Result<Document, kotowari_markdown_schema::ParseError> =
        Document::parse(DOCUMENT);
    let document = document.unwrap();
    for _ in 0..2 {
        let values = extract_validated(&schema, &document, ValidationOptions::default()).unwrap();
        assert_eq!(values.values()["requirements"][0]["id"], "REQ-001");
    }
}

// @kotowari[REQ-schema-068, EX-schema-086]
#[test]
fn semantic_schema_errors_are_rejected_before_document_operations() {
    use kotowari_markdown_schema::Schema;
    let error =
        Schema::parse("document:\n  sections:\n    - name: x\n      repeat: { min: 3, max: 1 }\n")
            .unwrap_err();
    assert!(error.0.contains("min is greater than max"));
    assert!(!error.to_string().trim().is_empty());
    let _: &dyn std::error::Error = &error;
}

// @kotowari[REQ-schema-069, EX-schema-087]
#[test]
fn violating_documents_have_partial_values_but_no_validated_values() {
    use kotowari_markdown_schema::{Schema, ValidationOptions, extract_partial, extract_validated};
    let schema = Schema::parse(SCHEMA).unwrap();
    let document = Document::parse(&DOCUMENT.replace("ubiquitous", "bogus")).unwrap();
    let findings = extract_validated(&schema, &document, ValidationOptions::default()).unwrap_err();
    assert_eq!(findings[0].kind(), FindingKind::FieldEnumInvalid);
    let partial = extract_partial(&schema, &document, ValidationOptions::default());
    assert_eq!(partial.values()["requirements"][0]["kind"], "bogus");
    assert_eq!(partial.findings()[0].kind(), FindingKind::FieldEnumInvalid);
}

// @kotowari[REQ-schema-069, REQ-schema-070, EX-schema-087, EX-schema-088]
#[test]
fn validation_respects_schema_open_and_explicit_relaxation() {
    use kotowari_markdown_schema::{Schema, ValidationOptions};
    let document = Document::parse("## Undeclared\n").unwrap();
    for open in [false, true] {
        let schema = Schema::parse(&format!("open: {open}\ndocument: {{}}\n")).unwrap();
        assert_eq!(schema.is_open(), open);
        for relax in [false, true] {
            let findings =
                kotowari_markdown_schema::validate(&schema, &document, ValidationOptions { relax });
            assert_eq!(findings.is_empty(), open || relax);
            let partial = kotowari_markdown_schema::extract_partial(
                &schema,
                &document,
                ValidationOptions { relax },
            );
            assert_eq!(partial.findings().is_empty(), open || relax);
            assert_eq!(
                kotowari_markdown_schema::extract_validated(
                    &schema,
                    &document,
                    ValidationOptions { relax }
                )
                .is_ok(),
                open || relax
            );
        }
    }
}

// @kotowari[REQ-schema-049, REQ-schema-014]
#[test]
fn a_null_schema_reference_has_an_explanatory_public_error() {
    let error = frontmatter_schema("---\n$schema: null\n---\n# Topic\n").unwrap_err();
    let message = error.to_string();
    assert!(message.contains("$schema"));
    assert!(!message.trim().is_empty());
}

// @kotowari[REQ-schema-071, EX-schema-089]
#[test]
fn typed_partial_json_keeps_obtainable_values_and_schema_name() {
    use kotowari_markdown_schema::{Schema, ValidationOptions, extract_typed_partial};
    let schema = Schema::parse(SCHEMA).unwrap();
    let document = Document::parse(&DOCUMENT.replace("ubiquitous", "bogus")).unwrap();
    let partial = extract_typed_partial(&schema, &document, ValidationOptions::default());
    assert_eq!(partial.values()["type"], "ir");
    assert_eq!(partial.values()["requirements"][0]["kind"], "bogus");
    assert_eq!(partial.findings()[0].kind(), FindingKind::FieldEnumInvalid);
}

// @kotowari[REQ-schema-049, EX-schema-015]
#[test]
fn a_dependent_crate_can_run_the_whole_pipeline() {
    let ast: Result<serde_json::Value, kotowari_markdown_schema::ParseError> =
        kotowari_markdown_schema::ast_json(DOCUMENT);
    assert_eq!(ast.unwrap()["type"], "root");
    // 1. 文書から "$schema" を読む
    let schema_ref = frontmatter_schema(DOCUMENT).unwrap().unwrap();
    assert!(
        matches!(schema_ref, SchemaRef::Relative(ref p) if p == "../.kotowari/schemas/ir.yaml"),
        "相対パスの参照をそのまま返す"
    );

    // 2. スキーマの位置を決める。読み込みは呼び出し側が行う
    let resolved = resolve_schema(Path::new("docs/ir/x.md"), &schema_ref);
    assert_eq!(
        resolved,
        ResolvedSchema::File(PathBuf::from("docs/.kotowari/schemas/ir.yaml")),
        "相対パスは文書の位置から解決する"
    );

    // 3. スキーマを読む
    let schema = parse_schema(SCHEMA).unwrap();

    // 4. 文書を読む
    let document = Document::parse(DOCUMENT).unwrap();

    // 5. 検査する
    let findings = validate(&schema, &document, false);
    assert!(
        findings.is_empty(),
        "合格の文書に指摘は出ない: {findings:?}"
    );

    // 6. 抽出する
    let values = extract_values(&schema, &document);
    assert_eq!(values["requirements"][0]["id"], "REQ-001");
    assert_eq!(values["requirements"][0]["kind"], "ubiquitous");
    assert_eq!(values["requirements"][0]["text"], "本文。");
    assert_eq!(values["requirements"][0]["line"], 10);
}

// @kotowari[REQ-schema-049]
#[test]
fn findings_carry_the_kind_line_and_detail() {
    let broken = DOCUMENT.replace("- 種類: ubiquitous", "- 種類: bogus");
    let schema = parse_schema(SCHEMA).unwrap();
    let document = Document::parse(&broken).unwrap();
    let findings = validate(&schema, &document, false);
    let finding = findings.first().expect("指摘が1件は出る");
    assert_eq!(finding.kind(), FindingKind::FieldEnumInvalid);
    assert_eq!(finding.line(), Some(12));
    assert!(!finding.detail().is_empty());
}

// @kotowari[REQ-schema-049, REQ-schema-050, EX-schema-016]
#[test]
fn a_url_schema_resolves_to_a_url_for_the_caller_to_fetch() {
    let source = "---\n$schema: https://example.com/ir.yaml\n---\n# 題名\n\n範囲。\n";
    let schema_ref = frontmatter_schema(source).unwrap().unwrap();
    let resolved = resolve_schema(Path::new("docs/ir/x.md"), &schema_ref);
    assert_eq!(
        resolved,
        ResolvedSchema::Url("https://example.com/ir.yaml".to_string()),
        "URL は位置だけを返し、取得は呼び出し側が行う"
    );
}
