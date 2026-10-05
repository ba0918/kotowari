#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari::{Project, ProjectOptions};

fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for path in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(dir.path().join(path)).unwrap();
    }
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "tests:\n  files: []\n",
    )
    .unwrap();
    std::fs::write(dir.path().join("docs/ir/topic.md"), "# Topic\n\nScope.\n\n## Requirements\n\n### REQ-001: Name\n\n- kind: ubiquitous\n- verification: review\n- how_to_verify: Inspect.\n\nStatement.\n").unwrap();
    dir
}

// @kotowari[REQ-core-310, REQ-core-311, REQ-core-312, REQ-core-316, REQ-core-317, EX-core-483, EX-core-484, EX-core-490, EX-core-491]
#[test]
fn project_operations_reuse_owned_results_and_explicit_reads_observe_changes() {
    let dir = project();
    let cwd = std::env::current_dir().unwrap();
    let project = Project::new(ProjectOptions::new(dir.path())).unwrap();
    let read = project.read().unwrap();
    let inspection = project.inspect().unwrap();
    assert!(read.query("REQ-001").is_ok());
    assert!(!read.list().items().is_empty());
    assert!(read.query("REQ-999").is_err());
    assert!(!inspection.check().findings().is_empty());
    assert!(!inspection.status().complete());
    std::fs::write(dir.path().join("docs/ir/topic.md"), "# Changed\n").unwrap();
    assert!(read.query("REQ-001").is_ok());
    assert!(inspection.read().query("REQ-001").is_ok());
    assert!(project.read().unwrap().query("REQ-001").is_err());
    assert!(project.query("REQ-001").is_err());
    assert!(project.list().unwrap().items().is_empty());
    assert!(!project.check().unwrap().findings().is_empty());
    assert!(!project.status().unwrap().complete());
    assert_eq!(std::env::current_dir().unwrap(), cwd);
}

// @kotowari[REQ-core-310, REQ-core-312, REQ-core-323, EX-core-484, EX-core-486, EX-core-500]
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

// @kotowari[REQ-core-310, REQ-core-312, EX-core-484, EX-core-486]
#[test]
fn mutants_and_changes_have_typed_inputs_and_failures_without_ir_loading() {
    use kotowari::{ChangesOptions, ErrorKind, MutantsOptions, Phase, Target, Tool};
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
    let error = project
        .changes(&ChangesOptions {
            base: "HEAD".into(),
            target: Target::Index,
            phase: Phase::Implementation,
        })
        .err()
        .unwrap();
    assert_eq!(error.kind(), ErrorKind::GitFailure);
}

// @kotowari[REQ-core-311, REQ-core-323, EX-core-483, EX-core-501]
#[test]
fn absolute_resolution_bases_survive_a_child_process_directory_change() {
    const CHILD: &str = "KOTOWARI_PROJECT_BASIS_CHILD";
    if let Some(start) = std::env::var_os(CHILD) {
        let mut options = ProjectOptions::new(std::path::PathBuf::from(start));
        options.config = Some(".kotowari/config.yaml".into());
        let project = Project::new(options).unwrap();
        let other = std::env::var_os("KOTOWARI_PROJECT_OTHER_ROOT").unwrap();
        std::env::set_current_dir(other).unwrap();
        assert!(project.query("REQ-001").is_ok());
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
// @kotowari[REQ-core-322, EX-core-499]
#[test]
fn facade_names_are_the_same_lower_result_types() {
    fn same_query(value: &kotowari::QueryItem) -> &kotowari_core::QueryItem {
        value
    }
    fn same_requirement(value: &kotowari::RequirementItem) -> &kotowari_core::RequirementItem {
        value
    }
    fn same_guides(value: &kotowari::GuideTally) -> &kotowari_core::guides::GuideTally {
        value
    }
    fn same_mutants(value: &kotowari::MutantCounts) -> &kotowari_core::mutants::MutantCounts {
        value
    }
    let _ = (same_query, same_requirement, same_guides, same_mutants);
}

// @kotowari[REQ-core-312, REQ-core-315, REQ-core-322, EX-core-484, EX-core-488, EX-core-498]
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

// @kotowari[REQ-core-290, REQ-core-310, TBL-core-041]
#[test]
fn project_check_status_and_inspect_include_the_overview_group() {
    let dir = project();
    std::fs::write(
        dir.path().join(".kotowari/config.yaml"),
        "tests:\n  files: []\noverview:\n  files: ['.kotowari/overview/*.md']\n  toc: .kotowari/toc.yaml\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.path().join(".kotowari/overview")).unwrap();
    std::fs::write(
        dir.path().join(".kotowari/overview/a.md"),
        "---\nir:\n  - docs/ir/topic.md\n---\n\n# a\n\n## no lead\n",
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
        "tests:\n  files: []\noverview:\n  files: ['.kotowari/overview/*.md']\n  toc: .kotowari/toc.yaml\n",
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
        format!("---\nir:\n  - docs/ir/topic.md\n---\n\n# a\n\n{lead}\n## s\n\ntext\n"),
    )
    .unwrap();
    std::fs::write(
        dir.path().join(".kotowari/toc.yaml"),
        "title: t\nitems: [a]\n",
    )
    .unwrap();
    dir
}

// @kotowari[REQ-core-310, TBL-core-041, REQ-core-293, REQ-core-294]
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

// @kotowari[REQ-core-294, REQ-core-312, REQ-core-279]
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

// @kotowari[REQ-core-324, REQ-core-312]
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

// @kotowari[REQ-core-324, REQ-core-312]
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

// @kotowari[REQ-core-324, TBL-core-020]
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
