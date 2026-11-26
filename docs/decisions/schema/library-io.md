# The library that loads schemas and documents

English | [日本語](library-io.ja.md)

This covers the entry points of the separate crate `kotowari-markdown-schema-io`, which reads schemas and documents from files and URLs, and the reuse, as they are, of the existing CLI's loading rules. The side of the pure library that reads no files is decided in [the "Library entry points" topic](library.md), and validation and extraction after loading in [the "Validation and value extraction" topic](library-extraction.md).

## Loading of schemas and documents is placed in a separate crate shared by the CLI and outside applications

Status: implemented

### Problem

The dependencies of users who validate from strings should not grow ([The library reads no files and does not fetch schemas at URLs](library.md#the-library-reads-no-files-and-does-not-fetch-schemas-at-urls)). On the other hand, the same loading the CLI does should be reusable in outside applications as well.

Even when the internals are rebuilt, the existing CLI's contract for commands, arguments, exit codes, and output, and the way schemas are accepted, are not broken. This is so as not to break the input and processing of CLI users.

The entry point for loading must be able to handle several projects in the same process. If the entry point rewrote the process's current directory, the location read by other processing in the same process would change too.

### Decision

The processing that reads schemas and documents from files and URLs is placed in `kotowari-markdown-schema-io`. The existing CLI's rules for file search, URL resolution, caching, fetch limits, and timeouts are moved into this crate as they are (the rules are decided in the ["CLI and how results are output"](cli.md#a-directory-check-checks-only-documents-with-schema-and-does-not-follow-directories-starting-with--or-symbolic-links) and ["Schema resolution"](schema-resolution.md) topics). The location and lifetime of the cache also stay as they are. Both the mds CLI and outside applications use this crate.

Loading through `SchemaLoader` does not change the process's current directory.

The entry point is `SchemaLoader::new`, which takes, through `LoaderOptions`, the starting location of the work and an optional cache base. The handling of the starting location of the work and the cache base is shared with kotowari's library entry points, and is decided in the "Paths of library input" topic.

`load` returns the pair of the loaded schema and document, and that pair can be used repeatedly for validation and extraction. `check` returns the result of checking one file or a directory, and `extract_validated` and `extract_partial` return their respective extraction results (the difference between the two is decided in [The normal entry point for value extraction returns values only from a document with no violation](library-extraction.md#the-normal-entry-point-for-value-extraction-returns-values-only-from-a-document-with-no-violation)).

### Confirmations

- C-schema-io-1 Validation and extraction can be repeated with the pair of schema and document loaded by `load` of a `SchemaLoader`, and passing the same document path to `check`, `extract_validated`, and `extract_partial` of the `SchemaLoader` returns results for the same document
- C-schema-io-2 The documents that `check` of a `SchemaLoader` selects from a directory are the same as the documents that mds's `check` selects from the same directory
- C-schema-io-8 Calling `load` of a `SchemaLoader` does not change the process's current directory

## An entry point that reads only a document is published, and mds's ast also uses it

Status: implemented

### Problem

The rules for reading a document (the handling of a leading BOM and the judgment of whether it is UTF-8) should not be held in two places. Adding an entry point is an addition to the public API, not a breaking change.

### Decision

`read_document` reads a document the same way `load` does, and returns the string with the leading UTF-8 BOM removed. It does not resolve the schema. A document that cannot be read as UTF-8 is a read failure.

mds's `ast` reads documents through this entry point.

### Confirmations

- C-schema-io-3 `read_document` returns the string with the leading BOM removed, and returns a read failure for a document that is not UTF-8
- C-schema-io-4 (by reading) mds's `ast` reads documents with `read_document`

## Loading failures are returned separately from the document's findings

Status: implemented

### Problem

Callers should be able to handle separately the result of a check that found problems in a document and a failure in which loading did not succeed and the check could not be completed.

### Decision

Findings found by completing the check of a document are returned as the result of the check. A failure that prevents completing the check (for example, the schema is not found, the schema is broken, or the document cannot be read) is returned as the `Err` of a `Result`, in a form whose kind can be told apart. The failure type implements `std::error::Error` and `Display`.

### Confirmations

- C-schema-io-5 In each of the cases where the schema is not found, the schema is broken, or the document cannot be read, `load` and `check` return a failure whose kind can be told apart, not the result of a check

## A call that selects the Tokio feature also gets the same results as the synchronous one

Status: implemented

### Problem

It should be usable from asynchronous applications as well. In that case, the synchronous validation and the existing loading processing should be reused.

Users are not given more packages to choose for asynchronous use, and users who use it synchronously are not required to depend on Tokio.

### Decision

`kotowari-markdown-schema-io` provides `AsyncSchemaLoader` behind the `tokio` feature, which is disabled by default. `AsyncSchemaLoader` reuses the synchronous loading processing, and for the same input returns the same values, findings, and kinds of failure as the synchronous `SchemaLoader`. `kotowari-markdown-schema`, which reads no files, does not depend on Tokio. The CLI uses the synchronous API.

The upper limit on how many run at the same time, the execution slots among cloned objects, the fact that processing already started does not stop when waiting is cancelled, and which runtime it runs on are shared with kotowari's asynchronous entry points, and are decided in the "Optional Tokio support" topic.

### Rejected options

- Provide only the synchronous API and an example of calling it from an asynchronous application. The user chose the option of going as far as Tokio support in which high-level operations can be awaited.
- Rebuild the waiting for HTTP fetches with asynchronous I/O, and also add a mechanism that stops processing already started partway. The user chose the option of reusing the synchronous processing and making it callable from asynchronous applications.

### Confirmations

- C-schema-io-6 When the `tokio` feature is enabled, `load`, `check`, `extract_validated`, and `extract_partial` of `AsyncSchemaLoader` return, on Tokio, the same values, findings, and kinds of failure as the synchronous `SchemaLoader`
- C-schema-io-7 (by reading) The `tokio` feature is disabled by default, and `kotowari-markdown-schema` does not depend on Tokio
