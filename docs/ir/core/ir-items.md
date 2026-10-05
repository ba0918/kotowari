# The form of items

English | [日本語](ir-items.ja.md)

Covers the form of requirements, decision tables, properties, scenarios and flag records, and the checks of the lines under a heading.

## Requirements

### REQ-core-042: The form of items

- kind: algorithm
- source: docs/decision/records/records.md#A27, docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/records.md#A52
- definition: TBL-core-011
- verification: unit

### REQ-core-043: A heading that does not fit the form

- kind: event_driven
- source: docs/decision/records/records.md#A52, docs/decision/records/records.md#A82, docs/decision/records/records.md#A111, docs/decision/records/2026-09-22-ir-engine.md#A84, docs/decision/records/2026-09-24-review8-gaps.md#A2
- verification: unit

When a "### " heading is not of the form "### ID: name", an `ID` of REQ, TBL, PROP or FLAG followed by a name (including when there is no name after ":"), kotowari raises an unknown_heading `error`. The same applies when an `ID` of EX is used in a heading, and to headings deeper than "#### ". The lines under a heading that does not fit the form are also read by the rules of an `item`, and the applicable findings are raised.

### REQ-core-044: Unknown lines

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A52, docs/decision/records/records.md#A81, docs/decision/records/ir-form.md#項目, docs/decision/records/records.md#A87, docs/decision/records/records.md#A111, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-review3-gaps.md#A2
- verification: unit

When there is an unknown "- xxx:" line under a heading, or a list line not of the "xxx:" form (a line starting with "- ", "* ", "+ ", or digits followed by "." or ")", and a line of only "-"), kotowari raises an unknown_field `error` with the characters of the line as read as the detail. It does not read the contents of an unknown line (even if an `ID` is written there, it is not taken as a reference). The known lines are, for each kind of `item`, only those in the "lines it has" column of TBL-core-011; for a `property`, only "- source:".

### REQ-core-045: Duplicates of the same line

- kind: event_driven
- source: docs/decision/records/records.md#A52, docs/decision/records/records.md#A113, docs/decision/records/2026-09-22-ir-engine.md#A75, docs/decision/records/2026-09-24-review8-gaps.md#A3
- verification: unit

When there are two or more of the same known "- xxx:" line under a heading, kotowari raises one duplicate_field `error` on the second line. Even with three or more it is one. The value read is that of the first line; the values of the second and later lines are not read. Unknown lines raise only unknown_field even when duplicated.

### REQ-core-046: How the lines under a heading are read

- kind: ubiquitous
- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A52, docs/decision/records/ir-form.md#項目, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A2
- verification: unit

kotowari always reads the "- " lines under a heading in any order, allows blank lines between them, and reads the values of "- definition:", "- related:", "- source:" and "- deferred:" separated by commas.

### REQ-core-178: Statements are read one line at a time

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A2, docs/decision/records/2026-09-23-ir-engine-gaps.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A21, docs/decision/records/2026-09-23-ir-engine-gaps.md#A22, docs/decision/records/2026-09-23-ir-engine-gaps.md#A26, docs/decision/records/2026-09-23-ir-engine-gaps.md#A35, docs/decision/records/2026-09-23-ir-engine-gaps.md#A38, docs/decision/records/2026-09-23-ir-engine-gaps.md#A43
- verification: unit

kotowari always, by the declarations of the embedded schemas (REQ-core-179), reads each non-blank line under a heading that is neither a list line nor a table line as a `statement`, one line at a time. A "- name:" line and other list lines end at one line; a line that follows directly after one without a blank line, and an indented non-list line that follows after a blank line, are also read as a `statement` and are not included in the value of the list line. An indented list line is read as a list line. Quote, horizontal rule, HTML and image lines, and lines starting with "|" that have no delimiter row and do not form a table, are also read as a `statement`. Headings are only CommonMark ATX heading lines (at most three spaces at the line head, one to six "#", followed by whitespace or the end of the line); a line of only "---" or "===" is a `statement` together with the line before it. A `code block` is only one that starts with a fence line; a line indented by four or more spaces after a blank line is a `statement`.

### REQ-core-047: No statement

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A131, docs/decision/records/2026-09-22-ir-engine.md#A75, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

When a `requirement` whose kind is other than "algorithm" (including a `requirement` with no "- kind:" line), a `property`, or the `item` of a `flag record` has no `statement`, kotowari raises a missing_statement `error`.

### REQ-core-048: No verification line

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

When a `requirement` has no "- verification:" line, kotowari raises a verification_missing `error`.

### REQ-core-049: An invalid verification value

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類
- verification: unit

When the verification value of a `requirement` is none of "unit", "property", "proof" and "review", kotowari raises a verification_invalid `error`.

### REQ-core-050: An invalid kind value

- kind: event_driven
- source: docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類
- verification: unit

When the kind of a `requirement` or of the `item` of a `flag record` is not a value decided in TBL-core-011, kotowari raises an unknown_kind `error`.

### REQ-core-051: An algorithm without a definition

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-22-ir-engine.md#A71, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

When a `requirement` whose kind is "algorithm" has no "- definition:" line pointing at a `decision table` or a `property` (including when the line itself is missing), kotowari raises an algorithm_without_definition `error`.

## Decision tables

### TBL-core-011: The form of items

- source: docs/decision/records/records.md#A27, docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#文書, docs/decision/records/2026-09-19-read-commands.md#A5, docs/decision/records/2026-09-19-read-commands.md#A11, docs/decision/records/2026-09-20-query-status.md#A10, docs/decision/records/2026-09-23-ir-engine-gaps.md#A4, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-review8-gaps.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A2

| Item | Where it is placed | Heading | Lines it has | Statements |
|---|---|---|---|---|
| Requirement | Under ## Requirements | ### REQ-nnn: name | - kind: (event_driven, state_driven, ubiquitous, prohibition, invariant, algorithm), - source:, - verification: (unit, property, proof, review), - definition: (present for algorithm; others may have it), - how_to_verify: (present when "- verification:" is review; others may have it; free text of the procedure a person or an LLM follows to confirm), - deferred: (present only when put under `deferral`; sources separated by commas) | Present except for algorithm. An algorithm need not have them, and may have them |
| Decision table | Under ## Decision tables | ### TBL-nnn: name | - source: and a Markdown table | Need not have them, and may have them (it has a table) |
| Property | Under ## Properties | ### PROP-nnn: name | - source: | Present |
| Scenario | In a gherkin code block under ## Examples | The Scenario: line | The tags on the line just before: @id=EX-nnn, @about=ID,..., @source=source,... | None (it has step lines) |
| Flag record | Under ## Flags of a flag record document, or directly under the document with no section in between (the same document may have both) | ### FLAG-nnn: name | - kind: (contradiction, gap, ambiguity), - related: (IDs separated by commas), - source: | Has a body |
| Term | A glossary document | A row of the first table with a three-column header of Term, Meaning and Source (REQ-core-117) | None | None (it has the meaning column) |

## Examples

```gherkin
@id=EX-core-290 @about=REQ-core-043 @source=docs/decision/records/2026-09-24-review8-gaps.md#A2
Scenario: A heading without a name does not fit the form
  Given under the heading "### REQ-001:" there is a `requirement` with the required lines and a statement
  When "kotowari check" is run
  Then unknown_heading is raised on that heading line, and "REQ-001" is not counted as defined

@id=EX-core-291 @about=TBL-core-011 @source=docs/decision/records/2026-09-24-review8-gaps.md#A1
Scenario: An algorithm requirement and a decision table may have statements
  Given there are an algorithm `requirement` with a statement and a `decision table` with a statement before its table
  When "kotowari check" is run
  Then no `finding` on the form is raised

@id=EX-core-292 @about=REQ-core-045 @source=docs/decision/records/2026-09-24-review8-gaps.md#A3
Scenario: When the same line appears twice, the value of the first is read
  Given there is a `requirement` that has "- kind: algorithm" followed by "- kind: ubiquitous", and has neither a statement nor a definition
  When "kotowari check" is run
  Then duplicate_field and algorithm_without_definition are raised, and missing_statement is not raised

@id=EX-core-008 @about=REQ-core-044 @source=docs/decision/records/records.md#A42
Scenario: An unknown line is an error
  Given there is a line "- 優先度: 高" under the heading of a `requirement`
  When "kotowari check" is run
  Then one unknown_field error is raised

@id=EX-core-273 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A1,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13,docs/decision/records/2026-09-23-ir-english-tokens.md#A2,docs/decision/records/ir-form.md#検査の種類
Scenario: A statement directly after a line is not part of the line's value
  Given in a `requirement` whose verification is "review", a `statement` is on the line after "- how_to_verify: 見る" with no blank line between, and a `statement` is also on the line after "- verification: review" with no blank line between
  When "kotowari check --format json" is run
  Then none of missing_statement, verification_invalid and requirement_without_test is raised for that `requirement`

@id=EX-core-274 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A2,docs/decision/records/2026-09-23-ir-engine-gaps.md#A5,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: An unclosed backquote in an indented line, a quote and an HTML line is an error
  Given after the "- source:" line of a `requirement` there is, after a blank line, an indented line, below it a quote line and an HTML line, and each of the three lines has an unclosed backquote
  When "kotowari check --format json" is run
  Then one unclosed_backtick error is raised on each of the three lines

@id=EX-core-275 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A26
Scenario: A vertical-bar line that does not form a table is checked as a statement
  Given under a `requirement` there is a line starting with "| a |" that has no delimiter row and contains an unclosed backquote
  When "kotowari check --format json" is run
  Then one unclosed_backtick error is raised on that line, and unknown_line is not raised

@id=EX-core-276 @about=REQ-core-042,TBL-core-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A4,docs/decision/records/2026-09-23-ir-engine-gaps.md#A15,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: The items of a flag record without a section are read too
  Given "FLAGS.md" has no "## Flags" section, and after the `title` there is the `item` "### FLAG-001: 例" with its "- kind:", "- related:" and "- source:" lines and a body
  And another "FLAGS.md" has "### FLAG-002: 例" with no section and "### FLAG-003: 例" under "## Flags"
  When "kotowari check --format json" is run
  Then none of unknown_heading, unknown_field and unknown_line is raised
  And the number of flag record items in "kotowari status" is 3
```
