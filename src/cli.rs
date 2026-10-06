use kotowari::{
    ChangesReport, CheckReport, Finding, MutantsReport, Phase, PlanReport, Project, ProjectOptions,
    QueryReport, ReadList, StatusReport, Target, Tool,
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
mod list;
mod output;
mod query;
mod serve;
mod status;
#[cfg(test)]
mod tests;
#[derive(Debug)]
pub enum StopReason {
    ArgumentError(String),
    UnreadableFile(String),
    /// serve がポートを使えない（REQ-core-298）
    PortError(String),
    Library(kotowari::Error),
}
/// ポートの誤りの文言（TBL-core-018）。serve を持つ CLI だけが出す
const PORT_ERROR: &str = "port error";
impl StopReason {
    /// core とライブラリに無い、CLI の停止の文言のすべて
    #[cfg(test)]
    pub const WORDINGS: &'static [&'static str] = &[PORT_ERROR];
}
impl std::fmt::Display for StopReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentError(detail) => write!(f, "argument error: {detail}"),
            Self::UnreadableFile(detail) => write!(f, "unreadable file: {detail}"),
            Self::PortError(detail) => write!(f, "{PORT_ERROR}: {detail}"),
            Self::Library(error) => error.fmt(f),
        }
    }
}
impl From<kotowari::Error> for StopReason {
    fn from(error: kotowari::Error) -> Self {
        if error.kind() == kotowari::ErrorKind::UnknownQuery {
            Self::ArgumentError(format!(
                "unknown id: {}",
                error
                    .detail()
                    .strip_prefix("unknown query: ")
                    .unwrap_or(error.detail())
            ))
        } else {
            Self::Library(error)
        }
    }
}
fn parse_tool(s: &str) -> Result<Tool, String> {
    match s {
        "cargo-mutants" => Ok(Tool::CargoMutants),
        _ => Err(format!("unknown tool: {s}")),
    }
}
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
/// 引数の解析結果
#[derive(Debug)]
pub enum Cli {
    Changes {
        format: Format,
        config_path: Option<PathBuf>,
        base: String,
        target: Target,
        phase: Phase,
    },
    /// 検査を行う
    Check {
        format: Format,
        config_path: Option<PathBuf>,
        /// テスト側の指摘を終了コードに数えない（REQ-core-357）
        allow_test_findings: bool,
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
    /// 全体像を ".kotowari/cache/overview/" の下に書く（REQ-core-293）
    OverviewBuild {
        format: Format,
        config_path: Option<PathBuf>,
    },
    /// 全体像を書いて手元で配る（REQ-core-297）
    OverviewServe {
        port: u16,
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
const COMMANDS: [&str; 8] = [
    "changes", "check", "list", "mutants", "overview", "plan", "query", "status",
];

/// serve の既定のポート（REQ-core-297）
const DEFAULT_PORT: u16 = 4590;

/// REQ-core-304: "--port" の値は1から65535までの10進の整数
fn parse_port(value: &str) -> Result<u16, StopReason> {
    let invalid = || StopReason::ArgumentError(format!("invalid port: {value}"));
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(invalid());
    }
    match value.parse::<u16>() {
        Ok(port) if port >= 1 => Ok(port),
        _ => Err(invalid()),
    }
}

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
    let mut saw_port = false;
    let mut port: Option<String> = None;
    let mut change_options = BTreeMap::new();
    let mut staged = false;
    let mut allow_test_findings = false;
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
        if arg == "--allow-test-findings" {
            if allow_test_findings {
                return Err(StopReason::ArgumentError(
                    "repeated option: --allow-test-findings".into(),
                ));
            }
            allow_test_findings = true;
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
                "--port" => &mut saw_port,
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
                "--port" => port = Some(args[i].clone()),
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
            "expected command: check, changes, list, mutants, overview, plan, query or status"
                .to_string(),
        ));
    };

    // REQ-core-004: "--port" は "overview serve" だけが受ける
    let serve = command == "overview" && positionals.first().map(String::as_str) == Some("serve");
    if port.is_some() && !serve {
        return Err(StopReason::ArgumentError(format!(
            "unexpected option for {command}: --port"
        )));
    }
    // REQ-core-004: "--allow-test-findings" は "check" だけが受ける
    if allow_test_findings && command != "check" {
        return Err(StopReason::ArgumentError(format!(
            "unexpected option for {command}: --allow-test-findings"
        )));
    }
    if command == "changes" {
        if tool.is_some() || !positionals.is_empty() {
            return Err(StopReason::ArgumentError(
                "unexpected changes argument".into(),
            ));
        }
        let base = change_options
            .remove("--base")
            .ok_or_else(|| StopReason::ArgumentError("changes requires --base".into()))?;
        let phase = Phase::parse(
            &change_options
                .remove("--phase")
                .ok_or_else(|| StopReason::ArgumentError("changes requires --phase".into()))?,
        )
        .map_err(|error| {
            StopReason::ArgumentError(
                error
                    .to_string()
                    .strip_prefix("argument error: ")
                    .unwrap_or(&error.to_string())
                    .to_owned(),
            )
        })?;
        let head = change_options.remove("--head");
        let target = match (staged, head) {
            (true, None) if base == "HEAD" && phase == Phase::Implementation => Target::Index,
            (false, Some(head)) => Target::Commit(head),
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

    // REQ-core-004: "overview serve" は "--format" を受けない
    if serve && format_str.is_some() {
        return Err(StopReason::ArgumentError(
            "unexpected option for overview serve: --format".to_string(),
        ));
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
        // REQ-core-304: "overview" の後の位置引数は "build" か "serve" のちょうど1つ
        if command == "overview" {
            return match positionals.as_slice() {
                [sub] if sub == "build" => Ok(Cli::OverviewBuild {
                    format,
                    config_path,
                }),
                [sub] if sub == "serve" => Ok(Cli::OverviewServe {
                    port: port.as_deref().map_or(Ok(DEFAULT_PORT), parse_port)?,
                    config_path,
                }),
                _ => Err(StopReason::ArgumentError(format!(
                    "overview expects exactly one of build or serve, got: {}",
                    positionals.join(" ")
                ))),
            };
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
            if !kotowari::is_valid_id(id) {
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
                allow_test_findings,
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
    let tool = parse_tool(&tool).map_err(StopReason::ArgumentError)?;

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
            let result = run_changes(cwd, &base, target, phase, config_path.as_deref())?;
            match format {
                Format::Json => {
                    println!("{}", output::changes(&result))
                }
                Format::Text => print_findings_as_text(result.findings()),
            }
            Ok(exit_code_for(result.findings()))
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
            allow_test_findings,
        } => with_cwd(|cwd| {
            let (result, format) = run_check(cwd, format, config_path.as_deref())?;
            print_check(&result, format);
            Ok(check_exit_code(&result, allow_test_findings))
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
            Ok(u8::from(!result.complete()))
        }),
        Cli::Mutants {
            format,
            config_path,
            tool,
            results,
        } => with_cwd(|cwd| {
            let result = run_mutants(cwd, config_path.as_deref(), tool, &results)?;
            print_mutants(&result, format);
            Ok(exit_code_for(result.findings()))
        }),
        // REQ-core-293、REQ-core-295: 書き終えたら書いたファイルと消したファイルを出す
        Cli::OverviewBuild {
            format,
            config_path,
        } => with_cwd(|cwd| {
            let result = project(cwd, config_path.as_deref())?.overview_build()?;
            print_overview_build(&result, format);
            Ok(0)
        }),
        // REQ-core-297: 検査、ポートの確保、書き込みの順に行い、割り込みで終了コード0
        Cli::OverviewServe { port, config_path } => {
            with_cwd(|cwd| serve::run(&project(cwd, config_path.as_deref())?, port))
        }
        // REQ-core-196: 設定を読まず、計画書のファイルだけを読む
        Cli::Plan { format, path } => with_cwd(|cwd| {
            let result = run_plan(cwd, &path)?;
            print_plan(&result, format);
            Ok(exit_code_for(result.findings()))
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
    if findings.iter().any(|f| f.severity() == "error") {
        1
    } else {
        0
    }
}

/// TBL-core-002: "--allow-test-findings" があれば`テスト側の指摘`でない誤りだけを数える（REQ-core-357）
fn check_exit_code(result: &CheckReport, allow_test_findings: bool) -> u8 {
    if !allow_test_findings {
        return exit_code_for(result.findings());
    }
    u8::from(
        result
            .findings()
            .iter()
            .any(|f| f.severity() == "error" && !result.is_test_side_finding(f)),
    )
}

/// "kotowari check" の結果を出す（TBL-core-005, REQ-core-025, REQ-core-026）
fn print_check(result: &CheckReport, format: Format) {
    match format {
        Format::Json => println!("{}", output::check(result)),
        Format::Text => {
            print_findings_as_text(result.findings());
            // REQ-core-228: 指摘の行の後の最後の1行。指摘が0件でも出す
            if let Some(surface) = result.surface() {
                println!("surface: unspecified={}", surface.unspecified());
            }
        }
    }
}

/// "kotowari list" の一覧を出す（REQ-core-155）
fn print_list(result: &ReadList, format: Format) {
    match format {
        Format::Json => println!("{}", output::list(result)),
        Format::Text => list::print_text(result),
    }
}

/// "kotowari query" の1件を出す（REQ-core-161）
fn print_query(result: &QueryReport, format: Format) {
    match format {
        Format::Json => println!("{}", output::query(result)),
        Format::Text => query::print_text(result),
    }
}

/// "kotowari status" の集計を出す（REQ-core-166）
fn print_status(result: &StatusReport, format: Format) {
    match format {
        Format::Json => println!("{}", output::status(result)),
        Format::Text => status::print_text(result),
    }
}

/// "kotowari mutants" の結果を出す（TBL-core-025, REQ-core-146）
fn print_mutants(result: &MutantsReport, format: Format) {
    match format {
        Format::Json => println!("{}", output::mutants(result)),
        Format::Text => {
            print_findings_as_text(result.findings());
            // REQ-core-146: 指摘の行の後の最後の1行。指摘が0件でも出す
            println!("{}", result.mutants().summary_line());
        }
    }
}

/// "kotowari overview build" の結果を出す（REQ-core-295）
fn print_overview_build(result: &kotowari::OverviewBuild, format: Format) {
    match format {
        Format::Json => println!("{}", output::overview_build(result)),
        Format::Text => {
            for path in result.written() {
                println!("written {}", one_line(path));
            }
            for path in result.removed() {
                println!("removed {}", one_line(path));
            }
        }
    }
}

/// "kotowari plan" の結果を出す（REQ-core-194、REQ-core-025）
fn print_plan(result: &PlanReport, format: Format) {
    match format {
        Format::Json => println!("{}", output::plan(result)),
        Format::Text => print_findings_as_text(result.findings()),
    }
}

/// REQ-core-025, REQ-core-026: 1つの指摘を1行で出し、"line" が null なら "-" と書く
fn print_findings_as_text(findings: &[Finding]) {
    for f in findings {
        let line = f.line().map_or("-".to_string(), |l| l.to_string());
        println!(
            "{}:{} [{}] {} {}",
            one_line(f.path()),
            line,
            f.severity(),
            f.kind(),
            one_line(f.detail())
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
    println!("  overview   build: write the overview pages; serve: write and show them locally");
    println!("  plan       Check the form of one plan file against the bundled schema");
    println!("  query      Show one item or scenario with its body and back references");
    println!("  status     Summarise the IR and tell whether it is complete");
    println!();
    println!("Changes: --base <REV> (--head <REV> | --staged) --phase <implementation|review>");
    println!("Options:");
    println!("  --format <FORMAT>  Output format: json (default) or text");
    println!("  --config <PATH>    Path to configuration file");
    println!("  --tool <TOOL>      Mutation testing tool of the result file: cargo-mutants");
    println!("  --port <PORT>      Port of overview serve on 127.0.0.1 (default 4590)");
    println!("  --allow-test-findings  check: exit 0 when every error is a test-side finding");
    println!("  --help             Show this help message");
    println!("  --version          Show version");
}

fn project(cwd: &Path, config: Option<&Path>) -> Result<Project, StopReason> {
    let mut options = ProjectOptions::new(cwd);
    options.config = config.map(Path::to_path_buf);
    Ok(Project::new(options)?)
}
fn run_check(
    cwd: &Path,
    format: Format,
    config: Option<&Path>,
) -> Result<(CheckReport, Format), StopReason> {
    Ok((project(cwd, config)?.check()?, format))
}
fn run_list(cwd: &Path, config: Option<&Path>) -> Result<ReadList, StopReason> {
    Ok(project(cwd, config)?.list()?)
}
fn run_query(cwd: &Path, config: Option<&Path>, id: &str) -> Result<QueryReport, StopReason> {
    Ok(project(cwd, config)?.query(id)?)
}
fn run_status(cwd: &Path, config: Option<&Path>) -> Result<StatusReport, StopReason> {
    Ok(project(cwd, config)?.status()?)
}
fn run_plan(cwd: &Path, path: &Path) -> Result<PlanReport, StopReason> {
    Ok(project(cwd, None)?.plan(path)?)
}
fn run_mutants(
    cwd: &Path,
    config: Option<&Path>,
    tool: Tool,
    results: &Path,
) -> Result<MutantsReport, StopReason> {
    Ok(project(cwd, config)?.mutants(&kotowari::MutantsOptions {
        tool,
        results: results.into(),
    })?)
}
fn run_changes(
    cwd: &Path,
    base: &str,
    target: Target,
    phase: Phase,
    config: Option<&Path>,
) -> Result<ChangesReport, StopReason> {
    Ok(project(cwd, config)?.changes(&kotowari::ChangesOptions {
        base: base.into(),
        target,
        phase,
    })?)
}
