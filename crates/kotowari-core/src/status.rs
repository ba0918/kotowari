//! "kotowari status" の集計（docs/decisions/core/status.md）

use crate::guides::GuideTally;
use crate::surface::SurfaceTally;
use crate::tests_discovery::TestMarker;
use crate::{Finding, TestFileTally};
use serde::Serialize;
use std::collections::BTreeMap;

/// "kotowari status" の出力の最上位
#[derive(Debug, Clone, Serialize)]
pub struct StatusResult {
    pub documents: Documents,
    pub tests: Tests,
    /// "kotowari check" の "guides" と同じ
    pub guides: GuideTally,
    /// `面`の数。"surface.rules" が空の一覧なら3つとも 0
    pub surface: SurfaceTally,
    pub findings: Findings,
    /// 状態ごとの決定の数（C-core-confirm-9）
    pub decisions: crate::decisions::DecisionStates,
    /// 確認することの行の数（C-core-status-13）
    pub confirmations: crate::decisions::ConfirmationCounts,
    /// `誤り`が0件で、決定の文書の「未決」の節に項目が無いときだけ true（C-core-docs-12、C-core-docs-13）
    pub complete: bool,
    /// 追加の指摘の群の数。check と同じ
    #[serde(skip)]
    pub(crate) groups: Vec<crate::GroupTally>,
}

/// 読んだ決定の文書の数と行の数（C-core-status-10）
#[derive(Debug, Clone, Default, Serialize)]
pub struct Documents {
    pub(crate) files: usize,
    pub(crate) lines: usize,
}
impl Documents {
    readonly!(copy files: usize, lines: usize);
}

/// `印`と読んだ`テストのファイル`
#[derive(Debug, Clone, Serialize)]
pub struct Tests {
    /// `印`の出現の数。1つの`印`に `ID` が複数あれば `ID` ごとに1つ
    pub(crate) marks: usize,
    /// "kotowari check" の "tests" と同じ
    pub(crate) files: BTreeMap<String, TestFileTally>,
}
impl Tests {
    readonly!(copy marks: usize);
    readonly!(borrow files: BTreeMap<String, TestFileTally>);
}

/// "kotowari check" の`指摘`の数
#[derive(Debug, Clone, Default, Serialize)]
pub struct Findings {
    pub(crate) error: usize,
    pub(crate) notice: usize,
}
impl Findings {
    readonly!(copy error: usize, notice: usize);
}

/// check と同じ読み取りと検査の結果から集計を作る（docs/decisions/core/status.md）
pub(crate) fn build(
    documents: &[crate::decisions::Document],
    markers: &[TestMarker],
    tally: BTreeMap<String, TestFileTally>,
    guides: GuideTally,
    surface: SurfaceTally,
    findings: &[Finding],
) -> StatusResult {
    let mut counts = Findings::default();
    for finding in findings {
        if finding.severity == "error" {
            counts.error += 1;
        } else {
            counts.notice += 1;
        }
    }
    StatusResult {
        documents: Documents {
            files: documents.len(),
            lines: documents.iter().map(|doc| doc.line_count).sum(),
        },
        // 誤りが0件で、決定の文書の「未決」の節に項目が無いときだけ complete
        complete: counts.error == 0 && !crate::decisions::has_open_items(documents),
        tests: Tests {
            marks: markers.len(),
            files: tally,
        },
        guides,
        surface,
        findings: counts,
        decisions: crate::decisions::states(documents),
        confirmations: crate::decisions::confirmation_counts(documents, markers),
        groups: Vec::new(),
    }
}
