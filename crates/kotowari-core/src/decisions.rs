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

impl State {
    /// list と query の "state" の値
    fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::Implemented => "implemented",
        }
    }
}

/// 確認することの1行
#[derive(Debug, Clone)]
pub(crate) struct Confirmation {
    pub(crate) id: String,
    pub(crate) line: usize,
    /// 頭に「（読んで確認）」と書いた行
    pub(crate) by_reading: bool,
    /// ID と「（読んで確認）」のあとの文。行を折り返していれば1つの空白でつなぐ
    pub(crate) name: String,
    /// 文書に書いたままの行
    pub(crate) body: Vec<String>,
}

/// 1つの決定
#[derive(Debug, Clone)]
pub(crate) struct Decision {
    /// 見出しの文
    pub(crate) heading: String,
    /// 見出しの行
    pub(crate) line: usize,
    /// 見出しへのリンクの行き先になる名前。同じ見出しの2つ目からは "-1"、"-2" が付いている
    pub(crate) anchor: String,
    /// 「決めたこと」と「確認すること」の節の、見出しの行から節の最後のブロックの行まで。文書の順
    pub(crate) fingerprinted: Vec<String>,
    /// `fingerprinted` の指紋。ガイドの印と照らす（C-core-lang-5、C-core-lang-6）
    pub(crate) fingerprint: String,
    /// 「決めたこと」と「確認すること」の中の、引用を探す文字。表はセルごと、コードブロックは入れない
    pub(crate) quotable: Vec<String>,
    /// 状態が「提案」でも「実装済み」でもなければ None
    pub(crate) state: Option<State>,
    /// 状態の行。無ければ見出しの行
    pub(crate) state_line: usize,
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

/// 状態ごとの決定の数（status の "decisions"、C-core-confirm-9）。状態の行が誤った決定はどちらにも数えない
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct DecisionStates {
    pub(crate) proposed: usize,
    pub(crate) implemented: usize,
}

impl DecisionStates {
    readonly!(copy proposed: usize, implemented: usize);
}

/// 形の語の、1つの言語での綴り。どちらの言語の綴りも、どの文書でも同じ意味に読む
struct Keywords {
    /// 状態の行の頭
    state: &'static str,
    proposed: &'static str,
    implemented: &'static str,
    problem: &'static str,
    decided: &'static str,
    rejected: &'static str,
    confirmations: &'static str,
    /// 文書の最後に置く、決定として扱わない見出し
    open: &'static str,
    /// テストを求めない行の頭の印
    by_reading: &'static str,
}

const JAPANESE: Keywords = Keywords {
    state: "状態:",
    proposed: "提案",
    implemented: "実装済み",
    problem: "問題",
    decided: "決めたこと",
    rejected: "見送った案",
    confirmations: "確認すること",
    open: "未決",
    by_reading: "（読んで確認）",
};

const ENGLISH: Keywords = Keywords {
    state: "Status:",
    proposed: "proposed",
    implemented: "implemented",
    problem: "Problem",
    decided: "Decision",
    rejected: "Rejected options",
    confirmations: "Confirmations",
    open: "Open",
    by_reading: "(by reading)",
};

/// 認める綴りの組。最初が、どの言語で書いたか分からないときに指摘に使う綴り
const KEYWORDS: [&Keywords; 2] = [&JAPANESE, &ENGLISH];

/// 決定の中の "###" の節のうち、形の語の付いたもの
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Section {
    Problem,
    Decided,
    Rejected,
    Confirmations,
}

impl Section {
    fn spelled(self, keywords: &Keywords) -> &'static str {
        match self {
            Self::Problem => keywords.problem,
            Self::Decided => keywords.decided,
            Self::Rejected => keywords.rejected,
            Self::Confirmations => keywords.confirmations,
        }
    }

    /// 見出しの文がどれかの言語の節の名前なら、その節
    fn of(heading: &str) -> Option<Self> {
        let heading = heading.trim();
        [
            Self::Problem,
            Self::Decided,
            Self::Rejected,
            Self::Confirmations,
        ]
        .into_iter()
        .find(|section| {
            KEYWORDS
                .iter()
                .any(|keywords| section.spelled(keywords) == heading)
        })
    }
}

/// "##" の見出しの文が、どれかの言語の「未決」か
fn is_open(heading: &str) -> bool {
    KEYWORDS
        .iter()
        .any(|keywords| keywords.open == heading.trim())
}

/// 状態の行の値を状態にする。どれかの言語の「提案」か「実装済み」でなければ None
fn state_of(value: &str) -> Option<State> {
    KEYWORDS.iter().find_map(|keywords| {
        if value == keywords.proposed {
            Some(State::Proposed)
        } else if value == keywords.implemented {
            Some(State::Implemented)
        } else {
            None
        }
    })
}

/// 段落の最初の行が状態の行なら、その綴りの組と、頭のあとの値
fn state_line(first: &str) -> Option<(&'static Keywords, &str)> {
    KEYWORDS.iter().find_map(|keywords| {
        first
            .strip_prefix(keywords.state)
            .map(|value| (*keywords, value.trim()))
    })
}

/// 確認することの行の ID のあとの文から、頭の「（読んで確認）」をどれかの言語の綴りで外す
fn strip_by_reading(rest: &str) -> Option<&str> {
    KEYWORDS
        .iter()
        .find_map(|keywords| rest.strip_prefix(keywords.by_reading))
}

/// 1つの決定を読んでいる途中のもの
struct Building {
    heading_line: usize,
    heading: String,
    anchor: String,
    /// 指紋に入れる節の (見出しの行, 最後のブロックの最後の行)
    fingerprinted: Vec<(usize, usize)>,
    /// 今の "###" の節が指紋に入れる節か
    in_fingerprinted: bool,
    quotable: Vec<String>,
    /// 状態の行を探すのは、見出しの直後の段落だけ。(行, 書いた綴りの組, 値)
    state_line: Option<(usize, &'static Keywords, String)>,
    awaiting_state: bool,
    sections: BTreeSet<Section>,
    section: Option<Section>,
    confirmations: Vec<Confirmation>,
}

impl Building {
    fn new(heading_line: usize, heading: String, anchor: String) -> Self {
        Self {
            heading_line,
            heading,
            anchor,
            fingerprinted: Vec::new(),
            in_fingerprinted: false,
            quotable: Vec::new(),
            state_line: None,
            awaiting_state: true,
            sections: BTreeSet::new(),
            section: None,
            confirmations: Vec::new(),
        }
    }

    fn finish(self, path: &str, lines: &[&str], findings: &mut Vec<Finding>) -> Decision {
        let state = match &self.state_line {
            Some((_, _, value)) if state_of(value).is_some() => state_of(value),
            Some((line, _, value)) => {
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
        // 欠けた節の名前は、状態の行を書いた言語の綴りで出す
        let keywords = self
            .state_line
            .as_ref()
            .map_or(KEYWORDS[0], |(_, keywords, _)| *keywords);
        for required in [Section::Problem, Section::Decided] {
            if !self.sections.contains(&required) {
                findings.push(Finding::new(
                    FindingKind::DecisionSectionMissing,
                    path.to_owned(),
                    Some(self.heading_line),
                    required.spelled(keywords).to_owned(),
                ));
            }
        }
        let fingerprinted: Vec<String> = self
            .fingerprinted
            .iter()
            .flat_map(|&(first, last)| {
                let mut section = lines
                    .get(first - 1..last.min(lines.len()))
                    .unwrap_or_default();
                // 一覧の最後の行の位置は、続く空行まで届くことがある
                while let [rest @ .., last] = section
                    && last.trim().is_empty()
                {
                    section = rest;
                }
                section.iter().map(|line| (*line).to_owned())
            })
            .collect();
        Decision {
            heading: self.heading,
            line: self.heading_line,
            anchor: self.anchor,
            fingerprint: crate::fingerprint::digest(&fingerprinted),
            fingerprinted,
            state,
            state_line: self
                .state_line
                .as_ref()
                .map_or(self.heading_line, |(line, _, _)| *line),
            confirmations: self.confirmations,
            quotable: self.quotable,
        }
    }
}

/// どこを読んでいるか
enum Place {
    /// 題名より前か、題名と最初の決定の間
    Head,
    Decision(Box<Building>),
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
    let lines: Vec<&str> = text.lines().collect();
    let anchored = heading_anchors(&root);
    let blocks = root.children().map(Vec::as_slice).unwrap_or_default();
    for node in blocks {
        if let Node::Heading(heading) = node
            && heading.depth <= 2
        {
            if let Place::Decision(building) = std::mem::replace(&mut place, Place::Head) {
                decisions.push(building.finish(path, &lines, findings));
            }
            if heading.depth == 1 {
                if title.is_none() {
                    title = Some((line_of(node), node.to_string()));
                }
            } else if is_open(&node.to_string()) {
                place = Place::Open;
            } else {
                let line = line_of(node);
                place = Place::Decision(Box::new(Building::new(
                    line,
                    node.to_string().trim().to_owned(),
                    anchored.get(&line).cloned().unwrap_or_default(),
                )));
            }
            continue;
        }
        match &mut place {
            Place::Head => {
                if title.is_some() && matches!(node, Node::Paragraph(_)) {
                    lead = true;
                }
            }
            Place::Decision(building) => read_decision_block(node, text, &lines, building),
            Place::Open => {
                if let Node::List(list) = node {
                    open_items += list.children.len();
                }
            }
        }
    }
    if let Place::Decision(building) = place {
        decisions.push(building.finish(path, &lines, findings));
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
        anchors: anchored.into_values().collect(),
        links: crate::translations::markdown_links(&root),
    }
}

/// `対`の`側`の`骨組み`（C-core-lang-4）。見出しは深さと、形の語の見出しならその節の種類だけで、文は
/// 比べない。状態の行は決定ごとの状態、確認することは行ごとの ID と（読んで確認）の有無。どれも
/// どちらの言語の綴りで書いても同じ要素になる
pub(crate) fn skeleton(text: &str, root: Option<&Node>) -> crate::skeleton::Skeleton {
    use crate::skeleton::Element;
    fn walk(node: &Node, out: &mut Vec<Element>) {
        if let Node::Heading(heading) = node {
            let name = node.to_string();
            let kind = match heading.depth {
                2 if is_open(&name) => ":open",
                3 => match Section::of(&name) {
                    Some(Section::Problem) => ":problem",
                    Some(Section::Decided) => ":decided",
                    Some(Section::Rejected) => ":rejected",
                    Some(Section::Confirmations) => ":confirmations",
                    None => "",
                },
                _ => "",
            };
            out.push(Element {
                value: format!("{}{kind}", heading.depth),
                line: Some(line_of(node)),
            });
        }
        for child in node.children().into_iter().flatten() {
            walk(child, out);
        }
    }
    let mut headings = Vec::new();
    if let Some(root) = root {
        walk(root, &mut headings);
    }
    let document = parse("", text, &mut Vec::new());
    let states = document
        .decisions
        .iter()
        .map(|decision| Element {
            value: decision.state.map_or("invalid", State::as_str).to_owned(),
            line: Some(decision.state_line),
        })
        .collect();
    let rows = document
        .decisions
        .iter()
        .flat_map(|decision| &decision.confirmations)
        .map(|row| Element {
            value: if row.by_reading {
                format!("{} (by reading)", row.id)
            } else {
                row.id.clone()
            },
            line: Some(row.line),
        })
        .collect();
    vec![
        ("heading", headings),
        ("state", states),
        ("confirmation", rows),
    ]
}

/// 用語集のファイルの名前。置き場のまとまりのフォルダの直下にあれば、話題の文書として読まない（C-core-docs-20）
const GLOSSARY: &str = "CONTEXT.md";

/// 置き場のまとまりのフォルダの直下の用語集か。置き場の直下と、まとまりの下のフォルダのものは違う
pub(crate) fn is_glossary(path: &str, place: &str) -> bool {
    crate::sources::relative_to_place(path, &crate::normalize_path(place))
        .and_then(|relative| relative.split_once('/'))
        .is_some_and(|(group, name)| !group.is_empty() && name == GLOSSARY)
}

/// リンクの行き先としてだけ読む文書。形を検査せず、見出しの行き先の名前だけを持つ。
/// links が真ならその文書の中のリンクも持ち、check_links がその行き先を確かめる
pub(crate) fn parse_target(path: &str, text: &str, links: bool) -> Document {
    let root = crate::markdown::parse(text).ok();
    Document {
        path: path.to_owned(),
        decisions: Vec::new(),
        open_items: 0,
        anchors: root
            .as_ref()
            .map(|root| heading_anchors(root).into_values().collect())
            .unwrap_or_default(),
        links: root
            .as_ref()
            .filter(|_| links)
            .map(crate::translations::markdown_links)
            .unwrap_or_default(),
    }
}

/// 文書のすべての見出しの、リンクの行き先になる名前を見出しの行から引く。
/// 同じ名前の2つ目からは "-1"、"-2" を付ける
fn heading_anchors(root: &Node) -> std::collections::BTreeMap<usize, String> {
    fn walk(node: &Node, out: &mut Vec<(usize, String)>) {
        if let Node::Heading(_) = node {
            out.push((line_of(node), github_anchor(&node.to_string())));
        }
        for child in node.children().into_iter().flatten() {
            walk(child, out);
        }
    }
    let mut headings = Vec::new();
    walk(root, &mut headings);
    let mut anchors = BTreeSet::new();
    let mut by_line = std::collections::BTreeMap::new();
    for (line, heading) in headings {
        let mut anchor = heading.clone();
        let mut suffix = 0;
        while !anchors.insert(anchor.clone()) {
            suffix += 1;
            anchor = format!("{heading}-{suffix}");
        }
        by_line.insert(line, anchor);
    }
    by_line
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
/// 置き場の外を指すリンク、Markdown の文書でないファイルを指すリンク、スキームで始まるリンクは見ない。
/// targets は話題の文書でない、行き先としてだけ読んだ文書
pub(crate) fn check_links(
    documents: &[Document],
    targets: &[Document],
    place: &str,
    findings: &mut Vec<Finding>,
) {
    let by_path: std::collections::BTreeMap<&str, &Document> = documents
        .iter()
        .chain(targets)
        .map(|doc| (doc.path.as_str(), doc))
        .collect();
    let inside = |path: &str| place.is_empty() || path.starts_with(&format!("{place}/"));
    for doc in documents.iter().chain(targets) {
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
fn read_decision_block(node: &Node, text: &str, lines: &[&str], building: &mut Building) {
    let awaiting_state = std::mem::replace(&mut building.awaiting_state, false);
    read_fingerprinted(node, text, building);
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "markdown::mdast::Node is a foreign enum: blocks other than headings, paragraphs and lists carry nothing a decision is read from"
    )]
    match node {
        Node::Heading(_) => {
            let section = Section::of(&node.to_string());
            building.sections.extend(section);
            building.section = section;
        }
        Node::Paragraph(_) if awaiting_state => {
            let text = node.to_string();
            let first = text.lines().next().unwrap_or_default();
            if let Some((keywords, value)) = state_line(first) {
                building.state_line = Some((line_of(node), keywords, value.to_owned()));
            }
        }
        Node::List(list) if building.section == Some(Section::Confirmations) => {
            for item in &list.children {
                building.confirmations.push(confirmation(item, lines));
            }
        }
        _ => {}
    }
}

/// 確認することの1行。ID は行の最初の語
fn confirmation(item: &Node, lines: &[&str]) -> Confirmation {
    let text = item.to_string();
    let text = text.trim_start();
    let (id, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
    let rest = rest.trim_start();
    let (by_reading, rest) = strip_by_reading(rest).map_or((false, rest), |after| (true, after));
    let line = line_of(item);
    let end = item.position().map_or(line, |position| position.end.line);
    let mut body: Vec<String> = lines
        .get(line.saturating_sub(1)..end.min(lines.len()))
        .unwrap_or_default()
        .iter()
        .map(|line| (*line).to_owned())
        .collect();
    // 一覧の最後の行の位置は、続く空行まで届くことがある
    while body.last().is_some_and(|line| line.trim().is_empty()) {
        body.pop();
    }
    Confirmation {
        id: id.to_owned(),
        line,
        by_reading,
        name: rest.split_whitespace().collect::<Vec<_>>().join(" "),
        body,
    }
}

/// 提案の決定と実装済みの決定を分けて数える
pub(crate) fn states(documents: &[Document]) -> DecisionStates {
    let mut states = DecisionStates::default();
    for decision in documents.iter().flat_map(|doc| &doc.decisions) {
        match decision.state {
            Some(State::Proposed) => states.proposed += 1,
            Some(State::Implemented) => states.implemented += 1,
            None => {}
        }
    }
    states
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
pub fn is_confirmation_id(id: &str) -> bool {
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

/// "kotowari list" の、形の正しい ID を持つ確認することの行の1件ずつ（docs/decisions/core/confirmations.md）
pub(crate) fn list_items(
    documents: &[Document],
    markers: &[crate::tests_discovery::TestMarker],
) -> Vec<crate::list::ListItem> {
    let mut items = decision_items(documents);
    items.extend(row_items(documents, markers));
    items
}

/// 確認することの行の1件ずつ
fn row_items(
    documents: &[Document],
    markers: &[crate::tests_discovery::TestMarker],
) -> Vec<crate::list::ListItem> {
    let tests_by_id = crate::list::tests_by_id(markers);
    // 形の誤った ID の行は check が invalid_id にしている。IR の ID の無い項目と同じく出さない
    rows(documents)
        .filter(|(_, _, row)| is_confirmation_id(&row.id))
        .map(|(doc, decision, row)| {
            crate::list::ListItem::Confirmation(crate::list::ConfirmationItem {
                id: row.id.clone(),
                kind: "confirmation",
                name: row.name.clone(),
                path: doc.path.clone(),
                line: row.line,
                decision: decision.heading.clone(),
                state: decision.state.map(State::as_str),
                by_reading: row.by_reading,
                tests: tests_by_id
                    .get(row.id.as_str())
                    .cloned()
                    .unwrap_or_default(),
            })
        })
        .collect()
}

/// "kotowari query" の、その ID を持つ確認することの行の1件ずつ（C-core-confirm-5）と、
/// その行き先の決定（本文は指紋に入れる行）。確認することの行と決定を ID で指す文は無いので、
/// 逆引きはいつも空
pub(crate) fn query_items(
    documents: &[Document],
    markers: &[crate::tests_discovery::TestMarker],
    id: &str,
) -> Vec<crate::query::QueryItem> {
    let decisions = documents.iter().flat_map(|doc| {
        doc.decisions
            .iter()
            .map(move |decision| (target(doc, decision), decision))
    });
    let mut items: Vec<crate::query::QueryItem> = decision_items(documents)
        .into_iter()
        .zip(decisions)
        .filter(|(_, (target, _))| target == id)
        .map(|(item, (_, decision))| crate::query::QueryItem {
            item,
            body: decision.fingerprinted.clone(),
            referenced_by: Vec::new(),
        })
        .collect();
    let bodies = rows(documents)
        .filter(|(_, _, row)| row.id == id)
        .map(|(_, _, row)| row.body.clone());
    items.extend(
        row_items(documents, markers)
            .into_iter()
            .filter(|item| item.id() == id)
            .zip(bodies)
            .map(|(item, body)| crate::query::QueryItem {
                item,
                body,
                referenced_by: Vec::new(),
            }),
    );
    items
}

/// 「決めたこと」と「確認すること」の節を、指紋の行の範囲と引用を探す文字に加える。
/// 節は "###" の見出しから、次の "###" か "##" の見出しまで。"####" より深い見出しは節の中にある
fn read_fingerprinted(node: &Node, text: &str, building: &mut Building) {
    let line = line_of(node);
    let end = node.position().map_or(line, |position| position.end.line);
    if let Node::Heading(heading) = node
        && heading.depth == 3
    {
        let name = node.to_string();
        building.in_fingerprinted = matches!(
            Section::of(&name),
            Some(Section::Decided | Section::Confirmations)
        );
        if building.in_fingerprinted {
            building.fingerprinted.push((line, end));
        }
        return;
    }
    if !building.in_fingerprinted {
        return;
    }
    if let Some(section) = building.fingerprinted.last_mut() {
        section.1 = end;
    }
    quotable_texts(node, text, &mut building.quotable);
}

/// ブロックの元の文字。表はセルごとに分け、コードブロックは入れない。一覧と引用の中の
/// ブロックも1つずつ同じく見る
fn quotable_texts(node: &Node, text: &str, out: &mut Vec<String>) {
    let source = |node: &Node| {
        node.position()
            .and_then(|position| text.get(position.start.offset..position.end.offset))
            .map(str::to_owned)
    };
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "markdown::mdast::Node is a foreign enum: every block other than code and the containers of blocks is read as written"
    )]
    match node {
        Node::Code(_) => {}
        Node::Table(_)
        | Node::TableRow(_)
        | Node::List(_)
        | Node::ListItem(_)
        | Node::Blockquote(_) => {
            for child in node.children().into_iter().flatten() {
                quotable_texts(child, text, out);
            }
        }
        Node::TableCell(cell) => {
            let span = cell
                .children
                .first()
                .zip(cell.children.last())
                .and_then(|(first, last)| {
                    Some(first.position()?.start.offset..last.position()?.end.offset)
                });
            if let Some(content) = span.and_then(|span| text.get(span)) {
                out.push(content.to_owned());
            }
        }
        _ => out.extend(source(node)),
    }
}

/// ガイドの印が決定を指す形（"<文書のパス>#<見出しの行き先の名前>"）か。パスは ".md" で終わり、
/// 名前は空でなく、どちらにも空白と "#" を含まない
pub fn is_decision_target(target: &str) -> bool {
    target.split_once('#').is_some_and(|(path, anchor)| {
        path.ends_with(".md")
            && !anchor.is_empty()
            && !anchor.contains('#')
            && !target.chars().any(char::is_whitespace)
    })
}

/// 決定を指すガイドの印の行き先（"<文書のパス>#<見出しの行き先の名前>"）
fn target(doc: &Document, decision: &Decision) -> String {
    format!("{}#{}", doc.path, decision.anchor)
}

/// 決定を指す行き先から、その決定の指紋を引く
pub(crate) fn fingerprints(documents: &[Document]) -> impl Iterator<Item = (String, String)> {
    documents.iter().flat_map(|doc| {
        doc.decisions
            .iter()
            .map(move |decision| (target(doc, decision), decision.fingerprint.clone()))
    })
}

/// どこかの決定の「決めたこと」か「確認すること」の、引用を探す文字
pub(crate) fn quotable(documents: &[Document]) -> impl Iterator<Item = &str> {
    documents
        .iter()
        .flat_map(|doc| &doc.decisions)
        .flat_map(|decision| decision.quotable.iter().map(String::as_str))
}

/// "kotowari list" の、決定の1件ずつ。ID はガイドの印が指す行き先
fn decision_items(documents: &[Document]) -> Vec<crate::list::ListItem> {
    documents
        .iter()
        .flat_map(|doc| {
            doc.decisions.iter().map(move |decision| {
                crate::list::ListItem::Decision(crate::list::DecisionItem {
                    id: target(doc, decision),
                    kind: "decision",
                    name: decision.heading.clone(),
                    path: doc.path.clone(),
                    line: decision.line,
                    state: decision.state.map(State::as_str),
                    fingerprint: decision.fingerprint.clone(),
                })
            })
        })
        .collect()
}
