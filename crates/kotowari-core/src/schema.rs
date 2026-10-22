//! IR の形を宣言したスキーマ（REQ-core-168）。
//!
//! スキーマの YAML はコンパイル時に取り込む。実行時にスキーマのファイルを読む経路は無く、
//! IR の文書は自分の形を宣言しないので、文書の種類からスキーマを選ぶ。

use crate::doc_kind::DocKind;
use kotowari_markdown_schema::{Schema, SchemaError};
use std::sync::LazyLock;
fn parse_schema(yaml: &str) -> Result<Schema, SchemaError> {
    Schema::parse(yaml)
}

/// 話題ごとの文書のスキーマ
const TOPIC: &str = include_str!("../schemas/ir.yaml");
/// 用語集（CONTEXT.md）のスキーマ
const GLOSSARY: &str = include_str!("../schemas/context.yaml");
/// 問題の記録（FLAGS.md）のスキーマ
const FLAGS: &str = include_str!("../schemas/flags.yaml");
/// 計画書のスキーマ（REQ-core-191）
const PLAN: &str = include_str!("../schemas/plan.yaml");

/// 文書の種類に応じたスキーマ（REQ-core-168）。取り込んだ YAML は実行中に変わらないので、
/// 種類ごとに最初に要ったときに1回だけ読む
pub fn schema_for(kind: DocKind) -> Result<&'static Schema, &'static SchemaError> {
    static TOPIC_SCHEMA: LazyLock<Result<Schema, SchemaError>> =
        LazyLock::new(|| parse_schema(TOPIC));
    static GLOSSARY_SCHEMA: LazyLock<Result<Schema, SchemaError>> =
        LazyLock::new(|| parse_schema(GLOSSARY));
    static FLAGS_SCHEMA: LazyLock<Result<Schema, SchemaError>> =
        LazyLock::new(|| parse_schema(FLAGS));
    match kind {
        DocKind::Topic => &TOPIC_SCHEMA,
        DocKind::Glossary => &GLOSSARY_SCHEMA,
        DocKind::Flags => &FLAGS_SCHEMA,
    }
    .as_ref()
}

/// 計画書のスキーマを読む（REQ-core-191）。IR の文書の種類とは別に持つ。
pub fn plan_schema() -> Result<Schema, SchemaError> {
    parse_schema(PLAN)
}

#[cfg(test)]
mod tests {
    use super::*;

    // @kotowari[REQ-core-168]
    #[test]
    fn req_core_168_every_embedded_schema_parses() {
        for kind in [DocKind::Topic, DocKind::Glossary, DocKind::Flags] {
            assert!(
                schema_for(kind).is_ok(),
                "{kind:?} のスキーマが parse_schema を通らない: {:?}",
                schema_for(kind).err()
            );
        }
    }

    // @kotowari[REQ-core-168]
    #[test]
    fn req_core_168_the_schema_chosen_per_document_kind_is_the_one_named_for_it() {
        let name = |kind| schema_for(kind).unwrap().name().map(str::to_owned);
        assert_eq!(name(DocKind::Topic).as_deref(), Some("ir"));
        assert_eq!(name(DocKind::Glossary).as_deref(), Some("ir-context"));
        assert_eq!(name(DocKind::Flags).as_deref(), Some("ir-flags"));
    }

    // @kotowari[REQ-core-191]
    #[test]
    fn req_core_191_the_embedded_plan_schema_parses() {
        let schema = plan_schema();
        assert!(
            schema.is_ok(),
            "計画書のスキーマが parse_schema を通らない: {:?}",
            schema.err()
        );
        assert_eq!(schema.unwrap().name(), Some("plan"));
    }
}
