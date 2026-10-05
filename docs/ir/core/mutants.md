# Checking mutation outcomes

English | [日本語](mutants.ja.md)

Covers how "kotowari mutants" turns each `miss` and each timeout among the `mutation outcome` entries into a `finding` and outputs the counts. Reading the results file is covered in mutants-input.md, and how the `list of equivalents` is read in equivalents.md.

## Requirements

### REQ-core-139: The finding for a miss

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A7, docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A50
- verification: unit

When, on "kotowari mutants", the outcome of a `mutation outcome` is missed and it matches no entry of the `list of equivalents`, kotowari outputs a mutant_survived `error` with "path" set to the file of that `mutation outcome`, "line" to its line, and detail to the change description. For a `mutation outcome` whose outcome is caught or unviable, it outputs no `finding` and does not look for a match with the `list of equivalents`. Even when two or more `mutation outcome` entries have the same content, it does not fold them and outputs one `finding` for each.

### REQ-core-140: The finding for a timeout

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A37, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A50
- verification: unit

When, on "kotowari mutants", the outcome of a `mutation outcome` is timed out, kotowari outputs a mutant_timeout `notice` with "path" set to the file of that `mutation outcome`, "line" to its line, and detail to the change description. It does not look for a match with the `list of equivalents`.

### REQ-core-145: Counting mutations

- kind: algorithm
- source: docs/decision/records/2026-09-17-mutation-tests.md#A28, docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A38, docs/decision/records/2026-09-17-mutation-tests.md#A50, docs/decision/records/2026-09-17-mutation-tests.md#A55
- definition: TBL-core-025, PROP-core-005
- verification: unit

### REQ-core-146: Text output of the counts

- kind: event_driven
- source: docs/decision/records/2026-09-17-mutation-tests.md#A28, docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A38
- verification: unit

When, on "kotowari mutants", "--format" is "text", kotowari outputs the counts as the last line, after the lines of each `finding`, in the form "mutants: caught=count survived=count timeout=count unviable=count equivalent=count". It outputs it even when there is no `finding`.

### REQ-core-147: What it reads

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-mutation-tests.md#A8, docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A47, docs/decision/records/2026-09-17-mutation-tests.md#A55
- verification: unit

kotowari always, on "kotowari mutants", reads only the `configuration file`, the results file, the `list of equivalents`, the files that each `mutation outcome` points to and the files that the "file" of each entry of the `list of equivalents` points to; it does not read the `IR`, any `test file`, any `decision record` or any ADR, does not perform the checks of "kotowari check", and does not make a `stop` even when what "ir", "decisions.records" or "decisions.adr" points to does not exist.

### REQ-core-150: No words specific to the tool

- kind: prohibition
- source: docs/decision/records/2026-09-17-mutation-tests.md#A12, docs/decision/records/2026-09-17-mutation-tests.md#A13, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A54
- verification: review
- how_to_verify: Confirm that the spellings of the tool's result values (CaughtMutant, MissedMutant) do not appear in crates/kotowari-core/src/mutants.rs and crates/kotowari-core/src/lib.rs, which build the finding kind names (mutant_survived, mutant_timeout, equivalent_stale, equivalent_invalid) and detail, by checking that rg -n 'CaughtMutant|MissedMutant' crates/kotowari-core/src/mutants.rs crates/kotowari-core/src/lib.rs prints nothing. crates/kotowari-core/src/cargo_mutants.rs, which maps the tool's values, is out of scope

kotowari shall not use words specific to a mutation-testing tool in the kind or the form of the detail of a `finding`. The change description inside detail is the text as the tool produced it and is not subject to this.

## Decision tables

### TBL-core-025: The JSON of "kotowari mutants"

- source: docs/decision/records/2026-09-17-mutation-tests.md#A35, docs/decision/records/2026-09-17-mutation-tests.md#A38, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A55

The top level of the JSON of "kotowari mutants" has only the three keys findings, counts and mutants.

| Level | Key | Content |
|---|---|---|
| top level | findings | the list of findings. Keys per TBL-core-006, order per TBL-core-007 |
| top level | counts | the number of findings per kind. A kind with none is not included |
| top level | mutants | an object with the five keys below |
| inside "mutants" | caught | the number of mutation outcomes whose outcome is caught |
| inside "mutants" | survived | the number of mutation outcomes whose outcome is missed and that match no entry of the list of equivalents |
| inside "mutants" | timeout | the number of mutation outcomes whose outcome is timed out |
| inside "mutants" | unviable | the number of mutation outcomes whose outcome is unviable |
| inside "mutants" | equivalent | the number of mutation outcomes whose outcome is missed and that match one or more entries of the list of equivalents |

## Properties

### PROP-core-005: The total of the counts

- source: docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A38, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A50

The sum of the five values of "mutants" equals the number of `mutation outcome` entries, "survived" equals the number of mutant_survived `finding` entries, and "timeout" equals the number of mutant_timeout `finding` entries.

## Examples

```gherkin
@id=EX-core-204 @about=REQ-core-138,REQ-core-139,TBL-core-024 @source=docs/decision/records/2026-09-17-mutation-tests.md#A2,docs/decision/records/2026-09-17-mutation-tests.md#A7,docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A40,docs/decision/records/2026-09-17-mutation-tests.md#A42,docs/decision/records/2026-09-17-mutation-tests.md#A50,docs/decision/records/2026-09-17-mutation-tests.md#A16
Scenario: A miss becomes an error
  Given the "outcomes" of the results file has a baseline run whose "summary" is "Success", and one entry with "file" "src/a.rs", "line" 3 and "column" 5 in "span.start", "name" "src/a.rs:3:5: replace f with ()" and "summary" "MissedMutant"
  And the configuration has no key for the `list of equivalents`
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then a mutant_survived error is output with "path" "src/a.rs", "line" 3 and detail "replace f with ()"
  And the exit code is 1

@id=EX-core-205 @about=REQ-core-139,REQ-core-145 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38
Scenario: Nothing is output for a caught mutation and an unviable mutation
  Given the results file has one entry whose "summary" is "CaughtMutant" and one whose "summary" is "Unviable"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then "findings" is empty, and the "caught" of "mutants" is 1 and "unviable" is 1
  And the exit code is 0

@id=EX-core-206 @about=REQ-core-140 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A37,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: A timeout is a notice and is not removed by writing it in the list
  Given the results file has an entry on line 3 of "src/a.rs" whose "summary" is "Timeout", and the `list of equivalents` has an entry whose "file", "change" and "text" fit that `mutation`, whose "class" is "equivalent" and whose "why" is not empty
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then a mutant_timeout notice is output with "path" "src/a.rs" and "line" 3
  And the exit code is 0

@id=EX-core-209 @about=REQ-core-145,REQ-core-146,PROP-core-005 @source=docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38
Scenario: Results with no mutations give exit code 0 and all counts 0
  Given the "outcomes" of the results file has only one baseline run entry
  When "kotowari mutants --tool cargo-mutants --format text outcomes.json" is run
  Then standard output is only the one line "mutants: caught=0 survived=0 timeout=0 unviable=0 equivalent=0"
  And the exit code is 0

@id=EX-core-210 @about=REQ-core-147 @source=docs/decision/records/2026-09-17-mutation-tests.md#A8,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A55
Scenario: Results can be read even without the IR location
  Given what "ir" of the configuration points to does not exist, and the results file has an entry whose "summary" is "CaughtMutant"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then the exit code is 0

@id=EX-core-229 @about=REQ-core-139,PROP-core-005 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A50
Scenario: Two identical mutations give two findings
  Given the results file has two entries with the same file, line and "name" whose "summary" is "MissedMutant"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then two mutant_survived errors are output, and the "survived" of "mutants" is 2

@id=EX-core-230 @about=REQ-core-146 @source=docs/decision/records/2026-09-17-mutation-tests.md#A11,docs/decision/records/2026-09-17-mutation-tests.md#A31,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A38,docs/decision/records/2026-09-17-mutation-tests.md#A40
Scenario: The counts are the last line, after the finding lines
  Given the results file has only one `miss`, on line 3 of "src/a.rs", whose change description is "replace f with ()"
  When "kotowari mutants --tool cargo-mutants --format text outcomes.json" is run
  Then the first line of standard output is "src/a.rs:3 [error] mutant_survived replace f with ()"
  And the last line of standard output is "mutants: caught=0 survived=1 timeout=0 unviable=0 equivalent=0"

@id=EX-core-231 @about=REQ-core-147,TBL-core-025 @source=docs/decision/records/2026-09-17-mutation-tests.md#A8,docs/decision/records/2026-09-17-mutation-tests.md#A35,docs/decision/records/2026-09-17-mutation-tests.md#A55
Scenario: mutants outputs no finding for a test without a mark, and the JSON has only three keys
  Given a function with "#[test]" has no `mark`, and the results file has an entry whose "summary" is "CaughtMutant"
  When "kotowari mutants --tool cargo-mutants outcomes.json" is run
  Then "findings" is empty, and the top-level keys of the JSON are only the three "findings", "counts" and "mutants"
```
