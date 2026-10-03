use kotowari_core::{Finding, SourceText, StopReason, config::Config};
use std::path::Path;
pub fn analyze(
    base: &Path,
    cfg: &Config,
) -> Result<
    (
        Option<Vec<kotowari_core::NativeSurfaceAnalysis>>,
        Option<Vec<kotowari_core::NativeSourceText>>,
    ),
    StopReason,
> {
    if cfg.surface.rules.is_empty() {
        return Ok((None, None));
    }
    let analyzer = kotowari_source_analysis::Analyzer::new(
        cfg.clone(),
        vec![],
        crate::test_files::read_rules(base, &cfg.surface.rules, "surface.rules")?,
    )
    .map_err(|error| StopReason::ConfigError(error.detail().into()))?;
    let files = crate::test_files::collect_files(base, &cfg.surface.files)?;
    let mut analysis = vec![];
    for (path, absolute) in files {
        if !analyzer.supports_surfaces(&path) {
            continue;
        }
        let text = crate::acquisition::read_utf8_file(&absolute, &path)?;
        let original = kotowari_core::NativeSourceText::new(absolute, path.clone(), text.clone());
        let source = SourceText::new(path.clone(), text)
            .map_err(|error| StopReason::MappingError(error.to_string()))?;
        let mut file = analyzer
            .surfaces_at(source, &path)
            .map_err(|error| StopReason::ConfigError(error.detail().into()))?;
        for surface in &mut file.surfaces {
            surface.path = path.clone();
        }
        for finding in &mut file.findings {
            *finding = Finding::new(finding.kind(), path.clone(), finding.line(), path.clone());
        }
        analysis.push(kotowari_core::NativeSurfaceAnalysis {
            source: original,
            analysis: file,
        });
    }
    let mut unspecified = vec![];
    if let Some(path) = &cfg.surface.unspecified {
        let text = crate::acquisition::read_utf8_file(&base.join(path), path)?;
        unspecified.push(kotowari_core::NativeSourceText::new(
            base.join(path),
            path.clone(),
            text,
        ));
    }
    Ok((Some(analysis), Some(unspecified)))
}
