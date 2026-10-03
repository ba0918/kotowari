use crate::acquisition::read_utf8_file;
use kotowari_core::{StopReason, config::Config};
use std::path::Path;
pub fn read_texts(base: &Path, cfg: &Config) -> Result<Option<Vec<(String, String)>>, StopReason> {
    if cfg.guides.files.is_empty() {
        return Ok(None);
    }
    crate::test_files::collect_files(base, &cfg.guides.files)?
        .into_iter()
        .map(|(path, absolute)| {
            let text = read_utf8_file(Path::new(&absolute), &path)?;
            Ok((path, text))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
