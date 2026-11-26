use kotowari_core::{Finding, Inspection, StopReason};
use std::path::Path;
/// check と同じ経路で読んで検査する。設定は既定の値
fn inspect(base: &Path) -> Result<Inspection, StopReason> {
    crate::acquisition::load_with_guides(base, None)
}
/// check と同じ経路で、"docs/decisions" の決定の文書を読んで検査する
fn inspect_decisions(base: &Path) -> Result<Inspection, StopReason> {
    for dir in ["docs/decisions", ".kotowari"] {
        std::fs::create_dir_all(base.join(dir)).unwrap();
    }
    std::fs::write(
        base.join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\n",
    )
    .unwrap();
    inspect(base)
}
fn find_by_kind<'a>(findings: &'a [Finding], kind: &str) -> Vec<&'a Finding> {
    findings.iter().filter(|f| f.kind() == kind).collect()
}
// @kotowari[C-core-document-1]
#[test]
fn req_033_subdirectories_are_read_at_any_depth() {
    let tmp = tempfile::tempdir().unwrap();
    for dir in ["docs/decisions/sub/deep", "docs/decisions/empty"] {
        std::fs::create_dir_all(tmp.path().join(dir)).unwrap();
    }
    for file in ["a.md", "sub/b.md", "sub/deep/c.md"] {
        std::fs::write(
            tmp.path().join("docs/decisions").join(file),
            "# Title\n\nScope.\n",
        )
        .unwrap();
    }
    let inspection = inspect_decisions(tmp.path()).unwrap();
    let (docs, findings) = (
        inspection.read().decision_documents(),
        inspection.check().findings(),
    );
    assert_eq!(
        docs.iter().map(|doc| doc.path()).collect::<Vec<_>>(),
        [
            "docs/decisions/a.md",
            "docs/decisions/sub/b.md",
            "docs/decisions/sub/deep/c.md"
        ]
    );
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-core-document-2, C-core-document-3, C-core-document-4, C-core-document-13]
#[test]
#[cfg(unix)]
fn req_033_hidden_dir_and_dir_symlink_are_not_followed_at_any_depth() {
    use std::os::unix::{fs::symlink, net::UnixListener};
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("outside")).unwrap();
    std::fs::write(tmp.path().join("outside/a.md"), "bad").unwrap();
    for prefix in ["docs/decisions", "docs/decisions/sub/deep"] {
        let dir = tmp.path().join(prefix);
        std::fs::create_dir_all(dir.join(".hidden")).unwrap();
        symlink("missing", dir.join(".hidden/broken")).unwrap();
        symlink(tmp.path().join("outside"), dir.join("linked")).unwrap();
        std::fs::write(dir.join("a.MD"), "bad").unwrap();
        let _socket = UnixListener::bind(dir.join("socket.md")).unwrap();
        std::fs::write(dir.join("visible.md"), "# Title\n\nScope.\n").unwrap();
        symlink("visible.md", dir.join("file.md")).unwrap();
    }
    let inspection = inspect_decisions(tmp.path()).unwrap();
    let (docs, findings) = (
        inspection.read().decision_documents(),
        inspection.check().findings(),
    );
    assert_eq!(docs.len(), 4);
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[C-core-clienv-5, C-core-clienv-12, C-core-document-4]
#[test]
#[cfg(unix)]
fn req_033_broken_symlink_in_a_subdirectory_stops() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decisions/sub")).unwrap();
    std::os::unix::fs::symlink("missing", tmp.path().join("docs/decisions/sub/broken.md")).unwrap();
    let Err(err) = inspect_decisions(tmp.path()) else {
        panic!("a broken link in the place of decision documents stops");
    };
    assert!(
        matches!(err, kotowari_core::StopReason::UnreadableFile(ref detail)
        if detail.starts_with("docs/decisions/sub/broken.md: "))
    );
}

// @kotowari[C-core-findings-9]
#[test]
fn req_032_first_occurrence_is_bytewise_first_relative_path() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("docs/decisions/a")).unwrap();
    for file in ["a.md", "a/b.md"] {
        std::fs::write(
            tmp.path().join("docs/decisions").join(file),
            "# Title\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nP.\n\n### Decision\n\nD.\n\n### Confirmations\n\n- C-a-x-1 Seen\n",
        )
        .unwrap();
    }
    let inspection = inspect_decisions(tmp.path()).unwrap();
    let findings = inspection.check().findings();
    let duplicates = find_by_kind(findings, "duplicate_id");
    assert_eq!(duplicates.len(), 1, "{findings:?}");
    assert_eq!(duplicates[0].path(), "docs/decisions/a/b.md");
    assert_eq!(duplicates[0].line(), Some(19));
}
