//! 検査の段取りと、段を順に踏んだ検査の結果

use super::*;

pub(super) struct CheckPreparation {
    pub(super) read: ReadModel,
    pub(super) findings: Vec<crate::Finding>,
    pub(super) guides: crate::guides::GuideTally,
    pub(super) surface: Option<crate::surface::SurfaceTally>,
    pub(super) groups: Vec<GroupTally>,
    pub(super) translations: crate::translations::Pairs,
}

impl CheckPreparation {
    pub(super) fn new(mut read: ReadModel) -> Self {
        // 読み取りの指摘を読むのは検査だけなので、写さずに移す
        let findings = std::mem::take(&mut read.findings);
        Self {
            read,
            findings,
            guides: Default::default(),
            surface: None,
            groups: Vec::new(),
            translations: Default::default(),
        }
    }
    pub(super) fn translations(&mut self, pairs: crate::translations::Pairs) {
        if self.read.policy.translations() {
            self.translations.extend(pairs);
        }
    }
    pub(super) fn group(&mut self, group: FindingGroup) -> Result<(), InputError> {
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
    pub(super) fn guides(&mut self, files: &[Text]) -> Result<(), crate::StopReason> {
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
    pub(super) fn surface(
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
    pub(super) fn finish(mut self) -> Result<Inspection, crate::StopReason> {
        self.findings
            .extend(self.translations.findings(self.read.config()));
        // REQ-core-342: ほかの言語の`側`の文の検査と、"files" と "lines" に数えるすべての`側`
        let sides = crate::translations::ir_sides(&self.translations, self.read.config())?;
        let (mut side_files, mut side_lines) = (0, 0);
        for documents in sides.values() {
            crate::translations::side_findings(
                documents,
                &self.read.docs,
                self.read.config(),
                &mut self.findings,
            );
            side_files += documents.len();
            side_lines += documents
                .iter()
                .map(|document| document.line_count)
                .sum::<usize>();
        }
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
        status.documents.files += side_files;
        status.documents.lines += side_lines;
        let status = StatusReport(status);
        let check = CheckReport(crate::CheckResult {
            files: self.read.docs.len() + side_files,
            lines: self
                .read
                .docs
                .iter()
                .map(|doc| doc.line_count)
                .sum::<usize>()
                + side_lines,
            counts: crate::count_findings(&self.findings),
            findings: self.findings,
            tests: self.read.discovered.tally.clone(),
            guides: self.guides,
            surface: self.surface.map(|tally| crate::surface::Unlisted {
                unspecified: tally.unspecified,
            }),
            groups: self.groups,
            test_side: crate::test_side::TestSideFiles::new(
                &self.read.discovered.files,
                &self.read.config().surface.files,
            ),
        });
        Ok(Inspection {
            read: self.read,
            check,
            status,
        })
    }
}

/// Results cannot be assembled until all acquisition phases have been admitted.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum InspectionPhase {
    Guides,
    Surface,
    Complete,
}

pub struct RepositoryInspectionPreparation {
    pub(super) inner: CheckPreparation,
    pub(super) phase: InspectionPhase,
}

impl RepositoryInspectionPreparation {
    pub(super) fn at_phase(&self, phase: InspectionPhase) -> Result<(), crate::StopReason> {
        if self.phase != phase {
            return Err(native_error(&InputError::InvalidInput(
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
    pub fn guides(
        &mut self,
        input: Option<Vec<NativeSourceText>>,
    ) -> Result<(), crate::StopReason> {
        self.at_phase(InspectionPhase::Guides)?;
        let files = texts(
            selected(input, self.inner.read.policy.guides(), "guides")
                .map_err(|error| native_error(&error))?,
        );
        self.inner
            .read
            .originals
            .admit(&files)
            .map_err(|error| native_error(&error))?;
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
                .map_err(|error| native_error(&error))?,
        )
        .map_err(|error| native_error(&error))?;
        let unspecified = texts(
            selected(
                unspecified,
                self.inner.read.policy.unspecified(),
                "unspecified surfaces",
            )
            .map_err(|error| native_error(&error))?,
        );
        self.inner
            .read
            .originals
            .admit(files.iter().map(|file| &file.source))
            .map_err(|error| native_error(&error))?;
        self.inner
            .read
            .originals
            .admit(&unspecified)
            .map_err(|error| native_error(&error))?;
        self.inner.surface(&files, &unspecified)?;
        self.phase = InspectionPhase::Complete;
        Ok(())
    }
    /// 追加の指摘の群を加える（TBL-core-042）。どの段の間でも加えられる
    pub fn group(&mut self, group: FindingGroup) -> Result<(), crate::StopReason> {
        self.inner
            .group(group)
            .map_err(|error| native_error(&error))
    }
    /// `対`を加える。同じ`先頭の言語`の`側`の`対`は1つにする（REQ-core-337）。どの段の間でも加えられる
    pub fn translations(&mut self, pairs: crate::translations::Pairs) {
        self.inner.translations(pairs);
    }
    pub fn finish(self) -> Result<Inspection, crate::StopReason> {
        self.at_phase(InspectionPhase::Complete)?;
        self.inner.finish()
    }
}

pub struct Inspection {
    pub(super) read: ReadModel,
    pub(super) check: CheckReport,
    pub(super) status: StatusReport,
}

impl Inspection {
    pub fn build(inputs: CheckInputs) -> Result<Self, InputError> {
        let policy = Policy::new(&inputs.read.config).map_err(input_stop)?;
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
        let groups = inputs.groups;
        let translations = inputs.translations;
        let mut read = logical_read(inputs.read, policy)?;
        read.originals.admit(&guides)?;
        read.originals
            .admit(surface.iter().map(|file| &file.source))?;
        read.originals.admit(&unspecified)?;
        let mut preparation = CheckPreparation::new(read);
        preparation.guides(&guides).map_err(input_stop)?;
        preparation
            .surface(&surface, &unspecified)
            .map_err(input_stop)?;
        for group in groups {
            preparation.group(group)?;
        }
        preparation.translations(translations);
        preparation.finish().map_err(input_stop)
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
