//! 読み取りの段取りと、check、list、query、status が共有する読み取りの結果

use super::*;

pub(super) struct ReadPreparation {
    pub(super) policy: Policy,
    pub(super) originals: Originals,
    pub(super) docs: Vec<crate::ir::IrDocument>,
}

impl ReadPreparation {
    pub(super) fn new(policy: Policy) -> Self {
        Self {
            policy,
            originals: Originals::default(),
            docs: vec![],
        }
    }
    pub(super) fn parse_ir(
        &mut self,
        source: &Text,
        relative: &str,
    ) -> Result<(), crate::StopReason> {
        // REQ-core-346: `切り替えの行`は`文書が扱う範囲`の行に数えず、文の検査も受けない
        let config = &self.policy.config;
        let first = config.languages().remove(0);
        let text = crate::translations::without_switcher(config, relative, &first, &source.text);
        let doc = crate::ir::parse_document(relative, text.as_deref().unwrap_or(&source.text))?;
        self.docs.push(doc);
        Ok(())
    }
    pub(super) fn finish(
        mut self,
        records: &[Text],
        adr: &[Text],
        tests: &[Test],
    ) -> Result<ReadModel, InputError> {
        self.originals.admit(records)?;
        self.originals.admit(adr)?;
        self.originals
            .admit(tests.iter().map(|file| &file.source))?;
        Ok(self.calculate(records, adr, tests))
    }
    pub(super) fn calculate(self, records: &[Text], adr: &[Text], tests: &[Test]) -> ReadModel {
        let config = &self.policy.config;
        let mut docs = self.docs;
        docs.sort_by(|left, right| {
            left.relative_path
                .as_bytes()
                .cmp(right.relative_path.as_bytes())
        });
        let duplicates = crate::ir::GlossaryDuplicates::new(&docs);
        let mut findings = crate::ir::check_documents_with_duplicates(&docs, config, &duplicates);
        let context = crate::sources::context_from_entries(config, entries(records), entries(adr));
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
        let source_texts = records
            .iter()
            .chain(adr)
            .map(|source| (source.path.clone(), source.text.clone()))
            .collect();
        ReadModel {
            policy: self.policy,
            originals: self.originals,
            docs,
            findings,
            discovered,
            context,
            source_texts,
        }
    }
}

/// Opaque preparation preserves parse stops between native acquisition steps.
pub struct RepositoryReadPreparation {
    pub(super) inner: ReadPreparation,
    pub(super) ir_identities: BTreeSet<Identity>,
}

impl RepositoryReadPreparation {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "public API: changing the parameter type breaks callers"
    )]
    pub fn new(config: crate::config::Config) -> Result<Self, crate::StopReason> {
        Ok(Self {
            inner: ReadPreparation::new(Policy::new(&config)?),
            ir_identities: BTreeSet::new(),
        })
    }
    pub fn config(&self) -> &crate::config::Config {
        &self.inner.policy.config
    }
    pub fn push_ir(&mut self, source: NativeSourceText) -> Result<(), crate::StopReason> {
        let source: Text = source.into();
        if !self.ir_identities.insert(source.identity.clone()) {
            return Err(native_error(&InputError::InvalidInput(format!(
                "duplicate source identity: {}",
                source.path
            ))));
        }
        self.inner
            .originals
            .admit([&source])
            .map_err(|error| native_error(&error))?;
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
            .map_err(|error| native_error(&error))?;
        self.inner
            .finish(&texts(records), &texts(adr), &tests)
            .map_err(|error| native_error(&error))
    }
}

pub struct ReadModel {
    pub(super) policy: Policy,
    pub(super) originals: Originals,
    pub(crate) docs: Vec<crate::ir::IrDocument>,
    pub(crate) findings: Vec<crate::Finding>,
    pub(crate) discovered: crate::tests_discovery::DiscoveredTests,
    pub(crate) context: crate::sources::SourceContext,
    /// 判断の記録の置き場と ADR の置き場から読んだ文書の中身。出典の指す先の中身を引く
    pub(super) source_texts: BTreeMap<String, Arc<str>>,
}

pub struct ReadList(
    crate::list::ListResult,
    Option<Vec<crate::translations::Translation>>,
);

pub struct QueryReport(crate::query::QueryResult);

impl ReadList {
    pub fn items(&self) -> &[crate::list::ListItem] {
        &self.0.items
    }
    /// `対`ごとの blob hash。`言語の一覧`の言語が2つ以上のときだけ持つ（REQ-core-155）
    pub fn translations(&self) -> Option<&[crate::translations::Translation]> {
        self.1.as_deref()
    }
}

impl QueryReport {
    pub fn items(&self) -> &[crate::query::QueryItem] {
        &self.0.items
    }
}

pub(super) fn logical_read(inputs: ReadInputs, policy: Policy) -> Result<ReadModel, InputError> {
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
        let relative =
            crate::sources::relative_to_place(&source.path, &place).ok_or_else(|| {
                InputError::InvalidInput("IR source must be under its configured place".into())
            })?;
        preparation.parse_ir(source, relative).map_err(input_stop)?;
    }
    Ok(preparation.calculate(&records, &adr, &tests))
}

impl ReadModel {
    pub fn build(inputs: ReadInputs) -> Result<Self, InputError> {
        let policy = Policy::new(&inputs.config).map_err(input_stop)?;
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
        self.list_with(&crate::translations::Pairs::default())
    }
    /// "items" と、`言語の一覧`の言語が2つ以上なら pairs の "translations"（REQ-core-155）
    pub fn list_with(&self, pairs: &crate::translations::Pairs) -> ReadList {
        let items = crate::list::build(&self.docs, &self.config().ir, &self.discovered.markers);
        let translations = self.policy.translations().then(|| pairs.listing());
        ReadList(items, translations)
    }
    pub fn query(&self, id: &str) -> Result<QueryReport, InputError> {
        crate::query::build(&self.docs, &self.config().ir, &self.discovered.markers, id)
            .map(QueryReport)
            .ok_or_else(|| InputError::UnknownQuery(id.into()))
    }
    /// 読んだ`IR`の文書。パスのバイト順に並ぶ
    pub fn documents(&self) -> &[crate::ir::IrDocument] {
        &self.docs
    }
    /// 出典が指す判断の記録の決定か、判断の記録でない Markdown の見出しの節（TBL-core-012）。
    /// 指す先が無ければ None
    pub fn source_target(&self, source: &str) -> Option<crate::sources::SourceTarget> {
        let resolved = self.context.resolve(source)?;
        let text = self.source_texts.get(&resolved.path)?;
        crate::sources::SourceTarget::read(resolved, text)
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
            phase: InspectionPhase::Guides,
        }
    }
    pub fn inspect_repository(
        self,
        input: RepositoryCheckInputs,
    ) -> Result<Inspection, crate::StopReason> {
        let mut preparation = self.prepare_repository_inspection();
        preparation.guides(input.guides)?;
        preparation.surface(input.surface, input.unspecified)?;
        preparation.translations(input.translations);
        preparation.finish()
    }
}
