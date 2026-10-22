//! 指摘の種類、重大度、指摘と、その並べ方と数え方（TBL-core-005、TBL-core-007）

use std::collections::BTreeMap;

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
    // 全体像の元データの指摘（TBL-core-008）。core はこの種類を作らず、追加の指摘の群として受け取って並べ、数える
    OverviewFormInvalid => "overview_form_invalid",
    OverviewPartUnknown => "overview_part_unknown",
    OverviewPartInvalid => "overview_part_invalid",
    OverviewLeadMissing => "overview_lead_missing",
    OverviewIrMissing => "overview_ir_missing",
    OverviewIrShared => "overview_ir_shared",
    OverviewRefUnresolved => "overview_ref_unresolved",
    OverviewNameConflict => "overview_name_conflict",
    // 目次の指摘（TBL-core-008）。全体像の元データの指摘と同じく kotowari-overview が作る
    OverviewTocInvalid => "overview_toc_invalid",
    OverviewTocPageMissing => "overview_toc_page_missing",
    OverviewTocPageUnknown => "overview_toc_page_unknown",
    OverviewTocPageDuplicate => "overview_toc_page_duplicate",
    OverviewTocGroupEmpty => "overview_toc_group_empty",
    // 対の指摘（TBL-core-008）
    TranslationMissing => "translation_missing",
    TranslationRecordInvalid => "translation_record_invalid",
    TranslationStale => "translation_stale",
    TranslationStructureMismatch => "translation_structure_mismatch",
    TranslationSwitcherInvalid => "translation_switcher_invalid",
    LinkLanguageMismatch => "link_language_mismatch",
    LinkToRecord => "link_to_record",
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
            FindingKind::AlgorithmWithoutDefinition
            | FindingKind::DuplicateField
            | FindingKind::DuplicateId
            | FindingKind::DuplicateTerm
            | FindingKind::EquivalentInvalid
            | FindingKind::IdDomainMismatch
            | FindingKind::InvalidGlossaryRow
            | FindingKind::InvalidMarker
            | FindingKind::MissingDocument
            | FindingKind::MissingField
            | FindingKind::MissingScope
            | FindingKind::MissingSource
            | FindingKind::MissingStatement
            | FindingKind::MissingTable
            | FindingKind::MissingTag
            | FindingKind::MissingTitle
            | FindingKind::MultipleTitles
            | FindingKind::MutantSurvived
            | FindingKind::RecordFieldMissing
            | FindingKind::RecordFieldUnknown
            | FindingKind::RequirementWithoutTest
            | FindingKind::RevisionLinkInvalid
            | FindingKind::ScenarioWithoutTest
            | FindingKind::SourceInvalid
            | FindingKind::SurfaceUnspecifiedInvalid
            | FindingKind::SurfaceWithoutSpec
            | FindingKind::TestWithoutId
            | FindingKind::UnclosedBacktick
            | FindingKind::UnclosedCodeBlock
            | FindingKind::UnknownCodeBlock
            | FindingKind::UnknownField
            | FindingKind::UnknownHeading
            | FindingKind::UnknownKind
            | FindingKind::UnknownLine
            | FindingKind::UnknownTag
            | FindingKind::UnknownTerm
            | FindingKind::UnparsableFile
            | FindingKind::InvalidGherkinLine
            | FindingKind::InvalidId
            | FindingKind::InvalidPlan
            | FindingKind::GlossaryInvalid
            | FindingKind::GlossaryTitleInvalid
            | FindingKind::UnresolvedReference
            | FindingKind::VagueWord
            | FindingKind::VerificationInvalid
            | FindingKind::VerificationMissing
            | FindingKind::OverviewFormInvalid
            | FindingKind::OverviewPartUnknown
            | FindingKind::OverviewPartInvalid
            | FindingKind::OverviewLeadMissing
            | FindingKind::OverviewIrMissing
            | FindingKind::OverviewIrShared
            | FindingKind::OverviewRefUnresolved
            | FindingKind::OverviewNameConflict
            | FindingKind::OverviewTocInvalid
            | FindingKind::OverviewTocPageMissing
            | FindingKind::OverviewTocPageUnknown
            | FindingKind::OverviewTocPageDuplicate
            | FindingKind::OverviewTocGroupEmpty
            | FindingKind::TranslationMissing
            | FindingKind::TranslationRecordInvalid
            | FindingKind::TranslationStale
            | FindingKind::TranslationStructureMismatch
            | FindingKind::TranslationSwitcherInvalid
            | FindingKind::LinkLanguageMismatch
            | FindingKind::LinkToRecord => "error",
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
    pub(crate) severity: &'static str,
    pub(crate) path: String,
    pub(crate) line: Option<usize>,
    pub(crate) detail: String,
}

impl Finding {
    pub fn kind(&self) -> FindingKind {
        self.kind
    }
    pub fn severity(&self) -> &str {
        self.severity
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
            severity: kind.severity(),
            kind,
            path,
            line,
            detail,
        }
    }
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
