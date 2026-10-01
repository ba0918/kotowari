# Project Context

## What this is

A Cargo workspace holding three Rust packages:

- `kotowari` at the repository root — the `kotowari` binary (`src/main.rs`). A normalised
  specification (the IR) is written as Markdown and checked mechanically by `kotowari check`,
  with the read commands `list`, `query` and `status` and the mutation-test reader `mutants`
  beside it.
- `crates/kotowari-core` — the `kotowari_core` library the root binary is built on. It is the
  root package's only dependency inside the workspace.
- `crates/kotowari-markdown-schema` — the `kotowari_markdown_schema` library and the `kotowari-mds`
  binary. See that crate's own `README.md`.

`Cargo.toml` at the root declares the workspace members and excludes `experiments/`.

The Claude Code skills live in `agent/skills/`, ten of them: the `kotowari` skill (`kotowari/`:
how to write the IR and decision records, read `check`, and place marks) and nine workflow
skills that depend on it (`kotowari-brainstorm/`, `-plan/`, `-cycle/`, `-implement/`,
`-review/`, `-iterate/`, `-investigate/`, `-using-workflow/`, `-adopt/`).
`agent/skills/README.md` says how they relate and how to install them.

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

`lefthook.yml` defines the gates: `pre-commit` runs rustfmt and explicitly restages those paths before the secret scan,
`kotowari check` and `changes --base HEAD --staged --phase implementation`,
`pre-push` runs the full test suite, `kotowari check` with no exemptions, and the mutation tests
in `scripts/mutants.sh`.

The mutation tests in the hook cover only a diff: a branch push runs the mutants in the diff from
`origin/main`, and a tag push runs those in the diff from the product's previous release tag (the
whole workspace only when there is no previous tag). A miss in code that did not change — one
created by deleting or weakening the test that caught a mutant there — is not caught by either.
Run the whole workspace by hand with `scripts/mutants.sh full` when that matters; it takes about
two hours (1,800 mutants on 2026-09-26).

The mutation tests run the whole workspace's test suite once per mutant, so the suite's wall
time is multiplied by the number of mutants (over a thousand on a large diff). Do not write a
test that waits on real time — a timeout actually elapsing, a `sleep`, a slow server. Inject the
duration and keep each wait at 100 ms or less; check a specified value such as a 10-second
timeout by testing the constant, not by waiting for it. After adding tests, look at each test
binary's `finished in` time and find the cause of any that exceeds 0.5 s.

## Specification

The specification is the IR under `docs/ir/`, and nowhere else — `docs/ir/core/` for `kotowari`
and `docs/ir/schema/` for `kotowari-markdown-schema`. `.kotowari/config.yaml` points `check` at
`docs/ir/`, so one run covers both products.

The documents under `docs/ir/schema/` are checked by `kotowari check` alone, the same as those
under `docs/ir/core/`. Neither carries a `$schema` frontmatter, so `kotowari-mds check` does not read them:
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

## Release

The release flow is decided in
[docs/decision/records/2026-09-26-release-flow.md](docs/decision/records/2026-09-26-release-flow.md);
the version and tag form in
[docs/decision/records/2026-09-23-versions-and-cli-name.md](docs/decision/records/2026-09-23-versions-and-cli-name.md).
It is not part of the IR.

The two products carry separate versions, each declared in one place:

| Product | Where the version lives | Declarations that follow it | Tag | Changelog |
|---|---|---|---|---|
| `kotowari` (with `kotowari-core` and the skills in `agent/skills/`) | `version` in `[package]` of the root `Cargo.toml` | `version` of `crates/kotowari-core/Cargo.toml`; the `kotowari` and `kotowari-core` entries of `Cargo.lock` | `kotowari-v<version>` | `CHANGELOG.md` |
| `kotowari-mds` | `version` in `[package]` of `crates/kotowari-markdown-schema/Cargo.toml` | the `kotowari-markdown-schema` entry of `Cargo.lock` | `kotowari-mds-v<version>` | `crates/kotowari-markdown-schema/CHANGELOG.md` (created at the next mds release; until then `kotowari-mds` cannot be released) |

`scripts/check-versions.sh` exits 1 when a following declaration of either product disagrees
with that product's version; given a tag, it also checks the tag's version against that product's
`Cargo.toml`. The
release script and the release workflow both run it. Fix a mismatch by correcting the versions,
never by loosening the check.

A change a user of the product can notice — a command, an option, a finding, a skill's
instructions, the install procedure, what a release contains — is written under
`## [Unreleased]` of that product's changelog, in Keep a Changelog form, on the same branch as the
change. Nothing checks this mechanically. The entry says what changed for someone who installed
the product, not how it was built.

To release:

1. On `main`, with a clean tree and a filled Unreleased section, run
   `scripts/release.sh kotowari <version>`. It writes the version into every declaration, turns
   the Unreleased section into `## [<version>] - <date>` under a new empty Unreleased, and adds the
   comparison links. It then runs `scripts/check-versions.sh <tag>`, `kotowari check` (exit 0
   required) and the full test suite. Only when all pass does it make one commit and the annotated
   tag; otherwise it restores the files and leaves no commit and no tag. It refuses to start when
   the tag already exists locally or on `origin`, or when `origin` cannot be reached to tell. It
   never pushes.
2. Push with the command it prints, `git push origin main && git push origin kotowari-v<version>`:
   main first, and the tag only when main was accepted. The
   pre-push hook runs the mutation tests in the diff from the product's previous release tag
   because a tag is pushed (the whole workspace when there is none). If the
   push is rejected, first check whether the tag is already on the remote
   (`git ls-remote --tags origin kotowari-v<version>`). If it is not, nothing was published: delete
   the local tag (`git tag -d`), drop the release commit (`git reset --keep HEAD~1`), fix and
   commit, and run the script again with the same version. If it is, that version is published
   and must not be reused: drop the local tag and commit the same way, bring in the remote, and
   release a new version.
3. The pushed tag starts `.github/workflows/release.yml`. It reruns the tests, `kotowari check` and
   the version check, builds `x86_64-unknown-linux-gnu` and `aarch64-apple-darwin` binaries as
   `<product>-v<version>-<target>.tar.gz` (binary, README, licences) each with a `.sha256`, and
   creates the GitHub Release with that version's changelog section as its notes.

A pushed tag is published: never move or re-create it. A published release is fixed by releasing
a new version. The release workflow is the only CI; the other gates are the local hooks.

## Change conformance

`.kotowari/config.yaml` enables change records under `docs/changes/**/*.yaml` for code, tests,
skills, build/hook/CI configuration and guides. Before each commit, prepare implementer records
against HEAD and the formatted index. Final records use the branch-wide comparison base and
both implementation and an independent review; archive completed comparisons.

Before integration, pin the full base and candidate head, run the workspace tests, then require
both `kotowari check` and `kotowari changes --base <base> --head <head> --phase review` to exit 0.
`status complete` is not evidence that the branch's changes have been reconciled. Out-of-scope
findings remain reported without widening the fix; they may still block the integration gate.

The pull-request workflow derives the base from the event base/head merge-base and checks the
head SHA after fetching both histories. `scripts/change-base.sh` also supports a push example
using before/after and stops for a zero before until a comparison base is provided. Workflow
files are prepared locally; publishing requires approval. See `docs/guides/change-conformance.md`.
