# Bundled queries

English | [日本語](test-queries.ja.md)

Covers the queries kotowari bundles for each language, and what counts as a test in each language.

## Requirements

### REQ-core-182: Languages with bundled queries

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A8, docs/decision/records/2026-09-24-multi-language-tests.md#A21
- verification: unit

kotowari always bundles `query` entries for Rust, TypeScript, Tsx, JavaScript, Python and Php. TypeScript and Tsx are separate languages, and a `query` with the same contents is bundled for both.

### REQ-core-082: Rust tests

- kind: algorithm
- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A39, docs/decision/records/records.md#A49, docs/decision/records/2026-09-24-multi-language-tests.md#A1, docs/decision/records/2026-09-24-multi-language-tests.md#A12
- definition: TBL-core-017
- verification: unit

### REQ-core-183: TypeScript and JavaScript tests

- kind: algorithm
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A18, docs/decision/records/2026-09-24-multi-language-tests.md#A25
- definition: TBL-core-032
- verification: unit

### REQ-core-184: Python tests

- kind: algorithm
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A19
- definition: TBL-core-033
- verification: unit

### REQ-core-185: Php tests

- kind: algorithm
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A20
- definition: TBL-core-034
- verification: unit

## Decision tables

### TBL-core-017: What counts as a test in Rust

- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A39, docs/decision/records/records.md#A49, docs/decision/records/records.md#A47, docs/decision/records/records.md#A121, docs/decision/records/records.md#A122, docs/decision/records/2026-09-24-multi-language-tests.md#A38, docs/decision/records/2026-09-24-multi-language-tests.md#A53, docs/decision/records/2026-09-24-review10-gaps.md#A4

| Target | How it is counted |
|---|---|
| A function whose attribute path ends with the element "test" ("#[test]", "#[ test ]", "#[core::prelude::v1::test]", "#[tokio::test]") | Fixed in the bundled query, and always counted |
| A function with an attribute of tests.rust.attributes | Counted when the path, with "#[", "]" and the arguments removed from the attribute, matches exactly |
| When a function of the two rows above or a macro of tests.rust.macros is inside the body of another function | Counted regardless of where it is placed |
| A macro of tests.rust.macros | Written in the configuration by its name without "!". When the last element of the macro's name matches, its contents are re-read as Rust items, and each top-level function (a function not inside the body of another function, including functions inside "mod" and "impl") is counted. The name is the function's name. Mark tying and invalid_marker are the same as for ordinary functions |

### TBL-core-032: What counts as a test in TypeScript and JavaScript

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A18, docs/decision/records/2026-09-24-multi-language-tests.md#A25, docs/decision/records/2026-09-24-multi-language-tests.md#A13, docs/decision/records/2026-09-24-multi-language-tests.md#A37, docs/decision/records/2026-09-24-multi-language-tests.md#A21, docs/decision/records/2026-09-24-review11-gaps.md#A1

Common to TypeScript, Tsx and JavaScript. The name has its quotes removed as in REQ-core-180.

| Target | How it is counted |
|---|---|
| A call whose callee is "it" or "test" ("it(...)", "test(...)") | Counted. The name is the first argument |
| A call whose callee is "it" or "test" followed by one or more "." and name ("it.only(...)", "it.skip(...)", "it.todo(...)", "test.concurrent(...)", "it.only.each(...)") | Counted. The name is the first argument. Except the inner call of the next row |
| Of the forms above, a form that further calls the result of a call whose last element is "each" ("it.each(table)(name, function)", "test.concurrent.each(table)(name, function)") | The outer call is counted. The name is the first argument of the outer call. The inner "it.each(table)" is not counted |
| "describe(...)" and its "." forms | Not counted. The calls inside are counted as in the rows above, and the "describe" name is not added to their names |
| A call whose callee is neither "it" nor "test" ("regex.test(s)") | Not counted |
| A tagged template ("test.each`table`" alone) | Not counted. The form that calls its result ("test.each`table`(name, function)") counts the outer call |

### TBL-core-033: What counts as a test in Python

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A19, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/2026-09-24-multi-language-tests.md#A32, docs/decision/records/2026-09-24-multi-language-tests.md#A48, docs/decision/records/2026-09-24-review10-gaps.md#A3

| Target | How it is counted |
|---|---|
| A top-level function of the file whose name starts with "test" | Counted. The name is the function's name |
| A method in a class whose name starts with "test" (even when the class is inside a function) | Counted. The name is the method's name |
| The functions and methods above with decorators | Counted. The decorators are outside the test node |
| A function nested inside a function | Not counted |

### TBL-core-034: What counts as a test in Php

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A20, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/2026-09-24-multi-language-tests.md#A32, docs/decision/records/2026-09-24-multi-language-tests.md#A48

| Target | How it is counted |
|---|---|
| A method whose name starts with "test" | Counted. The name is the method's name |
| A method with an attribute whose path ends with the element "Test" ("#[Test]", "#[\PHPUnit\Framework\Attributes\Test]") | Counted. The name is the method's name. The attribute is inside the test node |
| A method whose immediately preceding comment starting with "/**" has "@test" | Counted. The name is the method's name |
| A call of a function whose callee is "test" or "it" (Pest's "test('...', fn)") | Counted. The name is the first argument |

## Examples

```gherkin
@id=EX-core-017 @about=REQ-core-082 @source=docs/decision/records/records.md#A39,docs/decision/records/records.md#A49,docs/decision/records/records.md#A47
Scenario: An attribute with arguments is counted too
  Given "tests.rust.attributes" is a list of only "kani::proof"
  And there is a function with "#[kani::proof(unwind = 3)]"
  When "kotowari check" is run
  Then that function is counted as a `test`

@id=EX-core-018 @about=REQ-core-082 @source=docs/decision/records/records.md#A49,docs/decision/records/records.md#A26,docs/decision/records/records.md#A39,docs/decision/records/records.md#A47
Scenario: A macro with a path is counted too
  Given "tests.rust.macros" is a list of only "proptest"
  And there are two functions inside "proptest::proptest!"
  When "kotowari check" is run
  Then two `test` entries are counted

@id=EX-core-298 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: A skipped test is counted too
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has "it.skip('x', () => {})" with no mark
  When "kotowari check" is run
  Then one test_without_id error with detail "x" is raised

@id=EX-core-299 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A37
Scenario: A test with an each table is counted as one
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has "it.each([1])('each %i', (n) => {})" with no mark
  When "kotowari check" is run
  Then exactly one test_without_id error with detail "each %i" is raised

@id=EX-core-300 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: describe and calls that are not it are not counted
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has only "describe('d', () => { regex.test(s); })"
  When "kotowari check" is run
  Then no test_without_id error is raised

@id=EX-core-301 @about=REQ-core-182,REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A21,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: Tests in a tsx file are counted too
  Given "tests.files" contains "tests/**/*.tsx", and "tests/a.test.tsx" has "it('x', () => {})" with no mark
  When "kotowari check" is run
  Then one test_without_id error with detail "x" is raised

@id=EX-core-302 @about=REQ-core-184 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A19,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: Nested functions are not counted
  Given "tests.files" contains "tests/**/*.py", "tests/test_a.py" has "def test_foo():" with no mark with "def test_inner():" inside it, and "def test_baz(self):" with no mark inside "class TestBar:"
  When "kotowari check" is run
  Then one test_without_id error is raised with detail "test_foo" and one with detail "test_baz", and no error is raised for "test_inner"

@id=EX-core-303 @about=REQ-core-185 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A20,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: A method with a Test attribute with a path is counted
  Given "tests.files" contains "tests/**/*.php", and a class in "tests/FooTest.php" has a method "other" with "#[\PHPUnit\Framework\Attributes\Test]" and no mark
  When "kotowari check" is run
  Then one test_without_id error with detail "other" is raised

@id=EX-core-304 @about=REQ-core-185 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A20,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: A method with @test in its docblock is counted
  Given "tests.files" contains "tests/**/*.php", and a class in "tests/FooTest.php" has a method "itWorks" right after "/** @test */"
  When "kotowari check" is run
  Then one test_without_id error with detail "itWorks" is raised

@id=EX-core-305 @about=REQ-core-185 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A20,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A48
Scenario: A Pest test call is counted
  Given "tests.files" contains "tests/**/*.php", and "tests/FooTest.php" has "test('adds', function () {});" with no mark
  When "kotowari check" is run
  Then one test_without_id error with detail "adds" is raised
@id=EX-core-317 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A37,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: A test with a chained each table is counted as one too
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has "it.only.each([1])('each %i', (n) => {})" with no mark
  When "kotowari check" is run
  Then exactly one test_without_id error with detail "each %i" is raised
@id=EX-core-326 @about=REQ-core-082 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A53
Scenario: A test inside the body of a function is counted too
  Given inside the body of "fn helper() {" there is a function "inner" with "#[test]" and no mark
  When "kotowari check" is run
  Then one test_without_id error with detail "inner" is raised
@id=EX-core-329 @about=REQ-core-184 @source=docs/decision/records/2026-09-24-review10-gaps.md#A3,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: A method of a class inside a function is counted too
  Given "tests.files" contains "tests/**/*.py", and in "tests/test_a.py" there is "class C:" inside "def test_outer():", with "def test_in_class(self):" with no mark inside it
  When "kotowari check" is run
  Then a test_without_id error with detail "test_in_class" is raised

@id=EX-core-330 @about=REQ-core-082 @source=docs/decision/records/2026-09-24-review10-gaps.md#A4
Scenario: A function in a mod inside a macro is counted too
  Given "tests.rust.macros" is a list of only "proptest", and inside "mod nested {" inside "proptest!" there is a function "inside_module" with no mark
  When "kotowari check" is run
  Then one test_without_id error with detail "inside_module" is raised

@id=EX-core-331 @about=REQ-core-183 @source=docs/decision/records/2026-09-24-review11-gaps.md#A1
Scenario: A tagged template alone is not counted
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has "test.each`a`('tagged %s', () => {})" with no mark and "test.each`foo`" whose result is not called
  When "kotowari check" is run
  Then exactly one test_without_id error with detail "tagged %s" is raised
```
