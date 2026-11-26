//! Rust 以外の言語の`問い合わせ`（TS/JS、Python、PHP）と、"tests.rules" で足すルールの検査
#![expect(
    clippy::unwrap_used,
    reason = "テストと例の補助関数は、準備の失敗をそのまま panic で知らせる"
)]

use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn cmd() -> Command {
    Command::cargo_bin("kotowari").unwrap()
}

/// "tests.files" に glob を並べ、確認することの行 C-t-a-1 と C-t-a-2 を持つ決定の文書を置いた
/// プロジェクトを作る
fn make_project(tmp: &Path, test_globs: &[&str]) {
    fs::create_dir_all(tmp.join(".kotowari")).unwrap();
    fs::create_dir_all(tmp.join("docs/decisions")).unwrap();
    write(
        tmp,
        "docs/decisions/t/a.md",
        "# Title\n\nScope.\n\n## Decision\n\nStatus: implemented\n\n### Problem\n\nTrouble.\n\n\
### Decision\n\nDo this.\n\n### Confirmations\n\n- C-t-a-1 One\n- C-t-a-2 Two\n",
    );
    let mut config = String::from("decisions:\n  documents: docs/decisions\ntests:\n  files:\n");
    for glob in test_globs {
        config.push_str(&format!("    - \"{glob}\"\n"));
    }
    fs::write(tmp.join(".kotowari/config.yaml"), config).unwrap();
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

// @kotowari[C-core-discover-8]
#[test]
fn syntax_error_in_a_file_of_a_language_with_a_query() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(tmp.path(), "tests/a.test.ts", "it('x', () => {");
    let v = check(tmp.path());
    let unparsable = findings(&v, "unparsable_file");
    assert_eq!(unparsable.len(), 1, "{v}");
    assert_eq!(unparsable[0]["detail"], "tests/a.test.ts");
}

// @kotowari[C-core-discover-9, C-core-discover-10]
#[test]
fn string_name_loses_its_quotes() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "describe('d', () => {\n  // @kotowari[C-t-a-1]\n  // @kotowari[C-t-a-2]\n  it('does x', () => {});\n});\n",
    );
    let tests = listed_tests(tmp.path(), "C-t-a-2");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["line"], 3);
    assert_eq!(
        tests[0]["name"], "does x",
        "no describe name is prefixed: {tests:?}"
    );
}

// @kotowari[C-core-discover-9, C-core-queries-8]
#[test]
fn only_one_pair_of_the_same_quotes_is_stripped() {
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

// @kotowari[C-core-discover-13]
#[test]
fn test_inside_a_test_is_counted_apart() {
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

// @kotowari[C-core-queries-8]
#[test]
fn skipped_test_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(tmp.path(), "tests/a.test.ts", "it.skip('x', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[C-core-queries-9]
#[test]
fn each_table_test_is_one_test() {
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

// @kotowari[C-core-queries-9]
#[test]
fn chained_each_table_test_is_one_test() {
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

// @kotowari[C-core-queries-8]
#[test]
fn test_whose_result_is_called_again_is_counted() {
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

// @kotowari[C-core-queries-8]
#[test]
fn dotted_form_with_a_unicode_name_or_a_comment_is_counted() {
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

// @kotowari[C-core-queries-9]
#[test]
fn tagged_template_alone_is_not_counted() {
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

// @kotowari[C-core-queries-10]
#[test]
fn describe_and_other_callers_are_not_counted() {
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

// @kotowari[C-core-queries-8, C-core-queries-9, C-core-queries-10]
#[test]
fn dotted_forms_are_counted_with_the_first_argument() {
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

// @kotowari[C-core-queries-2]
#[test]
fn test_in_a_tsx_file_is_counted() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.tsx"]);
    write(tmp.path(), "tests/a.test.tsx", "it('x', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[C-core-queries-3]
#[test]
fn javascript_files_are_counted() {
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

// @kotowari[C-core-markers-16]
#[test]
fn code_line_between_breaks_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "// @kotowari[C-t-a-1]\nconst n = 1;\nit('x', () => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[C-core-markers-10]
#[test]
fn mark_before_a_nested_test_binds_to_it() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "// @kotowari[C-t-a-1]\nit('outer', () => {\n  // @kotowari[C-t-a-2]\n  it('inner', () => {});\n});\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let outer = listed_tests(tmp.path(), "C-t-a-1");
    assert_eq!(outer.len(), 1, "{outer:?}");
    assert_eq!(outer[0]["name"], "outer");
    let inner = listed_tests(tmp.path(), "C-t-a-2");
    assert_eq!(inner.len(), 1, "{inner:?}");
    assert_eq!(inner[0]["name"], "inner");
}

// @kotowari[C-core-markers-17]
#[test]
fn comment_after_a_full_width_space_joins_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "\u{3000}// @kotowari[C-t-a-1]\n\u{a0}// @kotowari[C-t-a-2]\nit('x', () => {});\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    assert_eq!(listed_tests(tmp.path(), "C-t-a-1")[0]["name"], "x");
    assert_eq!(listed_tests(tmp.path(), "C-t-a-2")[0]["name"], "x");
}

// @kotowari[C-core-markers-16]
#[test]
fn comment_after_code_on_the_same_line_breaks_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.ts"]);
    write(
        tmp.path(),
        "tests/a.test.ts",
        "// @kotowari[C-t-a-1]\nsetup(); // prepare\nit('x', () => {});\n",
    );
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["x"], "{v}");
}

// @kotowari[C-core-output-12, C-core-discover-7, C-core-queries-1]
#[test]
fn extensions_of_the_bundled_languages_have_a_query() {
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

// @kotowari[C-core-queries-11]
#[test]
fn nested_function_is_not_counted() {
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

// @kotowari[C-core-forder-6, C-core-queries-12]
#[test]
fn decorated_test_is_counted_at_its_def_line() {
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

// @kotowari[C-core-queries-11]
#[test]
fn method_of_a_class_inside_a_function_is_counted() {
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

// @kotowari[C-core-markers-14]
#[test]
fn mark_binds_across_a_decorator() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "# @kotowari[C-t-a-1]\n@pytest.mark.parametrize('a', [1])\ndef test_x(a):\n    pass\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let tests = listed_tests(tmp.path(), "C-t-a-1");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["name"], "test_x");
}

// @kotowari[C-core-markers-13]
#[test]
fn mark_before_a_method_binds() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "class TestBar:\n    # @kotowari[C-t-a-1]\n    def test_baz(self):\n        pass\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let tests = listed_tests(tmp.path(), "C-t-a-1");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["name"], "test_baz");
}

// @kotowari[C-core-markers-14]
#[test]
fn mark_binds_across_a_multi_line_decorator_and_a_comment() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "# @kotowari[C-t-a-1]\n@pytest.mark.parametrize(\n    'a', [1]\n)\n# note\ndef test_x(a):\n    pass\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let tests = listed_tests(tmp.path(), "C-t-a-1");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["name"], "test_x");
}

// @kotowari[C-core-markers-15]
#[test]
fn blank_line_inside_a_multi_line_decorator_does_not_cut_the_block() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.py"]);
    write(
        tmp.path(),
        "tests/test_a.py",
        "# @kotowari[C-t-a-1]\n@pytest.mark.parametrize(\n    'a',\n\n    [1],\n)\ndef test_x(a):\n    pass\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    let tests = listed_tests(tmp.path(), "C-t-a-1");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert_eq!(tests[0]["name"], "test_x");
}

// --- Php ---

// @kotowari[C-core-forder-6, C-core-queries-13]
#[test]
fn test_methods_and_attributes() {
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

// @kotowari[C-core-queries-13]
#[test]
fn method_with_a_qualified_test_attribute_is_counted() {
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

// @kotowari[C-core-queries-14]
#[test]
fn method_with_a_test_docblock_is_counted() {
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

// @kotowari[C-core-queries-15]
#[test]
fn pest_test_call_is_counted() {
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

// @kotowari[C-core-markers-13, C-core-queries-14]
#[test]
fn mark_before_a_docblock_test_binds() {
    // "@test" の docblock はコメントの行なので、その上の印も同じ塊に入る
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &["tests/**/*.php"]);
    write(
        tmp.path(),
        "tests/FooTest.php",
        "<?php\nclass FooTest\n{\n    // @kotowari[C-t-a-1]\n    /**\n     * @test\n     */\n    public function itWorks()\n    {\n    }\n}\n",
    );
    let v = check(tmp.path());
    assert!(unmarked(&v).is_empty(), "{v}");
    assert_eq!(listed_tests(tmp.path(), "C-t-a-1")[0]["name"], "itWorks");
}

// --- "tests.rules" で足すルール ---

/// "tests.files" と "tests.rules" を書いた設定に置き換える
fn configure(tmp: &Path, test_globs: &[&str], rules: &[&str]) {
    let mut config = String::from("decisions:\n  documents: docs/decisions\ntests:\n  files:\n");
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

// @kotowari[C-core-qrules-1]
#[test]
fn added_rule_counts_tests() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/bench.yml"]);
    write(tmp.path(), "rules/bench.yml", BENCH_RULE);
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["fast"], "{v}");
}

// @kotowari[C-core-qrules-6]
#[test]
fn c_core_qrules_6_a_typescript_rule_does_not_match_tsx_files() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(
        tmp.path(),
        &["tests/**/*.ts", "tests/**/*.tsx"],
        &["rules/bench.yml"],
    );
    write(tmp.path(), "rules/bench.yml", BENCH_RULE);
    write(tmp.path(), "tests/a.test.ts", "bench('fast', () => {})\n");
    write(tmp.path(), "tests/b.test.tsx", "bench('tsx', () => {})\n");
    let v = check(tmp.path());
    assert_eq!(unmarked(&v), vec!["fast"], "{v}");
    assert_eq!(v["tests"]["tsx"]["files"], 1, "{v}");
}

// @kotowari[C-core-qrules-4]
#[test]
fn rules_separated_by_dashes_all_apply_and_give_a_language_a_query() {
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

// @kotowari[C-core-qrules-2, C-core-discover-12]
#[test]
fn rule_hitting_the_bundled_node_counts_once() {
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

// @kotowari[C-core-qrules-7]
#[test]
fn rule_is_not_applied_outside_its_files() {
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

// @kotowari[C-core-qrules-7]
#[test]
fn rule_is_not_applied_to_its_ignores() {
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

// @kotowari[C-core-qrules-8]
#[test]
fn rule_with_severity_off_is_applied() {
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

// @kotowari[C-core-qrules-8]
#[test]
fn fix_message_note_and_metadata_do_not_change_the_tests() {
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

// @kotowari[C-core-qrules-9]
#[test]
fn missing_rule_file_stops() {
    let tmp = TempDir::new().unwrap();
    make_project(tmp.path(), &[]);
    configure(tmp.path(), &["tests/**/*.ts"], &["rules/missing.yml"]);
    let (code, first) = stop(tmp.path());
    assert_eq!(code, Some(2));
    assert!(first.starts_with("config error"), "{first}");
}

// @kotowari[C-core-qrules-9]
#[test]
fn rule_of_an_unknown_language_stops() {
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

// @kotowari[C-core-clienv-9, C-core-qrules-10]
#[test]
fn a_missing_rule_file_stop_points_at_the_rule_file() {
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

// @kotowari[C-core-qrules-11]
#[test]
fn an_unknown_language_stop_names_the_language_in_one_line() {
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

// @kotowari[C-core-qrules-9]
#[test]
fn same_rule_file_twice_stops() {
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

// @kotowari[C-core-qrules-9]
#[test]
fn unreadable_or_malformed_rule_files_stop() {
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

// @kotowari[C-core-qrules-5]
#[test]
fn language_alias_is_accepted() {
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

// @kotowari[C-core-qrules-5]
#[test]
fn language_is_matched_without_case() {
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

// @kotowari[C-core-cover-2, C-core-findings-5]
#[test]
fn test_without_a_name_uses_its_first_line() {
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

// @kotowari[C-core-list-4, C-core-discover-11]
#[test]
fn test_without_a_name_is_listed_with_a_null_name() {
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
        "// @kotowari[C-t-a-1]\nbench(caseName, () => {});\n",
    );
    let tests = listed_tests(tmp.path(), "C-t-a-1");
    assert_eq!(tests.len(), 1, "{tests:?}");
    assert!(tests[0]["name"].is_null(), "{tests:?}");
}

// @kotowari[C-core-discover-12]
#[test]
fn name_comes_from_the_first_rule_that_captures_it() {
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

// @kotowari[C-core-qrules-7, C-core-discover-12]
#[test]
fn rule_with_files_keeps_its_place_in_the_order() {
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
