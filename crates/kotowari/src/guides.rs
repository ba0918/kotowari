use crate::acquisition::read_utf8_file;
use kotowari_core::{StopReason, config::Config};
use std::path::Path;
pub fn read_texts(
    base: &Path,
    preparation: &kotowari_core::RepositoryInspectionPreparation,
) -> Result<Option<Vec<kotowari_core::NativeSourceText>>, StopReason> {
    let cfg: &Config = preparation.config();
    if cfg.guides.files.is_empty() {
        return Ok(None);
    }
    let files = crate::test_files::collect_files(base, &cfg.guides.files)?;
    preparation.validate_guide_paths(files.iter().map(|(path, _)| path.as_str()))?;
    files
        .into_iter()
        .map(|(path, absolute)| {
            let text = read_utf8_file(&absolute, &path)?;
            Ok(kotowari_core::NativeSourceText::new(absolute, path, text))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
