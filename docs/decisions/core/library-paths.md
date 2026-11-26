# Paths of library input

English | [日本語](library-paths.ja.md)

This document covers the logical paths of input passed in memory, and the working start location received by the entry points that read files. The input groups and how missing input is treated are decided in [In-memory input and already-read results](library-inputs.md), and the operations of the entry points in [Rust API for using a project](library-api.md).

## In-memory input is received as pairs of content and logical path, and no file is looked for

Status: implemented

### Problem

We want to check generated documents and unsaved strings without saving files. To check several documents, it must be possible to pass, as pairs, each one's content and a logical path that shows which document it is.

Even if the internals are rebuilt as a library, the configuration of a specification location in a parent directory, which the existing CLI accepted, is not broken.

If an empty or absolute logical path were accepted, input for which it is not determined which document in the project it is would enter the check. The reason for rejecting this is not recorded.

### Decision

In-memory input is received as a `SourceText`, a pair of a logical path that represents a position in the project and a string. The file the logical path refers to need not exist, and kotowari does not look for files. The logical path is handled as a string, and is not normalized by the file system.

A logical path is a path relative to the project base. An empty logical path and an absolute logical path are rejected (an absolute path is invalid input, `InvalidInput`).

Logical paths that point at a parent directory (those containing ".."), which the existing CLI accepts, are not rejected and are kept as they are.

### Confirmations

- C-core-libpaths-1 Building a `SourceText` from a logical path with no file and a string does not read any file, and holds that string as its content
- C-core-libpaths-2 A `SourceText` whose logical path starts with ".." is not rejected, and the ".." of the path remains as it is
- C-core-libpaths-5 A `SourceText` cannot be built from an empty logical path or an absolute logical path, and the failure for an absolute path is of the `InvalidInput` kind

## When inputs with the same logical path overlap, they are accepted only when they can be treated as the same file

Status: implemented

### Problem

If the same file could be passed as two paths spelled differently, or as two inputs with different contents, the same file would be treated as different things. Counts and findings would be doubled, and it would not be determined which content was checked.

Unless the test and surface analysis results hold the body used for analysis, it cannot be compared, without reading further files, whether their content is the same as that of another input with the same path.

### Decision

If within the same group there are logical paths that become the same after normalizing differences in separator characters, a leading "./", an intermediate "/./", repeated separators and a trailing separator, it is invalid input (`InvalidInput`).

If another group has an input with the same logical path and the contents differ, it is invalid input (`InvalidInput`). If the contents are the same, it is accepted, and overlap between groups is judged by the existing overlap rules ([A check of in-memory input reads no further files, and gives results by the same rules as reading from files](library-inputs.md#a-check-of-in-memory-input-reads-no-further-files-and-gives-results-by-the-same-rules-as-reading-from-files)). The test information and the surface analysis results hold the full text of the `SourceText` used for analysis, and it is this body that is compared. Even if the analysis results and findings are the same, different bodies are not treated as the same content.

When taking input in by reading from files, too, inputs are told apart by the original file. If the same group contains the same original file twice, it is rejected even if the spelling of the displayed path differs. If the body of the same original file differs between reading and checking, or between analysis and taking in, it is rejected.

### Confirmations

- C-core-libpaths-6 When the same group has two logical paths that become the same after normalizing separators, a failure of the `InvalidInput` kind is returned
- C-core-libpaths-7 When another group has an input with the same logical path, a failure of the `InvalidInput` kind is returned if the contents differ, and it is accepted if the contents are the same
- C-core-libpaths-8 Passing to the test information and the surface analysis results `SourceText` values with the same logical path but different bodies returns a failure of the `InvalidInput` kind
- C-core-libpaths-9 When taking input in by reading from files, if the same group has the same original file twice, it is rejected even if the spelling of the displayed path differs
- C-core-libpaths-10 When taking input in by reading from files, if the body of the same original file differs between reading and checking, or between analysis and taking in, it is rejected

## The entry points that read files receive the working start location, and resolve from it by the same rules as the existing CLI

Status: implemented

### Problem

We want external Rust applications to be able to check an existing project by specifying a directory. In doing so, the meaning of how the existing configuration is accepted and how relative paths are resolved does not change from the CLI. In schema loading too, the file search and cache handling of the CLI are reused as they are.

The working start location is kept apart from the shared state that is the current directory of the process. If a relative working start location were accepted, or an entry point changed the current directory, a different directory would be checked when other processing in the same process changed the current directory.

### Decision

`ProjectOptions` receives the working start location and an optional configuration file to use. `LoaderOptions`, which loads schemas, receives the working start location and an optional cache base (the entry point for schema loading is decided in [Loading of schemas and documents is placed in a separate crate shared by the CLI and outside applications](../schema/library-io.md#loading-of-schemas-and-documents-is-placed-in-a-separate-crate-shared-by-the-cli-and-outside-applications)).

The working start location and an explicitly given cache base are received as absolute paths. A relative path is rejected as invalid input (`InvalidInput`).

A relative path of the configuration file is resolved from the working start location, as in the existing CLI. The way the configuration is searched for and the way relative paths of the documents to load are resolved also stay as in the existing CLI. When the cache base is omitted, it is searched for from the working start location in the same way as the existing CLI.

`Project` and `SchemaLoader` do not change the current directory of the process. Even if the current directory changes after they are created, the base of resolution does not change.

### Confirmations

- C-core-libpaths-3 Passing the configuration file to `ProjectOptions` as a relative path operates with the configuration file resolved from the working start location
- C-core-libpaths-4 A document passed to `SchemaLoader` as a relative path is read resolved from the working start location of `LoaderOptions`
- C-core-libpaths-11 With a relative working start location, both `Project::new` and `SchemaLoader::new` return a failure, and `SchemaLoader::new` also returns a failure when a relative cache base is passed to `LoaderOptions`
- C-core-libpaths-12 Even if the current directory of the process changes after `Project` and `SchemaLoader` are created, they read resolving from the working start location
- C-core-libpaths-13 The operations of `Project` and the loading of `SchemaLoader` do not change the current directory of the process
