//! `面`の検査（docs/ir/core/surface.md）。`面の規則`で`面のファイル`から`面`を取り出す。

use crate::config::Config;
use crate::test_queries::{ParsedFile, RuleSet, language_of};
use crate::{Finding, FindingKind, StopReason};
use std::path::Path;

/// `面の規則`で取り出した`面`1つ（REQ-core-223）
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

/// "surface.rules" で "surface.files" から`面`を取り出す。
/// "surface.rules" が空の一覧なら何も読まない（REQ-core-223、REQ-core-227）。
/// 規則の言語の`面のファイル`の構文の誤りは unparsable_file にし、同じパスに
/// `テストのファイル`として出していれば重ねない（REQ-core-236）
pub fn extract(
    base: &Path,
    cfg: &Config,
    findings: &mut Vec<Finding>,
) -> Result<Vec<Surface>, StopReason> {
    if cfg.surface.rules.is_empty() {
        return Ok(Vec::new());
    }
    let rules = RuleSet::load(base, &cfg.surface.rules, "surface.rules")?;
    let files = crate::tests_discovery::collect_files(base, &cfg.surface.files)?;
    let mut surfaces = Vec::new();
    for (rel, abs) in &files {
        // REQ-core-224: 読めないファイルと UTF-8 でないファイルでの停止は "tests.files" と同じ
        let content =
            crate::tests_discovery::lone_cr_to_lf(&crate::read_utf8_file(Path::new(abs), rel)?);
        let Some(lang) = language_of(rel).filter(|lang| rules.has_language(*lang)) else {
            continue;
        };
        let Some(parsed) = ParsedFile::parse(&content, lang) else {
            let reported = findings
                .iter()
                .any(|f| f.kind == FindingKind::UnparsableFile && f.path == *rel);
            if !reported {
                findings.push(Finding::new(
                    FindingKind::UnparsableFile,
                    rel.clone(),
                    None,
                    rel.clone(),
                ));
            }
            continue;
        };
        surfaces.extend(
            parsed
                .find_named(&rules, lang, rel)
                .into_iter()
                .map(|m| Surface {
                    kind: m.rule_id,
                    name: m.name,
                    path: rel.clone(),
                    line: m.line,
                }),
        );
    }
    Ok(surfaces)
}
