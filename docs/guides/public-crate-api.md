# Using kotowari from a Rust library

English | [日本語](public-crate-api.ja.md)

This page explains how to use specification checks and Markdown value extraction without launching the executable.

## Choosing a crate

<!-- @kotowari[docs/decisions/core/library-crates.md#packages-are-split-by-purpose-and-users-choose-a-package-by-purpose:9a903158, docs/decisions/core/library-crates.md#the-judgement-of-test-correspondence-receives-the-found-tests-and-markers-and-does-not-analyse-sources:66da2e57] -->

Use `kotowari` for operations on a repository, and `kotowari-core` for in-memory computation only.
`kotowari-source-analysis` discovers tests and surfaces from strings and returns core types that include the original strings and diagnostics. It does not fetch files or match them against the IR.
`kotowari-overview` checks in-memory overview data and builds the reference table, the stale sections and the rendering input. `kotowari-markdown-view` builds overview pages in memory from the rendering input, and depends on no other package in the workspace.
Use `kotowari-markdown-schema` for schema computation and `kotowari-markdown-schema-io` for loading.
The CLI packages are `kotowari-cli` and `kotowari-mds`. `serve`, which serves the overview, lives in `kotowari-cli`, not in the libraries.

## Loading and the lifetime of results

<!-- @kotowari[docs/decisions/core/library-api.md#another-rust-application-calls-kotowaris-operations-as-methods-of-project-with-typed-inputs:c11cbaab, docs/decisions/core/library-inputs.md#already-read-results-can-be-reused-and-do-not-change-until-explicitly-read-again:109328a9] -->

State the starting point explicitly with `Project::new(ProjectOptions::new(absolute_start))`.
`check`, `list`, `query`, `status`, `plan` and `mutants` return typed results. `check` and `status` include the findings on overview data and the `overview` group.
`overview_prepare` checks and renders the overview data without writing files. `write()` on the returned result writes under `.kotowari/cache/overview/`. `overview_build` does both in sequence and returns the lists of files written and files removed, and the number of files not written. `AsyncProject` has the same two.
Read the findings of a completed run with `findings()` on the result; unreadable files and configuration errors are told apart by `Error` and `ErrorKind`.
The results of `read()` and `inspect()` can be kept and reused, and are not updated automatically when files change later. To read again, call Project again.
`plan` and `mutants` do not read the IR.

## In-memory input and paths

<!-- @kotowari[docs/decisions/core/library-inputs.md#a-public-entry-point-is-provided-that-parses-the-string-of-a-decision-document-and-returns-read-only-the-decisions-and-confirmations-obtained:6c063685, docs/decisions/core/library-inputs.md#if-a-required-input-group-is-not-provided-inputmissing-is-returned-and-an-empty-group-is-checked-by-the-ordinary-rules:c81b1826, docs/decisions/core/library-paths.md#in-memory-input-is-received-as-pairs-of-content-and-logical-path-and-no-file-is-looked-for:93ddcc73, docs/decisions/core/library-paths.md#the-entry-points-that-read-files-receive-the-working-start-location-and-resolve-from-it-by-the-same-rules-as-the-existing-cli:2fb622cf] -->

`SourceText` holds a relative logical path and the original string. It normalizes separators and dot components, and rejects absolute and empty paths. Building an input group also rejects duplicates within the same group. Parent-relative paths are allowed.
When the same path is used in another enabled input group, the strings must match too.
`None` for an input means not provided, and `Some(vec![])` means provided but empty. Groups enabled in the configuration are required, and disabled groups are ignored.
`ir::parse` also returns items with invalid or missing IDs, together with the positions and findings it obtained. This differs from what the CLI lists.
The starting points of Project and SchemaLoader, and the Loader's cache base, are absolute paths. If the caller's current directory changes after construction, the resolution base does not change.

## Schemas and value extraction

<!-- @kotowari[docs/decisions/schema/library-extraction.md#a-schema-is-returned-only-after-it-passes-semantic-validation-and-a-parsed-schema-can-be-reused:ae534266, docs/decisions/schema/library-extraction.md#the-normal-entry-point-for-value-extraction-returns-values-only-from-a-document-with-no-violation:9379fe8e, docs/decisions/schema/library-extraction.md#validation-in-the-library-does-not-cancel-the-schemas-open:142c086d] -->

`Schema::parse` validates syntax and meaning, and the result is reused with the result of `Document::parse`.
`extract_validated` returns values only on success. To read values even when there are findings, use `extract_partial` and look at both the values and the findings.
For typed partial extraction, use `extract_typed_partial`. Its format is separate from the raw AST JSON.
The defaults of `ValidationOptions` respect the schema's open setting. Relaxations you state explicitly can also apply to schemas that are not open.
SchemaLoader's `load` returns the schema and document as a pair, so you can validate and extract without loading again.

## Awaiting from Tokio

<!-- @kotowari[docs/decisions/core/library-async.md#the-high-level-operations-can-be-awaited-from-tokio-through-a-tokio-feature-disabled-by-default:c8aac0f2, docs/decisions/core/library-async.md#the-number-of-concurrent-executions-is-limited-per-asynchronous-entry-point-object-and-clones-share-execution-slots:15c39651, docs/decisions/core/library-async.md#stopping-waiting-does-not-stop-processing-that-has-started-and-it-holds-its-execution-slot-until-it-finishes:1608454e, docs/decisions/core/library-async.md#awaiting-outside-a-runtime-or-a-runtime-that-stops-returns-a-failure-whose-kind-can-be-told-apart-without-panicking:91b15c91, docs/decisions/schema/library-io.md#a-call-that-selects-the-tokio-feature-also-gets-the-same-results-as-the-synchronous-one:3539145a] -->

In the two high-level libraries, enabling the `tokio` feature (disabled by default) gives you `AsyncProject` and `AsyncSchemaLoader`.
They await on the caller's Tokio runtime and create no internal runtime. A missing runtime and task failures are also returned as types.
`AsyncOptions` has a default limit of 1, which you can change with a `NonZeroUsize`. Cloned objects share the slots.
Dropping a Future that is waiting for a slot means its work is never submitted. Work already submitted holds its slot until it completes, even if you stop waiting. Cancellation of HTTP, computation or cache writes is not guaranteed.

Each crate's `examples/` has calling examples. To run from the distributed packages, see [Package validation](package-validation.md).
