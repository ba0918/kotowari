# Changelog

Notable changes to mds — the `kotowari-mds` command and the `kotowari-markdown-schema` crates — newest first. Versions follow [Semantic Versioning](https://semver.org/).

Entries are written for someone upgrading from one release to the next: what changed, and what that asks of you.

## [Unreleased]

### Added

- A preamble, a section, an item and the child rules of a bullet (`children`) can declare `open: true`. Such a node accepts, with their contents, blocks of the kinds it has no rule for (paragraphs, ordered lists, tables, code blocks, quotes, bullets), while its own rules keep every constraint. This is separate from the schema's top-level `open` and `check --open`, which still do not relax a declared node.
- A section rule can select headings by a list of names (`name: [未決, Open]`; a single string still works) or by a regular expression searched in the heading text (`pattern: "^Ch"`; `pattern: ""` selects every heading). A heading belongs to the first rule, in declared order, that selects it. Findings of a name-list rule carry the first name of the list as `node`; findings of a pattern rule carry no `node` and name the pattern in the detail. A section rule must declare exactly one of `name` and `pattern`, and a name may appear only once across the section rules.
- The derived value `name` can be declared in `of` of a section's extraction; it holds the section's heading text.
- A key of `of` can take `{ group: <name> }`, the text a named capture of the node's regular expression captured, on the title and on bullets (for example `of: { id: { group: id }, line: line }`). A capture that took no part in the match leaves its key out of that element. `end` can be declared on bullets too: the last line of the list item, continuation paragraphs and child lists included.
- A new finding `unclosed_codeblock` reports a code block opened with a fence (` ``` ` or `~~~`) that is never closed, at the fence's opening line, whatever the schema: in the closed and the open world, in any node, inside list items and quotes, and in both reading modes. Such a block swallows everything after it, which used to pass unchecked.
- A preamble, a section and an item can declare `subheading`, a rule for headings of depth 4 or more, with `required`, `repeat` and `pattern` (no `extract`). A heading that does not match the pattern is the new finding `subheading_pattern_mismatch`; a count outside the bounds is `repeat_min_not_met` or `repeat_max_exceeded` with the rule kind `subheading`.

### Changed

- **BREAKING** `group` written next to `path` (`extract: { path: id, group: id }`, title only) is no longer accepted and stops with `schema_invalid`. Write it in `of` instead: `extract: { path: id, of: { id: { group: id } } }` puts the capture under `id.id`; the title's own `value` stays the heading text.
- **BREAKING** When reading by paragraph, the regular expression of a bullet (and its named captures) is matched against the lead paragraph of the list item joined into one line (each line trimmed, line breaks replaced by one space), not only against the marker line. A pattern anchored at the end of the marker line (`^first line$`) no longer matches a bullet that wraps onto a second line; widen it to cover the whole sentence. Continuation paragraphs and child lists are still not matched, and `reading: line` still matches the marker line only.
- **BREAKING** Item rules are written as a list in `items` (under a section) and `document.items` (directly under the document); a schema that writes `item` or `document.item` now stops with `schema_invalid`. To migrate, rename `item:` to `items:` and make its rule the one element of the list (`item: X` becomes `items: [X]`); the findings and values stay the same. An item rule can select headings by `name` (one name or a list) or `pattern`, like a section rule; only the last rule may have neither, and that rule (the ID rule) takes every remaining heading and is the only one that requires the `ID: Name` form. A heading no rule selects is `undeclared_heading`, and its lines are `undeclared_line`. An item selected by name or pattern has its whole heading as `name` and no `id`. Findings of a name-list item rule carry its first name as `node`.
- **BREAKING** When reading by paragraph (the default), a block quote, an HTML block and a thematic break inside a node that does not declare `open: true` are now `undeclared_line` findings, with the rule kind `block_quote`, `html` or `thematic_break`; they used to pass silently, even in the closed world. Image-only paragraphs, link reference definitions and blocks of only HTML comments are still ignored. To keep accepting such blocks, declare `open: true` on the node that holds them.
- A heading of depth 4 or more is now read as a line of the preamble, section or item that contains it: it no longer ends that node for anything that follows it, and the lines after it are checked and extracted with the same node. Without a `subheading` rule it is still `heading_level_mismatch`, also inside a node that declares `open: true` and inside an undeclared section in the open world. Findings come out in document order, so a `heading_level_mismatch` is no longer listed after all other findings.

### Fixed

- A document whose lines end with a lone carriage return (CR, without LF) had its frontmatter overlooked, so `kotowari-mds` stopped with `schema_not_found` although the document declared `$schema`. The frontmatter is now read with all three line breaks (LF, CRLF and CR), as the rest of the document already was.

## [0.2.1] - 2026-11-25

### Changed

- The release archives of `kotowari-mds` now carry `THIRD-PARTY-LICENSES.txt`: the crates built into the binary, each with its licence, authors and source, followed by their licence texts. Earlier archives carried only the project's own licences. Where a crate's package ships no licence file, the standard text of its licence is given and the list names the crate's authors.

### Security

- The `kotowari-mds` binary is built with `rustls-webpki` 0.103.17 and `webpki-roots` 1.0.12. The versions in 0.2.0 could accept a certificate chain through distrusted emSign intermediates when fetching a schema from an `https` URL (RUSTSEC-2026-0381, RUSTSEC-2026-0382). A project that depends on `kotowari-markdown-schema-io` as a library resolves these crates itself: run `cargo update -p rustls-webpki -p webpki-roots`.

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

[Unreleased]: https://github.com/ba0918/kotowari/compare/kotowari-mds-v0.2.1...HEAD
[0.2.1]: https://github.com/ba0918/kotowari/compare/kotowari-mds-v0.2.0...kotowari-mds-v0.2.1
[0.2.0]: https://github.com/ba0918/kotowari/releases/tag/kotowari-mds-v0.2.0
[0.1.0]: https://github.com/ba0918/mds/releases/tag/v0.1.0
