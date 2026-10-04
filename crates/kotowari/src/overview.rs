//! 全体像の元データの読み込みと、置き場の重なりの検査（REQ-core-278、REQ-core-280）

use kotowari_core::{FindingGroup, ReadModel, SourceText, StopReason, config::Config};
use std::collections::BTreeSet;
use std::path::Path;

/// 読んだ`全体像の元データ`と`目次`
pub(crate) struct Texts {
    pub(crate) data: Vec<SourceText>,
    pub(crate) toc: SourceText,
}

/// "overview" の鍵があれば、"overview.files" に当たり拡張子が小文字の ".md" のファイルと、"overview.toc"
/// の指す`目次`を読む。隠しディレクトリは glob が名指ししたものだけに入る（REQ-core-019）。読む前に、
/// 元データとガイドとテストの置き場との重なり（REQ-core-280）、次に`目次`の重なり（REQ-core-326）を
/// 検査する。鍵が無ければ何も読まずに None
pub(crate) fn read_texts(
    base: &Path,
    config: &Config,
    guides: &[&str],
    tests: &[String],
) -> Result<Option<Texts>, StopReason> {
    let Some(overview) = &config.overview else {
        return Ok(None);
    };
    let files: Vec<(String, std::path::PathBuf)> =
        crate::test_files::collect_named_hidden(base, &overview.files)?
            .into_iter()
            .filter(|(path, _)| Path::new(path).extension().is_some_and(|ext| ext == "md"))
            .collect();
    validate_overlap(files.iter().map(|(path, _)| path.as_str()), guides, tests)?;
    validate_toc_overlap(
        &overview.toc,
        files.iter().map(|(path, _)| path.as_str()),
        guides,
        tests,
    )?;
    let data = files
        .into_iter()
        .map(|(path, absolute)| {
            let text = crate::test_files::read_collected_text(&absolute, &path)?;
            SourceText::new(path, text).map_err(|error| StopReason::MappingError(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // REQ-core-325: 読めないときと UTF-8 でないときは、元データと同じ理由で止まる
    let text = crate::acquisition::read_utf8_file(&base.join(&overview.toc), &overview.toc)?;
    let toc = SourceText::new(overview.toc.as_str(), text)
        .map_err(|error| StopReason::MappingError(error.to_string()))?;
    Ok(Some(Texts { data, toc }))
}

/// REQ-core-326: `目次`のファイルが、元データ、ガイド、テストの走査で読むファイルのどれかに入るか。
/// 当たった鍵はこの順の最初のもの
fn validate_toc_overlap<'a>(
    toc: &str,
    overview: impl IntoIterator<Item = &'a str>,
    guides: &[&str],
    tests: &[String],
) -> Result<(), StopReason> {
    let key = if overview.into_iter().any(|path| path == toc) {
        "overview.files"
    } else if guides.contains(&toc) {
        "guides.files"
    } else if tests.iter().any(|path| path == toc) {
        "tests.files"
    } else {
        return Ok(());
    };
    Err(StopReason::ConfigError(format!(
        "{toc}: matched by both overview.toc and {key}"
    )))
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
pub(crate) fn group(read: &ReadModel, texts: Option<Texts>) -> FindingGroup {
    match texts {
        Some(texts) => kotowari_overview::inspect(read, &texts.data, &texts.toc).into_group(),
        None => FindingGroup::new(kotowari_overview::GROUP, 0, 0, Vec::new()),
    }
}

/// 置き場。`基準のディレクトリ`の下に固定し、設定で変えられない（REQ-core-296）
pub(crate) const CACHE: &str = ".kotowari/cache/overview";

/// "overview" の鍵が無いまま build か serve を実行したとき（REQ-core-279）
pub(crate) fn not_configured() -> StopReason {
    StopReason::ConfigError("overview is not configured".into())
}

/// 置き場の下のファイルを、置き場からの相対パスで集める。ディレクトリのシンボリックリンクは辿らない
pub(crate) fn existing(cache: &Path) -> Result<Vec<String>, StopReason> {
    if !cache.is_dir() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(cache)
        .follow_links(false)
        .min_depth(1)
    {
        let entry =
            entry.map_err(|error| StopReason::UnreadableFile(format!("{CACHE}: {error}")))?;
        if !entry.file_type().is_dir() {
            let relative = entry
                .path()
                .strip_prefix(cache)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/");
            files.push(relative);
        }
    }
    Ok(files)
}
