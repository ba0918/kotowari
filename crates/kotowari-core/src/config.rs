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
    pub surface: SurfaceConfig,
    pub changes: Option<ChangesConfig>,
    /// `全体像の元データ`の置き場。鍵が無ければ None で、元データを読まない（TBL-core-004）
    pub overview: Option<OverviewConfig>,
    pub limits: LimitsConfig,
    pub vague_words: Vec<String>,
}

/// `全体像の元データ`の置き場（TBL-core-004）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverviewConfig {
    /// `全体像の元データ`に当たる glob の一覧。"overview" を書くときは必須
    pub files: Vec<String>,
    /// `目次`のファイルのパス（基準のディレクトリからの相対）。"overview" を書くときは必須
    pub toc: String,
}

/// `面`の検査の設定（TBL-core-004、docs/ir/core/surface.md）
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SurfaceConfig {
    /// `面のファイル`に当たる glob の一覧。既定は空の一覧
    pub files: Vec<String>,
    /// `面の規則`の YAML ファイルのパス（基準のディレクトリからの相対）。空の一覧なら面の検査をしない
    pub rules: Vec<String>,
    /// `未記載の面の一覧`のファイルのパス。既定は無く、無ければ一覧は0件（REQ-core-231）
    pub unspecified: Option<String>,
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
            surface: SurfaceConfig::default(),
            changes: None,
            overview: None,
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
    surface: Option<Option<RawSurface>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    limits: Option<Option<RawLimits>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    changes: Option<Option<ChangesConfig>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    overview: Option<Option<RawOverview>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    vague_words: Option<Option<Vec<String>>>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawOverview {
    #[serde(default, deserialize_with = "deserialize_nullable")]
    files: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    toc: Option<Option<String>>,
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
struct RawSurface {
    #[serde(default, deserialize_with = "deserialize_nullable")]
    files: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    rules: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    unspecified: Option<Option<String>>,
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

/// REQ-core-014: glob として読めない要素があれば設定の誤りで停止する（"tests.files"、"guides.files"、"surface.files"）
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

/// 一覧のファイル（`等価の一覧`と`未記載の面の一覧`）の中身を、最上位の並びの要素として読む。
/// 空（0バイトか注釈だけ）なら0件。YAML として読めないか最上位が並びでないときは、
/// 一覧のファイルの相対パス `display` を詳細にして設定の誤りで`停止`する（REQ-core-148、REQ-core-231）
pub(crate) fn read_yaml_sequence(
    text: &str,
    display: &str,
) -> Result<Vec<serde_json::Value>, StopReason> {
    if is_blank_yaml(text) {
        return Ok(Vec::new());
    }
    let root: serde_json::Value = serde_saphyr::from_str(text)
        .map_err(|e| StopReason::ConfigError(format!("{display}: {e}")))?;
    match root {
        serde_json::Value::Array(items) => Ok(items),
        _ => Err(StopReason::ConfigError(format!(
            "{display}: the list is not a sequence"
        ))),
    }
}

/// YAML のライブラリの誤りを設定の誤りにする。キーの重複はライブラリの文言と抜粋を出さず、
/// "duplicate key: キー" の1行にする（REQ-core-014）
fn yaml_error(e: serde_saphyr::Error) -> StopReason {
    match e.without_snippet() {
        serde_saphyr::Error::DuplicateMappingKey { key: Some(key), .. } => {
            StopReason::ConfigError(format!("duplicate key: {key}"))
        }
        _ => StopReason::ConfigError(format!("{e}")),
    }
}

/// "surface" の鍵を読む。null、絶対パス、glob として読めない要素（REQ-core-014）と、
/// 鍵の組み合わせの誤り（REQ-core-225）は設定の誤りで停止する
fn read_surface(raw: RawSurface) -> Result<SurfaceConfig, StopReason> {
    let files = non_null_or_default(raw.files, "surface.files", Vec::new())?;
    check_globs(&files)?;
    let rules = non_null_or_default(raw.rules, "surface.rules", Vec::new())?;
    for rule in &rules {
        check_not_absolute(rule, "surface.rules")?;
    }
    let unspecified = non_null(raw.unspecified, "surface.unspecified")?;
    if let Some(path) = &unspecified {
        check_not_absolute(path, "surface.unspecified")?;
    }
    // REQ-core-225: 規則だけ、ファイルだけ、規則の無い一覧は、検査しているつもりで何もしない
    if rules.is_empty() && !files.is_empty() {
        return Err(StopReason::ConfigError(
            "surface.files is set but surface.rules is empty".to_string(),
        ));
    }
    if files.is_empty() && !rules.is_empty() {
        return Err(StopReason::ConfigError(
            "surface.rules is set but surface.files is empty".to_string(),
        ));
    }
    if rules.is_empty() && unspecified.is_some() {
        return Err(StopReason::ConfigError(
            "surface.unspecified is set but surface.rules is empty".to_string(),
        ));
    }
    Ok(SurfaceConfig {
        files,
        // REQ-core-110: パスの正規化
        rules: rules
            .iter()
            .map(|rule| crate::normalize_path(rule))
            .collect(),
        unspecified: unspecified.map(|path| crate::normalize_path(&path)),
    })
}

impl Config {
    /// YAML 文字列から設定を読む
    pub fn parse(yaml: &str) -> Result<Self, StopReason> {
        // REQ-core-012: 空の設定ファイルは既定値
        if is_blank_yaml(yaml) {
            return Ok(Config::default());
        }

        let raw: RawConfig = serde_saphyr::from_str(yaml).map_err(yaml_error)?;
        Self::from_raw(raw)
    }

    pub(crate) fn validated(&self) -> Result<Self, StopReason> {
        Self::from_raw(RawConfig {
            ir: Some(Some(self.ir.clone())),
            decisions: Some(Some(RawDecisions {
                records: Some(Some(self.decisions.records.clone())),
                adr: Some(Some(self.decisions.adr.clone())),
            })),
            tests: Some(Some(RawTests {
                files: Some(Some(self.tests.files.clone())),
                rules: Some(Some(self.tests.rules.clone())),
                rust: Some(Some(RawRustTests {
                    attributes: Some(Some(self.tests.rust.attributes.clone())),
                    macros: Some(Some(self.tests.rust.macros.clone())),
                })),
            })),
            guides: Some(Some(RawGuides {
                files: Some(Some(self.guides.files.clone())),
            })),
            mutants: Some(Some(RawMutants {
                equivalents: self.mutants.equivalents.clone().map(Some),
            })),
            surface: Some(Some(RawSurface {
                files: Some(Some(self.surface.files.clone())),
                rules: Some(Some(self.surface.rules.clone())),
                unspecified: self.surface.unspecified.clone().map(Some),
            })),
            changes: self.changes.clone().map(Some),
            overview: self.overview.as_ref().map(|overview| {
                Some(RawOverview {
                    files: Some(Some(overview.files.clone())),
                    toc: Some(Some(overview.toc.clone())),
                })
            }),
            limits: Some(Some(RawLimits {
                lines: Some(Some(self.limits.lines)),
                requirements: Some(Some(self.limits.requirements)),
            })),
            vague_words: Some(Some(self.vague_words.clone())),
        })
    }

    fn from_raw(raw: RawConfig) -> Result<Self, StopReason> {
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

        let changes = non_null(raw.changes, "changes")?;
        if let Some(c) = &changes {
            if c.files.is_empty() || c.records.is_empty() {
                return Err(StopReason::ConfigError(
                    "changes.files and changes.records must be nonempty".into(),
                ));
            }
            for patterns in [&c.files, &c.exclude, &c.records] {
                check_globs(patterns)?;
                for pattern in patterns {
                    check_not_absolute(pattern, "changes")?;
                    if pattern.is_empty() {
                        return Err(StopReason::ConfigError("empty changes glob".into()));
                    }
                }
            }
        }

        // TBL-core-004: "overview" を書くときは "overview.files" と "overview.toc" が必須。glob と
        // パスは REQ-core-014 のとおり検査する
        let overview = match non_null(raw.overview, "overview")? {
            Some(o) => {
                let files = non_null(o.files, "overview.files")?.ok_or_else(|| {
                    StopReason::ConfigError("overview.files is required".to_string())
                })?;
                check_globs(&files)?;
                let toc = non_null(o.toc, "overview.toc")?.ok_or_else(|| {
                    StopReason::ConfigError("overview.toc is required".to_string())
                })?;
                check_not_absolute(&toc, "overview.toc")?;
                Some(OverviewConfig {
                    files,
                    toc: crate::normalize_path(&toc),
                })
            }
            None => None,
        };

        let surface = match non_null(raw.surface, "surface")? {
            Some(s) => read_surface(s)?,
            None => defaults.surface,
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
            surface,
            changes,
            overview,
            limits,
            // REQ-core-015: 一覧は既定を置き換える
            vague_words,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangesConfig {
    #[serde(deserialize_with = "required_list")]
    pub files: Vec<String>,
    #[serde(deserialize_with = "required_list")]
    pub records: Vec<String>,
    #[serde(default, deserialize_with = "required_list")]
    pub exclude: Vec<String>,
}

fn required_list<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    Option::<Vec<String>>::deserialize(d)?.ok_or_else(|| serde::de::Error::custom("null list"))
}
