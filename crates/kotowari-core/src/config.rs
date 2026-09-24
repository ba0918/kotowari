use crate::StopReason;
use std::num::NonZeroU64;

/// 設定ファイルの構造（TBL-core-004）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub ir: String,
    pub decisions: DecisionsConfig,
    pub tests: TestsConfig,
    pub guides: GuidesConfig,
    pub mutants: MutantsConfig,
    pub limits: LimitsConfig,
    pub vague_words: Vec<String>,
}

/// `ガイド`の置き場（TBL-core-004、REQ-core-198）
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GuidesConfig {
    /// `ガイド`に当たる glob の一覧。既定は空の一覧で、空なら`ガイド`を1つも読まない
    pub files: Vec<String>,
}

/// 変異テストに関わる設定（TBL-core-004）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantsConfig {
    /// `等価の一覧`のファイルのパス。既定は無く、無ければ一覧は0件（REQ-core-148）
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
    /// ast-grep のルールの YAML ファイルのパス（基準のディレクトリからの相対、REQ-core-186）
    pub rules: Vec<String>,
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
                rules: vec![],
            },
            guides: GuidesConfig::default(),
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
    guides: Option<Option<RawGuides>>,
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
    #[serde(default, deserialize_with = "deserialize_nullable")]
    rules: Option<Option<Vec<String>>>,
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
struct RawGuides {
    #[serde(default, deserialize_with = "deserialize_nullable")]
    files: Option<Option<Vec<String>>>,
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
    #[serde(default, deserialize_with = "deserialize_nullable")]
    lines: Option<Option<NonZeroU64>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
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

use serde::Deserialize;

/// キーの値を取り出す。キーが無ければ既定値にし、値が null なら設定の誤りで停止する（REQ-core-014）
fn non_null_or_default<T>(
    field: Option<Option<T>>,
    key: &str,
    default: T,
) -> Result<T, StopReason> {
    Ok(non_null(field, key)?.unwrap_or(default))
}

/// null チェック付きで、入れ子のキーを (キー不在 → None、値あり → Some(v)) にする。
/// 値が null なら設定の誤りで停止する（REQ-core-014）。
fn non_null<T>(field: Option<Option<T>>, key: &str) -> Result<Option<T>, StopReason> {
    match field {
        None => Ok(None),
        Some(None) => Err(StopReason::ConfigError(format!(
            "null value for key: {key}"
        ))),
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

/// REQ-core-014: glob として読めない要素があれば設定の誤りで停止する（"tests.files" と "guides.files"）
fn check_globs(patterns: &[String]) -> Result<(), StopReason> {
    for pattern in patterns {
        if globset::Glob::new(pattern).is_err() {
            return Err(StopReason::ConfigError(format!(
                "invalid glob pattern: {pattern}"
            )));
        }
    }
    Ok(())
}

/// 中身が空（0バイトか注釈だけ）の YAML か（REQ-core-012 の設定ファイル、REQ-core-148 の等価の一覧）。
/// 空行は内容に数えないので、空行と注釈の行だけのファイルは空である。
/// 1行も無いファイル（0バイト）では `all` が真になる。
pub fn is_blank_yaml(text: &str) -> bool {
    text.lines().all(|line| {
        let trimmed = line.trim();
        trimmed.is_empty() || trimmed.starts_with('#')
    })
}

impl Config {
    /// YAML 文字列から設定を読む
    pub fn parse(yaml: &str) -> Result<Self, StopReason> {
        // REQ-core-012: 空の設定ファイルは既定値
        if is_blank_yaml(yaml) {
            return Ok(Config::default());
        }

        let raw: RawConfig =
            serde_saphyr::from_str(yaml).map_err(|e| StopReason::ConfigError(format!("{e}")))?;

        let defaults = Config::default();

        // REQ-core-014: null 値の検出と絶対パスの検出
        // REQ-core-110: パスの正規化
        let ir = non_null_or_default(raw.ir, "ir", defaults.ir)?;
        check_not_absolute(&ir, "ir")?;
        let ir = crate::normalize_path(&ir);

        // REQ-core-014: "decisions:" 自体が null のときも設定の誤りで停止する
        let decisions = match non_null(raw.decisions, "decisions")? {
            Some(d) => {
                let records = non_null_or_default(
                    d.records,
                    "decisions.records",
                    defaults.decisions.records,
                )?;
                check_not_absolute(&records, "decisions.records")?;
                let records = crate::normalize_path(&records);
                let adr = non_null_or_default(d.adr, "decisions.adr", defaults.decisions.adr)?;
                check_not_absolute(&adr, "decisions.adr")?;
                let adr = crate::normalize_path(&adr);
                DecisionsConfig { records, adr }
            }
            None => defaults.decisions,
        };

        // REQ-core-014: "tests:" と "tests.rust:" 自体、"tests.files"、
        // "tests.rust.attributes"、"tests.rust.macros"、"tests.rules" が null のときも停止する
        let tests = match non_null(raw.tests, "tests")? {
            Some(t) => {
                let rust = match non_null(t.rust, "tests.rust")? {
                    Some(r) => RustTestsConfig {
                        attributes: non_null_or_default(
                            r.attributes,
                            "tests.rust.attributes",
                            defaults.tests.rust.attributes,
                        )?,
                        macros: non_null_or_default(
                            r.macros,
                            "tests.rust.macros",
                            defaults.tests.rust.macros,
                        )?,
                    },
                    None => defaults.tests.rust,
                };
                let files = non_null_or_default(t.files, "tests.files", defaults.tests.files)?;
                check_globs(&files)?;
                let rules = non_null_or_default(t.rules, "tests.rules", defaults.tests.rules)?;
                for rule in &rules {
                    check_not_absolute(rule, "tests.rules")?;
                }
                TestsConfig {
                    // REQ-core-015: 一覧は既定を置き換える
                    files,
                    rust,
                    // REQ-core-110: パスの正規化
                    rules: rules
                        .iter()
                        .map(|rule| crate::normalize_path(rule))
                        .collect(),
                }
            }
            None => defaults.tests,
        };

        // REQ-core-014: "guides:" 自体と "guides.files" が null のときも停止する
        let guides = match non_null(raw.guides, "guides")? {
            Some(g) => {
                let files = non_null_or_default(g.files, "guides.files", defaults.guides.files)?;
                check_globs(&files)?;
                GuidesConfig { files }
            }
            None => defaults.guides,
        };

        // REQ-core-014: "mutants:" 自体と "mutants.equivalents" が null のときも停止する
        let mutants = match non_null(raw.mutants, "mutants")? {
            Some(m) => {
                let equivalents = match non_null(m.equivalents, "mutants.equivalents")? {
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

        // REQ-core-014: "limits:" 自体が null のときも停止する
        let limits = match non_null(raw.limits, "limits")? {
            Some(l) => {
                let lines = non_null_or_default(l.lines, "limits.lines", defaults.limits.lines)?;
                let requirements = non_null_or_default(
                    l.requirements,
                    "limits.requirements",
                    defaults.limits.requirements,
                )?;
                LimitsConfig {
                    lines,
                    requirements,
                }
            }
            None => defaults.limits,
        };

        // REQ-core-014: "vague_words:" 自体が null のときも停止する
        let vague_words =
            non_null_or_default(raw.vague_words, "vague_words", defaults.vague_words)?;
        if vague_words.iter().any(|w| w.is_empty()) {
            return Err(StopReason::ConfigError(
                "vague_words contains an empty string".to_string(),
            ));
        }
        // REQ-core-014: 重複語の検出
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
            guides,
            mutants,
            limits,
            // REQ-core-015: 一覧は既定を置き換える
            vague_words,
        })
    }
}
