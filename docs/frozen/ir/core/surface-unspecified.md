# List of unspecified surfaces

English | [日本語](surface-unspecified.ja.md)

Covers the place and form of the `list of unspecified surfaces`, how its entries are matched with a `surface`, and the `finding` entries on one entry of the list.

## Requirements

### REQ-core-231: The place of the list

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: unit

In "kotowari check" or "kotowari status", when "surface.rules" is not an empty list and the configuration has no "surface.unspecified" key, or when what it points at is empty (0 bytes or only comments), kotowari continues with the `list of unspecified surfaces` as zero entries. When what the key points at is missing or cannot be read, kotowari makes a `stop` with an unreadable file as the reason. When what it points at is not UTF-8, kotowari makes a `stop` with a non-UTF-8 file as the reason. When what it points at cannot be read as YAML, or when its top level is not a sequence, kotowari makes a `stop` with a configuration error as the reason, and outputs in the detail the relative path of the file of the `list of unspecified surfaces`. When "surface.rules" is an empty list and there is a "surface.unspecified" key, it makes a `stop` as in REQ-core-225.

### REQ-core-232: Matching an entry of the list

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A22
- verification: unit

kotowari always takes a well-formed entry of the `list of unspecified surfaces` to match a `surface` when "kind" and the kind of the `surface`, and "name" and the name of the `surface`, are both the same strings without removing surrounding whitespace, and does not raise an `error` of surface_without_spec for a matched `surface` even when it is not in the `IR`. Two or more entries with the same content are not in themselves checked.

### REQ-core-233: A malformed entry

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22
- verification: unit

In "kotowari check" or "kotowari status", when an entry of the `list of unspecified surfaces` is not a set of key-value pairs, when any of the keys "kind", "name" and "why" is missing, when it has a key other than these three, when a value is not a string, or when "why" is empty after removing leading and trailing half-width spaces and tabs, kotowari raises, per entry, an `error` of surface_unspecified_invalid with "path" set to the file of the `list of unspecified surfaces`, "line" set to null, and detail set to the "kind" and "name" as written in the list, separated by one half-width space, and does not let that entry match any `surface`. The "kind" and "name" of the detail are empty strings when missing or not a string.

### REQ-core-234: An entry no longer needed

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22
- verification: unit

In "kotowari check" or "kotowari status", when no `surface` matches a well-formed entry of the `list of unspecified surfaces`, or when the matching `surface` is in the `IR`, kotowari raises, per entry, a `notice` of surface_unspecified_stale with "path" set to the file of the `list of unspecified surfaces`, "line" set to null, and detail set to the "kind" and "name" as written in the list, separated by one half-width space. It is not raised for a malformed entry.

## Examples

```gherkin
@id=EX-core-419 @about=REQ-core-232,REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A16,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: A surface in the list is not an error and is counted among those set aside
  Given in the scene of EX-core-408, the `list of unspecified surfaces` has one entry whose "kind" is "flag", whose "name" is "--verbose", and whose "why" is not empty
  When "kotowari check --format json" is run
  Then no error of surface_without_spec is raised, "unspecified" of "surface" in the JSON is 1, and the exit code is 0

@id=EX-core-420 @about=REQ-core-232 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: An entry of a different kind does not match
  Given in the scene of EX-core-419, the "kind" of the entry of the `list of unspecified surfaces` is "subcommand"
  When "kotowari check --format json" is run
  Then an error of surface_without_spec with detail "flag --verbose" is raised

@id=EX-core-421 @about=REQ-core-233 @source=docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: An entry whose reason is only whitespace is an error and does not set the surface aside
  Given in the scene of EX-core-419, the "why" of the entry of the `list of unspecified surfaces` is only half-width spaces
  When "kotowari check --format json" is run
  Then an error of surface_unspecified_invalid whose "line" is null and whose detail is "flag --verbose", and an error of surface_without_spec with detail "flag --verbose" are raised
  And no notice of surface_unspecified_stale is raised

@id=EX-core-422 @about=REQ-core-234 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: An entry for a surface gone from the code is a notice
  Given the `list of unspecified surfaces` has a well-formed entry whose "kind" is "flag" and whose "name" is "--old", and no `surface file` has a `surface` named "--old"
  When "kotowari check --format json" is run
  Then a notice of surface_unspecified_stale whose "line" is null and whose detail is "flag --old" is raised

@id=EX-core-423 @about=REQ-core-234 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A16,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: An entry for a surface written into the IR is a notice
  Given in the scene of EX-core-419, "--verbose" enclosed in double quotes has been added to a `statement` of a `requirement`
  When "kotowari check --format json" is run
  Then a notice of surface_unspecified_stale with detail "flag --verbose" is raised, and "unspecified" of "surface" in the JSON is 0

@id=EX-core-424 @about=REQ-core-231 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: When what the list points at is missing, it stops
  Given in the scene of EX-core-407, "surface.unspecified" of the configuration is "docs/surface.yaml", and that file does not exist
  When "kotowari check" is run
  Then the exit code is 2 and the first line of standard error starts with "unreadable file: "

@id=EX-core-425 @about=REQ-core-231 @source=docs/decision/records/2026-09-27-surface-check.md#A6,docs/decision/records/2026-09-27-surface-check.md#A22
Scenario: A list whose top level is not a sequence stops, showing the list's path
  Given in the scene of EX-core-407, "surface.unspecified" of the configuration is "docs/surface.yaml", and its content is the one line "kind: flag"
  When "kotowari check" is run
  Then the exit code is 2 and the first line of standard error starts with "config error: docs/surface.yaml"
```
