use kotowari_markdown_schema_io::{LoaderOptions, SchemaLoader};
use std::path::{Path, PathBuf};

// @kotowari[REQ-schema-044, REQ-schema-073]
#[test]
fn directory_checks_dot_prefixed_documents_but_skips_hidden_directories() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("schema.yaml"), "document:\n  title: {}\n").unwrap();
    std::fs::write(
        root.path().join(".draft.md"),
        "---\n$schema: schema.yaml\n---\n",
    )
    .unwrap();
    std::fs::create_dir(root.path().join(".hidden")).unwrap();
    std::fs::write(
        root.path().join(".hidden/ignored.md"),
        "---\n$schema: missing.yaml\n---\n",
    )
    .unwrap();
    let loader = SchemaLoader::new(LoaderOptions::new(root.path().to_path_buf())).unwrap();
    let result = loader.check(Path::new("."), Default::default()).unwrap();
    assert_eq!(result.files().len(), 1);
    assert_eq!(result.files()[0].path().file_name().unwrap(), ".draft.md");
    assert!(
        result.files()[0]
            .findings()
            .iter()
            .any(|finding| finding.kind().as_str() == "missing_title")
    );
}

// @kotowari[REQ-core-323, EX-core-500]
#[test]
fn relative_start_and_cache_bases_are_rejected() {
    let root = tempfile::tempdir().unwrap();
    assert!(SchemaLoader::new(LoaderOptions::new(PathBuf::from("relative"))).is_err());
    let mut options = LoaderOptions::new(root.path().to_path_buf());
    options.cache_base = Some(PathBuf::from("relative"));
    assert!(SchemaLoader::new(options).is_err());
}

// @kotowari[REQ-schema-072, REQ-schema-073, EX-schema-090]
#[test]
fn loaded_pairs_are_reusable_and_findings_do_not_prevent_partial_extraction() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("schema.yaml"),
        "document:\n  title:\n    pattern: '^Expected$'\n    extract: title\n",
    )
    .unwrap();
    std::fs::write(
        root.path().join("doc.md"),
        "---\n$schema: schema.yaml\n---\n# Other\n",
    )
    .unwrap();
    let loader = SchemaLoader::new(LoaderOptions::new(root.path().to_path_buf())).unwrap();
    let pair = loader.load(Path::new("doc.md")).unwrap();
    assert_eq!(pair.document().raw_line(4), Some("# Other"));
    assert_eq!(pair.document().title_line(), Some(4));
    assert!(!pair.validate(Default::default()).is_empty());
    assert!(pair.extract_validated(Default::default()).is_err());
    assert!(
        loader
            .extract_validated(Path::new("doc.md"), Default::default())
            .unwrap()
            .is_err()
    );
    assert_eq!(
        loader
            .extract_partial(Path::new("doc.md"), Default::default())
            .unwrap()
            .values()["title"],
        "Other"
    );
    assert_eq!(
        pair.extract_partial(Default::default()).values()["title"],
        "Other"
    );
    assert_eq!(
        loader
            .check(Path::new("doc.md"), Default::default())
            .unwrap()
            .files()
            .len(),
        1
    );
}

// @kotowari[REQ-schema-074, REQ-schema-073, EX-schema-091]
#[test]
fn execution_failures_are_typed_and_distinct_from_document_findings() {
    use kotowari_markdown_schema_io::ErrorKind;
    let root = tempfile::tempdir().unwrap();
    let loader = SchemaLoader::new(LoaderOptions::new(root.path().to_path_buf())).unwrap();
    let missing = loader
        .check(Path::new("missing.md"), Default::default())
        .unwrap_err();
    assert_eq!(missing.kind(), ErrorKind::UnreadableFile);
    let _: &dyn std::error::Error = &missing;
    assert!(!missing.to_string().is_empty());
    std::fs::write(
        root.path().join("doc.md"),
        "---\n$schema: schema.yaml\n---\n# Title\n",
    )
    .unwrap();
    assert_eq!(
        loader.load(Path::new("doc.md")).unwrap_err().kind(),
        ErrorKind::SchemaNotFound
    );
    std::fs::write(root.path().join("schema.yaml"), "not: a-schema").unwrap();
    assert_eq!(
        loader.load(Path::new("doc.md")).unwrap_err().kind(),
        ErrorKind::SchemaInvalid
    );
    std::fs::write(root.path().join("doc.md"), [0xff]).unwrap();
    assert_eq!(
        loader.load(Path::new("doc.md")).unwrap_err().kind(),
        ErrorKind::UnreadableFile
    );
}

// @kotowari[REQ-core-323, EX-core-501, REQ-schema-073]
#[test]
fn later_current_directory_changes_do_not_change_the_load_base() {
    const CHILD: &str = "KOTOWARI_LOADER_CWD_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        std::fs::write(
            first.path().join("schema.yaml"),
            "open: true\ndocument: {}\n",
        )
        .unwrap();
        std::fs::write(
            first.path().join("doc.md"),
            "---\n$schema: schema.yaml\n---\n# First\n",
        )
        .unwrap();
        let loader = SchemaLoader::new(LoaderOptions::new(first.path().to_path_buf())).unwrap();
        std::env::set_current_dir(second.path()).unwrap();
        assert!(loader.load(Path::new("doc.md")).is_ok());
        assert_eq!(std::env::current_dir().unwrap(), second.path());
        return;
    }
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "later_current_directory_changes_do_not_change_the_load_base",
        ])
        .env(CHILD, "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
