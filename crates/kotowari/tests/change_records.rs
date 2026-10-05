#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
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

// @kotowari[REQ-core-249, REQ-core-254, EX-core-439]
#[test]
fn check_and_status_validate_records_without_git_or_extra_tallies() {
    let dir = project(&entry());
    let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .unwrap();
    assert!(
        !result
            .findings()
            .iter()
            .any(|f| f.kind().as_str() == "change_record_invalid")
    );
    let dir = project("version: 2\nentries: []\n");
    let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
        .unwrap()
        .status()
        .unwrap();
    assert!(result.findings().error() > 0);
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
    let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .unwrap();
    assert!(
        !result
            .findings()
            .iter()
            .any(|f| f.kind().as_str() == "change_record_invalid"),
        "{:?}",
        result.findings()
    );
}

// @kotowari[REQ-core-274]
#[test]
fn reference_error_positions_are_local_to_each_record_file() {
    let dir = project(&entry());
    let second = entry()
        .replace("id: entry", "id: other")
        .replace("test.md#A1", "missing.md#A1");
    std::fs::write(dir.path().join("docs/changes/z-other.yaml"), second).unwrap();
    let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .unwrap();
    let f = result
        .findings()
        .iter()
        .find(|f| f.path() == "docs/changes/z-other.yaml")
        .unwrap();
    assert!(f.detail().starts_with("entry 0: "), "{}", f.detail());
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
        let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
            .unwrap()
            .check()
            .unwrap();
        assert!(
            result
                .findings()
                .iter()
                .any(|f| f.path() == ".kotowari/changes/commit.yaml"
                    && f.kind().as_str() == "change_record_invalid")
        );
        assert!(
            !result
                .findings()
                .iter()
                .any(|f| f.path().starts_with(".hidden/"))
        );
        assert!(
            kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
                .unwrap()
                .status()
                .unwrap()
                .findings()
                .error()
                > 0
        );
    }
}

const TOPIC: &str = "# 範囲\n\n内容。\n\n## Requirements\n\n### REQ-core-999: 既存\n\n- kind: ubiquitous\n- source: docs/decision/records/test.md#A1\n- verification: unit\n\n既存。\n";
const OTHER: &str = "# 別\n\n内容。\n";

fn details_with_ir(record: &str) -> Vec<String> {
    let dir = project(record);
    std::fs::create_dir_all(dir.path().join("docs/ir/core")).unwrap();
    std::fs::write(dir.path().join("docs/ir/core/topic.md"), TOPIC).unwrap();
    std::fs::write(dir.path().join("docs/ir/core/other.md"), OTHER).unwrap();
    let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .unwrap();
    result
        .findings()
        .iter()
        .filter(|f| f.kind().as_str() == "change_record_invalid")
        .map(|f| f.detail().to_owned())
        .collect()
}

fn with_ir(record: &str, path: &str) -> String {
    record.replace(
        "ir: []",
        &format!(
            "ir: [{{path: {path}, sha256: 'sha256:{}'}}]",
            "a".repeat(64)
        ),
    )
}

// @kotowari[REQ-core-268]
#[test]
fn a_base_must_be_a_full_lowercase_hex_object_id() {
    for base in ["g".repeat(40), "a".repeat(39)] {
        let record = entry().replace(&"0".repeat(40), &base);
        let details = details_with_ir(&record);
        assert!(
            details.iter().any(|d| d.ends_with("invalid base")),
            "{base}: {details:?}"
        );
    }
}

// @kotowari[REQ-core-269]
#[test]
fn an_unnormalized_ir_path_is_reported_as_an_invalid_path() {
    let details = details_with_ir(&with_ir(&entry(), "../docs/ir/core/topic.md"));
    assert!(
        details
            .iter()
            .any(|d| d.ends_with("invalid or duplicate IR path")),
        "{details:?}"
    );
}

// @kotowari[REQ-core-271]
#[test]
fn a_recorded_gap_alone_needs_no_requirement() {
    let record = entry().replace("gaps: []", "gaps: [{category: missing_spec, disposition: recorded, refs: ['docs/decision/records/test.md#A1']}]");
    assert_eq!(details_with_ir(&record), Vec::<String>::new());
}

// @kotowari[REQ-core-270]
#[test]
fn an_existing_conclusion_without_ir_is_reported_as_lacking_both() {
    let record = entry()
        .replace("conclusion: new", "conclusion: existing")
        .replace("requirements: []", "requirements: [REQ-core-999]");
    let details = details_with_ir(&record);
    assert!(
        details
            .iter()
            .any(|d| d.ends_with("requirements and definition IR required")),
        "{details:?}"
    );
}

// @kotowari[REQ-core-271]
#[test]
fn only_fixed_gaps_require_an_existing_conclusion() {
    let record = with_ir(&entry(), "docs/ir/core/topic.md")
        .replace("requirements: []", "requirements: [REQ-core-999]")
        .replace("gaps: []", "gaps: [{category: spec_conflict, disposition: fixed, refs: ['docs/decision/records/test.md#A1']}]");
    let details = details_with_ir(&record);
    assert!(
        details
            .iter()
            .any(|d| d.ends_with("gap conclusion mismatch")),
        "{details:?}"
    );
}

// @kotowari[REQ-core-270]
#[test]
fn a_requirement_must_be_cited_with_the_ir_that_defines_it() {
    let record = with_ir(&entry(), "docs/ir/core/other.md")
        .replace("requirements: []", "requirements: [REQ-core-999]");
    let details = details_with_ir(&record);
    assert!(
        details
            .iter()
            .any(|d| d.ends_with("missing definition IR docs/ir/core/topic.md")),
        "{details:?}"
    );
}

// @kotowari[REQ-core-270, REQ-core-274]
#[test]
fn a_malformed_decision_reference_is_reported_once() {
    let record = entry().replace("test.md#A1", "test.md#bad");
    let details = details_with_ir(&record);
    assert_eq!(details.len(), 1, "{details:?}");
}

// @kotowari[REQ-core-249]
#[cfg(unix)]
#[test]
fn a_record_reached_through_a_file_symlink_is_checked() {
    let dir = project("version: 1\nentries: []\n");
    std::fs::create_dir_all(dir.path().join("store")).unwrap();
    std::fs::write(
        dir.path().join("store/real.yaml"),
        "version: 2\nentries: []\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(
        dir.path().join("store/real.yaml"),
        dir.path().join("docs/changes/link.yaml"),
    )
    .unwrap();
    let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .unwrap();
    assert!(
        result
            .findings()
            .iter()
            .any(|f| f.kind().as_str() == "change_record_invalid"
                && f.path() == "docs/changes/link.yaml"),
        "{:?}",
        result.findings()
    );
}

// @kotowari[REQ-core-019]
#[test]
fn a_records_glob_with_a_brace_across_a_slash_does_not_panic() {
    // 波括弧の中の "/" で切った前置きは glob にならない。設定の glob は有効なので、
    // 取得は panic せずに記録を読む
    let dir = project("version: 2\nentries: []\n");
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "changes:\n  files: ['src/**']\n  records: ['{docs/.changes,docs/changes}/*.yaml']\n",
    )
    .unwrap();
    let result = kotowari::Project::new(kotowari::ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .unwrap();
    assert!(
        result
            .findings()
            .iter()
            .any(|f| f.path() == "docs/changes/test.yaml"
                && f.kind().as_str() == "change_record_invalid"),
        "{:?}",
        result.findings()
    );
}
