use kotowari_core::config::Config;
pub fn read_texts(
    base: &std::path::Path,
    cfg: &Config,
) -> Result<Option<Vec<kotowari_core::NativeSourceText>>, kotowari_core::StopReason> {
    let Some(changes) = &cfg.changes else {
        return Ok(None);
    };
    crate::test_files::collect_named_hidden(base, &changes.records)?
        .into_iter()
        .map(|(path, absolute)| {
            let text = crate::test_files::read_collected_text(&absolute, &path)?;
            Ok(kotowari_core::NativeSourceText::new(absolute, path, text))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
