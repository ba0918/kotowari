# Changelog

## [Unreleased]

### Changed

- The minimum supported Rust version (`rust-version`) is raised from 1.89 to 1.99 (`kotowari-markdown-schema`, `kotowari-markdown-schema-io`, `kotowari-markdown-view`, `kotowari-mds`).
- **BREAKING (Rust API and package layout)** Unvalidated construction and modification of `Schema` are now private, and `Schema::parse` performs semantic validation. `extract_validated` returns the values of a successful validation, and `extract_partial` returns the values obtained so far together with the findings. Calling code written against the old API needs to migrate.
- Fetching is split into the `kotowari-markdown-schema-io` package and the executable into the `kotowari-mds` package. The binary's commands, output and exit codes are unchanged.

### Added

- `SchemaLoader`, which returns load results for reuse, and `AsyncSchemaLoader` behind the `tokio` feature, disabled by default.
- The `kotowari-markdown-view` crate, which turns documents made of Markdown text and parts of fixed kinds into static HTML pages in memory. `part_schema` returns the JSON Schema of the eight part kinds. It reads and writes no files, and the pages load nothing from outside.
- `SchemaLoader::read_document`, which reads a document as a UTF-8 string with a leading BOM removed, the same way `load` does, without resolving a schema.

### Fixed

- A dot-separated name that is empty (`""` or `a..b`) in the `value` and `of` keys of a schema's `extract` now stops with `schema_invalid`, the same as `path`. Before, it passed, and the extracted values got an empty-string key.
- In paragraph reading, the `pattern` of a list now applies only to the marker line. Before, it was matched including lazy continuation lines (lines continued without indentation), and the result disagreed with line reading.
- In a CRLF document, `\r` no longer gets into the values of list and field lines in paragraph reading. Value lines are joined with one newline, the same as in an LF document. Before, child lines and wrapped lines were joined with `\r\n`, and the values disagreed with line reading too.
- In line reading, an item that opens a fence after a list marker (`- ```python`) is now read as an item whose content is a code block, the same as in paragraph reading. Before, the closing line was mistaken for the start of a new fence, and everything up to the end of the document, later headings included, was swallowed into the code block.
- `kotowari-markdown-view` no longer draws GFM footnote syntax (`[^1]`) as footnotes. Drawing them put an English heading "Footnotes", which is not in the UI text, and back-link characters into the page. Footnote syntax is read as CommonMark reads it.
- `kotowari-mds check` given a directory no longer reads the same schema again for each document. A URL schema is fetched only once per check. Before, when the cache could not be written, the same URL was fetched once per document pointing at it.
- `kotowari-markdown-view` now draws a reference missing from the reference table as text that opens nothing when chosen, the same as a reference without a body. Before, it drew an open-and-close element with nothing inside.

The version is based on this crate's `Cargo.toml`. These changes do not include a release, a tag or publication.
