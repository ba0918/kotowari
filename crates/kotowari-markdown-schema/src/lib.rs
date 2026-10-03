//! スキーマ言語・検証・抽出のコア。CLI のフレームワークには依存しない。
//!
//! このクレートの仕様は `docs/ir/schema/` の IR ただ1つである。コメントから
//! 仕様を参照するときは、そこの要求・決定表・具体例・性質の ID
//! （`REQ-schema-nnn`・`TBL-schema-nnn`・`EX-schema-nnn`・`PROP-schema-nnn`）を書く。
//! `docs/spec/` は IR から起こした人間向けのビューなので、その節番号（`R<n>`）は
//! 参照しない。節番号は文書を書き直すたびに指す先が変わる。

pub mod ast;
mod document;
mod extract;
pub mod finding;
pub mod frontmatter;
mod line_reading;
mod schema;
mod validate;

pub use ast::{ParseError, ast_json};
pub use document::Document;
pub use frontmatter::{frontmatter_schema, resolve_schema};
pub use schema::{Schema, SchemaError};

pub fn validate(
    schema: &Schema,
    document: &Document,
    options: ValidationOptions,
) -> Vec<finding::Finding> {
    validate::validate(schema, document, schema.open || options.relax)
}

/// Options for document validation. Relaxation never closes an open schema.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationOptions {
    pub relax: bool,
}

/// Values obtained from a document without validation findings.
#[derive(Debug, Clone)]
pub struct ValidatedValues(serde_json::Value);

impl ValidatedValues {
    pub fn values(&self) -> &serde_json::Value {
        &self.0
    }
}

/// Obtainable JSON values together with all document validation findings.
#[derive(Debug)]
pub struct PartialExtraction {
    values: serde_json::Value,
    findings: Vec<finding::Finding>,
}

impl PartialExtraction {
    pub fn values(&self) -> &serde_json::Value {
        &self.values
    }

    pub fn findings(&self) -> &[finding::Finding] {
        &self.findings
    }
}

pub fn extract_partial(
    schema: &Schema,
    document: &Document,
    options: ValidationOptions,
) -> PartialExtraction {
    PartialExtraction {
        values: extract::extract_values(schema, document),
        findings: validate::validate(schema, document, schema.open || options.relax),
    }
}

pub fn extract_typed_partial(
    schema: &Schema,
    document: &Document,
    options: ValidationOptions,
) -> PartialExtraction {
    PartialExtraction {
        values: extract::extract_typed(schema, document),
        findings: validate(schema, document, options),
    }
}

pub fn extract_validated(
    schema: &Schema,
    document: &Document,
    options: ValidationOptions,
) -> Result<ValidatedValues, Vec<finding::Finding>> {
    let findings = validate::validate(schema, document, schema.open || options.relax);
    if findings.is_empty() {
        Ok(ValidatedValues(extract::extract_values(schema, document)))
    } else {
        Err(findings)
    }
}
