# kotowari-markdown-view

Render documents made of Markdown text and declared parts into static HTML pages in memory.
`render` takes a `RenderInput` (the documents, a reference table and a table of contents) and returns the pages `index.html`, one `<name>.html` per document and `style.css`, as names and contents.
The index draws the table of contents as nested, foldable groups in the written order, with page counts and the counts of sections not yet reviewed and of open and planned statements.
A name in the contents without a document is skipped, a document missing from the contents gets its page but no index entry, and a repeated name is drawn each time.
It reads and writes no files, opens no network connection, and does not validate its input.
The pages load nothing from outside themselves; a reference opens its body in place instead of linking out.

Each part kind (`lead`, `flow`, `steps`, `cards`, `status`, `compare`, `decisions`, `quiz`) has an embedded JSON Schema that `part_schema` returns.
The schema is the authority for a part's shape; validate values against it before rendering.

Run `cargo run --example render -p kotowari-markdown-view`.
