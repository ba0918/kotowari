//! "kotowari list" の項目の組み立て（REQ-core-151、REQ-core-153、REQ-core-154、TBL-core-026）

use crate::deferred;
use crate::fingerprint::fingerprint_of;
use crate::ir::{self, IrDocument, Item};
use crate::tests_discovery::{ScenarioCoverage, TestMarker, collect_scenarios};
use serde::Serialize;
use std::collections::BTreeMap;

/// "kotowari list" の出力の最上位。"items" だけを持つ（REQ-core-155）
#[derive(Debug, Serialize)]
pub struct ListResult {
    pub(crate) items: Vec<ListItem>,
}

/// "tests" の1件（TBL-core-026）
#[derive(Debug, Clone, Serialize)]
pub struct TestRef {
    pub(crate) path: String,
    pub(crate) line: usize,
    /// `問い合わせの無い言語`では null（REQ-core-081）
    pub(crate) name: Option<String>,
}
impl TestRef {
    readonly!(copy line: usize);
    readonly!(borrow path: String, name: Option<String>);
}

/// 要求の持つ鍵（TBL-core-026）
#[derive(Debug, Serialize)]
pub struct RequirementItem {
    pub(crate) id: String,
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) line: usize,
    #[serde(rename = "type")]
    pub(crate) type_: Option<String>,
    pub(crate) verification: Option<String>,
    pub(crate) definition: Vec<String>,
    pub(crate) examples: Vec<String>,
    pub(crate) how_to_verify: Option<String>,
    pub(crate) sources: Vec<String>,
    pub(crate) tests: Vec<TestRef>,
    /// その`項目`か`シナリオ`の`指紋`（REQ-core-203）
    pub(crate) fingerprint: String,
    /// `後回し`の`要求`と`後回しのシナリオ`は true
    pub(crate) deferred: bool,
}
impl RequirementItem {
    readonly!(copy kind: &'static str, line: usize, deferred: bool);
    readonly!(borrow id: String, name: String, path: String, type_: Option<String>, verification: Option<String>, definition: Vec<String>, examples: Vec<String>, how_to_verify: Option<String>, sources: Vec<String>, tests: Vec<TestRef>, fingerprint: String);
}

/// 決定表と性質の持つ鍵（TBL-core-026。2つは同じ集合で、"kind" の値だけが違う）
#[derive(Debug, Serialize)]
pub struct ExampleItem {
    pub(crate) id: String,
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) examples: Vec<String>,
    pub(crate) sources: Vec<String>,
    pub(crate) tests: Vec<TestRef>,
    /// その`項目`か`シナリオ`の`指紋`（REQ-core-203）
    pub(crate) fingerprint: String,
    /// `後回し`の`要求`と`後回しのシナリオ`は true
    pub(crate) deferred: bool,
}
impl ExampleItem {
    readonly!(copy kind: &'static str, line: usize, deferred: bool);
    readonly!(borrow id: String, name: String, path: String, examples: Vec<String>, sources: Vec<String>, tests: Vec<TestRef>, fingerprint: String);
}

/// シナリオの持つ鍵（TBL-core-026）
#[derive(Debug, Serialize)]
pub struct ScenarioItem {
    pub(crate) id: String,
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) line: usize,
    pub(crate) sources: Vec<String>,
    pub(crate) tests: Vec<TestRef>,
    /// その`項目`か`シナリオ`の`指紋`（REQ-core-203）
    pub(crate) fingerprint: String,
    /// `後回し`の`要求`と`後回しのシナリオ`は true
    pub(crate) deferred: bool,
}
impl ScenarioItem {
    readonly!(copy kind: &'static str, line: usize, deferred: bool);
    readonly!(borrow id: String, name: String, path: String, sources: Vec<String>, tests: Vec<TestRef>, fingerprint: String);
}

/// 問題の記録の持つ鍵（TBL-core-026）
#[derive(Debug, Serialize)]
pub struct FlagItem {
    pub(crate) id: String,
    pub(crate) kind: &'static str,
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) line: usize,
    #[serde(rename = "type")]
    pub(crate) type_: Option<String>,
    pub(crate) relations: Vec<String>,
    pub(crate) sources: Vec<String>,
    pub(crate) tests: Vec<TestRef>,
    /// その`項目`か`シナリオ`の`指紋`（REQ-core-203）
    pub(crate) fingerprint: String,
    /// `後回し`の`要求`と`後回しのシナリオ`は true
    pub(crate) deferred: bool,
}
impl FlagItem {
    readonly!(copy kind: &'static str, line: usize, deferred: bool);
    readonly!(borrow id: String, name: String, path: String, type_: Option<String>, relations: Vec<String>, sources: Vec<String>, tests: Vec<TestRef>, fingerprint: String);
}

/// 一覧の1件。鍵の集合は種類で決まるので、種類ごとの構造をそのまま出す（TBL-core-026）
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ListItem {
    Requirement(RequirementItem),
    /// 決定表と性質
    WithExamples(ExampleItem),
    Scenario(ScenarioItem),
    Flag(FlagItem),
}

impl ListItem {
    /// TBL-core-026: 1件の `ID`
    pub fn id(&self) -> &str {
        match self {
            Self::Requirement(item) => &item.id,
            Self::WithExamples(item) => &item.id,
            Self::Scenario(item) => &item.id,
            Self::Flag(item) => &item.id,
        }
    }

    /// TBL-core-026: 1件の "kind"
    pub fn kind(&self) -> &'static str {
        match self {
            ListItem::Requirement(i) => i.kind,
            ListItem::WithExamples(i) => i.kind,
            ListItem::Scenario(i) => i.kind,
            ListItem::Flag(i) => i.kind,
        }
    }

    /// TBL-core-026: 1件の "path" と "line"（REQ-core-154 の並べ替えの鍵）
    pub fn location(&self) -> (&str, usize) {
        match self {
            Self::Requirement(item) => (&item.path, item.line),
            Self::WithExamples(item) => (&item.path, item.line),
            Self::Scenario(item) => (&item.path, item.line),
            Self::Flag(item) => (&item.path, item.line),
        }
    }
}

/// 読めた`項目`と`シナリオ`から "items" を組み立てる（REQ-core-151、REQ-core-153、REQ-core-154）
pub fn build(docs: &[IrDocument], ir_path: &str, markers: &[TestMarker]) -> ListResult {
    let tests_by_id = tests_by_id(markers);
    let scenarios = collect_scenarios(docs, ir_path);
    let examples_by_about = examples_by_about(&scenarios);
    let requirements = deferred::Requirements::new(docs);
    let mut items: Vec<ListItem> = Vec::new();

    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.relative_path);
        let lines = ir::split_lines(&doc.raw_content);
        for item in &doc.items {
            // 記録の A6: `ID` の無い項目（"@id" の無いシナリオ、形に合わない見出し、用語）は出さない
            let Some(id) = item.id() else { continue };
            let tests = tests_by_id.get(id).cloned().unwrap_or_default();
            let examples = examples_by_about.get(id).cloned().unwrap_or_default();
            let fingerprint = fingerprint_of(doc, item, &lines);
            let deferred = match item {
                Item::Requirement { .. } => requirements.is_deferred(id),
                Item::Scenario { .. } => scenarios.get(id).is_some_and(|s| s.deferred),
                Item::DecisionTable { .. }
                | Item::Property { .. }
                | Item::FlagEntry { .. }
                | Item::GlossaryTerm { .. } => false,
            };
            let id = id.to_string();
            let path = path.clone();
            items.push(match item {
                Item::Requirement {
                    name,
                    line,
                    kind,
                    sources,
                    verification,
                    definitions,
                    how_to_verify,
                    ..
                } => ListItem::Requirement(RequirementItem {
                    id,
                    kind: "requirement",
                    name: name.clone(),
                    path,
                    line: *line,
                    type_: kind.clone(),
                    verification: verification.clone(),
                    definition: definitions.clone(),
                    examples,
                    how_to_verify: how_to_verify.clone(),
                    sources: sources.clone(),
                    tests,
                    fingerprint,
                    deferred,
                }),
                Item::DecisionTable {
                    name,
                    line,
                    sources,
                    ..
                } => ListItem::WithExamples(ExampleItem {
                    id,
                    kind: "table",
                    name: name.clone(),
                    path,
                    line: *line,
                    examples,
                    sources: sources.clone(),
                    tests,
                    fingerprint,
                    deferred,
                }),
                Item::Property {
                    name,
                    line,
                    sources,
                    ..
                } => ListItem::WithExamples(ExampleItem {
                    id,
                    kind: "property",
                    name: name.clone(),
                    path,
                    line: *line,
                    examples,
                    sources: sources.clone(),
                    tests,
                    fingerprint,
                    deferred,
                }),
                Item::Scenario {
                    line,
                    sources,
                    scenario_text,
                    ..
                } => ListItem::Scenario(ScenarioItem {
                    id,
                    kind: "scenario",
                    name: scenario_name(scenario_text),
                    path,
                    line: *line,
                    sources: sources.clone(),
                    tests,
                    fingerprint,
                    deferred,
                }),
                Item::FlagEntry {
                    name,
                    line,
                    kind,
                    relations,
                    sources,
                    ..
                } => ListItem::Flag(FlagItem {
                    id,
                    kind: "flag",
                    name: name.clone(),
                    path,
                    line: *line,
                    type_: kind.clone(),
                    relations: relations.clone(),
                    sources: sources.clone(),
                    tests,
                    fingerprint,
                    deferred,
                }),
                Item::GlossaryTerm { .. } => continue,
            });
        }
    }

    items.sort_by(|a, b| a.location().cmp(&b.location()));
    ListResult { items }
}

/// TBL-core-026: `シナリオ`の名前は "Scenario:" の後の文字から前後の半角空白とタブを除いたもの
fn scenario_name(scenario_text: &str) -> String {
    let after = scenario_text.trim_start();
    let after = after.strip_prefix("Scenario:").unwrap_or(after);
    after.trim_matches(|c| c == ' ' || c == '\t').to_string()
}

/// `ID` から、その `ID` を`印`に含む`テスト`の並びを引く（TBL-core-026 の "tests"、REQ-core-154）
fn tests_by_id(markers: &[TestMarker]) -> BTreeMap<&str, Vec<TestRef>> {
    let mut by_id: BTreeMap<&str, Vec<TestRef>> = BTreeMap::new();
    for marker in markers {
        by_id.entry(&marker.id).or_default().push(TestRef {
            path: marker.path.clone(),
            line: marker.line,
            name: marker.name.clone(),
        });
    }
    for refs in by_id.values_mut() {
        refs.sort_by(|a, b| (&a.path, a.line).cmp(&(&b.path, b.line)));
    }
    by_id
}

/// `ID` から、その `ID` を "@about" に持つ`シナリオ`の `ID` の並びを引く（TBL-core-026 の "examples"）。
/// 同じ `ID` の`シナリオ`が2か所以上にあるときは REQ-core-032 の1つ目だけを数える
fn examples_by_about(
    scenarios: &BTreeMap<String, ScenarioCoverage>,
) -> BTreeMap<String, Vec<String>> {
    let mut by_about: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (scenario_id, coverage) in scenarios {
        for about in &coverage.about {
            by_about
                .entry(about.clone())
                .or_default()
                .push(scenario_id.clone());
        }
    }
    by_about
}
