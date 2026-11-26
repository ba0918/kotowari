# Languages

English | [日本語](languages.ja.md)

This covers view indicating the language of a page, view taking the text it writes from the `UI text`, links to the same `page` in the `other languages`, and a `reference` with no body. view holds no text of any language inside itself, and is drawn once per language.

## Requirements

### REQ-view-026: Language of the page

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A9
- verification: unit

view always sets the lang attribute of the "html" element of an HTML `page` to the language tag of the `render input`.

### REQ-view-027: No text held inside

- kind: prohibition
- source: docs/decision/records/2026-10-05-localization.md#A9
- verification: unit

view must not take, from anywhere other than the `UI text`, any text it writes into a `page` that does not come from the `document`, the `reference table`, the `table of contents` or the `other languages`.

### REQ-view-028: Text that takes a number

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A8
- verification: unit

When drawing the `UI text` of a key that takes a number in TBL-view-002, view always replaces the "{n}" in it with that number in decimal.

### REQ-view-029: Links to other languages

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A20, docs/decision/records/2026-10-05-localization.md#A32
- verification: unit

view always draws on every HTML `page`, for each entry of the `other languages` and in the order of the `other languages`, a link whose text is the name of that entry and whose target is the relative path to its place followed by the name of the same `page`. When the `other languages` is empty, it draws no link. It puts no script on any `page` and does not remember the chosen language.

### REQ-view-030: A reference with no body

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A34
- verification: unit

When an entry of the `reference table` has no body, view draws that `reference` with its display name only, so that selecting it opens nothing.

## Decision tables

### TBL-view-002: Keys of the UI text

- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A32, docs/decision/records/2026-10-05-localization.md#D1

| Key | Where it is used | Takes a number |
|---|---|---|
| language_name | Not used (the names of the `other languages` are decided outside view) | No |
| index_link | The link to the listing in REQ-view-019 | No |
| pages | The count in REQ-view-017 | Yes |
| stale_sections | The counts in REQ-view-016 and REQ-view-017 | Yes |
| open_items | The counts in REQ-view-016 and REQ-view-017 | Yes |
| planned_items | The count in REQ-view-016 | Yes |
| stale_mark | The mark in REQ-view-009 | No |
| outline_stale | The mark in REQ-view-022 | No |
| superseded | The superseded mark in REQ-view-008 | No |
| deferred | The deferred mark in REQ-view-008 | No |
| compare_before | The heading of the before column of the compare `part` | No |
| compare_after | The heading of the after column of the compare `part` | No |
| compare_why | The heading of the reason column of the compare `part` | No |
| state_decided | The status tag "decided" | No |
| state_planned | The status tag "planned" | No |
| state_open | The status tag "open" | No |
| state_dropped | The status tag "dropped" | No |

## Examples

```gherkin
@id=EX-view-018 @about=REQ-view-026,REQ-view-027 @source=docs/decision/records/2026-10-05-localization.md#A5,docs/decision/records/2026-10-05-localization.md#A9,docs/decision/records/2026-10-05-localization.md#D1
Scenario: The language and the text of the page follow the input
  Given the language tag is "fr", the "stale_mark" of the UI text is "Pas encore relu", and there is a document with a section marked stale
  When it is rendered with view
  Then the html element of every HTML page has the lang attribute "fr", and "Pas encore relu" is near the stale section

@id=EX-view-019 @about=REQ-view-027 @source=docs/decision/records/2026-10-05-localization.md#A9
Scenario: Changing the UI text changes all the text view writes
  Given there is a document that uses all eight kinds of part and has a stale section and superseded and deferred references, and every value of the UI text is "X" followed by the name of the key (with " {n}" at the end for keys that take a number)
  When it is rendered with view
  Then every text of the pages that does not come from the document, the reference table, the table of contents or the other languages is UI text starting with "X", or such text with its "{n}" replaced by a number

@id=EX-view-020 @about=REQ-view-028 @source=docs/decision/records/2026-10-05-localization.md#A8,docs/decision/records/2026-10-05-localization.md#D1
Scenario: A number goes into text with a fixed word order
  Given the "pages" of the UI text is "全{n}件", and there are three documents under a contents group
  When it is rendered with view
  Then the heading of that contents group has "全3件"

@id=EX-view-021 @about=REQ-view-029 @source=docs/decision/records/2026-10-05-localization.md#A20,docs/decision/records/2026-10-05-localization.md#A32
Scenario: Link to the same page in other languages
  Given the other languages is one entry with the name "English" and the place "en/", and there is a document "a"
  When it is rendered with view
  Then "a.html" has an "English" link to "en/a.html", "index.html" has an "English" link to "en/index.html", and no page has a script element

@id=EX-view-022 @about=REQ-view-029 @source=docs/decision/records/2026-10-05-localization.md#A20
Scenario: No other languages means no links
  Given the other languages is empty, and there is a document "a"
  When it is rendered with view
  Then "a.html" and "index.html" have no link to a page in another language

@id=EX-view-023 @about=REQ-view-030 @source=docs/decision/records/2026-10-05-localization.md#A34
Scenario: A reference with no body shows only its display name and does not open
  Given the reference table has one entry where the reference "docs/x.md#A1" has the display name "x A1", no body and the status "current", and there is a part that has it in refs
  When it is rendered with view
  Then the page has the display name "x A1", and there is no element that opens when selected
```
