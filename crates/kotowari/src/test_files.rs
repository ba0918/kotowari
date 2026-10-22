use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
/// glob の一覧に当たるファイルを集める。`テストのファイル`（"tests.files"）と`ガイド`（"guides.files"）が
/// 同じ走査と`除外`を使う（REQ-core-019、REQ-core-079、REQ-core-018、REQ-core-198）。
/// (`基準のディレクトリ`からの相対パス, 絶対パス) を相対パスのバイト順に並べて返す
pub fn collect_files(
    base: &Path,
    patterns: &[String],
) -> Result<Vec<(String, PathBuf)>, kotowari_core::StopReason> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        // glob の構文は Config::parse で検証済み
        let g = Glob::new(pattern).map_err(|e| {
            kotowari_core::StopReason::ConfigError(format!("invalid glob: {pattern}: {e}"))
        })?;
        builder.add(g);
    }
    let globset = builder
        .build()
        .map_err(|e| kotowari_core::StopReason::ConfigError(format!("glob build error: {e}")))?;
    walk(base, &globset, &GlobSet::empty())
}

/// glob の一覧に当たるファイルを集める。隠しディレクトリは、glob がパスの成分で名指ししたものだけに入る
/// （REQ-core-019 の "overview.files" の例外）。ほかは `collect_files` と同じ走査で、
/// (`基準のディレクトリ`からの相対パス, 絶対パス) を相対パスのバイト順に並べて返す
pub fn collect_named_hidden(
    base: &Path,
    patterns: &[String],
) -> Result<Vec<(String, PathBuf)>, kotowari_core::StopReason> {
    let wanted = kotowari_core::config::glob_set(patterns);
    let hidden_prefixes: Vec<String> = patterns
        .iter()
        .flat_map(|pattern| {
            let components: Vec<_> = pattern.split('/').collect();
            components
                .iter()
                .enumerate()
                .filter(|(_, component)| component.starts_with('.') && **component != ".")
                .map(|(index, _)| components[..=index].join("/"))
                // 波括弧の中の "/" で切った前置きは glob にならない。その成分は名指しに数えない
                .filter(|prefix| globset::Glob::new(prefix).is_ok())
                .collect::<Vec<_>>()
        })
        .collect();
    let named_hidden = kotowari_core::config::glob_set(&hidden_prefixes);
    walk(base, &wanted, &named_hidden)
}

/// 基準のディレクトリを辿り、wanted に当たるファイルを集める。隠しディレクトリは named_hidden に
/// 当たるものだけに入り（REQ-core-019）、ディレクトリのシンボリックリンクは辿らない（REQ-core-079、A102）。
/// 辿る途中で読めないディレクトリと、先の無いシンボリックリンクは読めないファイルとして止める
/// （REQ-core-018、A96、A146）
fn walk(
    base: &Path,
    wanted: &GlobSet,
    named_hidden: &GlobSet,
) -> Result<Vec<(String, PathBuf)>, kotowari_core::StopReason> {
    let mut files = vec![];
    for entry in WalkDir::new(base)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            if entry.file_type().is_symlink() {
                return !std::fs::metadata(entry.path()).is_ok_and(|meta| meta.is_dir());
            }
            !entry.file_type().is_dir()
                || !entry.file_name().to_string_lossy().starts_with('.')
                || named_hidden.is_match(relative_to(base, entry.path()))
        })
    {
        let entry = entry.map_err(|error| {
            let path = error
                .path()
                .map(|path| relative_to(base, path))
                .unwrap_or_default();
            kotowari_core::StopReason::UnreadableFile(format!("{path}: {error}"))
        })?;
        let path = relative_to(base, entry.path());
        let is_file = if entry.file_type().is_symlink() {
            std::fs::metadata(entry.path())
                .map_err(|error| {
                    kotowari_core::StopReason::UnreadableFile(format!("{path}: {error}"))
                })?
                .is_file()
        } else {
            entry.file_type().is_file()
        };
        if is_file && wanted.is_match(&path) {
            files.push((path, entry.path().to_path_buf()));
        }
    }
    files.sort_by_cached_key(|(display, original)| {
        (display.clone(), original.to_string_lossy().into_owned())
    });
    Ok(files)
}

pub(crate) fn read_collected_text(
    original: &Path,
    display: &str,
) -> Result<String, kotowari_core::StopReason> {
    // Legacy collectors read a lossy absolute string. Keeping enumeration identity lossless
    // must not change which file supplies the acquired text or its existing read failure.
    crate::acquisition::read_utf8_file(Path::new(original.to_string_lossy().as_ref()), display)
}

/// 基準のディレクトリからの相対パスを、Windows の区切りも "/" にして作る。基準の外のパスは相対にしない
fn relative_to(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub fn read_rules(
    base: &Path,
    paths: &[String],
    key: &str,
) -> Result<Vec<kotowari_core::SourceText>, kotowari_core::StopReason> {
    let mut seen = std::collections::BTreeSet::new();
    paths
        .iter()
        .map(|path| {
            let config_error = |detail| kotowari_core::StopReason::ConfigError(detail);
            if !seen.insert(path) {
                return Err(config_error(format!("duplicate path in {key}: {path}")));
            }
            let full = base.join(path);
            let metadata = std::fs::metadata(&full).map_err(|error| {
                config_error(format!("unreadable file in {key}: {path}: {error}"))
            })?;
            if !metadata.is_file() {
                return Err(config_error(format!("not a file in {key}: {path}")));
            }
            let text = crate::acquisition::read_utf8_file(&full, path)
                .map_err(|error| config_error(format!("unreadable file in {key}: {error}")))?;
            kotowari_core::SourceText::new(path.clone(), text)
                .map_err(|error| config_error(error.to_string()))
        })
        .collect()
}
pub fn analyze(
    base: &Path,
    config: &kotowari_core::config::Config,
) -> Result<Vec<kotowari_core::NativeTestAnalysis>, kotowari_core::StopReason> {
    let files = collect_files(base, &config.tests.files)?;
    let analyzer = kotowari_source_analysis::Analyzer::new(
        config.clone(),
        read_rules(base, &config.tests.rules, "tests.rules")?,
        vec![],
    )
    .map_err(|error| kotowari_core::StopReason::ConfigError(error.detail().into()))?;
    files
        .into_iter()
        .map(|(path, absolute)| {
            let text = read_collected_text(&absolute, &path)?;
            let original =
                kotowari_core::NativeSourceText::new(absolute, path.clone(), text.clone());
            let source = kotowari_core::SourceText::new(path.clone(), text)
                .map_err(|error| kotowari_core::StopReason::MappingError(error.to_string()))?;
            analyzer
                .tests_at(source, &path)
                .map(|analysis| kotowari_core::NativeTestAnalysis {
                    source: original,
                    analysis,
                })
                .map_err(|error| kotowari_core::StopReason::ConfigError(error.detail().into()))
        })
        .collect()
}
