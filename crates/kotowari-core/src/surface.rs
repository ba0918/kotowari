//! `面`の検査（docs/decisions/core/surface.md）。`面の規則`で`面のファイル`から`面`を取り出す。

use crate::split_lines;
use crate::{Finding, FindingKind, StopReason};
use serde_json::Value;
use std::collections::BTreeSet;

/// `面の規則`で取り出した`面`1つ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Surface {
    /// 当てた`面の規則`の "id"
    pub kind: String,
    /// "$NAME" に入った節の文字から、前後の同じ引用符を1組外したもの
    pub name: String,
    /// `面のファイル`の`基準のディレクトリ`からの相対パス
    pub path: String,
    /// 節の最初の行
    pub line: usize,
}

/// `面`の種類と名前の組の数
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize)]
pub struct SurfaceTally {
    /// `面`の種類と名前の組の数
    pub(crate) total: usize,
    /// そのうち決定の文書にあるものの数
    pub(crate) specified: usize,
    /// 決定の文書になく`未記載の面の一覧`の形の正しい1件に一致したものの数
    pub(crate) unspecified: usize,
}
impl SurfaceTally {
    readonly!(copy total: usize, specified: usize, unspecified: usize);
}

/// "kotowari check" の "surface"
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct Unlisted {
    /// `未記載の面の一覧`で外した`面`の種類と名前の組の数
    pub(crate) unspecified: usize,
}
impl Unlisted {
    readonly!(copy unspecified: usize);
}

/// 形の正しい`未記載の面の一覧`の1件
#[derive(Debug, Clone, PartialEq, Eq)]
struct Unspecified {
    kind: String,
    name: String,
}

/// `未記載の面の一覧`を読んだ結果
#[derive(Debug, Default)]
struct UnspecifiedList {
    /// 一覧のファイルの`基準のディレクトリ`からの相対パス
    path: String,
    /// 形の正しい1件
    entries: Vec<Unspecified>,
    /// 形の誤った1件への surface_unspecified_invalid（この1件には surface_unspecified_stale を出さない）
    findings: Vec<Finding>,
}

/// `面`の検査を、取り出した`面`と、決定の文書と、`未記載の面の一覧`だけから行う。決定の文書の形の
/// 誤りはここでは出さない（check が出す）
pub fn check_analysis(
    analysis: &[crate::SurfaceAnalysis],
    decisions: &[crate::SourceText],
    unspecified: &[crate::SourceText],
    findings: &mut Vec<Finding>,
) -> Result<SurfaceTally, StopReason> {
    let decisions = decisions
        .iter()
        .map(|source| crate::decisions::parse(source.path(), source.text(), &[], &mut Vec::new()))
        .collect::<Result<Vec<_>, _>>()?;
    check_entries(
        analysis.iter(),
        &decisions,
        unspecified
            .iter()
            .map(|source| (source.path(), source.text())),
        findings,
    )
}

pub(crate) fn check_entries<'a>(
    analysis: impl IntoIterator<Item = &'a crate::SurfaceAnalysis>,
    decisions: &[crate::decisions::Document],
    unspecified: impl IntoIterator<Item = (&'a str, &'a str)>,
    findings: &mut Vec<Finding>,
) -> Result<SurfaceTally, StopReason> {
    let mut surfaces = Vec::new();
    for file in analysis {
        surfaces.extend(file.surfaces.iter().cloned());
        for finding in &file.findings {
            if finding.kind == FindingKind::UnparsableFile
                && findings
                    .iter()
                    .any(|old| old.kind == finding.kind && old.path == finding.path)
            {
                continue;
            }
            findings.push(finding.clone());
        }
    }
    let mut list = UnspecifiedList::default();
    for (path, text) in unspecified {
        list.path = path.to_owned();
        for item in &crate::config::read_yaml_sequence(text, path)? {
            match read_entry(item) {
                Some(entry) => list.entries.push(entry),
                None => list.findings.push(Finding::new(
                    FindingKind::SurfaceUnspecifiedInvalid,
                    path.to_owned(),
                    None,
                    written_detail(item),
                )),
            }
        }
    }
    Ok(report(&surfaces, decisions, &list, findings))
}

/// 1件を読む。鍵と値の組で、"kind"、"name"、"why" のちょうど3つの鍵の値がどれも文字列で、
/// "why" が前後の半角空白とタブを除いて空でないときだけ形が正しい
fn read_entry(item: &Value) -> Option<Unspecified> {
    const KEYS: [&str; 3] = ["kind", "name", "why"];
    let map = item.as_object()?;
    if map.len() != KEYS.len() {
        return None;
    }
    let [kind, name, why] = KEYS.map(|key| map.get(key).and_then(Value::as_str));
    let (kind, name, why) = (kind?, name?, why?);
    if crate::mutants::trim_spaces_and_tabs(why).is_empty() {
        return None;
    }
    Some(Unspecified {
        kind: kind.to_string(),
        name: name.to_string(),
    })
}

/// 一覧に書かれたままの "kind" と "name" を半角空白1つで区切る。無いか文字列でない方は空の文字列
fn written_detail(item: &Value) -> String {
    let written = |key: &str| item.get(key).and_then(Value::as_str).unwrap_or("");
    format!("{} {}", written("kind"), written("name"))
}

/// 決定の文書に無く一覧のどの1件にも一致しない`面`に、種類と名前の組ごとに1件の surface_without_spec を出す。
/// 場所は`面のファイル`のパスのバイト順、次に行の小さい順で最初の`面`。
/// 一致する`面`が無いか、一致する`面`が決定の文書にある1件には surface_unspecified_stale を出す
fn report(
    surfaces: &[Surface],
    decisions: &[crate::decisions::Document],
    list: &UnspecifiedList,
    findings: &mut Vec<Finding>,
) -> SurfaceTally {
    let quoted = quoted_in_decisions(decisions);
    let in_spec = |name: &str| quoted.contains(name);
    let listed = |surface: &Surface| {
        list.entries
            .iter()
            .any(|entry| entry.kind == surface.kind && entry.name == surface.name)
    };
    findings.extend(list.findings.iter().cloned());
    for entry in &list.entries {
        let needed = surfaces
            .iter()
            .any(|s| s.kind == entry.kind && s.name == entry.name && !in_spec(&s.name));
        if !needed {
            findings.push(Finding::new(
                FindingKind::SurfaceUnspecifiedStale,
                list.path.clone(),
                None,
                format!("{} {}", entry.kind, entry.name),
            ));
        }
    }
    let mut tally = SurfaceTally::default();
    // 種類と名前の組ごとに、面のファイルのパスのバイト順、次に行の小さい順で最初の面だけを見る
    let mut ordered: Vec<&Surface> = surfaces.iter().collect();
    ordered.sort_by(|a, b| a.path.cmp(&b.path).then(a.line.cmp(&b.line)));
    let mut seen: BTreeSet<(&str, &str)> = BTreeSet::new();
    for surface in ordered {
        if !seen.insert((&surface.kind, &surface.name)) {
            continue;
        }
        tally.total += 1;
        if in_spec(&surface.name) {
            tally.specified += 1;
        } else if listed(surface) {
            tally.unspecified += 1;
        } else {
            findings.push(Finding::new(
                FindingKind::SurfaceWithoutSpec,
                surface.path.clone(),
                Some(surface.line),
                format!("{} {}", surface.kind, surface.name),
            ));
        }
    }
    tally
}

/// 決定の文書の決定の「決めたこと」と「確認すること」の中（表はセルごと、コードブロックは除く）で、
/// 二重引用符の対かバッククォートの対で囲んだ中身の集まり。中身の前後の空白は除かない
fn quoted_in_decisions(decisions: &[crate::decisions::Document]) -> BTreeSet<String> {
    let mut quoted = BTreeSet::new();
    for text in crate::decisions::quotable(decisions) {
        for line in split_lines(text) {
            quoted.extend(
                crate::double_quoted_contents(line)
                    .into_iter()
                    .map(str::to_string),
            );
            quoted.extend(
                crate::extract_backtick_contents_outside_quotes(line)
                    .into_iter()
                    .map(str::to_string),
            );
        }
    }
    quoted
}
