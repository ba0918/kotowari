//! `項目`の参照（REQ-core-054）

use super::*;

/// 項目が `ID` を指している場所（TBL-core-027 の "via"）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Via {
    /// "- definition:" の行
    Definition,
    /// "- related:" の行
    Relations,
    /// "@about" のタグ
    About,
    /// 要求の文、性質の文、シナリオのステップの中のバッククォートで囲んだ `ID`
    Text,
}

impl Via {
    pub fn as_str(&self) -> &'static str {
        match self {
            Via::Definition => "definition",
            Via::Relations => "relations",
            Via::About => "about",
            Via::Text => "text",
        }
    }
}

/// 項目が指している `ID` の1件
#[derive(Debug, Clone, Copy)]
pub struct ItemReference<'a> {
    /// 指している先の `ID`
    pub id: &'a str,
    /// 指している場所
    pub via: Via,
    /// TBL-core-019: 指摘の行（定義・関係・文ならその行、"@about" ならタグの行）
    pub finding_line: usize,
}

/// REQ-core-054: 項目が指している `ID` をすべて拾う。
/// check の unresolved_reference と query の逆引きはどちらもここを読む。
/// 文とステップの中は `ID` の形に合うものだけを拾い、地の文の `ID` は拾わない
pub fn item_references<'a>(item: &'a Item) -> Vec<ItemReference<'a>> {
    // TBL-core-019: "- definition:" か "- related:" の行（行が無ければ見出しの行）
    let from_text = |texts: &'a [(usize, String)]| {
        texts.iter().flat_map(|(text_line, text)| {
            backtick_ids(text).into_iter().map(move |id| ItemReference {
                id,
                via: Via::Text,
                finding_line: *text_line,
            })
        })
    };

    match item {
        Item::Requirement {
            line,
            definitions,
            definition_line,
            statements,
            ..
        } => {
            let def_line = definition_line.unwrap_or(*line);
            definitions
                .iter()
                .map(|id| ItemReference {
                    id,
                    via: Via::Definition,
                    finding_line: def_line,
                })
                .chain(from_text(statements))
                .collect()
        }
        Item::Property { statements, .. } => from_text(statements).collect(),
        Item::Scenario {
            line,
            tag_line,
            about,
            steps,
            ..
        } => {
            let about_line = tag_line.unwrap_or(*line);
            about
                .iter()
                .map(|id| ItemReference {
                    id,
                    via: Via::About,
                    finding_line: about_line,
                })
                .chain(from_text(steps))
                .collect()
        }
        Item::FlagEntry {
            line,
            relations,
            relation_line,
            ..
        } => {
            let rel_line = relation_line.unwrap_or(*line);
            relations
                .iter()
                .map(|id| ItemReference {
                    id,
                    via: Via::Relations,
                    finding_line: rel_line,
                })
                .collect()
        }
        Item::DecisionTable { .. } | Item::GlossaryTerm { .. } => Vec::new(),
    }
}

/// 文やステップの中の、バッククォートで囲んだ `ID` を拾う（REQ-core-054、REQ-core-116）
pub(super) fn backtick_ids(text: &str) -> Vec<&str> {
    // REQ-core-054/REQ-core-116: 二重引用符の外だけを見る
    if crate::has_odd_backticks_outside_quotes(text) {
        return Vec::new();
    }
    crate::extract_backtick_contents_outside_quotes(text)
        .into_iter()
        .map(str::trim)
        .filter(|id| !id.is_empty() && is_valid_id(id))
        .collect()
}
