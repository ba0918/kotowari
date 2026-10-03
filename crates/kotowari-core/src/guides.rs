//! `ガイド`を読み、`ガイドの印`を取り出す（REQ-core-198〜REQ-core-202、REQ-core-206、TBL-core-036）

use crate::config::Config;
use crate::fingerprint::fingerprint_of;
use crate::ir::{IrDocument, is_valid_id, split_lines};
use crate::{Finding, FindingKind, StopReason};
use markdown::mdast::Node;
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::Path;

/// 形の正しい`ガイドの印`の1件（TBL-core-036）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuideEntry {
    /// `ガイド`の`基準のディレクトリ`からの相対パス
    pub path: String,
    /// その`ガイドの印`の始まりの行
    pub line: usize,
    pub id: String,
    /// 1件に書かれた`指紋`
    pub fingerprint: String,
}

/// "kotowari check" の "guides"（REQ-core-206、TBL-core-005）
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize)]
pub struct GuideTally {
    /// 読んだ`ガイド`の数。パスごとに1回
    pub files: usize,
    /// 形の正しい`ガイドの印`の1件の数
    pub marks: usize,
}

/// "guides.files" に当たるファイルを`ガイド`として読み、`ガイドの印`を今の`IR`の`指紋`と照らす。
/// `テストのファイル`と重なるファイルがあれば、バイト順で最初の1つを詳細にして設定の誤りで停止する（REQ-core-199）。
/// `test_files` はバイト順に並んだ`テストのファイル`の相対パス
pub fn read_guides(
    base: &Path,
    cfg: &Config,
    test_files: &[String],
    docs: &[IrDocument],
    findings: &mut Vec<Finding>,
) -> Result<GuideTally, StopReason> {
    // 空の一覧ならガイドは1つも読まない（REQ-core-198）。走査そのものを省く
    if cfg.guides.files.is_empty() {
        return Ok(GuideTally::default());
    }
    let files = crate::tests_discovery::collect_files(base, &cfg.guides.files)?;
    // files はバイト順なので、最初に見つかる重なりがバイト順で最初の1つ
    if let Some((overlap, _)) = files
        .iter()
        .find(|(rel, _)| test_files.binary_search(rel).is_ok())
    {
        return Err(StopReason::ConfigError(format!(
            "{overlap}: matched by both guides.files and tests.files"
        )));
    }
    let mut sources = Vec::new();
    for (rel, abs) in &files {
        let content = crate::read_utf8_file(Path::new(abs), rel)?;
        sources.push((rel.as_str(), content));
    }
    check_entries(
        sources.iter().map(|(path, text)| (*path, text.as_str())),
        test_files,
        docs,
        findings,
    )
}

/// `ID` から、その `ID` の`項目`と`シナリオ`の`指紋`を REQ-core-032 の順（文書はパスのバイト順、
/// 同じ文書では行の小さい順）に引く。docs も items もその順に並んでいる
fn fingerprints_by_id(docs: &[IrDocument]) -> BTreeMap<&str, Vec<String>> {
    let mut by_id: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for doc in docs {
        let lines = split_lines(&doc.raw_content);
        for item in &doc.items {
            if let Some(id) = item.id() {
                by_id
                    .entry(id)
                    .or_default()
                    .push(fingerprint_of(doc, item, &lines));
            }
        }
    }
    by_id
}

pub(crate) fn check_texts(
    files: &[crate::SourceText],
    test_files: &[String],
    docs: &[IrDocument],
    findings: &mut Vec<Finding>,
) -> Result<GuideTally, StopReason> {
    check_entries(
        files.iter().map(|source| (source.path(), source.text())),
        test_files,
        docs,
        findings,
    )
}

fn check_entries<'a>(
    files: impl IntoIterator<Item = (&'a str, &'a str)>,
    test_files: &[String],
    docs: &[IrDocument],
    findings: &mut Vec<Finding>,
) -> Result<GuideTally, StopReason> {
    let files: Vec<_> = files.into_iter().collect();
    let mut paths: Vec<_> = files.iter().map(|(path, _)| *path).collect();
    paths.sort();
    if let Some(overlap) = paths
        .into_iter()
        .find(|path| test_files.binary_search(&path.to_string()).is_ok())
    {
        return Err(StopReason::ConfigError(format!(
            "{overlap}: matched by both guides.files and tests.files"
        )));
    }
    let mut entries = Vec::new();
    for (path, text) in &files {
        read_marks(path, text, &mut entries, findings);
    }
    check_stale(&entries, &fingerprints_by_id(docs), findings);
    Ok(GuideTally {
        files: files.len(),
        marks: entries.len(),
    })
}

/// REQ-core-204: どの`項目`と`シナリオ`の`指紋`とも同じでない1件ごとに guide_stale を出す。
/// detail の今の`指紋`は、`IR`に無ければ "-"、あれば1つ目の`指紋`
fn check_stale(
    entries: &[GuideEntry],
    fingerprints: &BTreeMap<&str, Vec<String>>,
    findings: &mut Vec<Finding>,
) {
    for entry in entries {
        let current = fingerprints.get(entry.id.as_str());
        if current.is_some_and(|all| all.contains(&entry.fingerprint)) {
            continue;
        }
        let first = current
            .and_then(|all| all.first())
            .map_or("-", String::as_str);
        findings.push(Finding::new(
            FindingKind::GuideStale,
            entry.path.clone(),
            Some(entry.line),
            format!("{} {} {first}", entry.id, entry.fingerprint),
        ));
    }
}

const MARK_START: &str = "@kotowari[";

/// 1つの`ガイド`から`ガイドの印`を読む。形の正しい印の1件を entries に、形の誤った印ごとに
/// invalid_marker を findings に積む（REQ-core-200、REQ-core-202）
fn read_marks(
    path: &str,
    content: &str,
    entries: &mut Vec<GuideEntry>,
    findings: &mut Vec<Finding>,
) {
    let lines = LineIndex::new(content);
    for comment in html_comments(content) {
        let mut from = comment.start;
        while let Some(found) = content[from..comment.end].find(MARK_START) {
            let start = from + found;
            let line = lines.line_of(start);
            let inner_start = start + MARK_START.len();
            // TBL-core-036: 終わりは始まりと同じ行の "]"。コメントの外の "]" は読まない
            let line_end = comment.end.min(lines.end_of(line));
            let parsed = content[inner_start..line_end]
                .find(']')
                .and_then(|close| parse_entries(&content[inner_start..inner_start + close]));
            match parsed {
                Some(pairs) => {
                    entries.extend(pairs.into_iter().map(|(id, fingerprint)| GuideEntry {
                        path: path.to_string(),
                        line,
                        id,
                        fingerprint,
                    }));
                }
                None => findings.push(Finding::new(
                    FindingKind::InvalidMarker,
                    path.to_string(),
                    Some(line),
                    lines.text(line).to_string(),
                )),
            }
            from = inner_start;
        }
    }
}

/// TBL-core-036 の中身を (`ID`, `指紋`) の並びに分ける。形の誤りがあれば None（REQ-core-202）
fn parse_entries(inner: &str) -> Option<Vec<(String, String)>> {
    let parts: Vec<&str> = inner.split(',').map(str::trim).collect();
    // 中が空か区切りだけ
    if parts.iter().all(|part| part.is_empty()) {
        return None;
    }
    parts
        .into_iter()
        .map(|part| {
            let (id, fingerprint) = part.split_once(':')?;
            let (id, fingerprint) = (id.trim(), fingerprint.trim());
            (is_valid_id(id) && is_fingerprint(fingerprint))
                .then(|| (id.to_string(), fingerprint.to_string()))
        })
        .collect()
}

/// TBL-core-036: 16進の小文字の8文字
fn is_fingerprint(text: &str) -> bool {
    text.len() == 8 && text.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// HTML のコメントの "<!--" と "-->" の間のバイトの範囲（REQ-core-200）。
/// `ガイド`を CommonMark として読み、HTML の節（ブロックの形と段落の中の形）だけを見るので、
/// コードブロック（字下げの形とフェンスの形）とコードスパンの中は入らない。
/// 範囲は節の値ではなく元の文字から取る。リストの中などでは節の値から字下げが除かれるが、
/// 行番号と detail の行の文字には元の行が要る。"-->" の無い "<!--" はコメントにしない
fn html_comments(content: &str) -> Vec<Range<usize>> {
    // GFM と frontmatter の読み方は MDX の構文を持たないので、parse_mdast が誤りを返すことはない
    let Ok(root) = crate::markdown::parse(content) else {
        return Vec::new();
    };
    let mut html = Vec::new();
    collect_html(&root, &mut html);
    let mut comments = Vec::new();
    for node in html {
        let mut from = node.start;
        while let Some(open) = content[from..node.end].find("<!--") {
            let inner = from + open + "<!--".len();
            let Some(close) = content[inner..node.end].find("-->") else {
                break;
            };
            comments.push(inner..inner + close);
            from = inner + close + "-->".len();
        }
    }
    comments
}

/// HTML の節の元の文字のバイトの範囲を、文書の順に集める
fn collect_html(node: &Node, html: &mut Vec<Range<usize>>) {
    if let Node::Html(_) = node
        && let Some(position) = node.position()
    {
        html.push(position.start.offset..position.end.offset);
    }
    for child in node.children().into_iter().flatten() {
        collect_html(child, html);
    }
}

/// バイトの位置から行（1始まり）と行の文字を引く。行の区切りは TBL-core-010（`split_lines`）と同じ
struct LineIndex<'a> {
    lines: Vec<&'a str>,
    /// 各行の始まりのバイトの位置
    starts: Vec<usize>,
}

impl<'a> LineIndex<'a> {
    fn new(content: &'a str) -> Self {
        let lines = split_lines(content);
        let base = content.as_ptr() as usize;
        let starts = lines
            .iter()
            .map(|line| line.as_ptr() as usize - base)
            .collect();
        LineIndex { lines, starts }
    }

    fn line_of(&self, offset: usize) -> usize {
        self.starts.partition_point(|&start| start <= offset)
    }

    /// その行の終わり（行の終わりの文字を含まない）のバイトの位置
    fn end_of(&self, line: usize) -> usize {
        self.starts[line - 1] + self.lines[line - 1].len()
    }

    fn text(&self, line: usize) -> &'a str {
        self.lines[line - 1]
    }
}
