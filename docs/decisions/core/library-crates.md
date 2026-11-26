# Crate boundaries of the library

English | [日本語](library-crates.ja.md)

This document covers how the published Rust libraries and the CLI packages are divided, the responsibilities and direction of dependency of each, and which entry point users choose. The entry points awaited from Tokio are decided in [Optional Tokio support](library-async.md), and that the Markdown schema library reads no files is decided in the topic "Library entry points", in [The library reads no files and does not fetch schemas at URLs](../schema/library.md#the-library-reads-no-files-and-does-not-fetch-schemas-at-urls). The entry points and results of the operations that use a project are decided in [Rust API for using a project](library-api.md), and how in-memory input is held in [In-memory input and already-read results](library-inputs.md).

## Packages are split by purpose, and users choose a package by purpose

Status: implemented

### Problem

We want kotowari to be usable both for calling checks and queries from another Rust application and for validating with Markdown schemas independently of kotowari. Until now the responsibilities were gathered inside a few crates, and users could not choose the crate and API that fit their purpose. We want library users to be able to start from `cargo add kotowari`.

Users who use only in-memory checks are not required to take on environment operations such as file search, Git and HTTP, or dependencies for the CLI. This is also to bind the direction of dependency with the compiler. The resulting increase in the number of crates and in release effort was accepted.

### Decision

There are the following nine packages.

| Package | Purpose of use |
|---|---|
| kotowari | Reads a project and calls typed operations |
| kotowari-core | In-memory parsing and checking of the specification, and assembly of results |
| kotowari-source-analysis | Finds tests, markers and surfaces from in-memory source strings |
| kotowari-overview | Checks in-memory overview source data, and builds the reference table, stale sections and rendering input |
| kotowari-cli | The arguments, display and exit codes of the kotowari command, and serve, which delivers the overview |
| kotowari-markdown-schema | In-memory Markdown schema validation and extraction |
| kotowari-markdown-schema-io | Loading of documents and schemas, and checking and extraction |
| kotowari-markdown-view | Builds overview pages from in-memory rendering input |
| kotowari-mds | The arguments, display and exit codes of the kotowari-mds command |

The name of the library package is kotowari, and the name of the CLI package is kotowari-cli. The name of the command that is run stays kotowari.

Computation is divided into kotowari-core and kotowari-source-analysis. The dependency for analysing sources (ast-grep) is confined to kotowari-source-analysis, and is not required of users who handle only specification documents. The inside of kotowari-core is divided into modules by responsibility. Computation for which the benefit of a separate crate is not yet clear is divided by module, not by crate.

The direction of dependency is one way: from the CLI to the library, and from the library to computation.

- The CLI and external applications use the same high-level operations. kotowari-cli uses kotowari, and kotowari-mds uses kotowari-markdown-schema-io.
- kotowari depends on kotowari-core, kotowari-source-analysis and kotowari-overview.
- kotowari-overview depends on kotowari-core, kotowari-markdown-schema and kotowari-markdown-view. kotowari-core does not know about the overview.
- kotowari-markdown-view depends on no package inside the workspace, because it is a general-purpose rendering engine that does not know about kotowari.
- kotowari-markdown-schema-io adds loading from files and URLs on top of kotowari-markdown-schema.

kotowari-markdown-view and kotowari-overview are also made publishable, like the other packages, because the dependencies of a publishable kotowari must be publishable.

### Rejected options

- Make kotowari-markdown-view and kotowari-overview internal crates that are not published. kotowari could no longer be published.

### Confirmations

- C-core-libcrates-1 (by reading) The packages of the workspace are the nine in the table, the normal dependencies inside the workspace follow the direction of dependency written above, and no library depends on a CLI package
- C-core-libcrates-2 (by reading) For each library in the table, `examples/` has a usage example that calls, from outside, the entry point for its purpose of use

## The computation crates do not operate on the environment and have no Tokio or CLI dependencies

Status: implemented

### Problem

We do not want to require users who use only in-memory computation and checks to take on dependencies for file search, Git operations and HTTP fetches, or dependencies for the CLI. If computation is kept apart from environment operations, the direction of dependency can be bound with the compiler.

### Decision

kotowari-core, kotowari-source-analysis and kotowari-overview do not take input from files, environment variables, Git or HTTP. They do not operate on files, the network or environment variables. These crates have no dependencies for Tokio or the CLI.

Reading and writing are the responsibility of kotowari's library. Reading the overview source data and writing out the overview are also done by kotowari's library, and serve, which delivers the overview over HTTP, is done by kotowari-cli. Placing serve in the CLI keeps the dependency on an HTTP server out of library users' dependencies.

### Rejected options

- Place reading and writing of the overview source data, and even the HTTP server, in kotowari-overview. This does not fit with not placing environment operations in the computation crates, and brings the dependency on an HTTP server into library users' dependencies.

### Confirmations

- C-core-libcrates-3 (by reading) The code of kotowari-core, kotowari-source-analysis and kotowari-overview does not operate on files, environment variables, Git, HTTP or the network
- C-core-libcrates-4 (by reading) The normal dependencies of kotowari-core, kotowari-source-analysis and kotowari-overview contain no crates for Tokio, for the CLI or for HTTP, and ast-grep is not in the dependency tree of an external application that depends only on kotowari-core

## The judgement of test correspondence receives the found tests and markers, and does not analyse sources

Status: implemented

### Problem

We want to separate the direction of dependency between source analysis and judgements on the specification. If the judgement of test correspondence called source analysis, users who handle only specification documents would also need the dependency for analysing sources.

### Decision

The judgement of test correspondence in kotowari-core receives the found tests and markers and judges from them, and does not call source analysis.

Finding tests, markers and surfaces from source strings is done by the `Analyzer` of kotowari-source-analysis, which returns what it found and the analysis findings. `Analyzer` does not go to fetch files, and does not judge correspondence.

### Confirmations

- C-core-libcrates-5 The checks of kotowari-core judge test correspondence from only the found tests and markers passed to them, requiring neither source files nor source analysis
- C-core-libcrates-6 `Analyzer` returns tests, markers and analysis findings even from a source string given a path with no file

## Public signatures do not expose internal implementation types

Status: implemented

### Problem

We want to keep external users from relying on internal implementation types. If the syntax tree types of the Markdown parsing tool or the types of the internal search engine appeared in public signatures, just replacing that tool would break the public API.

### Decision

Public signatures do not contain the syntax tree types of the Markdown parsing tool or the types of the engine that searches sources. Which types specifically are not exposed by the Markdown schema library is decided in the topic "Library entry points", in [Internal implementation types do not appear in public signatures](../schema/library.md#internal-implementation-types-do-not-appear-in-public-signatures).

The APIs of the lower crates published as dependencies are also managed for compatibility as public APIs. Implementation details are not made public.

### Confirmations

- C-core-libcrates-7 (by reading) The public signatures of the published crates do not contain the syntax tree types of the Markdown parsing tool or the types of the engine that searches sources (ast-grep)

## Whether something is public is decided by whether external users rely on it, and kotowari re-exports, as the same types, only the lower types reachable from its entry points

Status: implemented

### Problem

Since the lower crates are also managed for compatibility, leaving public the items that external users do not rely on makes them that much harder to change. On the other hand, deciding by whether the workspace uses them would remove even the openings written in the specification as entry points.

We want kotowari users to get the types used at the entry points just by depending on kotowari.

### Decision

Whether an element of the API of a published crate is public is decided not by whether the workspace uses it but by whether external users rely on it as a promise. The following are not made public.

- Functions that construct, from outside, result types meant to be read-only
- Parameters and fields that are received but not read
- Intermediate parts that external callers do not need

What is written in the specification as an entry point, natural openings for reading, and the re-exports of types reachable from kotowari's results are kept.

kotowari re-exports the lower input types and result types used at its entry points as the same types, without duplicating them. Lower types not reachable from the inputs and results of the entry points are not re-exported. Users who check in memory take the input types for that from kotowari-core.

### Rejected options

- Narrow every public item not used from outside its crate. This would change even the openings written in the specification as entry points, and remove openings external users rely on.
- Keep re-exports of types not reachable from the entry points. This would remain wider than the rule of re-exporting the types used at the entry points.
- Make, on kotowari's side, a separate result type that wraps core's result. Counting would be in two places.

### Confirmations

- C-core-libcrates-8 (by reading) Functions that construct, from outside, result types meant to be read-only, parameters and fields that are received but not read, and intermediate parts that external callers do not need are not public
- C-core-libcrates-9 (by reading) Every lower type that kotowari re-exports is reachable from the inputs or results of kotowari's entry points
- C-core-libcrates-10 A type kotowari re-exports can be passed as the same type of the lower crate
