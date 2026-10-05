#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari::{Project, ProjectOptions};
fn main() {
    let root = std::env::current_dir()
        .unwrap()
        .join(format!("project-example-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    for path in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(root.join(path)).unwrap();
    }
    std::fs::write(root.join(".kotowari/config.yaml"), "tests:\n  files: []\n").unwrap();
    std::fs::write(root.join("docs/ir/topic.md"), "# Topic\n").unwrap();
    let project = Project::new(ProjectOptions::new(&root)).unwrap();
    let read = project.read().unwrap();
    let inspection = project.inspect().unwrap();
    assert_eq!(
        read.list().items().len(),
        inspection.read().list().items().len()
    );
    assert!(!project.check().unwrap().findings().is_empty());
    #[cfg(feature = "tokio")]
    {
        let adapter =
            kotowari::AsyncProject::new(ProjectOptions::new(&root), Default::default()).unwrap();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        assert_eq!(
            runtime.block_on(adapter.check()).unwrap().counts(),
            inspection.check().counts()
        );
    }
    std::fs::remove_dir_all(root).unwrap();
}
