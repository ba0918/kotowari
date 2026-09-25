//! `後回し`の`要求`と`後回しのシナリオ`の判定（REQ-core-208、用語「後回しのシナリオ」）。
//! check のテストの無さの検査、2つの`注意`、status、list はどれもこの判定を使う

use crate::ir::{IrDocument, Item, VERIFICATION_VALUES, item_references};
use crate::tests_discovery::{ScenarioCoverage, TestCoverage, collect_scenarios};
use crate::{Finding, FindingKind};
use std::collections::BTreeMap;

/// `要求`の `ID` ごとの、判定に要る値
#[derive(Debug, Clone, Copy)]
struct RequirementFacts<'a> {
    /// "- verification:" の値。行が無ければ None
    verification: Option<&'a str>,
    /// 見出しの下か文書単位の "- deferred:" の行がある
    deferred: bool,
    /// その`要求`の文書
    doc: &'a IrDocument,
    /// その`要求`の見出しの行
    line: usize,
}

/// `要求`の `ID` から、その判定に使う1つ目の`要求`を引く表。同じ `ID` の`要求`が2か所以上に
/// あるときは REQ-core-032 の1つ目（文書はパスのバイト順、同じ文書では行の小さい方）を使う
pub struct Requirements<'a>(BTreeMap<&'a str, RequirementFacts<'a>>);

impl<'a> Requirements<'a> {
    /// docs も items も REQ-core-032 の順に並んでいる
    pub fn new(docs: &'a [IrDocument]) -> Self {
        let mut facts = BTreeMap::new();
        for doc in docs {
            for item in &doc.items {
                if let Item::Requirement {
                    id,
                    line,
                    verification,
                    deferred,
                    ..
                } = item
                {
                    facts.entry(id.as_str()).or_insert(RequirementFacts {
                        verification: verification.as_deref(),
                        deferred: deferred.is_some() || doc.deferred.is_some(),
                        doc,
                        line: *line,
                    });
                }
            }
        }
        Requirements(facts)
    }

    /// REQ-core-208: その `ID` の`要求`が`後回し`か。`要求`でない `ID` は`後回し`でない
    pub fn is_deferred(&self, id: &str) -> bool {
        self.0.get(id).is_some_and(|facts| facts.deferred)
    }

    /// 用語「後回しのシナリオ」: "@about" の`要求`のうち検証の値が4つのどれかであるものに
    /// `後回し`が1つ以上あり、それらがすべて`後回し`か検証が review
    pub fn is_deferred_scenario(&self, about: &[String]) -> bool {
        let mut any_deferred = false;
        for (verification, facts) in self.qualifying(about) {
            if facts.deferred {
                any_deferred = true;
            } else if verification != "review" {
                return false;
            }
        }
        any_deferred
    }

    /// REQ-core-137: "@about" に、検証の値が review 以外で`後回し`でない`要求`がある
    pub fn needs_test(&self, about: &[String]) -> bool {
        self.qualifying(about)
            .any(|(verification, facts)| verification != "review" && !facts.deferred)
    }

    /// "@about" の `ID` のうち、`要求`として解決でき、検証の値が4つのどれかであるものの値
    fn qualifying<'s>(
        &'s self,
        about: &'s [String],
    ) -> impl Iterator<Item = (&'a str, RequirementFacts<'a>)> + 's {
        about.iter().filter_map(|id| {
            let facts = self.0.get(id.as_str())?;
            let verification = facts.verification?;
            VERIFICATION_VALUES
                .contains(&verification)
                .then_some((verification, *facts))
        })
    }
}

/// REQ-core-211: `後回し`の`要求`と`後回しのシナリオ`のうち、その `ID` を含む`印`があるものごとに、
/// 1つ目の`要求`の見出しの行（`シナリオ`はタグの行）に deferred_with_test を1件出す
pub(crate) fn check_marked(
    requirements: &Requirements<'_>,
    scenarios: &BTreeMap<String, ScenarioCoverage>,
    coverage: &TestCoverage,
    ir_path: &str,
    findings: &mut Vec<Finding>,
) {
    for (id, facts) in &requirements.0 {
        if facts.deferred && coverage.is_marked(id) {
            findings.push(Finding::new(
                FindingKind::DeferredWithTest,
                crate::join_display_path(ir_path, &facts.doc.relative_path),
                Some(facts.line),
                id.to_string(),
            ));
        }
    }
    for (id, scenario) in scenarios {
        if scenario.deferred && coverage.is_marked(id) {
            findings.push(Finding::new(
                FindingKind::DeferredWithTest,
                scenario.path.clone(),
                Some(scenario.line),
                id.clone(),
            ));
        }
    }
}

/// REQ-core-212: `後回し`でない`要求`と`性質`と、`後回しのシナリオ`でない`シナリオ`から
/// `後回し`の`要求`への参照1件ごとに、参照の書かれた行に depends_on_deferred を出す。
/// 参照は REQ-core-054 が読む場所と同じで、`問題の記録`の "- related:" は含めない
pub fn check_dependencies(docs: &[IrDocument], ir_path: &str, findings: &mut Vec<Finding>) {
    let requirements = Requirements::new(docs);
    let scenarios = collect_scenarios(docs, ir_path);
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
                _ => continue,
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
