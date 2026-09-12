use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // 引数の解析
    let mut format_str: Option<String> = None;
    let mut config_path: Option<PathBuf> = None;
    let mut saw_check = false;
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        if !saw_check {
            if arg == "check" {
                saw_check = true;
                i += 1;
                continue;
            } else {
                stop(&format!("unknown command: {arg}"));
                return ExitCode::from(2);
            }
        }

        if arg.starts_with("--") {
            match arg.as_str() {
                "--format" => {
                    i += 1;
                    if i >= args.len() {
                        stop("--format requires a value");
                        return ExitCode::from(2);
                    }
                    format_str = Some(args[i].clone());
                }
                "--config" => {
                    i += 1;
                    if i >= args.len() {
                        stop("--config requires a value");
                        return ExitCode::from(2);
                    }
                    config_path = Some(PathBuf::from(&args[i]));
                }
                _ => {
                    stop(&format!("unknown option: {arg}"));
                    return ExitCode::from(2);
                }
            }
        } else {
            stop(&format!("unexpected argument: {arg}"));
            return ExitCode::from(2);
        }
        i += 1;
    }

    if !saw_check {
        stop("expected command: check");
        return ExitCode::from(2);
    }

    // format の解析
    let format = match format_str.as_deref().unwrap_or("json") {
        "json" => kotowari::Format::Json,
        "text" => kotowari::Format::Text,
        other => {
            stop(&format!("unknown format value: {other}"));
            return ExitCode::from(2);
        }
    };

    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            stop(&format!("cannot get current directory: {e}"));
            return ExitCode::from(2);
        }
    };

    match kotowari::run_check(&cwd, format, config_path.as_deref()) {
        Ok((result, fmt)) => {
            match fmt {
                kotowari::Format::Json => {
                    let json = serde_json::to_string(&result).unwrap();
                    println!("{json}");
                }
                kotowari::Format::Text => {
                    for f in &result.findings {
                        let line_str = f.line.map_or("-".to_string(), |l| l.to_string());
                        let severity = &f.severity;
                        println!(
                            "{}:{} [{}] {} {}",
                            f.path, line_str, severity, f.kind, f.detail
                        );
                    }
                }
            }
            if result.findings.iter().any(|f| f.severity == "error") {
                ExitCode::from(1)
            } else {
                ExitCode::from(0)
            }
        }
        Err(reason) => {
            stop(&reason.to_string());
            ExitCode::from(2)
        }
    }
}

fn stop(reason: &str) {
    eprintln!("{reason}");
}
