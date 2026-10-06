# Project Context

## What this is

A Cargo workspace holding nine Rust packages. The root package is `kotowari-cli`; `crates/kotowari` owns acquisition and typed APIs, and `crates/kotowari-source-analysis` owns ast-grep analysis. The schema I/O and mds binary are separate packages at `crates/kotowari-markdown-schema-io` and `crates/kotowari-mds`. `crates/kotowari-overview` checks overview data and builds its render input in memory, and `crates/kotowari-markdown-view` renders it to static HTML pages in memory; `kotowari overview build` and `serve` write and show those pages.

- `kotowari-cli` at the repository root — the `kotowari` binary (`src/main.rs`). A normalised
  specification (the IR) is written as Markdown and checked mechanically by `kotowari check`,
  with the read commands `list`, `query` and `status`, the mutation-test reader `mutants`, and
  `overview build` and `overview serve` for the overview pages beside it.
- `crates/kotowari-core` — pure memory parsing, comparison and inspection. The root binary depends normally only on the high-level `kotowari` library.
- `crates/kotowari-markdown-schema` — pure schema/document validation and extraction. See that crate's own `README.md`.

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
| Check this repository's own IR | `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- check --format text` |
| Test optional adapters | `CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked` |
| Quality gates (pre-commit) | `lefthook run pre-commit --no-auto-install` |

`CARGO_BUILD_JOBS=4` caps the parallel build jobs for memory reasons; `lefthook.yml` runs cargo
the same way in both hooks.

`rust-toolchain.toml` pins the toolchain (Rust 1.99.0). The root `Cargo.toml` declares the
edition (2024), the minimum supported Rust version (1.99, the same as the pin) and the lint
configuration once under `[workspace.package]` and `[workspace.lints]`, and all nine packages
inherit them. The pin applies only when cargo runs through rustup: a version manager that sets
`RUSTUP_TOOLCHAIN` or puts its own cargo first on `PATH` overrides it, and an older toolchain then
fails on `rust-version`.

`scripts/gates.sh` runs the Rust gates — `cargo fmt --all --check`, clippy over all targets with
warnings as errors (default features and all features) and the whole test suite, all `--locked`.
The CI workflow (`.github/workflows/ci.yml`) runs it on every pull request and on `main`; its
"Rust gates" job is the check to require. Lints are suppressed only with
`#[expect(..., reason = "...")]`; integration-test and example files carry one
`#![expect(clippy::unwrap_used, ...)]` for their helpers.

`lefthook.yml` defines the local gates: `pre-commit` runs the secret scan and `kotowari check`,
and `pre-push` runs the full test suite and `kotowari check` with no exemptions. The mutation
tests do not run in the hooks; they blocked every push for one to two hours.

The mutation tests run in GitHub Actions, split into sixteen parallel shards
(`.github/workflows/mutants-run.yml`), each inside a systemd scope capped at 12G of memory and
300% CPU, with cargo-mutants on the pinned `nightly-2026-10-03` and a per-mutant timeout derived
from the baseline run. Every pull request runs the mutants in the diff from its merge base
(`.github/workflows/mutants.yml`), and branch protection on `main` requires that workflow's
`mutants` job, for administrators too, so `main` only takes commits that passed it. A weekly
run over the whole code base on `main` (`.github/workflows/mutants-scheduled.yml`) opens an
issue when it finds survivors; it is not a gate.
A release runs the mutants in the diff from the product's previous release tag (the whole
workspace only when there is no previous tag) before building binaries. A miss in code that did
not change — one created by deleting or weakening the test that caught a mutant there — is not
caught by either gate; the weekly whole run finds it. On a workstation, run only the files whose
tests are being written (`scripts/mutants.sh full -- -f <path>`), in the background; it needs the
pinned nightly (`rustup toolchain install nightly-2026-10-03`) and runs inside a user service
capped at 12G and 400% CPU at the lowest priority. `MUTANTS_JOBS` sets how many mutants run at once; keep the default of 1 locally, since 3
and 4 were slower on 2026-10-04 and produced a false timeout.

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
document and before acting on a `kotowari check` finding. To decide where a new request starts,
use the `kotowari-using-workflow` skill, not `ba0918-using-workflow`.

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
| `kotowari` | root `Cargo.toml` package version | CLI, kotowari, core, source-analysis and overview manifests/lock entries and incoming dependency versions | `kotowari-v<version>` | `CHANGELOG.md` |
| `kotowari-mds` | schema `Cargo.toml` package version | schema, schema-io, mds and markdown-view manifests/lock entries and incoming dependency versions, including core-to-schema and overview-to-view | `kotowari-mds-v<version>` | `crates/kotowari-markdown-schema/CHANGELOG.md` |

`scripts/check-versions.sh` exits 1 when a following declaration of either product disagrees
with that product's version; given a tag, it also checks the tag's version against that product's
`Cargo.toml`. The
release script and the release workflow both run it. Fix a mismatch by correcting the versions,
never by loosening the check.

A change a user of the product can notice — a command, an option, a finding, a skill's
instructions, the install procedure, what a release contains — is written under
`## [Unreleased]` of that product's changelog, in Keep a Changelog form, on the same branch as the
change. Nothing checks this mechanically. The entry says what changed for someone who installed
the product, not how it was built. Both changelogs are written in English.

To release:

1. On `main`, with a clean tree and a filled Unreleased section, run
   `scripts/release.sh kotowari <version>`. It writes the version into every declaration, turns
   the Unreleased section into `## [<version>] - <date>` under a new empty Unreleased, and adds the
   comparison links. It then runs `scripts/check-versions.sh <tag>`, `kotowari check` (exit 0
   required) and the full test suite. Only when all pass does it make one commit and the annotated
   tag; otherwise it restores the files and leaves no commit and no tag. It refuses to start when
   the tag already exists locally or on `origin`, or when `origin` cannot be reached to tell. It
   never pushes.
2. Push the release commit to a `release/<tag>` branch and open a pull request, as the script
   prints, so that the required checks run on that commit. Push the branch with
   `--no-follow-tags`, so that the annotated tag the script made stays local even when
   `push.followTags` is set. When they pass, push with
   `git push --no-follow-tags origin main && git push origin kotowari-v<version>`: main first, and
   the tag only when main was accepted. If the
   push is rejected, first check whether the tag is already on the remote
   (`git ls-remote --tags origin kotowari-v<version>`). If it is not, nothing was published: delete
   the local tag (`git tag -d`), drop the release commit (`git reset --keep HEAD~1`), fix and
   commit, and run the script again with the same version. If it is, that version is published
   and must not be reused: drop the local tag and commit the same way, bring in the remote, and
   release a new version.
3. The pushed tag starts `.github/workflows/release.yml`. It reruns the tests, `kotowari check` and
   the version check, runs the mutation tests in the diff from the product's previous release tag
   (the whole workspace when there is none), builds `x86_64-unknown-linux-gnu` and `aarch64-apple-darwin` binaries as
   `<product>-v<version>-<target>.tar.gz` (binary, README, licences) each with a `.sha256`, and
   creates the GitHub Release with that version's changelog section as its notes.

A pushed tag is published: never move or re-create it. A published release is fixed by releasing
a new version. The release workflow publishes releases.
The local hooks remain required gates.
