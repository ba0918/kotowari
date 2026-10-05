//! 全体像の元データを検査し、参照の表と古い節を求めて描画の入力を作る。
//!
//! 仕様は `docs/ir/core/overview-data.md` と `docs/ir/core/overview-commands.md` にある。
//! このクレートはファイル、ネットワーク、環境変数に触れない。元データの文字列と、kotowari-core で
//! 読み込み済みの IR と判断の記録だけから計算する（REQ-core-287）。

mod document;
mod form;
mod parts;
mod references;
mod toc;

use kotowari_core::{DocKind, Finding, FindingGroup, FindingKind, ReadModel, SourceText};
pub use kotowari_markdown_view::Page;
use kotowari_markdown_view::{
    Block, Document, OtherLanguage, Part, Reference, RenderInput, Section,
};
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
    /// `言語の一覧`の順の、ページの置き場（`先頭の言語`は空、ほかは "<言語タグ>/"）と描画の入力
    inputs: Vec<(String, RenderInput)>,
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
    /// `先頭の言語`のページの描画の入力。誤りがあるときは描ける文書だけを持つ
    pub fn render_input(&self) -> &RenderInput {
        &self.inputs[0].1
    }
    /// 誤りが無ければ描画のエンジンが返すページの並び、あれば誤りの件数（REQ-core-294）。
    /// `先頭の言語`のページは置き場の直下に、ほかの言語のページは "<言語タグ>/" の下に同じ名前で
    /// 置く（REQ-core-353）
    pub fn pages(&self) -> Result<Vec<Page>, usize> {
        match self.errors() {
            0 => Ok(self
                .inputs
                .iter()
                .flat_map(|(place, input)| {
                    kotowari_markdown_view::render(input)
                        .into_iter()
                        .map(move |page| Page {
                            name: format!("{place}{}", page.name),
                            content: page.content,
                        })
                })
                .collect()),
            errors => Err(errors),
        }
    }
    /// core の検査に渡す "overview" の群（REQ-core-290）
    pub fn into_group(self) -> FindingGroup {
        FindingGroup::new(GROUP, self.files, self.marks, self.findings)
    }
}

/// `先頭の言語`でない1つの言語の、`全体像の元データ`と`目次`と`IR`の`側`（REQ-core-343、REQ-core-354）
#[derive(Debug, Default)]
pub struct Translation {
    /// 言語タグ
    pub language: String,
    /// その言語の`側`の`全体像の元データ`。path はその`側`の基準のディレクトリからの相対パス
    pub files: Vec<SourceText>,
    /// その言語の`目次`の`側`。無ければ None
    pub toc: Option<SourceText>,
    /// その言語の`IR`の`側`の文書。参照の本文をここから取る
    pub ir: Vec<kotowari_core::ir::IrDocument>,
}

/// 元データと目次を検査する。files と toc の path は基準のディレクトリからの相対パスである
pub fn inspect(read: &ReadModel, files: &[SourceText], toc: &SourceText) -> Overview {
    inspect_translations(read, files, toc, &[])
}

/// files と toc を`先頭の言語`の`側`とし、translations をほかの言語の`側`として検査する。
/// `全体像の元データ`の形、`部品`、lead、参照、`ガイドの印`と`目次`の形はどの`側`にも、扱う IR の文書、
/// ページの名前の重なり、`目次`との照合は`先頭の言語`の`側`だけに行う（REQ-core-343）。
/// 描画の入力は言語ごとに作る（REQ-core-353〜REQ-core-355）
pub fn inspect_translations(
    read: &ReadModel,
    files: &[SourceText],
    toc: &SourceText,
    translations: &[Translation],
) -> Overview {
    let mut files: Vec<&SourceText> = files.iter().collect();
    files.sort_by(|left, right| left.path().as_bytes().cmp(right.path().as_bytes()));
    let mut inspection = Inspection::new(read);
    let mut read_files = files.len();
    for file in &files {
        if let Some(document) = inspection.file(file, true) {
            inspection.documents.push(document);
        }
    }
    let mut translated = Vec::new();
    for translation in translations {
        read_files += translation.files.len();
        let suffix = format!(".{}", translation.language);
        let documents: Vec<Document> = translation
            .files
            .iter()
            .filter_map(|file| inspection.file(file, false))
            .map(|mut document| {
                // ページの名前は`先頭の言語`の`側`の名前と同じにする（REQ-core-293）
                if let Some(name) = document.name.strip_suffix(&suffix) {
                    document.name = name.to_string();
                }
                document
            })
            .collect();
        translated.push(documents);
    }
    let mut findings = std::mem::take(&mut inspection.findings);
    let names = files
        .iter()
        .map(|file| stem(file_name(file.path())).to_string())
        .collect();
    let checked = toc::check(toc.text(), &names);
    findings.extend(
        checked
            .findings
            .into_iter()
            .map(|(kind, detail)| Finding::new(kind, toc.path().to_string(), None, detail)),
    );
    let mut tocs = Vec::new();
    for translation in translations {
        let Some(toc) = &translation.toc else {
            tocs.push(None);
            continue;
        };
        let checked = toc::check(toc.text(), &names);
        let form = checked.findings.into_iter().filter(|(kind, _)| {
            !matches!(
                kind,
                FindingKind::OverviewTocPageMissing
                    | FindingKind::OverviewTocPageUnknown
                    | FindingKind::OverviewTocPageDuplicate
            )
        });
        findings.extend(
            form.map(|(kind, detail)| Finding::new(kind, toc.path().to_string(), None, detail)),
        );
        tocs.push(checked.toc);
    }
    kotowari_core::sort_findings(&mut findings);
    let languages = Languages::new(read);
    let mut inputs = vec![(
        String::new(),
        RenderInput {
            toc: checked.toc.unwrap_or_default(),
            documents: std::mem::take(&mut inspection.documents),
            references: std::mem::take(&mut inspection.references)
                .into_values()
                .collect(),
            ..languages.input(0)
        },
    )];
    for ((translation, documents), toc) in translations.iter().zip(translated).zip(tocs) {
        let Some(index) = languages.index(&translation.language) else {
            continue;
        };
        let resolver = references::Resolver::for_language(read, &translation.ir);
        let references = references_of(&documents, &resolver);
        inputs.push((
            format!("{}/", translation.language),
            RenderInput {
                toc: toc.unwrap_or_default(),
                documents,
                references,
                ..languages.input(index)
            },
        ));
    }
    Overview {
        findings,
        files: read_files,
        marks: inspection.marks,
        inputs,
    }
}

/// `言語の一覧`と各言語の`UI の文字`（REQ-core-351、REQ-core-355）
struct Languages {
    tags: Vec<String>,
    ui: Vec<BTreeMap<String, String>>,
}

impl Languages {
    fn new(read: &ReadModel) -> Self {
        let tags = read.config().languages();
        let ui = tags
            .iter()
            .map(|tag| read.config().ui_text(tag).entries().clone())
            .collect();
        Self { tags, ui }
    }

    fn index(&self, tag: &str) -> Option<usize> {
        self.tags.iter().position(|other| other == tag)
    }

    /// その言語のページの置き場から、ほかの言語のページの置き場への相対パス
    fn place(from: usize, to: usize, tag: &str) -> String {
        match (from, to) {
            (0, _) => format!("{tag}/"),
            (_, 0) => "../".to_string(),
            _ => format!("../{tag}/"),
        }
    }

    /// その言語の描画の入力のうち、言語タグ、`UI の文字`、ほかの言語（`言語の一覧`の順）
    fn input(&self, index: usize) -> RenderInput {
        let others = self
            .tags
            .iter()
            .enumerate()
            .filter(|(other, _)| *other != index)
            .map(|(other, tag)| OtherLanguage {
                name: self.ui[other]
                    .get("language_name")
                    .cloned()
                    .unwrap_or_default(),
                place: Self::place(index, other, tag),
            })
            .collect();
        RenderInput {
            language: self.tags[index].clone(),
            ui: self.ui[index].clone(),
            others,
            ..RenderInput::default()
        }
    }
}

/// 文書の部品の中の参照の、参照の表（REQ-core-354）
fn references_of(documents: &[Document], resolver: &references::Resolver) -> Vec<Reference> {
    let mut keys = Vec::new();
    for document in documents {
        let blocks = document.sections.iter().flat_map(|section| &section.blocks);
        let parts = std::iter::once(&document.lead)
            .chain(&document.preamble)
            .chain(blocks.filter_map(|block| match block {
                Block::Part(part) => Some(part),
                Block::Markdown(_) => None,
            }));
        for part in parts {
            parts::references(&part.value, &mut keys);
        }
    }
    let mut table = BTreeMap::new();
    for key in keys {
        if !table.contains_key(&key)
            && let Some(reference) = resolver.resolve(&key)
        {
            table.insert(key, reference);
        }
    }
    table.into_values().collect()
}

/// ファイル名から ".md" を除いた名前
fn stem(name: &str) -> &str {
    name.strip_suffix(".md").unwrap_or(name)
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

    /// 1つの元データを検査し、描ける文書を返す。first でなければ`先頭の言語`でない言語の`側`で、
    /// 扱う IR の文書とページの名前の重なりを検査しない（REQ-core-343）
    fn file(&mut self, file: &SourceText, first: bool) -> Option<Document> {
        let (path, text) = (file.path(), file.text());
        let name = file_name(path);
        let stem = stem(name).to_string();
        // REQ-core-305: パスのバイト順で2つ目以降の重なりと、"index" と "style"
        if first && (stem == "index" || stem == "style" || !self.names.insert(stem.clone())) {
            self.error(FindingKind::OverviewNameConflict, path, None, stem.clone());
        }
        let raw = document::parse(text);
        let (ir, mut form) = form::frontmatter(raw.frontmatter.as_deref());
        form.extend(self.form.markdown(text));
        for (line, detail) in form {
            self.error(FindingKind::OverviewFormInvalid, path, line, detail.into());
        }
        if first {
            for entry in ir.unwrap_or_default() {
                self.ir_entry(path, &entry);
            }
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
        let (Some(title), Some(lead)) = (raw.title.clone(), lead) else {
            return None;
        };
        let sections = sections(&raw, &values, &stale_lines);
        let preamble = raw
            .preamble
            .iter()
            .filter_map(|index| {
                values[*index].clone().map(|value| Part {
                    kind: raw.parts[*index].kind.clone(),
                    value,
                })
            })
            .collect();
        Some(Document {
            name: stem,
            title,
            lead: Part {
                kind: "lead".into(),
                value: lead,
            },
            preamble,
            sections,
        })
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
            parts::Checked::Unreadable(at) => {
                // 中身の1行目はフェンスの開始の次の行なので、中身の行をそのまま足すとファイルの行になる
                let line = at.map_or(line, |at| Some(part.line + at));
                let detail = format!("{} (yaml)", part.kind);
                self.error(FindingKind::OverviewPartInvalid, path, line, detail);
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
