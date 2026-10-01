use kotowari_core::{
    change_records::{self, LocatedEntry},
    changes::{Phase, evaluate},
    config::Config,
    git_snapshot::{Blob, Snapshot, ir_identity},
};
use std::collections::BTreeMap;
fn fixture() -> (Snapshot, Vec<LocatedEntry>) {
    let blob = Blob {
        mode: "100644".into(),
        bytes: b"new".to_vec(),
    };
    let file = change_records::FileChange {
        path: "src/a".into(),
        before: None,
        after: Some(blob.identity()),
    };
    let record = format!(
        "version: 1\nentries:\n- id: impl\n  base: '{}'\n  role: implementer\n  state: active\n  files: [{{path: src/a, before: null, after: '{}'}}]\n  ir: [{{path: docs/ir/a.md, sha256: '{}'}}]\n  conclusion: existing\n  reason: reason\n  requirements: [REQ-core-001]\n  decisions: []\n  handoff: null\n  gaps: []\n",
        "0".repeat(40),
        blob.identity(),
        ir_identity(b"IR")
    );
    let (entries, findings) = change_records::parse("docs/changes/test.yaml", &record);
    assert!(findings.is_empty());
    let s = Snapshot {
        base: "0".repeat(40),
        target: "1".repeat(40),
        config: Config::default(),
        files: vec![file],
        blobs: BTreeMap::from([
            ("src/a".into(), blob),
            (
                "docs/ir/a.md".into(),
                Blob {
                    mode: "100644".into(),
                    bytes: b"IR".to_vec(),
                },
            ),
        ]),
    };
    (s, entries)
}
fn reviewer(entries: &mut Vec<LocatedEntry>) {
    let mut e = entries[0].entry.clone();
    e.id = "review".into();
    e.role = change_records::Role::Reviewer;
    entries.push(LocatedEntry {
        index: 0,
        path: "docs/changes/review.yaml".into(),
        entry: e,
    });
}
fn has(result: &kotowari_core::changes::ChangeResult, kind: &str) -> bool {
    result.findings.iter().any(|f| f.kind.as_str() == kind)
}

// @kotowari[REQ-core-242, REQ-core-272, EX-core-430]
#[test]
fn an_unreported_file_is_uncovered() {
    let (mut s, e) = fixture();
    let mut file = s.files[0].clone();
    file.path = "src/b".into();
    s.files.push(file);
    let result = evaluate(&s, &e, Phase::Implementation);
    assert_eq!(result.files, 2);
    assert_eq!(result.covered, 1);
    assert!(has(&result, "change_uncovered"));
}
// @kotowari[REQ-core-243, REQ-core-272, REQ-core-273, EX-core-432, EX-core-447]
#[test]
fn one_stale_file_invalidates_all_files_in_its_entry() {
    let (mut s, mut e) = fixture();
    let mut file = s.files[0].clone();
    file.path = "src/b".into();
    s.files.push(file.clone());
    e[0].entry.files.push(file);
    e[0].entry.files[0].after = Some(format!("sha256:{}", "f".repeat(64)));
    let result = evaluate(&s, &e, Phase::Implementation);
    assert_eq!(result.covered, 0);
    assert!(has(&result, "change_stale"));
}
// @kotowari[REQ-core-244, REQ-core-272, EX-core-433]
#[test]
fn changed_ir_requires_reconciliation_even_when_code_matches() {
    let (mut s, e) = fixture();
    s.blobs.get_mut("docs/ir/a.md").unwrap().bytes = b"changed".to_vec();
    let result = evaluate(&s, &e, Phase::Implementation);
    assert_eq!(result.covered, 0);
    assert!(has(&result, "change_ir_stale"));
}
// @kotowari[REQ-core-272, EX-core-450]
#[test]
fn deferred_passes_implementation_and_fails_review() {
    let (s, mut e) = fixture();
    e[0].entry.conclusion = change_records::Conclusion::Deferred;
    reviewer(&mut e);
    assert_eq!(evaluate(&s, &e, Phase::Implementation).covered, 1);
    let result = evaluate(&s, &e, Phase::Review);
    assert_eq!(result.covered, 0);
    assert!(has(&result, "change_deferred"));
}
// @kotowari[REQ-core-272]
#[test]
fn reviewer_must_include_implementer_related_ir() {
    let (s, mut e) = fixture();
    reviewer(&mut e);
    e[1].entry.ir.clear();
    let result = evaluate(&s, &e, Phase::Review);
    assert_eq!(result.covered, 0);
    assert!(has(&result, "change_uncovered"));
}
// @kotowari[REQ-core-272]
#[test]
fn different_conclusions_for_matching_content_conflict() {
    let (s, mut e) = fixture();
    reviewer(&mut e);
    e[1].entry.conclusion = change_records::Conclusion::New;
    let result = evaluate(&s, &e, Phase::Review);
    assert_eq!(result.covered, 0);
    assert!(has(&result, "change_conclusion_conflict"));
}
// @kotowari[REQ-core-273, EX-core-451]
#[test]
fn unrelated_history_and_archived_entries_are_not_stale() {
    let (s, mut e) = fixture();
    e[0].entry.base = "f".repeat(40);
    e[0].entry.ir[0].sha256 = "old".into();
    let result = evaluate(&s, &e, Phase::Implementation);
    assert!(!has(&result, "change_ir_stale"));
    assert_eq!(result.covered, 0);
}
// @kotowari[REQ-core-242, REQ-core-272, REQ-core-274, EX-core-431, EX-core-452, EX-core-454]
#[test]
fn different_file_groupings_cover_the_same_changes() {
    let (mut s, mut e) = fixture();
    let mut file = s.files[0].clone();
    file.path = "src/b".into();
    s.files.push(file.clone());
    e[0].entry.files.push(file.clone());
    reviewer(&mut e);
    e[1].entry.files.truncate(1);
    let mut second = e[1].entry.clone();
    second.id = "second".into();
    second.files = vec![file];
    e.push(LocatedEntry {
        index: 0,
        path: "docs/changes/review.yaml".into(),
        entry: second,
    });
    let result = evaluate(&s, &e, Phase::Review);
    assert_eq!(result.files, 2);
    assert_eq!(result.covered, 2);
    assert!(result.findings.is_empty());
    let value = serde_json::to_value(result).unwrap();
    assert_eq!(value.as_object().unwrap().len(), 6);
}
