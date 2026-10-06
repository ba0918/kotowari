# Commands and exit

English | [日本語](cli.ja.md)

Covers kotowari's commands, the arguments they accept, stopping, and exit codes.

## Requirements

### REQ-core-001: Eight commands

- kind: ubiquitous
- source: docs/decision/records/records.md#A19, docs/decision/records/2026-09-17-mutation-tests.md#A8, docs/decision/records/2026-09-17-mutation-tests.md#A41, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-19-read-commands.md#A1, docs/decision/records/2026-09-19-read-commands.md#A10, docs/decision/records/2026-09-20-query-status.md#A1, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-10-01-change-conformance.md#A2, docs/decision/records/2026-10-02-whole-picture.md#A23, docs/decision/records/2026-10-02-whole-picture.md#A27
- verification: unit

kotowari always has only the eight commands "kotowari check", "kotowari list", "kotowari mutants", "kotowari plan", "kotowari query", "kotowari status", "kotowari changes" and "kotowari overview", and the single command "kotowari check" performs both the check of the `IR` and the check of its correspondence with the `test`. The check for missing and stale conformance records against changes is done by "kotowari changes" (REQ-core-240). "kotowari overview" has two subcommands, "build" and "serve" (REQ-core-293, REQ-core-297).

### REQ-core-002: Accepted options

- kind: ubiquitous
- source: docs/decision/records/records.md#A19, docs/decision/records/records.md#A103, docs/decision/records/2026-09-17-mutation-tests.md#A14, docs/decision/records/2026-09-17-mutation-tests.md#A41, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A58, docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-09-24-plan-schema.md#A16, docs/decision/records/2026-09-24-guide-gaps.md#A3, docs/decision/records/2026-10-02-whole-picture.md#A62, docs/decision/records/2026-10-02-whole-picture.md#A63, docs/decision/records/2026-10-02-whole-picture.md#A78, docs/decision/records/2026-10-06-spec-first-commit.md#A5, docs/decision/records/2026-10-06-spec-first-commit.md#A6, docs/decision/records/2026-10-06-spec-first-commit.md#A12
- verification: unit

kotowari always accepts only "--format", "--config", "--help" and "--version" as options for "list", "query" and "status"; accepts "--allow-test-findings", which takes no value, in addition to those for "check"; accepts "--tool" in addition to those for "mutants"; accepts only "--format", "--help" and "--version" for "plan"; accepts only "--format", "--config", "--help" and "--version" for "overview build"; accepts only "--port", "--config", "--help" and "--version" for "overview serve"; and, for every command, accepts an option written either before or after the command, regardless of the order of positional arguments and options.

### REQ-core-003: Base of the configuration path

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A3, docs/decision/records/records.md#A60
- verification: unit

kotowari always reads the path given to "--config" as a path relative to the current directory, except for changes. "changes" reads the configuration inside the target snapshot by a path relative to the Git root (REQ-core-265).

### REQ-core-004: Argument errors

- kind: event_driven
- source: docs/decision/records/2026-10-01-change-details.md#A3, docs/decision/records/records.md#A60, docs/decision/records/records.md#A103, docs/decision/records/records.md#A136, docs/decision/records/2026-09-17-mutation-tests.md#A41, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-09-24-plan-schema.md#A15, docs/decision/records/2026-10-01-change-conformance.md#A2, docs/decision/records/2026-10-02-whole-picture.md#A27, docs/decision/records/2026-10-02-whole-picture.md#A63, docs/decision/records/2026-10-06-spec-first-commit.md#A5, docs/decision/records/2026-10-06-spec-first-commit.md#A12
- verification: unit

When neither "--help" nor "--version" is given and kotowari receives any of the following: an unknown option, "--tool" on a command other than "mutants", "--port" on a command other than "overview serve", "--format" on "overview serve", "--allow-test-findings" on a command other than "check", a first positional argument that is none of "check", "list", "mutants", "plan", "query", "status", "changes" and "overview", a positional argument after "check", "list" or "status", an unknown value of "--format", an option without a value, or a second occurrence of the same option; or when there are no arguments at all; or when there are only options and no first positional argument; or when, other than for "changes", the target of "--config" does not exist or is a directory, kotowari will `stop` on the grounds of an argument error.

### REQ-core-149: Arguments of mutants

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A14, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A41
- verification: unit

In "kotowari mutants" with neither "--help" nor "--version", when "--tool" is absent, when the value of "--tool" is not "cargo-mutants", or when there is not exactly one positional argument after "mutants", kotowari will `stop` on the grounds of an argument error. The positional argument is the path of the outcomes file, read as a path relative to the current directory.

### REQ-core-304: Arguments of overview

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A27, docs/decision/records/2026-10-02-whole-picture.md#A57, docs/decision/records/2026-10-02-whole-picture.md#A63, docs/decision/records/2026-10-02-whole-picture.md#A72
- verification: unit

In "kotowari overview" with neither "--help" nor "--version", when there is not exactly one positional argument after "overview", when it is neither "build" nor "serve", or when the value of "--port" is not a decimal integer from 1 to 65535, kotowari will `stop` on the grounds of an argument error.

### REQ-core-005: Output on stop

- kind: event_driven
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A104, docs/decision/records/records.md#A137
- verification: unit

When it will `stop`, kotowari writes nothing to standard output and writes the reason for the stop to standard error. The first line of standard error is the wording of TBL-core-018 followed by ": " and the detail; the detail is as in TBL-core-020, any path it contains is relative to the `base directory`, and the wording is written in English.

### REQ-core-006: Reasons for stopping

- kind: algorithm
- source: docs/decision/records/records.md#A44, docs/decision/records/records.md#A48, docs/decision/records/records.md#A60, docs/decision/records/records.md#A104
- definition: TBL-core-001, TBL-core-018, TBL-core-020
- verification: unit

### REQ-core-007: Exit codes

- kind: algorithm
- source: docs/decision/records/records.md#A20, docs/decision/records/records.md#A29
- definition: TBL-core-002
- verification: unit

### REQ-core-008: Commands not built

- kind: prohibition
- source: docs/decision/records/records.md#P1, docs/decision/records/records.md#A99, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A23, docs/decision/records/2026-09-19-read-commands.md#A2, docs/decision/records/2026-09-20-query-status.md#A1, docs/decision/records/2026-10-02-whole-picture.md#A60, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

kotowari shall not generate human-facing documents that copy their structure and content from the `IR`. The `overview` renders `overview data` written by an LLM, and opening and showing on the spot the body of the `item` a `part`'s reference points to (REQ-core-291) does not count as this.

## Decision tables

### TBL-core-001: Reasons for stopping

- source: docs/decision/records/records.md#A20, docs/decision/records/records.md#A44, docs/decision/records/records.md#A48, docs/decision/records/records.md#A60, docs/decision/records/records.md#A12, docs/decision/records/records.md#A41, docs/decision/records/records.md#A66, docs/decision/records/records.md#A95, docs/decision/records/records.md#A96, docs/decision/records/records.md#A93, docs/decision/records/records.md#A103, docs/decision/records/records.md#A105, docs/decision/records/records.md#A135, docs/decision/records/records.md#A136, docs/decision/records/records.md#A146, docs/decision/records/records.md#A160, docs/decision/records/2026-09-16-ir-tree.md#A16, docs/decision/records/2026-09-17-mutation-tests.md#A32, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A49, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-22-ir-engine.md#A73, docs/decision/records/2026-09-24-plan-schema.md#A16, docs/decision/records/2026-09-24-doc-marks.md#A15, docs/decision/records/2026-09-24-doc-marks.md#A16, docs/decision/records/2026-09-24-guide-gaps.md#A2, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-09-27-surface-check.md#A29, docs/decision/records/2026-10-02-whole-picture.md#A32, docs/decision/records/2026-10-02-whole-picture.md#A57, docs/decision/records/2026-10-02-whole-picture.md#A58, docs/decision/records/2026-10-02-whole-picture.md#A61, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-02-whole-picture.md#A76, docs/decision/records/2026-10-04-overview-on-public-api.md#A9, docs/decision/records/2026-10-05-overview-index.md#A11, docs/decision/records/2026-10-05-overview-index.md#A17, docs/decision/records/2026-10-05-overview-index.md#A32

| Reason | Situation |
|---|---|
| Configuration error | The situation of REQ-core-014; the situation of an error in "tests.rules" in REQ-core-189; the situation of an error in the surface configuration and "surface.rules" in REQ-core-225; the situation in REQ-core-231 where the list of unspecified surfaces cannot be read as YAML or its top level is not a sequence; the situation in REQ-core-148 where the list of equivalents cannot be read as YAML or its top level is not a sequence; the situation in REQ-core-199 where the locations of guides and tests overlap; the situations of the `overview data` in REQ-core-279 and REQ-core-280; the situation where the "overview" key has no "overview.toc"; and the situation of the `table of contents` in REQ-core-326 |
| Argument error | The situations of REQ-core-004, REQ-core-149, REQ-core-190 and REQ-core-304 |
| Unreadable file | The `plan` file of "kotowari plan" does not exist, is a directory, or cannot be read (REQ-core-197); a file to be read cannot be read (excluding, in "kotowari mutants", a file pointed to by one entry of the mutation outcomes or of the list of equivalents; REQ-core-141, REQ-core-142); the outcomes file does not exist or cannot be read; the target of "mutants.equivalents" does not exist or cannot be read; in "kotowari check" or "kotowari status", the target of "surface.unspecified" does not exist or cannot be read; the target of "overview.toc" does not exist or cannot be read (REQ-core-325); in "kotowari check", "kotowari overview build" or "kotowari overview serve", a directory pointed to by "ir", "decisions.records" or "decisions.adr" does not exist or cannot be read; a directory under "ir", "decisions.records" or "decisions.adr" cannot be read; a directory cannot be read while scanning any of "tests.files", "guides.files", "surface.files" or "overview.files"; a scan meets a symbolic link with no target; or the current directory cannot be obtained |
| File that is not UTF-8 | Any of an IR document, a test file, a guide, a surface file (what REQ-core-236 reads), the list of unspecified surfaces, the configuration file, a decision record, an ADR, the outcomes file, the list of equivalents, a plan, the `overview data` or the `table of contents` is not UTF-8 |
| Outcome error | The situation of REQ-core-144 |
| Copy error | The situation of REQ-core-175 |
| Source data error | The situation of REQ-core-294 |
| Port error | The situation of REQ-core-298 |
| Location error | The situation of REQ-core-324 |

### TBL-core-002: Exit codes

- source: docs/decision/records/records.md#A20, docs/decision/records/records.md#A29, docs/decision/records/records.md#A103, docs/decision/records/2026-09-16-notice.md#A2, docs/decision/records/2026-10-06-spec-first-commit.md#A1, docs/decision/records/2026-10-06-spec-first-commit.md#A10

| Exit code | Situation |
|---|---|
| 0 | There are no errors (including when there are only notices), every error of "check" with "--allow-test-findings" is a `test-side finding`, or the run ended with "--help" or "--version" |
| 1 | There is one or more errors (for "check" with "--allow-test-findings", one or more errors that are not a `test-side finding`) |
| 2 | The run stopped |

## Examples

```gherkin
@id=EX-core-001 @about=REQ-core-004,REQ-core-005 @source=docs/decision/records/records.md#A60,docs/decision/records/records.md#A40
Scenario: An unknown option stops the run
  Given there is an IR that can be checked
  When "kotowari check --verbose" is run
  Then the exit code is 2
  And nothing is written to standard output
  And the reason for the stop is written to standard error

@id=EX-core-218 @about=REQ-core-149 @source=docs/decision/records/2026-09-17-mutation-tests.md#A14,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A41
Scenario: mutants without a tool specified stops
  Given there is a readable outcomes file "outcomes.json"
  When "kotowari mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "argument error: "

@id=EX-core-219 @about=REQ-core-004,TBL-core-020 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-19-read-commands.md#A10,docs/decision/records/2026-09-19-read-commands.md#A20,docs/decision/records/2026-09-20-query-status.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A10,docs/decision/records/2026-09-24-plan-schema.md#A30,docs/decision/records/2026-10-01-change-conformance.md#A2
Scenario: With no arguments the commands are listed
  When "kotowari" is run without arguments
  Then the exit code is 2
  And the reason on standard error lists the eight command names of REQ-core-001

@id=EX-core-240 @about=REQ-core-149 @source=docs/decision/records/2026-09-17-mutation-tests.md#A14,docs/decision/records/2026-09-17-mutation-tests.md#A39
Scenario: An unknown tool name stops the run
  Given there is a readable outcomes file "a.json"
  When "kotowari mutants --tool stryker a.json" is run
  Then the exit code is 2 and the first line of standard error starts with "argument error: "

@id=EX-core-242 @about=REQ-core-149 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A41
Scenario: Passing two outcomes files stops the run
  Given there are readable outcomes files "a.json" and "b.json"
  When "kotowari mutants --tool cargo-mutants a.json b.json" is run
  Then the exit code is 2 and the first line of standard error starts with "argument error: "

@id=EX-core-241 @about=REQ-core-004,TBL-core-020 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A55,docs/decision/records/2026-09-19-read-commands.md#A10,docs/decision/records/2026-09-19-read-commands.md#A20,docs/decision/records/2026-09-20-query-status.md#A18,docs/decision/records/2026-09-24-plan-schema.md#A10,docs/decision/records/2026-09-24-plan-schema.md#A30,docs/decision/records/2026-10-01-change-conformance.md#A2
Scenario: A run with only options lists the commands and stops
  When "kotowari --format text" is run
  Then the exit code is 2
  And the reason on standard error lists the eight command names of REQ-core-001

@id=EX-core-244 @about=REQ-core-002 @source=docs/decision/records/2026-09-17-mutation-tests.md#A41,docs/decision/records/2026-09-17-mutation-tests.md#A58
Scenario: The options of mutants can be written before the command or after the outcomes path
  Given the outcomes file "outcomes.json" has one entry whose "summary" is "CaughtMutant"
  When "kotowari --tool cargo-mutants mutants outcomes.json --format text" is run
  Then the exit code is 0
@id=EX-core-380 @about=REQ-core-002 @source=docs/decision/records/2026-09-24-guide-gaps.md#A3,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-20-query-status.md#A7
Scenario: Options can be written after the ID in query
  Given the `IR` has "REQ-001"
  When "kotowari query REQ-001 --format text" is run
  Then the exit code is 0
```
