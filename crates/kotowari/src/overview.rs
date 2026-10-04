//! 全体像の元データの読み込みと、置き場の重なりの検査（REQ-core-278、REQ-core-280）

use kotowari_core::{FindingGroup, ReadModel, SourceText, StopReason, config::Config};
use std::collections::BTreeSet;
use std::path::Path;

/// "overview" の鍵があれば、"overview.files" に当たり拡張子が小文字の ".md" のファイルを読む。
/// 隠しディレクトリは glob が名指ししたものだけに入る（REQ-core-019）。読む前に、ガイドとテストの
/// 置き場との重なりを検査する（REQ-core-280）。鍵が無ければ何も読まずに None
pub(crate) fn read_texts(
    base: &Path,
    config: &Config,
    guides: &[&str],
    tests: &[String],
) -> Result<Option<Vec<SourceText>>, StopReason> {
    let Some(overview) = &config.overview else {
        return Ok(None);
    };
    let files: Vec<(String, std::path::PathBuf)> =
        crate::test_files::collect_named_hidden(base, &overview.files)?
            .into_iter()
            .filter(|(path, _)| Path::new(path).extension().is_some_and(|ext| ext == "md"))
            .collect();
    validate_overlap(files.iter().map(|(path, _)| path.as_str()), guides, tests)?;
    files
        .into_iter()
        .map(|(path, absolute)| {
            let text = crate::test_files::read_collected_text(&absolute, &path)?;
            SourceText::new(path, text).map_err(|error| StopReason::MappingError(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

/// REQ-core-280: パスのバイト順で最初の重なったファイル。3つすべてに当たれば guides の文言にする
fn validate_overlap<'a>(
    overview: impl IntoIterator<Item = &'a str>,
    guides: &[&str],
    tests: &[String],
) -> Result<(), StopReason> {
    let guides: BTreeSet<&str> = guides.iter().copied().collect();
    let tests: BTreeSet<&str> = tests.iter().map(String::as_str).collect();
    let first = overview
        .into_iter()
        .filter(|path| guides.contains(path) || tests.contains(path))
        .min_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    match first {
        Some(path) => {
            let other = if guides.contains(path) {
                "guides.files"
            } else {
                "tests.files"
            };
            Err(StopReason::ConfigError(format!(
                "{path}: matched by both overview.files and {other}"
            )))
        }
        None => Ok(()),
    }
}

/// check と status に加える "overview" の群。鍵が無ければ数は両方 0（REQ-core-288）
pub(crate) fn group(read: &ReadModel, texts: Option<Vec<SourceText>>) -> FindingGroup {
    match texts {
        Some(texts) => kotowari_overview::inspect(read, &texts).into_group(),
        None => FindingGroup::new(kotowari_overview::GROUP, 0, 0, Vec::new()),
    }
}
