# Project Context

## What this is

A Cargo workspace holding three Rust packages:

- `kotowari` at the repository root — the `kotowari` binary (`src/main.rs`). A normalised
  specification (the IR) is written as Markdown and checked mechanically by `kotowari check`,
  with the read commands `list`, `query` and `status` and the mutation-test reader `mutants`
  beside it.
- `crates/kotowari-core` — the `kotowari_core` library the root binary is built on. It is the
  root package's only dependency inside the workspace.
- `crates/kotowari-markdown-schema` — the `kotowari_markdown_schema` library and the `mds`
  binary. See that crate's own `README.md`.

`Cargo.toml` at the root declares the workspace members and excludes `experiments/`.

The `kotowari` skill for Claude Code lives in `skills/kotowari/`.

## Build and test

Run cargo from the repository root; it covers the whole workspace.

| Purpose | Command |
|---|---|
| Build | `CARGO_BUILD_JOBS=4 cargo build --workspace` |
| Test | `CARGO_BUILD_JOBS=4 cargo test --workspace` |
| Test one crate | `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema` |
| Check this repository's own IR | `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` |
| Quality gates (pre-commit) | `lefthook run pre-commit --no-auto-install` |

`CARGO_BUILD_JOBS=4` caps the parallel build jobs for memory reasons; `lefthook.yml` runs cargo
the same way in both hooks.

The minimum supported Rust version is declared per crate, not once for the workspace:
`crates/kotowari-markdown-schema` declares `rust-version = "1.89"`, and neither `kotowari` nor
`kotowari-core` declares one. All three are on `edition = "2024"`.

`lefthook.yml` defines the gates: `pre-commit` runs the secret scan and `kotowari check`,
`pre-push` runs the full test suite, `kotowari check` with no exemptions, and the mutation tests
in `scripts/mutants.sh`.

## Specification

The specification is the IR under `docs/ir/`, and nowhere else — `docs/ir/core/` for `kotowari`
and `docs/ir/schema/` for `kotowari-markdown-schema`. `.kotowari/config.yaml` points `check` at
`docs/ir/`, so one run covers both products.

The documents under `docs/ir/schema/` are checked by `kotowari check` alone, the same as those
under `docs/ir/core/`. Neither carries a `$schema` frontmatter, so `mds check` does not read them:
given a directory it skips them, and given one of them by name it stops.

`docs/spec/` is **not** the specification. It holds prose — concept notes and the write-ups a
brainstorm produced — and `check` never reads it, because `.kotowari/config.yaml` points only at
`docs/ir/`. Where prose and IR disagree, the IR is what holds.

Terms are defined in the `CONTEXT.md` of each IR directory, and rules the IR does not yet carry
are recorded in its `FLAGS.md`. Read the `kotowari` skill before writing or revising an IR
document.

## Decisions

Decisions live in `docs/decision/records/`, one file per brainstorm, one line per decision. The
IR cites them as its sources. `docs/decision/adr/` holds the four ADRs written before
2026-09-17; they are kept as citation targets and no new ones are written.

Implementation plans are in `docs/plans/`. Unresolved specification questions are in `TODO.md`.
