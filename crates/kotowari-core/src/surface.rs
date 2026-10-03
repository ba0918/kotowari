//! `面`の検査（docs/ir/core/surface.md）。`面の規則`で`面のファイル`から`面`を取り出す。

use crate::config::Config;
use crate::doc_kind::DocKind;
use crate::ir::{IrDocument, Item, split_lines};
use crate::test_queries::{ParsedFile, RuleSet, language_of};
use crate::{Finding, FindingKind, StopReason};
use markdown::mdast::Node;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

/// `面の規則`で取り出した`面`1つ（REQ-core-223）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Surface {
    /// 当てた`面の規則`の "id"
    pub kind: String,
    /// "$NAME" に入った節の文字から、前後の同じ引用符を1組外したもの
    pub name: String,
    /// `面のファイル`の`基準のディレクトリ`からの相対パス
    pub path: String,
    /// 節の最初の行
    pub line: usize,
}

/// "surface.rules" で "surface.files" から`面`を取り出す（REQ-core-223）。
/// 規則の言語の`面のファイル`だけを読み、構文の誤りは unparsable_file にし、同じパスに
/// `テストのファイル`として出していれば重ねない（REQ-core-236）
pub fn extract(
    base: &Path,
    cfg: &Config,
    findings: &mut Vec<Finding>,
) -> Result<Vec<Surface>, StopReason> {
    let rules = RuleSet::load(base, &cfg.surface.rules, "surface.rules")?;
    let files = crate::tests_discovery::collect_files(base, &cfg.surface.files)?;
    let mut surfaces = Vec::new();
    for (rel, abs) in &files {
        // REQ-core-236: 規則の言語でないファイルは読まない
        let Some(lang) = language_of(rel).filter(|lang| rules.has_language(*lang)) else {
            continue;
        };
        // REQ-core-224: 読めないファイルと UTF-8 でないファイルでの停止は`テストのファイル`と同じ
        let content =
            crate::tests_discovery::lone_cr_to_lf(&crate::read_utf8_file(Path::new(abs), rel)?);
        let Some(parsed) = ParsedFile::parse(&content, lang) else {
            let reported = findings
                .iter()
                .any(|f| f.kind == FindingKind::UnparsableFile && f.path == *rel);
            if !reported {
                findings.push(Finding::new(
                    FindingKind::UnparsableFile,
                    rel.clone(),
                    None,
                    rel.clone(),
                ));
            }
            continue;
        };
        surfaces.extend(
            parsed
                .find_named(&rules, lang, rel)
                .into_iter()
                .map(|m| Surface {
                    kind: m.rule_id,
                    name: m.name,
                    path: rel.clone(),
                    line: m.line,
                }),
        );
    }
    Ok(surfaces)
}

/// `面`の種類と名前の組の数（TBL-core-028 の "surface"）
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize)]
pub struct SurfaceTally {
    /// `面`の種類と名前の組の数
    pub total: usize,
    /// そのうち`IR`にあるものの数
    pub specified: usize,
    /// `IR`になく`未記載の面の一覧`の形の正しい1件に一致したものの数
    pub unspecified: usize,
}

/// "kotowari check" の "surface"（TBL-core-005、REQ-core-228）
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Unlisted {
    /// `未記載の面の一覧`で外した`面`の種類と名前の組の数
    pub unspecified: usize,
}

/// check と status の`面`の検査（REQ-core-229）。`面`を取り出し、`未記載の面の一覧`を読み、
/// `指摘`を足して数を返す。"surface.rules" が空の一覧なら何も読まず None を返す
pub fn check(
    base: &Path,
    cfg: &Config,
    docs: &[IrDocument],
    findings: &mut Vec<Finding>,
) -> Result<Option<SurfaceTally>, StopReason> {
    if cfg.surface.rules.is_empty() {
        return Ok(None);
    }
    let surfaces = extract(base, cfg, findings)?;
    let list = read_unspecified_file(base, cfg)?;
    Ok(Some(report(&surfaces, docs, &list, findings)))
}

/// 形の正しい`未記載の面の一覧`の1件（REQ-core-232）
#[derive(Debug, Clone, PartialEq, Eq)]
struct Unspecified {
    kind: String,
    name: String,
}

/// `未記載の面の一覧`を読んだ結果
#[derive(Debug, Default)]
struct UnspecifiedList {
    /// 一覧のファイルの`基準のディレクトリ`からの相対パス
    path: String,
    /// 形の正しい1件
    entries: Vec<Unspecified>,
    /// 形の誤った1件への surface_unspecified_invalid（この1件には surface_unspecified_stale を出さない）
    findings: Vec<Finding>,
}

/// "surface.unspecified" の指す先を読む。鍵が無ければ0件。無いか読めなければ読めないファイル、
/// UTF-8 でなければ UTF-8 でないファイル、YAML として読めないか最上位が並びでなければ
/// 設定の誤りで`停止`する（REQ-core-231）。形の誤った1件は surface_unspecified_invalid にして
/// 一覧から外す（REQ-core-233）
fn read_unspecified_file(base: &Path, cfg: &Config) -> Result<UnspecifiedList, StopReason> {
    let Some(path) = &cfg.surface.unspecified else {
        return Ok(UnspecifiedList::default());
    };
    let text = crate::read_utf8_file(&base.join(path), path)?;
    let mut entries = Vec::new();
    let mut findings = Vec::new();
    for item in &crate::config::read_yaml_sequence(&text, path)? {
        match read_entry(item) {
            Some(entry) => entries.push(entry),
            None => findings.push(Finding::new(
                FindingKind::SurfaceUnspecifiedInvalid,
                path.clone(),
                None,
                written_detail(item),
            )),
        }
    }
    Ok(UnspecifiedList {
        path: path.clone(),
        entries,
        findings,
    })
}

pub(crate) fn check_analysis(
    analysis: &[crate::SurfaceAnalysis],
    docs: &[IrDocument],
    unspecified: &[crate::SourceText],
    findings: &mut Vec<Finding>,
) -> Result<SurfaceTally, StopReason> {
    let mut surfaces = Vec::new();
    for file in analysis {
        surfaces.extend(file.surfaces.iter().cloned());
        for finding in &file.findings {
            if finding.kind == FindingKind::UnparsableFile
                && findings
                    .iter()
                    .any(|old| old.kind == finding.kind && old.path == finding.path)
            {
                continue;
            }
            findings.push(finding.clone());
        }
    }
    let mut list = UnspecifiedList::default();
    for source in unspecified {
        list.path = source.path().to_owned();
        for item in &crate::config::read_yaml_sequence(source.text(), source.path())? {
            match read_entry(item) {
                Some(entry) => list.entries.push(entry),
                None => list.findings.push(Finding::new(
                    FindingKind::SurfaceUnspecifiedInvalid,
                    source.path().to_owned(),
                    None,
                    written_detail(item),
                )),
            }
        }
    }
    Ok(report(&surfaces, docs, &list, findings))
}

/// 1件を読む。鍵と値の組で、"kind"、"name"、"why" のちょうど3つの鍵の値がどれも文字列で、
/// "why" が前後の半角空白とタブを除いて空でないときだけ形が正しい（REQ-core-233）
fn read_entry(item: &Value) -> Option<Unspecified> {
    const KEYS: [&str; 3] = ["kind", "name", "why"];
    let map = item.as_object()?;
    if map.len() != KEYS.len() {
        return None;
    }
    let [kind, name, why] = KEYS.map(|key| map.get(key).and_then(Value::as_str));
    let (kind, name, why) = (kind?, name?, why?);
    if crate::mutants::trim_spaces_and_tabs(why).is_empty() {
        return None;
    }
    Some(Unspecified {
        kind: kind.to_string(),
        name: name.to_string(),
    })
}

/// 一覧に書かれたままの "kind" と "name" を半角空白1つで区切る。無いか文字列でない方は空の文字列
/// （REQ-core-233、REQ-core-234）
fn written_detail(item: &Value) -> String {
    let written = |key: &str| item.get(key).and_then(Value::as_str).unwrap_or("");
    format!("{} {}", written("kind"), written("name"))
}

/// IR に無く一覧のどの1件にも一致しない`面`に、種類と名前の組ごとに1件の surface_without_spec を出す。
/// 場所は`面のファイル`のパスのバイト順、次に行の小さい順で最初の`面`（REQ-core-227）。
/// 一致する`面`が無いか、一致する`面`が IR にある1件には surface_unspecified_stale を出す（REQ-core-234）
fn report(
    surfaces: &[Surface],
    docs: &[IrDocument],
    list: &UnspecifiedList,
    findings: &mut Vec<Finding>,
) -> SurfaceTally {
    let quoted = quoted_in_ir(docs);
    let in_ir = |name: &str| quoted.contains(name);
    let listed = |surface: &Surface| {
        list.entries
            .iter()
            .any(|entry| entry.kind == surface.kind && entry.name == surface.name)
    };
    findings.extend(list.findings.iter().cloned());
    for entry in &list.entries {
        let needed = surfaces
            .iter()
            .any(|s| s.kind == entry.kind && s.name == entry.name && !in_ir(&s.name));
        if !needed {
            findings.push(Finding::new(
                FindingKind::SurfaceUnspecifiedStale,
                list.path.clone(),
                None,
                format!("{} {}", entry.kind, entry.name),
            ));
        }
    }
    let mut tally = SurfaceTally::default();
    // 種類と名前の組ごとに、面のファイルのパスのバイト順、次に行の小さい順で最初の面だけを見る（REQ-core-227）
    let mut ordered: Vec<&Surface> = surfaces.iter().collect();
    ordered.sort_by(|a, b| a.path.cmp(&b.path).then(a.line.cmp(&b.line)));
    let mut seen: BTreeSet<(&str, &str)> = BTreeSet::new();
    for surface in ordered {
        if !seen.insert((&surface.kind, &surface.name)) {
            continue;
        }
        tally.total += 1;
        if in_ir(&surface.name) {
            tally.specified += 1;
        } else if listed(surface) {
            tally.unspecified += 1;
        } else {
            findings.push(Finding::new(
                FindingKind::SurfaceWithoutSpec,
                surface.path.clone(),
                Some(surface.line),
                format!("{} {}", surface.kind, surface.name),
            ));
        }
    }
    tally
}

/// `話題ごとの文書`の`要求`の`文`、`決定表`の表のセル（見出しの行を含む）、`シナリオ`のステップの行で、
/// 二重引用符の対かバッククォートの対で囲んだ中身の集まり。中身の前後の空白は除かない（REQ-core-226）
fn quoted_in_ir(docs: &[IrDocument]) -> BTreeSet<String> {
    let mut quoted = BTreeSet::new();
    let mut collect = |text: &str| {
        for line in split_lines(text) {
            quoted.extend(
                crate::double_quoted_contents(line)
                    .into_iter()
                    .map(str::to_string),
            );
            quoted.extend(
                crate::extract_backtick_contents_outside_quotes(line)
                    .into_iter()
                    .map(str::to_string),
            );
        }
    };
    for doc in docs.iter().filter(|doc| doc.kind == DocKind::Topic) {
        for item in &doc.items {
            match item {
                Item::Requirement { statements, .. } => {
                    statements.iter().for_each(|(_, text)| collect(text));
                }
                Item::Scenario { steps, .. } => steps.iter().for_each(|(_, text)| collect(text)),
                _ => {}
            }
        }
        for cell in decision_table_cells(doc) {
            collect(&cell);
        }
    }
    quoted
}

/// `決定表`の`項目`の中にある表の、見出しの行を含むすべてのセルの元の文字
fn decision_table_cells(doc: &IrDocument) -> Vec<String> {
    let ranges: Vec<(usize, usize)> = doc
        .items
        .iter()
        .filter_map(|item| match item {
            Item::DecisionTable { line, end, .. } => Some((*line, *end)),
            _ => None,
        })
        .collect();
    if ranges.is_empty() {
        return Vec::new();
    }
    let Ok(root) = crate::markdown::parse(&doc.raw_content) else {
        return Vec::new();
    };
    let mut cells = Vec::new();
    collect_cells(&root, &doc.raw_content, &ranges, &mut cells);
    cells
}

/// 行が `ranges` のどれかに入る表のセルの、中身の元の文字を集める
fn collect_cells(node: &Node, source: &str, ranges: &[(usize, usize)], cells: &mut Vec<String>) {
    if let Node::TableCell(cell) = node {
        let span = cell
            .children
            .first()
            .zip(cell.children.last())
            .and_then(|(first, last)| {
                Some((
                    first.position()?.start.clone(),
                    last.position()?.end.clone(),
                ))
            });
        if let Some((start, end)) = span
            && ranges
                .iter()
                .any(|(from, to)| (*from..=*to).contains(&start.line))
        {
            cells.push(source[start.offset..end.offset].to_string());
        }
        return;
    }
    for child in node.children().into_iter().flatten() {
        collect_cells(child, source, ranges, cells);
    }
}
