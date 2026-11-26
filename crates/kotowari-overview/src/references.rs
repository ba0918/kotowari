//! 参照を解き、参照の表の1件を作る（C-core-ovcmd-13、C-core-ovcmd-14、C-core-ovcmd-29、
//! C-core-ovlang-11）。参照が指すのは決定の見出し（"<文書のパス>#<見出しの行き先の名前>"）か、
//! 確認することの ID だけで、古い形の IR の ID と出典の形は解かない

use kotowari_core::{ParsedDecision, ParsedDecisionDocument, ReadModel};
use kotowari_markdown_view::{Reference, ReferenceState};
use std::collections::BTreeMap;

/// 参照の行き先。`先頭の言語`の`側`の決定の文書の位置
struct Target {
    document: usize,
    decision: usize,
    /// 確認することの ID を指す参照なら、その ID
    row: Option<String>,
}

pub(crate) struct Resolver<'a> {
    /// `先頭の言語`の`側`の決定の文書。参照はこの側の見出しの名前と ID で解く
    first: Vec<ParsedDecisionDocument>,
    /// 本文を取る`側`。`先頭の言語`のページなら None で、first から取る。ほかの言語のページなら、
    /// その言語の`側`の文書を`先頭の言語`の`側`のパスから引く
    side: Option<BTreeMap<String, &'a ParsedDecisionDocument>>,
}

impl<'a> Resolver<'a> {
    pub(crate) fn new(read: &ReadModel) -> Self {
        Self {
            first: read.decision_documents(),
            side: None,
        }
    }

    /// `先頭の言語`でない言語のページの参照。本文はその言語の`側`の同じ決定から取る。`対`の`骨組み`が
    /// 同じなので、同じ決定は文書の中の同じ位置にある
    pub(crate) fn for_language(
        read: &ReadModel,
        language: &str,
        documents: &'a [ParsedDecisionDocument],
    ) -> Self {
        let suffix = format!(".{language}.md");
        let side = documents
            .iter()
            .filter_map(|document| {
                let first = document.path().strip_suffix(&suffix)?;
                Some((format!("{first}.md"), document))
            })
            .collect();
        Self {
            first: read.decision_documents(),
            side: Some(side),
        }
    }

    fn target(&self, key: &str) -> Option<Target> {
        for (document, parsed) in self.first.iter().enumerate() {
            for (decision, found) in parsed.decisions().iter().enumerate() {
                let heading = key
                    .strip_prefix(parsed.path())
                    .and_then(|rest| rest.strip_prefix('#'))
                    .is_some_and(|anchor| anchor == found.anchor());
                let row = found.confirmations().iter().any(|row| row.id() == key)
                    && kotowari_core::is_confirmation_id(key);
                if heading || row {
                    return Some(Target {
                        document,
                        decision,
                        row: row.then(|| key.to_string()),
                    });
                }
            }
        }
        None
    }

    /// 本文を取る決定。ほかの言語の`側`が無いか、その位置に決定が無ければ None
    fn body_decision(&self, target: &Target) -> Option<&ParsedDecision> {
        let first = &self.first[target.document];
        let document = match &self.side {
            None => first,
            Some(side) => side.get(first.path())?,
        };
        document.decisions().get(target.decision)
    }

    /// 参照の表の1件。参照が解けなければ None（C-core-lang-8）
    pub(crate) fn resolve(&self, key: &str) -> Option<Reference> {
        let target = self.target(key)?;
        let decision = &self.first[target.document].decisions()[target.decision];
        // 提案の決定とその行は後回し、実装済みは今のもの。置き換え済みは作らない（C-core-ovcmd-29）
        let state = if decision.state() == Some("proposed") {
            ReferenceState::Deferred
        } else {
            ReferenceState::Current
        };
        let shown = self.body_decision(&target);
        let (label, body) = match &target.row {
            Some(id) => (
                id.clone(),
                shown
                    .and_then(|shown| shown.confirmations().iter().find(|row| row.id() == id))
                    .map(|row| row.lines().join("\n")),
            ),
            None => (
                shown
                    .map_or(decision.heading(), ParsedDecision::heading)
                    .to_string(),
                shown.map(|shown| shown.lines().join("\n")),
            ),
        };
        Some(Reference {
            key: key.to_string(),
            label,
            body,
            state,
        })
    }
}
