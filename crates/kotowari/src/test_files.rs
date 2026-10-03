use globset::{Glob, GlobSetBuilder};
use std::path::Path;
use walkdir::WalkDir;
/// glob の一覧に当たるファイルを集める。`テストのファイル`（"tests.files"）と`ガイド`（"guides.files"）が
/// 同じ走査と`除外`を使う（REQ-core-019、REQ-core-079、REQ-core-018、REQ-core-198）。
/// (`基準のディレクトリ`からの相対パス, 絶対パス) を相対パスのバイト順に並べて返す
pub fn collect_files(
    base: &Path,
    patterns: &[String],
) -> Result<Vec<(String, String)>, kotowari_core::StopReason> {
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

    let mut files = Vec::new();

    for entry in WalkDir::new(base)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // 隠しディレクトリを除外（REQ-core-019）。ルートは除外しない
            // ディレクトリのシンボリックリンクは辿らない（REQ-core-079, A102）
            if e.depth() > 0 {
                let ft = e.file_type();
                if ft.is_symlink() {
                    // シンボリックリンク: ファイルなら含める、ディレクトリなら除外
                    // filter_entry ではディレクトリかどうかで判定
                    // WalkDir は follow_links(false) なのでシンボリックリンクは展開されない
                    // ここで辿って判定する
                    if let Ok(meta) = std::fs::metadata(e.path())
                        && meta.is_dir()
                    {
                        return false; // ディレクトリリンクは辿らない
                    }
                    return true; // ファイルリンクは含める
                }
                if ft.is_dir() {
                    let name = e.file_name().to_string_lossy();
                    return !name.starts_with('.');
                }
            }
            true
        })
    {
        // REQ-core-018（A96）: 走査でディレクトリが読めなければ停止する
        let entry = entry.map_err(|e| {
            let where_ = e.path().map(|p| relative_to(base, p)).unwrap_or_default();
            kotowari_core::StopReason::UnreadableFile(format!("{where_}: {e}"))
        })?;
        // ファイルまたはファイルのシンボリックリンク
        let is_file = if entry.file_type().is_symlink() {
            // A146: 先の無いシンボリックリンクは読めないファイルとして停止する
            std::fs::metadata(entry.path())
                .map(|m| m.is_file())
                .map_err(|e| {
                    let rel = relative_to(base, entry.path());
                    kotowari_core::StopReason::UnreadableFile(format!("{rel}: {e}"))
                })?
        } else {
            entry.file_type().is_file()
        };
        if is_file {
            let rel = relative_to(base, entry.path());
            if globset.is_match(&rel) {
                files.push((rel, entry.path().to_string_lossy().to_string()));
            }
        }
    }

    files.sort();
    Ok(files)
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
pub fn discover_and_check(
    base: &Path,
    config: &kotowari_core::config::Config,
    docs: &[kotowari_core::ir::IrDocument],
    known_ids: &std::collections::BTreeSet<String>,
    ir_path: &str,
    findings: &mut Vec<kotowari_core::Finding>,
) -> Result<kotowari_core::tests_discovery::DiscoveredTests, kotowari_core::StopReason> {
    let files = collect_files(base, &config.tests.files)?;
    let analyzer = kotowari_source_analysis::Analyzer::new(
        config.clone(),
        read_rules(base, &config.tests.rules, "tests.rules")?,
        vec![],
    )
    .map_err(|error| kotowari_core::StopReason::ConfigError(error.detail().into()))?;
    let analysis = files
        .iter()
        .map(|(path, absolute)| {
            let text = crate::acquisition::read_utf8_file(Path::new(absolute), path)?;
            let source = kotowari_core::SourceText::new(path.clone(), text)
                .map_err(|error| kotowari_core::StopReason::MappingError(error.to_string()))?;
            analyzer
                .tests_at(source, path)
                .map_err(|error| kotowari_core::StopReason::ConfigError(error.detail().into()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(kotowari_core::tests_discovery::check_entries(
        files
            .iter()
            .zip(&analysis)
            .map(|((path, _), file)| (path.as_str(), file)),
        docs,
        known_ids,
        ir_path,
        findings,
    ))
}
