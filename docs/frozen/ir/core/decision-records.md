# Decision records and ADRs

English | [日本語](decision-records.ja.md)

Covers how the decision records and ADRs that sources point to are kept.

## Requirements

### REQ-core-092: Keep both

- kind: invariant
- source: docs/decision/records/records.md#A22, docs/decision/records/2026-09-17-decision-log.md#A5
- verification: review
- how_to_verify: Confirm in `read_texts` of `crates/kotowari/src/sources.rs` that the directories of both the decision records and the ADRs are read

The relation in which the `decision record` and the `ADR` files that have been written are kept always holds. It holds even in a repository with no `ADR` at all.

### REQ-core-093: A decision record per unit of deliberation

- kind: ubiquitous
- source: docs/decision/records/records.md#A22, docs/decision/records/2026-10-01-change-conformance.md#A13
- verification: review
- how_to_verify: Check the skills and records that read decision records, and confirm that, even without starting brainstorm, each deliberation is saved with a date and a title, and that changes to the body of a decision can be traced through added decisions and revision references

A `decision record` is always kept as a separate file, with a date and a title, for each unit of deliberation in which decisions were made, whether or not brainstorm was started. An existing decision line is not overwritten by another decision; a change is kept as an added decision and a revision reference.

### REQ-core-094: Sections of an ADR

- kind: ubiquitous
- source: docs/decision/records/records.md#A43, docs/decision/records/records.md#A50
- verification: review
- how_to_verify: The only check of ADR sections is the heading match for sources. The structure of the five sections is not checked

An `ADR` always has the five sections 「状況」 (context), 「決定」 (decision), 「理由」 (reasons), 「却下した案」 (rejected options) and 「結果」 (consequences).

### REQ-core-095: The decision section of an ADR

- kind: ubiquitous
- source: docs/decision/records/records.md#A22, docs/decision/records/records.md#A43
- verification: review
- how_to_verify: The only check of an ADR's decision section is the heading match for sources

The 「決定」 section of an `ADR` always points to the `decision number` of a `decision record` and does not repeat the behaviour statements.

### REQ-core-096: Not ADRs alone

- kind: prohibition
- source: docs/decision/records/records.md#R1
- verification: review
- how_to_verify: kotowari has no check that forbids running with ADRs only (the configuration requires both paths)

The record-keeping practice shall not use `ADR` files alone, cutting one `ADR` per decision.

### REQ-core-097: Existing ADRs are not deleted

- kind: prohibition
- source: docs/decision/records/2026-09-17-decision-log.md#A5, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: review
- how_to_verify: List the files under `docs/decision/adr/` that appear in `- source:` lines, `@source=` tags, and the source column of glossaries (only the source lines from `rg -o 'docs/decision/adr/[^ ,|]+' docs/ir`), and confirm that all of those files exist. As of 2026-09-17 these are 0002 and 0003

The record-keeping practice shall not delete an existing `ADR`.

### REQ-core-103: ADR file names

- kind: ubiquitous
- source: docs/decision/records/records.md#A3, docs/decision/records/records.md#A23
- verification: review
- how_to_verify: The ADR file name format is looked at when checking sources, but the format itself is not checked

The file name of an `ADR` is always of the form "0001-<slug>.md".
