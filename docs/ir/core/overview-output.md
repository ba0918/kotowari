# Reporting overview data

English | [日本語](overview-output.ja.md)

Covers how the results of checking the `overview data` are added to the output of "kotowari check" and "kotowari status". The check itself is covered in overview-data.md.

## Requirements

### REQ-core-288: The overview group of check

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A64, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A78
- verification: unit

kotowari always, in the top-level "overview" of the JSON of "kotowari check", outputs the number of files of `overview data` read as "files" and the number of entries of each well-formed `guide mark` in the `overview data` as "marks". They are counted the same way as "guides" (REQ-core-206), and both are output as 0 even when the `configuration file` has no "overview" key. It does not output this when "--format" is "text".

### REQ-core-289: The overview group of status

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A64
- verification: unit

kotowari always outputs in "kotowari status" the same "overview" group ("files" and "marks") as "kotowari check".

### REQ-core-290: Findings on the data join the findings of check

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A33, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-02-whole-picture.md#A50, docs/decision/records/2026-10-02-whole-picture.md#A64, docs/decision/records/2026-10-03-public-crate-api.md#A33, docs/decision/records/2026-10-04-overview-on-public-api.md#A2
- verification: unit

kotowari always adds each `finding` on the `overview data` to the `finding` entries of "kotowari check" and "kotowari status", outputs them together with the other `finding` entries in the order of REQ-core-024, and counts them in "counts", the exit code and "complete" the same way as the other `finding` entries. The kotowari library passes each `finding` on the `overview data` and the "overview" group to core's check as a group of additional findings, and core orders and counts them together with the other `finding` entries.

## Examples

```gherkin
@id=EX-core-472 @about=REQ-core-288,REQ-core-290 @source=docs/decision/records/2026-10-02-whole-picture.md#A64,docs/decision/records/2026-10-02-whole-picture.md#A50,docs/decision/records/2026-10-02-whole-picture.md#A38,docs/decision/records/2026-10-02-whole-picture.md#A69
Scenario: Errors in the data go into the findings and counts of check, and the exit code is 1
  Given two `overview data` files are matched by "overview.files", and the opening of one is not a lead
  When "kotowari check --format json" is run
  Then the exit code is 1, the "files" of "overview" is 2, "findings" has one overview_lead_missing, and overview_lead_missing in "counts" is 1

@id=EX-core-473 @about=REQ-core-289,REQ-core-290 @source=docs/decision/records/2026-10-02-whole-picture.md#A64,docs/decision/records/2026-10-02-whole-picture.md#A38,docs/decision/records/2026-10-02-whole-picture.md#A78
Scenario: With errors in the data, status is not complete
  Given there is an `IR` with no other `error`, and `overview data` whose opening is not a lead
  When "kotowari status --format json" is run
  Then there is an "overview" group, "complete" is false, and the exit code is 1
```
