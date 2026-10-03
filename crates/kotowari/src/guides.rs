use crate::acquisition::read_utf8_file;
use kotowari_core::guides::*;
use kotowari_core::ir::IrDocument;
use kotowari_core::{Finding, StopReason, config::Config};
use std::path::Path;
/// "guides.files" に当たるファイルを`ガイド`として読み、`ガイドの印`を今の`IR`の`指紋`と照らす。
/// `テストのファイル`と重なるファイルがあれば、バイト順で最初の1つを詳細にして設定の誤りで停止する（REQ-core-199）。
/// `test_files` はバイト順に並んだ`テストのファイル`の相対パス
pub fn read_guides(
    base: &Path,
    cfg: &Config,
    test_files: &[String],
    docs: &[IrDocument],
    findings: &mut Vec<Finding>,
) -> Result<GuideTally, StopReason> {
    // 空の一覧ならガイドは1つも読まない（REQ-core-198）。走査そのものを省く
    if cfg.guides.files.is_empty() {
        return Ok(GuideTally::default());
    }
    let files = crate::test_files::collect_files(base, &cfg.guides.files)?;
    // files はバイト順なので、最初に見つかる重なりがバイト順で最初の1つ
    if let Some((overlap, _)) = files
        .iter()
        .find(|(rel, _)| test_files.binary_search(rel).is_ok())
    {
        return Err(StopReason::ConfigError(format!(
            "{overlap}: matched by both guides.files and tests.files"
        )));
    }
    let mut sources = Vec::new();
    for (rel, abs) in &files {
        let content = read_utf8_file(Path::new(abs), rel)?;
        sources.push((rel.as_str(), content));
    }
    check_entries(
        sources.iter().map(|(path, text)| (*path, text.as_str())),
        test_files,
        docs,
        findings,
    )
}
