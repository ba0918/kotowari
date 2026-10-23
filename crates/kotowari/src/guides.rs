use kotowari_core::{StopReason, config::Config};
use std::path::Path;
/// "guides.files" に当たるファイルを集める。`ガイド`の`対`のすべての`側`を`ガイド`にし（REQ-core-343）、
/// テストのファイルと重なれば止める（REQ-core-199）。鍵が空なら None
pub fn collect(
    base: &Path,
    cfg: &Config,
    test_paths: &[String],
    assembly: &mut crate::translations::Assembly,
) -> Result<Option<crate::translations::Files>, StopReason> {
    if cfg.guides.files.is_empty() {
        return Ok(None);
    }
    let files = crate::test_files::collect_files(base, &cfg.guides.files)?;
    let files = assembly.place(kotowari_core::translations::Place::Guide, files, true)?;
    kotowari_core::guides::validate_overlap(
        files.iter().map(|(path, _)| path.as_str()),
        test_paths,
    )?;
    Ok(Some(files))
}

/// `collect` で集めた`ガイド`を読む
pub fn read_texts(
    base: &Path,
    cfg: &Config,
    test_paths: &[String],
    assembly: &mut crate::translations::Assembly,
) -> Result<Option<Vec<kotowari_core::NativeSourceText>>, StopReason> {
    let Some(files) = collect(base, cfg, test_paths, assembly)? else {
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
