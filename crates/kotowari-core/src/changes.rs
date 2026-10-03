use crate::{
    Finding, FindingKind,
    change_records::{Conclusion, LocatedEntry, Role},
    comparison::{Comparison, ir_identity},
};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Implementation,
    Review,
}
impl Phase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Implementation => "implementation",
            Self::Review => "review",
        }
    }
    pub fn parse(value: &str) -> Result<Self, crate::StopReason> {
        match value {
            "implementation" => Ok(Self::Implementation),
            "review" => Ok(Self::Review),
            _ => Err(crate::StopReason::ArgumentError(format!(
                "unknown phase: {value}"
            ))),
        }
    }
}
#[derive(serde::Serialize)]
pub struct ChangeResult {
    pub base: String,
    pub target: String,
    pub phase: String,
    pub files: usize,
    pub covered: usize,
    pub findings: Vec<Finding>,
}
fn finding(entry: &LocatedEntry, kind: FindingKind, path: &str) -> Finding {
    Finding::new(
        kind,
        entry.path.clone(),
        None,
        format!("{}: {path}", entry.entry.id),
    )
}
pub fn evaluate(snapshot: &Comparison, entries: &[LocatedEntry], phase: Phase) -> ChangeResult {
    let applicable: Vec<_> = entries
        .iter()
        .filter(|e| {
            e.entry.base == snapshot.base
                && e.entry
                    .files
                    .iter()
                    .any(|f| snapshot.files.iter().any(|s| s.path == f.path))
        })
        .collect();
    let mut findings = vec![];
    let fresh: Vec<_> = applicable
        .iter()
        .map(|entry| {
            let mut fresh = true;
            for file in &entry.entry.files {
                if let Some(actual) = snapshot.files.iter().find(|f| f.path == file.path) {
                    if actual != file {
                        findings.push(finding(entry, FindingKind::ChangeStale, &file.path));
                        fresh = false;
                    }
                }
            }
            for ir in &entry.entry.ir {
                if snapshot
                    .blobs
                    .get(&ir.path)
                    .map(|b| ir_identity(&b.bytes))
                    .as_ref()
                    != Some(&ir.sha256)
                {
                    findings.push(finding(entry, FindingKind::ChangeIrStale, &ir.path));
                    fresh = false;
                }
            }
            fresh
        })
        .collect();
    let mut covered = 0;
    for file in &snapshot.files {
        let matching: Vec<_> = applicable
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.entry.files.iter().any(|f| f == file))
            .collect();
        let conclusions: BTreeSet<_> = matching
            .iter()
            .map(|(_, e)| match e.entry.conclusion {
                Conclusion::Existing => "existing",
                Conclusion::New => "new",
                Conclusion::Deferred => "deferred",
            })
            .collect();
        let conflict = conclusions.len() > 1;
        if conflict {
            for (_, e) in &matching {
                findings.push(finding(
                    e,
                    FindingKind::ChangeConclusionConflict,
                    &file.path,
                ));
            }
        }
        let deferred = phase == Phase::Review
            && matching
                .iter()
                .any(|(_, e)| e.entry.conclusion == Conclusion::Deferred);
        if deferred {
            for (_, e) in &matching {
                if e.entry.conclusion == Conclusion::Deferred {
                    findings.push(finding(e, FindingKind::ChangeDeferred, &file.path));
                }
            }
        }
        let implementers: Vec<_> = matching
            .iter()
            .filter(|(i, e)| fresh[*i] && e.entry.role == Role::Implementer)
            .collect();
        let required_ir: BTreeSet<_> = implementers
            .iter()
            .flat_map(|(_, e)| e.entry.ir.iter().map(|ir| &ir.path))
            .collect();
        let reviewer = matching.iter().any(|(i, e)| {
            fresh[*i]
                && e.entry.role == Role::Reviewer
                && required_ir
                    .iter()
                    .all(|path| e.entry.ir.iter().any(|ir| &ir.path == *path))
        });
        let has_impl = !implementers.is_empty();
        for (needed, present) in [
            (Role::Implementer, has_impl),
            (Role::Reviewer, phase != Phase::Review || reviewer),
        ] {
            if !present {
                findings.push(Finding::new(
                    FindingKind::ChangeUncovered,
                    file.path.clone(),
                    None,
                    needed.as_str().into(),
                ));
            }
        }
        if has_impl && (phase != Phase::Review || reviewer) && !conflict && !deferred {
            covered += 1;
        }
    }
    findings.sort_by(|a, b| {
        (&a.path, a.kind.as_str(), &a.detail).cmp(&(&b.path, b.kind.as_str(), &b.detail))
    });
    ChangeResult {
        base: snapshot.base.clone(),
        target: snapshot.target.clone(),
        phase: phase.as_str().into(),
        files: snapshot.files.len(),
        covered,
        findings,
    }
}
