//! 出典の検査（REQ-057〜REQ-061, REQ-106, TBL-012）

use crate::{Finding, FindingKind};
use std::path::Path;

/// 判断の記録のファイルの内容
#[derive(Debug)]
pub struct RecordsFile {
    /// ファイルのパス（decisions.records からの相対）
    pub rel_path: String,
    /// 決定の行（"- A26 ..." なら "A26" がキー）
    pub decision_numbers: Vec<String>,
    /// ## 見出し（前後の空白を除いた文字列）
    pub headings: Vec<String>,
    /// 判断の記録かどうか（決定の番号を持つ行があるか）
    pub is_records: bool,
}

/// ADR やその他の Markdown ファイルの内容
#[derive(Debug)]
pub struct OtherFile {
    pub rel_path: String,
    pub headings: Vec<String>,
}

/// 決定の番号の形（A26, P1, D1, R6 など）
pub fn is_decision_number(s: &str) -> bool {
    if s.len() < 2 {
        return false;
    }
    let first = s.as_bytes()[0];
    if !first.is_ascii_alphabetic() || !first.is_ascii_uppercase() {
        return false;
    }
    s[1..].chars().all(|c| c.is_ascii_digit())
}

/// 判断の記録のファイルを解析する
pub fn parse_records_file(rel_path: &str, content: &str) -> RecordsFile {
    let mut decision_numbers = Vec::new();
    let mut headings = Vec::new();
    let mut in_decision_section = false;
    let decision_sections = ["Agreements", "Prohibitions", "Delegated", "Rejected"];

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("## ") {
            let heading = trimmed[3..].trim().to_string();
            if decision_sections.contains(&heading.as_str()) {
                in_decision_section = true;
            } else {
                in_decision_section = false;
            }
            headings.push(heading);
            continue;
        }

        if in_decision_section && trimmed.starts_with("- ") {
            let rest = &trimmed[2..];
            // "A26 ..." の形
            if let Some(space_pos) = rest.find(' ') {
                let num = &rest[..space_pos];
                if is_decision_number(num) {
                    decision_numbers.push(num.to_string());
                }
            } else if is_decision_number(rest) {
                decision_numbers.push(rest.to_string());
            }
        }
    }

    let is_records = !decision_numbers.is_empty();

    RecordsFile {
        rel_path: rel_path.to_string(),
        decision_numbers,
        headings,
        is_records,
    }
}

/// ADR やその他の Markdown ファイルを解析する
pub fn parse_other_file(rel_path: &str, content: &str) -> OtherFile {
    let mut headings = Vec::new();
    for line in content.lines() {
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
        let (path, anchor) = split_source(source).ok_or_else(|| source.to_string())?;

        // パスが records の中か adr の中かを判定
        let in_records = path.starts_with(&self.records_path)
            && path.len() > self.records_path.len()
            && (path.as_bytes()[self.records_path.len()] == b'/');
        let in_adr = path.starts_with(&self.adr_path)
            && path.len() > self.adr_path.len()
            && (path.as_bytes()[self.adr_path.len()] == b'/');

        if !in_records && !in_adr {
            return Err(source.to_string());
        }

        if in_records {
            // records 内のファイルを探す
            if let Some(rf) = self.records_files.iter().find(|rf| {
                let full_path = format!("{}/{}", self.records_path, rf.rel_path);
                full_path == path
            }) {
                if rf.is_records {
                    // 判断の記録: 印は決定の番号
                    if is_decision_number(anchor) {
                        if rf.decision_numbers.iter().any(|n| n == anchor) {
                            return Ok(());
                        }
                    }
                    return Err(source.to_string());
                } else {
                    // 判断の記録でない Markdown: 印は ## 見出し
                    if rf.headings.iter().any(|h| h == anchor) {
                        return Ok(());
                    }
                    return Err(source.to_string());
                }
            }

            // records_other_files（判断の記録でないファイル）を探す
            if let Some(of) = self.records_other_files.iter().find(|of| {
                let full_path = format!("{}/{}", self.records_path, of.rel_path);
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
                let full_path = format!("{}/{}", self.adr_path, of.rel_path);
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
        load_all_md(&records_dir, "", &mut records_files, &mut records_other_files)?;
    }

    // adr ディレクトリを読む
    if adr_dir.is_dir() {
        load_all_md_as_other(&adr_dir, "", &mut adr_files)?;
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
    records: &mut Vec<RecordsFile>,
    others: &mut Vec<OtherFile>,
) -> Result<(), crate::StopReason> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", dir.display())))?;

    let mut sorted: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    sorted.sort_by_key(|e| e.file_name());

    for entry in sorted {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if ft.is_dir() {
            load_all_md(&path, &rel, records, others)?;
        } else if ft.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            let bytes = std::fs::read(&path)
                .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", path.display())))?;
            let content = String::from_utf8(bytes)
                .map_err(|_| crate::StopReason::NonUtf8File(format!("{}", path.display())))?;

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
    files: &mut Vec<OtherFile>,
) -> Result<(), crate::StopReason> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", dir.display())))?;

    let mut sorted: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    sorted.sort_by_key(|e| e.file_name());

    for entry in sorted {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        let ft = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if ft.is_dir() {
            load_all_md_as_other(&path, &rel, files)?;
        } else if ft.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            let bytes = std::fs::read(&path)
                .map_err(|e| crate::StopReason::UnreadableFile(format!("{}: {e}", path.display())))?;
            let content = String::from_utf8(bytes)
                .map_err(|_| crate::StopReason::NonUtf8File(format!("{}", path.display())))?;

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
        let path = format!("{}/{}", ir_path, doc.filename);
        for item in &doc.items {
            let sources = match item {
                crate::ir::Item::Requirement { sources, .. }
                | crate::ir::Item::DecisionTable { sources, .. }
                | crate::ir::Item::Property { sources, .. }
                | crate::ir::Item::FlagEntry { sources, .. } => sources.clone(),
                crate::ir::Item::Scenario { sources, .. } => sources.clone(),
                crate::ir::Item::GlossaryTerm { sources, .. } => sources.clone(),
                crate::ir::Item::UnknownHeading { .. } => continue,
            };

            for source in &sources {
                if let Err(bad) = ctx.check_source(source) {
                    findings.push(Finding::new(FindingKind::SourceInvalid, path.clone(), Some(item.item_line()), bad));
                }
            }
        }
    }
}
