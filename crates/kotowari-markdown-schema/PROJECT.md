# Project Context

## What this is

`mds` is a CLI that validates Markdown documents against a YAML schema declared in the
document's frontmatter (`$schema`) and extracts structured values from them. It exists so that
the format of LLM-written Markdown can be pinned down mechanically. ADR is the first
application.

## Specification

The specification is `docs/ir/schema/` at the repository root, and nowhere else. It is written
in kotowari's IR form so that it can be checked mechanically: each IR document declares a schema
from the repository root's `.mds/schemas/` in its `$schema` frontmatter, so it passes both
`kotowari check` and `mds check docs/ir/schema`.

There is no prose specification beside it. One existed (`docs/spec/mds.md`) until the IR
superseded it; keeping both meant the same rules were written twice, and the prose copy drifted
because nothing checked it. It stays in git history.

Rules the IR does not yet carry are recorded as gaps in `docs/ir/schema/FLAGS.md`. Decisions
the IR cites as sources live in the repository root's `docs/decision/records/`. Terms are
defined in `docs/ir/schema/CONTEXT.md`.

Read the `kotowari` skill before writing or revising an IR document.

## Stack and layout

Rust. One crate inside the `kotowari` Cargo workspace:

- `src/lib.rs` and its modules — the schema language, validation, and extraction. Kept free of
  CLI dependencies.
- `src/main.rs` — the CLI entry point, built as the `mds` binary.

The library is kept free of CLI dependencies so that it can be reused, which is what `kotowari`
does from the same workspace.

## Commands

| Purpose | Command |
|---|---|
| Build | `cargo build` |
| Test | `cargo test` |
| Lint | `cargo clippy` |
| Run locally | `cargo run -- <args>` |
| Quality gates (pre-commit) | `lefthook run pre-commit --no-auto-install` |
| Check the IR against its schemas | `cargo run -- check docs/ir` |
| Check the IR against the spec form | `kotowari check` |

lefthook runs the quality gates (fmt / clippy / test) on every `pre-commit`, as defined in
`lefthook.yml`. Do not run `lefthook install` on a machine whose global pre-commit hook already
delegates to `lefthook run pre-commit --no-auto-install` (as on the developer machines provisioned
with mise): installing would replace that global hook. Which case applies can be determined by
checking the file at `git rev-parse --git-path hooks`/pre-commit (the shared hooks directory, even
in a worktree) for a delegation to `lefthook run pre-commit --no-auto-install`.
On a machine without such a global hook, run `lefthook install` once to generate the local hooks.
With nothing staged, the gates are skipped and the run reports success, so run it with at least
one staged change to actually verify the gates.

## Release

The version is declared in this crate's own `Cargo.toml`. Whether the products in the workspace
share one version or carry a version and a tag each is not decided yet, so `scripts/release.sh`
and `scripts/check-version.sh` still assume the single-workspace-version layout this crate was
imported from, and do not run.

Nothing is published to a registry. The names `mds` and `mds-core` are taken on crates.io by
unrelated projects, which is why this crate is named `kotowari-markdown-schema`. Whether to
publish is not decided yet.

To cut a release:

```console
$ scripts/release.sh 0.2.0
```

It sets the version, promotes the `## Unreleased` section of `CHANGELOG.md` under that version
with a comparison link, runs the gates, commits, tags and pushes. Pushing the tag triggers
`.github/workflows/release.yml`, which re-checks the tag against the manifest, re-runs the gates
and creates the GitHub release from the changelog section.

Write the changelog entry when the change is made, under `## Unreleased`. A published tag is
fixed: correct a released state by releasing a new version, never by moving the tag.

## Conventions specific to this project

Language rules:

- User-visible CLI text (help, errors, output) is English.
- Documentation (specifications, plans, decision records) is Japanese.
- Code comments are Japanese.
- README, `AGENTS.md`, and `PROJECT.md` are English.

## Constraints

## Glossary

Domain terms whose meaning in this project differs from ordinary usage are in `CONTEXT.md`.
