//! 元データの形の検査（REQ-core-281、TBL-core-038）。Markdown としての形は kotowari-markdown-schema の
//! スキーマで、frontmatter は Markdown の形の外なのでここで検査する

use kotowari_markdown_schema::{Document, Schema, ValidationOptions, finding::FindingKind};
use serde_json::Value;

/// 形のスキーマ。パッケージの中に置いてコンパイル時に取り込む（TBL-core-038）
const SCHEMA: &str = include_str!("../schemas/overview.yaml");

/// 形に合わない所の行（文書全体にかかるものは None）と、kotowari-markdown-schema の指摘の種類の名前
pub(crate) type FormFinding = (Option<usize>, &'static str);

pub(crate) struct Form {
    schema: Schema,
}

impl Form {
    pub(crate) fn new() -> Self {
        Self {
            // 埋め込んだスキーマはテストで読めることを確かめている
            schema: Schema::parse(SCHEMA).expect("the embedded overview schema parses"),
        }
    }

    /// Markdown としての形の違反
    pub(crate) fn markdown(&self, text: &str) -> Vec<FormFinding> {
        // GFM と frontmatter の読み方は MDX の構文を持たないので誤りを返さない
        let Ok(document) = Document::parse(text) else {
            return Vec::new();
        };
        kotowari_markdown_schema::validate(&self.schema, &document, ValidationOptions::default())
            .into_iter()
            .map(|finding| (finding.line(), finding.kind().as_str()))
            .collect()
    }
}

/// frontmatter の "ir" の一覧と、形の違反。一覧は "ir" そのものが1件以上の文字列の一覧のときだけ返し、
/// ほかの鍵の違反があっても返す（REQ-core-284 の検査を続けるため）。
/// frontmatter の違反には kotowari-markdown-schema の指摘の種類のうち、意味の近い名前を使う
pub(crate) fn frontmatter(yaml: Option<&str>) -> (Option<Vec<String>>, Vec<FormFinding>) {
    let missing = (None, FindingKind::MissingRequiredField.as_str());
    let mismatch = (None, FindingKind::FieldPatternMismatch.as_str());
    let Some(yaml) = yaml else {
        return (None, vec![missing]);
    };
    let Ok(Value::Object(map)) = serde_saphyr::from_str::<Value>(yaml) else {
        return (None, vec![mismatch]);
    };
    let mut findings: Vec<FormFinding> = map
        .keys()
        .filter(|key| key.as_str() != "ir")
        .map(|_| (None, FindingKind::UndeclaredLine.as_str()))
        .collect();
    let ir = match map.get("ir") {
        None => {
            findings.push(missing);
            None
        }
        Some(Value::Array(items)) if items.is_empty() => {
            findings.push((None, FindingKind::RepeatMinNotMet.as_str()));
            None
        }
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| item.as_str().map(str::to_string))
            .collect::<Option<Vec<_>>>()
            .or_else(|| {
                findings.push(mismatch);
                None
            }),
        Some(_) => {
            findings.push(mismatch);
            None
        }
    };
    (ir, findings)
}

#[cfg(test)]
mod tests {
    // @kotowari[TBL-core-038]
    #[test]
    fn the_embedded_overview_schema_parses() {
        assert!(kotowari_markdown_schema::Schema::parse(super::SCHEMA).is_ok());
    }
}
