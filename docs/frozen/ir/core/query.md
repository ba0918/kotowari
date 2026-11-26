# Reading one entry

English | [日本語](query.ja.md)

Covers how "kotowari query" receives one `ID` and outputs the `item` or `scenario` having that `ID` in the form of one entry of "kotowari list", with the body and the reverse references added.

## Requirements

### REQ-core-156: Reading in query

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A1, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A6, docs/decision/records/2026-09-20-query-status.md#A19
- verification: unit

In "kotowari query", kotowari always reads the documents of the `IR` and the `test file` files from the same configuration and places as "kotowari check", outputs in "items" every `item` and `scenario` that has the same `ID` as the positional argument, outputs no `finding`, and makes the exit code 0. Even when the documents of the `IR` have an `error`, it outputs the `item` and `scenario` entries it could read. When several have the same `ID`, it outputs them all.

### REQ-core-157: Arguments of query

- kind: event_driven
- source: docs/decision/records/records.md#A136, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A6
- verification: unit

When "kotowari query" has neither "--help" nor "--version" and the positional arguments are not exactly one, when the positional argument is not of the form of an `ID`, or when neither an `item` nor a `scenario` has the same `ID` as the positional argument, kotowari makes a `stop` with an argument error as the reason. The detail when nothing has the `ID` is "unknown id: " followed by the text of the positional argument.

### REQ-core-158: Stops of query

- kind: event_driven
- source: docs/decision/records/2026-09-20-query-status.md#A19, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

In "kotowari query", when a condition under which "kotowari check" makes a `stop` (a configuration error, an unreadable file, an argument error) holds, kotowari makes a `stop` with the same reason and wording as check. However, "kotowari query" does not read any `guide`, and makes no `stop` caused by the places or the reading of `guide` files (REQ-core-198, REQ-core-199). It does not read the `surface file` files, the `surface rule` files or the `list of unspecified surfaces` either, and makes no `stop` caused by reading them (REQ-core-229). It does not read the `overview data` either, and makes no `stop` caused by its places or its reading (REQ-core-278, REQ-core-280).

### REQ-core-159: The form of one entry of query

- kind: algorithm
- source: docs/decision/records/2026-09-20-query-status.md#A2, docs/decision/records/2026-09-20-query-status.md#A3, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A16, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
- definition: TBL-core-027
- verification: unit

### REQ-core-160: The order of query

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A15, docs/decision/records/2026-09-20-query-status.md#A20
- verification: unit

kotowari always orders "items" in the same order as "kotowari list", and orders the "referenced_by" of one entry in ascending order of "path", and within the same "path" in ascending order of "line".

### REQ-core-161: The form of the output of query

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A5, docs/decision/records/2026-09-20-query-status.md#A7, docs/decision/records/2026-09-20-query-status.md#A13
- verification: unit

kotowari always accepts only the two values "json" and "text" for "--format" of "kotowari query", with "json" as the default. With "json" it outputs one JSON whose top level is only "items", and "items" is a sequence of entries, each with the keys of `TBL-core-027`. With "text", for each entry it outputs the same first line and "tests" lines as the "text" of "kotowari list", then outputs each line of "body" indented by two half-width spaces, and finally outputs one line "  <- ID via path:line" per entry of "referenced_by".

## Decision tables

### TBL-core-027: Keys of one entry of query

- source: docs/decision/records/2026-09-20-query-status.md#A2, docs/decision/records/2026-09-20-query-status.md#A3, docs/decision/records/2026-09-20-query-status.md#A4, docs/decision/records/2026-09-20-query-status.md#A16, docs/decision/records/2026-09-23-ir-engine-gaps.md#A9, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24, docs/decision/records/2026-09-23-ir-engine-gaps.md#A34, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-review6-gaps.md#A2

| Key | Kinds that have it | Content |
|---|---|---|
| Every key of `TBL-core-026` | As in `TBL-core-026` | As in `TBL-core-026` |
| body | All | The sequence of lines of the body. For an `item`, from the line after the heading up to the last line of that `item` that the schema side returns. The last line is the line before the next heading at the same depth as that `item`'s heading or shallower ("### ", "## ", "# ") (the last line of the document when there is none); lines of the heading form inside a code block are not counted, and kotowari does not search the raw lines for headings. For a `scenario`, from the "@id" tag line to the last step line. Blank lines at the start and end are not included. The characters of the lines are as they are |
| referenced_by | All | The sequence of the `item` and `scenario` entries that point at that `ID`. However many times one `item` or `scenario` points at the same `ID` with the same via, it is made one entry. One entry has "id", "kind", "path", "line" (the same meaning as in `TBL-core-026`) and "via" |
| via of referenced_by | All | One of "definition" (the "- definition:" line), "relations" (the "- related:" line), "about" (the "@about" tag) and "text" (an `ID` enclosed in backquotes outside double quotes in a `statement` of a `requirement`, a `statement` of a `property`, or a step of a `scenario`; the same judgement as REQ-core-054) |

## Examples

```gherkin
@id=EX-core-287 @about=TBL-core-027 @source=docs/decision/records/2026-09-24-review6-gaps.md#A2
Scenario: An item pointing at the same ID twice is one reverse reference
  Given there is "REQ-001" that points at "REQ-002" twice in its statement
  When "kotowari query REQ-002" is run
  Then "referenced_by" has only one entry of "REQ-001" whose via is "text"

@id=EX-core-250 @about=REQ-core-156,REQ-core-159,TBL-core-027 @source=docs/decision/records/2026-09-20-query-status.md#A2,docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A4,docs/decision/records/2026-09-20-query-status.md#A16,docs/decision/records/2026-09-20-query-status.md#A19,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: One entry comes with its body and reverse references
  Given in the IR there is a requirement "REQ-001" named "例" on line 7 of "docs/ir/a.md", and under it are the lines "- kind: ubiquitous", "- source:" and "- verification: unit" and the statement "文。"
  And in the same document there is a scenario "EX-001" with "@about=REQ-001" on line 20
  When "kotowari query REQ-001" is run
  Then the exit code is 0, "items" is one entry whose "id" is "REQ-001", its "body" is the sequence of lines from the "- kind: ubiquitous" line to the "文。" line, and its "referenced_by" is one entry whose "id" is "EX-001", "via" is "about" and "line" is 20

@id=EX-core-251 @about=REQ-core-157 @source=docs/decision/records/2026-09-20-query-status.md#A6
Scenario: A missing ID stops
  Given in the IR neither an item nor a scenario has "REQ-999"
  When "kotowari query REQ-999" is run
  Then the exit code is 2 and the first line of standard error is "argument error: unknown id: REQ-999"

@id=EX-core-252 @about=REQ-core-157 @source=docs/decision/records/2026-09-20-query-status.md#A5
Scenario: Two positional arguments stop
  Given there is an IR that can be checked
  When "kotowari query REQ-001 REQ-002" is run
  Then the exit code is 2 and the first line of standard error starts with "argument error: "

@id=EX-core-253 @about=REQ-core-156 @source=docs/decision/records/2026-09-20-query-status.md#A6
Scenario: Duplicate IDs are all output
  Given in the IR there is one requirement "REQ-001" in each of "docs/ir/a.md" and "docs/ir/b.md"
  When "kotowari query REQ-001" is run
  Then the exit code is 0 and "items" is two entries whose "path" are "docs/ir/a.md" and "docs/ir/b.md"

@id=EX-core-254 @about=REQ-core-161 @source=docs/decision/records/2026-09-19-read-commands.md#A7,docs/decision/records/2026-09-19-read-commands.md#A19,docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A7,docs/decision/records/2026-09-20-query-status.md#A13,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: text continues with the lines of the body and the reverse references
  Given there is the same IR as in EX-core-250, and in "tests/a.rs" the test "req_001_x" comes right after the mark "@kotowari[REQ-001]" on line 3
  When "kotowari query --format text REQ-001" is run
  Then the first line is "REQ-001 unit 例 docs/ir/a.md:7 tests=1", the second line is "  tests/a.rs:3 req_001_x", the third line is "  - kind: ubiquitous", and the last line is "  <- EX-001 about docs/ir/a.md:20"

@id=EX-core-255 @about=TBL-core-027 @source=docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-20-query-status.md#A16
Scenario: The body of a scenario starts at the tag line
  Given in the IR there is a scenario whose tag line "@id=EX-001 @about=REQ-001" is on line 19, whose "Scenario:" line is on line 20, and whose steps are on lines 21 to 23
  When "kotowari query EX-001" is run
  Then the "body" of the one entry of "items" is the five lines from line 19 to line 23, and its "referenced_by" is empty

@id=EX-core-256 @about=REQ-core-158 @source=docs/decision/records/2026-09-20-query-status.md#A19
Scenario: When the configuration cannot be read, it stops as check does
  Given the configuration file cannot be read as YAML
  When "kotowari query REQ-001" is run
  Then the exit code is 2 and the first line of standard error has the same wording as "kotowari check"

@id=EX-core-257 @about=TBL-core-027 @source=docs/decision/records/2026-09-20-query-status.md#A3,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: IDs in a definition and in a statement appear among the reverse references
  Given in the IR there are a decision table "TBL-001", a requirement "REQ-001" with the line "- definition: TBL-001", and a requirement "REQ-002" whose statement has "TBL-001" enclosed in backquotes
  When "kotowari query TBL-001" is run
  Then the "referenced_by" of the one entry of "items" is one entry whose "id" is "REQ-001" and "via" is "definition", and one entry whose "id" is "REQ-002" and "via" is "text"

@id=EX-core-277 @about=REQ-core-159,TBL-core-027 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A18,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: The body ends before the next heading and drops trailing blank lines
  Given in the "## Requirements" section of the IR there are requirements "REQ-001" and "REQ-002" in this order, after the statement of "REQ-002" a blank line and then the "## Properties" section follow, and the property "PROP-001" under it is the last item of the document
  When "kotowari query REQ-001", "kotowari query REQ-002" and "kotowari query PROP-001" are run
  Then the "body" of "REQ-001" runs up to the line before the heading of "REQ-002" and does not include the heading line of "REQ-002"
  And the "body" of "REQ-002" ends with the statement line and includes neither the trailing blank line nor the "## Properties" line
  And the "body" of "PROP-001" ends with the last non-blank line of the document

@id=EX-core-280 @about=REQ-core-159,TBL-core-027 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A34,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
Scenario: A line of the heading form inside a code block of the body does not cut the body
  Given after the statement of requirement "REQ-001" in the IR there is a block enclosed in "```" fences, inside it there is the line "## 例", and after the block the heading of requirement "REQ-002" follows
  When "kotowari query REQ-001" is run
  Then the "body" of "REQ-001" includes up to the closing line of the fence and does not end at the "## 例" line
```
