# Marks in tests

English | [日本語](test-markers.ja.md)

Covers the syntax of the marks written in tests, and the rules that tie marks to tests.

## Requirements

### REQ-core-071: The syntax of a mark

- kind: algorithm
- source: docs/decision/records/records.md#A14, docs/decision/records/records.md#A57
- definition: TBL-core-015
- verification: unit

### REQ-core-072: Malformed marks

- kind: event_driven
- source: docs/decision/records/records.md#A57, docs/decision/records/records.md#A67, docs/decision/records/records.md#A39, docs/decision/records/records.md#A121, docs/decision/records/records.md#A111, docs/decision/records/2026-09-24-multi-language-tests.md#A15
- verification: unit

For a `mark` other than one in a `language with a query` that is in no `test`'s `preceding comment block`, when its contents are empty or only separators, or when that `mark` has no closing bracket on the same line, kotowari raises an invalid_marker `error` with the line's text as detail.

### REQ-core-073: Several marks on one line

- kind: ubiquitous
- source: docs/decision/records/records.md#A57
- verification: unit

kotowari always picks up every `mark` in a line.

### REQ-core-074: Comment symbols are not looked at

- kind: ubiquitous
- source: docs/decision/records/records.md#A14
- verification: unit

kotowari always picks up a `mark` from any position in a line, and does not look at comment symbols.

### REQ-core-075: Tying marks

- kind: algorithm
- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A34, docs/decision/records/records.md#A39, docs/decision/records/records.md#A57, docs/decision/records/2026-09-24-multi-language-tests.md#A15, docs/decision/records/2026-09-24-multi-language-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/2026-09-24-multi-language-tests.md#A26, docs/decision/records/2026-09-24-multi-language-tests.md#A39, docs/decision/records/2026-09-24-multi-language-tests.md#A40, docs/decision/records/2026-09-24-multi-language-tests.md#A41, docs/decision/records/2026-09-24-multi-language-tests.md#A50, docs/decision/records/2026-09-24-multi-language-tests.md#A51, docs/decision/records/2026-09-24-multi-language-tests.md#A52
- definition: TBL-core-016, TBL-core-035
- verification: unit

### REQ-core-076: Marks in a language without a query

- kind: event_driven
- source: docs/decision/records/records.md#A39, docs/decision/records/records.md#A57
- verification: unit

When reading a `test file` of a `language without a query`, kotowari picks up every `mark` in the text of the file, whether or not it is in a comment.

### REQ-core-077: A mark that points only at nonexistent IDs

- kind: event_driven
- source: docs/decision/records/records.md#A57, docs/decision/records/records.md#A89
- verification: unit

When a `mark` points only at nonexistent `ID` values, kotowari counts the `test` that `mark` is tied to as one that has a `mark`. An element inside the square brackets of a `mark` that is not of the form of an `ID` (one without separators, such as "REQ001") is treated as pointing at a nonexistent `ID` and raises an unresolved_reference `error`.

### REQ-core-078: A mark that points at a review requirement

- kind: event_driven
- source: docs/decision/records/records.md#A39
- verification: unit

When a `mark` points at a `requirement` whose verification value is "review", kotowari does not make it an `error`.

### REQ-core-118: The line of findings from marks

- kind: ubiquitous
- source: docs/decision/records/records.md#A121
- verification: unit

kotowari always sets the "line" of an unresolved_reference or invalid_marker raised from a `mark` to the line with the `mark` (for a `mark` spanning lines, the line with "@kotowari[").

## Decision tables

### TBL-core-015: The syntax of a mark

- source: docs/decision/records/records.md#A14, docs/decision/records/records.md#A57

| Part | Form |
|---|---|
| Start | @kotowari[ |
| Contents | IDs separated by commas. Whitespace may be placed before and after a comma |
| End | ] |

### TBL-core-016: Tying marks (languages with a query)

- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A34, docs/decision/records/records.md#A39, docs/decision/records/records.md#A57, docs/decision/records/records.md#A67, docs/decision/records/records.md#A121, docs/decision/records/2026-09-24-multi-language-tests.md#A15, docs/decision/records/2026-09-24-multi-language-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A26, docs/decision/records/2026-09-24-multi-language-tests.md#A41, docs/decision/records/2026-09-24-multi-language-tests.md#A49, docs/decision/records/2026-09-24-multi-language-tests.md#A52

| Position of the mark | Handling |
|---|---|
| The preceding comment block of a test | Tied to that test. When two or more tests have the same first line, tied only to the test that starts first on that line |
| Inside the node of a test (including the comment at the start of a function body). Except a preceding comment block of another test inside the node, which is tied to that other test as in the first row | Ignored; neither invalid_marker nor unresolved_reference is raised |
| In no test's preceding comment block | Ignored; neither invalid_marker nor unresolved_reference is raised |
| A function inside a macro of "tests.rust.macros" | The same rules as above apply |

### TBL-core-035: Lines allowed between the comment block and the test

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A16, docs/decision/records/2026-09-24-multi-language-tests.md#A17, docs/decision/records/records.md#A39, docs/decision/records/2026-09-24-multi-language-tests.md#A39, docs/decision/records/2026-09-24-multi-language-tests.md#A40, docs/decision/records/2026-09-24-multi-language-tests.md#A50, docs/decision/records/2026-09-24-multi-language-tests.md#A51, docs/decision/records/2026-09-24-review10-gaps.md#A1

The preceding comment block is the block of comment-only lines and of the allowed lines of this table that goes upward from just above the first line of a test node until a blank line comes. A comment-only line is a line whose characters, after removing the surrounding whitespace (Unicode whitespace, including the full-width space and NBSP), are all characters of comments (tree-sitter extra nodes); it includes the lines in the middle of a multi-line comment, and does not include a comment line that is on the same line as code. Comment-only lines and allowed lines may mix without blank lines, and for attributes and decorators spanning several lines all their lines are allowed lines. A whitespace-only line in the middle of a multi-line comment, attribute or decorator does not break the block. Among the lines of the block, marks are read only from the characters of comments; marks in the body of an attribute or decorator are not read.

| Language | Allowed lines |
|---|---|
| Rust | Attribute ("#[...]") lines |
| Python | Decorator ("@...") lines |
| Other languages | None |

## Examples

```gherkin
@id=EX-core-015 @about=REQ-core-073 @source=docs/decision/records/records.md#A57,docs/decision/records/records.md#A26,docs/decision/records/records.md#A39
Scenario: Both marks on one line are picked up
  Given the comment just before a `test` has "@kotowari[REQ-001] @kotowari[TBL-002]"
  When "kotowari check" is run
  Then that `test` is tied to "REQ-001" and "TBL-002"

@id=EX-core-016 @about=REQ-core-075 @source=docs/decision/records/records.md#A39,docs/decision/records/records.md#A47,docs/decision/records/ir-form.md#検査の種類,docs/decision/records/records.md#A26,docs/decision/records/records.md#A49
Scenario: A mark in a comment separated by a blank line is not tied
  Given there is a blank line between the comment with "@kotowari[REQ-001]" and the "#[test]" function
  When "kotowari check" is run
  Then a test_without_id error is raised on that function

@id=EX-core-306 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A15
Scenario: A mark at the start of a function body is not tied
  Given the first line of the body of a function with "#[test]" is "// @kotowari[REQ-001]", and there is no comment just before the function
  When "kotowari check" is run
  Then a test_without_id error is raised on that function

@id=EX-core-307 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A17,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: A mark with a decorator in between is tied
  Given "tests.files" contains "tests/**/*.py", and "tests/test_a.py" has a "# @kotowari[REQ-001]" line, a "@pytest.mark.parametrize('a', [1])" line and a "def test_x(a):" line in a row without blank lines
  When "kotowari check" is run
  Then no test_without_id error is raised, and "test_x" is tied to "REQ-001"

@id=EX-core-308 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: The comment just before a method in a class is tied too
  Given "tests.files" contains "tests/**/*.py", and inside "class TestBar:" in "tests/test_a.py" a "# @kotowari[REQ-001]" line and a "def test_baz(self):" line follow in a row without blank lines
  When "kotowari check" is run
  Then no test_without_id error is raised, and "test_baz" is tied to "REQ-001"

@id=EX-core-309 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A17,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: In a language without allowed lines a code line in between breaks the block
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has a "// @kotowari[REQ-001]" line, a "const n = 1;" line and an "it('x', () => {});" line in a row without blank lines
  When "kotowari check" is run
  Then a test_without_id error with detail "x" is raised
@id=EX-core-318 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A40,docs/decision/records/2026-09-24-multi-language-tests.md#A17,docs/decision/records/2026-09-24-multi-language-tests.md#A19
Scenario: A mark with a multi-line decorator in between is tied
  Given "tests.files" contains "tests/**/*.py", and "tests/test_a.py" has a "# @kotowari[REQ-001]" line, a "@pytest.mark.parametrize(" decorator spanning three lines, a "# note" line and a "def test_x(a):" line in a row without blank lines
  When "kotowari check" is run
  Then no test_without_id error is raised, and "test_x" is tied to "REQ-001"

@id=EX-core-319 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A41,docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: A mark just before a nested test is tied to the inner one
  Given "tests.files" contains "tests/**/*.ts", "tests/a.test.ts" has "// @kotowari[REQ-001]" just before "it('outer', ...)", and "// @kotowari[REQ-002]" just before "it('inner', ...)" inside it
  When "kotowari check" is run
  Then no test_without_id error is raised, "outer" is tied to "REQ-001", and "inner" is tied to "REQ-002"

@id=EX-core-320 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A39,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A17
Scenario: A comment on the same line as code breaks the block
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has a "// @kotowari[REQ-001]" line, a "setup(); // prepare" line and an "it('x', () => {});" line in a row without blank lines
  When "kotowari check" is run
  Then a test_without_id error with detail "x" is raised
@id=EX-core-323 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A50,docs/decision/records/2026-09-24-multi-language-tests.md#A40
Scenario: A blank line in the middle of a multi-line decorator does not break the block
  Given "tests.files" contains "tests/**/*.py", "tests/test_a.py" has a "# @kotowari[REQ-001]" line, a "@pytest.mark.parametrize(" decorator of three or more lines containing a whitespace-only line in the middle, and a "def test_x(a):" line in a row, and there is no blank line outside the decorator
  When "kotowari check" is run
  Then no test_without_id error is raised, and "test_x" is tied to "REQ-001"

@id=EX-core-324 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A51
Scenario: A mark in the body of an attribute is not read
  Given right after the line of a "#[doc = ...]" attribute whose string value has "@kotowari[REQ-999]" there is a function with "#[test]", there is no comment just before it, and "REQ-999" does not exist
  When "kotowari check" is run
  Then no unresolved_reference error is raised, and a test_without_id error is raised on that function

@id=EX-core-325 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A52
Scenario: Of the tests starting on the same line only the first is tied
  Given the line right after the "// @kotowari[REQ-001]" line is "#[test] fn a() {} #[test] fn b() {}"
  When "kotowari check" is run
  Then "a" is tied to "REQ-001", and one test_without_id error with detail "b" is raised
@id=EX-core-327 @about=REQ-core-075 @source=docs/decision/records/2026-09-24-review10-gaps.md#A1,docs/decision/records/2026-09-24-multi-language-tests.md#A16,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: A comment after a full-width space also belongs to the block
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has a "// @kotowari[REQ-001]" line starting with a full-width space and an "it('x', () => {});" line in a row without blank lines
  When "kotowari check" is run
  Then no test_without_id error is raised, and "x" is tied to "REQ-001"
```
