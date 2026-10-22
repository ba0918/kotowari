//! 部分の解析の公開の入口（REQ-core-314）

use super::*;

#[derive(Debug, Clone)]
pub struct IrOptions {
    pub ir: String,
}

impl Default for IrOptions {
    fn default() -> Self {
        Self {
            ir: "docs/ir".into(),
        }
    }
}

#[derive(Debug)]
pub struct ParsedIrDocument {
    path: String,
    items: Vec<ParsedItem>,
    findings: Vec<Finding>,
}

#[derive(Debug)]
pub struct ParsedItem(Item);

impl ParsedItem {
    pub fn id(&self) -> Option<&str> {
        self.0.id().filter(|id| !id.is_empty())
    }
    pub fn line(&self) -> usize {
        self.0.item_line()
    }
    pub fn end_line(&self) -> Option<usize> {
        self.0.end_line()
    }
    pub fn references(&self) -> Vec<ItemReference<'_>> {
        item_references(&self.0)
    }
}

impl ParsedIrDocument {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn items(&self) -> &[ParsedItem] {
        &self.items
    }
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "public API: changing the parameter type breaks callers"
)]
pub fn parse(
    source: &crate::SourceText,
    options: IrOptions,
) -> Result<ParsedIrDocument, crate::InputError> {
    let place = crate::normalize_path(&options.ir);
    let relative = crate::sources::relative_to_place(source.path(), &place).ok_or_else(|| {
        crate::InputError::InvalidInput("IR source must be under its configured place".into())
    })?;
    let filename = relative
        .rsplit('/')
        .next()
        .expect("rsplit always yields at least one piece");
    let document = parse_document_mode(filename, DocKind::of(filename), source.text(), true)
        .map_err(|error| crate::InputError::InvalidInput(error.to_string()))?;
    let mut findings = document.parse_findings;
    for finding in &mut findings {
        finding.path = source.path().to_owned();
    }
    Ok(ParsedIrDocument {
        path: source.path().to_owned(),
        items: document.items.into_iter().map(ParsedItem).collect(),
        findings,
    })
}
