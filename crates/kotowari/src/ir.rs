use crate::acquisition::read_utf8_file;
#[cfg(test)]
use kotowari_core::Finding;
use kotowari_core::config::Config;
#[cfg(test)]
use kotowari_core::ir::*;
use std::path::Path;
/// IR のディレクトリからすべての文書を読んで検査する
#[cfg(test)]
pub fn load_and_check(
    base: &Path,
    config: &Config,
) -> Result<(Vec<IrDocument>, Vec<Finding>), kotowari_core::StopReason> {
    let (docs, findings, _) = load_and_check_with_duplicates(base, config)?;
    Ok((docs, findings))
}

#[cfg(test)]
pub fn load_and_check_with_duplicates(
    base: &Path,
    config: &Config,
) -> Result<(Vec<IrDocument>, Vec<Finding>, GlossaryDuplicates), kotowari_core::StopReason> {
    let ir_dir = base.join(&config.ir);
    let mut entries = Vec::new();
    collect_ir_paths(&ir_dir, "", &config.ir, &mut entries)?;
    entries.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

    let mut docs = Vec::new();
    for (relative_path, path) in entries {
        let display = kotowari_core::join_display_path(&config.ir, &relative_path);
        let content = read_utf8_file(&path, &display)?;
        let doc = parse_document(&relative_path, &content)?;
        docs.push(doc);
    }
    let duplicates = GlossaryDuplicates::new(&docs);
    let findings = check_documents_with_duplicates(&docs, config, &duplicates);
    Ok((docs, findings, duplicates))
}

fn collect_ir_paths(
    dir: &Path,
    prefix: &str,
    ir_path: &str,
    paths: &mut Vec<(String, std::path::PathBuf)>,
) -> Result<(), kotowari_core::StopReason> {
    let display = kotowari_core::join_display_path(ir_path, prefix);
    let display = if prefix.is_empty() { ir_path } else { &display };
    let unreadable = |e| kotowari_core::StopReason::UnreadableFile(format!("{display}: {e}"));
    for entry in std::fs::read_dir(dir).map_err(unreadable)? {
        let entry = entry.map_err(unreadable)?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let relative_path = kotowari_core::join_display_path(prefix, &name);
        let entry_display = kotowari_core::join_display_path(ir_path, &relative_path);
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
            collect_ir_paths(&path, &relative_path, ir_path, paths)?;
        } else if is_file && path.extension().is_some_and(|ext| ext == "md") {
            paths.push((relative_path, path));
        }
    }
    Ok(())
}

pub fn prepare(
    base: &Path,
    config: &Config,
    assembly: &mut crate::translations::Assembly,
) -> Result<kotowari_core::RepositoryReadPreparation, kotowari_core::StopReason> {
    let mut entries = Vec::new();
    collect_ir_paths(&base.join(&config.ir), "", &config.ir, &mut entries)?;
    // REQ-core-342: `対`の`先頭の言語`の`側`だけを`IR`の文書として読む
    let from_base = entries
        .into_iter()
        .map(|(path, absolute)| {
            (
                kotowari_core::join_display_path(&config.ir, &path),
                absolute,
            )
        })
        .collect();
    let mut entries: Vec<(String, std::path::PathBuf)> = assembly
        .place(kotowari_core::translations::Place::Ir, from_base, false)?
        .into_iter()
        .map(|(path, absolute)| (ir_relative(&config.ir, &path), absolute))
        .collect();
    entries.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let mut preparation = kotowari_core::RepositoryReadPreparation::new(config.clone())?;
    for (path, absolute) in entries {
        let text = read_utf8_file(
            &absolute,
            &kotowari_core::join_display_path(&config.ir, &path),
        )?;
        preparation.push_ir(kotowari_core::NativeSourceText::new(absolute, path, text))?;
    }
    Ok(preparation)
}

/// `基準のディレクトリ`からの相対パスを、`IR`の置き場からの相対パスにする
fn ir_relative(place: &str, path: &str) -> String {
    if place.is_empty() {
        return path.to_string();
    }
    path.strip_prefix(place)
        .and_then(|rest| rest.strip_prefix('/'))
        .unwrap_or(path)
        .to_string()
}
