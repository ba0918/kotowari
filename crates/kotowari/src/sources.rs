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
    // 置き場がディレクトリであることは呼ぶ側が確かめてある（REQ-core-018）
    let read_place = |place: &str| {
        let mut texts = Vec::new();
        for_each_md(&base.join(place), "", place, &mut |path, rel, content| {
            texts.push(kotowari_core::NativeSourceText::new(
                path,
                kotowari_core::join_display_path(place, &rel),
                content.to_owned(),
            ));
        })?;
        Ok(texts)
    };
    Ok((
        read_place(&config.decisions.records)?,
        read_place(&config.decisions.adr)?,
    ))
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
