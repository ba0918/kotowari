# Using kotowari from a Rust library

English | [日本語](public-crate-api.ja.md)

This page explains how to use specification checks and Markdown value extraction without launching the executable.

## Choosing a crate

<!-- @kotowari[REQ-core-306:713197d2, REQ-core-308:b5d19f8b, TBL-core-040:4f46993e] -->

Use `kotowari` for operations on a repository, and `kotowari-core` for in-memory computation only.
`kotowari-source-analysis` discovers tests and surfaces from strings and returns core types that include the original strings and diagnostics. It does not fetch files or match them against the IR.
`kotowari-overview` checks in-memory overview data and builds the reference table, the stale sections and the rendering input. `kotowari-markdown-view` builds overview pages in memory from the rendering input, and depends on no other package in the workspace.
Use `kotowari-markdown-schema` for schema computation and `kotowari-markdown-schema-io` for loading.
The CLI packages are `kotowari-cli` and `kotowari-mds`. `serve`, which serves the overview, lives in `kotowari-cli`, not in the libraries.

## Loading and the lifetime of results

<!-- @kotowari[REQ-core-310:ba432526, REQ-core-316:676d9736, TBL-core-041:3ffaa875] -->

State the starting point explicitly with `Project::new(ProjectOptions::new(absolute_start))`.
`check`, `list`, `query`, `status`, `plan`, `mutants` and `changes` return typed results. `check` and `status` include the findings on overview data and the `overview` group.
`overview_prepare` checks and renders the overview data without writing files. `write()` on the returned result writes under `.kotowari/cache/overview/`. `overview_build` does both in sequence and returns the lists of files written and files removed, and the number of files not written. `AsyncProject` has the same two.
Read the findings of a completed run with `findings()` on the result; unreadable files and configuration errors are told apart by `Error` and `ErrorKind`.
The results of `read()` and `inspect()` can be kept and reused, and are not updated automatically when files change later. To read again, call Project again.
`plan` and `mutants` do not read the IR.

## In-memory input and paths

<!-- @kotowari[REQ-core-314:baa022e4, REQ-core-315:6a916b03, REQ-core-322:747061bc, REQ-core-323:77b70fdf] -->

`SourceText` holds a relative logical path and the original string. It normalizes separators and dot components, and rejects absolute and empty paths. Building an input group also rejects duplicates within the same group. Parent-relative paths are allowed.
When the same path is used in another enabled input group, the strings must match too.
`None` for an input means not provided, and `Some(vec![])` means provided but empty. Groups enabled in the configuration are required, and disabled groups are ignored.
`ir::parse` also returns items with invalid or missing IDs, together with the positions and findings it obtained. This differs from what the CLI lists.
The starting points of Project and SchemaLoader, and the Loader's cache base, are absolute paths. If the caller's current directory changes after construction, the resolution base does not change.

## Schemas and value extraction

<!-- @kotowari[REQ-schema-068:a2ca7ddc, REQ-schema-069:61fa2bf9, REQ-schema-070:ff5208d1] -->

`Schema::parse` validates syntax and meaning, and the result is reused with the result of `Document::parse`.
`extract_validated` returns values only on success. To read values even when there are findings, use `extract_partial` and look at both the values and the findings.
For typed partial extraction, use `extract_typed_partial`. Its format is separate from the raw AST JSON.
The defaults of `ValidationOptions` respect the schema's open setting. Relaxations you state explicitly can also apply to schemas that are not open.
SchemaLoader's `load` returns the schema and document as a pair, so you can validate and extract without loading again.

## Awaiting from Tokio

<!-- @kotowari[REQ-core-318:c7f6eb87, REQ-core-319:3c721e0f, REQ-core-320:1b7a7dfe, REQ-core-321:5a28ec39, REQ-schema-075:a82f0403] -->

In the two high-level libraries, enabling the `tokio` feature (disabled by default) gives you `AsyncProject` and `AsyncSchemaLoader`.
They await on the caller's Tokio runtime and create no internal runtime. A missing runtime and task failures are also returned as types.
`AsyncOptions` has a default limit of 1, which you can change with a `NonZeroUsize`. Cloned objects share the slots.
Dropping a Future that is waiting for a slot means its work is never submitted. Work already submitted holds its slot until it completes, even if you stop waiting. Cancellation of Git, HTTP, computation or cache writes is not guaranteed.

Each crate's `examples/` has calling examples. To run from the distributed packages, see [Package validation](package-validation.md).
