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

fn entry() -> String {
    format!(
        "version: 1\nentries:\n  - id: entry\n    base: '{}'\n    role: implementer\n    files:\n      - path: src/main.rs\n        before: null\n        after: 'sha256:{}'\n    ir: []\n    conclusion: new\n    reason: 根拠\n    requirements: []\n    decisions: ['docs/decision/records/test.md#A1']\n    handoff: null\n    gaps: []\n",
        "0".repeat(40),
        "a".repeat(64)
    )
}

fn invalids(record: &str) -> Vec<kotowari_core::Finding> {
    let config =
        Config::parse("changes:\n  files: ['src/**']\n  records: ['docs/changes/**']\n").unwrap();
    let context = kotowari_core::sources::SourceContext {
        records_path: config.decisions.records.clone(),
        adr_path: config.decisions.adr.clone(),
        records_files: vec![kotowari_core::sources::parse_records_file(
            "test.md",
            "# 判断\n\n## Context\n\nテスト\n\n## Agreements\n\n- A1 選択\n  - why: 根拠\n",
        )],
        records_other_files: vec![],
        adr_files: vec![],
    };
    let (mut entries, mut findings) =
        kotowari_core::change_records::parse("docs/changes/test.yaml", record);
    findings.extend(kotowari_core::change_records::validate_references(
        &mut entries,
        &Default::default(),
        &Default::default(),
        &context,
    ));
    findings
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

// @kotowari[REQ-core-032, REQ-core-268]
#[test]
fn a_duplicated_requirement_is_defined_by_the_first_document_in_path_order() {
    // 重複した ID の1つ目はパスのバイト順で先の文書。その文書を引く記録は定義の IR を持つ
    let config =
        Config::parse("changes:\n  files: ['src/**']\n  records: ['docs/changes/**']\n").unwrap();
    let ir = "# A\n\nScope.\n\n## Requirements\n\n### REQ-core-001: Name\n\n- kind: ubiquitous\n- source: docs/decision/records/test.md#A1\n- verification: unit\n\nStatement.\n";
    let doc = |name: &str| kotowari_core::ir::parse_document(name, ir).unwrap();
    let context = kotowari_core::sources::SourceContext {
        records_path: config.decisions.records.clone(),
        adr_path: config.decisions.adr.clone(),
        records_files: vec![kotowari_core::sources::parse_records_file(
            "test.md",
            "# 判断\n\n## Context\n\nテスト\n\n## Agreements\n\n- A1 選択\n  - why: 根拠\n",
        )],
        records_other_files: vec![],
        adr_files: vec![],
    };
    let record = entry()
        .replace(
            "ir: []",
            &format!(
                "ir: [{{path: docs/ir/a.md, sha256: 'sha256:{}'}}]",
                "b".repeat(64)
            ),
        )
        .replace("conclusion: new", "conclusion: existing")
        .replace("requirements: []", "requirements: [REQ-core-001]");
    // 渡す順に依らず、パスの順で決める
    for docs in [
        vec![doc("a.md"), doc("b.md")],
        vec![doc("b.md"), doc("a.md")],
    ] {
        let mut findings = Vec::new();
        kotowari_core::change_records::check_entries(
            [("docs/changes/test.yaml", record.as_str())],
            &config,
            &docs,
            &context,
            &mut findings,
        );
        assert!(findings.is_empty(), "{findings:?}");
    }
}
