//! 解析した IR 文書と`項目`の型

use super::*;

/// 解析した項目
#[derive(Debug, Clone)]
pub enum Item {
    Requirement {
        id: String,
        name: String,
        line: usize,
        /// `項目`の最後の行。スキーマの側が返す（REQ-core-170）
        end: usize,
        kind: Option<String>,
        sources: Vec<String>,
        /// "- source:" の行。行が無ければ None
        source_line: Option<usize>,
        verification: Option<String>,
        definitions: Vec<String>,
        /// "- definition:" の行。行が無ければ None
        definition_line: Option<usize>,
        /// TBL-core-011: "- how_to_verify:" の値。人が確かめる手順の自由文
        how_to_verify: Option<String>,
        /// 見出しの下の "- deferred:" の行（REQ-core-208）。行が無ければ None
        deferred: Option<Deferral>,
        statements: Vec<(usize, String)>,
    },
    DecisionTable {
        id: String,
        name: String,
        line: usize,
        end: usize,
        sources: Vec<String>,
        source_line: Option<usize>,
        statements: Vec<(usize, String)>,
    },
    Property {
        id: String,
        name: String,
        line: usize,
        end: usize,
        sources: Vec<String>,
        source_line: Option<usize>,
        statements: Vec<(usize, String)>,
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
        end: usize,
        kind: Option<String>,
        relations: Vec<String>,
        /// "- related:" の行。行が無ければ None
        relation_line: Option<usize>,
        sources: Vec<String>,
        source_line: Option<usize>,
        body: Vec<(usize, String)>,
    },
    GlossaryTerm {
        term: String,
        meaning: String,
        sources: Vec<String>,
        line: usize,
    },
}

/// "- deferred:" の行1つ。値は`出典`の並びで、空でもよい（REQ-core-208、REQ-core-210）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deferral {
    /// コンマで区切った`出典`の並び（空の値は除く）
    pub sources: Vec<String>,
    /// その行
    pub line: usize,
}

/// REQ-core-049: "- verification:" に書ける4つの値
pub const VERIFICATION_VALUES: [&str; 4] = ["unit", "property", "proof", "review"];

impl Item {
    pub fn id(&self) -> Option<&str> {
        match self {
            Item::Requirement { id, .. }
            | Item::DecisionTable { id, .. }
            | Item::Property { id, .. }
            | Item::FlagEntry { id, .. } => Some(id),
            Item::Scenario { id, .. } => id.as_deref(),
            Item::GlossaryTerm { .. } => None,
        }
    }

    /// `項目`の最後の行（REQ-core-170）。`シナリオ`と`用語`は持たない
    pub fn end_line(&self) -> Option<usize> {
        match self {
            Item::Requirement { end, .. }
            | Item::DecisionTable { end, .. }
            | Item::Property { end, .. }
            | Item::FlagEntry { end, .. } => Some(*end),
            Item::Scenario { .. } | Item::GlossaryTerm { .. } => None,
        }
    }

    pub fn item_line(&self) -> usize {
        match self {
            Item::Requirement { line, .. }
            | Item::DecisionTable { line, .. }
            | Item::Property { line, .. }
            | Item::Scenario { line, .. }
            | Item::FlagEntry { line, .. }
            | Item::GlossaryTerm { line, .. } => *line,
        }
    }
}

/// 解析した IR 文書
/// ```compile_fail
/// let mut document = kotowari_core::ir::parse_document("topic.md", "# Topic\n").unwrap();
/// document.items.clear();
/// ```
#[derive(Debug)]
pub struct IrDocument {
    pub(crate) relative_path: String,
    pub(crate) directory: String,
    pub(crate) kind: DocKind,
    /// `文書が扱う範囲`の`文`の行（行番号と行の文字そのまま）
    pub(crate) scope_lines: Vec<(usize, String)>,
    /// 文書単位の "- deferred:" の行（REQ-core-209）。2つ目以降の行は読まない
    pub(crate) deferred: Option<Deferral>,
    pub(crate) line_count: usize,
    pub(crate) items: Vec<Item>,
    pub(crate) raw_content: String,
    /// 文書1つで決まる指摘（スキーマの側から写したもの、gherkin の中身、閉じないコードブロック、
    /// 用語集の行）。パスは空で、`check_documents` が入れる
    pub(crate) parse_findings: Vec<crate::Finding>,
}

impl IrDocument {
    /// 相対パスの最後の段
    pub fn filename(&self) -> &str {
        self.relative_path
            .rsplit('/')
            .next()
            .unwrap_or(&self.relative_path)
    }
    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }
    pub fn kind(&self) -> DocKind {
        self.kind
    }
    pub fn line_count(&self) -> usize {
        self.line_count
    }
    pub fn items(&self) -> &[Item] {
        &self.items
    }
    pub(crate) fn is_glossary_in_chain(&self, directory: &str) -> bool {
        self.kind == DocKind::Glossary
            && (self.directory.is_empty()
                || self.directory == directory
                || directory
                    .strip_prefix(&self.directory)
                    .is_some_and(|rest| rest.starts_with('/')))
    }

    pub(crate) fn duplicate_glossary_rows(&self, docs: &[IrDocument]) -> BTreeSet<usize> {
        let ancestors: BTreeSet<&str> = docs
            .iter()
            .filter(|doc| {
                doc.directory != self.directory && doc.is_glossary_in_chain(&self.directory)
            })
            .flat_map(|doc| &doc.items)
            .filter_map(|item| match item {
                Item::GlossaryTerm { term, .. } => Some(term.as_str()),
                Item::Requirement { .. }
                | Item::DecisionTable { .. }
                | Item::Property { .. }
                | Item::Scenario { .. }
                | Item::FlagEntry { .. } => None,
            })
            .collect();
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::GlossaryTerm { term, line, .. } if ancestors.contains(term.as_str()) => {
                    Some(*line)
                }
                Item::Requirement { .. }
                | Item::DecisionTable { .. }
                | Item::Property { .. }
                | Item::Scenario { .. }
                | Item::FlagEntry { .. }
                | Item::GlossaryTerm { .. } => None,
            })
            .collect()
    }

    /// 文書の`文`の行（`文書が扱う範囲`、`項目`の`文`、`問題の記録`の本文）。
    /// `文書名の参照`はこの行の上で探す（REQ-core-170）
    pub fn statement_lines(&self) -> impl Iterator<Item = &(usize, String)> {
        let item_lines = self.items.iter().flat_map(|item| match item {
            Item::Requirement { statements, .. }
            | Item::DecisionTable { statements, .. }
            | Item::Property { statements, .. }
            | Item::FlagEntry {
                body: statements, ..
            } => statements.as_slice(),
            Item::Scenario { .. } | Item::GlossaryTerm { .. } => &[],
        });
        self.scope_lines.iter().chain(item_lines)
    }
}

/// 文書群に対して一度求めた、親の用語集と重なる行。
pub(crate) struct GlossaryDuplicates(Vec<BTreeSet<usize>>);

impl GlossaryDuplicates {
    pub(crate) fn new(docs: &[IrDocument]) -> Self {
        Self(
            docs.iter()
                .map(|doc| doc.duplicate_glossary_rows(docs))
                .collect(),
        )
    }

    pub(crate) fn rows(&self) -> impl Iterator<Item = &BTreeSet<usize>> {
        self.0.iter()
    }
}
