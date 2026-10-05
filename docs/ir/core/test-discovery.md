# How tests are found

English | [日本語](test-discovery.ja.md)

Covers how test files are chosen, and the rules that determine the language from the extension and find tests and their names with that language's queries. The contents of the queries bundled for each language are covered by test-queries.md, and the queries added through the configuration by query-rules.md.

## Requirements

### REQ-core-079: Test files

- kind: ubiquitous
- source: docs/decision/records/records.md#A36, docs/decision/records/records.md#A47, docs/decision/records/records.md#A94, docs/decision/records/records.md#A102, docs/decision/records/records.md#A146, docs/decision/records/records.md#A159, docs/decision/records/records.md#A165
- verification: unit

kotowari always reads the files matched by the globs of "tests.files" as a `test file`. Entries that are neither directories nor regular files (sockets, named pipes, devices) are not read even when a glob matches them (`exclusion`). The walk goes through the whole `base directory` (except hidden directories) and then selects with the globs, so it comes to a `stop` on an unreadable directory or a dangling symbolic link even in a place no glob matches. The walk does not follow symbolic links to directories, reads symbolic links to files, and on a dangling symbolic link comes to a `stop` with an unreadable file as the reason.

### REQ-core-080: Reading with tree-sitter

- kind: ubiquitous
- source: docs/decision/records/records.md#A24, docs/decision/records/records.md#A58, docs/decision/records/2026-09-24-multi-language-tests.md#A1, docs/decision/records/2026-09-24-multi-language-tests.md#A7
- verification: unit

kotowari always reads a `test file` of a `language with a query` with tree-sitter, and finds each `test` by applying every `query` of that language. A `test file` of a `language without a query` is not read with tree-sitter.

### REQ-core-081: Mapping of extensions to languages

- kind: algorithm
- source: docs/decision/records/records.md#A39, docs/decision/records/records.md#A58, docs/decision/records/records.md#A24, docs/decision/records/records.md#A123, docs/decision/records/2026-09-24-multi-language-tests.md#A3, docs/decision/records/2026-09-24-multi-language-tests.md#A6, docs/decision/records/2026-09-24-multi-language-tests.md#A21
- definition: TBL-core-031
- verification: unit

### REQ-core-083: Unreadable test files

- kind: event_driven
- source: docs/decision/records/records.md#A58, docs/decision/records/records.md#A120, docs/decision/records/records.md#A149, docs/decision/records/2026-09-24-multi-language-tests.md#A23
- verification: unit

When there is a `test file` of a `language with a query` that tree-sitter cannot read (a file with even one syntax error), kotowari raises an unparsable_file `error`, skips that file, and does not come to a `stop`. Syntax errors when re-reading the contents of a macro of "tests.rust.macros" are not included in this; only the top-level functions that could be read are counted.

### REQ-core-084: Tests are not found with regular expressions

- kind: prohibition
- source: docs/decision/records/records.md#R3, docs/decision/records/2026-09-24-multi-language-tests.md#A2
- verification: review
- how_to_verify: Confirm that the code that finds tests does not look for definition lines by applying regular expressions to the lines of test files. The only regular expressions that come from the configuration are those applied to syntax-tree nodes inside ast-grep rules

kotowari must not find the definition lines of tests with regular expressions written in the `configuration file`.

### REQ-core-180: The node and name of a test

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A10, docs/decision/records/2026-09-24-multi-language-tests.md#A13, docs/decision/records/2026-09-24-multi-language-tests.md#A25, docs/decision/records/2026-09-24-multi-language-tests.md#A36
- verification: unit

kotowari always takes one syntax-tree node matched by a `query` as one `test`, and takes the text of the node captured by the metavariable "$NAME" as the name of the `test`. When that text starts and ends with the same quote character (one of the single quote, the double quote and the backquote), one character is removed from each end; any other form becomes the name as is. When the `query` does not capture "$NAME", the name of the `test` is null.

### REQ-core-181: Overlapping matches

- kind: ubiquitous
- source: docs/decision/records/2026-09-24-multi-language-tests.md#A27, docs/decision/records/2026-09-24-review10-gaps.md#A2
- verification: unit

kotowari always counts a node as one `test` when several `query` entries match the same node, and counts a node matched further inside the node of a `test` as a separate `test`. The name for a node matched by several `query` entries is the name from the first, in the order of the bundled `query` entries and then the order of "tests.rules", among the `query` entries that captured "$NAME".

## Decision tables

### TBL-core-031: Mapping of extensions to languages

- source: docs/decision/records/2026-09-24-multi-language-tests.md#A3, docs/decision/records/2026-09-24-multi-language-tests.md#A6, docs/decision/records/2026-09-24-multi-language-tests.md#A21, docs/decision/records/records.md#A123, docs/decision/records/2026-09-24-multi-language-tests.md#A7, docs/decision/records/2026-09-17-check-reach.md#A19, docs/decision/records/2026-09-24-multi-language-tests.md#A46, docs/decision/records/2026-09-24-multi-language-tests.md#A47

The extension is the text after the last "." of the file name, and is case-sensitive. A file whose extension is not in the table has no determined language and is treated as a language without a query.

| Language | Extensions |
|---|---|
| Bash | bash, bats, cgi, command, env, fcgi, ksh, sh, tmux, tool, zsh |
| C | c, h |
| Cpp | cc, hpp, cpp, c++, hh, cxx, cu, ino |
| CSharp | cs |
| Css | css, scss |
| Dart | dart |
| Elixir | ex, exs |
| Go | go |
| Haskell | hs |
| Hcl | hcl, nomad, tf, tfvars, workflow |
| Html | html, htm, xhtml |
| Java | java |
| JavaScript | cjs, js, mjs, jsx |
| Json | json |
| Kotlin | kt, ktm, kts |
| Lua | lua |
| Markdown | markdown, md |
| Nix | nix |
| Php | php |
| Python | py, py3, pyi, bzl, bazel |
| Ruby | rb, rbw, gemspec |
| Rust | rs |
| Scala | scala, sc, sbt |
| Solidity | sol |
| Swift | swift |
| TypeScript | ts, cts, mts |
| Tsx | tsx |
| Yaml | yaml, yml |

## Examples

```gherkin
@id=EX-core-293 @about=REQ-core-080 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A7,docs/decision/records/2026-09-24-multi-language-tests.md#A23
Scenario: A file of a language without a query is not read into a syntax tree
  Given "tests.files" contains "tests/**/*.go", "tests/a.go" has a syntax error, and there is no `query` for ".go"
  When "kotowari check" is run
  Then no unparsable_file error is raised

@id=EX-core-294 @about=REQ-core-083 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A23,docs/decision/records/2026-09-24-multi-language-tests.md#A8,docs/decision/records/2026-09-24-multi-language-tests.md#A21
Scenario: A syntax error in a file of a language with a query
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" ends with "it('x', () => {"
  When "kotowari check" is run
  Then one unparsable_file error for "tests/a.test.ts" is raised

@id=EX-core-295 @about=REQ-core-081 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A21,docs/decision/records/2026-09-24-multi-language-tests.md#A6,docs/decision/records/records.md#A123
Scenario: An uppercase extension determines no language
  Given "tests.files" contains "tests/**/*", and "tests/A.PY" has "def test_x():" with no mark
  When "kotowari check" is run
  Then no test_without_id error is raised

@id=EX-core-296 @about=REQ-core-180 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A13,docs/decision/records/2026-09-24-multi-language-tests.md#A25,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A15,docs/decision/records/2026-09-24-multi-language-tests.md#A16
Scenario: A string name has its quotes removed
  Given "tests.files" contains "tests/**/*.ts", and in "tests/a.test.ts", inside "describe('d', ...)", "it('does x', () => {})" comes right after the mark "@kotowari[REQ-001]" on line 3
  When "kotowari list" is run
  Then the "name" in the "tests" of "REQ-001" is "does x"

@id=EX-core-297 @about=REQ-core-181 @source=docs/decision/records/2026-09-24-multi-language-tests.md#A27,docs/decision/records/2026-09-24-multi-language-tests.md#A18,docs/decision/records/2026-09-24-multi-language-tests.md#A13
Scenario: A test inside a test is counted separately
  Given "tests.files" contains "tests/**/*.ts", and "tests/a.test.ts" has "it('outer', ...)" with no mark, with "it('inner', ...)" with no mark inside it
  When "kotowari check" is run
  Then one test_without_id error is raised with detail "outer" and one with detail "inner"
@id=EX-core-328 @about=REQ-core-181 @source=docs/decision/records/2026-09-24-review10-gaps.md#A2,docs/decision/records/2026-09-24-multi-language-tests.md#A9
Scenario: The name of the query that captured a name is taken
  Given "tests.files" contains "tests/**/*.ts", "tests.rules" lists, in this order, a rule file with "language: typescript" that matches "bench($$$)" and a rule file that matches "bench($NAME, $$$)", and "tests/a.test.ts" has "bench('chosen', () => {})" with no mark
  When "kotowari check" is run
  Then one test_without_id error with detail "chosen" is raised
```
