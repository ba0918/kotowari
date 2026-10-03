use crate::ir;
use kotowari_core::{Finding, config::Config};
fn default_config() -> Config {
    Config::default()
}
fn find_by_kind<'a>(findings: &'a [Finding], kind: &str) -> Vec<&'a Finding> {
    findings.iter().filter(|f| f.kind() == kind).collect()
}
// @kotowari[REQ-core-033, REQ-core-037, TBL-core-005]
#[test]
fn req_033_subdirectories_are_read_at_any_depth() {
    let tmp = tempfile::tempdir().unwrap();
    for dir in ["docs/ir/sub/deep", "docs/ir/empty"] {
        std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    for file in ["a.md", "sub/b.md", "sub/deep/c.md"] {
        std::fs::write(tmp.path().join("docs/ir").join(file), "# Title\n\nScope.\n").unwrap();
    }
    let (docs, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    assert_eq!(docs.len(), 3);
    assert_eq!(docs.iter().map(|doc| doc.line_count()).sum::<usize>(), 9);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[EX-core-282]
#[test]
fn ex_core_282_finding_after_bare_cr_has_the_split_line_number() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir")).unwrap();
    std::fs::write(
        tmp.path().join("docs/ir/a.md"),
        "# Title\rScope.\r## Requirements\r### foo\r",
    )
    .unwrap();
    let (_, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    let uh = find_by_kind(&findings, "unknown_heading");
    assert_eq!(uh.len(), 1, "{findings:?}");
    assert_eq!(uh[0].line(), Some(4), "{findings:?}");
}

// @kotowari[REQ-core-033]
#[test]
#[cfg(unix)]
fn req_033_hidden_dir_and_dir_symlink_are_not_followed_at_any_depth() {
    use std::os::unix::{fs::symlink, net::UnixListener};
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("outside")).unwrap();
    std::fs::write(tmp.path().join("outside/a.md"), "bad").unwrap();
    for prefix in ["docs/ir", "docs/ir/sub/deep"] {
        let dir = tmp.path().join(prefix);
        std::fs::create_dir_all(dir.join(".hidden")).unwrap();
        symlink("missing", dir.join(".hidden/broken")).unwrap();
        symlink(tmp.path().join("outside"), dir.join("linked")).unwrap();
        std::fs::write(dir.join("a.MD"), "bad").unwrap();
        let _socket = UnixListener::bind(dir.join("socket.md")).unwrap();
        std::fs::write(dir.join("visible.md"), "# Title\n\nScope.\n").unwrap();
        symlink("visible.md", dir.join("file.md")).unwrap();
    }
    let (docs, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    assert_eq!(docs.len(), 4);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[REQ-core-033, REQ-core-018, TBL-core-020]
#[test]
#[cfg(unix)]
fn req_033_broken_symlink_in_a_subdirectory_stops() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    std::os::unix::fs::symlink("missing", tmp.path().join("docs/ir/sub/broken.md")).unwrap();
    let err = ir::load_and_check(tmp.path(), &default_config()).unwrap_err();
    assert!(
        matches!(err, kotowari_core::StopReason::UnreadableFile(ref detail)
        if detail.starts_with("docs/ir/sub/broken.md: "))
    );
}

// @kotowari[REQ-core-033, REQ-core-036, REQ-core-117]
#[test]
fn req_033_context_and_flags_in_a_subdirectory_are_glossary_and_flags() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    for file in ["CONTEXT.md", "FLAGS.md"] {
        std::fs::write(tmp.path().join("docs/ir/sub").join(file), "# Title\n").unwrap();
    }
    let (docs, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    assert_eq!(
        docs.iter().map(|d| d.kind()).collect::<Vec<_>>(),
        [
            kotowari_core::ir::DocKind::Glossary,
            kotowari_core::ir::DocKind::Flags
        ]
    );
    assert!(find_by_kind(&findings, "missing_scope").is_empty());
    assert_eq!(find_by_kind(&findings, "glossary_invalid").len(), 1);
}

// @kotowari[TBL-core-008, REQ-core-034, REQ-core-036, REQ-core-117]
#[test]
fn tbl_008_whole_document_detail_is_the_bare_filename_in_a_subdirectory() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/sub")).unwrap();
    for file in ["CONTEXT.md", "a.md"] {
        std::fs::write(tmp.path().join("docs/ir/sub").join(file), "").unwrap();
    }
    let (_, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    for (kind, expected) in [
        ("missing_title", vec!["CONTEXT.md", "a.md"]),
        ("missing_scope", vec!["a.md"]),
        ("glossary_invalid", vec!["CONTEXT.md"]),
    ] {
        assert_eq!(
            find_by_kind(&findings, kind)
                .iter()
                .map(|f| f.detail())
                .collect::<Vec<_>>(),
            expected
        );
    }
}

// @kotowari[REQ-core-032]
#[test]
fn req_032_first_occurrence_is_bytewise_first_relative_path() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/ir/a")).unwrap();
    for file in ["a.md", "a/b.md"] {
        std::fs::write(
            tmp.path().join("docs/ir").join(file),
            "# Title\n\nScope.\n## Requirements\n### REQ-001: Name\n",
        )
        .unwrap();
    }
    let (_, findings) = ir::load_and_check(tmp.path(), &default_config()).unwrap();
    let duplicates = find_by_kind(&findings, "duplicate_id");
    assert_eq!(duplicates.len(), 1);
    assert_eq!(duplicates[0].path(), "docs/ir/a/b.md");
    assert_eq!(duplicates[0].line(), Some(5));
}
