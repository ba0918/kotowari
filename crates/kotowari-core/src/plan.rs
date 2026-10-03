//! "kotowari plan": 計画書1つの形を、同梱のスキーマで検査する（docs/ir/core/plan.md）。
//!
//! 形の読み取りはスキーマの側（mds）に任せ、IR の対応表（TBL-core-030）は通さずに、
//! スキーマの側の指摘を1件ずつ invalid_plan へ写す（REQ-core-193）。
//! 題名より前の空でない行は`除外`なので、空の行に置き換えてから検査する（REQ-core-192）。

use crate::schema::plan_schema;
use crate::{Finding, FindingKind};
use kotowari_markdown_schema::{Document, ValidationOptions};
use std::collections::BTreeMap;

/// "kotowari plan" の JSON の最上位（REQ-core-194）
#[derive(serde::Serialize)]
pub struct PlanResult {
    pub(crate) findings: Vec<Finding>,
    pub(crate) counts: BTreeMap<String, usize>,
}
impl PlanResult {
    readonly!(borrow findings: Vec<Finding>, counts: BTreeMap<String, usize>);
}
pub fn check(path: &str, content: &str) -> PlanResult {
    let mut findings = check_plan(path, content);
    crate::sort_findings(&mut findings);
    PlanResult {
        counts: crate::count_findings(&findings),
        findings,
    }
}

/// 計画書の中身を同梱のスキーマで検査し、指摘を invalid_plan にして返す（REQ-core-191、REQ-core-193）。
/// `path` は指摘の "path" に入れる表示用のパス。
pub fn check_plan(path: &str, content: &str) -> Vec<Finding> {
    // 取り込んだスキーマは組み立ての時点で決まり、読めることは schema.rs のテストが確かめる
    let schema = plan_schema().expect("the embedded plan schema parses");
    // mds は GFM と frontmatter だけで読み、MDX を使わない。その読み方の Markdown は
    // 構文の誤りを持たず、読み取りは失敗しない。先頭の frontmatter は中身を見ずに飛ばす
    // （REQ-core-191）
    let document = Document::parse(content).expect("Markdown without MDX always parses");
    // 題名より前の節が必須の節を満たしたり、題名の後の節と重なって数えられたりしないよう、
    // 行の番号を保ったまま空にしてから読み直す
    let document = match document.title_line() {
        Some(line) => Document::parse(&blank_lines_before(content, line))
            .expect("Markdown without MDX always parses"),
        None => document,
    };
    kotowari_markdown_schema::validate(&schema, &document, ValidationOptions::default())
        .into_iter()
        .map(|f| {
            Finding::new(
                FindingKind::InvalidPlan,
                path.to_string(),
                f.line(),
                format!("{}: {}", f.kind().as_str(), f.detail()),
            )
        })
        .collect()
}

/// 1始まりで `line` 行目より前の行を、改行を残して空にする
fn blank_lines_before(content: &str, line: usize) -> String {
    content
        .split_inclusive('\n')
        .enumerate()
        .map(|(i, text)| {
            if i + 1 < line {
                &text[text.trim_end_matches(['\r', '\n']).len()..]
            } else {
                text
            }
        })
        .collect()
}
