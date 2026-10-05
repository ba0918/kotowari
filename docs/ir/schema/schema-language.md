# Skeleton of the schema language

English | [日本語](schema-language.ja.md)

This document covers which rule kinds a schema has, and how to write cardinality and conditional rules.

## Requirements

### REQ-schema-016: Shape of the schema

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A3, docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23
- verification: unit

mds always reads a `schema` as a YAML mapping, and accepts four root `node` kinds: the `title`, the `preamble`, the `section`, and the `item` directly under the document. An `item` directly under the document is declared in "document.item", in the same shape as "item" under a `section`.

### REQ-schema-017: List of rule kinds

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A3, docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A68
- definition: TBL-schema-004
- verification: review
- how_to_verify: Read and confirm that the public structs of `crates/kotowari-markdown-schema/src/schema.rs` correspond one to one with the rows of TBL-schema-004, that no rule kind outside the table exists, and that where each struct can be placed matches the "Where it can be placed" column of the table. A rule kind added outside the table would still pass the check, so a machine cannot see this

### REQ-schema-060: Declaring the reading mode

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A30
- verification: unit

mds always accepts the top-level "reading" key of a `schema` as the declaration of the `reading mode`, accepts only the two values "paragraph" and "line", and reads as "paragraph" when the key is not written. A `schema` whose value is neither "paragraph" nor "line" is a `stop`.

### REQ-schema-018: Unknown keys

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#P1
- verification: unit

When a `schema` has a key that the rule kind does not accept, mds performs a `stop` without checking.

### REQ-schema-019: How to write cardinality

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A40
- definition: TBL-schema-005
- verification: unit

### REQ-schema-020: Conditional rules

- kind: state_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A28
- verification: unit

While the condition of a `conditional rule` is true, mds applies the constraint it is attached to, and while it is false, mds does not apply it.

### REQ-schema-021: Finding the field line a condition refers to

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A28
- verification: unit

mds always looks for the `field line` that a `conditional rule` refers to only under the same `node`, and when it is not found, treats an equality condition as false and an inequality condition as true.

## Decision tables

### TBL-schema-004: Rule kinds

- source: docs/decision/records/2026-09-21-mds-spec.md#A3, docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A12, docs/decision/records/2026-09-21-mds-spec.md#A34, docs/decision/records/2026-09-21-mds-spec.md#A38, docs/decision/records/2026-09-21-mds-spec.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A68, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23

| Rule kind | What it validates | Where it can be placed |
|---|---|---|
| `title` | The depth-1 heading | The root of the `schema` |
| `preamble` | The part after the `title` and before the first `section`. When an `item` directly under the document is declared, the part before the first `section` or `item` | The root of the `schema` |
| `section` | A depth-2 heading | The root of the `schema` |
| `item` | A depth-3 heading | Under a `section`; the root of the `schema` (an `item` directly under the document) |
| `field line` | A list line of the name-and-value form | `preamble`, `section`, `item`, child of a `bullet` |
| `statement` | A non-blank line that is neither a list nor a `table` | `preamble`, `section`, `item` |
| `bullet` | A list line that is not a `field line` | `preamble`, `section`, `item`, child of a `bullet` |
| `table` | A Markdown table | `preamble`, `section`, `item` |
| `code block` | A block enclosed by a fence | `preamble`, `section`, `item` |

### TBL-schema-005: How to write cardinality

- source: docs/decision/records/2026-09-21-mds-spec.md#A40

| How it is written | Meaning |
|---|---|
| Default (nothing written) | Exactly one |
| The required declaration set to false | Zero or one |
| Only the lower bound written | That number or more |
| Only the upper bound written | From zero up to that number |
| Both lower and upper bounds written | That range |

## Properties

### PROP-schema-004: The cardinality declaration decides the shape of extraction

- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A49, docs/decision/records/2026-09-21-mds-spec.md#A50, docs/decision/records/2026-09-21-mds-spec.md#A55

The `extraction` of a `node` that declares a range of `cardinality` is an array even when there is only one value. The `extraction` of a `node` that declares no range is a single value. This correspondence applies to a `field line` that declares no delimiter, and to `statement`, `section`, `item`, `title` and `code block`. A `bullet` and a `table` are always arrays regardless of the `cardinality` declaration, and the value of a `field line` that declares a delimiter is also always an array.

### PROP-schema-008: The default reading mode is paragraph

- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20

For the same `document`, a `schema` that does not write "reading" and the same `schema` with only "reading: paragraph" added return the same sequence of `finding` entries and the same `extraction` values.

## Examples

```gherkin
@id=EX-schema-007 @about=REQ-schema-020 @source=docs/decision/records/2026-09-21-mds-spec.md#A28
Scenario: Required only while the condition is true
  Given a `schema` that declares a `field line` that is required only when the value of another `field line` is a particular value
  When that `field line` is removed from a `document` that meets the condition and "kotowari-mds check" is run
  Then a `finding` for the missing line is reported

@id=EX-schema-008 @about=REQ-schema-018 @source=docs/decision/records/2026-09-21-mds-spec.md#P1
Scenario: A schema with an unknown key stops
  Given a `schema` that writes a key the rule kind does not accept
  When "kotowari-mds check" is run
  Then the exit code is 2

@id=EX-schema-040 @about=REQ-schema-060,PROP-schema-008 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A12,docs/decision/records/2026-09-23-ir-engine-gaps.md#A20,docs/decision/records/2026-09-21-mds-spec.md#A30
Scenario: A schema that does not write reading reads by paragraph
  Given a `schema` that does not write "reading" and a `schema` that writes "reading: paragraph", both declaring a `cardinality` upper bound of 1 on the `statement` of an `item`
  And a `document` with an `item` that has a two-line paragraph with no blank line between the lines
  When "kotowari-mds check --format json" and "kotowari-mds values --format json" are run with each `schema`
  Then no `finding` is reported with either (the two-line paragraph counts as one `statement`), and the outputs of "kotowari-mds check" match each other and the outputs of "kotowari-mds values" match each other

@id=EX-schema-041 @about=REQ-schema-060 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A20,docs/decision/records/2026-09-23-ir-engine-gaps.md#A30
Scenario: A reading with an unaccepted value stops
  Given a `schema` that writes "reading: word"
  When "kotowari-mds check" is run
  Then the exit code is 2
```
