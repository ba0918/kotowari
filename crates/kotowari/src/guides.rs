use kotowari_core::{StopReason, config::Config};
use std::path::Path;

/// `collect` で集めた`ガイド`
pub struct Collected<T> {
    /// 読む`ガイド`。`ガイド`の`対`のすべての`側`（REQ-core-343）
    pub files: T,
    /// 置き場の重なりの判定に使うパス: glob に当たったファイルと読む`ガイド`を合わせたもの。
    /// `先頭の言語`の`側`の無い`側`（REQ-core-338）も、glob に当たれば重なりに数える（REQ-core-199、REQ-core-280）
    pub overlap: Vec<String>,
}

/// "guides.files" に当たるファイルを集める。`ガイド`の`対`のすべての`側`を`ガイド`にし（REQ-core-343）、
/// glob に当たったファイルか読む`ガイド`がテストのファイルと重なれば止める（REQ-core-199）。鍵が空なら None
pub fn collect(
    base: &Path,
    cfg: &Config,
    test_paths: &[String],
    assembly: &mut crate::translations::Assembly,
) -> Result<Option<Collected<crate::translations::Files>>, StopReason> {
    if cfg.guides.files.is_empty() {
        return Ok(None);
    }
    let matched = crate::test_files::collect_files(base, &cfg.guides.files)?;
    let mut overlap: Vec<String> = matched.iter().map(|(path, _)| path.clone()).collect();
    let files = assembly.place(kotowari_core::translations::Place::Guide, matched, true)?;
    overlap.extend(files.iter().map(|(path, _)| path.clone()));
    overlap.sort();
    overlap.dedup();
    kotowari_core::guides::validate_overlap(overlap.iter().map(String::as_str), test_paths)?;
    Ok(Some(Collected { files, overlap }))
}

/// `collect` で集めた`ガイド`を読む
pub fn read_texts(
    base: &Path,
    cfg: &Config,
    test_paths: &[String],
    assembly: &mut crate::translations::Assembly,
) -> Result<Option<Collected<Vec<kotowari_core::NativeSourceText>>>, StopReason> {
    let Some(Collected { files, overlap }) = collect(base, cfg, test_paths, assembly)? else {
        return Ok(None);
    };
    let files = files
        .into_iter()
        .map(|(path, absolute)| {
            let text = crate::test_files::read_collected_text(&absolute, &path)?;
            Ok(kotowari_core::NativeSourceText::new(absolute, path, text))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some(Collected { files, overlap }))
}
