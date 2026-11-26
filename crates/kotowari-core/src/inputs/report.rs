//! check と status の報告の外側の型

use super::*;

pub struct StatusReport(pub(super) crate::status::StatusResult);

impl StatusReport {
    pub fn complete(&self) -> bool {
        self.0.complete
    }
    pub fn documents(&self) -> &crate::status::Documents {
        &self.0.documents
    }
    pub fn tests(&self) -> &crate::status::Tests {
        &self.0.tests
    }
    pub fn guides(&self) -> &crate::guides::GuideTally {
        &self.0.guides
    }
    pub fn surface(&self) -> &crate::surface::SurfaceTally {
        &self.0.surface
    }
    pub fn findings(&self) -> &crate::status::Findings {
        &self.0.findings
    }
    /// 状態ごとの決定の数
    pub fn decisions(&self) -> &crate::DecisionStates {
        &self.0.decisions
    }
    /// 確認することの行の数（C-core-status-13）
    pub fn confirmations(&self) -> &crate::ConfirmationCounts {
        &self.0.confirmations
    }
    /// 追加の指摘の群の数。渡した順に並ぶ
    pub fn groups(&self) -> &[GroupTally] {
        &self.0.groups
    }
    pub fn group(&self, name: &str) -> Option<&GroupTally> {
        find_group(&self.0.groups, name)
    }
}

pub struct CheckReport(pub(super) crate::CheckResult);

impl CheckReport {
    pub fn guides(&self) -> &crate::guides::GuideTally {
        &self.0.guides
    }
    pub fn surface(&self) -> Option<&crate::surface::Unlisted> {
        self.0.surface.as_ref()
    }
    pub fn findings(&self) -> &[crate::Finding] {
        &self.0.findings
    }
    /// 決定の文書の置き場から読んだものの数
    pub fn decisions(&self) -> &crate::DecisionTally {
        &self.0.decisions
    }
    pub fn tests(&self) -> &BTreeMap<String, crate::TestFileTally> {
        &self.0.tests
    }
    pub fn counts(&self) -> &BTreeMap<String, usize> {
        &self.0.counts
    }
    /// 追加の指摘の群の数。渡した順に並ぶ
    pub fn groups(&self) -> &[GroupTally] {
        &self.0.groups
    }
    pub fn group(&self, name: &str) -> Option<&GroupTally> {
        find_group(&self.0.groups, name)
    }
    /// finding が`テスト側の指摘`か。"--allow-test-findings" の終了コードの判断に使う
    pub fn is_test_side_finding(&self, finding: &crate::Finding) -> bool {
        self.0.test_side.contains(finding)
    }
}
