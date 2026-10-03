use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SourceText {
    path: String,
    text: Arc<str>,
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
    pub language: Option<String>,
    pub has_query: bool,
    pub tests: Vec<crate::tests_discovery::DiscoveredTest>,
    pub line_markers: Vec<crate::tests_discovery::Marker>,
    pub findings: Vec<crate::Finding>,
}

pub struct ReadModel {
    inputs: ReadInputs,
    pub(crate) docs: Vec<crate::ir::IrDocument>,
    pub(crate) findings: Vec<crate::Finding>,
    pub(crate) discovered: crate::tests_discovery::DiscoveredTests,
}

pub struct ReadList(crate::list::ListResult);

pub struct QueryReport(crate::query::QueryResult);

impl QueryReport {
    pub fn items(&self) -> &[crate::query::QueryItem] {
        &self.0.items
    }
}

impl ReadList {
    pub fn items(&self) -> &[crate::list::ListItem] {
        &self.0.items
    }
}

impl ReadModel {
    #[doc(hidden)]
    pub fn from_calculation(
        config: crate::config::Config,
        docs: Vec<crate::ir::IrDocument>,
        findings: Vec<crate::Finding>,
        discovered: crate::tests_discovery::DiscoveredTests,
    ) -> Self {
        Self {
            inputs: ReadInputs {
                config,
                ..Default::default()
            },
            docs,
            findings,
            discovered,
        }
    }
    pub fn build(mut inputs: ReadInputs) -> Result<Self, InputError> {
        inputs.config = inputs
            .config
            .validated()
            .map_err(|error| InputError::ConfigError(error.to_string()))?;
        required(&inputs.ir, "IR")?;
        required(&inputs.records, "records")?;
        required(&inputs.adr, "ADR")?;
        if !inputs.config.tests.files.is_empty() {
            required(&inputs.tests, "test information")?;
        }
        let mut identity = std::collections::BTreeMap::new();
        for group in [&inputs.ir, &inputs.records, &inputs.adr] {
            validate_group(group.as_ref().unwrap().iter(), &mut identity)?;
        }
        if !inputs.config.tests.files.is_empty() {
            validate_group(
                inputs
                    .tests
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|file| &file.source),
                &mut identity,
            )?;
        }
        let mut docs = Vec::new();
        let place = crate::normalize_path(&inputs.config.ir);
        for source in inputs.ir.as_ref().unwrap() {
            let relative = if place.is_empty() {
                source.path()
            } else {
                source
                    .path()
                    .strip_prefix(&format!("{place}/"))
                    .ok_or_else(|| {
                        InputError::InvalidInput(
                            "IR source must be under its configured place".into(),
                        )
                    })?
            };
            let filename = relative.rsplit('/').next().unwrap();
            let mut doc = crate::ir::parse_document(filename, source.text())
                .map_err(|error| InputError::InvalidInput(error.to_string()))?;
            doc.relative_path = relative.to_owned();
            doc.directory = relative
                .rsplit_once('/')
                .map(|(dir, _)| dir)
                .unwrap_or("")
                .to_owned();
            docs.push(doc);
        }
        docs.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
        let duplicates = crate::ir::GlossaryDuplicates::new(&docs);
        let mut findings =
            crate::ir::check_documents_with_duplicates(&docs, &inputs.config, &duplicates);
        let known_ids = crate::collect_known_ids(&docs);
        let context = crate::sources::context_from_texts(
            &inputs.config,
            inputs.records.as_ref().unwrap(),
            inputs.adr.as_ref().unwrap(),
        );
        crate::sources::check_sources_with_duplicates(
            &docs,
            &context,
            &inputs.config.ir,
            &duplicates,
            &mut findings,
        );
        crate::record_form::check_record_forms(&context, &mut findings);
        crate::terms::check_terms_and_vague_words_with_duplicates(
            &docs,
            &known_ids,
            &inputs.config.vague_words,
            &inputs.config.ir,
            &duplicates,
            &mut findings,
        );
        let ir_paths = docs.iter().map(|doc| doc.relative_path.clone()).collect();
        crate::terms::check_document_references(&docs, &inputs.config.ir, &ir_paths, &mut findings);
        let analysis = if inputs.config.tests.files.is_empty() {
            &[][..]
        } else {
            inputs.tests.as_ref().unwrap().as_slice()
        };
        let discovered = crate::tests_discovery::check_analysis(
            analysis,
            &docs,
            &known_ids,
            &inputs.config.ir,
            &mut findings,
        );
        crate::deferred_notices::check(
            &docs,
            &inputs.config.ir,
            &discovered.markers,
            &mut findings,
        );
        Ok(Self {
            inputs,
            docs,
            findings,
            discovered,
        })
    }

    pub fn config(&self) -> &crate::config::Config {
        &self.inputs.config
    }

    pub fn list(&self) -> ReadList {
        ReadList(crate::list::build(
            &self.docs,
            &self.inputs.config.ir,
            &self.discovered.markers,
        ))
    }

    pub fn query(&self, id: &str) -> Result<QueryReport, InputError> {
        crate::query::build(
            &self.docs,
            &self.inputs.config.ir,
            &self.discovered.markers,
            id,
        )
        .map(QueryReport)
        .ok_or_else(|| InputError::UnknownQuery(id.to_owned()))
    }
}

fn validate_group<'a>(
    sources: impl IntoIterator<Item = &'a SourceText>,
    identity: &mut std::collections::BTreeMap<&'a str, &'a str>,
) -> Result<(), InputError> {
    let mut paths = std::collections::BTreeSet::new();
    for source in sources {
        if !paths.insert(source.path()) {
            return Err(InputError::InvalidInput(format!(
                "duplicate logical path: {}",
                source.path()
            )));
        }
        if let Some(previous) = identity.insert(source.path(), source.text())
            && previous != source.text()
        {
            return Err(InputError::InvalidInput(format!(
                "different source text for {}",
                source.path()
            )));
        }
    }
    Ok(())
}

fn required<T>(group: &Option<T>, name: &str) -> Result<(), InputError> {
    if group.is_none() {
        Err(InputError::InputMissing(name.to_owned()))
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct CheckInputs {
    pub read: ReadInputs,
    pub guides: Option<Vec<SourceText>>,
    pub surface: Option<Vec<SurfaceAnalysis>>,
    pub unspecified: Option<Vec<SourceText>>,
    pub changes: Option<Vec<SourceText>>,
}

#[derive(Debug, Clone)]
pub struct SurfaceAnalysis {
    pub source: SourceText,
    pub language: Option<String>,
    pub has_query: bool,
    pub surfaces: Vec<crate::surface::Surface>,
    pub findings: Vec<crate::Finding>,
}

pub struct Inspection {
    read: ReadModel,
    check: CheckReport,
    status: StatusReport,
}

pub struct StatusReport(crate::status::StatusResult);

impl StatusReport {
    #[doc(hidden)]
    pub fn from_calculation(result: crate::status::StatusResult) -> Self {
        Self(result)
    }
    pub fn complete(&self) -> bool {
        self.0.complete
    }
    pub fn documents(&self) -> &crate::status::Documents {
        &self.0.documents
    }
    pub fn items(&self) -> &crate::status::Items {
        &self.0.items
    }
    pub fn requirements(&self) -> &crate::status::Requirements {
        &self.0.requirements
    }
    pub fn scenarios(&self) -> &crate::status::Scenarios {
        &self.0.scenarios
    }
    pub fn tests(&self) -> &crate::status::Tests {
        &self.0.tests
    }
    pub fn guides(&self) -> &crate::guides::GuideTally {
        &self.0.guides
    }
    pub fn surface(&self) -> &crate::surface::SurfaceTally {
        &self.0.surface
    }
    pub fn findings(&self) -> &crate::status::Findings {
        &self.0.findings
    }
}

pub struct CheckReport(crate::CheckResult);

impl CheckReport {
    pub fn findings(&self) -> &[crate::Finding] {
        &self.0.findings
    }
    pub fn tests(&self) -> &std::collections::BTreeMap<String, crate::TestFileTally> {
        &self.0.tests
    }
    pub fn files(&self) -> usize {
        self.0.files
    }
    pub fn lines(&self) -> usize {
        self.0.lines
    }
    pub fn counts(&self) -> &std::collections::BTreeMap<String, usize> {
        &self.0.counts
    }
}

impl Inspection {
    #[doc(hidden)]
    pub fn into_check(self) -> CheckReport {
        self.check
    }
    #[doc(hidden)]
    pub fn into_status(self) -> StatusReport {
        self.status
    }
    #[doc(hidden)]
    pub fn from_calculation(
        read: ReadModel,
        check: crate::CheckResult,
        status: crate::status::StatusResult,
    ) -> Self {
        Self {
            read,
            check: CheckReport(check),
            status: StatusReport(status),
        }
    }
    pub fn build(mut inputs: CheckInputs) -> Result<Self, InputError> {
        inputs.read.config = inputs
            .read
            .config
            .validated()
            .map_err(|error| InputError::ConfigError(error.to_string()))?;
        let config = &inputs.read.config;
        if !config.guides.files.is_empty() {
            required(&inputs.guides, "guides")?;
        }
        if !config.surface.rules.is_empty() {
            required(&inputs.surface, "surface analysis")?;
            if config.surface.unspecified.is_some() {
                required(&inputs.unspecified, "unspecified surfaces")?;
            }
        }
        if config.changes.is_some() {
            required(&inputs.changes, "change records")?;
        }
        let read = ReadModel::build(inputs.read.clone())?;
        let mut identity = std::collections::BTreeMap::new();
        for group in [&inputs.read.ir, &inputs.read.records, &inputs.read.adr] {
            validate_group(group.as_ref().unwrap().iter(), &mut identity)?;
        }
        if !config.tests.files.is_empty() {
            validate_group(
                inputs
                    .read
                    .tests
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|file| &file.source),
                &mut identity,
            )?;
        }
        if !config.guides.files.is_empty() {
            validate_group(inputs.guides.as_ref().unwrap().iter(), &mut identity)?;
        }
        if !config.surface.rules.is_empty() {
            validate_group(
                inputs
                    .surface
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|file| &file.source),
                &mut identity,
            )?;
            if config.surface.unspecified.is_some() {
                validate_group(inputs.unspecified.as_ref().unwrap().iter(), &mut identity)?;
            }
        }
        if config.changes.is_some() {
            validate_group(inputs.changes.as_ref().unwrap().iter(), &mut identity)?;
        }
        let mut findings = read.findings.clone();
        let context = crate::sources::context_from_texts(
            config,
            inputs.read.records.as_ref().unwrap(),
            inputs.read.adr.as_ref().unwrap(),
        );
        if config.changes.is_some() {
            crate::change_records::check_texts(
                inputs.changes.as_ref().unwrap(),
                config,
                &read.docs,
                &context,
                &mut findings,
            );
        }
        let guides = if config.guides.files.is_empty() {
            Default::default()
        } else {
            crate::guides::check_texts(
                inputs.guides.as_ref().unwrap(),
                &read.discovered.files,
                &read.docs,
                &mut findings,
            )
            .map_err(|error| InputError::ConfigError(error.to_string()))?
        };
        let surface = if config.surface.rules.is_empty() {
            None
        } else {
            let unspecified = if config.surface.unspecified.is_some() {
                inputs.unspecified.as_ref().unwrap().as_slice()
            } else {
                &[]
            };
            Some(
                crate::surface::check_analysis(
                    inputs.surface.as_ref().unwrap(),
                    &read.docs,
                    unspecified,
                    &mut findings,
                )
                .map_err(|error| InputError::ConfigError(error.to_string()))?,
            )
        };
        crate::sort_findings(&mut findings);
        let counts = crate::count_findings(&findings);
        let status = StatusReport(crate::status::build(
            &read.docs,
            &config.ir,
            &read.discovered.markers,
            read.discovered.tally.clone(),
            guides,
            surface.unwrap_or_default(),
            &findings,
        ));
        let check = CheckReport(crate::CheckResult {
            files: read.docs.len(),
            lines: read.docs.iter().map(|doc| doc.line_count).sum(),
            findings,
            counts,
            tests: read.discovered.tally.clone(),
            guides,
            surface: surface.map(|tally| crate::surface::Unlisted {
                unspecified: tally.unspecified,
            }),
        });
        Ok(Self {
            read,
            check,
            status,
        })
    }

    pub fn read(&self) -> &ReadModel {
        &self.read
    }
    pub fn check(&self) -> &CheckReport {
        &self.check
    }
    pub fn status(&self) -> &StatusReport {
        &self.status
    }
}

impl CheckReport {
    #[doc(hidden)]
    pub fn from_calculation(result: crate::CheckResult) -> Self {
        Self(result)
    }
    #[doc(hidden)]
    pub fn presentation(&self) -> &crate::CheckResult {
        &self.0
    }
}
impl ReadList {
    #[doc(hidden)]
    pub fn presentation(&self) -> &crate::list::ListResult {
        &self.0
    }
}
impl QueryReport {
    #[doc(hidden)]
    pub fn presentation(&self) -> &crate::query::QueryResult {
        &self.0
    }
}
impl StatusReport {
    #[doc(hidden)]
    pub fn presentation(&self) -> &crate::status::StatusResult {
        &self.0
    }
}
