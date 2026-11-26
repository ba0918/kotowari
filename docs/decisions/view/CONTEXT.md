# Glossary

English | [日本語](CONTEXT.ja.md)

Terms used in the decision documents of kotowari-markdown-view (view). view is a rendering engine that does not know kotowari, and the terms here are the words of view's input and output.

| Term | Meaning |
|---|---|
| render input | Everything view receives for one rendering. Consists of a sequence of `document` entries, the `reference table`, the `table of contents`, a language tag, the `UI text` and the `other languages` |
| document | The source of one `page` within the `render input`. Has a name, a title, the opening lead `part`, the sequence of opening `part` entries that follow the lead, and the sequence of `section` entries. Not a file |
| section | One "## " heading within a `document` together with the sequence of `block` entries under it. Has a true-or-false value telling whether it is stale |
| block | The unit of the contents of a `section`. Either a chunk of Markdown text or a `part` |
| part | A unit of drawing of a fixed kind. Has the name of its kind and a value that fits the schema of that kind. Not called a "block" or a "component" |
| part schema | A JSON Schema that declares, for each kind of `part`, the shape of its contents. view holds and publishes it, and it is the authority on the shape of parts |
| reference | A string written in a field named "refs" or "ref" within the value of a `part` |
| reference table | A mapping from the string of a `reference` to display name, body and status. Built outside view and passed to it. Not called a "link table" |
| page | The combination of the name and the contents of one file that view outputs. view writes no files; it returns the sequence of pages |
| table of contents | Part of the `render input`: the outermost `contents group`, which decides the order and nesting of each `page` on the listing page. Not called a "map" or an "index" |
| contents group | A bundle inside the `table of contents`. Has a title, an optional one-line description, and items that list in order names of a `document` and nested `contents group` entries. Not called a "category" or a "folder" |
| outline | The sequence of the headings of each `section` of a `page`, drawn on the `page` of a `document`. Each entry links to its `section`. Not called a `table of contents` |
| UI text | Part of the `render input`: a mapping from keys to strings. Every text view writes into a page that does not come from another part of the `render input` is taken from here. Not called "labels" or "translations" |
| other languages | Part of the `render input`: the sequence of places where the same `page` is drawn in other languages. Each entry has the name of the language and the path to that place relative to the place of this sequence of `page` entries |
