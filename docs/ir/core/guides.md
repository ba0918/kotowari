# Guide marks

English | [日本語](guides.ja.md)

Covers the location and reading of each `guide`, the form of a `guide mark`, how a `fingerprint` is taken, and the `finding` raised by comparing a `fingerprint`. The key of "kotowari list" that outputs a `fingerprint` is `TBL-core-026` in list.md, and the output of the "guides" group is covered by output.md and status.md.

## Requirements

### REQ-core-198: Reading guides

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A1, docs/decision/records/2026-09-24-doc-marks.md#A4, docs/decision/records/2026-09-24-doc-marks.md#A16, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-24-doc-marks.md#A24, docs/decision/records/2026-09-24-doc-marks.md#A29
- verification: unit

kotowari always, in "kotowari check" and "kotowari status", reads the files matched by the globs of the configuration's "guides.files" as each `guide`, and reads no `guide` at all when "guides.files" is an empty list. The reading of globs, the traversal, each `exclusion`, and the `stop` on an unreadable file, a file that is not UTF-8 and a symbolic link without a target follow "tests.files" and each `test file` (REQ-core-019, REQ-core-079, REQ-core-018, `TBL-core-001`). A glob that matches nothing is not made an `error`. Even when a glob of "guides.files" matches a file in the location of the `IR` or of each `decision record`, kotowari does not `stop`, and reads that file as a `guide` too.

### REQ-core-199: Overlap of the guide and test locations

- kind: event_driven
- source: docs/decision/records/2026-09-24-doc-marks.md#A15, docs/decision/records/2026-09-24-doc-marks.md#A28, docs/decision/records/2026-09-24-doc-marks.md#A36
- verification: unit

When, in "kotowari check", one file matches both a glob of "guides.files" and a glob of "tests.files", kotowari does a `stop` on the grounds of a configuration error, and outputs as the detail the string made of the path, relative to the `base directory`, of the first of the overlapping files in byte order of path, followed by ": matched by both guides.files and tests.files".

### REQ-core-200: Where guide marks are read

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A5, docs/decision/records/2026-09-24-doc-marks.md#A9, docs/decision/records/2026-09-24-doc-marks.md#A25, docs/decision/records/2026-09-24-doc-marks.md#A30
- verification: unit

kotowari always reads a `guide` as CommonMark, and reads as a `guide mark` only a sequence starting with "@kotowari[" inside an HTML comment, from "<!--" to "-->". It does not read "@kotowari[" inside a CommonMark code block (the indented form and the fenced form; wider than a `code block`) or a code span, or in the characters after "-->" (including the rest of the same line of an HTML block that starts with "<!--") (`exclusion`). It picks up every `guide mark` inside one comment and within one line.

### REQ-core-201: The form of a guide mark

- kind: algorithm
- source: docs/decision/records/2026-09-24-doc-marks.md#A5, docs/decision/records/2026-09-24-doc-marks.md#A7, docs/decision/records/2026-09-24-doc-marks.md#A10
- definition: TBL-core-036
- verification: unit

### REQ-core-202: A guide mark of the wrong form

- kind: event_driven
- source: docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A23, docs/decision/records/2026-09-24-doc-marks.md#A31, docs/decision/records/2026-09-24-doc-marks.md#A32
- verification: unit

When the inside of a `guide mark` is empty or only separators, when there is no closing bracket on the same line, when a separated entry has no ":", when the part before ":" is not of the `ID` form (including empty), or when the part after ":" is not of the `fingerprint` form of `TBL-core-036`, kotowari raises one invalid_marker `error` per `guide mark`, with the `guide` as "path", the start line of that `guide mark` as "line" and the characters of that line as the detail, and compares none of the entries of that `guide mark`.

### REQ-core-203: How a fingerprint is taken

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A6, docs/decision/records/2026-09-24-doc-marks.md#A7, docs/decision/records/2026-09-24-doc-marks.md#A26, docs/decision/records/2026-09-24-doc-marks.md#A27, docs/decision/records/2026-09-24-doc-marks.md#A33, docs/decision/records/2026-09-25-deferred-items.md#A6, docs/decision/records/2026-09-25-deferred-items.md#A20, docs/decision/records/2026-09-25-deferred-items.md#A23, docs/decision/records/2026-09-25-deferred-items.md#A26
- verification: unit

kotowari always makes the `fingerprint` of an `item` and of a `scenario` the first eight characters of the SHA-256, written in lowercase hexadecimal, of the UTF-8 bytes of the string that joins the following sequence of lines with "\n" (with no "\n" after the last line). For an `item`, the sequence is the "body" lines of `TBL-core-027` without the "- source:" lines (for a `requirement` in a document that has a document-level "- deferred:" line, the first such line is added at the start of the sequence); for a `scenario`, it is only the step lines. Line-ending characters (including the "\r" of "\r\n") are not part of a line. The name in a heading, and the tag line and the "Scenario:" line of a `scenario`, are not part of the `fingerprint`.

### REQ-core-204: A stale guide mark

- kind: event_driven
- source: docs/decision/records/2026-09-24-doc-marks.md#A2, docs/decision/records/2026-09-24-doc-marks.md#A8, docs/decision/records/2026-09-24-doc-marks.md#A11, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-24-doc-marks.md#A20, docs/decision/records/2026-09-24-doc-marks.md#A12, docs/decision/records/2026-09-24-doc-marks.md#A32, docs/decision/records/2026-09-24-doc-marks.md#A34, docs/decision/records/2026-09-24-doc-marks.md#A31, docs/decision/records/2026-09-24-doc-marks.md#A27
- verification: unit

For one entry of a `guide mark` of the correct form, when the `IR` has no `item` or `scenario` with that `ID`, or when the `fingerprint` of no `item` or `scenario` with that `ID` is the same as the `fingerprint` written in the entry, kotowari raises one guide_stale `notice` per entry, with the `guide` as "path", the start line of that `guide mark` as "line", and as the detail the string of the `ID`, the `fingerprint` written in the entry and the current `fingerprint`, separated by single half-width spaces. The current `fingerprint` is "-" when it is not in the `IR`, and the first `fingerprint` under REQ-core-032 when the same `ID` is in two or more places.

### REQ-core-205: Occasions for writing guides

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-doc-marks.md#A12, docs/decision/records/2026-09-24-doc-marks.md#A13, docs/decision/records/2026-09-24-doc-marks.md#A18, docs/decision/records/2026-09-24-doc-marks.md#A2, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-24-doc-marks.md#A37
- verification: review
- how_to_verify: Read the skill under "agent/skills/kotowari/" and confirm that it has an occasion for writing each `guide` and one for reviewing it, and that the occasion has a guideline on how finely to place each `guide mark` (up to a few `ID` values per `guide mark`), the rule of placing one `guide mark` next to the heading of the section it covers, the procedure of confirming with "kotowari query" the `item` entries that correspond to the content of the section and adding them to the `guide mark`, and the procedure of reviewing the section before copying the current `fingerprint` in the detail of guide_stale into the `guide mark`

The kotowari skill always has an occasion for writing each `guide` and one for reviewing it.

## Decision tables

### TBL-core-036: Syntax of a guide mark

- source: docs/decision/records/2026-09-24-doc-marks.md#A5, docs/decision/records/2026-09-24-doc-marks.md#A7, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A23, docs/decision/records/2026-09-24-doc-marks.md#A31

| Part | Form |
|---|---|
| Start | @kotowari[ |
| Contents | Each entry is written in the "ID:fingerprint" form, and entries are listed separated by commas. For each comma-separated entry, the surrounding whitespace is removed, the entry is split at the first ":" into the `ID` and the `fingerprint`, and the surrounding whitespace of each is removed. Even when there are two or more entries with the same `ID`, each is treated as an entry of its own |
| `fingerprint` | Eight lowercase hexadecimal characters ("0" to "9" and "a" to "f") |
| End | "]" on the same line as the start |

## Examples

```gherkin
@id=EX-core-362 @about=REQ-core-198,REQ-core-200,REQ-core-203,REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A2,docs/decision/records/2026-09-24-doc-marks.md#A5,docs/decision/records/2026-09-24-doc-marks.md#A6,docs/decision/records/2026-09-24-doc-marks.md#A7,docs/decision/records/2026-09-24-doc-marks.md#A4,docs/decision/records/2026-09-24-doc-marks.md#A17,docs/decision/records/2026-09-24-doc-marks.md#A27
Scenario: Nothing is raised when the fingerprint matches the current IR
  Given "guides.files" is "guides/**/*.md", "docs/decision/records/r.md" has the decision "A1", and the body of "REQ-001" in "docs/ir/a.md" is the five lines "- kind: ubiquitous", "- source: docs/decision/records/r.md#A1", "- verification: unit", an empty line and "文。"
  And "guides/a.md" has the line "<!-- @kotowari[REQ-001:51b1f3da] -->"
  When "kotowari check --format json" is run
  Then no `finding` is raised for "guides/a.md", and "files" of "guides" is 1 and "marks" is 1

@id=EX-core-363 @about=REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A2,docs/decision/records/2026-09-24-doc-marks.md#A8,docs/decision/records/2026-09-24-doc-marks.md#A14,docs/decision/records/2026-09-24-doc-marks.md#A7,docs/decision/records/2026-09-24-doc-marks.md#A12,docs/decision/records/2026-09-24-doc-marks.md#A32
Scenario: Changing the body turns the stale mark into a notice, without blocking completeness
  Given the `IR` and the `guide` of EX-core-362 exist, and the line "文。" in the body of "REQ-001" is changed to "別の文。"
  When "kotowari check --format text" is run
  Then one guide_stale `notice` is raised on the line of the `guide mark` of "guides/a.md", and its detail starts with "REQ-001 51b1f3da " followed by eight lowercase hexadecimal characters other than "51b1f3da"
  And the exit code is 0, and if there is no other `error` and no `flag record`, "complete" of "kotowari status" is true

@id=EX-core-364 @about=REQ-core-203 @source=docs/decision/records/2026-09-24-doc-marks.md#A6,docs/decision/records/2026-09-24-doc-marks.md#A27,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: Changing only the heading name and the sources does not make it stale
  Given the `IR` and the `guide` of EX-core-362 exist, the name in the heading of "REQ-001" is changed, and one source is added to the "- source:" line
  When "kotowari check --format json" is run
  Then no guide_stale `notice` is raised, and "marks" of "guides" is 1

@id=EX-core-365 @about=REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A11,docs/decision/records/2026-09-24-doc-marks.md#A14
Scenario: A mark pointing at an item removed from the IR becomes a notice
  Given "guides/a.md" has the line "<!-- @kotowari[REQ-009:51b1f3da] -->", and the `IR` has no "REQ-009"
  When "kotowari check --format text" is run
  Then one guide_stale `notice` with the detail "REQ-009 51b1f3da -" is raised, and no unresolved_reference `error` is raised

@id=EX-core-366 @about=REQ-core-200 @source=docs/decision/records/2026-09-24-doc-marks.md#A9,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: Sequences outside comments and inside code are not read
  Given "guides/a.md" has the line "<!-- @kotowari[REQ-001] -->" inside a code block starting with "```", and the body line "@kotowari[REQ-001] の形で書く" outside any comment
  When "kotowari check --format json" is run
  Then neither invalid_marker nor guide_stale is raised for "guides/a.md", and "marks" of "guides" is 0

@id=EX-core-367 @about=REQ-core-202,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A10,docs/decision/records/2026-09-24-doc-marks.md#A31,docs/decision/records/ir-form.md#出力,docs/decision/records/2026-09-24-doc-marks.md#A32
Scenario: A mark without a fingerprint and an uppercase fingerprint are form errors
  Given line 3 of "guides/a.md" is "<!-- @kotowari[REQ-001] -->" and line 5 is "<!-- @kotowari[REQ-001:8C0D7663] -->"
  When "kotowari check --format text" is run
  Then one invalid_marker `error` each is raised on lines 3 and 5, and guide_stale is raised on neither line
  And the exit code is 1

@id=EX-core-368 @about=REQ-core-199 @source=docs/decision/records/2026-09-24-doc-marks.md#A15,docs/decision/records/2026-09-24-doc-marks.md#A36
Scenario: Overlapping guide and test locations cause a stop
  Given "guides.files" is "docs/**/*.md", "tests.files" is "**/*", and "docs/guide.md" exists
  When "kotowari check" is run
  Then the exit code is 2, and the detail of the stop is "docs/guide.md: matched by both guides.files and tests.files"

@id=EX-core-369 @about=REQ-core-198,REQ-core-206 @source=docs/decision/records/2026-09-24-doc-marks.md#A4,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: Without guides.files no guide is read
  Given the `configuration file` has no "guides" key, and "docs/guide.md" has the line "<!-- @kotowari[REQ-001:00000000] -->"
  When "kotowari check --format json" is run
  Then no guide_stale `notice` is raised, and "files" and "marks" of "guides" are both 0

@id=EX-core-370 @about=REQ-core-204 @source=docs/decision/records/2026-09-24-doc-marks.md#A20,docs/decision/records/2026-09-24-doc-marks.md#A17
Scenario: When the same ID is in two places, matching either one is not stale
  Given "REQ-001" is in "docs/ir/a.md" and "docs/ir/b.md", the `fingerprint` of "REQ-001" in "docs/ir/a.md" is not "51b1f3da", the `fingerprint` of "REQ-001" in "docs/ir/b.md" is "51b1f3da", and "guides/a.md" has the line "<!-- @kotowari[REQ-001:51b1f3da] -->"
  When "kotowari check --format json" is run
  Then a duplicate_id `error` is raised, but no guide_stale `notice` is raised, and "marks" of "guides" is 1

@id=EX-core-371 @about=REQ-core-200,REQ-core-206,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A5,docs/decision/records/2026-09-24-doc-marks.md#A17,docs/decision/records/2026-09-24-doc-marks.md#A2,docs/decision/records/2026-09-24-doc-marks.md#A14
Scenario: Two entries listed in one mark are counted
  Given "EX-001", whose `fingerprint` is not "51b1f3da", is added to the `IR` of EX-core-362, and the `guide mark` of "guides/a.md" is made "<!-- @kotowari[REQ-001:51b1f3da, EX-001:51b1f3da] -->"
  When "kotowari check --format json" is run
  Then "files" of "guides" is 1 and "marks" is 2, and the only guide_stale `notice` is one whose detail starts with "EX-001 51b1f3da "

@id=EX-core-372 @about=REQ-core-203,TBL-core-026 @source=docs/decision/records/2026-09-24-doc-marks.md#A6,docs/decision/records/2026-09-24-doc-marks.md#A7,docs/decision/records/2026-09-24-doc-marks.md#A14,docs/decision/records/2026-09-24-doc-marks.md#A27
Scenario: An entry of list shows the fingerprint
  Given the `IR` of EX-core-362 exists
  When "kotowari list --format json" is run
  Then "fingerprint" of the entry whose "id" is "REQ-001" is "51b1f3da"

@id=EX-core-373 @about=REQ-core-202,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A10,docs/decision/records/2026-09-24-doc-marks.md#A23,docs/decision/records/2026-09-24-doc-marks.md#A31,docs/decision/records/2026-09-24-doc-marks.md#A35
Scenario: A mark with even one entry of the wrong form compares none of its entries
  Given the `IR` of EX-core-362 exists, and the `guide mark` of "guides/a.md" is "<!-- @kotowari[REQ-001:00000000, foo:51b1f3da] -->"
  When "kotowari check --format json" is run
  Then one invalid_marker `error` is raised on that line, no guide_stale `notice` is raised, and "marks" of "guides" is 0

@id=EX-core-374 @about=REQ-core-200,TBL-core-036 @source=docs/decision/records/2026-09-24-doc-marks.md#A23,docs/decision/records/2026-09-24-doc-marks.md#A25,docs/decision/records/2026-09-24-doc-marks.md#A17,docs/decision/records/2026-09-24-doc-marks.md#A35
Scenario: Whitespace inside the brackets is allowed, and a sequence after the comment is not read
  Given the `IR` of EX-core-362 exists, line 3 of "guides/a.md" is "<!-- @kotowari[ REQ-001 : 51b1f3da ] -->", and line 5 is "<!-- a --> @kotowari[REQ-001:00000000]"
  When "kotowari check --format json" is run
  Then no `finding` is raised for "guides/a.md", and "marks" of "guides" is 1

@id=EX-core-375 @about=REQ-core-203 @source=docs/decision/records/2026-09-24-doc-marks.md#A27
Scenario: Changing the name of a scenario does not make it stale
  Given a `guide mark` pointing at the `scenario` "EX-001" has the current `fingerprint`, and only the name on the "Scenario:" line of "EX-001" and its "@source" tag are changed
  When "kotowari check --format json" is run
  Then no guide_stale `notice` is raised
```
