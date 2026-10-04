//! "kotowari overview build"（docs/ir/core/overview-commands.md、docs/ir/core/cli.md）

use std::path::Path;
use tempfile::TempDir;

const IR: &str = "# CLI\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- source: docs/decision/records/r.md#A1\n- verification: review\n- how_to_verify: read\n\nBody.\n";

const OVERVIEW: &str =
    "overview:\n  files:\n    - \".kotowari/overview/*.md\"\n  toc: .kotowari/toc.yaml\n";

fn make_project(tmp: &Path, config: &str) {
    for dir in [".kotowari", "docs/decision/adr", "tests"] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    write(tmp, "docs/ir/cli.md", IR);
    write(tmp, "docs/ir/other.md", &IR.replace("REQ-001", "REQ-002"));
    write(
        tmp,
        "docs/decision/records/r.md",
        "# R\n\n## Context\n\nc\n\n## Agreements\n\n- A1 決めた\n  - why: w\n",
    );
    write(
        tmp,
        ".kotowari/config.yaml",
        &format!("tests:\n  files:\n    - \"tests/**/*.rs\"\n{config}"),
    );
}

fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

fn run(tmp: &Path, args: &[&str]) -> (Option<i32>, String, String) {
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(args)
        .current_dir(tmp)
        .output()
        .unwrap();
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn json(stdout: &str) -> serde_json::Value {
    serde_json::from_str(stdout).unwrap_or_else(|e| panic!("{e}: {stdout}"))
}

fn data(ir: &str, title: &str, text: &str) -> String {
    format!(
        "---\nir:\n  - {ir}\n---\n\n# {title}\n\n```view lead\nconclusion: 結論\n```\n\n## 節\n\n{text}\n"
    )
}

/// a.md と b.md の2つの元データを持つプロジェクト
fn two_documents() -> TempDir {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), OVERVIEW);
    write(
        tmp.path(),
        ".kotowari/overview/a.md",
        &data("docs/ir/cli.md", "A", "a の文"),
    );
    write(
        tmp.path(),
        ".kotowari/overview/b.md",
        &data("docs/ir/other.md", "B", "b の文"),
    );
    tmp
}

const CACHE: &str = ".kotowari/cache/overview";

/// 置き場の下のファイルの名前と中身
fn cache(tmp: &Path) -> Vec<(String, Vec<u8>)> {
    let mut files: Vec<(String, Vec<u8>)> = std::fs::read_dir(tmp.join(CACHE))
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            (
                entry.file_name().to_string_lossy().to_string(),
                std::fs::read(entry.path()).unwrap(),
            )
        })
        .collect();
    files.sort();
    files
}

fn strings(value: &serde_json::Value) -> Vec<&str> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_str().unwrap())
        .collect()
}

// @kotowari[REQ-core-293, REQ-core-295, REQ-core-001]
#[test]
fn req_core_293_the_first_build_writes_every_page_under_the_cache() {
    let tmp = two_documents();
    let (code, stdout, stderr) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0), "{stderr}");
    let value = json(&stdout);
    assert_eq!(
        strings(&value["written"]),
        [
            ".kotowari/cache/overview/a.html",
            ".kotowari/cache/overview/b.html",
            ".kotowari/cache/overview/index.html",
            ".kotowari/cache/overview/style.css",
        ]
    );
    assert_eq!(strings(&value["removed"]), Vec::<&str>::new());
    assert_eq!(value["unchanged"], 0);
    let names: Vec<String> = cache(tmp.path())
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    assert_eq!(names, ["a.html", "b.html", "index.html", "style.css"]);
    let a = std::fs::read_to_string(tmp.path().join(CACHE).join("a.html")).unwrap();
    assert!(a.contains("a の文"));
}

// @kotowari[EX-core-475, REQ-core-293, REQ-core-295]
#[test]
fn ex_core_475_a_second_build_writes_only_the_changed_page() {
    let tmp = two_documents();
    assert_eq!(run(tmp.path(), &["overview", "build"]).0, Some(0));
    // 書き直されれば更新時刻が今になるので、前の build のファイルを古い時刻にしておく
    let old = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
    let a = tmp.path().join(CACHE).join("a.html");
    std::fs::File::options()
        .write(true)
        .open(&a)
        .unwrap()
        .set_modified(old)
        .unwrap();
    write(
        tmp.path(),
        ".kotowari/overview/b.md",
        &data("docs/ir/other.md", "B", "b の新しい文"),
    );
    let (code, stdout, stderr) = run(tmp.path(), &["overview", "build", "--format", "json"]);
    assert_eq!(code, Some(0), "{stderr}");
    let value = json(&stdout);
    let mut keys: Vec<&String> = value.as_object().unwrap().keys().collect();
    keys.sort();
    assert_eq!(keys, ["removed", "unchanged", "written"]);
    assert_eq!(
        strings(&value["written"]),
        [".kotowari/cache/overview/b.html"]
    );
    assert_eq!(strings(&value["removed"]), Vec::<&str>::new());
    assert_eq!(value["unchanged"], 3);
    assert_eq!(std::fs::metadata(&a).unwrap().modified().unwrap(), old);
}

// @kotowari[EX-core-476, REQ-core-293]
#[test]
fn ex_core_476_the_page_of_removed_overview_data_is_removed() {
    let tmp = two_documents();
    assert_eq!(run(tmp.path(), &["overview", "build"]).0, Some(0));
    std::fs::remove_file(tmp.path().join(".kotowari/overview/b.md")).unwrap();
    let (code, stdout, _) = run(tmp.path(), &["overview", "build", "--format", "json"]);
    assert_eq!(code, Some(0));
    let value = json(&stdout);
    assert_eq!(
        strings(&value["removed"]),
        [".kotowari/cache/overview/b.html"]
    );
    assert!(!tmp.path().join(CACHE).join("b.html").exists());
    // 一覧は b を並べなくなったので書き直される
    assert_eq!(
        strings(&value["written"]),
        [".kotowari/cache/overview/index.html"]
    );
}

// @kotowari[REQ-core-293]
#[test]
fn req_core_293_any_other_file_under_the_cache_is_removed() {
    let tmp = two_documents();
    write(tmp.path(), ".kotowari/cache/overview/stray.txt", "x");
    let (code, stdout, _) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0));
    assert_eq!(
        strings(&json(&stdout)["removed"]),
        [".kotowari/cache/overview/stray.txt"]
    );
}

// @kotowari[REQ-core-295]
#[test]
fn req_core_295_the_text_format_writes_written_lines_before_removed_lines() {
    let tmp = two_documents();
    write(tmp.path(), ".kotowari/cache/overview/0-stray.txt", "x");
    let (code, stdout, _) = run(tmp.path(), &["--format", "text", "overview", "build"]);
    assert_eq!(code, Some(0));
    assert_eq!(
        stdout.lines().collect::<Vec<_>>(),
        [
            "written .kotowari/cache/overview/a.html",
            "written .kotowari/cache/overview/b.html",
            "written .kotowari/cache/overview/index.html",
            "written .kotowari/cache/overview/style.css",
            "removed .kotowari/cache/overview/0-stray.txt",
        ]
    );
    let (_, stdout, _) = run(tmp.path(), &["overview", "build", "--format", "text"]);
    assert_eq!(stdout, "");
}

// @kotowari[EX-core-477, REQ-core-294, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020]
#[test]
fn ex_core_477_errors_in_overview_data_stop_the_build_without_writing() {
    let tmp = two_documents();
    assert_eq!(run(tmp.path(), &["overview", "build"]).0, Some(0));
    let before = cache(tmp.path());
    write(
        tmp.path(),
        ".kotowari/overview/b.md",
        "---\nir:\n  - docs/ir/other.md\n---\n\n# B\n\n## 節\n\n文。\n",
    );
    let (code, stdout, stderr) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert_eq!(
        stderr.lines().next(),
        Some("overview error: 1 errors in overview data; run kotowari check")
    );
    assert_eq!(cache(tmp.path()), before);
}

// @kotowari[EX-core-463, REQ-core-279, TBL-core-020]
#[test]
fn ex_core_463_build_without_the_overview_key_is_a_config_error() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), "");
    write(tmp.path(), ".kotowari/overview/x.md", "# broken\n");
    let (code, stdout, stderr) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(2));
    assert!(stdout.is_empty());
    assert_eq!(
        stderr.lines().next(),
        Some("config error: overview is not configured")
    );
    assert!(!tmp.path().join(".kotowari/cache").exists());
}

// @kotowari[REQ-core-280, REQ-core-278]
#[test]
fn req_core_280_build_stops_on_an_overlap_with_the_test_files() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "overview:\n  files: ['tests/**/*.md']\n  toc: .kotowari/toc.yaml\n",
    );
    std::fs::write(
        tmp.path().join(".kotowari/config.yaml"),
        "tests:\n  files: ['tests/**']\noverview:\n  files: ['tests/**/*.md']\n  toc: .kotowari/toc.yaml\n",
    )
    .unwrap();
    write(tmp.path(), "tests/a.md", &data("docs/ir/cli.md", "A", "a"));
    let (code, _, stderr) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(2));
    assert_eq!(
        stderr.lines().next(),
        Some("config error: tests/a.md: matched by both overview.files and tests.files")
    );
}

// @kotowari[REQ-core-278, REQ-core-018]
#[test]
fn req_core_278_build_reads_the_ir_places_and_stops_like_check() {
    let tmp = two_documents();
    std::fs::remove_dir_all(tmp.path().join("docs/decision/adr")).unwrap();
    let (code, _, stderr) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(2));
    assert!(
        stderr.starts_with("unreadable file: docs/decision/adr"),
        "{stderr}"
    );
}

/// ディレクトリの全エントリを、相対パスと（ファイルなら）中身で写し取る
fn snapshot(root: &Path, skip: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    fn walk(root: &Path, dir: &Path, skip: &Path, out: &mut Vec<(String, Option<Vec<u8>>)>) {
        let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        paths.sort();
        for path in paths {
            if path.starts_with(skip) {
                continue;
            }
            let rel = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .to_string();
            if path.is_dir() {
                out.push((rel, None));
                walk(root, &path, skip, out);
            } else {
                out.push((rel, Some(std::fs::read(&path).unwrap())));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, skip, &mut out);
    out
}

// @kotowari[REQ-core-296, REQ-core-102]
#[test]
fn req_core_296_build_writes_nothing_outside_the_cache_place() {
    let tmp = two_documents();
    let home = TempDir::new().unwrap();
    let skip = tmp.path().join(".kotowari/cache");
    let before = snapshot(tmp.path(), &skip);
    let output = assert_cmd::Command::cargo_bin("kotowari")
        .unwrap()
        .args(["overview", "build"])
        .env("HOME", home.path())
        .env("TMPDIR", home.path())
        .current_dir(tmp.path())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(snapshot(tmp.path(), &skip), before);
    assert_eq!(snapshot(home.path(), &home.path().join("none")), []);
    let cache_entries: Vec<String> = std::fs::read_dir(&skip)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(cache_entries, ["overview"]);
}

// @kotowari[REQ-core-102]
#[test]
fn req_core_102_check_and_status_with_overview_data_write_nothing() {
    let tmp = two_documents();
    let before = snapshot(tmp.path(), &tmp.path().join("none"));
    for command in ["check", "status", "list"] {
        assert_ne!(run(tmp.path(), &[command]).0, Some(2), "{command}");
    }
    assert_eq!(snapshot(tmp.path(), &tmp.path().join("none")), before);
}

fn argument_error(tmp: &Path, args: &[&str]) {
    let (code, stdout, stderr) = run(tmp, args);
    assert_eq!(code, Some(2), "{args:?}");
    assert!(stdout.is_empty(), "{args:?}");
    assert!(stderr.starts_with("argument error: "), "{args:?}: {stderr}");
}

// @kotowari[REQ-core-304, TBL-core-001]
#[test]
fn req_core_304_overview_takes_exactly_build_or_serve() {
    let tmp = two_documents();
    argument_error(tmp.path(), &["overview"]);
    argument_error(tmp.path(), &["overview", "render"]);
    argument_error(tmp.path(), &["overview", "build", "extra"]);
    argument_error(tmp.path(), &["overview", "serve", "build"]);
    assert!(!tmp.path().join(".kotowari/cache").exists());
}

// @kotowari[REQ-core-304]
#[test]
fn req_core_304_the_port_is_a_decimal_integer_from_1_to_65535() {
    let tmp = two_documents();
    for port in [
        "0",
        "65536",
        "-1",
        "abc",
        "1.5",
        "",
        "0x50",
        "+80",
        "99999999999999999999",
    ] {
        argument_error(tmp.path(), &["overview", "serve", "--port", port]);
    }
    assert!(!tmp.path().join(".kotowari/cache").exists());
}

// @kotowari[REQ-core-004, REQ-core-002]
#[test]
fn req_core_004_port_belongs_to_serve_and_format_does_not() {
    let tmp = two_documents();
    argument_error(tmp.path(), &["overview", "build", "--port", "4590"]);
    argument_error(tmp.path(), &["check", "--port", "4590"]);
    argument_error(tmp.path(), &["overview", "serve", "--format", "json"]);
    argument_error(
        tmp.path(),
        &["overview", "build", "--tool", "cargo-mutants"],
    );
    argument_error(
        tmp.path(),
        &["overview", "build", "--format", "json", "--format", "text"],
    );
    assert!(!tmp.path().join(".kotowari/cache").exists());
}

// @kotowari[REQ-core-002, REQ-core-003]
#[test]
fn req_core_002_build_takes_format_and_config_before_or_after_the_command() {
    let tmp = two_documents();
    std::fs::rename(
        tmp.path().join(".kotowari/config.yaml"),
        tmp.path().join("other.yaml"),
    )
    .unwrap();
    let (code, stdout, stderr) = run(
        tmp.path(),
        &[
            "--config",
            "other.yaml",
            "overview",
            "--format",
            "text",
            "build",
        ],
    );
    assert_eq!(code, Some(0), "{stderr}");
    assert!(stdout.starts_with("written "), "{stdout}");
}

// @kotowari[REQ-core-296]
#[cfg(unix)]
#[test]
fn req_core_296_a_symbolic_link_in_the_cache_is_replaced_not_written_through() {
    let tmp = two_documents();
    std::fs::create_dir_all(tmp.path().join(CACHE)).unwrap();
    write(tmp.path(), "outside.css", "outside");
    std::os::unix::fs::symlink(
        tmp.path().join("outside.css"),
        tmp.path().join(CACHE).join("style.css"),
    )
    .unwrap();
    let (code, stdout, _) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(0));
    assert_eq!(
        std::fs::read_to_string(tmp.path().join("outside.css")).unwrap(),
        "outside"
    );
    let style = tmp.path().join(CACHE).join("style.css");
    assert!(
        !std::fs::symlink_metadata(&style)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert!(strings(&json(&stdout)["written"]).contains(&".kotowari/cache/overview/style.css"));
}

// @kotowari[EX-core-504, REQ-core-324, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020]
#[cfg(unix)]
#[test]
fn ex_core_504_a_linked_cache_place_stops_without_writing_outside() {
    let tmp = two_documents();
    let outside = TempDir::new().unwrap();
    write(outside.path(), "keep", "keep");
    std::fs::create_dir_all(tmp.path().join(".kotowari/cache")).unwrap();
    std::os::unix::fs::symlink(outside.path(), tmp.path().join(CACHE)).unwrap();
    let (code, stdout, stderr) = run(tmp.path(), &["overview", "build"]);
    assert_eq!(code, Some(2), "{stderr}");
    assert!(stdout.is_empty());
    assert!(
        stderr.starts_with("cache error: .kotowari/cache/overview"),
        "{stderr}"
    );
    let names: Vec<String> = std::fs::read_dir(outside.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(names, ["keep"]);
    assert_eq!(
        std::fs::read_to_string(outside.path().join("keep")).unwrap(),
        "keep"
    );
}

// @kotowari[REQ-core-324, REQ-core-296]
#[cfg(unix)]
#[test]
fn req_core_324_a_link_above_the_cache_place_also_stops() {
    for linked in [".kotowari/cache", ".kotowari"] {
        let tmp = two_documents();
        let outside = TempDir::new().unwrap();
        if linked == ".kotowari" {
            // 設定と元データはリンク先から読める。書き込みだけが外に届く
            std::fs::rename(tmp.path().join(".kotowari"), outside.path().join("k")).unwrap();
            std::os::unix::fs::symlink(outside.path().join("k"), tmp.path().join(linked)).unwrap();
        } else {
            std::os::unix::fs::symlink(outside.path(), tmp.path().join(linked)).unwrap();
        }
        let before = snapshot(outside.path(), &outside.path().join("none"));
        let (code, _, stderr) = run(tmp.path(), &["overview", "build"]);
        assert_eq!(code, Some(2), "{linked}: {stderr}");
        assert_eq!(
            stderr.lines().next(),
            Some(format!("cache error: {linked}").as_str()),
            "{linked}"
        );
        assert_eq!(
            snapshot(outside.path(), &outside.path().join("none")),
            before,
            "{linked}"
        );
    }
}
