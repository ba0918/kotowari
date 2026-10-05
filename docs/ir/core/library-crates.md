# Crate boundaries of the library

English | [日本語](library-crates.ja.md)

Covers the responsibilities of the published Rust libraries and the CLI, the direction of dependencies, and the entry points users choose. The procedures for publishing and version management are kept in decision records.

## Requirements

### REQ-core-306: Packages by purpose

- kind: algorithm
- source: docs/decision/records/2026-10-03-public-crate-api.md#A14, docs/decision/records/2026-10-03-public-crate-api.md#A25, docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A35, docs/decision/records/2026-10-04-overview-on-public-api.md#A4
- definition: TBL-core-040
- verification: review
- how_to_verify: Compare the Cargo package list, the normal dependency graph and external usage examples of each public entry point with the table.

### REQ-core-307: No environment operations on the computation side

- kind: prohibition
- source: docs/decision/records/2026-10-03-public-crate-api.md#A11, docs/decision/records/2026-10-03-public-crate-api.md#A25, docs/decision/records/2026-10-03-public-crate-api.md#A35, docs/decision/records/2026-10-04-overview-on-public-api.md#A1
- verification: review
- how_to_verify: Check the normal dependencies of core, source-analysis and overview, the call paths from the public entry points, and where file, environment variable, process, terminal and network operations are located.

"kotowari-core", "kotowari-source-analysis" and "kotowari-overview" do not obtain input from files, environment variables, Git or HTTP, do not write to standard output or standard error, and have no Tokio or CLI dependencies. core divides IR parsing, sources, test correspondence, mutation outcomes, change conformance, plans and read results into modules by responsibility.

### REQ-core-308: Direction of dependency between analysis and judgement

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A26, docs/decision/records/2026-10-03-public-crate-api.md#A41
- verification: unit

The test correspondence judgement in core receives already-discovered tests and marks, and does not call source analysis. core holds the "Comparison" of change conformance and the computation of identifier values, and the Git retrieval side builds the same type. "Analyzer" analyses source strings with the existing built-in rules and additional rules given as strings, and returns tests, marks, surfaces and findings. The existing meaning of the rules and of unsupported languages is kept.

### REQ-core-309: Internal types are not mixed into the public contract

- kind: prohibition
- source: docs/decision/records/2026-10-03-public-crate-api.md#A13, docs/decision/records/2026-10-03-public-crate-api.md#A16, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A40
- verification: review
- how_to_verify: Check the public signatures in rustdoc and external usage examples of the lower crates. Confirm by compiling that a re-exported type can be passed as the same type as the original.

Public signatures do not contain internal types of the Markdown parser or of the source search engine. The APIs of the published lower crates are also subject to compatibility management, and implementation details are kept private. "kotowari" re-exports the lower input and result types it uses at its entry points as the same types, without duplicating them.

## Decision tables

### TBL-core-040: Packages and normal dependencies

- source: docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A35, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A41, docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A44, docs/decision/records/2026-10-04-overview-on-public-api.md#A1, docs/decision/records/2026-10-04-overview-on-public-api.md#A4

| Package | Purpose of use | Normal dependencies within the workspace |
|---|---|---|
| kotowari | Reads a project and calls typed operations | kotowari-core, kotowari-source-analysis, kotowari-overview |
| kotowari-core | In-memory IR parsing, checking and assembly of results | kotowari-markdown-schema |
| kotowari-source-analysis | Discovers tests, marks and surfaces from in-memory sources | kotowari-core |
| kotowari-overview | Checks in-memory overview data and builds the reference table, stale sections and rendering input | kotowari-core, kotowari-markdown-schema, kotowari-markdown-view |
| kotowari-cli | The arguments, display and exit codes of the kotowari command, and serve, which delivers the overview | kotowari |
| kotowari-markdown-schema | In-memory Markdown schema validation and extraction | None |
| kotowari-markdown-schema-io | Loading documents and schemas, and checking and extraction | kotowari-markdown-schema |
| kotowari-markdown-view | Builds overview pages from in-memory rendering input | None |
| kotowari-mds | The arguments, display and exit codes of the kotowari-mds command | kotowari-markdown-schema-io, kotowari-markdown-schema |

## Examples

```gherkin
@id=EX-core-480 @about=REQ-core-306,REQ-core-307,REQ-core-309 @source=docs/decision/records/2026-10-03-public-crate-api.md#A11,docs/decision/records/2026-10-03-public-crate-api.md#A13,docs/decision/records/2026-10-03-public-crate-api.md#A25,docs/decision/records/2026-10-03-public-crate-api.md#A35,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: IR-only use does not bring in dependencies for environment operations
  Given an external application depends only on kotowari-core to handle in-memory IR
  When the usage examples of the public API and the normal dependency graph are checked
  Then ast-grep, Tokio and CLI or HTTP dependencies are not needed
  And the user does not need to handle internal syntax tree types

@id=EX-core-481 @about=REQ-core-308 @source=docs/decision/records/2026-10-03-public-crate-api.md#A26,docs/decision/records/2026-10-03-public-crate-api.md#A41
Scenario: Correspondence is judged from already-discovered tests alone
  Given the IR and already-discovered tests and marks are in memory
  When core judges test correspondence
  Then it returns the judgement without requiring source files or source analysis

@id=EX-core-482 @about=REQ-core-306,REQ-core-307,REQ-core-309 @source=docs/decision/records/2026-10-03-public-crate-api.md#A13,docs/decision/records/2026-10-03-public-crate-api.md#A35,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: A candidate that exposes internal dependencies does not pass the boundary verification
  Given there is a candidate in which core runs Git or a public signature returns an internal type of the search engine
  When the direction of dependencies and the public API are checked
  Then that candidate is not judged to conform to the crate boundaries
```
