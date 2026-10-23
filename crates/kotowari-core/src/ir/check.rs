//! 文書をまたぐ検査と、kotowari の側に残る検査（REQ-core-173）

use super::*;

/// 文書をまたぐ検査と、kotowari の側に残る検査をして指摘を返す（REQ-core-173）。
/// 文書1つで決まる指摘は`parse_findings`から写す。形の指摘が出た文書も、取れた値で
/// 文書をまたぐ検査を受ける（REQ-core-176）
pub fn check_documents(docs: &[IrDocument], config: &Config) -> Vec<Finding> {
    let duplicates = GlossaryDuplicates::new(docs);
    check_documents_with_duplicates(docs, config, &duplicates)
}

pub(crate) fn check_documents_with_duplicates(
    docs: &[IrDocument],
    config: &Config,
    duplicates: &GlossaryDuplicates,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    // 全 ID を収集して重複を検出
    let mut all_ids: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new(); // id -> [(path, line)]

    for (doc, duplicate_rows) in docs.iter().zip(duplicates.rows()) {
        let path = crate::join_display_path(&config.ir, &doc.relative_path);

        // REQ-core-038: 行数の上限
        let limit_lines = config.limits.lines.get() as usize;
        if doc.line_count > limit_lines {
            findings.push(Finding::new(
                FindingKind::TooManyLines,
                path.clone(),
                None,
                doc.line_count.to_string(),
            ));
        }

        // REQ-core-039: 要求の数の上限（用語集と問題の記録を除く）
        if doc.kind == DocKind::Topic {
            let req_count = doc
                .items
                .iter()
                .filter(|item| matches!(item, Item::Requirement { .. }))
                .count();
            let limit_reqs = config.limits.requirements.get() as usize;
            if req_count > limit_reqs {
                findings.push(Finding::new(
                    FindingKind::TooManyRequirements,
                    path.clone(),
                    None,
                    req_count.to_string(),
                ));
            }
        }

        check_empty_deferral(doc.deferred.as_ref(), &path, &mut findings);

        // 項目の検査
        for item in &doc.items {
            if duplicate_rows.contains(&item.item_line()) {
                if let Item::GlossaryTerm { term, line, .. } = item {
                    findings.push(Finding::new(
                        FindingKind::DuplicateTerm,
                        path.clone(),
                        Some(*line),
                        term.clone(),
                    ));
                }
                continue;
            }
            check_item(item, &path, &mut findings);

            // ID を収集（形に合う ID だけ。REQ-core-114）
            if let Some(id) = item.id()
                && is_valid_id(id)
            {
                all_ids
                    .entry(id.to_string())
                    .or_default()
                    .push((path.clone(), item.item_line()));

                // REQ-core-167: ID の名前は文書の置き場の第1階層と一致する。
                // 第1階層を持たない文書（IR の置き場の直下）では指す先が無いので不一致になる
                if let Some(name) = id_name(id) {
                    let domain = doc.directory.split('/').next().unwrap_or("");
                    if name != domain {
                        let line = match item {
                            Item::Scenario { tag_line, line, .. } => tag_line.unwrap_or(*line),
                            Item::Requirement { .. }
                            | Item::DecisionTable { .. }
                            | Item::Property { .. }
                            | Item::FlagEntry { .. }
                            | Item::GlossaryTerm { .. } => item.item_line(),
                        };
                        findings.push(Finding::new(
                            FindingKind::IdDomainMismatch,
                            path.clone(),
                            Some(line),
                            id.to_string(),
                        ));
                    }
                }
            }
        }
    }

    // REQ-core-032: ID の重複
    for (id, locations) in &all_ids {
        // 2つ目以降の場所に指摘
        for (path, line) in locations.iter().skip(1) {
            findings.push(Finding::new(
                FindingKind::DuplicateId,
                path.clone(),
                Some(*line),
                id.clone(),
            ));
        }
    }

    // 参照の解決チェック（REQ-core-054）
    let known_ids = crate::collect_known_ids(docs);
    for doc in docs {
        let path = crate::join_display_path(&config.ir, &doc.relative_path);
        check_references(&doc.items, &known_ids, &path, &mut findings);
    }

    // 文書1つで決まる指摘に文書のパスを入れる
    for doc in docs {
        let doc_path = crate::join_display_path(&config.ir, &doc.relative_path);
        for pf in &doc.parse_findings {
            let mut f = pf.clone();
            if f.path.is_empty() {
                f.path = doc_path.clone();
            }
            findings.push(f);
        }
    }

    findings
}

/// "- source:" の行はあるのに値が1つも無い（REQ-core-059）。行そのものが無いときは
/// スキーマの側が missing_source にしている
pub(super) fn has_empty_source_line(sources: &[String], source_line: &Option<usize>) -> bool {
    source_line.is_some() && sources.is_empty()
}

/// REQ-core-210: 値の空の "- deferred:" の行には、その行に detail が "deferred" の missing_source
pub(super) fn check_empty_deferral(
    deferral: Option<&Deferral>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    if let Some(deferral) = deferral
        && deferral.sources.is_empty()
    {
        findings.push(Finding::new(
            FindingKind::MissingSource,
            path.to_string(),
            Some(deferral.line),
            "deferred".to_string(),
        ));
    }
}

/// 項目の検査のうち、スキーマの側に宣言の無いもの
pub(super) fn check_item(item: &Item, path: &str, findings: &mut Vec<Finding>) {
    match item {
        Item::Requirement {
            id,
            line,
            kind,
            sources,
            source_line,
            definitions,
            definition_line,
            deferred,
            ..
        } => {
            if has_empty_source_line(sources, source_line) {
                findings.push(Finding::new(
                    FindingKind::MissingSource,
                    path.to_string(),
                    Some(*line),
                    id.clone(),
                ));
            }
            check_empty_deferral(deferred.as_ref(), path, findings);

            // REQ-core-051: algorithm に決定表か性質を指す定義がない。
            // "- definition:" の行そのものが無いときはスキーマの側が出している（A71）
            let has_tbl_or_prop_def = definitions
                .iter()
                .any(|d| matches!(id_prefix(d), Some(IdPrefix::Tbl) | Some(IdPrefix::Prop)));
            if kind.as_deref() == Some("algorithm")
                && definition_line.is_some()
                && !has_tbl_or_prop_def
            {
                findings.push(Finding::new(
                    FindingKind::AlgorithmWithoutDefinition,
                    path.to_string(),
                    Some(*line),
                    id.clone(),
                ));
            }
        }

        Item::DecisionTable {
            id,
            line,
            sources,
            source_line,
            ..
        }
        | Item::Property {
            id,
            line,
            sources,
            source_line,
            ..
        }
        | Item::FlagEntry {
            id,
            line,
            sources,
            source_line,
            ..
        } => {
            if has_empty_source_line(sources, source_line) {
                findings.push(Finding::new(
                    FindingKind::MissingSource,
                    path.to_string(),
                    Some(*line),
                    id.clone(),
                ));
            }
        }

        Item::Scenario {
            id,
            line,
            tag_line,
            tags,
            sources,
            scenario_text,
            ..
        } => {
            // TBL-core-019: タグの行（無ければ Scenario: の行）
            let tag_or_scenario_line = tag_line.unwrap_or(*line);

            // REQ-core-052: 知らないタグ（結び付くかを問わない）
            for (tag_name, tag_value) in tags {
                if let Some(detail) = unknown_gherkin_tag(tag_name, tag_value) {
                    findings.push(Finding::new(
                        FindingKind::UnknownTag,
                        path.to_string(),
                        Some(tag_or_scenario_line),
                        detail.to_string(),
                    ));
                }
            }

            // REQ-core-114: @id の値が EX の ID の形でないとき
            let malformed_id = tags
                .iter()
                .find(|(n, v)| n == "@id" && !v.is_empty() && !is_item_id(v, IdPrefix::Ex))
                .map(|(_, v)| v.clone());
            if let Some(malformed_value) = &malformed_id {
                findings.push(Finding::new(
                    FindingKind::InvalidId,
                    path.to_string(),
                    Some(tag_or_scenario_line),
                    malformed_value.clone(),
                ));
                // missing_tag は出さない、missing_source の detail は Scenario: の行の文字
            } else if !tags.iter().any(|(n, v)| n == "@id" && !v.is_empty()) {
                // REQ-core-053: 無いタグ
                findings.push(Finding::new(
                    FindingKind::MissingTag,
                    path.to_string(),
                    Some(tag_or_scenario_line),
                    "@id".to_string(),
                ));
            }
            if !tags.iter().any(|(n, v)| n == "@about" && !v.is_empty()) {
                findings.push(Finding::new(
                    FindingKind::MissingTag,
                    path.to_string(),
                    Some(tag_or_scenario_line),
                    "@about".to_string(),
                ));
            }

            // REQ-core-059: シナリオの出典
            if sources.is_empty() && !tags.iter().any(|(n, v)| n == "@source" && !v.is_empty()) {
                // REQ-core-114: @id が形に合わないときは Scenario: の行の文字を使う
                let detail = if malformed_id.is_some() {
                    scenario_text.clone()
                } else {
                    id.as_deref().unwrap_or(scenario_text).to_string()
                };
                findings.push(Finding::new(
                    FindingKind::MissingSource,
                    path.to_string(),
                    Some(tag_or_scenario_line),
                    detail,
                ));
            }
        }

        Item::GlossaryTerm {
            term,
            sources,
            line,
            ..
        } => {
            // REQ-core-059/REQ-core-060: 用語の出典が空
            if sources.is_empty() {
                findings.push(Finding::new(
                    FindingKind::MissingSource,
                    path.to_string(),
                    Some(*line),
                    term.clone(),
                ));
            }
        }
    }
}

/// 参照の解決チェック（REQ-core-054）。
/// `ID` の形でない "- definition:" の値は`ID`の一覧に入らないので、ここで誤りになる
pub(super) fn check_references(
    items: &[Item],
    known_ids: &BTreeSet<String>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    for item in items {
        for reference in item_references(item) {
            if !known_ids.contains(reference.id) {
                findings.push(Finding::new(
                    FindingKind::UnresolvedReference,
                    path.to_string(),
                    Some(reference.finding_line),
                    reference.id.to_string(),
                ));
            }
        }
    }
}
