//! 決定の文書の置き場を辿って読む（docs/decisions/core/decision-documents.md）

use crate::acquisition::read_utf8_file;
use kotowari_core::config::Config;
use std::path::Path;

use kotowari_core::is_frozen;

/// 置き場の下の Markdown の文書を読んで加える。置き場が無いかディレクトリでなければ停止する
/// （C-core-config-10）。言語が2つ以上なら文書を`対`に入れ、
/// `先頭の言語`の`側`のある`対`の、ファイルのある`側`を加える（C-core-lang-1）
pub fn push(
    base: &Path,
    config: &Config,
    assembly: &mut crate::translations::Assembly,
    preparation: &mut kotowari_core::RepositoryReadPreparation,
) -> Result<(), kotowari_core::StopReason> {
    let place = &config.decisions.documents;
    let dir = base.join(place);
    if !dir.is_dir() {
        let detail = std::fs::read_dir(&dir)
            .err()
            .map_or_else(|| "not a directory".to_string(), |e| e.to_string());
        return Err(kotowari_core::StopReason::UnreadableFile(format!(
            "{place}: {detail}"
        )));
    }
    let mut entries = Vec::new();
    collect_markdown_paths(&dir, "", place, &mut entries)?;
    let mut entries: Vec<(String, std::path::PathBuf)> = entries
        .into_iter()
        .map(|(relative, absolute)| (kotowari_core::join_display_path(place, &relative), absolute))
        .filter(|(path, _)| !is_frozen(path))
        .collect();
    entries.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let entries = assembly.place(kotowari_core::translations::Place::Decision, entries, true)?;
    for (path, absolute) in entries {
        let text = read_utf8_file(&absolute, &path)?;
        preparation
            .push_decision_document(kotowari_core::NativeSourceText::new(absolute, path, text))?;
    }
    Ok(())
}

/// 置き場の下の ".md" のファイルを深さ優先で集める。"." で始まるディレクトリには入らず、
/// ディレクトリへのシンボリックリンクは辿らない
pub(crate) fn collect_markdown_paths(
    dir: &Path,
    prefix: &str,
    place: &str,
    paths: &mut Vec<(String, std::path::PathBuf)>,
) -> Result<(), kotowari_core::StopReason> {
    let display = kotowari_core::join_display_path(place, prefix);
    let display = if prefix.is_empty() { place } else { &display };
    let unreadable = |e| kotowari_core::StopReason::UnreadableFile(format!("{display}: {e}"));
    for entry in std::fs::read_dir(dir).map_err(unreadable)? {
        let entry = entry.map_err(unreadable)?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let relative_path = kotowari_core::join_display_path(prefix, &name);
        let entry_display = kotowari_core::join_display_path(place, &relative_path);
        let entry_error =
            |e| kotowari_core::StopReason::UnreadableFile(format!("{entry_display}: {e}"));
        let file_type = entry.file_type().map_err(entry_error)?;
        let (is_dir, is_file) = if file_type.is_symlink() {
            let metadata = std::fs::metadata(&path).map_err(entry_error)?;
            (false, metadata.is_file())
        } else {
            (file_type.is_dir(), file_type.is_file())
        };
        if is_dir && !name.starts_with('.') {
            collect_markdown_paths(&path, &relative_path, place, paths)?;
        } else if is_file && path.extension().is_some_and(|ext| ext == "md") {
            paths.push((relative_path, path));
        }
    }
    Ok(())
}
