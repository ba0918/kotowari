# Checking the table of contents

English | [日本語](overview-toc.ja.md)

Covers the location and reading of the `table of contents`, its form, the check for mismatches with the `overview data`, and passing the `table of contents` to the rendering engine. Like the check of the `overview data` (overview-data.md), the check is performed by kotowari-overview, and its results join the `finding` entries of "kotowari check" and "kotowari status" (overview-output.md).

## Requirements

### REQ-core-325: The location of the table of contents

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A11, docs/decision/records/2026-10-05-overview-index.md#A17, docs/decision/records/2026-10-05-overview-index.md#A32
- verification: unit

When the `configuration file` has the "overview" key, kotowari, on "kotowari check", "kotowari status", "kotowari overview build" and "kotowari overview serve", reads the file that "overview.toc" points to as the `table of contents`. When the target does not exist or cannot be read, kotowari makes a `stop` with an unreadable file as the reason, and when it is not UTF-8, it makes a `stop` with a non-UTF-8 file as the reason.

### REQ-core-326: Overlap of the table of contents with globs

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A58, docs/decision/records/2026-10-05-overview-index.md#A23, docs/decision/records/2026-10-05-overview-index.md#A28, docs/decision/records/2026-10-05-overview-index.md#A32
- verification: unit

When, on "kotowari check", "kotowari status", "kotowari overview build" or "kotowari overview serve", the file that "overview.toc" points to is among the files read by the walk of any of "overview.files" (REQ-core-278), "guides.files" or "tests.files", kotowari makes a `stop` with a configuration error as the reason, and outputs as the detail the path of that file relative to the `base directory`, followed by ": matched by both overview.toc and " and the name of the first matched key in this order. If there is an overlap of REQ-core-199 or REQ-core-280, that is judged first, and this judgement is made before the `table of contents` is read under REQ-core-325.

### REQ-core-327: The form of the table of contents

- kind: algorithm
- source: docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A22
- definition: TBL-core-043
- verification: unit

### REQ-core-328: A page not in the table of contents

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A21, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A39, docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When nowhere in a well-formed `table of contents` (for a `pair`, the `side` in the `first language`) is there a name entry equal to the file name, with ".md" removed, of some `overview data` of the `side` in the `first language`, kotowari outputs an overview_toc_page_missing `error` with "path" set to the file of the `table of contents`, "line" to null and detail to that name.

### REQ-core-329: Names without data and duplicated names

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A29, docs/decision/records/2026-10-05-overview-index.md#A21, docs/decision/records/2026-10-05-overview-index.md#A34, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A39, docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When a name entry of a well-formed `table of contents` (for a `pair`, the `side` in the `first language`) is not equal to the file name, with ".md" removed, of any `overview data` of the `side` in the `first language`, kotowari outputs an overview_toc_page_unknown `error` with "path" set to the file of the `table of contents`, "line" to null and detail to the JSON Pointer of that entry. When there are two or more entries for a name that has `overview data`, kotowari walks the `table of contents` depth-first in the written order and, for each such entry from the second on, outputs an overview_toc_page_duplicate `error` with "path" set to the file of the `table of contents`, "line" to null and detail to the JSON Pointer of that entry.

### REQ-core-330: An empty contents group

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A21, docs/decision/records/2026-10-05-overview-index.md#A34
- verification: unit

When the "items" of a `contents group` of a well-formed `table of contents` is an empty list, kotowari outputs an overview_toc_group_empty `error` with "path" set to the file of the `table of contents`, "line" to null and detail to the JSON Pointer of that `contents group` ("(root)" for the outermost one).

### REQ-core-331: A malformed table of contents is not matched

- kind: event_driven
- source: docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A21
- verification: unit

When the `table of contents` has one or more overview_toc_invalid `error` entries, kotowari does not perform the checks of REQ-core-328, REQ-core-329 and REQ-core-330 on that `table of contents`.

### REQ-core-332: Passing the table of contents to the rendering engine

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A7, docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15
- verification: unit

kotowari always, on "kotowari overview build" and "kotowari overview serve", passes the "title", "note" and "items" of the `table of contents`, in the order and nesting as written, to the rendering engine as its `table of contents`.

## Decision tables

### TBL-core-043: The form of the table of contents

- source: docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A22, docs/decision/records/2026-10-05-overview-index.md#A27, docs/decision/records/2026-10-05-overview-index.md#A34

For each violation of the form, kotowari outputs an overview_toc_invalid `error` with "path" set to the file of the `table of contents`, "line" to null and detail to the place that did not fit. The place that did not fit is written as in REQ-core-282: the JSON Pointer of the value that did not fit, with the name of the key appended with "/" for an unknown key and a missing key, "(root)" for the whole value, and "(yaml)" when the content cannot be read as YAML. `error` entries with the same detail are merged into one.

| Part | Form |
|---|---|
| top level | one `contents group` |
| `contents group` | a mapping of keys to values. The keys are only "title" (required; a non-empty string), "note" (may be omitted; a string without a line break) and "items" (required; a list of name entries or each `contents group`; an empty list is handled by REQ-core-330). An unknown key, a missing required key, a value of the wrong type and an empty "title" are violations of the form |
| name entry | a string. It is the file name of the `overview data` with ".md" removed. An empty string is a violation of the form |

## Examples

```gherkin
@id=EX-core-505 @about=REQ-core-013,REQ-core-325 @source=docs/decision/records/2026-10-05-overview-index.md#A11,docs/decision/records/2026-10-05-overview-index.md#A17,docs/decision/records/2026-10-05-overview-index.md#A32,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104
Scenario: It stops when the table of contents key or file is missing
  Given there is a `configuration file` that has "overview.files" but no "overview.toc" key, and a `configuration file` whose "overview.toc" points to a file that does not exist
  When "kotowari overview build" is run with each
  Then both exit with code 2; the first line of standard error of the former starts with "config error: ", and the latter makes a `stop` with an unreadable file as the reason

@id=EX-core-506 @about=REQ-core-326 @source=docs/decision/records/2026-10-05-overview-index.md#A17,docs/decision/records/2026-10-05-overview-index.md#A23,docs/decision/records/2026-10-05-overview-index.md#A28,docs/decision/records/records.md#A20,docs/decision/records/records.md#A104,docs/decision/records/2026-10-05-overview-index.md#A32
Scenario: A table of contents matched by the glob of the data is a configuration error
  Given "overview.files" is ".kotowari/overview/*.md", and "overview.toc" is ".kotowari/overview/toc.md"
  When "kotowari check" is run
  Then the exit code is 2, and the first line of standard error is "config error: .kotowari/overview/toc.md: matched by both overview.toc and overview.files"

@id=EX-core-507 @about=REQ-core-327,REQ-core-331 @source=docs/decision/records/2026-10-05-overview-index.md#A10,docs/decision/records/2026-10-05-overview-index.md#A16,docs/decision/records/2026-10-05-overview-index.md#A21,docs/decision/records/2026-10-05-overview-index.md#A34
Scenario: A contents group with an unknown key is a form error and is not matched
  Given the first entry of "items" of the `table of contents` is a `contents group` with the keys "title", "items" and "color", and no name of any `overview data` is in the `table of contents`
  When "kotowari check --format json" is run
  Then an overview_toc_invalid `error` is output with "line" null and detail "/items/0/color", and no overview_toc_page_missing is output

@id=EX-core-508 @about=REQ-core-328,REQ-core-329,REQ-core-294 @source=docs/decision/records/2026-10-05-overview-index.md#A12,docs/decision/records/2026-10-05-overview-index.md#A16,docs/decision/records/2026-10-02-whole-picture.md#A33
Scenario: A page not in the table of contents, a name without data and a duplicated name are errors
  Given the `overview data` files are "a.md", "b.md" and "c.md", and the "items" of the `table of contents` are "a", "z", "a" in this order
  When "kotowari check --format json" is run
  Then overview_toc_page_missing with detail "b" and with detail "c", overview_toc_page_unknown with detail "/items/1", and overview_toc_page_duplicate with detail "/items/2" are output as each `error`
  And "kotowari overview build" exits with code 2 and writes nothing

@id=EX-core-509 @about=REQ-core-330 @source=docs/decision/records/2026-10-05-overview-index.md#A12,docs/decision/records/2026-10-05-overview-index.md#A16
Scenario: A contents group without entries is an error
  Given the "items" of the `table of contents` are "a" and a `contents group` whose "title" is "空" and whose "items" is an empty list
  When "kotowari check --format json" is run
  Then an overview_toc_group_empty `error` with detail "/items/1" is output

@id=EX-core-510 @about=REQ-core-332 @source=docs/decision/records/2026-10-05-overview-index.md#A7,docs/decision/records/2026-10-05-overview-index.md#A15
Scenario: The index is ordered as written in the table of contents
  Given the `overview data` files are "a.md" and "b.md", and the "title" of the `table of contents` is "kotowari" and its "items" are "b", "a" in this order
  When "kotowari overview build" is run
  Then the heading of ".kotowari/cache/overview/index.html" is "kotowari", and the title of "b" comes before the title of "a"
```
