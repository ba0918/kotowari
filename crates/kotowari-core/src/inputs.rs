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
    pub(crate) context: crate::sources::SourceContext,
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
            context,
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
    pub fn guides(&self) -> &crate::guides::GuideTally {
        &self.0.guides
    }
    pub fn surface(&self) -> Option<&crate::surface::Unlisted> {
        self.0.surface.as_ref()
    }
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
    pub fn into_check(self) -> CheckReport {
        self.check
    }
    pub fn into_status(self) -> StatusReport {
        self.status
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

/// Acquired text and discovery facts, with the acquisition layer's display spellings.
/// Unlike canonical logical SourceText paths, these paths preserve native CLI identities.
/// No document, finding result or independently assembled report is accepted here.
pub struct RepositoryReadInputs {
    pub config: crate::config::Config,
    /// Paths relative to the configured IR place and original text.
    pub ir: Vec<(String, String)>,
    pub records: Vec<(String, String)>,
    pub adr: Vec<(String, String)>,
    pub tests: Vec<(String, TestAnalysis)>,
}

/// Additional acquired inputs for an inspection of an existing repository read.
#[derive(Default)]
pub struct RepositoryCheckInputs {
    pub guides: Option<Vec<(String, String)>>,
    pub changes: Option<Vec<(String, String)>>,
    pub surface: Option<Vec<SurfaceAnalysis>>,
    pub unspecified: Option<Vec<SourceText>>,
}

impl ReadModel {
    /// Parse and calculate all read results from acquired input. Configuration is validated
    /// and all IR documents and diagnostics are derived here, not supplied by the caller.
    pub fn build_repository(mut input: RepositoryReadInputs) -> Result<Self, crate::StopReason> {
        input.config = input.config.validated()?;
        let config = &input.config;
        let mut docs = Vec::new();
        for (relative, text) in &input.ir {
            let (directory, filename) = relative.rsplit_once('/').unwrap_or(("", relative));
            let mut doc = crate::ir::parse_document(filename, text)?;
            doc.relative_path = relative.clone();
            doc.directory = directory.into();
            docs.push(doc);
        }
        docs.sort_by(|left, right| {
            left.relative_path
                .as_bytes()
                .cmp(right.relative_path.as_bytes())
        });
        let duplicates = crate::ir::GlossaryDuplicates::new(&docs);
        let mut findings = crate::ir::check_documents_with_duplicates(&docs, config, &duplicates);
        let context = crate::sources::context_from_entries(
            config,
            input
                .records
                .iter()
                .map(|(path, text)| (path.as_str(), text.as_str())),
            input
                .adr
                .iter()
                .map(|(path, text)| (path.as_str(), text.as_str())),
        );
        crate::sources::check_sources_with_duplicates(
            &docs,
            &context,
            &config.ir,
            &duplicates,
            &mut findings,
        );
        crate::record_form::check_record_forms(&context, &mut findings);
        let known_ids = crate::collect_known_ids(&docs);
        crate::terms::check_terms_and_vague_words_with_duplicates(
            &docs,
            &known_ids,
            &config.vague_words,
            &config.ir,
            &duplicates,
            &mut findings,
        );
        let paths = docs.iter().map(|doc| doc.relative_path.clone()).collect();
        crate::terms::check_document_references(&docs, &config.ir, &paths, &mut findings);
        let discovered = crate::tests_discovery::check_entries(
            input.tests.iter().map(|(path, file)| (path.as_str(), file)),
            &docs,
            &known_ids,
            &config.ir,
            &mut findings,
        );
        crate::deferred_notices::check(&docs, &config.ir, &discovered.markers, &mut findings);
        Ok(Self {
            inputs: ReadInputs {
                config: input.config,
                ..Default::default()
            },
            docs,
            findings,
            discovered,
            context,
        })
    }

    /// Compute an inspection from this read and the additional acquired text/facts.
    pub fn inspect_repository(
        self,
        input: RepositoryCheckInputs,
    ) -> Result<Inspection, crate::StopReason> {
        let config = &self.inputs.config;
        let required = |provided: bool, group: &str| {
            if provided {
                Ok(())
            } else {
                Err(crate::StopReason::ConfigError(format!(
                    "input missing: {group}"
                )))
            }
        };
        if config.changes.is_some() {
            required(input.changes.is_some(), "change records")?;
        }
        if !config.guides.files.is_empty() {
            required(input.guides.is_some(), "guides")?;
        }
        if !config.surface.rules.is_empty() {
            required(input.surface.is_some(), "surface analysis")?;
            if config.surface.unspecified.is_some() {
                required(input.unspecified.is_some(), "unspecified surfaces")?;
            }
        }
        let mut findings = self.findings.clone();
        if config.changes.is_some() {
            crate::change_records::check_entries(
                input
                    .changes
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(path, text)| (path.as_str(), text.as_str())),
                config,
                &self.docs,
                &self.context,
                &mut findings,
            );
        }
        let guides = if config.guides.files.is_empty() {
            Default::default()
        } else {
            crate::guides::check_entries(
                input
                    .guides
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(path, text)| (path.as_str(), text.as_str())),
                &self.discovered.files,
                &self.docs,
                &mut findings,
            )?
        };
        let surface = if config.surface.rules.is_empty() {
            None
        } else {
            Some(crate::surface::check_analysis(
                input.surface.as_ref().unwrap(),
                &self.docs,
                input.unspecified.as_deref().unwrap_or(&[]),
                &mut findings,
            )?)
        };
        crate::sort_findings(&mut findings);
        let status = StatusReport(crate::status::build(
            &self.docs,
            &config.ir,
            &self.discovered.markers,
            self.discovered.tally.clone(),
            guides,
            surface.unwrap_or_default(),
            &findings,
        ));
        let check = CheckReport(crate::CheckResult {
            files: self.docs.len(),
            lines: self.docs.iter().map(|doc| doc.line_count).sum(),
            counts: crate::count_findings(&findings),
            findings,
            tests: self.discovered.tally.clone(),
            guides,
            surface: surface.map(|tally| crate::surface::Unlisted {
                unspecified: tally.unspecified,
            }),
        });
        Ok(Inspection {
            read: self,
            check,
            status,
        })
    }
}
