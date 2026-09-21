//! "kotowari list" の項目の組み立て（REQ-core-151、REQ-core-153、REQ-core-154、TBL-core-026）

use crate::ir::{IrDocument, Item};
use crate::tests_discovery::{collect_scenarios, TestMarker};
use serde::Serialize;
use std::collections::BTreeMap;

/// "kotowari list" の出力の最上位。"items" だけを持つ（REQ-core-155）
#[derive(Debug, Serialize)]
pub struct ListResult {
    pub items: Vec<ListItem>,
}

/// "tests" の1件（TBL-core-026）
#[derive(Debug, Clone, Serialize)]
pub struct TestRef {
    pub path: String,
    pub line: usize,
    /// `問い合わせの無い言語`では null（REQ-core-081）
    pub name: Option<String>,
}

/// 要求の持つ鍵（TBL-core-026）
#[derive(Debug, Serialize)]
pub struct RequirementItem {
    pub id: String,
    pub kind: &'static str,
    pub name: String,
    pub path: String,
    pub line: usize,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub verification: Option<String>,
    pub definition: Vec<String>,
    pub examples: Vec<String>,
    pub how_to_verify: Option<String>,
    pub sources: Vec<String>,
    pub tests: Vec<TestRef>,
}

/// 決定表と性質の持つ鍵（TBL-core-026。2つは同じ集合で、"kind" の値だけが違う）
#[derive(Debug, Serialize)]
pub struct ExampleItem {
    pub id: String,
    pub kind: &'static str,
    pub name: String,
    pub path: String,
    pub line: usize,
    pub examples: Vec<String>,
    pub sources: Vec<String>,
    pub tests: Vec<TestRef>,
}

/// シナリオの持つ鍵（TBL-core-026）
#[derive(Debug, Serialize)]
pub struct ScenarioItem {
    pub id: String,
    pub kind: &'static str,
    pub name: String,
    pub path: String,
    pub line: usize,
    pub sources: Vec<String>,
    pub tests: Vec<TestRef>,
}

/// 問題の記録の持つ鍵（TBL-core-026）
#[derive(Debug, Serialize)]
pub struct FlagItem {
    pub id: String,
    pub kind: &'static str,
    pub name: String,
    pub path: String,
    pub line: usize,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub relations: Vec<String>,
    pub sources: Vec<String>,
    pub tests: Vec<TestRef>,
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

/// REQ-core-155: 1行に出す値。"検証" は要求以外では "-"
struct TextParts<'a> {
    id: &'a str,
    verification: &'a str,
    name: &'a str,
    path: &'a str,
    line: usize,
    tests: &'a [TestRef],
}

impl ListItem {
    fn text_parts(&self) -> TextParts<'_> {
        match self {
            ListItem::Requirement(i) => TextParts {
                id: &i.id,
                // REQ-core-155: "- 検証:" の行の無い要求も "-"
                verification: i.verification.as_deref().unwrap_or("-"),
                name: &i.name,
                path: &i.path,
                line: i.line,
                tests: &i.tests,
            },
            ListItem::WithExamples(i) => TextParts {
                id: &i.id,
                verification: "-",
                name: &i.name,
                path: &i.path,
                line: i.line,
                tests: &i.tests,
            },
            ListItem::Scenario(i) => TextParts {
                id: &i.id,
                verification: "-",
                name: &i.name,
                path: &i.path,
                line: i.line,
                tests: &i.tests,
            },
            ListItem::Flag(i) => TextParts {
                id: &i.id,
                verification: "-",
                name: &i.name,
                path: &i.path,
                line: i.line,
                tests: &i.tests,
            },
        }
    }

    /// TBL-core-026: 1件の `ID`
    pub fn id(&self) -> &str {
        self.text_parts().id
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
        let parts = self.text_parts();
        (parts.path, parts.line)
    }
}

/// REQ-core-155: 1つの項目を1行で出し、その直後に "tests" の1件ごとの行を字下げして続ける。
/// 名前と検証の値はエスケープせずそのまま出す
pub fn print_text(result: &ListResult) {
    for item in &result.items {
        print_item_text(item);
    }
}

/// REQ-core-155: 1つの項目の1行と、その "tests" の1件ごとの行。
/// query の "text" もこの2種類の行から始まる（REQ-core-161）
pub fn print_item_text(item: &ListItem) {
    let p = item.text_parts();
    println!(
        "{} {} {} {}:{} tests={}",
        p.id,
        p.verification,
        p.name,
        p.path,
        p.line,
        p.tests.len()
    );
    for test in p.tests {
        println!(
            "  {}:{} {}",
            test.path,
            test.line,
            test.name.as_deref().unwrap_or("-")
        );
    }
}

/// 読めた`項目`と`シナリオ`から "items" を組み立てる（REQ-core-151、REQ-core-153、REQ-core-154）
pub fn build(docs: &[IrDocument], ir_path: &str, markers: &[TestMarker]) -> ListResult {
    let tests_by_id = tests_by_id(markers);
    let examples_by_about = examples_by_about(docs, ir_path);
    let mut items: Vec<ListItem> = Vec::new();

    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.relative_path);
        for item in &doc.items {
            // 記録の A6: `ID` の無い項目（"@id" の無いシナリオ、形に合わない見出し、用語）は出さない
            let Some(id) = item.id() else { continue };
            let tests = tests_by_id.get(id).cloned().unwrap_or_default();
            let examples = examples_by_about.get(id).cloned().unwrap_or_default();
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
                }),
                Item::DecisionTable {
                    name, line, sources, ..
                } => ListItem::WithExamples(ExampleItem {
                    id,
                    kind: "table",
                    name: name.clone(),
                    path,
                    line: *line,
                    examples,
                    sources: sources.clone(),
                    tests,
                }),
                Item::Property {
                    name, line, sources, ..
                } => ListItem::WithExamples(ExampleItem {
                    id,
                    kind: "property",
                    name: name.clone(),
                    path,
                    line: *line,
                    examples,
                    sources: sources.clone(),
                    tests,
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
                }),
                Item::GlossaryTerm { .. } | Item::UnknownHeading { .. } => continue,
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
fn examples_by_about(docs: &[IrDocument], ir_path: &str) -> BTreeMap<String, Vec<String>> {
    let mut by_about: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (scenario_id, coverage) in collect_scenarios(docs, ir_path) {
        for about in &coverage.about {
            by_about
                .entry(about.clone())
                .or_default()
                .push(scenario_id.clone());
        }
    }
    by_about
}
