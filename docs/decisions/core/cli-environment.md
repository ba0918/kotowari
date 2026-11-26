# Usage display and target environment

English | [日本語](cli-environment.ja.md)

Covers how "--help" and "--version" are handled, the target operating systems, the sorting of unreadable input into a stop or a finding, and the wording of the reason and details of a stop. The exit code on a stop is decided in [The exit code is 0 when there is no error, 1 when there is an error, and 2 when kotowari stops](cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops), the form that joins the wording of the reason, ": " and the details on the first line of standard error in [When kotowari stops, it outputs nothing to standard output and outputs the reason and the details on the first line of standard error](cli.md#when-kotowari-stops-it-outputs-nothing-to-standard-output-and-outputs-the-reason-and-the-details-on-the-first-line-of-standard-error), and the base directory in [The base of relative paths is the first directory with .kotowari/ found by searching upward](base-directory.md#the-base-of-relative-paths-is-the-first-directory-with-kotowari-found-by-searching-upward).

## "--help" and "--version" are taken before the other arguments, and kotowari outputs the usage or the version and ends

Status: implemented

### Problem

How "--help" and "--version" are handled in combination with other arguments and with checks has to be decided. Letting them win over the other arguments and accepting them without "check" were decided from review findings. The reason is not recorded.

### Decision

When kotowari receives "--help" or "--version", it does not look at the other arguments, does not perform checks, outputs the usage or version string to standard output, and ends with exit code 0. "check" may be absent, as in "kotowari --version".

### Confirmations

- C-core-clienv-1 "kotowari --help" and "kotowari --version" output the usage or the version to standard output and end with exit code 0, even without "check"
- C-core-clienv-2 If "--help" or "--version" is present, kotowari ends with exit code 0 even when an argument error (for example an unknown option) is present with it

## The target operating systems are Linux and macOS, and on Windows only the normalization of path separators is promised

Status: implemented

### Problem

On which operating systems kotowari is promised to work has to be decided. The reason is not recorded.

### Decision

kotowari targets Linux and macOS. On Windows, it performs only the normalization that turns the path separator "\\" into "/" ([Paths in configuration values are normalized before comparing and before output](base-directory.md#paths-in-configuration-values-are-normalized-before-comparing-and-before-output)), and promises no other behavior.

### Confirmations

- C-core-clienv-3 (by reading) `cargo test` passes on Linux and macOS

## Unreadable input and broken input become a stop or an error finding, and are not silently skipped

Status: implemented

### Problem

The user judged that kotowari did not meet the minimum level for being provided as a general-purpose tool. When behavior not in the specification was looked into after implementation, there were 53 cases, and most of them came not from the freedom of the form but from silently letting through input outside the contract. Input that was silently skipped was believed to be checked but was not checked.

### Decision

When kotowari meets input that cannot be read, input that is broken, or input that does not match the form of the contract, it always either stops or reports an error finding. It does not silently skip, silently fall back to a default value, silently take the last specification, or silently read only one of two.

Which of the two it does is divided as follows.

| Input | Handling |
|---|---|
| Input that breaks the premise of reading a file or the configuration as a whole (unreadable, not UTF-8, an error in the syntax, type or value of the configuration, an argument error, an error in glob syntax, an error in a mutation testing results file, an error in the syntax of the list of equivalents or of the list of unspecified surfaces) | Stop |
| Input that could be read but departs from the form locally | Report an error finding at that place |

A document with places that depart from the form locally is also not removed from checks that span documents (for example the check for duplicate IDs), and the checks continue with the values that could be taken.

The only input that is not read is what is enumerated as exclusions (what the exclusions are is in the glossary). Skipping anything not in the enumeration is a violation of this decision. When the file that one entry of the mutation testing results or of the list of equivalents points at does not exist, cannot be read, or is not UTF-8, kotowari does not stop, and reports an error or notice finding (decided in [An entry of the list identifies a mutation by the file, the description of the change, and the text of the line](equivalents.md#an-entry-of-the-list-identifies-a-mutation-by-the-file-the-description-of-the-change-and-the-text-of-the-line) and [An entry whose text is gone from the source gets an equivalent_stale notice](equivalents.md#an-entry-whose-text-is-gone-from-the-source-gets-an-equivalent_stale-notice)).

### Confirmations

- C-core-clienv-4 (by reading) The code that reads input has no path that skips input not enumerated as an exclusion without a stop (StopReason) or a finding
- C-core-clienv-5 When a file or directory under a location cannot be read, "kotowari check" does not skip it, and stops with unreadable file as the reason
- C-core-clienv-6 A document with places that depart from the form locally also undergoes checks that span documents (for example the check for duplicate IDs), and the output of "kotowari check --format json" shows both the form error and the error that spans documents

## The reason for a stop is shown with fixed English wording

Status: implemented

### Problem

When kotowari stops, what the reason is is shown on the first line of standard error. kotowari is released as a general-purpose tool, so the output is in English (documents may be in Japanese). When a reason is added, it is made two lowercase English words that correspond to "... error", matching the other reasons.

### Decision

When kotowari stops, the first line of standard error starts with the wording of one of the following reasons.

| Reason | Wording |
|---|---|
| Configuration error | config error |
| Argument error | argument error |
| Unreadable file | unreadable file |
| File that is not UTF-8 | non-UTF-8 file |
| Error in mutation testing results ([A results file whose form does not fit stops with a results error](mutants-input.md#a-results-file-whose-form-does-not-fit-stops-with-a-results-error)) | results error |
| Mapping error ([A mismatch in kotowari's internal mapping stops with mapping error, kept apart from errors in the user's input](#a-mismatch-in-kotowaris-internal-mapping-stops-with-mapping-error-kept-apart-from-errors-in-the-users-input)) | mapping error |
| Error in overview source data | overview error |
| Port error | port error |
| Error in the overview's location | cache error |

When the overview's location (".kotowari/cache/overview/" and above it) cannot be written to or deleted from, it is cache error, kept apart from unreadable file. This is so that not being able to write is not mistaken for not being able to read. When kotowari stops with an overview source data error, a port error or a location error is decided in [If the source data or the table of contents has errors, it writes nothing and stops with overview error](overview-commands.md#if-the-source-data-or-the-table-of-contents-has-errors-it-writes-nothing-and-stops-with-overview-error), [If the port of overview serve cannot be used, it stops with port error without trying other ports](overview-commands.md#if-the-port-of-overview-serve-cannot-be-used-it-stops-with-port-error-without-trying-other-ports) and [If the location or a directory above it is a symbolic link or not a directory, it writes nothing and stops with cache error](overview-commands.md#if-the-location-or-a-directory-above-it-is-a-symbolic-link-or-not-a-directory-it-writes-nothing-and-stops-with-cache-error).

### Rejected options

- Also stop with the existing unreadable file when the overview's location cannot be written to. Stopping on a location that cannot be written to as an unreadable file would be mistaken for a failure to read.

### Confirmations

- C-core-clienv-7 When kotowari stops, the first line of standard error starts with the wording that corresponds to the reason in the table

## The details of a stop show the file the person fixing it should look at, as a path relative to the base directory

Status: implemented

### Problem

The details of a stop should show the path the person fixing it should look at. If the path of the configuration file is shown when what is broken is another file the configuration file points at (the list of equivalents, a rule file), the person fixing it cannot reach the path they should look at. Also, if only the path is shown, it looks the same as other configuration errors, and the file reads as if it were a broken configuration file.

### Decision

The details of a stop are written in English, and when they include a path, it is a path relative to the base directory. The details for each reason are as follows.

| Reason | Details |
|---|---|
| Configuration error | The relative path of the configuration file and a description of the error. If what is broken is the list of equivalents, a rule file of "tests.rules" or a rule file of "surface.rules" that the configuration file points at, the relative path of that file, not of the configuration file, and a description of the error |
| Argument error | The text of the offending argument. When there are no arguments at all, and when there is no first positional argument, "expected command: check, list, mutants, overview, plan, query or status" |
| Unreadable file | The relative path and the operating system's error message |
| File that is not UTF-8 | The relative path |
| Error in mutation testing results | Decided in [A results file whose form does not fit stops with a results error](mutants-input.md#a-results-file-whose-form-does-not-fit-stops-with-a-results-error) |
| Mapping error | The kind of the finding that could not be mapped and the name of the node, or a description of the value that could not be mapped |

The details in the following cases are decided in each topic.

| Case | Where it is decided |
|---|---|
| Overlap between the locations of "guides.files" and "tests.files" | [When the guide and test locations overlap, it is a stop with a configuration error](guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error) |
| Overlap between "overview.files" and "guides.files" or "tests.files" | [If the overview source data overlaps the location of guides or tests, it stops with a configuration error](overview-data.md#if-the-overview-source-data-overlaps-the-location-of-guides-or-tests-it-stops-with-a-configuration-error) |
| When "kotowari overview build" or "kotowari overview serve" is run without the "overview" key | [Without the overview key, overview build and overview serve stop with a configuration error](overview-data.md#without-the-overview-key-overview-build-and-overview-serve-stop-with-a-configuration-error) |
| An overview source data error, a port error, a location error | [Commands that build and show the overview](overview-commands.md) |
| When the ID passed to "kotowari query" does not exist | [If the positional argument is not exactly one ID, or that ID does not exist, it is a stop with an argument error](query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error) |
| When the list of unspecified surfaces is broken | [List of unspecified surfaces](surface-unspecified.md) |

### Confirmations

- C-core-clienv-8 The details of a configuration error start with the path of the configuration file relative to the base directory, not to the current directory
- C-core-clienv-9 The details of config error when the list of equivalents or a rule file of "tests.rules" or "surface.rules" is broken show the relative path of that file, not of the configuration file
- C-core-clienv-10 When there are no arguments at all, and when there are only options and no command, the first line of standard error is "argument error: expected command: check, list, mutants, overview, plan, query or status"
- C-core-clienv-11 The details of an argument error include the text of the offending argument (for example, adding "--verbose" makes the first line of standard error "argument error: unknown option: --verbose")
- C-core-clienv-12 The details of an unreadable file are the relative path and the operating system's error message, and the details of a file that is not UTF-8 are the relative path

## A mismatch in kotowari's internal mapping stops with mapping error, kept apart from errors in the user's input

Status: implemented

### Problem

kotowari reads parts of Markdown and YAML documents with a schema and an engine of its own, and maps the values and findings the engine returns to kotowari's types and findings. kotowari holds the schema itself, so a failure to map is not a problem in the user's input but a mismatch inside kotowari. Defects the user cannot fix are not to be mixed with errors in input the user can fix. Also, if findings with nothing to map to are silently dropped, the checks quietly loosen when a declaration is added to the schema in the future.

### Decision

When a value received from the engine cannot be mapped to kotowari's types, or a finding of the engine has nothing to map to, kotowari stops with the stop reason "Mapping error" (mapping error). The details are the kind of the finding that could not be mapped and the name of the node, or a description of the value that could not be mapped. The name of the reason says that what is broken is neither the engine nor kotowari but the mapping between them. The kind of the finding and the name of the node are put in the details so that which part of the mapping is missing can be seen at run time.

The schema is embedded in the binary, and no schema file is read at run time. Therefore there is no stop whose reason is that the schema cannot be read. The reasons and wording of stops caused by the user's input (for example unreadable file and non-UTF-8 file) do not change with whether the input is read by the engine.

### Confirmations

- C-core-clienv-13 When a value received from the engine cannot be mapped to kotowari's types, the first line of standard error starts with "mapping error: ", and the details include a description of the value that could not be mapped
