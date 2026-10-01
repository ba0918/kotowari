use crate::{
    StopReason,
    change_records::{self, LocatedEntry},
    changes::{self, ChangeResult, Phase},
    config::Config,
    git_snapshot::{self, Blob, Target},
    ir::{self, Item},
    sources::{self, SourceContext},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
fn content<'a>(path: &str, blob: &'a Blob) -> Result<&'a str, StopReason> {
    std::str::from_utf8(&blob.bytes).map_err(|_| StopReason::NonUtf8File(path.into()))
}
fn reference_context(
    blobs: &BTreeMap<String, Blob>,
    config: &Config,
) -> Result<(BTreeMap<String, String>, BTreeSet<String>, SourceContext), StopReason> {
    let mut requirements = BTreeMap::new();
    let mut ir_paths = BTreeSet::new();
    let mut sources = SourceContext {
        records_path: config.decisions.records.clone(),
        adr_path: config.decisions.adr.clone(),
        records_files: vec![],
        adr_files: vec![],
        records_other_files: vec![],
    };
    for (path, blob) in blobs {
        if !path.ends_with(".md") {
            continue;
        }
        if sources::is_under_place(path, &config.ir) {
            ir_paths.insert(path.clone());
            let doc = ir::parse_document(
                path.strip_prefix(&format!("{}/", config.ir))
                    .unwrap_or(path),
                content(path, blob)?,
            )?;
            for item in doc.items {
                if let Item::Requirement { id, .. } = item {
                    requirements.entry(id).or_insert_with(|| path.clone());
                }
            }
        }
        if sources::is_under_place(path, &config.decisions.records) {
            let rel = path
                .strip_prefix(&format!("{}/", config.decisions.records))
                .unwrap_or(path);
            let record = sources::parse_records_file(rel, content(path, blob)?);
            if record.is_records {
                sources.records_files.push(record);
            } else {
                sources.records_other_files.push(sources::OtherFile {
                    rel_path: rel.into(),
                    headings: record.headings,
                });
            }
        }
        if sources::is_under_place(path, &config.decisions.adr) {
            let rel = path
                .strip_prefix(&format!("{}/", config.decisions.adr))
                .unwrap_or(path);
            sources
                .adr_files
                .push(sources::parse_other_file(rel, content(path, blob)?));
        }
    }
    Ok((requirements, ir_paths, sources))
}
pub fn run(
    cwd: &Path,
    base: &str,
    target: Target,
    phase: Phase,
    config_path: Option<&Path>,
) -> Result<ChangeResult, StopReason> {
    let snapshot = git_snapshot::read(cwd, base, target, config_path)?;
    let patterns = change_records::glob(&snapshot.config.changes.as_ref().unwrap().records);
    let mut entries: Vec<LocatedEntry> = vec![];
    let mut findings = vec![];
    for (path, blob) in &snapshot.blobs {
        if patterns.is_match(path) {
            let (parsed, errors) = change_records::parse(path, content(path, blob)?);
            entries.extend(parsed);
            findings.extend(errors);
        }
    }
    let (requirements, ir_paths, sources) = reference_context(&snapshot.blobs, &snapshot.config)?;
    let reference_errors =
        change_records::validate_references(&entries, &requirements, &ir_paths, &sources);
    let valid_entries: Vec<_> = entries
        .into_iter()
        .filter(|entry| {
            !findings.iter().chain(&reference_errors).any(|f| {
                f.path == entry.path
                    && (f.detail.starts_with("file: ")
                        || f.detail.contains(&format!(": {}:", entry.entry.id)))
            })
        })
        .collect();
    findings.extend(reference_errors);
    let mut result = changes::evaluate(&snapshot, &valid_entries, phase);
    result.findings.extend(findings);
    result.findings.sort_by(|a, b| {
        (&a.path, a.kind.as_str(), &a.detail).cmp(&(&b.path, b.kind.as_str(), &b.detail))
    });
    Ok(result)
}
