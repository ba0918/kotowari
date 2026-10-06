# Checking overview data

English | [日本語](overview-data.ja.md)

Covers the location and reading of the `overview data`, its form, the content of each `part`, the opening lead, the list of `IR` documents it covers, the references inside each `part`, and the check of each `guide mark`. This check is performed by kotowari-overview, and its results join the `finding` entries of "kotowari check" and "kotowari status" (overview-output.md).

## Requirements

### REQ-core-278: The location of the data

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A15, docs/decision/records/2026-10-02-whole-picture.md#A17, docs/decision/records/2026-10-02-whole-picture.md#A32, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A76, docs/decision/records/2026-10-06-changes-rethink.md#A30
- verification: unit

kotowari always, on "kotowari check", "kotowari status", "kotowari overview build" and "kotowari overview serve", reads as `overview data` the files matched by the globs of "overview.files" in the configuration whose extension is lowercase ".md", and does not read other files (an `exclusion`). "kotowari overview build" and "kotowari overview serve" read the `IR`, each `decision record` and the ADRs from the same configuration and locations as "kotowari check" in order to resolve references, and make the `stop` for those locations (REQ-core-018) the same way as check. How the globs are read, the walk, the `exclusion`, and the `stop` on an unreadable file, a non-UTF-8 file and a symbolic link without a target follow "guides.files" and the `guide`; of hidden directories, only those that a glob names by a path component are read.

### REQ-core-279: When there is no key for the data

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A32
- verification: unit

When the `configuration file` has no "overview" key, kotowari reads no `overview data` on "kotowari check" and "kotowari status", and makes a `stop` with a configuration error as the reason on "kotowari overview build" and "kotowari overview serve".

### REQ-core-280: Overlap with guides and tests

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A58, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A75
- verification: unit

When, on "kotowari check", "kotowari status", "kotowari overview build" or "kotowari overview serve", one file is matched both by the globs of "overview.files" and by the globs of "guides.files" or "tests.files", kotowari makes a `stop` with a configuration error as the reason, and outputs as the detail the path, relative to the `base directory`, of the first overlapping file in the byte order of the path, followed by ": matched by both overview.files and guides.files" or ": matched by both overview.files and tests.files". If there is an overlap of REQ-core-199, that is judged first, and when one file is matched by all three, it outputs ": matched by both overview.files and guides.files".

### REQ-core-281: The form of the data

- kind: algorithm
- source: docs/decision/records/2026-10-02-whole-picture.md#A44, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A55, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A71
- definition: TBL-core-038
- verification: unit

### REQ-core-282: The content of a part

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A43, docs/decision/records/2026-10-02-whole-picture.md#A46, docs/decision/records/2026-10-02-whole-picture.md#A47, docs/decision/records/2026-10-02-whole-picture.md#A48, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-02-whole-picture.md#A86, docs/decision/records/2026-10-05-overview-page-reading.md#A10
- verification: unit

For a fenced code block of the `overview data` whose info string is "view", one or more spaces and a kind name, when the kind name is not a kind of `part` of the rendering engine, kotowari outputs an overview_part_unknown `error` with "line" set to the line where the fence opens and detail to the kind name. When the content cannot be read as YAML, or does not fit the schema for that kind that the rendering engine publishes, kotowari outputs an overview_part_invalid `error` with "line" set to the line where the fence opens and detail to the kind name, one space and the place that did not fit. However, when the content cannot be read as YAML and the YAML reader returns the position of the error, "line" is the line of that position converted to a line of the `overview data` file. The place that did not fit is the JSON Pointer of the value that did not fit, with the name of the key appended with "/" for an unknown key ("additionalProperties") and a missing key ("required"); it is written "(root)" for the whole value and "(yaml)" when the content cannot be read as YAML. `error` entries with the same "line" and detail are merged into one. The detail does not include text produced by the validation library.

### REQ-core-283: The opening lead

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A38, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A77
- verification: unit

When the first block after the `title` of the `overview data` that is neither an HTML comment nor a blank line is not a `part` whose kind name is "lead", kotowari outputs an overview_lead_missing `error` with "line" null and detail the document name.

### REQ-core-284: The IR documents covered

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A19, docs/decision/records/2026-10-02-whole-picture.md#A55, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74
- verification: unit

When an entry of "ir" in the frontmatter of the `overview data` is, as a path relative to the `base directory`, none of the `topic document` files of the `IR` read, kotowari outputs an overview_ir_missing `error` with "line" null and detail the text of that entry. When one `topic document` is in the "ir" of two or more `overview data` files, kotowari outputs, for each such `overview data` file from the second on in the byte order of the path, an overview_ir_shared `error` with "line" null and detail the path of that `topic document`.

### REQ-core-285: References inside a part

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A33, docs/decision/records/2026-10-02-whole-picture.md#A66, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A77
- verification: unit

Taking as references each string entry of a field named "refs" and each string of a field named "ref" in the value of a `part` that fits the schema (at any nesting depth), when a reference is in the form of an `ID` and the `IR` has no `item` or `scenario` with that `ID`, or a reference contains "#" and its target does not exist under the rules of the `source`, or a reference is in neither of these forms, kotowari outputs one overview_ref_unresolved `error` for each, with "line" set to the line where the fence of the `part` opens and detail to the text of the reference.

### REQ-core-286: Guide marks in the data

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A31, docs/decision/records/2026-10-02-whole-picture.md#A39, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A58, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A77
- verification: unit

kotowari always reads and matches each `guide mark` in the `overview data` by the same rules as a `guide` (REQ-core-200, REQ-core-201, REQ-core-202, REQ-core-203, REQ-core-204), and outputs the invalid_marker `error` and the guide_stale `notice` with "path" set to the `overview data`. The `overview data` is not read a second time as a `guide`. The `guide mark` of a section is placed on the line after the "## " heading of that section, as a line of its own.

### REQ-core-287: kotowari-overview checks the data

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A21, docs/decision/records/2026-10-02-whole-picture.md#A48, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-03-public-crate-api.md#A33, docs/decision/records/2026-10-04-overview-on-public-api.md#A1, docs/decision/records/2026-10-04-overview-on-public-api.md#A2
- verification: review
- how_to_verify: Confirm that parsing and checking are in kotowari-overview, that this crate depends on core, markdown-schema and markdown-view and does not touch files, the network or environment variables, and that core does not depend on overview or view. Confirm that reading the data and writing to the cache are in the kotowari library and serve is in kotowari-cli, and that merging the results into check goes through core's group of additional findings.

kotowari always parses and checks the `overview data` in the kotowari-overview crate, and kotowari-core does not know about it. The kotowari library reads the data files and writes the files of the `overview`, and kotowari-cli performs serve, which serves over HTTP. The kotowari library merges the check results into check, passing them as core's group of additional findings.

## Decision tables

### TBL-core-038: The form of the data

- source: docs/decision/records/2026-10-02-whole-picture.md#A44, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A55, docs/decision/records/2026-10-02-whole-picture.md#A71, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A77, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-04-overview-on-public-api.md#A5, docs/decision/records/2026-10-04-overview-on-public-api.md#A11

The form is declared in a schema ("crates/kotowari-overview/schemas/overview.yaml" in the kotowari-overview package, included at compile time), and kotowari-markdown-schema checks the form as Markdown. For each place that does not fit the form, kotowari outputs an overview_form_invalid `error` with "line" set to that line (null for one that concerns the whole document) and detail to the name of the kind of the `finding` of kotowari-markdown-schema. Only for a violation in the frontmatter lines is the detail "frontmatter".

| Part | Form |
|---|---|
| frontmatter | YAML at the start of the document. The only key is "ir", whose value is a list of one or more strings. Missing, unreadable, an unknown key, or "ir" missing, empty or not a list of strings is a violation of the form |
| title | exactly one "# " heading |
| after the title | before the first "## " heading, only each `part` and HTML comments may be placed (that the first `part` is a lead is checked by REQ-core-283) |
| section | a "## " heading. Under it, sentences, bulleted lists (including nested ones), numbered lists, tables, code blocks, HTML comments and "### " headings may be placed |
| part | a fenced code block whose info string is "view", one or more spaces and a kind name. Its content is checked by REQ-core-282 |

## Examples

```gherkin
@id=EX-core-463 @about=REQ-core-279 @source=docs/decision/records/2026-10-02-whole-picture.md#A32,docs/decision/records/2026-10-02-whole-picture.md#A64,docs/decision/records/2026-10-02-whole-picture.md#A78
Scenario: Without the overview key, check does not read the data and build stops
  Given the `configuration file` has no "overview" key, and ".kotowari/overview/x.md" is a malformed file
  When "kotowari check --format json" and "kotowari overview build" are run
  Then the "files" of "overview" in check is 0, and there is no `finding` pointing at that file
  And the exit code of build is 2, and the first line of standard error starts with "config error: "

@id=EX-core-464 @about=REQ-core-278 @source=docs/decision/records/2026-10-02-whole-picture.md#A17,docs/decision/records/2026-10-02-whole-picture.md#A32,docs/decision/records/2026-10-02-whole-picture.md#A64
Scenario: Data in a hidden directory named by the glob is read
  Given "overview.files" is ".kotowari/overview/*.md", and ".kotowari/overview/changes.md" has correct `overview data`
  When "kotowari check --format json" is run
  Then the "files" of "overview" is 1

@id=EX-core-465 @about=REQ-core-280 @source=docs/decision/records/2026-10-02-whole-picture.md#A58,docs/decision/records/2026-10-02-whole-picture.md#A75
Scenario: Data that overlaps a guide is a configuration error
  Given both "overview.files" and "guides.files" match "docs/a.md"
  When "kotowari check" is run
  Then the exit code is 2, and the first line of standard error is "config error: docs/a.md: matched by both overview.files and guides.files"

@id=EX-core-466 @about=REQ-core-282 @source=docs/decision/records/2026-10-02-whole-picture.md#A46,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A73,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: An unknown part and a part that does not fit the schema are errors
  Given the `overview data` has a "```view chart" fence, and a "```view cards" fence whose content has an unknown key "color"
  When "kotowari check --format json" is run
  Then an overview_part_unknown `error` with detail "chart" is output, and an overview_part_invalid `error` whose detail is "cards", a space, and the JSON Pointer of the value that has the "color" key with "/color" appended

@id=EX-core-511 @about=REQ-core-282 @source=docs/decision/records/2026-10-05-overview-page-reading.md#A10
Scenario: A part that cannot be read as YAML points at the line of the error
  Given line 10 of the `overview data` has a "```view cards" fence, and line 13 inside it cannot be read as YAML
  When "kotowari check --format json" is run
  Then one overview_part_invalid `error` is output with "line" 13 and detail "cards (yaml)"

@id=EX-core-467 @about=REQ-core-283 @source=docs/decision/records/2026-10-02-whole-picture.md#A38,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: An opening that is not a lead is an error
  Given there is `overview data` "x.md" whose first `part` right after the `title` is "```view steps"
  When "kotowari check --format json" is run
  Then an overview_lead_missing `error` is output with "line" null and detail "x.md"

@id=EX-core-468 @about=REQ-core-284 @source=docs/decision/records/2026-10-02-whole-picture.md#A19,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: A missing IR document and an IR document belonging to two overviews are errors
  Given the "ir" of both `overview data` files "a.md" and "b.md" has "docs/ir/core/cli.md", and the "ir" of "a.md" also has "docs/ir/core/none.md"
  When "kotowari check --format json" is run
  Then overview_ir_missing with detail "docs/ir/core/none.md" is output on "a.md", and overview_ir_shared with detail "docs/ir/core/cli.md" is output on "b.md"

@id=EX-core-469 @about=REQ-core-285 @source=docs/decision/records/2026-10-02-whole-picture.md#A66,docs/decision/records/2026-10-02-whole-picture.md#A33,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A74
Scenario: A reference that cannot be resolved is an error
  Given the "refs" of a steps `part` has "REQ-core-001", "REQ-core-999", "docs/decision/records/records.md#A9999" and "foo"
  When "kotowari check --format json" is run
  Then one overview_ref_unresolved `error` each is output with detail "REQ-core-999", "docs/decision/records/records.md#A9999" and "foo", and none for "REQ-core-001"

@id=EX-core-470 @about=REQ-core-286 @source=docs/decision/records/2026-10-02-whole-picture.md#A39,docs/decision/records/2026-10-02-whole-picture.md#A58,docs/decision/records/2026-10-02-whole-picture.md#A32,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A73,docs/decision/records/2026-10-02-whole-picture.md#A77
Scenario: A stale guide mark in the data becomes guide_stale
  Given on the line after the "## " heading of a section of the `overview data` there is a `guide mark` with a `fingerprint` different from the current `fingerprint`
  When "kotowari check --format json" is run
  Then one guide_stale `notice` is output whose "path" is that `overview data`

@id=EX-core-471 @about=REQ-core-281 @source=docs/decision/records/2026-10-02-whole-picture.md#A55,docs/decision/records/2026-10-02-whole-picture.md#A69,docs/decision/records/2026-10-02-whole-picture.md#A71
Scenario: An unknown key in the frontmatter is a form error
  Given there is `overview data` whose frontmatter has the keys "ir" and "title"
  When "kotowari check --format json" is run
  Then an overview_form_invalid `error` is output
```
