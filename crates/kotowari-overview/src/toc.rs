//! 目次の検査（REQ-core-327〜REQ-core-331）と、描画のエンジンに渡す目次（REQ-core-332）

use kotowari_core::FindingKind;
use kotowari_markdown_view::{TocGroup, TocItem};
use serde_json::Value;
use std::collections::BTreeSet;

/// 形のスキーマ。パッケージの中に置いてコンパイル時に取り込む（TBL-core-043）
const SCHEMA: &str = include_str!("../schemas/toc.json");

/// 目次の検査の結果。指摘はどれも "line" が null で、種類と detail だけを持つ（REQ-core-027）
pub(crate) struct Checked {
    pub(crate) findings: Vec<(FindingKind, String)>,
    /// 形が正しければ書かれたままの目次
    pub(crate) toc: Option<TocGroup>,
}

/// 目次を検査する。names は元データのファイル名から ".md" を除いた名前
pub(crate) fn check(text: &str, names: &BTreeSet<String>) -> Checked {
    let value = match form(text) {
        Ok(value) => value,
        Err(places) => {
            return Checked {
                findings: places
                    .into_iter()
                    .map(|place| (FindingKind::OverviewTocInvalid, place))
                    .collect(),
                toc: None,
            };
        }
    };
    let mut comparison = Comparison {
        names,
        seen: BTreeSet::new(),
        findings: Vec::new(),
    };
    let toc = comparison.group(&value, "");
    // REQ-core-328: 目次のどこにも無い名前
    let missing: Vec<String> = names
        .iter()
        .filter(|name| !comparison.seen.contains(*name))
        .cloned()
        .collect();
    comparison.findings.extend(
        missing
            .into_iter()
            .map(|name| (FindingKind::OverviewTocPageMissing, name)),
    );
    Checked {
        findings: comparison.findings,
        toc: Some(toc),
    }
}

/// REQ-core-327: YAML として読み、スキーマに合えば値を、合わなければ合わなかった場所の並びを返す
fn form(text: &str) -> Result<Value, Vec<String>> {
    let value = if text.trim().is_empty() {
        Value::Null
    } else {
        serde_saphyr::from_str::<Value>(text).map_err(|_| vec!["(yaml)".to_string()])?
    };
    // 埋め込んだスキーマはテストで読めることを確かめている
    let schema: Value = serde_json::from_str(SCHEMA).expect("the embedded toc schema parses");
    let validator = jsonschema::validator_for(&schema).expect("the embedded toc schema compiles");
    let mut places: Vec<String> = Vec::new();
    for error in validator.iter_errors(&value) {
        for place in crate::parts::places_of(&error) {
            if !places.contains(&place) {
                places.push(place);
            }
        }
    }
    if places.is_empty() {
        Ok(value)
    } else {
        Err(places)
    }
}

/// 形の正しい目次を書かれた順に深さ優先でたどり、元データと照らす
struct Comparison<'a> {
    names: &'a BTreeSet<String>,
    seen: BTreeSet<String>,
    findings: Vec<(FindingKind, String)>,
}

impl Comparison<'_> {
    /// pointer はその目次の群の JSON Pointer（いちばん外側は空の文字列）
    fn group(&mut self, value: &Value, pointer: &str) -> TocGroup {
        let text = |key: &str| value.get(key).and_then(Value::as_str).map(str::to_string);
        let items = value
            .get("items")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if items.is_empty() {
            // REQ-core-330: いちばん外側は "(root)"
            let place = if pointer.is_empty() {
                "(root)"
            } else {
                pointer
            };
            self.findings
                .push((FindingKind::OverviewTocGroupEmpty, place.to_string()));
        }
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "serde_json::Value is a foreign enum: the remaining JSON kinds are deliberately handled alike"
        )]
        let items = items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let pointer = format!("{pointer}/items/{index}");
                match item {
                    Value::String(name) => {
                        self.name(name, pointer);
                        TocItem::Document(name.clone())
                    }
                    group => TocItem::Group(self.group(group, &pointer)),
                }
            })
            .collect();
        TocGroup {
            title: text("title").unwrap_or_default(),
            note: text("note"),
            items,
        }
    }

    /// REQ-core-329: 元データの無い名前は出てくるたびに、ある名前は2つ目以降に
    fn name(&mut self, name: &str, pointer: String) {
        if !self.names.contains(name) {
            self.findings
                .push((FindingKind::OverviewTocPageUnknown, pointer));
        } else if !self.seen.insert(name.to_string()) {
            self.findings
                .push((FindingKind::OverviewTocPageDuplicate, pointer));
        }
    }
}

#[cfg(test)]
mod tests {
    // @kotowari[TBL-core-043]
    #[test]
    fn the_embedded_toc_schema_compiles() {
        let schema: serde_json::Value = serde_json::from_str(super::SCHEMA).unwrap();
        assert!(jsonschema::validator_for(&schema).is_ok());
    }
}
