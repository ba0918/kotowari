#![cfg(unix)]
use assert_cmd::Command;
use std::fs;
use std::os::unix::ffi::OsStringExt;

fn byte_filename_project(group: &str) -> tempfile::TempDir {
    let project = tempfile::tempdir().unwrap();
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        group,
        "rules",
    ] {
        fs::create_dir_all(project.path().join(dir)).unwrap();
    }
    let extra = match group {
        "tests" => "tests:\n  files: ['tests/**']\n",
        "guides" => "tests:\n  files: []\nguides:\n  files: ['guides/**']\n",
        "surface" => {
            "tests:\n  files: []\nsurface:\n  files: ['surface/**']\n  rules: ['rules/surface.yaml']\n"
        }
        "records" => {
            "tests:\n  files: []\nchanges:\n  files: ['src/**']\n  records: ['records/**']\n"
        }
        _ => unreachable!(),
    };
    fs::write(project.path().join(".kotowari/config.yaml"), extra).unwrap();
    fs::write(
        project.path().join("rules/surface.yaml"),
        "id: command\nlanguage: rust\nrule:\n  pattern: 'fn $NAME() { $$$ }'\n",
    )
    .unwrap();
    project
}

fn byte_path(project: &std::path::Path, group: &str, extension: &str) -> std::path::PathBuf {
    let mut name = b"a\xff.".to_vec();
    name.extend(extension.as_bytes());
    project.join(group).join(std::ffi::OsString::from_vec(name))
}

fn native_check(project: &std::path::Path) -> std::process::Output {
    Command::cargo_bin("kotowari")
        .unwrap()
        .args(["check", "--format", "json"])
        .current_dir(project)
        .output()
        .unwrap()
}

// @kotowari[REQ-core-313, REQ-core-018, REQ-core-198, REQ-core-224, EX-core-485]
#[test]
fn byte_named_files_without_legacy_read_aliases_keep_the_unreadable_stop() {
    for (group, extension) in [
        ("tests", "rs"),
        ("guides", "md"),
        ("surface", "rs"),
        ("records", "yaml"),
    ] {
        let project = byte_filename_project(group);
        fs::write(
            byte_path(project.path(), group, extension),
            "#[test]\nfn original() {}\n",
        )
        .unwrap();
        let output = native_check(project.path());
        assert_eq!(
            output.status.code(),
            Some(2),
            "{group}: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(output.stdout.is_empty());
        let display = format!("{group}/a�.{extension}");
        let expected = fs::read(project.path().join(&display)).unwrap_err();
        assert_eq!(
            String::from_utf8(output.stderr).unwrap(),
            format!("unreadable file: {display}: {expected}\n")
        );
    }
}

// @kotowari[REQ-core-313, REQ-core-198, REQ-core-206, REQ-core-224, EX-core-485]
#[test]
fn distinct_byte_named_entries_keep_the_legacy_alias_text_and_multiplicity() {
    for (group, extension, original, replacement) in [
        (
            "tests",
            "rs",
            "#[test]\nfn original() {}\n",
            "#[test]\nfn replacement() {}\n",
        ),
        (
            "guides",
            "md",
            "<!-- @kotowari[REQ-001:12345678] -->\n",
            "replacement\n",
        ),
        (
            "surface",
            "rs",
            "fn original() {}\n",
            "fn replacement() {}\n",
        ),
        ("records", "yaml", "original", "version: 1\nentries: []\n"),
    ] {
        let project = byte_filename_project(group);
        let original_path = byte_path(project.path(), group, extension);
        if group == "records" {
            fs::write(original_path, [0xff]).unwrap();
        } else {
            fs::write(original_path, original).unwrap();
        }
        fs::write(
            project.path().join(group).join(format!("a�.{extension}")),
            replacement,
        )
        .unwrap();
        let output = native_check(project.path());
        assert_eq!(
            output.status.code(),
            Some(if group == "tests" || group == "surface" {
                1
            } else {
                0
            }),
            "{group}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stderr.is_empty());
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        match group {
            "tests" => {
                assert_eq!(value["tests"]["rs"]["files"], 2);
                assert_eq!(
                    value["findings"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|finding| finding["detail"].as_str().unwrap())
                        .collect::<Vec<_>>(),
                    ["replacement", "replacement"]
                );
            }
            "guides" => {
                assert_eq!(value["guides"]["files"], 2);
                assert_eq!(value["guides"]["marks"], 0);
                assert!(value["findings"].as_array().unwrap().is_empty());
            }
            "surface" => {
                let findings = value["findings"].as_array().unwrap();
                assert_eq!(findings.len(), 1);
                assert_eq!(findings[0]["kind"], "surface_without_spec");
                assert_eq!(findings[0]["detail"], "command replacement");
            }
            "records" => assert!(value["findings"].as_array().unwrap().is_empty()),
            _ => unreachable!(),
        }
    }
}

// @kotowari[REQ-core-313, REQ-core-092, EX-core-485]
#[test]
fn ir_and_decision_source_walkers_still_read_original_byte_named_paths() {
    let project = byte_filename_project("tests");
    fs::write(
        project.path().join(".kotowari/config.yaml"),
        "tests:\n  files: []\n",
    )
    .unwrap();
    fs::write(
        byte_path(project.path(), "docs/ir", "md"),
        "# Topic\n\nScope.\n",
    )
    .unwrap();
    fs::write(
        byte_path(project.path(), "docs/decision/records", "md"),
        "# Records\n\n## Agreements\n\n- A1 Original\n",
    )
    .unwrap();
    fs::write(
        byte_path(project.path(), "docs/decision/adr", "md"),
        "# Original\n",
    )
    .unwrap();
    let output = native_check(project.path());
    assert_ne!(
        output.status.code(),
        Some(2),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["files"], 1);
}

// @kotowari[REQ-core-313, EX-core-485]
#[test]
fn distinct_native_files_with_equal_display_paths_keep_both_counts_and_findings() {
    let project = tempfile::tempdir().unwrap();
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "tests/a",
        "guides/a",
    ] {
        fs::create_dir_all(project.path().join(dir)).unwrap();
    }
    fs::write(
        project.path().join(".kotowari/config.yaml"),
        "tests:\n  files: ['tests/**']\nguides:\n  files: ['guides/**']\n",
    )
    .unwrap();
    for (path, text) in [
        ("tests/a\\b.rs", "#[test]\nfn first() {}\n"),
        ("tests/a/b.rs", "#[test]\nfn second() {}\n"),
        ("guides/a\\b.md", "first\n"),
        ("guides/a/b.md", "second\n"),
    ] {
        fs::write(project.path().join(path), text).unwrap();
    }
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
    assert_eq!(value["tests"]["rs"]["files"], 2);
    assert_eq!(value["guides"]["files"], 2);
    let findings = value["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 2, "{value}");
    assert_eq!(
        findings
            .iter()
            .map(|finding| finding["detail"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["first", "second"]
    );
    assert!(
        findings.iter().all(
            |finding| finding["path"] == "tests/a/b.rs" && finding["kind"] == "test_without_id"
        )
    );
}

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

// @kotowari[REQ-core-313, EX-core-485]
#[test]
fn configured_test_rules_match_the_original_acquired_path() {
    let project = tempfile::tempdir().unwrap();
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "tests",
        "rules",
    ] {
        fs::create_dir_all(project.path().join(dir)).unwrap();
    }
    fs::write(
        project.path().join(".kotowari/config.yaml"),
        "tests:\n  files: ['tests/**']\n  rules: ['rules/test.yaml']\n",
    )
    .unwrap();
    fs::write(
        project.path().join("tests/a\\..\\b.rs"),
        "// @kotowari[REQ-001]\nfn configured() {}\n",
    )
    .unwrap();
    fs::write(project.path().join("rules/test.yaml"), "id: configured\nlanguage: rust\nfiles: ['tests/a/../b.rs']\nrule:\n  pattern: 'fn $NAME() { $$$ }'\n").unwrap();
    let output = Command::cargo_bin("kotowari")
        .unwrap()
        .args(["check", "--format", "json"])
        .current_dir(project.path())
        .output()
        .unwrap();
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let findings: Vec<_> = value["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == "unresolved_reference")
        .collect();
    assert_eq!(findings.len(), 1, "{value}");
    assert_eq!(findings[0]["path"], "tests/a/../b.rs");
}
