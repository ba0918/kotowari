//! "kotowari query" の1件の形（docs/decisions/core/query.md）

use crate::list::ListItem;
use serde::Serialize;

/// "kotowari query" の出力の最上位。"items" だけを持つ
#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub items: Vec<QueryItem>,
}

/// query の1件。"kotowari list" の1件に本文と逆引きを足したもの
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

/// "referenced_by" の1件
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
