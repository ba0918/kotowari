//! `対`の`指摘`（REQ-core-338〜REQ-core-349）

use super::*;

/// `一致の記録`を読んだもの。誤りなら TBL-core-008 の detail
pub(super) fn read_record(
    pair: &Pair,
) -> Result<std::collections::BTreeMap<String, String>, &'static str> {
    let Some(bytes) = &pair.record else {
        return Err("missing");
    };
    let text = std::str::from_utf8(bytes).map_err(|_| "yaml")?;
    let value: serde_json::Value = if text.trim().is_empty() {
        serde_json::Value::Null
    } else {
        serde_saphyr::from_str(crate::strip_bom(text)).map_err(|_| "yaml")?
    };
    let serde_json::Value::Object(map) = value else {
        return Err("keys");
    };
    let mut record = std::collections::BTreeMap::new();
    for (key, value) in map {
        let serde_json::Value::String(value) = value else {
            return Err("keys");
        };
        record.insert(key, value);
    }
    let names: std::collections::BTreeSet<&str> = pair
        .sides
        .iter()
        .map(|side| file_name(&side.path))
        .collect();
    if !record.keys().map(String::as_str).eq(names.iter().copied()) {
        return Err("keys");
    }
    let hex = |value: &String| {
        value.len() == 40
            && value
                .bytes()
                .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
    };
    if !record.values().all(hex) {
        return Err("value");
    }
    Ok(record)
}

/// 欠けた側、`一致の記録`、古い側の`指摘`（REQ-core-338、REQ-core-339、REQ-core-341）
pub(super) fn pair_findings(pair: &Pair, context: &Context, findings: &mut Vec<Finding>) {
    let missing = |path: &str, detail: &str| {
        Finding::new(
            FindingKind::TranslationMissing,
            path.to_string(),
            None,
            detail.to_string(),
        )
    };
    let first = pair.first();
    if first.content.is_none() {
        for side in pair.others().iter().filter(|side| side.content.is_some()) {
            findings.push(missing(&side.path, &first.path));
        }
        return;
    }
    for side in pair.others().iter().filter(|side| side.content.is_none()) {
        findings.push(missing(&first.path, &side.path));
    }
    structure_findings(pair, context, findings);
    match read_record(pair) {
        Err(detail) => findings.push(Finding::new(
            FindingKind::TranslationRecordInvalid,
            record_path(&first.path),
            None,
            detail.to_string(),
        )),
        Ok(record) => {
            for side in &pair.sides {
                let Some(content) = &side.content else {
                    continue;
                };
                let recorded = &record[file_name(&side.path)];
                if *recorded != content.blob {
                    findings.push(Finding::new(
                        FindingKind::TranslationStale,
                        side.path.clone(),
                        None,
                        format!("{recorded} {}", content.blob),
                    ));
                }
            }
        }
    }
}

/// `対`の文書の種類（TBL-core-044）
pub(super) fn skeleton_kind(pair: &Pair) -> crate::skeleton::Kind {
    use crate::skeleton::Kind;
    match pair.place {
        Place::Ir => match crate::DocKind::of(file_name(pair.path())) {
            crate::DocKind::Topic => Kind::Topic,
            crate::DocKind::Glossary => Kind::Glossary,
            crate::DocKind::Flags => Kind::Flags,
        },
        Place::Guide => Kind::Guide,
        Place::OverviewData => Kind::OverviewData,
        Place::Toc => Kind::Toc,
    }
}

/// 検査に要る設定と、すべての`対`の`側`のパスからその言語と`先頭の言語`の`側`のパスへの対応
pub(crate) struct Context {
    pub(super) languages: Vec<String>,
    /// `言語の一覧`の順の、各言語の`UI の文字`の "language_name"
    pub(super) names: Vec<String>,
    pub(super) records: String,
    pub(super) adr: String,
    pub(super) sides: std::collections::BTreeMap<String, (String, String)>,
}

impl Context {
    pub(crate) fn new(pairs: &Pairs, config: &crate::config::Config) -> Self {
        let languages = config.languages();
        let names = languages
            .iter()
            .map(|language| {
                config
                    .ui_text(language)
                    .get("language_name")
                    .unwrap_or("")
                    .to_string()
            })
            .collect();
        let sides = pairs
            .iter()
            .flat_map(|pair| {
                pair.sides.iter().map(|side| {
                    (
                        side.path.clone(),
                        (side.language.clone(), pair.path().to_string()),
                    )
                })
            })
            .collect();
        Self {
            languages,
            names,
            records: config.decisions.records.clone(),
            adr: config.decisions.adr.clone(),
            sides,
        }
    }

    /// その`側`のあるべき`切り替えの行`（REQ-core-346）
    pub(crate) fn switcher(&self, first: &str, language: &str) -> String {
        self.languages
            .iter()
            .zip(&self.names)
            .map(|(other, name)| {
                if other == language {
                    name.clone()
                } else {
                    let target = side_path(first, other, &self.languages);
                    format!("[{name}]({})", file_name(&target))
                }
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }
}

/// `切り替えの行`を持つ`対`か（REQ-core-346、`全体像の元データ`と`目次`は持たない）
pub(super) fn has_switcher(place: Place) -> bool {
    matches!(place, Place::Ir | Place::Guide)
}

/// リンクを検査する`対`か（REQ-core-347）
pub(super) fn has_links(place: Place) -> bool {
    !matches!(place, Place::Toc)
}

/// 1つの`側`を検査し、`切り替えの行`を空にした文と、リンクを検査する`対`ならその文を
/// CommonMark と GFM として読んだ木を返す（REQ-core-346〜REQ-core-349）
pub(super) fn side_text(
    pair: &Pair,
    side: &Side,
    context: &Context,
    findings: &mut Vec<Finding>,
) -> (String, Option<markdown::mdast::Node>) {
    let text = side.content.as_ref().map_or("", |content| content.text());
    let mut checked = text.to_string();
    if has_switcher(pair.place) {
        let expected = context.switcher(pair.path(), &side.language);
        match switcher_line(text, &expected) {
            Some(line) => checked = blank_line(text, line),
            None => {
                let (title, candidate) = switcher_candidate(text);
                findings.push(Finding::new(
                    FindingKind::TranslationSwitcherInvalid,
                    side.path.clone(),
                    candidate.map(|(line, _)| line).or(title),
                    expected,
                ));
            }
        }
    }
    let root = has_links(pair.place)
        .then(|| crate::markdown::parse(&checked).ok())
        .flatten();
    if let Some(root) = &root {
        for link in links(root) {
            let Some(target) = target(&side.path, &link.destination) else {
                continue;
            };
            if let Some((language, _)) = context.sides.get(&target)
                && *language != side.language
            {
                findings.push(Finding::new(
                    FindingKind::LinkLanguageMismatch,
                    side.path.clone(),
                    Some(link.line),
                    link.destination.clone(),
                ));
            }
            let record = |place: &str| crate::sources::is_under_place(&target, place);
            if record(&context.records) || record(&context.adr) {
                findings.push(Finding::new(
                    FindingKind::LinkToRecord,
                    side.path.clone(),
                    Some(link.line),
                    link.destination,
                ));
            }
        }
    }
    (checked, root)
}

/// REQ-core-345: ほかの言語の`側`の`骨組み`が`先頭の言語`の`側`と食い違えば、`側`ごとに1件
pub(super) fn structure_findings(pair: &Pair, context: &Context, findings: &mut Vec<Finding>) {
    let kind = skeleton_kind(pair);
    let skeleton = |side: &Side, findings: &mut Vec<Finding>| {
        let (text, root) = side_text(pair, side, context, findings);
        let mut skeleton = crate::skeleton::skeleton(kind, &text, root.as_ref());
        if matches!(pair.place, Place::Guide | Place::OverviewData) {
            let links = root
                .as_ref()
                .map_or_else(Vec::new, |root| link_part(&side.path, root, context));
            skeleton.push(("link", links));
        }
        skeleton
    };
    let expected = skeleton(pair.first(), findings);
    for side in pair.others() {
        if side.content.is_none() {
            continue;
        }
        let actual = skeleton(side, findings);
        if let Some((part, line)) = crate::skeleton::compare(&expected, &actual) {
            findings.push(Finding::new(
                FindingKind::TranslationStructureMismatch,
                side.path.clone(),
                line,
                part.to_string(),
            ));
        }
    }
}
