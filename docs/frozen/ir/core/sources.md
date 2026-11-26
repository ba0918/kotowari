# Checking sources

English | [日本語](sources.ja.md)

Covers the format of sources and the check of whether the target of a source exists.

## Requirements

### REQ-core-057: The format of a source

- kind: ubiquitous
- source: docs/decision/records/records.md#A3, docs/decision/records/records.md#A13, docs/decision/records/records.md#A38, docs/decision/records/records.md#A84, docs/decision/records/records.md#A106
- verification: unit

kotowari always reads a `source` only in the form "path#anchor", and splits it into the path and the anchor at the first "#". A path cannot contain "#". The path is relative to the `base directory` and is written in full from the location ("docs/decision/records/records.md#A26" form). The path is compared with the locations after the normalization of REQ-core-110.

### REQ-core-058: Judging a source

- kind: algorithm
- source: docs/decision/records/records.md#A38, docs/decision/records/records.md#A48, docs/decision/records/records.md#A69
- definition: TBL-core-012
- verification: unit

### REQ-core-059: A missing source

- kind: event_driven
- source: docs/decision/records/records.md#A38, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A90
- verification: unit

When an `item` that is a `requirement`, a `decision table`, a `property` or a `flag record` has no source line or an empty one, when a `scenario` has no "@source" tag, or when the source column of a `term` is empty, kotowari raises an `error` of missing_source. For a `scenario` without "@id", the detail is the characters of the "Scenario:" line.

### REQ-core-060: Sources of glossaries and scenarios

- kind: ubiquitous
- source: docs/decision/records/records.md#A52, docs/decision/records/ir-form.md#出典
- verification: unit

kotowari always checks the source column of a `glossary` and the "@source" tag of a `scenario` by the same rules as a source line.

### REQ-core-061: Decision numbers are per file

- kind: ubiquitous
- source: docs/decision/records/records.md#A48, docs/decision/records/records.md#A115
- verification: unit

kotowari always looks for a `decision number` only inside the `decision record` file that the path of the `source` points at. A `decision section` starts at a "## " heading and ends at the next "## " heading; a "### " heading does not end a section.

### REQ-core-062: No comparison of content

- kind: prohibition
- source: docs/decision/records/records.md#A4, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari must not judge whether a `source` really states the content of its `item`.

### REQ-core-106: A source can point at the form contract

- kind: ubiquitous
- source: docs/decision/records/records.md#A52, docs/decision/records/records.md#A69, docs/decision/records/2026-09-16-ir-tree.md#A2
- verification: unit

kotowari always accepts the form contract "docs/decision/records/ir-form.md" as the target of a `source` that points at it by a "## " heading.

### REQ-core-115: The line of a source finding

- kind: ubiquitous
- source: docs/decision/records/records.md#A114, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A24
- verification: unit

kotowari always sets the "line" of source_invalid to the line where the `source` is written (for an `item`, the "- source:" line; for a `scenario`, the tag line; for a `term`, the table row; for a declaration of `deferral`, its "- deferred:" line).

## Decision tables

### TBL-core-012: Judging a source

- source: docs/decision/records/records.md#A38, docs/decision/records/records.md#A48, docs/decision/records/records.md#A69, docs/decision/records/records.md#A91, docs/decision/records/records.md#A115, docs/decision/records/records.md#A134, docs/decision/records/records.md#A158, docs/decision/records/records.md#A165, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A34, docs/decision/records/2026-09-17-record-form.md#A47, docs/decision/records/2026-09-24-review5-gaps.md#A3

When a path is inside both decisions.records and decisions.adr (when one location is under the other), it is judged as a file inside the deeper location.

| Step | Condition | Result |
|---|---|---|
| 1 | Not of the "path#anchor" format | source_invalid |
| 2 | The path is a `decision record` inside decisions.records (a file with one or more headings of a `decision section`), the anchor is of the decision number form (one uppercase English letter followed by one or more digits), and a decision section of that file has a `numbered line` with the anchor's number (the inside of a `code block` is excluded) | valid |
| 3 | The path is a `decision record` inside decisions.records, and 2 does not apply | source_invalid |
| 4 | The path is a Markdown file inside decisions.records or decisions.adr that is not a decision record (an ADR, the form contract, a supplementary document; entries that are neither directories nor regular files are an `exclusion` and are not counted as files), the file exists, and the anchor exactly matches the text of a "## " heading outside a `code block` with surrounding whitespace removed | valid |
| 5 | The path is inside decisions.records or decisions.adr, and none of 2 to 4 applies | source_invalid |
| 6 | The path is inside neither decisions.records nor decisions.adr | source_invalid |

## Examples

```gherkin
@id=EX-core-285 @about=TBL-core-012 @source=docs/decision/records/2026-09-24-review5-gaps.md#A3
Scenario: A file inside both locations is judged by the deeper location
  Given there is a configuration where decisions.records is "docs/decision" and decisions.adr is "docs/decision/adr"
  And there is a `requirement` whose sources are a heading of an ADR under "docs/decision/adr" and a decision of a `decision record` under "docs/decision/records"
  When "kotowari check" is run
  Then no source_invalid is raised

@id=EX-core-011 @about=REQ-core-058 @source=docs/decision/records/records.md#A38
Scenario: A number in a decision section is a valid source
  Given "decisions.records" is "docs/decision/records", and the Agreements section of "docs/decision/records/records.md" has a line starting with "- A26 "
  When the source "docs/decision/records/records.md#A26" is checked
  Then no error of source_invalid is raised

@id=EX-core-012 @about=REQ-core-058 @source=docs/decision/records/records.md#A38,docs/decision/records/ir-form.md#検査の種類
Scenario: A missing number is an error
  Given "decisions.records" is "docs/decision/records", and "docs/decision/records/records.md" has no line starting with "- A999 "
  When the source "docs/decision/records/records.md#A999" is checked
  Then an error of source_invalid with detail "docs/decision/records/records.md#A999" is raised

@id=EX-core-120 @about=REQ-core-058 @source=docs/decision/records/2026-09-17-record-form.md#A47,docs/decision/records/2026-09-17-record-form.md#A45
Scenario: A heading inside a code block of a file that is not a decision record is not the target of a source
  Given "decisions.records" is "docs/decision/records", and "docs/decision/records/g.md", which is not a decision record, has the heading "## 補足" outside a code block and the heading "## 例" only inside a code block
  When the sources "docs/decision/records/g.md#補足" and "docs/decision/records/g.md#例" are checked
  Then no source_invalid is raised for "docs/decision/records/g.md#補足", and an error of source_invalid with detail "docs/decision/records/g.md#例" is raised
```
