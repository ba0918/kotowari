# Correspondence between requirements and tests

English | [日本語](coverage.ja.md)

Covers the check of whether requirements and examples have tests with marks, and whether tests have marks.

## Requirements

### REQ-core-085: Requirements without tests

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A39, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A120, docs/decision/records/2026-09-17-scenario-tests.md#A4, docs/decision/records/2026-09-17-scenario-tests.md#A9, docs/decision/records/2026-09-17-scenario-tests.md#A15, docs/decision/records/2026-09-17-scenario-tests.md#A16, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A4
- verification: unit

When a `requirement` whose verification is other than "review" and which is not under `deferral` has neither a `mark` containing its `ID` nor a `mark` containing the `ID` of a `scenario` whose "@about" has that `ID`, kotowari emits a requirement_without_test `error`. When a `scenario` with the same `ID` is in two or more places, the "@about" is that of the first `scenario` per REQ-core-032. For a `requirement` with no "- verification:" line, kotowari emits only verification_missing and does not emit requirement_without_test. It is not emitted for a `requirement` under `deferral`.

### REQ-core-086: Tests without marks

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A24, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-24-multi-language-tests.md#A31, docs/decision/records/2026-09-24-multi-language-tests.md#A42
- verification: unit

When a `test` in a `language with a query` has no `mark`, kotowari emits a test_without_id `error` with the name of that `test` as the detail. If the name is null, the detail is the whole text of the first line of the `test`'s node with leading and trailing whitespace removed.

### REQ-core-087: Correspondence for languages without a query

- kind: event_driven
- source: docs/decision/records/records.md#A24, docs/decision/records/records.md#A39, docs/decision/adr/0002-tree-sitter.md#結果, docs/decision/records/records.md#A51, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-17-scenario-tests.md#A10
- verification: unit

When reading a `test file` of a `language without a query`, kotowari counts each `mark` it picks up toward clearing requirement_without_test and scenario_without_test, and does not emit test_without_id.

### REQ-core-137: Examples without tests

- kind: event_driven
- source: docs/decision/records/2026-09-17-scenario-tests.md#A2, docs/decision/records/2026-09-17-scenario-tests.md#A3, docs/decision/records/2026-09-17-scenario-tests.md#A6, docs/decision/records/2026-09-17-scenario-tests.md#A8, docs/decision/records/2026-09-17-scenario-tests.md#A11, docs/decision/records/2026-09-17-scenario-tests.md#A14, docs/decision/records/2026-09-17-scenario-tests.md#A15, docs/decision/records/2026-09-17-scenario-tests.md#A16, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A3, docs/decision/records/2026-09-25-deferred-items.md#A4
- verification: unit

For a `scenario` where, among the "@about" entries whose `ID` resolves as a `requirement`, at least one has a verification value other than "review" and is not under `deferral`, when there is no `mark` containing that `scenario`'s `ID`, kotowari emits a scenario_without_test `error` with that `ID` as the detail. It is not emitted for a `scenario` whose "@about" has no `requirement`, nor for a `scenario` in which every `requirement` of "@about" is "review" or under `deferral`. A `requirement` with no "- verification:" line and a `requirement` whose verification value is not one of the four are not counted. It is not emitted for a `scenario` without "@id" nor for an invalid_id `scenario`. When a `scenario` with the same `ID` is in two or more places, the "@about" of the first `scenario` per REQ-core-032 is used, and the `error` is emitted only once, on the tag line of the first `scenario`. Counts are not considered: it does not matter whether the same `ID` is named by more than one `mark` or one `mark` names more than one `ID`.

### REQ-core-088: When the IR has no documents

- kind: event_driven
- source: docs/decision/records/records.md#A41, docs/decision/records/records.md#A51
- verification: unit

When the `IR` has no documents, kotowari reports 0 `finding` items for the check of the `IR` and performs the check of correspondence with each `test`.

## Examples

```gherkin
@id=EX-core-019 @about=REQ-core-088 @source=docs/decision/records/records.md#A51,docs/decision/records/records.md#A29,docs/decision/records/records.md#A26
Scenario: Even with an empty IR, tests without marks are reported
  Given there are no documents in the location of the `IR`
  And there is one function with "#[test]" and no `mark`
  When "kotowari check" is run
  Then one test_without_id error is emitted
  And no requirement_without_test error is emitted
  And the exit code is 1

@id=EX-core-121 @about=REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A2,docs/decision/records/2026-09-17-scenario-tests.md#A3,docs/decision/records/2026-09-17-scenario-tests.md#A6,docs/decision/records/2026-09-17-scenario-tests.md#A4
Scenario: An example not named by any mark is an error
  Given "docs/ir/a.md" has "REQ-001" with verification "unit" and the scenario "@id=EX-201 @about=REQ-001", there is a test with the mark "@kotowari[REQ-001]", but no test has a mark containing "EX-201"
  When "kotowari check" is run
  Then a scenario_without_test error is emitted whose "line" is that tag line and whose detail is "EX-201"

@id=EX-core-122 @about=REQ-core-085,REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A4,docs/decision/records/2026-09-17-scenario-tests.md#A9,docs/decision/records/2026-09-17-scenario-tests.md#A2,docs/decision/records/2026-09-17-scenario-tests.md#A6,docs/decision/records/records.md#A26
Scenario: The mark of an example also covers the requirement
  Given "docs/ir/a.md" has "REQ-001" with verification "unit" and the scenario "@id=EX-201 @about=REQ-001", a function with "#[test]" has the mark "@kotowari[EX-201]", and there is no mark containing "REQ-001"
  When "kotowari check" is run
  Then no requirement_without_test error with detail "REQ-001" is emitted, and no scenario_without_test error with detail "EX-201" is emitted either

@id=EX-core-123 @about=REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A3,docs/decision/records/2026-09-17-scenario-tests.md#A8
Scenario: An example of only review requirements is not required to have a test
  Given the verification of "REQ-002" is "review", there is the scenario "@id=EX-202 @about=REQ-002", and no test has a mark containing "EX-202"
  When "kotowari check" is run
  Then no scenario_without_test error is emitted

@id=EX-core-124 @about=REQ-core-087 @source=docs/decision/records/2026-09-17-scenario-tests.md#A10,docs/decision/records/2026-09-17-scenario-tests.md#A6,docs/decision/records/records.md#A24,docs/decision/records/records.md#A39,docs/decision/records/records.md#A36,docs/decision/records/records.md#A47,docs/decision/records/2026-09-24-multi-language-tests.md#A7,docs/decision/records/2026-09-24-multi-language-tests.md#A8
Scenario: Marks of examples in files of languages without a query are counted too
  Given "tests.files" includes "tests/**/*.go", "docs/ir/a.md" has "REQ-001" with verification "unit" and the scenario "@id=EX-201 @about=REQ-001", "tests/a.go" contains "@kotowari[EX-201]", there is no `query` for ".go", and no Rust test has a mark containing "EX-201"
  When "kotowari check" is run
  Then no scenario_without_test error with detail "EX-201" is emitted

@id=EX-core-125 @about=REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A8,docs/decision/records/2026-09-17-scenario-tests.md#A14
Scenario: An example that names no requirement is not required to have a test
  Given "docs/ir/a.md" has the decision table "TBL-001" and the scenario "@id=EX-203 @about=TBL-001", and no test has a mark containing "EX-203"
  When "kotowari check" is run
  Then no scenario_without_test error is emitted

@id=EX-core-310 @about=REQ-core-086 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A31,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: A test without a name uses its first line as the detail
  Given "tests.files" includes "tests/**/*.ts", a rule with "language: typescript" in "tests.rules" matches "bench($$$)" and does not capture "$NAME", and "tests/a.test.ts" has an unmarked call starting with "  bench(caseName, () => {"
  When "kotowari check" is run
  Then one test_without_id error with detail "bench(caseName, () => {" is emitted
```
