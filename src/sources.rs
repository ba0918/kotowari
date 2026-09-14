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

/// 判断の記録のファイルを解析する
pub fn parse_records_file(rel_path: &str, content: &str) -> RecordsFile {
    let mut decision_numbers = Vec::new();
    let mut headings = Vec::new();
    let mut in_decision_section = false;
    let mut has_decision_section = false;
    let decision_sections = ["Agreements", "Prohibitions", "Delegated", "Rejected"];

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("## ") {
            let heading = trimmed[3..].trim().to_string();
            if decision_sections.contains(&heading.as_str()) {
                in_decision_section = true;
                has_decision_section = true;
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

    // A134: 決定の節の見出しを1つ以上持つファイルが判断の記録（番号の有無では決めない）
    let is_records = has_decision_section;

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
        let (raw_path, anchor) = split_source(source).ok_or_else(|| source.to_string())?;
        // 絶対パスは出典として不正
        if raw_path.starts_with('/') || raw_path.starts_with('\\') {
            return Err(source.to_string());
        }
        let path_normalized = crate::normalize_path(raw_path);
        let path = path_normalized.as_str();

        // パスが records の中か adr の中かを判定（置き場が空 = 基準の直下なら何でも中。REQ-110）
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
fn is_under_place(path: &str, place: &str) -> bool {
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
        let path = crate::join_display_path(ir_path, &doc.filename);
        for item in &doc.items {
            let (sources, source_line) = match item {
                crate::ir::Item::Requirement { sources, fields_seen, line, .. } => {
                    // REQ-115: 出典の行を探す
                    let sl = fields_seen.iter()
                        .find(|(_, n, _)| n == "出典")
                        .map(|(ln, _, _)| *ln)
                        .unwrap_or(*line);
                    (sources.clone(), sl)
                }
                crate::ir::Item::DecisionTable { sources, fields_seen, line, .. } => {
                    let sl = fields_seen.iter()
                        .find(|(_, n, _)| n == "出典")
                        .map(|(ln, _, _)| *ln)
                        .unwrap_or(*line);
                    (sources.clone(), sl)
                }
                crate::ir::Item::Property { sources, fields_seen, line, .. } => {
                    let sl = fields_seen.iter()
                        .find(|(_, n, _)| n == "出典")
                        .map(|(ln, _, _)| *ln)
                        .unwrap_or(*line);
                    (sources.clone(), sl)
                }
                crate::ir::Item::FlagEntry { sources, fields_seen, line, .. } => {
                    let sl = fields_seen.iter()
                        .find(|(_, n, _)| n == "出典")
                        .map(|(ln, _, _)| *ln)
                        .unwrap_or(*line);
                    (sources.clone(), sl)
                }
                crate::ir::Item::Scenario { sources, tag_line, line, .. } => {
                    // シナリオはタグの行
                    (sources.clone(), tag_line.unwrap_or(*line))
                }
                crate::ir::Item::GlossaryTerm { sources, line, .. } => {
                    // 用語は表の行
                    (sources.clone(), *line)
                }
                crate::ir::Item::UnknownHeading { .. } => continue,
            };

            for source in &sources {
                if let Err(bad) = ctx.check_source(source) {
                    findings.push(Finding::new(FindingKind::SourceInvalid, path.clone(), Some(source_line), bad));
                }
            }
        }
    }
}
