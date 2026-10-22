//! `IR`のほかの言語の`側`の文書（REQ-core-342、REQ-core-354）

use super::*;

/// `基準のディレクトリ`からの相対パスを、`IR`の置き場からの相対パスにする
pub(super) fn ir_relative(place: &str, path: &str) -> String {
    crate::sources::relative_to_place(path, place)
        .unwrap_or(path)
        .to_string()
}

/// `先頭の言語`でない言語ごとの、`先頭の言語`の`側`のある`IR`の`対`のその言語の`側`の文書。
/// 文書の種類は`先頭の言語`の`側`の名前で決まり（REQ-core-033）、文書はパスのバイト順に並ぶ
pub(crate) fn ir_sides(
    pairs: &Pairs,
    config: &crate::config::Config,
) -> Result<std::collections::BTreeMap<String, Vec<crate::ir::IrDocument>>, crate::StopReason> {
    let place = config.ir.as_str();
    let mut sides: std::collections::BTreeMap<String, Vec<crate::ir::IrDocument>> =
        std::collections::BTreeMap::new();
    for pair in pairs.iter().filter(|pair| pair.place == Place::Ir) {
        if pair.first().content.is_none() {
            continue;
        }
        let kind = crate::DocKind::of(file_name(pair.path()));
        for side in pair.others() {
            let Some(content) = &side.content else {
                continue;
            };
            let path = ir_relative(place, &side.path);
            let text = without_switcher(config, pair.path(), &side.language, content.text());
            let text = text.as_deref().unwrap_or(content.text());
            let document = crate::ir::parse_side(&path, kind, text)?;
            sides
                .entry(side.language.clone())
                .or_default()
                .push(document);
        }
    }
    for documents in sides.values_mut() {
        documents.sort_by(|left, right| {
            left.relative_path
                .as_bytes()
                .cmp(right.relative_path.as_bytes())
        });
    }
    Ok(sides)
}

/// REQ-core-342: ほかの言語の`側`に行う検査。`用語`（その言語の`用語集`の`連鎖`から引く）、`曖昧語`、
/// `文書名の参照`、閉じないバッククォート、`用語集`の形
pub(crate) fn side_findings(
    sides: &[crate::ir::IrDocument],
    firsts: &[crate::ir::IrDocument],
    config: &crate::config::Config,
    findings: &mut Vec<Finding>,
) {
    let duplicates = crate::ir::GlossaryDuplicates::new(sides);
    let known_ids = crate::collect_known_ids(firsts);
    crate::terms::check_terms_and_vague_words_with_duplicates(
        sides,
        &known_ids,
        &config.vague_words,
        &config.ir,
        &duplicates,
        findings,
    );
    let paths = firsts
        .iter()
        .map(|document| document.relative_path.clone())
        .collect();
    crate::terms::check_document_references(sides, &config.ir, &paths, findings);
    for (document, rows) in sides.iter().zip(duplicates.rows()) {
        if document.kind != crate::DocKind::Glossary {
            continue;
        }
        let path = crate::join_display_path(&config.ir, &document.relative_path);
        for finding in &document.parse_findings {
            if matches!(
                finding.kind,
                FindingKind::GlossaryInvalid
                    | FindingKind::InvalidGlossaryRow
                    | FindingKind::GlossaryTitleInvalid
                    | FindingKind::DuplicateTerm
            ) {
                let mut finding = finding.clone();
                if finding.path.is_empty() {
                    finding.path = path.clone();
                }
                findings.push(finding);
            }
        }
        for item in &document.items {
            if let crate::ir::Item::GlossaryTerm { term, line, .. } = item
                && rows.contains(line)
            {
                findings.push(Finding::new(
                    FindingKind::DuplicateTerm,
                    path.clone(),
                    Some(*line),
                    term.clone(),
                ));
            }
        }
    }
}

/// その言語の`IR`の`側`の文書。`先頭の言語`の`側`のある`対`の、ファイルのある`側`だけ。パスのバイト順
pub fn side_documents(
    pairs: &Pairs,
    config: &crate::config::Config,
    language: &str,
) -> Result<Vec<crate::ir::IrDocument>, crate::StopReason> {
    Ok(ir_sides(pairs, config)?
        .remove(language)
        .unwrap_or_default())
}
