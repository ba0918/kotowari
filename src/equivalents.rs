//! `等価の一覧`の読み取り（REQ-143、REQ-148）。一覧のファイルの形を知るのはこのモジュールだけ。

use crate::mutants::{normalize_source_path, trim_spaces_and_tabs};
use crate::{Finding, FindingKind, StopReason};
use serde_json::Value;

/// 1件が持つ鍵。この5つちょうどでなければ形の誤り（REQ-143）
const KEYS: [&str; 5] = ["file", "change", "text", "class", "why"];

/// 一覧に書ける分類（A17: 「等価」だけ）
const EQUIVALENT: &str = "equivalent";

/// 形の正しい`等価の一覧`の1件
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Equivalent {
    /// REQ-110 の正規化を掛けた "file"（比べるときに使う）
    pub file: String,
    /// 変更の説明
    pub change: String,
    /// 変異が入る行の元の文面
    pub text: String,
    /// A57: 一覧に書かれたままの "file" と "change" を ": " でつないだ detail
    pub detail: String,
}

/// 一覧を読んだ結果
#[derive(Debug, Default)]
pub struct EquivalentList {
    /// 形の正しい1件
    pub entries: Vec<Equivalent>,
    /// 形の誤った1件への equivalent_invalid（A53: この1件には equivalent_stale を出さない）
    pub findings: Vec<Finding>,
}

/// 一覧の中身を読む。`display` は一覧のファイルの`基準のディレクトリ`からの相対パス。
/// YAML として読めないか最上位が並びでないときは設定の誤りで`停止`する（REQ-148）。
pub fn read_list(text: &str, display: &str) -> Result<EquivalentList, StopReason> {
    // REQ-148: 空（0バイトか注釈だけ）の一覧は0件
    if crate::config::is_blank_yaml(text) {
        return Ok(EquivalentList {
            entries: Vec::new(),
            findings: Vec::new(),
        });
    }

    let root: Value = serde_saphyr::from_str(text)
        .map_err(|e| StopReason::ConfigError(format!("{display}: {e}")))?;
    let items = root.as_array().ok_or_else(|| {
        StopReason::ConfigError(format!("{display}: the list is not a sequence"))
    })?;

    let mut entries = Vec::new();
    let mut findings = Vec::new();
    for item in items {
        match read_entry(item) {
            Ok(entry) => entries.push(entry),
            // REQ-143: 形の誤った1件は1件ごとに誤りを出し、どの変異の結果とも一致させない
            Err(detail) => findings.push(Finding::new(
                FindingKind::EquivalentInvalid,
                display.to_string(),
                None,
                detail,
            )),
        }
    }
    Ok(EquivalentList { entries, findings })
}

/// 1件を読む。形が正しくなければ equivalent_invalid の detail を返す（REQ-143）
fn read_entry(item: &Value) -> Result<Equivalent, String> {
    let detail = written_detail(item);

    // 鍵と値の組でない、鍵が足りない、またはこの5つ以外の鍵を持つ
    let Some(map) = item.as_object() else {
        return Err(detail);
    };
    if map.len() != KEYS.len() || !KEYS.iter().all(|key| map.contains_key(*key)) {
        return Err(detail);
    }

    // 値が文字列でない
    let mut values = Vec::new();
    for key in KEYS {
        match map[key].as_str() {
            Some(value) => values.push(value),
            None => return Err(detail),
        }
    }
    let [file, change, text, class, why] = values[..] else {
        return Err(detail);
    };

    if class != EQUIVALENT {
        return Err(detail);
    }
    // A53: "why" は前後の半角空白とタブを除いて空なら空
    if trim_spaces_and_tabs(why).is_empty() {
        return Err(detail);
    }
    // A51: 絶対パスか ".." の要素を含む "file" は基準のディレクトリの外を指す
    let Ok(file) = normalize_source_path(file) else {
        return Err(detail);
    };

    Ok(Equivalent {
        file,
        change: change.to_string(),
        text: text.to_string(),
        detail,
    })
}

/// A57、REQ-143: 書かれたままの "file" と "change" を ": " でつなぐ。
/// 無いか文字列でない方は空の文字列にする
fn written_detail(item: &Value) -> String {
    let written = |key: &str| item.get(key).and_then(Value::as_str).unwrap_or("");
    format!("{}: {}", written("file"), written("change"))
}
