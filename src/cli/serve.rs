//! "kotowari overview serve"（REQ-core-297〜REQ-core-299）。全体像の置き場の下のファイルだけを
//! 127.0.0.1 で配り、割り込みまで続ける

use super::StopReason;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// 検査と描画、ポートの確保、書き込みの順に行い、URL を出して割り込みまで配る（REQ-core-297）
pub fn run(project: &kotowari::Project, port: u16) -> Result<u8, StopReason> {
    let prepared = project.overview_prepare()?;
    let cache = prepared.directory();
    // REQ-core-298: ほかのポートを試さない
    let listener = std::net::TcpListener::bind(("127.0.0.1", port))
        .map_err(|error| StopReason::PortError(format!("127.0.0.1:{port}: {error}")))?;
    let server = tiny_http::Server::from_listener(listener, None)
        .map_err(|error| StopReason::PortError(format!("127.0.0.1:{port}: {error}")))?;
    prepared.write()?;
    let server = Arc::new(server);
    let stopping = Arc::new(AtomicBool::new(false));
    {
        let (server, stopping) = (server.clone(), stopping.clone());
        ctrlc::set_handler(move || {
            stopping.store(true, Ordering::SeqCst);
            server.unblock();
        })
        .map_err(|error| StopReason::PortError(format!("127.0.0.1:{port}: {error}")))?;
    }
    println!("http://127.0.0.1:{port}/");
    use std::io::Write;
    // 標準出力が管でも、URL の1行をすぐ読めるようにする
    let _ = std::io::stdout().flush();
    loop {
        match server.recv() {
            Ok(request) => respond(&cache, request),
            Err(_) if stopping.load(Ordering::SeqCst) => return Ok(0),
            // 割り込みのほかの誤りは、受け付けの失敗である。HTTP のクレートは受け付けに一度
            // 失敗すると受け付けを再開しないので、配り続けられない（REQ-core-298）
            Err(error) => {
                return Err(StopReason::PortError(format!("127.0.0.1:{port}: {error}")));
            }
        }
    }
}

/// 1つの要求に答える。要求ごとに何も出力しない
fn respond(cache: &Path, request: tiny_http::Request) {
    let response = match resolve(cache, request.url()) {
        Some((path, bytes)) => {
            let mut response = tiny_http::Response::from_data(bytes);
            if let Ok(header) =
                tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type(&path).as_bytes())
            {
                response.add_header(header);
            }
            response
        }
        None => tiny_http::Response::from_string("not found").with_status_code(404),
    };
    // 相手が先に切っても、配り続ける
    let _ = request.respond(response);
}

/// ".html" と ".css" の Content-Type（REQ-core-297）
fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        _ => "application/octet-stream",
    }
}

/// "%xx" を戻す。戻した結果が UTF-8 でなければ None
fn decode(path: &str) -> Option<String> {
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hex = std::str::from_utf8(bytes.get(index + 1..index + 3)?).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            index += 3;
        } else {
            out.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// 要求の道を置き場の中のファイルに解く。".." の成分、絶対パス、シンボリックリンクを辿った先が
/// 置き場の外になるもの、ディレクトリ、無いファイルは None（REQ-core-299）。中身はここで読む
fn resolve(cache: &Path, url: &str) -> Option<(PathBuf, Vec<u8>)> {
    let path = url.split(['?', '#']).next().unwrap_or("");
    let path = decode(path)?;
    let relative = path.strip_prefix('/')?;
    let relative = if relative.is_empty() {
        "index.html"
    } else {
        relative
    };
    // バックスラッシュは Unix では名前の1文字なので、置き場の中のその名前のファイルを返す。
    // NUL を含む道は下の canonicalize が失敗して None になる
    let candidate = Path::new(relative);
    if !candidate
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
    {
        return None;
    }
    let root = cache.canonicalize().ok()?;
    let target = root.join(candidate).canonicalize().ok()?;
    if !target.starts_with(&root) || !target.is_file() {
        return None;
    }
    let bytes = std::fs::read(&target).ok()?;
    Some((target, bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    // @kotowari[REQ-core-299]
    #[test]
    fn req_core_299_paths_that_leave_the_cache_do_not_resolve() {
        let dir = tempfile::tempdir().unwrap();
        let cache = dir.path().join("cache");
        std::fs::create_dir(&cache).unwrap();
        std::fs::write(cache.join("index.html"), "index").unwrap();
        std::fs::write(dir.path().join("secret.txt"), "secret").unwrap();
        assert_eq!(
            resolve(&cache, "/").map(|(_, bytes)| bytes),
            Some(b"index".to_vec())
        );
        assert!(resolve(&cache, "/index.html?x=1").is_some());
        for url in [
            "/../secret.txt",
            "/%2e%2e/secret.txt",
            "/%2E%2E%2Fsecret.txt",
            "/./index.html",
            "//secret.txt",
            "/..%5csecret.txt",
            "/%ff",
            "/%2",
            "index.html",
        ] {
            assert!(resolve(&cache, url).is_none(), "{url}");
        }
    }
}
