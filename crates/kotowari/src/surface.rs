use kotowari_core::{
    Finding, SourceText, StopReason, config::Config, ir::IrDocument, surface::SurfaceTally,
};
use std::path::Path;
pub fn check(
    base: &Path,
    cfg: &Config,
    docs: &[IrDocument],
    findings: &mut Vec<Finding>,
) -> Result<Option<SurfaceTally>, StopReason> {
    if cfg.surface.rules.is_empty() {
        return Ok(None);
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
        let text = crate::acquisition::read_utf8_file(Path::new(&absolute), &path)?;
        let source = SourceText::new(path.clone(), text)
            .map_err(|error| StopReason::MappingError(error.to_string()))?;
        let mut file = analyzer
            .surfaces_at(source, &path)
            .map_err(|error| StopReason::ConfigError(error.detail().into()))?;
        for surface in &mut file.surfaces {
            surface.path = path.clone();
        }
        for finding in &mut file.findings {
            finding.path = path.clone();
            finding.detail = path.clone();
        }
        analysis.push(file);
    }
    let mut unspecified = vec![];
    if let Some(path) = &cfg.surface.unspecified {
        let text = crate::acquisition::read_utf8_file(&base.join(path), path)?;
        unspecified.push(
            SourceText::new(path.clone(), text)
                .map_err(|error| StopReason::MappingError(error.to_string()))?,
        );
    }
    Ok(Some(kotowari_core::surface::check_analysis(
        &analysis,
        docs,
        &unspecified,
        findings,
    )?))
}
