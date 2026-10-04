//! 部品の中身の検査（REQ-core-282）と参照の取り出し（REQ-core-285）

use serde_json::Value;
use std::collections::BTreeMap;

/// 描画のエンジンが公開するスキーマで部品を検査するもの。種類ごとに1回だけ組み立てる
pub(crate) struct Parts {
    validators: BTreeMap<&'static str, jsonschema::Validator>,
}

/// 部品の検査の結果
pub(crate) enum Checked {
    /// 描画のエンジンに無い種類
    Unknown,
    /// 合わなかった場所の並び。同じ場所は1つにまとめてある
    Invalid(Vec<String>),
    Valid(Value),
}

impl Parts {
    pub(crate) fn new() -> Self {
        let validators = kotowari_markdown_view::PART_KINDS
            .iter()
            .filter_map(|kind| {
                let schema = kotowari_markdown_view::part_schema(kind)?;
                let schema: Value = serde_json::from_str(schema).ok()?;
                Some((*kind, jsonschema::validator_for(&schema).ok()?))
            })
            .collect();
        Self { validators }
    }

    pub(crate) fn check(&self, kind: &str, content: &str) -> Checked {
        let Some(validator) = self.validators.get(kind) else {
            return Checked::Unknown;
        };
        let value = if content.trim().is_empty() {
            Value::Null
        } else {
            match serde_saphyr::from_str::<Value>(content) {
                Ok(value) => value,
                Err(_) => return Checked::Invalid(vec!["(yaml)".into()]),
            }
        };
        let mut places: Vec<String> = Vec::new();
        for error in validator.iter_errors(&value) {
            for place in places_of(&error) {
                if !places.contains(&place) {
                    places.push(place);
                }
            }
        }
        if places.is_empty() {
            Checked::Valid(value)
        } else {
            Checked::Invalid(places)
        }
    }
}

/// RFC 6901 のとおり、JSON Pointer の1つの成分にする
fn escape(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

/// 合わなかった場所。知らない鍵と欠けた鍵はその鍵の名前を "/" で足し、値の全体は "(root)"
pub(crate) fn places_of(error: &jsonschema::ValidationError) -> Vec<String> {
    use jsonschema::error::ValidationErrorKind;
    let base = error.instance_path().to_string();
    let keys: Vec<String> = match error.kind() {
        ValidationErrorKind::AdditionalProperties { unexpected } => {
            unexpected.iter().map(|key| escape(key)).collect()
        }
        ValidationErrorKind::Required { property } => {
            vec![escape(property.as_str().unwrap_or_default())]
        }
        _ => {
            return vec![if base.is_empty() {
                "(root)".into()
            } else {
                base
            }];
        }
    };
    keys.into_iter()
        .map(|key| format!("{base}/{key}"))
        .collect()
}

/// スキーマに合う部品の値の中の、名前が "refs" の欄の文字列の1件と "ref" の欄の文字列（入れ子の深さを問わない）
pub(crate) fn references(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                match (key.as_str(), child) {
                    ("refs", Value::Array(items)) => {
                        found.extend(items.iter().filter_map(Value::as_str).map(str::to_string));
                    }
                    ("ref", Value::String(reference)) => found.push(reference.clone()),
                    _ => references(child, found),
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                references(item, found);
            }
        }
        _ => {}
    }
}
