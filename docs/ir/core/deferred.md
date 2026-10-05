# Deferral

English | [日本語](deferred.ja.md)

Covers how a declaration that puts a `requirement` under `deferral` is written and read, the `notice` findings emitted for mismatches between a `deferral` and the `mark` of a `test` or a reference, and writing the handling of `deferral` into the skills. Exclusion from the check for missing tests is covered by coverage.md, and counting by status.md and list.md.

## Requirements

### REQ-core-208: Declaring a deferral

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A10, docs/decision/records/2026-09-25-deferred-items.md#A18, docs/decision/records/2026-09-25-deferred-items.md#A24, docs/decision/records/2026-09-25-deferred-items.md#A25
- verification: unit

kotowari always treats as under `deferral` a `requirement` that has a "- deferred:" line under its heading, and every `requirement` of a `topic document` that has a "- deferred:" line (a document-level declaration) after the `title` and before the first "## " or "### ". When a `requirement` with the same `ID` is in two or more places, whether it is under `deferral` is decided by the first `requirement` per REQ-core-032. It is treated as under `deferral` even if the value of the "- deferred:" line is empty or wrong as a `source`, and no `finding` is emitted even if both lines are present.

### REQ-core-209: The document-level declaration line

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A10, docs/decision/records/2026-09-25-deferred-items.md#A19, docs/decision/records/2026-09-25-deferred-items.md#A24, docs/decision/records/2026-09-25-deferred-items.md#A26
- verification: unit

kotowari always reads a "- deferred:" line located after the `title` of a `topic document` and before the first "## " or "### " as a document-level declaration, and does not count it as a line of the `scope`. If there are two or more document-level "- deferred:" lines, it emits one duplicate_field `error` on the second line and reads only the value of the first line. For a document-level declaration in a document with no `requirement`, it emits no `finding` on the grounds that there are no requirements, and performs the source check (REQ-core-210) and duplicate_field the same as for a document with a `requirement`. For a "- deferred:" line in the same position in a `glossary` or `flag record` document, it emits an unknown_field `error` as before.

### REQ-core-210: Sources of a deferral

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A23
- verification: unit

kotowari always reads the value of a "- deferred:" line as a comma-separated sequence of `source` entries, and checks it by the same rules as the "- source:" line (REQ-core-057, REQ-core-058). For a "- deferred:" line with an empty value, it emits a missing_source `error` with that line as "line" and "deferred" as the detail.

### REQ-core-211: A test mark despite deferral

- kind: event_driven
- source: docs/decision/records/2026-09-25-deferred-items.md#A11, docs/decision/records/2026-09-25-deferred-items.md#A15, docs/decision/records/2026-09-25-deferred-items.md#A23, docs/decision/records/2026-09-25-deferred-items.md#A24
- verification: unit

When at least one `mark` contains the `ID` of a `requirement` under `deferral` or the `ID` of a `deferred scenario`, kotowari emits, once per such `ID`, a deferred_with_test `notice` whose "line" is the heading line of that `requirement` (for a `scenario`, its tag line; when the same `ID` is in two or more places, the first one per REQ-core-032) and whose detail is that `ID`. A `mark` picked up from a `test file` of a `language without a query` is counted too.

### REQ-core-212: Depending on a deferral

- kind: event_driven
- source: docs/decision/records/2026-09-25-deferred-items.md#A12, docs/decision/records/2026-09-25-deferred-items.md#A15, docs/decision/records/2026-09-25-deferred-items.md#A17, docs/decision/records/2026-09-25-deferred-items.md#A21, docs/decision/records/2026-09-25-deferred-items.md#A22, docs/decision/records/2026-09-25-deferred-items.md#A23
- verification: unit

When a `requirement` or `property` not under `deferral` points to a `requirement` under `deferral` through its "- definition:" line or through a backticked `ID` in a `statement`, and when a `scenario` that is not a `deferred scenario` points to a `requirement` under `deferral` through its "@about" tag or through a backticked `ID` in a step, kotowari emits, for each reference, a depends_on_deferred `notice` whose "line" is the line where the reference is written and whose detail is the referring `ID` and the referenced `ID` separated by one half-width space. It is not emitted for references from a `requirement` under `deferral` or from a `deferred scenario`, nor for references to a `deferred scenario`. A backticked `ID` is determined the same way as in REQ-core-054.

### REQ-core-213: Handling of deferral in the skills

- kind: ubiquitous
- source: docs/decision/records/2026-09-25-deferred-items.md#A13, docs/decision/records/2026-09-25-deferred-items.md#A23
- verification: review
- how_to_verify: Read the skills under "agent/skills/" and confirm that ir-form.md in the kotowari skill's references lists the "- deferred:" line (per requirement and document-level) and findings.md lists deferred_with_test and depends_on_deferred; that kotowari-plan says it does not include the `ID` of a `requirement` under `deferral` in the plan; that kotowari-cycle and kotowari-review say they do not include `deferral` among the targets whose test-side `finding` count must reach 0; and that kotowari-brainstorm says that when deferring, it writes the decision giving the reason into the `decision record` and cites it as the source

The kotowari skills under "agent/skills/" always describe how to write a `deferral` and how a `deferral` is handled in planning, the implementation loop, review, and brainstorming.

## Examples

```gherkin
@id=EX-core-384 @about=REQ-core-208,REQ-core-210 @source=docs/decision/records/2026-09-25-deferred-items.md#A1,docs/decision/records/2026-09-25-deferred-items.md#A2,docs/decision/records/2026-09-25-deferred-items.md#A4
Scenario: A per-requirement declaration removes the missing-test error
  Given "docs/ir/a.md" has a requirement "REQ-001" with verification "unit", with the line "- deferred: docs/decision/records/r.md#A1" under its heading, "r.md" has the decision "A1", and no test has a mark containing "REQ-001"
  When "kotowari check" is run
  Then no requirement_without_test with detail "REQ-001" is emitted, and no source_invalid either

@id=EX-core-385 @about=REQ-core-208,REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A1,docs/decision/records/2026-09-25-deferred-items.md#A3,docs/decision/records/2026-09-25-deferred-items.md#A19,docs/decision/records/2026-09-25-deferred-items.md#A4
Scenario: A document-level declaration defers every requirement of the document
  Given "docs/ir/a.md" has, after its title, a scope line and the line "- deferred: docs/decision/records/r.md#A1", requirements "REQ-001" and "REQ-002" with verification "unit", and the scenario "@id=EX-001 @about=REQ-001", and there is no mark containing any of their IDs
  When "kotowari check" is run
  Then none of requirement_without_test, scenario_without_test, unknown_field and missing_scope is emitted

@id=EX-core-386 @about=REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A19
Scenario: A declaration line alone is not a scope
  Given "docs/ir/a.md" has only the line "- deferred: docs/decision/records/r.md#A1" between its title and the first "## "
  When "kotowari check" is run
  Then a missing_scope error is emitted

@id=EX-core-387 @about=REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A19,docs/decision/records/2026-09-25-deferred-items.md#A26
Scenario: Duplicate document-level declarations are an error
  Given "docs/ir/a.md" has two "- deferred:" lines among its scope lines
  When "kotowari check" is run
  Then one duplicate_field error with detail "deferred" is emitted on the second line

@id=EX-core-388 @about=REQ-core-209 @source=docs/decision/records/2026-09-25-deferred-items.md#A10
Scenario: A declaration in a glossary is an unknown line
  Given "docs/ir/CONTEXT.md" has the line "- deferred: docs/decision/records/r.md#A1" after its title
  When "kotowari check" is run
  Then an unknown_field error is emitted on that line

@id=EX-core-389 @about=REQ-core-210,REQ-core-208 @source=docs/decision/records/2026-09-25-deferred-items.md#A2,docs/decision/records/2026-09-25-deferred-items.md#A18,docs/decision/records/2026-09-25-deferred-items.md#A23,docs/decision/records/2026-09-25-deferred-items.md#A4
Scenario: A declaration with an empty value is a source error, and the requirement stays deferred
  Given there is a "- deferred:" line with an empty value under the heading of the requirement "REQ-001" with verification "unit", and there is no mark containing "REQ-001"
  When "kotowari check" is run
  Then a missing_source error with detail "deferred" is emitted on that line, and requirement_without_test is not emitted

@id=EX-core-390 @about=REQ-core-210 @source=docs/decision/records/2026-09-25-deferred-items.md#A2,docs/decision/records/2026-09-25-deferred-items.md#A24
Scenario: A declaration whose source target does not exist is an error
  Given there is the line "- deferred: docs/decision/records/r.md#A9" under the heading of the requirement "REQ-001", and "r.md" has no decision "A9"
  When "kotowari check" is run
  Then a source_invalid error is emitted on that line

@id=EX-core-391 @about=REQ-core-208 @source=docs/decision/records/2026-09-25-deferred-items.md#A1,docs/decision/records/2026-09-25-deferred-items.md#A18,docs/decision/records/2026-09-25-deferred-items.md#A25
Scenario: Form errors and duplicate IDs are errors even under deferral
  Given "docs/ir/a.md", which has a document-level declaration, has a requirement "REQ-001" without a "- verification:" line, and "docs/ir/b.md" also has "REQ-001"
  When "kotowari check" is run
  Then verification_missing and duplicate_id errors are emitted

@id=EX-core-392 @about=REQ-core-211 @source=docs/decision/records/2026-09-25-deferred-items.md#A11
Scenario: A mark pointing to a deferred requirement is a notice
  Given there are a deferred requirement "REQ-001" and a test with the mark "@kotowari[REQ-001]"
  When "kotowari check" is run
  Then one deferred_with_test notice with detail "REQ-001" is emitted on the heading line of "REQ-001"

@id=EX-core-393 @about=REQ-core-211 @source=docs/decision/records/2026-09-25-deferred-items.md#A11,docs/decision/records/2026-09-25-deferred-items.md#A15
Scenario: A mark pointing to a deferred scenario is a notice
  Given there are a deferred requirement "REQ-001" and the scenario "@id=EX-001 @about=REQ-001", and a test with the mark "@kotowari[EX-001]"
  When "kotowari check" is run
  Then one deferred_with_test notice with detail "EX-001" is emitted on that tag line

@id=EX-core-394 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A21
Scenario: A statement of a non-deferred requirement pointing to a deferred requirement is a notice
  Given there are a deferred requirement "REQ-001" and a non-deferred requirement "REQ-002" that points to "REQ-001" in backticks within a `statement`
  When "kotowari check" is run
  Then one depends_on_deferred notice with detail "REQ-002 REQ-001" is emitted on the line of that `statement`

@id=EX-core-395 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A15,docs/decision/records/2026-09-25-deferred-items.md#A21,docs/decision/records/2026-09-17-scenario-tests.md#A2,docs/decision/records/2026-09-17-scenario-tests.md#A6
Scenario: A scenario pointing to both deferred and non-deferred requirements gets a notice and a test error
  Given there are a deferred requirement "REQ-001", a non-deferred requirement "REQ-002" with verification "unit", and the scenario "@id=EX-001 @about=REQ-001,REQ-002", and there is no mark containing "EX-001"
  When "kotowari check" is run
  Then a depends_on_deferred notice with detail "EX-001 REQ-001" is emitted on that tag line, and a scenario_without_test error with detail "EX-001" is emitted as well

@id=EX-core-396 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A22
Scenario: References from the deferred side are not reported
  Given a `statement` of the deferred requirement "REQ-001" points to the non-deferred requirement "REQ-002" in backticks, and a `statement` of the deferred requirement "REQ-003" points to "REQ-001" in backticks
  When "kotowari check" is run
  Then no depends_on_deferred notice is emitted

@id=EX-core-397 @about=REQ-core-212 @source=docs/decision/records/2026-09-25-deferred-items.md#A17,docs/decision/records/2026-09-25-deferred-items.md#A21,docs/decision/records/2026-09-25-deferred-items.md#A12,docs/decision/records/2026-09-25-deferred-items.md#A23
Scenario: References in steps of a non-deferred scenario are notices too
  Given there are a deferred requirement "REQ-001" and a non-deferred scenario "EX-002" with "@about=REQ-002", and one line of its steps contains "REQ-001" in backticks twice
  When "kotowari check" is run
  Then two depends_on_deferred notices with detail "EX-002 REQ-001" are emitted on that step line
```
