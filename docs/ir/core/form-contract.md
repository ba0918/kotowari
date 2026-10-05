# Fixing the form of the IR

English | [日本語](form-contract.ja.md)

Covers where the form of the IR is decided, who reads that form, and how forms that are not adopted are decided.

## Requirements

### REQ-core-089: The form is fixed in code

- kind: ubiquitous
- source: docs/decision/records/records.md#A25, docs/decision/records/records.md#A52, docs/decision/records/records.md#A98
- verification: review
- how_to_verify: Confirm that "parse_document" in "crates/kotowari-core/src/ir.rs" reads the IR with the schema that "crates/kotowari-core/src/schema.rs" took in at compile time

kotowari always reads the `IR` in one form fixed in code.

### REQ-core-090: Does not read a schema file

- kind: prohibition
- source: docs/decision/records/records.md#R4, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari must not read a schema file that declares the form of the `IR`.

### REQ-core-091: Does not use an external mdschema

- kind: prohibition
- source: docs/decision/records/records.md#R5, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A18
- verification: unit

kotowari must not use an external mdschema as a stage before the check.

### REQ-core-168: Schemas are embedded in the binary and chosen by the kind of document

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A8, docs/decision/records/2026-09-22-ir-engine.md#A36
- verification: unit

kotowari always holds the schemas that declare the form of the `IR`, taken in at compile time, does not read a schema file at run time, and chooses among the schemas it took in according to whether a document is a `glossary`, a `flag record` or a `topic document`. Documents of the `IR` do not declare a schema, and kotowari does not use a frontmatter at the start of a document as a declaration of form.

### REQ-core-169: Does not have its own reading of the form

- kind: prohibition
- source: docs/decision/records/2026-09-22-ir-engine.md#A1, docs/decision/records/2026-09-22-ir-engine.md#A23, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-23-ir-engine-gaps.md#A9, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18
- verification: review
- how_to_verify: Look at the functions of every module under "crates/kotowari-core/src/" (including "ir.rs" and "query.rs"), and confirm that the only functions that read raw lines of a document of the `IR` are those for the two purposes of the contents of a gherkin block and the detection of an unclosed `code block`, and that there is no function that decides from raw lines the headings, the "- name:" lines, Markdown tables, the `title` and the `scope`, `statement` lines, or the range of the body of an `item`. Confirm that the body in query only cuts out the lines between the heading line of an `item` and the last line that the schema side returns

kotowari must not read the Markdown structure of a document of the `IR` by itself. It may read raw lines for only two purposes: the contents of a gherkin block, and the detection of an unclosed `code block`. This prohibition applies whatever the module, and includes query deciding the range of the body of an `item` from raw "### " and "## " lines.

### REQ-core-170: What is declared in the schema for the lines of findings

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A6, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-22-ir-engine.md#A67, docs/decision/records/2026-09-22-ir-engine.md#A72, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A24
- verification: review
- how_to_verify: Before and after the replacement, see that the existing tests that raise source_invalid, a `finding` inside gherkin, unclosed_backtick and missing_document, and the existing tests that check the body in query, pass

kotowari always puts in the schema declarations that take the line numbers of the "- source:" lines of a `source` and of the "- deferred:" lines (the lines under a heading and the document-level lines), the start line of the `code block` that wraps a `scenario`, the line number and the exact characters of each `statement` line, and the last line of an `item`; it builds the range of the body in query from the last line of an `item`, builds from these the "line" of source_invalid, the "line" counted from the start of the document of a `finding` inside gherkin, and the detail of unclosed_backtick, and also scans for each `document-name reference` over the characters of those `statement` lines.

### REQ-core-179: Declarations of how the embedded schemas are read and chosen

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23, docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A31, docs/decision/records/2026-09-23-ir-engine-gaps.md#A32, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-23-ir-english-tokens.md#A7, docs/decision/records/2026-09-24-kotowari-dir.md#A2
- verification: review
- how_to_verify: Read "crates/kotowari-core/schemas/ir.yaml", "crates/kotowari-core/schemas/context.yaml" and "crates/kotowari-core/schemas/flags.yaml", and confirm that all three write "reading: line" at the top level, that flags.yaml declares the `item` of a `flag record` both directly under the document ("document.item", "flags") and under the "## Flags" section ("flags_in_section"), that the table rule of context.yaml writes "header: [Term, Meaning, Source]" and "select: first", and that the extraction of the `item` in ir.yaml and of the `item` of a `flag record` in flags.yaml takes "end"

kotowari always declares "reading: line" in the three embedded schemas of the `IR` (`topic document`, `glossary`, `flag record`); in the schema of a `flag record` it declares the `item` of a `flag record` both directly under the document (extracted into "flags") and under the "## Flags" section (extracted into "flags_in_section"); in the schema of a `glossary` it declares the table header as "Term", "Meaning", "Source" together with "select: first"; and in the schemas of a `topic document` and a `flag record` it puts a declaration that takes the last line of an `item` ("end"). It does not rely on the engine's default way of reading.

### REQ-core-173: Checks kept in kotowari

- kind: ubiquitous
- source: docs/decision/records/2026-09-22-ir-engine.md#A1, docs/decision/records/2026-09-22-ir-engine.md#A4, docs/decision/records/2026-09-22-ir-engine.md#A23, docs/decision/records/2026-09-22-ir-engine.md#A24, docs/decision/records/2026-09-22-ir-engine.md#A30, docs/decision/records/2026-09-22-ir-engine.md#A37, docs/decision/records/2026-09-22-ir-engine.md#A83, docs/decision/records/2026-09-22-ir-engine.md#P1, docs/decision/records/2026-09-21-mds-spec.md#A6, docs/decision/records/2026-09-22-id-namespace.md#A3, docs/decision/records/records.md#A88, docs/decision/records/ir-form.md#文書名の参照
- verification: review
- how_to_verify: Before and after the replacement, confirm that the existing tests that raise these findings pass, and that the same judgements are not also declared on the schema side

kotowari always performs on its own side, and does not move to the schema, the checks that span documents (duplicate `ID` values, duplicate `term` entries, broken references, agreement of the name of an `ID` with its location), the upper limits on the number of lines and of `requirement` items, the check of a `source` (TBL-core-012), the checks based on `statement` lines (`term`, `vague word`, unclosed backquote), each `document-name reference`, the contents of gherkin, an unclosed `code block`, and the check that the `term` cell of a row of a `glossary` is empty.

### REQ-core-177: Replacing the reading does not change the output

- kind: invariant
- source: docs/decision/records/2026-09-22-ir-engine.md#A5, docs/decision/records/2026-09-22-ir-engine.md#A38, docs/decision/records/2026-09-22-ir-engine.md#A85
- verification: review
- how_to_verify: Before the replacement, save the output of "kotowari check --format json", take only the sequence of "findings" from it and from the output after, run "diff" on them, and see that there is not a single difference

The relation always holds that the sequence of each `finding` of "kotowari check --format json" against this `IR` location does not change by even one entry before and after the reading is replaced by the schema. None of the kind, the detail, the "line" or the order of output changes. The counts of documents and of lines that change because the documents themselves were fixed are not what this relation states. Differences in other `IR` locations are not covered either.

## Examples

```gherkin
@id=EX-core-040 @about=REQ-core-090,REQ-core-168 @source=docs/decision/records/2026-09-17-check-reach.md#A4,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: A schema file that is put in place is not read
  Given ".kotowari/schema.yaml" contains broken YAML
  When "kotowari check" is run
  Then it does not `stop`, and the exit code and standard output are the same as before the file was put in place

@id=EX-core-041 @about=REQ-core-091 @source=docs/decision/records/2026-09-17-check-reach.md#A18,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: The result does not change even when external commands are not found
  Given the environment variable "PATH" points only at an empty directory
  When "kotowari check" is run
  Then the exit code, standard output and standard error are the same as when "PATH" is left as it is

@id=EX-core-264 @about=REQ-core-168 @source=docs/decision/records/2026-09-22-ir-engine.md#A36,docs/decision/records/2026-09-22-ir-engine.md#A8
Scenario: Removing the schema location does not change the output
  Given no document of the `IR` has a frontmatter at its start, and there is a location for schema files
  When that location is moved aside and "kotowari check --format json" is run
  Then the exit code and standard output are the same as before it was moved aside
```
