use crate::StopReason;
use std::collections::BTreeMap;

/// 設定ファイルの構造
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub decisions: DecisionsConfig,
    pub tests: TestsConfig,
    pub guides: GuidesConfig,
    pub mutants: MutantsConfig,
    pub surface: SurfaceConfig,
    /// `全体像の元データ`の置き場。鍵が無ければ None で、元データを読まない
    pub overview: Option<OverviewConfig>,
    pub vague_words: Vec<String>,
    /// "languages" に書いた言語タグ。鍵が無ければ空（`言語の一覧`は "en" だけ）
    pub languages: Vec<String>,
    /// "labels" の言語タグから、`UI の文字`の鍵から文字列への対応
    pub labels: BTreeMap<String, BTreeMap<String, String>>,
}

/// `全体像の元データ`の置き場
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverviewConfig {
    /// `全体像の元データ`に当たる glob の一覧。"overview" を書くときは必須
    pub files: Vec<String>,
    /// `目次`のファイルのパス（基準のディレクトリからの相対）。"overview" を書くときは必須
    pub toc: String,
}

/// `面`の検査の設定（docs/decisions/core/surface.md）
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SurfaceConfig {
    /// `面のファイル`に当たる glob の一覧。既定は空の一覧
    pub files: Vec<String>,
    /// `面の規則`の YAML ファイルのパス（基準のディレクトリからの相対）。空の一覧なら面の検査をしない
    pub rules: Vec<String>,
    /// `未記載の面の一覧`のファイルのパス。既定は無く、無ければ一覧は0件
    pub unspecified: Option<String>,
}

/// `ガイド`の置き場
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GuidesConfig {
    /// `ガイド`に当たる glob の一覧。既定は空の一覧で、空なら`ガイド`を1つも読まない
    pub files: Vec<String>,
}

/// 変異テストに関わる設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantsConfig {
    /// `等価の一覧`のファイルのパス。既定は無く、無ければ一覧は0件
    pub equivalents: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionsConfig {
    /// 決定の文書の置き場（docs/decisions/core/config.md）。既定は "docs/decisions"
    pub documents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestsConfig {
    pub files: Vec<String>,
    pub rust: RustTestsConfig,
    /// ast-grep のルールの YAML ファイルのパス（基準のディレクトリからの相対）
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTestsConfig {
    pub attributes: Vec<String>,
    pub macros: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            decisions: DecisionsConfig {
                documents: "docs/decisions".to_string(),
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
            overview: None,
            vague_words: vec![
                "適切に".to_string(),
                "必要に応じて".to_string(),
                "通常は".to_string(),
                "など".to_string(),
            ],
            languages: Vec::new(),
            labels: BTreeMap::new(),
        }
    }
}

/// serde 用の YAML の形（deny_unknown_fields 付き）
/// Option<Option<T>> で、キー不在（None）と null（Some(None)）を区別する
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
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
    overview: Option<Option<RawOverview>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    vague_words: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    languages: Option<Option<Vec<String>>>,
    #[serde(default, deserialize_with = "deserialize_nullable")]
    labels: Option<Option<BTreeMap<String, BTreeMap<String, String>>>>,
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
    documents: Option<Option<String>>,
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

/// キーの値を取り出す。キーが無ければ既定値にし、値が null なら設定の誤りで停止する
fn non_null_or_default<T>(
    field: Option<Option<T>>,
    key: &str,
    default: T,
) -> Result<T, StopReason> {
    Ok(non_null(field, key)?.unwrap_or(default))
}

/// null チェック付きで、入れ子のキーを (キー不在 → None、値あり → Some(v)) にする。
/// 値が null なら設定の誤りで停止する。
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
    // 絶対パスかどうかは先頭の "/" だけで判定する（Windows のドライブ文字は見ない）
    if path.starts_with('/') {
        return Err(StopReason::ConfigError(format!(
            "absolute path not allowed for {key}: {path}"
        )));
    }
    Ok(())
}

/// glob として読めない要素か "/" で始まる要素があれば設定の誤りで停止する
/// （"tests.files"、"guides.files"、"surface.files"、"overview.files" の glob）
fn check_globs(patterns: &[String], key: &str) -> Result<(), StopReason> {
    for pattern in patterns {
        check_not_absolute(pattern, key)?;
        if globset::Glob::new(pattern).is_err() {
            return Err(StopReason::ConfigError(format!(
                "invalid glob pattern: {pattern}"
            )));
        }
    }
    Ok(())
}

/// glob として読めると確かめた一覧を1つの集まりにする。設定の glob は `check_globs` が確かめ、
/// それ以外は呼び出し側が確かめてから渡す
pub fn glob_set(patterns: &[String]) -> globset::GlobSet {
    let mut builder = globset::GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(globset::Glob::new(pattern).expect("validated config glob"));
    }
    builder.build().expect("validated globs")
}

/// 中身が空（0バイトか注釈だけ）の YAML か。
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
/// 一覧のファイルの相対パス `display` を詳細にして設定の誤りで`停止`する
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "serde_json::Value is a foreign enum: the remaining JSON kinds are deliberately handled alike"
)]
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
/// "duplicate key: キー" の1行にする
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "serde_saphyr::Error is a foreign enum: every other error deliberately keeps the library's own message"
)]
fn yaml_error(e: &serde_saphyr::Error) -> StopReason {
    match e.without_snippet() {
        serde_saphyr::Error::DuplicateMappingKey { key: Some(key), .. } => {
            StopReason::ConfigError(format!("duplicate key: {key}"))
        }
        _ => StopReason::ConfigError(format!("{e}")),
    }
}

/// "surface" の鍵を読む。null、絶対パス、glob として読めない要素と、
/// 鍵の組み合わせの誤りは設定の誤りで停止する
fn read_surface(raw: RawSurface) -> Result<SurfaceConfig, StopReason> {
    let files = non_null_or_default(raw.files, "surface.files", Vec::new())?;
    check_globs(&files, "surface.files")?;
    let rules = non_null_or_default(raw.rules, "surface.rules", Vec::new())?;
    for rule in &rules {
        check_not_absolute(rule, "surface.rules")?;
    }
    let unspecified = non_null(raw.unspecified, "surface.unspecified")?;
    if let Some(path) = &unspecified {
        check_not_absolute(path, "surface.unspecified")?;
    }
    // 規則だけ、ファイルだけ、規則の無い一覧は、検査しているつもりで何もしない
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
        // パスの正規化
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
        // 空の設定ファイルは既定値
        if is_blank_yaml(yaml) {
            return Ok(Config::default());
        }

        let raw: RawConfig = serde_saphyr::from_str(yaml).map_err(|e| yaml_error(&e))?;
        Self::from_raw(raw)
    }

    pub(crate) fn validated(&self) -> Result<Self, StopReason> {
        Self::from_raw(RawConfig {
            decisions: Some(Some(RawDecisions {
                documents: Some(Some(self.decisions.documents.clone())),
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
            overview: self.overview.as_ref().map(|overview| {
                Some(RawOverview {
                    files: Some(Some(overview.files.clone())),
                    toc: Some(Some(overview.toc.clone())),
                })
            }),
            vague_words: Some(Some(self.vague_words.clone())),
            languages: Some(Some(self.languages.clone())),
            labels: Some(Some(self.labels.clone())),
        })
    }

    fn from_raw(raw: RawConfig) -> Result<Self, StopReason> {
        let defaults = Config::default();

        // "decisions:" 自体が null のときも設定の誤りで停止する。値は null と絶対パスを止め、
        // 正規化する（docs/decisions/core/config.md、docs/decisions/core/base-directory.md）
        let decisions = match non_null(raw.decisions, "decisions")? {
            Some(d) => {
                let documents = non_null_or_default(
                    d.documents,
                    "decisions.documents",
                    defaults.decisions.documents,
                )?;
                check_not_absolute(&documents, "decisions.documents")?;
                DecisionsConfig {
                    documents: crate::normalize_path(&documents),
                }
            }
            None => defaults.decisions,
        };

        // "tests:" と "tests.rust:" 自体、"tests.files"、
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
                check_globs(&files, "tests.files")?;
                let rules = non_null_or_default(t.rules, "tests.rules", defaults.tests.rules)?;
                for rule in &rules {
                    check_not_absolute(rule, "tests.rules")?;
                }
                TestsConfig {
                    // 一覧は既定を置き換える
                    files,
                    rust,
                    // パスの正規化
                    rules: rules
                        .iter()
                        .map(|rule| crate::normalize_path(rule))
                        .collect(),
                }
            }
            None => defaults.tests,
        };

        // "guides:" 自体と "guides.files" が null のときも停止する
        let guides = match non_null(raw.guides, "guides")? {
            Some(g) => {
                let files = non_null_or_default(g.files, "guides.files", defaults.guides.files)?;
                check_globs(&files, "guides.files")?;
                GuidesConfig { files }
            }
            None => defaults.guides,
        };

        // "mutants:" 自体と "mutants.equivalents" が null のときも停止する
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

        // "overview" を書くときは "overview.files" と "overview.toc" が必須。glob と
        // パスは とおり検査する
        let overview = match non_null(raw.overview, "overview")? {
            Some(o) => {
                let files = non_null(o.files, "overview.files")?.ok_or_else(|| {
                    StopReason::ConfigError("overview.files is required".to_string())
                })?;
                check_globs(&files, "overview.files")?;
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

        // "vague_words:" 自体が null のときも停止する
        let vague_words =
            non_null_or_default(raw.vague_words, "vague_words", defaults.vague_words)?;
        if vague_words.iter().any(|w| w.is_empty()) {
            return Err(StopReason::ConfigError(
                "vague_words contains an empty string".to_string(),
            ));
        }
        // 重複語の検出
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

        let languages = non_null_or_default(raw.languages, "languages", Vec::new())?;
        check_languages(&languages)?;
        let labels = non_null_or_default(raw.labels, "labels", BTreeMap::new())?;
        crate::ui_text::check_labels(&language_list(&languages), &labels)
            .map_err(StopReason::ConfigError)?;

        Ok(Config {
            decisions,
            tests,
            guides,
            mutants,
            surface,
            overview,
            // 一覧は既定を置き換える
            vague_words,
            languages,
            labels,
        })
    }

    /// `言語の一覧`。"languages" が無いか空なら "en" だけ
    pub fn languages(&self) -> Vec<String> {
        language_list(&self.languages)
    }

    /// その言語の`UI の文字`
    pub fn ui_text(&self, tag: &str) -> crate::ui_text::UiText {
        crate::ui_text::UiText::resolve(tag, self.labels.get(tag))
    }
}

fn language_list(languages: &[String]) -> Vec<String> {
    if languages.is_empty() {
        vec![crate::ui_text::ENGLISH.to_string()]
    } else {
        languages.to_vec()
    }
}

/// 空の文字列、小文字の英字と数字と "-" のほかの文字を含む言語タグ、重なった言語タグ
fn check_languages(languages: &[String]) -> Result<(), StopReason> {
    let mut seen = std::collections::BTreeSet::new();
    for tag in languages {
        let well_formed = !tag.is_empty()
            && tag
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
        if !well_formed {
            return Err(StopReason::ConfigError(format!(
                "invalid language tag: {tag:?}"
            )));
        }
        if !seen.insert(tag) {
            return Err(StopReason::ConfigError(format!(
                "duplicate language: {tag}"
            )));
        }
    }
    Ok(())
}
