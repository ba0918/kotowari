//! "kotowari list" の項目の形（docs/decisions/core/list.md）

use crate::tests_discovery::TestMarker;
use serde::Serialize;
use std::collections::BTreeMap;

/// "kotowari list" の出力の最上位。"items" だけを持つ
#[derive(Debug, Serialize)]
pub struct ListResult {
    pub(crate) items: Vec<ListItem>,
}

/// "tests" の1件
#[derive(Debug, Clone, Serialize)]
pub struct TestRef {
    pub(crate) path: String,
    pub(crate) line: usize,
    /// 問い合わせの無い言語では null
    pub(crate) name: Option<String>,
}
impl TestRef {
    readonly!(copy line: usize);
    readonly!(borrow path: String, name: Option<String>);
}

/// 決定の文書の、確認することの1行の持つ鍵（docs/decisions/core/confirmations.md）
#[derive(Debug, Serialize)]
pub struct ConfirmationItem {
    pub(crate) id: String,
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) line: usize,
    /// その行を持つ決定の見出しの文
    pub(crate) decision: String,
    /// その決定の状態。"proposed" か "implemented"。状態の行が誤っていれば None
    pub(crate) state: Option<&'static str>,
    /// 頭に「（読んで確認）」と書いた行は true
    pub(crate) by_reading: bool,
    pub(crate) tests: Vec<TestRef>,
}
impl ConfirmationItem {
    readonly!(copy kind: &'static str, line: usize, state: Option<&'static str>, by_reading: bool);
    readonly!(borrow id: String, name: String, path: String, decision: String, tests: Vec<TestRef>);
}

/// 決定の文書の、決定1つの持つ鍵。ID はガイドの印が指す行き先（docs/decisions/core/languages-and-guides.md）
#[derive(Debug, Serialize)]
pub struct DecisionItem {
    /// "<文書のパス>#<見出しの行き先の名前>"
    pub(crate) id: String,
    pub(crate) kind: &'static str,
    /// 見出しの文
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) line: usize,
    /// "proposed" か "implemented"。状態の行が誤っていれば None
    pub(crate) state: Option<&'static str>,
    /// 「決めたこと」と「確認すること」の指紋
    pub(crate) fingerprint: String,
}
impl DecisionItem {
    readonly!(copy kind: &'static str, line: usize, state: Option<&'static str>);
    readonly!(borrow id: String, name: String, path: String, fingerprint: String);
}

/// 一覧の1件。鍵の集合は種類で決まるので、種類ごとの構造をそのまま出す
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ListItem {
    /// 決定の文書の、確認することの行
    Confirmation(ConfirmationItem),
    /// 決定の文書の決定
    Decision(DecisionItem),
}

impl ListItem {
    /// 1件の ID
    pub fn id(&self) -> &str {
        match self {
            Self::Confirmation(item) => &item.id,
            Self::Decision(item) => &item.id,
        }
    }

    /// 1件の "kind"
    pub fn kind(&self) -> &'static str {
        match self {
            ListItem::Confirmation(i) => i.kind,
            ListItem::Decision(i) => i.kind,
        }
    }

    /// 1件の "path" と "line"（並べ替えの鍵）
    pub fn location(&self) -> (&str, usize) {
        match self {
            Self::Confirmation(item) => (&item.path, item.line),
            Self::Decision(item) => (&item.path, item.line),
        }
    }
}

/// `ID` から、その `ID` を`印`に含む`テスト`の並びを引く
pub(crate) fn tests_by_id(markers: &[TestMarker]) -> BTreeMap<&str, Vec<TestRef>> {
    let mut by_id: BTreeMap<&str, Vec<TestRef>> = BTreeMap::new();
    for marker in markers {
        by_id.entry(&marker.id).or_default().push(TestRef {
            path: marker.path.clone(),
            line: marker.line,
            name: marker.name.clone(),
        });
    }
    for refs in by_id.values_mut() {
        refs.sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));
    }
    by_id
}
