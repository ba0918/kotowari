//! `ガイド`を読む（REQ-core-198、REQ-core-199、REQ-core-206）

use crate::config::Config;
use crate::{Finding, StopReason};
use std::path::Path;

/// "kotowari check" の "guides"（REQ-core-206、TBL-core-005）
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize)]
pub struct GuideTally {
    /// 読んだ`ガイド`の数。パスごとに1回
    pub files: usize,
    /// 形の正しい`ガイドの印`の1件の数
    pub marks: usize,
}

/// "guides.files" に当たるファイルを`ガイド`として読む。
/// `テストのファイル`と重なるファイルがあれば、バイト順で最初の1つを詳細にして設定の誤りで停止する（REQ-core-199）。
/// `test_files` はバイト順に並んだ`テストのファイル`の相対パス
pub fn read_guides(
    base: &Path,
    cfg: &Config,
    test_files: &[String],
    _findings: &mut Vec<Finding>,
) -> Result<GuideTally, StopReason> {
    let files = crate::tests_discovery::collect_files(base, &cfg.guides.files)?;
    // files はバイト順なので、最初に見つかる重なりがバイト順で最初の1つ
    if let Some((overlap, _)) = files
        .iter()
        .find(|(rel, _)| test_files.binary_search(rel).is_ok())
    {
        return Err(StopReason::ConfigError(overlap.clone()));
    }
    for (rel, abs) in &files {
        crate::read_utf8_file(Path::new(abs), rel)?;
    }
    Ok(GuideTally {
        files: files.len(),
        marks: 0,
    })
}
