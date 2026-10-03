use kotowari_core::{
    Comparison, NativeSourceText, NativeSurfaceAnalysis, NativeTestAnalysis, ReadModel,
    RepositoryCheckInputs, RepositoryReadInputs, SourceText, TestAnalysis,
    changes::{self, Phase},
    config::{ChangesConfig, Config},
};

fn source(identity: &str, path: &str, text: &str) -> NativeSourceText {
    NativeSourceText::new(identity, path, text)
}
fn inputs(config: Config) -> RepositoryReadInputs {
    RepositoryReadInputs {
        config,
        ir: vec![],
        records: vec![],
        adr: vec![],
        tests: vec![],
    }
}
fn test(identity: &str, path: &str, text: &str) -> NativeTestAnalysis {
    NativeTestAnalysis {
        source: source(identity, path, text),
        analysis: TestAnalysis {
            source: SourceText::new(path, text).unwrap(),
            language: Some("rust".into()),
            has_query: true,
            tests: vec![],
            line_markers: vec![],
            findings: vec![],
        },
    }
}

// @kotowari[REQ-core-315, EX-core-490]
#[test]
fn repeated_native_source_is_rejected_within_a_group() {
    let mut input = inputs(Config::default());
    input.config.tests.files.clear();
    input.ir = vec![
        source("docs/ir/topic.md", "topic.md", "# Topic\n"),
        source("docs/ir/topic.md", "topic.md", "# Topic\n"),
    ];
    assert!(ReadModel::build_repository(input).is_err());
}

// @kotowari[REQ-core-315, EX-core-490]
#[test]
fn native_original_text_is_retained_across_read_and_check() {
    let mut input = inputs(Config::default());
    input.config.tests.files.clear();
    input.config.guides.files = vec!["docs/ir/**".into()];
    input.ir = vec![source("docs/ir/topic.md", "topic.md", "# Topic\n")];
    let read = ReadModel::build_repository(input).unwrap();
    assert!(
        read.inspect_repository(RepositoryCheckInputs {
            guides: Some(vec![source(
                "docs/ir/topic.md",
                "different-display.md",
                "different"
            )]),
            ..Default::default()
        })
        .is_err()
    );
}

// @kotowari[REQ-core-312, EX-core-484]
#[test]
fn malformed_comparison_globs_return_a_configuration_error() {
    let mut config = Config::default();
    config.changes = Some(ChangesConfig {
        files: vec!["src/**".into()],
        records: vec!["[".into()],
        exclude: vec![],
    });
    let snapshot = Comparison {
        base: "base".into(),
        target: "head".into(),
        config,
        files: vec![],
        blobs: Default::default(),
    };
    assert!(matches!(
        changes::inspect(&snapshot, Phase::Implementation),
        Err(kotowari_core::StopReason::ConfigError(_))
    ));
    assert_eq!(
        changes::evaluate(&snapshot, &[], Phase::Implementation).files(),
        0
    );
}

// @kotowari[REQ-core-315, EX-core-488, EX-core-489]
#[test]
fn native_disabled_tests_do_not_contribute_stale_facts() {
    let mut input = inputs(Config::default());
    input.config.tests.files.clear();
    input.tests = vec![test("tests/stale.rs", "tests/stale.rs", "")];
    let inspection = ReadModel::build_repository(input)
        .unwrap()
        .inspect_repository(Default::default())
        .unwrap();
    assert!(inspection.check().tests().is_empty());
}

// @kotowari[REQ-core-315, EX-core-488, EX-core-489]
#[test]
fn native_disabled_unspecified_list_does_not_hide_a_surface() {
    let mut input = inputs(Config::default());
    input.config.tests.files.clear();
    input.config.surface.files = vec!["src/**".into()];
    input.config.surface.rules = vec!["rules.yaml".into()];
    let original = source("src/main.rs", "src/main.rs", "fn main() {}");
    let input_check = RepositoryCheckInputs {
        surface: Some(vec![NativeSurfaceAnalysis {
            source: original,
            analysis: kotowari_core::SurfaceAnalysis {
                source: SourceText::new("src/main.rs", "fn main() {}").unwrap(),
                language: Some("rust".into()),
                has_query: true,
                surfaces: vec![kotowari_core::surface::Surface {
                    kind: "command".into(),
                    name: "main".into(),
                    path: "src/main.rs".into(),
                    line: 1,
                }],
                findings: vec![],
            },
        }]),
        unspecified: Some(vec![source(
            "unspecified.yaml",
            "unspecified.yaml",
            "- kind: command\n  name: main\n  why: external\n",
        )]),
        ..Default::default()
    };
    let inspection = ReadModel::build_repository(input)
        .unwrap()
        .inspect_repository(input_check)
        .unwrap();
    assert!(
        inspection
            .check()
            .findings()
            .iter()
            .any(|finding| finding.kind() == kotowari_core::FindingKind::SurfaceWithoutSpec)
    );
}

// @kotowari[REQ-core-315, REQ-core-313, EX-core-490, EX-core-485]
#[test]
fn native_display_collisions_do_not_merge_original_files() {
    let mut input = inputs(Config::default());
    input.tests = vec![
        test("tests/a\\b.rs", "tests/a/b.rs", "first"),
        test("tests/a/b.rs", "tests/a/b.rs", "second"),
    ];
    input.config.guides.files = vec!["guides/**".into()];
    let inspection = ReadModel::build_repository(input)
        .unwrap()
        .inspect_repository(RepositoryCheckInputs {
            guides: Some(vec![
                source("guides/a\\b.md", "guides/a/b.md", "first"),
                source("guides/a/b.md", "guides/a/b.md", "second"),
            ]),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(inspection.check().tests()["rs"].files(), 2);
    assert_eq!(inspection.check().guides().files(), 2);
}

// @kotowari[REQ-core-315, EX-core-490]
#[test]
fn repeated_original_test_identity_cannot_be_evaded_by_display_spelling() {
    let mut input = inputs(Config::default());
    input.tests = vec![
        test("tests/a.rs", "tests/a.rs", "same"),
        test("tests/a.rs", "other.rs", "same"),
    ];
    assert!(ReadModel::build_repository(input).is_err());
}
