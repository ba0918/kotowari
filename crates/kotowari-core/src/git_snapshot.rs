pub use crate::comparison::{Blob, Comparison as Snapshot, ir_identity};
use crate::{
    StopReason,
    change_records::{self, FileChange},
    config::Config,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

#[derive(Debug, Clone)]
pub enum Target {
    Index,
    Commit(String),
}
fn error(message: impl Into<String>) -> StopReason {
    StopReason::GitError(message.into())
}
fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>, StopReason> {
    let out = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|e| error(e.to_string()))?;
    if !out.status.success() {
        return Err(error(
            String::from_utf8_lossy(&out.stderr).trim().to_string(),
        ));
    }
    Ok(out.stdout)
}
fn text(bytes: Vec<u8>) -> Result<String, StopReason> {
    String::from_utf8(bytes)
        .map(|s| s.trim().to_string())
        .map_err(|_| error("non-UTF-8 Git response"))
}
fn resolve(root: &Path, revision: &str) -> Result<String, StopReason> {
    text(git(
        root,
        &[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{revision}^{{commit}}"),
        ],
    )?)
}
#[derive(Debug)]
struct TreeEntry {
    path: Vec<u8>,
    mode: String,
    object: String,
}
fn tree(root: &Path, revision: Option<&str>) -> Result<Vec<TreeEntry>, StopReason> {
    let output = match revision {
        Some(rev) => git(root, &["ls-tree", "-r", "-z", "--full-tree", rev])?,
        None => git(root, &["ls-files", "--stage", "-z"])?,
    };
    output
        .split(|b| *b == 0)
        .filter(|r| !r.is_empty())
        .map(|record| {
            let tab = record
                .iter()
                .position(|b| *b == b'\t')
                .ok_or_else(|| error("invalid Git tree entry"))?;
            let meta =
                std::str::from_utf8(&record[..tab]).map_err(|_| error("invalid tree metadata"))?;
            let fields: Vec<_> = meta.split(' ').collect();
            if fields.len() != 3 {
                return Err(error("invalid Git tree metadata"));
            }
            if revision.is_none() && fields[2] != "0" {
                return Err(error("conflicted index"));
            }
            let object = if revision.is_some() {
                fields[2]
            } else {
                fields[1]
            };
            Ok(TreeEntry {
                path: record[tab + 1..].to_vec(),
                mode: fields[0].into(),
                object: object.into(),
            })
        })
        .collect()
}
fn blobs(root: &Path, entries: &[&TreeEntry]) -> Result<BTreeMap<String, Vec<u8>>, StopReason> {
    let objects: BTreeSet<_> = entries
        .iter()
        .filter(|e| e.mode != "160000")
        .map(|e| &e.object)
        .collect();
    if objects.is_empty() {
        return Ok(BTreeMap::new());
    }
    let input = objects.iter().map(|s| format!("{s}\n")).collect::<String>();
    let mut child = Command::new("git")
        .current_dir(root)
        .args(["cat-file", "--batch"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| error(e.to_string()))?;
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()));
    let out = child.wait_with_output().map_err(|e| error(e.to_string()))?;
    writer
        .join()
        .map_err(|_| error("Git input writer failed"))?
        .map_err(|e| error(e.to_string()))?;
    if !out.status.success() {
        return Err(error(String::from_utf8_lossy(&out.stderr).to_string()));
    }
    let mut remaining = out.stdout.as_slice();
    let mut result = BTreeMap::new();
    for expected in objects {
        let newline = remaining
            .iter()
            .position(|b| *b == b'\n')
            .ok_or_else(|| error("incomplete blob header"))?;
        let header =
            std::str::from_utf8(&remaining[..newline]).map_err(|_| error("invalid blob header"))?;
        let fields: Vec<_> = header.split(' ').collect();
        if fields.len() != 3 || fields[0] != expected || fields[1] != "blob" {
            return Err(error(format!("unreadable blob {expected}")));
        }
        let size: usize = fields[2].parse().map_err(|_| error("invalid blob size"))?;
        remaining = &remaining[newline + 1..];
        if remaining.len() <= size || remaining[size] != b'\n' {
            return Err(error("incomplete blob"));
        }
        result.insert(expected.clone(), remaining[..size].to_vec());
        remaining = &remaining[size + 1..];
    }
    Ok(result)
}
fn snapshot_blobs(
    entries: &[&TreeEntry],
    bytes: &BTreeMap<String, Vec<u8>>,
) -> BTreeMap<String, Blob> {
    entries
        .iter()
        .filter_map(|e| {
            std::str::from_utf8(&e.path).ok().map(|p| {
                (
                    p.into(),
                    Blob {
                        mode: e.mode.clone(),
                        bytes: bytes.get(&e.object).cloned().unwrap_or_default(),
                    },
                )
            })
        })
        .collect()
}
#[cfg(unix)]
fn byte_path(bytes: &[u8]) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;
    PathBuf::from(std::ffi::OsStr::from_bytes(bytes))
}
#[cfg(not(unix))]
fn byte_path(bytes: &[u8]) -> PathBuf {
    PathBuf::from(String::from_utf8_lossy(bytes).as_ref())
}

pub fn read(
    cwd: &Path,
    base: &str,
    target: Target,
    config_path: Option<&Path>,
) -> Result<Snapshot, StopReason> {
    let root_response = String::from_utf8(git(cwd, &["rev-parse", "--show-toplevel"])?)
        .map_err(|_| error("non-UTF-8 Git response"))?;
    // Whitespace is part of a valid path; only Git's output newline is removed.
    let root = PathBuf::from(root_response.strip_suffix('\n').unwrap_or(&root_response));
    let base = resolve(&root, base)?;
    let target_oid = match &target {
        Target::Index => "index".into(),
        Target::Commit(rev) => resolve(&root, rev)?,
    };
    let old = tree(&root, Some(&base))?;
    let new = tree(
        &root,
        match &target {
            Target::Index => None,
            Target::Commit(_) => Some(&target_oid),
        },
    )?;
    let config_path = config_path.unwrap_or(Path::new(".kotowari/config.yaml"));
    let raw_config = config_path
        .to_str()
        .filter(|p| !p.starts_with('/'))
        .ok_or_else(|| StopReason::ConfigError("config must be Git-root-relative".into()))?;
    let normalized_config = crate::normalize_path(raw_config);
    let config_path = normalized_config.as_str();
    if !change_records::normalized_relative(config_path) {
        return Err(StopReason::ConfigError(
            "config must stay inside Git root".into(),
        ));
    }
    let config_entry: Vec<&TreeEntry> = new
        .iter()
        .filter(|e| e.path == config_path.as_bytes())
        .collect();
    let config_bytes = blobs(&root, &config_entry)?;
    let config_blob = snapshot_blobs(&config_entry, &config_bytes)
        .remove(config_path)
        .ok_or_else(|| error(format!("unreadable target configuration {config_path}")))?;
    let content = std::str::from_utf8(&config_blob.bytes)
        .map_err(|_| StopReason::NonUtf8File(config_path.into()))?;
    let config = Config::parse(content)?;
    let changes = config
        .changes
        .as_ref()
        .ok_or_else(|| StopReason::ConfigError("changes configuration is required".into()))?;
    let included = change_records::glob(&changes.files);
    let excluded = change_records::glob(&changes.exclude);
    let records = change_records::glob(&changes.records);
    let places = [&config.ir, &config.decisions.records, &config.decisions.adr];
    let selected = |path: &Path| {
        included.is_match(path)
            && !excluded.is_match(path)
            && !records.is_match(path)
            && path != Path::new(config_path)
            && !places.iter().any(|place| path.starts_with(place))
    };
    let old_objects: BTreeMap<_, _> = old
        .iter()
        .map(|e| (&e.path, (&e.mode, &e.object)))
        .collect();
    let new_objects: BTreeMap<_, _> = new
        .iter()
        .map(|e| (&e.path, (&e.mode, &e.object)))
        .collect();
    let changed = |e: &TreeEntry| {
        old_objects.get(&e.path) != new_objects.get(&e.path) && selected(&byte_path(&e.path))
    };
    for e in old.iter().chain(&new) {
        if changed(e)
            && (std::str::from_utf8(&e.path).is_err()
                || !["100644", "100755"].contains(&e.mode.as_str()))
        {
            return Err(error("unsupported selected path or Git mode"));
        }
    }
    // Only changed files and the documents changes reads are loaded, so unrelated
    // large blobs never enter memory.
    let referenced = |e: &TreeEntry| {
        let path = byte_path(&e.path);
        records.is_match(&path) || places.iter().any(|place| path.starts_with(place))
    };
    let old_needed: Vec<&TreeEntry> = old.iter().filter(|e| changed(e)).collect();
    let new_needed: Vec<&TreeEntry> = new
        .iter()
        .filter(|e| changed(e) || referenced(e) || e.path == config_path.as_bytes())
        .collect();
    let bytes = blobs(
        &root,
        &[old_needed.as_slice(), new_needed.as_slice()].concat(),
    )?;
    let before = snapshot_blobs(&old_needed, &bytes);
    let after = snapshot_blobs(&new_needed, &bytes);
    let paths: BTreeSet<_> = before.keys().chain(after.keys()).collect();
    let files = paths
        .into_iter()
        .filter(|path| selected(Path::new(path)))
        .filter_map(|path| {
            let before = before.get(path).map(Blob::identity);
            let after = after.get(path).map(Blob::identity);
            (before != after).then(|| FileChange {
                path: path.clone(),
                before,
                after,
            })
        })
        .collect();
    Ok(Snapshot {
        base,
        target: target_oid,
        config,
        files,
        blobs: after,
    })
}
