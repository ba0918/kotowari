use crate::acquisition::read_utf8_file;
use kotowari_core::change_records::*;
use kotowari_core::config::Config;
fn collect_records(
    base: &std::path::Path,
    patterns: &[String],
) -> Result<Vec<(String, String)>, kotowari_core::StopReason> {
    let records = glob(patterns);
    let hidden_prefixes: Vec<String> = patterns
        .iter()
        .flat_map(|pattern| {
            let components: Vec<_> = pattern.split('/').collect();
            components
                .iter()
                .enumerate()
                .filter(|(_, component)| component.starts_with('.') && **component != ".")
                .map(|(index, _)| components[..=index].join("/"))
                .collect::<Vec<_>>()
        })
        .collect();
    let named_hidden = glob(&hidden_prefixes);
    let relative = |path: &std::path::Path| {
        path.strip_prefix(base)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    };
    let mut files = vec![];
    for entry in walkdir::WalkDir::new(base)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            if entry.file_type().is_symlink() {
                return !std::fs::metadata(entry.path()).is_ok_and(|meta| meta.is_dir());
            }
            !entry.file_type().is_dir()
                || !entry.file_name().to_string_lossy().starts_with('.')
                || named_hidden.is_match(relative(entry.path()))
        })
    {
        let entry = entry.map_err(|error| {
            let path = error.path().map(relative).unwrap_or_default();
            kotowari_core::StopReason::UnreadableFile(format!("{path}: {error}"))
        })?;
        let path = relative(entry.path());
        let is_file = if entry.file_type().is_symlink() {
            std::fs::metadata(entry.path())
                .map_err(|error| {
                    kotowari_core::StopReason::UnreadableFile(format!("{path}: {error}"))
                })?
                .is_file()
        } else {
            entry.file_type().is_file()
        };
        if is_file && records.is_match(&path) {
            files.push((path, entry.path().to_string_lossy().into_owned()));
        }
    }
    files.sort();
    Ok(files)
}

pub fn read_texts(
    base: &std::path::Path,
    cfg: &Config,
) -> Result<Option<Vec<(String, String)>>, kotowari_core::StopReason> {
    let Some(changes) = &cfg.changes else {
        return Ok(None);
    };
    collect_records(base, &changes.records)?
        .into_iter()
        .map(|(path, absolute)| {
            let text = read_utf8_file(std::path::Path::new(&absolute), &path)?;
            Ok((path, text))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
