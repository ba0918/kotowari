# Parts

English | [日本語](parts.ja.md)

This decides the kinds of parts view can draw, the publishing of the part schemas, the width of parts, and how flow parts are drawn. The source of truth for the shape of part contents is the part schemas, and view does not validate the shape.

## The kinds of parts are a fixed list of eight held by view

Status: implemented

### Problem

If overview documents are output as ordinary Markdown, they look poor and are a heavy burden to read. If clear output can be made from a fixed format, this problem is solved. So the form adopted is to choose parts per document from a fixed set of kinds, arrange them, and turn them into HTML with a fixed way of drawing. This direction was checked with a mock that built two documents with a throwaway renderer, and the judgment was that the content was good but the look needed adjusting.

The only reason on record for making the kinds a fixed list instead of having them created on the spot is "the recommendation is fine as the default; consider it when it becomes necessary".

The labels of status are useful only when the states can be told apart at a glance. If the words of the labels depended on the user's language, users who write in English would also have to write Japanese in the source data.

### Decision

The kinds of parts are a fixed list held by view. An LLM cannot create a new kind on the spot. A new kind is added as a change to the specification when it becomes necessary. Diagram parts such as flowcharts and sequence diagrams are considered when they become necessary.

The initial kinds are the following eight, with which the two documents of the mock could be written.

| Kind name | What it draws |
|---|---|
| lead | Conclusion and summary. [Placed at the top of a document](rendering.md#draw-a-document-page-in-the-order-title-outline-lead-opening-parts-sections) |
| flow | A flow diagram. Columns of boxes flow from left to right |
| steps | Stages |
| cards | Cards |
| status | A list of states. The labels are the four "decided", "planned", "open", "dropped" |
| compare | Before and after |
| decisions | A chain of decisions |
| quiz | Questions |

The labels of status are written in the source data with the words "decided", "planned", "open", "dropped", and drawn on the page in that language's text taken from the [UI strings](languages.md#view-holds-no-ui-strings-itself-and-receives-them-from-the-render-input). The four states are drawn with looks that can be told apart from each other.

### Confirmations

- C-view-parts-1 Each of the eight kinds of parts is drawn as described in the table's "What it draws"
- C-view-parts-2 The status part draws each of the four labels with that label's text from the UI strings, and even without the label text the four looks differ from each other

## The source of truth for the shape of parts is the part schemas that view embeds and publishes

Status: implemented

### Problem

Knowledge of the shape of part contents appears in two places: the side that validates the shape and the side that draws it. Writing it in two places makes them diverge. One source of truth has to be chosen and kept in line by tests.

Since view is an engine that draws its own input, it is odd for it not to know the shape of its own input. On the other hand, view has been decided to handle rendering only ([view does not know kotowari and receives everything it draws in the rendering input](rendering.md#view-does-not-know-kotowari-and-receives-everything-it-draws-in-the-rendering-input)), and holding validation of part contents as well would go beyond that scope.

### Decision

The source of truth for the shape of part contents is the part schemas, written in JSON Schema. Since an LLM writes both the schemas and the source data, the burden of reading and writing JSON Schema is not a problem.

The view crate holds the part schemas as the shape of its own input, embeds them in the crate, and hands them out through a public function that returns the schema for each kind name. view only publishes them and does not validate part contents. Validation is done by the kotowari side with the schemas received from view (a decision of the overview source data topic). view only draws from validated values.

That the schemas and view's rendering do not diverge is guarded by this repository's tests. For each kind, there is at least one example part that matches the schema, and the tests check that every example matches the schema and that view draws it without failing.

JSON Schema allows unknown keys by default. This hole is also closed by this repository's tests. They check that every part of the part schemas whose type is object declares "additionalProperties": false.

### Rejected options

- Write them in JSON Type Definition (using a crate or implementing it ourselves). The usable crates have not been updated for a long time.
- Put the part schemas in ".kotowari/schemas/overview/". view would then not know the shape of its own input.
- Give view validation of part contents. It goes beyond the scope decided for view, which is handling rendering only.
- view publishes types for the parts, and the kotowari side validates by loading into those types. The user judged that it is more natural for the rules of part contents to also be declarable as a schema, and that judging only that part in code is unreasonable.

### Confirmations

- C-view-parts-3 Passing each of the eight kind names to view's public function returns that part's schema in JSON Schema
- C-view-parts-4 If the part schemas contain even one part whose type is object and that does not declare "additionalProperties": false, this repository's tests fail
- C-view-parts-5 For each kind, there is at least one example part that matches the schema, every example matches the schema, and view draws it without failing

## The only layout setting is one that makes a part half width

Status: implemented

### Problem

If an LLM can specify position and even color, the result varies every time, and the point of making the look consistent through rendering weakens. The details will be adjusted during implementation.

### Decision

The only layout setting is one that makes a part half width. A part in the source data has "width": "half" written on it, and adjacent half parts in the same section are placed side by side. A part with no width setting is drawn at the full width of the section. The part schemas allow only "half" as the value of "width". Anything to add is decided during implementation.

### Confirmations

- C-view-parts-6 Every part schema allows only "half" as the value of "width"
- C-view-parts-7 Two "width": "half" parts placed in a row in the same section are drawn side by side, and a part with no width setting is drawn at the full width of the section

## Boxes in one column of a flow mean parallel; order is shown only by the flow from column to column

Status: implemented

### Problem

Whether boxes stacked vertically in one column of a flow meant an order or parallel items could not be told, and the meaning was not decided anywhere.

### Decision

Boxes stacked vertically in one column of a flow represent things at the same stage (parallel). Order is represented only by the left-to-right flow from column to column. So that this can be read, the boxes of one column are drawn inside one enclosure, and arrows are placed only between columns, never between boxes in the same column.

The concrete look of the column enclosures and arrows is left to the implementation. How it looks on screen cannot be checked by unit tests of the strings view returns, so a person checks it by looking at a drawn page.

### Confirmations

- C-view-parts-8 (by reading) Draw a flow part with two or more columns and two or more boxes stacked vertically in one column, and open it in a browser: the boxes of one column are inside one enclosure, and the arrows are only between columns, not between boxes in the same column

## On narrow screens, stack the columns of a flow from top to bottom

Status: implemented

### Problem

Because a flow draws its columns packed side by side, boxes overlap when the screen is not wide enough.

### Decision

On narrow screens, the columns of a flow are stacked from top to bottom, and the arrows between columns point downward. Boxes do not overlap.

The width at the boundary between wide and narrow screens and the shape of the arrows are left to the implementation. A look that changes with screen width cannot be checked by unit tests, so a person checks it by looking at a drawn page at a wide width and at a narrow width.

### Confirmations

- C-view-parts-9 (by reading) Draw a flow part with two or more columns and open it in a browser at a narrow width: the columns are stacked from top to bottom, the arrows between columns point downward, and boxes do not overlap
