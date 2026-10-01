use assert_cmd::Command;
use std::{fs, path::Path, process::Command as Git};
fn git(root: &Path, args: &[&str]) -> String {
    let output = Git::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().into()
}
fn repository() -> tempfile::TempDir {
    let d = tempfile::tempdir().unwrap();
    git(d.path(), &["init", "-q", "-b", "main"]);
    git(d.path(), &["config", "user.name", "Test"]);
    git(d.path(), &["config", "user.email", "test@example.invalid"]);
    for path in [
        ".kotowari",
        "src",
        ".kotowari/changes",
        "docs/decision/records",
    ] {
        fs::create_dir_all(d.path().join(path)).unwrap();
    }
    fs::write(
        d.path().join(".kotowari/config.yaml"),
        "changes:\n  files: ['src/**']\n  records: ['.kotowari/changes/**']\n",
    )
    .unwrap();
    fs::write(d.path().join("src/a"), "before").unwrap();
    fs::write(
        d.path().join("docs/decision/records/test.md"),
        "# 判断\n\n## Context\n\n内容。\n\n## Agreements\n\n- A1 選択\n  - why: 根拠\n",
    )
    .unwrap();
    git(
        d.path(),
        &[
            "add",
            ".kotowari/config.yaml",
            "src/a",
            "docs/decision/records/test.md",
        ],
    );
    git(d.path(), &["commit", "-qm", "base"]);
    d
}
fn run(root: &Path, args: &[&str]) -> std::process::Output {
    Command::cargo_bin("kotowari")
        .unwrap()
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}
fn record(root: &Path, role: &str, base: &str) {
    let snapshot = kotowari_core::git_snapshot::read(
        root,
        base,
        kotowari_core::git_snapshot::Target::Index,
        None,
    )
    .unwrap();
    let file = &snapshot.files[0];
    let content = format!(
        "version: 1\nentries:\n- id: {role}\n  base: '{}'\n  role: {role}\n  files: [{{path: src/a, before: '{}', after: '{}'}}]\n  ir: []\n  conclusion: new\n  reason: 根拠\n  requirements: []\n  decisions: ['docs/decision/records/test.md#A1']\n  handoff: null\n  gaps: []\n",
        snapshot.base,
        file.before.as_ref().unwrap(),
        file.after.as_ref().unwrap()
    );
    let slot = if role == "implementer" {
        "implementation"
    } else {
        "review"
    };
    let path = format!(".kotowari/changes/{slot}.yaml");
    fs::write(root.join(&path), content).unwrap();
}

// @kotowari[REQ-core-001, REQ-core-004, REQ-core-240, REQ-core-263, EX-core-219, EX-core-241, EX-core-441]
#[test]
fn changes_requires_explicit_target_base_phase_and_rejects_other_commands_options() {
    let d = repository();
    let out = run(
        d.path(),
        &[
            "changes",
            "--base",
            "HEAD",
            "--staged",
            "--phase",
            "implementation",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    for args in [
        vec!["changes", "--base", "HEAD", "--staged"],
        vec!["changes", "--head", "HEAD", "--phase", "review"],
        vec!["changes", "--base", "HEAD", "--staged", "--phase", "review"],
        vec![
            "changes",
            "--base",
            "HEAD~1",
            "--staged",
            "--phase",
            "implementation",
        ],
        vec!["check", "--base", "HEAD"],
        vec![
            "changes",
            "--base",
            "HEAD",
            "--head",
            "HEAD",
            "--staged",
            "--phase",
            "implementation",
        ],
        vec![
            "changes", "--base", "HEAD", "--head", "HEAD", "--phase", "unknown",
        ],
    ] {
        assert_eq!(run(d.path(), &args).status.code(), Some(2), "{args:?}");
    }
}
// @kotowari[REQ-core-249, REQ-core-253, REQ-core-254, REQ-core-265, REQ-core-274, EX-core-443, EX-core-452]
#[test]
fn staged_records_are_required_and_commit_review_is_read_only() {
    let d = repository();
    let base = git(d.path(), &["rev-parse", "HEAD"]);
    fs::write(d.path().join("src/a"), "changed").unwrap();
    git(d.path(), &["add", "src/a"]);
    record(d.path(), "implementer", "HEAD");
    let args = [
        "changes",
        "--base",
        "HEAD",
        "--staged",
        "--phase",
        "implementation",
    ];
    assert_eq!(run(d.path(), &args).status.code(), Some(1));
    git(d.path(), &["add", ".kotowari/changes/implementation.yaml"]);
    assert!(run(d.path(), &args).status.success());
    record(d.path(), "reviewer", "HEAD");
    git(d.path(), &["add", ".kotowari/changes/review.yaml"]);
    git(d.path(), &["commit", "-qm", "target"]);
    let head = git(d.path(), &["rev-parse", "HEAD"]);
    let state = git(d.path(), &["status", "--porcelain"]);
    let out = run(
        d.path(),
        &[
            "changes", "--base", &base, "--head", &head, "--phase", "review",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["files"], 1);
    assert_eq!(value["covered"], 1);
    assert_eq!(value["base"], base);
    assert_eq!(value["target"], head);
    assert_eq!(value["phase"], "review");
    assert_eq!(value.as_object().unwrap().len(), 6);
    assert_eq!(git(d.path(), &["status", "--porcelain"]), state);
}
// @kotowari[REQ-core-003, REQ-core-019, REQ-core-264, REQ-core-265, TBL-core-020, EX-core-442]
#[test]
fn configuration_comes_from_target_and_git_root_from_nested_directory() {
    let d = repository();
    fs::write(d.path().join(".kotowari/config.yaml"), "invalid").unwrap();
    let args = [
        "changes",
        "--base",
        "HEAD",
        "--head",
        "HEAD",
        "--phase",
        "review",
        "--config",
        ".kotowari/config.yaml",
    ];
    assert!(run(&d.path().join("src"), &args).status.success());
    let dotted = [
        "changes",
        "--base",
        "HEAD",
        "--head",
        "HEAD",
        "--phase",
        "review",
        "--config",
        "./.kotowari/config.yaml",
    ];
    assert!(run(d.path(), &dotted).status.success());
    assert_eq!(
        run(
            d.path(),
            &[
                "changes", "--base", "missing", "--head", "HEAD", "--phase", "review"
            ]
        )
        .status
        .code(),
        Some(2)
    );
    fs::write(d.path().join(".kotowari/config.yaml"), "{}\n").unwrap();
    git(d.path(), &["add", ".kotowari/config.yaml"]);
    let out = run(
        d.path(),
        &[
            "changes",
            "--base",
            "HEAD",
            "--staged",
            "--phase",
            "implementation",
        ],
    );
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("config error"));
}
// @kotowari[REQ-core-247, REQ-core-242, EX-core-436]
#[test]
fn final_comparison_includes_earlier_uncovered_commits() {
    let d = repository();
    let base = git(d.path(), &["rev-parse", "HEAD"]);
    fs::write(d.path().join("src/a"), "changed").unwrap();
    git(d.path(), &["commit", "-qam", "earlier"]);
    fs::write(
        d.path().join(".kotowari/changes/empty.yaml"),
        "version: 1\nentries: []\n",
    )
    .unwrap();
    git(d.path(), &["add", ".kotowari/changes/empty.yaml"]);
    git(d.path(), &["commit", "-qm", "record"]);
    let out = run(
        d.path(),
        &[
            "changes", "--base", &base, "--head", "HEAD", "--phase", "review",
        ],
    );
    assert_eq!(out.status.code(), Some(1));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["files"], 1);
    assert_eq!(value["covered"], 0);
}

// @kotowari[REQ-core-265, REQ-core-267, REQ-core-253, EX-core-458, EX-core-459]
#[test]
fn replacing_current_records_preserves_past_commit_reverification() {
    let d = repository();
    let root = d.path();
    let base = git(root, &["rev-parse", "HEAD"]);
    fs::write(root.join("src/a"), "first").unwrap();
    git(root, &["add", "src/a"]);
    record(root, "implementer", &base);
    record(root, "reviewer", &base);
    git(root, &["add", ".kotowari/changes"]);
    git(root, &["commit", "-qm", "first"]);
    let first = git(root, &["rev-parse", "HEAD"]);
    assert!(
        run(
            root,
            &[
                "changes", "--base", &base, "--head", &first, "--phase", "review"
            ]
        )
        .status
        .success()
    );
    fs::remove_file(root.join(".kotowari/changes/review.yaml")).unwrap();
    git(root, &["add", ".kotowari/changes"]);
    let record_only = run(
        root,
        &[
            "changes",
            "--base",
            "HEAD",
            "--staged",
            "--phase",
            "implementation",
        ],
    );
    assert!(record_only.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&record_only.stdout).unwrap()["files"],
        0
    );

    fs::write(root.join("src/a"), "second").unwrap();
    git(root, &["add", "src/a"]);
    record(root, "implementer", &first);
    git(root, &["add", ".kotowari/changes"]);
    assert!(
        run(
            root,
            &[
                "changes",
                "--base",
                "HEAD",
                "--staged",
                "--phase",
                "implementation"
            ]
        )
        .status
        .success()
    );
    git(root, &["commit", "-qm", "second"]);
    let second = git(root, &["rev-parse", "HEAD"]);
    assert!(
        !run(
            root,
            &[
                "changes", "--base", &first, "--head", &second, "--phase", "review"
            ]
        )
        .status
        .success()
    );
    assert!(
        run(
            root,
            &[
                "changes", "--base", &base, "--head", &first, "--phase", "review"
            ]
        )
        .status
        .success()
    );
}

// @kotowari[REQ-core-272, REQ-core-273, EX-core-460]
#[test]
fn changing_comparison_base_rejects_old_current_records() {
    let d = repository();
    let root = d.path();
    let base = git(root, &["rev-parse", "HEAD"]);
    fs::write(root.join("src/a"), "changed").unwrap();
    git(root, &["add", "src/a"]);
    record(root, "implementer", &base);
    record(root, "reviewer", &base);
    git(root, &["add", ".kotowari/changes"]);
    git(root, &["commit", "-qm", "recorded change"]);
    git(
        root,
        &["commit", "--allow-empty", "-qm", "new comparison base"],
    );
    let newer = git(root, &["rev-parse", "HEAD"]);
    fs::write(root.join("src/a"), "next").unwrap();
    git(root, &["add", "src/a"]);
    git(root, &["commit", "-qm", "next"]);
    let result = run(
        root,
        &[
            "changes", "--base", &newer, "--head", "HEAD", "--phase", "review",
        ],
    );
    assert_eq!(result.status.code(), Some(1));
    assert!(
        String::from_utf8(result.stdout)
            .unwrap()
            .contains("change_uncovered")
    );
}
