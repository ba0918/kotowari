pub mod cargo_mutants;
pub mod change_records;
mod change_service;
pub mod changes;
pub mod comment_block;
pub mod config;
pub mod deferred;
pub mod deferred_notices;
mod doc_kind;
pub mod equivalents;
pub mod finding_map;
pub mod fingerprint;
pub mod git_snapshot;
pub mod guides;
pub mod ir;
pub mod list;
pub mod mutants;
pub mod plan;
pub mod query;
pub mod record_form;
pub mod schema;
pub mod sources;
pub mod status;
pub mod surface;
pub mod terms;
mod test_markers;
pub mod test_queries;
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
    pub tests: BTreeMap<String, TestFileTally>,
    /// REQ-core-206: 読んだ`ガイド`の数と、形の正しい`ガイドの印`の1件の数
    pub guides: guides::GuideTally,
    /// REQ-core-228: "surface.rules" が空の一覧でないときだけ出す
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<surface::Unlisted>,
}

/// 読んだテストのファイルの、1つの拡張子の数と問い合わせの有無（TBL-core-021）
#[derive(Debug, Clone, serde::Serialize)]
pub struct TestFileTally {
    /// その拡張子の読んだテストのファイルの数
    pub files: usize,
    /// その拡張子が問い合わせのある言語か
    pub query: bool,
}

/// 指摘の種類と、その JSON での文字列。
/// この呼び出しの一覧が変種の唯一の在り処で、列挙体・`ALL`・`as_str` はここから作る。
macro_rules! finding_kinds {
    ($($variant:ident => $text:literal,)+) => {
        /// 指摘の種類
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum FindingKind {
            $($variant,)+
        }

        impl FindingKind {
            /// 出しうる種類のすべて
            pub const ALL: &'static [FindingKind] = &[$(FindingKind::$variant),+];

            /// 種類を文字列に変換する（JSON 出力・整列・counts のキーに使う）
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(FindingKind::$variant => $text),+
                }
            }
        }
    };
}

finding_kinds! {
    ChangeStale => "change_stale",
    ChangeIrStale => "change_ir_stale",
    ChangeUncovered => "change_uncovered",
    ChangeDeferred => "change_deferred",
    ChangeConclusionConflict => "change_conclusion_conflict",
    ChangeRecordInvalid => "change_record_invalid",
    AlgorithmWithoutDefinition => "algorithm_without_definition",
    DeferredWithTest => "deferred_with_test",
    DependsOnDeferred => "depends_on_deferred",
    DuplicateField => "duplicate_field",
    DuplicateId => "duplicate_id",
    DuplicateTerm => "duplicate_term",
    EquivalentInvalid => "equivalent_invalid",
    EquivalentStale => "equivalent_stale",
    IdDomainMismatch => "id_domain_mismatch",
    InvalidGlossaryRow => "invalid_glossary_row",
    InvalidMarker => "invalid_marker",
    MissingDocument => "missing_document",
    MissingField => "missing_field",
    MissingScope => "missing_scope",
    MissingSource => "missing_source",
    MissingStatement => "missing_statement",
    MissingTable => "missing_table",
    MissingTag => "missing_tag",
    MissingTitle => "missing_title",
    MultipleTitles => "multiple_titles",
    MutantSurvived => "mutant_survived",
    MutantTimeout => "mutant_timeout",
    RecordFieldMissing => "record_field_missing",
    RecordFieldUnknown => "record_field_unknown",
    RequirementWithoutTest => "requirement_without_test",
    RevisionLinkInvalid => "revision_link_invalid",
    ScenarioWithoutTest => "scenario_without_test",
    SourceInvalid => "source_invalid",
    SurfaceUnspecifiedInvalid => "surface_unspecified_invalid",
    SurfaceUnspecifiedStale => "surface_unspecified_stale",
    SurfaceWithoutSpec => "surface_without_spec",
    TestWithoutId => "test_without_id",
    TooManyLines => "too_many_lines",
    TooManyRequirements => "too_many_requirements",
    UnclosedBacktick => "unclosed_backtick",
    UnclosedCodeBlock => "unclosed_code_block",
    UnknownCodeBlock => "unknown_code_block",
    UnknownField => "unknown_field",
    UnknownHeading => "unknown_heading",
    UnknownKind => "unknown_kind",
    UnknownLine => "unknown_line",
    UnknownTag => "unknown_tag",
    UnknownTerm => "unknown_term",
    UnparsableFile => "unparsable_file",
    InvalidGherkinLine => "invalid_gherkin_line",
    InvalidId => "invalid_id",
    InvalidPlan => "invalid_plan",
    GlossaryInvalid => "glossary_invalid",
    GlossaryTitleInvalid => "glossary_title_invalid",
    GuideStale => "guide_stale",
    UnresolvedReference => "unresolved_reference",
    VagueWord => "vague_word",
    VerificationInvalid => "verification_invalid",
    VerificationMissing => "verification_missing",
}

impl FindingKind {
    /// 重大度を返す（1か所で管理する）
    pub fn severity(&self) -> &'static str {
        match self {
            FindingKind::TooManyLines
            | FindingKind::TooManyRequirements
            | FindingKind::MutantTimeout
            | FindingKind::EquivalentStale
            | FindingKind::GuideStale
            | FindingKind::SurfaceUnspecifiedStale
            | FindingKind::DeferredWithTest
            | FindingKind::DependsOnDeferred => "notice",
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

/// 停止の理由と、標準エラーの1行目で詳細の前に出る文言（TBL-core-018）。
/// この呼び出しの一覧が変種の唯一の在り処で、列挙体・`WORDINGS`・`Display` はここから作る。
macro_rules! stop_reasons {
    ($($variant:ident => $wording:literal,)+) => {
        /// 停止の理由
        #[derive(Debug)]
        pub enum StopReason {
            $($variant(String)),+
        }

        impl StopReason {
            /// 標準エラーの1行目に出しうる文言のすべて
            pub const WORDINGS: &'static [&'static str] = &[$($wording),+];

            /// この理由の文言と詳細
            fn wording_and_detail(&self) -> (&'static str, &str) {
                match self {
                    $(StopReason::$variant(detail) => ($wording, detail)),+
                }
            }
        }

        impl std::fmt::Display for StopReason {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let (wording, detail) = self.wording_and_detail();
                write!(f, "{wording}: {detail}")
            }
        }
    };
}

stop_reasons! {
    ArgumentError => "argument error",
    ConfigError => "config error",
    UnreadableFile => "unreadable file",
    NonUtf8File => "non-UTF-8 file",
    ResultsError => "results error",
    MappingError => "mapping error",
    GitError => "git error",
}

/// 結果のファイルを作った変異テストの道具（REQ-core-149）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    CargoMutants,
}

impl Tool {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "cargo-mutants" => Ok(Tool::CargoMutants),
            _ => Err(format!("unknown tool: {s}")),
        }
    }
}

/// 引数の解析結果
#[derive(Debug)]
pub enum Cli {
    Changes {
        format: Format,
        config_path: Option<PathBuf>,
        base: String,
        target: git_snapshot::Target,
        phase: changes::Phase,
    },
    /// 検査を行う
    Check {
        format: Format,
        config_path: Option<PathBuf>,
    },
    /// 項目とテストの一覧を出す
    List {
        format: Format,
        config_path: Option<PathBuf>,
    },
    /// 変異の結果を検査する
    Mutants {
        format: Format,
        config_path: Option<PathBuf>,
        tool: Tool,
        results: PathBuf,
    },
    /// 1件の項目かシナリオを出す
    Query {
        format: Format,
        config_path: Option<PathBuf>,
        id: String,
    },
    /// 揃っているかの集計を出す
    Status {
        format: Format,
        config_path: Option<PathBuf>,
    },
    /// 計画書の形を検査する（REQ-core-190）
    Plan {
        format: Format,
        /// 計画書のファイルのパス。カレントディレクトリからの相対
        path: PathBuf,
    },
    /// 使い方を表示する
    Help,
    /// 版を表示する
    Version,
}

/// REQ-core-001: 1つ目の位置引数として受けるコマンド
const COMMANDS: [&str; 7] = [
    "changes", "check", "list", "mutants", "plan", "query", "status",
];

/// 引数を解析する（REQ-core-002, REQ-core-004, REQ-core-107, REQ-core-149, REQ-core-157, REQ-core-190）
pub fn parse_args(args: &[String]) -> Result<Cli, StopReason> {
    // REQ-core-107: --help か --version があればほかの引数を見ない
    for arg in args {
        if arg == "--help" {
            return Ok(Cli::Help);
        }
        if arg == "--version" {
            return Ok(Cli::Version);
        }
    }

    let mut format_str: Option<String> = None;
    let mut config_path: Option<PathBuf> = None;
    let mut tool: Option<String> = None;
    let mut command: Option<String> = None;
    let mut positionals: Vec<String> = Vec::new();
    let mut saw_format = false;
    let mut saw_config = false;
    let mut saw_tool = false;
    let mut change_options = BTreeMap::new();
    let mut staged = false;
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if arg == "--staged" {
            if staged {
                return Err(StopReason::ArgumentError(
                    "repeated option: --staged".into(),
                ));
            }
            staged = true;
            i += 1;
            continue;
        }
        if ["--base", "--head", "--phase"].contains(&arg.as_str()) {
            if change_options.contains_key(arg) {
                return Err(StopReason::ArgumentError(format!("repeated option: {arg}")));
            }
            i += 1;
            let value = args
                .get(i)
                .ok_or_else(|| StopReason::ArgumentError(format!("{arg} requires a value")))?;
            change_options.insert(arg.clone(), value.clone());
            i += 1;
            continue;
        }
        if arg.starts_with("--") {
            let slot = match arg.as_str() {
                "--format" => &mut saw_format,
                "--config" => &mut saw_config,
                "--tool" => &mut saw_tool,
                _ => {
                    return Err(StopReason::ArgumentError(format!("unknown option: {arg}")));
                }
            };
            if *slot {
                return Err(StopReason::ArgumentError(format!("repeated option: {arg}")));
            }
            *slot = true;
            i += 1;
            if i >= args.len() {
                return Err(StopReason::ArgumentError(format!("{arg} requires a value")));
            }
            match arg.as_str() {
                "--format" => format_str = Some(args[i].clone()),
                "--config" => config_path = Some(PathBuf::from(&args[i])),
                _ => tool = Some(args[i].clone()),
            }
        } else if command.is_none() {
            // REQ-core-001: 1つ目の位置引数は check、list、mutants、plan、query、status のどれか
            if COMMANDS.contains(&arg.as_str()) {
                command = Some(arg.clone());
            } else {
                return Err(StopReason::ArgumentError(format!("unknown command: {arg}")));
            }
        } else {
            positionals.push(arg.clone());
        }
        i += 1;
    }

    let Some(command) = command else {
        return Err(StopReason::ArgumentError(
            "expected command: changes, check, list, mutants, plan, query or status".to_string(),
        ));
    };

    if command == "changes" {
        if tool.is_some() || !positionals.is_empty() {
            return Err(StopReason::ArgumentError(
                "unexpected changes argument".into(),
            ));
        }
        let base = change_options
            .remove("--base")
            .ok_or_else(|| StopReason::ArgumentError("changes requires --base".into()))?;
        let phase = changes::Phase::parse(
            &change_options
                .remove("--phase")
                .ok_or_else(|| StopReason::ArgumentError("changes requires --phase".into()))?,
        )?;
        let head = change_options.remove("--head");
        let target = match (staged, head) {
            (true, None) if base == "HEAD" && phase == changes::Phase::Implementation => git_snapshot::Target::Index,
            (false, Some(head)) => git_snapshot::Target::Commit(head),
            _ => return Err(StopReason::ArgumentError("changes requires --head or --staged; --staged requires --base HEAD --phase implementation".into())),
        };
        let format = Format::parse(format_str.as_deref().unwrap_or("json"))
            .map_err(StopReason::ArgumentError)?;
        return Ok(Cli::Changes {
            format,
            config_path,
            base,
            target,
            phase,
        });
    }
    if staged || !change_options.is_empty() {
        return Err(StopReason::ArgumentError(format!(
            "unexpected change option for {command}"
        )));
    }
    // REQ-core-190: plan は設定を読まないので "--config" を受けない。指す先を見る前に止める
    if command == "plan" && saw_config {
        return Err(StopReason::ArgumentError(
            "unexpected option for plan: --config".to_string(),
        ));
    }

    // REQ-core-004: --config がディレクトリを指すとき
    if let Some(ref cp) = config_path
        && cp.is_dir()
    {
        return Err(StopReason::ArgumentError(format!(
            "--config is a directory: {}",
            cp.display()
        )));
    }

    let format = Format::parse(format_str.as_deref().unwrap_or("json"))
        .map_err(StopReason::ArgumentError)?;

    // REQ-core-152、REQ-core-158、REQ-core-163: list、query、status は check と同じ条件で、
    // 同じ理由と文言で停止する
    if command != "mutants" {
        // REQ-core-004: "mutants" でないコマンドに付けた "--tool"
        if tool.is_some() {
            return Err(StopReason::ArgumentError(format!(
                "unexpected option for {command}: --tool"
            )));
        }
        // REQ-core-190: "plan" の位置引数は計画書のファイルのパスがちょうど1つ
        if command == "plan" {
            let [path] = positionals.as_slice() else {
                return Err(StopReason::ArgumentError(format!(
                    "plan expects exactly one plan file path, got {}",
                    positionals.len()
                )));
            };
            return Ok(Cli::Plan {
                format,
                path: PathBuf::from(path),
            });
        }
        // REQ-core-157: "query" の位置引数は ID がちょうど1つ
        if command == "query" {
            let [id] = positionals.as_slice() else {
                return Err(StopReason::ArgumentError(format!(
                    "query expects exactly one id, got {}",
                    positionals.len()
                )));
            };
            // REQ-core-124: ID の形でない位置引数
            if !ir::is_valid_id(id) {
                return Err(StopReason::ArgumentError(format!("not an id: {id}")));
            }
            return Ok(Cli::Query {
                format,
                config_path,
                id: id.clone(),
            });
        }
        // REQ-core-004: "check"、"list"、"status" の後の位置引数
        if let Some(extra) = positionals.first() {
            return Err(StopReason::ArgumentError(format!(
                "unexpected argument: {extra}"
            )));
        }
        return Ok(match command.as_str() {
            "check" => Cli::Check {
                format,
                config_path,
            },
            "list" => Cli::List {
                format,
                config_path,
            },
            _ => Cli::Status {
                format,
                config_path,
            },
        });
    }

    // REQ-core-149: "--tool" は必須で、値は知っている道具の名前だけ
    let Some(tool) = tool else {
        return Err(StopReason::ArgumentError(
            "mutants requires the option: --tool".to_string(),
        ));
    };
    let tool = Tool::parse(&tool).map_err(StopReason::ArgumentError)?;

    // REQ-core-149: "mutants" の後の位置引数は結果のファイルのパスがちょうど1つ
    let [results] = positionals.as_slice() else {
        return Err(StopReason::ArgumentError(format!(
            "mutants expects exactly one result file path, got {}",
            positionals.len()
        )));
    };

    Ok(Cli::Mutants {
        format,
        config_path,
        tool,
        results: PathBuf::from(results),
    })
}

/// REQ-core-111: 読むファイルの先頭の UTF-8 BOM (U+FEFF) を読み飛ばす。
/// 読むファイルはどれもここを通す（`read_utf8_file` と `read_source_lines`）。
pub fn strip_bom(text: &str) -> &str {
    text.strip_prefix('\u{FEFF}').unwrap_or(text)
}

/// UTF-8 のテキストファイルを読む。読めないか UTF-8 でなければ StopReason を返す。
/// `display_path` は誤りの詳細に使う表示用のパス。
/// 先頭の UTF-8 BOM (U+FEFF) があれば読み飛ばす（REQ-core-111）。
pub fn read_utf8_file(path: &Path, display_path: &str) -> Result<String, StopReason> {
    let bytes = std::fs::read(path)
        .map_err(|e| StopReason::UnreadableFile(format!("{display_path}: {e}")))?;
    let text =
        String::from_utf8(bytes).map_err(|_| StopReason::NonUtf8File(display_path.to_string()))?;
    Ok(strip_bom(&text).to_string())
}

/// パスを正規化する純粋な関数。
/// 末尾の "/"、先頭の "./"、途中の "/./" と連続する "/"、"\" を正規化し、"a/.." を畳む。
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
        // "a/.." は畳む。畳む相手の無い ".." は残し、基準の外を指すパスのままにする
        // （review7-gaps の A1）
        if part == ".." && parts.last().is_some_and(|last| *last != "..") {
            parts.pop();
            continue;
        }
        parts.push(part);
    }
    if parts.is_empty() {
        // REQ-core-110: "." や "./" は空の置き場になる（呼び出し元が文書名だけの path を作る）
        String::new()
    } else {
        parts.join("/")
    }
}

/// 置き場と文書名を "/" でつなぐ。置き場が空なら文書名だけにする（REQ-core-110）。
pub fn join_display_path(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        format!("{dir}/{name}")
    }
}

/// 二重引用符の外のバッククォートの数が奇数か（REQ-core-116。二重引用符の中は A145 で対象外）。
/// ir モジュールと terms モジュールの両方から使う
pub fn has_odd_backticks_outside_quotes(text: &str) -> bool {
    split_outside_quotes(text)
        .join("")
        .chars()
        .filter(|&c| c == '`')
        .count()
        % 2
        != 0
}

/// 二重引用符の外の部分からバッククォートで囲んだ語を集める（REQ-core-054, REQ-core-064, REQ-core-104）。
/// ir モジュールと terms モジュールの両方から使う
pub fn extract_backtick_contents_outside_quotes(text: &str) -> Vec<&str> {
    // 二重引用符の外のバッククォートを行の左から順に対にし、中身は対の間の元の文字にする。
    // 断片ごとに対にすると、引用符を挟んだ囲みが崩れて後の囲みと組み違える（review5-gaps の A1）
    let base = text.as_ptr() as usize;
    let backticks: Vec<usize> = split_outside_quotes(text)
        .into_iter()
        .flat_map(|part| {
            let offset = part.as_ptr() as usize - base;
            part.match_indices('`').map(move |(i, _)| offset + i)
        })
        .collect();
    backticks
        .chunks_exact(2)
        .map(|pair| &text[pair[0] + 1..pair[1]])
        .collect()
}

/// 二重引用符を行の左から順に対にし、対の中身を集める。対にならない最後の引用符は捨てる（REQ-core-226）
pub fn double_quoted_contents(line: &str) -> Vec<&str> {
    let quotes: Vec<usize> = line.match_indices('"').map(|(i, _)| i).collect();
    quotes
        .chunks_exact(2)
        .map(|pair| &line[pair[0] + 1..pair[1]])
        .collect()
}

/// TBL-core-014: 引用符が奇数のときは最後の引用符から行末を引用の中とみなす。
/// ir モジュールと terms モジュールの両方から使う（REQ-core-054, REQ-core-064, REQ-core-104）。
pub fn split_outside_quotes(line: &str) -> Vec<&str> {
    let quote_count = line.chars().filter(|&c| c == '"').count();
    let odd_quotes = quote_count % 2 != 0;

    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_quote = false;
    let last_quote_pos = if odd_quotes {
        line.rfind('"').unwrap_or(0)
    } else {
        0
    };

    for (i, c) in line.char_indices() {
        if c == '"' {
            if odd_quotes && i == last_quote_pos {
                parts.push(&line[start..i]);
                return parts;
            }
            if !in_quote {
                parts.push(&line[start..i]);
                in_quote = true;
            } else {
                in_quote = false;
                start = i + 1;
            }
        }
    }
    parts.push(&line[start..]);
    parts
}

/// パスの "." と ".." をファイルシステムに触れずに畳む（`基準のディレクトリ`からの相対パスの表示用）。
fn lexically_normalize(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            other => result.push(other.as_os_str()),
        }
    }
    result
}

/// 基準のディレクトリから見た相対パスを "/" 区切りで作る。外にあれば ".." を並べる（両方とも正規化済みの絶対パス）
fn relative_display(base: &Path, target: &Path) -> String {
    let base_parts: Vec<_> = base.components().collect();
    let target_parts: Vec<_> = target.components().collect();
    let common = base_parts
        .iter()
        .zip(target_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();
    let mut parts: Vec<String> = vec!["..".to_string(); base_parts.len() - common];
    parts.extend(
        target_parts[common..]
            .iter()
            .map(|c| c.as_os_str().to_string_lossy().replace('\\', "/")),
    );
    parts.join("/")
}

/// 文書の全項目から ID の集合を作る（形に合う ID だけ）
pub fn collect_known_ids(docs: &[ir::IrDocument]) -> std::collections::BTreeSet<String> {
    docs.iter()
        .flat_map(|d| d.items.iter())
        .filter_map(|item| item.id().map(|s| s.to_string()))
        .filter(|id| ir::is_valid_id(id))
        .collect()
}

/// 基準のディレクトリを探す（TBL-core-003）
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

/// カレントディレクトリからの相対パスを、`基準のディレクトリ`からの相対の表示に直す（TBL-core-020）
fn display_from_base(base: &Path, cwd: &Path, path: &Path) -> String {
    relative_display(
        &lexically_normalize(base),
        &lexically_normalize(&cwd.join(path)),
    )
}

/// カレントディレクトリを取得する（A160/TBL-core-001/TBL-core-020）
fn current_dir() -> Result<PathBuf, StopReason> {
    std::env::current_dir()
        .map_err(|e| StopReason::UnreadableFile(format!("current directory: {e}")))
}

/// 停止する（REQ-core-005: 標準出力に何も出さず、理由を標準エラーに出し、終了コードは2）
fn stop(reason: &StopReason) -> u8 {
    eprintln!("{reason}");
    2
}

/// コマンドを実行し、終了コードを返す（TBL-core-002）
pub fn run(args: &[String]) -> u8 {
    let cli = match parse_args(args) {
        Ok(cli) => cli,
        Err(reason) => return stop(&reason),
    };

    match cli {
        Cli::Changes {
            format,
            config_path,
            base,
            target,
            phase,
        } => with_cwd(|cwd| {
            let result = change_service::run(cwd, &base, target, phase, config_path.as_deref())?;
            match format {
                Format::Json => println!("{}", serde_json::to_string(&result).unwrap()),
                Format::Text => print_findings_as_text(&result.findings),
            }
            Ok(exit_code_for(&result.findings))
        }),
        // REQ-core-107: 検査を行わず、使い方か版を出して終了コード0
        Cli::Help => {
            print_help();
            0
        }
        Cli::Version => {
            println!("kotowari {}", env!("CARGO_PKG_VERSION"));
            0
        }
        Cli::Check {
            format,
            config_path,
        } => with_cwd(|cwd| {
            let (result, format) = run_check(cwd, format, config_path.as_deref())?;
            print_check(&result, format);
            Ok(exit_code_for(&result.findings))
        }),
        // REQ-core-151: check と同じ読み取りを通し、指摘は出さず、読めれば終了コードは 0
        Cli::List {
            format,
            config_path,
        } => with_cwd(|cwd| {
            let result = run_list(cwd, config_path.as_deref())?;
            print_list(&result, format);
            Ok(0)
        }),
        // REQ-core-156: check と同じ読み取りを通し、指摘は出さず、読めれば終了コードは 0
        Cli::Query {
            format,
            config_path,
            id,
        } => with_cwd(|cwd| {
            let result = run_query(cwd, config_path.as_deref(), &id)?;
            print_query(&result, format);
            Ok(0)
        }),
        // REQ-core-162: check と同じ検査を走らせ、指摘は出さず集計だけを出す
        Cli::Status {
            format,
            config_path,
        } => with_cwd(|cwd| {
            let result = run_status(cwd, config_path.as_deref())?;
            print_status(&result, format);
            // REQ-core-165: complete なら 0、そうでなければ 1
            Ok(u8::from(!result.complete))
        }),
        Cli::Mutants {
            format,
            config_path,
            tool,
            results,
        } => with_cwd(|cwd| {
            let result = run_mutants(cwd, config_path.as_deref(), tool, &results)?;
            print_mutants(&result, format);
            Ok(exit_code_for(&result.findings))
        }),
        // REQ-core-196: 設定を読まず、計画書のファイルだけを読む
        Cli::Plan { format, path } => with_cwd(|cwd| {
            let result = run_plan(cwd, &path)?;
            print_plan(&result, format);
            Ok(exit_code_for(&result.findings))
        }),
    }
}

/// カレントディレクトリを取って command を走らせ、その終了コードを返す。
/// カレントディレクトリが取れないか command が止まれば`停止`する
fn with_cwd(command: impl FnOnce(&Path) -> Result<u8, StopReason>) -> u8 {
    match current_dir().and_then(|cwd| command(&cwd)) {
        Ok(code) => code,
        Err(reason) => stop(&reason),
    }
}

/// TBL-core-002: 誤りが1件以上あれば1、無ければ0
fn exit_code_for(findings: &[Finding]) -> u8 {
    if findings.iter().any(|f| f.severity == "error") {
        1
    } else {
        0
    }
}

/// "kotowari check" の結果を出す（TBL-core-005, REQ-core-025, REQ-core-026）
fn print_check(result: &CheckResult, format: Format) {
    match format {
        Format::Json => println!("{}", serde_json::to_string(result).unwrap()),
        Format::Text => {
            print_findings_as_text(&result.findings);
            // REQ-core-228: 指摘の行の後の最後の1行。指摘が0件でも出す
            if let Some(surface) = &result.surface {
                println!("surface: unspecified={}", surface.unspecified);
            }
        }
    }
}

/// "kotowari list" の一覧を出す（REQ-core-155）
fn print_list(result: &list::ListResult, format: Format) {
    match format {
        Format::Json => println!("{}", serde_json::to_string(result).unwrap()),
        Format::Text => list::print_text(result),
    }
}

/// "kotowari query" の1件を出す（REQ-core-161）
fn print_query(result: &query::QueryResult, format: Format) {
    match format {
        Format::Json => println!("{}", serde_json::to_string(result).unwrap()),
        Format::Text => query::print_text(result),
    }
}

/// "kotowari status" の集計を出す（REQ-core-166）
fn print_status(result: &status::StatusResult, format: Format) {
    match format {
        Format::Json => println!("{}", serde_json::to_string(result).unwrap()),
        Format::Text => status::print_text(result),
    }
}

/// "kotowari mutants" の結果を出す（TBL-core-025, REQ-core-146）
fn print_mutants(result: &mutants::MutantsResult, format: Format) {
    match format {
        Format::Json => println!("{}", serde_json::to_string(result).unwrap()),
        Format::Text => {
            print_findings_as_text(&result.findings);
            // REQ-core-146: 指摘の行の後の最後の1行。指摘が0件でも出す
            println!("{}", result.mutants.summary_line());
        }
    }
}

/// "kotowari plan" の結果を出す（REQ-core-194、REQ-core-025）
fn print_plan(result: &plan::PlanResult, format: Format) {
    match format {
        Format::Json => println!("{}", serde_json::to_string(result).unwrap()),
        Format::Text => print_findings_as_text(&result.findings),
    }
}

/// REQ-core-025, REQ-core-026: 1つの指摘を1行で出し、"line" が null なら "-" と書く
fn print_findings_as_text(findings: &[Finding]) {
    for f in findings {
        let line = f.line.map_or("-".to_string(), |l| l.to_string());
        println!(
            "{}:{} [{}] {} {}",
            one_line(&f.path),
            line,
            f.severity,
            f.kind,
            one_line(&f.detail)
        );
    }
}

/// 改行を "\\n" と "\\r" の2文字で書き、1つの指摘が1行に収まるようにする（REQ-core-025、
/// review5-gaps の A2）。ファイル名は改行を含みうる
fn one_line(text: &str) -> String {
    text.replace('\r', "\\r").replace('\n', "\\n")
}

fn print_help() {
    println!("Usage: kotowari [OPTIONS] <COMMAND> [ARGUMENT]");
    println!();
    println!("Commands:");
    println!("  changes    Check change records against a Git base and target snapshot");
    println!("  check      Check IR documents and test markers");
    println!("  list       List IR items and the tests marked for them");
    println!("  mutants    Read a mutation testing result file and report survivors");
    println!("  plan       Check the form of one plan file against the bundled schema");
    println!("  query      Show one item or scenario with its body and back references");
    println!("  status     Summarise the IR and tell whether it is complete");
    println!();
    println!("Changes: --base <REV> (--head <REV> | --staged) --phase <implementation|review>");
    println!("Options:");
    println!("  --format <FORMAT>  Output format: json (default) or text");
    println!("  --config <PATH>    Path to configuration file");
    println!("  --tool <TOOL>      Mutation testing tool of the result file: cargo-mutants");
    println!("  --help             Show this help message");
    println!("  --version          Show version");
}

/// 計画書の検査のエントリポイント（REQ-core-196）。設定、IR、判断の記録、テストのファイルを読まない
pub fn run_plan(cwd: &Path, path: &Path) -> Result<plan::PlanResult, StopReason> {
    let base = find_base(cwd);
    // 計画書のパスはカレントディレクトリからの相対（REQ-core-190）、
    // 指摘と停止の path は基準のディレクトリからの相対（REQ-core-193、TBL-core-020）
    let display = display_from_base(&base, cwd, path);
    // REQ-core-197: 無い、ディレクトリ、読めないは読めないファイル、UTF-8 でなければ UTF-8 でないファイル
    let text = read_utf8_file(&cwd.join(path), &display)?;
    let mut findings = plan::check_plan(&display, &text);
    sort_findings(&mut findings);
    let counts = count_findings(&findings);
    Ok(plan::PlanResult { findings, counts })
}

/// 変異の結果の検査のエントリポイント
pub fn run_mutants(
    cwd: &Path,
    config_path: Option<&Path>,
    tool: Tool,
    results: &Path,
) -> Result<mutants::MutantsResult, StopReason> {
    let base = find_base(cwd);
    // REQ-core-147: mutants が読むのは設定、結果のファイル、等価の一覧、
    // 変異の結果と一覧の1件が指すソースだけ。"ir" などの指す先は見ない
    let cfg = load_config(cwd, &base, config_path)?;

    // 結果のファイルのパスはカレントディレクトリからの相対（REQ-core-149）、
    // 停止の詳細は基準のディレクトリからの相対（TBL-core-020）
    let display = display_from_base(&base, cwd, results);
    let text = read_utf8_file(&cwd.join(results), &display)?;

    // 道具の結果を変異の結果に写すのはここだけ（REQ-core-138、A12）
    let outcomes = match tool {
        Tool::CargoMutants => cargo_mutants::read_outcomes(&text),
    }
    .map_err(|e| StopReason::ResultsError(format!("{display}: {e}")))?;

    // REQ-core-148: 鍵が無ければ等価の一覧は0件。指す先が無いか読めなければ停止する
    let (list, list_path) = match &cfg.mutants.equivalents {
        None => (equivalents::EquivalentList::default(), String::new()),
        Some(path) => {
            let text = read_utf8_file(&base.join(path), path)?;
            (equivalents::read_list(&text, path)?, path.clone())
        }
    };

    let sources = read_sources(&base, &list.entries);
    let (mut findings, mutant_counts) =
        mutants::check_outcomes(&outcomes, &list.entries, &list_path, &sources);
    findings.extend(list.findings);
    sort_findings(&mut findings);
    let counts = count_findings(&findings);

    Ok(mutants::MutantsResult {
        findings,
        counts,
        mutants: mutant_counts,
    })
}

/// 一致を見るのに要るソースだけを読む。読めなかったファイルは持たない（REQ-core-141、REQ-core-142）
fn read_sources(base: &Path, entries: &[equivalents::Equivalent]) -> BTreeMap<String, Vec<String>> {
    let mut sources = BTreeMap::new();
    // 一致にも文面の検査にも要るのは一覧の1件が指すファイルだけ。
    // 一致には "file" が同じであることが要るので、どの1件も指さないファイルは読んでも使われない
    let files: std::collections::BTreeSet<&str> = entries.iter().map(|e| e.file.as_str()).collect();
    for file in files {
        if let Some(lines) = read_source_lines(base, file) {
            sources.insert(file.to_string(), lines);
        }
    }
    sources
}

/// ソースを行に分ける。無い、読めない、UTF-8 でないときは None を返して`停止`しない（REQ-core-141、REQ-core-142）。
/// 行の区切りは TBL-core-010 と同じで、行の終わりの "\r\n" の "\r" は文面に含めない（A52）。
/// 先頭の BOM は読み飛ばす（REQ-core-111）。読み飛ばさないと1行目の文面に BOM が残る。
fn read_source_lines(base: &Path, file: &str) -> Option<Vec<String>> {
    let bytes = std::fs::read(base.join(file)).ok()?;
    let text = String::from_utf8(bytes).ok()?;
    Some(
        ir::split_lines(strip_bom(&text))
            .into_iter()
            .map(str::to_string)
            .collect(),
    )
}

/// REQ-core-024: 指摘を TBL-core-007 の順（path → line → kind → detail）に並べる
fn sort_findings(findings: &mut [Finding]) {
    findings.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            // TBL-core-007: line は null が先、その後は小さい順（Option の順序が None を先に置く）
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
            .then_with(|| a.detail.cmp(&b.detail))
    });
}

/// PROP-core-002: 種類ごとの指摘の数。1件も無い種類は持たない
fn count_findings(findings: &[Finding]) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for f in findings {
        *counts.entry(f.kind.as_str().to_string()).or_insert(0) += 1;
    }
    counts
}

/// `設定ファイル`を読む（REQ-core-003, REQ-core-011, REQ-core-012。TBL-core-020: 詳細のパスは基準からの相対）
fn load_config(
    cwd: &Path,
    base: &Path,
    config_path: Option<&Path>,
) -> Result<config::Config, StopReason> {
    let (path, display) = match config_path {
        // --config は CWD からの相対パス（REQ-core-003）
        Some(cp) => {
            let abs = cwd.join(cp);
            if !abs.exists() {
                return Err(StopReason::ArgumentError(format!(
                    "config file not found: {}",
                    cp.display()
                )));
            }
            // TBL-core-020/A164: 詳細のパスは基準のディレクトリからの相対
            // （外にあれば "../" を含む。ファイルシステムには触れない）
            (abs, display_from_base(base, cwd, cp))
        }
        // 既定: base/.kotowari/config.yaml
        None => {
            let default_path = base.join(".kotowari/config.yaml");
            if !default_path.exists() {
                // REQ-core-012: 設定ファイルが無いときは既定の値
                return Ok(config::Config::default());
            }
            (default_path, ".kotowari/config.yaml".to_string())
        }
    };

    let text = read_utf8_file(&path, &display)?;
    config::Config::parse(&text).map_err(|e| match e {
        StopReason::ConfigError(msg) => StopReason::ConfigError(format!("{display}: {msg}")),
        other => other,
    })
}

/// check、list、query、status が共有する読み取りの結果
/// （REQ-core-151、REQ-core-156、REQ-core-162: どれも check と同じ読み取りを使う）
pub struct Loaded {
    /// `基準のディレクトリ`
    pub base: PathBuf,
    pub cfg: config::Config,
    pub docs: Vec<ir::IrDocument>,
    pub findings: Vec<Finding>,
    /// TBL-core-021: 読んだテストのファイルの拡張子ごとの数
    pub tally: BTreeMap<String, TestFileTally>,
    /// TBL-core-026: 印の出現ごとの (ID, テストのファイル, 行, テストの名前)
    pub markers: Vec<tests_discovery::TestMarker>,
    /// 読んだ`テストのファイル`の相対パス。バイト順（REQ-core-199）
    pub test_files: Vec<String>,
}

/// 設定と置き場から IR の文書とテストのファイルを読み、検査もする。
/// check はこの指摘を出し、list と query は捨て、status は数だけを出す（REQ-core-151、REQ-core-156、REQ-core-162）。
/// `ガイド`はここでは読まない。list と query は`ガイド`を読まない（REQ-core-152、REQ-core-158）ので、
/// check と status だけが `read_guides` を続けて呼ぶ
pub fn load_all(cwd: &Path, config_path: Option<&Path>) -> Result<Loaded, StopReason> {
    let base = find_base(cwd);
    let cfg = load_config(cwd, &base, config_path)?;

    // 設定のパスは基準のディレクトリからの相対（REQ-core-010）
    let ir_dir = base.join(&cfg.ir);
    let records_dir = base.join(&cfg.decisions.records);
    let adr_dir = base.join(&cfg.decisions.adr);

    // REQ-core-018: 置き場が無い、または読めないとき停止（TBL-core-020: 相対パスと OS の誤りの文）
    for (dir, configured) in [
        (&ir_dir, &cfg.ir),
        (&records_dir, &cfg.decisions.records),
        (&adr_dir, &cfg.decisions.adr),
    ] {
        if !dir.is_dir() {
            let err = std::fs::read_dir(dir)
                .err()
                .map(|e| e.to_string())
                .unwrap_or_else(|| "not a directory".to_string());
            return Err(StopReason::UnreadableFile(format!("{configured}: {err}")));
        }
    }

    // IR の文書を読んで検査する
    let (docs, mut findings, duplicates) = ir::load_and_check_with_duplicates(&base, &cfg)?;

    // 出典の検査
    let source_ctx = sources::build_context(&base, &cfg)?;
    sources::check_sources_with_duplicates(&docs, &source_ctx, &cfg.ir, &duplicates, &mut findings);

    // 判断の記録の形の検査
    record_form::check_record_forms(&source_ctx, &mut findings);

    // 用語と曖昧語の検査
    let known_ids = collect_known_ids(&docs);
    terms::check_terms_and_vague_words_with_duplicates(
        &docs,
        &known_ids,
        &cfg.vague_words,
        &cfg.ir,
        &duplicates,
        &mut findings,
    );

    // 文書名の参照の検査
    let ir_paths: std::collections::BTreeSet<String> =
        docs.iter().map(|d| d.relative_path.clone()).collect();
    terms::check_document_references(&docs, &cfg.ir, &ir_paths, &mut findings);

    // テストの発見と印の検査
    let discovered = tests_discovery::discover_and_check(
        &base,
        &cfg,
        &docs,
        &known_ids,
        &cfg.ir,
        &mut findings,
    )?;

    // 後回しとの食い違い（REQ-core-211、REQ-core-212）
    deferred_notices::check(&docs, &cfg.ir, &discovered.markers, &mut findings);

    Ok(Loaded {
        base,
        cfg,
        docs,
        findings,
        tally: discovered.tally,
        markers: discovered.markers,
        test_files: discovered.files,
    })
}

/// check と status の読み取り: `load_all` に続けて`ガイド`と`面`を読み、その`指摘`を足す
/// （REQ-core-198、REQ-core-162、REQ-core-229）
fn load_with_guides(
    cwd: &Path,
    config_path: Option<&Path>,
) -> Result<(Loaded, guides::GuideTally, Option<surface::SurfaceTally>), StopReason> {
    let mut loaded = load_all(cwd, config_path)?;
    change_records::static_check(
        &loaded.base,
        &loaded.cfg,
        &loaded.docs,
        &mut loaded.findings,
    )?;
    let tally = guides::read_guides(
        &loaded.base,
        &loaded.cfg,
        &loaded.test_files,
        &loaded.docs,
        &mut loaded.findings,
    )?;
    let surface = surface::check(
        &loaded.base,
        &loaded.cfg,
        &loaded.docs,
        &mut loaded.findings,
    )?;
    Ok((loaded, tally, surface))
}

/// 検査のエントリポイント
pub fn run_check(
    cwd: &Path,
    format: Format,
    config_path: Option<&Path>,
) -> Result<(CheckResult, Format), StopReason> {
    let (loaded, guides, surface) = load_with_guides(cwd, config_path)?;

    let files = loaded.docs.len();
    let lines: usize = loaded.docs.iter().map(|d| d.line_count).sum();

    let mut findings = loaded.findings;
    sort_findings(&mut findings);
    let counts = count_findings(&findings);

    let result = CheckResult {
        files,
        lines,
        findings,
        counts,
        tests: loaded.tally,
        guides,
        surface: surface.map(|tally| surface::Unlisted {
            unspecified: tally.unspecified,
        }),
    };

    Ok((result, format))
}

/// 一覧のエントリポイント（REQ-core-151）。指摘は計算しても出さない
pub fn run_list(cwd: &Path, config_path: Option<&Path>) -> Result<list::ListResult, StopReason> {
    let loaded = load_all(cwd, config_path)?;
    Ok(list::build(&loaded.docs, &loaded.cfg.ir, &loaded.markers))
}

/// 1件の読み取りのエントリポイント（REQ-core-156）。指摘は計算しても出さない。
/// REQ-core-157: 位置引数と同じ `ID` を持つものが無ければ引数の誤りで停止する
pub fn run_query(
    cwd: &Path,
    config_path: Option<&Path>,
    id: &str,
) -> Result<query::QueryResult, StopReason> {
    let loaded = load_all(cwd, config_path)?;
    query::build(&loaded.docs, &loaded.cfg.ir, &loaded.markers, id)
        .ok_or_else(|| StopReason::ArgumentError(format!("unknown id: {id}")))
}

/// 集計のエントリポイント（REQ-core-162）。指摘は計算しても数だけを出す
pub fn run_status(
    cwd: &Path,
    config_path: Option<&Path>,
) -> Result<status::StatusResult, StopReason> {
    let (loaded, guides, surface) = load_with_guides(cwd, config_path)?;
    Ok(status::build(
        &loaded.docs,
        &loaded.cfg.ir,
        &loaded.markers,
        loaded.tally,
        guides,
        surface.unwrap_or_default(),
        &loaded.findings,
    ))
}
