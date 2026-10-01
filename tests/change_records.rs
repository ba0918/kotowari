use kotowari_core::config::Config;

// @kotowari[REQ-core-016, REQ-core-264, TBL-core-004, EX-core-442]
#[test]
fn changes_configuration_accepts_explicit_nonempty_globs() {
    let cfg = Config::parse("changes:\n  files: ['src/**']\n  records: ['docs/changes/**']\n");
    assert!(cfg.is_ok(), "{cfg:?}");
}

// @kotowari[REQ-core-264]
#[test]
fn changes_configuration_rejects_invalid_values() {
    for body in [
        "null",
        "{}",
        "{files: [], records: ['a']}",
        "{files: ['a'], records: []}",
        "{files: [''], records: ['a']}",
        "{files: ['['], records: ['a']}",
        "{files: ['/a'], records: ['a']}",
        "{files: ['a'], records: ['a'], exclude: null}",
        "{files: ['a'], records: ['a'], extra: []}",
    ] {
        assert!(
            Config::parse(&format!("changes: {body}")).is_err(),
            "{body}"
        );
    }
}

fn project(record: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for path in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
        "docs/changes",
    ] {
        std::fs::create_dir_all(dir.path().join(path)).unwrap();
    }
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "changes:\n  files: ['src/**']\n  records: ['docs/changes/**']\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("docs/decision/records/test.md"),
        "# 判断\n\n## Context\n\nテスト\n\n## Agreements\n\n- A1 選択\n  - why: 根拠\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("docs/changes/test.yaml"), record).unwrap();
    dir
}

fn entry() -> String {
    format!(
        "version: 1\nentries:\n  - id: entry\n    base: '{}'\n    role: implementer\n    files:\n      - path: src/main.rs\n        before: null\n        after: 'sha256:{}'\n    ir: []\n    conclusion: new\n    reason: 根拠\n    requirements: []\n    decisions: ['docs/decision/records/test.md#A1']\n    handoff: null\n    gaps: []\n",
        "0".repeat(40),
        "a".repeat(64)
    )
}

fn invalids(record: &str) -> Vec<kotowari_core::Finding> {
    let dir = project(record);
    let (result, _) =
        kotowari_core::run_check(dir.path(), kotowari_core::Format::Json, None).unwrap();
    result
        .findings
        .into_iter()
        .filter(|f| f.kind.as_str() == "change_record_invalid")
        .collect()
}

// @kotowari[REQ-core-268, REQ-core-249, EX-core-446]
#[test]
fn unknown_record_version_is_a_static_finding() {
    let findings = invalids("version: 2\nentries: []\n");
    assert_eq!(findings.len(), 1);
    assert!(findings[0].detail.starts_with("file: "));
}

// @kotowari[REQ-core-248, REQ-core-268, REQ-core-269, REQ-core-273]
#[test]
fn malformed_current_entries_are_rejected() {
    for record in [
        entry().replace("    reason: 根拠\n", ""),
        entry().replace("src/main.rs", "../main.rs"),
        entry().replace("sha256:", "sha512:"),
        entry().replace("role: implementer", "role: unknown"),
        entry().replace("id: entry", "id: bad id"),
        entry().replace("    gaps: []", "    gaps: []\n    extra: true"),
        entry().replace("    reason: 根拠", "    reason: 根拠\n    reason: 重複"),
    ] {
        assert!(!invalids(&record).is_empty(), "{record}");
    }
}

// @kotowari[REQ-core-249, REQ-core-250, REQ-core-270, EX-core-437, EX-core-448]
#[test]
fn existing_conclusion_requires_requirement_and_definition_ir() {
    assert!(!invalids(&entry().replace("conclusion: new", "conclusion: existing")).is_empty());
}

// @kotowari[REQ-core-251, REQ-core-270, EX-core-438]
#[test]
fn new_conclusion_requires_decision_reference() {
    assert!(!invalids(&entry().replace("['docs/decision/records/test.md#A1']", "[]")).is_empty());
}

// @kotowari[REQ-core-251, REQ-core-270]
#[test]
fn one_missing_decision_is_reported_once() {
    let findings = invalids(&entry().replace("['docs/decision/records/test.md#A1']", "[]"));
    assert_eq!(findings.len(), 1, "{findings:?}");
}

// @kotowari[REQ-core-274]
#[test]
fn an_entry_without_an_id_is_named_by_position_only() {
    let findings = invalids("version: 1\nentries:\n  - 5\n");
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert!(findings[0].detail.starts_with("entry 0: "));
    assert!(
        !findings[0].detail.starts_with("entry 0: :"),
        "{}",
        findings[0].detail
    );
}

// @kotowari[REQ-core-252, REQ-core-270, EX-core-440]
#[test]
fn deferred_conclusion_requires_handoff() {
    assert!(!invalids(&entry().replace("conclusion: new", "conclusion: deferred")).is_empty());
}

// @kotowari[REQ-core-255, REQ-core-271, EX-core-449]
#[test]
fn unknown_gap_category_is_rejected() {
    let record = entry().replace("gaps: []", "gaps: [{category: unknown, disposition: recorded, refs: ['docs/decision/records/test.md#A1']}]");
    assert!(!invalids(&record).is_empty());
}

// @kotowari[REQ-core-273, EX-core-455]
#[test]
fn current_entry_requires_live_references() {
    let record = entry().replace("test.md#A1", "absent.md#A1");
    assert!(!invalids(&record).is_empty());
}

// @kotowari[REQ-core-249, REQ-core-254, EX-core-439]
#[test]
fn check_and_status_validate_records_without_git_or_extra_tallies() {
    let dir = project(&entry());
    let (result, _) =
        kotowari_core::run_check(dir.path(), kotowari_core::Format::Json, None).unwrap();
    assert!(
        !result
            .findings
            .iter()
            .any(|f| f.kind.as_str() == "change_record_invalid")
    );
    let dir = project("version: 2\nentries: []\n");
    let result = kotowari_core::run_status(dir.path(), None).unwrap();
    assert!(result.findings.error > 0);
    let value = serde_json::to_value(result).unwrap();
    assert!(value.get("changes").is_none());
}

// @kotowari[REQ-core-268, REQ-core-274]
#[test]
fn entry_shape_errors_identify_the_entry_position() {
    let findings = invalids(&entry().replace("    reason: 根拠\n", ""));
    assert!(
        findings[0].detail.starts_with("entry 0: "),
        "{}",
        findings[0].detail
    );
}

// @kotowari[REQ-core-268]
#[test]
fn null_lists_are_not_empty_lists() {
    assert!(!invalids("version: 1\nentries: null\n").is_empty());
    for (from, to) in [
        ("ir: []", "ir: null"),
        ("requirements: []", "requirements: null"),
        ("gaps: []", "gaps: null"),
    ] {
        assert!(!invalids(&entry().replace(from, to)).is_empty(), "{to}");
    }
}

// @kotowari[REQ-core-271, EX-core-453]
#[test]
fn recorded_and_fixed_gaps_can_share_a_new_conclusion() {
    let record = entry().replace("ir: []", &format!("ir: [{{path: docs/ir/core/topic.md, sha256: 'sha256:{}'}}]", "a".repeat(64)))
        .replace("requirements: []", "requirements: [REQ-core-999]")
        .replace("gaps: []", "gaps: [{category: missing_spec, disposition: recorded, refs: ['docs/decision/records/test.md#A1']}, {category: spec_conflict, disposition: fixed, refs: ['docs/decision/records/test.md#A1']}]");
    let dir = project(&record);
    std::fs::create_dir_all(dir.path().join("docs/ir/core")).unwrap();
    std::fs::write(dir.path().join("docs/ir/core/topic.md"), "# 範囲\n\n内容。\n\n## Requirements\n\n### REQ-core-999: 既存\n\n- kind: ubiquitous\n- source: docs/decision/records/test.md#A1\n- verification: unit\n\n既存。\n").unwrap();
    let (result, _) =
        kotowari_core::run_check(dir.path(), kotowari_core::Format::Json, None).unwrap();
    assert!(
        !result
            .findings
            .iter()
            .any(|f| f.kind.as_str() == "change_record_invalid"),
        "{:?}",
        result.findings
    );
}

// @kotowari[REQ-core-268, REQ-core-270, REQ-core-273]
#[test]
fn current_records_require_conclusion_shape_and_live_references() {
    for record in [
        entry().replace("conclusion: new", "conclusion: existing"),
        entry().replace("['docs/decision/records/test.md#A1']", "[]"),
        entry().replace("test.md#A1", "test.md#bad"),
    ] {
        assert!(!invalids(&record).is_empty());
    }
}

// @kotowari[REQ-core-274]
#[test]
fn reference_error_positions_are_local_to_each_record_file() {
    let dir = project(&entry());
    let second = entry()
        .replace("id: entry", "id: other")
        .replace("test.md#A1", "missing.md#A1");
    std::fs::write(dir.path().join("docs/changes/z-other.yaml"), second).unwrap();
    let (result, _) =
        kotowari_core::run_check(dir.path(), kotowari_core::Format::Json, None).unwrap();
    let f = result
        .findings
        .iter()
        .find(|f| f.path == "docs/changes/z-other.yaml")
        .unwrap();
    assert!(f.detail.starts_with("entry 0: "), "{}", f.detail);
}

// @kotowari[REQ-core-268, EX-core-456]
#[test]
fn current_record_accepts_no_state_and_rejects_legacy_state() {
    assert!(invalids(&entry()).is_empty());
    for state in ["active", "archived"] {
        let record = entry().replace(
            "    role: implementer",
            &format!("    role: implementer\n    state: {state}"),
        );
        assert!(!invalids(&record).is_empty());
    }
}

// @kotowari[REQ-core-019, REQ-core-249, REQ-core-253, EX-core-457]
#[test]
fn explicitly_named_hidden_records_are_checked_without_reading_other_hidden_dirs() {
    let dir = project("version: 1\nentries: []\n");
    std::fs::create_dir_all(dir.path().join(".kotowari/changes")).unwrap();
    std::fs::create_dir_all(dir.path().join(".hidden")).unwrap();
    std::fs::write(
        dir.path().join(".hidden/record.yaml"),
        "version: 2\nentries: []\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "changes:\n  files: ['src/**']\n  records: ['.kotowari/changes/*.yaml', '**/*.yaml']\n",
    )
    .unwrap();
    for content in [
        "version: 2\nentries: []\n".to_string(),
        entry().replace("test.md#A1", "missing.md#A1"),
    ] {
        std::fs::write(dir.path().join(".kotowari/changes/commit.yaml"), content).unwrap();
        let (result, _) =
            kotowari_core::run_check(dir.path(), kotowari_core::Format::Json, None).unwrap();
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.path == ".kotowari/changes/commit.yaml"
                    && f.kind.as_str() == "change_record_invalid")
        );
        assert!(
            !result
                .findings
                .iter()
                .any(|f| f.path.starts_with(".hidden/"))
        );
        assert!(
            kotowari_core::run_status(dir.path(), None)
                .unwrap()
                .findings
                .error
                > 0
        );
    }
}
