# Paths of library input

English | [日本語](library-paths.ja.md)

Covers the logical paths of in-memory input, and the working start location and cache base of the I/O entry points.

## Requirements

### REQ-core-322: The base and identity of logical paths

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A51, docs/decision/records/2026-10-03-public-crate-api.md#A53, docs/decision/records/2026-10-03-public-crate-api.md#A54, docs/decision/records/2026-10-06-changes-rethink.md#A10
- verification: unit

A logical path of in-memory input is a path relative to the project base, and the existing separator characters, a leading "./", an intermediate "/./", repeated separators and a trailing separator are normalized. Parent-relative paths that the existing rules allow and the ".." components that remain are kept. When the result is empty or an absolute path, when the same group has duplicate paths after normalization, or when the original bodies of the same path in different groups differ, it is "InvalidInput". The analysis results of tests and surfaces also keep the full text of the SourceText used for analysis, and the bodies are compared. Sharing of identical content between groups is judged by the existing overlap rules. Paths are not normalized through the file system, and symbolic links are not resolved. Display and source matching use the same project-relative path.

### REQ-core-323: The I/O locations are fixed

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A44, docs/decision/records/2026-10-03-public-crate-api.md#A52
- verification: unit

The working start location and an explicitly given cache base of "ProjectOptions" and "LoaderOptions" receive absolute paths, and a relative path returns "InvalidInput". A relative path of the configuration file is resolved from the working start location. When the cache base is omitted, the existing base search is performed from the working start location. A later change of the current directory does not change the base of resolution.

## Examples

```gherkin
@id=EX-core-498 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A51
Scenario: Duplicate paths written differently are rejected
  Given the same group has the same logical path differing only by a leading ./
  When the in-memory input is built
  Then InvalidInput is returned as a duplicate after normalization

@id=EX-core-499 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A51
Scenario: In-memory documents can be handled even with file names that do not exist
  Given there are a valid project-relative path and a string, and no corresponding file exists
  When the in-memory input is built
  Then the logical path and contents are accepted without looking for the file

@id=EX-core-500 @about=REQ-core-323 @source=docs/decision/records/2026-10-03-public-crate-api.md#A52
Scenario: A relative working location is not resolved implicitly
  Given a relative path is passed as the working start location
  When the options of an I/O entry point are built
  Then InvalidInput is returned

@id=EX-core-501 @about=REQ-core-323 @source=docs/decision/records/2026-10-03-public-crate-api.md#A52
Scenario: Changing the current directory after the call does not change the base
  Given there is an entry point given an absolute working start location
  When the caller changes the current directory and performs an operation
  Then paths are resolved with the given working start location as the base

@id=EX-core-502 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A54
Scenario: Parent-relative IR is handled as before
  Given the IR location is "../spec/ir" and the logical path is "../spec/ir/topic.md"
  When the in-memory input is parsed
  Then the path is not rejected for being parent-relative, and is handled with the same base

@id=EX-core-503 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A51,docs/decision/records/2026-10-03-public-crate-api.md#A53
Scenario: A mismatch in original contents is detected even when discovery results are the same
  Given the analysis results of tests and surfaces have the same logical path, and their original bodies differ
  When the in-memory input is built
  Then InvalidInput is returned regardless of whether the discovery results and findings agree
```
