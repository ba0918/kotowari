#![cfg(unix)]
use assert_cmd::Command;
use std::fs;

// @kotowari[REQ-core-313, EX-core-485]
#[test]
fn acquired_diagnostic_paths_do_not_collapse_filename_components() {
    let project = tempfile::tempdir().unwrap();
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "guides",
        "tests",
        "records",
    ] {
        fs::create_dir_all(project.path().join(dir)).unwrap();
    }
    fs::write(project.path().join(".kotowari/config.yaml"), "tests:\n  files: ['tests/**']\nguides:\n  files: ['guides/**']\nchanges:\n  files: ['src/**']\n  records: ['records/**']\n").unwrap();
    fs::write(
        project.path().join("guides/a\\..\\b.md"),
        "<!-- @kotowari[REQ-001:12345678] -->\n",
    )
    .unwrap();
    fs::write(
        project.path().join("tests/a\\..\\b.txt"),
        "@kotowari[REQ-001]\n",
    )
    .unwrap();
    fs::write(
        project.path().join("records/a\\..\\b.yaml"),
        "[not: valid\n",
    )
    .unwrap();
    let output = Command::cargo_bin("kotowari")
        .unwrap()
        .args(["check", "--format", "json"])
        .current_dir(project.path())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    for (kind, path) in [
        ("guide_stale", "guides/a/../b.md"),
        ("unresolved_reference", "tests/a/../b.txt"),
        ("change_record_invalid", "records/a/../b.yaml"),
    ] {
        let findings: Vec<_> = value["findings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|finding| finding["kind"] == kind)
            .collect();
        assert_eq!(findings.len(), 1, "{kind}: {value}");
        assert_eq!(findings[0]["path"], path);
    }
}
