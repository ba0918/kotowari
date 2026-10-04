//! 全体像の元データを検査し、参照の表と古い節を求めて描画の入力を作る。
//!
//! 仕様は `docs/ir/core/overview-data.md` と `docs/ir/core/overview-commands.md` にある。
//! このクレートはファイル、ネットワーク、環境変数に触れない。元データの文字列と、kotowari-core で
//! 読み込み済みの IR と判断の記録だけから計算する（REQ-core-287）。

mod document;
mod form;
mod parts;
mod references;

use kotowari_core::{DocKind, Finding, FindingGroup, FindingKind, ReadModel, SourceText};
use kotowari_markdown_view::{Block, Document, Page, Part, Reference, RenderInput, Section};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

/// check と status の結果に加える追加の指摘の群の名前（REQ-core-288）
pub const GROUP: &str = "overview";

/// 全体像の元データを検査した結果
#[derive(Debug, Clone, Default)]
pub struct Overview {
    findings: Vec<Finding>,
    files: usize,
    marks: usize,
    input: RenderInput,
}

impl Overview {
    /// 元データの指摘。REQ-core-024 の順に並ぶ
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }
    /// 読んだ元データの数
    pub fn files(&self) -> usize {
        self.files
    }
    /// 元データの中の形の正しいガイドの印の1件の数
    pub fn marks(&self) -> usize {
        self.marks
    }
    /// 誤りの件数
    pub fn errors(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.severity() == "error")
            .count()
    }
    /// 描画のエンジンに渡す入力。誤りがあるときは描ける文書だけを持つ
    pub fn render_input(&self) -> &RenderInput {
        &self.input
    }
    /// 誤りが無ければ描画のエンジンが返すページの並び、あれば誤りの件数（REQ-core-294）
    pub fn pages(&self) -> Result<Vec<Page>, usize> {
        match self.errors() {
            0 => Ok(kotowari_markdown_view::render(&self.input)),
            errors => Err(errors),
        }
    }
    /// core の検査に渡す "overview" の群（REQ-core-290）
    pub fn into_group(self) -> FindingGroup {
        FindingGroup::new(GROUP, self.files, self.marks, self.findings)
    }
}

/// 元データを検査する。files の path は基準のディレクトリからの相対パスである
pub fn inspect(read: &ReadModel, files: &[SourceText]) -> Overview {
    let mut files: Vec<&SourceText> = files.iter().collect();
    files.sort_by(|left, right| left.path().as_bytes().cmp(right.path().as_bytes()));
    let mut inspection = Inspection::new(read);
    for file in &files {
        inspection.file(file);
    }
    let mut findings = inspection.findings;
    kotowari_core::sort_findings(&mut findings);
    Overview {
        findings,
        files: files.len(),
        marks: inspection.marks,
        input: RenderInput {
            documents: inspection.documents,
            references: inspection.references.into_values().collect(),
        },
    }
}

/// パスの最後の成分
fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

struct Inspection<'a> {
    form: form::Form,
    parts: parts::Parts,
    resolver: references::Resolver<'a>,
    guides: kotowari_core::guides::GuideReader<'a>,
    /// 基準のディレクトリからの相対パスで読んだ IR の話題ごとの文書
    topics: BTreeSet<String>,
    /// 話題ごとの文書と、それを "ir" に最初に書いた元データ
    owners: BTreeMap<String, String>,
    names: BTreeSet<String>,
    findings: Vec<Finding>,
    marks: usize,
    references: BTreeMap<String, Reference>,
    documents: Vec<Document>,
}

impl<'a> Inspection<'a> {
    fn new(read: &'a ReadModel) -> Self {
        let place = kotowari_core::normalize_path(&read.config().ir);
        let topics = read
            .documents()
            .iter()
            .filter(|document| document.kind() == DocKind::Topic)
            .map(|document| kotowari_core::join_display_path(&place, document.relative_path()))
            .collect();
        Self {
            form: form::Form::new(),
            parts: parts::Parts::new(),
            resolver: references::Resolver::new(read),
            guides: read.guide_reader(),
            topics,
            owners: BTreeMap::new(),
            names: BTreeSet::new(),
            findings: Vec::new(),
            marks: 0,
            references: BTreeMap::new(),
            documents: Vec::new(),
        }
    }

    fn error(&mut self, kind: FindingKind, path: &str, line: Option<usize>, detail: String) {
        self.findings
            .push(Finding::new(kind, path.to_string(), line, detail));
    }

    fn file(&mut self, file: &SourceText) {
        let (path, text) = (file.path(), file.text());
        let name = file_name(path);
        let stem = name.strip_suffix(".md").unwrap_or(name).to_string();
        // REQ-core-305: パスのバイト順で2つ目以降の重なりと、"index" と "style"
        if stem == "index" || stem == "style" || !self.names.insert(stem.clone()) {
            self.error(FindingKind::OverviewNameConflict, path, None, stem.clone());
        }
        let raw = document::parse(text);
        let (ir, mut form) = form::frontmatter(raw.frontmatter.as_deref());
        form.extend(self.form.markdown(text));
        for (line, detail) in form {
            self.error(FindingKind::OverviewFormInvalid, path, line, detail.into());
        }
        for entry in ir.unwrap_or_default() {
            self.ir_entry(path, &entry);
        }
        if raw.lead.is_none() {
            self.error(FindingKind::OverviewLeadMissing, path, None, name.into());
        }
        let values: Vec<Option<Value>> =
            raw.parts.iter().map(|part| self.part(path, part)).collect();
        let marks = self.guides.read(path, text);
        self.marks += marks.marks().len();
        let stale_lines: Vec<usize> = marks
            .marks()
            .iter()
            .filter(|mark| mark.stale())
            .map(|mark| mark.line())
            .collect();
        self.findings.extend(marks.into_findings());
        let lead = raw.lead.and_then(|index| values[index].clone());
        if let (Some(title), Some(lead)) = (raw.title.clone(), lead) {
            let sections = sections(&raw, &values, &stale_lines);
            self.documents.push(Document {
                name: stem,
                title,
                lead: Part {
                    kind: "lead".into(),
                    value: lead,
                },
                sections,
            });
        }
    }

    /// REQ-core-284: "ir" の1件が話題ごとの文書か、ほかの元データの "ir" に無いか
    fn ir_entry(&mut self, path: &str, entry: &str) {
        let document = kotowari_core::normalize_path(entry);
        if !self.topics.contains(&document) {
            self.error(FindingKind::OverviewIrMissing, path, None, entry.into());
            return;
        }
        match self.owners.get(&document) {
            Some(owner) if owner != path => {
                self.error(FindingKind::OverviewIrShared, path, None, document);
            }
            Some(_) => {}
            None => {
                self.owners.insert(document, path.into());
            }
        }
    }

    /// REQ-core-282、REQ-core-285: 部品の中身と参照。スキーマに合えば値を返す
    fn part(&mut self, path: &str, part: &document::RawPart) -> Option<Value> {
        let line = Some(part.line);
        match self.parts.check(&part.kind, &part.content) {
            parts::Checked::Unknown => {
                self.error(
                    FindingKind::OverviewPartUnknown,
                    path,
                    line,
                    part.kind.clone(),
                );
                None
            }
            parts::Checked::Invalid(places) => {
                for place in places {
                    let detail = format!("{} {place}", part.kind);
                    self.error(FindingKind::OverviewPartInvalid, path, line, detail);
                }
                None
            }
            parts::Checked::Valid(value) => {
                let mut keys = Vec::new();
                parts::references(&value, &mut keys);
                for key in keys {
                    if self.references.contains_key(&key) {
                        continue;
                    }
                    match self.resolver.resolve(&key) {
                        Some(reference) => {
                            self.references.insert(key, reference);
                        }
                        None => self.error(FindingKind::OverviewRefUnresolved, path, line, key),
                    }
                }
                Some(value)
            }
        }
    }
}

/// 描画のエンジンに渡す節。古い節は guide_stale を受けるガイドの印を中に持つ節（REQ-core-292）
fn sections(
    raw: &document::RawDocument,
    values: &[Option<Value>],
    stale_lines: &[usize],
) -> Vec<Section> {
    raw.sections
        .iter()
        .enumerate()
        .map(|(index, section)| {
            let end = raw
                .sections
                .get(index + 1)
                .map_or(usize::MAX, |next| next.line);
            let blocks = section
                .blocks
                .iter()
                .filter_map(|block| match block {
                    document::RawBlock::Markdown(text) => Some(Block::Markdown(text.clone())),
                    document::RawBlock::Part(part) => values[*part].clone().map(|value| {
                        Block::Part(Part {
                            kind: raw.parts[*part].kind.clone(),
                            value,
                        })
                    }),
                })
                .collect();
            Section {
                heading: section.heading.clone(),
                stale: stale_lines
                    .iter()
                    .any(|line| (section.line..end).contains(line)),
                blocks,
            }
        })
        .collect()
}
