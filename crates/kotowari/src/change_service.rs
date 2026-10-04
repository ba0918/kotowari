use crate::git_snapshot::{self, Target};
use kotowari_core::{
    StopReason,
    changes::{self, ChangeResult, Phase},
};
use std::path::Path;

pub fn run(
    cwd: &Path,
    base: &str,
    target: Target,
    phase: Phase,
    config_path: Option<&Path>,
) -> Result<ChangeResult, StopReason> {
    let snapshot = git_snapshot::read(cwd, base, target, config_path)?;
    changes::inspect(&snapshot, phase)
}
