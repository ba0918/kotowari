//! "kotowari overview serve"（docs/ir/core/overview-commands.md の REQ-core-297〜REQ-core-299）。
//! サーバは URL の1行を読んでから要求を送り、時間を待たない

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use tempfile::TempDir;

const IR: &str = "# CLI\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: read\n\nBody.\n";

fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

fn make_project(config: &str, lead: bool) -> TempDir {
    let tmp = TempDir::new().unwrap();
    for dir in [".kotowari", "docs/decision/adr"] {
        std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    write(tmp.path(), "docs/ir/cli.md", IR);
    write(
        tmp.path(),
        "docs/decision/records/r.md",
        "# R\n\n## Context\n\nc\n\n## Agreements\n\n- A1 決めた\n  - why: w\n",
    );
    write(
        tmp.path(),
        ".kotowari/config.yaml",
        &format!("tests:\n  files: []\n{config}"),
    );
    let lead = if lead {
        "```view lead\nconclusion: 結論\n```\n"
    } else {
        ""
    };
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &format!("---\nir:\n  - docs/ir/cli.md\n---\n\n# A\n\n{lead}\n## 節\n\n文。\n"),
    );
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: 目次\nitems: [a]\n",
    );
    tmp
}

const OVERVIEW: &str =
    "overview:\n  files: ['.kotowari/overview/*.md']\n  toc: .kotowari/toc.yaml\n";

/// 空いているポート。OS に選ばせてすぐ離す
fn free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn kotowari() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin("kotowari"))
}

/// serve を起動し、標準出力の1行目（URL）を読む
fn serve(tmp: &Path, port: u16) -> (Child, BufReader<std::process::ChildStdout>, String) {
    let mut child = kotowari()
        .args(["overview", "serve", "--port", &port.to_string()])
        .current_dir(tmp)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let mut line = String::new();
    stdout.read_line(&mut line).unwrap();
    (child, stdout, line)
}

/// 生の道で要求を送り、状態の行と頭と本体を返す
fn get(port: u16, path: &str) -> (String, String, Vec<u8>) {
    get_with_host(port, path, Some(&format!("127.0.0.1:{port}")))
}

/// Host ヘッダーを選んで（None なら付けずに）要求を送る
fn get_with_host(port: u16, path: &str, host: Option<&str>) -> (String, String, Vec<u8>) {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    let host = host
        .map(|host| format!("Host: {host}\r\n"))
        .unwrap_or_default();
    write!(
        stream,
        "GET {path} HTTP/1.1\r\n{host}Connection: close\r\n\r\n"
    )
    .unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();
    let split = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .unwrap();
    let head = String::from_utf8_lossy(&response[..split]).to_string();
    let status = head.lines().next().unwrap().to_string();
    (status, head, response[split + 4..].to_vec())
}

#[cfg(unix)]
fn interrupt(child: &Child) {
    let status = Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap();
    assert!(status.success());
}

fn content_type(head: &str) -> Option<String> {
    head.lines().find_map(|line| {
        line.to_ascii_lowercase()
            .strip_prefix("content-type:")
            .map(|value| value.trim().to_string())
    })
}

// @kotowari[EX-core-478, REQ-core-297, REQ-core-299, REQ-core-294, REQ-core-296]
#[cfg(unix)]
#[test]
fn ex_core_478_serve_gives_only_the_files_in_the_cache_and_ends_with_0_on_interrupt() {
    let tmp = make_project(OVERVIEW, true);
    let port = free_port();
    let (mut child, mut stdout, line) = serve(tmp.path(), port);
    assert_eq!(line, format!("http://127.0.0.1:{port}/\n"));
    let cache = tmp.path().join(".kotowari/cache/overview");
    let (status, head, body) = get(port, "/");
    assert!(status.contains(" 200 "), "{status}");
    assert_eq!(body, std::fs::read(cache.join("index.html")).unwrap());
    assert_eq!(
        content_type(&head).as_deref(),
        Some("text/html; charset=utf-8")
    );
    let (status, head, body) = get(port, "/style.css");
    assert!(status.contains(" 200 "), "{status}");
    assert_eq!(body, std::fs::read(cache.join("style.css")).unwrap());
    assert_eq!(
        content_type(&head).as_deref(),
        Some("text/css; charset=utf-8")
    );
    let (status, _, _) = get(port, "/a.html");
    assert!(status.contains(" 200 "), "{status}");
    for path in ["/../config.yaml", "/%2e%2e/config.yaml", "/missing.html"] {
        let (status, _, body) = get(port, path);
        assert!(status.contains(" 404 "), "{path}: {status}");
        assert!(
            !String::from_utf8_lossy(&body).contains("overview:"),
            "{path}"
        );
    }
    interrupt(&child);
    assert_eq!(child.wait().unwrap().code(), Some(0));
    let mut rest = String::new();
    stdout.read_to_string(&mut rest).unwrap();
    assert_eq!(rest, "", "nothing is written per request");
}

// @kotowari[REQ-core-299]
#[cfg(unix)]
#[test]
fn req_core_299_links_out_of_the_cache_directories_and_absolute_paths_are_404() {
    let tmp = make_project(OVERVIEW, true);
    let port = free_port();
    let (mut child, _stdout, _) = serve(tmp.path(), port);
    let cache = tmp.path().join(".kotowari/cache/overview");
    std::os::unix::fs::symlink(
        tmp.path().join(".kotowari/config.yaml"),
        cache.join("out.html"),
    )
    .unwrap();
    std::os::unix::fs::symlink(cache.join("a.html"), cache.join("in.html")).unwrap();
    std::fs::create_dir(cache.join("dir")).unwrap();
    std::fs::write(cache.join("dir/x.html"), "x").unwrap();
    let absolute = format!("/{}", tmp.path().join(".kotowari/config.yaml").display());
    for path in ["/out.html", "/dir", "/dir/", &absolute, "//etc/passwd"] {
        let (status, _, body) = get(port, path);
        assert!(status.contains(" 404 "), "{path}: {status}");
        assert!(
            !String::from_utf8_lossy(&body).contains("overview:"),
            "{path}"
        );
    }
    let (status, _, _) = get(port, "/in.html");
    assert!(status.contains(" 200 "), "{status}");
    interrupt(&child);
    assert_eq!(child.wait().unwrap().code(), Some(0));
}

// @kotowari[REQ-core-299, REQ-core-297]
#[cfg(unix)]
#[test]
fn req_core_299_a_percent_encoded_page_name_is_decoded_to_the_file_in_the_cache() {
    let tmp = make_project(OVERVIEW, true);
    let data = tmp.path().join(".kotowari/overview");
    std::fs::rename(data.join("a.md"), data.join("変更 a.md")).unwrap();
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: 目次\nitems: ['変更 a']\n",
    );
    let port = free_port();
    let (mut child, _stdout, _) = serve(tmp.path(), port);
    let cache = tmp.path().join(".kotowari/cache/overview");
    // 一覧のページのリンクは、名前の UTF-8 のバイトをパーセント符号にした相対パスである
    let encoded: String = "変更 a.html"
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || byte == b'.' {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect();
    let (status, _, body) = get(port, &format!("/{encoded}"));
    assert!(status.contains(" 200 "), "{encoded}: {status}");
    assert_eq!(body, std::fs::read(cache.join("変更 a.html")).unwrap());
    // Unix ではバックスラッシュも名前の1文字で、置き場の中のファイルである
    std::fs::write(cache.join("b\\c.html"), "bc").unwrap();
    let (status, _, body) = get(port, "/b%5Cc.html");
    assert!(status.contains(" 200 "), "{status}");
    assert_eq!(body, std::fs::read(cache.join("b\\c.html")).unwrap());
    interrupt(&child);
    assert_eq!(child.wait().unwrap().code(), Some(0));
}

fn stopped(tmp: &Path, port: u16) -> (Option<i32>, String, String) {
    let output = kotowari()
        .args(["overview", "serve", "--port", &port.to_string()])
        .current_dir(tmp)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

// @kotowari[EX-core-479, REQ-core-298, TBL-core-001, TBL-core-018, TBL-core-020]
#[test]
fn ex_core_479_a_port_in_use_stops_with_a_port_error() {
    let tmp = make_project(OVERVIEW, true);
    let occupied = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = occupied.local_addr().unwrap().port();
    let (code, stdout, stderr) = stopped(tmp.path(), port);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert!(
        stderr.starts_with(&format!("port error: 127.0.0.1:{port}: ")),
        "{stderr}"
    );
    assert!(!tmp.path().join(".kotowari/cache").exists());
}

// @kotowari[REQ-core-294, REQ-core-297]
#[test]
fn req_core_294_serve_on_data_with_errors_stops_before_binding() {
    let tmp = make_project(OVERVIEW, false);
    // ポートが使用中でも、元データの誤りを先に報告する
    let occupied = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = occupied.local_addr().unwrap().port();
    let (code, stdout, stderr) = stopped(tmp.path(), port);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert_eq!(
        stderr.lines().next(),
        Some("overview error: 1 errors in overview data; run kotowari check")
    );
    assert!(!tmp.path().join(".kotowari/cache").exists());
}

// @kotowari[REQ-core-279]
#[test]
fn req_core_279_serve_without_the_overview_key_is_a_config_error() {
    let tmp = make_project("", true);
    let (code, _, stderr) = stopped(tmp.path(), free_port());
    assert_eq!(code, Some(2));
    assert_eq!(
        stderr.lines().next(),
        Some("config error: overview is not configured")
    );
}

// @kotowari[REQ-core-280]
#[test]
fn req_core_280_serve_stops_on_an_overlap_with_the_guides() {
    let tmp = make_project(
        "overview:\n  files: ['notes/*.md']\n  toc: .kotowari/toc.yaml\nguides:\n  files: ['notes/*.md']\n",
        true,
    );
    std::fs::create_dir_all(tmp.path().join("notes")).unwrap();
    std::fs::rename(
        tmp.path().join(".kotowari/overview/a.md"),
        tmp.path().join("notes/a.md"),
    )
    .unwrap();
    let (code, _, stderr) = stopped(tmp.path(), free_port());
    assert_eq!(code, Some(2));
    assert_eq!(
        stderr.lines().next(),
        Some("config error: notes/a.md: matched by both overview.files and guides.files")
    );
}

// @kotowari[REQ-core-294, REQ-core-328]
#[test]
fn req_core_294_serve_on_toc_errors_stops_before_binding() {
    let tmp = make_project(OVERVIEW, true);
    write(
        tmp.path(),
        ".kotowari/toc.yaml",
        "title: 目次\nitems: [z]\n",
    );
    let occupied = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = occupied.local_addr().unwrap().port();
    let (code, stdout, stderr) = stopped(tmp.path(), port);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert_eq!(
        stderr.lines().next(),
        Some("overview error: 2 errors in overview data; run kotowari check")
    );
    assert!(!tmp.path().join(".kotowari/cache").exists());
}

// @kotowari[EX-core-543, REQ-core-356]
#[cfg(unix)]
#[test]
fn ex_core_543_serve_answers_only_local_hosts() {
    let tmp = make_project(OVERVIEW, true);
    let port = free_port();
    let (mut child, mut stdout, _) = serve(tmp.path(), port);
    let index = std::fs::read(tmp.path().join(".kotowari/cache/overview/index.html")).unwrap();
    for host in [format!("127.0.0.1:{port}"), format!("LOCALHOST:{port}")] {
        let (status, _, body) = get_with_host(port, "/", Some(&host));
        assert!(status.contains(" 200 "), "{host}: {status}");
        assert_eq!(body, index, "{host}");
    }
    let other_port = format!("127.0.0.1:{}", port.wrapping_add(1));
    let evil = format!("evil.example:{port}");
    for host in [Some(evil.as_str()), Some(other_port.as_str()), None] {
        let (status, _, body) = get_with_host(port, "/", host);
        assert!(status.contains(" 403 "), "{host:?}: {status}");
        assert!(body.is_empty(), "{host:?}");
    }
    interrupt(&child);
    assert_eq!(child.wait().unwrap().code(), Some(0));
    let mut rest = String::new();
    stdout.read_to_string(&mut rest).unwrap();
    assert_eq!(rest, "", "nothing is written per request");
}
