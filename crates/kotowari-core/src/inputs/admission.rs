//! 入力の受け入れ。要る群が揃っているか、同じ入力が重ならないかを見る（REQ-core-315）

use super::*;

pub(super) struct Policy {
    pub(super) config: crate::config::Config,
}

impl Policy {
    pub(super) fn new(config: &crate::config::Config) -> Result<Self, crate::StopReason> {
        Ok(Self {
            config: config.validated()?,
        })
    }
    pub(super) fn tests(&self) -> bool {
        !self.config.tests.files.is_empty()
    }
    pub(super) fn guides(&self) -> bool {
        !self.config.guides.files.is_empty()
    }
    pub(super) fn surface(&self) -> bool {
        !self.config.surface.rules.is_empty()
    }
    pub(super) fn unspecified(&self) -> bool {
        self.surface() && self.config.surface.unspecified.is_some()
    }
    /// REQ-core-334: 言語が1つなら`対`を読まない
    pub(super) fn translations(&self) -> bool {
        self.config.languages().len() > 1
    }
}

pub(super) fn selected<T>(
    input: Option<Vec<T>>,
    enabled: bool,
    name: &str,
) -> Result<Vec<T>, InputError> {
    if !enabled {
        return Ok(vec![]);
    }
    input.ok_or_else(|| InputError::InputMissing(name.into()))
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Identity {
    Logical(String),
    Native(PathBuf),
}

pub(super) struct Text {
    pub(super) identity: Identity,
    pub(super) path: String,
    pub(super) text: Arc<str>,
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
pub(super) struct Originals(BTreeMap<Identity, Arc<str>>);

impl Originals {
    pub(super) fn admit<'a>(
        &mut self,
        sources: impl IntoIterator<Item = &'a Text>,
    ) -> Result<(), InputError> {
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

pub(super) struct Test {
    pub(super) source: Text,
    pub(super) analysis: TestAnalysis,
}

pub(super) struct Surface {
    pub(super) source: Text,
    pub(super) analysis: SurfaceAnalysis,
}

pub(super) fn logical_tests(files: Vec<TestAnalysis>) -> Vec<Test> {
    files
        .into_iter()
        .map(|analysis| Test {
            source: analysis.source.clone().into(),
            analysis,
        })
        .collect()
}

pub(super) fn logical_surfaces(files: Vec<SurfaceAnalysis>) -> Vec<Surface> {
    files
        .into_iter()
        .map(|analysis| Surface {
            source: analysis.source.clone().into(),
            analysis,
        })
        .collect()
}

pub(super) fn native_tests(files: Vec<NativeTestAnalysis>) -> Result<Vec<Test>, InputError> {
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

pub(super) fn native_surfaces(
    files: Vec<NativeSurfaceAnalysis>,
) -> Result<Vec<Surface>, InputError> {
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

pub(super) fn original_analysis(
    source: &NativeSourceText,
    analysis: &SourceText,
) -> Result<(), InputError> {
    if source.text() != analysis.text() {
        return Err(InputError::InvalidInput(format!(
            "different analysis source text for {}",
            source.path()
        )));
    }
    Ok(())
}

pub(super) fn texts<T: Into<Text>>(sources: Vec<T>) -> Vec<Text> {
    sources.into_iter().map(Into::into).collect()
}

pub(super) fn entries(sources: &[Text]) -> impl Iterator<Item = (&str, &str)> {
    sources
        .iter()
        .map(|source| (source.path.as_str(), source.text.as_ref()))
}
