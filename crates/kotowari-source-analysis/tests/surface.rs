use kotowari_core::config::Config;
use std::path::Path;
use tempfile::TempDir;
const SURFACE_SRC: &str =
    "surface:\n  files:\n    - \"src/**\"\n  rules:\n    - \"rules/surface.yml\"\n";

const FLAG_RULE: &str =
    "id: flag\nlanguage: rust\nrule:\n  kind: string_literal\n  regex: '^\"--'\n  pattern: $NAME\n";

/// 置き場と設定を作る。`surface` の行をそのまま設定に足す
fn make_project(tmp: &Path, surface: &str) {
    for dir in [
        ".kotowari",
        "docs/ir",
        "docs/decision/records",
        "docs/decision/adr",
    ] {
        std::fs::create_dir_all(tmp.join(dir)).unwrap();
    }
    std::fs::write(
        tmp.join(".kotowari/config.yaml"),
        format!("tests:\n  files:\n    - \"tests/**/*.rs\"\n{surface}"),
    )
    .unwrap();
    write(
        tmp,
        "docs/decision/records/r.md",
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    );
}

/// ファイルを書く（親のディレクトリは作る）
fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}

/// 設定を読んで面を取り出す。(面の (種類, 名前, パス, 行) の並び, 指摘の (種類, パス) の並び)
fn extract(tmp: &Path) -> (Vec<(String, String, String, usize)>, Vec<(String, String)>) {
    let config =
        Config::parse(&std::fs::read_to_string(tmp.join(".kotowari/config.yaml")).unwrap())
            .unwrap();
    let rules = config
        .surface
        .rules
        .iter()
        .map(|path| {
            kotowari_core::SourceText::new(
                path.clone(),
                std::fs::read_to_string(tmp.join(path)).unwrap(),
            )
            .unwrap()
        })
        .collect();
    let analyzer = kotowari_source_analysis::Analyzer::new(config.clone(), vec![], rules).unwrap();
    let mut builder = globset::GlobSetBuilder::new();
    for pattern in &config.surface.files {
        builder.add(globset::Glob::new(pattern).unwrap());
    }
    let globs = builder.build().unwrap();
    let mut paths: Vec<_> = walkdir::WalkDir::new(tmp)
        .into_iter()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect();
    paths.sort();
    let mut surfaces = vec![];
    let mut findings = vec![];
    for path in paths {
        let relative = path
            .strip_prefix(tmp)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        if !globs.is_match(&relative) || !analyzer.supports_surfaces(&relative) {
            continue;
        }
        let source =
            kotowari_core::SourceText::new(relative, std::fs::read_to_string(path).unwrap())
                .unwrap();
        let result = analyzer.surfaces(source).unwrap();
        surfaces.extend(
            result
                .surfaces
                .into_iter()
                .map(|surface| (surface.kind, surface.name, surface.path, surface.line)),
        );
        findings.extend(
            result
                .findings
                .into_iter()
                .map(|finding| (finding.kind().to_string(), finding.path().to_owned())),
        );
    }
    (surfaces, findings)
}

fn surface(kind: &str, name: &str, path: &str, line: usize) -> (String, String, String, usize) {
    (kind.to_string(), name.to_string(), path.to_string(), line)
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_a_matched_node_is_a_surface_named_by_name_with_one_quote_pair_removed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = \"--format\";\n    let b = \"x\";\n    let c = r#\"\"--raw\"\"#;\n}\n",
    );
    let (surfaces, findings) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![surface("flag", "--format", "src/cli.rs", 2)],
        "only the literal starting with \"--\" matches"
    );
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_only_one_pair_of_quotes_is_removed_and_other_forms_stay() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    // 識別子を名前にする規則と、raw の文字列のリテラルを名前にする規則
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: ident\nlanguage: rust\nrule:\n  kind: identifier\n  inside:\n    kind: let_declaration\n  pattern: $NAME\n---\nid: raw\nlanguage: rust\nrule:\n  kind: raw_string_literal\n  pattern: $NAME\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = r#\"'q'\"#;\n}\n",
    );
    let (surfaces, _) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![
            surface("ident", "a", "src/cli.rs", 2),
            surface("raw", "r#\"'q'\"#", "src/cli.rs", 2),
        ]
    );
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_a_match_without_name_is_not_a_surface() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: nameless\nlanguage: rust\nrule:\n  kind: string_literal\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = \"--format\";\n}\n",
    );
    let (surfaces, _) = extract(tmp.path());
    assert!(surfaces.is_empty(), "{surfaces:?}");
}

// @kotowari[REQ-core-223]
#[test]
fn req_223_language_is_matched_without_case_and_accepts_aliases() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(
        tmp.path(),
        "rules/surface.yml",
        "id: flag\nlanguage: Rust\nrule:\n  kind: string_literal\n  pattern: $NAME\n---\nid: route\nlanguage: ts\nrule:\n  kind: string\n  pattern: $NAME\n",
    );
    write(
        tmp.path(),
        "src/cli.rs",
        "fn f() {\n    let a = \"--format\";\n}\n",
    );
    write(tmp.path(), "src/app.ts", "const r = '/users';\n");
    let (surfaces, _) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![
            surface("route", "/users", "src/app.ts", 1),
            surface("flag", "--format", "src/cli.rs", 2),
        ]
    );
}

// @kotowari[EX-core-418, REQ-core-223]
#[test]
fn ex_core_418_a_rule_is_not_applied_to_a_file_of_another_language() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "src/a.ts", "const a = \"--format\";\n");
    let (surfaces, findings) = extract(tmp.path());
    assert!(surfaces.is_empty(), "{surfaces:?}");
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[REQ-core-224]
#[test]
fn req_224_rule_files_and_ignores_are_applied_and_severity_off_still_applies() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(
        tmp.path(),
        "rules/surface.yml",
        &format!(
            "{FLAG_RULE}severity: off\nfiles:\n  - \"src/cli/**\"\nignores:\n  - \"src/cli/skip.rs\"\n"
        ),
    );
    let body = "fn f() {\n    let a = \"--format\";\n}\n";
    write(tmp.path(), "src/cli/main.rs", body);
    write(tmp.path(), "src/cli/skip.rs", body);
    write(tmp.path(), "src/other.rs", body);
    let (surfaces, _) = extract(tmp.path());
    assert_eq!(
        surfaces,
        vec![surface("flag", "--format", "src/cli/main.rs", 2)]
    );
}

// @kotowari[EX-core-426, REQ-core-236]
#[test]
fn ex_core_426_a_file_not_in_a_rule_language_is_not_parsed() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), SURFACE_SRC);
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(tmp.path(), "src/notes.py", "def (:\n");
    let (_, findings) = extract(tmp.path());
    assert!(findings.is_empty(), "{findings:?}");
}

// @kotowari[EX-core-427, REQ-core-236]
#[test]
fn ex_core_427_a_syntax_error_in_a_rule_language_file_is_unparsable() {
    let tmp = TempDir::new().unwrap();
    make_project(
        tmp.path(),
        "surface:\n  files:\n    - \"src/**/*.rs\"\n  rules:\n    - \"rules/surface.yml\"\n",
    );
    write(tmp.path(), "rules/surface.yml", FLAG_RULE);
    write(
        tmp.path(),
        "src/bad.rs",
        "fn f( {\n    let a = \"--format\";\n",
    );
    let (surfaces, findings) = extract(tmp.path());
    assert!(surfaces.is_empty(), "{surfaces:?}");
    assert_eq!(
        findings,
        vec![("unparsable_file".to_string(), "src/bad.rs".to_string())]
    );
}
