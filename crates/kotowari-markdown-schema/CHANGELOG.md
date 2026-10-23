# Changelog

Notable changes to mds — the `kotowari-mds` command and the `kotowari-markdown-schema` crates — newest first. Versions follow [Semantic Versioning](https://semver.org/).

Entries are written for someone upgrading from one release to the next: what changed, and what that asks of you.

## [Unreleased]

## [0.2.0] - 2026-10-24

### Changed

- The minimum supported Rust version (`rust-version`) is raised from 1.89 to 1.99 (`kotowari-markdown-schema`, `kotowari-markdown-schema-io`, `kotowari-markdown-view`, `kotowari-mds`).
- **BREAKING (Rust API and package layout)** Unvalidated construction and modification of `Schema` are now private, and `Schema::parse` performs semantic validation. `extract_validated` returns the values of a successful validation, and `extract_partial` returns the values obtained so far together with the findings. Calling code written against the old API needs to migrate.
- **BREAKING** mds moved into the kotowari repository, and the command is renamed from `mds` to `kotowari-mds`. Install it with `cargo install --git https://github.com/ba0918/kotowari --bin kotowari-mds kotowari-mds`, or download the `kotowari-mds` archive of a GitHub Release. Scripts that call `mds` need the new name; the subcommands, their options and the exit codes are unchanged.
- **BREAKING** URL schemas are now cached in `.kotowari/cache/schemas/` under the nearest directory, from the current directory upwards, that has `.kotowari/` (the current directory when none has), instead of in `.mds/cache/` under the nearest directory that has `.mds/`. `.mds/` is no longer looked for, so an existing cache there is not used and URL schemas are fetched again; the old `.mds/cache/` can be deleted.
- **BREAKING (package layout)** The library crate `mds-core` is now `kotowari-markdown-schema`. Fetching is split into the `kotowari-markdown-schema-io` package and the executable into the `kotowari-mds` package.
- A schema that declares `name` and places a root path on the `type` key now stops with `document has the key "<path>", which conflicts with "type", where typed extraction puts "name"`. The reason used to name the `ast --schema` subcommand, which a library user does not run.

### Added

- `SchemaLoader`, which returns load results for reuse, and `AsyncSchemaLoader` behind the `tokio` feature, disabled by default.
- The `kotowari-markdown-view` crate, which turns documents made of Markdown text and parts of fixed kinds into static HTML pages in memory. `part_schema` returns the JSON Schema of the eight part kinds. It reads and writes no files, and the pages load nothing from outside.
- `SchemaLoader::read_document`, which reads a document as a UTF-8 string with a leading BOM removed, the same way `load` does, without resolving a schema.

### Removed

- **BREAKING (Rust API)** `finding::Finding::new`, `at`, `maybe_at`, `of_node` and `of_rule` are no longer public. A `Finding` is now made only by validation and extraction; outside the crate, read it through its accessors (`kind`, `line`, `node`, `raw`, `rule_kind`, `detail`).

### Fixed

- A dot-separated name that is empty (`""` or `a..b`) in the `value` and `of` keys of a schema's `extract` now stops with `schema_invalid`, the same as `path`. Before, it passed, and the extracted values got an empty-string key.
- In paragraph reading, the `pattern` of a list now applies only to the marker line. Before, it was matched including lazy continuation lines (lines continued without indentation), and the result disagreed with line reading.
- In a CRLF document, `\r` no longer gets into the values of list and field lines in paragraph reading. Value lines are joined with one newline, the same as in an LF document. Before, child lines and wrapped lines were joined with `\r\n`, and the values disagreed with line reading too.
- In line reading, an item that opens a fence after a list marker (`- ```python`) is now read as an item whose content is a code block, the same as in paragraph reading. Before, the closing line was mistaken for the start of a new fence, and everything up to the end of the document, later headings included, was swallowed into the code block.
- `kotowari-markdown-view` no longer draws GFM footnote syntax (`[^1]`) as footnotes. Drawing them put an English heading "Footnotes", which is not in the UI text, and back-link characters into the page. Footnote syntax is read as CommonMark reads it.
- `kotowari-mds check` given a directory no longer reads the same schema again for each document. A URL schema is fetched only once per check. Before, when the cache could not be written, the same URL was fetched once per document pointing at it.
- `kotowari-markdown-view` now draws a reference missing from the reference table as text that opens nothing when chosen, the same as a reference without a body. Before, it drew an open-and-close element with nothing inside.

## [0.1.0] - 2026-09-22

First release. `mds` validates a Markdown document against a YAML schema the document declares
itself, and hands its contents back as structured data.

### The command

- `mds check <file|dir>` validates a document, or every schema-declaring `.md` under a directory,
  and reports findings as text or JSON. Exit code 0 when clean, 1 when a document has findings,
  2 when the check could not run.
- `mds values <file>` extracts the declared values as JSON or as indented text.
- `mds ast <file>` prints the Markdown tree, with `--schema` for the typed form.
- `--open` relaxes closed-world validation for undeclared structures.

### The schema language

A schema declares the shape of a document: `title`, `preamble`, `sections`, `item`, `field`,
`statement`, `bullets`, `table`, `codeblock`. Every rule takes an occurrence count (`required`,
`repeat`), an `extract` naming where its value lands, and an optional `when` making it
conditional on another field's value.

Validation is closed-world by default: a heading or a line the schema never declared is an error.

Output keys come from the schema, not from the document. Renaming a heading changes the schema,
not the JSON your code reads.

### The library

`mds-core` is usable as a crate. Six entry points are the contract: `frontmatter_schema`,
`resolve_schema`, `parse_schema`, `Document::parse`, `validate`, `extract_values`. Reading files
and fetching URLs stays with the caller.

### Installing

```console
$ cargo install --git https://github.com/ba0918/mds
```

Requires Rust 1.89 or newer.

[Unreleased]: https://github.com/ba0918/kotowari/compare/kotowari-mds-v0.2.0...HEAD
[0.2.0]: https://github.com/ba0918/kotowari/releases/tag/kotowari-mds-v0.2.0
[0.1.0]: https://github.com/ba0918/mds/releases/tag/v0.1.0
