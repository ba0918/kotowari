# The CLI and how results are output

English | [日本語](cli.ja.md)

This document covers the commands mds has, its exit codes, its output formats, the shape of a finding, and its behavior when a check cannot be performed.

## Requirements

### REQ-schema-005: The list of commands

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A1, docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A26, docs/decision/records/2026-09-23-versions-and-cli-name.md#A4, docs/decision/records/2026-09-24-ast-schema-output.md#A1
- verification: unit

mds always accepts, as a command named "kotowari-mds", these four: "check" for checking, "ast" for the syntax tree (with "--schema", the typed values of REQ-schema-067), "values" for extraction, and "--version" for the version.

### REQ-schema-006: How the exit code is decided

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A15
- definition: TBL-schema-001
- verification: unit

### REQ-schema-007: Output formats

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A44
- verification: unit

mds always takes the output format through "--format" and lets the caller choose between two: "text" for humans and "json" for machines. However, "ast" accepts only "json", and when given "text" it is a `stop`.

### REQ-schema-066: Indentation of the text output of values

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-mutants-gaps.md#A8
- verification: unit

mds always, in the "text" output of "values", indents by two spaces for each level of nesting, and prefixes each array element with a number starting at 1 followed by ".".

### REQ-schema-008: The shape of a finding

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A17, docs/decision/records/2026-09-21-mds-spec.md#A45, docs/decision/records/2026-09-21-mds-spec.md#A61, docs/decision/records/2026-09-22-ir-engine.md#A74, docs/decision/records/2026-09-22-ir-engine.md#A25, docs/decision/records/2026-09-22-ir-engine.md#A27, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A86, docs/decision/records/2026-09-23-ir-engine-gaps.md#A8
- verification: unit

mds always represents one `finding` by eight parts: kind, severity, path of the `document`, line number, `node name`, `raw line`, line kind, and detail; a `finding` that has no line omits the line number. The line number is the start line of the violating `node` if there is one, or, for a missing `node`, the start line of the `node` that contains it, and it is omitted when the containing `node` has no line. When an element inside a `node` violates, such as a data row of a `table`, it is the line of that element. The `node name` is given only to a `finding` on a `node` that has a declared name, and only a `section` and a `field line` have a declared name. The line kind is given to an undeclared_line `finding` as how the undeclared line was read (REQ-schema-055), and to a `finding` on the lower or upper bound of the `cardinality` as the `rule kind` of the counted `node` (REQ-schema-057), and to no other `finding`. The `raw line` is given only to a `finding` that has a line number.

### REQ-schema-009: Stop when a check cannot be performed

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A42, docs/decision/records/2026-09-21-mds-spec.md#A62, docs/decision/records/2026-09-21-mds-spec.md#P1
- verification: unit

When any of the `stop` reasons in TBL-schema-009 applies, mds comes to a `stop` without outputting partial results.

### REQ-schema-010: Checking a directory

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A1, docs/decision/records/2026-09-21-mds-spec.md#A27, docs/decision/records/2026-09-21-mds-spec.md#A60
- verification: unit

When the target of a check is a directory, mds collects and checks only each `document` under it that declares a `schema`. A `document` without "$schema" is out of scope, and a `document` that has "$schema" but whose value is empty or whitespace only is not taken as a declaration and is a `stop`.

### REQ-schema-042: Reasons to stop

- kind: algorithm
- source: docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A19
- definition: TBL-schema-009
- verification: unit

### REQ-schema-043: How a stop is reported

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A42
- verification: unit

When mds comes to a `stop`, it writes to standard error only one line giving the name of the reason and an explanation, and outputs no `finding` at all.

### REQ-schema-065: The explanation of a stop on a wrong value type or word

- kind: event_driven
- source: docs/decision/records/2026-09-23-mutants-gaps.md#A6
- verification: unit

When mds comes to a `stop` because a value in the `schema` or the `frontmatter` has the wrong type or word, mds includes in the explanation the name of the field at fault and the expected type or the list of accepted words.

### REQ-schema-053: A stop in the middle of a directory check

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A42
- verification: unit

When a reason to `stop` applies in the middle of a directory check, mds brings the whole check to a `stop` and outputs not a single `finding` collected so far.

### REQ-schema-044: What a directory check does not traverse

- kind: ubiquitous
- source: docs/decision/records/2026-09-21-mds-spec.md#A21
- verification: unit

mds always, in a directory check, does not traverse directories whose names start with ".", the locations of the `schema` files and the cache, symbolic links, or files whose extension is not ".md".

### REQ-schema-067: The output of "ast --schema"

- kind: event_driven
- source: docs/decision/records/2026-09-21-mds-spec.md#A46, docs/decision/records/2026-09-21-mds-spec.md#A56, docs/decision/records/2026-09-24-ast-schema-output.md#A1, docs/decision/records/2026-09-24-ast-schema-output.md#A2
- verification: unit

When "--schema" is given to "kotowari-mds ast", mds builds, with the `schema` the `document` declares, the same values as "kotowari-mds values --format json", and, if the `schema` declares a top-level "name", puts that value under the root "type" key, and outputs the result as JSON. When "name" is not declared, it does not put "type". "name" is used only for the "type" of "ast --schema" and does not change the results of "check" and "values".

## Decision tables

### TBL-schema-001: Exit codes

- source: docs/decision/records/2026-09-21-mds-spec.md#A15

| Order | Condition | Exit code |
|---|---|---|
| 1 | The check could not be performed (one of the `stop` reasons in TBL-schema-009 applied) | 2 |
| 2 | 1 does not apply, and there is at least one `finding` | 1 |
| 3 | Neither 1 nor 2 applies | 0 |

### TBL-schema-009: Reasons to stop

- source: docs/decision/records/2026-09-21-mds-spec.md#A8, docs/decision/records/2026-09-21-mds-spec.md#A15, docs/decision/records/2026-09-21-mds-spec.md#P1, docs/decision/records/2026-09-21-mds-spec.md#A16, docs/decision/records/2026-09-21-mds-spec.md#A25, docs/decision/records/2026-09-21-mds-spec.md#A43, docs/decision/records/2026-09-21-mds-spec.md#A44, docs/decision/records/2026-09-21-mds-spec.md#A62, docs/decision/records/2026-09-22-ir-engine.md#A56, docs/decision/records/2026-09-23-ir-engine-gaps.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A19, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A42, docs/decision/records/2026-09-23-mutants-gaps.md#A11, docs/decision/records/2026-09-24-review2-gaps.md#A2, docs/decision/records/2026-09-24-review4-gaps.md#A3, docs/decision/records/2026-09-24-review4-gaps.md#A4, docs/decision/records/2026-09-24-review4-gaps.md#A5, docs/decision/records/2026-09-24-review7-gaps.md#A4

| Reason | When |
|---|---|
| Schema not found | The referenced `schema` does not exist, fetching the URL failed (including when the response exceeds 4MiB and when the whole fetch exceeds 10 seconds), or a `document` without "$schema" was given as the target |
| Schema does not fit its shape | The YAML of the `schema` cannot be read; it violates the shape of a `rule kind`; `placement path` values collide (the same path within the same placement, or one being a level before the other; this is judged both inside an element object and outside one, such as directly under a `section`, the "value" and "of" keys of an element object also count as paths of the same placement, and paths that share a prefix and then diverge, like "a.b" and "a.c", do not collide); the value of "reading" is neither "paragraph" nor "line"; a `table` rule writes "select" without "header"; the value of "select" is not "first"; in a `schema` that declares "name", a `placement path` outside an element object starts with "type" or "type." (it collides with the key where "ast --schema" puts "name"); a `section` or `field line` with the same name is declared twice in the same placement; the same column name is written twice in the header of a `table`; or a dot-separated name in a `placement path` is empty |
| Frontmatter is broken | The `frontmatter` is broken YAML, is not a YAML mapping, the value of "$schema" is empty or whitespace only, or the value of "$schema" is not a string |
| Document cannot be read | The file of the `document` cannot be read |
| Wrong arguments | An unaccepted "--format" value, an unknown flag, the same option passed twice, or "--format text" given to "ast" |

### TBL-schema-002: Categories of findings

- source: docs/decision/records/2026-09-21-mds-spec.md#A17, docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A41, docs/decision/records/2026-09-21-mds-spec.md#A40

| Category | What it finds |
|---|---|
| Title | The `title` is missing, there are two or more, or it does not match the regular expression |
| Missing | A required `node` is missing |
| Shape violation | A value does not match the regular expression or the allow list, the depth of a heading does not match, the shape of the `ID` of an `item` does not match, the order of `field line` entries does not match, the header of a `table` does not match, or the language of a `code block` does not match |
| Cardinality | Below the lower bound of the `cardinality`, or above its upper bound |
| Closed world | There are undeclared headings and lines |

## Properties

### PROP-schema-002: Extraction is independent of whether the check passes

- source: docs/decision/records/2026-09-21-mds-spec.md#A4, docs/decision/records/2026-09-21-mds-spec.md#A56

For the same `document` and the same `schema`, the result of `extraction` does not change depending on whether the check reported a `finding`.

## Examples

```gherkin
@id=EX-schema-082 @about=REQ-schema-067 @source=docs/decision/records/2026-09-24-ast-schema-output.md#A1
Scenario: With a schema that declares name, ast --schema adds type
  Given a `document` that points at a `schema` which declares "name" as "adr" and extracts the `title` as an `extraction`
  When "kotowari-mds ast --schema" is run
  Then the output is the values of "values --format json" plus a root "type" key whose value is "adr"

@id=EX-schema-083 @about=REQ-schema-067 @source=docs/decision/records/2026-09-24-ast-schema-output.md#A1,docs/decision/records/2026-09-24-ast-schema-output.md#A2
Scenario: With a schema that does not declare name, ast --schema is the same as values
  Given a `document` that points at a `schema` which does not declare "name" and extracts the `title` as an `extraction`
  When "kotowari-mds ast --schema" and "kotowari-mds values --format json" are run
  Then the two outputs have the same values and no "type" key

@id=EX-schema-081 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review7-gaps.md#A4
Scenario: Passing the same option twice stops
  When "kotowari-mds check" is run with "--format" passed twice
  Then the exit code is 2

@id=EX-schema-076 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review4-gaps.md#A3
Scenario: A schema that declares a section or field line with the same name twice stops
  Given a `schema` that declares two `section` rules with the same name, and a `schema` that declares two `field line` rules with the same name in the `preamble`
  When "kotowari-mds check" is run with each `schema`
  Then the exit code is 2 for both

@id=EX-schema-078 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review4-gaps.md#A5
Scenario: A schema that writes the same column name twice in a table header stops
  Given a `schema` that writes the same column name twice in the header of a `table`
  When "kotowari-mds check" is run with that `schema`
  Then the exit code is 2

@id=EX-schema-077 @about=REQ-schema-036,TBL-schema-009 @source=docs/decision/records/2026-09-24-review4-gaps.md#A4
Scenario: A placement path that contains an empty name stops
  Given a `schema` whose `placement path` for the `title` is "a..b"
  When "kotowari-mds values" is run with that `schema`
  Then the exit code is 2

@id=EX-schema-073 @about=TBL-schema-009 @source=docs/decision/records/2026-09-24-review2-gaps.md#A2
Scenario: Putting a value at type in a schema that declares name stops
  Given a `schema` that declares "name" and whose `placement path` for the `extraction` of the `title` is "type"
  When "kotowari-mds values --format json" is run with that `schema`
  Then the exit code is 2
  And the same `schema` without "name" does not stop and puts the value of the `title` at "type"

@id=EX-schema-003 @about=REQ-schema-006 @source=docs/decision/records/2026-09-21-mds-spec.md#A15
Scenario: A document with no finding ends with exit code 0
  Given a `document` that satisfies the whole `schema`
  When "kotowari-mds check" is run
  Then the exit code is 0
  And nothing is output

@id=EX-schema-004 @about=REQ-schema-009 @source=docs/decision/records/2026-09-21-mds-spec.md#A8
Scenario: A broken frontmatter stops
  Given a `document` whose `frontmatter` is not a YAML mapping
  When "kotowari-mds check" is run
  Then the exit code is 2
  And no `finding` is output

@id=EX-schema-026 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A21,docs/decision/records/2026-09-22-ir-engine.md#A25
Scenario: The line of a finding is the start line of the violating node or of the node containing it
  Given a `document` with an `item` that lacks a required `field line`, an `item` with a `field line` whose value does not fit the declared shape, and a `table` with a row that lacks columns
  When "kotowari-mds check --format json" is run
  Then the line of the missing `finding` is the heading line of that `item`
  And the line of the `finding` on the ill-shaped `field line` is the line of that `field line`
  And the line of the `finding` on the `table` row that lacks columns is that row

@id=EX-schema-027 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A27
Scenario: A finding carries the declared node name
  Given a `document` with an `item` that lacks a required `field line`, and a `title` that does not fit the declared shape
  When "kotowari-mds check --format json" is run
  Then the `node name` of the `finding` on the `field line` is the name the schema declared
  And the `finding` on the `title` has no `node name`

@id=EX-schema-028 @about=REQ-schema-008 @source=docs/decision/records/2026-09-22-ir-engine.md#A29
Scenario: A finding with a line carries the raw line as it is
  Given a `document` with an `item` whose heading has an ID that does not fit the declared shape
  When "kotowari-mds check --format json" is run
  Then the `raw line` of that `finding` does not differ by a single character from that line of the `document`

@id=EX-schema-029 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-22-ir-engine.md#A56
Scenario: A schema whose keys overlap inside an element object stops
  Given a `schema` in which the `placement path` of an inner `field line` overlaps the key of an outer `derived value`
  When "kotowari-mds values" is run
  Then the exit code is 2
  And standard error reports that the `schema` does not fit its shape
  And it comes to a `stop` without reading the `document`

@id=EX-schema-030 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-22-ir-engine.md#A65
Scenario: Placement paths that only share the level above a dot are not duplicates
  Given a `schema` that lists "a.b" and "a.c", and a `schema` that lists "a" and "a.b"
  When "kotowari-mds values" is run on each
  Then the former does not `stop`, and the latter ends with exit code 2

@id=EX-schema-047 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A11,docs/decision/records/2026-09-23-ir-engine-gaps.md#A19
Scenario: A schema whose placement paths collide outside an element object stops
  Given a `schema` that declares an `extraction` with the same `placement path` on the `statement` directly under two `section` rules, and a `schema` that declares "a" and "a.b" split across two `section` rules
  When "kotowari-mds values" is run on each
  Then both exit with code 2, and standard error reports that the `schema` does not fit its shape

@id=EX-schema-048 @about=REQ-schema-042,TBL-schema-009 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A19
Scenario: Placement paths that diverge are not a collision even outside an element object
  Given a `schema` that declares "a.b" and "a.c" split across two `section` rules
  When "kotowari-mds values" is run
  Then it does not `stop`

@id=EX-schema-049 @about=REQ-schema-008 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A8,docs/decision/records/2026-09-22-ir-engine.md#A86
Scenario: A cardinality finding carries the rule kind of the counted node
  Given a `schema` that declares a lower bound of 1 in the `cardinality` of the `statement` of an `item`
  And a `document` with an `item` that has no `statement`
  When "kotowari-mds check --format json" is run
  Then the line kind of the `finding` below the lower bound is `statement`

@id=EX-schema-062 @about=REQ-schema-065 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A6
Scenario: A stop on a frontmatter value of the wrong type reports the field name and the expected type
  Given a `document` with a `frontmatter` whose "$schema" value is a number
  When "kotowari-mds check" is run
  Then the exit code is 2
  And the explanation on standard error includes "$schema" and the expected type

@id=EX-schema-063 @about=REQ-schema-065 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A6,docs/decision/records/2026-09-23-ir-engine-gaps.md#A20,docs/decision/records/2026-09-23-ir-engine-gaps.md#A30,docs/decision/records/2026-09-21-mds-spec.md#A15,docs/decision/records/2026-09-21-mds-spec.md#A42
Scenario: A stop on a schema with an unaccepted word reports the field name and the list of accepted words
  Given a `document` that points at a `schema` whose "reading" value is "foo"
  When "kotowari-mds check" is run
  Then the exit code is 2
  And the explanation on standard error includes "reading", "paragraph", and "line"

@id=EX-schema-064 @about=REQ-schema-066 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A8
Scenario: The text output of values indents by two spaces per level of nesting
  Given a `schema` that declares "a.b" as a `placement path`, and a `document` that has that value
  When "kotowari-mds values --format text" is run
  Then the "b" line is indented two spaces deeper than the "a" line

@id=EX-schema-065 @about=REQ-schema-066 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A8
Scenario: The text output of values numbers array elements starting at 1
  Given a `document` with a `table` that has two data rows, and a `schema` that extracts that `table` without declaring "header"
  When "kotowari-mds values --format text" is run
  Then the elements of the outer array start with "1." and "2.", and the elements of an array inside an element also start from "1."
  And the lines of the array elements are indented two spaces deeper than the line of the array's key
```
