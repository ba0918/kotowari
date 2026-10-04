//! 参照を解き、参照の表の1件を作る（REQ-core-285、REQ-core-291、TBL-core-039）

use kotowari_core::ir::Item;
use kotowari_core::sources::SourceTarget;
use kotowari_core::{ListItem, ReadModel};
use kotowari_markdown_view::{Reference, ReferenceState};
use std::collections::BTreeMap;

/// `ID` から、REQ-core-032 の1つ目の`項目`か`シナリオ`と、`後回し`かどうか
pub(crate) struct Resolver<'a> {
    read: &'a ReadModel,
    items: BTreeMap<&'a str, &'a Item>,
    deferred: BTreeMap<String, bool>,
}

impl<'a> Resolver<'a> {
    pub(crate) fn new(read: &'a ReadModel) -> Self {
        let mut items = BTreeMap::new();
        // 文書はパスのバイト順、文書の中の項目は行の順に並んでいる
        for document in read.documents() {
            for item in document.items() {
                if let Some(id) = item.id()
                    && !matches!(item, Item::GlossaryTerm { .. })
                {
                    items.entry(id).or_insert(item);
                }
            }
        }
        let mut deferred = BTreeMap::new();
        for item in read.list().items() {
            let flag = match item {
                ListItem::Requirement(item) => item.deferred(),
                ListItem::Scenario(item) => item.deferred(),
                ListItem::WithExamples(item) => item.deferred(),
                ListItem::Flag(item) => item.deferred(),
            };
            deferred.entry(item.id().to_string()).or_insert(flag);
        }
        Self {
            read,
            items,
            deferred,
        }
    }

    /// 参照の表の1件。参照が解けなければ None（REQ-core-285）
    pub(crate) fn resolve(&self, key: &str) -> Option<Reference> {
        if kotowari_core::ir::is_valid_id(key) {
            return self.item(key);
        }
        if key.contains('#') {
            return self
                .read
                .source_target(key)
                .map(|target| source(key, target));
        }
        None
    }

    fn item(&self, id: &str) -> Option<Reference> {
        let item = self.items.get(id)?;
        let deferred = self.deferred.get(id).copied().unwrap_or(false);
        let lines = |lines: &[(usize, String)]| {
            lines
                .iter()
                .map(|(_, line)| line.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        };
        let (body, may_defer) = match item {
            Item::Requirement {
                kind,
                name,
                definitions,
                statements,
                ..
            } => {
                if kind.as_deref() == Some("algorithm") {
                    (format!("{name}\n{}", definitions.join(", ")), true)
                } else {
                    (lines(statements), true)
                }
            }
            Item::DecisionTable { name, .. } => (name.clone(), false),
            Item::Property { statements, .. } => (lines(statements), false),
            Item::Scenario {
                scenario_text,
                steps,
                ..
            } => {
                let mut body = vec![scenario_text.as_str()];
                body.extend(steps.iter().map(|(_, step)| step.as_str()));
                (body.join("\n"), true)
            }
            Item::FlagEntry { body, .. } => (lines(body), false),
            Item::GlossaryTerm { .. } => return None,
        };
        Some(Reference {
            key: id.to_string(),
            label: id.to_string(),
            body,
            state: if may_defer && deferred {
                ReferenceState::Deferred
            } else {
                ReferenceState::Current
            },
        })
    }
}

/// パスの最後の成分から ".md" を除いたもの
fn stem(path: &str) -> &str {
    let name = path.rsplit('/').next().unwrap_or(path);
    name.strip_suffix(".md").unwrap_or(name)
}

/// 先頭の "YYYY-MM-DD-" を除く
fn without_date(name: &str) -> &str {
    let bytes = name.as_bytes();
    let is_date = bytes.len() > 11
        && bytes[..10]
            .iter()
            .enumerate()
            .all(|(index, byte)| match index {
                4 | 7 => *byte == b'-',
                _ => byte.is_ascii_digit(),
            })
        && bytes[10] == b'-';
    if is_date { &name[11..] } else { name }
}

fn source(key: &str, target: SourceTarget) -> Reference {
    match target {
        SourceTarget::Decision {
            path,
            number,
            text,
            superseded,
        } => Reference {
            key: key.to_string(),
            label: format!("{} {number}", without_date(stem(&path))),
            body: text,
            state: if superseded {
                ReferenceState::Superseded
            } else {
                ReferenceState::Current
            },
        },
        SourceTarget::Heading {
            path,
            heading,
            lines,
        } => Reference {
            key: key.to_string(),
            label: format!("{} {heading}", stem(&path)),
            body: lines.join("\n"),
            state: ReferenceState::Current,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @kotowari[TBL-core-039]
    #[test]
    fn decision_labels_drop_the_leading_date_and_the_extension() {
        assert_eq!(
            without_date(stem("docs/r/2026-10-02-whole-picture.md")),
            "whole-picture"
        );
        assert_eq!(without_date(stem("docs/r/records.md")), "records");
        assert_eq!(without_date(stem("2026-10-02.md")), "2026-10-02");
        assert_eq!(without_date(stem("2026-1x-02-a.md")), "2026-1x-02-a");
    }
}
