# Rendering

English | [日本語](rendering.ja.md)

This covers view from receiving the rendering input to returning the sequence of pages. The kinds of parts and their schemas are decided in [the Parts topic](parts.md), the counts attached to the index page and the movement between pages in [the Index and navigation topic](navigation.md), and the language of a page and references without a body in [the Languages topic](languages.md). view is a rendering-only engine that knows nothing of kotowari's specifications and decision records.

## view does not know kotowari and receives everything it draws in the rendering input

Status: implemented

### Problem

For an overview document, the renderer builds HTML from source data that an LLM wrote in a fixed format. If kotowari's work, such as resolving references to specifications and decision records, is mixed into the renderer, the rendering part can neither be cut out and verified on its own nor used elsewhere. Since the input is source data and the output is HTML, the user judged that the renderer can be made generic to some degree.

### Decision

view is a rendering engine in a crate separate from kotowari. Dependencies point one way, from kotowari to view, and view knows neither kotowari's specifications nor its decision records.

view takes everything it draws from the rendering input passed in a single rendering. The rendering input consists of the following.

- The sequence of documents. A document is a sequence of Markdown text blocks and parts in order. Splitting the source data into which span is a part is done by the side outside view that reads the source data; view receives what has been split. A document has a title, the opening lead, the opening parts that follow the lead, and a sequence of sections, and each section carries whether it is stale.
- The reference table. It maps a reference string to a display name, a body and a state. The kotowari side builds it by reading the specifications and decision records; view only looks up the table and draws.
- The table of contents. It decides the order and nesting of the index. view draws it exactly as the table of contents passed to it.
- The language tag and a complete set of UI strings. view holds no UI strings of its own. Only the kotowari side holds the English strings and the rule for when they are omitted.
- Other languages. A sequence of locations where the same page is drawn in other languages.

### Rejected options

- Make it a kotowari-only crate and resolve references directly inside the crate. Because rendering can be made generic with just an input and an output, it was separated from kotowari's work.

### Confirmations

- C-view-rendering-1 The sections and blocks of the documents, the reference table and the contents of the table of contents passed in the rendering input are each drawn on the pages
- C-view-rendering-2 (by reading) The view crate does not depend on the kotowari crate and has no code that reads kotowari's specifications or decision records

## view does not validate its input and uses neither files nor the network

Status: implemented

### Problem

If knowledge of the shape of part contents is held in two places, the schema and view, the two diverge. The user judged that giving view validation would go beyond the responsibility decided for it, which is handling rendering only.

### Decision

The source of truth for the shape of part contents is the schema (the schema is decided in the Parts topic under [The source of truth for the shape of parts is the part schemas that view embeds and publishes](parts.md#the-source-of-truth-for-the-shape-of-parts-is-the-part-schemas-that-view-embeds-and-publishes)). view only draws from validated values; it does not validate the rendering input and reports no errors. Even when a value that does not match the schema is passed, it draws without raising an error.

view does not read or write files and does not connect to the network; it only returns a sequence of page names and contents. Writing the pages out to files is done by the side that calls view.

### Rejected options

- view holds types for the parts and validates by loading into those types. It is more natural for custom definitions inside code blocks to also be declarable as a schema, and judging only that part in code is unreasonable. The two-layer form, where view publishes the types and the kotowari side validates by loading into them, was also not adopted because it did not feel right.

### Confirmations

- C-view-rendering-3 Even when a part's value does not match the schema, view draws the page without raising an error
- C-view-rendering-4 (by reading) view's code does not read or write files and does not connect to the network

## Return the index, one page per document, and the shared style as separate pages

Status: implemented

### Problem

In addition to one page per overview document, an index listing all documents is wanted. Putting everything into one file makes it hard to handle as the scale grows, and also makes it impossible to re-output only the files that changed. The look should be kept consistent in one place.

### Decision

In a single rendering, view returns the index page "index.html", one page per document "<document name>.html", and the shared style "style.css", and returns no other pages. The index and document pages refer to "style.css" and can be navigated between by links. A document whose name is not in the table of contents still gets a page but is not shown in the index (how input that conflicts with the table of contents is handled is decided in the Index and navigation topic under [Input that conflicts with the table of contents is not an error either and is drawn in a fixed form](navigation.md#input-that-disagrees-with-the-contents-is-drawn-in-a-fixed-way-not-treated-as-an-error)).

### Rejected options

- Put the index and all pages into one HTML and write it to standard output. It becomes hard to handle as the scale grows, and only the files that changed cannot be re-output.

### Confirmations

- C-view-rendering-5 From a rendering input with two documents, exactly four are returned: "index.html", the two document pages and "style.css"; with no documents, only "index.html" and "style.css" are returned
- C-view-rendering-6 The index and document pages refer to "style.css", and the index and document pages link to each other

## Return the same pages from the same rendering input

Status: implemented

### Problem

The overview is built not by an LLM writing HTML each time but by a fixed rendering from the source data. This keeps the look consistent every time and leaves stale-content detection and diff readability on the source data side. The side that writes out the pages redraws everything every time and rewrites only the files whose contents differ from the existing ones, because then it does not need to keep a record of rendering dependencies.

### Decision

From the same rendering input, view returns a sequence of pages whose names and content bytes are all the same.

### Rejected options

- Fix only a shared stylesheet and have an LLM write HTML each time. The look is not consistent from time to time.

### Confirmations

- C-view-rendering-7 For any rendering input, the names and content bytes of the pages from two renderings are all equal

## Draw the index on one page, in the order and nesting written in the table of contents

Status: implemented

### Problem

The index page is the entrance for finding the page one wants to read, and the index itself, as a map of the product, shows the groupings of features and their order. There is no upper limit on the number of pages, so a form is needed that does not get confusing even at several dozen or over a hundred.

### Decision

The index is one page and draws every item of the table of contents. No search is built; it is left to the browser's in-page search.

The heading of the index is the title of the outermost table-of-contents group. Below it, the items of the table of contents are drawn in the order written. If an item is a table-of-contents group, its title becomes a heading and the group's items are drawn nested inside it. The depth of nesting is not limited. If an item is a document name, the document's title and the conclusion of its opening lead are drawn, linking to that document's page.

### Rejected options

- Only bundle into one level. As groups increase, the same problem occurs in the sequence of headings of the groups one level up.
- Sort automatically by the character order of titles or names. Japanese titles have no means of being sorted by reading, so they look unordered. The order itself, such as the order of use or the order of the workflow, is information as a map.
- Split pages by table-of-contents group. To search, one would have to guess the group and open it, or build a search oneself.

### Confirmations

- C-view-rendering-8 The heading of the index is the title of the table of contents, below it the items of the table of contents are in the order written, and the items of a table-of-contents group are drawn nested inside that group's heading
- C-view-rendering-9 A document item in the index shows the document's title and the conclusion of its opening lead, and the title links to that document's page

## Draw a document page in the order title, outline, lead, opening parts, sections

Status: implemented

### Problem

The structure that puts the conclusion and summary at the top of the overview proved right on comparison. After the title and before the first section, parts can be placed following the lead, but there was nowhere to draw them, and parts that had passed validation were silently disappearing. The outline goes below the title and above the lead.

### Decision

On a document page, after the title, the outline when one is drawn (which documents it is drawn on and its contents are decided in the Index and navigation topic under [Draw an outline listing the section headings on a document page](navigation.md#document-pages-get-an-outline-listing-the-section-headings)), the opening lead, and the opening parts following the lead are drawn in sequence order, and then the sections in sequence order. The lead comes directly after the title only when no outline is drawn.

### Rejected options

- Allow only the lead before the first section and make any other part an error in the shape of the source data. That would narrow the approved shape of the source data.

### Confirmations

- C-view-rendering-10 On a document page, the title, the outline, the opening lead, the opening parts and the sections appear in this order, and the opening parts and the sections are drawn in the order of the sequence passed

## view turns Markdown text blocks into HTML

Status: implemented

### Problem

Ordinary Markdown (headings, text, lists, tables, code) can be written between parts. Turning text into HTML is the renderer's job, and reading the source data is a job outside the renderer.

### Decision

view turns the Markdown text blocks in a document into HTML. It can draw headings, text, lists, tables and code.

### Confirmations

- C-view-rendering-11 Headings, lists, tables, code and emphasis in a text block are each drawn as HTML elements

## Output HTML in a text block as text, and do not output HTML comments

Status: implemented

### Problem

HTML can be written directly in Markdown. If view outputs it as HTML, characters written in the source data become page elements, and scripts and elements that load things from outside the page can be put into the page from the source data. [The decision not to include scripts](navigation.md#no-page-that-view-returns-contains-a-script) and [the decision not to load from outside](#pages-load-no-fonts-styles-or-images-from-outside) would be broken by a single way of writing the source data.

HTML comments are annotations for the people and tools that write the source data (such as markers for guide sections), not something to show the page's reader.

### Decision

HTML written in a text block is not interpreted as HTML and is shown as the characters written. HTML comments are not output to the page. The text before and after a comment is drawn the same as when there is no comment.

### Confirmations

- C-view-rendering-17 HTML tags written in a text block (such as "<script>") do not become page elements and are shown as text
- C-view-rendering-18 HTML comments written in a text block do not appear on the page, and the text before and after a comment is drawn the same as when there is no comment

## Resolve Markdown reference links only with definitions in the same block

Status: implemented

### Problem

It had to be decided how to draw a reference link in a text block whose definition is in another block across a part. The Markdown conversion tool view uses has no way to pass definitions in from outside, so a new mechanism would have to be built to collect definitions from the whole document. There is also no evidence of users who write references across blocks.

### Decision

A reference link in a block is resolved only with the reference definitions in the same block. It is not resolved with a definition in another block, and the reference link is output as plain text.

### Rejected options

- Pass all of the document's definitions to each block. A new mechanism to collect definitions would have to be built.
- Make reference links across blocks an error. The reason it was not adopted is not on record.

### Confirmations

- C-view-rendering-12 A reference link in a block is resolved with a definition in the same block, and a reference link that uses a definition in another block is output as plain text

## References do not link outside the page; selecting one opens its body in place

Status: implemented

### Problem

kotowari does not know where the repository is published, and the simple server that serves the overview serves only what is under the location of the drawn pages. The user judged that as long as the source of a reference can be properly viewed, there is no need to jump outside the page.

### Decision

A reference is drawn with its display name from the reference table, and selecting it opens and shows the reference's body without leaving the page. No reference is made a link outside the page. How a reference without a body is drawn is decided in the Languages topic under [A reference without a body draws only its display name and opens nothing when selected](languages.md#a-reference-without-a-body-is-drawn-as-its-display-name-only-and-opens-nothing-when-selected).

### Rejected options

- Link to the repository URL written in the configuration.
- Show only the display name.

### Confirmations

- C-view-rendering-13 A reference is drawn with its display name, selecting it opens its body, and the page has no link to the file the reference string points to

## Mark superseded and deferred references

Status: implemented

### Problem

That a chain of decisions has become stale is not made a finding. That a decision was superseded can be determined mechanically from the records, and it is enough for it to be visible on the page. Since planned things are also included in the overview, they need to be distinguishable from current ones.

### Decision

A reference has one of three states: "current", "superseded" and "deferred". A superseded reference gets the superseded marker, a deferred reference gets the deferred marker, and a current one gets none. The marker text is taken from the UI strings.

### Confirmations

- C-view-rendering-14 A superseded reference gets the superseded marker, a deferred reference gets a separate deferred marker, and a current reference gets none

## Draw a stale-section marker near the heading of a stale section

Status: implemented

### Problem

Readers should be able to tell which sections of the overview have not yet been reviewed since the specification they came from changed. The fixing is done not by the implementation loop but by the next brainstorm that touches the topic, so no work is added to the implementation loop.

### Decision

Whether a section is stale is computed per section with a guide marker by the side outside view and passed in the rendering input. Near the heading of a section passed as stale, view draws a marker showing that it has not yet been reviewed since the specification changed. The marker text is taken from the UI strings.

### Confirmations

- C-view-rendering-15 A marker appears only near the headings of sections passed as stale, and not on the headings of sections that are not stale

## Pages load no fonts, styles or images from outside

Status: implemented

### Problem

If a page loads something from an outside server, the display breaks where there is no network, and the fact that the page was opened is conveyed to the outside server. Since images can be written in Markdown in a text block, the person who wrote the source data may also unintentionally make the page load outside images. For fonts, the reason for deciding not to load them from outside is not on record.

### Decision

Pages do not load fonts, styles or images from outside the page (locations starting with "http://" or "https://"). Fonts use the OS fonts, and styles use only the "style.css" that view returns. Even if a text block contains a way of writing that points to an outside image or style, no element that loads it is created on the page.

Scripts, not only from outside, are [not included in any page](navigation.md#no-page-that-view-returns-contains-a-script).

### Confirmations

- C-view-rendering-16 No page has a specification that loads fonts from outside (a reference starting with "http://" or "https://")
- C-view-rendering-19 No page that draws every kind of part has a specification that loads styles or images from outside
- C-view-rendering-20 Even if Markdown or HTML pointing to an outside image or style is written in a text block, no image, style, script or embedding element is created on the page
