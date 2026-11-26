# Reading the results of mutation tests

English | [日本語](mutants-input.ja.md)

Covers how "kotowari mutants" maps the results file of a mutation-testing tool onto mutation outcomes, and how it stops on a results file that cannot be mapped. Findings and counts are decided in [Checking mutation outcomes](mutants.md). The handling of the "--tool" argument, which selects which tool the results come from, is decided in [kotowari mutants always takes the tool's name with --tool, and takes exactly one results file](cli.md#kotowari-mutants-always-takes-the-tools-name-with---tool-and-takes-exactly-one-results-file), and the normalization of paths in [Paths in configuration values are normalized before comparing and before output](base-directory.md#paths-in-configuration-values-are-normalized-before-comparing-and-before-output).

## Mutation outcomes are held in a form that does not know the tool, and the results file of cargo-mutants is mapped as in the table

Status: implemented

### Problem

kotowari itself is a tool that does not restrict the language, and the mutation-testing tool that produces the results may change. For now there are only Rust and cargo-mutants. If forms specific to a tool leak into the findings and what was decided, the existing rules change every time a tool is added.

Because there is only one tool producing results for now, deciding a neutral file format of kotowari's own would be a contract decided by imagination. A script that converts the tool's results into that format would be checked by no one. If the real format is read, it can be tested by just placing a results file.

If which tool the results come from could be omitted, the default tool would burn in the assumption of Rust. If the tool were identified from the content of the file, cases of misidentification would arise when tools increase.

### Decision

kotowari holds inside itself one form of mutation outcome that does not know the tool. A mutation outcome has a file, a line, a change description (the text as the tool produced it) and an outcome (four values: caught, missed, timed out, unviable). Findings and counts are decided only in the terms of this form. Reading for each tool is a part that maps that tool's results file onto this form. When a tool is added, the existing rules are not changed; a part and a mapping table are added.

What the first version reads is the results file of cargo-mutants (outcomes.json) as it is. Which tool the results come from is always specified with "--tool" of "kotowari mutants". For now its only value is "cargo-mutants".

When "--tool" is "cargo-mutants", the results file is JSON with a list under the top-level key "outcomes". An entry whose "scenario" is the string "Baseline" is the baseline run (a run without a mutation) and does not become a mutation outcome. Every other entry has the mutation in "Mutant" under "scenario". Each entry is mapped as follows.

| Field of the mutation outcome | Mapped from |
|---|---|
| File | "scenario.Mutant.file" (a string). Read as a path relative to the base directory, with the same normalization as configuration paths ([Paths in configuration values are normalized before comparing and before output](base-directory.md#paths-in-configuration-values-are-normalized-before-comparing-and-before-output)) applied |
| Line | "scenario.Mutant.span.start.line" (a number) |
| Change description | The rest of "scenario.Mutant.name" (a string) after removing from its start the prefix made by joining, in this order, the value of "scenario.Mutant.file", ":", the value of "span.start.line", ":", the value of "span.start.column" and ": " |
| Outcome "caught" | "summary" is "CaughtMutant" |
| Outcome "missed" | "summary" is "MissedMutant" |
| Outcome "timed out" | "summary" is "Timeout" |
| Outcome "unviable" | "summary" is "Unviable" |
| Baseline run succeeded | The "summary" of the baseline run entry is "Success" |

Keys other than those in the table are not looked at, and an unknown key does not stop it. This is so as not to be caught up in keys the tool adds later. A results file with no baseline run at all is also read. cargo-mutants has an option that skips the baseline run.

The normalization of paths in the results file exists so that a difference only in how a path is written does not break a match with the list of equivalents, and the normalized path is also output as the path of a finding.

### Rejected options

- Decide a neutral file format of kotowari's own, and keep a conversion script per tool outside. The neutral format would be a contract decided by imagination, and the conversion scripts would be checked by no one.
- Give a default value for which tool the results come from. The assumption of Rust burns in.
- Identify the tool from the content of the file. Cases of misidentification arise when tools increase.
- Previous decision: Mutation outcomes also hold the function name ("function.function_name" in cargo-mutants). It was only mapped and used by no finding and no count, and the change description of cargo-mutants contains the function name, so it was removed.

### Confirmations

- C-core-mutin-1 The "file" and "span.start.line" of an entry of the cargo-mutants results, and the rest of "name" after removing the prefix, become the path, line and detail of the finding respectively
- C-core-mutin-2 "CaughtMutant", "MissedMutant", "Timeout" and "Unviable" of "summary" are counted as caught, missed, timed out and unviable respectively
- C-core-mutin-3 A results file that has no baseline run and has keys not in the table is also read without stopping
- C-core-mutin-4 Even for results that write the "file" of a result as "./src/a.rs", the normalized path "src/a.rs" is output as the path of the finding

## A results file whose form does not fit stops with a results error

Status: implemented

### Problem

Silently skipping unknown values drops misses. Results in which the tests fail in the baseline run, which puts in no mutation, are meaningless as a whole. A match with the list of equivalents depends on the string of the change description, so if the tool's format changes, it should stop rather than silently pass. Nothing outside the base directory is read.

### Decision

A results file that falls under any of the following stops with a new reason for a stop, "results error". The wording of the stop is "results error", and the detail is the relative path of the results file and a description of the error.

- It cannot be read as JSON
- A key that the mapping table maps from is missing or has the wrong type
- A result of an entry matches no row of the mapping table (including when an entry with a mutation has "Success" or "Failure")
- The result of the baseline run is not "baseline run succeeded"
- A line is less than 1
- The prefix of the change description is not in the form of the mapping table
- A file is an absolute path or contains a ".." component

When the results file is missing, unreadable or not UTF-8, it stops with the existing reasons for a stop (unreadable file, non-UTF-8 file).

### Confirmations

- C-core-mutin-5 A results file that cannot be read as JSON, has a required key of the wrong type, has an unknown value of "summary", has a baseline run that did not succeed, has a line of 0, or has a "name" whose prefix does not fit the form stops with exit code 2, and the first line of standard error starts with "results error: " and the relative path of the results file. A mutation outcome whose line is 1 is read without stopping
- C-core-mutin-6 If the "file" of a result points outside the base directory (contains ".."), it stops with a results error
- C-core-mutin-7 If the results file is missing, it stops with exit code 2, and the first line of standard error starts with "unreadable file: "
