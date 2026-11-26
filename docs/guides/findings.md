# Finding kinds

English | [日本語](findings.ja.md)

<!-- @kotowari[docs/decisions/core/findings.md#the-detail-of-a-finding-is-a-short-string-that-identifies-that-finding-decided-per-kind-in-a-table:9134c812, docs/decisions/core/findings.md#findings-are-divided-into-errors-and-notices-and-only-the-kinds-in-the-table-are-notices:d2f0a1cd] -->

This page lists every kind of finding that `kotowari check`, `kotowari mutants` and `kotowari plan` report.
When an unfamiliar kind shows up in the output, look it up in the list below first, then read about its cause and fix in the section that groups it.

Findings come in two severities: error (`error`) and notice (`notice`).

| Severity | Exit code | Meaning |
|---|---|---|
| Error (`error`) | 1 if there is at least one | Must be fixed |
| Notice (`notice`) | Unchanged | A hint worth reviewing. Leaving it does not stand in the way of `complete` in `status` |

Only four kinds are notices: `mutant_timeout`, `equivalent_stale`, `guide_stale` and `surface_unspecified_stale`; all others are errors.
The keys of a finding (`kind`, `severity`, `path`, `line`, `detail`) and its text form are described in [cli.md](cli.md#how-findings-are-printed).

## List of kinds

<!-- @kotowari[docs/decisions/core/findings.md#the-detail-of-a-finding-is-a-short-string-that-identifies-that-finding-decided-per-kind-in-a-table:9134c812, docs/decisions/core/findings.md#findings-are-divided-into-errors-and-notices-and-only-the-kinds-in-the-table-are-notices:d2f0a1cd, docs/decisions/core/finding-order.md#the-line-of-findings-that-have-a-line-is-decided-per-kind-in-one-table:848f58fc, docs/decisions/core/finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null:b98176aa] -->

The "Line" column is the line that the finding's `line` points at.
"None" means a finding about the whole document: `line` is null, and text shows `-`.
"Line text" is the line as read, verbatim (including indentation and trailing whitespace).

| Kind | Severity | detail | Line | Section |
|---|---|---|---|---|
| `missing_title` | Error | File name of the document | None | [Document form](#document-form) |
| `multiple_titles` | Error | The second and later titles (without `# `) | None | [Document form](#document-form) |
| `topic_lead_missing` | Error | The title | The title line | [Document form](#document-form) |
| `unclosed_code_block` | Error | Text of the opening line | The opening line | [Document form](#document-form) |
| `decision_state_invalid` | Error | The value of the state line. Empty when there is none | The state line. The decision's heading when there is none | [Decisions and sections](#decisions-and-sections) |
| `decision_section_missing` | Error | Name of the missing section | The decision's heading | [Decisions and sections](#decisions-and-sections) |
| `unknown_heading` | Error | The heading line as written | That line | [Decisions and sections](#decisions-and-sections) |
| `unknown_line` | Error | Line text | First line of the block | [Decisions and sections](#decisions-and-sections) |
| `invalid_id` | Error | The first word of the row | The row | [Confirmation rows and IDs](#confirmation-rows-and-ids) |
| `duplicate_id` | Error | The ID | The second and later rows | [Confirmation rows and IDs](#confirmation-rows-and-ids) |
| `id_domain_mismatch` | Error | The ID | The row | [Confirmation rows and IDs](#confirmation-rows-and-ids) |
| `vague_word` | Error | The word | That line | [Vague words](#vague-words) |
| `link_target_missing` | Error | The link destination as written | The line of the link | [Links](#links) |
| `link_to_record` | Error | The link destination as written | The line of the link | [Links](#links) |
| `confirmation_without_test` | Error | The ID of the row | The row | [Correspondence with tests](#correspondence-with-tests) |
| `test_without_id` | Error | Name of the test | First line of the test | [Correspondence with tests](#correspondence-with-tests) |
| `unresolved_reference` | Error | The ID written in the mark | The line with the mark | [Correspondence with tests](#correspondence-with-tests) |
| `invalid_marker` | Error | Line text | The line with the mark | [Correspondence with tests](#correspondence-with-tests), [Guide marks](#guide-marks) |
| `unparsable_file` | Error | Path of the file | None | [Correspondence with tests](#correspondence-with-tests), [Surface check](#surface-check) |
| `mutant_survived` | Error | Description of the change | The line of the mutation outcome | [mutants and plan](#mutants-and-plan) |
| `equivalent_invalid` | Error | The list entry's `file` and `change` joined with `: ` | None | [mutants and plan](#mutants-and-plan) |
| `invalid_plan` | Error | The schema side's kind and details joined with `: ` | The line the schema side reported | [mutants and plan](#mutants-and-plan) |
| `surface_without_spec` | Error | `surface-kind surface-name` | First line of the surface's node | [Surface check](#surface-check) |
| `surface_unspecified_invalid` | Error | `kind` and `name` as written in the list, separated by a single space | None | [Surface check](#surface-check) |
| `overview_form_invalid` | Error | Name of the kotowari-markdown-schema finding kind. For a frontmatter violation, `frontmatter` | That line. None for findings about the whole document | [Overview data](../decisions/core/overview-data.md) |
| `overview_part_unknown` | Error | Name of the part kind | Opening line of the part's fence | [Overview data](../decisions/core/overview-data.md) |
| `overview_part_invalid` | Error | Name of the part kind, a single space, and the place that did not match (`(root)` for the whole value, `(yaml)` when it cannot be read as YAML) | Opening line of the part's fence. When it cannot be read as YAML, the error line the YAML reader returned (or the fence's opening line if none is returned) | [Overview data](../decisions/core/overview-data.md) |
| `overview_lead_missing` | Error | Document name (file name without directories) | None | [Overview data](../decisions/core/overview-data.md) |
| `overview_document_missing` | Error | Text of the `documents` entry | None | [Overview data](../decisions/core/overview-data.md) |
| `overview_document_shared` | Error | Path of a decision document listed in the `documents` of two or more overview data files | None | [Overview data](../decisions/core/overview-data.md) |
| `overview_ref_unresolved` | Error | The reference text | Opening line of the part's fence | [Overview data](../decisions/core/overview-data.md) |
| `overview_name_conflict` | Error | File name without `.md` | None | [Overview data](../decisions/core/overview-data.md) |
| `overview_toc_invalid` | Error | The place in the table of contents that did not match (written as for `overview_part_invalid`: `(root)` for the whole value, `(yaml)` when it cannot be read as YAML) | None | [Table of contents check](../decisions/core/overview-toc.md) |
| `overview_toc_page_missing` | Error | Name of an overview data file missing from the table of contents (file name without `.md`) | None | [Table of contents check](../decisions/core/overview-toc.md) |
| `overview_toc_page_unknown` | Error | JSON Pointer of an entry whose name has no overview data (e.g. `/items/1`) | None | [Table of contents check](../decisions/core/overview-toc.md) |
| `overview_toc_page_duplicate` | Error | JSON Pointer of the second and later entries with the same name | None | [Table of contents check](../decisions/core/overview-toc.md) |
| `overview_toc_group_empty` | Error | JSON Pointer of a group whose `items` is empty. `(root)` for the outermost one | None | [Table of contents check](../decisions/core/overview-toc.md) |
| `translation_missing` | Error | Path of the missing side | None | [Pairs](#pairs) |
| `translation_record_invalid` | Error | One of `missing`, `yaml`, `keys`, `value` | None | [Pairs](#pairs) |
| `translation_stale` | Error | `recorded-hash current-hash` | None | [Pairs](#pairs) |
| `translation_structure_mismatch` | Error | Name of the first part that differs (for example `heading`, `confirmation`, `mark`, `code`, `table`, `link`) | Line of the first element that differs. None when the counts differ or the element is absent on that side | [Pairs](#pairs) |
| `translation_switcher_invalid` | Error | The switcher line as it should be | First non-blank line after the title. If there is none, the title line; if there is no title either, none | [Pairs](#pairs) |
| `link_language_mismatch` | Error | The link destination as written | The line of the link | [Pairs](#pairs) |
| `mutant_timeout` | Notice | Description of the change | The line of the mutation outcome | [mutants and plan](#mutants-and-plan) |
| `equivalent_stale` | Notice | The list entry's `file` and `change` joined with `: ` | None | [mutants and plan](#mutants-and-plan) |
| `guide_stale` | Notice | `target written-fingerprint current-fingerprint` | The line where the guide mark starts | [Guide marks](#guide-marks) |
| `surface_unspecified_stale` | Notice | `kind` and `name` as written in the list, separated by a single space | None | [Surface check](#surface-check) |

The list is defined in [the finding kinds of the specification](../decisions/core/findings.md), and the lines in [finding order and lines](../decisions/core/finding-order.md).

## Errors: decision documents

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations:3990395b] -->

The kinds in this part appear on decision documents, the files in the location `decisions.documents` names (`docs/decisions/` by default).
The text lines in each section were actually produced from deliberately broken decision documents; the full output is in the [example](#example) below.
How a decision document is written is in [Writing decision documents](decision-documents.md).

### Document form

<!-- @kotowari[docs/decisions/core/ir-document.md#a-decision-document-has-one-title-and-each-title-from-the-second-on-is-a-multiple_titles-error:e414db7f, docs/decisions/core/findings.md#missing_title-on-a-decision-document-without-a-title-has-the-document-name-as-its-detail:3a7a6a61, docs/decisions/core/ir-document.md#for-the-paragraph-after-the-title-only-its-presence-is-checked-not-its-content-line-count-or-separation-of-responsibilities:cfff5c08, docs/decisions/core/ir-input.md#an-unclosed-code-block-is-an-unclosed_code_block-error-and-from-its-start-to-the-end-of-the-document-is-not-read:c1db707e] -->

```text
docs/decisions/shop/notitle.md:- [error] missing_title notitle.md
docs/decisions/shop/price.md:- [error] multiple_titles Another title
docs/decisions/shop/price.md:13 [error] topic_lead_missing Price
docs/decisions/shop/price.md:17 [error] unclosed_code_block ```text
```

| Kind | Common cause | How to fix |
|---|---|---|
| `missing_title` | There is no `# ` title | Write one `# Title` at the top of the document |
| `multiple_titles` | There are two or more `# ` headings | Keep one title; turn the others into `## ` or lower, or split the document |
| `topic_lead_missing` | No paragraph between the title and the first decision says what the topic covers. The switcher line of a pair does not count | Right after the title (after the switcher line in a pair), write a paragraph on what the topic covers. Only its presence is checked, not its content or length |
| `unclosed_code_block` | A code block was left unclosed | Write a closing line with the same character as the opening, at least as many. From the opening line to the end of the document nothing is read as decisions or rows |

### Decisions and sections

<!-- @kotowari[docs/decisions/core/decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations:3990395b] -->

```text
docs/decisions/shop/cart.md:25 [error] decision_section_missing Decision
docs/decisions/shop/cart.md:27 [error] decision_state_invalid done
docs/decisions/shop/cart.md:33 [error] unknown_heading ### Notes
docs/decisions/shop/cart.md:35 [error] unknown_line The rate is in percent.
docs/decisions/shop/cart.md:37 [error] decision_section_missing Problem
docs/decisions/shop/cart.md:41 [error] unknown_line An explanation written before the sections.
```

| Kind | Common cause | How to fix |
|---|---|---|
| `decision_state_invalid` | The first paragraph under a decision's heading is not a state line, or its value is none of `Status: proposed`, `Status: implemented`, `状態: 提案`, `状態: 実装済み` | Write the state in the side's language: proposed (decided, not built yet) or implemented (built, the current behaviour) |
| `decision_section_missing` | A decision has no `### Problem` or no `### Decision` section. detail is the missing section's name, spelled in the language of the decision's state line | Write the missing section |
| `unknown_heading` | A `###` heading inside a decision is none of `Problem`, `Decision`, `Rejected options` and `Confirmations` (or their Japanese spellings). A heading is placed before the first decision, between the state line and the first section, or under `## Open` | Rename it to one of the four sections, or move its content into a section |
| `unknown_line` | A block other than the state line stands between a decision's heading and its first section. A block under `### Confirmations` is not a list. A block stands under an unknown heading. A block under `## Open` is not a list | Move the text into a section of the decision. Write each open question as a list item |

Every `##` heading other than `## Open` starts one decision. `####` and deeper headings inside a section belong to that section.

### Confirmation rows and IDs

<!-- @kotowari[docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4, docs/decisions/core/findings.md#if-the-same-id-is-in-several-places-duplicate_id-is-reported-for-each-place-from-the-second-on:3c1e544c, docs/decisions/core/findings.md#if-the-group-name-of-a-confirmation-id-does-not-match-the-group-folder-it-is-placed-in-it-is-an-id_domain_mismatch-error:cf755b08] -->

```text
docs/decisions/shop/cart.md:21 [error] duplicate_id C-shop-cart-2
docs/decisions/shop/cart.md:22 [error] id_domain_mismatch C-cart-3
docs/decisions/shop/cart.md:23 [error] invalid_id cart-4
```

| Kind | Common cause | How to fix |
|---|---|---|
| `invalid_id` | A row under `### Confirmations` does not start with an ID of the form `C-<name>-<number>` (the name in lowercase letters, digits and `-`; the number from 1 without a leading zero) | Start the row with its ID, such as `C-shop-cart-4` |
| `duplicate_id` | Two rows have the same ID. It appears on the second and later rows, in byte order of path and then line order | Give the later row the number after the largest one the topic has used. Do not reuse a number of a deleted row |
| `id_domain_mismatch` | In a document under a group folder (`docs/decisions/shop/`), the group in the row's ID is not that folder's name | Match the name to the folder, as in `C-shop-cart-3`. Documents placed directly in the location are not checked |

### Vague words

<!-- @kotowari[docs/decisions/core/decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check:270bb62b, docs/decisions/core/finding-order.md#the-line-of-the-vague-word-finding-vague_word-is-the-line-that-has-the-word:b01cfba2, docs/decisions/core/translation-pairs.md#the-vague-word-check-is-made-on-decision-and-confirmations-of-every-side-of-decision-documents:c6a9614d] -->

```text
docs/decisions/shop/cart.md:15 [error] vague_word 適切に
```

| Kind | Common cause | How to fix |
|---|---|---|
| `vague_word` | The `### Decision` or `### Confirmations` section of a decision contains a vague word (substring match). The defaults are 「適切に」 ("appropriately"), 「必要に応じて」 ("as needed"), 「通常は」 ("normally") and 「など」 ("etc.") | Write concretely what happens. The word list can be changed with `vague_words` in the configuration |

`### Problem` and `### Rejected options` are not checked, nor are code blocks and text inside double quotes.
In a pair, every side is checked.
There is no check of terms: the glossary `CONTEXT.md` of a group is a document for people, and check does not read its content.

### Links

<!-- @kotowari[docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75, docs/decisions/core/translation-structure.md#links-from-decision-documents-and-their-paired-sides-to-the-frozen-tree-are-errors:f09f73fc] -->

```text
docs/decisions/shop/cart.md:3 [error] link_target_missing price.md#prices-exclude-tax
docs/decisions/shop/cart.md:45 [error] link_to_record ../../frozen/stock.md
```

| Kind | Common cause | How to fix |
|---|---|---|
| `link_target_missing` | A link in a decision document points, inside the location of decision documents, at a `.md` file that is not a decision document, or at a heading that the document does not have. A heading was renamed and the links to it were not fixed | Point the link at an existing document and heading. The anchor follows GitHub's rule for headings. When you rename a heading, fix the links and guide marks that point at it in the same change |
| `link_to_record` | A link on any side of a decision document points under `docs/frozen/` | Remove the link. The frozen tree holds old-form documents and is not grounds for the current rules |

Links outside the location of decision documents and links to files other than `.md` are not checked by `link_target_missing`.

## Errors: tests and guides

### Correspondence with tests

<!-- @kotowari[docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4, docs/decisions/core/confirmations.md#write-by-reading-at-the-head-of-a-line-not-checked-by-tests:bfbb46c7, docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678, docs/decisions/core/test-markers.md#a-malformed-marker-gets-an-invalid_marker-error-pointing-at-the-markers-line:915d8722, docs/decisions/core/test-discovery.md#a-test-file-with-a-syntax-error-gets-unparsable_file-and-is-skipped:4d52e291] -->

```text
docs/decisions/shop/cart.md:20 [error] confirmation_without_test C-shop-cart-2
tests/broken.rs:- [error] unparsable_file tests/broken.rs
tests/cart.rs:5 [error] unresolved_reference C-shop-cart-77
tests/cart.rs:9 [error] invalid_marker // @kotowari[
tests/cart.rs:11 [error] test_without_id broken_mark
tests/cart.rs:14 [error] test_without_id no_mark
```

| Kind | Common cause | How to fix |
|---|---|---|
| `confirmation_without_test` | No test's mark lists the ID of a row of an implemented decision, and the row does not start with `(by reading)` | Write `@kotowari[C-…]` in the comment just before a test that confirms the row. It does not appear for a decision whose state is proposed |
| `test_without_id` | The test has no mark. The mark was written inside the function body, or in a comment separated by a blank line | Write the mark in the comment block right before the test |
| `unresolved_reference` | A test mark lists an ID that no decision document has. Usually a typo | Fix the ID. Malformed elements in a mark, such as `C-shop-cart`, also give this error |
| `invalid_marker` | A test mark is empty or holds only separators. The `]` is not on the same line | Close `@kotowari[ID, ID]` on one line |
| `unparsable_file` | A test file has a syntax error and tree-sitter cannot read it | Fix the file's syntax. The file is skipped and its marks are not read |

A mark that is not in the preceding comment block of any test is ignored without a finding.
So if you put a mark in the wrong place, you get `test_without_id` and `confirmation_without_test`, not `invalid_marker`.
How to write marks is described in [marks.md](marks.md).

### Guide marks

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7, docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce, docs/decisions/core/languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change:de5a25a2] -->

```text
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-rounded 12345678 -
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-the-sum-of-the-line-prices 00000000 ff4409e4
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[C-shop-cart-1] -->
```

| Kind | Severity | Common cause | How to fix |
|---|---|---|---|
| `guide_stale` | Notice | The Decision or Confirmations of the decision changed after the guide was written, so the fingerprint in the mark differs from the current one. If the current fingerprint is `-`, the target names no decision (the document or heading is gone, or the heading was renamed) | Reread the section and update it to match the current decision from `kotowari query '<target>'`, then copy the third part of the detail (the current fingerprint) into the mark. For `-`, point the entry at the decision that holds the content now, or remove it |
| `invalid_marker` | Error | An entry in a guide mark has no `:fingerprint`. The fingerprint is not eight lowercase hexadecimal characters. The part before `:` is not of the target form `<path>#<anchor>`. The `]` is not on the same line | Use the form `<target>:<fingerprint>`. Copy the target from `id` and the fingerprint from `fingerprint` in `kotowari list` |

When a guide mark is malformed, even the valid entries in it are not compared.
Copying only the fingerprint without updating the section just hides the outdated section.
The steps are in [writing-guides.md](writing-guides.md).

### Surface check

<!-- @kotowari[docs/decisions/core/surface.md#a-surface-not-in-the-specification-gets-one-surface_without_spec-error-at-the-place-in-the-code-where-it-is-first-found:210038a0, docs/decisions/core/surface.md#if-the-name-of-a-surface-is-quoted-in-decision-or-confirmations-of-a-decision-that-surface-is-in-the-specification:6c702ea9, docs/decisions/core/surface-unspecified.md#a-malformed-entry-is-an-error-and-an-entry-no-longer-needed-is-a-notice:090bd4da, docs/decisions/core/surface.md#only-surface-files-in-the-languages-of-the-rules-are-read-and-parsed-into-syntax-trees-and-other-surface-files-are-not-read:82d0c416] -->

These appear only in projects that set `surface.rules` in the configuration ([surface.md](surface.md)).

```text
docs/surface-unspecified.yaml:- [error] surface_unspecified_invalid flag --legacy
docs/surface-unspecified.yaml:- [notice] surface_unspecified_stale flag --old
src/cli.rs:12 [error] surface_without_spec flag --verbose
```

| Kind | Severity | Common cause | How to fix |
|---|---|---|---|
| `surface_without_spec` | Error | The name of a surface taken from the code does not appear, as an exact match of the contents inside double quotes or backquotes, in the Decision or Confirmations of any decision. Surfaces with the same kind and name are reported once, at the first place in byte order of path and then line order | Quote the name in the decision that defines that surface (this is a specification change, so do it in brainstorming). If it is not going into the specification now, add it with a reason to the list of unspecified surfaces during brainstorming or adoption. Do not add it to the list midway through implementation just to make the check pass |
| `surface_unspecified_invalid` | Error | An entry in the list of unspecified surfaces does not have exactly the three keys `kind`, `name` and `why`, a value is not a string, or `why` is only whitespace | Fix the entry's form. Until it is fixed, the entry excludes no surface |
| `surface_unspecified_stale` | Notice | No surface in the code matches an entry in the list of unspecified surfaces, or the matching surface has been written into a decision | Delete the entry. If you renamed the surface, fix `name` |

If a file in a surface rule's language has a syntax error, `unparsable_file` appears. Other surface files are not read, so nothing appears for them.
If `unparsable_file` was already reported for the same file as a test file, it is not reported again.

### Pairs

<!-- @kotowari[docs/decisions/core/translation-pairs.md#guides-overview-source-data-and-the-table-of-contents-have-a-side-for-every-language-of-the-language-list:a581e9d7, docs/decisions/core/translation-pairs.md#the-agreement-record-holds-the-blob-hash-of-each-side-as-of-the-last-time-they-were-confirmed-to-have-the-same-content:f29e5af9, docs/decisions/core/translation-pairs.md#if-a-side-is-changed-without-updating-the-agreement-record-that-side-is-stale:af90c9ef, docs/decisions/core/translation-structure.md#the-sides-of-a-pair-must-match-in-their-skeleton-everything-other-than-natural-language-sentences:ee743f40, docs/decisions/core/translation-structure.md#each-side-of-a-guide-pair-has-a-switcher-line-right-after-the-title:b65862b3, docs/decisions/core/translation-structure.md#links-inside-guides-and-overview-source-data-point-at-the-side-of-the-same-language:5e21d8c6, docs/decisions/core/languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs:535e227a, docs/decisions/core/translation-pairs.md#for-pairs-of-decision-documents-counting-and-output-are-done-on-the-first-languages-side-only:f5bf6692, docs/decisions/core/translation-pairs.md#the-staleness-of-guide-markers-is-reported-on-every-side-of-a-guide-pair:de48ce93] -->

These appear only in projects that list two or more languages in `languages` in the configuration ([languages and pairs](config.md#languages-and-pairs--languages-and-labels)).
Decision documents, their glossaries, guides, overview data and the table of contents are paired.
translation_missing and translation_stale appear per side.

```text
docs/decisions/shop/cart.ja.md:- [error] translation_stale de4818cf633031472beea547627e00905cf3dede 2246e3d2ff17d246033d32bcb65c8bb8717bd840
docs/decisions/shop/cart.ja.md:22 [error] translation_structure_mismatch confirmation
guides/a.i18n.yaml:- [error] translation_record_invalid missing
guides/a.md:- [error] translation_missing guides/a.ja.md
guides/b.md:3 [error] translation_switcher_invalid English | [日本語](b.ja.md)
guides/b.md:5 [error] link_language_mismatch ../docs/decisions/shop/cart.ja.md#合計は行の値段の和にする
```

| Kind | Common cause | How to fix |
|---|---|---|
| `translation_missing` | The side for one of the pair's languages is missing. When the first language's side is missing and only other languages' sides exist, it appears on such a side, and that side is not read by the other checks | Write the missing side as a translation of an existing side. If the side is not needed, delete it |
| `translation_record_invalid` | The consistency record `<stem>.i18n.yaml` is missing, cannot be read as YAML, its keys are not exactly the file names of each language, or a value is not 40 lowercase hexadecimal characters. While this appears, `translation_stale` is not reported | Write the record using the hashes in `translations` from `kotowari list` |
| `translation_stale` | A side's current git blob hash differs from the record. Only one side was edited | Bring the other sides to the same content, then rewrite the record's hashes with the values from `kotowari list` |
| `translation_structure_mismatch` | The skeleton other than sentences differs from the first language's side. For a decision document: the level and order of headings and which section each `###` is, the state of each decision, and each row's ID and whether it starts with `(by reading)`. For a guide: headings, guide marks, code blocks, table shapes and link destinations. For overview data and the table of contents: the non-sentence fields of parts, and the nesting and names | Give it the same skeleton as the first language's side and translate only the sentences. Whether the meaning matches is confirmed in review |
| `translation_switcher_invalid` | On a side of a decision document or a guide, the first non-blank line after the title is not the switcher line. The switcher line lists each language's `language_name` in `languages` order, separated by ` \| `, with its own language as plain text and every other language as `[name](file name of that side)` | Write the line from detail as is, right after the title |
| `link_language_mismatch` | A link inside a side of a pair points at another language's side. URLs starting with a scheme, `#`-only links and the links in the switcher line are not checked | Point at the side in the same language |

The decisions, row IDs, the comparison with test marks, fingerprints and the output of `list` and `query` come from the first language's side alone.
On another side of a decision document, only its headings, its links and its vague words are checked.
`guide_stale` for guide marks appears on every side of a guide.

### mutants and plan

<!-- @kotowari[docs/decisions/core/mutants.md#a-miss-is-an-error-and-a-timeout-is-a-notice-and-nothing-is-output-for-caught-mutations-and-unviable-mutations:e1818807, docs/decisions/core/equivalents.md#an-entry-whose-text-is-gone-from-the-source-gets-an-equivalent_stale-notice:c9878315, docs/decisions/core/equivalents.md#a-malformed-entry-is-an-equivalent_invalid-error-and-removes-no-miss:78271fb2, docs/decisions/core/plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail:1608f12c] -->

These five kinds do not appear in `kotowari check`.
Only `kotowari mutants` and `kotowari plan` report them.
The lines below come from running `kotowari mutants --tool cargo-mutants outcomes.json --format text` with a small outcome file and list of equivalents.

```text
.kotowari/equivalents.yaml:- [notice] equivalent_stale src/price.rs: replace - with + in discount
src/price.rs:3 [error] mutant_survived replace >= with > in total
src/price.rs:18 [notice] mutant_timeout replace += with *= in count_up
```

| Kind | Severity | When it appears | How to fix |
|---|---|---|---|
| `mutant_survived` | Error | All tests passed even with the mutation in (a miss), and it matches no entry in the list of equivalents | Add a test that catches the mutation. If the mutation does not change the output, add it to the list of equivalents with a reason |
| `mutant_timeout` | Notice | The mutation outcome is a timeout | Check whether the mutation makes the tests never finish |
| `equivalent_stale` | Notice | The `file` has no line with the same wording as the `text` of an entry in the list of equivalents | The code has changed. Delete the entry or update it to the current wording |
| `equivalent_invalid` | Error | An entry in the list of equivalents is missing keys, has extra keys, or has values of the wrong form | Fix the entry's form |
| `invalid_plan` | Error | The plan deviates from the fixed sections and step form | Fix it as the schema side's explanation in detail says |

Details are in [commands/mutants.md](commands/mutants.md) and [commands/plan.md](commands/plan.md).

## Example

<!-- @kotowari[docs/decisions/core/finding-order.md#findings-are-arranged-by-path-line-kind-and-detail-in-that-order:430be622] -->

This is the result of running `check` in a small repository containing broken decision documents, tests and a guide.
The text lines in each section above were taken from here.
Findings are sorted by path, then line, then kind, so one row can carry several findings, as line 22 does here.

```console
$ kotowari check --format text
docs/decisions/shop/cart.md:3 [error] link_target_missing price.md#prices-exclude-tax
docs/decisions/shop/cart.md:15 [error] vague_word 適切に
docs/decisions/shop/cart.md:20 [error] confirmation_without_test C-shop-cart-2
docs/decisions/shop/cart.md:21 [error] duplicate_id C-shop-cart-2
docs/decisions/shop/cart.md:22 [error] confirmation_without_test C-cart-3
docs/decisions/shop/cart.md:22 [error] id_domain_mismatch C-cart-3
docs/decisions/shop/cart.md:23 [error] invalid_id cart-4
docs/decisions/shop/cart.md:25 [error] decision_section_missing Decision
docs/decisions/shop/cart.md:27 [error] decision_state_invalid done
docs/decisions/shop/cart.md:33 [error] unknown_heading ### Notes
docs/decisions/shop/cart.md:35 [error] unknown_line The rate is in percent.
docs/decisions/shop/cart.md:37 [error] decision_section_missing Problem
docs/decisions/shop/cart.md:41 [error] unknown_line An explanation written before the sections.
docs/decisions/shop/cart.md:45 [error] link_to_record ../../frozen/stock.md
docs/decisions/shop/notitle.md:- [error] missing_title notitle.md
docs/decisions/shop/price.md:- [error] multiple_titles Another title
docs/decisions/shop/price.md:13 [error] topic_lead_missing Price
docs/decisions/shop/price.md:17 [error] unclosed_code_block ```text
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-rounded 12345678 -
guides/cart.md:3 [notice] guide_stale docs/decisions/shop/cart.md#the-total-is-the-sum-of-the-line-prices 00000000 ff4409e4
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[C-shop-cart-1] -->
tests/broken.rs:- [error] unparsable_file tests/broken.rs
tests/cart.rs:5 [error] unresolved_reference C-shop-cart-77
tests/cart.rs:9 [error] invalid_marker // @kotowari[
tests/cart.rs:11 [error] test_without_id broken_mark
tests/cart.rs:14 [error] test_without_id no_mark
$ echo $?
1
```

## Common pitfalls

### A typo in a mark's ID does not give `test_without_id`

<!-- @kotowari[docs/decisions/core/test-markers.md#even-a-marker-that-points-only-at-ids-in-no-document-makes-its-test-count-as-one-with-a-marker:5e328dc1, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

A test whose mark points only at nonexistent IDs still counts as "marked".
Instead, `unresolved_reference` appears on the mark's line, and `confirmation_without_test` on the row the test was meant for.

```console
$ kotowari check --format text      # the mark says C-shop-cart-11
docs/decisions/shop/cart.md:19 [error] confirmation_without_test C-shop-cart-1
tests/cart.rs:1 [error] unresolved_reference C-shop-cart-11
```

### Notices remain, yet the exit code is 0

<!-- @kotowari[docs/decisions/core/findings.md#findings-are-divided-into-errors-and-notices-and-only-the-kinds-in-the-table-are-notices:d2f0a1cd] -->

This is as specified.
The four notice kinds change neither the exit code nor `complete` in `status`.

## Related

- Specification: [finding kinds](../decisions/core/findings.md), [finding order and lines](../decisions/core/finding-order.md)
- Form and order of findings: [cli.md](cli.md#how-findings-are-printed)
- Commands that report findings: [check](commands/check.md), [mutants](commands/mutants.md), [plan](commands/plan.md)
- How to write decision documents: [decision-documents.md](decision-documents.md)
- Test marks: [marks.md](marks.md)
- Guide marks: [writing-guides.md](writing-guides.md)
