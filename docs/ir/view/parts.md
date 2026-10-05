# Parts

English | [日本語](parts.ja.md)

This covers the kinds of `part` view can draw, publishing the `part schema`, and the width of parts. The authority on the shape of the contents of a `part` is the `part schema`; view does not check the shape.

## Requirements

### REQ-view-011: Kinds of part

- kind: algorithm
- source: docs/decision/records/2026-10-02-whole-picture.md#A13, docs/decision/records/2026-10-02-whole-picture.md#A36
- definition: TBL-view-001
- verification: unit

### REQ-view-012: Publishing the part schema

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A45, docs/decision/records/2026-10-02-whole-picture.md#A46, docs/decision/records/2026-10-02-whole-picture.md#A47, docs/decision/records/2026-10-02-whole-picture.md#A49, docs/decision/records/2026-10-02-whole-picture.md#A82
- verification: unit

view always has a public function that returns, for each kind name of TBL-view-001, the `part schema` of that kind (a JSON Schema string), and embeds the `part schema` inside view. For a name not in TBL-view-001 it returns nothing.

### REQ-view-013: The part schema is closed

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A46
- verification: unit

The tests of this repository always confirm that every subschema of type object inside every `part schema` declares "additionalProperties": false.

### REQ-view-014: A part that fits the schema can be drawn

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A45
- verification: unit

The tests of this repository always hold, for each kind of TBL-view-001, one or more examples of a `part` that fits the `part schema`, and confirm that every example fits the `part schema` and that view draws it without failing.

### REQ-view-015: Width of parts

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A12, docs/decision/records/2026-10-02-whole-picture.md#A54, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

view always pairs up, two at a time from the start, a consecutive run of `part` entries within the same `section` whose value has "width": "half", and places each pair side by side; the one left over without a pair, and every other `part`, are drawn at the full width of the `section`. Every `part schema` allows only "half" as the value of "width".

### REQ-view-024: Columns and parallelism in flow

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A8, docs/decision/records/2026-10-05-overview-page-reading.md#D2
- verification: review
- how_to_verify: Render a flow `part` that has two or more columns with two or more boxes stacked vertically in one column, open it in a browser, and have a person confirm that the boxes of one column are inside one enclosure, and that arrows appear only between columns and not between boxes of the same column

view always draws, for each column of a flow `part`, the boxes of that column inside one enclosure, and draws the arrows that show the left-to-right flow only between columns, not between boxes of the same column.

### REQ-view-025: flow on a narrow screen

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A9, docs/decision/records/2026-10-05-overview-page-reading.md#A15, docs/decision/records/2026-10-05-overview-page-reading.md#D2
- verification: review
- how_to_verify: Render a flow `part` that has two or more columns, open it in a browser, and have a person confirm that at a narrow width the columns are stacked from top to bottom, the arrows between columns point down, and the boxes do not overlap

view always, on a narrow screen, stacks the columns of a flow `part` from top to bottom, draws the arrows between columns pointing down, and does not overlap the boxes.

## Decision tables

### TBL-view-001: Kinds of part

- source: docs/decision/records/2026-10-02-whole-picture.md#A36, docs/decision/records/2026-10-02-whole-picture.md#A66, docs/decision/records/2026-10-02-whole-picture.md#A31, docs/decision/records/2026-10-02-whole-picture.md#A83, docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A38, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-04-overview-on-public-api.md#A13, docs/decision/records/2026-10-05-localization.md#A3, docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#D1

| Kind name | What it draws | Main fields |
|---|---|---|
| lead | The conclusion and the key points. Placed at the start of a `document` | conclusion, points |
| flow | A sequence of columns of boxes flowing from left to right. The boxes are laid out on a grid, and the position of text is not decided by hand | columns (a sequence of columns; a column is a sequence of boxes; a box has title, body, tone) |
| steps | A sequence of numbered stages | items (title, body, refs) |
| cards | A sequence of cards, each with a heading and a list of entries | cards (title, items, tone) |
| status | A sequence of status tags and sentences. The tags are "decided", "planned", "open" and "dropped", drawn with the texts "state_decided", "state_planned", "state_open" and "state_dropped" of the `UI text` respectively, and with appearances that tell the four apart | items (state, text, refs) |
| compare | A sequence of before, after and reason sets. The before is drawn struck through, and the column headings are the texts "compare_before", "compare_after" and "compare_why" of the `UI text` | items (before, after, why, refs) |
| decisions | A tree of root decisions and the decisions under them, tagged with who decided | roots (ref, text, by, children) |
| quiz | A sequence of questions and answers that open when selected | items (q, a, refs) |

## Examples

```gherkin
@id=EX-view-007 @about=REQ-view-012,REQ-view-011 @source=docs/decision/records/2026-10-02-whole-picture.md#A47,docs/decision/records/2026-10-02-whole-picture.md#A36,docs/decision/records/2026-10-02-whole-picture.md#A46,docs/decision/records/2026-10-02-whole-picture.md#A82
Scenario: Return the eight schemas and nothing for an unknown name
  When the function of view that returns the part schema is called with the eight names of TBL-view-001 and "chart"
  Then a JSON Schema string is returned for the eight names, and nothing is returned for "chart"

@id=EX-view-008 @about=REQ-view-015 @source=docs/decision/records/2026-10-02-whole-picture.md#A12,docs/decision/records/2026-10-02-whole-picture.md#A54
Scenario: Two consecutive half parts are placed side by side
  Given one section has two consecutive cards parts with "width": "half", followed by a status part with no width
  When it is rendered with view
  Then the two cards are drawn inside one side-by-side row, and the status is drawn outside it at the full width of the section

@id=EX-view-009 @about=REQ-view-013,REQ-view-014 @source=docs/decision/records/2026-10-02-whole-picture.md#A45,docs/decision/records/2026-10-02-whole-picture.md#A46
Scenario: Tests confirm the closedness of the schemas and the drawing of the examples
  Given one of the part schemas has an object subschema with no "additionalProperties"
  When the tests of this repository are run
  Then that test fails
```
