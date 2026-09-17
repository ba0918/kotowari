use crate::StopReason;
use std::num::NonZeroU64;

/// 設定ファイルの構造（TBL-004）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub ir: String,
    pub decisions: DecisionsConfig,
    pub tests: TestsConfig,
    pub mutants: MutantsConfig,
    pub limits: LimitsConfig,
    pub vague_words: Vec<String>,
}

/// 変異テストに関わる設定（TBL-004）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantsConfig {
    /// `等価の一覧`のファイルのパス。既定は無く、無ければ一覧は0件（REQ-148）
    pub equivalents: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionsConfig {
    pub records: String,
    pub adr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestsConfig {
    pub files: Vec<String>,
    pub rust: RustTestsConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTestsConfig {
    pub attributes: Vec<String>,
    pub macros: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimitsConfig {
    pub lines: NonZeroU64,
    pub requirements: NonZeroU64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ir: "docs/ir".to_string(),
            decisions: DecisionsConfig {
                records: "docs/decision/records".to_string(),
                adr: "docs/decision/adr".to_string(),
            },
            tests: TestsConfig {
                files: vec!["src/**/*.rs".to_string(), "tests/**/*.rs".to_string()],
                rust: RustTestsConfig {
                    attributes: vec![],
                    macros: vec![],
                },
            },
            mutants: MutantsConfig { equivalents: None },
            limits: LimitsConfig {
                lines: NonZeroU64::new(200).unwrap(),
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
    #[serde(default, deserialize_with = "deserialize_nullable")]
    decisions: Option<Option<RawDecisions>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    tests: Option<Option<RawTests>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    mutants: Option<Option<RawMutants>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    limits: Option<Option<RawLimits>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    vague_words: Option<Option<Vec<String>>>,
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
    #[serde(default, deserialize_with = "deserialize_nullable")]
    files: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    rust: Option<Option<RawRustTests>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRustTests {
    #[serde(default, deserialize_with = "deserialize_nullable")]
    attributes: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    macros: Option<Option<Vec<String>>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMutants {
    #[serde(default, deserialize_with = "deserialize_nullable")]
    equivalents: Option<Option<String>>,
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

/// null チェック付きで、入れ子のキーを (キー不在 → None、値あり → Some(v)) にする。
/// 値が null なら設定の誤りで停止する（REQ-014）。
fn unwrap_or_null_option<T>(field: Option<Option<T>>, key: &str) -> Result<Option<T>, StopReason> {
    match field {
        None => Ok(None),
        Some(None) => Err(StopReason::ConfigError(format!("null value for key: {key}"))),
        Some(Some(v)) => Ok(Some(v)),
    }
}

/// パスが絶対パスでないことを検証する
fn check_not_absolute(path: &str, key: &str) -> Result<(), StopReason> {
    // A161: 絶対パスかどうかは先頭の "/" だけで判定する（Windows のドライブ文字は見ない）
    if path.starts_with('/') {
        return Err(StopReason::ConfigError(format!(
            "absolute path not allowed for {key}: {path}"
        )));
    }
    Ok(())
}

/// 中身が空（0バイトか注釈だけ）の YAML か（REQ-012 の設定ファイル、REQ-148 の等価の一覧）
pub fn is_blank_yaml(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.is_empty() || trimmed.lines().all(|l| l.trim_start().starts_with('#'))
}

impl Config {
    /// YAML 文字列から設定を読む
    pub fn parse(yaml: &str) -> Result<Self, StopReason> {
        // REQ-012: 空の設定ファイルは既定値
        if is_blank_yaml(yaml) {
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

        // REQ-014: "decisions:" 自体が null のときも設定の誤りで停止する
        let decisions = match unwrap_or_null_option(raw.decisions, "decisions")? {
            Some(d) => {
                let records = unwrap_or_null(d.records, "decisions.records", defaults.decisions.records)?;
                check_not_absolute(&records, "decisions.records")?;
                let records = crate::normalize_path(&records);
                let adr = unwrap_or_null(d.adr, "decisions.adr", defaults.decisions.adr)?;
                check_not_absolute(&adr, "decisions.adr")?;
                let adr = crate::normalize_path(&adr);
                DecisionsConfig { records, adr }
            }
            None => defaults.decisions,
        };

        // REQ-014: "tests:" と "tests.rust:" 自体、"tests.files"、
        // "tests.rust.attributes"、"tests.rust.macros" が null のときも停止する
        let tests = match unwrap_or_null_option(raw.tests, "tests")? {
            Some(t) => {
                let rust = match unwrap_or_null_option(t.rust, "tests.rust")? {
                    Some(r) => RustTestsConfig {
                        attributes: unwrap_or_null(
                            r.attributes,
                            "tests.rust.attributes",
                            defaults.tests.rust.attributes,
                        )?,
                        macros: unwrap_or_null(
                            r.macros,
                            "tests.rust.macros",
                            defaults.tests.rust.macros,
                        )?,
                    },
                    None => defaults.tests.rust,
                };
                let files = unwrap_or_null(t.files, "tests.files", defaults.tests.files)?;
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
            }
            None => defaults.tests,
        };

        // REQ-014: "mutants:" 自体と "mutants.equivalents" が null のときも停止する
        let mutants = match unwrap_or_null_option(raw.mutants, "mutants")? {
            Some(m) => {
                let equivalents = match unwrap_or_null_option(m.equivalents, "mutants.equivalents")?
                {
                    Some(path) => {
                        check_not_absolute(&path, "mutants.equivalents")?;
                        Some(crate::normalize_path(&path))
                    }
                    None => None,
                };
                MutantsConfig { equivalents }
            }
            None => defaults.mutants,
        };

        // REQ-014: "limits:" 自体が null のときも停止する
        let limits = match unwrap_or_null_option(raw.limits, "limits")? {
            Some(l) => {
                let lines = unwrap_or_null(l.lines, "limits.lines", defaults.limits.lines)?;
                let requirements = unwrap_or_null(l.requirements, "limits.requirements", defaults.limits.requirements)?;
                LimitsConfig { lines, requirements }
            }
            None => defaults.limits,
        };

        // REQ-014: "vague_words:" 自体が null のときも停止する
        let vague_words = unwrap_or_null(raw.vague_words, "vague_words", defaults.vague_words)?;
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
            mutants,
            limits,
            // REQ-015: 一覧は既定を置き換える
            vague_words,
        })
    }
}
