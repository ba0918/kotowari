# Closed world and open world

English | [日本語](closed-world.ja.md)

This document covers how undeclared headings and lines are handled, and how far that can be relaxed.

## Requirements

### REQ-schema-001: The closed world is the default

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A2
- verification: unit

mds always makes headings and lines not declared in the `schema` a `finding`.

### REQ-schema-002: Relaxing to the open world

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A2
- verification: unit

When the `schema` declares "open: true", or when "--open" is given to the check, mds allows undeclared structures and all the lines inside them.

### REQ-schema-003: The inside of declared structures is not relaxed

- kind: prohibition
- source: docs/decision/records/2026-09-21-mds-spec.md#A2
- verification: unit

mds does not allow, even in the `open world`, undeclared structures and lines added inside a declared `preamble`, `section`, or `item`.

### REQ-schema-004: Missing nodes and shape violations are not relaxed

- kind: prohibition
- source: docs/decision/records/2026-09-21-mds-spec.md#A2
- verification: unit

mds does not allow, even in the `open world`, a missing required `node`, or a violation of the `cardinality` or of a shape.

### REQ-schema-055: The kind of a line outside the declarations

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A74
- verification: unit

mds always gives a `finding` on an undeclared line the kind it read that line as: a list line in name-and-value form, a `bullet`, an ordered list, a `statement`, a `table`, or a `code block`. This spares the caller from re-reading the characters of the `raw line` to decide the kind.

## Properties

### PROP-schema-001: It only ever relaxes

- source: docs/decision/records/2026-09-21-mds-spec.md#A2

Every `finding` reported in the `open world` is also among those reported when the same `document` is checked in the `closed world`; the former set is contained in the latter.

## Examples

```gherkin
@id=EX-schema-001 @about=REQ-schema-002 @source=docs/decision/records/2026-09-21-mds-spec.md#A2
Scenario: The open world allows an undeclared section
  Given a `document` with a `section` not declared in the `schema`
  When "kotowari-mds check --open" is run
  Then no `finding` is reported for that `section`

@id=EX-schema-002 @about=REQ-schema-003 @source=docs/decision/records/2026-09-21-mds-spec.md#A2
Scenario: Even in the open world, an undeclared line inside a declared section is an error
  Given a `document` with an undeclared line inside a declared `section`
  When "kotowari-mds check --open" is run
  Then a `finding` is reported for that line
```
