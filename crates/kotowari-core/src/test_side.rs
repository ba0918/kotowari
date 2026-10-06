//! `テスト側の指摘`の範囲（TBL-core-047）

use crate::{Finding, FindingKind};
use std::collections::BTreeSet;

/// TBL-core-047 の「`テスト側の指摘`である条件」の列
enum Condition {
    Always,
    InTestFile,
    InTestFileNotSurface,
    /// 表に無い種類
    Never,
}

fn condition(kind: FindingKind) -> Condition {
    match kind {
        FindingKind::RequirementWithoutTest
        | FindingKind::ScenarioWithoutTest
        | FindingKind::TestWithoutId => Condition::Always,
        FindingKind::InvalidMarker | FindingKind::UnresolvedReference => Condition::InTestFile,
        FindingKind::UnparsableFile => Condition::InTestFileNotSurface,
        FindingKind::ChangeStale
        | FindingKind::ChangeIrStale
        | FindingKind::ChangeUncovered
        | FindingKind::ChangeDeferred
        | FindingKind::ChangeConclusionConflict
        | FindingKind::ChangeRecordInvalid
        | FindingKind::AlgorithmWithoutDefinition
        | FindingKind::DeferredWithTest
        | FindingKind::DependsOnDeferred
        | FindingKind::DuplicateField
        | FindingKind::DuplicateId
        | FindingKind::DuplicateTerm
        | FindingKind::EquivalentInvalid
        | FindingKind::EquivalentStale
        | FindingKind::IdDomainMismatch
        | FindingKind::InvalidGlossaryRow
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
        | FindingKind::MutantTimeout
        | FindingKind::RecordFieldMissing
        | FindingKind::RecordFieldUnknown
        | FindingKind::RevisionLinkInvalid
        | FindingKind::SourceInvalid
        | FindingKind::SurfaceUnspecifiedInvalid
        | FindingKind::SurfaceUnspecifiedStale
        | FindingKind::SurfaceWithoutSpec
        | FindingKind::TooManyLines
        | FindingKind::TooManyRequirements
        | FindingKind::UnclosedBacktick
        | FindingKind::UnclosedCodeBlock
        | FindingKind::UnknownCodeBlock
        | FindingKind::UnknownField
        | FindingKind::UnknownHeading
        | FindingKind::UnknownKind
        | FindingKind::UnknownLine
        | FindingKind::UnknownTag
        | FindingKind::UnknownTerm
        | FindingKind::InvalidGherkinLine
        | FindingKind::InvalidId
        | FindingKind::InvalidPlan
        | FindingKind::GlossaryInvalid
        | FindingKind::GlossaryTitleInvalid
        | FindingKind::GuideStale
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
        | FindingKind::LinkToRecord => Condition::Never,
    }
}

/// 検査で読んだ`テストのファイル`と`面のファイル`のパス。指摘だけからは、両方に当たるファイルの
/// unparsable_file がどちらから出たかを見分けられないので、検査の間に集めて持つ
#[derive(Clone, Default)]
pub(crate) struct TestSideFiles {
    pub(crate) tests: BTreeSet<String>,
    pub(crate) surfaces: BTreeSet<String>,
}

impl TestSideFiles {
    /// finding が TBL-core-047 の条件に当たるか。表の種類の重大度はどれも "error" なので、
    /// 重大度は見ない
    pub(crate) fn contains(&self, finding: &Finding) -> bool {
        let in_tests = self.tests.contains(finding.path());
        match condition(finding.kind()) {
            Condition::Always => true,
            Condition::InTestFile => in_tests,
            Condition::InTestFileNotSurface => in_tests && !self.surfaces.contains(finding.path()),
            Condition::Never => false,
        }
    }
}
