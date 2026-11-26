# Base directory

English | [日本語](base-directory.ja.md)

Covers how the directory that relative paths are based on is decided, and what is handled relative to that base.

## Requirements

### REQ-core-009: How the base directory is decided

- kind: algorithm
- source: docs/decision/records/records.md#A37
- definition: TBL-core-003, PROP-core-001
- verification: unit

### REQ-core-010: Paths relative to the base

- kind: ubiquitous
- source: docs/decision/records/records.md#A13, docs/decision/records/records.md#A37
- verification: unit

kotowari always treats the paths in the values of the `configuration file`, the paths of a `source`, and the "path" of the output as paths relative to the `base directory`.

### REQ-core-110: Path normalization

- kind: ubiquitous
- source: docs/decision/records/records.md#A106, docs/decision/records/2026-09-16-ir-tree.md#A13, docs/decision/records/2026-09-24-review7-gaps.md#A1
- verification: unit

kotowari always normalizes the paths in the values of the `configuration file` and the paths of a `source`, before comparing them and before output, by removing a trailing "/" and a leading "./", collapsing an inner "/./" and consecutive "/" into a single "/", turning "\\" into "/", and collapsing "a/.." (a ".." with nothing to collapse against is kept), and builds the "path" of the output by joining the normalized location and the document's path relative to that location with "/".

## Decision tables

### TBL-core-003: Order in which the base directory is searched

- source: docs/decision/records/records.md#A37, docs/decision/records/records.md#A124

| Order | Condition | Base directory |
|---|---|---|
| 1 | Going up from the current directory, a directory containing a ".kotowari/" directory is found (a file named ".kotowari" is ignored and the search continues upward) | The first directory found |
| 2 | Nothing is found in 1 | The current directory |

## Properties

### PROP-core-001: The configuration path does not change the base

- source: docs/decision/records/records.md#A37

The `base directory` does not change with the path given to "--config".

## Examples

```gherkin
@id=EX-core-289 @about=REQ-core-110 @source=docs/decision/records/2026-09-24-review7-gaps.md#A1
Scenario: An "a/.." in the location is collapsed before comparing
  Given a configuration sets decisions.records to "docs/decision/../decision/records"
  And there is a `requirement` whose source is a decision of a `decision record` in that location
  When "kotowari check" is run
  Then source_invalid is not raised

@id=EX-core-002 @about=REQ-core-009 @source=docs/decision/records/records.md#A37
Scenario: The .kotowari of a directory above becomes the base
  Given "/repo/.kotowari/" exists, and "/repo/src/" has no ".kotowari/"
  When "kotowari check" is run in "/repo/src/"
  Then the `base directory` is "/repo"
```
