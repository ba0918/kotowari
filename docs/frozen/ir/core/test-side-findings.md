# Test-side findings

English | [日本語](test-side-findings.ja.md)

Covers the range of `test-side finding` that can be left out of the exit code of "kotowari check" when a specification is committed ahead of its tests, the behaviour of that option, and the guidance on putting it into hooks. Where the option is accepted and the table of exit codes are covered by cli.md.

## Requirements

### REQ-core-357: Allowing test-side findings

- kind: event_driven
- source: docs/decision/records/2026-10-06-spec-first-commit.md#A1, docs/decision/records/2026-10-06-spec-first-commit.md#A2, docs/decision/records/2026-10-06-spec-first-commit.md#A4, docs/decision/records/2026-10-06-spec-first-commit.md#A6
- definition: TBL-core-047
- verification: unit

When "--allow-test-findings" is given to "kotowari check", kotowari will leave every `error` that TBL-core-047 makes a `test-side finding` out of the judgement of the exit code, and will output every `finding`, `test-side finding` included, with the same kind, severity and order as without the option.

### REQ-core-358: Guidance on putting check into hooks

- kind: ubiquitous
- source: docs/decision/records/2026-10-06-spec-first-commit.md#A7
- verification: review
- how_to_verify: Read "agent/skills/kotowari/references/findings.md", "agent/skills/kotowari-adopt/" and "docs/guides/commands/check.md", and confirm that all three say that the pre-commit hook runs "kotowari check" with "--allow-test-findings" and the pre-push hook and CI run it without

The kotowari skills and the guide to check always describe running "kotowari check" with "--allow-test-findings" in the pre-commit hook and without it in the pre-push hook and in CI.

## Decision tables

### TBL-core-047: Range of test-side findings

- source: docs/decision/records/2026-10-06-spec-first-commit.md#A2, docs/decision/records/2026-10-06-spec-first-commit.md#A3, docs/decision/records/2026-10-06-spec-first-commit.md#A9

An `error` of a kind not in this table is not a `test-side finding`.

| Kind | Condition for being a `test-side finding` |
|---|---|
| requirement_without_test | Always |
| scenario_without_test | Always |
| test_without_id | Always |
| invalid_marker | When "path" is a `test file` |
| unresolved_reference | When "path" is a `test file` |
| unparsable_file | When "path" is a `test file` and not a `surface file` |

## Examples

```gherkin
@id=EX-core-547 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A1,docs/decision/records/2026-10-06-spec-first-commit.md#A2,docs/decision/records/2026-10-06-spec-first-commit.md#A4
Scenario: A specification written ahead of its tests passes with the option
  Given there is a requirement "REQ-greet-001" with "verification: unit" and no test whose mark contains its ID
  And there is no other error
  When "kotowari check --allow-test-findings" is run
  Then the exit code is 0
  And the output has one requirement_without_test finding whose severity is "error"

@id=EX-core-548 @about=REQ-core-357,TBL-core-002 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A1
Scenario: The same state stops without the option
  Given there is a requirement "REQ-greet-001" with "verification: unit" and no test whose mark contains its ID
  When "kotowari check" is run
  Then the exit code is 1

@id=EX-core-549 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A2
Scenario: An error in the IR stops even with the option
  Given there is a requirement with no test, and a line of an IR document encloses in backticks a word not in the glossary
  When "kotowari check --allow-test-findings" is run
  Then the exit code is 1

@id=EX-core-550 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A3
Scenario: An unreadable test file that is also a surface file stops
  Given "src/lib.rs", matched by both "tests.files" and "surface.files", has a syntax error
  When "kotowari check --allow-test-findings" is run
  Then the exit code is 1
  And the output has an unparsable_file finding whose path is "src/lib.rs"

@id=EX-core-551 @about=REQ-core-357,TBL-core-047 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A2
Scenario: A malformed guide mark stops even with the option
  Given a guide has a guide mark with nothing inside, and there is no other error
  When "kotowari check --allow-test-findings" is run
  Then the exit code is 1

@id=EX-core-552 @about=REQ-core-004,REQ-core-357 @source=docs/decision/records/2026-10-06-spec-first-commit.md#A5,docs/decision/records/2026-10-06-spec-first-commit.md#A12,docs/decision/records/records.md#A60,docs/decision/records/records.md#A20
Scenario: Giving it to a command other than check stops
  Given there is an IR that can be checked
  When "kotowari status --allow-test-findings" is run
  Then the exit code is 2
  And the reason for the stop is an argument error
```
