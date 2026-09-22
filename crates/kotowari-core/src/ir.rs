//! IR 文書の読み込みと検査
//!
//! 文書の形の検査と`項目`の値の取り出しはスキーマの側（kotowari-markdown-schema）に任せ、
//! 返った`抽出`から `IrDocument` と `Item` を組み立てる（REQ-core-169）。kotowari が生の行を
//! 読むのは、gherkin の塊の中身と、閉じない`コードブロック`の検出の2つだけ。

use crate::config::Config;
use crate::finding_map::read_document;
use crate::{Finding, FindingKind, StopReason};
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// IR の文書の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocKind {
    /// 話題ごとの文書
    Topic,
    /// 用語集（CONTEXT.md）
    Glossary,
    /// 問題の記録（FLAGS.md）
    Flags,
}

impl DocKind {
    /// 文書名から種類を決める
    fn of(filename: &str) -> Self {
        match filename {
            "CONTEXT.md" => DocKind::Glossary,
            "FLAGS.md" => DocKind::Flags,
            _ => DocKind::Topic,
        }
    }
}

/// 要求の種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementKind {
    EventDriven,
    StateDriven,
    Ubiquitous,
    Prohibition,
    Invariant,
    Algorithm,
}

/// 検証の種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verification {
    Unit,
    Property,
    Proof,
    Review,
}

/// 問題の記録の種類
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlagKind {
    Contradiction,
    Gap,
    Ambiguity,
}

/// 項目の ID の種別
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdPrefix {
    Req,
    Tbl,
    Prop,
    Ex,
    Flag,
}

/// 解析した項目
#[derive(Debug, Clone)]
pub enum Item {
    Requirement {
        id: String,
        name: String,
        line: usize,
        kind: Option<String>,
        sources: Vec<String>,
        /// "- 出典:" の行。行が無ければ None
        source_line: Option<usize>,
        verification: Option<String>,
        definitions: Vec<String>,
        /// "- 定義:" の行。行が無ければ None
        definition_line: Option<usize>,
        /// TBL-core-011: "- 確かめ方:" の値。人が確かめる手順の自由文
        how_to_verify: Option<String>,
        statements: Vec<(usize, String)>,
    },
    DecisionTable {
        id: String,
        name: String,
        line: usize,
        sources: Vec<String>,
        source_line: Option<usize>,
        statements: Vec<(usize, String)>,
    },
    Property {
        id: String,
        name: String,
        line: usize,
        sources: Vec<String>,
        source_line: Option<usize>,
        statements: Vec<(usize, String)>,
    },
    Scenario {
        id: Option<String>,
        line: usize,
        tag_line: Option<usize>,
        tags: Vec<(String, String)>,
        about: Vec<String>,
        sources: Vec<String>,
        steps: Vec<(usize, String)>,
        scenario_text: String,
    },
    FlagEntry {
        id: String,
        name: String,
        line: usize,
        kind: Option<String>,
        relations: Vec<String>,
        /// "- 関係:" の行。行が無ければ None
        relation_line: Option<usize>,
        sources: Vec<String>,
        source_line: Option<usize>,
        body: Vec<(usize, String)>,
    },
    GlossaryTerm {
        term: String,
        meaning: String,
        sources: Vec<String>,
        line: usize,
    },
}

/// REQ-core-049: "- 検証:" に書ける4つの値
pub const VERIFICATION_VALUES: [&str; 4] = ["unit", "property", "proof", "review"];

impl Item {
    pub fn id(&self) -> Option<&str> {
        match self {
            Item::Requirement { id, .. }
            | Item::DecisionTable { id, .. }
            | Item::Property { id, .. }
            | Item::FlagEntry { id, .. } => Some(id),
            Item::Scenario { id, .. } => id.as_deref(),
            Item::GlossaryTerm { .. } => None,
        }
    }

    pub fn item_line(&self) -> usize {
        match self {
            Item::Requirement { line, .. }
            | Item::DecisionTable { line, .. }
            | Item::Property { line, .. }
            | Item::Scenario { line, .. }
            | Item::FlagEntry { line, .. }
            | Item::GlossaryTerm { line, .. } => *line,
        }
    }
}

/// 解析した IR 文書
#[derive(Debug)]
pub struct IrDocument {
    pub filename: String,
    pub relative_path: String,
    pub directory: String,
    pub kind: DocKind,
    /// `文書が扱う範囲`の`文`の行（行番号と行の文字そのまま）
    pub scope_lines: Vec<(usize, String)>,
    pub line_count: usize,
    pub items: Vec<Item>,
    pub raw_content: String,
    /// 文書1つで決まる指摘（スキーマの側から写したもの、gherkin の中身、閉じないコードブロック、
    /// 用語集の行）。パスは空で、`check_documents` が入れる
    pub parse_findings: Vec<crate::Finding>,
}

impl IrDocument {
    pub(crate) fn is_glossary_in_chain(&self, directory: &str) -> bool {
        self.kind == DocKind::Glossary && (self.directory.is_empty()
            || self.directory == directory
            || directory.strip_prefix(&self.directory).is_some_and(|rest| rest.starts_with('/')))
    }

    pub(crate) fn duplicate_glossary_rows(&self, docs: &[IrDocument]) -> BTreeSet<usize> {
        let ancestors: BTreeSet<&str> = docs.iter()
            .filter(|doc| doc.directory != self.directory && doc.is_glossary_in_chain(&self.directory))
            .flat_map(|doc| &doc.items)
            .filter_map(|item| match item {
                Item::GlossaryTerm { term, .. } => Some(term.as_str()),
                _ => None,
            })
            .collect();
        self.items.iter().filter_map(|item| match item {
            Item::GlossaryTerm { term, line, .. } if ancestors.contains(term.as_str()) => Some(*line),
            _ => None,
        }).collect()
    }

    /// 文書の`文`の行（`文書が扱う範囲`、`項目`の`文`、`問題の記録`の本文）。
    /// `文書名の参照`はこの行の上で探す（REQ-core-170）
    pub fn statement_lines(&self) -> impl Iterator<Item = &(usize, String)> {
        let item_lines = self.items.iter().flat_map(|item| match item {
            Item::Requirement { statements, .. }
            | Item::DecisionTable { statements, .. }
            | Item::Property { statements, .. }
            | Item::FlagEntry { body: statements, .. } => statements.as_slice(),
            Item::Scenario { .. } | Item::GlossaryTerm { .. } => &[],
        });
        self.scope_lines.iter().chain(item_lines)
    }
}

/// 行を \n で分割し、\r\n は1行として数える（TBL-core-010）
pub fn split_lines(content: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let bytes = content.as_bytes();
    let len = bytes.len();

    while start < len {
        if let Some(pos) = content[start..].find('\n') {
            let end = start + pos;
            let line = if end > start && bytes[end - 1] == b'\r' {
                &content[start..end - 1]
            } else {
                &content[start..end]
            };
            lines.push(line);
            start = end + 1;
        } else {
            // 最後の行（改行なし）
            lines.push(&content[start..]);
            break;
        }
    }

    // 空の入力は0行
    if lines.is_empty() && !content.is_empty() {
        lines.push(content);
    }

    lines
}

/// 文書1つを読む。形の検査と値の取り出しはスキーマの側が行い、kotowari はその`抽出`から
/// `項目`を組み立てる（REQ-core-169、REQ-core-170）。写せない`指摘`や値は`停止`になる
/// （REQ-core-172、REQ-core-175）
pub fn parse_document(filename: &str, content: &str) -> Result<IrDocument, StopReason> {
    // BOM の読み飛ばし（read_utf8_file でも除去するが、直接呼ばれた場合にも対応）
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);
    let kind = DocKind::of(filename);
    let (values, mut findings) = read_document(filename, kind, content)?;

    let mut items = Vec::new();
    match kind {
        DocKind::Topic => {
            for obj in elements(values.get("requirements")) {
                items.extend(requirement(obj, &mut findings)?);
            }
            for obj in elements(values.get("tables")) {
                items.extend(decision_table(obj)?);
            }
            for obj in elements(values.get("properties")) {
                items.extend(property(obj, &mut findings)?);
            }
            for obj in elements(values.get("scenarios")) {
                let opening = number(obj, "line")?;
                let content = string(obj, "value").unwrap_or_default();
                GherkinBlock::read(opening, &content, &mut items, &mut findings);
            }
        }
        DocKind::Flags => {
            // 問題の記録の項目は文書の直下（"flags"）と節の下（"flags_in_section"）の両方にある。
            // 並びは下の行の順の並べ替えで揃う（TBL-core-011）
            let direct = elements(values.get("flags"));
            let in_section = elements(values.get("flags_in_section"));
            for obj in direct.into_iter().chain(in_section) {
                items.extend(flag_entry(obj, &mut findings)?);
            }
        }
        DocKind::Glossary => read_glossary(&values, &mut items, &mut findings)?,
    }
    items.sort_by_key(Item::item_line);

    // REQ-core-112: 閉じないコードブロックは、開始から文書の終わりまでを検査の対象から外す
    if let Some((opening, raw)) = unclosed_code_block(content) {
        findings.retain(|f| f.line.is_none_or(|l| l < opening));
        items.retain(|item| item.item_line() < opening);
        findings.push(Finding::new(FindingKind::UnclosedCodeBlock, String::new(), Some(opening), raw));
    }

    Ok(IrDocument {
        filename: filename.to_string(),
        relative_path: filename.to_string(),
        directory: String::new(),
        kind,
        scope_lines: statements_of(values.get("scope"))?,
        line_count: split_lines(content).len(),
        items,
        raw_content: content.to_string(),
        parse_findings: findings,
    })
}

// --- `抽出`の値から型へ写す ---

/// 抽出の値を型へ写せない（A20、REQ-core-175）
fn unmappable(what: &str) -> StopReason {
    StopReason::MappingError(format!("extracted value has no {what}"))
}

/// 抽出の値の並び。1つだけのときも並びと同じに読む
fn elements(value: Option<&Value>) -> Vec<&Map<String, Value>> {
    match value {
        Some(Value::Array(values)) => values.iter().filter_map(Value::as_object).collect(),
        Some(Value::Object(obj)) => vec![obj],
        _ => Vec::new(),
    }
}

fn string(obj: &Map<String, Value>, key: &str) -> Option<String> {
    obj.get(key).and_then(Value::as_str).map(str::to_string)
}

fn number(obj: &Map<String, Value>, key: &str) -> Result<usize, StopReason> {
    obj.get(key)
        .and_then(Value::as_u64)
        .map(|n| n as usize)
        .ok_or_else(|| unmappable(key))
}

/// `項目`の`ID`・名前・見出しの行
fn head(obj: &Map<String, Value>) -> Result<(String, String, usize), StopReason> {
    let id = string(obj, "id").ok_or_else(|| unmappable("id"))?;
    let name = string(obj, "name").ok_or_else(|| unmappable("name"))?;
    Ok((id, name, number(obj, "line")?))
}

/// 見出しの`ID`がその`項目`の種類の形に合うか。合わない見出しはスキーマの側が
/// unknown_heading にしているので、`項目`にしない（REQ-core-043、REQ-core-114）
fn is_item_id(id: &str, prefix: IdPrefix) -> bool {
    id_prefix(id) == Some(prefix) && is_valid_id(id)
}

/// "," で分けた`フィールド行`の値と、その行。行が無ければ空の並びと None
fn listed(value: Option<&Value>) -> Result<(Vec<String>, Option<usize>), StopReason> {
    let Some(obj) = value.and_then(Value::as_object) else {
        return Ok((Vec::new(), None));
    };
    let values = match obj.get("value") {
        Some(Value::Array(values)) => values.iter().filter_map(Value::as_str).collect(),
        Some(Value::String(value)) => vec![value.as_str()],
        _ => Vec::new(),
    };
    let values = values
        .into_iter()
        .flat_map(|value| value.split(','))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect();
    Ok((values, Some(number(obj, "line")?)))
}

/// `文`の行の並び（行番号と行の文字そのまま）
fn statements_of(value: Option<&Value>) -> Result<Vec<(usize, String)>, StopReason> {
    elements(value)
        .into_iter()
        .map(|obj| {
            let raw = string(obj, "raw").ok_or_else(|| unmappable("raw"))?;
            Ok((number(obj, "line")?, raw))
        })
        .collect()
}

/// `項目`の`文`。gherkin のブロックの外の "Scenario:" の行は`文`にしない（REQ-core-100、`除外`）。
/// スキーマの側はその行も`文`に数えるので、それを除くと`文`が無くなる`項目`には
/// missing_statement を足す（REQ-core-047）
fn item_statements(
    obj: &Map<String, Value>,
    requires_statement: bool,
    id: &str,
    line: usize,
    findings: &mut Vec<Finding>,
) -> Result<Vec<(usize, String)>, StopReason> {
    let all = statements_of(obj.get("text"))?;
    let kept: Vec<(usize, String)> = all
        .iter()
        .filter(|(_, text)| !text.trim_start().starts_with("Scenario:"))
        .cloned()
        .collect();
    if requires_statement && kept.is_empty() && !all.is_empty() {
        findings.push(Finding::new(FindingKind::MissingStatement, String::new(), Some(line), id.to_string()));
    }
    Ok(kept)
}

fn requirement(obj: &Map<String, Value>, findings: &mut Vec<Finding>) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !is_item_id(&id, IdPrefix::Req) {
        return Ok(None);
    }
    let (sources, source_line) = listed(obj.get("sources"))?;
    let (definitions, definition_line) = listed(obj.get("definitions"))?;
    let kind = string(obj, "kind");
    let requires_statement = kind.as_deref() != Some("algorithm");
    let statements = item_statements(obj, requires_statement, &id, line, findings)?;
    Ok(Some(Item::Requirement {
        id,
        name,
        line,
        kind,
        sources,
        source_line,
        verification: string(obj, "verification"),
        definitions,
        definition_line,
        how_to_verify: string(obj, "how_to_verify"),
        statements,
    }))
}

fn decision_table(obj: &Map<String, Value>) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !is_item_id(&id, IdPrefix::Tbl) {
        return Ok(None);
    }
    let (sources, source_line) = listed(obj.get("sources"))?;
    let statements = item_statements(obj, false, &id, line, &mut Vec::new())?;
    Ok(Some(Item::DecisionTable {
        id,
        name,
        line,
        sources,
        source_line,
        statements,
    }))
}

fn property(obj: &Map<String, Value>, findings: &mut Vec<Finding>) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !is_item_id(&id, IdPrefix::Prop) {
        return Ok(None);
    }
    let (sources, source_line) = listed(obj.get("sources"))?;
    let statements = item_statements(obj, true, &id, line, findings)?;
    Ok(Some(Item::Property {
        id,
        name,
        line,
        sources,
        source_line,
        statements,
    }))
}

fn flag_entry(obj: &Map<String, Value>, findings: &mut Vec<Finding>) -> Result<Option<Item>, StopReason> {
    let (id, name, line) = head(obj)?;
    if !is_item_id(&id, IdPrefix::Flag) {
        return Ok(None);
    }
    let (relations, relation_line) = listed(obj.get("relations"))?;
    let (sources, source_line) = listed(obj.get("sources"))?;
    let body = item_statements(obj, true, &id, line, findings)?;
    Ok(Some(Item::FlagEntry {
        id,
        name,
        line,
        kind: string(obj, "kind"),
        relations,
        relation_line,
        sources,
        source_line,
        body,
    }))
}

/// `用語集`の表の行から`用語`を組み立てる（REQ-core-117、REQ-core-122、REQ-core-123）。
/// 表そのものの誤りと列の数の誤りはスキーマの側が出す。`用語`のセルが空の行と、
/// 同じ文書の中で重なった`用語`は kotowari が見る（REQ-core-173）
fn read_glossary(
    values: &Value,
    items: &mut Vec<Item>,
    findings: &mut Vec<Finding>,
) -> Result<(), StopReason> {
    // REQ-core-117: 表の形が無ければ、その用語集の用語は0語
    if findings.iter().any(|f| f.kind == FindingKind::GlossaryInvalid) {
        return Ok(());
    }
    // REQ-core-122: 列の数の合わない行は用語にしない
    let invalid_rows: BTreeSet<usize> = findings
        .iter()
        .filter(|f| f.kind == FindingKind::InvalidGlossaryRow)
        .filter_map(|f| f.line)
        .collect();
    // REQ-core-117: 用語になるのは最初の表の行だけ。表が繰り返すと表ごとの段ができる
    let rows = match values.get("glossary") {
        Some(Value::Array(tables)) if tables.first().is_some_and(Value::is_array) => {
            elements(tables.first())
        }
        other => elements(other),
    };
    let mut seen_terms = BTreeSet::new();
    for row in rows {
        let line = number(row, "line")?;
        if invalid_rows.contains(&line) {
            continue;
        }
        let cells = row.get("value").and_then(Value::as_object);
        let cell = |name: &str| {
            cells
                .and_then(|cells| string(cells, name))
                .unwrap_or_default()
        };
        let term = cell("用語");
        if term.is_empty() {
            let raw = string(row, "raw").ok_or_else(|| unmappable("raw"))?;
            findings.push(Finding::new(FindingKind::InvalidGlossaryRow, String::new(), Some(line), raw));
        } else if !seen_terms.insert(term.clone()) {
            // REQ-core-123: 同じ用語の2つ目以降の行は duplicate_term だけを出し、用語にしない
            findings.push(Finding::new(FindingKind::DuplicateTerm, String::new(), Some(line), term));
        } else {
            let sources = cell("出典")
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();
            items.push(Item::GlossaryTerm { term, meaning: cell("意味"), sources, line });
        }
    }
    Ok(())
}

// --- 生の行を読む2つ: 閉じないコードブロックと gherkin の中身 ---

/// 閉じない`コードブロック`の開始の行と、その行の文字（REQ-core-112）
fn unclosed_code_block(content: &str) -> Option<(usize, String)> {
    let mut open: Option<(CodeFence, usize, &str)> = None;
    for (idx, line) in split_lines(content).into_iter().enumerate() {
        match &open {
            Some((fence, _, _)) => {
                if is_closing_fence(line, fence) {
                    open = None;
                }
            }
            None => {
                if let Some(fence) = parse_opening_fence(line) {
                    open = Some((fence, idx + 1, line));
                }
            }
        }
    }
    open.map(|(_, line, raw)| (line, raw.to_string()))
}

/// gherkin の塊1つの中身からタグとシナリオを組み立てる（REQ-core-113、A1、A6）
#[derive(Default)]
struct GherkinBlock {
    tags: Vec<(String, String)>,
    tag_line: Option<usize>,
    tag_raw: Option<String>,
    scenario_line: Option<usize>,
    scenario_text: String,
    steps: Vec<(usize, String)>,
    prev_was_tag: bool,
    /// 直前の行がステップだったか（Scenario: の無いブロックで続くステップを誤りにしないため）
    prev_was_step: bool,
}

impl GherkinBlock {
    /// 塊の開始の行（囲みの行）と中身を読む。中身の1行目は開始の行の次の行
    fn read(opening: usize, content: &str, items: &mut Vec<Item>, findings: &mut Vec<Finding>) {
        let mut block = GherkinBlock::default();
        for (offset, line) in content.split('\n').enumerate() {
            let line = line.strip_suffix('\r').unwrap_or(line);
            block.line(opening + 1 + offset, line, items, findings);
        }
        block.finish(items, findings);
    }

    fn invalid_tag_line(&self, findings: &mut Vec<Finding>) {
        if let Some(tl) = self.tag_line {
            findings.push(Finding::new(
                FindingKind::InvalidGherkinLine,
                String::new(),
                Some(tl),
                self.tag_raw.clone().unwrap_or_default(),
            ));
        }
    }

    fn clear_tags(&mut self) {
        self.tags.clear();
        self.tag_line = None;
        self.tag_raw = None;
    }

    fn flush_scenario(&mut self, items: &mut Vec<Item>) {
        if let Some(scenario_line) = self.scenario_line.take() {
            items.push(build_scenario(&self.tags, self.tag_line, scenario_line, &self.steps, &self.scenario_text));
            self.steps.clear();
        }
    }

    fn line(&mut self, line_num: usize, line: &str, items: &mut Vec<Item>, findings: &mut Vec<Finding>) {
        let trimmed = line.trim();
        let is_scenario_line = trimmed.starts_with("Scenario:");

        // REQ-core-113/A155: タグの行の直後が Scenario: でなければ、そのタグの行自体が invalid_gherkin_line
        if self.prev_was_tag && !is_scenario_line {
            self.invalid_tag_line(findings);
            // 結び付かないタグの検査（REQ-core-052: 結び付くかを問わない）をしてから捨てる
            check_gherkin_tags_findings(&self.tags, self.tag_line, findings);
            self.clear_tags();
        }

        if trimmed.starts_with('@') {
            // タグ行: 前のシナリオがあれば追加
            self.flush_scenario(items);
            self.tags.clear();
            self.tag_line = Some(line_num);
            self.tag_raw = Some(line.to_string());
            // タグを解析（REQ-core-052: "@" で始まる語だけを name=value に分ける）
            for part in trimmed.split_whitespace() {
                if part.starts_with('@') {
                    if let Some((tag_name, tag_value)) = part.split_once('=') {
                        self.tags.push((tag_name.to_string(), tag_value.to_string()));
                    } else {
                        // "=" のない裸のタグ（@wip 等）
                        self.tags.push((part.to_string(), String::new()));
                    }
                } else {
                    // REQ-core-052: "@" で始まらない語はその語自体を detail にする
                    self.tags.push((String::new(), part.to_string()));
                }
            }
            self.prev_was_tag = true;
            self.prev_was_step = false;
        } else if is_scenario_line {
            self.flush_scenario(items);
            // REQ-core-113: このタグは直前の行にあるときだけ、いま始まるシナリオに結び付く。
            // 結び付かない（直前がタグの行でない）ときは、前のシナリオで使い終えたタグを持ち越さない。
            if !self.prev_was_tag {
                self.clear_tags();
            }
            self.scenario_line = Some(line_num);
            // A150: detail の元になる Scenario: の行は生の行（字下げを含む）を持つ
            self.scenario_text = line.to_string();
            self.prev_was_tag = false;
            self.prev_was_step = false;
        } else if ["Given ", "When ", "Then ", "And ", "But "]
            .iter()
            .any(|keyword| trimmed.starts_with(keyword))
        {
            if self.scenario_line.is_some() {
                self.steps.push((line_num, line.to_string()));
            } else if !self.prev_was_step {
                // REQ-core-113/A155: 直前に Scenario: もステップも無いステップの行は invalid_gherkin_line。
                // 続く2つ目以降のステップは最初のステップの誤りに含め、別の誤りにしない
                findings.push(Finding::new(
                    FindingKind::InvalidGherkinLine,
                    String::new(),
                    Some(line_num),
                    line.to_string(),
                ));
            }
            self.prev_was_tag = false;
            self.prev_was_step = true;
        } else if trimmed.starts_with('#') || trimmed.is_empty() {
            // REQ-core-113: 注釈と空行は有効
            self.prev_was_tag = false;
            self.prev_was_step = false;
        } else {
            // REQ-core-113: それ以外は invalid_gherkin_line
            // TBL-core-008: detail は行の文字そのまま（字下げと末尾の空白を含む）
            findings.push(Finding::new(
                FindingKind::InvalidGherkinLine,
                String::new(),
                Some(line_num),
                line.to_string(),
            ));
            self.prev_was_tag = false;
            self.prev_was_step = false;
        }
    }

    /// 塊の終わり
    fn finish(mut self, items: &mut Vec<Item>, findings: &mut Vec<Finding>) {
        // REQ-core-113/A155: ブロックの終わりが Scenario: でなければ、直前のタグの行は invalid_gherkin_line
        if self.prev_was_tag {
            self.invalid_tag_line(findings);
        }
        if self.scenario_line.is_some() {
            self.flush_scenario(items);
        } else {
            // シナリオに結び付かなかったタグの検査
            check_gherkin_tags_findings(&self.tags, self.tag_line, findings);
        }
    }
}

/// ID の接頭辞を判定する
fn id_prefix(id: &str) -> Option<IdPrefix> {
    if id.starts_with("REQ-") {
        Some(IdPrefix::Req)
    } else if id.starts_with("TBL-") {
        Some(IdPrefix::Tbl)
    } else if id.starts_with("PROP-") {
        Some(IdPrefix::Prop)
    } else if id.starts_with("EX-") {
        Some(IdPrefix::Ex)
    } else if id.starts_with("FLAG-") {
        Some(IdPrefix::Flag)
    } else {
        None
    }
}

/// ID を接頭辞の後ろの「省いてよい名前」と「数字」に分ける。形に合わなければ None（REQ-core-124）
fn split_id(s: &str) -> Option<(Option<&str>, &str)> {
    let prefix = id_prefix(s)?;
    let rest = match prefix {
        IdPrefix::Req => &s[4..],
        IdPrefix::Tbl => &s[4..],
        IdPrefix::Prop => &s[5..],
        IdPrefix::Ex => &s[3..],
        IdPrefix::Flag => &s[5..],
    };
    // 最後の "-" より後ろを数字、その前を名前とする
    let (name, digits) = match rest.rsplit_once('-') {
        Some((name, digits)) => (Some(name), digits),
        None => (None, rest),
    };
    let digits_ok = digits.len() >= 3
        && (digits.len() == 3 || !digits.starts_with('0'))
        && digits.chars().all(|c| c.is_ascii_digit());
    if !digits_ok {
        return None;
    }
    if name.is_some_and(|name| !is_valid_id_name(name)) {
        return None;
    }
    Some((name, digits))
}

/// ID の名前の形（小文字の英字で始まり、2文字目からは小文字の英数字と "-"。REQ-core-124）
fn is_valid_id_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// ID の形式を検証する（省いてよい名前、数字は3桁以上、4桁以上では先頭の0を認めない）
pub fn is_valid_id(s: &str) -> bool {
    split_id(s).is_some()
}

/// ID の名前を返す。名前が無ければ None（REQ-core-124、REQ-core-167）
pub fn id_name(s: &str) -> Option<&str> {
    split_id(s).and_then(|(name, _)| name)
}

/// 結び付かなかったタグの行を検査する（REQ-core-052: 結び付くかを問わない）
fn check_gherkin_tags_findings(
    tags: &[(String, String)],
    tag_line: Option<usize>,
    findings: &mut Vec<crate::Finding>,
) {
    for (tag_name, tag_value) in tags {
        if tag_name.is_empty() {
            // "@" で始まらない語
            findings.push(crate::Finding::new(
                crate::FindingKind::UnknownTag,
                String::new(),
                tag_line,
                tag_value.clone(),
            ));
        } else if !["@id", "@about", "@source"].contains(&tag_name.as_str()) {
            findings.push(crate::Finding::new(
                crate::FindingKind::UnknownTag,
                String::new(),
                tag_line,
                tag_name.clone(),
            ));
        }
    }
}

/// gherkin のシナリオを組み立てる
fn build_scenario(
    tags: &[(String, String)],
    tag_line: Option<usize>,
    scenario_line: usize,
    steps: &[(usize, String)],
    scenario_text: &str,
) -> Item {
    let mut id = None;
    let mut about = Vec::new();
    let mut sources = Vec::new();
    for (tag_name, tag_value) in tags {
        match tag_name.as_str() {
            "@id" => {
                // REQ-core-114: @id の値が EX の ID の形でないときは定義に数えない
                if is_valid_id(tag_value) && id_prefix(tag_value) == Some(IdPrefix::Ex) {
                    id = Some(tag_value.clone());
                }
                // 形に合わない値は check_item で invalid_id として報告
            }
            "@about" => {
                about = tag_value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            "@source" => {
                sources = tag_value
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            _ => {}
        }
    }

    Item::Scenario {
        id,
        line: scenario_line,
        tag_line,
        tags: tags.to_vec(),
        about,
        sources,
        steps: steps.to_vec(),
        scenario_text: scenario_text.to_string(),
    }
}

/// 文書をまたぐ検査と、kotowari の側に残る検査をして指摘を返す（REQ-core-173）。
/// 文書1つで決まる指摘は`parse_findings`から写す。形の指摘が出た文書も、取れた値で
/// 文書をまたぐ検査を受ける（REQ-core-176）
pub fn check_documents(docs: &[IrDocument], config: &Config) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 全 ID を収集して重複を検出
    let mut all_ids: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new(); // id -> [(path, line)]

    for doc in docs {
        let path = crate::join_display_path(&config.ir, &doc.relative_path);

        // REQ-core-038: 行数の上限
        let limit_lines = config.limits.lines.get() as usize;
        if doc.line_count > limit_lines {
            findings.push(Finding::new(FindingKind::TooManyLines, path.clone(), None, doc.line_count.to_string()));
        }

        // REQ-core-039: 要求の数の上限（用語集と問題の記録を除く）
        if doc.kind == DocKind::Topic {
            let req_count = doc
                .items
                .iter()
                .filter(|item| matches!(item, Item::Requirement { .. }))
                .count();
            let limit_reqs = config.limits.requirements.get() as usize;
            if req_count > limit_reqs {
                findings.push(Finding::new(FindingKind::TooManyRequirements, path.clone(), None, req_count.to_string()));
            }
        }

        // 項目の検査
        let duplicate_rows = doc.duplicate_glossary_rows(docs);
        for item in &doc.items {
            if duplicate_rows.contains(&item.item_line()) {
                if let Item::GlossaryTerm { term, line, .. } = item {
                    findings.push(Finding::new(FindingKind::DuplicateTerm, path.clone(), Some(*line), term.clone()));
                }
                continue;
            }
            check_item(item, &path, &mut findings);

            // ID を収集（形に合う ID だけ。REQ-core-114）
            if let Some(id) = item.id() {
                if is_valid_id(id) {
                    all_ids
                        .entry(id.to_string())
                        .or_default()
                        .push((path.clone(), item.item_line()));

                    // REQ-core-167: ID の名前は文書の置き場の第1階層と一致する。
                    // 第1階層を持たない文書（IR の置き場の直下）では指す先が無いので不一致になる
                    if let Some(name) = id_name(id) {
                        let domain = doc.directory.split('/').next().unwrap_or("");
                        if name != domain {
                            let line = match item {
                                Item::Scenario { tag_line, line, .. } => tag_line.unwrap_or(*line),
                                _ => item.item_line(),
                            };
                            findings.push(Finding::new(
                                FindingKind::IdDomainMismatch,
                                path.clone(),
                                Some(line),
                                id.to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }

    // REQ-core-032: ID の重複
    for (id, locations) in &all_ids {
        // 2つ目以降の場所に指摘
        for (path, line) in locations.iter().skip(1) {
            findings.push(Finding::new(FindingKind::DuplicateId, path.clone(), Some(*line), id.clone()));
        }
    }

    // 参照の解決チェック（REQ-core-054）
    let known_ids = crate::collect_known_ids(docs);
    for doc in docs {
        let path = crate::join_display_path(&config.ir, &doc.relative_path);
        check_references(&doc.items, &known_ids, &path, &mut findings);
    }

    // 文書1つで決まる指摘に文書のパスを入れる
    for doc in docs {
        let doc_path = crate::join_display_path(&config.ir, &doc.relative_path);
        for pf in &doc.parse_findings {
            let mut f = pf.clone();
            if f.path.is_empty() {
                f.path = doc_path.clone();
            }
            findings.push(f);
        }
    }

    findings
}

/// "- 出典:" の行はあるのに値が1つも無い（REQ-core-059）。行そのものが無いときは
/// スキーマの側が missing_source にしている
fn has_empty_source_line(sources: &[String], source_line: &Option<usize>) -> bool {
    source_line.is_some() && sources.is_empty()
}

/// 項目の検査のうち、スキーマの側に宣言の無いもの
fn check_item(item: &Item, path: &str, findings: &mut Vec<Finding>) {
    match item {
        Item::Requirement { id, line, kind, sources, source_line, definitions, definition_line, .. } => {
            if has_empty_source_line(sources, source_line) {
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), id.clone()));
            }

            // REQ-core-051: algorithm に決定表か性質を指す定義がない。
            // "- 定義:" の行そのものが無いときはスキーマの側が出している（A71）
            let has_tbl_or_prop_def = definitions.iter().any(|d| {
                matches!(id_prefix(d), Some(IdPrefix::Tbl) | Some(IdPrefix::Prop))
            });
            if kind.as_deref() == Some("algorithm") && definition_line.is_some() && !has_tbl_or_prop_def {
                findings.push(Finding::new(FindingKind::AlgorithmWithoutDefinition, path.to_string(), Some(*line), id.clone()));
            }
        }

        Item::DecisionTable { id, line, sources, source_line, .. }
        | Item::Property { id, line, sources, source_line, .. }
        | Item::FlagEntry { id, line, sources, source_line, .. } => {
            if has_empty_source_line(sources, source_line) {
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), id.clone()));
            }
        }

        Item::Scenario {
            id,
            line,
            tag_line,
            tags,
            sources,
            scenario_text,
            ..
        } => {
            // TBL-core-019: タグの行（無ければ Scenario: の行）
            let tag_or_scenario_line = tag_line.unwrap_or(*line);

            // REQ-core-052: 知らないタグ（結び付くかを問わない）
            for (tag_name, tag_value) in tags {
                if tag_name.is_empty() {
                    // "@" で始まらない語
                    findings.push(Finding::new(FindingKind::UnknownTag, path.to_string(), Some(tag_or_scenario_line), tag_value.clone()));
                } else if !["@id", "@about", "@source"].contains(&tag_name.as_str()) {
                    findings.push(Finding::new(FindingKind::UnknownTag, path.to_string(), Some(tag_or_scenario_line), tag_name.clone()));
                }
            }

            // REQ-core-114: @id の値が EX の ID の形でないとき
            let malformed_id = tags.iter()
                .find(|(n, v)| n == "@id" && !v.is_empty() && !(is_valid_id(v) && id_prefix(v) == Some(IdPrefix::Ex)))
                .map(|(_, v)| v.clone());
            if let Some(malformed_value) = &malformed_id {
                findings.push(Finding::new(FindingKind::InvalidId, path.to_string(), Some(tag_or_scenario_line), malformed_value.clone()));
                // missing_tag は出さない、missing_source の detail は Scenario: の行の文字
            } else if !tags.iter().any(|(n, v)| n == "@id" && !v.is_empty()) {
                // REQ-core-053: 無いタグ
                findings.push(Finding::new(FindingKind::MissingTag, path.to_string(), Some(tag_or_scenario_line), "@id".to_string()));
            }
            if !tags.iter().any(|(n, v)| n == "@about" && !v.is_empty()) {
                findings.push(Finding::new(FindingKind::MissingTag, path.to_string(), Some(tag_or_scenario_line), "@about".to_string()));
            }

            // REQ-core-059: シナリオの出典
            if sources.is_empty() && !tags.iter().any(|(n, v)| n == "@source" && !v.is_empty()) {
                // REQ-core-114: @id が形に合わないときは Scenario: の行の文字を使う
                let detail = if malformed_id.is_some() {
                    scenario_text.clone()
                } else {
                    id.as_deref().unwrap_or(scenario_text).to_string()
                };
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(tag_or_scenario_line), detail));
            }
        }

        Item::GlossaryTerm { term, sources, line, .. } => {
            // REQ-core-059/REQ-core-060: 用語の出典が空
            if sources.is_empty() {
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), term.clone()));
            }
        }
    }
}

/// 項目が `ID` を指している場所（TBL-core-027 の "via"）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Via {
    /// "- 定義:" の行
    Definition,
    /// "- 関係:" の行
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
    // TBL-core-019: "- 定義:" か "- 関係:" の行（行が無ければ見出しの行）
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
        Item::Requirement { line, definitions, definition_line, statements, .. } => {
            let def_line = definition_line.unwrap_or(*line);
            definitions.iter()
                .map(|id| ItemReference { id, via: Via::Definition, finding_line: def_line })
                .chain(from_text(statements))
                .collect()
        }
        Item::Property { statements, .. } => from_text(statements).collect(),
        Item::Scenario { line, tag_line, about, steps, .. } => {
            let about_line = tag_line.unwrap_or(*line);
            about.iter()
                .map(|id| ItemReference { id, via: Via::About, finding_line: about_line })
                .chain(from_text(steps))
                .collect()
        }
        Item::FlagEntry { line, relations, relation_line, .. } => {
            let rel_line = relation_line.unwrap_or(*line);
            relations.iter()
                .map(|id| ItemReference { id, via: Via::Relations, finding_line: rel_line })
                .collect()
        }
        _ => Vec::new(),
    }
}

/// 参照の解決チェック（REQ-core-054）。
/// `ID` の形でない "- 定義:" の値は`ID`の一覧に入らないので、ここで誤りになる
fn check_references(
    items: &[Item],
    known_ids: &BTreeSet<String>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    for item in items {
        for reference in item_references(item) {
            if !known_ids.contains(reference.id) {
                findings.push(Finding::new(
                    FindingKind::UnresolvedReference,
                    path.to_string(),
                    Some(reference.finding_line),
                    reference.id.to_string(),
                ));
            }
        }
    }
}

/// コードブロックの囲みの情報
#[derive(Debug, Clone)]
pub struct CodeFence {
    /// 囲みの文字（'`' or '~'）
    pub fence_char: char,
    /// 囲みの長さ（3以上）
    pub fence_len: usize,
    /// 言語（あれば）
    pub lang: String,
}

/// 行がコードブロックの開始の囲みかどうかを判定する。
/// 3つ以上の ` か ~ で始まる行が該当する。
pub fn parse_opening_fence(line: &str) -> Option<CodeFence> {
    let bytes = line.as_bytes();
    if bytes.len() < 3 {
        return None;
    }
    let fence_char = bytes[0] as char;
    if fence_char != '`' && fence_char != '~' {
        return None;
    }
    let fence_len = bytes.iter().take_while(|&&b| b == bytes[0]).count();
    if fence_len < 3 {
        return None;
    }
    let lang = line[fence_len..].trim().to_string();
    Some(CodeFence {
        fence_char,
        fence_len,
        lang,
    })
}

/// 行が開いている囲みを閉じるかどうかを判定する。
pub fn is_closing_fence(line: &str, opening: &CodeFence) -> bool {
    let bytes = line.as_bytes();
    if bytes.is_empty() || bytes[0] as char != opening.fence_char {
        return false;
    }
    let fence_len = bytes.iter().take_while(|&&b| b == bytes[0]).count();
    if fence_len < opening.fence_len {
        return false;
    }
    // 閉じる囲みの後は空白だけ
    line[fence_len..].trim().is_empty()
}

/// バッククォートで囲まれた内容を抽出する。
/// 空の内容（``）も返す。
pub fn extract_backtick_contents(text: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    while let Some(open) = text[start..].find('`') {
        let open_abs = start + open + 1;
        if open_abs >= text.len() {
            break;
        }
        if let Some(close) = text[open_abs..].find('`') {
            let content = &text[open_abs..open_abs + close];
            result.push(content);
            start = open_abs + close + 1;
        } else {
            break;
        }
    }
    result
}

/// 文やステップの中の、バッククォートで囲んだ `ID` を拾う（REQ-core-054、REQ-core-116）
fn backtick_ids(text: &str) -> Vec<&str> {
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

/// IR のディレクトリからすべての文書を読んで検査する
pub fn load_and_check(
    base: &Path,
    config: &Config,
) -> Result<(Vec<IrDocument>, Vec<Finding>), crate::StopReason> {
    let ir_dir = base.join(&config.ir);
    let mut entries = Vec::new();
    collect_ir_paths(&ir_dir, "", &config.ir, &mut entries)?;
    entries.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

    let mut docs = Vec::new();
    for (relative_path, path) in entries {
        let (directory, filename) = relative_path.rsplit_once('/').unwrap_or(("", &relative_path));
        let display = crate::join_display_path(&config.ir, &relative_path);
        let content = crate::read_utf8_file(&path, &display)?;
        let mut doc = parse_document(filename, &content)?;
        doc.directory = directory.to_string();
        doc.relative_path = relative_path;
        docs.push(doc);
    }
    let findings = check_documents(&docs, config);
    Ok((docs, findings))
}

fn collect_ir_paths(
    dir: &Path,
    prefix: &str,
    ir_path: &str,
    paths: &mut Vec<(String, std::path::PathBuf)>,
) -> Result<(), crate::StopReason> {
    let display = crate::join_display_path(ir_path, prefix);
    let display = if prefix.is_empty() { ir_path } else { &display };
    let unreadable = |e| crate::StopReason::UnreadableFile(format!("{display}: {e}"));
    for entry in std::fs::read_dir(dir).map_err(unreadable)? {
        let entry = entry.map_err(unreadable)?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let relative_path = crate::join_display_path(prefix, &name);
        let entry_display = crate::join_display_path(ir_path, &relative_path);
        let entry_error = |e| crate::StopReason::UnreadableFile(format!("{entry_display}: {e}"));
        let file_type = entry.file_type().map_err(entry_error)?;
        let (is_dir, is_file) = if file_type.is_symlink() {
            let metadata = std::fs::metadata(&path).map_err(entry_error)?;
            (false, metadata.is_file())
        } else {
            (file_type.is_dir(), file_type.is_file())
        };
        if is_dir && !name.starts_with('.') {
            collect_ir_paths(&path, &relative_path, ir_path, paths)?;
        } else if is_file && path.extension().is_some_and(|ext| ext == "md") {
            paths.push((relative_path, path));
        }
    }
    Ok(())
}
