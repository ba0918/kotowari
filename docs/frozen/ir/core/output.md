# Output format

English | [日本語](output.ja.md)

Covers the output format chosen with "--format".

## Requirements

### REQ-core-021: Values of the output format

- kind: ubiquitous
- source: docs/decision/records/records.md#A7, docs/decision/records/records.md#A18
- verification: unit

kotowari always accepts the two values "json" and "text" for "--format", and makes "json" the default.

### REQ-core-022: Output one JSON

- kind: event_driven
- source: docs/decision/records/records.md#A40, docs/decision/records/ir-form.md#出力
- verification: unit

When "--format" is "json", kotowari outputs one JSON to standard output.

### REQ-core-023: The content of the JSON

- kind: algorithm
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A56, docs/decision/records/2026-09-17-check-reach.md#A15
- definition: TBL-core-005, TBL-core-006, TBL-core-021, PROP-core-002, PROP-core-004
- verification: unit

### REQ-core-025: Text output

- kind: event_driven
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A50, docs/decision/records/records.md#A18, docs/decision/records/ir-form.md#出力, docs/decision/records/2026-09-16-notice.md#A1, docs/decision/records/2026-09-24-review5-gaps.md#A2
- verification: unit

When "--format" is "text", kotowari outputs each `finding` as one line in the form "path:line [error] kind detail" or "path:line [notice] kind detail", including the square brackets. A line break ("\n" and "\r") in the path or the detail is written as the two characters backslash and "n" or "r".

### REQ-core-026: Text output of a finding without a line

- kind: event_driven
- source: docs/decision/records/records.md#A47
- verification: unit

When "--format" is "text" and the "line" of a `finding` is null, kotowari writes the line as "-".

### REQ-core-128: Reporting the test files

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-check-reach.md#A14, docs/decision/records/2026-09-17-check-reach.md#A15, docs/decision/records/2026-09-17-mutation-tests.md#A55
- verification: unit

kotowari always, in the top-level "tests" of the JSON of "kotowari check", groups each `test file` it read by extension and outputs the count and whether there is a `query`, with the keys of `TBL-core-021`. It does not output this when "--format" is "text".

### REQ-core-206: Reporting the guides

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A17, docs/decision/records/2026-09-24-doc-marks.md#A35
- verification: unit

kotowari always, in the top-level "guides" of the JSON of "kotowari check", outputs the number of `guide` files it read as "files" and the number of entries of each well-formed `guide mark` as "marks". "files" counts files the same way as TBL-core-021 (once per path, once even when matched by several globs, a symbolic link and its target as separate paths). When there is no `guide`, both are 0. It does not output this when "--format" is "text".

## Decision tables

### TBL-core-005: The top level of the JSON of check

- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#出力, docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-24-plan-schema.md#A17, docs/decision/records/2026-09-24-doc-marks.md#A17, docs/decision/records/2026-09-24-doc-marks.md#A35, docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A16, docs/decision/records/2026-10-02-whole-picture.md#A64

The top level of the JSON of "kotowari check". The top level of the JSON of "kotowari mutants" is TBL-core-025, and that of "kotowari plan" is REQ-core-194.

| Key | Content |
|---|---|
| files | the number of IR documents read (including glossaries and flag records) |
| lines | the total number of lines of the IR documents (including glossaries and flag records) |
| findings | the list of findings |
| counts | the number of findings per kind |
| tests | the number of test files read per extension, and whether that extension is a language with a query (TBL-core-021) |
| guides | an object with the two keys "files" (the number of `guide` files read) and "marks" (the number of entries of each well-formed `guide mark`) (REQ-core-206) |
| overview | an object with the two keys "files" (the number of files of `overview data` read) and "marks" (the number of entries of each well-formed `guide mark` in the `overview data`) (REQ-core-288) |
| surface | an object with the one key "unspecified" (the number of surfaces excluded by the list of unspecified surfaces). When "surface.rules" is an empty list, the key itself is not output (REQ-core-228) |

### TBL-core-006: The keys of a finding

- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A61, docs/decision/records/records.md#A106, docs/decision/records/2026-09-16-ir-tree.md#A13, docs/decision/records/2026-09-16-notice.md#A1, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-24-plan-schema.md#A26, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A11, docs/decision/records/2026-09-24-doc-marks.md#A32, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A7, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22

| Key | Content |
|---|---|
| kind | the kind of the finding (TBL-core-008, TBL-core-009) |
| severity | error or notice |
| path | the path relative to the base directory: the normalized location and the path of the document relative to the location, joined with "/" (REQ-core-110). For a finding of "kotowari mutants", the file of the mutation outcome or the file of the list of equivalents (REQ-core-139, REQ-core-140, REQ-core-142, REQ-core-143). For a finding of "kotowari plan", the file of the `plan` (REQ-core-193). For a finding on a guide, the path of the `guide` relative to the base directory (REQ-core-202, REQ-core-204). For surface_without_spec, the `surface file`; for a finding on the list of unspecified surfaces, the file of the list of unspecified surfaces (REQ-core-227, REQ-core-233, REQ-core-234) |
| line | the line (1-based). null for a finding on the whole document |
| detail | the string decided per kind in TBL-core-008 and TBL-core-009 |

### TBL-core-021: The content of "tests"

- source: docs/decision/records/2026-09-17-check-reach.md#A8, docs/decision/records/2026-09-17-check-reach.md#A14, docs/decision/records/2026-09-17-check-reach.md#A19, docs/decision/records/2026-09-17-check-reach.md#A20, docs/decision/records/2026-09-17-check-reach.md#A26, docs/decision/records/records.md#A128, docs/decision/records/records.md#A165, docs/decision/records/2026-09-24-multi-language-tests.md#A24, docs/decision/records/2026-09-24-multi-language-tests.md#A8, docs/decision/records/2026-09-24-multi-language-tests.md#A21, docs/decision/records/2026-09-24-multi-language-tests.md#A47

"tests" is an object whose keys are the extensions of the test files read and whose values are objects with the two keys "files" and "query". When there are no test files, "tests" is an empty object.

| Level | Key | Value | Condition |
|---|---|---|---|
| directly under "tests" | the extension | an object with "files" and "query" | one per extension of the test files read. The extension is the text after the last "." of the file name, not including the ".". A name that is only a leading "." (".rs") and a name without "." have no extension, and the extension of "foo." is empty; in all these cases the key is the empty string. Case-sensitive. Ordered by bytes. Files are counted once per path, once even when matched by several globs, and a symbolic link and its target are counted as separate paths. Anything matched by a glob but not read (an `exclusion`) is not counted |
| inside the value of an extension | files | the number of test files read with that extension | files that produced unparsable_file are counted too |
| inside the value of an extension | query | true if the language decided by that extension is a `language with a query`, false otherwise | with only the bundled queries, true for "rs", "ts", "mts", "cts", "tsx", "js", "jsx", "mjs", "cjs", "py", "py3", "pyi", "bzl", "bazel", "php" |

## Properties

### PROP-core-002: counts agrees with findings

- source: docs/decision/records/records.md#A40, docs/decision/records/ir-form.md#出力

The value of each kind in "counts" equals the number of `finding` entries of that kind in "findings", and a kind with no entry in "findings" is not in "counts".

### PROP-core-004: The total of files in tests

- source: docs/decision/records/2026-09-17-check-reach.md#A15

The total of "files" over the extensions of "tests" equals the number of each `test file` read.

## Examples

```gherkin
@id=EX-core-286 @about=REQ-core-025 @source=docs/decision/records/2026-09-24-review5-gaps.md#A2
Scenario: One finding is one line even for a file name with a line break
  Given there is a document of the `IR` whose name contains a line break and that has no `title`
  When "kotowari check --format text" is run
  Then every line starts with the path of that document, and the line break in the path is written as the two characters backslash and "n"

@id=EX-core-004 @about=REQ-core-025,REQ-core-026 @source=docs/decision/records/records.md#A47,docs/decision/records/records.md#A40,docs/decision/records/records.md#A50,docs/decision/records/ir-form.md#検査の種類
Scenario: Outputting a document without a title as text
  Given "docs/ir/a.md" has no title
  When "kotowari check --format text" is run
  Then the line "docs/ir/a.md:- [error] missing_title a.md" is output

@id=EX-core-035 @about=REQ-core-128,TBL-core-021 @source=docs/decision/records/2026-09-17-check-reach.md#A8,docs/decision/records/2026-09-17-check-reach.md#A14,docs/decision/records/2026-09-17-check-reach.md#A19,docs/decision/records/records.md#A128,docs/decision/records/2026-09-24-multi-language-tests.md#A7,docs/decision/records/2026-09-24-multi-language-tests.md#A24
Scenario: Test files of a language without a query are reported with query false
  Given the globs of "tests.files" match two ".rs" files and one ".go" file, and there is no `query` for ".go"
  When "kotowari check" is run
  Then the "tests" of the JSON has "rs" with "files" 2 and "query" true, and "go" with "files" 1 and "query" false

@id=EX-core-038 @about=REQ-core-128,PROP-core-004 @source=docs/decision/records/2026-09-17-check-reach.md#A14,docs/decision/records/2026-09-17-check-reach.md#A15
Scenario: An unreadable test file is counted too
  Given the globs of "tests.files" match two ".rs" files, one of which tree-sitter cannot read
  When "kotowari check" is run
  Then one unparsable_file error is output, and the "files" of "rs" in "tests" is 2
```
