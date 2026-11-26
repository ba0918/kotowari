# The skeleton of a document

English | [日本語](document-structure.ja.md)

This document covers how the schema validates the skeleton of a document: the title, the preamble, sections, and items.

## Requirements

### REQ-schema-022: Exactly one title

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-24-review2-gaps.md#A1, docs/decision/records/2026-09-24-review4-gaps.md#A1, docs/decision/records/2026-09-24-review7-gaps.md#A3
- verification: unit

When the `schema` declares a `title`, mds requires the `document` to have exactly one `title`; a missing one is one `finding`, and when there are two or more, it reports, for each `title` from the second on, one `finding` carrying the line number and `raw line` of that `title`. When the `schema` declares a regular expression for the `title`, the first `title` is matched against it even if there are two or more, and a mismatch also gets its `finding`. A `schema` that does not declare a `title` does not require one, and the `title` of the `document` becomes an undeclared heading. A second or later `title` ends the `section` and `item` open until then, and the lines after it are read as lines of the `preamble` until the next `section`.

### REQ-schema-023: What is declared on the preamble

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A68
- verification: unit

mds always gives the `preamble` itself no `extraction` key, and has it declared instead on each `node` inside the `preamble` (TBL-schema-004).

### REQ-schema-024: The name of a section

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-24-review7-gaps.md#A2
- verification: unit

mds always identifies a `section` by the text of its heading (the text with inline markup removed, keeping the text inside strikethrough), and makes a `section` that matches no name declared in the `schema` a `finding`.

### REQ-schema-025: The shape of an item heading

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A5
- definition: TBL-schema-006
- verification: unit

### REQ-schema-026: Headings that are too deep

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A5
- verification: unit

When there is a heading of depth 4 or more, mds makes that heading a `finding`.

### REQ-schema-027: Undeclared items

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A5
- verification: unit

When there is a depth-3 heading inside a `section` that declares no `item`, mds makes that heading and the lines inside it a `finding`. The same holds in the `open world`.

### REQ-schema-061: Items directly under the document

- kind: event_driven
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23, docs/decision/records/2026-09-23-ir-engine-gaps.md#A41, docs/decision/records/2026-09-23-ir-engine-gaps.md#A45, docs/decision/records/2026-10-06-todo-zero.md#A6
- verification: unit

When the `schema` declares an `item` directly under the document, mds reads a depth-3 heading before the first `section` as that `item`, and ends the `preamble` before the first `section` or `item`. The same `document` may have both an `item` under a `section` and an `item` directly under the document. In a `schema` that declares no `item` directly under the document, when the `preamble` is declared or in the `closed world`, a depth-3 heading before the first `section` and the lines inside it are made a `finding`. In the `open world` with no `preamble` declared, they are allowed as an undeclared structure (REQ-schema-002, REQ-schema-003). In a `schema` that declares no `item` directly under the document, a depth-3 heading before the `title`, including every one in a `document` with no `title`, is outside the `preamble`: in the `open world` it and the lines inside it are allowed as an undeclared structure even when the `preamble` is declared.

### REQ-schema-056: A missing required item

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A69, docs/decision/records/2026-09-24-review4-gaps.md#A2
- verification: unit

mds always, when an `item` is lacking, makes it a `finding` as falling below the lower bound, whether or not a `cardinality` is written. When none is written, the lower bound is 1, from the default of exactly one (TBL-schema-005). In this case no `finding` on a missing `section` is reported.

### REQ-schema-057: The rule kind in a cardinality finding

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A86
- verification: unit

mds always gives a `finding` on the lower or upper bound of a `cardinality` the `rule kind` of the `node` it counted.

## Decision tables

### TBL-schema-006: How an item heading is read

- source: docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A37

| Order | Shape of the heading | Read as |
|---|---|---|
| 1 | No separating colon | A `finding` that the ID does not fit its shape |
| 2 | Has a colon, and the part before it matches the regular expression of the `schema` | Read as an ID and a name |
| 3 | Has a colon, and the part before it does not match the regular expression | A `finding` that the ID does not fit its shape |

## Properties

### PROP-schema-005: The skeleton is closed at three levels of depth

- source: docs/decision/records/2026-09-21-mds-spec.md#A5

The heading depths a `schema` can declare are only three levels: the `title`, the `section`, and the `item`; there is no way to declare a further heading `node` under an `item`.

## Examples

```gherkin
@id=EX-schema-079 @about=REQ-schema-024 @source=docs/decision/records/2026-09-24-review7-gaps.md#A2
Scenario: Text inside strikethrough in a heading is included in the name
  Given a `schema` that declares a `section` "Ax"
  And a `document` with a `section` whose heading consists of "A" and "x" enclosed in strikethrough
  When "kotowari-mds check" is run
  Then the exit code is 0

@id=EX-schema-080 @about=REQ-schema-022 @source=docs/decision/records/2026-09-24-review7-gaps.md#A3
Scenario: A second title ends the item open until then
  Given a `schema` that extracts the `statement` of an `item` as an `extraction`
  And a `document` that has, after the `statement` of an `item`, a second `title` and lines after it
  When "kotowari-mds values --format json" is run
  Then the elements of the `statement` of the `item` are only the lines before the second `title`

@id=EX-schema-009 @about=REQ-schema-025 @source=docs/decision/records/2026-09-21-mds-spec.md#A5
Scenario: An item heading that does not fit the shape is an error
  Given a `schema` that declares a regular expression for the ID of an `item`
  When "kotowari-mds check" is run on a `document` with an ID that does not match the regular expression
  Then a `finding` on the shape of the ID is reported

@id=EX-schema-072 @about=REQ-schema-022 @source=docs/decision/records/2026-09-24-review2-gaps.md#A1
Scenario: Even with two titles, the first title is matched against the regular expression
  Given a `schema` that declares a regular expression for the `title`
  And a `document` that has, after a `title` not matching the regular expression, a second `title`
  When "kotowari-mds check --format json" is run
  Then a `finding` that it does not match the regular expression is reported on the line of the first `title`
  And a `finding` that there are several of the `title` is reported on the line of the second `title`

@id=EX-schema-074 @about=REQ-schema-022 @source=docs/decision/records/2026-09-24-review4-gaps.md#A1
Scenario: A schema that does not declare a title does not require one
  Given a `schema` that declares no `title` and declares a `statement` in the `preamble`
  And a `document` with no `title`
  When "kotowari-mds check" is run
  Then the exit code is 0

@id=EX-schema-075 @about=REQ-schema-056 @source=docs/decision/records/2026-09-24-review4-gaps.md#A2
Scenario: A missing item with no written cardinality is a finding below the lower bound
  Given a `schema` that declares, in a `section`, an `item` with no written `cardinality`
  And a `document` with no `item` in that `section`
  When "kotowari-mds check --format json" is run
  Then exactly one `finding` below the lower bound is reported on the line of that `section`

@id=EX-schema-010 @about=REQ-schema-022 @source=docs/decision/records/2026-09-21-mds-spec.md#A5
Scenario: A document with two titles is an error
  Given a `document` with two depth-1 headings
  When "kotowari-mds check" is run
  Then a `finding` that there are several of the `title` is reported

@id=EX-schema-042 @about=REQ-schema-022 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A17
Scenario: A document with three titles gets two findings
  Given a `document` with three depth-1 headings
  When "kotowari-mds check --format json" is run
  Then two of the `finding` that there are several of the `title` are reported
  And the line numbers and the `raw line` of the two are the lines of the second and third `title` and their exact text

@id=EX-schema-043 @about=REQ-schema-061 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A15,docs/decision/records/2026-09-23-ir-engine-gaps.md#A23
Scenario: Declaring an item directly under the document reads the item before a section
  Given a `schema` that declares, at "document.item" and at "item" under a `section`, an `item` of the same shape, each with an `extraction` at a different `placement path`
  And a `document` that has, after a `statement` in the `preamble`, an `item` with a depth-3 heading not inside any `section`, and an `item` under a `section`
  When "kotowari-mds check --format json" and "kotowari-mds values --format json" are run
  Then no `finding` is reported, and both of the `item` appear in the values of the `extraction`
  And the value of the `statement` of the `preamble` does not include the lines of the `item` directly under the document

@id=EX-schema-044 @about=REQ-schema-061 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A23
Scenario: Without an item declared directly under the document, a depth-3 heading before a section is a finding
  Given a `schema` that declares an `item` only under a `section`
  And a `document` with a depth-3 heading before the first `section`
  When "kotowari-mds check --format json" is run
  Then a `finding` is reported on that heading

@id=EX-schema-053 @about=REQ-schema-061 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A45
Scenario: In the open world with no preamble declared, a depth-3 heading before a section is allowed
  Given a `schema` with "open: true" that declares neither an `item` directly under the document nor a `preamble`
  And a `document` with a "### X-1: a" heading and lines under it before the first `section`
  When "kotowari-mds check --format json" is run
  Then no `finding` is reported on that heading or on the lines under it

@id=EX-schema-094 @about=REQ-schema-061 @source=docs/decision/records/2026-10-06-todo-zero.md#A6
Scenario: In a document with no title, a depth-3 heading is outside the declared preamble
  Given a `schema` with "open: true" that declares a `title` and a `preamble` and no `item` directly under the document
  And a `document` with no `title` that has a "### a" heading and a line under it before the first `section`
  When "kotowari-mds check --format json" is run
  Then the only `finding` is the one for the missing `title`
```
