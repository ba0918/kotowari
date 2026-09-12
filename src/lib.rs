pub mod config;

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

/// 指摘
#[derive(serde::Serialize, Clone)]
pub struct Finding {
    pub kind: String,
    pub severity: String,
    pub path: String,
    pub line: Option<usize>,
    pub detail: String,
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
        let bytes = std::fs::read(&abs)
            .map_err(|e| StopReason::UnreadableFile(format!("{}: {e}", abs.display())))?;
        let text = String::from_utf8(bytes)
            .map_err(|_| StopReason::NonUtf8File(format!("{}", abs.display())))?;
        config::Config::parse(&text)?
    } else {
        // 既定: base/.kotowari/config.yaml
        let default_path = base.join(".kotowari/config.yaml");
        if default_path.exists() {
            let bytes = std::fs::read(&default_path)
                .map_err(|e| StopReason::UnreadableFile(format!("{}: {e}", default_path.display())))?;
            let text = String::from_utf8(bytes)
                .map_err(|_| StopReason::NonUtf8File(format!("{}", default_path.display())))?;
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

    // REQ-018: 置き場が無いとき停止
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

    // Step 1: 空の結果を返す
    let result = CheckResult {
        files: 0,
        lines: 0,
        findings: vec![],
        counts: BTreeMap::new(),
    };

    Ok((result, format))
}
