# Checking a plan

English | [日本語](plan.ja.md)

Covers how `kotowari plan` reads one plan that describes the steps of an implementation, and checks its form against the schema bundled in the binary. The order of findings and how line is counted are decided in [Order and lines of findings](finding-order.md), the wording of the reasons for a stop in [The reason for a stop is shown with fixed English wording](cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording), the exit codes in [The exit code is 0 when there is no error, 1 when there is an error, and 2 when kotowari stops](cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops), the text form in [--format text outputs one finding per line](output.md#--format-text-outputs-one-finding-per-line), and the keys of a finding in JSON in [A finding is output with the five keys kind, severity, path, line and detail](output.md#a-finding-is-output-with-the-five-keys-kind-severity-path-line-and-detail).

## kotowari plan checks the form of a plan against the schema bundled in the binary

Status: implemented

### Problem

The form of the plan that the workflow writes was set by a template, but the only check was the writer comparing it with the template and self-reviewing in prose; there was no machine check. kotowari has an engine that checks the form of Markdown against a schema, yet it did not check the products of its own workflow with its own tool.

The schema is bundled in the binary because this can be explained as a feature that kotowari provides. The workflow already uses kotowari commands, so the binary can be assumed to be present, and whether the engine is installed separately need not be asked. A plan is deleted once it is complete, so it is always written against the schema of the binary at hand on the user's side, and there is almost no occasion to check a past plan again.

The check was made a separate command instead of being put into check because check takes no positional arguments, and for a command that takes and reads one file there is the precedent of mutants ([Mutation tests run outside kotowari, and kotowari mutants reads their results file and turns it into findings](mutants.md#mutation-tests-run-outside-kotowari-and-kotowari-mutants-reads-their-results-file-and-turns-it-into-findings)).

### Decision

The form of a plan is checked against the schema of the plan that the kotowari binary took in at compile time. No schema file is read at run time. A plan does not point at a schema itself, and the schema is not copied into the project. When the check fails, the plan is fixed to fit the schema of the binary.

The check is done with `kotowari plan <path>`. The positional argument is one plan file.

The frontmatter at the top of a plan is skipped unread, whatever its content. It is not used as a declaration of form (even if it has a `$schema` key, that key is not used), and it becomes neither a finding nor a stop.

### Rejected options

- Previous decision: Each time a plan is written, copy the schema held by the workflow skill into the project's `.kotowari/schemas/plan.yaml`, overwriting it, and point at it with a relative path in `$schema` of the plan's frontmatter. The copying step and the committed files increase, and the handling of environments without the engine is needed separately, so this was dropped.
- Point at the schema by URL. The cache has no expiry, so the old schema keeps being used even after the skill is updated.
- Add a setting for the place of plans to check, and check them together with the specification. It runs automatically in pre-commit, but findings on plans would affect the completion judgement of status, and it would get entangled with the rule of handling only that round's findings, spreading too far.

### Confirmations

- C-core-plan-1 Even if `$schema` in the frontmatter points at a file that does not exist, `kotowari plan` does not use it, checks against the bundled schema, and ends with exit code 0
- C-core-plan-2 Even if the frontmatter at the top cannot be read as YAML, `kotowari plan` skips it unread and outputs neither a finding nor a stop

## kotowari plan reads no configuration, and handles arguments and unreadable files in the same way as mutants

Status: implemented

### Problem

`kotowari plan` only checks the form of one plan against the bundled schema and does not use configuration. If it accepted configuration and silently ignored it, the person who gave it would mistakenly think it was in effect. If the handling of arguments and unreadable files follows the precedent of the command that takes and reads one file (mutants), nothing more needs to be decided.

### Decision

The only options `kotowari plan` takes are `--format`, `--help` and `--version`. If it receives `--config`, it stops with an argument error. It does not read the configuration file.

If there is not exactly one positional argument, it stops with an argument error. The path in the positional argument is read as a path relative to the current directory. When the file is missing, is a directory, or cannot be read, it stops with an unreadable file as the reason, and when it is not UTF-8, with a non-UTF-8 file as the reason. No new reason for a stop is created for plan.

### Confirmations

- C-core-plan-3 `kotowari plan` with no positional argument and `kotowari plan` given two plans stop with exit code 2, and the first line of standard error starts with "argument error: "
- C-core-plan-4 Giving `--config` to `kotowari plan` stops with an argument error whether what it points at is a file or a directory
- C-core-plan-5 Even if there is a configuration file that cannot be read as YAML, `kotowari plan` checks the plan without stopping
- C-core-plan-6 If the plan is missing or is a directory, the first line of standard error starts with "unreadable file: ", and if it is not UTF-8, with "non-UTF-8 file: "; both stop with exit code 2

## A plan is written with fixed plan-wide sections and steps that have eight fields

Status: implemented

### Problem

What the schema checks is only the form of a plan. Whether the content of a field is good (whether Done when is observable by someone else, for example) cannot be decided by machine, so it is left to self-review and review. Whether the specification IDs a plan points at exist is checked with `kotowari query` ([If the positional argument is not exactly one ID, or that ID does not exist, it is a stop with an argument error](query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error)).

The template's earlier form (repeated step headings, labels in running text, the leading word of Shown by) could not be checked with the current engine. The engine matches sections by exact name, reads as fields only list lines of the form "name: value", and its allow-list of values matches the whole value. The user decided that bringing the plan's format to the engine is cheaper than widening a generic engine for the sake of plans. The number of required sections and of step fields is constrained so that the form keeps each part of the plan from being missing or overlapping.

### Decision

A plan places the following plan-wide sections after the title. "## Goal", "## Specification", "## Approach and why", "## Scope of change", "## Step order and prerequisites", "## Verification map", "## Left to the implementer", "## Stop conditions", "## Out of scope" and "## Steps" appear exactly once each, and "## Test command" appears zero times or once.

The "## Steps" section has one or more steps. A step heading has the form "### S" followed by one or more digits and then ":". Under each step, the lines of the following fields are written once each, in this order.

| Order | Field |
|---|---|
| 1 | Purpose |
| 2 | Specification |
| 3 | Prerequisites |
| 4 | May change |
| 5 | Done when |
| 6 | Shown by |
| 7 | Left to the implementer |
| 8 | Stop and hand back if |

A field line is a list line of the form "name: value", and the list marker may be any of "-", "*" and "+". The value of "Shown by" starts with one of the words "test", "check", "artifact" and "external", and after that word comes whitespace or the end of the value.

### Rejected options

- Add to the engine regular expressions for section names and a check of section order. The engine is a generic one, and kotowari's needs are not brought into it.

### Confirmations

- C-core-plan-7 A plan that has all the required sections and, under "## Steps", a step with the eight fields in order gets no finding, and the exit code is 0
- C-core-plan-8 A plan in which any of the required sections is missing or appears twice gets an invalid_plan error. A plan without a "## Test command" section gets no error, and a plan with two of them gets an error
- C-core-plan-9 A plan whose "## Steps" section has no step gets an error
- C-core-plan-10 If a step heading is not in the form of "### S", digits and ":" (for example "### Step 1: …", or a heading without ":"), it is an error
- C-core-plan-11 It is an error if any of a step's fields is missing, if the same field appears twice, or if the fields are in a different order
- C-core-plan-12 It is an error if the value of "Shown by" starts with a word that is none of the four words, or if the word is followed by a character that is not whitespace; a value that is only one of the four words, or in which the word is followed by whitespace, is not an error
- C-core-plan-13 Using "*" or "+" as the list marker of a field line is not an error

## Under a step only field lines are placed, and plan-wide sections can hold statements, lists, tables and code blocks

Status: implemented

### Problem

Plan-wide sections should be written in a way that fits their content: the Verification map as a table, Approach as prose. On the other hand, a step is a contract read by the implementer, so its form should be fixed.

If field values could be written over several lines, then when read as a paragraph, a line that follows a field directly without a blank line would be absorbed into the field's value, and an indented line after a blank line would not become a finding, as a continuation paragraph. Then the rule that no other line is placed under a step would not hold.

Numbered lists cannot be declared, because the engine has no kind of rule for them. A summary can be written in Goal, so no line is placed between the title and the first section either.

### Decision

Under a step only the lines of the eight fields are placed, and any other non-blank line is an error. A field value is written on one line, and the plan is read line by line.

Under a plan-wide section, statements, unnumbered lists and their child lists, tables and code blocks may be placed. Numbered lists, and headings of "### " and deeper, may not be placed. No non-blank line may be placed between the title and the first "## " section.

### Confirmations

- C-core-plan-14 It is an error if running text follows a step's field line directly without a blank line, if an indented line is placed after a blank line, or if a field line with an unknown name is placed
- C-core-plan-15 Placing a table, a code block, or a bulleted list with child lists in a plan-wide section is not an error
- C-core-plan-16 A numbered list in a plan-wide section, a "### " or "#### " heading inside a plan-wide section, and running text between the title and the first section are each an error

## The order of sections, the step numbers and the names in step headings are not checked

Status: implemented

### Problem

The engine has no mechanism for checking the order of sections, and the engine is not widened for that. Gaps or duplicates in step numbers are a matter of content, and there is no reason to check them as form. Checking the name after the ":" of a step heading would mean widening the engine, which goes against the direction decided of bringing the format to the engine.

### Decision

The order of the plan-wide sections, whether the step numbers are consecutive, whether two steps have the same number, and the name after the ":" of a step heading (whether it is empty, whether there is whitespace after the ":") are not checked.

### Confirmations

- C-core-plan-17 A plan in which the "## Goal" section comes after the "## Out of scope" section and the step numbers jump from S1 to S3 is not an error
- C-core-plan-18 A plan with two steps of the same number, and a plan with no name after the ":" of a step heading, are not errors

## Findings on the form are gathered into the single kind invalid_plan, with the schema side's kind and details in detail

Status: implemented

### Problem

If the finding kinds the engine outputs were listed as they are as kinds of kotowari, the list of finding kinds, and what the tests that compare it against the binary cover, would grow. It is an LLM that fixes the plan, and being able to read the details is enough to know what is wrong.

Which line a finding points at had not been decided. Where a step with a missing field is pointed at, and how it is output in text, were decided to fit the current behaviour as observed. The way path is written is aligned with the details of a stop on the configuration file (including "../" when outside the base).

### Decision

When a plan departs from the form, each finding the schema side reports is output as a finding of kind invalid_plan with severity "error". The schema side's findings are not discarded, and are not mapped to other kinds of kotowari. The exit code is 1 if there is an error.

- detail is the string of the schema side's kind and details joined by ": ".
- line uses the line the schema side reported as it is. When a step lacks a required field, it is the line of that step's heading. How lines are counted is as in [line is counted from 1, and a finding that covers the whole document is null](finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null).
- path is the path of the plan file relative to the base directory. If it is outside the base, it includes "../". Even when it is passed in a form containing "./" or "..", or as an absolute path, it is normalized and output in the same form.

### Confirmations

- C-core-plan-19 For a plan with a step whose field is missing, every finding output has kind invalid_plan and severity "error", detail is the schema side's kind followed by ": " and the details, and the exit code is 1
- C-core-plan-20 The line of invalid_plan is the line at which the schema side reported the finding, and for a step with a missing field, it is the line of that step's heading
- C-core-plan-21 The path of invalid_plan is normalized to a path relative to the base directory even when passed with "./" or "..", and for a plan outside the base it starts with "../"

## The output of kotowari plan uses the same finding keys as check, and the JSON has only findings and counts

Status: implemented

### Problem

Of the counts in the JSON of check, files, lines and tests are amounts of specification and test files read, and do not apply to a single plan. The text output should have the same form as the text of check.

### Decision

With `--format json`, the top level has only the two keys "findings" and "counts". The keys of a finding in "findings" are the same as for check: kind, severity, path, line and detail.

With `--format text`, findings are output one per line in the same form as the text of check, and no other line is output. If there are zero findings, nothing is output.

### Confirmations

- C-core-plan-22 The top-level keys of `kotowari plan --format json` are only the two "findings" and "counts", and the keys of a finding are the five kind, severity, path, line and detail
- C-core-plan-23 `kotowari plan --format text` outputs findings one per line, as lines starting with "<path>:<line> [error] invalid_plan ", outputs no other line, and outputs nothing if there are zero findings
