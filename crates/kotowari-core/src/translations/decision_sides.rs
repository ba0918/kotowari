//! 決定の文書の、ほかの言語の`側`

use super::*;

/// その言語の、決定の文書の`側`を読んだもの（C-core-ovlang-11）。`先頭の言語`の`側`のある`対`の、
/// ファイルのある`側`だけで、用語集は入れない。パスのバイト順。path はその`側`のパス
pub fn decision_side_documents(
    pairs: &Pairs,
    config: &crate::config::Config,
    language: &str,
) -> Result<Vec<crate::ParsedDecisionDocument>, crate::StopReason> {
    let mut documents = Vec::new();
    for pair in pairs.iter().filter(|pair| pair.place == Place::Decision) {
        let glossary = crate::decisions::is_glossary(pair.path(), &config.decisions.documents);
        if pair.first().content.is_none() || glossary {
            continue;
        }
        for side in pair
            .others()
            .iter()
            .filter(|side| side.language == language)
        {
            let Some(content) = &side.content else {
                continue;
            };
            let text = without_switcher(config, pair.path(), &side.language, content.text());
            let source = crate::SourceText::new(
                side.path.as_str(),
                text.as_deref().unwrap_or(content.text()),
            )
            .map_err(|error| crate::StopReason::MappingError(error.to_string()))?;
            documents.push(
                crate::parse_decision_document(&source)
                    .map_err(|error| crate::StopReason::MappingError(error.to_string()))?,
            );
        }
    }
    documents.sort_by(|left, right| left.path().as_bytes().cmp(right.path().as_bytes()));
    Ok(documents)
}
