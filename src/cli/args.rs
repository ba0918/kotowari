//! 引数の解析（REQ-core-001、REQ-core-002、REQ-core-004、REQ-core-107）
use super::StopReason;
use kotowari::Tool;
use std::path::PathBuf;

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
const COMMANDS: [&str; 7] = [
    "check", "list", "mutants", "overview", "plan", "query", "status",
];

/// serve の既定のポート（REQ-core-297）
pub(super) const DEFAULT_PORT: u16 = 4590;

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
    let mut config: Option<String> = None;
    let mut tool: Option<String> = None;
    let mut port: Option<String> = None;
    let mut command: Option<String> = None;
    let mut positionals: Vec<String> = Vec::new();
    let mut allow_test_findings = false;
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
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
        if arg.starts_with("--") {
            let slot = match arg.as_str() {
                "--format" => &mut format_str,
                "--config" => &mut config,
                "--tool" => &mut tool,
                "--port" => &mut port,
                _ => {
                    return Err(StopReason::ArgumentError(format!("unknown option: {arg}")));
                }
            };
            if slot.is_some() {
                return Err(StopReason::ArgumentError(format!("repeated option: {arg}")));
            }
            i += 1;
            let Some(value) = args.get(i) else {
                return Err(StopReason::ArgumentError(format!("{arg} requires a value")));
            };
            *slot = Some(value.clone());
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
            "expected command: check, list, mutants, overview, plan, query or status".to_string(),
        ));
    };
    let config_path = config.map(PathBuf::from);

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
    // REQ-core-190: plan は設定を読まないので "--config" を受けない。指す先を見る前に止める
    if command == "plan" && config_path.is_some() {
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
