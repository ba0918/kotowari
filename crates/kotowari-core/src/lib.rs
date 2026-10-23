#![deny(clippy::print_stdout, clippy::print_stderr)]
pub mod cargo_mutants;
macro_rules! readonly {
    (copy $($name:ident : $type:ty),* $(,)?) => {
        $(pub fn $name(&self) -> $type { self.$name })*
    };
    (borrow $($name:ident : $type:ty),* $(,)?) => {
        $(pub fn $name(&self) -> &$type { &self.$name })*
    };
}
#[cfg(test)]
extern crate self as kotowari_core;
pub mod config;
mod deferred;
mod deferred_notices;
mod doc_kind;
#[cfg(test)]
#[path = "../tests/document_references.rs"]
mod document_reference_contract_tests;
#[cfg(test)]
#[path = "../tests/test_facts.rs"]
mod fact_contract_tests;
#[cfg(test)]
#[path = "../tests/guides.rs"]
mod guide_contract_tests;
#[cfg(test)]
#[path = "../tests/ir.rs"]
mod ir_contract_tests;
#[cfg(test)]
#[path = "../tests/surface.rs"]
mod surface_contract_tests;
#[cfg(test)]
#[path = "../tests/translations.rs"]
mod translation_contract_tests;
pub use doc_kind::DocKind;
pub mod equivalents;
mod finding;
mod finding_map;
pub use finding::{Finding, FindingKind, count_findings, sort_findings};
mod fingerprint;
pub mod guides;
mod inputs;
pub mod ir;
pub use inputs::{
    CheckInputs, CheckReport, FindingGroup, GroupTally, InputError, Inspection, NativeSourceText,
    NativeSurfaceAnalysis, NativeTestAnalysis, QueryReport, ReadInputs, ReadList, ReadModel,
    RepositoryCheckInputs, RepositoryInspectionPreparation, RepositoryReadInputs,
    RepositoryReadPreparation, SourceText, StatusReport, SurfaceAnalysis, TestAnalysis,
};
pub use ir::{IrOptions, ParsedIrDocument as IrDocument, ParsedItem};
mod list;
pub use list::{ExampleItem, FlagItem, ListItem, RequirementItem, ScenarioItem, TestRef};
mod markdown;
pub mod mutants;
pub mod plan;
mod query;
pub use query::{QueryItem, Reference};
mod record_form;
mod schema;
pub(crate) mod skeleton;
pub mod sources;
mod status;
mod stop;
pub use status::{Documents, Findings, Items, Requirements, Scenarios, Tests};
pub use stop::StopReason;
pub mod surface;
mod terms;
mod test_markers;
mod test_side;
pub mod tests_discovery;
pub mod translations;
pub mod ui_text;

use std::collections::BTreeMap;

/// 検査結果の最上位の構造
#[derive(Clone, serde::Serialize)]
pub(crate) struct CheckResult {
    pub(crate) files: usize,
    pub lines: usize,
    pub findings: Vec<Finding>,
    pub counts: BTreeMap<String, usize>,
    pub tests: BTreeMap<String, TestFileTally>,
    /// REQ-core-206: 読んだ`ガイド`の数と、形の正しい`ガイドの印`の1件の数
    pub guides: guides::GuideTally,
    /// REQ-core-228: "surface.rules" が空の一覧でないときだけ出す
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<surface::Unlisted>,
    /// 追加の指摘の群の数（TBL-core-042）。出し方は呼び出し側が決める
    #[serde(skip)]
    pub(crate) groups: Vec<GroupTally>,
    /// `テスト側の指摘`を見分けるためのファイルの集合（TBL-core-047）。出さない
    #[serde(skip)]
    pub(crate) test_side: test_side::TestSideFiles,
}

/// 読んだテストのファイルの、1つの拡張子の数と問い合わせの有無（TBL-core-021）
#[derive(Debug, Clone, serde::Serialize)]
pub struct TestFileTally {
    /// その拡張子の読んだテストのファイルの数
    pub(crate) files: usize,
    /// その拡張子が問い合わせのある言語か
    pub(crate) query: bool,
}

impl TestFileTally {
    readonly!(copy files: usize, query: bool);
}

/// REQ-core-111: 読むファイルの先頭の UTF-8 BOM (U+FEFF) を読み飛ばす。
/// 読むファイルはどれもここを通す（`read_utf8_file` と `read_source_lines`）。
pub fn strip_bom(text: &str) -> &str {
    text.strip_prefix('\u{FEFF}').unwrap_or(text)
}

/// パスを正規化する純粋な関数。
/// 末尾の "/"、先頭の "./"、途中の "/./" と連続する "/"、"\" を正規化し、"a/.." を畳む。
pub fn normalize_path(path: &str) -> String {
    let s = path.replace('\\', "/");
    let mut parts: Vec<&str> = Vec::new();
    for part in s.split('/') {
        if part == "." || part.is_empty() {
            // 先頭の空（= 先頭の "/"）は保持しない（相対パスの前提）
            // 途中の空（= 連続する "/"）は飛ばす
            // "." は飛ばす
            continue;
        }
        // "a/.." は畳む。畳む相手の無い ".." は残し、基準の外を指すパスのままにする
        // （review7-gaps の A1）
        if part == ".." && parts.last().is_some_and(|last| *last != "..") {
            parts.pop();
            continue;
        }
        parts.push(part);
    }
    if parts.is_empty() {
        // REQ-core-110: "." や "./" は空の置き場になる（呼び出し元が文書名だけの path を作る）
        String::new()
    } else {
        parts.join("/")
    }
}

/// 置き場と文書名を "/" でつなぐ。置き場が空なら文書名だけにする（REQ-core-110）。
pub fn join_display_path(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        format!("{dir}/{name}")
    }
}

/// 二重引用符の外のバッククォートの数が奇数か（REQ-core-116。二重引用符の中は A145 で対象外）。
/// ir モジュールと terms モジュールの両方から使う
pub fn has_odd_backticks_outside_quotes(text: &str) -> bool {
    split_outside_quotes(text)
        .join("")
        .chars()
        .filter(|&c| c == '`')
        .count()
        % 2
        != 0
}

/// 二重引用符の外の部分からバッククォートで囲んだ語を集める（REQ-core-054, REQ-core-064, REQ-core-104）。
/// ir モジュールと terms モジュールの両方から使う
pub fn extract_backtick_contents_outside_quotes(text: &str) -> Vec<&str> {
    // 二重引用符の外のバッククォートを行の左から順に対にし、中身は対の間の元の文字にする。
    // 断片ごとに対にすると、引用符を挟んだ囲みが崩れて後の囲みと組み違える（review5-gaps の A1）
    let base = text.as_ptr() as usize;
    let backticks: Vec<usize> = split_outside_quotes(text)
        .into_iter()
        .flat_map(|part| {
            let offset = part.as_ptr() as usize - base;
            part.match_indices('`').map(move |(i, _)| offset + i)
        })
        .collect();
    backticks
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| &text[pair[0] + 1..pair[1]])
        .collect()
}

/// 二重引用符を行の左から順に対にし、対の中身を集める。対にならない最後の引用符は捨てる（REQ-core-226）
pub fn double_quoted_contents(line: &str) -> Vec<&str> {
    let quotes: Vec<usize> = line.match_indices('"').map(|(i, _)| i).collect();
    quotes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| &line[pair[0] + 1..pair[1]])
        .collect()
}

/// TBL-core-014: 引用符が奇数のときは最後の引用符から行末を引用の中とみなす。
/// ir モジュールと terms モジュールの両方から使う（REQ-core-054, REQ-core-064, REQ-core-104）。
pub fn split_outside_quotes(line: &str) -> Vec<&str> {
    let quote_count = line.chars().filter(|&c| c == '"').count();
    let odd_quotes = quote_count % 2 != 0;

    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_quote = false;
    let last_quote_pos = if odd_quotes {
        line.rfind('"').unwrap_or(0)
    } else {
        0
    };

    for (i, c) in line.char_indices() {
        if c == '"' {
            if odd_quotes && i == last_quote_pos {
                parts.push(&line[start..i]);
                return parts;
            }
            if !in_quote {
                parts.push(&line[start..i]);
                in_quote = true;
            } else {
                in_quote = false;
                start = i + 1;
            }
        }
    }
    parts.push(&line[start..]);
    parts
}

/// 文書の全項目から ID の集合を作る（形に合う ID だけ）
pub fn collect_known_ids(docs: &[ir::IrDocument]) -> std::collections::BTreeSet<String> {
    docs.iter()
        .flat_map(|d| d.items.iter())
        .filter_map(|item| item.id().map(|s| s.to_string()))
        .filter(|id| ir::is_valid_id(id))
        .collect()
}
