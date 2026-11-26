# Where engine findings are mapped

English | [日本語](finding-map.ja.md)

This document covers, after the reading of form was replaced by the schema, the mapping of the findings returned by the schema side onto kotowari's finding kinds. The finding kinds themselves and their details are defined by the findings document.

## Requirements

### REQ-core-171: The finding mapping table

- kind: algorithm
- source: docs/decision/records/2026-09-22-ir-engine.md#A2, docs/decision/records/2026-09-22-ir-engine.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A40
- definition: TBL-core-029, TBL-core-030
- verification: review
- how_to_verify: A person compares the mapping table against the enumeration of kinds in `crates/kotowari-markdown-schema/src/finding.rs`, and sees that nothing is missing or extra and that the "line" handling of the rows that map to the nine kinds of REQ-core-027 is null

### REQ-core-172: A finding with no mapping stops the run

- kind: event_driven
- source: docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/records.md#A100, docs/decision/records/records.md#A101, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A36, docs/decision/records/2026-09-24-plan-schema.md#A11
- verification: unit

For a document of the `IR`, when kotowari receives from the schema side a `finding` of a kind that has no row in the mapping table (TBL-core-030), or a `finding` of the kind of a row whose mapping is written as "does not occur", kotowari will `stop` and does not silently discard that `finding`. Only a `finding` of a row whose mapping is written as "not emitted" is discarded without a `stop`, and "not emitted" may be written only on rows for inputs listed as an `exclusion`.

## Decision tables

### TBL-core-029: Columns of the table mapping engine findings

- source: docs/decision/records/2026-09-22-ir-engine.md#A2, docs/decision/records/2026-09-22-ir-engine.md#A3, docs/decision/records/2026-09-22-ir-engine.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A24, docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A31, docs/decision/records/2026-09-22-ir-engine.md#A40, docs/decision/records/records.md#A92, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-22-ir-engine.md#A71, docs/decision/records/2026-09-22-ir-engine.md#A74, docs/decision/records/2026-09-22-ir-engine.md#A76, docs/decision/records/2026-09-22-ir-engine.md#A77, docs/decision/records/2026-09-22-ir-engine.md#A79, docs/decision/records/2026-09-22-ir-engine.md#A81, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-english-tokens.md#A2

The table that maps each `finding` emitted by the schema side onto a kotowari `finding` kind has the five columns below. A row for a kind with the same name but a different meaning says so. The schema side's invalid_id concerns the form of the `ID` in an `item` heading, and is a different thing from kotowari's invalid_id (the value of "@id"). The schema side's missing_table does not occur, because a `cardinality` range is declared for both the `glossary` table and the `decision table` table. Instead, repeat_min_not_met occurs and splits into glossary_invalid for the `glossary` and missing_table for the `decision table`. The schema side's missing_required_field splits into kotowari's verification_missing, missing_source, missing_field and algorithm_without_definition according to the missing `field line`. The mapping is closed over these four. The "node name" column holds, besides the name the schema side returns, distinctions kotowari knows itself (which schema validated the document, whether the `finding` has a line). Since the schema side declares the "- definition:" line as conditionally required by the value of the "- kind:" line, a missing one is mapped to algorithm_without_definition.

| Column | Content |
|---|---|
| Schema-side kind | The kind of finding the schema side emits |
| Node name | The name used to split the mapping, the type of a line outside the declarations, or the `rule kind` of the `node` whose `cardinality` was counted. Empty when unused |
| kotowari kind | The mapping target. Kinds that do not occur with the current schema are written "does not occur", and those discarded without mapping as an `exclusion` are written "not emitted" |
| "line" handling | Used as is, set to null (the nine kinds of REQ-core-027), or reattached to the heading line of the `item` |
| Detail material | Which element of the finding the detail is built from (the text of the line read as is, the text of the line read with "# " removed, the node name, the document name, or the `item` of the `extraction`) |

### TBL-core-030: Mapping of engine findings

- source: docs/decision/records/2026-09-22-ir-engine.md#A2, docs/decision/records/2026-09-22-ir-engine.md#A3, docs/decision/records/2026-09-22-ir-engine.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A24, docs/decision/records/2026-09-22-ir-engine.md#A28, docs/decision/records/2026-09-22-ir-engine.md#A31, docs/decision/records/2026-09-22-ir-engine.md#A40, docs/decision/records/2026-09-22-ir-engine.md#A71, docs/decision/records/2026-09-22-ir-engine.md#A74, docs/decision/records/2026-09-22-ir-engine.md#A76, docs/decision/records/2026-09-22-ir-engine.md#A77, docs/decision/records/2026-09-22-ir-engine.md#A79, docs/decision/records/2026-09-22-ir-engine.md#A81, docs/decision/records/2026-09-22-ir-engine.md#A82, docs/decision/records/2026-09-22-ir-engine.md#A86, docs/decision/records/2026-09-22-ir-engine.md#A87, docs/decision/records/2026-09-23-ir-engine-gaps.md#A7, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16, docs/decision/records/2026-09-23-ir-engine-gaps.md#A40

Following the columns of TBL-core-029, all 24 kinds of the schema side are listed. On receiving a `finding` of a "does not occur" row, kotowari will `stop` as in REQ-core-172, and a `finding` of a "not emitted" row is discarded.

| Schema-side kind | Node name | kotowari kind | "line" handling | Detail material |
|---|---|---|---|---|
| missing_title |  | missing_title | Set to null | The document name |
| multiple_titles |  | multiple_titles (one for each `title` from the second on) | Set to null | The text of the line read, with "# " removed and leading and trailing whitespace removed |
| title_pattern_mismatch | Glossary | glossary_title_invalid | Used as is | The text of the line read, as is |
| undeclared_heading |  | unknown_heading | Used as is | The text of the line read, as is |
| undeclared_line | A name-and-value list line, a bullet list, an ordered list | unknown_field | Used as is | The text of the line read, as is |
| undeclared_line | Statement | unknown_line | Used as is | The text of the line read, as is |
| undeclared_line | Table (the `glossary` schema) | Not emitted (a table that did not become the `glossary` table under "select: first"; an `exclusion`) | — | — |
| undeclared_line | Table (a schema other than the `glossary`), code block | unknown_line | Used as is | The text of the line read, as is |
| missing_required_field | Verification | verification_missing | Used as is | The `ID` of the `item` of the `extraction` |
| missing_required_field | Source | missing_source | Used as is | The `ID` of the `item` of the `extraction` |
| missing_required_field | Kind | missing_field | Used as is | The node name |
| missing_required_field | How to verify | missing_field | Used as is | The node name |
| missing_required_field | Related | missing_field | Used as is | The node name |
| missing_required_field | Definition | algorithm_without_definition | Used as is | The `ID` of the `item` of the `extraction` |
| missing_required_section |  | Does not occur (no section is required) | — | — |
| missing_statement | Statement of the preamble | Does not occur (a `cardinality` range is declared for the statements of the preamble) | — | — |
| missing_statement | Statement of an item | Does not occur (a `cardinality` range is declared for the `statement` of an `item`) | — | — |
| missing_bullets |  | Does not occur (no bullets are declared) | — | — |
| missing_table | Glossary | Does not occur (a `cardinality` range is declared for the `glossary` table) | — | — |
| missing_table | Decision table | Does not occur (a `cardinality` range is declared for the `decision table` table) | — | — |
| missing_codeblock |  | Does not occur (the lower bound for code blocks is 0) | — | — |
| field_pattern_mismatch |  | Does not occur (the pattern for sources was removed) | — | — |
| field_enum_invalid | Kind | unknown_kind | Reattached to the heading line of the `item` | The corresponding value of the `item` of the `extraction` |
| field_enum_invalid | Verification | verification_invalid | Reattached to the heading line of the `item` | The corresponding value of the `item` of the `extraction` |
| statement_pattern_mismatch |  | Does not occur (no pattern is declared for statements) | — | — |
| statement_enum_invalid |  | Does not occur (no allow list is declared for statements) | — | — |
| bullet_pattern_mismatch |  | Does not occur (no bullets are declared) | — | — |
| heading_level_mismatch |  | unknown_heading | Used as is | The text of the line read, as is |
| invalid_id |  | unknown_heading | Used as is | The text of the line read, as is |
| field_order_mismatch |  | Does not occur (no ordering is declared) | — | — |
| table_header_mismatch | Glossary (the finding's line is the table's start line) | Does not occur (because "select: first" is declared for the `glossary` table, a table whose header does not match becomes a table outside the declarations) | — | — |
| table_header_mismatch | Glossary (the finding's line is a data row; a row with fewer cells than the header) | invalid_glossary_row | Used as is | The text of the line read, as is |
| codeblock_lang_mismatch |  | unknown_code_block | Used as is | The text of the line read, as is |
| codeblock_line_mismatch |  | Does not occur (the line pattern for gherkin was removed) | — | — |
| repeat_min_not_met | Statement, and the finding has no line | missing_scope | Set to null | The document name |
| repeat_min_not_met | Statement, and the finding has a line | missing_statement | Used as is | The `ID` of the `item` of the `extraction` |
| repeat_min_not_met | Table, and the finding has no line | glossary_invalid | Set to null | The document name |
| repeat_min_not_met | Table, and the finding has a line | missing_table | Used as is | The `ID` of the `item` of the `extraction` |
| repeat_min_not_met | None of the above | Does not occur (every other lower bound is 0) | — | — |
| repeat_max_exceeded | Name of a field line | duplicate_field | Used as is | The node name |
| repeat_max_exceeded | Other than a field line | Does not occur (no upper bound is imposed except on field lines) | — | — |

## Examples

```gherkin
@id=EX-core-265 @about=REQ-core-172 @source=docs/decision/records/2026-09-22-ir-engine.md#A28,docs/decision/records/records.md#A100,docs/decision/records/records.md#A101
Scenario: A finding with no mapping stops the run
  Given the schema side returns a `finding` of a kind that has no row in the mapping table TBL-core-030, and a `finding` of the kind of a row whose mapping is written as "does not occur"
  When "kotowari check" is run
  Then kotowari will `stop` in both cases

@id=EX-core-278 @about=REQ-core-035,TBL-core-030 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A7,docs/decision/records/2026-09-23-ir-engine-gaps.md#A17
Scenario: A document with three titles gets an error for each title from the second on
  Given there is a `topic document` with the three `title` lines "# 一", "# 二" and "# 三"
  When "kotowari check --format json" is run
  Then two multiple_titles errors are emitted for that document, with details "二" and "三", and "line" is null for both

@id=EX-core-279 @about=REQ-core-172,TBL-core-030 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A25,docs/decision/records/2026-09-23-ir-engine-gaps.md#A27
Scenario: A table that did not become the glossary table is discarded without mapping
  Given "CONTEXT.md" has, after a table whose header matches, a second table with a different header, separated by a blank line
  When "kotowari check --format json" is run
  Then kotowari does not `stop`, and no `finding` is emitted for "CONTEXT.md"
```
