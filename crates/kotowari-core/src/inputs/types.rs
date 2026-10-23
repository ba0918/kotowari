//! 取得した入力を core に渡す公開の型（TBL-core-042）

use super::*;

#[derive(Debug, Clone)]
pub struct SourceText {
    pub(super) path: String,
    pub(super) text: Arc<str>,
}

impl SourceText {
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Result<Self, InputError> {
        let path = path.into().replace('\\', "/");
        if path.starts_with('/') || std::path::Path::new(&path).is_absolute() {
            return Err(InputError::InvalidInput(
                "logical path must be relative".into(),
            ));
        }
        let path = crate::normalize_path(&path);
        if path.is_empty() {
            return Err(InputError::InvalidInput(
                "logical path must not be empty".into(),
            ));
        }
        Ok(Self {
            path,
            text: Arc::from(text.into()),
        })
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Lossless enumeration identity is not the lossy display/rule path.
/// Text is what the producer actually acquired, including any legacy read-path conversion.
/// IR display paths are relative to the configured IR place; other paths are project-relative.
#[derive(Debug, Clone)]
pub struct NativeSourceText {
    pub(super) identity: PathBuf,
    pub(super) path: String,
    pub(super) text: Arc<str>,
}

impl NativeSourceText {
    pub fn new(
        identity: impl Into<PathBuf>,
        path: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            identity: identity.into(),
            path: path.into(),
            text: Arc::from(text.into()),
        }
    }
    pub fn identity(&self) -> &std::path::Path {
        &self.identity
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum InputError {
    InvalidInput(String),
    InputMissing(String),
    UnknownQuery(String),
    ConfigError(String),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(detail) => write!(f, "invalid input: {detail}"),
            Self::InputMissing(group) => write!(f, "input missing: {group}"),
            Self::UnknownQuery(id) => write!(f, "unknown query: {id}"),
            Self::ConfigError(detail) => write!(f, "config error: {detail}"),
        }
    }
}

impl std::error::Error for InputError {}

pub(super) fn native_error(error: &InputError) -> crate::StopReason {
    crate::StopReason::ConfigError(error.to_string())
}

pub(super) fn input_stop(error: crate::StopReason) -> InputError {
    match error {
        crate::StopReason::ConfigError(detail) => InputError::ConfigError(detail),
        other @ (crate::StopReason::ArgumentError(_)
        | crate::StopReason::UnreadableFile(_)
        | crate::StopReason::NonUtf8File(_)
        | crate::StopReason::ResultsError(_)
        | crate::StopReason::MappingError(_)) => InputError::InvalidInput(other.to_string()),
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReadInputs {
    pub config: crate::config::Config,
    pub ir: Option<Vec<SourceText>>,
    pub records: Option<Vec<SourceText>>,
    pub adr: Option<Vec<SourceText>>,
    pub tests: Option<Vec<TestAnalysis>>,
}

#[derive(Debug, Clone)]
pub struct TestAnalysis {
    pub source: SourceText,
    pub has_query: bool,
    pub tests: Vec<crate::tests_discovery::DiscoveredTest>,
    pub line_markers: Vec<crate::tests_discovery::Marker>,
    pub findings: Vec<crate::Finding>,
}

#[derive(Debug, Clone)]
pub struct SurfaceAnalysis {
    pub source: SourceText,
    pub has_query: bool,
    pub surfaces: Vec<crate::surface::Surface>,
    pub findings: Vec<crate::Finding>,
}

#[derive(Debug, Clone)]
pub struct NativeTestAnalysis {
    pub source: NativeSourceText,
    pub analysis: TestAnalysis,
}

#[derive(Debug, Clone)]
pub struct NativeSurfaceAnalysis {
    pub source: NativeSourceText,
    pub analysis: SurfaceAnalysis,
}

#[derive(Debug, Clone, Default)]
pub struct CheckInputs {
    pub read: ReadInputs,
    pub guides: Option<Vec<SourceText>>,
    pub surface: Option<Vec<SurfaceAnalysis>>,
    pub unspecified: Option<Vec<SourceText>>,
    /// 追加の指摘の群。渡したものだけを加える（TBL-core-042）
    pub groups: Vec<FindingGroup>,
    /// `対`。`言語の一覧`の言語が2つ以上のときだけ読む（REQ-core-334、REQ-core-336）
    pub translations: crate::translations::Pairs,
}

/// 名前を付けた追加の指摘の群（TBL-core-042）。core は群の意味を知らず、指摘をほかの指摘と
/// 合わせて並べて数え、名前と数を結果に持たせるだけである
#[derive(Debug, Clone)]
pub struct FindingGroup {
    pub(super) tally: GroupTally,
    pub(super) findings: Vec<crate::Finding>,
}

impl FindingGroup {
    pub fn new(
        name: impl Into<String>,
        files: usize,
        marks: usize,
        findings: Vec<crate::Finding>,
    ) -> Self {
        Self {
            tally: GroupTally {
                name: name.into(),
                files,
                marks,
            },
            findings,
        }
    }
    pub fn tally(&self) -> &GroupTally {
        &self.tally
    }
    pub fn findings(&self) -> &[crate::Finding] {
        &self.findings
    }
}

/// 追加の指摘の群の名前と、読んだファイルの数と印の数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupTally {
    pub(super) name: String,
    pub(super) files: usize,
    pub(super) marks: usize,
}

impl GroupTally {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn files(&self) -> usize {
        self.files
    }
    pub fn marks(&self) -> usize {
        self.marks
    }
}

pub(super) fn find_group<'a>(groups: &'a [GroupTally], name: &str) -> Option<&'a GroupTally> {
    groups.iter().find(|group| group.name == name)
}

pub struct RepositoryReadInputs {
    pub config: crate::config::Config,
    pub ir: Vec<NativeSourceText>,
    pub records: Vec<NativeSourceText>,
    pub adr: Vec<NativeSourceText>,
    pub tests: Vec<NativeTestAnalysis>,
}

#[derive(Default)]
pub struct RepositoryCheckInputs {
    pub guides: Option<Vec<NativeSourceText>>,
    pub surface: Option<Vec<NativeSurfaceAnalysis>>,
    pub unspecified: Option<Vec<NativeSourceText>>,
    /// `対`。`言語の一覧`の言語が2つ以上のときだけ読む（REQ-core-334、REQ-core-336）
    pub translations: crate::translations::Pairs,
}
