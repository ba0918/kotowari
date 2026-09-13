//! IR 文書の読み込みと形の検査

use crate::config::Config;
use crate::{Finding, FindingKind};
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
        fields_seen: Vec<(usize, String, String)>,
    },
    DecisionTable {
        id: String,
        name: String,
        line: usize,
        sources: Vec<String>,
        has_table: bool,
        fields_seen: Vec<(usize, String, String)>,
    },
    Property {
        id: String,
        name: String,
        line: usize,
        sources: Vec<String>,
        statements: Vec<(usize, String)>,
        fields_seen: Vec<(usize, String, String)>,
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
        sources: Vec<String>,
        body: Vec<(usize, String)>,
        fields_seen: Vec<(usize, String, String)>,
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
    /// 解析中に見つかった指摘（この段では常に空）
    pub parse_findings: Vec<crate::Finding>,
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
    // BOM の読み飛ばし（read_utf8_file でも除去するが、直接呼ばれた場合にも対応）
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(content);
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

    let mut current_fence: Option<(CodeFence, usize, String)> = None; // (fence, opening_line, raw_line)
    let mut in_gherkin_block = false;
    let mut found_first_section = false;
    let mut parse_findings: Vec<crate::Finding> = Vec::new();

    // 現在の項目の解析状態
    let mut current_item: Option<ItemBuilder> = None;

    // gherkin ブロック内の状態
    let mut gherkin_tags: Vec<(String, String)> = Vec::new();
    let mut gherkin_tag_line: Option<usize> = None;
    let mut gherkin_scenario_line: Option<usize> = None;
    let mut gherkin_scenario_text: String = String::new();
    let mut gherkin_steps: Vec<(usize, String)> = Vec::new();
    let mut gherkin_prev_was_tag: bool = false;

    // 用語集の解析
    let mut in_glossary_table = false;
    let mut glossary_header_seen = false;
    let mut glossary_table_done = false; // REQ-117: 2つ目の表は用語にしない
    let mut glossary_separator_seen = false;

    for (idx, line) in lines.iter().enumerate() {
        let line_num = idx + 1; // 1-indexed

        // コードブロックの開始/終了
        if let Some((ref fence, _, _)) = current_fence {
            if is_closing_fence(line, fence) {
                // ブロックの終了
                if in_gherkin_block {
                    if let Some(scenario_line) = gherkin_scenario_line.take() {
                        let scenario = build_scenario(&gherkin_tags, gherkin_tag_line, scenario_line, &gherkin_steps, &gherkin_scenario_text);
                        items.push(scenario);
                    } else if !gherkin_tags.is_empty() {
                        // シナリオに結び付かなかったタグの検査
                        check_gherkin_tags_findings(
                            &gherkin_tags, gherkin_tag_line, &mut parse_findings,
                        );
                    }
                    gherkin_tags.clear();
                    gherkin_tag_line = None;
                    gherkin_steps.clear();
                    gherkin_scenario_text.clear();
                    gherkin_prev_was_tag = false;
                }
                current_fence = None;
                in_gherkin_block = false;
                continue;
            }
        } else if let Some(fence) = parse_opening_fence(line) {
            // ブロックの開始
            let is_gherkin = fence.lang == "gherkin";
            current_fence = Some((fence, line_num, line.to_string()));
            if is_gherkin {
                in_gherkin_block = true;
            }
            continue;
        }

        // コードブロック内（gherkin 以外）はスキップ
        if current_fence.is_some() && !in_gherkin_block {
            continue;
        }

        // gherkin ブロック内の処理
        if in_gherkin_block {
            let trimmed = line.trim();
            if trimmed.starts_with('@') {
                // タグ行: 前のシナリオがあれば追加
                if let Some(scenario_line) = gherkin_scenario_line.take() {
                    let scenario = build_scenario(&gherkin_tags, gherkin_tag_line, scenario_line, &gherkin_steps, &gherkin_scenario_text);
                    items.push(scenario);
                    gherkin_steps.clear();
                }
                gherkin_tags.clear();
                gherkin_tag_line = Some(line_num);
                // タグを解析
                for part in trimmed.split_whitespace() {
                    if let Some(eq_pos) = part.find('=') {
                        let tag_name = &part[..eq_pos];
                        let tag_value = &part[eq_pos + 1..];
                        gherkin_tags.push((tag_name.to_string(), tag_value.to_string()));
                    } else if part.starts_with('@') {
                        // = のない裸のタグ（@wip 等）
                        gherkin_tags.push((part.to_string(), String::new()));
                    } else {
                        // REQ-052: "@" で始まらない語は unknown_tag
                        gherkin_tags.push((String::new(), part.to_string()));
                    }
                }
                gherkin_prev_was_tag = true;
            } else if trimmed.starts_with("Scenario:") {
                // 前のシナリオがあればフラッシュ
                if let Some(scenario_line) = gherkin_scenario_line.take() {
                    let scenario = build_scenario(&gherkin_tags, gherkin_tag_line, scenario_line, &gherkin_steps, &gherkin_scenario_text);
                    items.push(scenario);
                    gherkin_steps.clear();
                }
                // REQ-113: タグは直前の行だけ結び付ける
                if !gherkin_prev_was_tag {
                    // 結び付かなかったタグの行の検査（REQ-052: 結び付くかを問わない）
                    if !gherkin_tags.is_empty() {
                        check_gherkin_tags_findings(
                            &gherkin_tags, gherkin_tag_line, &mut parse_findings,
                        );
                    }
                    gherkin_tags.clear();
                    gherkin_tag_line = None;
                }
                gherkin_scenario_line = Some(line_num);
                gherkin_scenario_text = trimmed.to_string();
                gherkin_prev_was_tag = false;
            } else if trimmed.starts_with("Given ")
                || trimmed.starts_with("When ")
                || trimmed.starts_with("Then ")
                || trimmed.starts_with("And ")
                || trimmed.starts_with("But ")
            {
                gherkin_steps.push((line_num, trimmed.to_string()));
                gherkin_prev_was_tag = false;
            } else if trimmed.starts_with('#') || trimmed.is_empty() {
                // REQ-113: 注釈と空行は有効
                gherkin_prev_was_tag = false;
            } else {
                // REQ-113: それ以外は invalid_gherkin_line
                parse_findings.push(crate::Finding::new(
                    crate::FindingKind::InvalidGherkinLine,
                    String::new(),
                    Some(line_num),
                    trimmed.to_string(),
                ));
                gherkin_prev_was_tag = false;
            }
            continue;
        }

        // 用語集の表の解析
        if kind == DocKind::Glossary {
            if line.starts_with('|') {
                // REQ-117: 表が完了していたら2つ目以降の表は無視
                if glossary_table_done {
                    continue;
                }
                if !glossary_header_seen {
                    // ヘッダー行をチェック（各セルの前後の空白を除いて完全一致）
                    let cols: Vec<&str> = line.split('|').collect();
                    let trimmed_cols: Vec<&str> = cols.iter()
                        .map(|c| c.trim())
                        .filter(|c| !c.is_empty())
                        .collect();
                    if trimmed_cols == ["用語", "意味", "出典"] {
                        glossary_header_seen = true;
                    }
                    continue;
                }
                if !glossary_separator_seen {
                    // 区切り行: 各セルが3つ以上の "-"（前後に ":" があってもよい）
                    let cols: Vec<&str> = line.split('|').collect();
                    let is_separator = cols.iter()
                        .filter(|c| !c.trim().is_empty())
                        .all(|c| {
                            let t = c.trim().trim_matches(':');
                            t.len() >= 3 && t.chars().all(|ch| ch == '-')
                        });
                    if is_separator {
                        glossary_separator_seen = true;
                        in_glossary_table = true;
                    }
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
                                .filter(|s| !s.is_empty())
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
            } else if in_glossary_table {
                // 空行か表でない行で表が終わる
                in_glossary_table = false;
                glossary_table_done = true;
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

        // REQ-043: #### より深い見出しは unknown_heading
        if line.starts_with("#### ") {
            // 現在の項目を完了させる
            if let Some(builder) = current_item.take() {
                items.push(builder.build());
            }
            let heading_text = line.trim_start_matches('#').trim().to_string();
            items.push(Item::UnknownHeading {
                heading: heading_text,
                line: line_num,
            });
            // この見出しの下の行は項目として読まない
            current_item = None;
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

    // REQ-112: 閉じないコードブロック
    if let Some((_fence, opening_line, raw_line)) = current_fence {
        parse_findings.push(crate::Finding::new(
            crate::FindingKind::UnclosedCodeBlock,
            String::new(), // path は呼び出し元が設定する
            Some(opening_line),
            raw_line,
        ));
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
        parse_findings,
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

        // REQ-044: "* ", "+ ", 数字+". ", "-" だけの行は unknown_field
        if trimmed.starts_with("* ") || trimmed.starts_with("+ ") || trimmed == "-" {
            // detail は読んだ行そのまま（字下げを含む）
            self.field_lines.push((line_num, String::new(), line.to_string()));
            return;
        }
        // 数字 + ". " で始まる行（例: "1. xxx"）
        if let Some(dot_pos) = trimmed.find(". ") {
            if dot_pos > 0 && trimmed[..dot_pos].chars().all(|c| c.is_ascii_digit()) {
                self.field_lines.push((line_num, String::new(), line.to_string()));
                return;
            }
        }

        if trimmed.starts_with("- ") {
            // フィールド行
            let field_content = &trimmed[2..];
            if let Some(colon_pos) = field_content.find(':') {
                let name = field_content[..colon_pos].trim().to_string();
                let value = field_content[colon_pos + 1..].trim().to_string();
                self.field_lines.push((line_num, name, value));
            } else {
                // "xxx:" の形でない "- " 行: detail は読んだ行そのまま
                self.field_lines
                    .push((line_num, String::new(), line.to_string()));
            }
        } else if trimmed.starts_with('|') {
            self.has_table = true;
        } else {
            // 文（statement）
            self.statement_lines.push((line_num, trimmed.to_string()));
        }
    }

    /// field_lines から fields_seen を構築する。
    /// (行番号, フィールド名, 行の文字) の三つ組を返す。
    /// name が空の場合は value が生の行（字下げ含む）をそのまま持つ。
    fn build_fields_seen(&self) -> Vec<(usize, String, String)> {
        self.field_lines
            .iter()
            .map(|(ln, name, value)| {
                if name.is_empty() {
                    // value にはもう生の行が入っている
                    (*ln, String::new(), value.clone())
                } else {
                    (*ln, name.clone(), format!("- {}: {}", name, value))
                }
            })
            .collect()
    }

    fn build(self) -> Item {
        // 見出しの形: "ID: 名前"（コロン必須）
        let Some((id, name)) = parse_heading(&self.heading) else {
            return Item::UnknownHeading {
                heading: self.heading,
                line: self.line,
            };
        };
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

                for (_, field_name, value) in &self.field_lines {
                    if field_name == "出典" {
                        sources = value
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }

                Item::Property {
                    id,
                    name,
                    line: self.line,
                    sources,
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

/// 見出し "ID: 名前" を分解する。コロンがなければ None。
fn parse_heading(heading: &str) -> Option<(String, String)> {
    let (id, name) = heading.split_once(':')?;
    Some((id.trim().to_string(), name.trim().to_string()))
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

/// 結び付かなかったタグの行を検査する（REQ-052: 結び付くかを問わない）
fn check_gherkin_tags_findings(
    tags: &[(String, String)],
    tag_line: Option<usize>,
    findings: &mut Vec<crate::Finding>,
) {
    let line = tag_line.unwrap_or(0);
    for (tag_name, tag_value) in tags {
        if tag_name.is_empty() {
            // "@" で始まらない語
            findings.push(crate::Finding::new(
                crate::FindingKind::UnknownTag,
                String::new(),
                if line > 0 { Some(line) } else { None },
                tag_value.clone(),
            ));
        } else if !["@id", "@about", "@source"].contains(&tag_name.as_str()) {
            findings.push(crate::Finding::new(
                crate::FindingKind::UnknownTag,
                String::new(),
                if line > 0 { Some(line) } else { None },
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
            "@id" if !tag_value.is_empty() => {
                // REQ-114: @id の値が EX の ID の形でないときは定義に数えない
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

/// 文書の検査をして指摘を返す
pub fn check_documents(docs: &[IrDocument], config: &Config) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 全 ID を収集して重複を検出
    let mut all_ids: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new(); // id -> [(path, line)]

    for doc in docs {
        let path = format!("{}/{}", config.ir, doc.filename);

        // REQ-034: 題名が無い
        if doc.title.is_none() {
            findings.push(Finding::new(FindingKind::MissingTitle, path.clone(), None, doc.filename.clone()));
        }

        // REQ-035: 題名が複数
        for (_, title_text) in &doc.extra_titles {
            findings.push(Finding::new(FindingKind::MultipleTitles, path.clone(), None, title_text.clone()));
        }

        // REQ-036: 範囲の行が無い（話題ごとの文書のみ）
        if doc.kind == DocKind::Topic && doc.scope_lines.is_empty() {
            findings.push(Finding::new(FindingKind::MissingScope, path.clone(), None, doc.filename.clone()));
        }

        // REQ-117: 用語集にこの形の表がない → glossary_invalid
        if doc.kind == DocKind::Glossary {
            let has_glossary_term = doc.items.iter().any(|i| matches!(i, Item::GlossaryTerm { .. }));
            if !has_glossary_term {
                findings.push(Finding::new(FindingKind::GlossaryInvalid, path.clone(), None, doc.filename.clone()));
            }
        }

        // REQ-038: 行数の上限
        let limit_lines = config.limits.lines.get() as usize;
        if doc.line_count > limit_lines {
            findings.push(Finding::new(FindingKind::TooManyLines, path.clone(), None, doc.line_count.to_string()));
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
                findings.push(Finding::new(FindingKind::TooManyRequirements, path.clone(), None, req_count.to_string()));
            }
        }

        // 項目の検査
        for item in &doc.items {
            check_item(item, &path, doc.kind, &mut findings);

            // ID を収集（形に合う ID だけ。REQ-114）
            if let Some(id) = item.id() {
                if is_valid_id(id) {
                    all_ids
                        .entry(id.to_string())
                        .or_default()
                        .push((path.clone(), item.item_line()));
                }
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
                        findings.push(Finding::new(FindingKind::UnknownHeading, path.clone(), Some(*line), heading_text));
                    }
                }
                Item::UnknownHeading { heading, line } => {
                    findings.push(Finding::new(FindingKind::UnknownHeading, path.clone(), Some(*line), heading.clone()));
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
                findings.push(Finding::new(FindingKind::DuplicateId, path.clone(), Some(*line), id.clone()));
            }
        }
    }

    // 参照の解決チェック（REQ-054）
    let known_ids = crate::collect_known_ids(docs);
    for doc in docs {
        let path = format!("{}/{}", config.ir, doc.filename);
        check_references(&doc.items, &known_ids, &path, &mut findings);
    }

    // parse_findings を統合（check() ヘルパーからも見えるようにする）
    for doc in docs {
        let doc_path = format!("{}/{}", config.ir, doc.filename);
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
            if kind.is_none() && !fields_seen.iter().any(|(_, n, _)| n == "種類") {
                findings.push(Finding::new(FindingKind::MissingField, path.to_string(), Some(*line), "種類".to_string()));
            }
            if sources.is_empty() {
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), id.clone()));
            }

            // REQ-048: 検証の行が無い
            if verification.is_none() && !fields_seen.iter().any(|(_, n, _)| n == "検証") {
                findings.push(Finding::new(FindingKind::VerificationMissing, path.to_string(), Some(*line), id.clone()));
            }

            // REQ-049: 検証の値の誤り
            if let Some(v) = verification {
                if !["unit", "property", "proof", "review"].contains(&v.as_str()) {
                    findings.push(Finding::new(FindingKind::VerificationInvalid, path.to_string(), Some(*line), v.clone()));
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
                    findings.push(Finding::new(FindingKind::UnknownKind, path.to_string(), Some(*line), k.clone()));
                }
            }

            // REQ-047: 文が無い（algorithm 以外。種類の行が無い要求を含む）
            let is_algorithm = kind.as_deref() == Some("algorithm");
            if !is_algorithm && statements.is_empty() {
                findings.push(Finding::new(FindingKind::MissingStatement, path.to_string(), Some(*line), id.clone()));
            }

            // REQ-051: algorithm に決定表か性質を指す定義がない
            if let Some(k) = kind {
                let has_tbl_or_prop_def = definitions.iter().any(|d| {
                    matches!(id_prefix(d), Some(IdPrefix::Tbl) | Some(IdPrefix::Prop))
                });
                if k == "algorithm" && !has_tbl_or_prop_def {
                    findings.push(Finding::new(FindingKind::AlgorithmWithoutDefinition, path.to_string(), Some(*line), id.clone()));
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
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), id.clone()));
            }

            // REQ-099: 表が無い
            if !has_table {
                findings.push(Finding::new(FindingKind::MissingTable, path.to_string(), Some(*line), id.clone()));
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

            let known_fields = ["出典"];
            check_fields(fields_seen, &known_fields, path, *line, findings);

            // REQ-098/REQ-059: 出典が必須
            if sources.is_empty() {
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), id.clone()));
            }

            // REQ-047: 性質には文が必要
            if statements.is_empty() {
                findings.push(Finding::new(FindingKind::MissingStatement, path.to_string(), Some(*line), id.clone()));
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
            // TBL-019: タグの行（無ければ Scenario: の行）
            let tag_or_scenario_line = tag_line.unwrap_or(*line);

            // REQ-052: 知らないタグ（結び付くかを問わない）
            for (tag_name, tag_value) in tags {
                if tag_name.is_empty() {
                    // "@" で始まらない語
                    findings.push(Finding::new(FindingKind::UnknownTag, path.to_string(), Some(tag_or_scenario_line), tag_value.clone()));
                } else if !["@id", "@about", "@source"].contains(&tag_name.as_str()) {
                    findings.push(Finding::new(FindingKind::UnknownTag, path.to_string(), Some(tag_or_scenario_line), tag_name.clone()));
                }
            }

            // REQ-114: @id の値が EX の ID の形でないとき
            let has_malformed_id = tags.iter().any(|(n, v)| {
                n == "@id" && !v.is_empty() && !(is_valid_id(v) && id_prefix(v) == Some(IdPrefix::Ex))
            });
            if has_malformed_id {
                let malformed_value = tags.iter()
                    .find(|(n, v)| n == "@id" && !v.is_empty() && !(is_valid_id(v) && id_prefix(v) == Some(IdPrefix::Ex)))
                    .map(|(_, v)| v.clone())
                    .unwrap();
                findings.push(Finding::new(FindingKind::InvalidId, path.to_string(), Some(tag_or_scenario_line), malformed_value));
                // missing_tag は出さない、missing_source の detail は Scenario: の行の文字
            } else {
                // REQ-053: 無いタグ
                if !tags.iter().any(|(n, v)| n == "@id" && !v.is_empty()) {
                    findings.push(Finding::new(FindingKind::MissingTag, path.to_string(), Some(tag_or_scenario_line), "@id".to_string()));
                }
            }
            if !tags.iter().any(|(n, v)| n == "@about" && !v.is_empty()) {
                findings.push(Finding::new(FindingKind::MissingTag, path.to_string(), Some(tag_or_scenario_line), "@about".to_string()));
            }

            // REQ-059: シナリオの出典
            if sources.is_empty() && !tags.iter().any(|(n, v)| n == "@source" && !v.is_empty()) {
                // REQ-114: @id が形に合わないときは Scenario: の行の文字を使う
                let detail = if has_malformed_id {
                    scenario_text.clone()
                } else {
                    id.as_deref().unwrap_or(scenario_text).to_string()
                };
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(tag_or_scenario_line), detail));
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
            if kind.is_none() && !fields_seen.iter().any(|(_, n, _)| n == "種類") {
                findings.push(Finding::new(FindingKind::MissingField, path.to_string(), Some(*line), "種類".to_string()));
            }
            if relations.is_empty() && !fields_seen.iter().any(|(_, n, _)| n == "関係") {
                findings.push(Finding::new(FindingKind::MissingField, path.to_string(), Some(*line), "関係".to_string()));
            }
            if sources.is_empty() {
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), id.clone()));
            }

            // REQ-050: 種類の値の誤り
            if let Some(k) = kind {
                if !["contradiction", "gap", "ambiguity"].contains(&k.as_str()) {
                    findings.push(Finding::new(FindingKind::UnknownKind, path.to_string(), Some(*line), k.clone()));
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
            if sources.is_empty() {
                findings.push(Finding::new(FindingKind::MissingSource, path.to_string(), Some(*line), term.clone()));
            }
        }

        Item::UnknownHeading { .. } => {
            // unknown_heading は check_documents で報告済み
        }
    }
}

/// フィールドの検査（unknown_field, duplicate_field）
fn check_fields(
    fields_seen: &[(usize, String, String)],
    known_fields: &[&str],
    path: &str,
    _item_line: usize,
    findings: &mut Vec<Finding>,
) {
    let mut seen_names: HashMap<String, usize> = HashMap::new();

    for (ln, name, raw) in fields_seen {
        if name.is_empty() {
            // "xxx:" の形でない "- " 行: detail は行の文字
            findings.push(Finding::new(FindingKind::UnknownField, path.to_string(), Some(*ln), raw.clone()));
            continue;
        }

        if !known_fields.contains(&name.as_str()) {
            // TBL-008: unknown_field の detail は行の文字
            // REQ-045: 知らない行の重複は unknown_field だけ
            findings.push(Finding::new(FindingKind::UnknownField, path.to_string(), Some(*ln), raw.clone()));
            continue;
        }

        // REQ-045: 同じ知っている行の重複（TBL-008: duplicate_field の detail は行の名前）
        if let Some(_prev_line) = seen_names.get(name) {
            findings.push(Finding::new(FindingKind::DuplicateField, path.to_string(), Some(*ln), name.clone()));
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
                fields_seen,
                ..
            } => {
                // 定義の参照チェック
                for def_id in definitions {
                    if !is_valid_id(def_id) {
                        // REQ-054: ID の形でない値は unresolved_reference
                        // 定義の行を探す
                        let def_line = fields_seen.iter()
                            .find(|(_, n, _)| n == "定義")
                            .map(|(ln, _, _)| *ln)
                            .unwrap_or(*line);
                        findings.push(Finding::new(FindingKind::UnresolvedReference, path.to_string(), Some(def_line), def_id.clone()));
                    } else if !known_ids.contains(def_id) {
                        findings.push(Finding::new(FindingKind::UnresolvedReference, path.to_string(), Some(*line), def_id.clone()));
                    }
                }
                // 文の中のバッククォートで囲んだ ID の参照チェック
                for (stmt_line, stmt) in statements {
                    check_backtick_ids(stmt, *stmt_line, known_ids, path, findings);
                }
            }
            Item::Property { statements, .. } => {
                for (stmt_line, stmt) in statements {
                    check_backtick_ids(stmt, *stmt_line, known_ids, path, findings);
                }
            }
            Item::Scenario {
                line, tag_line, about, steps, ..
            } => {
                let about_line = tag_line.unwrap_or(*line);
                for about_id in about {
                    if !known_ids.contains(about_id) {
                        findings.push(Finding::new(FindingKind::UnresolvedReference, path.to_string(), Some(about_line), about_id.clone()));
                    }
                }
                // REQ-054: ステップの行のバッククォートで囲んだ ID の参照チェック
                for (step_line, step_text) in steps {
                    check_backtick_ids(step_text, *step_line, known_ids, path, findings);
                }
            }
            Item::FlagEntry {
                line, relations, ..
            } => {
                for rel_id in relations {
                    if !known_ids.contains(rel_id) {
                        findings.push(Finding::new(FindingKind::UnresolvedReference, path.to_string(), Some(*line), rel_id.clone()));
                    }
                }
            }
            _ => {}
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

/// 行がコードブロックの境界かどうかを判定する（後方互換の簡易版）。
pub fn is_code_fence(line: &str) -> bool {
    parse_opening_fence(line).is_some()
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

/// 文の中のバッククォートで囲んだ ID の参照をチェック
fn check_backtick_ids(
    text: &str,
    line: usize,
    known_ids: &BTreeSet<String>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    // REQ-116: 奇数バッククォートの行では検査しない
    if text.chars().filter(|&c| c == '`').count() % 2 != 0 {
        return;
    }
    for content in extract_backtick_contents(text) {
        let trimmed = content.trim();
        if !trimmed.is_empty() && is_valid_id(trimmed) && !known_ids.contains(trimmed) {
            findings.push(Finding::new(FindingKind::UnresolvedReference, path.to_string(), Some(line), trimmed.to_string()));
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
    let mut raw_entries = Vec::new();
    for entry in std::fs::read_dir(&ir_dir)
        .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", config.ir)))?
    {
        let entry = entry.map_err(|e| {
            crate::StopReason::UnreadableFile(format!("{}: {e}", config.ir))
        })?;
        let p = entry.path();
        // REQ-033: .md（小文字）のファイルだけ読む。シンボリックリンクも辿る
        // 種類が取れないときは停止する（REQ-033）
        let is_file = std::fs::metadata(&p)
            .map(|m| m.is_file())
            .map_err(|e| {
                let rel = format!("{}/{}", config.ir, entry.file_name().to_string_lossy());
                crate::StopReason::UnreadableFile(format!("{rel}: {e}"))
            })?;
        if p.extension().is_some_and(|ext| ext == "md") && is_file {
            raw_entries.push(entry);
        }
    }

    // ファイル名でソート（安定な順序）
    raw_entries.sort_by_key(|e| e.file_name());

    for entry in raw_entries {
        let path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();
        let display = format!("{}/{}", config.ir, filename);
        let content = crate::read_utf8_file(&path, &display)?;
        let doc = parse_document(&filename, &content);
        docs.push(doc);
    }

    let findings = check_documents(&docs, config);

    Ok((docs, findings))
}
