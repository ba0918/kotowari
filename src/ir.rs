//! IR 文書の読み込みと形の検査

use crate::config::Config;
use crate::Finding;
use std::collections::{BTreeMap, BTreeSet, HashMap};
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
        verification: Option<String>,
        definitions: Vec<String>,
        statements: Vec<(usize, String)>,
        fields_seen: Vec<(usize, String)>,
    },
    DecisionTable {
        id: String,
        name: String,
        line: usize,
        sources: Vec<String>,
        has_table: bool,
        fields_seen: Vec<(usize, String)>,
    },
    Property {
        id: String,
        name: String,
        line: usize,
        sources: Vec<String>,
        definitions: Vec<String>,
        statements: Vec<(usize, String)>,
        fields_seen: Vec<(usize, String)>,
    },
    Scenario {
        id: Option<String>,
        line: usize,
        tags: Vec<(String, String)>,
        about: Vec<String>,
        sources: Vec<String>,
        steps: Vec<(usize, String)>,
    },
    FlagEntry {
        id: String,
        name: String,
        line: usize,
        kind: Option<String>,
        relations: Vec<String>,
        sources: Vec<String>,
        body: Vec<(usize, String)>,
        fields_seen: Vec<(usize, String)>,
    },
    GlossaryTerm {
        term: String,
        meaning: String,
        sources: Vec<String>,
        line: usize,
    },
    UnknownHeading {
        heading: String,
        line: usize,
    },
}

impl Item {
    pub fn id(&self) -> Option<&str> {
        match self {
            Item::Requirement { id, .. }
            | Item::DecisionTable { id, .. }
            | Item::Property { id, .. }
            | Item::FlagEntry { id, .. } => Some(id),
            Item::Scenario { id, .. } => id.as_deref(),
            Item::GlossaryTerm { .. } | Item::UnknownHeading { .. } => None,
        }
    }

    pub fn item_line(&self) -> usize {
        match self {
            Item::Requirement { line, .. }
            | Item::DecisionTable { line, .. }
            | Item::Property { line, .. }
            | Item::Scenario { line, .. }
            | Item::FlagEntry { line, .. }
            | Item::GlossaryTerm { line, .. }
            | Item::UnknownHeading { line, .. } => *line,
        }
    }
}

/// 解析した IR 文書
#[derive(Debug)]
pub struct IrDocument {
    pub filename: String,
    pub kind: DocKind,
    pub title: Option<(usize, String)>,
    pub extra_titles: Vec<(usize, String)>,
    pub scope_lines: Vec<(usize, String)>,
    pub line_count: usize,
    pub items: Vec<Item>,
    pub sections: Vec<(usize, String)>,
    pub raw_content: String,
}

/// 文書名の参照の検査用に行を分割する
pub fn split_lines_for_doc_ref<'a>(_filename: &str, content: &'a str) -> Vec<&'a str> {
    split_lines(content)
}

/// 行を \n で分割し、\r\n は1行として数える（TBL-010）
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

/// Markdown 文書を解析する
pub fn parse_document(filename: &str, content: &str) -> IrDocument {
    let kind = match filename {
        "CONTEXT.md" => DocKind::Glossary,
        "FLAGS.md" => DocKind::Flags,
        _ => DocKind::Topic,
    };

    let lines = split_lines(content);
    let line_count = lines.len();

    let mut title: Option<(usize, String)> = None;
    let mut extra_titles: Vec<(usize, String)> = Vec::new();
    let mut scope_lines: Vec<(usize, String)> = Vec::new();
    let mut items: Vec<Item> = Vec::new();
    let mut sections: Vec<(usize, String)> = Vec::new();

    let mut in_code_block = false;
    let mut in_gherkin_block = false;
    let mut found_first_section = false;

    // 現在の項目の解析状態
    let mut current_item: Option<ItemBuilder> = None;

    // gherkin ブロック内の状態
    let mut gherkin_tags: Vec<(String, String)> = Vec::new();
    let mut gherkin_scenario_line: Option<usize> = None;
    let mut gherkin_steps: Vec<(usize, String)> = Vec::new();

    // 用語集の解析
    let mut in_glossary_table = false;
    let mut glossary_header_seen = false;

    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1; // 1-indexed

        // コードブロックの開始/終了
        if line.starts_with("```") {
            if in_code_block || in_gherkin_block {
                // ブロックの終了
                if in_gherkin_block {
                    // gherkin ブロック終了: 未完了のシナリオがあれば追加
                    if let Some(scenario_line) = gherkin_scenario_line.take() {
                        let scenario = build_scenario(&gherkin_tags, scenario_line, &gherkin_steps);
                        items.push(scenario);
                        gherkin_tags.clear();
                        gherkin_steps.clear();
                    }
                }
                in_code_block = false;
                in_gherkin_block = false;
                continue;
            } else {
                // ブロックの開始
                let lang = line.trim_start_matches('`').trim();
                if lang == "gherkin" {
                    in_gherkin_block = true;
                } else {
                    in_code_block = true;
                }
                continue;
            }
        }

        // コードブロック内（gherkin 以外）はスキップ
        if in_code_block {
            continue;
        }

        // gherkin ブロック内の処理
        if in_gherkin_block {
            let trimmed = line.trim();
            if trimmed.starts_with('@') {
                // タグ行: 前のシナリオがあれば追加
                if let Some(scenario_line) = gherkin_scenario_line.take() {
                    let scenario = build_scenario(&gherkin_tags, scenario_line, &gherkin_steps);
                    items.push(scenario);
                    gherkin_steps.clear();
                }
                gherkin_tags.clear();
                // タグを解析
                for part in trimmed.split_whitespace() {
                    if let Some(eq_pos) = part.find('=') {
                        let tag_name = &part[..eq_pos];
                        let tag_value = &part[eq_pos + 1..];
                        gherkin_tags.push((tag_name.to_string(), tag_value.to_string()));
                    }
                }
            } else if trimmed.starts_with("Scenario:") {
                gherkin_scenario_line = Some(line_num);
            } else if trimmed.starts_with("Given ")
                || trimmed.starts_with("When ")
                || trimmed.starts_with("Then ")
                || trimmed.starts_with("And ")
                || trimmed.starts_with("But ")
            {
                gherkin_steps.push((line_num, trimmed.to_string()));
            }
            continue;
        }

        // 用語集の表の解析
        if kind == DocKind::Glossary {
            if line.starts_with('|') {
                if !glossary_header_seen {
                    // ヘッダー行をチェック
                    if line.contains("用語") && line.contains("意味") && line.contains("出典") {
                        glossary_header_seen = true;
                    }
                    continue;
                }
                // 区切り行（|---|---|---|）をスキップ
                let trimmed = line.trim().trim_matches('|').trim();
                if trimmed.chars().all(|c| c == '-' || c == '|' || c == ' ') {
                    in_glossary_table = true;
                    continue;
                }
                if in_glossary_table {
                    // 用語行を解析
                    let cols: Vec<&str> = line.split('|').collect();
                    if cols.len() >= 4 {
                        let term = cols[1].trim().to_string();
                        let meaning = cols[2].trim().to_string();
                        let sources_str = cols[3].trim();
                        let sources: Vec<String> = if sources_str.is_empty() {
                            vec![]
                        } else {
                            sources_str
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect()
                        };
                        if !term.is_empty() {
                            items.push(Item::GlossaryTerm {
                                term,
                                meaning,
                                sources,
                                line: line_num,
                            });
                        }
                    }
                }
                continue;
            }
        }

        // 題名（# ）
        if line.starts_with("# ") {
            let title_text = line[2..].trim().to_string();
            if title.is_none() {
                title = Some((line_num, title_text));
            } else {
                extra_titles.push((line_num, title_text));
            }
            continue;
        }

        // ## セクション
        if line.starts_with("## ") {
            // 現在の項目を完了させる
            if let Some(builder) = current_item.take() {
                items.push(builder.build());
            }
            let section_name = line[3..].trim().to_string();
            sections.push((line_num, section_name));
            found_first_section = true;
            continue;
        }

        // ### 見出し（項目）
        if line.starts_with("### ") {
            // 現在の項目を完了させる
            if let Some(builder) = current_item.take() {
                items.push(builder.build());
            }
            let heading_text = line[4..].trim().to_string();
            current_item = Some(ItemBuilder::new(heading_text, line_num, kind));
            found_first_section = true;
            continue;
        }

        // 範囲の行: 題名の後、最初の ## か ### より前の空でない行
        if title.is_some() && !found_first_section && !line.trim().is_empty() {
            scope_lines.push((line_num, line.trim().to_string()));
            continue;
        }

        // 項目の中の行
        if let Some(builder) = current_item.as_mut() {
            builder.add_line(line_num, line);
        }
    }

    // 最後の項目を完了させる
    if let Some(builder) = current_item.take() {
        items.push(builder.build());
    }

    IrDocument {
        filename: filename.to_string(),
        kind,
        title,
        extra_titles,
        scope_lines,
        line_count,
        items,
        sections,
        raw_content: content.to_string(),
    }
}

/// 項目の組み立て
struct ItemBuilder {
    heading: String,
    line: usize,
    field_lines: Vec<(usize, String, String)>, // (line, name, value)
    statement_lines: Vec<(usize, String)>,
    has_table: bool,
}

impl ItemBuilder {
    fn new(heading: String, line: usize, _doc_kind: DocKind) -> Self {
        ItemBuilder {
            heading,
            line,
            field_lines: Vec::new(),
            statement_lines: Vec::new(),
            has_table: false,
        }
    }

    fn add_line(&mut self, line_num: usize, line: &str) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return;
        }
        if trimmed.starts_with("- ") {
            // フィールド行
            let field_content = &trimmed[2..];
            if let Some(colon_pos) = field_content.find(':') {
                let name = field_content[..colon_pos].trim().to_string();
                let value = field_content[colon_pos + 1..].trim().to_string();
                self.field_lines.push((line_num, name, value));
            } else {
                // "xxx:" の形でない "- " 行
                self.field_lines
                    .push((line_num, String::new(), field_content.to_string()));
            }
        } else if trimmed.starts_with('|') {
            self.has_table = true;
        } else {
            // 文（statement）
            self.statement_lines.push((line_num, trimmed.to_string()));
        }
    }

    /// field_lines から fields_seen を構築する。
    /// name が空の場合は "- {value}" を入れる（行の文字として）。
    fn build_fields_seen(&self) -> Vec<(usize, String)> {
        self.field_lines
            .iter()
            .map(|(ln, name, value)| {
                if name.is_empty() {
                    (*ln, format!("- {}", value))
                } else {
                    (*ln, name.clone())
                }
            })
            .collect()
    }

    fn build(self) -> Item {
        // 見出しの形: "ID: 名前"
        let (id, name) = parse_heading(&self.heading);
        let fields_seen = self.build_fields_seen();

        match id_prefix(&id) {
            Some(IdPrefix::Req) => {
                let mut kind = None;
                let mut sources = Vec::new();
                let mut verification = None;
                let mut definitions = Vec::new();

                for (_, field_name, value) in &self.field_lines {
                    match field_name.as_str() {
                        "種類" => kind = Some(value.clone()),
                        "出典" => {
                            sources = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                        "検証" => verification = Some(value.clone()),
                        "定義" => {
                            definitions = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                        _ => {}
                    }
                }

                Item::Requirement {
                    id,
                    name,
                    line: self.line,
                    kind,
                    sources,
                    verification,
                    definitions,
                    statements: self.statement_lines,
                    fields_seen,
                }
            }
            Some(IdPrefix::Tbl) => {
                let mut sources = Vec::new();

                for (_, field_name, value) in &self.field_lines {
                    if field_name == "出典" {
                        sources = value
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }

                Item::DecisionTable {
                    id,
                    name,
                    line: self.line,
                    sources,
                    has_table: self.has_table,
                    fields_seen,
                }
            }
            Some(IdPrefix::Prop) => {
                let mut sources = Vec::new();
                let mut definitions = Vec::new();

                for (_, field_name, value) in &self.field_lines {
                    match field_name.as_str() {
                        "出典" => {
                            sources = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                        "定義" => {
                            definitions = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                        _ => {}
                    }
                }

                Item::Property {
                    id,
                    name,
                    line: self.line,
                    sources,
                    definitions,
                    statements: self.statement_lines,
                    fields_seen,
                }
            }
            Some(IdPrefix::Flag) => {
                let mut kind = None;
                let mut relations = Vec::new();
                let mut sources = Vec::new();

                for (_, field_name, value) in &self.field_lines {
                    match field_name.as_str() {
                        "種類" => kind = Some(value.clone()),
                        "関係" => {
                            relations = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                        "出典" => {
                            sources = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                        }
                        _ => {}
                    }
                }

                Item::FlagEntry {
                    id,
                    name,
                    line: self.line,
                    kind,
                    relations,
                    sources,
                    body: self.statement_lines,
                    fields_seen,
                }
            }
            _ => {
                // 認識できない見出し → 後で unknown_heading として報告
                let heading_text = if name.is_empty() {
                    id
                } else {
                    format!("{}: {}", id, name)
                };
                Item::UnknownHeading {
                    heading: heading_text,
                    line: self.line,
                }
            }
        }
    }
}

/// 見出し "ID: 名前" を分解する
fn parse_heading(heading: &str) -> (String, String) {
    if let Some(colon_pos) = heading.find(':') {
        let id = heading[..colon_pos].trim().to_string();
        let name = heading[colon_pos + 1..].trim().to_string();
        (id, name)
    } else {
        (heading.to_string(), String::new())
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

/// ID の形式を検証する（接頭辞 + 3桁の数字）
pub fn is_valid_id(s: &str) -> bool {
    if let Some(prefix) = id_prefix(s) {
        let suffix = match prefix {
            IdPrefix::Req => &s[4..],
            IdPrefix::Tbl => &s[4..],
            IdPrefix::Prop => &s[5..],
            IdPrefix::Ex => &s[3..],
            IdPrefix::Flag => &s[5..],
        };
        suffix.len() == 3 && suffix.chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}

/// gherkin のシナリオを組み立てる
fn build_scenario(
    tags: &[(String, String)],
    scenario_line: usize,
    steps: &[(usize, String)],
) -> Item {
    let mut id = None;
    let mut about = Vec::new();
    let mut sources = Vec::new();

    for (tag_name, tag_value) in tags {
        match tag_name.as_str() {
            "@id" => id = Some(tag_value.clone()),
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
        tags: tags.to_vec(),
        about,
        sources,
        steps: steps.to_vec(),
    }
}

/// 文書の検査をして指摘を返す
pub fn check_documents(docs: &[IrDocument], config: &Config) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 全 ID を収集して重複を検出
    let mut all_ids: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new(); // id -> [(path, line)]

    for doc in docs {
        let path = format!("{}/{}", config.ir, doc.filename);

        // REQ-034: 題名が無い
        if doc.title.is_none() {
            findings.push(Finding {
                kind: "missing_title".to_string(),
                severity: "error".to_string(),
                path: path.clone(),
                line: None,
                detail: doc.filename.clone(),
            });
        }

        // REQ-035: 題名が複数
        for (_, title_text) in &doc.extra_titles {
            findings.push(Finding {
                kind: "multiple_titles".to_string(),
                severity: "error".to_string(),
                path: path.clone(),
                line: None,
                detail: title_text.clone(),
            });
        }

        // REQ-036: 範囲の行が無い（話題ごとの文書のみ）
        if doc.kind == DocKind::Topic && doc.scope_lines.is_empty() {
            findings.push(Finding {
                kind: "missing_scope".to_string(),
                severity: "error".to_string(),
                path: path.clone(),
                line: None,
                detail: doc.filename.clone(),
            });
        }

        // REQ-038: 行数の上限
        let limit_lines = config.limits.lines.get() as usize;
        if doc.line_count > limit_lines {
            findings.push(Finding {
                kind: "too_many_lines".to_string(),
                severity: "warning".to_string(),
                path: path.clone(),
                line: None,
                detail: doc.line_count.to_string(),
            });
        }

        // REQ-039: 要求の数の上限（用語集と問題の記録を除く）
        if doc.kind == DocKind::Topic {
            let req_count = doc
                .items
                .iter()
                .filter(|item| matches!(item, Item::Requirement { .. }))
                .count();
            let limit_reqs = config.limits.requirements.get() as usize;
            if req_count > limit_reqs {
                findings.push(Finding {
                    kind: "too_many_requirements".to_string(),
                    severity: "warning".to_string(),
                    path: path.clone(),
                    line: None,
                    detail: req_count.to_string(),
                });
            }
        }

        // 項目の検査
        for item in &doc.items {
            check_item(item, &path, doc.kind, &mut findings);

            // ID を収集
            if let Some(id) = item.id() {
                all_ids
                    .entry(id.to_string())
                    .or_default()
                    .push((path.clone(), item.item_line()));
            }
        }

        // 見出しの形の検査
        for item in &doc.items {
            match item {
                Item::Requirement { id, name, line, .. }
                | Item::DecisionTable { id, name, line, .. }
                | Item::Property { id, name, line, .. }
                | Item::FlagEntry { id, name, line, .. } => {
                    if !is_valid_id(id) {
                        let heading_text = if name.is_empty() {
                            id.clone()
                        } else {
                            format!("{}: {}", id, name)
                        };
                        findings.push(Finding {
                            kind: "unknown_heading".to_string(),
                            severity: "error".to_string(),
                            path: path.clone(),
                            line: Some(*line),
                            detail: heading_text,
                        });
                    }
                }
                Item::UnknownHeading { heading, line } => {
                    findings.push(Finding {
                        kind: "unknown_heading".to_string(),
                        severity: "error".to_string(),
                        path: path.clone(),
                        line: Some(*line),
                        detail: heading.clone(),
                    });
                }
                _ => {}
            }
        }
    }

    // REQ-032: ID の重複
    for (id, locations) in &all_ids {
        if locations.len() > 1 {
            // 2つ目以降の場所に指摘
            for (path, line) in &locations[1..] {
                findings.push(Finding {
                    kind: "duplicate_id".to_string(),
                    severity: "error".to_string(),
                    path: path.clone(),
                    line: Some(*line),
                    detail: id.clone(),
                });
            }
        }
    }

    // 参照の解決チェック（REQ-054）
    let known_ids: BTreeSet<String> = all_ids.keys().cloned().collect();
    for doc in docs {
        let path = format!("{}/{}", config.ir, doc.filename);
        check_references(&doc.items, &known_ids, &path, &mut findings);
    }

    findings
}

/// 項目の検査
fn check_item(item: &Item, path: &str, _doc_kind: DocKind, findings: &mut Vec<Finding>) {
    match item {
        Item::Requirement {
            id,
            line,
            kind,
            sources,
            verification,
            definitions,
            statements,
            fields_seen,
            ..
        } => {
            if !is_valid_id(id) {
                return; // unknown_heading で報告済み
            }

            // 既知のフィールド
            let known_fields = ["種類", "出典", "検証", "定義"];
            check_fields(fields_seen, &known_fields, path, *line, findings);

            // REQ-098: 必須の行
            if kind.is_none() && !fields_seen.iter().any(|(_, n)| n == "種類") {
                findings.push(Finding {
                    kind: "missing_field".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: "種類".to_string(),
                });
            }
            if sources.is_empty() {
                if fields_seen.iter().any(|(_, n)| n == "出典") {
                    // REQ-059: 出典の行があるが値が空
                    findings.push(Finding {
                        kind: "missing_source".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: id.clone(),
                    });
                } else {
                    // REQ-098: 出典の行がない
                    findings.push(Finding {
                        kind: "missing_field".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: "出典".to_string(),
                    });
                }
            }

            // REQ-048: 検証の行が無い
            if verification.is_none() && !fields_seen.iter().any(|(_, n)| n == "検証") {
                findings.push(Finding {
                    kind: "verification_missing".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: id.clone(),
                });
            }

            // REQ-049: 検証の値の誤り
            if let Some(v) = verification {
                if !["unit", "property", "proof", "review"].contains(&v.as_str()) {
                    findings.push(Finding {
                        kind: "verification_invalid".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: v.clone(),
                    });
                }
            }

            // REQ-050: 種類の値の誤り
            if let Some(k) = kind {
                let valid_kinds = [
                    "event_driven",
                    "state_driven",
                    "ubiquitous",
                    "prohibition",
                    "invariant",
                    "algorithm",
                ];
                if !valid_kinds.contains(&k.as_str()) {
                    findings.push(Finding {
                        kind: "unknown_kind".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: k.clone(),
                    });
                }
            }

            // REQ-047: 文が無い（algorithm 以外）
            if let Some(k) = kind {
                if k != "algorithm" && statements.is_empty() {
                    findings.push(Finding {
                        kind: "missing_statement".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: id.clone(),
                    });
                }
            }

            // REQ-051: algorithm に定義がない
            if let Some(k) = kind {
                if k == "algorithm" && definitions.is_empty() {
                    findings.push(Finding {
                        kind: "algorithm_without_definition".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: id.clone(),
                    });
                }
            }
        }

        Item::DecisionTable {
            id,
            line,
            sources,
            has_table,
            fields_seen,
            ..
        } => {
            if !is_valid_id(id) {
                return;
            }

            let known_fields = ["出典"];
            check_fields(fields_seen, &known_fields, path, *line, findings);

            // REQ-098/REQ-059: 出典が必須
            if sources.is_empty() {
                if fields_seen.iter().any(|(_, n)| n == "出典") {
                    findings.push(Finding {
                        kind: "missing_source".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: id.clone(),
                    });
                } else {
                    findings.push(Finding {
                        kind: "missing_field".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: "出典".to_string(),
                    });
                }
            }

            // REQ-099: 表が無い
            if !has_table {
                findings.push(Finding {
                    kind: "missing_table".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: id.clone(),
                });
            }
        }

        Item::Property {
            id,
            line,
            sources,
            statements,
            fields_seen,
            ..
        } => {
            if !is_valid_id(id) {
                return;
            }

            let known_fields = ["出典", "定義"];
            check_fields(fields_seen, &known_fields, path, *line, findings);

            // REQ-098/REQ-059: 出典が必須
            if sources.is_empty() {
                if fields_seen.iter().any(|(_, n)| n == "出典") {
                    findings.push(Finding {
                        kind: "missing_source".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: id.clone(),
                    });
                } else {
                    findings.push(Finding {
                        kind: "missing_field".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: "出典".to_string(),
                    });
                }
            }

            // REQ-047: 性質には文が必要
            if statements.is_empty() {
                findings.push(Finding {
                    kind: "missing_statement".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: id.clone(),
                });
            }
        }

        Item::Scenario {
            id,
            line,
            tags,
            sources,
            ..
        } => {
            // REQ-052: 知らないタグ
            for (tag_name, _) in tags {
                if !["@id", "@about", "@source"].contains(&tag_name.as_str()) {
                    findings.push(Finding {
                        kind: "unknown_tag".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: tag_name.clone(),
                    });
                }
            }

            // REQ-053: 無いタグ
            if !tags.iter().any(|(n, _)| n == "@id") {
                findings.push(Finding {
                    kind: "missing_tag".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: "@id".to_string(),
                });
            }
            if !tags.iter().any(|(n, _)| n == "@about") {
                findings.push(Finding {
                    kind: "missing_tag".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: "@about".to_string(),
                });
            }

            // REQ-059: シナリオの出典
            if sources.is_empty() && !tags.iter().any(|(n, _)| n == "@source") {
                findings.push(Finding {
                    kind: "missing_source".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: id.as_deref().unwrap_or("(no id)").to_string(),
                });
            }
        }

        Item::FlagEntry {
            id,
            line,
            kind,
            relations,
            sources,
            fields_seen,
            ..
        } => {
            if !is_valid_id(id) {
                return;
            }

            let known_fields = ["種類", "関係", "出典"];
            check_fields(fields_seen, &known_fields, path, *line, findings);

            // REQ-098: 必須の行
            if kind.is_none() && !fields_seen.iter().any(|(_, n)| n == "種類") {
                findings.push(Finding {
                    kind: "missing_field".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: "種類".to_string(),
                });
            }
            if relations.is_empty() && !fields_seen.iter().any(|(_, n)| n == "関係") {
                findings.push(Finding {
                    kind: "missing_field".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: "関係".to_string(),
                });
            }
            if sources.is_empty() {
                if fields_seen.iter().any(|(_, n)| n == "出典") {
                    findings.push(Finding {
                        kind: "missing_source".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: id.clone(),
                    });
                } else {
                    findings.push(Finding {
                        kind: "missing_field".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: "出典".to_string(),
                    });
                }
            }

            // REQ-050: 種類の値の誤り
            if let Some(k) = kind {
                if !["contradiction", "gap", "ambiguity"].contains(&k.as_str()) {
                    findings.push(Finding {
                        kind: "unknown_kind".to_string(),
                        severity: "error".to_string(),
                        path: path.to_string(),
                        line: Some(*line),
                        detail: k.clone(),
                    });
                }
            }
        }

        Item::GlossaryTerm {
            term,
            sources,
            line,
            ..
        } => {
            // REQ-059/REQ-060: 用語の出典が空
            if sources.is_empty()
                || (sources.len() == 1 && sources[0].is_empty())
            {
                findings.push(Finding {
                    kind: "missing_source".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(*line),
                    detail: term.clone(),
                });
            }
        }

        Item::UnknownHeading { .. } => {
            // unknown_heading は check_documents で報告済み
        }
    }
}

/// フィールドの検査（unknown_field, duplicate_field）
fn check_fields(
    fields_seen: &[(usize, String)],
    known_fields: &[&str],
    path: &str,
    _item_line: usize,
    findings: &mut Vec<Finding>,
) {
    let mut seen_names: HashMap<String, usize> = HashMap::new();

    for (ln, name) in fields_seen {
        if name.starts_with("- ") {
            // "xxx:" の形でない "- " 行: detail は行の文字
            findings.push(Finding {
                kind: "unknown_field".to_string(),
                severity: "error".to_string(),
                path: path.to_string(),
                line: Some(*ln),
                detail: name.clone(),
            });
            continue;
        }

        if !known_fields.contains(&name.as_str()) {
            findings.push(Finding {
                kind: "unknown_field".to_string(),
                severity: "error".to_string(),
                path: path.to_string(),
                line: Some(*ln),
                detail: name.clone(),
            });
        }

        // REQ-045: 同じ行の重複
        if let Some(_prev_line) = seen_names.get(name) {
            findings.push(Finding {
                kind: "duplicate_field".to_string(),
                severity: "error".to_string(),
                path: path.to_string(),
                line: Some(*ln),
                detail: name.clone(),
            });
        } else {
            seen_names.insert(name.clone(), *ln);
        }
    }
}

/// 参照の解決チェック
fn check_references(
    items: &[Item],
    known_ids: &BTreeSet<String>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    for item in items {
        match item {
            Item::Requirement {
                line,
                definitions,
                statements,
                ..
            } => {
                // 定義の参照チェック
                for def_id in definitions {
                    if !known_ids.contains(def_id) {
                        findings.push(Finding {
                            kind: "unresolved_reference".to_string(),
                            severity: "error".to_string(),
                            path: path.to_string(),
                            line: Some(*line),
                            detail: def_id.clone(),
                        });
                    }
                }
                // 文の中のバッククォートで囲んだ ID の参照チェック
                for (stmt_line, stmt) in statements {
                    check_backtick_ids(stmt, *stmt_line, known_ids, path, findings);
                }
            }
            Item::Property {
                line,
                definitions,
                statements,
                ..
            } => {
                for def_id in definitions {
                    if !known_ids.contains(def_id) {
                        findings.push(Finding {
                            kind: "unresolved_reference".to_string(),
                            severity: "error".to_string(),
                            path: path.to_string(),
                            line: Some(*line),
                            detail: def_id.clone(),
                        });
                    }
                }
                for (stmt_line, stmt) in statements {
                    check_backtick_ids(stmt, *stmt_line, known_ids, path, findings);
                }
            }
            Item::Scenario {
                line, about, ..
            } => {
                for about_id in about {
                    if !known_ids.contains(about_id) {
                        findings.push(Finding {
                            kind: "unresolved_reference".to_string(),
                            severity: "error".to_string(),
                            path: path.to_string(),
                            line: Some(*line),
                            detail: about_id.clone(),
                        });
                    }
                }
            }
            Item::FlagEntry {
                line, relations, ..
            } => {
                for rel_id in relations {
                    if !known_ids.contains(rel_id) {
                        findings.push(Finding {
                            kind: "unresolved_reference".to_string(),
                            severity: "error".to_string(),
                            path: path.to_string(),
                            line: Some(*line),
                            detail: rel_id.clone(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

/// 文の中のバッククォートで囲んだ ID の参照をチェック
fn check_backtick_ids(
    text: &str,
    line: usize,
    known_ids: &BTreeSet<String>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    let mut start = 0;
    while let Some(open) = text[start..].find('`') {
        let open_abs = start + open + 1;
        if open_abs >= text.len() {
            break;
        }
        if let Some(close) = text[open_abs..].find('`') {
            let content = &text[open_abs..open_abs + close];
            // ID の形なら参照チェック
            if is_valid_id(content) && !known_ids.contains(content) {
                findings.push(Finding {
                    kind: "unresolved_reference".to_string(),
                    severity: "error".to_string(),
                    path: path.to_string(),
                    line: Some(line),
                    detail: content.to_string(),
                });
            }
            start = open_abs + close + 1;
        } else {
            break;
        }
    }
}

/// IR のディレクトリからすべての文書を読んで検査する
pub fn load_and_check(
    base: &Path,
    config: &Config,
) -> Result<(Vec<IrDocument>, Vec<Finding>), crate::StopReason> {
    let ir_dir = base.join(&config.ir);

    let mut docs = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(&ir_dir)
        .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", ir_dir.display())))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "md")
                && e.file_type().is_ok_and(|ft| ft.is_file())
        })
        .collect();

    // ファイル名でソート（安定な順序）
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();
        let bytes = std::fs::read(&path)
            .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", path.display())))?;
        let content = String::from_utf8(bytes)
            .map_err(|_| crate::StopReason::NonUtf8File(format!("{}", path.display())))?;
        let doc = parse_document(&filename, &content);
        docs.push(doc);
    }

    let findings = check_documents(&docs, config);
    Ok((docs, findings))
}
