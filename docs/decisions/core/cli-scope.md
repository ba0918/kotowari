# Users and construction

English | [日本語](cli-scope.ja.md)

Covers who uses kotowari, where kotowari may write, and how the code is laid out. The responsibilities and dependency direction of the library crates are decided in the [Crate boundaries of the library](library-crates.md) topic, and the details of the location the overview commands write to in the [Commands that build and show the overview](overview-commands.md) topic.

## kotowari is a CLI used first of all by an LLM

Status: implemented

### Problem

Who kotowari is a tool for, and where it is used, has to be decided. The user intends to have the LLM itself use kotowari, to support the workflow of deciding the specification and then implementing. A person may run it to check things, but is not the primary user.

### Decision

kotowari is a CLI used first of all by an LLM. A person may also run it to check things. The place it is used is the repository of a developer who, in their own repository, runs the flow of using an LLM to decide the specification in a brainstorm and then implementing it.

## kotowari saves no state and writes nowhere other than standard output and standard error

Status: implemented

### Problem

kotowari's commands read, check, and output the results to standard output and standard error. Only the commands that build the overview need to write page files. If the destination could be changed by configuration, where output goes would depend on the configuration, and a large hole would open in the strong guarantee of writing nothing. The hole is to be kept as small as possible. The reason for deciding not to write anywhere else in the first place is not recorded.

### Decision

kotowari saves no state. It writes nowhere other than standard output and standard error.

The only exceptions are that "kotowari overview build" and "kotowari overview serve" write under ".kotowari/cache/overview/" in the base directory, and delete files under it that they created. The destination cannot be changed by configuration ([The overview commands write and delete nothing outside .kotowari/cache/overview](overview-commands.md#the-overview-commands-write-and-delete-nothing-outside-kotowaricacheoverview)). Which files under it are deleted is decided in [overview build draws everything every time, writes only the files whose contents differ, and deletes files no longer produced](overview-commands.md#overview-build-draws-everything-every-time-writes-only-the-files-whose-contents-differ-and-deletes-files-no-longer-produced). No exception is made for the other commands.

### Confirmations

- C-core-scope-1 Running "kotowari check" does not change the list of files or their contents under the base directory and under the directories that the environment variables "HOME" and "TMPDIR" point at
- C-core-scope-2 Even when there is overview source data, "kotowari check" and "kotowari status" write nothing

## A crate is added each time a layer is added, and the CLI is held in src/ at the repository root

Status: implemented

### Problem

In what units the code is divided has to be decided. The reason for adding a crate each time a layer is added is not recorded. The library package is named kotowari so that people who use the library can start from `cargo add kotowari`.

### Decision

A crate is added each time a layer is added (how the crates are divided is in [Packages are split by purpose, and users choose a package by purpose](library-crates.md#packages-are-split-by-purpose-and-users-choose-a-package-by-purpose)). The CLI is held in "src/" at the repository root.

The package at the repository root is kotowari-cli, and it provides kotowari's commands in "src/main.rs". The library package is named kotowari. The command's name stays kotowari.

### Rejected options

- Place `crates/kotowari-core` as one of the layers from the start. The reason it was not taken is not recorded.

### Confirmations

- C-core-scope-3 (by reading) The package kotowari-cli at the repository root provides kotowari's commands in "src/main.rs", and the library is divided into crates per layer
