use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let cli = match kotowari::parse_args(&args) {
        Ok(c) => c,
        Err(reason) => {
            stop(&reason.to_string());
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

    match kotowari::run_check(&cwd, cli.format, cli.config_path.as_deref()) {
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
