# Missing lines and tables of items

English | [日本語](ir-missing.ja.md)

Covers the checks when an item lacks a required line or table, and "Scenario:" lines outside a gherkin block.

## Requirements

### REQ-core-098: A required line is missing

- kind: event_driven
- source: docs/decision/records/records.md#A68, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A92, docs/decision/records/records.md#A157, docs/decision/records/2026-09-22-ir-engine.md#A75, docs/decision/records/2026-09-19-read-commands.md#A23, docs/decision/records/2026-09-20-query-status.md#A10, docs/decision/records/2026-09-20-query-status.md#A17, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

When a `requirement` has no "- kind:" line, when a `requirement` whose "- verification:" is review has no "- how_to_verify:" line, or when the `item` of a `flag record` has no "- kind:" or "- related:" line, kotowari raises a missing_field `error` with the name of the missing line as the detail. When the "- verification:" line is missing it raises only verification_missing, and when the "- source:" line is missing only missing_source, and does not raise missing_field. A "- kind:", "- verification:", "- definition:", "- related:" or "- how_to_verify:" line whose value is empty is treated as a line that is present, and no `finding` for a missing line is raised. For "- kind:" and "- verification:", whose values have an allow list, an empty value is made a `finding` of an invalid value.

### REQ-core-099: A decision table has no table

- kind: event_driven
- source: docs/decision/records/records.md#A68, docs/decision/records/ir-form.md#検査の種類
- verification: unit

When a `decision table` has no Markdown table, kotowari raises a missing_table `error` with the `ID` of that `decision table` as the detail.

### REQ-core-100: Scenario outside a gherkin block

- kind: ubiquitous
- source: docs/decision/records/records.md#A68, docs/decision/records/ir-form.md#項目
- verification: unit

kotowari always ignores a "Scenario:" line outside a gherkin code block, without regarding it as a `scenario`.
