# Usage display and target environment

English | [日本語](cli-environment.ja.md)

Covers how "--help" and "--version" are handled, the target OS, how stops and findings are divided, and the wording of stop reasons and details.

## Requirements

### REQ-core-107: Displaying usage and version

- kind: event_driven
- source: docs/decision/records/records.md#A103, docs/decision/records/records.md#A136
- verification: unit

When it receives "--help" or "--version", kotowari looks at no other argument, performs no inspection, writes the usage or version string to standard output, and ends with exit code 0. "check" need not be present.

### REQ-core-108: Target environment

- kind: ubiquitous
- source: docs/decision/records/records.md#A125
- verification: review
- how_to_verify: Confirm that `cargo test` passes on Linux and macOS. Windows behavior is not promised (only the path separator normalization of REQ-core-110)

kotowari always targets Linux and macOS. On Windows it performs only the path separator normalization (REQ-core-110) and promises no other behavior.

### REQ-core-109: Dividing stops and findings

- kind: invariant
- source: docs/decision/records/records.md#A100, docs/decision/records/records.md#A101, docs/decision/records/records.md#P2, docs/decision/records/records.md#A102, docs/decision/records/2026-09-17-mutation-tests.md#A48, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: review
- how_to_verify: Confirm that `load_all` in `crates/kotowari/src/acquisition.rs` inspects that the locations exist and stops with `StopReason`. Confirm with `rg 'filter_map|if let Ok' crates/kotowari/src/ crates/kotowari-core/src/` that there is no path that silently skips

It always holds that for an unreadable input, a broken input, or an input that does not fit the form of the contract, kotowari either makes a `stop` or raises an `error` `finding`, and never silently skips it. For an input that breaks the premise of reading a file or the configuration as a whole (unreadable, not UTF-8, an error in the syntax, type or value of the configuration, an argument error, a glob syntax error, an error in the results file, a syntax error in the `list of equivalents` or the `list of unspecified surfaces`), it makes a `stop`; for an input that could be read but locally deviates from the form, it raises an `error` `finding` at that place. The only things not read are the `exclusion` ones. When the file that one entry of the `mutation outcome` or of the `list of equivalents` points to is missing, unreadable or not UTF-8, it does not make a `stop` but falls back to an `error` or `notice` `finding`, as in REQ-core-141 and REQ-core-142.

### REQ-core-120: Do not silently skip reading

- kind: prohibition
- source: docs/decision/records/records.md#P2, docs/decision/records/records.md#A100
- verification: review
- how_to_verify: Confirm in `crates/kotowari-core/src/ir.rs` and `crates/kotowari/src/acquisition.rs` that no behavior not listed in the specification is decided silently. Confirm that the gherkin parsing of `GherkinBlock` makes any line other than the valid kinds invalid_gherkin_line, that `read_utf8_file` makes an unreadable file a stop, that `parse_document` makes findings and values it cannot map a stop, and that `check_documents` has no skipping that is not in the specification

kotowari must not skip an input not listed in the `exclusion` without making a `stop` or a `finding`.

### REQ-core-175: Boundary of the stop added by the replacement

- kind: event_driven
- source: docs/decision/records/2026-09-22-ir-engine.md#A9, docs/decision/records/2026-09-22-ir-engine.md#A20, docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/2026-09-22-ir-engine.md#A36, docs/decision/records/2026-09-24-plan-schema.md#A11
- verification: unit

When, for an `IR` document, a value received from the schema side cannot be mapped to kotowari's types, or the mapping table (TBL-core-030) has no target for it, kotowari makes a `stop` with the single reason added to TBL-core-018. The reasons and wording of a `stop` caused by the user's input, such as an `IR` document that cannot be read or is not UTF-8, are unchanged, and there is no `stop` whose reason is that the schema cannot be read.

### REQ-core-176: A document with form findings still gets the cross-document inspection

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A5, docs/decision/records/2026-09-22-ir-engine.md#A18, docs/decision/records/records.md#A100, docs/decision/records/records.md#A101
- verification: unit

kotowari always continues the cross-document inspection with the values it obtained even for a document with a `finding` from the schema side, and does not drop that document from the inspection.

## Decision tables

### TBL-core-018: Wording of stop reasons

- source: docs/decision/records/records.md#A104, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-22-ir-engine.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A61, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-04-overview-on-public-api.md#A9

| Reason | Wording of the first line of standard error |
|---|---|
| Configuration error | config error |
| Argument error | argument error |
| Unreadable file | unreadable file |
| Non-UTF-8 file | non-UTF-8 file |
| Results error | results error |
| Mapping error | mapping error |
| Overview data error | overview error |
| Port error | port error |
| Cache location error | cache error |

### TBL-core-020: Stop details

- source: docs/decision/records/2026-09-22-ir-engine.md#A73, docs/decision/records/records.md#A137, docs/decision/records/records.md#A147, docs/decision/records/records.md#A160, docs/decision/records/records.md#A164, docs/decision/records/2026-09-17-mutation-tests.md#A39, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A49, docs/decision/records/2026-09-17-mutation-tests.md#A55, docs/decision/records/2026-09-19-read-commands.md#A20, docs/decision/records/2026-09-20-query-status.md#A6, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-24-plan-schema.md#A10, docs/decision/records/2026-09-24-plan-schema.md#A30, docs/decision/records/2026-09-24-doc-marks.md#A15, docs/decision/records/2026-09-24-doc-marks.md#A28, docs/decision/records/2026-09-24-doc-marks.md#A36, docs/decision/records/2026-09-24-guide-gaps.md#A2, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A61, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A75, docs/decision/records/2026-10-02-whole-picture.md#A78, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-04-overview-on-public-api.md#A9, docs/decision/records/2026-10-06-changes-rethink.md#A10, docs/decision/records/2026-10-06-changes-rethink.md#A23, docs/decision/records/2026-10-06-changes-rethink.md#A30

| Reason | Detail (in English) |
|---|---|
| Configuration error | The configuration file's path relative to the base directory (including "../" if it is outside the base), and a description of the error. For an error in the list of equivalents (REQ-core-148), the list of equivalents file's relative path and a description of the error. For an error in "tests.rules" (REQ-core-189) and in "surface.rules" (REQ-core-225), the rule file's relative path and a description of the error. For an error in the list of unspecified surfaces (REQ-core-231), the list of unspecified surfaces file's relative path and a description of the error. For an overlap between the guide and test locations (REQ-core-199), the relative path of the first of the overlapping files in byte order of path, followed by ": matched by both guides.files and tests.files". For an overlap of the `overview data` location (REQ-core-280), likewise followed by ": matched by both overview.files and guides.files" or ": matched by both overview.files and tests.files". When "kotowari overview build" or "kotowari overview serve" is run without the "overview" key (REQ-core-279), only "overview is not configured" |
| Argument error | A descriptive sentence and the text of the offending argument. When there are no arguments at all, or no first positional argument, "expected command: check, list, mutants, overview, plan, query or status". When nothing has the `ID` in "kotowari query", "unknown id: " and the text of the positional argument (REQ-core-157) |
| Unreadable file | The relative path and the OS error message. When the current directory cannot be obtained, "current directory: " and the OS error message |
| Non-UTF-8 file | The relative path |
| Results error | The results file's relative path and a description of the error |
| Mapping error | The kind and `node name` of the `finding` that could not be mapped, or a description of the value that could not be mapped |
| Overview data error | The count of `error` findings and " errors in overview data; run kotowari check" (REQ-core-294) |
| Port error | "127.0.0.1:<port>", ": " and the OS error message (REQ-core-298) |
| Cache location error | The offending path relative to the `base directory`, and, if there is an OS error, ": " and the OS error message (REQ-core-324) |

## Examples

```gherkin
@id=EX-core-268 @about=REQ-core-175 @source=docs/decision/records/2026-09-22-ir-engine.md#A9,docs/decision/records/2026-09-22-ir-engine.md#A20
Scenario: A value that cannot be mapped to a type is a stop
  Given there is a build that includes a schema returning a value that cannot be mapped to kotowari's types
  When "kotowari check" is run
  Then the exit code is 2, and the first line of standard error is the wording of the reason added to TBL-core-018

@id=EX-core-269 @about=REQ-core-176 @source=docs/decision/records/2026-09-22-ir-engine.md#A5,docs/decision/records/2026-09-22-ir-engine.md#A18,docs/decision/records/records.md#A100,docs/decision/records/records.md#A101,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: A document with form findings still gets the cross-document inspection
  Given there is an `IR` with a `requirement` missing its "- verification:" line, whose `ID` overlaps the `ID` of a `requirement` in another document
  When "kotowari check --format json" is run
  Then both verification_missing and duplicate_id `error` findings are raised
```
