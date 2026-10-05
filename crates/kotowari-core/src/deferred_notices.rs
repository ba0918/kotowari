//! `後回し`との食い違いの`注意`（REQ-core-211、REQ-core-212）。
//! 判定は deferred の、`シナリオ`の "@about" と`印`は tests_discovery のものを使う

use crate::deferred::Requirements;
use crate::ir::{IrDocument, Item, item_references};
use crate::tests_discovery::{TestCoverage, TestMarker, collect_scenarios};
use crate::{Finding, FindingKind};

/// deferred_with_test と depends_on_deferred を出す
pub fn check(
    docs: &[IrDocument],
    ir_path: &str,
    markers: &[TestMarker],
    findings: &mut Vec<Finding>,
) {
    let requirements = Requirements::new(docs);
    let scenarios = collect_scenarios(docs, ir_path);
    let coverage = TestCoverage::new(markers, &scenarios);

    // REQ-core-211: `後回し`の`要求`と`後回しのシナリオ`のうち、その `ID` を含む`印`があるものごとに、
    // 1つ目の`要求`の見出しの行（`シナリオ`はタグの行）に1件
    for (id, doc, line) in requirements.deferred() {
        if coverage.is_marked(id) {
            findings.push(Finding::new(
                FindingKind::DeferredWithTest,
                crate::join_display_path(ir_path, &doc.relative_path),
                Some(line),
                id.to_string(),
            ));
        }
    }
    for (id, scenario) in &scenarios {
        if scenario.deferred && coverage.is_marked(id) {
            findings.push(Finding::new(
                FindingKind::DeferredWithTest,
                scenario.path.clone(),
                Some(scenario.line),
                id.clone(),
            ));
        }
    }

    // REQ-core-212: `後回し`でない`要求`と`性質`と、`後回しのシナリオ`でない`シナリオ`から
    // `後回し`の`要求`への参照1件ごとに、参照の書かれた行に1件。参照は REQ-core-054 が読む場所と
    // 同じで、`問題の記録`の "- related:" は含めない
    for doc in docs {
        let path = crate::join_display_path(ir_path, &doc.relative_path);
        for item in &doc.items {
            let source = match item {
                Item::Requirement { id, .. } if !requirements.is_deferred(id) => id,
                Item::Property { id, .. } => id,
                Item::Scenario { id: Some(id), .. }
                    if !scenarios.get(id).is_some_and(|scenario| scenario.deferred) =>
                {
                    id
                }
                // `ID` の無い`シナリオ`は参照元の `ID` を持たない（query の逆引きにも出ない）
                Item::Requirement { .. }
                | Item::DecisionTable { .. }
                | Item::Scenario { .. }
                | Item::FlagEntry { .. }
                | Item::GlossaryTerm { .. } => continue,
            };
            for reference in item_references(item) {
                if requirements.is_deferred(reference.id) {
                    findings.push(Finding::new(
                        FindingKind::DependsOnDeferred,
                        path.clone(),
                        Some(reference.finding_line),
                        format!("{source} {}", reference.id),
                    ));
                }
            }
        }
    }
}
