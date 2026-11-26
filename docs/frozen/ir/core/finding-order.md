# Order and lines of findings

English | [日本語](finding-order.ja.md)

Covers the order of findings and how the "line" of a finding is determined.

## Requirements

### REQ-core-024: Order of findings

- kind: algorithm
- source: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70
- definition: TBL-core-007, PROP-core-003
- verification: property

### REQ-core-027: Findings about the whole document

- kind: ubiquitous
- source: docs/decision/records/records.md#A40, docs/decision/records/records.md#A83, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A112, docs/decision/records/records.md#A144, docs/decision/records/2026-09-17-mutation-tests.md#A34, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D1
- verification: unit

kotowari always sets to null the "line" of a `finding` whose kind is missing_title, multiple_titles, missing_scope, too_many_lines, too_many_requirements, unparsable_file, glossary_invalid, equivalent_stale, equivalent_invalid, surface_unspecified_invalid, surface_unspecified_stale, overview_lead_missing, overview_ir_missing, overview_ir_shared, overview_name_conflict, overview_toc_invalid, overview_toc_page_missing, overview_toc_page_unknown, overview_toc_page_duplicate, overview_toc_group_empty, translation_missing, translation_record_invalid or translation_stale, and sets the "line" of other kinds as in TBL-core-019.

### REQ-core-028: Lines start at 1

- kind: ubiquitous
- source: docs/decision/records/records.md#A61
- verification: unit

kotowari always counts the "line" of a `finding` starting from 1.

## Decision tables

### TBL-core-007: Order of findings

- source: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70

| Order | Key | Ordering |
|---|---|---|
| 1 | path | Byte order |
| 2 | line | null first, then ascending |
| 3 | kind | Byte order |
| 4 | detail | Byte order |

### TBL-core-019: Line of a finding

- source: docs/decision/records/2026-09-22-ir-engine.md#A25, docs/decision/records/2026-09-22-ir-engine.md#A33, docs/decision/records/2026-09-22-ir-engine.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A40, docs/decision/records/records.md#A144, docs/decision/records/records.md#A114, docs/decision/records/records.md#A121, docs/decision/records/records.md#A139, docs/decision/records/records.md#A108, docs/decision/records/records.md#A61, docs/decision/records/records.md#A72, docs/decision/records/records.md#A153, docs/decision/records/records.md#A154, docs/decision/records/2026-09-16-ir-tree.md#A4, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-scenario-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-22-id-namespace.md#A3, docs/decision/records/2026-09-24-multi-language-tests.md#A32, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A11, docs/decision/records/2026-09-24-doc-marks.md#A12, docs/decision/records/2026-09-24-doc-marks.md#A32, docs/decision/records/2026-09-24-guide-gaps.md#A10, docs/decision/records/2026-09-25-deferred-items.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A11, docs/decision/records/2026-09-25-deferred-items.md#A21, docs/decision/records/2026-09-25-deferred-items.md#A23, docs/decision/records/2026-09-27-surface-check.md#A7, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A19, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D2

| Kind | line |
|---|---|
| missing_source | The heading line of the item. For a scenario, the tag line (or the "Scenario:" line if there is none). For a term, the table row. For a "- deferred:" line with an empty value, that line (REQ-core-210) |
| source_invalid | The line where the source is written (REQ-core-115) |
| missing_tag, unknown_tag, invalid_id, scenario_without_test | The tag line (or the "Scenario:" line if there is none) |
| unknown_term, vague_word, unclosed_backtick, missing_document | That line |
| unknown_field, duplicate_field, unknown_heading, invalid_gherkin_line | That line |
| unclosed_code_block | The opening line |
| invalid_glossary_row | That line |
| duplicate_term | The line of the term on the duplicating side (in the same glossary, the second and later; in a chain, the side farther from the root) |
| duplicate_id | The heading line of the second and later occurrences (REQ-core-032) |
| id_domain_mismatch | The heading line. For a value of "@id", the tag line |
| unresolved_reference, invalid_marker | For a mark, the line with the mark (REQ-core-118). For a guide mark, its starting line (REQ-core-202). Within a definition, a related line or a statement, that line. For "@about", the tag line |
| missing_field, missing_statement, missing_table, verification_missing, verification_invalid, unknown_kind, algorithm_without_definition, requirement_without_test | The heading line of the item |
| test_without_id | The first line of the test's node |
| record_field_missing | The numbered line |
| record_field_unknown, revision_link_invalid | That line |
| mutant_survived, mutant_timeout | The line of the mutation outcome |
| guide_stale | The starting line of the guide mark (REQ-core-204) |
| deferred_with_test | The heading line of the requirement. For a scenario, the tag line (REQ-core-211) |
| depends_on_deferred | The line where the reference is written (REQ-core-212) |
| unknown_line | That line |
| unknown_code_block | The opening line |
| glossary_title_invalid | The title line |
| surface_without_spec | The first line of the surface's node (REQ-core-227) |
| overview_form_invalid | That line. null for one that concerns the whole document (TBL-core-038) |
| overview_part_unknown, overview_part_invalid, overview_ref_unresolved | The opening line of the fence of the `part` (REQ-core-282, REQ-core-285) |
| translation_structure_mismatch | The line of the first element that differs, or null (REQ-core-345) |
| translation_switcher_invalid | The first non-empty line after the `title`, the line of the `title`, or null (REQ-core-346) |
| link_language_mismatch, link_to_record | The line of the link (REQ-core-348, REQ-core-349) |

## Properties

### PROP-core-003: findings are in order

- source: docs/decision/records/records.md#A61, docs/decision/records/records.md#A70, docs/decision/records/ir-form.md#出力

For any two adjacent `finding` entries in "findings", the later one never comes before the earlier one when compared in the order of TBL-core-007.
