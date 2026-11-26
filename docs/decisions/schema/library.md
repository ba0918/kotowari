# Library entry points

English | [日本語](library.ja.md)

This covers the entry points that the pure library `kotowari-markdown-schema`, which reads no files, publishes, and which part takes charge of reading and writing. The behavior of the entry points for validation and extraction is decided in [the "Validation and value extraction" topic](library-extraction.md), and loading from files and URLs in [the "Loading library" topic](library-io.md).

## A depending crate can go from reading the reference to extraction with the decided entry points alone

Status: implemented

### Problem

kotowari uses mds as a crate, not by starting it as a CLI. This spares the user from installing a second binary, and a version mismatch shows up at compile time. But if a depending crate relies on public items that the specification does not describe, the mechanical matching of the specification against the implementation misses exactly that boundary.

Also, the policy for how the public API changes over time should be made clear to users who depend directly on the lower crate.

### Decision

`kotowari-markdown-schema` publishes the following entry points.

| Entry point | What it does |
|---|---|
| `frontmatter_schema` | Reads the "$schema" reference from the string of a document, and returns the reference or that there is no reference |
| `resolve_schema` | Decides the location of the schema (a file path or a URL) from the location of the document and the reference |
| `Schema::parse` | Parses the YAML of a schema ([A schema is returned only after it passes semantic validation, and a parsed schema can be reused](library-extraction.md#a-schema-is-returned-only-after-it-passes-semantic-validation-and-a-parsed-schema-can-be-reused)) |
| `Document::parse` | Reads the string of a document and returns the structure of the document |
| `validate` | Validates a schema and a document according to `ValidationOptions`, and returns the list of findings ([Validation in the library does not cancel the schema's open](library-extraction.md#validation-in-the-library-does-not-cancel-the-schemas-open)) |
| `extract_validated` | Validates, and returns values only when there is no violation ([The normal entry point for value extraction returns values only from a document with no violation](library-extraction.md#the-normal-entry-point-for-value-extraction-returns-values-only-from-a-document-with-no-violation)) |
| `extract_partial` | Returns the findings and the values that could be obtained (same as above) |
| `ast_json` | Produces the JSON of the plain syntax tree ([The CLI's values and ast --schema output values as before even for a document with violations](library-extraction.md#the-clis-values-and-ast---schema-output-values-as-before-even-for-a-document-with-violations)) |
| `extract_typed_partial` | Produces the JSON of the typed extraction (same as above) |

All the published entry points and the types of their inputs and outputs are under compatibility management. A published item is not treated as "an implementation convenience that may change at any time".

### Rejected options

- Previous decision: List only the entry points that a depending crate may rely on as a contract, and treat public items not in the list as implementation conveniences that may change at any time. Because it was decided that the public API of the lower crate is also under compatibility management, everything published is treated as a contract, and the internal implementation is not published ([Internal implementation types do not appear in public signatures](#internal-implementation-types-do-not-appear-in-public-signatures)).
- Start the mds CLI from kotowari and read its output JSON. This would require the user to install mds, and a version mismatch would show up only at run time. The difference in performance, when measured, was small enough to ignore.

### Confirmations

- C-schema-lib-1 From the string of a document that declares a schema, calling only the public entry points in order yields the "$schema" reference, the location of the schema, the list of findings, the extracted values, and the JSON of the plain syntax tree

## Internal implementation types do not appear in public signatures

Status: implemented

### Problem

If the syntax tree types of the Markdown parsing tool or the types of the internal search tool were published, outside users would come to rely on internal implementation types.

The fields of a finding are already limited to inside the crate, and what is needed from outside is only the means to read them. Yet if the functions that construct findings were public, a way of constructing them that does not go through validation would remain public.

### Decision

Public signatures do not include `markdown::mdast::Node` or `regex::Captures`. Extracted values and the syntax tree JSON are handed out as `serde_json::Value`, as the existing extraction contract says.

The functions that construct a finding (`Finding`) (`new`, `at`, `maybe_at`, `of_node`, `of_rule`) are limited to inside the crate, and only the means to read findings is published to the outside.

### Confirmations

- C-schema-lib-2 (by reading) `markdown::mdast::Node` and `regex::Captures` do not appear in the public signatures of `kotowari-markdown-schema`
- C-schema-lib-3 (by reading) The functions that construct findings cannot be called from outside the crate
- C-schema-lib-4 From a finding returned by `validate`, the kind, line, and detail of the finding can be read through the public means of reading

## The library reads no files and does not fetch schemas at URLs

Status: implemented

### Problem

The core of the schema language, validation, and extraction is placed in a separate crate that does not depend on the CLI framework, so that it can be reused from inside kotowari.

Users who use only in-memory validation should not be asked to take on dependencies for file search or HTTP fetching. It is also so that the direction of dependencies is enforced by the compiler. The resulting increase in the number of crates and in release work was accepted.

### Decision

`kotowari-markdown-schema` does not read the files of documents and schemas, and does not fetch schemas at URLs. `resolve_schema` goes as far as deciding the location of the schema (a file path or a URL). Reading and writing are taken charge of by the caller or by a separate crate that reads from files and URLs ([Loading of schemas and documents is placed in a separate crate shared by the CLI and outside applications](library-io.md#loading-of-schemas-and-documents-is-placed-in-a-separate-crate-shared-by-the-cli-and-outside-applications)).

### Confirmations

- C-schema-lib-5 For a document that declares a schema at a URL, `resolve_schema` returns the URL and nothing is fetched
- C-schema-lib-6 (by reading) `kotowari-markdown-schema` has neither code that reads files nor dependencies for fetching over HTTP
