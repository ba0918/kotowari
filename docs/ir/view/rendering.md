# Rendering

English | [日本語](rendering.ja.md)

This covers view from receiving the `render input` to returning the sequence of `page` entries. The kinds of `part` and their schemas are covered by parts.md, and the counts added to the listing `page` and moving between `page` entries by navigation.md. view knows neither the kotowari IR nor the decision records.

## Requirements

### REQ-view-001: Render input

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A42, docs/decision/records/2026-10-02-whole-picture.md#A68, docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A81, docs/decision/records/2026-10-02-whole-picture.md#A82, docs/decision/records/2026-10-04-overview-on-public-api.md#A10, docs/decision/records/2026-10-05-overview-index.md#A7, docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A33, docs/decision/records/2026-10-05-localization.md#A9, docs/decision/records/2026-10-05-localization.md#A34, docs/decision/records/2026-10-05-localization.md#A20, docs/decision/records/2026-10-05-localization.md#A32
- verification: unit

view always receives, as the `render input`, a sequence of `document` entries, the `reference table`, the `table of contents`, a language tag, the `UI text` and the `other languages`. A `document` has a name, a title, the opening lead `part`, the sequence of opening `part` entries that follow the lead, and the sequence of `section` entries; a `section` has the heading text, a true-or-false value telling whether it is stale, and the sequence of `block` entries. An entry of the `reference table` has the string of the `reference`, a display name, an optional body, and a status (one of "current", "superseded" and "deferred"). The `table of contents` is one `contents group`; a `contents group` has a title, an optional one-line description, and a sequence of entries, each of which is either the name of a `document` or a `contents group`.

### REQ-view-002: Return the sequence of pages

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A26, docs/decision/records/2026-10-02-whole-picture.md#A56, docs/decision/records/2026-10-05-overview-index.md#A18
- verification: unit

view always returns, in one rendering, the listing `page` "index.html", a `page` "<name>.html" per `document`, and the shared style `page` "style.css", and returns no other `page`. The per-`document` `page` entries and the listing `page` refer to "style.css" by relative path and link to each other by relative path. However, links from the listing `page` to the `page` of a `document` are limited to `document` entries whose name is in the `table of contents` (REQ-view-021).

### REQ-view-003: Write no files

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A43, docs/decision/records/2026-10-02-whole-picture.md#A45, docs/decision/records/2026-10-02-whole-picture.md#A47
- verification: unit

view must not read or write files, connect to a network, or check the `render input` and report errors.

### REQ-view-004: The same pages from the same input

- kind: invariant
- source: docs/decision/records/2026-10-02-whole-picture.md#A10, docs/decision/records/2026-10-02-whole-picture.md#A29
- verification: property

view always returns, from the same `render input`, a sequence of `page` entries whose names and content bytes are all the same. It puts no time, random numbers or environment variables into a `page`.

### REQ-view-005: The listing page

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A42, docs/decision/records/2026-10-05-overview-index.md#A2, docs/decision/records/2026-10-05-overview-index.md#A4, docs/decision/records/2026-10-05-overview-index.md#A6, docs/decision/records/2026-10-05-overview-index.md#A8, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A26
- verification: unit

view always draws on the listing `page` the title of the `table of contents` as a heading, its description below it if the `table of contents` has one, and below that the entries of the `table of contents` in written order. If an entry is a `contents group`, it uses its title and description as a heading and draws the entries of that `contents group` nested inside it in written order. If an entry is the name of a `document`, it draws the title of that `document` and the conclusion of its opening lead, and links to the `page` of that `document` by relative path.

### REQ-view-006: The opening conclusion

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A38, docs/decision/records/2026-10-04-overview-on-public-api.md#A10, docs/decision/records/2026-10-05-overview-page-reading.md#A17, docs/decision/records/2026-10-05-overview-page-reading.md#A19
- verification: unit

view always draws, after the title of the `page` of a `document`, the `outline` when REQ-view-022 draws one, then the opening lead `part`, then the opening `part` entries that follow the lead in sequence order, and then the `section` entries in sequence order.

### REQ-view-007: Markdown text

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A36, docs/decision/records/2026-10-02-whole-picture.md#A42, docs/decision/records/2026-10-02-whole-picture.md#A70, docs/decision/records/2026-10-02-whole-picture.md#A82
- verification: unit

view always turns a `block` that is a chunk of Markdown text into HTML as CommonMark with GFM tables. Raw HTML in the text is not interpreted but output as text, and HTML comments are not output.

### REQ-view-008: Opening and showing a reference

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A67, docs/decision/records/2026-10-02-whole-picture.md#A31, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A34, docs/decision/records/2026-10-05-localization.md#D1
- verification: unit

view always draws a `reference` by its display name in the `reference table`, and when it is selected, opens and shows the body of that `reference` without leaving the page. A `reference` whose status is "superseded" gets the "superseded" of the `UI text` as the superseded mark, and a `reference` whose status is "deferred" gets "deferred" as the deferred mark. A `reference` with no body is drawn as in REQ-view-030. view makes no `reference` a link outside the page.

### REQ-view-009: Mark of a stale section

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A39, docs/decision/records/2026-10-02-whole-picture.md#A68, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#D1
- verification: unit

view always draws, near the heading of a `section` marked stale, the "stale_mark" of the `UI text` as a mark showing that the `section` has not yet been reviewed since the IR changed.

### REQ-view-010: Load nothing from outside

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A37, docs/decision/records/2026-10-02-whole-picture.md#A16, docs/decision/records/2026-10-02-whole-picture.md#A82
- verification: unit

view must not make a `page` load fonts, scripts, styles or images from outside the `page` entries view returns.

## Examples

```gherkin
@id=EX-view-001 @about=REQ-view-002 @source=docs/decision/records/2026-10-02-whole-picture.md#A24,docs/decision/records/2026-10-02-whole-picture.md#A56,docs/decision/records/2026-10-02-whole-picture.md#A42,docs/decision/records/2026-10-02-whole-picture.md#A26,docs/decision/records/2026-10-02-whole-picture.md#A82
Scenario: Two documents return four pages
  Given a render input with two documents named "changes" and "guides"
  When it is rendered with view
  Then the names of the returned pages are only the four "changes.html", "guides.html", "index.html" and "style.css"

@id=EX-view-002 @about=REQ-view-004 @source=docs/decision/records/2026-10-02-whole-picture.md#A10,docs/decision/records/2026-10-02-whole-picture.md#A29
Scenario: Rendering the same input twice gives the same bytes
  Given any render input
  When it is rendered twice with view
  Then the names and content bytes of the pages of the two results are all equal

@id=EX-view-003 @about=REQ-view-007 @source=docs/decision/records/2026-10-02-whole-picture.md#A70
Scenario: Raw HTML in the text is output as text
  Given the Markdown text of a section has a line "<script>x</script>" and a line "<!-- @kotowari[REQ-core-001:00000000] -->"
  When it is rendered with view
  Then the page of that document has no "<script>" element, has the text "&lt;script&gt;", and does not have the text "@kotowari["

@id=EX-view-004 @about=REQ-view-008 @source=docs/decision/records/2026-10-02-whole-picture.md#A65,docs/decision/records/2026-10-02-whole-picture.md#A67
Scenario: A superseded reference is drawn with its body and mark and does not link outside
  Given the reference table has one entry where the reference "docs/x.md#A1" has the display name "x A1", the body "古い決定" and the status "superseded", and there is a part that has it in refs
  When it is rendered with view
  Then the page has the display name "x A1", the superseded mark, and the body "古い決定" that opens when selected, and there is no href to "docs/x.md"

@id=EX-view-005 @about=REQ-view-009 @source=docs/decision/records/2026-10-02-whole-picture.md#A39,docs/decision/records/2026-10-02-whole-picture.md#A68
Scenario: Only a stale section gets the mark
  Given a document with a section "A" marked stale and a non-stale section "B"
  When it is rendered with view
  Then only near the heading of section "A" is there the mark of a section not yet reviewed

@id=EX-view-006 @about=REQ-view-010,REQ-view-003 @source=docs/decision/records/2026-10-02-whole-picture.md#A37,docs/decision/records/2026-10-02-whole-picture.md#A22,docs/decision/records/2026-10-02-whole-picture.md#A82,docs/decision/records/2026-10-02-whole-picture.md#A65,docs/decision/records/2026-10-02-whole-picture.md#A73
Scenario: Pages load nothing from outside
  Given a render input with a document that uses all eight kinds of part
  When it is rendered with view
  Then no page has a src, href, "@import" or "url(" reference starting with "http://" or "https://"

@id=EX-view-010 @about=REQ-view-005 @source=docs/decision/records/2026-10-05-overview-index.md#A4,docs/decision/records/2026-10-05-overview-index.md#A6,docs/decision/records/2026-10-05-overview-index.md#A15
Scenario: The listing is drawn in the order and nesting written in the table of contents
  Given there are two documents named "a" and "b", the title of the table of contents is "kotowari", and its entries are, in order, a contents group with the title "テスト" and the description "テストとの対応" (entry "b") and "a"
  When it is rendered with view
  Then the heading of "index.html" is "kotowari"; inside the contents group "テスト" are the title of "b" and the conclusion of its lead; and after that are the title of "a" and the conclusion of its lead
  And the title of "b" links relatively to "b.html", and the title of "a" to "a.html"
```
