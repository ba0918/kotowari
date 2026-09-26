//! `面`の検査（docs/ir/core/surface.md）。`面の規則`で`面のファイル`から`面`を取り出す。

use crate::config::Config;
use crate::doc_kind::DocKind;
use crate::ir::{IrDocument, Item, split_lines};
use crate::test_queries::{ParsedFile, RuleSet, language_of};
use crate::{Finding, FindingKind, StopReason};
use markdown::mdast::Node;
use std::collections::{BTreeMap, BTreeSet};
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

/// "surface.rules" で "surface.files" から`面`を取り出す。
/// "surface.rules" が空の一覧なら何も読まない（REQ-core-223、REQ-core-227）。
/// 規則の言語の`面のファイル`の構文の誤りは unparsable_file にし、同じパスに
/// `テストのファイル`として出していれば重ねない（REQ-core-236）
pub fn extract(
    base: &Path,
    cfg: &Config,
    findings: &mut Vec<Finding>,
) -> Result<Vec<Surface>, StopReason> {
    if cfg.surface.rules.is_empty() {
        return Ok(Vec::new());
    }
    let rules = RuleSet::load(base, &cfg.surface.rules, "surface.rules")?;
    let files = crate::tests_discovery::collect_files(base, &cfg.surface.files)?;
    let mut surfaces = Vec::new();
    for (rel, abs) in &files {
        // REQ-core-224: 読めないファイルと UTF-8 でないファイルでの停止は "tests.files" と同じ
        let content =
            crate::tests_discovery::lone_cr_to_lf(&crate::read_utf8_file(Path::new(abs), rel)?);
        let Some(lang) = language_of(rel).filter(|lang| rules.has_language(*lang)) else {
            continue;
        };
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

/// IR に無い`面`に、種類と名前の組ごとに1件の surface_without_spec を出す。
/// 場所は`面のファイル`のパスのバイト順、次に行の小さい順で最初の`面`（REQ-core-227）
pub fn report(surfaces: &[Surface], docs: &[IrDocument], findings: &mut Vec<Finding>) {
    let quoted = quoted_in_ir(docs);
    let mut first: BTreeMap<(&str, &str), &Surface> = BTreeMap::new();
    for surface in surfaces {
        if quoted.contains(surface.name.as_str()) {
            continue;
        }
        first
            .entry((surface.kind.as_str(), surface.name.as_str()))
            .and_modify(|current| {
                if (&surface.path, surface.line) < (&current.path, current.line) {
                    *current = surface;
                }
            })
            .or_insert(surface);
    }
    for ((kind, name), surface) in first {
        findings.push(Finding::new(
            FindingKind::SurfaceWithoutSpec,
            surface.path.clone(),
            Some(surface.line),
            format!("{kind} {name}"),
        ));
    }
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
    let Ok(root) = kotowari_markdown_schema::ast::parse_mdast(&doc.raw_content) else {
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
