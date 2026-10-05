#![deny(clippy::print_stdout, clippy::print_stderr)]
mod comment_block;
mod discovery;
mod test_queries;

use kotowari_core::{Finding, FindingKind, config::Config};
pub use kotowari_core::{SourceText, SurfaceAnalysis, TestAnalysis};

#[derive(Debug)]
#[non_exhaustive]
pub enum AnalysisError {
    InvalidRules(String),
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRules(detail) => write!(f, "invalid rules: {detail}"),
        }
    }
}
impl std::error::Error for AnalysisError {}
impl AnalysisError {
    pub fn detail(&self) -> &str {
        match self {
            Self::InvalidRules(detail) => detail.strip_prefix("config error: ").unwrap_or(detail),
        }
    }
}

pub struct Analyzer {
    config: Config,
    tests: test_queries::TestQueries,
    surfaces: test_queries::RuleSet,
}

impl Analyzer {
    pub fn supports_surfaces(&self, path: &str) -> bool {
        test_queries::language_of(path).is_some_and(|lang| self.surfaces.has_language(lang))
    }
    #[expect(
        clippy::needless_pass_by_value,
        reason = "public API: changing the parameter type breaks callers"
    )]
    pub fn new(
        config: Config,
        test_rules: Vec<SourceText>,
        surface_rules: Vec<SourceText>,
    ) -> Result<Self, AnalysisError> {
        let tests = test_queries::TestQueries::from_texts(&config, &test_rules)
            .map_err(|error| AnalysisError::InvalidRules(error.to_string()))?;
        let surfaces = test_queries::RuleSet::from_texts(&surface_rules, "surface.rules")
            .map_err(|error| AnalysisError::InvalidRules(error.to_string()))?;
        Ok(Self {
            config,
            tests,
            surfaces,
        })
    }

    pub fn tests(&self, source: SourceText) -> Result<TestAnalysis, AnalysisError> {
        let path = source.path().to_owned();
        self.tests_at(source, &path)
    }

    #[doc(hidden)]
    pub fn tests_at(
        &self,
        source: SourceText,
        acquired_path: &str,
    ) -> Result<TestAnalysis, AnalysisError> {
        let content = discovery::lone_cr_to_lf(source.text());
        let language =
            test_queries::language_of(acquired_path).filter(|lang| self.tests.has_query(*lang));
        let mut result = TestAnalysis {
            language: language.map(|lang| format!("{lang:?}")),
            has_query: language.is_some(),
            source,
            tests: vec![],
            line_markers: vec![],
            findings: vec![],
        };
        if let Some(language) = language {
            match discovery::discover_tests(
                &content,
                acquired_path,
                language,
                &self.tests,
                &self.config,
            ) {
                Ok(tests) => result.tests = tests,
                Err(_) => result.findings.push(Finding::new(
                    FindingKind::UnparsableFile,
                    acquired_path.into(),
                    None,
                    acquired_path.into(),
                )),
            }
        } else {
            result.line_markers = content
                .lines()
                .enumerate()
                .flat_map(|(index, line)| {
                    kotowari_core::tests_discovery::parse_markers_in_line(line, index + 1)
                })
                .collect();
        }
        Ok(result)
    }

    pub fn surfaces(&self, source: SourceText) -> Result<SurfaceAnalysis, AnalysisError> {
        let path = source.path().to_owned();
        self.surfaces_at(source, &path)
    }

    #[doc(hidden)]
    pub fn surfaces_at(
        &self,
        source: SourceText,
        acquired_path: &str,
    ) -> Result<SurfaceAnalysis, AnalysisError> {
        let language = test_queries::language_of(acquired_path)
            .filter(|lang| self.surfaces.has_language(*lang));
        let mut result = SurfaceAnalysis {
            language: language.map(|lang| format!("{lang:?}")),
            has_query: language.is_some(),
            source,
            surfaces: vec![],
            findings: vec![],
        };
        if let Some(language) = language {
            let content = discovery::lone_cr_to_lf(result.source.text());
            if let Some(parsed) = test_queries::ParsedFile::parse(&content, language) {
                result.surfaces = parsed
                    .find_named(&self.surfaces, language, acquired_path)
                    .into_iter()
                    .map(|item| kotowari_core::surface::Surface {
                        kind: item.rule_id,
                        name: item.name,
                        path: acquired_path.into(),
                        line: item.line,
                    })
                    .collect();
            } else {
                result.findings.push(Finding::new(
                    FindingKind::UnparsableFile,
                    acquired_path.into(),
                    None,
                    acquired_path.into(),
                ));
            }
        }
        Ok(result)
    }
}
