//! 部品を HTML にする（TBL-view-001）。値はスキーマに合うものとして読み、合わない欄は描かない

use crate::html::escape;
use crate::{Part, Reference, ReferenceState};
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

    /// 参照を表示名で描き、選ぶとページを移らずに本文を開く。どの参照もページの外へリンクしない
    /// （REQ-view-008）。表に無い参照は文字のまま描く
    fn draw(&self, key: &str) -> String {
        let Some(reference) = self.0.get(key) else {
            return format!(
                "<details class=\"ref\"><summary>{}</summary></details>",
                escape(key)
            );
        };
        let (class, mark) = match reference.state {
            ReferenceState::Current => ("ref", ""),
            ReferenceState::Superseded => (
                "ref ref-superseded",
                "<span class=\"ref-mark\">（置き換え済み）</span>",
            ),
            ReferenceState::Deferred => (
                "ref ref-deferred",
                "<span class=\"ref-mark\">（後回し）</span>",
            ),
        };
        format!(
            "<details class=\"{class}\"><summary>{}{mark}</summary><div class=\"ref-body\">{}</div></details>",
            escape(&reference.label),
            escape(&reference.body)
        )
    }

    /// "refs" の欄の参照の並び
    fn draw_all(&self, value: &Value) -> String {
        let keys: Vec<&str> = list(value, "refs")
            .iter()
            .filter_map(Value::as_str)
            .collect();
        if keys.is_empty() {
            return String::new();
        }
        let mut out = String::from("<div class=\"refs\">");
        for key in keys {
            out.push_str(&self.draw(key));
        }
        out.push_str("</div>");
        out
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

/// 欄の文字列を逃がした要素で包む。空なら何も描かない
fn optional(value: &Value, key: &str, class: &str) -> String {
    match text(value, key) {
        "" => String::new(),
        found => format!("<div class=\"{class}\">{}</div>", escape(found)),
    }
}

fn tone(value: &Value) -> String {
    match text(value, "tone") {
        "" => String::new(),
        tone => format!(" tone-{}", escape(tone)),
    }
}

/// 値に "width": "half" を持つか（REQ-view-015）
pub(crate) fn is_half(part: &Part) -> bool {
    part.value.get("width").and_then(Value::as_str) == Some("half")
}

/// 一覧に出す lead の結論
pub(crate) fn conclusion(lead: &Part) -> &str {
    text(&lead.value, "conclusion")
}

/// 部品1つを描く。知らない種類は何も描かない
pub(crate) fn part(part: &Part, refs: &Refs) -> String {
    let value = &part.value;
    let inner = match part.kind.as_str() {
        "lead" => lead(value),
        "flow" => flow(value),
        "steps" => steps(value, refs),
        "cards" => cards(value),
        "status" => status(value, refs),
        "compare" => compare(value, refs),
        "decisions" => decisions(value, refs),
        "quiz" => quiz(value, refs),
        _ => return String::new(),
    };
    format!("<div class=\"part {}\">{inner}</div>\n", escape(&part.kind))
}

fn lead(value: &Value) -> String {
    let mut out = format!(
        "<p class=\"conclusion\">{}</p>",
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
    out
}

/// 列を左から右へ並べ、列の中の箱を上から下へ並べる。位置は格子が決める
fn flow(value: &Value) -> String {
    let mut out = String::from("<div class=\"flow\">");
    for column in list(value, "columns") {
        out.push_str("<div class=\"flow-column\">");
        for item in column.as_array().map_or(&[][..], Vec::as_slice) {
            out.push_str(&format!(
                "<div class=\"box{}\"><div class=\"box-title\">{}</div>{}</div>",
                tone(item),
                escape(text(item, "title")),
                optional(item, "body", "box-body")
            ));
        }
        out.push_str("</div>");
    }
    out.push_str("</div>");
    out
}

fn steps(value: &Value, refs: &Refs) -> String {
    let mut out = String::from("<ol class=\"steps\">");
    for item in list(value, "items") {
        out.push_str(&format!(
            "<li><div class=\"step-title\">{}</div>{}{}</li>",
            escape(text(item, "title")),
            optional(item, "body", "step-body"),
            refs.draw_all(item)
        ));
    }
    out.push_str("</ol>");
    out
}

fn cards(value: &Value) -> String {
    let mut out = String::from("<div class=\"cards\">");
    for card in list(value, "cards") {
        out.push_str(&format!(
            "<div class=\"card{}\"><div class=\"card-title\">{}</div><ul>",
            tone(card),
            escape(text(card, "title"))
        ));
        for item in list(card, "items") {
            out.push_str(&format!("<li>{}</li>", escape(item.as_str().unwrap_or(""))));
        }
        out.push_str("</ul></div>");
    }
    out.push_str("</div>");
    out
}

/// 状態の札の class。スキーマが許す4つの状態のほかは札の色を付けない
fn state_class(state: &str) -> &'static str {
    match state {
        "決定" => "state-decided",
        "予定" => "state-planned",
        "未決" => "state-open",
        "取り下げ" => "state-dropped",
        _ => "state-unknown",
    }
}

fn status(value: &Value, refs: &Refs) -> String {
    let mut out = String::from("<ul class=\"status\">");
    for item in list(value, "items") {
        let state = text(item, "state");
        out.push_str(&format!(
            "<li><span class=\"badge {}\">{}</span><span class=\"text\">{}</span>{}</li>",
            state_class(state),
            escape(state),
            escape(text(item, "text")),
            refs.draw_all(item)
        ));
    }
    out.push_str("</ul>");
    out
}

fn compare(value: &Value, refs: &Refs) -> String {
    let mut out =
        String::from("<table><thead><tr><th>前</th><th>後</th><th>理由</th></tr></thead><tbody>");
    for item in list(value, "items") {
        out.push_str(&format!(
            "<tr><td><del>{}</del></td><td>{}</td><td class=\"why\">{}{}</td></tr>",
            escape(text(item, "before")),
            escape(text(item, "after")),
            escape(text(item, "why")),
            refs.draw_all(item)
        ));
    }
    out.push_str("</tbody></table>");
    out
}

fn decisions(value: &Value, refs: &Refs) -> String {
    format!(
        "<ul class=\"decisions\">{}</ul>",
        decision_nodes(list(value, "roots"), refs)
    )
}

fn decision_nodes(nodes: &[Value], refs: &Refs) -> String {
    let mut out = String::new();
    for node in nodes {
        out.push_str(&format!(
            "<li class=\"decision\">{}<span class=\"text\">{}</span><span class=\"by\">{}</span>",
            refs.draw(text(node, "ref")),
            escape(text(node, "text")),
            escape(text(node, "by"))
        ));
        let children = list(node, "children");
        if !children.is_empty() {
            out.push_str(&format!("<ul>{}</ul>", decision_nodes(children, refs)));
        }
        out.push_str("</li>");
    }
    out
}

fn quiz(value: &Value, refs: &Refs) -> String {
    let mut out = String::new();
    for item in list(value, "items") {
        out.push_str(&format!(
            "<details><summary>{}</summary><div class=\"answer\">{}</div>{}</details>",
            escape(text(item, "q")),
            escape(text(item, "a")),
            refs.draw_all(item)
        ));
    }
    out
}
