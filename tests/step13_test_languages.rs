//! Rust 以外の言語の`問い合わせ`（TS/JS、Python、PHP）と、"tests.rules" で足すルールの検査

use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// "tests.files" に glob を並べ、検証が "unit" の REQ-001 と REQ-002 を持つプロジェクトを作る
fn make_project(tmp: &Path, test_globs: &[&str]) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/ir")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/records")).unwrap();
    fs::create_dir_all(tmp.join("docs/decision/adr")).unwrap();
    let mut config = String::from("tests:\n  files:\n");
    for glob in test_globs {
        config.push_str(&format!("    - \"{glob}\"\n"));
    }
    fs::write(tmp.join(".kotowari/config.yaml"), config).unwrap();
    fs::write(
        tmp.join("docs/decision/records/records.md"),
        "# Records\n\n## Agreements\n\n- A1 Agreement\n",
    )
    .unwrap();
    let requirement = |id: &str| {
        format!(
            "### {id}: Req\n\n- kind: ubiquitous\n- source: docs/decision/records/records.md#A1\n- verification: unit\n\nStatement.\n\n"
        )
    };
    fs::write(
        tmp.join("docs/ir/a.md"),
        format!(
            "# Title\n\nScope.\n\n## Requirements\n\n{}{}",
            requirement("REQ-001"),
            requirement("REQ-002")
        ),
    )
    .unwrap();
}

fn write(tmp: &Path, rel: &str, content: &str) {
    let path = tmp.join(rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn check(tmp: &Path) -> serde_json::Value {
    let output = cmd().arg("check").current_dir(tmp).output().unwrap();
    serde_json::from_slice(&output.stdout).expect("valid JSON")
}

fn findings(v: &serde_json::Value, kind: &str) -> Vec<serde_json::Value> {
    v["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|f| f["kind"] == kind)
        .cloned()
        .collect()
}

/// test_without_id の detail の並び
fn unmarked(v: &serde_json::Value) -> Vec<String> {
    findings(v, "test_without_id")
        .iter()
        .map(|f| f["detail"].as_str().unwrap().to_string())
        .collect()
}

/// "kotowari list" の、その ID の "tests"
fn listed_tests(tmp: &Path, id: &str) -> Vec<serde_json::Value> {
    let output = cmd().arg("list").current_dir(tmp).output().unwrap();
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    v["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["id"] == id)
        .map(|item| item["tests"].as_array().unwrap().clone())
        .unwrap_or_default()
}

// --- TypeScript、Tsx、JavaScript ---

// @kotowari[REQ-core-083, EX-core-294]
#[test]
fn ex_core_294_syntax_error_in_a_file_of_a_language_with_a_query() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(tmp.path(), "tests/a.test.ts", "it('x', () => {");
    let v = check(tmp.path());
    let unparsable = findings(&v, "unparsable_file");
    assert_eq!(unparsable.len(), 1, "{v}");
    assert_eq!(unparsable[0]["detail"], "tests/a.test.ts");
}

// @kotowari[REQ-core-180, EX-core-296]
#[test]
fn ex_core_296_string_name_loses_its_quotes() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "describe('d', () => {\n  // @kotowari[REQ-001]\n  // @kotowari[REQ-002]\n  it('does x', () => {});\n});\n",
    );
    let tests = listed_tests(tmp.path(), "REQ-002");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["line"], 3);
    assert_eq!(
        tests[0]["name"], "does x",
        "no describe name is prefixed: {tests:?}"
    );
}

// @kotowari[REQ-core-180, TBL-core-032]
#[test]
fn req_180_only_one_pair_of_the_same_quotes_is_stripped() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "it(\"double\", () => {});\nit(`back`, () => {});\nit(caseName, () => {});\nit('\"inner\"', () => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(
        unmarked(&v),
        vec!["double", "back", "caseName", "\"inner\""],
        "{v}"
    );
}

// @kotowari[REQ-core-181, EX-core-297]
#[test]
fn ex_core_297_test_inside_a_test_is_counted_apart() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "it('outer', () => {\n  it('inner', () => {});\n});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["outer", "inner"], "{v}");
}

// @kotowari[REQ-core-183, TBL-core-032, EX-core-298]
#[test]
fn ex_core_298_skipped_test_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(tmp.path(), "tests/a.test.ts", "it.skip('x', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[REQ-core-183, TBL-core-032, EX-core-299]
#[test]
fn ex_core_299_each_table_test_is_one_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "it.each([1])('each %i', (n) => {})\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["each %i"], "{v}");
}

// @kotowari[REQ-core-183, TBL-core-032, EX-core-317]
#[test]
fn ex_core_317_chained_each_table_test_is_one_test() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "it.only.each([1])('each %i', (n) => {})\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["each %i"], "{v}");
}

// @kotowari[REQ-core-183, TBL-core-032, EX-core-300]
#[test]
fn ex_core_300_describe_and_other_callers_are_not_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "describe('d', () => { regex.test(s); })\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
}

// @kotowari[REQ-core-183, TBL-core-032]
#[test]
fn tbl_032_dotted_forms_are_counted_with_the_first_argument() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "test('t', () => {});\nit.only('o', () => {});\nit.todo('d');\ntest.concurrent('c', async () => {});\ntest.concurrent.each([1])('ce %i', (n) => {});\ndescribe.each([1])('de %i', (n) => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["t", "o", "d", "c", "ce %i"], "{v}");
}

// @kotowari[REQ-core-182, REQ-core-183, EX-core-301]
#[test]
fn ex_core_301_test_in_a_tsx_file_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.tsx"]);
    write(tmp.path(), "tests/a.test.tsx", "it('x', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[REQ-core-182, REQ-core-183]
#[test]
fn req_183_javascript_files_are_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*"]);
    for ext in ["js", "jsx", "mjs", "cjs"] {
        write(
            tmp.path(),
            &format!("tests/a.{ext}"),
            &format!("it('{ext}', () => {{}})\n"),
        );
    }
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["cjs", "js", "jsx", "mjs"], "{v}");
}

// @kotowari[REQ-core-075, EX-core-309]
#[test]
fn ex_core_309_code_line_between_breaks_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "// @kotowari[REQ-001]\nconst n = 1;\nit('x', () => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[REQ-core-075, EX-core-319]
#[test]
fn ex_core_319_mark_before_a_nested_test_binds_to_it() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "// @kotowari[REQ-001]\nit('outer', () => {\n  // @kotowari[REQ-002]\n  it('inner', () => {});\n});\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let outer = listed_tests(tmp.path(), "REQ-001");
    assert_eq!(outer.len(), 1, "{outer:?}");
    assert_eq!(outer[0]["name"], "outer");
    let inner = listed_tests(tmp.path(), "REQ-002");
    assert_eq!(inner.len(), 1, "{inner:?}");
    assert_eq!(inner[0]["name"], "inner");
}

// @kotowari[REQ-core-075, EX-core-320]
#[test]
fn ex_core_320_comment_after_code_on_the_same_line_breaks_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "// @kotowari[REQ-001]\nsetup(); // prepare\nit('x', () => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[TBL-core-021, TBL-core-031, REQ-core-182]
#[test]
fn tbl_021_extensions_of_the_bundled_languages_have_a_query() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*"]);
    let with_query = [
        "rs", "ts", "mts", "cts", "tsx", "js", "jsx", "mjs", "cjs", "py", "py3", "pyi", "bzl",
        "bazel", "php",
    ];
    for ext in with_query {
        write(tmp.path(), &format!("tests/a.{ext}"), "\n");
    }
    write(tmp.path(), "tests/a.go", "\n");
    let v = check(tmp.path());
    for ext in with_query {
        assert_eq!(v["tests"][ext]["query"], true, "{ext}: {v}");
    }
    assert_eq!(v["tests"]["go"]["query"], false, "{v}");
}

// --- Python ---

// @kotowari[REQ-core-184, TBL-core-033, EX-core-302]
#[test]
fn ex_core_302_nested_function_is_not_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "def test_foo():\n    def test_inner():\n        pass\n\n\nclass TestBar:\n    def test_baz(self):\n        pass\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["test_foo", "test_baz"], "{v}");
}

// @kotowari[REQ-core-184, TBL-core-033, TBL-core-019]
#[test]
fn tbl_033_decorated_test_is_counted_at_its_def_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "import pytest\n\n\n@pytest.fixture\ndef helper():\n    pass\n\n\n@pytest.mark.slow\nasync def test_x():\n    pass\n\n\nclass TestBar:\n    @staticmethod\n    def test_y():\n        pass\n",
    );
    let v = check(tmp.path());
    let found: Vec<_> = findings(&v, "test_without_id")
        .iter()
        .map(|f| {
            (
                f["detail"].as_str().unwrap().to_string(),
                f["line"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        found,
        vec![("test_x".to_string(), 10), ("test_y".to_string(), 16)],
        "{v}"
    );
}

// @kotowari[REQ-core-075, TBL-core-035, EX-core-307]
#[test]
fn ex_core_307_mark_binds_across_a_decorator() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "# @kotowari[REQ-001]\n@pytest.mark.parametrize('a', [1])\ndef test_x(a):\n    pass\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let tests = listed_tests(tmp.path(), "REQ-001");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["name"], "test_x");
}

// @kotowari[REQ-core-075, TBL-core-035, EX-core-308]
#[test]
fn ex_core_308_mark_before_a_method_binds() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "class TestBar:\n    # @kotowari[REQ-001]\n    def test_baz(self):\n        pass\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let tests = listed_tests(tmp.path(), "REQ-001");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["name"], "test_baz");
}

// @kotowari[REQ-core-075, TBL-core-035, EX-core-318]
#[test]
fn ex_core_318_mark_binds_across_a_multi_line_decorator_and_a_comment() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "# @kotowari[REQ-001]\n@pytest.mark.parametrize(\n    'a', [1]\n)\n# note\ndef test_x(a):\n    pass\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let tests = listed_tests(tmp.path(), "REQ-001");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["name"], "test_x");
}

// --- Php ---

// @kotowari[REQ-core-185, TBL-core-034, TBL-core-019]
#[test]
fn tbl_034_test_methods_and_attributes() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.php"]);
    write(
        tmp.path(),
        "tests/FooTest.php",
        "<?php\nclass FooTest\n{\n    public function testAdds()\n    {\n    }\n\n    #[Test]\n    public function other()\n    {\n    }\n\n    #[DataProvider('p')]\n    public function helper()\n    {\n    }\n}\n",
    );
    let v = check(tmp.path());
    let found: Vec<_> = findings(&v, "test_without_id")
        .iter()
        .map(|f| {
            (
                f["detail"].as_str().unwrap().to_string(),
                f["line"].as_u64().unwrap(),
            )
        })
        .collect();
    // "#[Test]" はメソッドの節の中にあるので、行は "#[Test]" の行
    assert_eq!(
        found,
        vec![("testAdds".to_string(), 4), ("other".to_string(), 8)],
        "{v}"
    );
}

// @kotowari[REQ-core-185, TBL-core-034, EX-core-303]
#[test]
fn ex_core_303_method_with_a_qualified_test_attribute_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.php"]);
    write(
        tmp.path(),
        "tests/FooTest.php",
        "<?php\nclass FooTest\n{\n    #[\\PHPUnit\\Framework\\Attributes\\Test]\n    public function other()\n    {\n    }\n}\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["other"], "{v}");
}

// @kotowari[REQ-core-185, TBL-core-034, EX-core-304]
#[test]
fn ex_core_304_method_with_a_test_docblock_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.php"]);
    write(
        tmp.path(),
        "tests/FooTest.php",
        "<?php\nclass FooTest\n{\n    /** @test */\n    public function itWorks()\n    {\n    }\n\n    /* @test */\n    public function notADocblock()\n    {\n    }\n\n    /** @testdox adds */\n    public function notATestTag()\n    {\n    }\n}\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["itWorks"], "{v}");
}

// @kotowari[REQ-core-185, TBL-core-034, EX-core-305]
#[test]
fn ex_core_305_pest_test_call_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.php"]);
    write(
        tmp.path(),
        "tests/FooTest.php",
        "<?php\ntest('adds', function () {});\nit(\"works\", function () {});\ndescribe('d', function () {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["adds", "works"], "{v}");
}

// @kotowari[REQ-core-075, TBL-core-034]
#[test]
fn tbl_034_mark_before_a_docblock_test_binds() {
    // "@test" の docblock はコメントの行なので、その上の印も同じ塊に入る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.php"]);
    write(
        tmp.path(),
        "tests/FooTest.php",
        "<?php\nclass FooTest\n{\n    // @kotowari[REQ-001]\n    /**\n     * @test\n     */\n    public function itWorks()\n    {\n    }\n}\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    assert_eq!(listed_tests(tmp.path(), "REQ-001")[0]["name"], "itWorks");
}
