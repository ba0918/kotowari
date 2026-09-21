//! cargo-mutants の結果のファイルを`変異の結果`に写す（REQ-core-138、TBL-core-024）。
//! 道具に固有の鍵と値を知るのはこのモジュールだけで、写した先はどの語も知らない。

use crate::mutants::{normalize_source_path, MutantOutcome, MutantResult};
use serde_json::Value;

/// 基準の実行を表す "scenario" の値
const BASELINE: &str = "Baseline";
/// 基準の実行の成功を表す "summary" の値
const BASELINE_SUCCESS: &str = "Success";

/// 結果のファイルの中身を`変異の結果`の並びに写す。
/// 写せないときは誤りの説明を返す（停止の理由と結果のファイルのパスは呼び出す側が付ける）。
pub fn read_outcomes(text: &str) -> Result<Vec<MutantOutcome>, String> {
    let root: Value =
        serde_json::from_str(text).map_err(|e| format!("not readable as JSON: {e}"))?;
    let entries = root
        .get("outcomes")
        .and_then(Value::as_array)
        .ok_or_else(|| "\"outcomes\" is missing or not an array".to_string())?;

    let mut outcomes = Vec::new();
    for entry in entries {
        let summary = string_at(entry, "summary", "summary")?;
        let scenario = entry
            .get("scenario")
            .ok_or_else(|| "\"scenario\" is missing".to_string())?;

        // TBL-core-024: 基準の実行は変異の結果にしない。成功でなければ結果の誤り
        if scenario.as_str() == Some(BASELINE) {
            if summary != BASELINE_SUCCESS {
                return Err(format!("the baseline run did not succeed: {summary}"));
            }
            continue;
        }

        let mutant = scenario
            .get("Mutant")
            .ok_or_else(|| "\"scenario.Mutant\" is missing".to_string())?;
        outcomes.push(read_mutant(mutant, summary)?);
    }
    Ok(outcomes)
}

/// 変異の1件を写す（TBL-core-024）
fn read_mutant(mutant: &Value, summary: &str) -> Result<MutantOutcome, String> {
    let file = string_at(mutant, "file", "scenario.Mutant.file")?;
    let line = number(mutant, "line")?;
    let column = number(mutant, "column")?;
    let name = string_at(mutant, "name", "scenario.Mutant.name")?;

    if line < 1 {
        return Err(format!("the line is not 1 or greater: {line}"));
    }

    // TBL-core-024: 変更の説明は、名前から「ファイル:行:列: 」の前置きを除いた残り
    let prefix = format!("{file}:{line}:{column}: ");
    let change = name.strip_prefix(&prefix).ok_or_else(|| {
        format!("\"scenario.Mutant.name\" does not start with {prefix:?}: {name:?}")
    })?;

    Ok(MutantOutcome {
        file: normalize_source_path(file)
            .map_err(|e| format!("\"scenario.Mutant.file\" is {e}"))?,
        line: line as usize,
        change: change.to_string(),
        result: read_result(summary)?,
    })
}

/// 文字列の鍵を引く。無いか文字列でなければ誤りの説明を返す（TBL-core-020: 詳細は誤りの説明）
fn string_at<'a>(value: &'a Value, key: &str, shown: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{shown:?} is missing or not a string"))
}

/// "span.start" の下の数の鍵を読む
fn number(mutant: &Value, key: &str) -> Result<u64, String> {
    mutant
        .pointer(&format!("/span/start/{key}"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!("\"scenario.Mutant.span.start.{key}\" is missing or not a number")
        })
}

/// TBL-core-024: "summary" の4つの値を結果に写す。ほかの値は結果の誤り
fn read_result(summary: &str) -> Result<MutantResult, String> {
    match summary {
        "CaughtMutant" => Ok(MutantResult::Caught),
        "MissedMutant" => Ok(MutantResult::Survived),
        "Timeout" => Ok(MutantResult::Timeout),
        "Unviable" => Ok(MutantResult::Unviable),
        other => Err(format!("unknown outcome: {other}")),
    }
}
