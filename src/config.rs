use crate::StopReason;
use std::num::NonZeroU64;

/// 設定ファイルの構造（TBL-004）
#[derive(Debug, Clone)]
pub struct Config {
    pub ir: String,
    pub decisions: DecisionsConfig,
    pub tests: TestsConfig,
    pub limits: LimitsConfig,
    pub vague_words: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DecisionsConfig {
    pub records: String,
    pub adr: String,
}

#[derive(Debug, Clone)]
pub struct TestsConfig {
    pub files: Vec<String>,
    pub rust: RustTestsConfig,
}

#[derive(Debug, Clone)]
pub struct RustTestsConfig {
    pub attributes: Vec<String>,
    pub macros: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LimitsConfig {
    pub lines: NonZeroU64,
    pub requirements: NonZeroU64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ir: "docs/ir".to_string(),
            decisions: DecisionsConfig {
                records: "docs/decision/brainstorm".to_string(),
                adr: "docs/decision/adr".to_string(),
            },
            tests: TestsConfig {
                files: vec!["src/**/*.rs".to_string(), "tests/**/*.rs".to_string()],
                rust: RustTestsConfig {
                    attributes: vec![],
                    macros: vec![],
                },
            },
            limits: LimitsConfig {
                lines: NonZeroU64::new(120).unwrap(),
                requirements: NonZeroU64::new(10).unwrap(),
            },
            vague_words: vec![
                "適切に".to_string(),
                "必要に応じて".to_string(),
                "通常は".to_string(),
                "など".to_string(),
            ],
        }
    }
}

/// serde 用の YAML の形（deny_unknown_fields 付き）
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    ir: Option<String>,
    decisions: Option<RawDecisions>,
    tests: Option<RawTests>,
    limits: Option<RawLimits>,
    vague_words: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDecisions {
    records: Option<String>,
    adr: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTests {
    files: Option<Vec<String>>,
    rust: Option<RawRustTests>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRustTests {
    attributes: Option<Vec<String>>,
    macros: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLimits {
    lines: Option<NonZeroU64>,
    requirements: Option<NonZeroU64>,
}

impl Config {
    /// YAML 文字列から設定を読む
    pub fn parse(yaml: &str) -> Result<Self, StopReason> {
        let raw: RawConfig = serde_saphyr::from_str(yaml)
            .map_err(|e| StopReason::ConfigError(format!("{e}")))?;

        let defaults = Config::default();

        let decisions = if let Some(d) = raw.decisions {
            DecisionsConfig {
                records: d.records.unwrap_or(defaults.decisions.records),
                adr: d.adr.unwrap_or(defaults.decisions.adr),
            }
        } else {
            defaults.decisions
        };

        let tests = if let Some(t) = raw.tests {
            let rust = if let Some(r) = t.rust {
                RustTestsConfig {
                    attributes: r.attributes.unwrap_or(defaults.tests.rust.attributes),
                    macros: r.macros.unwrap_or(defaults.tests.rust.macros),
                }
            } else {
                defaults.tests.rust
            };
            TestsConfig {
                // REQ-015: 一覧は既定を置き換える
                files: t.files.unwrap_or(defaults.tests.files),
                rust,
            }
        } else {
            defaults.tests
        };

        let limits = if let Some(l) = raw.limits {
            LimitsConfig {
                lines: l.lines.unwrap_or(defaults.limits.lines),
                requirements: l.requirements.unwrap_or(defaults.limits.requirements),
            }
        } else {
            defaults.limits
        };

        let vague_words = raw.vague_words.unwrap_or(defaults.vague_words);
        if vague_words.iter().any(|w| w.is_empty()) {
            return Err(StopReason::ConfigError(
                "vague_words contains an empty string".to_string(),
            ));
        }

        Ok(Config {
            ir: raw.ir.unwrap_or(defaults.ir),
            decisions,
            tests,
            limits,
            // REQ-015: 一覧は既定を置き換える
            vague_words,
        })
    }
}
