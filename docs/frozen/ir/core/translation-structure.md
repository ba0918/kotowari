# The skeleton of pairs, the switcher line and links

English | [日本語](translation-structure.ja.md)

Covers the `skeleton` that must match between the `side` files of a `pair`, the `switcher line` placed on each `side` of the `IR` and of a `guide`, the checks of links inside the `side` files of a `pair`, and the LLM's procedure for keeping a `pair` in step. How a `pair` is read and the `consistency record` are covered by translation-pairs.md.

## Requirements

### REQ-core-344: Matching the skeleton

- kind: algorithm
- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#D3
- definition: TBL-core-044, TBL-core-045
- verification: unit

### REQ-core-345: Skeleton mismatches

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When the `skeleton` of the `side` of a language other than the `first language` does not match the `skeleton` of the `side` of the `first language` in any of the parts of TBL-core-044, kotowari raises one translation_structure_mismatch `error` for each such `side`, with "path" set to that `side` and detail to the name of the first non-matching part in the order of the rows of TBL-core-044 for that kind of document. "line" is the line, on that `side`, of the first element within that part that differs when compared in the order of the lines of the `side` of the `first language`; when the difference is a difference in count, or that element is absent from that `side`, it is null. Line numbers themselves are not compared. Whether the meaning is the same is not checked.

### REQ-core-346: The switcher line

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#A36, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

On each `side` of a `pair` of the `IR` and of a `guide`, when the first non-blank line after the `title` (the first non-blank line of the file if there is no `title`) is not the same as the line that lists the "language_name" of the `UI text` of each language in the order of the `language list`, separated by " | ", with the one for that `side`'s language as plain text and the ones for other languages in the form "[<name>](<file name of that language's side>)", kotowari raises a translation_switcher_invalid `error` with "path" set to that `side`, "line" to that line (the `title` line if that line does not exist, and null if there is no `title` either), and detail to the line that should be there. Only a line that is the same as the line that should be there is a `switcher line`; a `switcher line` is not counted as a line of the `scope`, and does not undergo the `term`, `vague word` and `document-name reference` checks. A `pair` of the `overview data` or of the `table of contents` has no `switcher line`.

### REQ-core-347: Links that are checked

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A14, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari always, on each `side` of a `pair` of the `IR`, of a `guide` and of the `overview data`, takes as the links to check those destinations of CommonMark links and images and of link reference definitions outside the `switcher line` that do not start with a scheme (a sequence that starts with an English letter and is followed by ":") and do not start with "#", and reads the part of each destination before "#" and "?" as a path followed from the directory containing that `side`.

### REQ-core-348: Links to the side of another language

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A14, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When the destination of a link that is checked is a `side` of some `pair` and its language differs from the language of the `side` that has the link, kotowari raises a link_language_mismatch `error` with "path" set to the `side` that has the link, "line" to the line of the link, and detail to the destination as written.

### REQ-core-349: Links to decision records

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A14, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A38, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When the destination of a link that is checked is a file under the "decisions.records" or "decisions.adr" location, kotowari raises a link_to_record `error` with "path" set to the `side` that has the link, "line" to the line of the link, and detail to the destination as written.

### REQ-core-350: The procedure for keeping pairs in step

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A25, docs/decision/records/2026-10-05-localization.md#A35, docs/decision/records/2026-10-05-localization.md#A40
- verification: review
- how_to_verify: Read the skill in "agent/skills/kotowari/" and confirm that each of the following is written as a procedure: when any `side` of a `pair` is edited, every other `side` is edited in the same change; when editing, the previous text is taken out with git from the blob hash of the `consistency record` and only the difference is translated, and if it cannot be taken out, the whole text is translated again; after editing, the `consistency record` is rewritten with the values of "translations" of "kotowari list"; and when a language other than English is added to "languages", "labels.<tag>" is written

The skill of kotowari always has the procedure for keeping a `pair` in step.

## Decision tables

### TBL-core-044: Skeleton

- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A30, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#A33, docs/decision/records/2026-10-05-localization.md#D3

In every document, the `switcher line` and natural-language sentences are not part of the `skeleton`. For link destinations, one pointing at a `side` of another language of a `pair` is read as the `side` of the `first language` of that `pair`, and only the part before "#" is compared.

| Document | Part name | What must match |
|---|---|---|
| `topic document` | heading | The depth and order of the "## " and "### " headings. For "### " headings, the `ID` too |
| `topic document` | field | Among the "- " lines under each `item` and at document level, the names, values and order of "- kind:", "- source:", "- verification:", "- definition:", "- deferred:" and "- related:", and the presence of the "- how_to_verify:" line |
| `topic document` | table | The number of tables, the number of rows and columns of each table, and the sequence of `ID` values in each cell |
| `topic document` | gherkin | The contents of the tag lines inside gherkin code blocks, and the sequence of tag lines, "Scenario:" lines and the starting words of step lines (Given, When, Then, And, But) |
| `topic document` | code | The info string and contents of code blocks that are not gherkin |
| `glossary` | glossary | The number of rows of the table of the `glossary`, and the source column of each row |
| `flag record` | flag | The `ID` and order of the "### FLAG-" headings, and the values of "- kind:", "- related:" and "- source:" |
| `guide` | heading | The depth and order of the headings |
| `guide` | mark | The contents and order of each `guide mark` |
| `guide` | code | The info string and contents of code blocks |
| `guide` | table | The number of tables, and the number of rows and columns of each table |
| `guide` | link | The sequence of destinations of the links that are checked |
| `overview data` | frontmatter | The contents of the frontmatter |
| `overview data` | heading | The depth and order of the headings |
| `overview data` | part | The kind and order of each `part`, and the values of each `part` with the strings of the sentence fields of TBL-core-045 removed (the lengths of sequences, the keys, and the values that are not sentences) |
| `overview data` | mark | The contents and order of each `guide mark` |
| `overview data` | table | The number of tables, and the number of rows and columns of each table |
| `overview data` | link | The sequence of destinations of the links that are checked |
| `table of contents` | toc | The nesting of each `contents group`, the order of the name items, and the presence of "note" in each `contents group` |

### TBL-core-045: Sentence fields of overview parts

- source: docs/decision/records/2026-10-05-localization.md#A19, docs/decision/records/2026-10-05-localization.md#A3

Fields not listed here ("tone", "state", "refs", "ref", "width", and the shapes of sequences and nesting) are not sentences, and are part of the `skeleton`.

| Kind of `part` | Sentence fields |
|---|---|
| lead | conclusion, each element of points |
| flow | title and body of a box |
| steps | title, body |
| cards | title, each element of items |
| status | text |
| compare | before, after, why |
| decisions | text, by |
| quiz | q, a |

## Examples

```gherkin
@id=EX-core-526 @about=REQ-core-344,REQ-core-345 @source=docs/decision/records/2026-10-05-localization.md#A19,docs/decision/records/2026-10-05-localization.md#A27
Scenario: When only the sentences differ the skeleton matches
  Given "languages" of the configuration is "[ja, en]", "a.md" and "a.en.md" of the `IR` have the same headings, "- " lines and scenario tags, and differ only in the text of sentences and step lines
  When "kotowari check --format json" is run
  Then no translation_structure_mismatch `finding` is raised

@id=EX-core-527 @about=REQ-core-345 @source=docs/decision/records/2026-10-05-localization.md#A19,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: A differing source raises an error on the first differing line
  Given the documents of EX-core-526 exist, and only the value of the "- source:" line on line 9 of "a.en.md" differs from "a.md"
  When "kotowari check --format json" is run
  Then one translation_structure_mismatch `error` is raised with "path" "docs/ir/a.en.md", "line" 9 and detail "field"

@id=EX-core-528 @about=REQ-core-344 @source=docs/decision/records/2026-10-05-localization.md#A19,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: A differing non-sentence field of an overview part is an error
  Given "languages" of the configuration is "[ja, en]", and in the `overview data` ".kotowari/overview/a.md" and ".kotowari/overview/a.en.md" the "state" of the status `part` differs, "open" and "decided", and "text" differs too
  When "kotowari check --format json" is run
  Then a translation_structure_mismatch `error` is raised with "path" ".kotowari/overview/a.en.md" and detail "part"

@id=EX-core-529 @about=REQ-core-344 @source=docs/decision/records/2026-10-05-localization.md#A26,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D3
Scenario: When only the titles of the table of contents differ it matches
  Given "languages" of the configuration is "[ja, en]", and the `table of contents` "toc.yaml" and "toc.en.yaml" have the same nesting and name items, and differ only in the text of "title" and "note"
  When "kotowari check --format json" is run
  Then no translation_structure_mismatch `finding` is raised

@id=EX-core-530 @about=REQ-core-346 @source=docs/decision/records/2026-10-05-localization.md#A33,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: A correct switcher line raises nothing
  Given "languages" of the configuration is "[ja, en]", "labels.ja.language_name" is "日本語", the next non-blank line after the `title` of "guides/a.md" is "日本語 | [English](a.en.md)", and that line of "guides/a.en.md" is "[日本語](a.md) | English"
  When "kotowari check --format json" is run
  Then no translation_switcher_invalid `finding` is raised

@id=EX-core-531 @about=REQ-core-346 @source=docs/decision/records/2026-10-05-localization.md#A33,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: Without a switcher line the line that should be there is raised
  Given the configuration of EX-core-530 exists, and the next non-blank line after the `title` of "guides/b.md" is "本文。"
  When "kotowari check --format json" is run
  Then a translation_switcher_invalid `error` is raised with "path" "guides/b.md" and detail "日本語 | [English](b.en.md)"

@id=EX-core-532 @about=REQ-core-346 @source=docs/decision/records/2026-10-05-localization.md#A36,docs/decision/records/2026-10-05-localization.md#A33
Scenario: Overview data has no switcher line
  Given "languages" of the configuration is "[ja, en]", and in the `overview data` "a.md" a lead `part` comes right after the `title`
  When "kotowari check --format json" is run
  Then no translation_switcher_invalid `finding` is raised on "a.md"

@id=EX-core-533 @about=REQ-core-347,REQ-core-348 @source=docs/decision/records/2026-10-05-localization.md#A14,docs/decision/records/2026-10-05-localization.md#A30,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: An English guide pointing at the Japanese IR is an error
  Given "languages" of the configuration is "[ja, en]", "docs/ir/a.md" and "docs/ir/a.en.md" of the `IR` exist, and line 5 of "guides/g.en.md" has a link to "../docs/ir/a.md#REQ-001"
  When "kotowari check --format json" is run
  Then a link_language_mismatch `error` is raised with "path" "guides/g.en.md", "line" 5 and detail "../docs/ir/a.md#REQ-001"

@id=EX-core-534 @about=REQ-core-347,REQ-core-348 @source=docs/decision/records/2026-10-05-localization.md#A14,docs/decision/records/2026-10-05-localization.md#A30,docs/decision/records/2026-10-05-localization.md#A27
Scenario: Links to a side of the same language and to outside URLs are not errors
  Given the `IR` of EX-core-533 exists, and "guides/g.en.md" has links to "../docs/ir/a.en.md#other", "https://example.com/a.md" and "#top"
  When "kotowari check --format json" is run
  Then no link_language_mismatch `finding` is raised on "guides/g.en.md"

@id=EX-core-535 @about=REQ-core-349 @source=docs/decision/records/2026-10-05-localization.md#A14,docs/decision/records/2026-10-05-localization.md#A38,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: A link from a paired document to a decision record is an error
  Given "languages" of the configuration is "[ja, en]", and line 3 of "guides/g.md" has a link to "../docs/decision/records/r.md#A1"
  When "kotowari check --format json" is run
  Then a link_to_record `error` is raised with "path" "guides/g.md" and "line" 3
```
