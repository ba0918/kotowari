# kotowari guide

English | [日本語](index.ja.md)

<!-- @kotowari[docs/decisions/core/cli.md#kotowari-has-seven-commands-and-the-single-command-check-does-both-the-check-of-the-specification-and-the-check-of-the-correspondence-with-tests:d77a0bd7, docs/decisions/core/decision-documents.md#the-specification-lives-only-in-decision-documents-one-per-topic-holding-the-decisions-now-in-force:d2ea663b] -->

kotowari is a CLI that mechanically checks a specification written in Markdown as decision documents.
You write, per topic, the decisions that hold now, each with the results that show it is kept, and `kotowari check` confirms that the documents have the fixed form and that a test is tied to each of those results.

This guide is a reference for developers who use kotowari.
The specification itself is in the [decision documents](../decisions/core/); when the guide and the decision documents disagree, the decision documents are right.

## Start here

| What you want to do | Page to read |
|---|---|
| Learn the common command syntax, the exit codes, and how to read a stop | [CLI (common to all commands)](cli.md) |
| Call the typed API from Rust | [Public crate API](public-crate-api.md) |
| Validate standalone distribution archives | [Package validation](package-validation.md) |
| Configure where the decision documents and test files are | [Configuration file](config.md) |
| Write the specification as decision documents | [Writing decision documents](decision-documents.md) |
| Check the specification and fix findings | [kotowari check](commands/check.md) and [Finding kinds](findings.md) |
| Tie tests to confirmation rows | [Marking tests](marks.md) |
| Find, from the code, features built without being written in the specification | [Surface check](surface.md) |

## Commands

| Command | What it does |
|---|---|
| [check](commands/check.md) | Checks the decision documents and the marks in tests, and reports findings |
| [list](commands/list.md) | Lists decisions and confirmation rows together with their marked tests |
| [query](commands/query.md) | Prints the body of one decision or row, and the decisions that link to it |
| [status](commands/status.md) | Answers whether everything lines up, with counts and one boolean |
| [mutants](commands/mutants.md) | Reads mutation testing results and turns misses into findings |
| [plan](commands/plan.md) | Checks the form of an implementation plan with the bundled schema |
| [overview](../decisions/core/overview-commands.md) | Writes overview pages from overview data with `build`, and with `serve` writes them and serves them locally |

## Reference

- [CLI (common to all commands)](cli.md) — syntax, common options, exit codes, reasons for stopping
- [Configuration file](config.md) — every key of `.kotowari/config.yaml`
- [Writing decision documents](decision-documents.md) — the form of a topic document, states, confirmation rows and their IDs, open questions
- [Finding kinds](findings.md) — the list of errors and notices, their detail, and how to fix them
- [Marking tests](marks.md) — how to write `@kotowari[...]`, and which tests are found
- [Surface check](surface.md) — how to write surface rules, where a surface counts as being in the specification, and the list of unspecified surfaces
- [Writing guides](writing-guides.md) — put marks and fingerprints in user-facing documents like this guide so they keep up with changes to the decisions

## How this guide is kept up to date

<!-- @kotowari[docs/decisions/core/languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change:de5a25a2] -->

Under each section heading, an HTML comment holds the decisions that section explains, with their fingerprints (it does not appear on the rendered page).
When the Decision or Confirmations of one of them changes, `kotowari check` raises a guide_stale notice for the sections that went stale, so you only review those sections.
How to do this is in [Writing guides](writing-guides.md).
