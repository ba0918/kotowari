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
/// Option<Option<T>> で、キー不在（None）と null（Some(None)）を区別する
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    #[serde(default, deserialize_with = "deserialize_nullable")]
    ir: Option<Option<String>>,
    decisions: Option<RawDecisions>,
    tests: Option<RawTests>,
    limits: Option<RawLimits>,
    vague_words: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDecisions {
    #[serde(default, deserialize_with = "deserialize_nullable")]
    records: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    adr: Option<Option<String>>,
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
    #[serde(default, deserialize_with = "deserialize_nullable_nonzero")]
    lines: Option<Option<NonZeroU64>>,
    #[serde(default, deserialize_with = "deserialize_nullable_nonzero")]
    requirements: Option<Option<NonZeroU64>>,
}

/// null を Some(None)、値を Some(Some(v))、不在を None にデシリアライズ
fn deserialize_nullable<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(Some(opt))
}

fn deserialize_nullable_nonzero<'de, D>(deserializer: D) -> Result<Option<Option<NonZeroU64>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<NonZeroU64>::deserialize(deserializer)?;
    Ok(Some(opt))
}

use serde::Deserialize;

/// null チェック付きで Option<Option<T>> から値を取り出す
fn unwrap_or_null<T>(field: Option<Option<T>>, key: &str, default: T) -> Result<T, StopReason> {
    match field {
        None => Ok(default),           // キー不在 → 既定値
        Some(None) => Err(StopReason::ConfigError(format!("null value for key: {key}"))),
        Some(Some(v)) => Ok(v),
    }
}

/// パスが絶対パスでないことを検証する
fn check_not_absolute(path: &str, key: &str) -> Result<(), StopReason> {
    if path.starts_with('/') || (path.len() >= 2 && path.as_bytes()[1] == b':') {
        return Err(StopReason::ConfigError(format!(
            "absolute path not allowed for {key}: {path}"
        )));
    }
    Ok(())
}

impl Config {
    /// YAML 文字列から設定を読む
    pub fn parse(yaml: &str) -> Result<Self, StopReason> {
        // REQ-012: 空の設定ファイルは既定値
        let trimmed = yaml.trim();
        if trimmed.is_empty() || trimmed.lines().all(|l| l.trim_start().starts_with('#')) {
            return Ok(Config::default());
        }

        let raw: RawConfig = serde_saphyr::from_str(yaml)
            .map_err(|e| StopReason::ConfigError(format!("{e}")))?;

        let defaults = Config::default();

        // REQ-014: null 値の検出と絶対パスの検出
        // REQ-110: パスの正規化
        let ir = unwrap_or_null(raw.ir, "ir", defaults.ir)?;
        check_not_absolute(&ir, "ir")?;
        let ir = crate::normalize_path(&ir);

        let decisions = if let Some(d) = raw.decisions {
            let records = unwrap_or_null(d.records, "decisions.records", defaults.decisions.records)?;
            check_not_absolute(&records, "decisions.records")?;
            let records = crate::normalize_path(&records);
            let adr = unwrap_or_null(d.adr, "decisions.adr", defaults.decisions.adr)?;
            check_not_absolute(&adr, "decisions.adr")?;
            let adr = crate::normalize_path(&adr);
            DecisionsConfig { records, adr }
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
            let files = t.files.unwrap_or(defaults.tests.files);
            // REQ-014: glob として読めない要素
            for pattern in &files {
                if globset::Glob::new(pattern).is_err() {
                    return Err(StopReason::ConfigError(format!(
                        "invalid glob pattern: {pattern}"
                    )));
                }
            }
            TestsConfig {
                // REQ-015: 一覧は既定を置き換える
                files,
                rust,
            }
        } else {
            defaults.tests
        };

        let limits = if let Some(l) = raw.limits {
            let lines = unwrap_or_null(l.lines, "limits.lines", defaults.limits.lines)?;
            let requirements = unwrap_or_null(l.requirements, "limits.requirements", defaults.limits.requirements)?;
            LimitsConfig { lines, requirements }
        } else {
            defaults.limits
        };

        let vague_words = raw.vague_words.unwrap_or(defaults.vague_words);
        if vague_words.iter().any(|w| w.is_empty()) {
            return Err(StopReason::ConfigError(
                "vague_words contains an empty string".to_string(),
            ));
        }
        // REQ-014: 重複語の検出
        {
            let mut seen = std::collections::HashSet::new();
            for word in &vague_words {
                if !seen.insert(word) {
                    return Err(StopReason::ConfigError(format!(
                        "duplicate vague_word: {word}"
                    )));
                }
            }
        }

        Ok(Config {
            ir,
            decisions,
            tests,
            limits,
            // REQ-015: 一覧は既定を置き換える
            vague_words,
        })
    }
}
