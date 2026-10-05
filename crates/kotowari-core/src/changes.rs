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
#[derive(Clone, serde::Serialize)]
pub struct ChangeResult {
    pub(crate) base: String,
    pub(crate) target: String,
    pub(crate) phase: String,
    pub(crate) files: usize,
    pub(crate) covered: usize,
    pub(crate) findings: Vec<Finding>,
}
impl ChangeResult {
    readonly!(copy files: usize, covered: usize);
    readonly!(borrow base: String, target: String, phase: String, findings: Vec<Finding>);
}

/// Parse change records, validate their references and evaluate one acquired comparison.
pub fn inspect(snapshot: &Comparison, phase: Phase) -> Result<ChangeResult, crate::StopReason> {
    use crate::{change_records, ir, sources};
    use std::collections::BTreeMap;
    let admitted_config = snapshot.config.clone().validated()?;
    let config = &admitted_config;
    let content = |path: &str| {
        std::str::from_utf8(&snapshot.blobs[path].bytes)
            .map_err(|_| crate::StopReason::NonUtf8File(path.into()))
    };
    let changes = config
        .changes
        .as_ref()
        .ok_or_else(|| crate::StopReason::ConfigError("changes configuration required".into()))?;
    let patterns = change_records::glob(&changes.records);
    let mut entries = vec![];
    let mut findings = vec![];
    for path in snapshot.blobs.keys() {
        if patterns.is_match(path) {
            let (parsed, errors) = change_records::parse(path, content(path)?);
            entries.extend(parsed);
            findings.extend(errors);
        }
    }
    let mut requirements = BTreeMap::new();
    let mut ir_paths = BTreeSet::new();
    let mut context = sources::SourceContext {
        records_path: config.decisions.records.clone(),
        adr_path: config.decisions.adr.clone(),
        records_files: vec![],
        adr_files: vec![],
        records_other_files: vec![],
    };
    for path in snapshot.blobs.keys().filter(|path| path.ends_with(".md")) {
        // REQ-core-342: `対`の`項目`は`先頭の言語`の`側`からだけ読む
        let name = path.rsplit('/').next().unwrap_or(path);
        let first_side = matches!(
            crate::translations::classify(name, &config.languages),
            crate::translations::Classified::First
        );
        if first_side && sources::is_under_place(path, &config.ir) {
            ir_paths.insert(path.clone());
            let doc = ir::parse_document(
                path.strip_prefix(&format!("{}/", config.ir))
                    .unwrap_or(path),
                content(path)?,
            )?;
            for item in doc.items() {
                if let ir::Item::Requirement { id, .. } = item {
                    requirements
                        .entry(id.clone())
                        .or_insert_with(|| path.clone());
                }
            }
        }
        if sources::is_under_place(path, &config.decisions.records) {
            let rel = path
                .strip_prefix(&format!("{}/", config.decisions.records))
                .unwrap_or(path);
            let record = sources::parse_records_file(rel, content(path)?);
            if record.is_records {
                context.records_files.push(record);
            } else {
                context.records_other_files.push(sources::OtherFile {
                    rel_path: rel.into(),
                    headings: record.headings,
                });
            }
        }
        if sources::is_under_place(path, &config.decisions.adr) {
            let rel = path
                .strip_prefix(&format!("{}/", config.decisions.adr))
                .unwrap_or(path);
            context
                .adr_files
                .push(sources::parse_other_file(rel, content(path)?));
        }
    }
    findings.extend(change_records::validate_references(
        &mut entries,
        &requirements,
        &ir_paths,
        &context,
    ));
    let valid_entries = entries
        .into_iter()
        .filter(|entry| !entry.invalid)
        .collect::<Vec<_>>();
    let mut result = evaluate(snapshot, &valid_entries, phase);
    result.findings.extend(findings);
    result.findings.sort_by(|a, b| {
        (a.path(), a.kind().as_str(), a.detail()).cmp(&(b.path(), b.kind().as_str(), b.detail()))
    });
    Ok(result)
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
                if let Some(actual) = snapshot.files.iter().find(|f| f.path == file.path)
                    && actual != file
                {
                    findings.push(finding(entry, FindingKind::ChangeStale, &file.path));
                    fresh = false;
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
