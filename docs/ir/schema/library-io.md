# The library that loads schemas and documents

English | [日本語](library-io.ja.md)

Covers the entry points of a separate crate that obtains schemas and documents from files and URLs, and the reuse of the existing loading behavior.

## Requirements

### REQ-schema-072: Entry points for loading

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A17, docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A44, docs/decision/records/2026-10-03-public-crate-api.md#A52
- verification: unit

"kotowari-markdown-schema-io" takes, through "SchemaLoader::new" and "LoaderOptions", an absolute path as the starting location of the work and an optional cache base. The contract for locations follows REQ-core-323. "load" returns a reusable schema and document, "check" returns the result for a single file or a directory, and "extract_validated" and "extract_partial" return their respective extraction results.

### REQ-schema-073: Responsibilities for I/O and display

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A9, docs/decision/records/2026-10-03-public-crate-api.md#A17, docs/decision/records/2026-10-03-public-crate-api.md#A44
- verification: unit

The I/O crate keeps the existing CLI's rules for searching, URL resolution, caching, fetch limits, and timeouts, and does not change the current directory. The cache follows the existing location and lifetime. It does not write results to standard output or standard error, leaving display and the decision of the exit code to the CLI.

### REQ-schema-074: Loading failures and document findings

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A8, docs/decision/records/2026-10-03-public-crate-api.md#A40, docs/decision/records/2026-10-03-public-crate-api.md#A44
- verification: unit

The I/O crate distinguishes the findings of a completed document check from failures of loading or of the input format. A failure that prevents execution is an "Err" of a "Result" whose kind can be told apart, and the failure type implements "std::error::Error" and "Display".

### REQ-schema-075: Optional asynchronous entry point

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A22, docs/decision/records/2026-10-03-public-crate-api.md#A24, docs/decision/records/2026-10-03-public-crate-api.md#A27, docs/decision/records/2026-10-03-public-crate-api.md#A30, docs/decision/records/2026-10-03-public-crate-api.md#A31, docs/decision/records/2026-10-03-public-crate-api.md#A32, docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

"kotowari-markdown-schema-io" provides "AsyncSchemaLoader" behind the "tokio" feature, which is disabled by default. The contracts for the scope of asynchronous operations, the runtime, the execution slots, and cancellation while waiting follow REQ-core-318, REQ-core-319, REQ-core-320, and REQ-core-321.

## Examples

```gherkin
@id=EX-schema-090 @about=REQ-schema-072,REQ-schema-073 @source=docs/decision/records/2026-10-03-public-crate-api.md#A17,docs/decision/records/2026-10-03-public-crate-api.md#A44
Scenario: Loading a document and its schema by passing locations
  Given a SchemaLoader with the starting location of the work and the cache base specified
  When a document with a schema reference is loaded with load
  Then it returns a reusable schema and document
  And it does not change the current directory and does not display results on the terminal

@id=EX-schema-091 @about=REQ-schema-073,REQ-schema-074 @source=docs/decision/records/2026-10-03-public-crate-api.md#A8,docs/decision/records/2026-10-03-public-crate-api.md#A40,docs/decision/records/2026-10-03-public-crate-api.md#A44
Scenario: A failure to fetch the schema is not confused with a violation in the document
  Given the schema cannot be loaded under the existing fetch rules
  When check is called on the document
  Then it returns a typed execution failure rather than the result of a completed document check

@id=EX-schema-092 @about=REQ-schema-075 @source=docs/decision/records/2026-10-03-public-crate-api.md#A22,docs/decision/records/2026-10-03-public-crate-api.md#A27,docs/decision/records/2026-10-03-public-crate-api.md#A45
Scenario: A call that opts into Tokio support gets the same results as the synchronous one
  Given a user who has enabled the tokio feature prepares the same input
  When AsyncSchemaLoader is awaited on Tokio
  Then it returns the same values and findings as the synchronous SchemaLoader
```
