# How terms are written and the glossary table

English | [日本語](terms-form.ja.md)

Covers when to use double quotes and when to use backquotes, unclosed backquotes, and the extent of the glossary table.

## Requirements

### REQ-core-104: Concrete values are written in double quotes

- kind: ubiquitous
- source: docs/decision/records/records.md#A31
- verification: unit

A `statement` of the `IR` always writes concrete values in double quotes, and encloses only `term` and `ID` in backquotes.

### REQ-core-116: Unclosed backquotes

- kind: event_driven
- source: docs/decision/records/records.md#A116, docs/decision/records/records.md#A138, docs/decision/records/records.md#A145
- verification: unit

When the number of backquotes outside double quotes in a `target line` is odd, kotowari raises an unclosed_backtick `error` with the line's text as detail, does not perform the reference check of `term` and `ID` on that line, and does perform the `vague word` check.

### REQ-core-117: The extent of the glossary table

- kind: event_driven
- source: docs/decision/records/records.md#A112, docs/decision/records/records.md#A141, docs/decision/records/records.md#A148, docs/decision/records/2026-09-16-ir-tree.md#A3, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A39, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

The table of a `glossary` is, among the tables that start with a header row that matches the three columns "Term", "Meaning" and "Source" once the surrounding whitespace of each cell is removed, followed by a delimiter row whose every cell is three or more "-" (optionally with ":" before or after), the first one that appears after the title of the `glossary` document and before the first "## " heading; it ends at a blank line or a line that is not part of a table. Neither a table with a non-matching header before it nor any table after it, whether or not its header matches, is made into a `term` or a `finding` (`exclusion`). A line starting with "|" outside the table is not made into a `term`. When a `glossary` document exists but has no header and delimiter rows of this form, kotowari raises a glossary_invalid `error` with the document name as detail, treats that `glossary` as having zero `term` entries, and continues the check with the `term` entries of the other `glossary` files in the `chain` still visible. When the header and delimiter rows exist, the table is treated as present even with zero `term` rows.

### REQ-core-122: Broken rows of the glossary table

- kind: event_driven
- source: docs/decision/records/records.md#A153, docs/decision/records/records.md#A163, docs/decision/records/2026-09-23-ir-engine-gaps.md#A6, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16
- verification: unit

When the table of a `glossary` has a row with fewer than three cells (the parts split by "|" after removing the "|" at the start and end of the row) or a row whose `term` cell is empty, kotowari raises an invalid_glossary_row `error` with the row's text as detail, and does not make that row into a `term`. A row with four or more cells is not a broken row; it is made into a `term` from its first three cells, and the remaining cells are discarded.

### REQ-core-123: Duplicate terms

- kind: event_driven
- source: docs/decision/records/records.md#A154, docs/decision/records/records.md#A162, docs/decision/records/2026-09-16-ir-tree.md#A4
- verification: unit

When the `term` of a row of the table of a `glossary` is in an earlier row of the same `glossary`, or in a `glossary` on the side nearer the root of that `glossary`'s `chain`, kotowari raises a duplicate_term `error` for each such row with the `term` as detail, and does not count the duplicate row as a definition of a `term`. The word stays visible as a `term` through the first definition on the side nearer the root, and no new kind of `finding` is created. A duplicate row is not treated as an `item`, and does not undergo the `source` checks (missing_source, source_invalid) either.

## Examples

```gherkin
@id=EX-core-027 @about=REQ-core-123 @source=docs/decision/records/2026-09-16-ir-tree.md#A4
Scenario: Defining the same term above and below in the chain raises it on the lower row
  Given both "docs/ir/CONTEXT.md" and "docs/ir/network/CONTEXT.md" have a row for "宛先"
  When "kotowari check" is run
  Then one duplicate_term error is raised on the row of "docs/ir/network/CONTEXT.md" and none on "docs/ir/CONTEXT.md", and enclosing "宛先" in a document of "docs/ir/network/" raises no unknown_term

@id=EX-core-270 @about=REQ-core-117,REQ-core-174 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A25,docs/decision/records/2026-09-23-ir-engine-gaps.md#A27
Scenario: The first table with a matching header is the glossary table
  Given "CONTEXT.md" has, in this order and each separated by a blank line, a table whose header is "a", "b", "c", a table with a matching header that has a row for "宛先", and a second table with a matching header that has a row for "経路"
  And a `statement` of a `requirement` in another document has "宛先" and "経路" enclosed in backquotes
  When "kotowari check --format json" is run
  Then "CONTEXT.md" raises neither glossary_invalid, invalid_glossary_row nor unknown_line
  And "宛先" raises no unknown_term, and "経路" raises unknown_term

@id=EX-core-271 @about=REQ-core-117 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A25
Scenario: Without a single table with a matching header the glossary is invalid
  Given "CONTEXT.md" has only a table whose header is "a", "b", "c"
  When "kotowari check --format json" is run
  Then one glossary_invalid error is raised on "CONTEXT.md"

@id=EX-core-272 @about=REQ-core-122 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A6,docs/decision/records/2026-09-23-ir-engine-gaps.md#A16
Scenario: A four-column row becomes a term from its first three columns
  Given the table of the `glossary` of "CONTEXT.md" has a row whose `term` cell is "宛先" and that has a fourth cell after a correct source cell, and a row with two cells
  When "kotowari check --format json" is run
  Then no `finding` is raised for the four-cell row, and "宛先" is visible as a `term`
  And one invalid_glossary_row error is raised on the two-cell row
```
