#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari_core::{ReadInputs, ReadModel, SourceText};
fn main() {
    let mut inputs = ReadInputs::default();
    inputs.config.tests.files.clear();
    inputs.decisions = Some(vec![
        SourceText::new(
            "docs/decisions/topic.md",
            "# Topic\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nStatement.\n\n### Confirmations\n\n- C-t-1 (by reading) Read it\n",
        )
        .unwrap(),
    ]);
    let read = ReadModel::build(inputs).unwrap();
    let data = SourceText::new(
        ".kotowari/overview/topic.md",
        "---\ndocuments:\n  - docs/decisions/topic.md\n---\n\n# Topic\n\n```view lead\nconclusion: One decision\n```\n\n## Steps\n\n```view steps\nitems:\n  - title: Read\n    refs: [C-t-1]\n```\n",
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
