# Tallying whether everything is in place

English | [日本語](status.ja.md)

Covers how "kotowari status" tallies the `IR`, the `test` entries with a `mark`, and the `finding` entries of "kotowari check", and outputs whether everything is in place (complete) as counts and a truth value.

## Requirements

### REQ-core-162: Reading in status

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A19, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A64
- verification: unit

In "kotowari status", kotowari always reads the documents of the `IR`, the `test file` files, the `guide` files and the `overview data` from the same configuration and places as "kotowari check", and, when "surface.rules" is not an empty list, also reads the `surface file` files, the `surface rule` files and the `list of unspecified surfaces`; it performs the same checks as check, outputs no `finding`, and outputs to standard output one tally with the keys of `TBL-core-028`.

### REQ-core-163: Stops of status

- kind: event_driven
- source: docs/decision/records/2026-09-20-query-status.md#A19
- verification: unit

In "kotowari status", when a condition under which "kotowari check" makes a `stop` (a configuration error, an unreadable file, an argument error) holds, kotowari makes a `stop` with the same reason and wording as check.

### REQ-core-164: Keys of the tally

- kind: algorithm
- source: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A12
- definition: TBL-core-028
- verification: unit

### REQ-core-165: complete and the exit code

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A9, docs/decision/records/2026-09-20-query-status.md#A11
- verification: unit

kotowari always makes "complete" true only when "kotowari check" has zero `error` entries and there are zero `item` entries of a `flag record`; in that case it makes the exit code 0, and when it is false it makes the exit code 1. A `stop` is 2.

### REQ-core-166: The form of the output of status

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-20-query-status.md#A13, docs/decision/records/2026-09-20-query-status.md#A14, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-20-query-status.md#A20
- verification: unit

kotowari always accepts only the two values "json" and "text" for "--format" of "kotowari status", with "json" as the default. With "json" it outputs one JSON whose top level is only the group keys of `TBL-core-028`, with the groups in the order of the table of `TBL-core-028` and "complete" last. With "text" it outputs, for each group of `TBL-core-028`, one line of the form "group key=value key=value", in the order of the table of `TBL-core-028`. The key words are the same as in JSON, values are separated by one half-width space, and no spaces for column alignment are inserted. The "complete" line is "complete true" or "complete false".

## Decision tables

### TBL-core-028: Keys of status

- source: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A9, docs/decision/records/2026-09-20-query-status.md#A12, docs/decision/records/2026-09-20-query-status.md#A14, docs/decision/records/2026-09-20-query-status.md#A20, docs/decision/records/2026-09-17-scenario-tests.md#A9, docs/decision/records/2026-09-19-read-commands.md#A24, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-doc-marks.md#A17, docs/decision/records/2026-09-25-deferred-items.md#A8, docs/decision/records/2026-09-25-deferred-items.md#A15, docs/decision/records/2026-09-25-deferred-items.md#A16, docs/decision/records/2026-09-25-deferred-items.md#A24, docs/decision/records/2026-09-25-deferred-items.md#A26, docs/decision/records/2026-09-27-surface-check.md#A10, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A17, docs/decision/records/2026-09-27-surface-check.md#A16, docs/decision/records/2026-09-27-surface-check.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A64

| Group | Keys | Content |
|---|---|---|
| documents | files, lines | The number of documents of the `IR` read and the total number of their lines (the same as "files" and "lines" of "kotowari check") |
| items | requirement, table, property, scenario, flag | The number of `item` and `scenario` entries having an `ID`, per kind |
| requirements | unit, property, proof, review | The number of `requirement` entries per value of "- verification:". A `requirement` without the line is counted in none |
| requirements | with_tests, without_tests | Among the `requirement` entries whose "- verification:" is not review and that are not under `deferral`, the number of those for which there is a `test` whose `mark` contains its `ID` or a `test` whose `mark` contains the `ID` of a `scenario` that has its `ID` in "@about", and the number of those for which there is none |
| requirements | review_with_how_to_verify, review_without_how_to_verify | Among the `requirement` entries whose "- verification:" is review, the number of those that have a "- how_to_verify:" line and the number of those that do not |
| requirements | without_examples | The number of `requirement` entries for which no `scenario` has its `ID` in "@about". `requirement` entries under `deferral` are counted too |
| requirements | deferred | The number of `requirement` entries under `deferral` (counted per occurrence of an `item`, like "unit" and the others; for `requirement` entries with the same `ID`, the first one under REQ-core-032 decides whether it is under `deferral`) |
| scenarios | with_tests, without_tests | Among the `scenario` entries that are not a `deferred scenario`, the number of those for which there is a `test` whose `mark` contains its `ID`, and the number of those for which there is none |
| scenarios | deferred | The number of `deferred scenario` entries |
| tests | marks | The number of occurrences of a `mark`. When one `mark` has several `ID` entries, one per `ID` (counted the same way as one entry of "tests" of list) |
| tests | files | The same as "tests" of "kotowari check" (`TBL-core-021`). In "text", "extension=number of files" per extension |
| guides | files, marks | The same as "guides" of "kotowari check" (`TBL-core-005`) |
| overview | files, marks | The same as "overview" of "kotowari check" (`TBL-core-005`) |
| surface | total, specified, unspecified | The number of pairs of kind and name of `surface` entries, the number of those that are in the `IR`, and the number of those that are not in the `IR` and matched one well-formed entry of the `list of unspecified surfaces` (REQ-core-229). When "surface.rules" is an empty list, all three are 0 |
| findings | error, notice | The numbers of `finding` entries of "kotowari check" whose severity is "error" and whose severity is "notice" |
| complete | (value only) | true or false (REQ-core-165) |

## Examples

```gherkin
@id=EX-core-258 @about=REQ-core-162,REQ-core-165,TBL-core-028 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A11,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: When everything is in place, complete with exit code 0
  Given the only document of the IR is "docs/ir/a.md", which has a requirement "REQ-001" verified by unit, a requirement "REQ-002" verified by review with a "- how_to_verify:" line, and a scenario "EX-001" with "@about=REQ-001", and "tests/a.rs" has a test with the mark "@kotowari[REQ-001, EX-001]"
  And "kotowari check" has 0 findings and there are 0 flag records
  When "kotowari status" is run
  Then the exit code is 0, "requirements" has "unit" 1, "review" 1, "with_tests" 1 and "review_with_how_to_verify" 1, "with_tests" of "scenarios" is 1, "marks" of "tests" is 2, "error" of "findings" is 0, and "complete" is true

@id=EX-core-259 @about=REQ-core-165,REQ-core-098 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A10,docs/decision/records/2026-09-20-query-status.md#A11,docs/decision/records/2026-09-20-query-status.md#A17,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-23-ir-english-tokens.md#A2,docs/decision/records/2026-09-23-ir-english-tokens.md#A7
Scenario: A review requirement without a way to verify is an error and not complete
  Given the IR has a requirement "REQ-002" verified by review without a "- how_to_verify:" line
  When "kotowari status" is run
  Then the exit code is 1, "error" of "findings" is 1 or more, and "complete" is false
  And "kotowari check" raises an error of missing_field with detail "how_to_verify"

@id=EX-core-260 @about=REQ-core-165 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A11
Scenario: With a flag record, it is not complete
  Given "kotowari check" has 0 findings and the flag record has one "FLAG-001"
  When "kotowari status" is run
  Then the exit code is 1, "flag" of "items" is 1, and "complete" is false

@id=EX-core-261 @about=REQ-core-166 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A13,docs/decision/records/2026-09-20-query-status.md#A14,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-25-deferred-items.md#A8
Scenario: text is one line per group
  Given there are the same IR and tests as in EX-core-258
  When "kotowari status --format text" is run
  Then the first line of the output starts with "documents files=1 lines=", the line starting with "requirements " contains "unit=1 property=0 proof=0 review=1 with_tests=1 without_tests=0 review_with_how_to_verify=1 review_without_how_to_verify=0 without_examples=1 deferred=0", the line starting with "scenarios " ends with "with_tests=1 without_tests=0 deferred=0", and the last line is "complete true"

@id=EX-core-262 @about=REQ-core-163 @source=docs/decision/records/2026-09-20-query-status.md#A19
Scenario: When the configuration cannot be read, it stops as check does
  Given the configuration file cannot be read as YAML
  When "kotowari status" is run
  Then the exit code is 2 and the first line of standard error has the same wording as "kotowari check"

@id=EX-core-263 @about=TBL-core-028 @source=docs/decision/records/2026-09-20-query-status.md#A12,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-17-scenario-tests.md#A9
Scenario: A requirement with a test through a scenario counts in with_tests too
  Given the IR has a requirement "REQ-001" verified by unit and a scenario "EX-001" with "@about=REQ-001", and "tests/a.rs" has only a test with the mark "@kotowari[EX-001]"
  When "kotowari status" is run
  Then "with_tests" of "requirements" is 1 and "without_tests" is 0

@id=EX-core-398 @about=TBL-core-028,REQ-core-165 @source=docs/decision/records/2026-09-25-deferred-items.md#A4,docs/decision/records/2026-09-25-deferred-items.md#A8,docs/decision/records/2026-09-25-deferred-items.md#A15,docs/decision/records/2026-09-25-deferred-items.md#A16
Scenario: Deferral appears in the counts and does not affect complete
  Given the IR has a requirement "REQ-001" verified by unit and a scenario "EX-001" with "@about=REQ-001", "REQ-001" is under deferral, there is no mark containing either ID, and there are no other findings and no flag records
  When "kotowari status" is run
  Then "deferred" of "requirements" is 1, "with_tests", "without_tests" and "without_examples" are 0, "deferred" of "scenarios" is 1, "with_tests" and "without_tests" are 0, and "complete" is true
```
