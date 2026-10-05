# Checking surfaces

English | [日本語](surface.ja.md)

Covers how `surface` entries are taken out of the code by the configuration's "surface.rules" and "surface.files" and "kotowari check" confirms whether the name of each `surface` appears in the `IR`, and how the counts of `surface` entries are output. The place of the `list of unspecified surfaces` and the handling of one of its entries are covered by surface-unspecified.md.

## Requirements

### REQ-core-223: Taking out surfaces

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A5, docs/decision/records/2026-09-27-surface-check.md#A8, docs/decision/records/2026-09-27-surface-check.md#A19, docs/decision/records/2026-09-27-surface-check.md#A20
- verification: unit

When "surface.rules" is not an empty list, kotowari always applies each `surface rule` to the `surface file` entries whose language, determined from the extension as in TBL-core-031, is the same as the "language" of the `surface rule`, and takes each syntax-tree node matched by a `surface rule` as one `surface`. "language" is matched without regard to case, as in REQ-core-189, and ast-grep's aliases ("ts", "py") are accepted too. The kind of a `surface` is the "id" of its `surface rule`, and the name is the text of the node captured in the metavariable "$NAME". When that text starts and ends with the same quote (one of a single quote, a double quote and a backquote), one character is removed from each end; any other form becomes the name as it is. A match that does not capture "$NAME" is not made a `surface`. kotowari bundles no `surface rule`.

### REQ-core-224: Reading surface rules and surface files

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A29, docs/decision/records/2026-09-27-surface-check.md#A25
- verification: unit

kotowari always reads the paths listed in "surface.rules" as paths relative to the `base directory`, reads their content, as in REQ-core-186, as ast-grep rule YAML (several `surface rule` entries may be listed, separated by "---"), and does not add a `surface rule` to the `query` entries. It applies the "files" and "ignores" of a `surface rule`, read the same way as in REQ-core-187, to the path of each `surface file` relative to the `base directory`, leaves "fix", "message", "severity", "note" and "metadata" unused in taking out `surface` entries, and applies a `surface rule` whose "severity" is "off" too. The reading of the globs of "surface.files", the walk, the `exclusion` and the `stop` on a symbolic link without a target follow "tests.files", and the `stop` when a `surface file` that REQ-core-236 reads cannot be read or is not UTF-8 follows that of a `test file` (REQ-core-019, REQ-core-079, REQ-core-018).

### REQ-core-236: Surface files made into trees

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A29
- verification: unit

kotowari always reads, and makes into trees with tree-sitter, only the `surface file` entries whose language, determined from the extension as in TBL-core-031, is the same as the "language" of some `surface rule`; when there is even one syntax error, it raises an `error` of unparsable_file as in REQ-core-083 and skips that file. When unparsable_file has already been raised for the same path as a `test file`, it is not raised again. It does not read the other `surface file` entries, raises no unparsable_file for them, and makes neither their being unreadable nor their not being UTF-8 a reason for a `stop`.

### REQ-core-225: Errors in the surface configuration

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A12, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: unit

When "surface.rules" is not an empty list and "surface.files" is an empty list, when "surface.files" is not an empty list and "surface.rules" is an empty list, or when the configuration has a "surface.unspecified" key and "surface.rules" is an empty list, kotowari makes a `stop` with a configuration error as the reason in every command that reads the `configuration file`. In "kotowari check" and "kotowari status", when the file at a "surface.rules" path meets any of the conditions of REQ-core-189, kotowari makes a `stop` with the same reason and wording as for "tests.rules".

### REQ-core-226: Surfaces in the IR

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A2, docs/decision/records/2026-09-27-surface-check.md#A13, docs/decision/records/2026-09-27-surface-check.md#A18
- verification: unit

kotowari always takes a `surface` to be in the `IR` when its name appears, as the content enclosed in a pair of double quotes or a pair of backquotes, in a `statement` of a `requirement`, a cell of the table of a `decision table` (including the cells of the header row), or a step line of a `scenario`, in any `topic document`. Double quotes are paired from the left of the line in order, and backquotes outside double quotes are paired as in REQ-core-064. The name is taken to appear only when the enclosed content, without removing surrounding whitespace, is the same string as the name; the name being only part of the content does not count. Names appearing in a `statement` of a `requirement` under `deferral` are counted too. Names appearing in a `statement` of a `property`, in the lines of a `scope`, in lines starting with "- " under the heading of an `item` (including the "- how_to_verify:" line), in the body of a `flag record`, or in a `glossary` are not counted.

### REQ-core-227: Findings on surfaces not in the IR

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A7, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A19
- verification: unit

In "kotowari check" or "kotowari status", when a `surface` is not in the `IR` and matches no well-formed entry of the `list of unspecified surfaces`, kotowari raises, only once per set of `surface` entries with the same kind and name, an `error` of surface_without_spec with "path" set to the `surface file` of the first `surface` among them in byte order of the path of the `surface file` and then in ascending order of line, "line" set to the first line of that `surface`'s node, and detail set to the kind and the name separated by one half-width space. When "surface.rules" is an empty list, kotowari takes out no `surface` and does not raise this `error`.

### REQ-core-228: Outputting the count set aside

- kind: event_driven
- source: docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A16
- verification: unit

In "kotowari check", when "surface.rules" is not an empty list, kotowari outputs in the top-level "surface" of the JSON an object with the single key "unspecified", and when "--format" is "text", outputs "surface: unspecified=count" as the last line after the lines of the `finding` entries. The count is the number of pairs of kind and name of the `surface` entries that are not in the `IR` and matched a well-formed entry of the `list of unspecified surfaces`. It is output even when there are zero `finding` entries, and even when the count is 0. When "surface.rules" is an empty list, neither the "surface" key of the JSON nor the text line is output.

### REQ-core-229: Commands that read surfaces

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A10, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: unit

kotowari always reads the `surface file` entries, the `surface rule` files and the `list of unspecified surfaces` only in "kotowari check" and "kotowari status", and in "kotowari status" performs the same `surface` check as check and outputs the "surface" counts of TBL-core-028. "kotowari list", "kotowari query", "kotowari plan" and "kotowari mutants" do not read them and make no `stop` caused by reading them. The output of "kotowari list" does not include the counts of `surface` entries.

### REQ-core-230: How the skill writes about surfaces

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A5, docs/decision/records/2026-09-27-surface-check.md#A11, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A26, docs/decision/records/2026-09-27-surface-check.md#A1, docs/decision/records/2026-09-27-surface-check.md#A6
- verification: review
- how_to_verify: Confirm that "agent/skills/kotowari/references/surface.md" exists and contains how to write and how to fix for the surface check and, as an example of writing surface rules, rules that take out clap flags and subcommands; that "agent/skills/kotowari/references/config.md" lists the keys "surface.files", "surface.rules" and "surface.unspecified", and "agent/skills/kotowari/references/findings.md" lists the kinds surface_without_spec, surface_unspecified_invalid and surface_unspecified_stale; and that, as the fix for surface_without_spec, only kotowari-brainstorm and kotowari-adopt add an entry to the list of unspecified surfaces, while for the implementer it says only to return a surface not in the IR to brainstorm

The kotowari skill under "agent/skills/" always has, in an independent reference, how to write and how to fix for the `surface` check, includes examples of writing a `surface rule`, and lists the configuration keys of `surface` and the kinds of `finding` in the existing references.

## Examples

```gherkin
@id=EX-core-407 @about=REQ-core-223,REQ-core-226,REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A1,docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A8,docs/decision/records/2026-09-27-surface-check.md#A16,docs/decision/records/2026-09-27-surface-check.md#A19
Scenario: A surface whose name appears in a statement of a requirement of the IR raises no finding
  Given "surface.files" is "src/**/*.rs", a `surface rule` with "language: rust" and "id: flag" in "surface.rules" matches the string literal "--format" on line 3 of "src/cli.rs" and captures it in "$NAME", and a `statement` of a `requirement` has "--format" written enclosed in double quotes
  When "kotowari check --format json" is run
  Then no error of surface_without_spec is raised, and "unspecified" of "surface" in the JSON is 0

@id=EX-core-408 @about=REQ-core-227 @source=docs/decision/records/2026-09-27-surface-check.md#A3,docs/decision/records/2026-09-27-surface-check.md#A7,docs/decision/records/2026-09-27-surface-check.md#A15
Scenario: A surface that does not appear in the IR is an error at the surface's place
  Given the `surface rule` of EX-core-407 also matches the string literal "--verbose" on line 12 of "src/cli.rs", nowhere in the `IR` is "--verbose" the enclosed content, and there is no `list of unspecified surfaces`
  When "kotowari check --format json" is run
  Then one error of surface_without_spec is raised whose "path" is "src/cli.rs", "line" is 12 and detail is "flag --verbose", and the exit code is 1

@id=EX-core-409 @about=REQ-core-227 @source=docs/decision/records/2026-09-27-surface-check.md#A7,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A19
Scenario: The same surface in two places gives only one finding, at the first place
  Given in the scene of EX-core-408, both line 12 and line 30 of "src/cli.rs" have the string literal "--verbose"
  When "kotowari check --format json" is run
  Then the error of surface_without_spec with detail "flag --verbose" is only one, whose "line" is 12

@id=EX-core-410 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: A name that is only part of the enclosed content is not taken to be in the IR
  Given in the scene of EX-core-408, a `statement` of a `requirement` has "--verbose true" written enclosed in double quotes
  When "kotowari check --format json" is run
  Then an error of surface_without_spec with detail "flag --verbose" is raised

@id=EX-core-411 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A13,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: Names appearing in cells of a decision table, statements of deferred requirements and steps of scenarios are counted too
  Given a `surface rule` takes out three `surface` entries named "list", "query" and "plan", and "list" is written in a cell of a `decision table`, "query" in a `statement` of a `requirement` under `deferral`, and "plan" in a step line of a `scenario`, each enclosed in double quotes
  When "kotowari check --format json" is run
  Then no error of surface_without_spec is raised

@id=EX-core-412 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: The name of a term enclosed in backquotes is counted too
  Given a `glossary` has the term "status", a `surface rule` takes out a `surface` named "status", and a `statement` of a `requirement` has that term written enclosed in backquotes
  When "kotowari check --format json" is run
  Then neither an error of surface_without_spec nor an error of unknown_term is raised

@id=EX-core-413 @about=REQ-core-227,REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A3,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A16
Scenario: Nothing happens to a project that writes no surface rules
  Given the configuration has neither the "surface.rules" key nor the "surface.files" key
  When "kotowari check --format text" is run
  Then no error of surface_without_spec is raised, and no line starting with "surface: " is output either

@id=EX-core-414 @about=REQ-core-228 @source=docs/decision/records/2026-09-27-surface-check.md#A4,docs/decision/records/2026-09-27-surface-check.md#A16
Scenario: The count set aside appears on the last line even with zero findings
  Given in the scene of EX-core-407 there is no other `finding`
  When "kotowari check --format text" is run
  Then standard output is only the one line "surface: unspecified=0"

@id=EX-core-415 @about=REQ-core-225 @source=docs/decision/records/2026-09-27-surface-check.md#A12,docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: Rules alone with no files to search stop
  Given "surface.rules" has "rules/surface.yml", and there is no "surface.files" key
  When "kotowari check" is run
  Then the exit code is 2 and the first line of standard error starts with "config error: "

@id=EX-core-416 @about=REQ-core-225 @source=docs/decision/records/2026-09-27-surface-check.md#A12,docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: Files to search alone with no rules stop
  Given "surface.files" has "src/**/*.rs", and there is no "surface.rules" key
  When "kotowari check" is run
  Then the exit code is 2

@id=EX-core-417 @about=REQ-core-225,REQ-core-229 @source=docs/decision/records/2026-09-27-surface-check.md#A12,docs/decision/records/2026-09-27-surface-check.md#A14,docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: A missing rule file stops check and does not stop list
  Given "surface.files" is "src/**/*.rs", "surface.rules" has "rules/missing.yml", and that file does not exist
  When "kotowari check" and "kotowari list" are run
  Then the exit code of check is 2 and the exit code of list is 0

@id=EX-core-418 @about=REQ-core-223 @source=docs/decision/records/2026-09-27-surface-check.md#A1,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A20
Scenario: A rule is not applied to a surface file of a different language
  Given "surface.files" is "src/**", there is only one `surface rule`, with "language: rust", and "src/a.ts" has a string of the same form
  When "kotowari check --format json" is run
  Then no error of surface_without_spec is raised in "src/a.ts"

@id=EX-core-426 @about=REQ-core-236 @source=docs/decision/records/2026-09-27-surface-check.md#A20,docs/decision/records/2026-09-27-surface-check.md#A23
Scenario: A surface file not in a rule's language is not made into a tree and is not an error
  Given "surface.files" is "src/**", the only `surface rule` is "language: rust", and "src/notes.py" has a Python syntax error
  When "kotowari check --format json" is run
  Then no error of unparsable_file is raised in "src/notes.py"

@id=EX-core-427 @about=REQ-core-236 @source=docs/decision/records/2026-09-27-surface-check.md#A20,docs/decision/records/2026-09-27-surface-check.md#A23
Scenario: A syntax error in a surface file of a rule's language is unparsable_file
  Given "surface.files" is "src/**/*.rs", the `surface rule` is "language: rust", "src/bad.rs" has a Rust syntax error, and it does not match "tests.files"
  When "kotowari check --format json" is run
  Then one error of unparsable_file whose "path" is "src/bad.rs" is raised

@id=EX-core-428 @about=REQ-core-225,REQ-core-231 @source=docs/decision/records/2026-09-27-surface-check.md#A24
Scenario: Only the list key without rules stops list too
  Given in the configuration "surface.unspecified" is "docs/surface.yaml", and there are neither "surface.rules" nor "surface.files" keys
  When "kotowari list" is run
  Then the exit code is 2 and the first line of standard error starts with "config error: "

@id=EX-core-429 @about=REQ-core-226 @source=docs/decision/records/2026-09-27-surface-check.md#A2,docs/decision/records/2026-09-27-surface-check.md#A15,docs/decision/records/2026-09-27-surface-check.md#A18
Scenario: A name only in a statement of a property and a how_to_verify line is not taken to be in the IR
  Given in the scene of EX-core-408, "--verbose" is written enclosed in double quotes only in a `statement` of a `property` and in the "- how_to_verify:" line of a `requirement`
  When "kotowari check --format json" is run
  Then an error of surface_without_spec with detail "flag --verbose" is raised
```
