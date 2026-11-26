# Listing items

English | [日本語](list.ja.md)

Covers how "kotowari list" outputs the `item` and `scenario` entries of the `IR`, together with the `test` entries that have a `mark` pointing at them, in a form people and LLMs can read. What it reads is the same as check, and it outputs no `finding`.

## Requirements

### REQ-core-151: What list reads

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A1, docs/decision/records/2026-09-19-read-commands.md#A2, docs/decision/records/2026-09-19-read-commands.md#A9, docs/decision/records/2026-09-19-read-commands.md#A21
- verification: unit

kotowari always, on "kotowari list", reads the documents of the `IR` and each `test file` with the same configuration and locations as "kotowari check", outputs every `item` and `scenario` it could read to standard output, outputs no `finding`, and sets the exit code to 0. Even when a document of the `IR` has an `error`, it outputs the `item` and `scenario` entries it could read.

### REQ-core-152: When list stops

- kind: event_driven
- source: docs/decision/records/2026-09-19-read-commands.md#A9, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-05-localization.md#A37, docs/decision/records/2026-10-05-localization.md#A21
- verification: unit

When, on "kotowari list", a condition under which "kotowari check" makes a `stop` holds (a configuration error, an unreadable file, an argument error), kotowari makes a `stop` with the same reason and wording as check. However, "kotowari list" does not read any `guide`, and does not make the `stop` caused by the `guide` locations and reading (REQ-core-198, REQ-core-199). It also does not read the `surface file`, the `surface rule` file or the `list of unspecified surfaces`, and does not make the `stop` caused by reading them (REQ-core-229). It also does not read the `overview data`, and does not make the `stop` caused by its locations and reading (REQ-core-278, REQ-core-280). However, when the `language list` has two or more languages, it walks the locations of the `guide` files, the `overview data` and the `table of contents` the same way as check in order to output "translations", makes the `stop` caused by those locations and reading the same way as check, and does not check their contents.

### REQ-core-153: The form of an item

- kind: algorithm
- source: docs/decision/records/2026-09-19-read-commands.md#A3, docs/decision/records/2026-09-19-read-commands.md#A6, docs/decision/records/2026-09-19-read-commands.md#A12, docs/decision/records/2026-09-19-read-commands.md#A13, docs/decision/records/2026-09-19-read-commands.md#A14, docs/decision/records/2026-09-19-read-commands.md#A18
- definition: TBL-core-026
- verification: unit

### REQ-core-154: The order of the list

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A12
- verification: unit

kotowari always orders the entries of "items" by "path" ascending, and within the same "path" by "line" ascending, and orders the "tests" of each entry likewise by "path" ascending, and within the same "path" by "line" ascending.

### REQ-core-155: The form of the output

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-19-read-commands.md#A8, docs/decision/records/2026-09-19-read-commands.md#A12, docs/decision/records/2026-09-19-read-commands.md#A19, docs/decision/records/2026-09-19-read-commands.md#A25, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A9, docs/decision/records/2026-10-05-localization.md#A29, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari always accepts only the two values "json" and "text" for "--format" of "kotowari list", makes "json" the default, and has no filtering options. With "json" it outputs one JSON whose top level has "items" and, only when the `language list` has two or more languages, "translations"; "items" is a list of entries, each with the keys of `TBL-core-026`. "translations" is a list with one entry per `pair` (a `pair` without a `side` in the `first language` is also one entry); each entry has "path" (the path of the `side` in the `first language`, relative to the base directory) and "sides" (a list, in the order of the `language list`, of entries with "language" (the language tag), "path" (the path of that `side`, relative to the base directory) and "blob" (the blob hash of that `side`; null if the `side` does not exist)), and the entries are ordered by "path" ascending. With "text", after the lines of "items", it outputs one line per entry of "translations": "path" followed, for each entry of "sides", by one space and "language-tag=blob-hash" ("language-tag=-" for a missing `side`). With "text" it outputs each entry of "items" as one line of the form "ID verification name path:line tests=count" ("verification" is "-" for anything other than a requirement and for a requirement without a "- verification:" line), appends " deferred" to the end of that line for an entry whose "deferred" is true, and follows it immediately with one line "path:line name", indented by two spaces, per entry of "tests" ("name" is "-" when it is null).

## Decision tables

### TBL-core-026: The keys of an item

- source: docs/decision/records/2026-09-19-read-commands.md#A6, docs/decision/records/2026-09-19-read-commands.md#A13, docs/decision/records/2026-09-19-read-commands.md#A14, docs/decision/records/2026-09-19-read-commands.md#A18, docs/decision/records/2026-09-19-read-commands.md#A22, docs/decision/records/2026-09-19-read-commands.md#A24, docs/decision/records/2026-09-19-read-commands.md#A26, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-23-ir-english-tokens.md#A8, docs/decision/records/2026-09-24-review6-gaps.md#A1, docs/decision/records/2026-09-24-multi-language-tests.md#A13, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-25-deferred-items.md#A9

| Key | Kinds that have it | Content |
|---|---|---|
| id | all | `ID` |
| kind | all | one of "requirement", "table", "property", "scenario", "flag" |
| name | all | the name of the heading. For a `scenario`, the text after "Scenario:" with leading and trailing spaces and tabs removed |
| path | all | the path of the document containing the `item`, relative to the base directory |
| line | all | the line of the heading. For a `scenario`, the "Scenario:" line |
| type | requirement, flag record | the value of "- kind:". null if absent |
| verification | requirement | the value of "- verification:". null if absent |
| definition | requirement | the list of each `ID` in "- definition:". An empty list if absent |
| examples | requirement, decision table, property | the list of the `ID` of each `scenario` that has that `ID` in "@about", in ascending `ID` order. When two or more `scenario` entries have the same `ID`, only the first one per REQ-core-032 counts |
| how_to_verify | requirement | the value of "- how_to_verify:". null if absent |
| relations | flag record | the list of each `ID` in "- related:" |
| sources | all | the list of each `source` |
| tests | all | the list of each `test` whose `mark` includes that `ID`. If the same `test` has several `mark` entries with the same `ID`, one entry per occurrence of the `mark`. Each entry has "path" (the path of the `test file`, relative to the base directory), "line" (the line with the `mark`) and "name" (the name of the `test`; null for a `language without a query` and for a `test` whose name is null) |
| fingerprint | all | the `fingerprint` of that `item` or `scenario` (REQ-core-203) |
| deferred | all | true for a `requirement` under `deferral` and for a `deferred scenario`, false otherwise |

## Examples

```gherkin
@id=EX-core-288 @about=TBL-core-026 @source=docs/decision/records/2026-09-24-review6-gaps.md#A1
Scenario: Only the first scenario with a duplicated ID counts as an example
  Given two `scenario` entries have the same `ID`; the first has "REQ-001" in "@about" and the second has "REQ-002"
  When "kotowari list --format json" is run
  Then the "examples" of "REQ-001" has that `ID`, and the "examples" of "REQ-002" is empty

@id=EX-core-245 @about=REQ-core-151,REQ-core-153,TBL-core-026 @source=docs/decision/records/2026-09-19-read-commands.md#A2,docs/decision/records/2026-09-19-read-commands.md#A6,docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A18
Scenario: One requirement and one test with a mark are output
  Given the IR has a requirement "REQ-001" named "例" on line 7 of "docs/ir/a.md", whose verification is "unit"
  And a test "req_001_x" follows immediately after the mark "@kotowari[REQ-001]" on line 3 of "tests/a.rs"
  When "kotowari list" is run
  Then the exit code is 0, "items" has one entry whose "id" is "REQ-001", "kind" is "requirement" and "verification" is "unit", and its "tests" is one entry whose "path" is "tests/a.rs", "line" is 3 and "name" is "req_001_x"

@id=EX-core-246 @about=REQ-core-151 @source=docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A18,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: Items that could be read are output even when the IR has errors
  Given the IR has a requirement "REQ-002" without a "- verification:" line
  When "kotowari list" is run
  Then the exit code is 0, "items" has one entry whose "id" is "REQ-002" and "verification" is null, and the output has no "findings"

@id=EX-core-247 @about=REQ-core-153,TBL-core-026 @source=docs/decision/records/2026-09-19-read-commands.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A7
Scenario: A test in a language without a query has a null name
  Given line 2 of "tests/a.go" has the mark "@kotowari[REQ-001]", and there is no `query` for ".go"
  When "kotowari list" is run
  Then the "tests" of "REQ-001" has one entry whose "path" is "tests/a.go", "line" is 2 and "name" is null

@id=EX-core-248 @about=REQ-core-155 @source=docs/decision/records/2026-09-19-read-commands.md#A7,docs/decision/records/2026-09-19-read-commands.md#A19
Scenario: text puts one item per line followed by its test lines
  Given the same IR and tests as EX-core-245
  When "kotowari list --format text" is run
  Then the first line is "REQ-001 unit 例 docs/ir/a.md:7 tests=1" and the second line is "  tests/a.rs:3 req_001_x"

@id=EX-core-249 @about=REQ-core-152 @source=docs/decision/records/2026-09-19-read-commands.md#A9,docs/decision/records/2026-09-19-read-commands.md#A20
Scenario: When the configuration cannot be read, it stops the same way as check
  Given the configuration file cannot be read as YAML
  When "kotowari list" is run
  Then the exit code is 2, and the first line of standard error has the same wording as "kotowari check"

@id=EX-core-399 @about=TBL-core-026,REQ-core-155 @source=docs/decision/records/2026-09-25-deferred-items.md#A9,docs/decision/records/2026-09-25-deferred-items.md#A15,docs/decision/records/2026-09-19-read-commands.md#A19
Scenario: A deferred requirement and scenario get deferred
  Given line 7 of "docs/ir/a.md" has a deferred requirement "REQ-001" named "例" whose verification is "unit", there is a scenario "EX-001" with "@about=REQ-001" and a requirement "REQ-002" that is not deferred, and no mark includes any of these IDs
  When "kotowari list" and "kotowari list --format text" are run
  Then in json the "deferred" of "REQ-001" and "EX-001" is true and that of "REQ-002" is false, and the "REQ-001" line in text is "REQ-001 unit 例 docs/ir/a.md:7 tests=0 deferred"
```
