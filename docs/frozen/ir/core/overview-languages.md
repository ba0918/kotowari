# Overviews per language and UI text

English | [日本語](overview-languages.ja.md)

Covers writing the `overview` for each language of the `language list`, the `reference table` per language, the language and `UI text` passed to the rendering engine, and deciding the `UI text` from "labels" in the configuration, with its errors.

## Requirements

### REQ-core-353: Writing per language

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A20, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A22
- verification: unit

kotowari always, on "kotowari overview build" and "kotowari overview serve", draws, for each language of the `language list`, the `overview data` and the `table of contents` of that language's `side` with the rendering engine, writes the pages of the `first language` with the names of REQ-core-293, and writes the pages of the other languages with the same names under ".kotowari/cache/overview/<tag>/". The stale sections of a page in a language are derived, as REQ-core-292 says, from each `guide mark` of the `overview data` of that language's `side`. When the `language list` has one language, it writes only the pages of that language, with the names of REQ-core-293.

### REQ-core-354: The reference table per language

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A34, docs/decision/records/2026-10-05-localization.md#A38, docs/decision/records/2026-10-05-localization.md#D2
- verification: unit

kotowari always, in the `reference table` of a page in a language, takes the body of a reference pointing at an `item` or `scenario` of the `IR` or an `item` of the `flag record`, as TBL-core-039 says, from the `item` or `scenario` with the same `ID` in that language's `side` of the `pair` of the `IR` that has that `ID` (that this `side` exists and has the same `ID` is ensured by REQ-core-294). A reference pointing at a `decision record`, an ADR or Markdown that is not a decision record has the body of TBL-core-039 on the pages of the `first language`, and has no body on the pages of the other languages.

### REQ-core-355: Passing the language to the rendering engine

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A9, docs/decision/records/2026-10-05-localization.md#A20, docs/decision/records/2026-10-05-localization.md#A32, docs/decision/records/2026-10-05-localization.md#D1
- verification: unit

kotowari always, when drawing a page in a language, passes to the rendering engine that language tag, the `UI text` of that language (REQ-core-351), and, for each other language, its name (the "language_name" of that language's `UI text`) and the relative path to the location of that language's page (in the order of the `language list`).

### REQ-core-351: Deciding the UI text

- kind: algorithm
- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A7, docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D1
- definition: TBL-core-046
- verification: unit

### REQ-core-352: UI text errors

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

When "labels" in the `configuration file` has a key that is a language tag not in the `language list`, when "labels.<tag>" has a key not in TBL-core-046, when "labels.<tag>" of a language of the `language list` other than "en" is missing or lacks any key of TBL-core-046, or when the value of a key of TBL-core-046 that takes a number does not contain exactly one "{n}", kotowari makes a `stop` with a configuration error as the reason.

## Decision tables

### TBL-core-046: The keys of the UI text and the English text

- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A7, docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A32, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3

The `UI text` of the language "en" is the English text of this table with the values of the keys written in "labels.en" overriding it. The `UI text` of a language other than "en" is the value of "labels.<tag>" itself. "{n}" is replaced by a number when drawn.

| Key | Where it is used | Takes a number | English text |
|---|---|---|---|
| language_name | the name of that language written in the `switcher line` and in the links of the `overview` to other languages | no | English |
| index_link | the link to the index from a page of an `overview` whose name is not in the `table of contents` | no | Overview |
| pages | the number of `overview` pages of a `contents group` on the index | yes | {n} pages |
| stale_sections | the number of stale sections on the index | yes | {n} sections to review |
| open_items | the number of items with the label open on the index | yes | {n} open |
| planned_items | the number of items with the label planned on the index | yes | {n} planned |
| stale_mark | the marker near the heading of a stale section | no | Not reviewed since the IR changed |
| outline_stale | the marker of a stale section in the outline | no | not reviewed |
| superseded | the marker of a superseded reference | no | (superseded) |
| deferred | the marker of a deferred reference | no | (deferred) |
| compare_before | the heading of the before column of the compare `part` | no | Before |
| compare_after | the heading of the after column of the compare `part` | no | After |
| compare_why | the heading of the reason column of the compare `part` | no | Why |
| state_decided | the display of the status label decided | no | Decided |
| state_planned | the display of the status label planned | no | Planned |
| state_open | the display of the status label open | no | Open |
| state_dropped | the display of the status label dropped | no | Dropped |

## Examples

```gherkin
@id=EX-core-540 @about=REQ-core-353,REQ-core-355 @source=docs/decision/records/2026-10-05-localization.md#A20,docs/decision/records/2026-10-05-localization.md#A32,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: English pages are written under en and link to each other
  Given "languages" in the configuration is "[ja, en]", "labels.ja.language_name" is "日本語", and there are the `overview data` files "a.md" and "a.en.md" and the `pair` of its `table of contents`
  When "kotowari overview build" is run
  Then ".kotowari/cache/overview/a.html" and ".kotowari/cache/overview/en/a.html" are written; the former has an "English" link to "en/a.html", and the latter has a "日本語" link to "../a.html"

@id=EX-core-541 @about=REQ-core-353 @source=docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#A20
Scenario: With one language no subdirectory is made
  Given "languages" in the configuration is "[ja]", and "labels.ja" has all the keys of TBL-core-046
  When "kotowari overview build" is run
  Then there is no "ja" directory under ".kotowari/cache/overview/", and the index is "index.html"

@id=EX-core-542 @about=REQ-core-354 @source=docs/decision/records/2026-10-05-localization.md#A34
Scenario: On an English page the IR body is taken from the English side, and a decision record has no body
  Given "languages" in the configuration is "[ja, en]", the statement of "REQ-001" is "文。" in "a.md" and "Text." in "a.en.md", and a `part` of the `overview data` refers to "REQ-001" and "docs/decision/records/r.md#A1"
  When "kotowari overview build" is run
  Then on the page under "en/" the body of "REQ-001" is "Text." and "r A1" has no body, and on the page of the `first language` the body of "REQ-001" is "文。" and "r A1" has the text of the decision as its body
@id=EX-core-536 @about=REQ-core-351 @source=docs/decision/records/2026-10-05-localization.md#A7,docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1
Scenario: English layers the overrides on top of kotowari's text
  Given there is no "languages" in the configuration, and "labels.en.pages" is "{n} docs"
  When "kotowari overview build" is run
  Then the count of a `contents group` on the index page is in the form "<number> docs", and the rest of the `UI text` is the English text of TBL-core-046

@id=EX-core-537 @about=REQ-core-352 @source=docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1
Scenario: It stops when a key of a non-English language is missing
  Given "languages" in the configuration is "[ja]", and "labels.ja" has no "stale_mark" key
  When "kotowari check" is run
  Then the exit code is 2, and it makes a `stop` with a configuration error

@id=EX-core-538 @about=REQ-core-352 @source=docs/decision/records/2026-10-05-localization.md#A8,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1
Scenario: It stops when a text that takes a number has no "{n}"
  Given there is no "languages" in the configuration, and "labels.en.pages" is "pages"
  When "kotowari check" is run
  Then the exit code is 2, and it makes a `stop` with a configuration error

@id=EX-core-539 @about=REQ-core-352 @source=docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D3
Scenario: It stops when text is written for a language not in the list
  Given there is no "languages" in the configuration, and "labels.ja" exists
  When "kotowari check" is run
  Then the exit code is 2, and it makes a `stop` with a configuration error
```
