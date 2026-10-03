mod acquisition;
#[cfg(feature = "tokio")]
mod asynchronous;
#[cfg(feature = "tokio")]
pub use asynchronous::{AsyncOptions, AsyncProject};
mod change_records;
mod change_service;
mod git_snapshot;
#[cfg(test)]
mod git_snapshot_tests;
mod guides;
mod ir;
#[cfg(test)]
mod ir_tests;
mod sources;
mod surface;
mod test_files;

pub use git_snapshot::Target;
pub use kotowari_core::changes::Phase;
pub use kotowari_core::config::{ChangesConfig, Config};
pub use kotowari_core::ir::is_valid_id;
pub use kotowari_core::{
    CheckInputs, CheckReport, Comparison, Finding, FindingKind, InputError, Inspection, IrDocument,
    IrOptions, ParsedItem, QueryReport, ReadInputs, ReadList, ReadModel, SourceText, StatusReport,
    SurfaceAnalysis, TestAnalysis, Tool,
};
#[doc(hidden)]
pub mod presentation {
    pub use kotowari_core::{list, query, status};
}
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectOptions {
    pub start: PathBuf,
    pub config: Option<PathBuf>,
}
impl ProjectOptions {
    pub fn new(start: impl Into<PathBuf>) -> Self {
        Self {
            start: start.into(),
            config: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    InputMissing,
    InvalidInput,
    ConfigError,
    UnknownQuery,
    ReadFailure,
    GitFailure,
    InternalMapping,
    RuntimeUnavailable,
    TaskFailure,
}
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    detail: String,
}
impl Error {
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.detail)
    }
}
impl std::error::Error for Error {}
impl From<kotowari_core::StopReason> for Error {
    fn from(reason: kotowari_core::StopReason) -> Self {
        use kotowari_core::StopReason as S;
        let kind = match &reason {
            S::ArgumentError(_) | S::ResultsError(_) => ErrorKind::InvalidInput,
            S::ConfigError(_) => ErrorKind::ConfigError,
            S::UnreadableFile(_) | S::NonUtf8File(_) => ErrorKind::ReadFailure,
            S::MappingError(_) => ErrorKind::InternalMapping,
            S::GitError(_) => ErrorKind::GitFailure,
        };
        Self {
            kind,
            detail: reason.to_string(),
        }
    }
}
impl From<InputError> for Error {
    fn from(error: InputError) -> Self {
        let kind = match &error {
            InputError::InvalidInput(_) => ErrorKind::InvalidInput,
            InputError::InputMissing(_) => ErrorKind::InputMissing,
            InputError::UnknownQuery(_) => ErrorKind::UnknownQuery,
            InputError::ConfigError(_) => ErrorKind::ConfigError,
            _ => ErrorKind::InternalMapping,
        };
        Self {
            kind,
            detail: error.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MutantsOptions {
    pub tool: Tool,
    pub results: PathBuf,
}
#[derive(Debug, Clone)]
pub struct ChangesOptions {
    pub base: String,
    pub target: Target,
    pub phase: Phase,
}

pub struct PlanReport(kotowari_core::plan::PlanResult);
impl PlanReport {
    pub fn findings(&self) -> &[Finding] {
        &self.0.findings
    }
    pub fn counts(&self) -> &std::collections::BTreeMap<String, usize> {
        &self.0.counts
    }
    #[doc(hidden)]
    pub fn presentation(&self) -> &kotowari_core::plan::PlanResult {
        &self.0
    }
}
pub struct MutantsReport(kotowari_core::mutants::MutantsResult);
impl MutantsReport {
    pub fn findings(&self) -> &[Finding] {
        &self.0.findings
    }
    pub fn mutants(&self) -> &kotowari_core::mutants::MutantCounts {
        &self.0.mutants
    }
    #[doc(hidden)]
    pub fn presentation(&self) -> &kotowari_core::mutants::MutantsResult {
        &self.0
    }
}
pub struct ChangesReport(kotowari_core::changes::ChangeResult);
impl ChangesReport {
    pub fn findings(&self) -> &[Finding] {
        &self.0.findings
    }
    #[doc(hidden)]
    pub fn presentation(&self) -> &kotowari_core::changes::ChangeResult {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    options: ProjectOptions,
}
impl Project {
    pub fn new(options: ProjectOptions) -> Result<Self, Error> {
        if !options.start.is_absolute() {
            return Err(Error {
                kind: ErrorKind::InvalidInput,
                detail: "invalid input: project start must be absolute".into(),
            });
        }
        Ok(Self { options })
    }
    pub fn read(&self) -> Result<ReadModel, Error> {
        let loaded = acquisition::load_all(&self.options.start, self.options.config.as_deref())?;
        Ok(loaded.into_read())
    }
    pub fn inspect(&self) -> Result<Inspection, Error> {
        let (loaded, guides, surface) =
            acquisition::load_with_guides(&self.options.start, self.options.config.as_deref())?;
        Ok(loaded.into_inspection(guides, surface))
    }
    pub fn check(&self) -> Result<CheckReport, Error> {
        self.inspect().map(Inspection::into_check)
    }
    pub fn list(&self) -> Result<ReadList, Error> {
        Ok(self.read()?.list())
    }
    pub fn query(&self, id: &str) -> Result<QueryReport, Error> {
        Ok(self.read()?.query(id)?)
    }
    pub fn status(&self) -> Result<StatusReport, Error> {
        self.inspect().map(Inspection::into_status)
    }
    pub fn plan(&self, path: &Path) -> Result<PlanReport, Error> {
        Ok(PlanReport(acquisition::run_plan(
            &self.options.start,
            path,
        )?))
    }
    pub fn mutants(&self, options: &MutantsOptions) -> Result<MutantsReport, Error> {
        Ok(MutantsReport(acquisition::run_mutants(
            &self.options.start,
            self.options.config.as_deref(),
            options.tool,
            &options.results,
        )?))
    }
    pub fn changes(&self, options: &ChangesOptions) -> Result<ChangesReport, Error> {
        Ok(ChangesReport(change_service::run(
            &self.options.start,
            &options.base,
            options.target.clone(),
            options.phase,
            self.options.config.as_deref(),
        )?))
    }
}
