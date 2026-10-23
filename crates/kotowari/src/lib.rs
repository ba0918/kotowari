#![deny(clippy::print_stdout, clippy::print_stderr)]
mod acquisition;
#[cfg(feature = "tokio")]
mod asynchronous;
mod cache;
#[cfg(feature = "tokio")]
pub use asynchronous::{AsyncOptions, AsyncProject};
mod guides;
mod ir;
#[cfg(test)]
mod ir_tests;
mod overview;
mod sources;
mod surface;
mod test_files;
mod translations;

pub use kotowari_core::config::Config;
pub use kotowari_core::guides::{GuideReader, GuideTally};
pub use kotowari_core::ir::is_valid_id;
pub use kotowari_core::mutants::MutantCounts;
pub use kotowari_core::surface::{SurfaceTally, Unlisted};
pub use kotowari_core::translations::{Translation, TranslationSide};
pub use kotowari_core::{
    CheckReport, Finding, FindingGroup, FindingKind, GroupTally, InputError, Inspection,
    IrDocument, QueryReport, ReadList, ReadModel, StatusReport, TestFileTally,
};
pub use kotowari_core::{
    Documents, ExampleItem, Findings, FlagItem, Items, ListItem, QueryItem, Reference,
    RequirementItem, Requirements, ScenarioItem, Scenarios, TestRef, Tests,
};
/// check と status の結果の中の、全体像の元データの群の名前（REQ-core-288）
pub use kotowari_overview::GROUP as OVERVIEW_GROUP;
pub use kotowari_overview::Page;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ProjectOptions {
    pub start: PathBuf,
    /// 設定ファイルのパス。相対パスは開始位置から解く（REQ-core-323）
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
    InternalMapping,
    RuntimeUnavailable,
    TaskFailure,
}
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    detail: String,
    /// 種類が UnknownQuery のときだけ、問い合わせた知らない ID
    unknown_id: Option<String>,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
            unknown_id: None,
        }
    }
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
    /// 種類が UnknownQuery のときの知らない ID。ほかの種類では None（REQ-core-312）
    pub fn unknown_id(&self) -> Option<&str> {
        self.unknown_id.as_deref()
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
        Self::new(ErrorKind::CacheFailure, detail)
    }
    fn overview_data(errors: usize) -> Self {
        Self::new(
            ErrorKind::OverviewData,
            format!("{OVERVIEW_ERROR}: {errors} errors in overview data; run kotowari check"),
        )
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
        };
        Self::new(kind, reason.to_string())
    }
}
impl From<InputError> for Error {
    fn from(error: InputError) -> Self {
        let (kind, unknown_id) = match &error {
            InputError::InvalidInput(_) => (ErrorKind::InvalidInput, None),
            InputError::InputMissing(_) => (ErrorKind::InputMissing, None),
            InputError::UnknownQuery(id) => (ErrorKind::UnknownQuery, Some(id.clone())),
            InputError::ConfigError(_) => (ErrorKind::ConfigError, None),
            _ => (ErrorKind::InternalMapping, None),
        };
        Self {
            unknown_id,
            ..Self::new(kind, error.to_string())
        }
    }
}

/// 結果のファイルを作った変異テストの道具（REQ-core-149）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    CargoMutants,
}

#[derive(Debug, Clone)]
pub struct MutantsOptions {
    pub tool: Tool,
    pub results: PathBuf,
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
        self.base.join(cache::CACHE)
    }
    /// `基準のディレクトリ`の ".kotowari/cache/overview/" の下へ書く。同じ名前で同じバイト列の
    /// ファイルは書かず、今回返さなかったファイルを消す（REQ-core-293）
    pub fn write(&self) -> Result<OverviewBuild, Error> {
        cache::write(&self.base, &self.pages)
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
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid input: project start must be absolute",
            ));
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
        Ok(acquisition::load_list(
            &self.options.start,
            self.options.config.as_deref(),
        )?)
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
        let (base, overview, blockers) =
            acquisition::load_overview(&self.options.start, self.options.config.as_deref())?;
        let errors = overview.errors() + blockers;
        if errors > 0 {
            return Err(Error::overview_data(errors));
        }
        let pages = overview.pages().map_err(Error::overview_data)?;
        Ok(OverviewPrepared { base, pages })
    }
    /// overview_prepare に続けて ".kotowari/cache/overview/" の下へ書く（TBL-core-041）
    pub fn overview_build(&self) -> Result<OverviewBuild, Error> {
        self.overview_prepare()?.write()
    }
}
