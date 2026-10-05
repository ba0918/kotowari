# Finding kinds

English | [日本語](findings.ja.md)

<!-- @kotowari[REQ-core-029:f774ea58, REQ-core-030:4fe666c1, REQ-core-031:069a10b3] -->

This page lists every kind of finding that `kotowari check`, `kotowari mutants` and `kotowari plan` report.
When an unfamiliar kind shows up in the output, look it up in the list below first, then read about its cause and fix in the section that groups it.

Findings come in two severities: error (`error`) and notice (`notice`).

| Severity | Exit code | Meaning |
|---|---|---|
| Error (`error`) | 1 if there is at least one | Must be fixed |
| Notice (`notice`) | Unchanged | A hint worth reviewing. Leaving it does not stand in the way of `complete` in `status` |

Only eight kinds are notices: `too_many_lines`, `too_many_requirements`, `mutant_timeout`, `equivalent_stale`, `guide_stale`, `deferred_with_test`, `depends_on_deferred` and `surface_unspecified_stale`; all others are errors.
The keys of a finding (`kind`, `severity`, `path`, `line`, `detail`) and its text form are described in [cli.md](cli.md#how-findings-are-printed).

## List of kinds

<!-- @kotowari[TBL-core-008:67ba1ee9, TBL-core-009:e3c60d7c, TBL-core-019:d5c9adce, REQ-core-027:a594c5e0] -->

The "Line" column is the line that the finding's `line` points at.
"None" means a finding about the whole document: `line` is null, and text shows `-`.
"Line text" is the line as read, verbatim (including indentation and trailing whitespace).

| Kind | Severity | detail | Line | Section |
|---|---|---|---|---|
| `missing_title` | Error | File name of the document | None | [Document form](#document-form) |
| `multiple_titles` | Error | The second and later titles (without `# `) | None | [Document form](#document-form) |
| `missing_scope` | Error | File name of the document | None | [Document form](#document-form) |
| `unknown_line` | Error | Line text | That line | [Document form](#document-form) |
| `unknown_code_block` | Error | Text of the opening line | The opening line | [Document form](#document-form) |
| `unclosed_code_block` | Error | Text of the opening line | The opening line | [Document form](#document-form) |
| `unknown_heading` | Error | Heading text | That line | [Item headings and lines](#item-headings-and-lines) |
| `unknown_field` | Error | Line text | That line | [Item headings and lines](#item-headings-and-lines) |
| `duplicate_field` | Error | Name of the line | The second line | [Item headings and lines](#item-headings-and-lines) |
| `missing_field` | Error | Name of the missing line | The item's heading | [Item headings and lines](#item-headings-and-lines) |
| `missing_statement` | Error | ID of the item | The item's heading | [Item values and statements](#item-values-and-statements) |
| `missing_table` | Error | ID of the decision table | The item's heading | [Item values and statements](#item-values-and-statements) |
| `verification_missing` | Error | ID of the requirement | The item's heading | [Item values and statements](#item-values-and-statements) |
| `verification_invalid` | Error | The value | The item's heading | [Item values and statements](#item-values-and-statements) |
| `unknown_kind` | Error | The value | The item's heading | [Item values and statements](#item-values-and-statements) |
| `algorithm_without_definition` | Error | ID of the requirement | The item's heading | [Item values and statements](#item-values-and-statements) |
| `missing_source` | Error | ID of the item, or the term. For a `- deferred:` line with an empty value, `deferred` | The item's heading. For a scenario, its tag line; for a term, the table row; for a `- deferred:` line with an empty value, that line | [Sources](#sources) |
| `source_invalid` | Error | The source string | The line where the source is written | [Sources](#sources) |
| `unknown_term` | Error | The backquoted string | That line | [Terms and vague words](#terms-and-vague-words) |
| `unclosed_backtick` | Error | Line text | That line | [Terms and vague words](#terms-and-vague-words) |
| `vague_word` | Error | The word | That line | [Terms and vague words](#terms-and-vague-words) |
| `missing_document` | Error | The document-name reference string | That line | [Terms and vague words](#terms-and-vague-words) |
| `glossary_invalid` | Error | File name of the document | None | [Glossaries](#glossaries) |
| `glossary_title_invalid` | Error | Text of the title line | The title line | [Glossaries](#glossaries) |
| `invalid_glossary_row` | Error | Line text | That line | [Glossaries](#glossaries) |
| `duplicate_term` | Error | The term | The line of the duplicate | [Glossaries](#glossaries) |
| `duplicate_id` | Error | The ID | The second and later headings | [IDs and references](#ids-and-references) |
| `unresolved_reference` | Error | The ID | The line where the reference is written | [IDs and references](#ids-and-references) |
| `invalid_id` | Error | The value | The tag line | [IDs and references](#ids-and-references) |
| `id_domain_mismatch` | Error | The ID | The heading or the tag line | [IDs and references](#ids-and-references) |
| `missing_tag` | Error | Name of the missing tag | The tag line | [Scenarios](#scenarios) |
| `unknown_tag` | Error | Name of the tag | The tag line | [Scenarios](#scenarios) |
| `invalid_gherkin_line` | Error | Line text | That line | [Scenarios](#scenarios) |
| `requirement_without_test` | Error | ID of the requirement | The item's heading | [Correspondence with tests](#correspondence-with-tests) |
| `scenario_without_test` | Error | ID of the scenario | The tag line | [Correspondence with tests](#correspondence-with-tests) |
| `test_without_id` | Error | Name of the test | First line of the test | [Correspondence with tests](#correspondence-with-tests) |
| `invalid_marker` | Error | Line text | The line with the mark | [Correspondence with tests](#correspondence-with-tests), [Guide marks](#guide-marks) |
| `unparsable_file` | Error | Path of the file | None | [Correspondence with tests](#correspondence-with-tests), [Surface check](#surface-check) |
| `record_field_missing` | Error | Name of the missing supplementary line | The numbered line | [Decision records](#decision-records) |
| `record_field_unknown` | Error | Name of the supplementary line | That line | [Decision records](#decision-records) |
| `revision_link_invalid` | Error | The link's href | That line | [Decision records](#decision-records) |
| `mutant_survived` | Error | Description of the change | The line of the mutation outcome | [mutants and plan](#mutants-and-plan) |
| `equivalent_invalid` | Error | The list entry's `file` and `change` joined with `: ` | None | [mutants and plan](#mutants-and-plan) |
| `invalid_plan` | Error | The schema side's kind and details joined with `: ` | The line the schema side reported | [mutants and plan](#mutants-and-plan) |
| `surface_without_spec` | Error | `surface-kind surface-name` | First line of the surface's node | [Surface check](#surface-check) |
| `surface_unspecified_invalid` | Error | `kind` and `name` as written in the list, separated by a single space | None | [Surface check](#surface-check) |
| `overview_form_invalid` | Error | Name of the kotowari-markdown-schema finding kind. For a frontmatter violation, `frontmatter` | That line. None for findings about the whole document | [Overview data](../ir/core/overview-data.md) |
| `overview_part_unknown` | Error | Name of the part kind | Opening line of the part's fence | [Overview data](../ir/core/overview-data.md) |
| `overview_part_invalid` | Error | Name of the part kind, a single space, and the place that did not match (`(root)` for the whole value, `(yaml)` when it cannot be read as YAML) | Opening line of the part's fence. When it cannot be read as YAML, the error line the YAML reader returned (or the fence's opening line if none is returned) | [Overview data](../ir/core/overview-data.md) |
| `overview_lead_missing` | Error | Document name (file name without directories) | None | [Overview data](../ir/core/overview-data.md) |
| `overview_ir_missing` | Error | Text of the `ir` entry | None | [Overview data](../ir/core/overview-data.md) |
| `overview_ir_shared` | Error | Path of an IR document listed in the `ir` of two or more overview data files | None | [Overview data](../ir/core/overview-data.md) |
| `overview_ref_unresolved` | Error | The reference text | Opening line of the part's fence | [Overview data](../ir/core/overview-data.md) |
| `overview_name_conflict` | Error | File name without `.md` | None | [Overview data](../ir/core/overview-data.md) |
| `overview_toc_invalid` | Error | The place in the table of contents that did not match (written as for `overview_part_invalid`: `(root)` for the whole value, `(yaml)` when it cannot be read as YAML) | None | [Table of contents check](../ir/core/overview-toc.md) |
| `overview_toc_page_missing` | Error | Name of an overview data file missing from the table of contents (file name without `.md`) | None | [Table of contents check](../ir/core/overview-toc.md) |
| `overview_toc_page_unknown` | Error | JSON Pointer of an entry whose name has no overview data (e.g. `/items/1`) | None | [Table of contents check](../ir/core/overview-toc.md) |
| `overview_toc_page_duplicate` | Error | JSON Pointer of the second and later entries with the same name | None | [Table of contents check](../ir/core/overview-toc.md) |
| `overview_toc_group_empty` | Error | JSON Pointer of a group whose `items` is empty. `(root)` for the outermost one | None | [Table of contents check](../ir/core/overview-toc.md) |
| `translation_missing` | Error | Path of the missing side | None | [Pairs](#pairs) |
| `translation_record_invalid` | Error | One of `missing`, `yaml`, `keys`, `value` | None | [Pairs](#pairs) |
| `translation_stale` | Error | `recorded-hash current-hash` | None | [Pairs](#pairs) |
| `translation_structure_mismatch` | Error | Name of the first part that differs (`heading`, `field`, `table`, `gherkin`, `code`, `glossary`, `flag`, `mark`, `link`, `frontmatter`, `part`, `toc`) | Line of the first element that differs. None when the counts differ or the element is absent on that side | [Pairs](#pairs) |
| `translation_switcher_invalid` | Error | The switcher line as it should be | First non-blank line after the title. If there is none, the title line; if there is no title either, none | [Pairs](#pairs) |
| `link_language_mismatch` | Error | The link destination as written | The line of the link | [Pairs](#pairs) |
| `link_to_record` | Error | The link destination as written | The line of the link | [Pairs](#pairs) |
| `too_many_lines` | Notice | Number of lines | None | [Document size](#document-size) |
| `too_many_requirements` | Notice | Number of requirements | None | [Document size](#document-size) |
| `mutant_timeout` | Notice | Description of the change | The line of the mutation outcome | [mutants and plan](#mutants-and-plan) |
| `equivalent_stale` | Notice | The list entry's `file` and `change` joined with `: ` | None | [mutants and plan](#mutants-and-plan) |
| `guide_stale` | Notice | `ID written-fingerprint current-fingerprint` | The line where the guide mark starts | [Guide marks](#guide-marks) |
| `deferred_with_test` | Notice | ID of the deferred requirement or deferred scenario | The requirement's heading. For a scenario, its tag line | [Deferral](#deferral) |
| `depends_on_deferred` | Notice | `referring-ID referenced-ID` | The line where the reference is written | [Deferral](#deferral) |
| `surface_unspecified_stale` | Notice | `kind` and `name` as written in the list, separated by a single space | None | [Surface check](#surface-check) |

The list is defined in [TBL-core-008 and TBL-core-009 of findings.md](../ir/core/findings.md), and the lines in [TBL-core-019 of finding-order.md](../ir/core/finding-order.md).

## Errors: IR documents and items

The kinds in this part appear on documents in the IR location (`docs/ir/` by default).
The text lines in each section were actually produced from deliberately broken IR; the full output is in the [example](#example) below.

### Document form

<!-- @kotowari[REQ-core-034:085fbd4f, REQ-core-035:c0b9706a, REQ-core-036:cb09c25d, REQ-core-174:18bfdd84, REQ-core-112:2fc914d5] -->

```text
docs/ir/misc/notitle.md:- [error] missing_title notitle.md
docs/ir/misc/table.md:- [error] missing_scope table.md
docs/ir/misc/table.md:- [error] multiple_titles もう一つの題名
docs/ir/misc/table.md:6 [error] unknown_line 説明の行をここに書いた。
docs/ir/misc/table.md:14 [error] unknown_code_block ```text
docs/ir/misc/table.md:18 [error] unclosed_code_block ```gherkin
```

| Kind | Common cause | How to fix |
|---|---|---|
| `missing_title` | There is no `# ` title | Write one `# Title` at the top of the document |
| `multiple_titles` | There are two or more `# ` headings | Keep one title; turn the others into `## ` or lower, or split the document |
| `missing_scope` | No line between the title and the first `## ` states what the document covers | Right after the title, write one to three lines on what the document covers (glossaries and problem records are exempt) |
| `unknown_line` | Prose written directly under a `## ` (before the first `### `) | Move the explanation into the scope lines or into an item's statements |
| `unknown_code_block` | A non-gherkin code block placed under `## Examples` | Put concrete examples in a ` ```gherkin ` block. Lines inside a non-gherkin block are not read as gherkin, so `invalid_gherkin_line` does not appear |
| `unclosed_code_block` | A code block was left unclosed | Write a closing line with the same character as the opening, at least as many. Until it is closed, nothing after it is checked |

### Item headings and lines

<!-- @kotowari[REQ-core-043:3153d158, REQ-core-044:a3667e95, REQ-core-045:d0c5af4a, REQ-core-098:1e550dfe] -->

```text
docs/ir/shop/cart.md:12 [error] duplicate_field verification
docs/ir/shop/cart.md:26 [error] unknown_field - owner: 山田
docs/ir/shop/cart.md:30 [error] missing_field how_to_verify
docs/ir/shop/cart.md:47 [error] unknown_heading ### 備考
```

| Kind | Common cause | How to fix |
|---|---|---|
| `unknown_heading` | A `### ` heading is not of the form `### REQ-…: name`. A `### EX-…` was used as a heading. A `#### ` or deeper heading was used | Use the form `### ID: name`. Write concrete examples as gherkin scenarios, not headings |
| `unknown_field` | A `- xxx:` line that the item kind does not have, or a list line not of the `xxx:` form | Use only the lines each item kind can have ([TBL-core-011 of ir-items.md](../ir/core/ir-items.md)). Put explanations in statements |
| `duplicate_field` | The same `- xxx:` line written twice | Merge them into one. Only the first value is read |
| `missing_field` | A requirement has no `- kind:`. A `verification: review` requirement has no `- how_to_verify:`. A problem record has no `- kind:` or `- related:` | Add the missing line. For a review requirement, write in `- how_to_verify:` the steps a person or an LLM follows to confirm it |

When `- verification:` is missing, only `verification_missing` appears, and when `- source:` is missing, only `missing_source` appears; `missing_field` does not.
The content under a malformed heading is still read as an item, so other findings may appear alongside `unknown_heading` ([common pitfalls](#one-mistake-produces-several-findings-on-the-same-line)).

### Item values and statements

<!-- @kotowari[REQ-core-047:5359a274, REQ-core-099:05bae6bf, REQ-core-048:8122d6a7, REQ-core-049:741e10ff, REQ-core-050:bf67c1c1, REQ-core-051:84d0e9f4] -->

```text
docs/ir/shop/cart.md:16 [error] algorithm_without_definition REQ-shop-002
docs/ir/shop/cart.md:22 [error] unknown_kind sometimes
docs/ir/shop/cart.md:22 [error] verification_invalid manual
docs/ir/misc/table.md:8 [error] missing_table TBL-misc-001
```

| Kind | Common cause | How to fix |
|---|---|---|
| `missing_statement` | A requirement other than algorithm, a property, or a problem record has no statement | Under the heading, write a statement that is neither a list nor a table |
| `missing_table` | A decision table has no Markdown table | Write the table |
| `verification_missing` | A requirement has no `- verification:` line | Write one of `unit`, `property`, `proof`, `review` |
| `verification_invalid` | The `- verification:` value is none of the four | Change it to one of the four |
| `unknown_kind` | The `- kind:` value is not one of the fixed values | For requirements: `event_driven`, `state_driven`, `ubiquitous`, `prohibition`, `invariant`, `algorithm`. For problem records: `contradiction`, `gap`, `ambiguity` |
| `algorithm_without_definition` | A `kind: algorithm` requirement has no `- definition:` pointing at a decision table or property | Write the rule as a decision table or property and point at it with `- definition: TBL-…` |

### Sources

<!-- @kotowari[REQ-core-059:8ec34dd6, REQ-core-058:0012abf7, TBL-core-012:df66ea46, REQ-core-210:0772cc2a] -->

```text
docs/ir/shop/cart.md:22 [error] missing_source REQ-shop-003
docs/ir/shop/cart.md:52 [error] missing_source EX-shop-001
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
```

| Kind | Common cause | How to fix |
|---|---|---|
| `missing_source` | An item has no `- source:`, or it is empty. A scenario has no `@source`. The source column of a glossary is empty. The value of `- deferred:` is empty (detail is `deferred`, and the line is that line) | Write the decision it is based on as `path#decision-number`. For a scenario without `@id`, the detail is the text of the `Scenario:` line |
| `source_invalid` | The source target has no line with that decision number. The path is inside neither `decisions.records` nor `decisions.adr`. It is not of the form `path#anchor`. The value of `- deferred:` is checked by the same rule, and the line is that `- deferred:` line | Point at a decision number that exists in a decision record (such as `A12`), or at the text of a `## ` heading in an ADR. Write the full path from the base directory (`docs/decision/records/records.md#A26`) |

kotowari only checks that the source target exists.
It does not check whether that decision really states what the requirement says.

### Terms and vague words

<!-- @kotowari[REQ-core-064:75e8708a, REQ-core-065:e0fe5913, REQ-core-116:a444487b, REQ-core-066:79261ab9, REQ-core-070:08653b2b] -->

```text
docs/ir/greet/greet.md:21 [error] unknown_term 停止
docs/ir/shop/cart.md:28 [error] unclosed_backtick 在庫が無いとき、システムは`在庫切れ を出し、適切に知らせる。
docs/ir/shop/cart.md:28 [error] vague_word 適切に
docs/ir/shop/cart.md:36 [error] missing_document price.md
```

`unknown_term`, `unclosed_backtick` and `vague_word` look at the statements of requirements and properties, and at the Given, When, Then, And and But lines of gherkin.
They do not look at `- ` lines, tag lines, `Scenario:` lines, the meaning column of a glossary, or the body of a problem record.

| Kind | Common cause | How to fix |
|---|---|---|
| `unknown_term` | A backquoted word is neither a glossary term nor an ID. Paths and code fragments are no exception | If it is a term, add a row to the `CONTEXT.md` in the document's directory or a directory above it. If it is a concrete value, enclose it in double quotes (`"…"`). When there is no glossary at all above the document, every backquoted span other than an ID becomes this error |
| `unclosed_backtick` | The number of backquotes in the line is odd (one was left unclosed) | Close it. On a line with this error, the term and ID checks are not performed |
| `vague_word` | A statement contains a vague word (substring match). The defaults are 「適切に」 ("appropriately"), 「必要に応じて」 ("as needed"), 「通常は」 ("normally") and 「など」 ("etc.") | Write concretely what happens. The word list can be changed with `vague_words` in the configuration |
| `missing_document` | The document referenced as `xxx.md` in a statement does not exist | Fix the document name. A name without `/` is looked up as a document in the same directory, and a name with `/` as a path relative to the IR location. Parent directories are not searched |

### Glossaries

<!-- @kotowari[REQ-core-117:8d685215, REQ-core-122:6644ecb4, REQ-core-123:d1df9959, REQ-core-174:18bfdd84] -->

```text
docs/ir/misc/CONTEXT.md:- [error] glossary_invalid CONTEXT.md
docs/ir/misc/CONTEXT.md:1 [error] glossary_title_invalid # 用語
docs/ir/shop/CONTEXT.md:6 [error] duplicate_term かご
docs/ir/shop/CONTEXT.md:7 [error] invalid_glossary_row | 在庫 | |
```

| Kind | Common cause | How to fix |
|---|---|---|
| `glossary_invalid` | `CONTEXT.md` has no table with the header `| Term | Meaning | Source |` and a delimiter row (for example, the column names were translated into Japanese) | Place a table with this header after the title and before the first `## `. While this error stands, the glossary is treated as having zero terms |
| `glossary_title_invalid` | The glossary's title is not `# Glossary` (for example, `# 用語集`) | Change the title to `# Glossary` |
| `invalid_glossary_row` | A table row has fewer than three cells, or the term cell is empty | Fill in the three cells: term, meaning and source |
| `duplicate_term` | The same term appears in an earlier row of the same glossary, or in a glossary in a directory above | Delete the duplicate row. The definition that counts is the first one, on the side nearer the root |

### IDs and references

<!-- @kotowari[REQ-core-032:6a7acc09, REQ-core-054:cb34b375, REQ-core-114:85e0323f, REQ-core-167:33769b03] -->

```text
docs/ir/shop/cart.md:30 [error] duplicate_id REQ-shop-001
docs/ir/shop/cart.md:38 [error] id_domain_mismatch REQ-cart-004
docs/ir/shop/cart.md:62 [error] invalid_id EX-1
tests/cart.rs:1 [error] unresolved_reference REQ-shop-077
```

| Kind | Common cause | How to fix |
|---|---|---|
| `duplicate_id` | A heading or scenario with the same ID exists in two or more places | Change the ID of the second and later ones. The first is the one in the document earlier in byte order of path, or the higher one within the same document |
| `unresolved_reference` | `- definition:`, `@about`, `- related:`, a backquoted ID in a statement, or a test mark points at an ID that does not exist. Usually a typo | Fix the ID. Malformed elements in a mark, such as `REQ001`, also give this error |
| `invalid_id` | The `@id` value is not of the `EX-…` ID form | Use the form `EX-name-001`. The number has three or more digits |
| `id_domain_mismatch` | The name in the ID differs from the name of the first-level directory under the IR location | For a document in `docs/ir/shop/`, match the name to the directory, as in `REQ-shop-001` |

The ID form is described in [REQ-core-124 of ir-references.md](../ir/core/ir-references.md).

### Scenarios

<!-- @kotowari[REQ-core-053:a27fbee7, REQ-core-052:20cc2383, REQ-core-113:5093fb04] -->

```text
docs/ir/shop/cart.md:52 [error] unknown_tag @tags
docs/ir/shop/cart.md:58 [error] missing_tag @id
docs/ir/shop/cart.md:64 [error] invalid_gherkin_line   Examples:
```

| Kind | Common cause | How to fix |
|---|---|---|
| `missing_tag` | The line just before the scenario has no `@id` or `@about`, or its value is empty | Write `@id=EX-… @about=REQ-… @source=…` on the one line right before `Scenario:` |
| `unknown_tag` | A tag other than `@id`, `@about`, `@source` was written. A word not starting with `@` was written on the tag line | Use only those three |
| `invalid_gherkin_line` | A gherkin block contains a line that is none of: a tag, `Scenario:`, a Given/When/Then/And/But step, a `#` comment, or a blank line (this includes `Feature:`, `Background:`, `Scenario Outline:`, `Examples:` and data tables). A step was written before `Scenario:` | Write scenarios with only `Scenario:` and step lines. If there are several situations, split them into separate scenarios |

### Document size

<!-- @kotowari[REQ-core-038:170fdd3e, REQ-core-039:f3d75ba1] -->

These two are notices and do not change the exit code.

```text
docs/ir/shop/cart.md:- [notice] too_many_lines 65
docs/ir/shop/cart.md:- [notice] too_many_requirements 5
```

| Kind | When it appears | How to review |
|---|---|---|
| `too_many_lines` | The document has more lines than `limits.lines` (default 200) | Check whether one document mixes several responsibilities. If it does not, leaving it as is is fine |
| `too_many_requirements` | A topic document has more requirements than `limits.requirements` (default 10) | Same as above |

There is no need to split a document just because of its line count or number of requirements.
The example above was produced with `limits.lines` lowered to 60 and `limits.requirements` to 3.

## Errors: tests, guides and decision records

### Correspondence with tests

<!-- @kotowari[REQ-core-085:288046ea, REQ-core-137:cb66f5a8, REQ-core-086:8035b3f8, REQ-core-072:0d066a71, REQ-core-083:c11fae0c] -->

```text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
docs/ir/greet/greet.md:26 [error] scenario_without_test EX-greet-001
tests/greet.rs:6 [error] test_without_id rejects_empty_name
tests/cart.rs:5 [error] invalid_marker // @kotowari[tests/cart.rs:9 [error] invalid_marker // @kotowari[]
tests/broken.rs:- [error] unparsable_file tests/broken.rs
```

| Kind | Common cause | How to fix |
|---|---|---|
| `requirement_without_test` | For a non-review requirement that is not deferred, there is neither a mark listing its ID nor a mark listing the ID of a scenario that has the requirement in its `@about` | Write `@kotowari[REQ-…]` or `@kotowari[EX-…]` in the comment just before a test. A scenario's mark also covers the requirements in that scenario's `@about`. If you have decided not to build the requirement yet, defer it instead of writing a test ([deferred.md](deferred.md)) |
| `scenario_without_test` | No mark lists the scenario's ID (it does not appear for a scenario whose `@about` requirements are all review, or for a deferred scenario) | Write `@kotowari[EX-…]` on a test that checks the scenario. A requirement's mark does not cover the scenario |
| `test_without_id` | The test has no mark. The mark was written inside the function body, or in a comment separated by a blank line | Write the mark in the comment block right before the test |
| `invalid_marker` | A test mark is empty or holds only separators. The `]` is not on the same line | Close `@kotowari[ID, ID]` on one line |
| `unparsable_file` | A test file has a syntax error and tree-sitter cannot read it | Fix the file's syntax. The file is skipped and its marks are not read |

A mark that is not in the preceding comment block of any test is ignored without a finding.
So if you put a mark in the wrong place, you get `test_without_id` and `requirement_without_test`, not `invalid_marker`.
How to write marks is described in [marks.md](marks.md).

### Deferral

<!-- @kotowari[REQ-core-211:e38f935b, REQ-core-212:070f1cfc] -->

```text
docs/ir/greet/greet.md:13 [notice] depends_on_deferred REQ-greet-001 REQ-greet-003
docs/ir/greet/greet.md:15 [notice] deferred_with_test REQ-greet-003
```

| Kind | Severity | Common cause | How to fix |
|---|---|---|---|
| `deferred_with_test` | Notice | A mark contains the ID of a deferred requirement or deferred scenario. Either you finished building it and forgot to remove `- deferred:`, or the mark is wrong | If it is built, write the decision in a decision record and then remove the `- deferred:` line. If it is not built yet, remove the mark |
| `depends_on_deferred` | Notice | A non-deferred requirement or property, or a scenario that is not a deferred scenario, refers to a deferred requirement (through `- definition:`, `@about`, or a backquoted ID in a statement or step). One per reference | Defer the referring item too, bring the requirement back from deferral, or drop the reference. Each is a specification decision |

How to write a deferral and what it does is described in [deferred.md](deferred.md).

### Guide marks

<!-- @kotowari[REQ-core-202:907dd989, REQ-core-204:fa82249e] -->

```text
guides/cart.md:3 [notice] guide_stale REQ-shop-001 00000000 b27eedf4
guides/cart.md:3 [notice] guide_stale REQ-shop-404 12345678 -
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[REQ-shop-002] -->
```

| Kind | Severity | Common cause | How to fix |
|---|---|---|---|
| `guide_stale` | Notice | The body of the IR item changed after the guide was written, so the fingerprint in the mark differs from the current one. If the current fingerprint is `-`, the ID has been removed from the IR | Reread the section and update it to match the current body from `kotowari query ID`, then copy the third part of the detail (the current fingerprint) into the mark |
| `invalid_marker` | Error | An entry in a guide mark has no `:fingerprint`. The fingerprint is not eight lowercase hexadecimal characters. It is not of the ID form. The `]` is not on the same line | Use the form `ID:fingerprint`. Copy the fingerprint from `fingerprint` in `kotowari query ID` |

When a guide mark is malformed, even the valid entries in it are not compared.
Copying only the fingerprint without updating the section just hides the outdated section.
The steps are in [writing-guides.md](writing-guides.md).

### Surface check

<!-- @kotowari[REQ-core-227:9203538c, REQ-core-233:f788681d, REQ-core-234:8a2d9f6e, REQ-core-236:45fa5c5f] -->

These appear only in projects that set `surface.rules` in the configuration ([surface.md](surface.md)).

```text
docs/surface-unspecified.yaml:- [error] surface_unspecified_invalid flag --legacy
docs/surface-unspecified.yaml:- [notice] surface_unspecified_stale flag --old
src/cli.rs:12 [error] surface_without_spec flag --verbose
```

| Kind | Severity | Common cause | How to fix |
|---|---|---|---|
| `surface_without_spec` | Error | The name of a surface taken from the code does not appear, as an exact match of the contents inside double quotes or backquotes, in any requirement statement, decision table cell or scenario step. Surfaces with the same kind and name are reported once, at the first place in byte order of path and then line order | Quote the name in the requirement that defines that surface (this is a specification change, so do it in brainstorming). If it is not going into the specification now, add it with a reason to the list of unspecified surfaces during brainstorming or adoption. Do not add it to the list midway through implementation just to make the check pass |
| `surface_unspecified_invalid` | Error | An entry in the list of unspecified surfaces does not have exactly the three keys `kind`, `name` and `why`, a value is not a string, or `why` is only whitespace | Fix the entry's form. Until it is fixed, the entry excludes no surface |
| `surface_unspecified_stale` | Notice | No surface in the code matches an entry in the list of unspecified surfaces, or the matching surface has been written into the IR | Delete the entry. If you renamed the surface, fix `name` |

If a file in a surface rule's language has a syntax error, `unparsable_file` appears. Other surface files are not read, so nothing appears for them.
If `unparsable_file` was already reported for the same file as a test file, it is not reported again.

### Decision records

<!-- @kotowari[REQ-core-130:f72db551, REQ-core-131:f15cd286, REQ-core-132:cb010297] -->

```text
docs/decision/records/2026-09-24-shop.md:10 [error] record_field_missing why
docs/decision/records/2026-09-24-shop.md:13 [error] record_field_unknown reason
docs/decision/records/2026-09-24-shop.md:17 [error] revision_link_invalid #A9
```

`record_field_missing` and `record_field_unknown` apply only to decision records that have a `## Context` heading.
`revision_link_invalid` applies to every decision record.

| Kind | Common cause | How to fix |
|---|---|---|
| `record_field_missing` | A decision line lacks the supplementary line required for its section (`why` for Agreements, Prohibitions, Delegated and Rejected; `decides` for Undecided; `superseded_by` for Superseded) | Add a supplementary line such as `  - why: reason` |
| `record_field_unknown` | The name of a supplementary line is none of `why`, `rejected`, `decided_by`, `superseded_by`, `decides`, `related` | Change it to one of the six |
| `revision_link_invalid` | The `superseded_by` value has no link. The record or decision number the link points at does not exist | Point at an existing decision in the form `[A9](#A9)` or `[A3](./other.md#A3)` |

### Pairs

<!-- @kotowari[REQ-core-338:e8fc8d74, REQ-core-339:81c3b742, REQ-core-341:3b853a41, REQ-core-345:589adca5, REQ-core-346:80c8eb1e, REQ-core-347:9420480c, REQ-core-348:12801181, REQ-core-349:f2ae1da2, REQ-core-342:7087b408, REQ-core-343:4f2cb7d4] -->

These appear only in projects that list two or more languages in `languages` in the configuration ([languages and pairs](config.md#languages-and-pairs--languages-and-labels)).
Even when one pair is read from two locations, such as the IR and the guides, the same finding appears only once. translation_missing and translation_stale appear per side.

```text
docs/ir/a.md:- [error] translation_missing docs/ir/a.en.md
docs/ir/a.i18n.yaml:- [error] translation_record_invalid keys
guides/a.md:- [error] translation_stale 78981922613b2afb6025042ff6bd878ac1994e85 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391
docs/ir/b.en.md:9 [error] translation_structure_mismatch field
guides/b.md:3 [error] translation_switcher_invalid 日本語 | [English](b.en.md)
guides/g.en.md:5 [error] link_language_mismatch ../docs/ir/a.md#REQ-001
guides/g.md:3 [error] link_to_record ../docs/decision/records/r.md#A1
```

| Kind | Common cause | How to fix |
|---|---|---|
| `translation_missing` | The side for one of the pair's languages is missing. When the first language's side is missing and only other languages' sides exist, it appears on such a side, and that side is not read by the other checks | Write the missing side as a translation of an existing side. If the side is not needed, delete it |
| `translation_record_invalid` | The consistency record `<stem>.i18n.yaml` is missing, cannot be read as YAML, its keys are not exactly the file names of each language, or a value is not 40 lowercase hexadecimal characters. While this appears, `translation_stale` is not reported | Write the record using the hashes in `translations` from `kotowari list` |
| `translation_stale` | A side's current git blob hash differs from the record. Only one side was edited | Bring the other sides to the same content, then rewrite the record's hashes with the values from `kotowari list` |
| `translation_structure_mismatch` | The skeleton other than sentences (headings, item fields and sources, table shape and IDs, scenario tags and step keywords, code blocks, glossary sources, guide marks, link destinations, the non-sentence fields of overview parts, the nesting and names of the table of contents) differs from the first language's side | Give it the same skeleton as the first language's side and translate only the sentences. Whether the meaning matches is confirmed in review |
| `translation_switcher_invalid` | On a side of the IR or a guide, the first non-blank line after the title is not the switcher line. The switcher line lists each language's `language_name` in `languages` order, separated by ` \| `, with its own language as plain text and every other language as `[name](file name of that side)` | Write the line from detail as is, right after the title |
| `link_language_mismatch` | A link inside a side of a pair points at another language's side. URLs starting with a scheme, `#`-only links and the links in the switcher line are not checked | Point at the side in the same language |
| `link_to_record` | A link inside a side of a pair points at a file in the decision record or ADR location | Remove the link. The history can be traced from the IR's sources |

On IR sides in other languages, only the checks for terms (looked up through the chain of that language's glossaries `CONTEXT.<language-tag>.md`), vague words, document-name references, unclosed backquotes and glossary form are run.
Items, IDs, sources, the comparison with test marks, fingerprints, and the output of `list` and `query` are determined by the first language's side alone.
`guide_stale` for guide marks appears on every side of a guide.

### mutants and plan

<!-- @kotowari[REQ-core-139:5736cb73, REQ-core-140:49dd0d6f, REQ-core-142:3cafdd68, REQ-core-143:de58796f, REQ-core-193:ee3eed54] -->

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

This is the result of running `check` in a small repository containing broken IR, tests, guides and decision records.
The text lines in each section above were taken from here and from the example in [commands/check.md](commands/check.md#example).
It shows that several kinds can appear on the same line, and that the content under a malformed heading is still read as an item, so findings pile up.

```console
$ kotowari check --format text
docs/decision/records/2026-09-24-shop.md:10 [error] record_field_missing why
docs/decision/records/2026-09-24-shop.md:13 [error] record_field_unknown reason
docs/decision/records/2026-09-24-shop.md:17 [error] revision_link_invalid #A9
docs/ir/misc/CONTEXT.md:- [error] glossary_invalid CONTEXT.md
docs/ir/misc/CONTEXT.md:1 [error] glossary_title_invalid # 用語
docs/ir/misc/notitle.md:- [error] missing_title notitle.md
docs/ir/misc/table.md:- [error] missing_scope table.md
docs/ir/misc/table.md:- [error] multiple_titles もう一つの題名
docs/ir/misc/table.md:6 [error] unknown_line 説明の行をここに書いた。
docs/ir/misc/table.md:8 [error] missing_table TBL-misc-001
docs/ir/misc/table.md:14 [error] unknown_code_block ```text
docs/ir/misc/table.md:18 [error] unclosed_code_block ```gherkin
docs/ir/shop/CONTEXT.md:6 [error] duplicate_term かご
docs/ir/shop/CONTEXT.md:7 [error] invalid_glossary_row | 在庫 | |
docs/ir/shop/cart.md:- [notice] too_many_lines 65
docs/ir/shop/cart.md:- [notice] too_many_requirements 5
docs/ir/shop/cart.md:12 [error] duplicate_field verification
docs/ir/shop/cart.md:16 [error] algorithm_without_definition REQ-shop-002
docs/ir/shop/cart.md:16 [error] requirement_without_test REQ-shop-002
docs/ir/shop/cart.md:22 [error] missing_source REQ-shop-003
docs/ir/shop/cart.md:22 [error] requirement_without_test REQ-shop-003
docs/ir/shop/cart.md:22 [error] unknown_kind sometimes
docs/ir/shop/cart.md:22 [error] verification_invalid manual
docs/ir/shop/cart.md:26 [error] unknown_field - owner: 山田
docs/ir/shop/cart.md:28 [error] unclosed_backtick 在庫が無いとき、システムは`在庫切れ を出し、適切に知らせる。
docs/ir/shop/cart.md:28 [error] vague_word 適切に
docs/ir/shop/cart.md:30 [error] duplicate_id REQ-shop-001
docs/ir/shop/cart.md:30 [error] missing_field how_to_verify
docs/ir/shop/cart.md:36 [error] missing_document price.md
docs/ir/shop/cart.md:38 [error] id_domain_mismatch REQ-cart-004
docs/ir/shop/cart.md:47 [error] missing_field kind
docs/ir/shop/cart.md:47 [error] missing_source 備考
docs/ir/shop/cart.md:47 [error] missing_statement 備考
docs/ir/shop/cart.md:47 [error] unknown_heading ### 備考
docs/ir/shop/cart.md:47 [error] verification_missing 備考
docs/ir/shop/cart.md:52 [error] missing_source EX-shop-001
docs/ir/shop/cart.md:52 [error] scenario_without_test EX-shop-001
docs/ir/shop/cart.md:52 [error] unknown_tag @tags
docs/ir/shop/cart.md:58 [error] missing_tag @id
docs/ir/shop/cart.md:62 [error] invalid_id EX-1
docs/ir/shop/cart.md:64 [error] invalid_gherkin_line   Examples:
guides/cart.md:3 [notice] guide_stale REQ-shop-001 00000000 b27eedf4
guides/cart.md:3 [notice] guide_stale REQ-shop-404 12345678 -
guides/cart.md:7 [error] invalid_marker <!-- @kotowari[REQ-shop-002] -->
tests/broken.rs:- [error] unparsable_file tests/broken.rs
tests/cart.rs:1 [error] unresolved_reference REQ-shop-077
tests/cart.rs:5 [error] invalid_marker // @kotowari[tests/cart.rs:7 [error] test_without_id broken_mark
tests/cart.rs:9 [error] invalid_marker // @kotowari[]
tests/cart.rs:11 [error] test_without_id empty_mark
$ echo $?
1
```

## Common pitfalls

### One mistake produces several findings on the same line

<!-- @kotowari[REQ-core-043:3153d158] -->

The content under a malformed heading (`### 備考` in the example above) is still read by the item rules.
So `missing_source`, `missing_statement`, `verification_missing` and others appear alongside `unknown_heading`.
Fix the heading first, then run the check again.

### A typo in a mark's ID does not give `test_without_id`

<!-- @kotowari[REQ-core-077:082e5fe4, REQ-core-054:cb34b375] -->

A test whose mark points only at nonexistent IDs still counts as "marked".
Instead, `unresolved_reference` appears on the mark's line, and `requirement_without_test` on the requirement's side.

```console
$ kotowari check --format text      # 印に REQ-greet-02 と書いた
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
tests/greet.rs:5 [error] unresolved_reference REQ-greet-02
```

### Notices remain, yet the exit code is 0

<!-- @kotowari[REQ-core-031:069a10b3] -->

This is as specified.
The eight notice kinds change neither the exit code nor `complete` in `status`.
In kotowari's own repository, eight `too_many_lines` and `too_many_requirements` notices remain, and the exit code is still 0.

## Related

- Specification: [check kinds](../ir/core/findings.md), [finding order and lines](../ir/core/finding-order.md)
- Form and order of findings: [cli.md](cli.md#how-findings-are-printed)
- Commands that report findings: [check](commands/check.md), [mutants](commands/mutants.md), [plan](commands/plan.md)
- Test marks: [marks.md](marks.md)
- Guide marks: [writing-guides.md](writing-guides.md)
