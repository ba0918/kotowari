//! 変異の結果の形（A13、A56）。変異テストの道具に固有の語は持たない（REQ-150）。
//! 道具の結果のファイルからの写し取りは道具ごとのモジュール（`crate::cargo_mutants`）が行う。

/// 変異1件の結果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutantResult {
    /// 捕まえた（変異を入れるとテストが落ちた）
    Caught,
    /// 見逃した（変異を入れてもテストが全部通った）
    Survived,
    /// 時間切れ
    Timeout,
    /// ビルド不能
    Unviable,
}

/// 変異の結果。ファイル、行、変更の説明、結果の4つを持つ
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutantOutcome {
    /// 変異の入ったファイル（`基準のディレクトリ`からの相対パス。REQ-110 の正規化済み）
    pub file: String,
    /// 変異の入った行（1始まり）
    pub line: usize,
    /// 変更の説明（道具が出した文のまま）
    pub change: String,
    /// 結果
    pub result: MutantResult,
}

/// ソースのパスに REQ-110 の正規化を掛ける。
/// 絶対パスと ".." の要素を含むものは`基準のディレクトリ`の外を指すので受けない（A51）。
/// 正規化は先頭の "/" を落とすので、判定は正規化の前に行う。
pub fn normalize_source_path(raw: &str) -> Result<String, String> {
    let slashed = raw.replace('\\', "/");
    if slashed.starts_with('/') {
        return Err(format!("absolute path: {raw}"));
    }
    if slashed.split('/').any(|part| part == "..") {
        return Err(format!("path outside the base directory: {raw}"));
    }
    Ok(crate::normalize_path(&slashed))
}
