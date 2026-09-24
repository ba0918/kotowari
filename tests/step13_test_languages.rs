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
        "it(\"double\", () => {});\nit(`back`, () => {});\nit(caseName, () => {});\nit('\"inner\"', () => {});\nit('', () => {});\nit('a' + b, () => {});\nit(a + 'b', () => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(
        unmarked(&v),
        vec![
            "double",
            "back",
            "caseName",
            "\"inner\"",
            "",
            "'a' + b",
            "a + 'b'"
        ],
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

// @kotowari[REQ-core-183, TBL-core-032]
#[test]
fn tbl_032_test_whose_result_is_called_again_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "it('curried', () => {})('other')\nit.only('chained', () => {})('other')\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["curried", "chained"], "{v}");
}

// @kotowari[REQ-core-183, TBL-core-032]
#[test]
fn tbl_032_dotted_form_with_a_unicode_name_or_a_comment_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "it.検証('unicode', () => {});\nit /* note */ .only('withComment', () => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["unicode", "withComment"], "{v}");
}

// @kotowari[REQ-core-183, TBL-core-032, EX-core-331]
#[test]
fn ex_core_331_tagged_template_alone_is_not_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "test.each`a`('tagged %s', () => {});\ntest.each`foo`;\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["tagged %s"], "{v}");
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

// @kotowari[REQ-core-075, TBL-core-035, EX-core-327]
#[test]
fn ex_core_327_comment_after_a_full_width_space_joins_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "\u{3000}// @kotowari[REQ-001]\n\u{a0}// @kotowari[REQ-002]\nit('x', () => {});\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    assert_eq!(listed_tests(tmp.path(), "REQ-001")[0]["name"], "x");
    assert_eq!(listed_tests(tmp.path(), "REQ-002")[0]["name"], "x");
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

// @kotowari[REQ-core-184, TBL-core-033, EX-core-329]
#[test]
fn ex_core_329_method_of_a_class_inside_a_function_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "def test_outer():\n    class C:\n        def test_in_class(self):\n            pass\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["test_outer", "test_in_class"], "{v}");
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

// @kotowari[REQ-core-075, TBL-core-035, EX-core-323]
#[test]
fn ex_core_323_blank_line_inside_a_multi_line_decorator_does_not_cut_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "# @kotowari[REQ-001]\n@pytest.mark.parametrize(\n    'a',\n\n    [1],\n)\ndef test_x(a):\n    pass\n",
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

// --- "tests.rules" で足すルール ---

/// "tests.files" と "tests.rules" を書いた設定に置き換える
fn configure(tmp: &Path, test_globs: &[&str], rules: &[&str]) {
    let mut config = String::from("tests:\n  files:\n");
    for glob in test_globs {
        config.push_str(&format!("    - \"{glob}\"\n"));
    }
    config.push_str("  rules:\n");
    for rule in rules {
        config.push_str(&format!("    - \"{rule}\"\n"));
    }
    fs::write(tmp.join(".kotowari/config.yaml"), config).unwrap();
}

const BENCH_RULE: &str = "id: bench\nlanguage: typescript\nrule:\n  pattern: bench($NAME, $$$)\n";

/// 終了コードと標準エラーの1行目
fn stop(tmp: &Path) -> (Option<i32>, String) {
    let output = cmd().arg("check").current_dir(tmp).output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    (
        output.status.code(),
        stderr.lines().next().unwrap_or("").to_string(),
    )
}

// @kotowari[REQ-core-186, EX-core-311]
#[test]
fn ex_core_311_added_rule_counts_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(tmp.path(), "rules/bench.yml", BENCH_RULE);
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["fast"], "{v}");
}

// @kotowari[REQ-core-186]
#[test]
fn req_186_rules_separated_by_dashes_all_apply_and_give_a_language_a_query() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*"], &["rules/many.yml"]);
    write(
        tmp.path(),
        "rules/many.yml",
        "id: go-test\nlanguage: go\nrule:\n  kind: function_declaration\n  has:\n    field: name\n    regex: ^Test\n    pattern: $NAME\n---\nid: bench\nlanguage: typescript\nrule:\n  pattern: bench($NAME, $$$)\n",
    );
    write(
        tmp.path(),
        "tests/a_test.go",
        "package a\n\nfunc TestA(t *testing.T) {}\n",
    );
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["fast", "TestA"], "{v}");
    assert_eq!(v["tests"]["go"]["query"], true, "{v}");
}

// @kotowari[REQ-core-121, REQ-core-181, EX-core-312]
#[test]
fn ex_core_312_rule_hitting_the_bundled_node_counts_once() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/it.yml"]);
    write(
        tmp.path(),
        "rules/it.yml",
        "id: it\nlanguage: typescript\nrule:\n  pattern: it($NAME, $$$)\n",
    );
    write(tmp.path(), "tests/a.test.ts", "it('x', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[REQ-core-187, EX-core-313]
#[test]
fn ex_core_313_rule_is_not_applied_outside_its_files() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(
        tmp.path(),
        "rules/bench.yml",
        &format!("{BENCH_RULE}files:\n  - \"**/*.spec.ts\"\n"),
    );
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    write(tmp.path(), "tests/b.spec.ts", "bench('spec', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["spec"], "{v}");
}

// @kotowari[REQ-core-187]
#[test]
fn req_187_rule_is_not_applied_to_its_ignores() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(
        tmp.path(),
        "rules/bench.yml",
        &format!("{BENCH_RULE}ignores:\n  - \"tests/skip/**\"\n"),
    );
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    write(
        tmp.path(),
        "tests/skip/b.test.ts",
        "bench('skipped', () => {})\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["fast"], "{v}");
}

// @kotowari[REQ-core-188, EX-core-314]
#[test]
fn ex_core_314_rule_with_severity_off_is_applied() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(
        tmp.path(),
        "rules/bench.yml",
        &format!("{BENCH_RULE}severity: off\n"),
    );
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["fast"], "{v}");
}

// @kotowari[REQ-core-188]
#[test]
fn req_188_fix_message_note_and_metadata_do_not_change_the_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(
        tmp.path(),
        "rules/bench.yml",
        &format!(
            "{BENCH_RULE}fix: other($NAME)\nmessage: a bench\nnote: see the docs\nseverity: error\nmetadata:\n  owner: me\n"
        ),
    );
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["fast"], "{v}");
}

// @kotowari[REQ-core-189, EX-core-315]
#[test]
fn ex_core_315_missing_rule_file_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/missing.yml"]);
    let (code, first) = stop(tmp.path());
    assert_eq!(code, Some(2));
    assert!(first.starts_with("config error"), "{first}");
}

// @kotowari[REQ-core-189, EX-core-316]
#[test]
fn ex_core_316_rule_of_an_unknown_language_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/cobol.yml"]);
    write(
        tmp.path(),
        "rules/cobol.yml",
        "id: cobol\nlanguage: cobol\nrule:\n  pattern: foo\n",
    );
    let (code, first) = stop(tmp.path());
    assert_eq!(code, Some(2));
    assert!(first.starts_with("config error"), "{first}");
}

// @kotowari[REQ-core-189, TBL-core-020, EX-core-378]
#[test]
fn ex_core_378_a_missing_rule_file_stop_points_at_the_rule_file() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/missing.yml"]);
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.starts_with("config error: "), "{stderr}");
    assert!(stderr.contains("rules/missing.yml"), "{stderr}");
    assert!(!stderr.contains(".kotowari/config.yaml"), "{stderr}");
}

// @kotowari[REQ-core-189, EX-core-379]
#[test]
fn ex_core_379_an_unknown_language_stop_names_the_language_in_one_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["r.yml"]);
    write(
        tmp.path(),
        "r.yml",
        "id: cobol\nlanguage: cobol\nrule:\n  pattern: foo\n",
    );
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("r.yml"), "{stderr}");
    assert!(stderr.contains("unknown language: cobol"), "{stderr}");
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
}

// @kotowari[REQ-core-189, EX-core-321]
#[test]
fn ex_core_321_same_rule_file_twice_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(
        tmp.path(),
        &["tests/**/*.ts"],
        &["rules/bench.yml", "rules/bench.yml"],
    );
    write(tmp.path(), "rules/bench.yml", BENCH_RULE);
    let (code, first) = stop(tmp.path());
    assert_eq!(code, Some(2));
    assert!(first.starts_with("config error"), "{first}");
}

// @kotowari[REQ-core-189]
#[test]
fn req_189_unreadable_or_malformed_rule_files_stop() {
    let cases: [(&str, &[u8]); 4] = [
        ("a directory", b""),
        ("not UTF-8", b"\xff\xfe"),
        ("not YAML", b"id: [unclosed\n"),
        ("not an ast-grep rule", b"id: x\nlanguage: typescript\n"),
    ];
    for (case, content) in cases {
        let tmp = TempDir::new().unwrap();
        make_project(tmp.path(), &[]);
        configure(tmp.path(), &["tests/**/*.ts"], &["rules/r.yml"]);
        if case == "a directory" {
            fs::create_dir_all(tmp.path().join("rules/r.yml")).unwrap();
        } else {
            fs::create_dir_all(tmp.path().join("rules")).unwrap();
            fs::write(tmp.path().join("rules/r.yml"), content).unwrap();
        }
        let (code, first) = stop(tmp.path());
        assert_eq!(code, Some(2), "{case}");
        assert!(first.starts_with("config error"), "{case}: {first}");
    }
}

// @kotowari[REQ-core-189, EX-core-322]
#[test]
fn ex_core_322_language_alias_is_accepted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(
        tmp.path(),
        "rules/bench.yml",
        "id: bench\nlanguage: ts\nrule:\n  pattern: bench($NAME, $$$)\n",
    );
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    let output = cmd().arg("check").current_dir(tmp.path()).output().unwrap();
    assert_eq!(output.status.code(), Some(1));
    let v: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(unmarked(&v), vec!["fast"], "{v}");
}

// @kotowari[REQ-core-189]
#[test]
fn req_189_language_is_matched_without_case() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.py"], &["rules/r.yml"]);
    write(
        tmp.path(),
        "rules/r.yml",
        "id: check\nlanguage: PY\nrule:\n  kind: function_definition\n  has:\n    field: name\n    regex: ^check_\n    pattern: $NAME\n",
    );
    write(tmp.path(), "tests/test_a.py", "def check_x():\n    pass\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["check_x"], "{v}");
}

// @kotowari[REQ-core-086, EX-core-310]
#[test]
fn ex_core_310_test_without_a_name_uses_its_first_line() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(
        tmp.path(),
        "rules/bench.yml",
        "id: bench\nlanguage: typescript\nrule:\n  pattern: bench($$$)\n",
    );
    write(
        tmp.path(),
        "tests/a.test.ts",
        "describe('d', () => {\n  bench(caseName, () => {\n  });\n});\n",
    );
    let v = check(tmp.path());
    let found = findings(&v, "test_without_id");
    assert_eq!(found.len(), 1, "{v}");
    assert_eq!(found[0]["detail"], "bench(caseName, () => {");
    assert_eq!(found[0]["line"], 2);
}

// @kotowari[REQ-core-180, TBL-core-026]
#[test]
fn req_180_test_without_a_name_is_listed_with_a_null_name() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(
        tmp.path(),
        "rules/bench.yml",
        "id: bench\nlanguage: typescript\nrule:\n  pattern: bench($$$)\n",
    );
    write(
        tmp.path(),
        "tests/a.test.ts",
        "// @kotowari[REQ-001]\nbench(caseName, () => {});\n",
    );
    let tests = listed_tests(tmp.path(), "REQ-001");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert!(tests[0]["name"].is_null(), "{tests:?}");
}

// @kotowari[REQ-core-181, EX-core-328]
#[test]
fn ex_core_328_name_comes_from_the_first_rule_that_captures_it() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(
        tmp.path(),
        &["tests/**/*.ts"],
        &["rules/unnamed.yml", "rules/named.yml"],
    );
    write(
        tmp.path(),
        "rules/unnamed.yml",
        "id: unnamed\nlanguage: typescript\nrule:\n  pattern: bench($$$)\n",
    );
    write(tmp.path(), "rules/named.yml", BENCH_RULE);
    write(tmp.path(), "tests/a.test.ts", "bench('chosen', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["chosen"], "{v}");
}

// @kotowari[REQ-core-181, REQ-core-187]
#[test]
fn req_181_rule_with_files_keeps_its_place_in_the_order() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(
        tmp.path(),
        &["tests/**/*.ts"],
        &["rules/first.yml", "rules/second.yml"],
    );
    write(
        tmp.path(),
        "rules/first.yml",
        "id: first\nlanguage: typescript\nfiles:\n  - \"**/*.ts\"\nrule:\n  pattern: bench($NAME, $$$)\n",
    );
    write(
        tmp.path(),
        "rules/second.yml",
        "id: second\nlanguage: typescript\nrule:\n  pattern: bench($A, $NAME)\n",
    );
    write(tmp.path(), "tests/a.test.ts", "bench('first', 'second')\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["first"], "{v}");
}
