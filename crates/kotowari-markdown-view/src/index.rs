//! 一覧のページ。目次のとおりに入れ子と順番で描き、状態の数を添える（REQ-view-005、REQ-view-016〜018、
//! REQ-view-021）

use crate::{Block, Document, Part, TocGroup, TocItem, html, parts};
use std::collections::BTreeMap;

/// 名前から文書を引く表。目次の名前のうち文書の無いものは描かない（REQ-view-021）
pub(crate) type Documents<'a> = BTreeMap<&'a str, &'a Document>;

/// 1つの文書か、目次の群の下の文書の、状態の数
#[derive(Default, Clone, Copy)]
struct Counts {
    pages: usize,
    stale: usize,
    open: usize,
    planned: usize,
}

impl Counts {
    /// REQ-view-016: 古い節と、lead に続く冒頭と節の中の status の部品の札
    fn of(document: &Document) -> Self {
        let in_sections = document.sections.iter().flat_map(|section| {
            section.blocks.iter().filter_map(|block| match block {
                Block::Part(part) => Some(part),
                Block::Markdown(_) => None,
            })
        });
        let labels: Vec<&str> = document
            .preamble
            .iter()
            .chain(in_sections)
            .flat_map(labels)
            .collect();
        Self {
            pages: 1,
            stale: document.sections.iter().filter(|s| s.stale).count(),
            open: labels.iter().filter(|label| **label == "未決").count(),
            planned: labels.iter().filter(|label| **label == "予定").count(),
        }
    }

    fn add(&mut self, other: Self) {
        self.pages += other.pages;
        self.stale += other.stale;
        self.open += other.open;
        self.planned += other.planned;
    }

    /// 文書の項目に添える数。どれも 0 なら何も描かない（REQ-view-016）
    fn card(&self) -> String {
        let badges = badges(&[
            ("見直していない節", self.stale),
            ("未決", self.open),
            ("予定", self.planned),
        ]);
        if badges.is_empty() {
            return badges;
        }
        format!("<p class=\"counts\">{badges}</p>")
    }

    /// 目次の群の見出しに添える数。ページの数は常に出す（REQ-view-017）
    fn group(&self) -> String {
        format!(
            "<span class=\"counts\"><span class=\"count\">{} ページ</span>{}</span>",
            self.pages,
            badges(&[("見直していない節", self.stale), ("未決", self.open)])
        )
    }
}

/// 0 でない数の札
fn badges(words: &[(&str, usize)]) -> String {
    words
        .iter()
        .filter(|(_, count)| *count != 0)
        .map(|(word, count)| format!("<span class=\"count\">{word} {count}</span>"))
        .collect()
}

/// status の部品の項目の札
fn labels(part: &Part) -> Vec<&str> {
    if part.kind != "status" {
        return Vec::new();
    }
    part.value
        .get("items")
        .and_then(serde_json::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("state").and_then(serde_json::Value::as_str))
        .collect()
}

/// 目次の群の場所。目次そのものは "toc"、入れ子の群は項目の位置を "-" でつなげる
pub(crate) fn anchor(path: &[usize]) -> String {
    let mut id = String::from("toc");
    for index in path {
        id.push_str(&format!("-{index}"));
    }
    id
}

/// 一覧のページ（REQ-view-005）
pub(crate) fn page(toc: &TocGroup, documents: &Documents) -> String {
    let (items, counts) = items(&toc.items, documents, &mut Vec::new());
    let mut body = format!(
        "<main class=\"page index\">\n<header class=\"toc-head\" id=\"{}\">\n<h1>{}</h1>\n{}\n</header>\n",
        anchor(&[]),
        html::escape(&toc.title),
        counts.group()
    );
    body.push_str(&note(toc));
    body.push_str(&items);
    body.push_str("</main>\n");
    html::shell(&toc.title, &body)
}

fn note(group: &TocGroup) -> String {
    group
        .note
        .as_ref()
        .map(|note| format!("<p class=\"note\">{}</p>\n", html::escape(note)))
        .unwrap_or_default()
}

/// 項目を書かれた順に描き、その下の数を合わせる（REQ-view-017、REQ-view-021）
fn items(items: &[TocItem], documents: &Documents, path: &mut Vec<usize>) -> (String, Counts) {
    let mut out = String::from("<ul class=\"entries\">\n");
    let mut total = Counts::default();
    for (index, item) in items.iter().enumerate() {
        match item {
            TocItem::Document(name) => {
                let Some(document) = documents.get(name.as_str()) else {
                    continue;
                };
                let counts = Counts::of(document);
                total.add(counts);
                out.push_str(&format!(
                    "<li class=\"entry\"><a href=\"{}\"><span class=\"title\">{}</span></a><p class=\"conclusion\">{}</p>{}</li>\n",
                    html::href(&format!("{name}.html")),
                    html::escape(&document.title),
                    html::escape(parts::conclusion(&document.lead)),
                    counts.card()
                ));
            }
            TocItem::Group(group) => {
                path.push(index);
                let (inner, counts) = self::items(&group.items, documents, path);
                total.add(counts);
                // REQ-view-018: 開いた状態で描き、見出しを選ぶと畳める。スクリプトは使わない
                out.push_str(&format!(
                    "<li class=\"group\" id=\"{}\"><details open>\n<summary><span class=\"group-title\">{}</span>{}</summary>\n{}{inner}</details></li>\n",
                    anchor(path),
                    html::escape(&group.title),
                    counts.group(),
                    note(group)
                ));
                path.pop();
            }
        }
    }
    out.push_str("</ul>\n");
    (out, total)
}

/// 文書の目次の中の位置。たどる目次の群（外側から、題名と一覧の中の場所）と、項目が直接属する群
pub(crate) struct Place<'a> {
    pub(crate) chain: Vec<(&'a str, String)>,
    pub(crate) group: &'a TocGroup,
}

/// 目次の名前ごとの位置。目次を書かれた順に深さ優先でたどって、最初に出てくるその名前の項目の位置を
/// 取る（REQ-view-019）。目次は描画ごとに1度だけたどる
pub(crate) fn places(toc: &TocGroup) -> BTreeMap<&str, Place<'_>> {
    let mut places = BTreeMap::new();
    let mut chain = vec![(toc.title.as_str(), anchor(&[]))];
    walk(toc, &mut Vec::new(), &mut chain, &mut places);
    places
}

fn walk<'a>(
    group: &'a TocGroup,
    path: &mut Vec<usize>,
    chain: &mut Vec<(&'a str, String)>,
    places: &mut BTreeMap<&'a str, Place<'a>>,
) {
    for (index, item) in group.items.iter().enumerate() {
        match item {
            TocItem::Document(name) => {
                places.entry(name.as_str()).or_insert_with(|| Place {
                    chain: chain.clone(),
                    group,
                });
            }
            TocItem::Group(inner) => {
                path.push(index);
                chain.push((inner.title.as_str(), anchor(path)));
                walk(inner, path, chain, places);
                chain.pop();
                path.pop();
            }
        }
    }
}
