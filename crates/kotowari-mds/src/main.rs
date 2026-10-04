use clap::{Parser, Subcommand};
use kotowari_markdown_schema::finding::Finding;
use kotowari_markdown_schema_io::{LoaderOptions, SchemaLoader};
use serde::Serialize;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "kotowari-mds",
    version,
    about = "Validate Markdown documents against a YAML schema declared in their frontmatter and extract structured values"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate a document or every schema-declaring document under a directory
    Check {
        /// File or directory to check
        path: PathBuf,
        /// Output format
        #[arg(long, value_parser = ["json", "text"], default_value = "text")]
        format: String,
        /// Allow undeclared headings and lines (overrides the schema's `open`)
        #[arg(long)]
        open: bool,
    },
    /// Output the raw syntax tree (mdast) of a document as JSON
    Ast {
        /// Markdown file to parse
        file: PathBuf,
        /// Output a typed tree built from the schema's extract rules
        #[arg(long)]
        schema: bool,
        /// Output format (json only)
        #[arg(long, value_parser = ["json"])]
        format: Option<String>,
    },
    /// Extract the values declared by the schema
    Values {
        /// Markdown file to extract from
        file: PathBuf,
        /// Output format
        #[arg(long, value_parser = ["json", "text"], default_value = "text")]
        format: String,
    },
}

struct Stop {
    kind: &'static str,
    detail: String,
}

impl From<kotowari_markdown_schema_io::Error> for Stop {
    fn from(error: kotowari_markdown_schema_io::Error) -> Self {
        Self {
            kind: error.kind().as_str(),
            detail: error.detail().to_owned(),
        }
    }
}

impl std::fmt::Display for Stop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Parser diagnostics can quote arbitrary file contents; the CLI contract permits only one line.
        write!(
            f,
            "{}: {}",
            self.kind,
            self.detail.lines().next().unwrap_or_default()
        )
    }
}

#[derive(Serialize)]
struct FindingJson<'a> {
    kind: &'a str,
    severity: &'a str,
    path: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_kind: Option<&'a str>,
    detail: &'a str,
}

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            match error.kind() {
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    let _ = error.print();
                    return ExitCode::SUCCESS;
                }
                clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => {
                    eprintln!(
                        "kotowari-mds: argument_error: no subcommand given (see `kotowari-mds --help`)"
                    );
                }
                _ => {
                    let message = error.to_string();
                    let first = message.lines().next().unwrap_or("invalid arguments");
                    eprintln!(
                        "kotowari-mds: argument_error: {}",
                        first.strip_prefix("error: ").unwrap_or(first)
                    );
                }
            }
            return ExitCode::from(2);
        }
    };
    match run(cli) {
        Ok(code) => ExitCode::from(code),
        Err(stop) => {
            eprintln!("kotowari-mds: {stop}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<u8, Stop> {
    let start = std::env::current_dir().map_err(|error| Stop {
        kind: "unreadable_file",
        detail: error.to_string(),
    })?;
    let loader = SchemaLoader::new(LoaderOptions::new(start))?;
    match cli.command {
        Command::Ast {
            file,
            schema,
            format: _,
        } => {
            let json = if schema {
                loader
                    .load(&file)?
                    .extract_typed_partial(Default::default())
                    .values()
                    .clone()
            } else {
                let bytes = std::fs::read(&file).map_err(|error| Stop {
                    kind: "unreadable_file",
                    detail: format!("{}: {error}", file.display()),
                })?;
                let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(&bytes);
                let source = std::str::from_utf8(bytes).map_err(|error| Stop {
                    kind: "unreadable_file",
                    detail: format!("{}: {error}", file.display()),
                })?;
                kotowari_markdown_schema::ast_json(source).map_err(|error| Stop {
                    kind: "unreadable_file",
                    detail: format!("{}: {error}", file.display()),
                })?
            };
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
            Ok(0)
        }
        Command::Values { file, format } => {
            let pair = loader.load(&file)?;
            let partial = pair.extract_partial(Default::default());
            match format.as_str() {
                "json" => println!(
                    "{}",
                    serde_json::to_string_pretty(partial.values()).unwrap()
                ),
                "text" => print!("{}", render_values_text(partial.values())),
                _ => unreachable!("clap restricts format"),
            }
            Ok(0)
        }
        Command::Check { path, format, open } => {
            let result = loader.check(
                &path,
                kotowari_markdown_schema::ValidationOptions { relax: open },
            )?;
            let files: Vec<_> = result
                .files()
                .iter()
                .map(|file| (file.path().to_path_buf(), file.findings()))
                .collect();
            emit_check(&files, &format);
            Ok(u8::from(
                files.iter().any(|(_, findings)| !findings.is_empty()),
            ))
        }
    }
}

fn emit_check(files: &[(PathBuf, &[Finding])], format: &str) {
    match format {
        "text" => {
            for (path, findings) in files {
                for finding in *findings {
                    match finding.line() {
                        Some(line) => println!(
                            "{}:{line}: {}: {}",
                            path.display(),
                            finding.kind().as_str(),
                            finding.detail()
                        ),
                        None => println!(
                            "{}: {}: {}",
                            path.display(),
                            finding.kind().as_str(),
                            finding.detail()
                        ),
                    }
                }
            }
        }
        "json" => {
            let files_json: Vec<_> = files
                .iter()
                .filter(|(_, findings)| !findings.is_empty())
                .map(|(path, findings)| {
                    let path = path.to_string_lossy();
                    let findings: Vec<_> = findings
                        .iter()
                        .map(|finding| FindingJson {
                            kind: finding.kind().as_str(),
                            severity: "error",
                            path: &path,
                            line: finding.line(),
                            node: finding.node(),
                            text: finding.raw(),
                            rule_kind: finding.rule_kind().map(|kind| kind.as_str()),
                            detail: finding.detail(),
                        })
                        .collect();
                    serde_json::json!({"path": path, "findings": findings})
                })
                .collect();
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({"files": files_json})).unwrap()
            );
        }
        _ => unreachable!("clap restricts format"),
    }
}

fn render_values_text(value: &serde_json::Value) -> String {
    let mut out = String::new();
    render_text(value, &mut out, 0);
    out
}

fn render_text(value: &serde_json::Value, out: &mut String, indent: usize) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                match value {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        push_indent(out, indent);
                        out.push_str(key);
                        out.push_str(":\n");
                        render_text(value, out, indent + 2);
                    }
                    serde_json::Value::String(text) => {
                        push_indent(out, indent);
                        out.push_str(key);
                        out.push_str(": ");
                        push_string(out, indent + 2, text);
                        out.push('\n');
                    }
                    _ => {
                        push_indent(out, indent);
                        out.push_str(&format!("{key}: {value}\n"));
                    }
                }
            }
        }
        serde_json::Value::Array(values) => {
            for (i, value) in values.iter().enumerate() {
                match value {
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        push_indent(out, indent);
                        out.push_str(&format!("{}.\n", i + 1));
                        render_text(value, out, indent + 2);
                    }
                    serde_json::Value::String(text) => {
                        push_indent(out, indent);
                        out.push_str(&format!("{}. ", i + 1));
                        push_string(out, indent + 2, text);
                        out.push('\n');
                    }
                    other => {
                        push_indent(out, indent);
                        out.push_str(&format!("{}. {other}\n", i + 1));
                    }
                }
            }
        }
        _ => {}
    }
}

fn push_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push(' ');
    }
}
fn push_string(out: &mut String, indent: usize, text: &str) {
    for (i, line) in text.trim_end_matches('\n').split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
            push_indent(out, indent);
        }
        out.push_str(line);
    }
}
