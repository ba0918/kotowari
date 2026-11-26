# Rust API for using a project

English | [日本語](library-api.ja.md)

This document covers the entry points through which another Rust application calls kotowari's operations, how check results and failures are separated, and the relation to the CLI. Input passed in memory and the reuse of already-read results are decided in [In-memory input and already-read results](library-inputs.md), and the working start location and the paths of in-memory input in [Paths of library input](library-paths.md). How the crates are divided and which types are re-exported are decided in [Crate boundaries of the library](library-crates.md), and Tokio support in [Optional Tokio support](library-async.md).

## Another Rust application calls kotowari's operations as methods of Project with typed inputs

Status: implemented

### Problem

We want another Rust application to be able to call kotowari's existing functions without going through the CLI. Instead of assembling and passing CLI arguments, we want to pass, as types, the inputs each operation needs. Alongside the use of checking without saving files, the use of checking an existing project by specifying a directory is also supported.

For the overview too, the place where it is integrated into check is the library, not the CLI. The CLI is a thin layer on top of Project, so the integration of checks is done in Project.

serve, which delivers the overview, proceeds in the order of checking the source data, securing the port, and writing. If the operation that builds the overview wrote everything in one call, the CLI could only take the port before checking or write before taking the port, and would write the overview even when the port cannot be used.

### Decision

The entry point is `Project::new`, which receives, through `ProjectOptions`, the project's working start location and an optional configuration file to use (how the location is treated is decided in [The entry points that read files receive the working start location, and resolve from it by the same rules as the existing CLI](library-paths.md#the-entry-points-that-read-files-receive-the-working-start-location-and-resolve-from-it-by-the-same-rules-as-the-existing-cli); the working start location is received as an absolute path, and `Project` does not change the current directory of the process). `Project` has the following operations as methods.

| Method | What it does | What it returns |
|---|---|---|
| `check` | Checks the project according to the configuration. Includes the findings on the overview's source data and the "overview" group | Findings and counts |
| `list` | Lists what could be read from the specification documents | A typed list |
| `query` | Queries the given ID | A typed result including the body and what points at it |
| `status` | Checks the same scope as `check` | Counts and the completion state |
| `plan` | Checks the given plan | Findings and counts |
| `mutants` | Checks the given mutation testing outcomes and the equivalence information | Findings and mutation counts |
| `overview_prepare` | Checks and renders the overview's source data | The rendering result before writing |
| `overview_build` | Following `overview_prepare`, writes the rendering result to the overview location | The lists of written files and removed files, and the number of files not written |

`check`, `status`, and `inspect`, which returns already-read check results, include the overview findings and the "overview" group.

`overview_prepare` finishes checking and rendering in memory and writes nothing. Only `overview_build` and the `write` of the prepared result write the overview. The overview location and the rules for writing, deleting and not writing are decided in [overview build draws everything every time, writes only the files whose contents differ, and deletes files no longer produced](overview-commands.md#overview-build-draws-everything-every-time-writes-only-the-files-whose-contents-differ-and-deletes-files-no-longer-produced). The asynchronous entry point ([`AsyncProject`](library-async.md#the-high-level-operations-can-be-awaited-from-tokio-through-a-tokio-feature-disabled-by-default)) has the same operations. serve, which delivers the overview, is not placed in the library.

That `plan` and `mutants` do not read the specification documents is decided in [An operation reads only the input that operation uses](library-inputs.md#an-operation-reads-only-the-input-that-operation-uses).

### Rejected options

- Place the overview build only in the synchronous `Project`. The form in which the asynchronous entry point corresponds to the synchronous high-level operations breaks.
- Do not expose the overview operations in the library.

### Confirmations

- C-core-libapi-1 The `check`, `list`, `query`, `status`, `plan` and `mutants` of `Project` return typed results from typed inputs without going through the CLI
- C-core-libapi-2 In a project with overview source data, the results of `check`, `status` and `inspect` contain the overview findings, and the results of `check` and `status` show the "overview" group
- C-core-libapi-3 `overview_build` writes the overview files and returns the written files, the removed files and the number of files not written. Files that need no rewriting are included in the number of files not written
- C-core-libapi-13 Calling `overview_prepare` does not create the overview location, and it is written when `overview_build` is called

## Findings found by a check and failures that kept the check from finishing are returned separately

Status: implemented

### Problem

We want the caller to be able to handle results without parsing strings meant for display. We also want the caller to be able to handle separately a result in which problems were found in the target of the check, and a failure in which the check could not be completed, such as a read failure.

### Decision

When a check finishes, a result holding the findings on the documents is returned. A finding holds, read-only, its kind, severity, path and, when there is one, its line. The library does not display results; how they are displayed is decided by the caller.

When a check cannot be run, the `Err` of `Result` is returned. The kind of a failure can be told apart by a program, and failures implement `std::error::Error` and `Display`. Examples of failure kinds are invalid input (`InvalidInput`), read failure (`ReadFailure`) and an unknown query ID (`UnknownQuery`). The case where a required input was not passed (`InputMissing`) is decided in [If a required input group is not provided, InputMissing is returned, and an empty group is checked by the ordinary rules](library-inputs.md#if-a-required-input-group-is-not-provided-inputmissing-is-returned-and-an-empty-group-is-checked-by-the-ordinary-rules).

### Confirmations

- C-core-libapi-4 Calling `check` on a project with specification documents that have findings returns not an `Err` but a result holding the findings, and the kinds of the findings can be told apart by type
- C-core-libapi-5 When a check or the writing of the overview cannot be finished (for example, when a file needed for the check cannot be read, when the configuration is not correct, or when the overview location cannot be written), an `Err` whose kind can be told apart is returned, not a result without findings
- C-core-libapi-6 (by reading) kotowari's library writes check results to neither standard output nor standard error

## A given configuration file that is a directory is invalid input; a default location that is a directory is a read failure

Status: implemented

### Problem

When the given configuration file does not exist, it is invalid input. If, when it is a directory, the library tried to read it and failed, it would be a read failure, and the same mistake in specification would split into different failure kinds. Keeping the judgement in one place in the library makes the CLI and the library agree.

On the other hand, when no configuration file is given, the user has not passed an argument, so it is not made an argument error. It is treated as something placed in the default location that cannot be read.

### Decision

When the configuration file given through `ProjectOptions` does not exist or is a directory, it is invalid input (`InvalidInput`). It is not made a read failure (`ReadFailure`). The CLI does not judge this on its own, and maps the library's result.

When `ProjectOptions` gives no configuration file and ".kotowari/config.yaml" in the base directory is a directory, it is a read failure (`ReadFailure`).

### Confirmations

- C-core-libapi-7 Both when the configuration file given through `ProjectOptions` is a directory and when it does not exist, `check` returns an `Err` of the same `InvalidInput` kind, not `ReadFailure`
- C-core-libapi-8 When no configuration file is given and ".kotowari/config.yaml" in the base directory is a directory, `check` returns an `Err` of the `ReadFailure` kind

## The ID can be taken from the failure of a query for an unknown ID

Status: implemented

### Problem

If the CLI took the ID out by stripping a fixed prefix from the text of the library's failure, it would break easily when the wording changes. Adding only a method that returns the ID is not a breaking change, and keeps the CLI output.

### Decision

`kotowari::Error` has `pub fn unknown_id(&self) -> Option<&str>`. Only when the failure kind is `UnknownQuery` does it return the queried ID; for failures of other kinds it returns no ID. The name matches the CLI text (unknown id), so that the name tells what the ID is. The CLI takes the ID out with this method, and the CLI output does not change.

### Confirmations

- C-core-libapi-9 Calling `query` with an ID that exists nowhere returns an `Err` of the `UnknownQuery` kind, and the ID can be taken out from `unknown_id`
- C-core-libapi-10 For failures of other kinds, `unknown_id` returns no ID

## The CLI only maps the library's results onto the existing output and stop reasons

Status: implemented

### Problem

Even if the internals are rebuilt as a library, we do not want to break the inputs and processing of CLI users. We want the CLI and external applications to use the same high-level operations. Until then only the CLI binary was provided, so compatibility of the existing Rust API and the internal structure need not be kept.

### Decision

kotowari-cli is a thin layer on top of kotowari's library. The CLI maps the library's results onto the existing JSON and text output and the stop reasons.

For both the kotowari and mds binaries, the commands, arguments, exit codes, output, the way configuration and schemas are accepted, and the meaning of check results do not change when placed on top of the library. Even if the name of the package installed from source changes, the name of the command that is run does not change (the package names are decided in [Packages are split by purpose, and users choose a package by purpose](library-crates.md#packages-are-split-by-purpose-and-users-choose-a-package-by-purpose)).

### Confirmations

- C-core-libapi-11 For the inputs that the CLI contract tests define, the CLI placed on top of the library returns the expected JSON, text and exit codes as they are
- C-core-libapi-12 (by reading) kotowari-cli does checks and queries through the operations of `Project` of kotowari's library, and only formats their results as JSON or text and maps them onto stop reasons
