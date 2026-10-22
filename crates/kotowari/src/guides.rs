use kotowari_core::{StopReason, config::Config};
use std::path::Path;
/// "guides.files" に当たるファイルを集める。`ガイド`の`対`のすべての`側`を`ガイド`にし（REQ-core-343）、
/// テストのファイルと重なれば止める（REQ-core-199）。鍵が空なら None
pub fn collect(
    base: &Path,
    read: &kotowari_core::ReadModel,
    assembly: &mut crate::translations::Assembly,
) -> Result<Option<crate::translations::Files>, StopReason> {
    let cfg: &Config = read.config();
    if cfg.guides.files.is_empty() {
        return Ok(None);
    }
    let files = crate::test_files::collect_files(base, &cfg.guides.files)?;
    let files = assembly.place(kotowari_core::translations::Place::Guide, files, true)?;
    read.validate_guide_paths(files.iter().map(|(path, _)| path.as_str()))?;
    Ok(Some(files))
}

/// `collect` で集めた`ガイド`を読む
pub fn read_texts(
    base: &Path,
    read: &kotowari_core::ReadModel,
    assembly: &mut crate::translations::Assembly,
) -> Result<Option<Vec<kotowari_core::NativeSourceText>>, StopReason> {
    let Some(files) = collect(base, read, assembly)? else {
        return Ok(None);
    };
    files
        .into_iter()
        .map(|(path, absolute)| {
            let text = crate::test_files::read_collected_text(&absolute, &path)?;
            Ok(kotowari_core::NativeSourceText::new(absolute, path, text))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
