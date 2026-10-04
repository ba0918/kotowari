use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::Arc,
};

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

/// Lossless enumeration identity is not the lossy display/rule path.
/// Text is what the producer actually acquired, including any legacy read-path conversion.
/// IR display paths are relative to the configured IR place; other paths are project-relative.
#[derive(Debug, Clone)]
pub struct NativeSourceText {
    identity: PathBuf,
    path: String,
    text: Arc<str>,
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
fn native_error(error: InputError) -> crate::StopReason {
    crate::StopReason::ConfigError(error.to_string())
}
fn input_stop(error: crate::StopReason) -> InputError {
    match error {
        crate::StopReason::ConfigError(detail) => InputError::ConfigError(detail),
        other => InputError::InvalidInput(other.to_string()),
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
    pub language: Option<String>,
    pub has_query: bool,
    pub tests: Vec<crate::tests_discovery::DiscoveredTest>,
    pub line_markers: Vec<crate::tests_discovery::Marker>,
    pub findings: Vec<crate::Finding>,
}
#[derive(Debug, Clone)]
pub struct SurfaceAnalysis {
    pub source: SourceText,
    pub language: Option<String>,
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
    pub changes: Option<Vec<SourceText>>,
    /// 追加の指摘の群。渡したものだけを加える（TBL-core-042）
    pub groups: Vec<FindingGroup>,
}

/// 名前を付けた追加の指摘の群（TBL-core-042）。core は群の意味を知らず、指摘をほかの指摘と
/// 合わせて並べて数え、名前と数を結果に持たせるだけである
#[derive(Debug, Clone)]
pub struct FindingGroup {
    tally: GroupTally,
    findings: Vec<crate::Finding>,
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
    name: String,
    files: usize,
    marks: usize,
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

fn find_group<'a>(groups: &'a [GroupTally], name: &str) -> Option<&'a GroupTally> {
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
    pub changes: Option<Vec<NativeSourceText>>,
    pub surface: Option<Vec<NativeSurfaceAnalysis>>,
    pub unspecified: Option<Vec<NativeSourceText>>,
}

struct Policy {
    config: crate::config::Config,
}
impl Policy {
    fn new(config: crate::config::Config) -> Result<Self, crate::StopReason> {
        Ok(Self {
            config: config.validated()?,
        })
    }
    fn tests(&self) -> bool {
        !self.config.tests.files.is_empty()
    }
    fn guides(&self) -> bool {
        !self.config.guides.files.is_empty()
    }
    fn surface(&self) -> bool {
        !self.config.surface.rules.is_empty()
    }
    fn unspecified(&self) -> bool {
        self.surface() && self.config.surface.unspecified.is_some()
    }
    fn changes(&self) -> bool {
        self.config.changes.is_some()
    }
}
fn selected<T>(input: Option<Vec<T>>, enabled: bool, name: &str) -> Result<Vec<T>, InputError> {
    if !enabled {
        return Ok(vec![]);
    }
    input.ok_or_else(|| InputError::InputMissing(name.into()))
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Identity {
    Logical(String),
    Native(PathBuf),
}
struct Text {
    identity: Identity,
    path: String,
    text: Arc<str>,
}
impl From<SourceText> for Text {
    fn from(source: SourceText) -> Self {
        Self {
            identity: Identity::Logical(source.path.clone()),
            path: source.path,
            text: source.text,
        }
    }
}
impl From<NativeSourceText> for Text {
    fn from(source: NativeSourceText) -> Self {
        Self {
            identity: Identity::Native(source.identity),
            path: source.path,
            text: source.text,
        }
    }
}
#[derive(Default)]
struct Originals(BTreeMap<Identity, Arc<str>>);
impl Originals {
    fn admit<'a>(&mut self, sources: impl IntoIterator<Item = &'a Text>) -> Result<(), InputError> {
        let mut seen = BTreeSet::new();
        for source in sources {
            if matches!(&source.identity, Identity::Native(path) if path.as_os_str().is_empty()) {
                return Err(InputError::InvalidInput(
                    "native source identity must not be empty".into(),
                ));
            }
            if !seen.insert(source.identity.clone()) {
                return Err(InputError::InvalidInput(format!(
                    "duplicate source identity: {}",
                    source.path
                )));
            }
            if let Some(previous) = self.0.get(&source.identity) {
                if previous != &source.text {
                    return Err(InputError::InvalidInput(format!(
                        "different source text for {}",
                        source.path
                    )));
                }
            } else {
                self.0.insert(source.identity.clone(), source.text.clone());
            }
        }
        Ok(())
    }
}
struct Test {
    source: Text,
    analysis: TestAnalysis,
}
struct Surface {
    source: Text,
    analysis: SurfaceAnalysis,
}
fn logical_tests(files: Vec<TestAnalysis>) -> Vec<Test> {
    files
        .into_iter()
        .map(|analysis| Test {
            source: analysis.source.clone().into(),
            analysis,
        })
        .collect()
}
fn logical_surfaces(files: Vec<SurfaceAnalysis>) -> Vec<Surface> {
    files
        .into_iter()
        .map(|analysis| Surface {
            source: analysis.source.clone().into(),
            analysis,
        })
        .collect()
}
fn native_tests(files: Vec<NativeTestAnalysis>) -> Result<Vec<Test>, InputError> {
    files
        .into_iter()
        .map(|file| {
            original_analysis(&file.source, &file.analysis.source)?;
            Ok(Test {
                source: file.source.into(),
                analysis: file.analysis,
            })
        })
        .collect()
}
fn native_surfaces(files: Vec<NativeSurfaceAnalysis>) -> Result<Vec<Surface>, InputError> {
    files
        .into_iter()
        .map(|file| {
            original_analysis(&file.source, &file.analysis.source)?;
            Ok(Surface {
                source: file.source.into(),
                analysis: file.analysis,
            })
        })
        .collect()
}
fn original_analysis(source: &NativeSourceText, analysis: &SourceText) -> Result<(), InputError> {
    if source.text() != analysis.text() {
        return Err(InputError::InvalidInput(format!(
            "different analysis source text for {}",
            source.path()
        )));
    }
    Ok(())
}
fn texts<T: Into<Text>>(sources: Vec<T>) -> Vec<Text> {
    sources.into_iter().map(Into::into).collect()
}
fn entries(sources: &[Text]) -> impl Iterator<Item = (&str, &str)> {
    sources
        .iter()
        .map(|source| (source.path.as_str(), source.text.as_ref()))
}

struct ReadPreparation {
    policy: Policy,
    originals: Originals,
    docs: Vec<crate::ir::IrDocument>,
}
impl ReadPreparation {
    fn new(policy: Policy) -> Self {
        Self {
            policy,
            originals: Originals::default(),
            docs: vec![],
        }
    }
    fn parse_ir(&mut self, source: &Text, relative: &str) -> Result<(), crate::StopReason> {
        let (directory, filename) = relative.rsplit_once('/').unwrap_or(("", relative));
        let mut doc = crate::ir::parse_document(filename, &source.text)?;
        doc.relative_path = relative.into();
        doc.directory = directory.into();
        self.docs.push(doc);
        Ok(())
    }
    fn finish(
        mut self,
        records: Vec<Text>,
        adr: Vec<Text>,
        tests: Vec<Test>,
    ) -> Result<ReadModel, InputError> {
        self.originals.admit(&records)?;
        self.originals.admit(&adr)?;
        self.originals
            .admit(tests.iter().map(|file| &file.source))?;
        Ok(self.calculate(records, adr, tests))
    }
    fn calculate(self, records: Vec<Text>, adr: Vec<Text>, tests: Vec<Test>) -> ReadModel {
        let config = &self.policy.config;
        let mut docs = self.docs;
        docs.sort_by(|left, right| {
            left.relative_path
                .as_bytes()
                .cmp(right.relative_path.as_bytes())
        });
        let duplicates = crate::ir::GlossaryDuplicates::new(&docs);
        let mut findings = crate::ir::check_documents_with_duplicates(&docs, config, &duplicates);
        let context =
            crate::sources::context_from_entries(config, entries(&records), entries(&adr));
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
            tests
                .iter()
                .map(|file| (file.source.path.as_str(), &file.analysis)),
            &docs,
            &known_ids,
            &config.ir,
            &mut findings,
        );
        crate::deferred_notices::check(&docs, &config.ir, &discovered.markers, &mut findings);
        ReadModel {
            policy: self.policy,
            originals: self.originals,
            docs,
            findings,
            discovered,
            context,
        }
    }
}

/// Opaque preparation preserves parse stops between native acquisition steps.
pub struct RepositoryReadPreparation {
    inner: ReadPreparation,
    ir_identities: BTreeSet<Identity>,
}
impl RepositoryReadPreparation {
    pub fn new(config: crate::config::Config) -> Result<Self, crate::StopReason> {
        Ok(Self {
            inner: ReadPreparation::new(Policy::new(config)?),
            ir_identities: BTreeSet::new(),
        })
    }
    pub fn config(&self) -> &crate::config::Config {
        &self.inner.policy.config
    }
    pub fn push_ir(&mut self, source: NativeSourceText) -> Result<(), crate::StopReason> {
        let source: Text = source.into();
        if !self.ir_identities.insert(source.identity.clone()) {
            return Err(native_error(InputError::InvalidInput(format!(
                "duplicate source identity: {}",
                source.path
            ))));
        }
        self.inner
            .originals
            .admit([&source])
            .map_err(native_error)?;
        self.inner.parse_ir(&source, &source.path)
    }
    pub fn finish(
        self,
        records: Vec<NativeSourceText>,
        adr: Vec<NativeSourceText>,
        tests: Vec<NativeTestAnalysis>,
    ) -> Result<ReadModel, crate::StopReason> {
        let tests = selected(Some(tests), self.inner.policy.tests(), "test information")
            .and_then(native_tests)
            .map_err(native_error)?;
        self.inner
            .finish(texts(records), texts(adr), tests)
            .map_err(native_error)
    }
}

pub struct ReadModel {
    policy: Policy,
    originals: Originals,
    pub(crate) docs: Vec<crate::ir::IrDocument>,
    pub(crate) findings: Vec<crate::Finding>,
    pub(crate) discovered: crate::tests_discovery::DiscoveredTests,
    pub(crate) context: crate::sources::SourceContext,
}
pub struct ReadList(crate::list::ListResult);
pub struct QueryReport(crate::query::QueryResult);
impl ReadList {
    pub fn items(&self) -> &[crate::list::ListItem] {
        &self.0.items
    }
}
impl QueryReport {
    pub fn items(&self) -> &[crate::query::QueryItem] {
        &self.0.items
    }
}

fn logical_read(inputs: ReadInputs, policy: Policy) -> Result<ReadModel, InputError> {
    let ir = texts(selected(inputs.ir, true, "IR")?);
    let records = texts(selected(inputs.records, true, "records")?);
    let adr = texts(selected(inputs.adr, true, "ADR")?);
    let tests = logical_tests(selected(inputs.tests, policy.tests(), "test information")?);
    let mut preparation = ReadPreparation::new(policy);
    preparation.originals.admit(&ir)?;
    preparation.originals.admit(&records)?;
    preparation.originals.admit(&adr)?;
    preparation
        .originals
        .admit(tests.iter().map(|file| &file.source))?;
    let place = crate::normalize_path(&preparation.policy.config.ir);
    for source in &ir {
        let relative = if place.is_empty() {
            source.path.as_str()
        } else {
            source
                .path
                .strip_prefix(&format!("{place}/"))
                .ok_or_else(|| {
                    InputError::InvalidInput("IR source must be under its configured place".into())
                })?
        };
        preparation.parse_ir(source, relative).map_err(input_stop)?;
    }
    Ok(preparation.calculate(records, adr, tests))
}
impl ReadModel {
    pub fn build(inputs: ReadInputs) -> Result<Self, InputError> {
        let policy = Policy::new(inputs.config.clone()).map_err(input_stop)?;
        logical_read(inputs, policy)
    }
    pub fn build_repository(input: RepositoryReadInputs) -> Result<Self, crate::StopReason> {
        let mut preparation = RepositoryReadPreparation::new(input.config)?;
        for source in input.ir {
            preparation.push_ir(source)?;
        }
        preparation.finish(input.records, input.adr, input.tests)
    }
    pub fn config(&self) -> &crate::config::Config {
        &self.policy.config
    }
    pub fn list(&self) -> ReadList {
        ReadList(crate::list::build(
            &self.docs,
            &self.config().ir,
            &self.discovered.markers,
        ))
    }
    pub fn query(&self, id: &str) -> Result<QueryReport, InputError> {
        crate::query::build(&self.docs, &self.config().ir, &self.discovered.markers, id)
            .map(QueryReport)
            .ok_or_else(|| InputError::UnknownQuery(id.into()))
    }
    /// ほかの文書の`ガイドの印`を`ガイド`と同じ規則で読むもの（REQ-core-286）
    pub fn guide_reader(&self) -> crate::guides::GuideReader<'_> {
        crate::guides::GuideReader::new(&self.docs)
    }
    pub fn validate_guide_paths<'a>(
        &self,
        paths: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), crate::StopReason> {
        if self.policy.guides() {
            crate::guides::validate_overlap(paths, &self.discovered.files)?;
        }
        Ok(())
    }
    pub fn prepare_repository_inspection(self) -> RepositoryInspectionPreparation {
        RepositoryInspectionPreparation {
            inner: CheckPreparation::new(self),
            phase: InspectionPhase::Changes,
        }
    }
    pub fn inspect_repository(
        self,
        input: RepositoryCheckInputs,
    ) -> Result<Inspection, crate::StopReason> {
        let mut preparation = self.prepare_repository_inspection();
        preparation.changes(input.changes)?;
        preparation.guides(input.guides)?;
        preparation.surface(input.surface, input.unspecified)?;
        preparation.finish()
    }
}

struct CheckPreparation {
    read: ReadModel,
    findings: Vec<crate::Finding>,
    guides: crate::guides::GuideTally,
    surface: Option<crate::surface::SurfaceTally>,
    groups: Vec<GroupTally>,
}
impl CheckPreparation {
    fn new(read: ReadModel) -> Self {
        let findings = read.findings.clone();
        Self {
            read,
            findings,
            guides: Default::default(),
            surface: None,
            groups: Vec::new(),
        }
    }
    fn group(&mut self, group: FindingGroup) -> Result<(), InputError> {
        if find_group(&self.groups, &group.tally.name).is_some() {
            return Err(InputError::InvalidInput(format!(
                "duplicate finding group: {}",
                group.tally.name
            )));
        }
        self.findings.extend(group.findings);
        self.groups.push(group.tally);
        Ok(())
    }
    fn changes(&mut self, files: &[Text]) {
        if self.read.policy.changes() {
            crate::change_records::check_entries(
                entries(files),
                self.read.config(),
                &self.read.docs,
                &self.read.context,
                &mut self.findings,
            );
        }
    }
    fn guides(&mut self, files: &[Text]) -> Result<(), crate::StopReason> {
        self.read
            .validate_guide_paths(files.iter().map(|file| file.path.as_str()))?;
        if self.read.policy.guides() {
            self.guides = crate::guides::check_entries(
                entries(files),
                &self.read.discovered.files,
                &self.read.docs,
                &mut self.findings,
            )?;
        }
        Ok(())
    }
    fn surface(
        &mut self,
        files: &[Surface],
        unspecified: &[Text],
    ) -> Result<(), crate::StopReason> {
        if self.read.policy.surface() {
            self.surface = Some(crate::surface::check_entries(
                files.iter().map(|file| &file.analysis),
                &self.read.docs,
                entries(unspecified),
                &mut self.findings,
            )?);
        }
        Ok(())
    }
    fn finish(mut self) -> Inspection {
        crate::sort_findings(&mut self.findings);
        let mut status = crate::status::build(
            &self.read.docs,
            &self.read.config().ir,
            &self.read.discovered.markers,
            self.read.discovered.tally.clone(),
            self.guides,
            self.surface.unwrap_or_default(),
            &self.findings,
        );
        status.groups = self.groups.clone();
        let status = StatusReport(status);
        let check = CheckReport(crate::CheckResult {
            files: self.read.docs.len(),
            lines: self.read.docs.iter().map(|doc| doc.line_count).sum(),
            counts: crate::count_findings(&self.findings),
            findings: self.findings,
            tests: self.read.discovered.tally.clone(),
            guides: self.guides,
            surface: self.surface.map(|tally| crate::surface::Unlisted {
                unspecified: tally.unspecified,
            }),
            groups: self.groups,
        });
        Inspection {
            read: self.read,
            check,
            status,
        }
    }
}

/// Results cannot be assembled until all acquisition phases have been admitted.
#[derive(PartialEq, Eq)]
enum InspectionPhase {
    Changes,
    Guides,
    Surface,
    Complete,
}
pub struct RepositoryInspectionPreparation {
    inner: CheckPreparation,
    phase: InspectionPhase,
}
impl RepositoryInspectionPreparation {
    fn at_phase(&self, phase: InspectionPhase) -> Result<(), crate::StopReason> {
        if self.phase != phase {
            return Err(native_error(InputError::InvalidInput(
                "inspection phase out of order".into(),
            )));
        }
        Ok(())
    }
    pub fn config(&self) -> &crate::config::Config {
        self.inner.read.config()
    }
    pub fn validate_guide_paths<'a>(
        &self,
        paths: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), crate::StopReason> {
        self.inner.read.validate_guide_paths(paths)
    }
    pub fn changes(
        &mut self,
        input: Option<Vec<NativeSourceText>>,
    ) -> Result<(), crate::StopReason> {
        self.at_phase(InspectionPhase::Changes)?;
        let files = texts(
            selected(input, self.inner.read.policy.changes(), "change records")
                .map_err(native_error)?,
        );
        self.inner
            .read
            .originals
            .admit(&files)
            .map_err(native_error)?;
        self.inner.changes(&files);
        self.phase = InspectionPhase::Guides;
        Ok(())
    }
    pub fn guides(
        &mut self,
        input: Option<Vec<NativeSourceText>>,
    ) -> Result<(), crate::StopReason> {
        self.at_phase(InspectionPhase::Guides)?;
        let files = texts(
            selected(input, self.inner.read.policy.guides(), "guides").map_err(native_error)?,
        );
        self.inner
            .read
            .originals
            .admit(&files)
            .map_err(native_error)?;
        self.inner.guides(&files)?;
        self.phase = InspectionPhase::Surface;
        Ok(())
    }
    pub fn surface(
        &mut self,
        input: Option<Vec<NativeSurfaceAnalysis>>,
        unspecified: Option<Vec<NativeSourceText>>,
    ) -> Result<(), crate::StopReason> {
        self.at_phase(InspectionPhase::Surface)?;
        let files = native_surfaces(
            selected(input, self.inner.read.policy.surface(), "surface analysis")
                .map_err(native_error)?,
        )
        .map_err(native_error)?;
        let unspecified = texts(
            selected(
                unspecified,
                self.inner.read.policy.unspecified(),
                "unspecified surfaces",
            )
            .map_err(native_error)?,
        );
        self.inner
            .read
            .originals
            .admit(files.iter().map(|file| &file.source))
            .map_err(native_error)?;
        self.inner
            .read
            .originals
            .admit(&unspecified)
            .map_err(native_error)?;
        self.inner.surface(&files, &unspecified)?;
        self.phase = InspectionPhase::Complete;
        Ok(())
    }
    /// 追加の指摘の群を加える（TBL-core-042）。どの段の間でも加えられる
    pub fn group(&mut self, group: FindingGroup) -> Result<(), crate::StopReason> {
        self.inner.group(group).map_err(native_error)
    }
    pub fn finish(self) -> Result<Inspection, crate::StopReason> {
        self.at_phase(InspectionPhase::Complete)?;
        Ok(self.inner.finish())
    }
}

pub struct Inspection {
    read: ReadModel,
    check: CheckReport,
    status: StatusReport,
}
impl Inspection {
    pub fn build(inputs: CheckInputs) -> Result<Self, InputError> {
        let policy = Policy::new(inputs.read.config.clone()).map_err(input_stop)?;
        let guides = texts(selected(inputs.guides, policy.guides(), "guides")?);
        let surface = logical_surfaces(selected(
            inputs.surface,
            policy.surface(),
            "surface analysis",
        )?);
        let unspecified = texts(selected(
            inputs.unspecified,
            policy.unspecified(),
            "unspecified surfaces",
        )?);
        let changes = texts(selected(
            inputs.changes,
            policy.changes(),
            "change records",
        )?);
        let groups = inputs.groups;
        let mut read = logical_read(inputs.read, policy)?;
        read.originals.admit(&guides)?;
        read.originals
            .admit(surface.iter().map(|file| &file.source))?;
        read.originals.admit(&unspecified)?;
        read.originals.admit(&changes)?;
        let mut preparation = CheckPreparation::new(read);
        preparation.changes(&changes);
        preparation.guides(&guides).map_err(input_stop)?;
        preparation
            .surface(&surface, &unspecified)
            .map_err(input_stop)?;
        for group in groups {
            preparation.group(group)?;
        }
        Ok(preparation.finish())
    }
    pub fn into_check(self) -> CheckReport {
        self.check
    }
    pub fn into_status(self) -> StatusReport {
        self.status
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
    /// 追加の指摘の群の数。渡した順に並ぶ（TBL-core-042）
    pub fn groups(&self) -> &[GroupTally] {
        &self.0.groups
    }
    pub fn group(&self, name: &str) -> Option<&GroupTally> {
        find_group(&self.0.groups, name)
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
    pub fn tests(&self) -> &BTreeMap<String, crate::TestFileTally> {
        &self.0.tests
    }
    pub fn files(&self) -> usize {
        self.0.files
    }
    pub fn lines(&self) -> usize {
        self.0.lines
    }
    pub fn counts(&self) -> &BTreeMap<String, usize> {
        &self.0.counts
    }
    /// 追加の指摘の群の数。渡した順に並ぶ（TBL-core-042）
    pub fn groups(&self) -> &[GroupTally] {
        &self.0.groups
    }
    pub fn group(&self, name: &str) -> Option<&GroupTally> {
        find_group(&self.0.groups, name)
    }
}
