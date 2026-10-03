//! 面の検査（docs/ir/core/surface.md、surface-unspecified.md）と、その設定の鍵 "surface"

use kotowari_core::config::Config;

// @kotowari[TBL-core-004]
#[test]
fn tbl_004_surface_keys_default_to_empty_lists_and_no_list_file() {
    let cfg = Config::default();
    assert!(cfg.surface.files.is_empty());
    assert!(cfg.surface.rules.is_empty());
    assert_eq!(cfg.surface.unspecified, None);
}

// @kotowari[TBL-core-004, REQ-core-224]
#[test]
fn tbl_004_surface_keys_are_read_and_their_paths_normalized() {
    let cfg = Config::parse(
        "surface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"./rules/surface.yml\"\n  unspecified: \"docs//surface.yaml\"\n",
    )
    .unwrap();
    assert_eq!(cfg.surface.files, vec!["src/**/*.rs"]);
    assert_eq!(cfg.surface.rules, vec!["rules/surface.yml"]);
    assert_eq!(
        cfg.surface.unspecified.as_deref(),
        Some("docs/surface.yaml")
    );
}

// @kotowari[REQ-core-014]
#[test]
fn req_014_an_invalid_surface_files_glob_stops() {
    let result = Config::parse(
        "surface:\n  files:\n    - \"[invalid\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    assert!(result.is_err(), "invalid glob in surface.files should stop");
}

// @kotowari[REQ-core-014]
#[test]
fn req_014_null_or_absolute_surface_values_stop() {
    for yaml in [
        "surface:\n",
        "surface:\n  files:\n  rules:\n    - \"r.yml\"\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"r.yml\"\n  unspecified:\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"/r.yml\"\n",
        "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"r.yml\"\n  unspecified: \"/s.yaml\"\n",
    ] {
        assert!(Config::parse(yaml).is_err(), "should stop: {yaml}");
    }
}
