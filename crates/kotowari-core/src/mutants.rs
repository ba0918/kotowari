//! 変異の結果の形（A13、A56）と、そこから作る`指摘`と集計。
//! 変異テストの道具に固有の語は持たない（REQ-core-150）。
//! 道具の結果のファイルからの写し取りは道具ごとのモジュール（`crate::cargo_mutants`）が行う。

use crate::equivalents::Equivalent;
use crate::{Finding, FindingKind};
use std::collections::BTreeMap;

/// A52: 前後の半角空白とタブだけを除く（全角空白は文面の一部として残す）
pub fn trim_spaces_and_tabs(s: &str) -> &str {
    s.trim_matches(|c| c == ' ' || c == '\t')
}

/// 変異1件の結果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutantResult {
    /// 捕まえた（変異を入れるとテストが落ちた）
    Caught,
    /// 見逃した（変異を入れてもテストが全部通った）
    Survived,
    /// 時間切れ
    Timeout,
    /// ビルド不能
    Unviable,
}

/// 変異の結果。ファイル、行、変更の説明、結果の4つを持つ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantOutcome {
    /// 変異の入ったファイル（`基準のディレクトリ`からの相対パス。REQ-core-110 の正規化済み）
    pub file: String,
    /// 変異の入った行（1始まり）
    pub line: usize,
    /// 変更の説明（道具が出した文のまま）
    pub change: String,
    /// 結果
    pub result: MutantResult,
}

/// ソースのパスに REQ-core-110 の正規化を掛ける。
/// 絶対パスと ".." の要素を含むものは`基準のディレクトリ`の外を指すので受けない（A51）。
/// 正規化は先頭の "/" を落とすので、判定は正規化の前に行う。
pub fn normalize_source_path(raw: &str) -> Result<String, String> {
    let slashed = raw.replace('\\', "/");
    if slashed.starts_with('/') {
        return Err(format!("absolute path: {raw}"));
    }
    if slashed.split('/').any(|part| part == "..") {
        return Err(format!("path outside the base directory: {raw}"));
    }
    Ok(crate::normalize_path(&slashed))
}

/// "kotowari mutants" の出力（TBL-core-025。最上位はこの3つの鍵だけ）
#[derive(Debug, serde::Serialize)]
pub struct MutantsResult {
    pub findings: Vec<Finding>,
    pub counts: BTreeMap<String, usize>,
    pub mutants: MutantCounts,
}

/// 変異の集計（TBL-core-025、PROP-core-005）
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct MutantCounts {
    pub caught: usize,
    pub survived: usize,
    pub timeout: usize,
    pub unviable: usize,
    pub equivalent: usize,
}

impl MutantCounts {
    /// REQ-core-146: 文字の出力の最後の1行
    pub fn summary_line(&self) -> String {
        format!(
            "mutants: caught={} survived={} timeout={} unviable={} equivalent={}",
            self.caught, self.survived, self.timeout, self.unviable, self.equivalent
        )
    }
}

/// `変異の結果`から`指摘`と集計を作る（REQ-core-139、REQ-core-140、REQ-core-142、REQ-core-145）。
/// 同じ内容の`変異の結果`が2件以上あっても畳まない。
/// `sources` はファイルごとの行の並びで、読めなかったファイルは持たない（REQ-core-141、REQ-core-142）。
/// `list_path` は`等価の一覧`のファイルの相対パス。
pub fn check_outcomes(
    outcomes: &[MutantOutcome],
    equivalents: &[Equivalent],
    list_path: &str,
    sources: &BTreeMap<String, Vec<String>>,
) -> (Vec<Finding>, MutantCounts) {
    let mut findings = Vec::new();
    let mut counts = MutantCounts::default();

    for outcome in outcomes {
        match outcome.result {
            // 捕まえた変異とビルド不能の変異には指摘を出さず、一覧との一致も見ない（REQ-core-139）
            MutantResult::Caught => counts.caught += 1,
            MutantResult::Unviable => counts.unviable += 1,
            // REQ-core-140: 時間切れは一覧との一致を見ない
            MutantResult::Timeout => {
                counts.timeout += 1;
                findings.push(finding(FindingKind::MutantTimeout, outcome));
            }
            MutantResult::Survived => {
                if equivalents
                    .iter()
                    .any(|entry| matches(entry, outcome, sources))
                {
                    counts.equivalent += 1;
                } else {
                    counts.survived += 1;
                    findings.push(finding(FindingKind::MutantSurvived, outcome));
                }
            }
        }
    }

    // REQ-core-142: 文面の無くなった1件（ファイルが無い、読めない、UTF-8 でないときを含む）
    for entry in equivalents {
        if !has_line_with_text(sources, &entry.file, &entry.text) {
            findings.push(Finding::new(
                FindingKind::EquivalentStale,
                list_path.to_string(),
                None,
                entry.detail.clone(),
            ));
        }
    }

    (findings, counts)
}

/// REQ-core-141: "file" と "change" が同じ文字列で、"text" が今のソースのその行の文面と、
/// どちらも前後の半角空白とタブを除いて同じとき一致とする。
/// 行がファイルの行数を超えるときと、ファイルを読めなかったときは一致しない。
fn matches(
    entry: &Equivalent,
    outcome: &MutantOutcome,
    sources: &BTreeMap<String, Vec<String>>,
) -> bool {
    if entry.file != outcome.file || entry.change != outcome.change {
        return false;
    }
    let Some(line) = sources
        .get(&entry.file)
        .and_then(|lines| lines.get(outcome.line - 1))
    else {
        return false;
    };
    trim_spaces_and_tabs(line) == trim_spaces_and_tabs(&entry.text)
}

/// その文面の行がファイルに1つでもあるか（REQ-core-141: 同じ文面の行が複数あればどの行にも効く）
fn has_line_with_text(sources: &BTreeMap<String, Vec<String>>, file: &str, text: &str) -> bool {
    sources.get(file).is_some_and(|lines| {
        lines
            .iter()
            .any(|line| trim_spaces_and_tabs(line) == trim_spaces_and_tabs(text))
    })
}

/// TBL-core-006、TBL-core-019: path はファイル、line は変異の結果の行、detail は変更の説明
fn finding(kind: FindingKind, outcome: &MutantOutcome) -> Finding {
    Finding::new(
        kind,
        outcome.file.clone(),
        Some(outcome.line),
        outcome.change.clone(),
    )
}
