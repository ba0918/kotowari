# Schema resolution

English | [日本語](schema-resolution.ja.md)

This document covers how the schema that a document's frontmatter points at is found, fetched and cached.

## Requirements

### REQ-schema-011: Resolving the schema reference

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A20, docs/decision/records/2026-09-23-mutants-gaps.md#A7
- definition: TBL-schema-003
- verification: unit

### REQ-schema-012: Base of relative paths

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A14
- verification: unit

mds always resolves a relative path written in the `frontmatter` against the location of the `document`. The `base directory` is not used to resolve relative paths.

### REQ-schema-013: Caching a URL schema

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A20, docs/decision/records/2026-09-23-mutants-gaps.md#A9, docs/decision/records/2026-09-24-kotowari-dir.md#A3
- verification: unit

When the `frontmatter` points at a `schema` by URL, mds places the fetched content in the cache under its SHA-256 name and reads the cache from then on. If the cache is corrupt, it fetches again and recovers. The cache is placed in ".kotowari/cache/schemas/" of the `base directory`, and if there is no `base directory`, in ".kotowari/cache/schemas/" of the current directory.

### REQ-schema-014: A document that does not point at a schema

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A43, docs/decision/records/2026-09-21-mds-spec.md#P1
- verification: unit

When the `frontmatter` is not a YAML mapping, when the value of "$schema" is empty or only whitespace, or when the value of "$schema" is not a string, mds performs a `stop`.

### REQ-schema-015: Extra keys in the frontmatter

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A1
- verification: unit

mds always ignores keys of the `frontmatter` other than "$schema", and does not make them a `finding` either.

### REQ-schema-052: Masking credentials in a URL

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A25
- verification: unit

When putting a URL in the explanation of a `stop`, mds masks the credentials in its authority.

## Decision tables

### TBL-schema-003: Resolving the schema reference

- source: docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A27, docs/decision/records/2026-09-21-mds-spec.md#A43, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-23-mutants-gaps.md#A7

| Order | Value of "$schema" | Resolution |
|---|---|---|
| 1 | Absent (when a file is given as the target; for a directory see REQ-schema-010), empty, only whitespace, not a string, or the `frontmatter` is not a mapping | `stop` |
| 2 | Starts with "http://" or "https://" | Fetch it and place it in the cache. A `stop` when it cannot be fetched, when the response exceeds 4MiB, or when the whole fetch exceeds 10 seconds |
| 3 | Anything else | Read it as a path relative to the location of the `document`. A `stop` if it cannot be read |

## Properties

### PROP-schema-003: The resolution does not change with the command

- source: docs/decision/records/2026-09-21-mds-spec.md#A14

The "$schema" of the same `document` resolves to the same `schema` whether it is read by the check, `extraction` or bare-syntax-tree command.

## Examples

```gherkin
@id=EX-schema-005 @about=REQ-schema-012 @source=docs/decision/records/2026-09-21-mds-spec.md#A14
Scenario: A relative path is resolved from the location of the document
  Given a `document` that points by relative path at a `schema` located away from the `document`
  When "kotowari-mds check" is run
  Then the `schema` is resolved from the location of the `document`
  And the exit code is 0

@id=EX-schema-017 @about=REQ-schema-052 @source=docs/decision/records/2026-09-21-mds-spec.md#A25
Scenario: A URL containing credentials is output masked
  Given a `document` that points at a `schema` by a URL containing credentials, and the fetch fails
  When "kotowari-mds check" is run
  Then no credentials appear on standard error
  And the URL appears in masked form

@id=EX-schema-006 @about=REQ-schema-013 @source=docs/decision/records/2026-09-21-mds-spec.md#A14,docs/decision/records/2026-09-21-mds-spec.md#A15
Scenario: A corrupt cache is fetched again and recovered
  Given a `document` that points at a URL `schema` and fully satisfies that `schema`, and a corrupt cache
  When "kotowari-mds check" is run
  Then the `schema` is fetched again
  And the exit code is 0

@id=EX-schema-066 @about=TBL-schema-003,REQ-schema-011 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A7
Scenario: A URL schema exceeding 4MiB stops
  Given a `document` that points at a `schema` by a URL that returns a response exceeding 4MiB
  When "kotowari-mds check" is run
  Then the exit code is 2

@id=EX-schema-067 @about=TBL-schema-003,REQ-schema-011 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A7
Scenario: A URL schema whose whole fetch exceeds 10 seconds stops
  Given a `document` that points at a `schema` by a URL that has not finished responding after 10 seconds
  When "kotowari-mds check" is run
  Then the exit code is 2

@id=EX-schema-068 @about=TBL-schema-003,REQ-schema-011 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A7,docs/decision/records/2026-09-21-mds-spec.md#A15
Scenario: A URL schema fetched within the limits does not stop
  Given a `document` that points at a `schema` by a URL that returns a response of 4MiB or less within 10 seconds, and fully satisfies that `schema`
  When "kotowari-mds check" is run
  Then the exit code is 0

@id=EX-schema-069 @about=REQ-schema-013 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A9,docs/decision/records/2026-09-24-kotowari-dir.md#A1,docs/decision/records/2026-09-24-kotowari-dir.md#A3
Scenario: The cache of a URL schema is placed under the base directory
  Given the current directory is a subdirectory under a directory that has ".kotowari/", and there is a `document` that points at a URL `schema`
  When "kotowari-mds check" is run
  Then the cache is placed in ".kotowari/cache/schemas/" of the directory that has ".kotowari/", and not under the current directory

@id=EX-schema-070 @about=REQ-schema-013 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A9,docs/decision/records/2026-09-24-kotowari-dir.md#A1,docs/decision/records/2026-09-24-kotowari-dir.md#A3
Scenario: Without a base directory the cache is placed under the current directory
  Given there is no ".kotowari/" anywhere from the current directory upward, and there is a `document` that points at a URL `schema`
  When "kotowari-mds check" is run
  Then the cache is placed in ".kotowari/cache/schemas/" of the current directory
```
