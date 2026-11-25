//! 決定の文書の読み取りと検査（docs/decisions/core/decision-documents.md、docs/decisions/core/confirmations.md）。
//! 古い形の IR とは別の経路で読み、古い形の検査には確認することの ID だけを渡す

use crate::{Finding, FindingKind};
use markdown::mdast::Node;
use std::collections::BTreeSet;

/// 決定の状態
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum State {
    /// 決めたが、まだ作っていない
    Proposed,
    /// 作ってあり、今の振る舞い
    Implemented,
}

/// 確認することの1行
#[derive(Debug, Clone)]
pub(crate) struct Confirmation {
    pub(crate) id: String,
    pub(crate) line: usize,
    /// 頭に「（読んで確認）」と書いた行
    pub(crate) by_reading: bool,
}

/// 1つの決定
#[derive(Debug, Clone)]
pub(crate) struct Decision {
    /// 状態が「提案」でも「実装済み」でもなければ None
    pub(crate) state: Option<State>,
    pub(crate) confirmations: Vec<Confirmation>,
}

/// 読んだ1つの話題の文書
pub(crate) struct Document {
    /// `基準のディレクトリ`からの相対パス
    pub(crate) path: String,
    pub(crate) decisions: Vec<Decision>,
    /// 「未決」の節の一覧の項目の数
    pub(crate) open_items: usize,
    /// 見出しへのリンクの行き先になる名前（GitHub の規則）
    pub(crate) anchors: BTreeSet<String>,
    pub(crate) links: Vec<crate::translations::MarkdownLink>,
}

/// 決定の文書の置き場から読んだものの数（check の "decisions"）
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct DecisionTally {
    pub(crate) documents: usize,
    pub(crate) decisions: usize,
    pub(crate) confirmations: usize,
}

impl DecisionTally {
    readonly!(copy documents: usize, decisions: usize, confirmations: usize);
}

const STATE: &str = "状態:";
const PROPOSED: &str = "提案";
const IMPLEMENTED: &str = "実装済み";
const PROBLEM: &str = "問題";
const DECIDED: &str = "決めたこと";
const CONFIRMATIONS: &str = "確認すること";
/// 文書の最後に置く、決定として扱わない見出し
const OPEN: &str = "未決";
/// テストを求めない行の頭の印
const BY_READING: &str = "（読んで確認）";

/// 1つの決定を読んでいる途中のもの
struct Building {
    heading_line: usize,
    /// 状態の行を探すのは、見出しの直後の段落だけ
    state_line: Option<(usize, String)>,
    awaiting_state: bool,
    sections: BTreeSet<String>,
    section: Option<String>,
    confirmations: Vec<Confirmation>,
}

impl Building {
    fn new(heading_line: usize) -> Self {
        Self {
            heading_line,
            state_line: None,
            awaiting_state: true,
            sections: BTreeSet::new(),
            section: None,
            confirmations: Vec::new(),
        }
    }

    fn finish(self, path: &str, findings: &mut Vec<Finding>) -> Decision {
        let state = match &self.state_line {
            Some((_, value)) if value == PROPOSED => Some(State::Proposed),
            Some((_, value)) if value == IMPLEMENTED => Some(State::Implemented),
            Some((line, value)) => {
                findings.push(Finding::new(
                    FindingKind::DecisionStateInvalid,
                    path.to_owned(),
                    Some(*line),
                    value.clone(),
                ));
                None
            }
            None => {
                findings.push(Finding::new(
                    FindingKind::DecisionStateInvalid,
                    path.to_owned(),
                    Some(self.heading_line),
                    String::new(),
                ));
                None
            }
        };
        for required in [PROBLEM, DECIDED] {
            if !self.sections.contains(required) {
                findings.push(Finding::new(
                    FindingKind::DecisionSectionMissing,
                    path.to_owned(),
                    Some(self.heading_line),
                    required.to_owned(),
                ));
            }
        }
        Decision {
            state,
            confirmations: self.confirmations,
        }
    }
}

/// どこを読んでいるか
enum Place {
    /// 題名より前か、題名と最初の決定の間
    Head,
    Decision(Building),
    Open,
}

fn line_of(node: &Node) -> usize {
    node.position().map_or(1, |position| position.start.line)
}

/// 話題の文書を読み、形の誤りを findings に積む
pub(crate) fn parse(path: &str, text: &str, findings: &mut Vec<Finding>) -> Document {
    let root = match crate::markdown::parse(text) {
        Ok(root) => root,
        Err(detail) => {
            findings.push(Finding::new(
                FindingKind::UnparsableFile,
                path.to_owned(),
                None,
                detail,
            ));
            return Document {
                path: path.to_owned(),
                decisions: Vec::new(),
                open_items: 0,
                anchors: BTreeSet::new(),
                links: Vec::new(),
            };
        }
    };
    let mut decisions = Vec::new();
    let mut place = Place::Head;
    let mut title: Option<(usize, String)> = None;
    let mut lead = false;
    let mut open_items = 0;
    let blocks = root.children().map(Vec::as_slice).unwrap_or_default();
    for node in blocks {
        if let Node::Heading(heading) = node
            && heading.depth <= 2
        {
            if let Place::Decision(building) = std::mem::replace(&mut place, Place::Head) {
                decisions.push(building.finish(path, findings));
            }
            if heading.depth == 1 {
                if title.is_none() {
                    title = Some((line_of(node), node.to_string()));
                }
            } else if node.to_string().trim() == OPEN {
                place = Place::Open;
            } else {
                place = Place::Decision(Building::new(line_of(node)));
            }
            continue;
        }
        match &mut place {
            Place::Head => {
                if title.is_some() && matches!(node, Node::Paragraph(_)) {
                    lead = true;
                }
            }
            Place::Decision(building) => read_decision_block(node, building),
            Place::Open => {
                if let Node::List(list) = node {
                    open_items += list.children.len();
                }
            }
        }
    }
    if let Place::Decision(building) = place {
        decisions.push(building.finish(path, findings));
    }
    match title {
        None => findings.push(Finding::new(
            FindingKind::MissingTitle,
            path.to_owned(),
            None,
            String::new(),
        )),
        Some((line, title)) if !lead => findings.push(Finding::new(
            FindingKind::TopicLeadMissing,
            path.to_owned(),
            Some(line),
            title,
        )),
        Some(_) => {}
    }
    Document {
        path: path.to_owned(),
        decisions,
        open_items,
        anchors: anchors(&root),
        links: crate::translations::markdown_links(&root),
    }
}

/// 文書のすべての見出しの、リンクの行き先になる名前。同じ名前の2つ目からは "-1"、"-2" を付ける
fn anchors(root: &Node) -> BTreeSet<String> {
    fn walk(node: &Node, out: &mut Vec<String>) {
        if let Node::Heading(_) = node {
            out.push(github_anchor(&node.to_string()));
        }
        for child in node.children().into_iter().flatten() {
            walk(child, out);
        }
    }
    let mut headings = Vec::new();
    walk(root, &mut headings);
    let mut anchors = BTreeSet::new();
    for heading in headings {
        let mut anchor = heading.clone();
        let mut suffix = 0;
        while !anchors.insert(anchor.clone()) {
            suffix += 1;
            anchor = format!("{heading}-{suffix}");
        }
    }
    anchors
}

/// GitHub が Markdown の見出しに付ける名前。小文字にし、文字・数字・"_"・"-"・空白のほかを除き、
/// 空白を "-" にする
fn github_anchor(heading: &str) -> String {
    heading
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '_' | '-' | ' '))
        .map(|c| if c == ' ' { '-' } else { c })
        .collect()
}

/// "%XX" の並びを UTF-8 として戻す。戻せなければ書いたまま
fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = bytes
            .get(i + 1..i + 3)
            .and_then(|pair| std::str::from_utf8(pair).ok())
            .and_then(|pair| u8::from_str_radix(pair, 16).ok());
        match (bytes[i], hex) {
            (b'%', Some(byte)) => {
                out.push(byte);
                i += 3;
            }
            (byte, _) => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8(out).unwrap_or_else(|_| text.to_owned())
}

/// スキームで始まるリンクの行き先（"https:"、"mailto:" など）
fn has_scheme(destination: &str) -> bool {
    destination.split_once(':').is_some_and(|(scheme, _)| {
        scheme
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
    })
}

/// 決定の文書の中のリンクのうち、置き場の中を指すものが、無い文書か無い見出しを指していれば指摘する。
/// 置き場の外を指すリンク、Markdown の文書でないファイルを指すリンク、スキームで始まるリンクは見ない
pub(crate) fn check_links(documents: &[Document], place: &str, findings: &mut Vec<Finding>) {
    let by_path: std::collections::BTreeMap<&str, &Document> = documents
        .iter()
        .map(|doc| (doc.path.as_str(), doc))
        .collect();
    let inside = |path: &str| place.is_empty() || path.starts_with(&format!("{place}/"));
    for doc in documents {
        let directory = doc.path.rsplit_once('/').map_or("", |(dir, _)| dir);
        for link in &doc.links {
            if has_scheme(&link.destination) {
                continue;
            }
            let (target, anchor) = link
                .destination
                .split_once('#')
                .map_or((link.destination.as_str(), None), |(path, anchor)| {
                    (path, Some(anchor))
                });
            let target = if target.is_empty() {
                doc.path.clone()
            } else {
                crate::normalize_path(&crate::join_display_path(directory, target))
            };
            // 決定の文書でないファイル（図など）は core が読まないので、在るかを判断できない
            if !inside(&target) || !target.ends_with(".md") {
                continue;
            }
            let found = by_path.get(target.as_str()).is_some_and(|target| {
                anchor.is_none_or(|anchor| target.anchors.contains(&percent_decode(anchor)))
            });
            if !found {
                findings.push(Finding::new(
                    FindingKind::LinkTargetMissing,
                    doc.path.clone(),
                    Some(link.line),
                    link.destination.clone(),
                ));
            }
        }
    }
}

/// どの文書かの「未決」の節に項目があるか
pub(crate) fn has_open_items(documents: &[Document]) -> bool {
    documents.iter().any(|doc| doc.open_items > 0)
}

/// 決定の見出しの下の1つのブロックを読む
fn read_decision_block(node: &Node, building: &mut Building) {
    let awaiting_state = std::mem::replace(&mut building.awaiting_state, false);
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "markdown::mdast::Node is a foreign enum: blocks other than headings, paragraphs and lists carry nothing a decision is read from"
    )]
    match node {
        Node::Heading(_) => {
            let name = node.to_string().trim().to_owned();
            building.sections.insert(name.clone());
            building.section = Some(name);
        }
        Node::Paragraph(_) if awaiting_state => {
            let text = node.to_string();
            let first = text.lines().next().unwrap_or_default();
            if let Some(value) = first.strip_prefix(STATE) {
                building.state_line = Some((line_of(node), value.trim().to_owned()));
            }
        }
        Node::List(list) if building.section.as_deref() == Some(CONFIRMATIONS) => {
            for item in &list.children {
                building.confirmations.push(confirmation(item));
            }
        }
        _ => {}
    }
}

/// 確認することの1行。ID は行の最初の語
fn confirmation(item: &Node) -> Confirmation {
    let text = item.to_string();
    let text = text.trim_start();
    let (id, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
    Confirmation {
        id: id.to_owned(),
        line: line_of(item),
        by_reading: rest.trim_start().starts_with(BY_READING),
    }
}

/// 読んだ文書の数
pub(crate) fn tally(documents: &[Document]) -> DecisionTally {
    DecisionTally {
        documents: documents.len(),
        decisions: documents.iter().map(|doc| doc.decisions.len()).sum(),
        confirmations: documents
            .iter()
            .flat_map(|doc| &doc.decisions)
            .map(|decision| decision.confirmations.len())
            .sum(),
    }
}

/// 確認することの ID の形（`C-<まとまりと話題の短い名前>-<連番>`）。名前は IR の ID の名前と同じ形で、
/// 連番は 1 から始まり先頭の 0 を認めない
pub(crate) fn is_confirmation_id(id: &str) -> bool {
    let Some((name, number)) = id.strip_prefix("C-").and_then(|rest| rest.rsplit_once('-')) else {
        return false;
    };
    crate::ir::is_valid_id_name(name)
        && !number.is_empty()
        && !number.starts_with('0')
        && number.chars().all(|c| c.is_ascii_digit())
}

fn rows(documents: &[Document]) -> impl Iterator<Item = (&Document, &Decision, &Confirmation)> {
    documents.iter().flat_map(|doc| {
        doc.decisions.iter().flat_map(move |decision| {
            decision
                .confirmations
                .iter()
                .map(move |row| (doc, decision, row))
        })
    })
}

/// 形の正しい確認することの ID。テストの印が指してよい ID になる
pub(crate) fn known_ids(documents: &[Document]) -> impl Iterator<Item = String> {
    rows(documents)
        .map(|(_, _, row)| row.id.clone())
        .filter(|id| is_confirmation_id(id))
}

/// 形の誤った ID と、同じ ID の2つ目以降の行を指摘する。documents はパスの順
pub(crate) fn check_ids(documents: &[Document], findings: &mut Vec<Finding>) {
    let mut seen = BTreeSet::new();
    for (doc, _, row) in rows(documents) {
        let kind = if !is_confirmation_id(&row.id) {
            FindingKind::InvalidId
        } else if !seen.insert(row.id.as_str()) {
            FindingKind::DuplicateId
        } else {
            continue;
        };
        findings.push(Finding::new(
            kind,
            doc.path.clone(),
            Some(row.line),
            row.id.clone(),
        ));
    }
}

/// 実装済みの決定の、テストで確かめる行のうち、どの印も指していない行を指摘する
pub(crate) fn check_tests(
    documents: &[Document],
    markers: &[crate::tests_discovery::TestMarker],
    findings: &mut Vec<Finding>,
) {
    let marked: BTreeSet<&str> = markers.iter().map(|marker| marker.id.as_str()).collect();
    for (doc, decision, row) in rows(documents) {
        if decision.state == Some(State::Implemented)
            && !row.by_reading
            && is_confirmation_id(&row.id)
            && !marked.contains(row.id.as_str())
        {
            findings.push(Finding::new(
                FindingKind::ConfirmationWithoutTest,
                doc.path.clone(),
                Some(row.line),
                row.id.clone(),
            ));
        }
    }
}
