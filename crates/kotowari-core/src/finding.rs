//! 指摘の種類、重大度、指摘と、その並べ方と数え方

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
    DuplicateId => "duplicate_id",
    EquivalentInvalid => "equivalent_invalid",
    EquivalentStale => "equivalent_stale",
    IdDomainMismatch => "id_domain_mismatch",
    InvalidMarker => "invalid_marker",
    MissingTitle => "missing_title",
    MultipleTitles => "multiple_titles",
    MutantSurvived => "mutant_survived",
    MutantTimeout => "mutant_timeout",
    SurfaceUnspecifiedInvalid => "surface_unspecified_invalid",
    SurfaceUnspecifiedStale => "surface_unspecified_stale",
    SurfaceWithoutSpec => "surface_without_spec",
    TestWithoutId => "test_without_id",
    UnclosedCodeBlock => "unclosed_code_block",
    UnknownHeading => "unknown_heading",
    UnknownLine => "unknown_line",
    UnparsableFile => "unparsable_file",
    InvalidId => "invalid_id",
    InvalidPlan => "invalid_plan",
    GuideStale => "guide_stale",
    UnresolvedReference => "unresolved_reference",
    VagueWord => "vague_word",
    // 全体像の元データの指摘。core はこの種類を作らず、追加の指摘の群として受け取って並べ、数える
    OverviewFormInvalid => "overview_form_invalid",
    OverviewPartUnknown => "overview_part_unknown",
    OverviewPartInvalid => "overview_part_invalid",
    OverviewLeadMissing => "overview_lead_missing",
    OverviewDocumentMissing => "overview_document_missing",
    OverviewDocumentShared => "overview_document_shared",
    OverviewRefUnresolved => "overview_ref_unresolved",
    OverviewNameConflict => "overview_name_conflict",
    // 目次の指摘。全体像の元データの指摘と同じく kotowari-overview が作る
    OverviewTocInvalid => "overview_toc_invalid",
    OverviewTocPageMissing => "overview_toc_page_missing",
    OverviewTocPageUnknown => "overview_toc_page_unknown",
    OverviewTocPageDuplicate => "overview_toc_page_duplicate",
    OverviewTocGroupEmpty => "overview_toc_group_empty",
    // 対の指摘
    TranslationMissing => "translation_missing",
    TranslationRecordInvalid => "translation_record_invalid",
    TranslationStale => "translation_stale",
    TranslationStructureMismatch => "translation_structure_mismatch",
    TranslationSwitcherInvalid => "translation_switcher_invalid",
    LinkLanguageMismatch => "link_language_mismatch",
    LinkToRecord => "link_to_record",
    // 決定の文書の指摘（docs/decisions/core/decision-documents.md）
    DecisionStateInvalid => "decision_state_invalid",
    DecisionSectionMissing => "decision_section_missing",
    TopicLeadMissing => "topic_lead_missing",
    ConfirmationWithoutTest => "confirmation_without_test",
    LinkTargetMissing => "link_target_missing",
}

impl FindingKind {
    /// 重大度を返す（1か所で管理する）
    pub fn severity(&self) -> &'static str {
        match self {
            FindingKind::MutantTimeout
            | FindingKind::EquivalentStale
            | FindingKind::GuideStale
            | FindingKind::SurfaceUnspecifiedStale => "notice",
            FindingKind::DuplicateId
            | FindingKind::EquivalentInvalid
            | FindingKind::IdDomainMismatch
            | FindingKind::InvalidMarker
            | FindingKind::MissingTitle
            | FindingKind::MultipleTitles
            | FindingKind::MutantSurvived
            | FindingKind::SurfaceUnspecifiedInvalid
            | FindingKind::SurfaceWithoutSpec
            | FindingKind::TestWithoutId
            | FindingKind::UnclosedCodeBlock
            | FindingKind::UnknownHeading
            | FindingKind::UnknownLine
            | FindingKind::UnparsableFile
            | FindingKind::InvalidId
            | FindingKind::InvalidPlan
            | FindingKind::UnresolvedReference
            | FindingKind::VagueWord
            | FindingKind::OverviewFormInvalid
            | FindingKind::OverviewPartUnknown
            | FindingKind::OverviewPartInvalid
            | FindingKind::OverviewLeadMissing
            | FindingKind::OverviewDocumentMissing
            | FindingKind::OverviewDocumentShared
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
            | FindingKind::LinkToRecord
            | FindingKind::DecisionStateInvalid
            | FindingKind::DecisionSectionMissing
            | FindingKind::TopicLeadMissing
            | FindingKind::ConfirmationWithoutTest
            | FindingKind::LinkTargetMissing => "error",
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

/// 指摘を 順（path → line → kind → detail）に並べる
pub fn sort_findings(findings: &mut [Finding]) {
    findings.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            // line は null が先、その後は小さい順（Option の順序が None を先に置く）
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
            .then_with(|| a.detail.cmp(&b.detail))
    });
}

/// 種類ごとの指摘の数。1件も無い種類は持たない
pub fn count_findings(findings: &[Finding]) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for f in findings {
        *counts.entry(f.kind.as_str().to_string()).or_insert(0) += 1;
    }
    counts
}
