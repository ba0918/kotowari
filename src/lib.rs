pub mod config;
pub mod ir;
pub mod sources;
pub mod terms;
pub mod tests_discovery;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 検査結果の最上位の構造
#[derive(serde::Serialize)]
pub struct CheckResult {
    pub files: usize,
    pub lines: usize,
    pub findings: Vec<Finding>,
    pub counts: BTreeMap<String, usize>,
}

/// 指摘の種類
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FindingKind {
    AlgorithmWithoutDefinition,
    DuplicateField,
    DuplicateId,
    InvalidMarker,
    MissingDocument,
    MissingField,
    MissingScope,
    MissingSource,
    MissingStatement,
    MissingTable,
    MissingTag,
    MissingTitle,
    MultipleTitles,
    RequirementWithoutTest,
    SourceInvalid,
    TestWithoutId,
    TooManyLines,
    TooManyRequirements,
    UnknownField,
    UnknownHeading,
    UnknownKind,
    UnknownTag,
    UnknownTerm,
    UnparsableFile,
    UnresolvedReference,
    VagueWord,
    VerificationInvalid,
    VerificationMissing,
}

impl FindingKind {
    /// 種類を文字列に変換する（JSON 出力・整列・counts のキーに使う）
    pub fn as_str(&self) -> &'static str {
        match self {
            FindingKind::AlgorithmWithoutDefinition => "algorithm_without_definition",
            FindingKind::DuplicateField => "duplicate_field",
            FindingKind::DuplicateId => "duplicate_id",
            FindingKind::InvalidMarker => "invalid_marker",
            FindingKind::MissingDocument => "missing_document",
            FindingKind::MissingField => "missing_field",
            FindingKind::MissingScope => "missing_scope",
            FindingKind::MissingSource => "missing_source",
            FindingKind::MissingStatement => "missing_statement",
            FindingKind::MissingTable => "missing_table",
            FindingKind::MissingTag => "missing_tag",
            FindingKind::MissingTitle => "missing_title",
            FindingKind::MultipleTitles => "multiple_titles",
            FindingKind::RequirementWithoutTest => "requirement_without_test",
            FindingKind::SourceInvalid => "source_invalid",
            FindingKind::TestWithoutId => "test_without_id",
            FindingKind::TooManyLines => "too_many_lines",
            FindingKind::TooManyRequirements => "too_many_requirements",
            FindingKind::UnknownField => "unknown_field",
            FindingKind::UnknownHeading => "unknown_heading",
            FindingKind::UnknownKind => "unknown_kind",
            FindingKind::UnknownTag => "unknown_tag",
            FindingKind::UnknownTerm => "unknown_term",
            FindingKind::UnparsableFile => "unparsable_file",
            FindingKind::UnresolvedReference => "unresolved_reference",
            FindingKind::VagueWord => "vague_word",
            FindingKind::VerificationInvalid => "verification_invalid",
            FindingKind::VerificationMissing => "verification_missing",
        }
    }

    /// 重大度を返す（1か所で管理する）
    pub fn severity(&self) -> &'static str {
        match self {
            FindingKind::TooManyLines | FindingKind::TooManyRequirements => "warning",
            _ => "error",
        }
    }
}

impl std::fmt::Display for FindingKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<&str> for FindingKind {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<str> for FindingKind {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl serde::Serialize for FindingKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

/// 指摘
#[derive(Debug, serde::Serialize, Clone)]
pub struct Finding {
    pub kind: FindingKind,
    pub severity: String,
    pub path: String,
    pub line: Option<usize>,
    pub detail: String,
}

impl Finding {
    /// 指摘を作る。severity は kind から自動で決まる。
    pub fn new(kind: FindingKind, path: String, line: Option<usize>, detail: String) -> Self {
        Finding {
            severity: kind.severity().to_string(),
            kind,
            path,
            line,
            detail,
        }
    }
}

/// 出力の形式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Text,
}

impl Format {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s {
            "json" => Ok(Format::Json),
            "text" => Ok(Format::Text),
            _ => Err(format!("unknown format: {s}")),
        }
    }
}

/// 停止の理由
#[derive(Debug)]
pub enum StopReason {
    ArgumentError(String),
    ConfigError(String),
    UnreadableFile(String),
    NonUtf8File(String),
}

impl std::fmt::Display for StopReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StopReason::ArgumentError(s) => write!(f, "argument error: {s}"),
            StopReason::ConfigError(s) => write!(f, "config error: {s}"),
            StopReason::UnreadableFile(s) => write!(f, "unreadable file: {s}"),
            StopReason::NonUtf8File(s) => write!(f, "non-UTF-8 file: {s}"),
        }
    }
}

/// UTF-8 のテキストファイルを読む。読めないか UTF-8 でなければ StopReason を返す。
/// `display_path` は誤りの詳細に使う表示用のパス。
pub fn read_utf8_file(path: &Path, display_path: &str) -> Result<String, StopReason> {
    let bytes = std::fs::read(path)
        .map_err(|e| StopReason::UnreadableFile(format!("{display_path}: {e}")))?;
    String::from_utf8(bytes)
        .map_err(|_| StopReason::NonUtf8File(display_path.to_string()))
}

/// パスを正規化する純粋な関数。
/// 末尾の "/"、先頭の "./"、途中の "/./" と連続する "/"、"\" を正規化する。
pub fn normalize_path(path: &str) -> String {
    let s = path.replace('\\', "/");
    let mut parts: Vec<&str> = Vec::new();
    for part in s.split('/') {
        if part == "." || part.is_empty() {
            // 先頭の空（= 先頭の "/"）は保持しない（相対パスの前提）
            // 途中の空（= 連続する "/"）は飛ばす
            // "." は飛ばす
            continue;
        }
        parts.push(part);
    }
    if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

/// 文書の全項目から ID の集合を作る
pub fn collect_known_ids(docs: &[ir::IrDocument]) -> std::collections::BTreeSet<String> {
    docs.iter()
        .flat_map(|d| d.items.iter())
        .filter_map(|item| item.id().map(|s| s.to_string()))
        .collect()
}

/// 基準のディレクトリを探す（TBL-003）
/// カレントディレクトリから上に向かって .kotowari/ があるディレクトリを探す。
/// 見つからなければカレントディレクトリを返す。
pub fn find_base(cwd: &Path) -> PathBuf {
    let mut dir = cwd.to_path_buf();
    loop {
        if dir.join(".kotowari").is_dir() {
            return dir;
        }
        if !dir.pop() {
            return cwd.to_path_buf();
        }
    }
}

/// 検査のエントリポイント
pub fn run_check(
    cwd: &Path,
    format: Format,
    config_path: Option<&Path>,
) -> Result<(CheckResult, Format), StopReason> {
    let base = find_base(cwd);

    // 設定ファイルを読む
    let cfg = if let Some(cp) = config_path {
        // --config は CWD からの相対パス（REQ-003）
        let abs = cwd.join(cp);
        if !abs.exists() {
            return Err(StopReason::ArgumentError(format!(
                "config file not found: {}",
                cp.display()
            )));
        }
        let text = read_utf8_file(&abs, &abs.display().to_string())?;
        config::Config::parse(&text)?
    } else {
        // 既定: base/.kotowari/config.yaml
        let default_path = base.join(".kotowari/config.yaml");
        if default_path.exists() {
            let text = read_utf8_file(&default_path, &default_path.display().to_string())?;
            config::Config::parse(&text)?
        } else {
            // REQ-012: 設定ファイルが無いときは既定の値
            config::Config::default()
        }
    };

    // 設定のパスは基準のディレクトリからの相対（REQ-010）
    let ir_dir = base.join(&cfg.ir);
    let records_dir = base.join(&cfg.decisions.records);
    let adr_dir = base.join(&cfg.decisions.adr);

    // REQ-018: 置き場が無い、または読めないとき停止
    if !ir_dir.is_dir() {
        return Err(StopReason::UnreadableFile(format!(
            "ir directory not found: {}",
            cfg.ir
        )));
    }
    if !records_dir.is_dir() {
        return Err(StopReason::UnreadableFile(format!(
            "decisions.records directory not found: {}",
            cfg.decisions.records
        )));
    }
    if !adr_dir.is_dir() {
        return Err(StopReason::UnreadableFile(format!(
            "decisions.adr directory not found: {}",
            cfg.decisions.adr
        )));
    }

    // IR の文書を読んで検査する
    let (docs, mut findings) = ir::load_and_check(&base, &cfg)?;

    // 出典の検査
    let source_ctx = sources::build_context(&base, &cfg)?;
    sources::check_sources(&docs, &source_ctx, &cfg.ir, &mut findings);

    // 用語と曖昧語の検査
    let glossary = terms::collect_glossary_terms(&docs);
    let known_ids = collect_known_ids(&docs);
    terms::check_terms_and_vague_words(
        &docs, &glossary, &known_ids, &cfg.vague_words, &cfg.ir, &mut findings,
    );

    // 文書名の参照の検査
    let ir_filenames: std::collections::BTreeSet<String> =
        docs.iter().map(|d| d.filename.clone()).collect();
    terms::check_document_references(&docs, &cfg.ir, &ir_filenames, &mut findings);

    // テストの発見と印の検査
    tests_discovery::discover_and_check(
        &base, &cfg, &docs, &known_ids, &cfg.ir, &mut findings,
    )?;

    let files = docs.len();
    let lines: usize = docs.iter().map(|d| d.line_count).sum();

    // REQ-024: 指摘を並べる（TBL-007: path → line → kind → detail）
    findings.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then_with(|| match (a.line, b.line) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(al), Some(bl)) => al.cmp(&bl),
            })
            .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
            .then_with(|| a.detail.cmp(&b.detail))
    });

    // counts を作る（PROP-002: 0件は含まない）
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for f in &findings {
        *counts.entry(f.kind.as_str().to_string()).or_insert(0) += 1;
    }

    let result = CheckResult {
        files,
        lines,
        findings,
        counts,
    };

    Ok((result, format))
}
