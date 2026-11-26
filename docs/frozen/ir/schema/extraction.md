# Extraction and the bare syntax tree

English | [日本語](extraction.ja.md)

This document covers the behaviour of assembling values from the extraction rules a schema declares, and the behaviour of outputting the bare syntax tree separately from checking.

## Requirements

### REQ-schema-035: Format of extraction

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A4
- definition: TBL-schema-008
- verification: unit

### REQ-schema-036: Placement path

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A46, docs/decision/records/2026-09-24-review4-gaps.md#A4
- verification: unit

mds always places an extracted value (`extraction`) nested along the dot-separated names of its `placement path`. None of the separated names is empty.

### REQ-schema-037: Values are strings

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A23
- verification: property

mds always outputs the values of an `extraction` taken from a `document` as strings, and does not convert them to dates or numbers. Position information the engine derives is outside the scope of this rule.

### REQ-schema-038: A missing value outputs no key

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A47
- verification: unit

When the `node` targeted by an `extraction` is absent from the `document`, mds does not output the key of its `placement path`.

### REQ-schema-039: Inner extraction with no place to go

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A22, docs/decision/records/2026-09-21-mds-spec.md#A48, docs/decision/records/2026-09-21-mds-spec.md#A68
- verification: unit

When a `field line`, `statement`, `bullet`, `table` or `code block` inside an `item` declares an `extraction` and that `item` itself declares no `extraction`, mds performs a `stop`. A `field line` that is a child of a `bullet` also counts as inside the `item`.

### REQ-schema-047: Shape of the item object

- kind: state_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A22, docs/decision/records/2026-09-22-ir-engine.md#A52, docs/decision/records/2026-09-22-ir-engine.md#A60, docs/decision/records/2026-09-22-ir-engine.md#A69
- verification: unit

While a `node` inside an `item` declares an `extraction`, or while the `extraction` of the `item` declares a place for the `element value` or a `derived value`, mds assembles one object per `item` and resolves the inner `placement path` as a relative path within that object. While none of these holds, it assembles no object and uses the shorthand shape of TBL-schema-008.

### REQ-schema-048: Derived values

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A22, docs/decision/records/2026-09-21-mds-spec.md#A23, docs/decision/records/2026-09-21-mds-spec.md#A54, docs/decision/records/2026-09-21-mds-spec.md#A64, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-22-ir-engine.md#A45, docs/decision/records/2026-09-22-ir-engine.md#A49, docs/decision/records/2026-09-22-ir-engine.md#A52, docs/decision/records/2026-09-22-ir-engine.md#A53, docs/decision/records/2026-09-22-ir-engine.md#A56, docs/decision/records/2026-09-22-ir-engine.md#A58, docs/decision/records/2026-09-22-ir-engine.md#A69, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
- verification: unit

mds always accepts the declaration of the `extraction` of a `node` as one nested mapping, taking exactly one `placement path` in "path", the place for the `element value` in "value", and `derived value` entries in "of". A "group" named capture can be declared only on the `title`; a `schema` that declares it on any other `node`, and a `schema` whose `title` has no regular expression or whose regular expression does not contain the named capture, are a `stop`. The `derived value` entries are five: the line number, the ID of the `item` heading, the name of the `item` heading, the `raw line`, and the last line of the element ("end"; REQ-schema-062). The line number and the last line of the element are output as numbers, and any other word is a `stop`. The ID and the name of the `item` heading can be declared only on an `item`; a `schema` that declares them on a `node` outside an `item` is a `stop`. The last line of the element can be declared only on an `item` and a `section`; a `schema` that declares it on any other `node` is a `stop`. When a `node` declares "value" or a `derived value`, mds assembles one object per element of that `node` and resolves the keys of "value" and "of" and the `placement path` of the inner `node` entries as relative paths within that object.

### REQ-schema-062: Last line of the element

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
- verification: unit

mds always sets the "end" `derived value` of an `item` and a `section` to the line number of the line just before the next heading, after its own heading, that is of the same depth or shallower; if there is no such heading, to the line number of the last line of the `document`. Blank lines at the end of the range are included in the last line.

### REQ-schema-040: Bare syntax tree

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A19
- verification: review
- how_to_verify: Compare the output of "kotowari-mds ast" against the mdast (unist) specification and confirm that the names of the nodes' "type", the nesting of "children", and the kinds of inline elements conform. Conformance means agreement with an external specification, so our own tests cannot observe it

mds always outputs the bare syntax tree as JSON that follows mdast, including inline elements and excluding position information.

### REQ-schema-045: Extraction of a field line that declares a delimiter

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A50
- verification: unit

mds always extracts (`extraction`) a `field line` that declares a delimiter into an array regardless of its `cardinality` declaration, and when a range of `cardinality` is also declared, into an array of arrays. When a place for the `element value` or a `derived value` is also declared, it places this array under the key of the `element value`.

### REQ-schema-046: Order of splitting and continuation paragraphs

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-21-mds-spec.md#A50, docs/decision/records/2026-09-23-extract-original-lines.md#A7
- verification: unit

mds always splits by the delimiter only the value that does not include a `continuation paragraph`, and attaches the `continuation paragraph` to the last element of the split, joined as in REQ-schema-063.

### REQ-schema-063: Value lines stay as the original lines

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-23-extract-original-lines.md#A1, docs/decision/records/2026-09-23-extract-original-lines.md#A2, docs/decision/records/2026-09-23-extract-original-lines.md#A3, docs/decision/records/2026-09-23-extract-original-lines.md#A4, docs/decision/records/2026-09-23-extract-original-lines.md#A5, docs/decision/records/2026-09-23-extract-original-lines.md#A6
- verification: unit

mds always uses the lines that go into an `element value` as the original lines of the `document`, keeping indentation and the trailing whitespace of lines in the middle, and removing the trailing whitespace and blank lines at the end of the value. The value starts at the marker for a `bullet`, after the colon and whitespace following the name for a `field line`, and at the first non-whitespace character of the paragraph for a `statement`. A value made of several parts joins the included parts with one blank line if there was a blank line between them in the original `document`, and with one newline if not; the gap left by removing the lines of an excluded part, together with the blank lines next to it, is collapsed into one blank line; a line of only whitespace is treated as a blank line. This rule does not depend on the `reading mode`. The `element value` of a `statement` split into lines by declaring a `derived value` is outside the scope of this rule and follows TBL-schema-008.

### REQ-schema-064: Inline markup in heading names and cell values

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-mutants-gaps.md#A4
- verification: unit

mds always reads heading names and the cell values of a `table` as the text with the inline markup symbols removed. Emphasis, strong emphasis, links and reference links join the text inside them, and inline code keeps its contents.

## Decision tables

### TBL-schema-008: Shape of extraction

- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A13, docs/decision/records/2026-09-21-mds-spec.md#A65, docs/decision/records/2026-09-21-mds-spec.md#A66, docs/decision/records/2026-09-21-mds-spec.md#A50, docs/decision/records/2026-09-21-mds-spec.md#A51, docs/decision/records/2026-09-21-mds-spec.md#A52, docs/decision/records/2026-09-21-mds-spec.md#A53, docs/decision/records/2026-09-21-mds-spec.md#A55, docs/decision/records/2026-09-22-ir-engine.md#A43, docs/decision/records/2026-09-22-ir-engine.md#A44, docs/decision/records/2026-09-22-ir-engine.md#A48, docs/decision/records/2026-09-22-ir-engine.md#A53, docs/decision/records/2026-09-22-ir-engine.md#A57, docs/decision/records/2026-09-22-ir-engine.md#A59, docs/decision/records/2026-09-22-ir-engine.md#A64, docs/decision/records/2026-09-22-ir-engine.md#A69, docs/decision/records/2026-09-22-ir-engine.md#A70, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16, docs/decision/records/2026-09-23-extract-original-lines.md#A3, docs/decision/records/2026-09-23-mutants-gaps.md#A5, docs/decision/records/2026-09-23-mutants-gaps.md#A10

For any `node`, if neither "value" nor a `derived value` is declared, the "shorthand shape" below is output as is; if either is declared, an object is assembled per element, with the `element value` under the "value" key and the `derived value` entries under the "of" key.

| `node` | Unit of element | `element value` | Line the `raw line` points at | Shorthand shape |
|---|---|---|---|---|
| `title` | Not split | The heading text. When "group" is declared, the text captured by the named capture | The heading line of the `title` | The heading text. When "group" is declared, the named capture of the regular expression |
| `field line` | Not split | The value string (REQ-schema-063). When a delimiter is declared, the array of split strings (REQ-schema-045) | The line of that `field line` | The value string. When a delimiter is declared, the array of split strings (REQ-schema-045) |
| `statement` | Lines only when a `derived value` is declared; not split otherwise | When split into lines, the text of the line with leading and trailing whitespace removed; when not split, the body string (REQ-schema-063) | That line | The body string (REQ-schema-063) |
| `section` | Not split | The body string joining only the lines of `statement` and `bullet` entries (REQ-schema-063). `field line`, `table`, `code block` and `item` entries are not included | The heading line of the `section` | The body string joining only the lines of `statement` and `bullet` entries (REQ-schema-063). `field line`, `table`, `code block` and `item` entries are not included |
| `item` | `item` | The string joining the heading and the body with a newline. The heading is reduced to the `ID` and the name; the body includes `field line`, `statement` and `bullet` entries and excludes `table` and `code block` entries. A declared `field line` contributes its line and its `continuation paragraph` but not its child `bullet` entries; an undeclared line contributes its child lines as well. The body is joined as in REQ-schema-063 | The heading line of the `item` | An object keyed by the inner `placement path` entries (REQ-schema-047). When no object is assembled, the string joining the heading and the body with a newline. The heading is reduced to the `ID` and the name; the body includes `field line`, `statement` and `bullet` entries and excludes `table` and `code block` entries. A declared `field line` contributes its line and its `continuation paragraph` but not its child `bullet` entries; an undeclared line contributes its child lines as well. The body is joined as in REQ-schema-063 |
| `bullet` | Line | The original lines (including the lines of the `continuation paragraph` and of child `bullet` entries; REQ-schema-063) | The marker line of that line | An array of strings that keep the original lines (REQ-schema-063) |
| `table` | Data row | The values of the data row. Cells beyond the header row of the `document` are discarded (REQ-schema-033). The keys are the declared names if "header" is declared, otherwise the column positions (an array). When a `table` that declares a place for the `element value` or a `derived value` also declares a range of `cardinality`, a level per `table` is made directly under the `placement path`; otherwise the data rows of the `table` entries with the same place are joined, in order of appearance, into one array | The line of that data row | An array of rows. The keys of a row are the declared names if "header" is declared, otherwise the column positions (an array); the text of the header row of the `document` is not used as keys |
| `code block` | Block | The string of the whole block | The opening line of the fence | The string of the whole block |

## Properties

### PROP-schema-007: Extraction does not depend on the closed-world setting

- source: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A56, docs/decision/records/2026-09-21-mds-spec.md#A59

For the same `document` and the same `schema`, the result of `extraction` is the same whether checked in the `closed world` or the `open world`.

## Examples

```gherkin
@id=EX-schema-013 @about=REQ-schema-036 @source=docs/decision/records/2026-09-21-mds-spec.md#A4
Scenario: Output nested JSON along the placement path
  Given a `schema` declares a `placement path` that contains dots
  When "kotowari-mds values --format json" is run
  Then the values are output nested by the dot-separated names

@id=EX-schema-014 @about=REQ-schema-039 @source=docs/decision/records/2026-09-21-mds-spec.md#A12,docs/decision/records/2026-09-21-mds-spec.md#A48
Scenario: Extraction inside an item stops
  Given a `schema` in which the `item` itself declares no `extraction` and only a `table` inside the `item` declares an `extraction`
  When "kotowari-mds check" is run
  Then the exit code is 2

@id=EX-schema-018 @about=TBL-schema-008,REQ-schema-035 @source=docs/decision/records/2026-09-22-ir-engine.md#A43,docs/decision/records/2026-09-22-ir-engine.md#A44,docs/decision/records/2026-09-22-ir-engine.md#A48
Scenario: Keys of table rows come from the schema declaration or the column positions
  Given a `document` that has a `table` with an empty header cell and a `table` with two columns of the same name
  When "kotowari-mds values --format json" is run
  Then the rows of the `table` that declares "header" become objects keyed by the declared names
  And the rows of the `table` that does not declare "header" become arrays by column position
  And in neither `table` is any column value lost

@id=EX-schema-019 @about=TBL-schema-008,REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A13,docs/decision/records/2026-09-22-ir-engine.md#A59
Scenario: The line numbers of a table that declares derived values point at the data rows
  Given there are two `table` entries each with a header and three data rows, and a `schema` that declares a `cardinality` and a `derived value` on that `table`
  When "kotowari-mds values --format json" is run
  Then the line number of each row matches the line number of that data row of the `table`
  And directly under the `placement path` there is a level per `table`

@id=EX-schema-020 @about=REQ-schema-048,TBL-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A37,docs/decision/records/2026-09-22-ir-engine.md#A57,docs/decision/records/2026-09-22-ir-engine.md#A66
Scenario: A statement that declares derived values becomes one element per line
  Given there is an `item` with a three-line `statement` and a paragraph that follows after a blank line, and a `schema` that declares a `derived value` on the `statement`
  When "kotowari-mds values --format json" is run
  Then as many elements as lines are output
  And the `element value` of an indented line is the text with leading and trailing whitespace removed
  And the `raw line` of the same line is the text including indentation and trailing whitespace
  And the `continuation paragraph` of a list line does not go into the elements

@id=EX-schema-021 @about=TBL-schema-008,REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A55,docs/decision/records/2026-09-22-ir-engine.md#A63
Scenario: Shorthand extraction does not split into elements
  Given a `schema` that declares only the shorthands "extract: text" and "extract: rows"
  When "kotowari-mds values --format json" is run
  Then the value of the `statement` is one string, and the value of the `table` is a sequence of rows
  And no per-element objects are made

@id=EX-schema-022 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A45,docs/decision/records/2026-09-22-ir-engine.md#A52,docs/decision/records/2026-09-21-mds-spec.md#A15,docs/decision/records/2026-09-21-mds-spec.md#P1
Scenario: Extraction with no placement path stops
  Given a `schema` that declares an `extraction` without writing "path"
  When "kotowari-mds values" is run
  Then the exit code is 2

@id=EX-schema-023 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A60,docs/decision/records/2026-09-22-ir-engine.md#A56
Scenario: Omitting value leaves an element with only the keys of the derived values
  Given a `schema` that declares an `extraction` writing only "of" and no "value"
  When "kotowari-mds values --format json" is run
  Then the object of an element has only the keys of the `derived value` entries and the `placement path` entries declared by the inner `node` entries

@id=EX-schema-024 @about=REQ-schema-048 @source=docs/decision/records/2026-09-22-ir-engine.md#A17,docs/decision/records/2026-09-22-ir-engine.md#A49,docs/decision/records/2026-09-21-mds-spec.md#A23,docs/decision/records/2026-09-21-mds-spec.md#A15,docs/decision/records/2026-09-21-mds-spec.md#P1,docs/decision/records/2026-09-23-ir-engine-gaps.md#A18,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
Scenario: A derived value with an unaccepted word stops
  Given a `schema` that declares as a `derived value` a word that is none of the five
  When "kotowari-mds values" is run
  Then the exit code is 2

@id=EX-schema-025 @about=REQ-schema-048,TBL-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A49,docs/decision/records/2026-09-22-ir-engine.md#A50,docs/decision/records/2026-09-22-ir-engine.md#A54
Scenario: The raw line can be declared on any node
  Given a `schema` that declares the `raw line` and the line number on the `title` and on the rows of a `table`
  When "kotowari-mds values --format json" is run
  Then the `title` outputs its heading line, and a row of the `table` outputs that data row, as is

@id=EX-schema-045 @about=REQ-schema-062,REQ-schema-048 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A18,docs/decision/records/2026-09-23-ir-engine-gaps.md#A24
Scenario: The last line of an element is just before the next heading of the same depth or shallower
  Given a `schema` that declares the line number and the "end" `derived value` on `item` and `section`
  And a `document` whose first `section` has two `item` entries, where the second `section` follows the second `item` after a blank line and continues to the end of the `document`
  When "kotowari-mds values --format json" is run
  Then the "end" of the first `item` is the line before the heading of the second `item`
  And the "end" of the second `item` and of the first `section` is the blank line before the heading of the second `section`
  And the "end" of the second `section` is the last line of the `document`

@id=EX-schema-046 @about=REQ-schema-048 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A24,docs/decision/records/2026-09-21-mds-spec.md#P1
Scenario: The last line of an element outside item and section stops
  Given a `schema` that declares the "end" `derived value` in the `extraction` of a `table`
  When "kotowari-mds values" is run
  Then the exit code is 2

@id=EX-schema-054 @about=REQ-schema-063,TBL-schema-008 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A1,docs/decision/records/2026-09-23-extract-original-lines.md#A3,docs/decision/records/2026-09-23-extract-original-lines.md#A4
Scenario: The value of a bullet keeps indentation and blank lines as the original lines
  Given a `document` with a `bullet` that has a two-line lead paragraph, a `continuation paragraph` after a blank line, and child `bullet` entries after a blank line, and a `bullet` that has a lead paragraph and a `continuation paragraph` after a line with only the marker
  When "kotowari-mds values --format json" is run
  Then the value of each `bullet` holds the lines after the marker as the original lines, including indentation
  And one blank line each goes between the lead paragraph and the `continuation paragraph`, and between the `continuation paragraph` and the child `bullet` entries

@id=EX-schema-055 @about=REQ-schema-063,REQ-schema-046 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A1,docs/decision/records/2026-09-23-extract-original-lines.md#A2,docs/decision/records/2026-09-23-extract-original-lines.md#A3,docs/decision/records/2026-09-23-extract-original-lines.md#A7
Scenario: The value of a field line starts after the name and attaches the continuation paragraph with a blank line
  Given a `document` with a `field line` that has a value spanning two lines and a `continuation paragraph` after a blank line, and a `field line` of the same shape that declares a delimiter
  When "kotowari-mds values --format json" is run
  Then the value starts after the whitespace following the name and keeps the indentation of the second line
  And the `continuation paragraph` is attached, after a blank line, to the value or to the last element of the split

@id=EX-schema-056 @about=REQ-schema-063,TBL-schema-008 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A3,docs/decision/records/2026-09-21-mds-spec.md#A51
Scenario: The body of a section removes the excluded parts and collapses blank lines into one
  Given a `document` with a `section` laid out as `statement`, blank line, `field line`, blank line, `bullet`, which also has another sequence with no blank line between a `statement` and a `bullet`
  When "kotowari-mds values --format json" is run
  Then the blank lines left by removing the `field line` collapse into one
  And the `statement` and the `bullet` with no blank line between them are joined by one newline

@id=EX-schema-057 @about=REQ-schema-063 @source=docs/decision/records/2026-09-23-extract-original-lines.md#A6
Scenario: Reading by line does not add blank lines between the lines of a statement
  Given a `schema` that declares "reading: line" and declares no `derived value` in the `extraction` of the `statement`
  And a `document` that has a `statement` of two lines with no blank line between them and a third `statement` line after a blank line
  When "kotowari-mds values --format json" is run
  Then lines 1 and 2 are joined by one newline, and lines 2 and 3 by one blank line

@id=EX-schema-058 @about=REQ-schema-064 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A4
Scenario: Heading names and cell values become the text with inline markup symbols removed
  Given a `document` that has an `item` heading whose name contains strong emphasis, a link, a reference link, inline code and emphasis, and a `table` whose cells contain strong emphasis, a reference link and inline code
  And a `schema` that declares the heading-name `derived value` on the `item` and an `extraction` on the `table`
  When "kotowari-mds values --format json" is run
  Then the heading name and the cell values become the text with the symbols removed and the inner text joined, and inline code keeps its contents

@id=EX-schema-059 @about=REQ-schema-064 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A4,docs/decision/records/2026-09-21-mds-spec.md#A2
Scenario: A heading whose text without symbols differs from the declared name is a finding
  Given a `schema` that declares the name of a `section` as "Req", and a `document` with the heading "## **Req** x"
  When "kotowari-mds check" is run
  Then the heading is read as "Req x" and becomes a `finding` as an undeclared heading

@id=EX-schema-060 @about=TBL-schema-008,REQ-schema-035 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A5,docs/decision/records/2026-09-23-mutants-gaps.md#A10
Scenario: The body of an item does not include the child lines of a declared field line
  Given a `document` with an `item` that has a declared `field line` with child `bullet` entries, and a `schema` that declares a shorthand `extraction` on the `item`
  When "kotowari-mds values --format json" is run
  Then the body of the `item` includes the line of that `field line` and does not include the lines of the child `bullet` entries

@id=EX-schema-061 @about=TBL-schema-008,REQ-schema-035 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A5,docs/decision/records/2026-09-23-mutants-gaps.md#A10
Scenario: The body of an item includes the child lines of an undeclared line
  Given a `document` with an `item` that has a list line, with child `bullet` entries, whose name is not declared in the `schema`, and a `schema` that declares a shorthand `extraction` on the `item`
  When "kotowari-mds values --format json" is run
  Then the body of the `item` includes that line and the lines of the child `bullet` entries
```
