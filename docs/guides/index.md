# kotowari guide

English | [日本語](index.ja.md)

<!-- @kotowari[REQ-core-001:f6b868d0] -->

kotowari is a CLI that mechanically checks a specification (the IR) written in Markdown.
You write requirements, decision tables and scenarios in a fixed form, and `kotowari check` confirms that each one has a source in a decision record and is tied to tests.

This guide is a reference for developers who use kotowari.
The specification itself is in the [IR](../ir/core/); when the guide and the IR disagree, the IR is right.

## Start here

| What you want to do | Page to read |
|---|---|
| Learn the common command syntax, the exit codes, and how to read a stop | [CLI (common to all commands)](cli.md) |
| Call the typed API from Rust | [Public crate API](public-crate-api.md) |
| Validate standalone distribution archives | [Package validation](package-validation.md) |
| Configure where the IR, records and test files are | [Configuration file](config.md) |
| Check the specification and fix findings | [kotowari check](commands/check.md) and [Finding kinds](findings.md) |
| Tie tests to requirements | [Marking tests](marks.md) |
| Leave requirements you are not building now out of the test checks | [Deferring requirements](deferred.md) |
| Find, from the code, features built without being written in the IR | [Surface check](surface.md) |

## Commands

| Command | What it does |
|---|---|
| [check](commands/check.md) | Checks the IR and the marks in tests, and reports findings |
| [list](commands/list.md) | Lists items and scenarios together with their marked tests |
| [query](commands/query.md) | Prints the body of one item and the items that point at it |
| [status](commands/status.md) | Answers whether everything lines up, with counts and one boolean |
| [mutants](commands/mutants.md) | Reads mutation testing results and turns misses into findings |
| [plan](commands/plan.md) | Checks the form of an implementation plan with the bundled schema |
| [overview](../ir/core/overview-commands.md) | Writes overview pages from overview data with `build`, and with `serve` writes them and serves them locally |

## Reference

- [CLI (common to all commands)](cli.md) — syntax, common options, exit codes, reasons for stopping
- [Configuration file](config.md) — every key of `.kotowari/config.yaml`
- [Finding kinds](findings.md) — the list of errors and notices, their detail, and how to fix them
- [Marking tests](marks.md) — how to write `@kotowari[...]`, and which tests are found
- [Deferring requirements](deferred.md) — how to write `- deferred:`, what it does, and the mismatch notices
- [Surface check](surface.md) — how to write surface rules, where a surface counts as being in the IR, and the list of unspecified surfaces
- [Writing guides](writing-guides.md) — put marks and fingerprints in user-facing documents like this guide so they keep up with IR changes

## How this guide is kept up to date

Under each section heading, an HTML comment holds the IR items that section explains, with their fingerprints (it does not appear on the rendered page).
When the IR changes, `kotowari check` raises a guide_stale notice for the sections that went stale, so you only review those sections.
How to do this is in [Writing guides](writing-guides.md).
