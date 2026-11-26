# Reading the results of mutation tests

English | [日本語](mutants-input.ja.md)

Covers how "kotowari mutants" maps the results file of a mutation-testing tool onto each `mutation outcome`, and the `stop` on a results file that cannot be mapped. Each `finding` and the counts are covered in mutants.md.

## Requirements

### REQ-core-138: Mapping the tool's results onto mutation outcomes

- kind: algorithm
- source: docs/decision/records/2026-09-17-mutation-tests.md#A1, docs/decision/records/2026-09-17-mutation-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A13, docs/decision/records/2026-09-17-mutation-tests.md#A14, docs/decision/records/2026-09-17-mutation-tests.md#A42, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A56
- definition: TBL-core-024
- verification: unit

### REQ-core-144: Results errors

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A32, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A42, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A51
- verification: unit

When the results file cannot be read as JSON, when a key that TBL-core-024 maps from is missing or has the wrong type, when one result matches no row of TBL-core-024, when the result of the baseline run is not "baseline run succeeded" of TBL-core-024, when a line is less than 1, when the prefix of the change description is not in the form of TBL-core-024, or when a file is an absolute path or contains a ".." component, kotowari makes a `stop` with a results error as the reason. It does not make a `stop` for a results file that has no baseline run at all. It does not look at keys not listed in TBL-core-024. When the results file is missing, unreadable or not UTF-8, it makes a `stop` with an unreadable file or a non-UTF-8 file as the reason.

## Decision tables

### TBL-core-024: How the results of cargo-mutants are mapped

- source: docs/decision/records/2026-09-17-mutation-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A13, docs/decision/records/2026-09-17-mutation-tests.md#A33, docs/decision/records/2026-09-17-mutation-tests.md#A42, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A51, docs/decision/records/2026-09-17-mutation-tests.md#A56

When "--tool" is "cargo-mutants", the results file is JSON with a list under the top-level key "outcomes", and each entry of the list is mapped as follows. An entry whose "scenario" is the string "Baseline" is the baseline run and does not become a mutation outcome. Every other entry has the mutation in "Mutant" under "scenario".

| Field of the mutation outcome | Mapped from |
|---|---|
| file | "scenario.Mutant.file" (a string). Read as a path relative to the base directory, with the normalization of REQ-core-110 applied |
| line | "scenario.Mutant.span.start.line" (a number) |
| change description | the rest of "scenario.Mutant.name" (a string) after removing from its start the prefix made by joining, in this order, the value of "scenario.Mutant.file", ":", the value of "span.start.line", ":", the value of "span.start.column" and ": " |
| outcome caught | "summary" is "CaughtMutant" |
| outcome missed | "summary" is "MissedMutant" |
| outcome timed out | "summary" is "Timeout" |
| outcome unviable | "summary" is "Unviable" |
| baseline run succeeded | the "summary" of the baseline run entry is "Success" |

## Examples

```gherkin
@id=EX-core-207 @about=REQ-core-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A42
Scenario: An unknown result value stops it
  Given the results file has an entry whose "summary" is "Flaky"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "results error: "

@id=EX-core-208 @about=REQ-core-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A42
Scenario: Results whose baseline run failed stop it
  Given in the results file, the "summary" of the entry whose "scenario" is "Baseline" is "Failure"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "results error: "

@id=EX-core-222 @about=REQ-core-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39
Scenario: Results that cannot be read as JSON stop it
  Given the content of the results file is only "{"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "results error: "

@id=EX-core-223 @about=REQ-core-144,TBL-core-024 @source=docs/decision/records/2026-09-17-mutation-tests.md#A32,docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: Results where a required key has the wrong type stop it
  Given in the results file, the "span.start.line" of one mutation entry is the string "3"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "results error: "

@id=EX-core-224 @about=REQ-core-144,TBL-core-024 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: Results whose name prefix does not fit the form stop it
  Given in the results file, one mutation entry has "file" "src/a.rs", "line" 3 and "column" 5 in "span.start", and "name" "replace f with ()"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "results error: "

@id=EX-core-225 @about=REQ-core-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: Results with line 0 stop it
  Given in the results file, the "span.start.line" of one mutation entry is 0
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "results error: "

@id=EX-core-226 @about=REQ-core-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A39,docs/decision/records/2026-09-17-mutation-tests.md#A51
Scenario: Results pointing outside the base directory stop it
  Given in the results file, the "file" of one mutation entry is "../x/src/a.rs"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 2
  And the first line of standard error starts with "results error: "

@id=EX-core-227 @about=REQ-core-144 @source=docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: Results without a baseline run and with an unknown key can still be read
  Given the "outcomes" of the results file has no baseline run, and an entry whose "summary" is "CaughtMutant" has a key "extra" not in TBL-core-024
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 0, and the "caught" of "mutants" is 1
```
