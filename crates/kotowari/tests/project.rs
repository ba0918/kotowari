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
