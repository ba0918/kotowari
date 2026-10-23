//! IR 文書の読み込みと検査
//!
//! 文書の形の検査と`項目`の値の取り出しはスキーマの側（kotowari-markdown-schema）に任せ、
//! 返った`抽出`から `IrDocument` と `Item` を組み立てる（REQ-core-169）。kotowari が生の行を
//! 読むのは、gherkin の塊の中身と、閉じない`コードブロック`の検出の2つだけ。

use crate::config::Config;
pub use crate::doc_kind::DocKind;
use crate::finding_map::read_document;
use crate::{Finding, FindingKind, StopReason};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

mod check;
mod extract;
mod fence;
mod gherkin;
mod id;
mod model;
mod partial;
mod references;
pub use check::*;
pub use extract::*;
pub(crate) use fence::*;
use gherkin::*;
pub use id::*;
pub use model::*;
pub use partial::*;
pub use references::*;

/// 行を "\n"、"\r\n"、単独の "\r" で分割する。"\r\n" は1つの行の終わりに数える（TBL-core-010）
pub fn split_lines(content: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut rest = content;
    while let Some(pos) = rest.find(['\n', '\r']) {
        lines.push(&rest[..pos]);
        let ending = if rest[pos..].starts_with("\r\n") {
            2
        } else {
            1
        };
        rest = &rest[pos + ending..];
    }
    // 最後の行（改行なし）
    if !rest.is_empty() {
        lines.push(rest);
    }
    lines
}
