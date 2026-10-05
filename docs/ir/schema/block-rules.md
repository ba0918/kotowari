# Rules for the lines under a heading

English | [日本語](block-rules.ja.md)

This document covers how the lines under a heading or the preamble are read: as a field line, a statement, a bullet, a table, or a code block.

## Requirements

### REQ-schema-028: Telling list lines apart

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A10
- definition: TBL-schema-007
- verification: property

### REQ-schema-029: Constraints on field line values

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A29
- verification: unit

mds always applies a regular expression and an allow list to the value of a `field line`, and, when a separator is declared, applies them to each separated element.

### REQ-schema-054: Constraints on statement and bullet values

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A31, docs/decision/records/2026-09-21-mds-spec.md#A32
- verification: unit

mds always applies a regular expression to the lines of a `statement` and a `bullet`, and also applies an allow list to a `statement`. A `bullet` is matched only on the part of the original marker line without the marker, never on a `continuation paragraph`.

### REQ-schema-030: A continuation paragraph is part of its line

- kind: state_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
- verification: unit

While the `reading mode` is "paragraph", mds reads a `continuation paragraph` as part of the preceding list line, does not count it as a `statement`, and does not make it a `finding` even in the `closed world`. While the `reading mode` is "line", no `continuation paragraph` is created (TBL-schema-011).

### REQ-schema-031: Nested bullets

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A34, docs/decision/records/2026-09-21-mds-spec.md#A67
- verification: unit

When a `bullet` has a child list, mds checks it against the child rules the `schema` declared, and, if none are declared, makes the child lines a `finding`. Child rules can declare a `field line` and a `bullet`, and a child `bullet` can in turn have child rules. Child lines are told apart the same way as in TBL-schema-007, with no name treated specially. A `schema` that declares an `extraction` on a child `bullet` is a `stop`.

### REQ-schema-032: How statements are counted

- kind: state_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A33
- verification: unit

While the `reading mode` is "paragraph", mds counts a paragraph delimited by blank lines as one `statement`, and does not count quotes, thematic breaks, HTML, or image-only lines as a `statement` nor make them a `finding` even in the `closed world`. How lines are counted while the `reading mode` is "line" is as in TBL-schema-011.

### REQ-schema-058: Telling lines apart per reading mode

- kind: algorithm
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A21, docs/decision/records/2026-09-23-ir-engine-gaps.md#A22, docs/decision/records/2026-09-23-ir-engine-gaps.md#A26, docs/decision/records/2026-09-23-ir-engine-gaps.md#A1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A2, docs/decision/records/2026-09-23-ir-engine-gaps.md#A5
- definition: TBL-schema-011
- verification: unit

### REQ-schema-033: Checking tables

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16, docs/decision/records/2026-09-23-ir-engine-gaps.md#A46
- verification: unit

mds always matches the header and the number of columns of a `table` only when the header cells are declared, and when they are not declared, looks only at whether the `table` exists and at its `cardinality`. When matching the number of columns, a data row with fewer cells than the header row of the `document` is made a `finding`. A data row with more cells than the header row of the `document` has its extra cells dropped and is not made a `finding`, regardless of whether the header cells are declared.

### REQ-schema-059: Selecting a table

- kind: event_driven
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A42
- verification: unit

When a `table` rule declares "select: first" together with "header", mds uses, within the `node` where the rule is placed, only the first `table` whose header matches the declaration as that rule's `table` for checking and `extraction`, and treats every other `table` as an undeclared `table`, whether or not its header matches. A `table` rule without "select" makes a `table` whose header does not match a `finding` of a shape violation. A `schema` that writes "select" without "header" is a `stop`. "select" accepts only the value "first"; a `schema` with any other value is a `stop`.

### REQ-schema-034: Checking code blocks

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A12, docs/decision/records/2026-09-21-mds-spec.md#A35
- verification: unit

mds always matches the language of a `code block` only when the language is declared, and, when a per-line regular expression is declared, matches only the non-empty lines with their leading whitespace removed.

### REQ-schema-041: Order of field lines

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A36
- verification: unit

When a list of `field line` declarations declares that its order is enforced, mds makes a `field line` that does not appear in the order written in the `schema` a `finding`. When this is not declared, the order is free.

## Decision tables

### TBL-schema-007: Telling list lines apart

- source: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A33, docs/decision/records/2026-09-24-review3-gaps.md#A2

| Order | Shape of the line | Read as |
|---|---|---|
| 1 | The marker is followed by "name and value", and the name matches a declaration of the `schema` | `field line` |
| 2 | Follows a marker but does not match 1 | `bullet` |
| 3 | Starts with a number and a "." or ")" delimiter | Belongs to no rule kind; a `finding` in the `closed world` |

### TBL-schema-011: Telling lines apart per reading mode

- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A2, docs/decision/records/2026-09-23-ir-engine-gaps.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A21, docs/decision/records/2026-09-23-ir-engine-gaps.md#A22, docs/decision/records/2026-09-23-ir-engine-gaps.md#A26, docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-21-mds-spec.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A29, docs/decision/records/2026-09-23-ir-engine-gaps.md#A35, docs/decision/records/2026-09-23-ir-engine-gaps.md#A33, docs/decision/records/2026-09-23-ir-engine-gaps.md#A38, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A43, docs/decision/records/2026-09-23-ir-engine-gaps.md#A44, docs/decision/records/2026-09-23-indented-fence.md#A1

Decides how the lines under a heading and the `preamble` are read in each `reading mode`. In either `reading mode`, list lines themselves are told apart as in TBL-schema-007, and child lists are as in REQ-schema-031.

| Shape of the line | "paragraph" | "line" |
|---|---|---|
| A CommonMark ATX heading line (up to three leading spaces, one to six "#", then whitespace or the end of the line; includes a line of only "#") | Heading | Heading |
| Any other line starting with "#" ("#foo", a line of seven or more "#") | `statement` | `statement` |
| A line of only "===" or "---" following a `statement` | One heading together with the previous line | Both the previous line and that line are a `statement` |
| Several consecutive lines, not separated by blank lines, that are neither a list nor a `table` | Together one `statement` | Each line one `statement` |
| A non-list line directly following a list line with no blank line between | Part of the preceding list line (for a `field line`, it goes into the value) | `statement` |
| An indented non-list line following a list line after a blank line | `continuation paragraph` | `statement` |
| An indented list line following a list line | List line | List line |
| A quote, thematic break, HTML, or image line | Not counted as a `statement`, and not made a `finding` | `statement` |
| A line indented by four or more spaces after a blank line, not following a list line | Indented `code block` | `statement` (a line opening a fence is as in the next row) |
| A block enclosed in a "```" or "~~~" fence | `code block` | `code block` (whatever the indentation depth of the opening line) |
| A GFM table (a run with a header row and a delimiter row; it need not start with a vertical bar; it ends as GFM specifies) | `table` | `table` |
| A line starting with a vertical bar that has no delimiter row and does not form a `table` | `statement` | `statement` |

## Properties

### PROP-schema-006: The marker kind does not change how lines are told apart

- source: docs/decision/records/2026-09-21-mds-spec.md#A10

Whether the list marker is "-", "*", or "+", the same line is read as the same `rule kind`.

## Examples

```gherkin
@id=EX-schema-011 @about=REQ-schema-028 @source=docs/decision/records/2026-09-21-mds-spec.md#A10
Scenario: A line with an undeclared name is read as a bullet
  Given a `schema` that declares the names of its `field line` rules
  When a `document` with a "name and value" line whose name is not declared is checked
  Then that line is read as a `bullet`

@id=EX-schema-012 @about=REQ-schema-033 @source=docs/decision/records/2026-09-21-mds-spec.md#A13
Scenario: A table with no declared header passes with any header
  Given a `schema` with a `table` rule that does not declare header cells
  When "kotowari-mds check" is run on a `document` with a `table` that has an arbitrary header
  Then no header `finding` is reported

@id=EX-schema-031 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A1,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
Scenario: Read by line, the line right after a field line becomes a statement
  Given a `schema` that declares "reading: line" and declares, on an `item`, an `extraction` for a `field line` and for a `statement`
  And a `document` with an `item` that has a non-list line right after a `field line`, with no blank line between
  When "kotowari-mds values --format json" is run
  Then the value of the `field line` is only the value of its own line and does not include the next line
  And the next line appears as an element of the `statement`

@id=EX-schema-032 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A2,docs/decision/records/2026-09-23-ir-engine-gaps.md#A5,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
Scenario: Read by line, an indented line after a blank line and a quote line become statements
  Given a `schema` that declares "reading: line" and has an `item` that declares no `statement`
  And a `document` whose `item` has, after a list line and a blank line, an indented line, a quote line, an HTML line, and an image-only line
  When "kotowari-mds check --format json" is run
  Then one `finding` whose undeclared line kind is `statement` is reported for each of the four lines

@id=EX-schema-033 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A21,docs/decision/records/2026-09-23-ir-engine-gaps.md#A22
Scenario: Read by line, no setext heading and no indented code block are created
  Given a `schema` that declares "reading: line" and declares a line-number `derived value` on the `statement` of an `item`
  And a `document` whose `item` has a line of only "---" following a `statement`, and a line indented by four spaces after a blank line
  When "kotowari-mds values --format json" is run
  Then the "---" line, the line before it, and the indented line all appear as elements of the `statement`
  And no heading or `code block` `finding` is reported

@id=EX-schema-034 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A26
Scenario: A vertical-bar line that does not form a table is a statement in either reading mode
  Given two variants, "reading: paragraph" and "reading: line", of a `schema` with an `item` that declares no `statement`
  And a `document` whose `item` has a "| a | b |" line with no delimiter row
  When "kotowari-mds check --format json" is run with each `schema`
  Then in both, one `finding` whose undeclared line kind is `statement` is reported for that line

@id=EX-schema-071 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-indented-fence.md#A1
Scenario: Read by line, a fence indented by four or more spaces is also a code block
  Given a `schema` that declares "reading: line" and declares a line-number `derived value` on the `statement` of an `item`
  And a `document` whose `item` has, after a `statement` and a blank line, a block enclosed in "```" lines indented by four spaces
  When "kotowari-mds values --format json" is run
  Then the only element of the `statement` is the preceding `statement`, and neither the fence lines nor the lines inside appear as elements of the `statement`

@id=EX-schema-035 @about=REQ-schema-030,REQ-schema-032 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A12,docs/decision/records/2026-09-21-mds-spec.md#A11,docs/decision/records/2026-09-21-mds-spec.md#A30
Scenario: Read by paragraph, a following line joins the list line and a quote line is not counted as a statement
  Given a `schema` that does not write "reading" and has an `item` that declares no `statement`
  And a `document` whose `item` has a line following a `field line` with no blank line between, an indented `continuation paragraph` after a blank line, and a quote line
  When "kotowari-mds check --format json" is run
  Then no `finding` is reported

@id=EX-schema-036 @about=REQ-schema-059 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A27,docs/decision/records/2026-09-23-ir-engine-gaps.md#A28
Scenario: select first uses only the first table whose header matches
  Given a `schema` that declares "header", "select: first", and an `extraction` on a `table` of the `preamble`
  And a `document` whose `preamble` has, in this order and separated by blank lines, a `table` whose header does not match, a `table` whose header matches, and a second `table` whose header matches
  When "kotowari-mds check --format json" and "kotowari-mds values --format json" are run
  Then the value of the `extraction` is only the data rows of the second `table` to appear
  And the first and third `table` each get a `finding` whose undeclared line kind is `table`, and no header shape-violation `finding` is reported

@id=EX-schema-037 @about=REQ-schema-059 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A27,docs/decision/records/2026-09-23-ir-engine-gaps.md#A28
Scenario: A table rule without select makes a table whose header does not match a finding
  Given a `schema` that declares only "header" on a `table` of the `preamble`, and a `schema` that declares "select: first" without writing "header"
  And a `document` whose `preamble` has a `table` whose header does not match
  When "kotowari-mds check" is run with each `schema`
  Then with the former, a header shape-violation `finding` is reported
  And with the latter, the exit code is 2

@id=EX-schema-050 @about=REQ-schema-059,TBL-schema-009 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A42
Scenario: A schema that writes a value other than first for select stops
  Given a `schema` that declares "header" and "select: last" on a `table` of the `preamble`
  When "kotowari-mds check" is run with that `schema`
  Then the exit code is 2

@id=EX-schema-038 @about=REQ-schema-033 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A16
Scenario: Cells beyond the header are dropped, and missing cells are a finding
  Given a `schema` with a `table` rule that declares a header of three cells
  And a `document` with a `table` that has a data row of four cells and a data row of two cells
  When "kotowari-mds check --format json" and "kotowari-mds values --format json" are run
  Then no `finding` is reported for the four-cell row, and the `extraction` value of that row has only its first three cells
  And one `finding` is reported for the two-cell row

@id=EX-schema-039 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A13,docs/decision/records/2026-09-23-ir-engine-gaps.md#A35
Scenario: Read by line, an indented list line stays a list line
  Given a `schema` that declares "reading: line" and declares only a `field line` on an `item`
  And a `document` whose `item` has an indented "  - b" line right after a `field line`
  When "kotowari-mds check --format json" is run
  Then that line gets a `finding` whose undeclared line kind is `bullet`, and no `finding` whose kind is `statement`

@id=EX-schema-051 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A43
Scenario: In either reading mode a heading is a CommonMark ATX heading
  Given two variants, "reading: paragraph" and "reading: line", of a `schema` that declares no heading
  And a `document` whose `preamble` has a "  ## x" line with two leading spaces, a "####### y" line, and a "#z" line
  When "kotowari-mds check --format json" is run with each `schema`
  Then in both, the "  ## x" line is a `finding` as a heading, the "####### y" and "#z" lines are a `finding` as a `statement`, and the two outputs are identical

@id=EX-schema-052 @about=REQ-schema-058,TBL-schema-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A44
Scenario: Read by line, a GFM table that does not start with a vertical bar is still a table
  Given a `schema` that declares an `extraction` on a `table` of the `preamble` and writes "reading: line"
  And a `document` whose `preamble` has an "a | b" line, a "--- | ---" line, and a "1 | 2" line in a row
  When "kotowari-mds values --format json" is run
  Then one `table` row is extracted, and its values are "1" and "2"
```
