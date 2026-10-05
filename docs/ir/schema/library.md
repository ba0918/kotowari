# Library entry points

English | [日本語](library.ja.md)

Covers the public entry points of the pure "kotowari-markdown-schema" and its boundary for reading and writing. The results of validation and extraction are covered in library-extraction.md, and file and HTTP operations in library-io.md.

## Requirements

### REQ-schema-049: Library entry points

- kind: algorithm
- source: docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A43
- definition: TBL-schema-010
- verification: unit

### REQ-schema-050: Reading and writing are the caller's responsibility

- kind: prohibition
- source: docs/decision/records/2026-09-21-mds-spec.md#A18, docs/decision/records/2026-09-21-mds-spec.md#A57, docs/decision/records/2026-10-03-public-crate-api.md#A11, docs/decision/records/2026-10-03-public-crate-api.md#A17
- verification: unit

"kotowari-markdown-schema" does not read the files of the `document` and the `schema`, and does not fetch a `schema` at a URL. It goes as far as deciding the location of the `schema`, and leaves reading and writing to the caller or to the separate I/O crate.

### REQ-schema-051: Public contract and internal implementation

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A13, docs/decision/records/2026-10-03-public-crate-api.md#A16, docs/decision/records/2026-10-03-public-crate-api.md#A40, docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: review
- how_to_verify: Check the public entry points and input and output types in rustdoc, and confirm that no unvalidated construction path and no internal type of a dependency is public. Exposing JSON values is allowed as the existing extraction contract.

"kotowari-markdown-schema" places the entry points of TBL-schema-010 and the public types of their inputs and outputs under compatibility management, and keeps the internal implementation private. It does not include "markdown::mdast::Node" or "regex::Captures" in public signatures. Extracted values and the AST JSON are handled as "serde_json::Value".

## Decision tables

### TBL-schema-010: Library entry points

- source: docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A43, docs/decision/records/2026-10-03-public-crate-api.md#A56

| Entry point | What it does | What it returns |
|---|---|---|
| frontmatter_schema | Reads the "$schema" reference from the string of a `document` | The reference, or none. An error if the `frontmatter` is broken |
| resolve_schema | Decides the location of the `schema` from the location of the `document` and the reference | A file path, or a URL |
| Schema::parse | Parses the YAML of a `schema` and validates its semantics | An immutable Schema, or a schema failure |
| Document::parse | Reads the string of a `document` | The structure of the `document` |
| validate | Validates a Schema and a Document with ValidationOptions | The document's findings |
| extract_validated | Validates, and extracts if there is no violation | ValidatedValues, or the document's findings |
| extract_partial | Validates and extracts the values that can be obtained | A PartialExtraction holding the findings and the JSON values |
| ast_json | Produces the existing plain AST JSON | A JSON value, or an execution failure |
| extract_typed_partial | Produces the existing typed extraction JSON | The JSON value of the typed extraction, and the findings |

## Examples

```gherkin
@id=EX-schema-015 @about=REQ-schema-049 @source=docs/decision/records/2026-10-03-public-crate-api.md#A42,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: A depending crate can go all the way through with the entry points alone
  Given the string of a `document` that declares a `schema`
  When the entry points of TBL-schema-010 are called in order
  Then both the list of each `finding` and the values of the `extraction` are obtained

@id=EX-schema-016 @about=REQ-schema-050 @source=docs/decision/records/2026-09-21-mds-spec.md#A24,docs/decision/records/2026-09-21-mds-spec.md#A57
Scenario: For a schema at a URL, only the location is returned
  Given the string of a `document` that declares a `schema` at a URL
  When resolve_schema is called
  Then a URL is returned, and nothing is fetched

@id=EX-schema-084 @about=REQ-schema-051 @source=docs/decision/records/2026-10-03-public-crate-api.md#A13,docs/decision/records/2026-10-03-public-crate-api.md#A16,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: A candidate that exposes the plain syntax tree type does not conform to the contract
  Given a candidate whose public signature returns markdown::mdast::Node
  When the dependencies of the public API are checked
  Then it is not judged conforming, because an internal type is public
```
