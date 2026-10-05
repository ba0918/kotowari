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
mod overview;
mod sources;
mod surface;
mod test_files;
mod translations;

pub use git_snapshot::Target;
pub use kotowari_core::changes::Phase;
pub use kotowari_core::config::{ChangesConfig, Config};
pub use kotowari_core::guides::GuideTally;
pub use kotowari_core::ir::is_valid_id;
pub use kotowari_core::mutants::MutantCounts;
pub use kotowari_core::surface::{SurfaceTally, Unlisted};
pub use kotowari_core::{
    CheckInputs, CheckReport, Comparison, Finding, FindingGroup, FindingKind, GroupTally,
    InputError, Inspection, IrDocument, IrOptions, ParsedItem, QueryReport, ReadInputs, ReadList,
    ReadModel, SourceText, StatusReport, SurfaceAnalysis, TestAnalysis, TestFileTally, Tool,
};
pub use kotowari_core::{
    Documents, ExampleItem, Findings, FlagItem, Items, ListItem, QueryItem, Reference,
    RequirementItem, Requirements, ScenarioItem, Scenarios, TestRef, Tests,
};
pub use kotowari_overview::GROUP as OVERVIEW_GROUP;
/// check と status の結果の中の、全体像の元データの群の名前（REQ-core-288）
pub use kotowari_overview::Page;
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
    /// 全体像の元データに誤りがあり、全体像を書かない（REQ-core-294）
    OverviewData,
    /// 全体像の置き場がシンボリックリンクかディレクトリでないか、作成、書き込み、削除に失敗した（REQ-core-324）
    CacheFailure,
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
/// 全体像の元データの誤りで止まるときの、標準エラーの1行目で詳細の前に出る文言（TBL-core-018）
const OVERVIEW_ERROR: &str = "overview error";
/// 全体像の置き場が使えずに止まるときの文言（TBL-core-018）
const CACHE_ERROR: &str = "cache error";
impl Error {
    /// core の停止の理由に無い、このライブラリの失敗の文言のすべて
    pub const WORDINGS: &'static [&'static str] = &[OVERVIEW_ERROR, CACHE_ERROR];
    /// 置き場の誤り。詳細は`基準のディレクトリ`からの相対パスと、あれば OS の誤りの文（TBL-core-020）
    fn cache(path: &str, error: Option<std::io::Error>) -> Self {
        let detail = match error {
            Some(error) => format!("{CACHE_ERROR}: {path}: {error}"),
            None => format!("{CACHE_ERROR}: {path}"),
        };
        Self {
            kind: ErrorKind::CacheFailure,
            detail,
        }
    }
    fn overview_data(errors: usize) -> Self {
        Self {
            kind: ErrorKind::OverviewData,
            detail: format!(
                "{OVERVIEW_ERROR}: {errors} errors in overview data; run kotowari check"
            ),
        }
    }
}
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
        self.0.findings()
    }
    pub fn counts(&self) -> &std::collections::BTreeMap<String, usize> {
        self.0.counts()
    }
}
pub struct MutantsReport(kotowari_core::mutants::MutantsResult);
impl MutantsReport {
    pub fn findings(&self) -> &[Finding] {
        self.0.findings()
    }
    pub fn mutants(&self) -> &kotowari_core::mutants::MutantCounts {
        self.0.mutants()
    }
    pub fn counts(&self) -> &std::collections::BTreeMap<String, usize> {
        self.0.counts()
    }
}
#[derive(Clone)]
pub struct ChangesReport(kotowari_core::changes::ChangeResult);
impl ChangesReport {
    pub fn findings(&self) -> &[Finding] {
        self.0.findings()
    }
    pub fn base(&self) -> &str {
        self.0.base()
    }
    pub fn target(&self) -> &str {
        self.0.target()
    }
    pub fn phase(&self) -> &str {
        self.0.phase()
    }
    pub fn files(&self) -> usize {
        self.0.files()
    }
    pub fn covered(&self) -> usize {
        self.0.covered()
    }
}

/// 検査と描画を済ませ、まだ何も書いていない全体像（TBL-core-041 の overview_prepare）
#[derive(Debug, Clone)]
pub struct OverviewPrepared {
    base: PathBuf,
    pages: Vec<Page>,
}
impl OverviewPrepared {
    /// 描画のエンジンが返したページ
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }
    /// 書く先の置き場。`基準のディレクトリ`の ".kotowari/cache/overview/"（REQ-core-296）
    pub fn directory(&self) -> PathBuf {
        self.base.join(overview::CACHE)
    }
    /// `基準のディレクトリ`の ".kotowari/cache/overview/" の下へ書く。同じ名前で同じバイト列の
    /// ファイルは書かず、今回返さなかったファイルを消す（REQ-core-293）
    pub fn write(&self) -> Result<OverviewBuild, Error> {
        let cache = self.directory();
        let write_error =
            |path: &str, error| Error::cache(&format!("{}/{path}", overview::CACHE), Some(error));
        // REQ-core-324: 置き場かその上がリンクかディレクトリでなければ、何も書かず消さずに止める
        let mut place = String::new();
        for component in overview::CACHE.split('/') {
            if !place.is_empty() {
                place.push('/');
            }
            place.push_str(component);
            match std::fs::symlink_metadata(self.base.join(&place)) {
                Ok(meta) if meta.file_type().is_dir() => {}
                Ok(_) => return Err(Error::cache(&place, None)),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
                Err(error) => return Err(Error::cache(&place, Some(error))),
            }
        }
        std::fs::create_dir_all(&cache)
            .map_err(|error| Error::cache(overview::CACHE, Some(error)))?;
        let existing = overview::existing(&cache)?;
        let mut build = OverviewBuild::default();
        for page in &self.pages {
            let path = cache.join(&page.name);
            // シンボリックリンクを辿って置き場の外に書かないよう、リンクは消してからファイルを書く（REQ-core-296）
            let link =
                std::fs::symlink_metadata(&path).is_ok_and(|meta| meta.file_type().is_symlink());
            if link {
                std::fs::remove_file(&path).map_err(|error| write_error(&page.name, error))?;
            } else if std::fs::read(&path).is_ok_and(|bytes| bytes == page.content.as_bytes()) {
                build.unchanged += 1;
                continue;
            }
            std::fs::write(&path, &page.content).map_err(|error| write_error(&page.name, error))?;
            build
                .written
                .push(format!("{}/{}", overview::CACHE, page.name));
        }
        let names: std::collections::BTreeSet<&str> =
            self.pages.iter().map(|page| page.name.as_str()).collect();
        for name in existing {
            if names.contains(name.as_str()) {
                continue;
            }
            std::fs::remove_file(cache.join(&name)).map_err(|error| write_error(&name, error))?;
            build.removed.push(format!("{}/{name}", overview::CACHE));
        }
        build.written.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        build.removed.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        Ok(build)
    }
}

/// 全体像を書いた結果（REQ-core-295）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OverviewBuild {
    written: Vec<String>,
    removed: Vec<String>,
    unchanged: usize,
}
impl OverviewBuild {
    /// 書いたファイルの`基準のディレクトリ`からの相対パス。パスのバイト順
    pub fn written(&self) -> &[String] {
        &self.written
    }
    /// 消したファイルの`基準のディレクトリ`からの相対パス。パスのバイト順
    pub fn removed(&self) -> &[String] {
        &self.removed
    }
    /// 書かなかったファイルの数
    pub fn unchanged(&self) -> usize {
        self.unchanged
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
        Ok(acquisition::load_all(
            &self.options.start,
            self.options.config.as_deref(),
        )?)
    }
    pub fn inspect(&self) -> Result<Inspection, Error> {
        Ok(acquisition::load_with_guides(
            &self.options.start,
            self.options.config.as_deref(),
        )?)
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
    /// 全体像の元データを検査して描画し、何も書かない（TBL-core-041）。元データに誤りがあれば
    /// OverviewData の失敗を返す（REQ-core-294）
    pub fn overview_prepare(&self) -> Result<OverviewPrepared, Error> {
        let (base, overview) =
            acquisition::load_overview(&self.options.start, self.options.config.as_deref())?;
        let pages = overview.pages().map_err(Error::overview_data)?;
        Ok(OverviewPrepared { base, pages })
    }
    /// overview_prepare に続けて ".kotowari/cache/overview/" の下へ書く（TBL-core-041）
    pub fn overview_build(&self) -> Result<OverviewBuild, Error> {
        self.overview_prepare()?.write()
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
