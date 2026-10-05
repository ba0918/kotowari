# Listing and navigation

English | [日本語](navigation.ja.md)

This covers the status counts added to the listing `page` and how each `contents group` is drawn, how to move from the `page` of a `document` to other places in the `table of contents`, and how to move from the `outline` to a `section` within the `page` of a `document`. Even when the `table of contents` disagrees with the `document` entries, view draws it in a fixed way without checking.

## Requirements

### REQ-view-016: Status counts of a document

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A2, docs/decision/records/2026-10-05-overview-index.md#A13, docs/decision/records/2026-10-05-overview-index.md#A24, docs/decision/records/2026-10-05-overview-index.md#A35, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A3, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#A8
- verification: unit

view always adds to the entry of a `document` on the listing `page` the number of `section` entries marked stale, as the "stale_sections" text of the `UI text`; the number of entries of status `part` entries whose tag is "open", as "open_items"; and the number whose tag is "planned", as "planned_items" — each as the text with its "{n}" replaced by that number, and each only when the number is not 0. The tags counted are those in every status `part` among the opening `part` entries that follow the lead of that `document` and in the `block` entries of its `section` entries.

### REQ-view-017: Counts of a contents group

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A13, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A20, docs/decision/records/2026-10-05-overview-index.md#A24, docs/decision/records/2026-10-05-overview-index.md#A35, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A3
- verification: unit

view always adds to the heading of each `contents group` on the listing `page` (including the `table of contents` itself) the number of `document` entries drawn under that `contents group` at any depth of nesting, as the "pages" text of the `UI text` with its "{n}" replaced by that number; and adds the total number of stale `section` entries of those `document` entries as "stale_sections", and the total number of entries whose tag is "open" as "open_items", each as the text with its "{n}" replaced by that number and only when the total is not 0.

### REQ-view-018: Folding a contents group

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#D1, docs/decision/records/2026-10-05-overview-index.md#A31
- verification: unit

view always draws every `contents group` in the `table of contents` of the listing `page` (excluding the `table of contents` itself) in the open state, so that selecting its heading folds its contents. It does not remember the folded state, and puts no script on any `page` that view returns.

### REQ-view-019: Position in the table of contents

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A14, docs/decision/records/2026-10-05-overview-index.md#A18, docs/decision/records/2026-10-05-overview-index.md#A25, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#D1
- verification: unit

view always draws, above the title of the `page` of a `document` whose name is in the `table of contents`, the titles of the `contents group` entries traversed from the `table of contents` down to the entry of that name, from the outermost in order, and links each to the place of that `contents group` on the listing `page`. For a name that appears two or more times in the `table of contents`, it traverses down to the first entry found by a depth-first walk of the `table of contents` in written order. On the `page` of a `document` whose name is not in the `table of contents`, it draws only a link to the listing `page`, with the "index_link" text of the `UI text`.

### REQ-view-020: Links to documents in the same contents group

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A14, docs/decision/records/2026-10-05-overview-index.md#A25, docs/decision/records/2026-10-05-overview-index.md#A30, docs/decision/records/2026-10-05-overview-index.md#A36, docs/decision/records/2026-10-05-overview-index.md#A6, docs/decision/records/2026-10-02-whole-picture.md#A26
- verification: unit

view always draws, after the last `section` of the `page` of a `document` whose name is in the `table of contents`, the name entries directly under the `contents group` to which the entry traversed in REQ-view-019 directly belongs, excluding those with the same name as that `document`, in the order written in the `table of contents`, each by the title of its `document`, and links each to the `page` of that `document` by relative path. Entries inside a nested `contents group`, and names with no `document` in the `render input`, are not drawn.

### REQ-view-021: Input that disagrees with the table of contents

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A18, docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-05-overview-index.md#A25, docs/decision/records/2026-10-05-overview-index.md#A35
- verification: unit

view always leaves a name that is in the `table of contents` but has no `document` in the `render input` undrawn on the listing `page` and out of the counts. For a `document` whose name is not in the `table of contents`, it returns its `page` but does not draw it on the listing `page`. A name that appears two or more times in the `table of contents` is drawn and counted each time it appears. view does not report any of these as errors.

### REQ-view-022: Outline of sections

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A1, docs/decision/records/2026-10-05-overview-page-reading.md#A2, docs/decision/records/2026-10-05-overview-page-reading.md#A3, docs/decision/records/2026-10-05-overview-page-reading.md#A4, docs/decision/records/2026-10-05-overview-page-reading.md#A5, docs/decision/records/2026-10-05-overview-page-reading.md#A6, docs/decision/records/2026-10-05-overview-page-reading.md#A7, docs/decision/records/2026-10-05-overview-page-reading.md#A14, docs/decision/records/2026-10-05-overview-page-reading.md#A17, docs/decision/records/2026-10-05-overview-page-reading.md#D1, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#D1
- verification: unit

view always draws an `outline` on the `page` of a `document` that has one or more `section` entries, listing the heading text of each `section` in the order of the `section` entries, and making each a link to that `section` within that `page`. To the entry of a `section` marked stale, it adds after the heading text the "outline_stale" of the `UI text`, as a mark with the same meaning as the mark of REQ-view-009. The `outline` does not include entries for the lead and the opening `part` entries that follow the lead, headings of "### " or deeper inside a `section`, or links to other `page` entries. The link targets of sections do not overlap within one `page`; even two `section` entries with the same heading get different targets. No `outline` is drawn on the listing `page` or on the `page` of a `document` with no `section`.

### REQ-view-023: Placement of the outline

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A12, docs/decision/records/2026-10-05-overview-page-reading.md#A13, docs/decision/records/2026-10-05-overview-page-reading.md#A15, docs/decision/records/2026-10-05-overview-page-reading.md#D2
- verification: review
- how_to_verify: Render the `page` of a `document` with plenty of `section` entries, open it in a browser, and have a person confirm that at a wide width the `outline` is to the left of the body and stays visible while the body scrolls, and that at a narrow width the `outline` is below the title and above the lead and does not follow the scroll

view always draws the `outline`, on a wide screen, to the left of the body so that it stays visible while the body scrolls, and on a narrow screen, below the title and above the lead, without following the scroll.

## Examples

```gherkin
@id=EX-view-011 @about=REQ-view-016 @source=docs/decision/records/2026-10-05-overview-index.md#A13,docs/decision/records/2026-10-05-localization.md#A5,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#A8,docs/decision/records/2026-10-05-localization.md#A3
Scenario: Only the status counts that are not 0 are added to the card
  Given the "stale_sections" of the UI text is "見直していない節 {n}", "open_items" is "未決 {n}", and "planned_items" is "予定 {n}"
  And there is a document "a" with two stale sections, one entry with the tag "open" in a status part, and no entry with the tag "planned"
  When it is rendered with view
  Then the entry of "a" in "index.html" has "見直していない節 2" and "未決 1", and has no "予定" text

@id=EX-view-012 @about=REQ-view-017 @source=docs/decision/records/2026-10-05-overview-index.md#A13,docs/decision/records/2026-10-05-overview-index.md#A20,docs/decision/records/2026-10-05-overview-index.md#A35,docs/decision/records/2026-10-05-overview-index.md#A24,docs/decision/records/2026-10-05-localization.md#A5,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#A8,docs/decision/records/2026-10-05-localization.md#A3
Scenario: The counts of a contents group count all descendant documents
  Given the "pages" of the UI text is "{n} ページ", "open_items" is "未決 {n}", and "stale_sections" is "見直していない節 {n}"
  And the entries of the contents group "テスト" are "a" and the contents group "変異テスト", which has "b" and "c"; "b" has two entries with the tag "open"; and no document has a stale section
  When it is rendered with view
  Then the heading of "テスト" has "3 ページ" and "未決 2", the heading of "変異テスト" has "2 ページ" and "未決 2", and neither has the text "見直していない節"

@id=EX-view-013 @about=REQ-view-018 @source=docs/decision/records/2026-10-05-overview-index.md#D1
Scenario: Contents groups are drawn open and use no script
  Given there is a table of contents with two levels of nested contents groups
  When it is rendered with view
  Then every contents group in "index.html" is drawn as an element in the open state whose contents fold when its heading is selected, and no page has a script element

@id=EX-view-014 @about=REQ-view-019,REQ-view-020 @source=docs/decision/records/2026-10-05-overview-index.md#A14,docs/decision/records/2026-10-05-overview-index.md#A25,docs/decision/records/2026-10-05-overview-index.md#A30,docs/decision/records/2026-10-05-overview-index.md#A36,docs/decision/records/2026-10-05-overview-index.md#A15
Scenario: The page of a document shows its position in the table of contents and the documents of the same group
  Given the title of the table of contents is "kotowari", its entry is the contents group "テスト" (entries "a", "z", "b"), and the only documents are "a" and "b"
  When it is rendered with view
  Then above the title of "a.html" are "kotowari" and "テスト" in this order, each linking to the place of that contents group in "index.html"
  And after the last section of "a.html" there is a link to "b.html" with the title of "b", and there are no links to "z" or to "a" itself

@id=EX-view-015 @about=REQ-view-021,REQ-view-019 @source=docs/decision/records/2026-10-05-overview-index.md#A18,docs/decision/records/2026-10-05-overview-index.md#A25,docs/decision/records/2026-10-05-overview-index.md#A35,docs/decision/records/2026-10-05-overview-index.md#A24,docs/decision/records/2026-10-05-overview-index.md#A13,docs/decision/records/2026-10-05-overview-index.md#A36,docs/decision/records/2026-10-05-localization.md#A5,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#A8
Scenario: Input that disagrees with the table of contents is drawn without being an error
  Given the "pages" of the UI text is "{n} ページ", the entries of the table of contents are "a", "z", "a" in this order, and the documents are "a" and "c"
  When it is rendered with view
  Then the returned pages include "a.html" and "c.html"; "index.html" has the title of "a" twice and neither the title of "c" nor "z"; and the heading of the table of contents has "2 ページ"
  And above the title of "c.html" there is only a link to "index.html"

@id=EX-view-016 @about=REQ-view-022 @source=docs/decision/records/2026-10-05-overview-page-reading.md#A1,docs/decision/records/2026-10-05-overview-page-reading.md#A4,docs/decision/records/2026-10-05-overview-page-reading.md#A2,docs/decision/records/2026-10-05-overview-page-reading.md#A3,docs/decision/records/2026-10-05-overview-page-reading.md#A5,docs/decision/records/2026-10-05-overview-page-reading.md#A14,docs/decision/records/2026-10-05-overview-page-reading.md#A17,docs/decision/records/2026-10-05-overview-page-reading.md#D1
Scenario: The outline lists the sections in order and marks stale sections
  Given document "a" has, in this order, a non-stale section "読む", a section "書く" marked stale, and a non-stale section "読む", and section "書く" contains a "### 細部" heading
  When it is rendered with view
  Then the outline of "a.html" has "読む", "書く", "読む" in this order, and only after the text "書く" is there the mark of a section not yet reviewed
  And the three entries link to mutually different places in "a.html", each place being that section, and the outline has neither "細部" nor the conclusion of the lead

@id=EX-view-017 @about=REQ-view-022 @source=docs/decision/records/2026-10-05-overview-page-reading.md#A1,docs/decision/records/2026-10-05-overview-page-reading.md#A4
Scenario: A document with no sections and the listing have no outline
  Given there are a document "a" with no sections and a document "b" with one section
  When it is rendered with view
  Then "b.html" has an outline, and "a.html" and "index.html" have no outline
```
