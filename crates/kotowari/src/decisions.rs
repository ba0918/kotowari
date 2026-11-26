//! 決定の文書の置き場を辿って読む（docs/decisions/core/decision-documents.md）

use crate::acquisition::read_utf8_file;
use kotowari_core::config::Config;
use std::path::Path;

use kotowari_core::is_frozen;

/// 設定に置き場があれば、その下の Markdown の文書を読んで加える。置き場が無いか
/// ディレクトリでなければ、ほかの置き場と同じく停止する。言語が2つ以上なら文書を`対`に入れ、
/// `先頭の言語`の`側`のある`対`の、ファイルのある`側`を加える（C-core-lang-1）
pub fn push(
    base: &Path,
    config: &Config,
    assembly: &mut crate::translations::Assembly,
    preparation: &mut kotowari_core::RepositoryReadPreparation,
) -> Result<(), kotowari_core::StopReason> {
    let Some(place) = &config.decisions.documents else {
        return Ok(());
    };
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
    crate::ir::collect_ir_paths(&dir, "", place, &mut entries)?;
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
