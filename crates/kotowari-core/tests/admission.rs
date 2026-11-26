#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]
use kotowari_core::{
    NativeSourceText, NativeSurfaceAnalysis, NativeTestAnalysis, ReadModel, RepositoryCheckInputs,
    RepositoryReadInputs, SourceText, TestAnalysis, config::Config,
};

fn source(identity: &str, path: &str, text: &str) -> NativeSourceText {
    NativeSourceText::new(identity, path, text)
}
fn inputs(config: Config) -> RepositoryReadInputs {
    RepositoryReadInputs {
        config,
        decisions: vec![],
        tests: vec![],
    }
}
fn test(identity: &str, path: &str, text: &str) -> NativeTestAnalysis {
    NativeTestAnalysis {
        source: source(identity, path, text),
        analysis: TestAnalysis {
            source: SourceText::new(path, text).unwrap(),
            has_query: true,
            tests: vec![],
            line_markers: vec![],
            findings: vec![],
        },
    }
}

// @kotowari[C-core-libpaths-9]
#[test]
fn repeated_native_source_is_rejected_within_a_group() {
    let mut input = inputs(Config::default());
    input.config.tests.files.clear();
    input.decisions = vec![
        source(
            "docs/decisions/topic.md",
            "docs/decisions/topic.md",
            "# Topic\n",
        ),
        source(
            "docs/decisions/topic.md",
            "docs/decisions/topic.md",
            "# Topic\n",
        ),
    ];
    assert!(ReadModel::build_repository(input).is_err());
}

// @kotowari[C-core-libpaths-9]
#[test]
fn native_source_without_an_identity_is_rejected() {
    let mut input = inputs(Config::default());
    input.config.tests.files.clear();
    input.decisions = vec![source("", "docs/decisions/topic.md", "# Topic\n")];
    let Err(error) = ReadModel::build_repository(input) else {
        panic!("a native source needs its identity");
    };
    assert!(
        error
            .to_string()
            .contains("native source identity must not be empty"),
        "{error}"
    );
}

// @kotowari[C-core-libpaths-10]
#[test]
fn native_original_text_is_retained_across_read_and_check() {
    let mut input = inputs(Config::default());
    input.config.tests.files.clear();
    input.config.guides.files = vec!["docs/decisions/**".into()];
    input.decisions = vec![source(
        "docs/decisions/topic.md",
        "docs/decisions/topic.md",
        "# Topic\n",
    )];
    let read = ReadModel::build_repository(input).unwrap();
    assert!(
        read.inspect_repository(RepositoryCheckInputs {
            guides: Some(vec![source(
                "docs/decisions/topic.md",
                "different-display.md",
                "different"
            )]),
            ..Default::default()
        })
        .is_err()
    );
}

// @kotowari[C-core-libinputs-3]
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

// @kotowari[C-core-libinputs-3]
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

// @kotowari[C-core-libpaths-9]
#[test]
fn native_display_collisions_do_not_merge_original_files() {
    let mut input = inputs(Config::default());
    input.tests = vec![
        test("tests/a\\b.rs", "tests/a/b.rs", "first"),
        test("tests/a/b.rs", "tests/a/b.rs", "second"),
    ];
    assert_eq!(input.tests[0].source.path(), "tests/a/b.rs");
    assert_eq!(input.tests[1].source.path(), "tests/a/b.rs");
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

// @kotowari[C-core-libinputs-1, C-core-libinputs-2]
#[test]
fn native_inspection_cannot_finish_without_its_required_guides() {
    let read = || {
        let mut config = Config::default();
        config.tests.files.clear();
        config.guides.files = vec!["guides/**".into()];
        ReadModel::build_repository(inputs(config)).unwrap()
    };
    assert!(read().prepare_repository_inspection().finish().is_err());
    let mut preparation = read().prepare_repository_inspection();
    preparation.guides(Some(vec![])).unwrap();
    preparation.surface(None, None).unwrap();
    assert_eq!(preparation.finish().unwrap().check().guides().files(), 0);
}

// @kotowari[C-core-libpaths-9]
#[test]
fn repeated_original_test_identity_cannot_be_evaded_by_display_spelling() {
    let mut input = inputs(Config::default());
    input.tests = vec![
        test("tests/a.rs", "tests/a.rs", "same"),
        test("tests/a.rs", "other.rs", "same"),
    ];
    assert!(ReadModel::build_repository(input).is_err());
}

// @kotowari[C-core-guides-9, C-core-libinputs-6]
#[test]
fn a_guide_path_that_is_also_a_test_file_stops_the_native_inspection() {
    let mut config = Config::default();
    config.tests.files = vec!["notes/**".into()];
    config.guides.files = vec!["notes/**".into()];
    let mut input = inputs(config);
    input.tests = vec![test("notes/x.md", "notes/x.md", "text")];
    let preparation = ReadModel::build_repository(input)
        .unwrap()
        .prepare_repository_inspection();
    assert!(preparation.validate_guide_paths(["notes/y.md"]).is_ok());
    let stop = preparation
        .validate_guide_paths(["notes/x.md"])
        .unwrap_err()
        .to_string();
    assert!(stop.starts_with("config error: notes/x.md"), "{stop}");
}
