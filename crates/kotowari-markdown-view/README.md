# kotowari-markdown-view

Render documents made of Markdown text and declared parts into static HTML pages in memory.
`render` takes a `RenderInput` (the documents, a reference table, a table of contents, a language tag, the UI text and the other languages) and returns the pages `index.html`, one `<name>.html` per document and `style.css`, as names and contents.
The view holds no text of any language: every word it writes itself (counts, marks, column headings, status labels, the link to the index) comes from the UI text, a map from the keys `index_link`, `pages`, `stale_sections`, `open_items`, `planned_items`, `stale_mark`, `outline_stale`, `superseded`, `deferred`, `compare_before`, `compare_after`, `compare_why`, `state_decided`, `state_planned`, `state_open` and `state_dropped` to strings, with `{n}` replaced by the number in the counted ones.
The language tag becomes the `lang` of every page, and every page links the same page of each other language (its name and the relative path to its pages) without a script.
Render once per language.
The index draws the table of contents as nested, foldable groups in the written order, with page counts and the counts of sections not yet reviewed and of open and planned statements; a document page shows its place in the contents above its title and links to the other pages of its group after its last section.
A name in the contents without a document is skipped, a document missing from the contents gets its page but no index entry, and a repeated name is drawn each time.
It reads and writes no files, opens no network connection, and does not validate its input.
The pages load nothing from outside themselves; a reference opens its body in place instead of linking out, and a reference without a body is drawn as its label only.
The `status` part's states are `decided`, `planned`, `open` and `dropped`.

Each part kind (`lead`, `flow`, `steps`, `cards`, `status`, `compare`, `decisions`, `quiz`) has an embedded JSON Schema that `part_schema` returns.
The schema is the authority for a part's shape; validate values against it before rendering.

Run `cargo run --example render -p kotowari-markdown-view`.
