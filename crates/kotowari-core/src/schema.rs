//! kotowari が読む文書の形を宣言したスキーマ。
//!
//! スキーマの YAML はコンパイル時に取り込む。実行時にスキーマのファイルを読む経路は無い。

use kotowari_markdown_schema::{Schema, SchemaError};
use std::sync::LazyLock;
fn parse_schema(yaml: &str) -> Result<Schema, SchemaError> {
    Schema::parse(yaml)
}

/// 計画書のスキーマ
const PLAN: &str = include_str!("../schemas/plan.yaml");

/// 計画書のスキーマを読む
pub fn plan_schema() -> Result<Schema, SchemaError> {
    parse_schema(PLAN)
}

/// 決定の文書のスキーマ（docs/decisions/core/form-contract.md）。形の語の一覧は
/// `crate::decisions` の綴りの表と同じでなければならない
pub(crate) const DECISION: &str = include_str!("../schemas/decision.yaml");

/// 決定の文書のスキーマ。最初に要ったときに1回だけ読む
pub(crate) fn decision_schema() -> Result<&'static Schema, &'static SchemaError> {
    static DECISION_SCHEMA: LazyLock<Result<Schema, SchemaError>> =
        LazyLock::new(|| parse_schema(DECISION));
    DECISION_SCHEMA.as_ref()
}

#[cfg(test)]
mod tests {
    use super::*;

    // @kotowari[C-core-plan-1]
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
