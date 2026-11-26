#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari::{Project, ProjectOptions};

fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for path in [".kotowari", "docs/decisions"] {
        std::fs::create_dir_all(dir.path().join(path)).unwrap();
    }
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "decisions:\n  documents: docs/decisions\ntests:\n  files: []\n",
    )
    .unwrap();
    // テストの指していない確認することの行が1つあり、check の誤りになる
    std::fs::write(dir.path().join("docs/decisions/topic.md"), "# Topic\n\nScope.\n\n## Name\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n### Decision\n\nStatement.\n\n### Confirmations\n\n- C-t-1 Inspected\n").unwrap();
    dir
}

// @kotowari[C-core-libapi-1, C-core-libapi-4, C-core-libpaths-13, C-core-libinputs-9, C-core-libinputs-10]
#[test]
fn project_operations_reuse_owned_results_and_explicit_reads_observe_changes() {
    let dir = project();
    let cwd = std::env::current_dir().unwrap();
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let read = project.read().unwrap();
    let inspection = project.inspect().unwrap();
    assert!(read.query("C-t-1").is_ok());
    assert!(!read.list().items().is_empty());
    assert!(read.query("C-t-999").is_err());
    assert!(!inspection.check().findings().is_empty());
    assert!(!inspection.status().complete());
    std::fs::write(dir.path().join("docs/decisions/topic.md"), "# Changed\n").unwrap();
    assert!(read.query("C-t-1").is_ok());
    assert!(inspection.read().query("C-t-1").is_ok());
    assert!(project.read().unwrap().query("C-t-1").is_err());
    assert!(project.query("C-t-1").is_err());
    assert!(project.list().unwrap().items().is_empty());
    assert!(!project.check().unwrap().findings().is_empty());
    assert!(!project.status().unwrap().complete());
    assert_eq!(std::env::current_dir().unwrap(), cwd);
}

// @kotowari[C-core-libapi-1, C-core-libapi-5, C-core-libinputs-12, C-core-libpaths-11]
#[test]
fn project_rejects_relative_roots_and_plan_does_not_require_ir() {
    assert!(Project::new(ProjectOptions::new("relative")).is_err());
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("plan.md"), "# Incomplete\n").unwrap();
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    assert!(
        !project
            .plan(std::path::Path::new("plan.md"))
            .unwrap()
            .findings()
            .is_empty()
    );
    assert!(project.plan(std::path::Path::new("missing.md")).is_err());
}

// @kotowari[C-core-libapi-1, C-core-libapi-5, C-core-libinputs-12]
#[test]
fn mutants_have_typed_inputs_and_failures_without_ir_loading() {
    use kotowari::{MutantsOptions, Tool};
    let dir = tempfile::tempdir().unwrap();
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    std::fs::write(dir.path().join("outcomes.json"), "{\"outcomes\": []}").unwrap();
    let report = project
        .mutants(&MutantsOptions {
            tool: Tool::CargoMutants,
            results: "outcomes.json".into(),
        })
        .unwrap();
    assert!(report.findings().is_empty());
    assert!(
        project
            .mutants(&MutantsOptions {
                tool: Tool::CargoMutants,
                results: "missing.json".into()
            })
            .is_err()
    );
}

// @kotowari[C-core-libpaths-3, C-core-libpaths-12]
#[test]
fn absolute_resolution_bases_survive_a_child_process_directory_change() {
    const CHILD: &str = "KOTOWARI_PROJECT_BASIS_CHILD";
    if let Some(start) = std::env::var_os(CHILD) {
        let mut options = ProjectOptions::new(std::path::PathBuf::from(start));
        options.config = Some(".kotowari/config.yaml".into());
        let project = Project::new(options).unwrap();
        let other = std::env::var_os("KOTOWARI_PROJECT_OTHER_ROOT").unwrap();
        std::env::set_current_dir(other).unwrap();
        assert!(project.query("C-t-1").is_ok());
        return;
    }
    let original = project();
    let other = tempfile::tempdir().unwrap();
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "absolute_resolution_bases_survive_a_child_process_directory_change",
        ])
        .env(CHILD, original.path())
        .env("KOTOWARI_PROJECT_OTHER_ROOT", other.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
}
// @kotowari[C-core-libcrates-10]
#[test]
fn facade_names_are_the_same_lower_result_types() {
    fn same_query(value: &kotowari::QueryItem) -> &kotowari_core::QueryItem {
        value
    }
    fn same_decision(value: &kotowari::DecisionItem) -> &kotowari_core::DecisionItem {
        value
    }
    fn same_guides(value: &kotowari::GuideTally) -> &kotowari_core::guides::GuideTally {
        value
    }
    fn same_mutants(value: &kotowari::MutantCounts) -> &kotowari_core::mutants::MutantCounts {
        value
    }
    fn same_guide_reader(model: &kotowari::ReadModel) -> kotowari::GuideReader {
        model.guide_reader()
    }
    fn same_documents(model: &kotowari::ReadModel) -> Vec<kotowari_core::ParsedDecisionDocument> {
        model.decision_documents()
    }
    let _ = (
        same_query,
        same_decision,
        same_guides,
        same_mutants,
        same_guide_reader,
        same_documents,
    );
}

// @kotowari[C-core-libapi-5, C-core-libinputs-1, C-core-libpaths-5]
#[test]
fn public_admission_errors_keep_distinct_facade_classifications_and_details() {
    use kotowari::ErrorKind;
    use kotowari_core::{ReadInputs, ReadModel, SourceText};
    let invalid = SourceText::new("/absolute/topic.md", "text").unwrap_err();
    let missing = ReadModel::build(ReadInputs::default()).err().unwrap();
    let mut malformed = ReadInputs::default();
    malformed.config.tests.files = vec!["[".into()];
    let config = ReadModel::build(malformed).err().unwrap();
    for (error, expected) in [
        (invalid, ErrorKind::InvalidInput),
        (missing, ErrorKind::InputMissing),
        (config, ErrorKind::ConfigError),
    ] {
        let detail = error.to_string();
        let facade = kotowari::Error::from(error);
        assert_eq!(facade.kind(), expected);
        assert!(facade.to_string().contains(&detail));
    }
}

fn check_with_config(dir: &std::path::Path, config: &str) -> kotowari::Error {
    let mut options = ProjectOptions::new(dir);
    options.config = Some(config.into());
    Project::new(options).unwrap().check().err().unwrap()
}

// @kotowari[C-core-libapi-7]
#[test]
fn ex_core_554_a_configuration_file_that_is_a_directory_is_invalid_input_like_a_missing_one() {
    use kotowari::ErrorKind;
    let dir = project();
    let directory = check_with_config(dir.path(), ".kotowari");
    let missing = check_with_config(dir.path(), "missing.yaml");
    assert_eq!(missing.kind(), ErrorKind::InvalidInput);
    assert_eq!(directory.kind(), missing.kind());
    assert_ne!(directory.kind(), ErrorKind::ReadFailure);
}

// @kotowari[C-core-libapi-8]
#[test]
fn ex_core_555_a_directory_at_the_default_location_is_a_read_failure() {
    let dir = project();
    let config = dir.path().join(".kotowari/config.yaml");
    std::fs::remove_file(&config).unwrap();
    std::fs::create_dir(&config).unwrap();
    let error = Project::new(ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .err()
        .unwrap();
    assert_eq!(error.kind(), kotowari::ErrorKind::ReadFailure);
}

// @kotowari[C-core-libapi-9]
#[test]
fn ex_core_556_the_unknown_id_can_be_taken_from_a_failed_query() {
    let dir = project();
    let error = Project::new(ProjectOptions::new(dir.path()))
        .unwrap()
        .query("REQ-core-999")
        .err()
        .unwrap();
    assert_eq!(error.kind(), kotowari::ErrorKind::UnknownQuery);
    assert_eq!(error.unknown_id(), Some("REQ-core-999"));
}

// @kotowari[C-core-libapi-10, C-core-libapi-5]
#[test]
fn ex_core_557_a_failure_of_another_kind_provides_no_unknown_id() {
    let dir = project();
    std::fs::remove_dir_all(dir.path().join("docs/decisions")).unwrap();
    let error = Project::new(ProjectOptions::new(dir.path()))
        .unwrap()
        .check()
        .err()
        .unwrap();
    assert_eq!(error.kind(), kotowari::ErrorKind::ReadFailure);
    assert_eq!(error.unknown_id(), None);
}

// @kotowari[C-core-libapi-2, C-core-ovout-2]
#[test]
fn project_check_status_and_inspect_include_the_overview_group() {
    let dir = project();
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "tests:\n  files: []\ndecisions:\n  documents: docs/decisions\noverview:\n  files: ['.kotowari/overview/*.md']\n  toc: .kotowari/toc.yaml\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join("docs/decisions")).unwrap();
    std::fs::write(
        dir.path().join("docs/decisions/topic.md"),
        "# Topic\n\nDecides the topic.\n\n## Do it\n\nStatus: proposed\n\n### Problem\n\nP.\n\n### Decision\n\nD.\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join(".kotowari/overview")).unwrap();
    std::fs::write(
        dir.path().join(".kotowari/overview/a.md"),
        "---\ndocuments:\n  - docs/decisions/topic.md\n---\n\n# a\n\n## no lead\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join(".kotowari/toc.yaml"),
        "title: t\nitems: [a]\n",
    )
    .unwrap();
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let lead_missing = |findings: &[kotowari::Finding]| {
        findings
            .iter()
            .filter(|finding| finding.kind() == kotowari::FindingKind::OverviewLeadMissing)
            .count()
    };
    let check = project.check().unwrap();
    assert_eq!(lead_missing(check.findings()), 1);
    let group = check.group(kotowari::OVERVIEW_GROUP).unwrap();
    assert_eq!((group.files(), group.marks()), (1, 0));
    let status = project.status().unwrap();
    assert!(!status.complete());
    assert_eq!(status.group(kotowari::OVERVIEW_GROUP).unwrap().files(), 1);
    let inspection = project.inspect().unwrap();
    assert_eq!(lead_missing(inspection.check().findings()), 1);
}

fn overview_project(lead: bool) -> tempfile::TempDir {
    let dir = project();
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "tests:\n  files: []\ndecisions:\n  documents: docs/decisions\noverview:\n  files: ['.kotowari/overview/*.md']\n  toc: .kotowari/toc.yaml\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join("docs/decisions")).unwrap();
    std::fs::write(
        dir.path().join("docs/decisions/topic.md"),
        "# Topic\n\nDecides the topic.\n\n## Do it\n\nStatus: proposed\n\n### Problem\n\nP.\n\n### Decision\n\nD.\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join(".kotowari/overview")).unwrap();
    let lead = if lead {
        "```view lead\nconclusion: c\n```\n"
    } else {
        ""
    };
    std::fs::write(
        dir.path().join(".kotowari/overview/a.md"),
        format!(
            "---\ndocuments:\n  - docs/decisions/topic.md\n---\n\n# a\n\n{lead}\n## s\n\ntext\n"
        ),
    )
    .unwrap();
    std::fs::write(
        dir.path().join(".kotowari/toc.yaml"),
        "title: t\nitems: [a]\n",
    )
    .unwrap();
    dir
}

// @kotowari[C-core-libapi-3, C-core-libapi-13, C-core-ovcmd-1, C-core-ovcmd-2]
#[test]
fn overview_prepare_writes_nothing_and_overview_build_writes_the_cache() {
    let dir = overview_project(true);
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let prepared = project.overview_prepare().unwrap();
    let names: Vec<&str> = prepared
        .pages()
        .iter()
        .map(|page| page.name.as_str())
        .collect();
    assert_eq!(names, ["a.html", "index.html", "style.css"]);
    assert!(!dir.path().join(".kotowari/cache").exists());
    let build = project.overview_build().unwrap();
    assert_eq!(build.written().len(), 3);
    assert_eq!(build.unchanged(), 0);
    let again = prepared.write().unwrap();
    assert!(again.written().is_empty() && again.removed().is_empty());
    assert_eq!(again.unchanged(), 3);
}

// @kotowari[C-core-libapi-5, C-core-ovcmd-8, C-core-ovdata-2]
#[test]
fn overview_operations_fail_with_kinds_for_bad_data_and_a_missing_key() {
    let dir = overview_project(false);
    let broken = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let error = broken.overview_build().unwrap_err();
    assert_eq!(error.kind(), kotowari::ErrorKind::OverviewData);
    assert!(!dir.path().join(".kotowari/cache").exists());
    let plain = project();
    let error = Project::new(ProjectOptions::new(plain.path()))
        .unwrap()
        .overview_prepare()
        .unwrap_err();
    assert_eq!(error.kind(), kotowari::ErrorKind::ConfigError);
}

// @kotowari[C-core-libapi-5, C-core-ovcmd-6]
#[test]
fn overview_build_stops_with_a_cache_error_when_the_place_is_not_a_directory() {
    let dir = overview_project(true);
    std::fs::write(dir.path().join(".kotowari/cache"), "a file").unwrap();
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let error = project.overview_build().unwrap_err();
    assert_eq!(error.kind(), kotowari::ErrorKind::CacheFailure);
    assert_eq!(error.detail(), "cache error: .kotowari/cache");
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".kotowari/cache")).unwrap(),
        "a file"
    );
}

// @kotowari[C-core-libapi-5, C-core-ovcmd-7]
#[test]
fn overview_build_stops_with_a_cache_error_and_the_os_error_when_a_write_fails() {
    let dir = overview_project(true);
    // 書くページの名前にディレクトリがあると、書き込みが失敗する
    std::fs::create_dir_all(dir.path().join(".kotowari/cache/overview/index.html")).unwrap();
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let error = project.overview_build().unwrap_err();
    assert_eq!(error.kind(), kotowari::ErrorKind::CacheFailure);
    let detail = error.detail();
    assert!(
        detail.starts_with("cache error: .kotowari/cache/overview/index.html: "),
        "{detail}"
    );
    assert!(detail.len() > "cache error: .kotowari/cache/overview/index.html: ".len());
}

// @kotowari[C-core-clienv-7, C-core-ovcmd-7]
#[cfg(unix)]
#[test]
fn overview_write_names_the_place_it_could_not_inspect() {
    use std::os::unix::fs::PermissionsExt;
    let dir = overview_project(true);
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let prepared = project.overview_prepare().unwrap();
    let place = dir.path().join(".kotowari");
    std::fs::set_permissions(&place, std::fs::Permissions::from_mode(0o000)).unwrap();
    // 権限を無視できる実行者（root）では、中を調べられない置き場を作れない
    let inspectable = std::fs::symlink_metadata(place.join("cache")).is_ok();
    let result = prepared.write();
    std::fs::set_permissions(&place, std::fs::Permissions::from_mode(0o755)).unwrap();
    if inspectable {
        return;
    }
    let error = result.unwrap_err();
    assert_eq!(error.kind(), kotowari::ErrorKind::CacheFailure);
    assert!(
        error.detail().starts_with("cache error: .kotowari/cache: "),
        "{}",
        error.detail()
    );
}
