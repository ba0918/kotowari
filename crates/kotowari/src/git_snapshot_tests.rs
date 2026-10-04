use crate::git_snapshot::{Target, read};
use std::{fs, path::Path, process::Command};

fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .current_dir(root)
        // The user's global hooks must not decide whether a fixture commit succeeds.
        .args(["-c", "core.hooksPath=/dev/null"])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).trim().into()
}
fn repository() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    git(d.path(), &["init", "-q", "-b", "main"]);
    git(d.path(), &["config", "user.name", "Test"]);
    git(d.path(), &["config", "user.email", "test@example.invalid"]);
    fs::create_dir_all(d.path().join(".kotowari")).unwrap();
    fs::create_dir_all(d.path().join("src")).unwrap();
    fs::write(
        d.path().join(".kotowari/config.yaml"),
        "changes:\n  files: ['**']\n  records: ['docs/changes/**']\n",
    )
    .unwrap();
    fs::write(d.path().join("src/a"), b"before\0bytes").unwrap();
    git(d.path(), &["add", ".kotowari/config.yaml", "src/a"]);
    git(d.path(), &["commit", "-qm", "base"]);
    d
}

// @kotowari[REQ-core-003, REQ-core-019, REQ-core-241, REQ-core-245, REQ-core-246, REQ-core-265, EX-core-434, EX-core-435, EX-core-443]
#[test]
fn index_snapshot_uses_staged_bytes_and_configuration() {
    let d = repository();
    fs::write(d.path().join("src/a"), b"staged\0bytes").unwrap();
    git(d.path(), &["add", "src/a"]);
    fs::write(d.path().join("src/a"), b"unstaged").unwrap();
    fs::write(d.path().join(".kotowari/config.yaml"), "invalid config").unwrap();
    let snapshot = read(d.path(), "HEAD", Target::Index, None).unwrap();
    assert_eq!(snapshot.files.len(), 1);
    assert_eq!(snapshot.blobs["src/a"].bytes, b"staged\0bytes");
    assert_eq!(snapshot.target, "index");
    assert_eq!(snapshot.base.len(), 40);
}

// @kotowari[REQ-core-241, REQ-core-247, REQ-core-266, EX-core-444]
#[test]
fn commit_snapshot_reports_move_as_delete_and_add() {
    let d = repository();
    git(d.path(), &["mv", "src/a", "src/b"]);
    git(d.path(), &["commit", "-qm", "move"]);
    let s = read(d.path(), "HEAD~1", Target::Commit("HEAD".into()), None).unwrap();
    assert_eq!(s.files.len(), 2);
    assert_eq!(s.files[0].path, "src/a");
    assert!(s.files[0].after.is_none());
    assert_eq!(s.files[1].path, "src/b");
    assert!(s.files[1].before.is_none());
}

// @kotowari[REQ-core-267, EX-core-445]
#[test]
fn records_ir_decisions_and_config_are_excluded() {
    let d = repository();
    for path in [
        "docs/changes/a.yaml",
        "docs/ir/core/a.md",
        "docs/decision/records/a.md",
        "docs/decision/adr/a.md",
    ] {
        fs::create_dir_all(d.path().join(path).parent().unwrap()).unwrap();
        fs::write(d.path().join(path), "content").unwrap();
        git(d.path(), &["add", path]);
    }
    let s = read(d.path(), "HEAD", Target::Index, None).unwrap();
    assert!(s.files.is_empty());
}

// @kotowari[REQ-core-266, REQ-core-269, EX-core-447]
#[test]
fn executable_mode_participates_in_full_byte_identity() {
    let d = repository();
    git(d.path(), &["update-index", "--chmod=+x", "src/a"]);
    let s = read(d.path(), "HEAD", Target::Index, None).unwrap();
    assert_eq!(s.files.len(), 1);
    assert_ne!(s.files[0].before, s.files[0].after);
    assert_eq!(s.blobs["src/a"].mode, "100755");
}

// @kotowari[REQ-core-265]
#[test]
fn conflict_index_and_unreadable_revision_stop() {
    let d = repository();
    assert!(read(d.path(), "missing", Target::Index, None).is_err());
    git(d.path(), &["checkout", "-qb", "other"]);
    fs::write(d.path().join("src/a"), "other").unwrap();
    git(d.path(), &["commit", "-qam", "other"]);
    git(d.path(), &["checkout", "-q", "main"]);
    fs::write(d.path().join("src/a"), "ours").unwrap();
    git(d.path(), &["commit", "-qam", "ours"]);
    Command::new("git")
        .current_dir(d.path())
        // The user's global hooks must not decide whether a fixture commit succeeds.
        .args(["-c", "core.hooksPath=/dev/null"])
        .args(["merge", "other"])
        .output()
        .unwrap();
    assert!(read(d.path(), "HEAD", Target::Index, None).is_err());
}

// @kotowari[REQ-core-266]
#[test]
fn selected_symlink_stops_but_excluded_symlink_does_not() {
    let d = repository();
    std::os::unix::fs::symlink("a", d.path().join("src/link")).unwrap();
    git(d.path(), &["add", "src/link"]);
    assert!(read(d.path(), "HEAD", Target::Index, None).is_err());
    fs::write(
        d.path().join(".kotowari/config.yaml"),
        "changes:\n  files: ['**']\n  exclude: ['src/link']\n  records: ['docs/changes/**']\n",
    )
    .unwrap();
    git(d.path(), &["add", ".kotowari/config.yaml"]);
    assert!(read(d.path(), "HEAD", Target::Index, None).is_ok());
}

// @kotowari[REQ-core-266]
#[test]
fn unchanged_symlink_is_not_a_selected_change() {
    let d = repository();
    std::os::unix::fs::symlink("a", d.path().join("src/link")).unwrap();
    git(d.path(), &["add", "src/link"]);
    git(d.path(), &["commit", "-qm", "link"]);
    assert!(read(d.path(), "HEAD", Target::Commit("HEAD".into()), None).is_ok());
}
