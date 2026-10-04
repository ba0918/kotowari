//! "kotowari query" の1件の組み立て（REQ-core-156、REQ-core-159、REQ-core-160、TBL-core-027）

use crate::fingerprint;
use crate::ir::{self, IrDocument};
use crate::list::{self, ListItem};
use crate::tests_discovery::TestMarker;
use serde::Serialize;
use std::collections::BTreeMap;

/// "kotowari query" の出力の最上位。"items" だけを持つ（REQ-core-161）
#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub items: Vec<QueryItem>,
}

/// query の1件。"kotowari list" の1件に本文と逆引きを足したもの（TBL-core-027）
#[derive(Debug, Serialize)]
pub struct QueryItem {
    #[serde(flatten)]
    pub(crate) item: ListItem,
    pub(crate) body: Vec<String>,
    pub(crate) referenced_by: Vec<Reference>,
}
impl QueryItem {
    readonly!(borrow item: ListItem, body: Vec<String>, referenced_by: Vec<Reference>);
}

/// "referenced_by" の1件（TBL-core-027）
#[derive(Debug, Clone, Serialize)]
pub struct Reference {
    pub(crate) id: String,
    pub(crate) kind: &'static str,
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) via: &'static str,
}
impl Reference {
    readonly!(copy kind: &'static str, line: usize, via: &'static str);
    readonly!(borrow id: String, path: String);
}

/// 位置引数と同じ `ID` を持つ`項目`と`シナリオ`を組み立てる（REQ-core-156）。
/// 1件も無ければ None を返す（REQ-core-157: 呼び出し元が`停止`する）
pub fn build(
    docs: &[IrDocument],
    ir_path: &str,
    markers: &[TestMarker],
    id: &str,
) -> Option<QueryResult> {
    let listed = list::build(docs, ir_path, markers);
    let referenced_by = references_to(docs, ir_path, id, &listed.items);
    let mut bodies = bodies_of(docs, ir_path, id);

    let items: Vec<QueryItem> = listed
        .items
        .into_iter()
        .filter(|item| item.id() == id)
        .map(|item| {
            let (path, line) = item.location();
            let body = bodies.remove(&(path.to_string(), line)).unwrap_or_default();
            QueryItem {
                item,
                body,
                referenced_by: referenced_by.clone(),
            }
        })
        .collect();

    if items.is_empty() {
        None
    } else {
        Some(QueryResult { items })
    }
}

/// その `ID` を指している`項目`と`シナリオ`を集める（TBL-core-027 の "referenced_by"、REQ-core-160）。
/// 指す場所は REQ-core-054 が `ID` を読む場所と同じで、"line" は指している側の見出しの行
fn references_to(
    docs: &[IrDocument],
    ir_path: &str,
    id: &str,
    listed: &[ListItem],
) -> Vec<Reference> {
    // 逆引きに出るのは "kotowari list" に出るものと同じ項目で、"kind" もそこから取る
    let by_location: BTreeMap<(&str, usize), &ListItem> =
        listed.iter().map(|item| (item.location(), item)).collect();

    let mut references: Vec<Reference> = Vec::new();
    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.relative_path);
        for item in &doc.items {
            let line = item.item_line();
            let Some(listed) = by_location.get(&(path.as_str(), line)) else {
                continue;
            };
            // 1つの項目が同じ ID を同じ via で何度指しても1件にする（review6-gaps の A2）
            let mut seen_vias = std::collections::BTreeSet::new();
            for reference in ir::item_references(item) {
                if reference.id == id && seen_vias.insert(reference.via.as_str()) {
                    references.push(Reference {
                        id: listed.id().to_string(),
                        kind: listed.kind(),
                        path: path.clone(),
                        line,
                        via: reference.via.as_str(),
                    });
                }
            }
        }
    }
    references.sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));
    references
}

/// その `ID` を持つ`項目`と`シナリオ`の本文を、"path" と "line" から引ける形で集める
fn bodies_of(
    docs: &[IrDocument],
    ir_path: &str,
    id: &str,
) -> BTreeMap<(String, usize), Vec<String>> {
    let mut bodies = BTreeMap::new();
    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.relative_path);
        let lines = ir::split_lines(&doc.raw_content);
        for item in &doc.items {
            if item.id() == Some(id) {
                bodies.insert(
                    (path.clone(), item.item_line()),
                    fingerprint::body_of(item, &lines),
                );
            }
        }
    }
    bodies
}
