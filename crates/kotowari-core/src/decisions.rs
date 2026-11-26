//! 決定の文書の読み取りと検査（docs/decisions/core/decision-documents.md、docs/decisions/core/confirmations.md）。
//!
//! 形は本体に埋め込んだスキーマ（`schemas/decision.yaml`）で mds のエンジンが読み分け、ここは
//! エンジンの抽出の値を決定ごとにまとめ、エンジンの指摘を kotowari の指摘に写す
//! （docs/decisions/core/form-contract.md）。エンジンの外に残すのは、見出しのアンカーとリンクの検査、
//! 対の骨組み、ガイドの印の指紋、面の名前の引用、確認することの ID とテストの印の照合である

use crate::{Finding, FindingKind, StopReason};
use kotowari_markdown_schema::finding::{FindingKind as EngineKind, RuleKind};
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
    pub(crate) fn as_str(self) -> &'static str {
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
    /// 決定の最後の行（次の "##" の見出しの前）
    pub(crate) end: usize,
    /// 見出しの行から最後の行までの、決定のすべての行。query の本文（C-core-query-14）
    pub(crate) whole: Vec<String>,
    /// 見出しへのリンクの行き先になる名前。同じ見出しの2つ目からは "-1"、"-2" が付いている
    pub(crate) anchor: String,
    /// 「決めたこと」と「確認すること」の節の、見出しの行から節の最後のブロックの行までの指紋（文書の順）。
    /// ガイドの印と照らす（C-core-lang-5、C-core-lang-6）
    pub(crate) fingerprint: String,
    /// 「決めたこと」と「確認すること」の中の、引用を探す文字。表はセルごと、コードブロックは入れない
    pub(crate) quotable: Vec<String>,
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
    /// 文書の行の数（status の "documents" の "lines"）
    pub(crate) line_count: usize,
}

impl Document {
    fn empty(path: &str, text: &str) -> Self {
        Self {
            path: path.to_owned(),
            line_count: crate::split_lines(text).len(),
            decisions: Vec::new(),
            open_items: 0,
            anchors: BTreeSet::new(),
            links: Vec::new(),
        }
    }
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

/// 確認することの行の数（status の "confirmations"、C-core-status-13）。決定の状態を問わず、形の
/// 正しい ID の行を数える。（読んで確認）の行は、テストの印があっても by_reading にだけ数える
#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct ConfirmationCounts {
    pub(crate) with_tests: usize,
    pub(crate) without_tests: usize,
    pub(crate) by_reading: usize,
}

impl ConfirmationCounts {
    readonly!(copy with_tests: usize, without_tests: usize, by_reading: usize);
}

/// 形の語の、1つの言語での綴り。どちらの言語の綴りも、どの文書でも同じ意味に読む。
/// 埋め込んだスキーマの名前の一覧と正規表現は、この表と同じ語を持つ（C-core-formc-4）
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
    /// テストを求めない行の頭の印。行を読み分けるのはスキーマの正規表現で、ここではそれと照らすだけ
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the row pattern of the embedded schema tells it apart; the parity test compares the schema with this spelling"
        )
    )]
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
    const ALL: [Self; 4] = [
        Self::Problem,
        Self::Decided,
        Self::Rejected,
        Self::Confirmations,
    ];

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
        Self::ALL.into_iter().find(|section| {
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

/// 状態の値を状態にする。どれかの言語の「提案」か「実装済み」でなければ None
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

/// エンジンが状態の行として返した行の、書いた綴りの組と頭のあとの値。頭が無ければ None
fn state_value(raw: &str) -> Option<(&'static Keywords, &str)> {
    let raw = raw.trim();
    KEYWORDS.iter().find_map(|keywords| {
        raw.strip_prefix(keywords.state)
            .map(|value| (*keywords, value.trim()))
    })
}

/// エンジンの抽出の値（`schemas/decision.yaml` の extract）。知らない鍵があれば、スキーマと
/// 読み手が食い違っている
#[derive(Debug, Default, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Values {
    title: Option<Title>,
    #[serde(default)]
    decisions: Vec<Span>,
    #[serde(default)]
    states: Vec<StateLine>,
    #[serde(default)]
    decided: Vec<Span>,
    #[serde(default)]
    confirmations: Vec<Confirmations>,
    #[serde(default)]
    open_items: Vec<Line>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Title {
    text: String,
    line: usize,
}

/// 見出しで始まる要素と、その最後の行
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Span {
    #[serde(default)]
    name: String,
    line: usize,
    end: usize,
}

/// 状態の行の場所の文の1行
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct StateLine {
    line: usize,
    raw: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Confirmations {
    #[serde(default)]
    rows: Vec<Row>,
    line: usize,
    end: usize,
}

/// 確認することの1行。行の形が違えば、キャプチャの鍵が無い
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Row {
    id: Option<String>,
    by_reading: Option<String>,
    name: Option<String>,
    line: usize,
    end: usize,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Line {
    #[expect(
        dead_code,
        reason = "only the number of open items is used; the line keeps the element an object as the schema extracts it"
    )]
    line: usize,
}

/// エンジンで読んだ文書
struct Reading {
    document: kotowari_markdown_schema::Document,
    values: Values,
    findings: Vec<EngineFinding>,
}

impl Reading {
    /// 1始まりの行番号の生の行
    fn raw(&self, line: usize) -> &str {
        self.document.raw_line(line).unwrap_or_default()
    }

    /// line から end までの生の行。後ろの空行は除く
    fn lines(&self, line: usize, end: usize) -> Vec<String> {
        let mut lines: Vec<String> = (line..=end)
            .map_while(|line| self.document.raw_line(line))
            .map(str::to_owned)
            .collect();
        while lines.last().is_some_and(|line| line.trim().is_empty()) {
            lines.pop();
        }
        lines
    }
}

/// エンジンの指摘のうち、写すのに要るもの。エンジンの指摘は crate の外で作れないので、
/// 写しはこの形で受ける
#[derive(Debug, Clone)]
struct EngineFinding {
    kind: EngineKind,
    line: Option<usize>,
    node: Option<String>,
    raw: Option<String>,
    rule_kind: Option<RuleKind>,
}

impl From<&kotowari_markdown_schema::finding::Finding> for EngineFinding {
    fn from(value: &kotowari_markdown_schema::finding::Finding) -> Self {
        Self {
            kind: value.kind(),
            line: value.line(),
            node: value.node().map(str::to_owned),
            raw: value.raw().map(str::to_owned),
            rule_kind: value.rule_kind(),
        }
    }
}

/// 埋め込んだスキーマで文書を読み、抽出の値と指摘を返す
fn read_with_engine(path: &str, text: &str) -> Result<Reading, StopReason> {
    let schema = crate::schema::decision_schema()
        .map_err(|error| StopReason::MappingError(format!("{path}: schema: {}", error.0)))?;
    let document = kotowari_markdown_schema::Document::parse(text)
        .map_err(|error| StopReason::MappingError(format!("{path}: document: {error}")))?;
    let extraction = kotowari_markdown_schema::extract_partial(
        schema,
        &document,
        kotowari_markdown_schema::ValidationOptions::default(),
    );
    let values = serde_json::from_value(extraction.values().clone())
        .map_err(|error| StopReason::MappingError(format!("{path}: values: {error}")))?;
    let findings = extraction
        .findings()
        .iter()
        .map(EngineFinding::from)
        .collect();
    Ok(Reading {
        document,
        values,
        findings,
    })
}

/// 1つの決定に属する抽出の値
struct Grouped<'a> {
    span: &'a Span,
    states: Vec<&'a StateLine>,
    decided: Vec<&'a Span>,
    confirmations: Vec<&'a Confirmations>,
}

impl Grouped<'_> {
    /// 状態の行の場所の最初の行
    fn state_line(&self) -> Option<&StateLine> {
        self.states.first().copied()
    }

    /// 状態の行を書いた綴りの組。状態の行が無ければ None
    fn keywords(&self) -> Option<&'static Keywords> {
        self.state_line()
            .and_then(|state| state_value(&state.raw))
            .map(|(keywords, _)| keywords)
    }

    fn state(&self) -> Option<State> {
        self.state_line()
            .and_then(|state| state_value(&state.raw))
            .and_then(|(_, value)| state_of(value))
    }
}

/// 抽出の値を、決定の見出しの行の範囲で決定ごとにまとめる
fn group(values: &Values) -> Vec<Grouped<'_>> {
    let inside = |span: &Span, line: usize| (span.line..=span.end).contains(&line);
    values
        .decisions
        .iter()
        .map(|span| Grouped {
            span,
            states: values
                .states
                .iter()
                .filter(|state| inside(span, state.line))
                .collect(),
            decided: values
                .decided
                .iter()
                .filter(|decided| inside(span, decided.line))
                .collect(),
            confirmations: values
                .confirmations
                .iter()
                .filter(|confirmations| inside(span, confirmations.line))
                .collect(),
        })
        .collect()
}

/// 写した指摘と、それをほかの言語の側にも出すか
struct Mapped {
    finding: Finding,
    /// 読み飛ばされる中身を作る誤り。ほかの言語の側は骨組みでしか比べないので、その側にも出す
    every_side: bool,
}

/// エンジンの指摘を kotowari の指摘に写す。写し先の無い種類は停止する（C-core-formc-3）
fn map_findings(
    path: &str,
    reading: &Reading,
    grouped: &[Grouped<'_>],
) -> Result<Vec<Mapped>, StopReason> {
    // 閉じないコードブロックは unclosed_code_block だけにする。置き場所の無いブロックとしても数えない
    let unclosed: BTreeSet<usize> = reading
        .findings
        .iter()
        .filter(|engine| engine.kind == EngineKind::UnclosedCodeblock)
        .filter_map(|engine| engine.line)
        .collect();
    let mut mapped = Vec::new();
    for engine in &reading.findings {
        if engine.kind == EngineKind::UndeclaredLine
            && engine.line.is_some_and(|line| unclosed.contains(&line))
        {
            continue;
        }
        if let Some(finding) = map_one(path, reading, grouped, engine)? {
            mapped.push(finding);
        }
    }
    for decision in grouped {
        for state in misplaced_state_lines(decision) {
            mapped.push(Mapped {
                finding: Finding::new(
                    FindingKind::UnknownLine,
                    path.to_owned(),
                    Some(state.line),
                    state.raw.clone(),
                ),
                every_side: true,
            });
        }
    }
    Ok(mapped)
}

/// 状態の行の場所の、置き場所の無い行。2つ目からの段落はその最初の行を、状態の行の段落の
/// 2行目からはその2行目を返す。エンジンは2つ目の段落を1度だけ数えて指摘し、段落の中の行は
/// 指摘しないので、抽出した行で見る
fn misplaced_state_lines<'a>(decision: &Grouped<'a>) -> Vec<&'a StateLine> {
    let mut misplaced = Vec::new();
    let mut previous: Option<&StateLine> = None;
    for state in &decision.states {
        let continues = previous.is_some_and(|previous| previous.line + 1 == state.line);
        match previous {
            // 2つ目からの段落の最初の行
            Some(_) if !continues => misplaced.push(*state),
            // 状態の行の段落の2行目。頭のある状態の行に続くときだけ
            Some(first)
                if first.line == decision.states[0].line && state_value(&first.raw).is_some() =>
            {
                misplaced.push(*state);
            }
            Some(_) | None => {}
        }
        previous = Some(state);
    }
    misplaced
}

/// 1件のエンジンの指摘の写し先。捨てる指摘は None
fn map_one(
    path: &str,
    reading: &Reading,
    grouped: &[Grouped<'_>],
    engine: &EngineFinding,
) -> Result<Option<Mapped>, StopReason> {
    let at = |kind: FindingKind, line: Option<usize>, detail: String, every_side: bool| {
        Some(Mapped {
            finding: Finding::new(kind, path.to_owned(), line, detail),
            every_side,
        })
    };
    let raw = || {
        engine
            .raw
            .clone()
            .or_else(|| engine.line.map(|line| reading.raw(line).to_owned()))
            .unwrap_or_default()
    };
    let decision = || {
        engine.line.and_then(|line| {
            grouped
                .iter()
                .find(|decision| (decision.span.line..=decision.span.end).contains(&line))
        })
    };
    // エンジンの指摘の種類は crate の外の non_exhaustive な列挙。埋め込んだスキーマが出さない種類は、
    // 写し先の無い種類として停止する
    let mapped = match (engine.kind, engine.rule_kind) {
        // detail は古い形と同じく、ディレクトリを除いたファイルの名前（C-core-findings-10）
        (EngineKind::MissingTitle, _) => {
            let name = path.rsplit_once('/').map_or(path, |(_, name)| name);
            at(FindingKind::MissingTitle, None, name.to_owned(), false)
        }
        // 古い形と同じく、line は持たず、detail はその行から "#" を除き前後の空白を除いた文字
        (EngineKind::MultipleTitles, _) => {
            let raw = raw();
            let title = raw.strip_prefix('#').unwrap_or(&raw).trim().to_owned();
            at(FindingKind::MultipleTitles, None, title, true)
        }
        (EngineKind::UnclosedCodeblock, _) => {
            at(FindingKind::UnclosedCodeBlock, engine.line, raw(), true)
        }
        (EngineKind::UndeclaredHeading | EngineKind::HeadingLevelMismatch, _) => {
            at(FindingKind::UnknownHeading, engine.line, raw(), true)
        }
        (EngineKind::UndeclaredLine, _) => at(FindingKind::UnknownLine, engine.line, raw(), true),
        // 状態の行の場所の2つ目からの段落。段落ごとの unknown_line を抽出した行から出している
        (EngineKind::RepeatMaxExceeded, Some(RuleKind::Statement)) if decision().is_some() => None,
        // 話題の段落が無い。題名も無ければ missing_title だけにする（題名の値と missing_title は、
        // エンジンがどちらも文書の最初の "#" の見出しから作る）
        (EngineKind::RepeatMinNotMet, Some(RuleKind::Statement)) if engine.line.is_none() => {
            reading.values.title.as_ref().and_then(|title| {
                at(
                    FindingKind::TopicLeadMissing,
                    Some(title.line),
                    title.text.clone(),
                    false,
                )
            })
        }
        (EngineKind::RepeatMinNotMet, Some(RuleKind::Statement)) => at(
            FindingKind::DecisionStateInvalid,
            engine.line,
            String::new(),
            false,
        ),
        // 欠けた節の名前は、状態の行を書いた言語の綴りで出す
        (EngineKind::RepeatMinNotMet, Some(RuleKind::Item)) => {
            let Some(section) = engine.node.as_deref().and_then(Section::of) else {
                return Err(no_row(engine));
            };
            let keywords = decision()
                .and_then(Grouped::keywords)
                .unwrap_or(KEYWORDS[0]);
            at(
                FindingKind::DecisionSectionMissing,
                engine.line,
                section.spelled(keywords).to_owned(),
                false,
            )
        }
        (EngineKind::StatementPatternMismatch, _) => {
            let Some(decision) = decision() else {
                return Err(no_row(engine));
            };
            let first = decision.state_line();
            if first.is_none_or(|first| Some(first.line) != engine.line) {
                // 状態の行の場所の2つ目からの文は、抽出した行から unknown_line にしている
                return Ok(None);
            }
            match state_value(&raw()) {
                // 1行の状態の行のあとに行が続く。2行目を抽出した行から unknown_line にしている
                Some((_, value)) if state_of(value).is_some() => None,
                Some((_, value)) => at(
                    FindingKind::DecisionStateInvalid,
                    engine.line,
                    value.to_owned(),
                    false,
                ),
                None => at(
                    FindingKind::DecisionStateInvalid,
                    Some(decision.span.line),
                    String::new(),
                    false,
                ),
            }
        }
        // 確認することの行の ID の形が違う。detail は行の最初の語
        (EngineKind::BulletPatternMismatch, _) => at(
            FindingKind::InvalidId,
            engine.line,
            first_word(&raw()),
            false,
        ),
        _ => return Err(no_row(engine)),
    };
    Ok(mapped)
}

/// 写し先の無いエンジンの指摘で停止する
fn no_row(engine: &EngineFinding) -> StopReason {
    let what = match (&engine.node, engine.rule_kind) {
        (Some(node), _) => format!(" {node}"),
        (None, Some(rule)) => format!(" {}", rule.as_str()),
        (None, None) => String::new(),
    };
    StopReason::MappingError(format!(
        "no mapping for decision document finding {}{what}",
        engine.kind.as_str()
    ))
}

/// 一覧の行の、印のあとの最初の語。形の違う ID の行の、指摘の detail と骨組みの要素に使う
fn first_word(raw: &str) -> String {
    let item = raw.trim_start();
    let item = item
        .strip_prefix(['-', '*', '+'])
        .map_or(item, str::trim_start);
    item.split_whitespace()
        .next()
        .unwrap_or_default()
        .to_owned()
}

/// 行の名前の Markdown の書式を除いた文字。前と同じく、書式の無い文字を1つの空白でつなぐ。
/// 名前は行の中で ID のあとにあるので、"# " や "> " で始まっても見出しや引用に読まないよう、
/// 語を1つ前に置いて段落の中で読む
fn plain(text: &str) -> String {
    const LEAD: &str = "x ";
    let source = format!("{LEAD}{text}");
    let plain =
        crate::markdown::parse(&source).map_or_else(|_| source.clone(), |root| root.to_string());
    let plain = plain.strip_prefix(LEAD).unwrap_or(&plain);
    plain.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// 確認することの行を、抽出の値から作る
fn confirmation(reading: &Reading, row: &Row) -> Confirmation {
    Confirmation {
        id: row
            .id
            .clone()
            .unwrap_or_else(|| first_word(reading.raw(row.line))),
        line: row.line,
        by_reading: row.by_reading.is_some(),
        name: row.name.as_deref().map(plain).unwrap_or_default(),
        body: reading.lines(row.line, row.end),
    }
}

/// 抽出の値から決定を作る。anchored は見出しの行からリンクの行き先の名前、blocks は文書の直下のブロック
fn decision(
    reading: &Reading,
    grouped: &Grouped<'_>,
    anchored: &std::collections::BTreeMap<usize, String>,
    text: &str,
    blocks: &[Node],
) -> Decision {
    // 指紋に入れるのは「決めたこと」と「確認すること」の節で、文書の順
    let mut sections: Vec<(usize, usize)> = grouped
        .decided
        .iter()
        .map(|span| (span.line, span.end))
        .chain(
            grouped
                .confirmations
                .iter()
                .map(|confirmations| (confirmations.line, confirmations.end)),
        )
        .collect();
    sections.sort_unstable();
    let fingerprinted: Vec<String> = sections
        .iter()
        .flat_map(|&(line, end)| reading.lines(line, end))
        .collect();
    let mut quotable = Vec::new();
    for node in blocks {
        let line = line_of(node);
        if sections
            .iter()
            .any(|&(heading, end)| heading < line && line <= end)
        {
            quotable_texts(node, text, &mut quotable);
        }
    }
    Decision {
        heading: grouped.span.name.trim().to_owned(),
        line: grouped.span.line,
        end: grouped.span.end,
        whole: reading.lines(grouped.span.line, grouped.span.end),
        anchor: anchored
            .get(&grouped.span.line)
            .cloned()
            .unwrap_or_default(),
        fingerprint: crate::fingerprint::digest(&fingerprinted),
        quotable,
        state: grouped.state(),
        confirmations: grouped
            .confirmations
            .iter()
            .flat_map(|confirmations| &confirmations.rows)
            .map(|row| confirmation(reading, row))
            .collect(),
    }
}

fn line_of(node: &Node) -> usize {
    node.position().map_or(1, |position| position.start.line)
}

/// 決定の「決めたこと」と「確認すること」の節の、見出しの行のあとの行の曖昧語を指摘する
/// （C-core-docs-18、C-core-pairs-17）。数え方は古い形と同じで、コードブロックの中の行と、二重引用符の
/// 中（語を挙げているだけで、使ってはいない）は見ない
fn check_vague_words(
    path: &str,
    reading: &Reading,
    grouped: &[Grouped<'_>],
    root: &Node,
    vague_words: &[String],
    findings: &mut Vec<Finding>,
) {
    fn code_lines(node: &Node, out: &mut BTreeSet<usize>) {
        if let (Node::Code(_), Some(position)) = (node, node.position()) {
            out.extend(position.start.line..=position.end.line);
        }
        for child in node.children().into_iter().flatten() {
            code_lines(child, out);
        }
    }
    let mut code = BTreeSet::new();
    code_lines(root, &mut code);
    let sections = grouped.iter().flat_map(|decision| {
        decision
            .decided
            .iter()
            .map(|span| (span.line, span.end))
            .chain(
                decision
                    .confirmations
                    .iter()
                    .map(|confirmations| (confirmations.line, confirmations.end)),
            )
    });
    for (heading, end) in sections {
        for line in (heading + 1..=end).filter(|line| !code.contains(line)) {
            for part in crate::split_outside_quotes(reading.raw(line)) {
                vague_words_in_line(part, line, vague_words, path, findings);
            }
        }
    }
}

/// 話題の文書を読み、形の誤りと曖昧語を findings に積む
pub(crate) fn parse(
    path: &str,
    text: &str,
    vague_words: &[String],
    findings: &mut Vec<Finding>,
) -> Result<Document, StopReason> {
    let root = match crate::markdown::parse(text) {
        Ok(root) => root,
        Err(detail) => {
            findings.push(Finding::new(
                FindingKind::UnparsableFile,
                path.to_owned(),
                None,
                detail,
            ));
            return Ok(Document::empty(path, text));
        }
    };
    let reading = read_with_engine(path, text)?;
    let grouped = group(&reading.values);
    findings.extend(
        map_findings(path, &reading, &grouped)?
            .into_iter()
            .map(|mapped| mapped.finding),
    );
    check_vague_words(path, &reading, &grouped, &root, vague_words, findings);
    let anchored = heading_anchors(&root);
    let blocks = root.children().map(Vec::as_slice).unwrap_or_default();
    let decisions = grouped
        .iter()
        .map(|grouped| decision(&reading, grouped, &anchored, text, blocks))
        .collect();
    Ok(Document {
        path: path.to_owned(),
        line_count: crate::split_lines(text).len(),
        decisions,
        open_items: reading.values.open_items.len(),
        anchors: anchored.into_values().collect(),
        links: crate::translations::markdown_links(&root),
    })
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
    // 骨組みは指摘を出さないので、写しは使わず抽出の値だけを見る。スキーマは本体に固定なので
    // 読めないことは無く、そのときは状態と行を空にする
    let reading = read_with_engine("", text).ok();
    let grouped = reading
        .as_ref()
        .map(|reading| group(&reading.values))
        .unwrap_or_default();
    let states = grouped
        .iter()
        .map(|decision| Element {
            value: decision.state().map_or("invalid", State::as_str).to_owned(),
            line: Some(
                decision
                    .state_line()
                    .map_or(decision.span.line, |state| state.line),
            ),
        })
        .collect();
    let rows = reading
        .as_ref()
        .into_iter()
        .flat_map(|reading| {
            grouped
                .iter()
                .flat_map(|decision| &decision.confirmations)
                .flat_map(|confirmations| &confirmations.rows)
                .map(move |row| {
                    let row = confirmation(reading, row);
                    Element {
                        value: if row.by_reading {
                            format!("{} (by reading)", row.id)
                        } else {
                            row.id
                        },
                        line: Some(row.line),
                    }
                })
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
    relative_to_place(path, &crate::normalize_path(place))
        .and_then(|relative| relative.split_once('/'))
        .is_some_and(|(group, name)| !group.is_empty() && name == GLOSSARY)
}

/// リンクの行き先としてだけ読む文書。決定の形を検査せず、見出しの行き先の名前だけを持つ。
/// content があれば（ほかの言語の側）その文書の中のリンクも持ち、check_links がその行き先を確かめる。
/// そのときは、読み飛ばされる中身を作る誤りだけを content に積む。決定の形のほかの
/// 誤りは、骨組みの比較が先頭の言語の側との違いとして出す。曖昧語は、その側の文で検査する
pub(crate) fn parse_target(
    path: &str,
    text: &str,
    content: Option<(&mut Vec<Finding>, &[String])>,
) -> Result<Document, StopReason> {
    let root = crate::markdown::parse(text).ok();
    let links = content.is_some();
    if let (Some((findings, vague_words)), Some(root)) = (content, &root) {
        let reading = read_with_engine(path, text)?;
        let grouped = group(&reading.values);
        findings.extend(
            map_findings(path, &reading, &grouped)?
                .into_iter()
                .filter(|mapped| mapped.every_side)
                .map(|mapped| mapped.finding),
        );
        check_vague_words(path, &reading, &grouped, root, vague_words, findings);
    }
    Ok(Document {
        path: path.to_owned(),
        line_count: crate::split_lines(text).len(),
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
    })
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

/// 凍結した木。切り替える前の古い形の文書を置き、check は読まない（C-core-docs-14）
const FROZEN: &str = "docs/frozen";

/// `基準のディレクトリ`からの相対パスが、凍結した木かその下か
pub fn is_frozen(path: &str) -> bool {
    path == FROZEN || path.starts_with(&format!("{FROZEN}/"))
}

/// リンクの行き先の文書のパス（`基準のディレクトリ`からの相対パス）と、"#" のあとの見出しの名前
/// （"%XX" を戻したもの）。スキームで始まるリンクは None。doc は書いてある文書のパス
fn link_target(doc: &str, destination: &str) -> Option<(String, Option<String>)> {
    if has_scheme(destination) {
        return None;
    }
    let (target, anchor) = destination
        .split_once('#')
        .map_or((destination, None), |(path, anchor)| (path, Some(anchor)));
    let target = if target.is_empty() {
        doc.to_owned()
    } else {
        let directory = doc.rsplit_once('/').map_or("", |(dir, _)| dir);
        crate::normalize_path(&crate::join_display_path(directory, target))
    };
    Some((target, anchor.map(percent_decode)))
}

/// 決定の見出しへのリンクを持つ、ほかの決定（C-core-query-15、C-core-query-16）。1つの決定が何度
/// 指しても1件にし、path の昇順、同じ path では見出しの行の昇順に並べる
fn referenced_by(
    documents: &[Document],
    doc: &Document,
    decision: &Decision,
) -> Vec<crate::query::Reference> {
    let mut references: Vec<crate::query::Reference> = documents
        .iter()
        .flat_map(|from| {
            from.decisions.iter().filter_map(move |linking| {
                let links = from.links.iter().filter(|link| {
                    (linking.line..=linking.end).contains(&link.line)
                        && link_target(&from.path, &link.destination).is_some_and(
                            |(path, anchor)| {
                                path == doc.path && anchor.as_deref() == Some(&decision.anchor)
                            },
                        )
                });
                let is_self = from.path == doc.path && linking.line == decision.line;
                (!is_self && links.count() > 0).then(|| crate::query::Reference {
                    id: target(from, linking),
                    kind: "decision",
                    path: from.path.clone(),
                    line: linking.line,
                    via: "link",
                })
            })
        })
        .collect();
    references.sort_by(|a, b| (a.path.as_bytes(), a.line).cmp(&(b.path.as_bytes(), b.line)));
    references
}

/// 決定の文書の中のリンクのうち、置き場の中を指すものが、無い文書か無い見出しを指していれば指摘する。
/// 置き場の外を指すリンク、Markdown の文書でないファイルを指すリンク、スキームで始まるリンクは見ない。
/// 凍結した木の下を指すリンクは、在るかを問わず link_to_record にする（C-core-pairstruct-17）。
/// targets は話題の文書でない、行き先としてだけ読んだ文書（ほかの言語の側と用語集）
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
        for link in &doc.links {
            let Some((target, anchor)) = link_target(&doc.path, &link.destination) else {
                continue;
            };
            if is_frozen(&target) {
                findings.push(Finding::new(
                    FindingKind::LinkToRecord,
                    doc.path.clone(),
                    Some(link.line),
                    link.destination.clone(),
                ));
                continue;
            }
            // 決定の文書でないファイル（図など）は core が読まないので、在るかを判断できない
            if !inside(&target) || !target.ends_with(".md") {
                continue;
            }
            let found = by_path.get(target.as_str()).is_some_and(|target| {
                anchor
                    .as_ref()
                    .is_none_or(|anchor| target.anchors.contains(anchor))
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

/// 確認することの行を、テストのあるもの、無いもの、読んで確認するものに分けて数える
pub(crate) fn confirmation_counts(
    documents: &[Document],
    markers: &[crate::tests_discovery::TestMarker],
) -> ConfirmationCounts {
    let marked: BTreeSet<&str> = markers.iter().map(|marker| marker.id.as_str()).collect();
    let mut counts = ConfirmationCounts::default();
    for (_, _, row) in rows(documents).filter(|(_, _, row)| is_confirmation_id(&row.id)) {
        if row.by_reading {
            counts.by_reading += 1;
        } else if marked.contains(row.id.as_str()) {
            counts.with_tests += 1;
        } else {
            counts.without_tests += 1;
        }
    }
    counts
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

/// 確認することの ID の形（`C-<まとまりと話題の短い名前>-<連番>`）。名前は小文字の英字で始まり小文字の英数字と "-" が続く形で、
/// 連番は 1 から始まり先頭の 0 を認めない
pub fn is_confirmation_id(id: &str) -> bool {
    let Some((name, number)) = id.strip_prefix("C-").and_then(|rest| rest.rsplit_once('-')) else {
        return false;
    };
    is_valid_id_name(name)
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

/// 同じ ID の2つ目以降の行を指摘する。documents はパスの順。形の誤った ID は、エンジンの指摘を
/// 写して invalid_id にしている
pub(crate) fn check_ids(documents: &[Document], findings: &mut Vec<Finding>) {
    let mut seen = BTreeSet::new();
    for (doc, _, row) in rows(documents) {
        if is_confirmation_id(&row.id) && !seen.insert(row.id.as_str()) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                doc.path.clone(),
                Some(row.line),
                row.id.clone(),
            ));
        }
    }
}

/// まとまりのフォルダの下の文書の、まとまりの名前がそのフォルダの名前と違う確認することの ID を
/// 指摘する（C-core-findings-11、C-core-findings-12）。まとまりはパスの置き場のあとの最初の要素で、
/// 置き場の直下の文書は見ない。ID のまとまりは "C-" のあとの最初の語。形の誤った ID は見ない
pub(crate) fn check_domains(documents: &[Document], place: &str, findings: &mut Vec<Finding>) {
    let place = crate::normalize_path(place);
    for (doc, _, row) in rows(documents) {
        let Some((folder, _)) =
            relative_to_place(&doc.path, &place).and_then(|relative| relative.split_once('/'))
        else {
            continue;
        };
        let group = row
            .id
            .strip_prefix("C-")
            .and_then(|rest| rest.split('-').next());
        if is_confirmation_id(&row.id) && group != Some(folder) {
            findings.push(Finding::new(
                FindingKind::IdDomainMismatch,
                doc.path.clone(),
                Some(row.line),
                row.id.clone(),
            ));
        }
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
    // 形の誤った ID の行は check が invalid_id にしているので、一覧には出さない
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
/// その行き先の決定（本文は決定のすべての行、逆引きはその見出しへリンクする決定）。確認することの
/// 行を ID で指す文は無いので、行の逆引きはいつも空
pub(crate) fn query_items(
    documents: &[Document],
    markers: &[crate::tests_discovery::TestMarker],
    id: &str,
) -> Vec<crate::query::QueryItem> {
    let decisions = documents.iter().flat_map(|doc| {
        doc.decisions
            .iter()
            .map(move |decision| (target(doc, decision), doc, decision))
    });
    let mut items: Vec<crate::query::QueryItem> = decision_items(documents)
        .into_iter()
        .zip(decisions)
        .filter(|(_, (target, _, _))| target == id)
        .map(|(item, (_, doc, decision))| crate::query::QueryItem {
            item,
            body: decision.whole.clone(),
            referenced_by: referenced_by(documents, doc, decision),
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

/// 置き場（正規化したパス）からの path の相対パス。置き場の外なら None、置き場が空なら path のまま
pub(crate) fn relative_to_place<'a>(path: &'a str, place: &str) -> Option<&'a str> {
    if place.is_empty() {
        return Some(path);
    }
    path.strip_prefix(place)?.strip_prefix('/')
}

/// ID の名前の形（小文字の英字で始まり、2文字目からは小文字の英数字と "-"）
fn is_valid_id_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// 1行の曖昧語の検査。行の左から最長一致で重ならない形で、出現ごとに1件を数える
fn vague_words_in_line(
    text: &str,
    line: usize,
    vague_words: &[String],
    path: &str,
    findings: &mut Vec<Finding>,
) {
    if vague_words.is_empty() {
        return;
    }
    let mut remaining = text;
    while let Some(first) = remaining.chars().next() {
        // 現在位置から始まる最長の曖昧語を探す
        let best_word = vague_words
            .iter()
            .filter(|w| !w.is_empty() && remaining.starts_with(w.as_str()))
            .max_by_key(|w| w.chars().count());
        if let Some(word) = best_word {
            findings.push(Finding::new(
                FindingKind::VagueWord,
                path.to_string(),
                Some(line),
                word.clone(),
            ));
            remaining = &remaining[word.len()..];
        } else {
            remaining = &remaining[first.len_utf8()..];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 埋め込んだスキーマを、検査を通さずに値として読む
    fn schema_value() -> serde_json::Value {
        serde_saphyr::from_str(crate::schema::DECISION).unwrap()
    }

    /// 正規表現の中で文字どおりに読ませる
    fn escaped(text: &str) -> String {
        text.replace('(', "\\(").replace(')', "\\)")
    }

    // @kotowari[C-core-formc-4]
    #[test]
    fn the_embedded_schema_loads_as_an_mds_schema() {
        assert_eq!(
            crate::schema::decision_schema().unwrap().name(),
            Some("decision-document")
        );
    }

    // @kotowari[C-core-formc-4]
    #[test]
    fn the_form_words_of_the_embedded_schema_equal_the_keyword_table() {
        let schema = schema_value();
        let sections = &schema["document"]["sections"];
        let names = |keywords: fn(&Keywords) -> &'static str| {
            serde_json::json!(KEYWORDS.iter().map(|k| keywords(k)).collect::<Vec<_>>())
        };
        assert_eq!(sections[0]["name"], names(|k| k.open));
        let items = sections[1]["items"].as_array().unwrap();
        assert_eq!(items.len(), Section::ALL.len());
        for (item, section) in items.iter().zip(Section::ALL) {
            assert_eq!(
                item["name"],
                serde_json::json!(
                    KEYWORDS
                        .iter()
                        .map(|k| section.spelled(k))
                        .collect::<Vec<_>>()
                ),
                "{section:?}"
            );
        }
        let heads: Vec<&str> = KEYWORDS
            .iter()
            .map(|k| k.state.strip_suffix(':').unwrap())
            .collect();
        let values: Vec<&str> = KEYWORDS
            .iter()
            .flat_map(|k| [k.proposed, k.implemented])
            .collect();
        assert_eq!(
            sections[1]["statement"]["pattern"],
            format!(
                "^(?:{}):[ \\t]*(?:{})[ \\t]*$",
                heads.join("|"),
                values.join("|")
            )
        );
        let rows = items[3]["bullets"]["pattern"].as_str().unwrap();
        let reading: Vec<String> = KEYWORDS.iter().map(|k| escaped(k.by_reading)).collect();
        assert!(
            rows.contains(&format!("(?P<reading>{})", reading.join("|"))),
            "{rows}"
        );
    }

    // @kotowari[C-core-formc-4]
    #[test]
    fn the_row_pattern_of_the_embedded_schema_accepts_the_ids_is_confirmation_id_accepts() {
        let ids = [
            "C-a-1",
            "C-core-docs-12",
            "C-a--1",
            "C-a1-b2-30",
            "C-a-01",
            "C-a-0",
            "C-A-1",
            "C-1-1",
            "c-a-1",
            "C-a-1:",
            "C-a",
            "C--1",
        ];
        let rows: String = ids.iter().map(|id| format!("- {id} 文\n")).collect();
        let text = format!(
            "# 題\n\n段落。\n\n## 決定\n\n状態: 提案\n\n### 問題\n\n問題。\n\n### 決めたこと\n\n決めた。\n\n\
             ### 確認すること\n\n{rows}"
        );
        let reading = read_with_engine("x.md", &text).unwrap();
        let read: Vec<(String, bool)> = reading.values.confirmations[0]
            .rows
            .iter()
            .map(|row| (first_word(reading.raw(row.line)), row.id.is_some()))
            .collect();
        let expected: Vec<(String, bool)> = ids
            .iter()
            .map(|id| ((*id).to_owned(), is_confirmation_id(id)))
            .collect();
        assert_eq!(read, expected);
    }

    // @kotowari[C-core-formc-3]
    #[test]
    fn an_engine_finding_without_a_mapping_stops_with_a_mapping_error() {
        let reading = read_with_engine("x.md", "# 題\n\n段落。\n").unwrap();
        let grouped = group(&reading.values);
        let engine = EngineFinding {
            kind: EngineKind::FieldEnumInvalid,
            line: Some(3),
            node: Some("状態".to_owned()),
            raw: None,
            rule_kind: None,
        };
        let stop = map_one("x.md", &reading, &grouped, &engine).err();
        assert_eq!(
            stop.map(|stop| stop.to_string()),
            Some(
                "mapping error: no mapping for decision document finding field_enum_invalid 状態"
                    .to_owned()
            )
        );
    }

    // @kotowari[C-core-formc-3]
    #[test]
    fn too_many_statements_outside_any_decision_stop_with_a_mapping_error() {
        let reading = read_with_engine("x.md", "# 題\n\n段落。\n").unwrap();
        let grouped = group(&reading.values);
        let engine = EngineFinding {
            kind: EngineKind::RepeatMaxExceeded,
            line: Some(3),
            node: None,
            raw: None,
            rule_kind: Some(RuleKind::Statement),
        };
        let stop = map_one("x.md", &reading, &grouped, &engine)
            .err()
            .map(|stop| stop.to_string())
            .unwrap_or_default();
        assert!(stop.starts_with("mapping error: "), "{stop}");
    }

    // @kotowari[C-core-clienv-13]
    #[test]
    fn a_node_name_that_cannot_be_mapped_to_a_section_stops_with_the_name_in_the_details() {
        let reading = read_with_engine("x.md", "# 題\n\n段落。\n").unwrap();
        let grouped = group(&reading.values);
        let engine = EngineFinding {
            kind: EngineKind::RepeatMinNotMet,
            line: Some(3),
            node: Some("未知の節".to_owned()),
            raw: None,
            rule_kind: Some(RuleKind::Item),
        };
        let stop = map_one("x.md", &reading, &grouped, &engine)
            .err()
            .map(|stop| stop.to_string())
            .unwrap_or_default();
        assert!(stop.starts_with("mapping error: "), "{stop}");
        assert!(stop.contains("未知の節"), "{stop}");
    }
}
