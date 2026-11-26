# Order and lines of findings

English | [日本語](finding-order.ja.md)

Covers the order in which the findings that kotowari's commands output are arranged, and how the line of a finding is decided. The line of findings on a plan is decided in [Findings on the form are gathered into the single kind invalid_plan, with the schema side's kind and details in detail](plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail).

## Findings are arranged by path, line, kind and detail, in that order

Status: implemented

### Problem

The order of the "findings" that check outputs had to be decided. When mutants findings were added later, they were given the same order as check's findings. If they have the same form as check's findings, the receiving side needs only one way of reading them. The reason for choosing these four keys and this order is not recorded.

### Decision

The findings in "findings" are arranged by comparing the following keys in order from the top. That mutants findings are arranged the same way is decided in [Counts are always output, and the number excluded as equivalent is shown every time](mutants.md#counts-are-always-output-and-the-number-excluded-as-equivalent-is-shown-every-time).

| Order | Key | Arrangement |
|---|---|---|
| 1 | path | Byte order |
| 2 | line | null first, then ascending |
| 3 | kind | Byte order |
| 4 | detail | Byte order |

### Confirmations

- C-core-forder-1 The "findings" of a check with findings in several files are arranged in byte order of path, within the same path with those whose line is null first and then in ascending order, followed by byte order of kind and then of detail
- C-core-forder-2 For any number of files and findings, of two adjacent findings in "findings", the later one does not come before the earlier one when compared in the order above

## line is counted from 1, and a finding that covers the whole document is null

Status: implemented

### Problem

How the "line" of a finding is counted, and how findings that cannot point at a single line are handled, had to be decided. The reason is not recorded.

### Decision

The "line" of a finding is counted from 1.

A finding that cannot point at a single line and covers a whole document or file has a "line" of null.

Which kinds of findings this applies to is decided in the topic that decided each finding. The kinds handled in their topics are missing_title and multiple_titles for document titles, unparsable_file for unreadable files, equivalent_stale and equivalent_invalid for the list of equivalents, surface_unspecified_invalid and surface_unspecified_stale for the list of unspecified surfaces, overview_lead_missing, overview_ir_missing, overview_ir_shared and overview_name_conflict for overview source data, overview_toc_invalid, overview_toc_page_missing, overview_toc_page_unknown, overview_toc_page_duplicate and overview_toc_group_empty for the table of contents, and translation_missing, translation_record_invalid and translation_stale for pairs. overview_ir_missing and overview_ir_shared are the current names and are renamed in the overview topic.

### Confirmations

- C-core-forder-3 The "line" of a finding on a problem on line 7 of a document is 7
- C-core-forder-4 A finding that covers a whole document or file has a "line" of null (for example, overview_toc_invalid on the table of contents file, missing_title on a decision document without a title)

## The line of findings that have a line is decided per kind in one table

Status: implemented

### Problem

Which line a finding that has a line points at had to be decided. Except for the kinds whose line is null, it is gathered into one table. The reason for gathering it into a table is not recorded.

### Decision

The "line" of findings that have a line is as in the following table.

| Kind | line |
|---|---|
| confirmation_without_test | The confirmation row that no test points at |
| duplicate_id | The line of the second and each later place ([If the same ID is in several places, duplicate_id is reported for each place from the second on](findings.md#if-the-same-id-is-in-several-places-duplicate_id-is-reported-for-each-place-from-the-second-on)) |
| unresolved_reference, invalid_marker | The line of the test's marker ([A malformed marker gets an invalid_marker error, pointing at the marker's line](test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line)) |
| test_without_id | The first line of the test's node |
| unclosed_code_block | The opening line ([An unclosed code block is an unclosed_code_block error, and from its start to the end of the document is not read](ir-input.md#an-unclosed-code-block-is-an-unclosed_code_block-error-and-from-its-start-to-the-end-of-the-document-is-not-read)) |

The line of kinds not in the table is decided in the topic that decided that kind. mutant_survived and mutant_timeout are decided by the mutation testing topic, invalid_marker on guide markers and guide_stale by the guides topic, surface_without_spec by the surface check topic, the kinds of overview source data and the table of contents by the overview topic, and translation_structure_mismatch, translation_switcher_invalid and link_language_mismatch by the pairs topic.

### Confirmations

- C-core-forder-5 When a confirmation row of an implemented decision has no test, the "line" of confirmation_without_test is that confirmation row
- C-core-forder-6 The "line" of test_without_id on a test without an ID is the first line of the test's node (for a function with a Python decorator, the "def" line; for a PHP method with "#[Test]", the "#[Test]" line)

## The line of the vague-word finding vague_word is the line that has the word

Status: implemented

### Problem

Which line the vague-word finding points at is also to be decided, as one of the findings that have a line. The reason is not recorded.

### Decision

The "line" of the vague-word finding vague_word in a decision document is the line that has that vague word.

### Confirmations

- C-core-forder-7 When only one line of a decision's "Decision" has a vague word, the "line" of vague_word is the number of the line that has that vague word
