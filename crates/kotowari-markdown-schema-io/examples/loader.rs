#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari_markdown_schema_io::{LoaderOptions, SchemaLoader};
fn main() {
    let root = std::env::current_dir()
        .unwrap()
        .join(format!("loader-example-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    std::fs::write(
        root.join("schema.yaml"),
        "document:\n  title:\n    pattern: '^Expected$'\n    extract: title\n",
    )
    .unwrap();
    std::fs::write(
        root.join("doc.md"),
        "---\n$schema: schema.yaml\n---\n# Expected\n",
    )
    .unwrap();
    let loader = SchemaLoader::new(LoaderOptions::new(root.clone())).unwrap();
    let path = std::path::Path::new("doc.md");
    assert!(
        loader
            .load(path)
            .unwrap()
            .extract_validated(Default::default())
            .is_ok()
    );
    #[cfg(feature = "tokio")]
    {
        let adapter = kotowari_markdown_schema_io::AsyncSchemaLoader::new(
            LoaderOptions::new(root.clone()),
            Default::default(),
        )
        .unwrap();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        assert_eq!(
            runtime
                .block_on(adapter.check(path, Default::default()))
                .unwrap()
                .files()
                .len(),
            1
        );
    }
    std::fs::remove_dir_all(root).unwrap();
}
