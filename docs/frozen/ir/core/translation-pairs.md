# Reading pairs and the consistency record

English | [日本語](translation-pairs.ja.md)

Covers how the `language list` is read, which documents are made into a `pair` and how each `side` is told apart, a missing `side`, the `consistency record` and the blob hash comparison, and how existing checks treat a `pair`. The match of the `skeleton`, the `switcher line` and the link checks are covered by translation-structure.md, and `UI text` by overview-languages.md.

## Requirements

### REQ-core-334: The language list

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A16, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A33
- verification: unit

kotowari always reads the value of "languages" of the `configuration file` as the `language list`, and takes its first language as the `first language`. When the `language list` has one language, kotowari does not read any `pair`, and raises no translation_missing, translation_record_invalid, translation_stale, translation_structure_mismatch, translation_switcher_invalid, link_language_mismatch or link_to_record `finding`.

### REQ-core-335: Errors in the language list

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A21
- verification: unit

When an element of "languages" of the `configuration file` is an empty string or a string containing characters other than lowercase English letters, digits and "-", or when the same language tag appears two or more times, kotowari comes to a `stop` with a configuration error as the reason.

### REQ-core-336: Documents made into pairs

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A11, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A38, docs/decision/records/2026-10-05-localization.md#A16, docs/decision/records/2026-10-05-localization.md#A21
- verification: unit

kotowari always, when the `language list` has two or more languages, reads the documents of the location of the `IR` (including each `glossary` and `flag record`), each `guide`, the `overview data` and the `table of contents` as a `pair`. Files of the "decisions.records" and "decisions.adr" locations are not read as a `pair`.

### REQ-core-337: Telling sides apart

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A15, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#A39, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari always, among the files read in the locations made into a `pair`, takes the part of the file name from its last "." as the extension, reads a file whose part before the extension has the form "<stem>.<tag>", where the tag is a language of the `language list` other than the `first language`, as the `side` of that language of the `pair` of "<stem><extension>" in the same directory, and reads the other files as the `side` of the `first language`. A name with the language tag of the `first language` or a language tag not in the `language list` is read as part of the name of a `side` of the `first language`. A file named "<stem>.i18n.yaml" is not read as a `side`. Even when one `pair` is read from two or more locations, as with the `IR` and a `guide`, the `finding` entries of the `pair` are raised only once. The `side` of another language is looked for by name in the same directory as the `side` of the `first language`, whether or not it matches the globs of "guides.files" and "overview.files".

### REQ-core-338: Missing sides

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A17, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When the `side` of the `first language` of a `pair` exists and the `side` of another language does not, kotowari raises a translation_missing `error` for each missing `side`, with "path" set to the `side` of the `first language`, "line" to null, and detail to the relative path of the missing `side` from the `base directory`. When the `side` of another language exists and the `side` of the `first language` does not, kotowari raises a translation_missing `error` with "path" set to that `side`, "line" to null, and detail to the relative path of the missing `side` of the `first language` from the `base directory`, and does not read that `side` in the other checks.

### REQ-core-339: The form of the consistency record

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A15, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#A28, docs/decision/records/2026-10-05-localization.md#A18, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A39
- verification: unit

For a `pair` that has the `side` "<stem><extension>" of the `first language`, when "<stem>.i18n.yaml" in the same directory does not exist, cannot be read as YAML, is not a mapping from string keys to string values, has a set of keys that is not the same as the set of file names (without directories) of the `side` of each language of the `language list`, or has a value that is not 40 lowercase hexadecimal characters, kotowari raises one translation_record_invalid `error` with "path" set to the relative path of "<stem>.i18n.yaml" from the `base directory` and "line" to null. The detail is "missing" when it does not exist, "yaml" when it cannot be read as YAML, "keys" when it is not a mapping or the set of keys differs, and "value" when the form of a value differs; when two or more apply, the earlier one in this order is used.

### REQ-core-340: blob hash

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A18
- verification: unit

kotowari always takes the blob hash of a file to be the SHA-1 of the bytes formed by joining, in this order, "blob ", the byte count of the file in decimal, one 0 byte, and the bytes of the file, written as 40 lowercase hexadecimal characters (the same value "git hash-object" gives for that file).

### REQ-core-341: Stale sides

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A18, docs/decision/records/2026-10-05-localization.md#A27, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When a `pair` has a `consistency record` with no translation_record_invalid `error`, and the current blob hash of an existing `side` differs from the value for that `side`'s file name in the `consistency record`, kotowari raises a translation_stale `error` for each such `side`, with "path" set to that `side`, "line" to null, and detail to the string of the value in the `consistency record` and the current blob hash, separated by one half-width space.

### REQ-core-342: A pair of the IR is treated as one document

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#A23, docs/decision/records/2026-10-05-localization.md#A24, docs/decision/records/2026-10-05-localization.md#D2, docs/decision/records/2026-10-05-localization.md#A34, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari always, for a `pair` of the `IR`, performs the reading of each `item` and `scenario`, the duplicate `ID` check, the reference checks of `source` and `ID`, the matching against the `mark` entries of tests, the `fingerprint`, the "items" of "kotowari list", the output of "kotowari query", and the judgement of each `surface` on the `side` of the `first language` only, and reads the `item` and `scenario` entries of the `side` of another language only when taking the body of a page of another language (REQ-core-354). Of the existing checks, the only ones performed on the `side` of another language are the `term`, `vague word` and `document-name reference` checks (REQ-core-063 to REQ-core-070), unclosed backquotes (REQ-core-116), and the form checks of the `glossary` (REQ-core-117, REQ-core-122, REQ-core-123, REQ-core-174); a `term` is looked up in the `chain` of each `glossary` of that language's `side` ("CONTEXT.<tag>.md"), and a `vague word` is checked against the single list of "vague_words" of the configuration. The "files" and "lines" of the output of check and status count every `side`.

### REQ-core-343: Pairs of guides and overviews

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A31, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

kotowari always reads every `side` of a `pair` of a `guide` as a `guide`, performs the `guide mark` checks and raises the guide_stale `notice` on every `side`. On every `side` of a `pair` of the `overview data` it performs the checks of the form of the `overview data`, each `part`, the opening lead, references and each `guide mark` (REQ-core-281 to REQ-core-283, REQ-core-285, REQ-core-286), and performs the check of the IR documents covered (REQ-core-284), the overlap of page names (REQ-core-305) and the matching against the `table of contents` (REQ-core-328, REQ-core-329) on the `side` of the `first language` only. On every `side` of a `pair` of the `table of contents` it performs the form checks of the `table of contents` (REQ-core-327, REQ-core-330).

## Examples

```gherkin
@id=EX-core-512 @about=REQ-core-334 @source=docs/decision/records/2026-10-05-localization.md#A16,docs/decision/records/2026-10-05-localization.md#A27
Scenario: With one language no pair is read
  Given the configuration has no "languages", and the location of the `IR` has "a.md" and "a.en.md"
  When "kotowari check --format json" is run
  Then no translation_missing `finding` is raised, and "a.en.md" is read as a `side` of the `first language`

@id=EX-core-513 @about=REQ-core-335 @source=docs/decision/records/2026-10-05-localization.md#A21
Scenario: A wrong form of a language tag stops
  Given "languages" of the configuration is "[ja, EN]"
  When "kotowari check" is run
  Then the exit code is 2, and it comes to a `stop` with a configuration error

@id=EX-core-514 @about=REQ-core-336,REQ-core-337,REQ-core-338 @source=docs/decision/records/2026-10-05-localization.md#A11,docs/decision/records/2026-10-05-localization.md#A15,docs/decision/records/2026-10-05-localization.md#A17,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: A missing English side is an error
  Given "languages" of the configuration is "[ja, en]", and the location of the `IR` has "a.md" but not "a.en.md"
  When "kotowari check --format json" is run
  Then a translation_missing `error` is raised with "path" "docs/ir/a.md" and detail "docs/ir/a.en.md"

@id=EX-core-515 @about=REQ-core-338 @source=docs/decision/records/2026-10-05-localization.md#A17,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: When the side of the first language is missing the existing side is the error
  Given "languages" of the configuration is "[ja, en]", and the location of the `IR` has only "b.en.md"
  When "kotowari check --format json" is run
  Then a translation_missing `error` is raised with "path" "docs/ir/b.en.md" and detail "docs/ir/b.md"

@id=EX-core-516 @about=REQ-core-337 @source=docs/decision/records/2026-10-05-localization.md#A31,docs/decision/records/2026-10-05-localization.md#A17,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: A suffix of the first language or of a language not in the list is part of the name
  Given "languages" of the configuration is "[ja, en]", and the location of the `IR` has "c.ja.md" and "c.fr.md" but not "c.ja.en.md" and "c.fr.en.md"
  When "kotowari check --format json" is run
  Then translation_missing `error` entries are raised with detail "docs/ir/c.ja.en.md" and "docs/ir/c.fr.en.md"

@id=EX-core-517 @about=REQ-core-336 @source=docs/decision/records/2026-10-05-localization.md#A11,docs/decision/records/2026-10-05-localization.md#A38,docs/decision/records/2026-10-05-localization.md#A27
Scenario: A decision record is not made into a pair
  Given "languages" of the configuration is "[ja, en]", and "docs/decision/records/r.md" exists but "r.en.md" does not
  When "kotowari check --format json" is run
  Then no translation_missing `finding` is raised on "docs/decision/records/r.md"

@id=EX-core-518 @about=REQ-core-339,REQ-core-340,REQ-core-341 @source=docs/decision/records/2026-10-05-localization.md#A18,docs/decision/records/2026-10-05-localization.md#A28,docs/decision/records/2026-10-05-localization.md#A27
Scenario: When the recorded hash equals the current file nothing is raised
  Given "languages" of the configuration is "[ja, en]", and the contents of both "guides/a.md" and "guides/a.en.md" are the two bytes "a" and a newline
  And "guides/a.i18n.yaml" has "78981922613b2afb6025042ff6bd878ac1994e85" for both "a.md" and "a.en.md"
  When "kotowari check --format json" is run
  Then no translation_stale `finding` is raised on "guides/a.md" or "guides/a.en.md"

@id=EX-core-519 @about=REQ-core-341 @source=docs/decision/records/2026-10-05-localization.md#A18,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: Editing only one side makes a stale side error
  Given the files of EX-core-518 exist, and the contents of "guides/a.md" are emptied
  When "kotowari check --format json" is run
  Then a translation_stale `error` is raised with "path" "guides/a.md" and detail "78981922613b2afb6025042ff6bd878ac1994e85 e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"

@id=EX-core-520 @about=REQ-core-339 @source=docs/decision/records/2026-10-05-localization.md#A28,docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: Record keys that differ from the side names are an error
  Given "languages" of the configuration is "[ja, en]", "guides/a.md" and "guides/a.en.md" exist, and the keys of "guides/a.i18n.yaml" are "a.md" and "a.zh.md"
  When "kotowari check --format json" is run
  Then a translation_record_invalid `error` is raised with "path" "guides/a.i18n.yaml" and detail "keys", and no translation_stale `finding` is raised

@id=EX-core-521 @about=REQ-core-339 @source=docs/decision/records/2026-10-05-localization.md#A27,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: A missing record is an error
  Given "languages" of the configuration is "[ja, en]", "guides/a.md" and "guides/a.en.md" exist, and "guides/a.i18n.yaml" does not
  When "kotowari check --format json" is run
  Then a translation_record_invalid `error` is raised with "path" "guides/a.i18n.yaml" and detail "missing"

@id=EX-core-522 @about=REQ-core-342 @source=docs/decision/records/2026-10-05-localization.md#A22,docs/decision/records/2026-10-05-localization.md#A24
Scenario: Requirements of the English side are not counted, and terms are looked up in the English glossary
  Given "languages" of the configuration is "[ja, en]", "a.md" and "a.en.md" of the `IR` have the same requirement "REQ-001", and a statement of "a.en.md" writes the term "term" enclosed in backquotes
  And "CONTEXT.en.md" has the term "term", and "CONTEXT.md" does not
  When "kotowari check --format json" is run
  Then no duplicate_id or unknown_term `finding` is raised, and "REQ-001" in the "items" of "kotowari list" is only one entry, with "path" "docs/ir/a.md"

@id=EX-core-523 @about=REQ-core-342 @source=docs/decision/records/2026-10-05-localization.md#A22
Scenario: For a term of the English side the Japanese glossary is not enough
  Given the same documents as EX-core-522 exist, and the term "term" is in "CONTEXT.md" but not in "CONTEXT.en.md"
  When "kotowari check --format json" is run
  Then an unknown_term `error` is raised on "docs/ir/a.en.md"

@id=EX-core-524 @about=REQ-core-343 @source=docs/decision/records/2026-10-05-localization.md#A22
Scenario: A stale guide mark is raised on every side
  Given "languages" of the configuration is "[ja, en]", and "guides/a.md" and "guides/a.en.md" have the same stale `guide mark`
  When "kotowari check --format json" is run
  Then one guide_stale `notice` is raised on each of "guides/a.md" and "guides/a.en.md"

@id=EX-core-525 @about=REQ-core-343 @source=docs/decision/records/2026-10-05-localization.md#A22,docs/decision/records/2026-10-05-localization.md#A31
Scenario: The English side of an overview is not counted as the name of a separate page
  Given "languages" of the configuration is "[ja, en]", the `overview data` "a.md" and "a.en.md" exist, and the only name item of the `table of contents` is "a"
  When "kotowari check --format json" is run
  Then no overview_name_conflict, overview_toc_page_missing or overview_ir_shared `finding` is raised
```
