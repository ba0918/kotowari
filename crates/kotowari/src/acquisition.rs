use crate::{
    change_records, guides, ir, overview, sources, surface, test_files as tests_discovery,
};
use kotowari_core::*;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
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

/// 計画書の検査のエントリポイント（REQ-core-196）。設定、IR、判断の記録、テストのファイルを読まない
pub fn run_plan(cwd: &Path, path: &Path) -> Result<plan::PlanResult, StopReason> {
    let base = find_base(cwd);
    // 計画書のパスはカレントディレクトリからの相対（REQ-core-190）、
    // 指摘と停止の path は基準のディレクトリからの相対（REQ-core-193、TBL-core-020）
    let display = display_from_base(&base, cwd, path);
    // REQ-core-197: 無い、ディレクトリ、読めないは読めないファイル、UTF-8 でなければ UTF-8 でないファイル
    let text = read_utf8_file(&cwd.join(path), &display)?;
    Ok(plan::check(&display, &text))
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

    let sources = read_sources(&base, list.entries());
    Ok(mutants::inspect(&outcomes, &list, &list_path, &sources))
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
        kotowari_core::ir::split_lines(strip_bom(&text))
            .into_iter()
            .map(str::to_string)
            .collect(),
    )
}

/// `設定ファイル`を読む（REQ-core-003, REQ-core-011, REQ-core-012。TBL-core-020: 詳細のパスは基準からの相対）
pub fn load_config(
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

/// 設定と置き場から IR の文書とテストのファイルを読み、検査もする。
/// check はこの指摘を出し、list と query は捨て、status は数だけを出す（REQ-core-151、REQ-core-156、REQ-core-162）。
/// `ガイド`はここでは読まない。list と query は`ガイド`を読まない（REQ-core-152、REQ-core-158）ので、
/// check と status だけが `read_guides` を続けて呼ぶ
pub fn load_all(cwd: &Path, config_path: Option<&Path>) -> Result<ReadModel, StopReason> {
    load_read(cwd, config_path).map(|(read, _)| read)
}

/// `load_all` と、読んだ`テストのファイル`の`基準のディレクトリ`からの相対パス
fn load_read(
    cwd: &Path,
    config_path: Option<&Path>,
) -> Result<(ReadModel, Vec<String>), StopReason> {
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

    let preparation = ir::prepare(&base, &cfg)?;
    let (records, adr) = sources::read_texts(&base, &cfg)?;
    let tests = tests_discovery::analyze(&base, &cfg)?;
    let test_paths = tests
        .iter()
        .map(|test| test.source.path().to_string())
        .collect();
    Ok((preparation.finish(records, adr, tests)?, test_paths))
}

/// check と status の読み取り: `load_all` に続けて`ガイド`と`全体像の元データ`と`面`を読み、その`指摘`を足す
/// （REQ-core-198、REQ-core-162、REQ-core-229、REQ-core-278、REQ-core-290）
pub fn load_with_guides(cwd: &Path, config_path: Option<&Path>) -> Result<Inspection, StopReason> {
    let (read, tests) = load_read(cwd, config_path)?;
    let base = find_base(cwd);
    let changes = change_records::read_texts(&base, read.config())?;
    // REQ-core-280: ガイドとテストの重なり（REQ-core-199）を先に判定する
    let guides = guides::read_texts(&base, &read)?;
    let guide_paths: Vec<&str> = guides
        .iter()
        .flatten()
        .map(kotowari_core::NativeSourceText::path)
        .collect();
    let overview_texts = overview::read_texts(&base, read.config(), &guide_paths, &tests)?;
    let overview = overview::group(&read, overview_texts);
    let mut preparation = read.prepare_repository_inspection();
    preparation.changes(changes)?;
    preparation.guides(guides)?;
    let (surface, unspecified) = surface::analyze(&base, preparation.config())?;
    preparation.surface(surface, unspecified)?;
    preparation.group(overview)?;
    preparation.finish()
}

/// build と serve の読み取り: check と同じ設定と置き場から IR と判断の記録と`テストのファイル`を読み、
/// `全体像の元データ`を読んで検査し描画の入力を作る（REQ-core-278、REQ-core-280）。
/// "overview" の鍵が無ければ設定の誤りで止まる（REQ-core-279）。ガイドの中身と面と照合記録は読まない
pub fn load_overview(
    cwd: &Path,
    config_path: Option<&Path>,
) -> Result<(PathBuf, kotowari_overview::Overview), StopReason> {
    let base = find_base(cwd);
    if load_config(cwd, &base, config_path)?.overview.is_none() {
        return Err(overview::not_configured());
    }
    let (read, tests) = load_read(cwd, config_path)?;
    // REQ-core-280: ガイドとテストの重なり（REQ-core-199）を先に判定する
    let guides = if read.config().guides.files.is_empty() {
        Vec::new()
    } else {
        tests_discovery::collect_files(&base, &read.config().guides.files)?
    };
    read.validate_guide_paths(guides.iter().map(|(path, _)| path.as_str()))?;
    let guide_paths: Vec<&str> = guides.iter().map(|(path, _)| path.as_str()).collect();
    let texts = overview::read_texts(&base, read.config(), &guide_paths, &tests)?
        .ok_or_else(overview::not_configured)?;
    Ok((base, kotowari_overview::inspect(&read, &texts)))
}
