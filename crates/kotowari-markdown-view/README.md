# kotowari-markdown-view

Render documents made of Markdown text and declared parts into static HTML pages in memory.
`render` takes a `RenderInput` (the documents and a reference table) and returns the pages `index.html`, one `<name>.html` per document and `style.css`, as names and contents.
It reads and writes no files, opens no network connection, and does not validate its input.
The pages load nothing from outside themselves; a reference opens its body in place instead of linking out.

Each part kind (`lead`, `flow`, `steps`, `cards`, `status`, `compare`, `decisions`, `quiz`) has an embedded JSON Schema that `part_schema` returns.
The schema is the authority for a part's shape; validate values against it before rendering.

Run `cargo run --example render -p kotowari-markdown-view`.
