use kotowari_core::{StopReason, config::Config};
use std::path::Path;
pub fn read_texts(
    base: &Path,
    read: &kotowari_core::ReadModel,
) -> Result<Option<Vec<kotowari_core::NativeSourceText>>, StopReason> {
    let cfg: &Config = read.config();
    if cfg.guides.files.is_empty() {
        return Ok(None);
    }
    let files = crate::test_files::collect_files(base, &cfg.guides.files)?;
    read.validate_guide_paths(files.iter().map(|(path, _)| path.as_str()))?;
    files
        .into_iter()
        .map(|(path, absolute)| {
            let text = crate::test_files::read_collected_text(&absolute, &path)?;
            Ok(kotowari_core::NativeSourceText::new(absolute, path, text))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
