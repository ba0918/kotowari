//! 判断の記録の読み取り（節・番号の行・補足の行・リンクの構造。REQ-core-133、REQ-core-135、REQ-core-136、TBL-core-022、TBL-core-023 の定数と走査）と、出典の検査（REQ-core-057〜REQ-core-061, REQ-core-106, TBL-core-012）

use crate::{Finding, FindingKind};
use std::path::Path;

/// `判断の記録` の `補足の行` の値にあるリンク（"[文字](href)"。TBL-core-023）
#[derive(Debug, Clone)]
pub struct RecordLink {
    /// "(" と ")" の間の href
    pub href: String,
}

/// `番号の行` に付く `補足の行`
#[derive(Debug, Clone)]
pub struct FieldLine {
    /// "- " の直後から最初の ":" までの名前
    pub name: String,
    /// ":" の後の前後の空白を除いた値
    pub value: String,
    /// 1始まりの行番号
    pub line: usize,
    /// 値の中のリンク
    pub links: Vec<RecordLink>,
}

/// 節の中の `番号の行`
#[derive(Debug, Clone)]
pub struct NumberedLine {
    /// 決定の番号（"A26" など）
    pub number: String,
    /// 1始まりの行番号
    pub line: usize,
    /// この行に付く `補足の行`
    pub fields: Vec<FieldLine>,
}

/// TBL-core-022 の表にある節
#[derive(Debug, Clone)]
pub struct RecordSection {
    /// "## " の後の見出し
    pub name: String,
    /// 節の中の `番号の行`
    pub numbered_lines: Vec<NumberedLine>,
}

/// 判断の記録のファイルの内容
#[derive(Debug)]
pub struct RecordsFile {
    /// ファイルのパス（decisions.records からの相対）
    pub rel_path: String,
    /// TBL-core-022 の表にある節（判断の記録でないファイルでは空）
    pub sections: Vec<RecordSection>,
    /// コードブロックの外の ## 見出し（前後の空白を除いた文字列。A47）
    pub headings: Vec<String>,
    /// 判断の記録かどうか（決定の節の見出しをコードブロックの外に持つか）
    pub is_records: bool,
    /// "## Context" の見出しをコードブロックの外に持つか（REQ-core-129）
    pub has_context: bool,
}

/// 決定の節の名前（用語集の `決定の節`）
pub const DECISION_SECTIONS: [&str; 4] = ["Agreements", "Prohibitions", "Delegated", "Rejected"];

/// TBL-core-022: 節と、その節の `番号の行` に必須の `補足の行` の名前
pub const REQUIRED_FIELDS: [(&str, &str); 6] = [
    ("Agreements", "why"),
    ("Prohibitions", "why"),
    ("Delegated", "why"),
    ("Rejected", "why"),
    ("Undecided", "decides"),
    ("Superseded", "superseded_by"),
];

/// TBL-core-022 が認める `補足の行` の名前
pub const KNOWN_FIELD_NAMES: [&str; 6] = [
    "why",
    "rejected",
    "decided_by",
    "superseded_by",
    "decides",
    "related",
];

/// TBL-core-022 の表にある節か（節の一覧はすべての判断の記録で使う。REQ-core-135）
pub fn required_field_of(heading: &str) -> Option<&'static str> {
    REQUIRED_FIELDS
        .iter()
        .find(|(name, _)| *name == heading)
        .map(|(_, field)| *field)
}

impl RecordsFile {
    /// 決定の節にその番号の `番号の行` があるか（TBL-core-012 の順2）
    pub fn has_decision_number(&self, number: &str) -> bool {
        self.sections
            .iter()
            .filter(|s| DECISION_SECTIONS.contains(&s.name.as_str()))
            .any(|s| s.numbered_lines.iter().any(|n| n.number == number))
    }

    /// 決定の節か Superseded の節にその番号の `番号の行` があるか（TBL-core-023 の順5）
    pub fn has_revision_target(&self, number: &str) -> bool {
        self.sections
            .iter()
            .filter(|s| DECISION_SECTIONS.contains(&s.name.as_str()) || s.name == "Superseded")
            .any(|s| s.numbered_lines.iter().any(|n| n.number == number))
    }

}

/// 決定の番号の形（英大文字1文字に1桁以上の数字。A26, P1, D1, R6 など）
pub fn is_decision_number(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }
    let bytes = s.as_bytes();
    let first = bytes[0];
    if !first.is_ascii_uppercase() {
        return false;
    }
    // 2文字目以降がすべて数字であること（英字が2文字以上続くのは不可）
    let rest = &s[1..];
    if rest.is_empty() {
        return false;
    }
    rest.chars().all(|c| c.is_ascii_digit())
}

/// `番号の行` の番号を取り出す（用語集の `番号の行`。行頭の空白は除いてある前提）
fn number_of_line(trimmed_rest: &str) -> Option<&str> {
    match trimmed_rest.find(' ') {
        Some(pos) => {
            let num = &trimmed_rest[..pos];
            is_decision_number(num).then_some(num)
        }
        None => is_decision_number(trimmed_rest).then_some(trimmed_rest),
    }
}

/// `補足の行` の名前と値を取り出す（REQ-core-133。行頭の空白は除いてある前提）
fn field_of_line(trimmed_rest: &str) -> Option<(&str, &str)> {
    let colon = trimmed_rest.find(':')?;
    let name = &trimmed_rest[..colon];
    if name.is_empty() || name.chars().any(char::is_whitespace) {
        return None;
    }
    Some((name, trimmed_rest[colon + 1..].trim()))
}

/// 値から "[文字](href)" の形のリンクを順に取り出す（TBL-core-023、A43、A44）
fn parse_links(value: &str) -> Vec<RecordLink> {
    let mut links = Vec::new();
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'[' {
            i += 1;
            continue;
        }
        // "]" が無ければ、これ以降の "[" も同じなので走査を終える
        let Some(offset) = value[i + 1..].find(']') else {
            break;
        };
        let text_end = i + 1 + offset;
        let after = text_end + 1;
        // "]" の直後が "(" でなければリンクでない。"[" の次から走査を続ける
        if bytes.get(after) != Some(&b'(') {
            i += 1;
            continue;
        }
        let Some(offset) = value[after + 1..].find(')') else {
            i += 1;
            continue;
        };
        let href_end = after + 1 + offset;
        links.push(RecordLink { href: value[after + 1..href_end].to_string() });
        i = href_end + 1;
    }
    links
}

/// 判断の記録のファイルを読んで構造にする（REQ-core-136。記録の行を読むのはこの関数だけ）
pub fn parse_records_file(rel_path: &str, content: &str) -> RecordsFile {
    let mut headings = Vec::new();
    let mut sections: Vec<RecordSection> = Vec::new();
    let mut current: Option<usize> = None;
    let mut has_decision_section = false;
    let mut has_context = false;
    let mut fence: Option<crate::ir::CodeFence> = None;

    for (index, line) in content.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim();
        let heading = trimmed.strip_prefix("## ").map(str::trim);

        // コードブロックの中は読まない。閉じずに文書が終わればそこまでが中（A34、A45）。
        // 判断の記録でないファイルの見出しも、コードブロックの外だけを数える（A47）
        if let Some(open) = &fence {
            if crate::ir::is_closing_fence(line, open) {
                fence = None;
            }
            continue;
        }
        if let Some(open) = crate::ir::parse_opening_fence(line) {
            fence = Some(open);
            continue;
        }

        if let Some(text) = heading {
            headings.push(text.to_string());
            if DECISION_SECTIONS.contains(&text) {
                has_decision_section = true;
            }
            if text == "Context" {
                has_context = true;
            }
            current = required_field_of(text).map(|_| {
                sections.push(RecordSection {
                    name: text.to_string(),
                    numbered_lines: Vec::new(),
                });
                sections.len() - 1
            });
            continue;
        }

        // TBL-core-022 の表に無い節と、最初の "## " の見出しより前の行は読まない（REQ-core-135）
        let Some(section) = current else { continue };
        let Some(rest) = trimmed.strip_prefix("- ") else {
            continue;
        };

        // 番号の行の判定を補足の行より先に行う（A33）
        if let Some(number) = number_of_line(rest) {
            sections[section].numbered_lines.push(NumberedLine {
                number: number.to_string(),
                line: line_number,
                fields: Vec::new(),
            });
            continue;
        }

        // 節の最初の番号の行より前の補足の行の形の行は読まない（REQ-core-135）
        let Some(numbered) = sections[section].numbered_lines.last_mut() else {
            continue;
        };
        let Some((name, value)) = field_of_line(rest) else {
            continue;
        };
        numbered.fields.push(FieldLine {
            name: name.to_string(),
            value: value.to_string(),
            line: line_number,
            links: parse_links(value),
        });
    }

    // A134、A46: 決定の節の見出しをコードブロックの外に1つ以上持つファイルが判断の記録
    if !has_decision_section {
        sections.clear();
        has_context = false;
    }

    RecordsFile {
        rel_path: rel_path.to_string(),
        sections,
        headings,
        is_records: has_decision_section,
        has_context,
    }
}

/// ADR やその他の Markdown ファイルの内容
#[derive(Debug)]
pub struct OtherFile {
    pub rel_path: String,
    pub headings: Vec<String>,
}

/// ADR やその他の Markdown ファイルを解析する
pub fn parse_other_file(rel_path: &str, content: &str) -> OtherFile {
    let mut headings = Vec::new();
    let mut fence: Option<crate::ir::CodeFence> = None;
    for line in content.lines() {
        // コードブロックの中の見出しは数えない。閉じなければ文書の終わりまで（A47）
        if let Some(open) = &fence {
            if crate::ir::is_closing_fence(line, open) {
                fence = None;
            }
            continue;
        }
        if let Some(open) = crate::ir::parse_opening_fence(line) {
            fence = Some(open);
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            headings.push(trimmed[3..].trim().to_string());
        }
    }
    OtherFile {
        rel_path: rel_path.to_string(),
        headings,
    }
}

/// 出典を "パス#印" に分割する
pub fn split_source(source: &str) -> Option<(&str, &str)> {
    let source = source.trim();
    if let Some(hash_pos) = source.find('#') {
        let path = &source[..hash_pos];
        let anchor = &source[hash_pos + 1..];
        if path.is_empty() || anchor.is_empty() {
            return None;
        }
        Some((path, anchor))
    } else {
        None
    }
}

/// 出典の検査コンテキスト
pub struct SourceContext {
    /// records ディレクトリのパス（基準からの相対）
    pub records_path: String,
    /// adr ディレクトリのパス（基準からの相対）
    pub adr_path: String,
    /// records ディレクトリ内のファイル
    pub records_files: Vec<RecordsFile>,
    /// adr ディレクトリ内のファイル
    pub adr_files: Vec<OtherFile>,
    /// records ディレクトリ内の判断の記録でないファイル
    pub records_other_files: Vec<OtherFile>,
}

impl SourceContext {
    /// 出典1つを検査する
    pub fn check_source(&self, source: &str) -> Result<(), String> {
        let (raw_path, anchor) = split_source(source).ok_or_else(|| source.to_string())?;
        // 絶対パスは出典として不正
        if raw_path.starts_with('/') || raw_path.starts_with('\\') {
            return Err(source.to_string());
        }
        let path_normalized = crate::normalize_path(raw_path);
        let path = path_normalized.as_str();

        // パスが records の中か adr の中かを判定（置き場が空 = 基準の直下なら何でも中。REQ-core-110）
        // 両方に当たるときは長い置き場を採る（"." の置き場の下に別の置き場があるとき）
        let in_records_raw = is_under_place(path, &self.records_path);
        let in_adr_raw = is_under_place(path, &self.adr_path);
        let (in_records, in_adr) = if in_records_raw && in_adr_raw {
            if self.records_path.len() >= self.adr_path.len() { (true, false) } else { (false, true) }
        } else {
            (in_records_raw, in_adr_raw)
        };

        if !in_records && !in_adr {
            return Err(source.to_string());
        }

        if in_records {
            // records 内のファイルを探す
            if let Some(rf) = self.records_files.iter().find(|rf| {
                let full_path = crate::join_display_path(&self.records_path, &rf.rel_path);
                full_path == path
            }) {
                // records_files に積むのは判断の記録と読めたファイルだけ（load_all_md）
                if rf.is_records {
                    // 判断の記録: 印は決定の番号
                    if is_decision_number(anchor) {
                        if rf.has_decision_number(anchor) {
                            return Ok(());
                        }
                    }
                    return Err(source.to_string());
                }
            }

            // records_other_files（判断の記録でないファイル）を探す
            if let Some(of) = self.records_other_files.iter().find(|of| {
                let full_path = crate::join_display_path(&self.records_path, &of.rel_path);
                full_path == path
            }) {
                if of.headings.iter().any(|h| h == anchor) {
                    return Ok(());
                }
                return Err(source.to_string());
            }

            return Err(source.to_string());
        }

        if in_adr {
            // adr 内のファイルを探す
            if let Some(of) = self.adr_files.iter().find(|of| {
                let full_path = crate::join_display_path(&self.adr_path, &of.rel_path);
                full_path == path
            }) {
                if of.headings.iter().any(|h| h == anchor) {
                    return Ok(());
                }
                return Err(source.to_string());
            }
            return Err(source.to_string());
        }

        Err(source.to_string())
    }
}

/// ソースコンテキストを構築する
/// パスが置き場の下にあるか。置き場が空（"." を正規化したもの）なら基準の直下なので常に真
pub fn is_under_place(path: &str, place: &str) -> bool {
    place.is_empty()
        || (path.starts_with(place) && path.len() > place.len() && path.as_bytes()[place.len()] == b'/')
}

pub fn build_context(
    base: &Path,
    config: &crate::config::Config,
) -> Result<SourceContext, crate::StopReason> {
    let records_dir = base.join(&config.decisions.records);
    let adr_dir = base.join(&config.decisions.adr);

    let mut records_files = Vec::new();
    let mut records_other_files = Vec::new();
    let mut adr_files = Vec::new();

    // records ディレクトリを読む
    if records_dir.is_dir() {
        load_all_md(&records_dir, "", &config.decisions.records, &mut records_files, &mut records_other_files)?;
    }

    // adr ディレクトリを読む
    if adr_dir.is_dir() {
        load_all_md_as_other(&adr_dir, "", &config.decisions.adr, &mut adr_files)?;
    }

    Ok(SourceContext {
        records_path: config.decisions.records.clone(),
        adr_path: config.decisions.adr.clone(),
        records_files,
        adr_files,
        records_other_files,
    })
}

fn load_all_md(
    dir: &Path,
    prefix: &str,
    config_key: &str,
    records: &mut Vec<RecordsFile>,
    others: &mut Vec<OtherFile>,
) -> Result<(), crate::StopReason> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", config_key)))?;

    let mut sorted = Vec::new();
    for entry in entries {
        sorted.push(entry.map_err(|e| {
            crate::StopReason::UnreadableFile(format!("{}: {e}", config_key))
        })?);
    }
    sorted.sort_by_key(|e| e.file_name());

    for entry in sorted {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        let ft = entry.file_type().map_err(|e| {
            let display = crate::join_display_path(config_key, &rel);
            crate::StopReason::UnreadableFile(format!("{display}: {e}"))
        })?;
        // A102: ファイルのシンボリックリンクは読む。ディレクトリのリンクは辿らない。
        // A146: 先の無いリンクは読めないファイルとして停止する
        let (is_dir, is_file) = if ft.is_symlink() {
            let meta = std::fs::metadata(&path).map_err(|e| {
                let display = crate::join_display_path(config_key, &rel);
                crate::StopReason::UnreadableFile(format!("{display}: {e}"))
            })?;
            (false, meta.is_file())
        } else {
            (ft.is_dir(), ft.is_file())
        };
        if is_dir {
            // 除外: 隠しディレクトリは辿らない（CONTEXT.md の除外）
            if name.starts_with('.') {
                continue;
            }
            load_all_md(&path, &rel, config_key, records, others)?;
        } else if is_file && path.extension().is_some_and(|ext| ext == "md") {
            let display = crate::join_display_path(config_key, &rel);
            let content = crate::read_utf8_file(&path, &display)?;

            let rf = parse_records_file(&rel, &content);
            if rf.is_records {
                records.push(rf);
            } else {
                others.push(OtherFile {
                    rel_path: rel,
                    headings: rf.headings,
                });
            }
        }
    }
    Ok(())
}

fn load_all_md_as_other(
    dir: &Path,
    prefix: &str,
    config_key: &str,
    files: &mut Vec<OtherFile>,
) -> Result<(), crate::StopReason> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", config_key)))?;

    let mut sorted = Vec::new();
    for entry in entries {
        sorted.push(entry.map_err(|e| {
            crate::StopReason::UnreadableFile(format!("{}: {e}", config_key))
        })?);
    }
    sorted.sort_by_key(|e| e.file_name());

    for entry in sorted {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        let ft = entry.file_type().map_err(|e| {
            let display = crate::join_display_path(config_key, &rel);
            crate::StopReason::UnreadableFile(format!("{display}: {e}"))
        })?;
        // A102: ファイルのシンボリックリンクは読む。ディレクトリのリンクは辿らない。
        // A146: 先の無いリンクは読めないファイルとして停止する
        let (is_dir, is_file) = if ft.is_symlink() {
            let meta = std::fs::metadata(&path).map_err(|e| {
                let display = crate::join_display_path(config_key, &rel);
                crate::StopReason::UnreadableFile(format!("{display}: {e}"))
            })?;
            (false, meta.is_file())
        } else {
            (ft.is_dir(), ft.is_file())
        };
        if is_dir {
            // 除外: 隠しディレクトリは辿らない（CONTEXT.md の除外）
            if name.starts_with('.') {
                continue;
            }
            load_all_md_as_other(&path, &rel, config_key, files)?;
        } else if is_file && path.extension().is_some_and(|ext| ext == "md") {
            let display = crate::join_display_path(config_key, &rel);
            let content = crate::read_utf8_file(&path, &display)?;

            let of = parse_other_file(&rel, &content);
            files.push(of);
        }
    }
    Ok(())
}

/// 出典を検査して Finding に追加する
pub fn check_sources(
    docs: &[crate::ir::IrDocument],
    ctx: &SourceContext,
    ir_path: &str,
    findings: &mut Vec<Finding>,
) {
    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.relative_path);
        let duplicate_rows = doc.duplicate_glossary_rows(docs);
        for item in &doc.items {
            if duplicate_rows.contains(&item.item_line()) {
                continue;
            }
            let (sources, source_line) = match item {
                // REQ-core-115: 出典の行（行が無ければ見出しの行）
                crate::ir::Item::Requirement { sources, source_line, line, .. }
                | crate::ir::Item::DecisionTable { sources, source_line, line, .. }
                | crate::ir::Item::Property { sources, source_line, line, .. }
                | crate::ir::Item::FlagEntry { sources, source_line, line, .. } => {
                    (sources.clone(), source_line.unwrap_or(*line))
                }
                crate::ir::Item::Scenario { sources, tag_line, line, .. } => {
                    // シナリオはタグの行
                    (sources.clone(), tag_line.unwrap_or(*line))
                }
                crate::ir::Item::GlossaryTerm { sources, line, .. } => {
                    // 用語は表の行
                    (sources.clone(), *line)
                }
            };

            for source in &sources {
                if let Err(bad) = ctx.check_source(source) {
                    findings.push(Finding::new(FindingKind::SourceInvalid, path.clone(), Some(source_line), bad));
                }
            }
        }
    }
}
