# Kinds of checks

English | [日本語](findings.ja.md)

Covers the severity and the content of the detail for each finding kind. The conditions for each kind are set by the requirement in the right-hand column.

## Requirements

### REQ-core-029: Error kinds

- kind: algorithm
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-24-plan-schema.md#A11
- definition: TBL-core-008
- verification: unit

### REQ-core-030: Notice kinds

- kind: algorithm
- source: docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2
- definition: TBL-core-009
- verification: unit

### REQ-core-031: Only eight notices

- kind: invariant
- source: docs/decision/records/records.md#A29, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A11, docs/decision/records/2026-09-17-mutation-tests.md#A20, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-24-doc-marks.md#A8, docs/decision/records/2026-09-25-deferred-items.md#A11, docs/decision/records/2026-09-25-deferred-items.md#A12, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A15
- verification: unit

The relation always holds that only a `finding` whose kind is too_many_lines, too_many_requirements, mutant_timeout, equivalent_stale, guide_stale, deferred_with_test, depends_on_deferred or surface_unspecified_stale is a `notice`, and every `finding` of any other kind is an `error`.

### REQ-core-032: Duplicate IDs

- kind: event_driven
- source: docs/decision/records/records.md#A47, docs/decision/records/records.md#A61, docs/decision/records/records.md#A72, docs/decision/records/records.md#A113
- verification: unit

When the same `ID` is in two or more places, kotowari emits, for each place from the second on, a duplicate_id `error` whose "line" is its heading line (for a `scenario`, the "Scenario:" line). The first is the one in the document that comes first in byte order of path, and within the same document, the one with the smaller line.

### REQ-core-174: Lines outside the declarations, code blocks, and the glossary title

- kind: event_driven
- source: docs/decision/records/2026-09-22-ir-engine.md#A33, docs/decision/records/2026-09-22-ir-engine.md#A35, docs/decision/records/2026-09-22-ir-engine.md#A39, docs/decision/records/2026-09-22-ir-engine.md#A82, docs/decision/records/records.md#A102, docs/decision/records/ir-form.md#文書, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-guide-gaps.md#A1, docs/decision/records/2026-09-24-guide-gaps.md#A6, docs/decision/records/2026-09-24-guide-gaps.md#A10, docs/decision/records/2026-09-22-ir-engine.md#A89, docs/decision/records/2026-10-06-todo-zero.md#A7
- verification: unit

When, directly under a "## " heading and before the first "### ", there is a non-empty line outside any `code block` that is neither a list nor a table, or when there is a table not declared by the schema (excluding a table inside a `glossary` document; REQ-core-117) or a code block, kotowari emits an unknown_line `error`; when there is a non-gherkin `code block` under the "## Examples" heading, it emits an unknown_code_block `error`; and when the `title` of a `glossary` is not "# Glossary", it emits a glossary_title_invalid `error`. It does not emit invalid_gherkin_line for lines inside a non-gherkin `code block` (REQ-core-113). The detail is as in TBL-core-008 and "line" is as in TBL-core-019.

## Decision tables

### TBL-core-008: Error kinds and details

- source: docs/decision/records/2026-09-22-ir-engine.md#A33, docs/decision/records/2026-09-22-ir-engine.md#A39, docs/decision/records/records.md#A142, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A68, docs/decision/records/records.md#A108, docs/decision/records/records.md#A109, docs/decision/records/records.md#A110, docs/decision/records/records.md#A112, docs/decision/records/records.md#A116, docs/decision/records/records.md#A111, docs/decision/records/records.md#A150, docs/decision/records/records.md#A153, docs/decision/records/records.md#A154, docs/decision/records/2026-09-16-ir-tree.md#A5, docs/decision/records/2026-09-16-ir-tree.md#A19, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-scenario-tests.md#A6, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A57, docs/decision/records/2026-09-22-id-namespace.md#A4, docs/decision/records/2026-09-23-ir-engine-gaps.md#A7, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17, docs/decision/records/2026-09-23-ir-engine-gaps.md#A40, docs/decision/records/2026-09-24-multi-language-tests.md#A31, docs/decision/records/2026-09-24-multi-language-tests.md#A42, docs/decision/records/2026-09-24-plan-schema.md#A18, docs/decision/records/2026-09-24-doc-marks.md#A10, docs/decision/records/2026-09-24-doc-marks.md#A31, docs/decision/records/2026-09-24-guide-gaps.md#A10, docs/decision/records/2026-09-25-deferred-items.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A19, docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A7, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A20, docs/decision/records/2026-09-27-surface-check.md#A22, docs/decision/records/2026-09-27-surface-check.md#A23, docs/decision/records/2026-10-02-whole-picture.md#A69, docs/decision/records/2026-10-02-whole-picture.md#A71, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-04-overview-on-public-api.md#A11, docs/decision/records/2026-10-05-overview-index.md#A16, docs/decision/records/2026-10-05-overview-index.md#A34, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#D3

For kinds whose detail is "the text of the line", "the text of the heading" or "the text of the Scenario: line", the detail is the text of the line read, as is (including indentation and trailing whitespace, not reconstructed).

| Kind | detail | Requirement setting the conditions |
|---|---|---|
| missing_title | The document name (the file name without the directory) | REQ-core-034 |
| multiple_titles | The title of that occurrence (the text of a second or later title with "# " removed and leading and trailing whitespace removed) | REQ-core-035 |
| missing_scope | The document name (the file name without the directory) | REQ-core-036 |
| unknown_heading | The text of the heading | REQ-core-043 |
| unknown_field | The text of the line | REQ-core-044 |
| missing_field | The name of the line | REQ-core-098 |
| missing_table | The ID of the decision table | REQ-core-099 |
| duplicate_field | The name of the line | REQ-core-045, REQ-core-209 |
| missing_source | The ID of the item, or the term. For a "- deferred:" line with an empty value, "deferred" | REQ-core-059, REQ-core-210 |
| source_invalid | The source string | REQ-core-058 |
| unknown_term | The enclosed string | REQ-core-064, REQ-core-065 |
| missing_document | The string of the document-name reference | REQ-core-070 |
| missing_statement | The ID of the item | REQ-core-047 |
| verification_missing | The ID of the requirement | REQ-core-048 |
| verification_invalid | The value | REQ-core-049 |
| unknown_kind | The value | REQ-core-050 |
| duplicate_id | The ID | REQ-core-032 |
| unresolved_reference | The ID | REQ-core-054 |
| algorithm_without_definition | The ID of the requirement | REQ-core-051 |
| missing_tag | The name of the missing tag | REQ-core-053 |
| unknown_tag | The name of the tag | REQ-core-052 |
| vague_word | The word | REQ-core-066 |
| requirement_without_test | The ID of the requirement | REQ-core-085 |
| scenario_without_test | The ID of the scenario | REQ-core-137 |
| test_without_id | The name of the test. If the name is null, the whole text of the first line of the test's node with leading and trailing whitespace removed | REQ-core-086 |
| invalid_marker | The text of the line | REQ-core-072, REQ-core-202 |
| unparsable_file | The path of the file | REQ-core-083, REQ-core-236 |
| unclosed_code_block | The text of the opening line | REQ-core-112 |
| invalid_gherkin_line | The text of the line | REQ-core-113 |
| invalid_id | The value | REQ-core-114 |
| id_domain_mismatch | The ID | REQ-core-167 |
| glossary_invalid | The document name (the file name without the directory) | REQ-core-117 |
| unclosed_backtick | The text of the line | REQ-core-116 |
| invalid_glossary_row | The text of the line | REQ-core-122 |
| duplicate_term | The term | REQ-core-123 |
| record_field_missing | The name of the missing supplementary line | REQ-core-130 |
| record_field_unknown | The name of the supplementary line | REQ-core-131 |
| revision_link_invalid | The href of the link. If there is no link, the value of the superseded_by line | REQ-core-132 |
| mutant_survived | The change description | REQ-core-139 |
| equivalent_invalid | The string of "file" and "change", exactly as written in the entry of the list of equivalents, joined by ": " | REQ-core-143 |
| unknown_line | The text of the line | REQ-core-174 |
| invalid_plan | The string of the schema-side kind and the detail joined by ": " | REQ-core-193 |
| unknown_code_block | The text of the opening line | REQ-core-174 |
| glossary_title_invalid | The text of the title line | REQ-core-174 |
| surface_without_spec | The string of the surface's kind and name separated by one half-width space | REQ-core-227 |
| surface_unspecified_invalid | The string of "kind" and "name", exactly as written in the entry of the list of unspecified surfaces, separated by one half-width space | REQ-core-233 |
| overview_form_invalid | The name of the kind of the kotowari-markdown-schema `finding`. For a violation in a frontmatter line, "frontmatter" | REQ-core-281 |
| overview_part_unknown | The name of the kind of the `part` | REQ-core-282 |
| overview_part_invalid | The name of the kind of the `part`, one half-width space, and the place that did not match (REQ-core-282; "(root)" for the whole value, "(yaml)" when it cannot be read as YAML) | REQ-core-282 |
| overview_lead_missing | The document name (the file name without the directory) | REQ-core-283 |
| overview_ir_missing | The text of the "ir" entry | REQ-core-284 |
| overview_ir_shared | The path of the `topic document` that is in the "ir" of two or more `overview data` files | REQ-core-284 |
| overview_ref_unresolved | The text of the reference | REQ-core-285 |
| overview_name_conflict | The file name without ".md" | REQ-core-305 |
| overview_toc_invalid | The place that did not match (TBL-core-043) | REQ-core-327 |
| overview_toc_page_missing | The name missing from the `table of contents` | REQ-core-328 |
| overview_toc_page_unknown | The JSON Pointer of that entry | REQ-core-329 |
| overview_toc_page_duplicate | The JSON Pointer of that entry | REQ-core-329 |
| overview_toc_group_empty | The JSON Pointer of that `contents group`, or "(root)" | REQ-core-330 |
| translation_missing | The path of the missing `side`, relative to the base directory | REQ-core-338 |
| translation_record_invalid | One of "missing", "yaml", "keys" and "value" | REQ-core-339 |
| translation_stale | The string of the value of the `consistency record` and the current blob hash separated by one half-width space | REQ-core-341 |
| translation_structure_mismatch | The name of the first part that differs (TBL-core-044) | REQ-core-345 |
| translation_switcher_invalid | The `switcher line` that should be there | REQ-core-346 |
| link_language_mismatch | The destination as written | REQ-core-348 |
| link_to_record | The destination as written | REQ-core-349 |

### TBL-core-009: Notice kinds and details

- source: docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2, docs/decision/records/2026-09-17-mutation-tests.md#A31, docs/decision/records/2026-09-17-mutation-tests.md#A40, docs/decision/records/2026-09-17-mutation-tests.md#A43, docs/decision/records/2026-09-17-mutation-tests.md#A57, docs/decision/records/2026-09-24-doc-marks.md#A8, docs/decision/records/2026-09-24-doc-marks.md#A14, docs/decision/records/2026-09-25-deferred-items.md#A11, docs/decision/records/2026-09-25-deferred-items.md#A12, docs/decision/records/2026-09-27-surface-check.md#A6, docs/decision/records/2026-09-27-surface-check.md#A15, docs/decision/records/2026-09-27-surface-check.md#A22

| Kind | detail | Requirement setting the conditions |
|---|---|---|
| too_many_lines | The number of lines | REQ-core-038 |
| too_many_requirements | The number of requirements | REQ-core-039 |
| mutant_timeout | The change description | REQ-core-140 |
| equivalent_stale | The string of "file" and "change", exactly as written in the entry of the list of equivalents, joined by ": " | REQ-core-142 |
| guide_stale | The string of the `ID`, the `fingerprint` written in the entry of the `guide mark`, and the current `fingerprint`, separated by one half-width space | REQ-core-204 |
| deferred_with_test | The `ID` of the `requirement` under `deferral` or of the `deferred scenario` | REQ-core-211 |
| depends_on_deferred | The string of the referring `ID` and the referenced `ID` separated by one half-width space | REQ-core-212 |
| surface_unspecified_stale | The string of "kind" and "name", exactly as written in the entry of the list of unspecified surfaces, separated by one half-width space | REQ-core-234 |

## Examples

```gherkin
@id=EX-core-005 @about=REQ-core-032 @source=docs/decision/records/records.md#A61,docs/decision/records/records.md#A47
Scenario: An ID in three places gives two duplicates
  Given the heading "REQ-001" is in three places
  When "kotowari check" is run
  Then two duplicate_id errors are emitted
  And no duplicate_id is emitted on the first heading line

@id=EX-core-266 @about=REQ-core-174 @source=docs/decision/records/2026-09-22-ir-engine.md#A33,docs/decision/records/2026-09-22-ir-engine.md#A35,docs/decision/records/records.md#A102,docs/decision/records/ir-form.md#文書,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: Each of the three situations outside the declarations is an error
  Given there are a `topic document` with a non-empty line outside any `code block` directly under a "## " heading, a `topic document` with a non-gherkin `code block` under the "## Examples" heading, and a `glossary` whose `title` is not in the form the schema declares
  When "kotowari check --format json" is run
  Then one `error` each of unknown_line, unknown_code_block and glossary_title_invalid is emitted
  And each detail is as in TBL-core-008 and each "line" is as in TBL-core-019

@id=EX-core-376 @about=REQ-core-174,REQ-core-113 @source=docs/decision/records/2026-09-24-guide-gaps.md#A1,docs/decision/records/2026-09-22-ir-engine.md#A89,docs/decision/records/ir-form.md#出力
Scenario: Lines inside a non-gherkin block are not read as gherkin
  Given under "## Examples" of a `topic document` there is a `code block` that starts with "```text" and contains the line "メモ"
  When "kotowari check --format text" is run
  Then an unknown_code_block `error` is emitted, and invalid_gherkin_line is not emitted on the line "メモ"

@id=EX-core-377 @about=REQ-core-174 @source=docs/decision/records/2026-09-24-guide-gaps.md#A6,docs/decision/records/2026-09-24-guide-gaps.md#A10,docs/decision/records/2026-09-22-ir-engine.md#A89,docs/decision/records/ir-form.md#出力
Scenario: The title of a glossary must be Glossary
  Given the `title` of a `glossary` is "# 用語集"
  When "kotowari check --format text" is run
  Then a glossary_title_invalid `error` is emitted
```
