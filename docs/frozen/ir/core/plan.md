# Checking a plan

English | [日本語](plan.ja.md)

Covers how "kotowari plan" reads one `plan` and checks its form against the schema bundled in the binary.

## Requirements

### REQ-core-190: Arguments of plan

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-09-24-plan-schema.md#A16
- verification: unit

When "kotowari plan" has neither "--help" nor "--version" and the positional arguments after "plan" are not exactly one, or when it receives "--config", kotowari makes a `stop` with an argument error as the reason. The positional argument is the path of the `plan` file, read as a path relative to the current directory.

### REQ-core-196: What plan reads

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A9, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-09-24-plan-schema.md#A32
- verification: unit

In "kotowari plan", kotowari always reads only the `plan` file, and reads neither the `configuration file`, the `IR`, any `decision record` nor any `test file`.

### REQ-core-197: When the plan cannot be read

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A16
- verification: unit

In "kotowari plan", when the `plan` file is missing, is a directory, or cannot be read, kotowari makes a `stop` with an unreadable file as the reason, and when the `plan` is not UTF-8, it makes a `stop` with a non-UTF-8 file as the reason.

### REQ-core-191: Reading with the bundled schema

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A9, docs/decision/records/2026-09-24-plan-schema.md#A12, docs/decision/records/2026-09-24-plan-schema.md#A14, docs/decision/records/2026-09-24-plan-schema.md#A27
- verification: unit

In "kotowari plan", kotowari always reads the `plan` with the schema of the `plan` taken in at compile time, does not read a schema file at run time, and skips the frontmatter at the top of the `plan` unread whatever its content, neither using it as a declaration of form nor turning it into a `finding` or a `stop`.

### REQ-core-192: The form of a plan

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-plan-schema.md#A3, docs/decision/records/2026-09-24-plan-schema.md#A7, docs/decision/records/2026-09-24-plan-schema.md#A19, docs/decision/records/2026-09-24-plan-schema.md#A20, docs/decision/records/2026-09-24-plan-schema.md#A21, docs/decision/records/2026-09-24-plan-schema.md#A22, docs/decision/records/2026-09-24-plan-schema.md#A23, docs/decision/records/2026-09-24-plan-schema.md#A24, docs/decision/records/2026-09-24-plan-schema.md#A25, docs/decision/records/2026-09-24-plan-schema.md#A28, docs/decision/records/2026-09-24-plan-schema.md#A29, docs/decision/records/2026-09-24-plan-schema.md#A31, docs/decision/records/2026-09-24-plan-schema.md#A11, docs/decision/records/2026-09-24-plan-schema.md#A10
- verification: unit

In "kotowari plan", kotowari always reads the `plan` line by line, and treats as free of any `error` only a `plan` that satisfies the following form. There is exactly one `title`, and there is no non-blank line between the `title` and the first "## " heading. There is exactly one section each of "## Goal", "## Specification", "## Approach and why", "## Scope of change", "## Step order and prerequisites", "## Verification map", "## Left to the implementer", "## Stop conditions", "## Out of scope" and "## Steps", zero or one "## Test command" section, and no other "## " section. The "## Steps" section has one or more steps, each with a heading that is "### S" followed by one or more digits and then ":", and before the first step there is neither a non-blank line nor any other heading. Under each step there are list lines of the name-and-value form named "Purpose", "Specification", "Prerequisites", "May change", "Done when", "Shown by", "Left to the implementer" and "Stop and hand back if", once each in this order, and no other non-blank line. The list marker may be any of "-", "*" and "+". The value of "Shown by" starts with one of the words "test", "check", "artifact" and "external", and after that word comes whitespace or the end of the value. Under sections other than "## Steps", statements, unnumbered lists and their child lists, tables and code blocks may be placed, while numbered lists and headings of "### " and below may not. The order of the sections, whether the step numbers are consecutive, whether two steps have the same number, and the name after the ":" of a heading are not checked.

### REQ-core-193: Findings on the form

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A11, docs/decision/records/2026-09-24-plan-schema.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A26, docs/decision/records/2026-09-24-plan-schema.md#A33, docs/decision/records/2026-09-24-guide-gaps.md#A4
- verification: unit

In "kotowari plan", when the `plan` departs from the form of REQ-core-192, kotowari does not map the schema side's `finding` entries through TBL-core-030, and for each one raises an `error` of invalid_plan whose "path" is the path of the `plan` file relative to the `base directory` (normalized, and including "../" when outside the base), whose "line" is the line the schema side reported (when a step lacks a required field, the line of that step's heading; null when there is no line), and whose detail is the schema side's kind and detail joined by ": ".

### REQ-core-194: JSON of plan

- kind: event_driven
- source: docs/decision/records/2026-09-24-plan-schema.md#A17, docs/decision/records/2026-09-24-plan-schema.md#A33
- verification: unit

In "kotowari plan", when "--format" is "json", kotowari outputs JSON whose top level has only the two keys "findings" and "counts". The keys of a `finding` in "findings" and the content of "counts" are the same as in "kotowari check".

### REQ-core-207: text of plan

- kind: event_driven
- source: docs/decision/records/2026-09-24-guide-gaps.md#A4
- verification: unit

In "kotowari plan", when "--format" is "text", kotowari outputs each `finding` on one line in the form of REQ-core-025 and outputs no other line. When there are zero `finding` entries, it outputs nothing.

## Examples

```gherkin
@id=EX-core-332 @about=REQ-core-192,REQ-core-193 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: A plan of the right form raises no finding
  Given there is a `plan` "docs/plans/a.md" with all the required sections and, under "## Steps", a step "### S1: 入力を読む" with the eight fields in order
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0 and no `finding` is raised

@id=EX-core-333 @about=REQ-core-192,REQ-core-193 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A26
Scenario: A step missing a field is an error
  Given the current directory is the `base directory`, and there is a `plan` "docs/plans/a.md" whose step "### S1: 入力を読む" has no "- Done when:" line
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1, an `error` of invalid_plan is raised, its "path" is "docs/plans/a.md", and its detail has the form of the schema side's kind followed by ": "

@id=EX-core-334 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A Shown by that does not start with one of the four words is an error
  Given there is a `plan` "docs/plans/a.md" in which the value of a step's "- Shown by:" is "manual — 目で見る"
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-335 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A Shown by whose word continues is an error
  Given there is a `plan` "docs/plans/a.md" in which the value of a step's "- Shown by:" is "tests — 名前"
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-336 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A plan with no step at all is an error
  Given there is a `plan` "docs/plans/a.md" whose "## Steps" section has no heading at all
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-337 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: The Test command section may be absent
  Given there is a `plan` "docs/plans/a.md" that has no "## Test command" section and is otherwise of the right form
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0

@id=EX-core-338 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A plan with a required section twice is an error
  Given there is a `plan` "docs/plans/a.md" with two "## Goal" sections
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-339 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A19,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: A table can be placed in a section of the whole plan
  Given there is a `plan` "docs/plans/a.md" with a table in the "## Verification map" section and otherwise of the right form
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0

@id=EX-core-340 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A19,docs/decision/records/2026-09-24-plan-schema.md#A23,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: Body text under a step is an error
  Given there is a `plan` "docs/plans/a.md" in which, right after the "- Done when:" line of step "### S1: 入力を読む" and with no blank line between, there is a non-blank line of body text
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-341 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A21,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: The order and gaps in numbering are not checked
  Given there is a `plan` "docs/plans/a.md" in which the "## Goal" section comes after the "## Out of scope" section and the steps are "### S1: 入力を読む" and "### S3: 結果を出す"
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0

@id=EX-core-342 @about=REQ-core-190 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: Passing two plans stops
  Given there are `plan` files "a.md" and "b.md" of the right form
  When "kotowari plan a.md b.md" is run
  Then the exit code is 2 and the first line of standard error starts with "argument error: "

@id=EX-core-343 @about=REQ-core-190 @source=docs/decision/records/2026-09-24-plan-schema.md#A15,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: Passing a configuration file to plan stops
  Given there are a `plan` "a.md" of the right form and a `configuration file` ".kotowari/config.yaml"
  When "kotowari plan --config .kotowari/config.yaml a.md" is run
  Then the exit code is 2 and the first line of standard error starts with "argument error: "

@id=EX-core-344 @about=REQ-core-197 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: A missing plan stops as an unreadable file
  Given "docs/plans/none.md" does not exist
  When "kotowari plan docs/plans/none.md" is run
  Then the exit code is 2 and the first line of standard error starts with "unreadable file: "

@id=EX-core-345 @about=REQ-core-191 @source=docs/decision/records/2026-09-24-plan-schema.md#A9,docs/decision/records/2026-09-24-plan-schema.md#A12,docs/decision/records/2026-09-24-plan-schema.md#A27,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: A schema declaration in the frontmatter is not used
  Given there is a `plan` "docs/plans/a.md" that has "$schema: missing.yaml" written in its leading frontmatter and is otherwise of the right form, and "docs/plans/missing.yaml" does not exist
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0

@id=EX-core-346 @about=REQ-core-194 @source=docs/decision/records/2026-09-24-plan-schema.md#A17,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A33,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/records.md#A40,docs/decision/records/2026-09-24-plan-schema.md#A10,docs/decision/records/2026-09-24-plan-schema.md#A15
Scenario: The JSON of plan has only findings and counts
  Given there is a `plan` "docs/plans/a.md" whose step "### S1: 入力を読む" has no "- Done when:" line
  When "kotowari plan docs/plans/a.md --format json" is run
  Then the top-level keys of the JSON are only the two "findings" and "counts", and "invalid_plan" in "counts" equals the number of invalid_plan entries in "findings"

@id=EX-core-347 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A plan missing a required section is an error
  Given there is a `plan` "docs/plans/a.md" with no "## Stop conditions" section
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-348 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A31
Scenario: A plan with an unknown section is an error
  Given there is a `plan` "docs/plans/a.md" with a "## Notes" section
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-349 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A step whose fields are out of order is an error
  Given there is a `plan` "docs/plans/a.md" in which a step's "- Done when:" line comes before its "- Purpose:" line
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-350 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A20,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A step with a field twice is an error
  Given there is a `plan` "docs/plans/a.md" in which a step has two "- Purpose:" lines
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-351 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A22,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A line before the first step is an error
  Given there is a `plan` "docs/plans/a.md" with a non-blank line of body text between the "## Steps" heading and the "### S1: 入力を読む" heading
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-352 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A7,docs/decision/records/2026-09-24-plan-schema.md#A24,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A step heading that does not start with S and digits is an error
  Given there is a `plan` "docs/plans/a.md" whose heading under "## Steps" is "### Step 1: 入力を読む"
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-353 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A25,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A numbered list in a section of the whole plan is an error
  Given there is a `plan` "docs/plans/a.md" with the line "1. S1 を先に行う" in the "## Step order and prerequisites" section
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-354 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A25,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: A child list can be placed in a section of the whole plan
  Given there is a `plan` "docs/plans/a.md" with a bulleted list that has an indented child list in the "## Scope of change" section, and otherwise of the right form
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0

@id=EX-core-355 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A25,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18
Scenario: A line between the title and the first section is an error
  Given there is a `plan` "docs/plans/a.md" with a non-blank line of body text between the `title` and "## Goal"
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 1 and an `error` of invalid_plan is raised

@id=EX-core-356 @about=REQ-core-192 @source=docs/decision/records/2026-09-24-plan-schema.md#A28,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: The list marker of the fields may be an asterisk
  Given there is a `plan` "docs/plans/a.md" whose step field lines are written with "*", as in "* Purpose:", and otherwise of the right form
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0

@id=EX-core-357 @about=REQ-core-190 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: plan without a plan stops
  When "kotowari plan" is run
  Then the exit code is 2 and the first line of standard error starts with "argument error: "

@id=EX-core-358 @about=REQ-core-197 @source=docs/decision/records/2026-09-24-plan-schema.md#A16,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: A plan that is not UTF-8 stops
  Given there is a `plan` "docs/plans/a.md" containing bytes that are not UTF-8
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 2 and the first line of standard error starts with "non-UTF-8 file: "

@id=EX-core-359 @about=REQ-core-196 @source=docs/decision/records/2026-09-24-plan-schema.md#A15,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: plan does not stop even with a broken configuration file
  Given there are a `configuration file` ".kotowari/config.yaml" that cannot be read as YAML and a `plan` "docs/plans/a.md" of the right form
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0

@id=EX-core-360 @about=REQ-core-193 @source=docs/decision/records/2026-09-24-plan-schema.md#A26
Scenario: The path of a plan outside the base includes "../"
  Given outside the `base directory` "work" there is a `plan` "plans/a.md" with a step that has no "- Done when:" line, and the current directory is "work"
  When "kotowari plan ../plans/a.md" is run
  Then the "path" of the `error` of invalid_plan is "../plans/a.md"

@id=EX-core-361 @about=REQ-core-191 @source=docs/decision/records/2026-09-24-plan-schema.md#A27,docs/decision/records/2026-09-24-plan-schema.md#A11
Scenario: A broken frontmatter is skipped unread too
  Given there is a `plan` "docs/plans/a.md" whose leading frontmatter cannot be read as YAML and that is otherwise of the right form
  When "kotowari plan docs/plans/a.md" is run
  Then the exit code is 0
@id=EX-core-381 @about=REQ-core-193,REQ-core-207 @source=docs/decision/records/2026-09-24-guide-gaps.md#A4,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-24-plan-schema.md#A11,docs/decision/records/2026-09-24-plan-schema.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A3
Scenario: A step missing a field points at the heading line
  Given in a `plan` "docs/plans/a.md" otherwise of the right form, the step "### S1: 作る" has no "- Done when:" line
  When "kotowari plan docs/plans/a.md --format text" is run
  Then the exit code is 1, and standard output is only a line that starts with "docs/plans/a.md:" followed by the line number of "### S1: 作る" and " [error] invalid_plan "

@id=EX-core-382 @about=REQ-core-207 @source=docs/decision/records/2026-09-24-guide-gaps.md#A4,docs/decision/records/ir-form.md#出力
Scenario: The text of a plan without findings outputs nothing
  Given there is a `plan` "docs/plans/a.md" of the right form
  When "kotowari plan docs/plans/a.md --format text" is run
  Then the exit code is 0 and standard output is empty
```
