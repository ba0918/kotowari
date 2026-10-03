pub mod cargo_mutants;
macro_rules! readonly {
    (copy $($name:ident : $type:ty),* $(,)?) => {
        $(pub fn $name(&self) -> $type { self.$name })*
    };
    (borrow $($name:ident : $type:ty),* $(,)?) => {
        $(pub fn $name(&self) -> &$type { &self.$name })*
    };
}
#[cfg(test)]
extern crate self as kotowari_core;
pub mod change_records;
pub mod changes;
pub mod comparison;
#[cfg(test)]
#[path = "../tests/test_facts.rs"]
mod fact_contract_tests;
#[cfg(test)]
#[path = "../tests/guides.rs"]
mod guide_contract_tests;
#[cfg(test)]
#[path = "../tests/ir.rs"]
mod ir_contract_tests;
#[cfg(test)]
#[path = "../tests/change_matching.rs"]
mod matching_contract_tests;
#[cfg(test)]
#[path = "../tests/change_records.rs"]
mod record_contract_tests;
#[cfg(test)]
#[path = "../tests/surface.rs"]
mod surface_contract_tests;
pub use comparison::Comparison;
pub mod config;
mod deferred;
mod deferred_notices;
mod doc_kind;
pub mod equivalents;
mod finding_map;
mod fingerprint;
pub mod guides;
mod inputs;
pub mod ir;
pub use inputs::{
    CheckInputs, CheckReport, InputError, Inspection, QueryReport, ReadInputs, ReadList, ReadModel,
    RepositoryCheckInputs, RepositoryReadInputs, SourceText, StatusReport, SurfaceAnalysis,
    TestAnalysis,
};
pub use ir::{IrOptions, ParsedIrDocument as IrDocument, ParsedItem};
mod list;
pub use list::{ExampleItem, FlagItem, ListItem, RequirementItem, ScenarioItem, TestRef};
mod markdown;
pub mod mutants;
pub mod plan;
mod query;
pub use query::{QueryItem, Reference};
mod record_form;
mod schema;
pub mod sources;
mod status;
pub use status::{Documents, Findings, Items, Requirements, Scenarios, Tests};
pub mod surface;
pub mod terms;
mod test_markers;
pub mod tests_discovery;

use std::collections::BTreeMap;

/// 検査結果の最上位の構造
#[derive(Clone, serde::Serialize)]
pub(crate) struct CheckResult {
    pub(crate) files: usize,
    pub lines: usize,
    pub findings: Vec<Finding>,
    pub counts: BTreeMap<String, usize>,
    pub tests: BTreeMap<String, TestFileTally>,
    /// REQ-core-206: 読んだ`ガイド`の数と、形の正しい`ガイドの印`の1件の数
    pub guides: guides::GuideTally,
    /// REQ-core-228: "surface.rules" が空の一覧でないときだけ出す
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<surface::Unlisted>,
}

/// 読んだテストのファイルの、1つの拡張子の数と問い合わせの有無（TBL-core-021）
#[derive(Debug, Clone, serde::Serialize)]
pub struct TestFileTally {
    /// その拡張子の読んだテストのファイルの数
    pub(crate) files: usize,
    /// その拡張子が問い合わせのある言語か
    pub(crate) query: bool,
}

impl TestFileTally {
    readonly!(copy files: usize, query: bool);
}

/// 指摘の種類と、その JSON での文字列。
/// この呼び出しの一覧が変種の唯一の在り処で、列挙体・`ALL`・`as_str` はここから作る。
macro_rules! finding_kinds {
    ($($variant:ident => $text:literal,)+) => {
        /// 指摘の種類
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum FindingKind {
            $($variant,)+
        }

        impl FindingKind {
            /// 出しうる種類のすべて
            pub const ALL: &'static [FindingKind] = &[$(FindingKind::$variant),+];

            /// 種類を文字列に変換する（JSON 出力・整列・counts のキーに使う）
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(FindingKind::$variant => $text),+
                }
            }
        }
    };
}

finding_kinds! {
    ChangeStale => "change_stale",
    ChangeIrStale => "change_ir_stale",
    ChangeUncovered => "change_uncovered",
    ChangeDeferred => "change_deferred",
    ChangeConclusionConflict => "change_conclusion_conflict",
    ChangeRecordInvalid => "change_record_invalid",
    AlgorithmWithoutDefinition => "algorithm_without_definition",
    DeferredWithTest => "deferred_with_test",
    DependsOnDeferred => "depends_on_deferred",
    DuplicateField => "duplicate_field",
    DuplicateId => "duplicate_id",
    DuplicateTerm => "duplicate_term",
    EquivalentInvalid => "equivalent_invalid",
    EquivalentStale => "equivalent_stale",
    IdDomainMismatch => "id_domain_mismatch",
    InvalidGlossaryRow => "invalid_glossary_row",
    InvalidMarker => "invalid_marker",
    MissingDocument => "missing_document",
    MissingField => "missing_field",
    MissingScope => "missing_scope",
    MissingSource => "missing_source",
    MissingStatement => "missing_statement",
    MissingTable => "missing_table",
    MissingTag => "missing_tag",
    MissingTitle => "missing_title",
    MultipleTitles => "multiple_titles",
    MutantSurvived => "mutant_survived",
    MutantTimeout => "mutant_timeout",
    RecordFieldMissing => "record_field_missing",
    RecordFieldUnknown => "record_field_unknown",
    RequirementWithoutTest => "requirement_without_test",
    RevisionLinkInvalid => "revision_link_invalid",
    ScenarioWithoutTest => "scenario_without_test",
    SourceInvalid => "source_invalid",
    SurfaceUnspecifiedInvalid => "surface_unspecified_invalid",
    SurfaceUnspecifiedStale => "surface_unspecified_stale",
    SurfaceWithoutSpec => "surface_without_spec",
    TestWithoutId => "test_without_id",
    TooManyLines => "too_many_lines",
    TooManyRequirements => "too_many_requirements",
    UnclosedBacktick => "unclosed_backtick",
    UnclosedCodeBlock => "unclosed_code_block",
    UnknownCodeBlock => "unknown_code_block",
    UnknownField => "unknown_field",
    UnknownHeading => "unknown_heading",
    UnknownKind => "unknown_kind",
    UnknownLine => "unknown_line",
    UnknownTag => "unknown_tag",
    UnknownTerm => "unknown_term",
    UnparsableFile => "unparsable_file",
    InvalidGherkinLine => "invalid_gherkin_line",
    InvalidId => "invalid_id",
    InvalidPlan => "invalid_plan",
    GlossaryInvalid => "glossary_invalid",
    GlossaryTitleInvalid => "glossary_title_invalid",
    GuideStale => "guide_stale",
    UnresolvedReference => "unresolved_reference",
    VagueWord => "vague_word",
    VerificationInvalid => "verification_invalid",
    VerificationMissing => "verification_missing",
}

impl FindingKind {
    /// 重大度を返す（1か所で管理する）
    pub fn severity(&self) -> &'static str {
        match self {
            FindingKind::TooManyLines
            | FindingKind::TooManyRequirements
            | FindingKind::MutantTimeout
            | FindingKind::EquivalentStale
            | FindingKind::GuideStale
            | FindingKind::SurfaceUnspecifiedStale
            | FindingKind::DeferredWithTest
            | FindingKind::DependsOnDeferred => "notice",
            _ => "error",
        }
    }
}

impl std::fmt::Display for FindingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<&str> for FindingKind {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<str> for FindingKind {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl serde::Serialize for FindingKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// 指摘
#[derive(Debug, serde::Serialize, Clone)]
/// A diagnostic whose kind, severity and source identity cannot be changed independently.
/// ```compile_fail
/// let mut finding = kotowari_core::Finding::new(
///     kotowari_core::FindingKind::MissingTitle, "topic.md".into(), None, String::new());
/// finding.severity = "notice".into();
/// ```
pub struct Finding {
    pub(crate) kind: FindingKind,
    pub(crate) severity: String,
    pub(crate) path: String,
    pub(crate) line: Option<usize>,
    pub(crate) detail: String,
}

impl Finding {
    pub fn kind(&self) -> FindingKind {
        self.kind
    }
    pub fn severity(&self) -> &str {
        &self.severity
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn line(&self) -> Option<usize> {
        self.line
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
    /// 指摘を作る。severity は kind から自動で決まる。
    pub fn new(kind: FindingKind, path: String, line: Option<usize>, detail: String) -> Self {
        Finding {
            severity: kind.severity().to_string(),
            kind,
            path,
            line,
            detail,
        }
    }
}

/// 出力の形式

/// 停止の理由と、標準エラーの1行目で詳細の前に出る文言（TBL-core-018）。
/// この呼び出しの一覧が変種の唯一の在り処で、列挙体・`WORDINGS`・`Display` はここから作る。
macro_rules! stop_reasons {
    ($($variant:ident => $wording:literal,)+) => {
        /// 停止の理由
        #[derive(Debug)]
        pub enum StopReason {
            $($variant(String)),+
        }

        impl StopReason {
            /// 標準エラーの1行目に出しうる文言のすべて
            pub const WORDINGS: &'static [&'static str] = &[$($wording),+];

            /// この理由の文言と詳細
            fn wording_and_detail(&self) -> (&'static str, &str) {
                match self {
                    $(StopReason::$variant(detail) => ($wording, detail)),+
                }
            }
        }

        impl std::fmt::Display for StopReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let (wording, detail) = self.wording_and_detail();
                write!(f, "{wording}: {detail}")
            }
        }
    };
}

stop_reasons! {
    ArgumentError => "argument error",
    ConfigError => "config error",
    UnreadableFile => "unreadable file",
    NonUtf8File => "non-UTF-8 file",
    ResultsError => "results error",
    MappingError => "mapping error",
    GitError => "git error",
}

/// 結果のファイルを作った変異テストの道具（REQ-core-149）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    CargoMutants,
}

/// REQ-core-111: 読むファイルの先頭の UTF-8 BOM (U+FEFF) を読み飛ばす。
/// 読むファイルはどれもここを通す（`read_utf8_file` と `read_source_lines`）。
pub fn strip_bom(text: &str) -> &str {
    text.strip_prefix('\u{FEFF}').unwrap_or(text)
}

/// パスを正規化する純粋な関数。
/// 末尾の "/"、先頭の "./"、途中の "/./" と連続する "/"、"\" を正規化し、"a/.." を畳む。
pub fn normalize_path(path: &str) -> String {
    let s = path.replace('\\', "/");
    let mut parts: Vec<&str> = Vec::new();
    for part in s.split('/') {
        if part == "." || part.is_empty() {
            // 先頭の空（= 先頭の "/"）は保持しない（相対パスの前提）
            // 途中の空（= 連続する "/"）は飛ばす
            // "." は飛ばす
            continue;
        }
        // "a/.." は畳む。畳む相手の無い ".." は残し、基準の外を指すパスのままにする
        // （review7-gaps の A1）
        if part == ".." && parts.last().is_some_and(|last| *last != "..") {
            parts.pop();
            continue;
        }
        parts.push(part);
    }
    if parts.is_empty() {
        // REQ-core-110: "." や "./" は空の置き場になる（呼び出し元が文書名だけの path を作る）
        String::new()
    } else {
        parts.join("/")
    }
}

/// 置き場と文書名を "/" でつなぐ。置き場が空なら文書名だけにする（REQ-core-110）。
pub fn join_display_path(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        format!("{dir}/{name}")
    }
}

/// 二重引用符の外のバッククォートの数が奇数か（REQ-core-116。二重引用符の中は A145 で対象外）。
/// ir モジュールと terms モジュールの両方から使う
pub fn has_odd_backticks_outside_quotes(text: &str) -> bool {
    split_outside_quotes(text)
        .join("")
        .chars()
        .filter(|&c| c == '`')
        .count()
        % 2
        != 0
}

/// 二重引用符の外の部分からバッククォートで囲んだ語を集める（REQ-core-054, REQ-core-064, REQ-core-104）。
/// ir モジュールと terms モジュールの両方から使う
pub fn extract_backtick_contents_outside_quotes(text: &str) -> Vec<&str> {
    // 二重引用符の外のバッククォートを行の左から順に対にし、中身は対の間の元の文字にする。
    // 断片ごとに対にすると、引用符を挟んだ囲みが崩れて後の囲みと組み違える（review5-gaps の A1）
    let base = text.as_ptr() as usize;
    let backticks: Vec<usize> = split_outside_quotes(text)
        .into_iter()
        .flat_map(|part| {
            let offset = part.as_ptr() as usize - base;
            part.match_indices('`').map(move |(i, _)| offset + i)
        })
        .collect();
    backticks
        .chunks_exact(2)
        .map(|pair| &text[pair[0] + 1..pair[1]])
        .collect()
}

/// 二重引用符を行の左から順に対にし、対の中身を集める。対にならない最後の引用符は捨てる（REQ-core-226）
pub fn double_quoted_contents(line: &str) -> Vec<&str> {
    let quotes: Vec<usize> = line.match_indices('"').map(|(i, _)| i).collect();
    quotes
        .chunks_exact(2)
        .map(|pair| &line[pair[0] + 1..pair[1]])
        .collect()
}

/// TBL-core-014: 引用符が奇数のときは最後の引用符から行末を引用の中とみなす。
/// ir モジュールと terms モジュールの両方から使う（REQ-core-054, REQ-core-064, REQ-core-104）。
pub fn split_outside_quotes(line: &str) -> Vec<&str> {
    let quote_count = line.chars().filter(|&c| c == '"').count();
    let odd_quotes = quote_count % 2 != 0;

    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_quote = false;
    let last_quote_pos = if odd_quotes {
        line.rfind('"').unwrap_or(0)
    } else {
        0
    };

    for (i, c) in line.char_indices() {
        if c == '"' {
            if odd_quotes && i == last_quote_pos {
                parts.push(&line[start..i]);
                return parts;
            }
            if !in_quote {
                parts.push(&line[start..i]);
                in_quote = true;
            } else {
                in_quote = false;
                start = i + 1;
            }
        }
    }
    parts.push(&line[start..]);
    parts
}

/// 文書の全項目から ID の集合を作る（形に合う ID だけ）
pub fn collect_known_ids(docs: &[ir::IrDocument]) -> std::collections::BTreeSet<String> {
    docs.iter()
        .flat_map(|d| d.items.iter())
        .filter_map(|item| item.id().map(|s| s.to_string()))
        .filter(|id| ir::is_valid_id(id))
        .collect()
}

/// REQ-core-024: 指摘を TBL-core-007 の順（path → line → kind → detail）に並べる
pub fn sort_findings(findings: &mut [Finding]) {
    findings.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            // TBL-core-007: line は null が先、その後は小さい順（Option の順序が None を先に置く）
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
            .then_with(|| a.detail.cmp(&b.detail))
    });
}

/// PROP-core-002: 種類ごとの指摘の数。1件も無い種類は持たない
pub fn count_findings(findings: &[Finding]) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for f in findings {
        *counts.entry(f.kind.as_str().to_string()).or_insert(0) += 1;
    }
    counts
}

/// check、list、query、status が共有する読み取りの結果
/// （REQ-core-151、REQ-core-156、REQ-core-162: どれも check と同じ読み取りを使う）

impl std::error::Error for StopReason {}
