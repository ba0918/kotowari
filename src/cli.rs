use kotowari::{
    CheckReport, Finding, MutantsReport, PlanReport, Project, ProjectOptions, QueryReport,
    ReadList, StatusReport,
};
use std::path::{Path, PathBuf};
mod args;
use args::{Cli, Format, parse_args};
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
            let result = project(cwd, config_path.as_deref())?.check()?;
            print_check(&result, format);
            Ok(check_exit_code(&result, allow_test_findings))
        }),
        // REQ-core-151: check と同じ読み取りを通し、指摘は出さず、読めれば終了コードは 0
        Cli::List {
            format,
            config_path,
        } => with_cwd(|cwd| {
            let result = project(cwd, config_path.as_deref())?.list()?;
            print_list(&result, format);
            Ok(0)
        }),
        // REQ-core-156: check と同じ読み取りを通し、指摘は出さず、読めれば終了コードは 0
        Cli::Query {
            format,
            config_path,
            id,
        } => with_cwd(|cwd| {
            let result = project(cwd, config_path.as_deref())?.query(&id)?;
            print_query(&result, format);
            Ok(0)
        }),
        // REQ-core-162: check と同じ検査を走らせ、指摘は出さず集計だけを出す
        Cli::Status {
            format,
            config_path,
        } => with_cwd(|cwd| {
            let result = project(cwd, config_path.as_deref())?.status()?;
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
            let result = project(cwd, config_path.as_deref())?
                .mutants(&kotowari::MutantsOptions { tool, results })?;
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
            let result = project(cwd, None)?.plan(&path)?;
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
    println!("  check      Check IR documents and test markers");
    println!("  list       List IR items and the tests marked for them");
    println!("  mutants    Read a mutation testing result file and report survivors");
    println!("  overview   build: write the overview pages; serve: write and show them locally");
    println!("  plan       Check the form of one plan file against the bundled schema");
    println!("  query      Show one item or scenario with its body and back references");
    println!("  status     Summarise the IR and tell whether it is complete");
    println!();
    println!("Options:");
    println!("  --format <FORMAT>      Output format: json (default) or text");
    println!("  --config <PATH>        Path to configuration file");
    println!("  --tool <TOOL>          Mutation testing tool of the result file: cargo-mutants");
    println!("  --port <PORT>          Port of overview serve on 127.0.0.1 (default 4590)");
    println!("  --allow-test-findings  check: exit 0 when every error is a test-side finding");
    println!("  --help                 Show this help message");
    println!("  --version              Show version");
}

fn project(cwd: &Path, config: Option<&Path>) -> Result<Project, StopReason> {
    let mut options = ProjectOptions::new(cwd);
    options.config = config.map(Path::to_path_buf);
    Ok(Project::new(options)?)
}
