use crate::acquisition::read_utf8_file;
use kotowari_core::config::Config;
use std::path::Path;
pub fn read_texts(
    base: &Path,
    config: &Config,
) -> Result<
    (
        Vec<kotowari_core::NativeSourceText>,
        Vec<kotowari_core::NativeSourceText>,
    ),
    kotowari_core::StopReason,
> {
    let records_dir = base.join(&config.decisions.records);
    let adr_dir = base.join(&config.decisions.adr);

    let mut records = Vec::new();
    let mut adr = Vec::new();

    // records ディレクトリを読む。判断の記録とそれ以外のファイルに分ける
    if records_dir.is_dir() {
        for_each_md(
            &records_dir,
            "",
            &config.decisions.records,
            &mut |path, rel, content| {
                records.push(kotowari_core::NativeSourceText::new(
                    path,
                    kotowari_core::join_display_path(&config.decisions.records, &rel),
                    content.to_owned(),
                ));
            },
        )?;
    }

    // adr ディレクトリを読む
    if adr_dir.is_dir() {
        for_each_md(
            &adr_dir,
            "",
            &config.decisions.adr,
            &mut |path, rel, content| {
                adr.push(kotowari_core::NativeSourceText::new(
                    path,
                    kotowari_core::join_display_path(&config.decisions.adr, &rel),
                    content.to_owned(),
                ));
            },
        )?;
    }

    Ok((records, adr))
}

/// 置き場の下の .md をファイル名の順に深さ優先で読み、置き場からの相対パスと中身を visit に渡す
fn for_each_md(
    dir: &Path,
    prefix: &str,
    config_key: &str,
    visit: &mut dyn FnMut(std::path::PathBuf, String, &str),
) -> Result<(), kotowari_core::StopReason> {
    let unreadable =
        |e: std::io::Error| kotowari_core::StopReason::UnreadableFile(format!("{config_key}: {e}"));
    let mut sorted = std::fs::read_dir(dir)
        .map_err(unreadable)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(unreadable)?;
    sorted.sort_by_key(|e| e.file_name());

    for entry in sorted {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        let ft = entry.file_type().map_err(|e| {
            let display = kotowari_core::join_display_path(config_key, &rel);
            kotowari_core::StopReason::UnreadableFile(format!("{display}: {e}"))
        })?;
        // A102: ファイルのシンボリックリンクは読む。ディレクトリのリンクは辿らない。
        // A146: 先の無いリンクは読めないファイルとして停止する
        let (is_dir, is_file) = if ft.is_symlink() {
            let meta = std::fs::metadata(&path).map_err(|e| {
                let display = kotowari_core::join_display_path(config_key, &rel);
                kotowari_core::StopReason::UnreadableFile(format!("{display}: {e}"))
            })?;
            (false, meta.is_file())
        } else {
            (ft.is_dir(), ft.is_file())
        };
        if is_dir {
            // 除外: 隠しディレクトリは辿らない（CONTEXT.md の除外）
            if name.starts_with('.') {
                continue;
            }
            for_each_md(&path, &rel, config_key, visit)?;
        } else if is_file && path.extension().is_some_and(|ext| ext == "md") {
            let display = kotowari_core::join_display_path(config_key, &rel);
            let content = read_utf8_file(&path, &display)?;

            visit(path, rel, &content);
        }
    }
    Ok(())
}
