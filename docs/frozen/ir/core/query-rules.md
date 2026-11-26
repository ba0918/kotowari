# Queries added by configuration

English | [日本語](query-rules.ja.md)

Covers the form and reading of the queries added by the configuration's "tests.rules", their relation to the bundled queries, and errors in rule files.

## Requirements

### REQ-core-121: The bundled queries cannot be removed

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A2, docs/decision/records/2026-09-24-multi-language-tests.md#A11
- verification: unit

kotowari always uses the bundled `query` entries regardless of the configuration, and adds the `query` entries of the "tests.rules" files to them.

### REQ-core-186: Rule files

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A9, docs/decision/records/2026-09-24-multi-language-tests.md#A10, docs/decision/records/2026-09-24-multi-language-tests.md#A28
- verification: unit

kotowari always reads the paths listed in "tests.rules" as paths relative to the `base directory`, reads their content as ast-grep rule YAML (several rules may be listed, separated by "---"), and adds each rule to the `query` entries of the language of its "language".

### REQ-core-187: Files a rule is applied to

- kind: event_driven
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A29, docs/decision/records/2026-09-24-multi-language-tests.md#A45
- verification: unit

When a rule has "files" or "ignores", kotowari applies those globs, read the same way as ast-grep reads them, to the path of each `test file` relative to the `base directory`, and does not apply the rule to a `test file` that does not match "files" or that matches "ignores".

### REQ-core-188: Fields that are not used

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A29
- verification: unit

kotowari always leaves a rule's "fix", "message", "severity", "note" and "metadata" unused in finding a `test`, and applies a rule whose "severity" is "off" too.

### REQ-core-189: Errors in rule files

- kind: event_driven
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A22, docs/decision/records/2026-09-24-multi-language-tests.md#A43, docs/decision/records/2026-09-24-multi-language-tests.md#A44, docs/decision/records/2026-09-24-guide-gaps.md#A2, docs/decision/records/2026-09-24-guide-gaps.md#A7
- verification: unit

When the file at a "tests.rules" path is missing, when the path is not a file, when it cannot be read, when it is not UTF-8, when the same path is listed twice, when it cannot be read as YAML, when it cannot be read as ast-grep rules, or when a rule's "language" is not among the languages of TBL-core-031, kotowari makes a `stop` with a configuration error as the reason. "language" is matched without regard to case, as ast-grep does, and ast-grep's aliases ("ts", "py") are accepted too. Overlapping rule "id" values are not looked at.

## Examples

```gherkin
@id=EX-core-311 @about=REQ-core-186 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A9,docs/decision/records/2026-09-24-multi-language-tests.md#A10,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: Tests are counted with an added rule
  Given "tests.files" includes "tests/**/*.ts", "tests.rules" is only "rules/bench.yml", that file has a rule with "language: typescript" that matches "bench($NAME, $$$)", and "tests/a.test.ts" has "bench('fast', () => {})" without a mark
  When "kotowari check" is run
  Then one error of test_without_id with detail "fast" is raised

@id=EX-core-312 @about=REQ-core-121,REQ-core-181 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A11,docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: A node matched by both a bundled rule and an added one counts as one
  Given "tests.files" includes "tests/**/*.ts", a rule with "language: typescript" in "tests.rules" matches "it($NAME, $$$)", and "tests/a.test.ts" has one "it('x', () => {})" without a mark
  When "kotowari check" is run
  Then only one error of test_without_id with detail "x" is raised

@id=EX-core-313 @about=REQ-core-187 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A29,docs/decision/records/2026-09-24-multi-language-tests.md#A18
Scenario: A rule is not applied to a file that does not match files
  Given "tests.files" includes "tests/**/*.ts", a rule with "language: typescript" in "tests.rules" has "**/*.spec.ts" in "files" and matches "bench($NAME, $$$)", and "tests/a.test.ts" has "bench('fast', () => {})" without a mark
  When "kotowari check" is run
  Then no error of test_without_id is raised

@id=EX-core-314 @about=REQ-core-188 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A29,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: A rule whose severity is off is applied too
  Given "tests.files" includes "tests/**/*.ts", a rule with "language: typescript" in "tests.rules" has "severity: off" and matches "bench($NAME, $$$)", and "tests/a.test.ts" has "bench('fast', () => {})" without a mark
  When "kotowari check" is run
  Then one error of test_without_id with detail "fast" is raised

@id=EX-core-315 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A22
Scenario: A missing rule file stops
  Given "tests.rules" has "rules/missing.yml", and that file does not exist
  When "kotowari check" is run
  Then the exit code is 2

@id=EX-core-316 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A22,docs/decision/records/2026-09-24-multi-language-tests.md#A6
Scenario: A rule of an unknown language stops
  Given a rule in a "tests.rules" file has "language: cobol"
  When "kotowari check" is run
  Then the exit code is 2
@id=EX-core-321 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A43
Scenario: The same rule file listed twice stops
  Given "tests.rules" lists "rules/bench.yml" twice
  When "kotowari check" is run
  Then the exit code is 2

@id=EX-core-322 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A44,docs/decision/records/2026-09-24-multi-language-tests.md#A9,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: An alias of language is accepted
  Given "tests.files" includes "tests/**/*.ts", a rule in "tests.rules" has "language: ts" and matches "bench($NAME, $$$)", and "tests/a.test.ts" has "bench('fast', () => {})" without a mark
  When "kotowari check" is run
  Then the exit code is 1 and one error of test_without_id with detail "fast" is raised
@id=EX-core-378 @about=REQ-core-189,TBL-core-020 @source=docs/decision/records/2026-09-24-guide-gaps.md#A2,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-24-multi-language-tests.md#A22,docs/decision/records/2026-09-24-multi-language-tests.md#A43,docs/decision/records/2026-09-24-multi-language-tests.md#A44
Scenario: When stopping on a missing rule file, the detail points at the rule file
  Given "tests.rules" lists "rules/missing.yml", and that file does not exist
  When "kotowari check" is run
  Then the exit code is 2, and standard error starts with "config error: ", contains "rules/missing.yml" and does not contain ".kotowari/config.yaml"

@id=EX-core-379 @about=REQ-core-189 @source=docs/decision/records/2026-09-24-guide-gaps.md#A7,docs/decision/records/2026-09-24-guide-gaps.md#A2,docs/decision/records/ir-form.md#出力
Scenario: When stopping on a rule of an unknown language, the language is shown
  Given the "language" of a rule in the "tests.rules" rule file "r.yml" is "cobol"
  When "kotowari check" is run
  Then the exit code is 2, and standard error is one line containing "r.yml" and "unknown language: cobol"
```
