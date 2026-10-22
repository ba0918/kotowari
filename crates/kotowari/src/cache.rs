//! 全体像のページを書く置き場（REQ-core-293、REQ-core-296、REQ-core-324、REQ-core-353）

use crate::{Error, OverviewBuild, Page};
use std::path::Path;

/// 置き場。`基準のディレクトリ`の下に固定し、設定で変えられない（REQ-core-296）
pub(crate) const CACHE: &str = ".kotowari/cache/overview";

/// `基準のディレクトリ`の ".kotowari/cache/overview/" の下へ書く。同じ名前で同じバイト列の
/// ファイルは書かず、今回返さなかったファイルを消す（REQ-core-293）
pub(crate) fn write(base: &Path, pages: &[Page]) -> Result<OverviewBuild, Error> {
    let cache = base.join(CACHE);
    let write_error = |path: &str, error| Error::cache(&format!("{CACHE}/{path}"), Some(error));
    // REQ-core-324: 置き場かその上がリンクかディレクトリでなければ、何も書かず消さずに止める
    let mut place = String::new();
    for component in CACHE.split('/') {
        if !place.is_empty() {
            place.push('/');
        }
        place.push_str(component);
        if !directory_or_absent(base, &place)? {
            break;
        }
    }
    // REQ-core-353: ほかの言語のページを書く "<言語タグ>/" も、1つも書く前に確かめる
    let directories: std::collections::BTreeSet<&str> = pages
        .iter()
        .filter_map(|page| page.name.rsplit_once('/').map(|(directory, _)| directory))
        .collect();
    for directory in &directories {
        directory_or_absent(base, &format!("{CACHE}/{directory}"))?;
    }
    std::fs::create_dir_all(&cache).map_err(|error| Error::cache(CACHE, Some(error)))?;
    let existing = existing(&cache).map_err(|error| Error::cache(CACHE, Some(error)))?;
    let mut build = OverviewBuild::default();
    for page in pages {
        let path = cache.join(&page.name);
        // REQ-core-353: ほかの言語のページは "<言語タグ>/" の下に書く。置き場は上で確かめてある
        if let Some((directory, _)) = page.name.rsplit_once('/') {
            std::fs::create_dir_all(cache.join(directory))
                .map_err(|error| Error::cache(&format!("{CACHE}/{directory}"), Some(error)))?;
        }
        // シンボリックリンクを辿って置き場の外に書かないよう、リンクは消してからファイルを書く（REQ-core-296）
        let link = std::fs::symlink_metadata(&path).is_ok_and(|meta| meta.file_type().is_symlink());
        if link {
            std::fs::remove_file(&path).map_err(|error| write_error(&page.name, error))?;
        } else if std::fs::read(&path).is_ok_and(|bytes| bytes == page.content.as_bytes()) {
            build.unchanged += 1;
            continue;
        }
        std::fs::write(&path, &page.content).map_err(|error| write_error(&page.name, error))?;
        build.written.push(format!("{CACHE}/{}", page.name));
    }
    let names: std::collections::BTreeSet<&str> =
        pages.iter().map(|page| page.name.as_str()).collect();
    for name in existing {
        if names.contains(name.as_str()) {
            continue;
        }
        std::fs::remove_file(cache.join(&name)).map_err(|error| write_error(&name, error))?;
        build.removed.push(format!("{CACHE}/{name}"));
    }
    build.written.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    build.removed.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    Ok(build)
}

/// `基準のディレクトリ`からの相対パスの place がディレクトリなら true、無ければ false。
/// ほかのもの（シンボリックリンクを含む）か、調べられなければ置き場の誤り（REQ-core-324）
fn directory_or_absent(base: &Path, place: &str) -> Result<bool, Error> {
    match std::fs::symlink_metadata(base.join(place)) {
        Ok(meta) if meta.file_type().is_dir() => Ok(true),
        Ok(_) => Err(Error::cache(place, None)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(Error::cache(place, Some(error))),
    }
}

/// 置き場の下のファイルを、置き場からの相対パスで集める。ディレクトリのシンボリックリンクは辿らない
pub(crate) fn existing(cache: &Path) -> Result<Vec<String>, std::io::Error> {
    if !cache.is_dir() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(cache)
        .follow_links(false)
        .min_depth(1)
    {
        let entry = entry?;
        if !entry.file_type().is_dir() {
            let relative = entry
                .path()
                .strip_prefix(cache)
                .unwrap_or(entry.path())
                .to_string_lossy()
                .replace('\\', "/");
            files.push(relative);
        }
    }
    Ok(files)
}
