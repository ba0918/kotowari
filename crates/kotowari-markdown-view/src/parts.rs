//! 部品を HTML にする（TBL-view-001）。値はスキーマに合うものとして読み、合わない欄は描かない

use crate::html::escape;
use crate::{Part, Reference};
use serde_json::Value;
use std::collections::BTreeMap;

/// 参照の文字列から参照の表の1件を引く
pub(crate) struct Refs<'a>(BTreeMap<&'a str, &'a Reference>);

impl<'a> Refs<'a> {
    pub(crate) fn new(references: &'a [Reference]) -> Self {
        Self(
            references
                .iter()
                .map(|reference| (reference.key.as_str(), reference))
                .collect(),
        )
    }
}

/// 欄の文字列。無いか文字列でなければ空
fn text<'v>(value: &'v Value, key: &str) -> &'v str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}

/// 欄の並び。無いか並びでなければ空
fn list<'v>(value: &'v Value, key: &str) -> &'v [Value] {
    value
        .get(key)
        .and_then(Value::as_array)
        .map_or(&[], Vec::as_slice)
}

/// 部品1つを描く
pub(crate) fn part(part: &Part, refs: &Refs) -> String {
    let _ = refs;
    let value = &part.value;
    match part.kind.as_str() {
        "lead" => lead(value),
        _ => String::new(),
    }
}

fn lead(value: &Value) -> String {
    let mut out = format!(
        "<div class=\"part lead\"><p class=\"conclusion\">{}</p>",
        escape(text(value, "conclusion"))
    );
    let points = list(value, "points");
    if !points.is_empty() {
        out.push_str("<ul class=\"points\">");
        for point in points {
            out.push_str(&format!(
                "<li>{}</li>",
                escape(point.as_str().unwrap_or(""))
            ));
        }
        out.push_str("</ul>");
    }
    out.push_str("</div>\n");
    out
}

/// 一覧に出す lead の結論
pub(crate) fn conclusion(lead: &Part) -> &str {
    text(&lead.value, "conclusion")
}
