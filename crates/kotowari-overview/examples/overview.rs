use kotowari_core::{ReadInputs, ReadModel, SourceText};
fn main() {
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    inputs.ir = Some(vec![
        SourceText::new(
            "docs/ir/topic.md",
            "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- verification: review\n- how_to_verify: Read.\n\nStatement.\n",
        )
        .unwrap(),
    ]);
    inputs.records = Some(vec![]);
    inputs.adr = Some(vec![]);
    let read = ReadModel::build(inputs).unwrap();
    let data = SourceText::new(
        ".kotowari/overview/topic.md",
        "---\nir:\n  - docs/ir/topic.md\n---\n\n# Topic\n\n```view lead\nconclusion: One requirement\n```\n\n## Steps\n\n```view steps\nitems:\n  - title: Read\n    refs: [REQ-001]\n```\n",
    )
    .unwrap();
    let toc = SourceText::new(".kotowari/toc.yaml", "title: Example\nitems: [topic]\n").unwrap();
    let overview = kotowari_overview::inspect(&read, &[data], &toc);
    assert!(overview.findings().is_empty());
    let pages = overview.pages().unwrap();
    for page in &pages {
        println!("{}", page.name);
    }
    let group = overview.into_group();
    assert_eq!(group.tally().name(), kotowari_overview::GROUP);
}
