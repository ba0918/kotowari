# Kinds of findings

English | [日本語](findings.ja.md)

Covers, for each kind of finding that kotowari's commands output, its severity (error or notice) and what goes in its detail. When each kind is reported is decided in the topic that decided that kind. The order and line of findings are decided in [Order and lines of findings](finding-order.md), and the JSON keys of findings and the text output form in [Output form](output.md).

## Findings are divided into errors and notices, and only the kinds in the table are notices

Status: implemented

### Problem

For each kind of finding, whether it is an error that changes the exit code or a notice that does not had to be decided.

That a section of a guide has become stale is something to notice, but having CI fail just because a guide lags behind is too heavy. A notice does not block complete in status, and it keeps remaining in the counts of check and status, so a neglected guide is visible at any time. An entry of the list of unspecified surfaces that is no longer needed is handled the same way as an entry of the mutation testing list of equivalents that is no longer needed.

### Decision

A finding has one of two severities, error or notice. A notice is a finding that does not change the exit code. When only notices are reported, check ends with the same exit code as when there is no error (the list of exit codes is decided in [The exit code is 0 when there is no error, 1 when there is an error, and 2 when kotowari stops](cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops)).

The only kinds that are notices are those in the following table. Findings of every kind not in the table are errors.

| Kind | What it is reported on | Where the condition for it is decided |
|---|---|---|
| mutant_timeout | A mutation that timed out | [A miss is an error and a timeout is a notice, and nothing is output for caught mutations and unviable mutations](mutants.md#a-miss-is-an-error-and-a-timeout-is-a-notice-and-nothing-is-output-for-caught-mutations-and-unviable-mutations) |
| equivalent_stale | An entry of the list of equivalents whose text is gone from the source | [An entry whose text is gone from the source gets an equivalent_stale notice](equivalents.md#an-entry-whose-text-is-gone-from-the-source-gets-an-equivalent_stale-notice) |
| guide_stale | An entry of a guide marker whose decision has changed or is gone | [A guide is stale when the Decision or Confirmations change](languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change) |
| surface_unspecified_stale | An entry of the list of unspecified surfaces that is no longer needed | [A malformed entry is an error, and an entry no longer needed is a notice](surface-unspecified.md#a-malformed-entry-is-an-error-and-an-entry-no-longer-needed-is-a-notice) |

### Rejected options

- Make guide_stale an error. CI would fail just because a guide lags behind.

### Confirmations

- C-core-findings-1 Of the kinds of findings kotowari reports, only the kinds in the table of notices have severity "notice", and all other kinds have severity "error"
- C-core-findings-2 A check that reports only notices ends with exit code 0, with the notices kept in the output

## The detail of a finding is a short string that identifies that finding, decided per kind in a table

Status: implemented

### Problem

For each kind, what goes in the detail had to be decided.

Which document a finding is on can be told by path, so the detail does not repeat the path. If the detail of a finding on a test whose name cannot be taken were an empty string, it could not be told which test it is. A stale-guide notice is to show, in that one line alone, which target to rewrite to what.

### Decision

The detail of a finding is a short string that identifies that finding. The kinds whose detail this document decides are as in the following table. "The text of the line" is the text of the line as read (including indentation and trailing whitespace, not reassembled).

| Kind | Severity | detail |
|---|---|---|
| duplicate_id | Error | The ID |
| invalid_id | Error | The value as written |
| unresolved_reference | Error | The ID written in the marker |
| test_without_id | Error | The name of the test. For a test whose name cannot be taken, the text of the whole first line of the test's node with leading and trailing whitespace removed |
| invalid_marker | Error | The text of the line with the marker |
| unparsable_file | Error | The path of the file |
| surface_without_spec | Error | The kind and name of the surface, separated by one half-width space |
| overview_form_invalid | Error | "frontmatter" for a violation of the frontmatter's form |
| overview_toc_invalid | Error | The JSON Pointer of the place in the table of contents that did not match |
| overview_toc_page_missing | Error | The name of the page not in the table of contents |
| overview_toc_page_unknown | Error | The JSON Pointer of that item |
| overview_toc_page_duplicate | Error | The JSON Pointer of that item |
| overview_toc_group_empty | Error | The JSON Pointer of that contents group |
| multiple_titles | Error | The text of the second and later titles with "#" and leading and trailing whitespace removed |
| guide_stale | Notice | The target of the marker entry, the fingerprint written in that entry, and the current fingerprint, separated by one half-width space each. If the target decision does not exist, the current fingerprint is "-" |

The detail of the following kinds and the conditions for them are decided in each topic. All of them are errors.

| Kind | What the error is | Where it is decided |
|---|---|---|
| mutant_survived | A mutation that the tests missed | [A miss is an error and a timeout is a notice, and nothing is output for caught mutations and unviable mutations](mutants.md#a-miss-is-an-error-and-a-timeout-is-a-notice-and-nothing-is-output-for-caught-mutations-and-unviable-mutations) |
| equivalent_invalid | A malformed entry of the list of equivalents | [A malformed entry is an equivalent_invalid error and removes no miss](equivalents.md#a-malformed-entry-is-an-equivalent_invalid-error-and-removes-no-miss) |
| invalid_plan | A plan that departs from the form | [Findings on the form are gathered into the single kind invalid_plan, with the schema side's kind and details in detail](plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail) |
| surface_unspecified_invalid | A malformed entry of the list of unspecified surfaces | [A malformed entry is an error, and an entry no longer needed is a notice](surface-unspecified.md#a-malformed-entry-is-an-error-and-an-entry-no-longer-needed-is-a-notice) |
| confirmation_without_test | A confirmation row of an implemented decision that no test points at | [Check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it) |
| decision_state_invalid | A decision with no state line, or one that is neither "proposed" nor "implemented" | [Write one decision as text in Problem, Decision, Rejected options and Confirmations](decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations) |
| decision_section_missing | A decision without the "Problem" or "Decision" section | Same as above |
| topic_lead_missing | A document with no paragraph after the title saying what the topic covers | Same as above |
| missing_title | A decision document without a title | Same as above. The detail is in [missing_title on a decision document without a title has the document name as its detail](#missing_title-on-a-decision-document-without-a-title-has-the-document-name-as-its-detail) |
| unclosed_code_block | A code block that ends without being closed | [An unclosed code block is an unclosed_code_block error, and from its start to the end of the document is not read](ir-input.md#an-unclosed-code-block-is-an-unclosed_code_block-error-and-from-its-start-to-the-end-of-the-document-is-not-read) |
| id_domain_mismatch | A confirmation ID whose group name does not match the group folder it is placed in | [If the group name of a confirmation ID does not match the group folder it is placed in, it is an id_domain_mismatch error](#if-the-group-name-of-a-confirmation-id-does-not-match-the-group-folder-it-is-placed-in-it-is-an-id_domain_mismatch-error) |
| link_target_missing | A link in a decision document that points at a document or heading that does not exist | [To refer to another decision, link to its heading](decision-documents.md#to-refer-to-another-decision-link-to-its-heading) |
| link_to_record | A link in a decision document or a side of its pair that points under the frozen tree ("docs/frozen/") (the current name; whether to reinterpret it or rename it is decided by the implementing side) | [Links from decision documents and their paired sides to the frozen tree are errors](translation-structure.md#links-from-decision-documents-and-their-paired-sides-to-the-frozen-tree-are-errors) |
| vague_word | A sentence that contains a word in the list of vague words | [Keep the glossary as a document for people, and drop the term check](decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check) |
| overview_part_unknown | A part of an unknown kind in overview source data | [The contents of parts are checked by the kotowari side with view's part schemas](overview-data.md#the-contents-of-parts-are-checked-by-the-kotowari-side-with-views-part-schemas) |
| overview_part_invalid | A part in overview source data that cannot be read or does not match the form | Same as above |
| overview_lead_missing | Overview source data without an opening part | [The first part right after the title is the lead](overview-data.md#the-first-part-right-after-the-title-is-the-lead) |
| overview_ir_missing | An entry in the list of overview source data that points at a decision document that does not exist (the current name; it is renamed) | [The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents) |
| overview_ir_shared | A decision document that is in the lists of two or more overview source data (the current name; it is renamed) | [One decision document is in the list of at most one piece of overview source data](overview-data.md#one-decision-document-is-in-the-list-of-at-most-one-piece-of-overview-source-data) |
| overview_ref_unresolved | A reference in an overview part that cannot be resolved | [References inside parts are written in refs and ref fields](overview-data.md#references-inside-parts-are-written-in-refs-and-ref-fields) |
| overview_name_conflict | Overview source data whose names collide | [overview build draws everything every time, writes only the files whose contents differ, and deletes files no longer produced](overview-commands.md#overview-build-draws-everything-every-time-writes-only-the-files-whose-contents-differ-and-deletes-files-no-longer-produced) |
| translation_missing | A document missing the side of one of the languages of its pair | [Guides, overview source data and the table of contents have a side for every language of the language list](translation-pairs.md#guides-overview-source-data-and-the-table-of-contents-have-a-side-for-every-language-of-the-language-list) |
| translation_record_invalid | A malformed agreement record | [The agreement record holds the blob hash of each side as of the last time they were confirmed to have the same content](translation-pairs.md#the-agreement-record-holds-the-blob-hash-of-each-side-as-of-the-last-time-they-were-confirmed-to-have-the-same-content) |
| translation_stale | A pair in which the current content of some side differs from the agreement record | [If a side is changed without updating the agreement record, that side is stale](translation-pairs.md#if-a-side-is-changed-without-updating-the-agreement-record-that-side-is-stale) |
| translation_structure_mismatch | A pair whose skeleton differs between sides | [The sides of a pair must match in their skeleton, everything other than natural-language sentences](translation-structure.md#the-sides-of-a-pair-must-match-in-their-skeleton-everything-other-than-natural-language-sentences) |
| translation_switcher_invalid | A side whose language switcher line is missing, or whose form or targets are wrong | [Each side of a guide pair has a switcher line right after the title](translation-structure.md#each-side-of-a-guide-pair-has-a-switcher-line-right-after-the-title) |
| link_language_mismatch | A link to a paired document points at the side of another language | [Links inside guides and overview source data point at the side of the same language](translation-structure.md#links-inside-guides-and-overview-source-data-point-at-the-side-of-the-same-language) |

### Rejected options

- For the detail of a violation of the frontmatter's form, borrow the name of a kind with another meaning that the form-checking side holds. The form-checking side does not check frontmatter, so there is no name that fits, and borrowing a name with another meaning would cause misreading.

### Confirmations

- C-core-findings-3 A malformed test marker and a malformed guide marker get invalid_marker with the text of the line with the marker as is (including indentation) as the detail
- C-core-findings-4 A malformed confirmation ID gets invalid_id with the value as written as the detail, and a marker that points at an ID in no document gets unresolved_reference with that ID as the detail
- C-core-findings-5 The detail of test_without_id on a test whose name cannot be taken is the text of the whole first line of the test's node with leading and trailing whitespace removed
- C-core-findings-6 The detail of surface_without_spec on a surface that does not appear in the decision documents is the kind and name of the surface separated by one half-width space ("flag --verbose")
- C-core-findings-7 When an entry of a guide marker points at a decision that has changed, guide_stale is reported with the target, the written fingerprint and the current fingerprint as the detail, and when it points at a decision that does not exist, the current fingerprint is "-"
- C-core-findings-13 The detail of multiple_titles on the second and later titles of a decision document ("#  Second ") is the text of the title with "#" and leading and trailing whitespace removed ("Second")

## If the same ID is in several places, duplicate_id is reported for each place from the second on

Status: implemented

### Problem

That two confirmation rows with the same ID are an error is decided in [Check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it). This decides, when the ID is in several places, where and how many findings are reported. The reason is not recorded.

### Decision

When the same ID is in two or more places, duplicate_id is reported once for each place from the second on, and its line is the line of that place. Nothing is reported for the first place.

The first place is the place in the document that comes first in byte order of path, and within the same document the place with the smaller line.

### Confirmations

- C-core-findings-8 When the same ID is in three places, two duplicate_id findings are reported, and none is reported for the first place
- C-core-findings-9 When two documents have the same ID, duplicate_id is reported on the line with that ID in the document that comes later in byte order of path

## missing_title on a decision document without a title has the document name as its detail

Status: proposed

### Problem

The detail of missing_title on a document without a title is the file name without the directory. Which directory the document is in is shown by path. The check that reads decision documents makes the detail of missing_title on a document without a title an empty string, which does not agree with this rule. The reason for using the document name is not recorded.

### Decision

A decision document without a title gets missing_title with line null and with the file name of that document without the directory as the detail.

### Confirmations

- C-core-findings-10 A decision document without a title in a directory under the location gets missing_title with line null and with the file name without the directory as the detail

## If the group name of a confirmation ID does not match the group folder it is placed in, it is an id_domain_mismatch error

Status: proposed

### Problem

A confirmation ID includes the group name (`C-<group>-<short name of the topic>-<number>`). Writing the name is left to the person writing it, so something is needed to look at whether the name is correct. Looking at whether the group name matches the group folder the document is placed in catches by machine both a mix-up of groups and a misplaced document, and the naming rule does not break down.

The current code that reads decision documents looks only at the form of a confirmation ID, and does not look at whether it matches the group folder.

### Decision

For a decision document under a directory directly below the location of decision documents (a group folder), when the group name of a confirmation ID differs from the name of that folder, an id_domain_mismatch error is reported on that confirmation row. The IDs of documents placed directly in the location are not looked at by this check.

### Rejected options

- Do not check the match, and leave it to the naming rule. Implicit promises would increase.

### Confirmations

- C-core-findings-11 When the ID of a confirmation row in a decision document in the "core" folder of the location starts with "C-view-", an id_domain_mismatch error is reported on that row, and none is reported on rows that start with "C-core-"
- C-core-findings-12 For the confirmation rows of a decision document placed directly in the location, id_domain_mismatch is not reported, whatever the group name of the ID
