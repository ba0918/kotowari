use crate::{Finding, FindingKind, config::Config, ir, sources::SourceContext};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordFile {
    pub version: u64,
    pub entries: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub id: String,
    pub base: String,
    pub role: Role,
    pub files: Vec<FileChange>,
    pub ir: Vec<IrIdentity>,
    pub conclusion: Conclusion,
    pub reason: String,
    pub requirements: Vec<String>,
    pub decisions: Vec<String>,
    #[serde(deserialize_with = "required_nullable")]
    pub handoff: Option<String>,
    pub gaps: Vec<Gap>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Implementer,
    Reviewer,
}
impl Role {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Implementer => "implementer",
            Self::Reviewer => "reviewer",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Conclusion {
    Existing,
    New,
    Deferred,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileChange {
    pub path: String,
    #[serde(deserialize_with = "required_nullable")]
    pub before: Option<String>,
    #[serde(deserialize_with = "required_nullable")]
    pub after: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IrIdentity {
    pub path: String,
    pub sha256: String,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub category: Category,
    pub disposition: Disposition,
    pub refs: Vec<String>,
}
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    MissingSpec,
    SpecConflict,
    PremiseConflict,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    Recorded,
    Fixed,
    Deferred,
}

fn required_nullable<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Option::<String>::deserialize(d)
}
pub fn normalized_relative(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && path
            .split('/')
            .all(|p| !p.is_empty() && p != "." && p != "..")
}
fn hex(s: &str, lengths: &[usize]) -> bool {
    lengths.contains(&s.len())
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn identity(s: &str) -> bool {
    s.strip_prefix("sha256:").is_some_and(|s| hex(s, &[64]))
}
fn invalid(path: &str, detail: String) -> Finding {
    Finding::new(FindingKind::ChangeRecordInvalid, path.into(), None, detail)
}

pub struct LocatedEntry {
    pub index: usize,
    pub path: String,
    pub entry: Entry,
}

pub fn parse(path: &str, content: &str) -> (Vec<LocatedEntry>, Vec<Finding>) {
    let value: serde_json::Value = match serde_saphyr::from_str(content) {
        Ok(raw) => raw,
        Err(error) => return (vec![], vec![invalid(path, format!("file: {error}"))]),
    };
    let raw: RecordFile = match serde_json::from_value(value) {
        Ok(raw) => raw,
        Err(error) => return (vec![], vec![invalid(path, format!("file: {error}"))]),
    };
    if raw.version != 1 {
        return (
            vec![],
            vec![invalid(
                path,
                format!("file: unsupported version {}", raw.version),
            )],
        );
    }
    let mut findings = vec![];
    let mut entries = vec![];
    for (index, value) in raw.entries.into_iter().enumerate() {
        let id = value
            .get("id")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        let entry: Entry = match serde_json::from_value(value.clone()) {
            Ok(entry) => entry,
            Err(error) => {
                findings.push(invalid(path, format!("entry {index}: {id}: {error}")));
                continue;
            }
        };
        let mut errors = vec![];
        let id_ok = entry
            .id
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
            && entry
                .id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b));
        if !id_ok {
            errors.push("invalid id");
        }
        if !hex(&entry.base, &[40, 64]) {
            errors.push("invalid base");
        }
        if entry.reason.trim().is_empty() {
            errors.push("empty reason");
        }
        if entry.files.is_empty() {
            errors.push("empty files");
        }
        let mut paths = BTreeSet::new();
        for file in &entry.files {
            if !normalized_relative(&file.path) || !paths.insert(&file.path) {
                errors.push("invalid or duplicate file path");
            }
            if file.before.is_none() && file.after.is_none() {
                errors.push("both identities null");
            }
            if [&file.before, &file.after]
                .into_iter()
                .flatten()
                .any(|s| !identity(s))
            {
                errors.push("invalid file identity");
            }
        }
        let mut paths = BTreeSet::new();
        for ir in &entry.ir {
            if !normalized_relative(&ir.path) || !paths.insert(&ir.path) {
                errors.push("invalid or duplicate IR path");
            }
            if !identity(&ir.sha256) {
                errors.push("invalid IR identity");
            }
        }
        if entry.conclusion == Conclusion::Existing
            || entry
                .gaps
                .iter()
                .any(|g| g.disposition == Disposition::Fixed)
        {
            if entry.requirements.is_empty() || entry.ir.is_empty() {
                errors.push("requirements and definition IR required");
            }
        }
        if entry.conclusion != Conclusion::Existing && entry.decisions.is_empty() {
            errors.push("decision required");
        }
        if entry.conclusion == Conclusion::Deferred && entry.handoff.is_none() {
            errors.push("handoff required");
        }
        for reference in entry
            .decisions
            .iter()
            .chain(entry.handoff.iter())
            .chain(entry.gaps.iter().flat_map(|g| &g.refs))
        {
            if !crate::sources::split_source(reference).is_some_and(|(path, anchor)| {
                normalized_relative(path) && crate::sources::is_decision_number(anchor)
            }) {
                errors.push("invalid decision reference shape");
            }
        }
        for gap in &entry.gaps {
            if gap.refs.is_empty() {
                errors.push("empty gap refs");
            }
        }
        if !entry.gaps.is_empty() {
            let expected = if entry
                .gaps
                .iter()
                .any(|g| g.disposition == Disposition::Deferred)
            {
                Conclusion::Deferred
            } else if entry
                .gaps
                .iter()
                .any(|g| g.disposition == Disposition::Recorded)
            {
                Conclusion::New
            } else {
                Conclusion::Existing
            };
            if entry.conclusion != expected {
                errors.push("gap conclusion mismatch");
            }
        }
        for error in errors {
            findings.push(invalid(
                path,
                format!("entry {index}: {}: {error}", entry.id),
            ));
        }
        entries.push(LocatedEntry {
            index,
            path: path.into(),
            entry,
        });
    }
    (entries, findings)
}

pub fn validate_references(
    entries: &[LocatedEntry],
    requirements: &BTreeMap<String, String>,
    ir_paths: &BTreeSet<String>,
    sources: &SourceContext,
) -> Vec<Finding> {
    let mut findings = vec![];
    let mut ids = BTreeSet::new();
    for located in entries {
        let index = located.index;
        let e = &located.entry;
        let mut errors: Vec<String> = vec![];
        if !ids.insert(&e.id) {
            errors.push("duplicate id".into());
        }
        for ir in &e.ir {
            if !ir_paths.contains(&ir.path) {
                errors.push(format!("missing IR {}", ir.path));
            }
        }
        for req in &e.requirements {
            match requirements.get(req) {
                None => errors.push(format!("missing requirement {req}")),
                Some(path) if !e.ir.iter().any(|ir| &ir.path == path) => {
                    errors.push(format!("missing definition IR {path}"))
                }
                Some(_) => {}
            }
        }
        if e.conclusion == Conclusion::Existing
            || e.gaps.iter().any(|g| g.disposition == Disposition::Fixed)
        {
            if e.requirements.is_empty() || e.ir.is_empty() {
                errors.push("requirements and definition IR required".into());
            }
        }
        if e.conclusion != Conclusion::Existing && e.decisions.is_empty() {
            errors.push("decision required".into());
        }
        if e.conclusion == Conclusion::Deferred && e.handoff.is_none() {
            errors.push("handoff required".into());
        }
        for reference in e
            .decisions
            .iter()
            .chain(e.handoff.iter())
            .chain(e.gaps.iter().flat_map(|g| &g.refs))
        {
            let valid = crate::sources::split_source(reference).is_some_and(|(path, anchor)| {
                normalized_relative(path) && crate::sources::is_decision_number(anchor)
            });
            if !valid || sources.check_source(reference).is_err() {
                errors.push(format!("invalid decision {reference}"));
            }
        }
        for error in errors {
            findings.push(invalid(
                &located.path,
                format!("entry {index}: {}: {error}", e.id),
            ));
        }
    }
    findings
}

pub fn glob(patterns: &[String]) -> globset::GlobSet {
    let mut builder = globset::GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(globset::Glob::new(pattern).expect("validated config glob"));
    }
    builder.build().expect("validated globs")
}

fn collect_records(
    base: &std::path::Path,
    patterns: &[String],
) -> Result<Vec<(String, String)>, crate::StopReason> {
    let records = glob(patterns);
    let hidden_prefixes: Vec<String> = patterns
        .iter()
        .flat_map(|pattern| {
            let components: Vec<_> = pattern.split('/').collect();
            components
                .iter()
                .enumerate()
                .filter(|(_, component)| component.starts_with('.') && **component != ".")
                .map(|(index, _)| components[..=index].join("/"))
                .collect::<Vec<_>>()
        })
        .collect();
    let named_hidden = glob(&hidden_prefixes);
    let relative = |path: &std::path::Path| {
        path.strip_prefix(base)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    };
    let mut files = vec![];
    for entry in walkdir::WalkDir::new(base)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            if entry.file_type().is_symlink() {
                return !std::fs::metadata(entry.path()).is_ok_and(|meta| meta.is_dir());
            }
            !entry.file_type().is_dir()
                || !entry.file_name().to_string_lossy().starts_with('.')
                || named_hidden.is_match(relative(entry.path()))
        })
    {
        let entry = entry.map_err(|error| {
            let path = error.path().map(relative).unwrap_or_default();
            crate::StopReason::UnreadableFile(format!("{path}: {error}"))
        })?;
        let path = relative(entry.path());
        let is_file = if entry.file_type().is_symlink() {
            std::fs::metadata(entry.path())
                .map_err(|error| crate::StopReason::UnreadableFile(format!("{path}: {error}")))?
                .is_file()
        } else {
            entry.file_type().is_file()
        };
        if is_file && records.is_match(&path) {
            files.push((path, entry.path().to_string_lossy().into_owned()));
        }
    }
    files.sort();
    Ok(files)
}

pub fn static_check(
    base: &std::path::Path,
    cfg: &Config,
    docs: &[ir::IrDocument],
    findings: &mut Vec<Finding>,
) -> Result<(), crate::StopReason> {
    let Some(changes) = &cfg.changes else {
        return Ok(());
    };
    let paths = collect_records(base, &changes.records)?;
    let mut entries = vec![];
    for (path, absolute) in paths {
        let content = crate::read_utf8_file(std::path::Path::new(&absolute), &path)?;
        let (parsed, errors) = parse(&path, &content);
        entries.extend(parsed);
        findings.extend(errors);
    }
    let requirements = docs
        .iter()
        .flat_map(|d| {
            d.items.iter().filter_map(|i| match i {
                ir::Item::Requirement { id, .. } => Some((
                    id.clone(),
                    crate::join_display_path(&cfg.ir, &d.relative_path),
                )),
                _ => None,
            })
        })
        .collect();
    let ir_paths = docs
        .iter()
        .map(|d| crate::join_display_path(&cfg.ir, &d.relative_path))
        .collect();
    findings.extend(validate_references(
        &entries,
        &requirements,
        &ir_paths,
        &crate::sources::build_context(base, cfg)?,
    ));
    Ok(())
}
