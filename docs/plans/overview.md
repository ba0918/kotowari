# Plan: draw the whole picture of a brainstorm with kotowari overview

## Goal

A person whose project sets `overview.files` gets `kotowari check` findings for broken overview data, `kotowari overview build` HTML pages under `.kotowari/cache/overview/`, and `kotowari overview serve` to view them, through the public-crate layout and its Rust API.

## Specification

The IR store is `docs/ir/`. Read every requirement with `kotowari query ID`, including its definition table and its scenarios; never work from this plan's paraphrase. The governing decision records are [the overview record](../decision/records/2026-10-02-whole-picture.md) (A1–A86; A53 fixes the order and excludes hot reload U1 and diagram parts), [the public-crate record](../decision/records/2026-10-03-public-crate-api.md) (A33) and [the record that puts overview on the public crates](../decision/records/2026-10-04-overview-on-public-api.md) (A1–A8).

New requirements and scenarios this plan must make tested:

| Document | Items |
|---|---|
| `docs/ir/view/rendering.md` | REQ-view-001 to REQ-view-010, EX-view-001 to EX-view-006 |
| `docs/ir/view/parts.md` | REQ-view-011 to REQ-view-015 (TBL-view-001), EX-view-007 to EX-view-009 |
| `docs/ir/core/overview-data.md` | REQ-core-278 to REQ-core-286 (TBL-core-038), EX-core-463 to EX-core-471 |
| `docs/ir/core/overview-output.md` | REQ-core-288 to REQ-core-290, EX-core-472, EX-core-473 |
| `docs/ir/core/overview-commands.md` | REQ-core-291 to REQ-core-299, REQ-core-305 (TBL-core-039), EX-core-474 to EX-core-479 |
| `docs/ir/core/cli.md` | REQ-core-304 |

Requirements verified by review: `docs/ir/core/overview-data.md#REQ-core-287`, `docs/ir/core/overview-workflow.md#REQ-core-300` to `#REQ-core-303`, `docs/ir/core/library-crates.md#REQ-core-306` and `#REQ-core-307`.

Existing items whose text the overview specification or the 2026-10-04 record changed, so their tests must follow the new text: REQ-core-001, REQ-core-002, REQ-core-004, REQ-core-008, REQ-core-014, REQ-core-018, REQ-core-019, REQ-core-027, REQ-core-102, REQ-core-152, REQ-core-158, REQ-core-162, REQ-core-290, REQ-core-310 (TBL-core-041 gains `overview_prepare` and `overview_build`), REQ-core-315 (TBL-core-042 gains the additional finding group), and the tables TBL-core-001, TBL-core-004, TBL-core-005, TBL-core-008, TBL-core-018, TBL-core-019, TBL-core-020, TBL-core-028. Compare each with `git show 89a9df4` and with the commit that adds `docs/decision/records/2026-10-04-overview-on-public-api.md`. The new finding kinds and stop wordings also reach the tests of REQ-core-125 and REQ-core-127, which compare the tables in `agent/skills/kotowari/references/findings.md` with the code.

## Approach and why

Build from the bottom of the dependency graph upward, in the order A53 fixes: the view engine, then the core vocabulary, then the overview checks and rendering, then their integration into check, then build, serve, and the skill texts. Each layer is testable before the next one consumes it.

The layering follows the 2026-10-04 record. `kotowari-markdown-view` (Markdown series, no workspace dependency) turns a render input into pages and publishes the part schemas; it reads and writes nothing. `kotowari-overview` (kotowari series) depends on core, `kotowari-markdown-schema` and the view; from overview texts and already-read IR and decision records it produces findings, the reference table, stale sections and the pages (by calling the view), and it touches no file, network or environment. The `kotowari` library reads the overview files, runs the overlap checks, passes the findings to core as an additional finding group, and offers `overview_prepare` (check and render in memory) and `overview_build` (prepare, then write `.kotowari/cache/overview/`) per record A7. `kotowari-cli` parses the `overview` subcommands, prints, and runs `serve` with `tiny_http` and `ctrlc` in the order prepare, bind, write.

Core learns no overview meaning. It gains only what the IR assigns to it: the overview finding kinds in its kind vocabulary and line rules (TBL-core-008, TBL-core-019, REQ-core-027), the `overview.files` configuration key, a generic additional finding group in its check inputs (TBL-core-042) that it sorts, counts and reports with the group's numbers in check, status and inspection, and a public way to read the guide marks of one text with their lines and staleness without the guides overlap stop, so that kotowari-overview can reuse the guide rules (REQ-core-286). Results stay assembled only by core, as the public API already requires. The stop wording "overview error" belongs to a kotowari library error kind and "port error" to a CLI stop reason (record A8); neither enters core, so the REQ-core-127 test must read the wordings from all three places.

Reuse instead of rebuilding: the overview form is a `kotowari-markdown-schema` schema embedded from `crates/kotowari-overview/schemas/overview.yaml` (record 2026-10-04 A5), the same way core embeds its schemas; the file walk reuses the `guides.files` acquisition rules, including the named-hidden-directory exception that `changes.records` already has (REQ-core-019). Part contents are YAML read with serde-saphyr and checked with `jsonschema` with its default features off (overview record A46). Markdown text in the view uses the `markdown` crate already in the workspace.

This repository lists its test files per crate in `tests.files` of `.kotowari/config.yaml`. S1 and S3 add the two new crates there so their marked tests count; that is not turning on `overview` for this repository.

## Scope of change

- New crates `crates/kotowari-markdown-view/` and `crates/kotowari-overview/` (sources, embedded schemas, package-local tests, examples, README, licences, CHANGELOG as the existing crates have)
- `crates/kotowari-core/src/` and `crates/kotowari-core/tests/` for the finding kinds, line rules, configuration key, the additional finding group and the guide-mark reading
- `crates/kotowari/` for acquisition, check/status/inspect integration, `overview_prepare`, `overview_build`, their async counterparts and the overview error kind
- Root `src/`, `tests/` and `Cargo.toml` for the CLI and its process-level tests
- Root `Cargo.toml` workspace members and `Cargo.lock`
- `.kotowari/config.yaml`, only the `tests.files` list
- `scripts/check-versions.sh`, `scripts/release.sh`, `scripts/check-packages.py`, `scripts/tests/` for the two new packages
- `.github/workflows/release.yml` and `.github/workflows/change-conformance.yml` only if package selection needs the new packages
- `agent/skills/kotowari/references/findings.md` for the new kinds and wordings, and `agent/skills/kotowari-brainstorm/`, `agent/skills/kotowari/`, `agent/skills/kotowari-cycle/`, `agent/skills/kotowari-implement/` as REQ-core-300 to REQ-core-303 require
- `README.md`, `PROJECT.md`, `CHANGELOG.md` and `crates/kotowari-markdown-schema/CHANGELOG.md` entries for the new command and packages
- `.kotowari/changes/implementation.yaml` and `.kotowari/changes/review.yaml`, each by its own role
- `.kotowari/equivalents.yaml` only for equivalents that pass the existing challenge rule

The IR and the decision records are approved and do not change in this plan.

## Step order and prerequisites

The specification update (the IR edits, the 2026-10-04 record and the edits to the 2026-10-02 record) and this plan are committed before S1. S1 and S2 have no other prerequisite and may run in either order. S3 needs S1 and S2. S4 needs S3. S5 needs S4. S6 needs S5. S7 needs S1 to S6 because it validates the final packages. S8 can run any time after S5. S9 needs S5. S10 comes last. One writer works the steps in order on branch `overview` in the worktree `.agents/worktrees/overview`.

Before S1, capture the branch-wide comparison base once with `git merge-base origin/main HEAD` and keep it in session-local evidence. The branch carries the unmerged public-crate work as well as this plan, so the final change records cover both against that base. Run the baseline: `CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked` must pass, and `kotowari check --format json` must report only the 38 requirement_without_test and 26 scenario_without_test errors that belong to the overview specification, 54 guide_stale notices and 8 size notices.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-view-001 to REQ-view-015 (TBL-view-001) | EX-view-001 to EX-view-009 |
| S2 | REQ-core-014, REQ-core-027, REQ-core-125, REQ-core-290, REQ-core-315 (TBL-core-004, TBL-core-008, TBL-core-019, TBL-core-042) | none new |
| S3 | REQ-core-281 to REQ-core-286, REQ-core-291, REQ-core-292, REQ-core-305 (TBL-core-038, TBL-core-039) | EX-core-466 to EX-core-471, EX-core-474 |
| S4 | REQ-core-018, REQ-core-019, REQ-core-278, REQ-core-279 (check half), REQ-core-280, REQ-core-288, REQ-core-289, REQ-core-290, REQ-core-152, REQ-core-158, REQ-core-162 (TBL-core-005, TBL-core-028) | EX-core-464, EX-core-465, EX-core-472, EX-core-473 |
| S5 | REQ-core-279 (build half), REQ-core-293 to REQ-core-296, REQ-core-304, REQ-core-001, REQ-core-002, REQ-core-004, REQ-core-008, REQ-core-102, REQ-core-127, REQ-core-310 (TBL-core-001, TBL-core-018, TBL-core-020, TBL-core-041) | EX-core-463, EX-core-475, EX-core-476, EX-core-477 |
| S6 | REQ-core-297, REQ-core-298, REQ-core-299, and the serve halves of REQ-core-279, REQ-core-280, REQ-core-294, REQ-core-296 | EX-core-478, EX-core-479 |
| S7 | REQ-core-287, REQ-core-306, REQ-core-307 (review, TBL-core-040) | none |
| S8 | REQ-core-300 to REQ-core-303 (review) | none |
| S9 | overview record A40 (measurement, no requirement) | none |
| S10 | all of the above, as the branch-wide gate | all of the above |

## Left to the implementer

- Type, function and module names inside the new crates, and how the render input types are shaped, as long as REQ-view-001 holds and no third-party AST type appears in a public signature
- The HTML and CSS of the pages, within REQ-view-002 to REQ-view-010 and REQ-view-015; the drawings of each part follow overview record A83 and A85
- The name of the kotowari library error kind for overview-data failures, as long as sync and async forms classify it the same way
- Which fixture files the tests use

## Stop conditions

- A requirement cannot be met without core learning overview semantics, or without a reverse dependency against TBL-core-040
- A part schema or the overview form cannot be expressed with `jsonschema` (default features off) or `kotowari-markdown-schema` as they are
- An existing marked test fails for a reason other than a text change listed under Specification
- The measurement in S9 is far slower than check without overview data, so that the overview record A40 asks to consider replacing the Markdown reader
- A step needs a dependency other than `jsonschema`, `tiny_http` and `ctrlc`
- A step needs a public operation beyond those in TBL-core-041 and the additional group of TBL-core-042

## Test command

PROJECT.md fixes the ordinary commands. The final gate of S10 runs, in order, with `CARGO_BUILD_JOBS=4` on every Cargo command:

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo test -p kotowari-markdown-view --locked
cargo test -p kotowari-overview --locked
cargo test -p kotowari --no-default-features --locked
cargo test -p kotowari --features tokio --locked
cargo test -p kotowari-cli --locked
cargo doc --workspace --all-features --no-deps --locked
scripts/check-versions.sh
python3 -m unittest discover -s scripts/tests -p 'test_public_crate_*.py'
python3 scripts/check-packages.py --output "$PACKAGE_EVIDENCE"
cargo run -q -p kotowari-cli --bin kotowari -- check --format json
cargo run -q -p kotowari-cli --bin kotowari -- changes --base "$BASE" --head "$HEAD" --phase review --format json
```

`PACKAGE_EVIDENCE` is a scratch path, `BASE` the base captured before S1, `HEAD` the final commit. Do not run `cargo fmt --all` to rewrite files. Do not write tests that wait on real time; the serve tests wait for the URL line, not for a sleep.

## Out of scope

- Hot reload in serve (overview record U1) and diagram parts such as flowcharts and sequence diagrams (A13, A53)
- Turning on `overview` in this repository's own `.kotowari/config.yaml` and writing overview data for kotowari itself
- A user guide under `docs/guides/` for the overview commands
- The compiler warnings left by the public-crate split, unless a step touches the same file
- Pushing, opening the pull request and merging; the mutation tests run in the pull request CI

## Steps

### S1: render pages with kotowari-markdown-view

- Purpose: add the view engine crate that turns a render input into pages and publishes the eight part schemas
- Specification: `docs/ir/view/rendering.md#REQ-view-001`, `docs/ir/view/rendering.md#REQ-view-002`, `docs/ir/view/rendering.md#REQ-view-003`, `docs/ir/view/rendering.md#REQ-view-004`, `docs/ir/view/rendering.md#REQ-view-005`, `docs/ir/view/rendering.md#REQ-view-006`, `docs/ir/view/rendering.md#REQ-view-007`, `docs/ir/view/rendering.md#REQ-view-008`, `docs/ir/view/rendering.md#REQ-view-009`, `docs/ir/view/rendering.md#REQ-view-010`, `docs/ir/view/parts.md#REQ-view-011`, `docs/ir/view/parts.md#REQ-view-012`, `docs/ir/view/parts.md#REQ-view-013`, `docs/ir/view/parts.md#REQ-view-014`, `docs/ir/view/parts.md#REQ-view-015`
- Prerequisites: the specification update and this plan are committed; the baseline passes
- May change: `crates/kotowari-markdown-view/`, root `Cargo.toml` workspace members, `Cargo.lock`, the `tests.files` list of `.kotowari/config.yaml` (add `crates/kotowari-markdown-view/**/*.rs`)
- Done when: `kotowari query` shows a marked test for every REQ-view requirement and for EX-view-001 to EX-view-009, and those tests pass; the crate has no workspace dependency and no file, network or environment access
- Shown by: test — one test per EX-view scenario, a property test for REQ-view-004, the closed-schema test for REQ-view-013 and the example-rendering test for REQ-view-014
- Left to the implementer: the page markup and CSS within the requirements; how the part examples are stored inside the crate's tests
- Stop and hand back if: a part field in TBL-view-001 or overview record A83 cannot be drawn as A83 and A85 describe without a field the table does not list

### S2: give core the overview vocabulary, the additional group and guide-mark reading

- Purpose: let core parse `overview.files`, know the overview finding kinds and their line rules, accept a named additional finding group in its check inputs, and read the guide marks of one text for reuse
- Specification: `docs/ir/core/config.md#REQ-core-014`, `docs/ir/core/finding-order.md#REQ-core-027`, `docs/ir/core/library-inputs.md#REQ-core-315`, `docs/ir/core/overview-output.md#REQ-core-290`, `docs/ir/core/overview-data.md#REQ-core-286`
- Prerequisites: the specification update and this plan are committed
- May change: `crates/kotowari-core/src/`, `crates/kotowari-core/tests/`, `tests/step1_config.rs`, `tests/step7_skill_references.rs`, `agent/skills/kotowari/references/findings.md` (the Kind table only)
- Done when: a config with `overview.files` loads and one with an unreadable glob element stops with a config error; findings of the overview kinds get the null or numbered line TBL-core-019 gives; a check, a status and an inspection built from memory inputs with an additional group sort its findings with the others, count them in counts and complete, and expose the group's name, files and marks; the guide marks of one text are returned with their lines and staleness without the guides overlap stop; the REQ-core-125 test passes with the new kinds in the Kind table
- Shown by: test — the existing marked tests of REQ-core-014 and REQ-core-027 extended with the overview cases, new core tests of the additional group for REQ-core-315 and REQ-core-290, a core test of the guide-mark reading, and the REQ-core-125 test
- Left to the implementer: the shape of the additional group type and of the guide-mark reading, as long as core only reads the group's name, numbers and findings
- Stop and hand back if: accepting the group requires letting callers build or rewrite a finished check result, which the public API forbids

### S3: check and render overview data in kotowari-overview

- Purpose: add the crate that, from overview texts and read IR and records, returns the findings, the reference table, the stale sections and the pages the view renders from them
- Specification: `docs/ir/core/overview-data.md#REQ-core-281`, `docs/ir/core/overview-data.md#REQ-core-282`, `docs/ir/core/overview-data.md#REQ-core-283`, `docs/ir/core/overview-data.md#REQ-core-284`, `docs/ir/core/overview-data.md#REQ-core-285`, `docs/ir/core/overview-data.md#REQ-core-286`, `docs/ir/core/overview-commands.md#REQ-core-291`, `docs/ir/core/overview-commands.md#REQ-core-292`, `docs/ir/core/overview-commands.md#REQ-core-305`
- Prerequisites: S1, S2
- May change: `crates/kotowari-overview/`, root `Cargo.toml` workspace members, `Cargo.lock`, the `tests.files` list of `.kotowari/config.yaml` (add `crates/kotowari-overview/**/*.rs`)
- Done when: from memory inputs, the crate returns each finding of EX-core-466 to EX-core-471 with the kind, line and detail the requirements state, returns the reference table of EX-core-474, marks a section stale only when a guide mark in it is stale, returns overview_name_conflict for a duplicated name and for "index" and "style", and for valid data returns the pages the view returns for its render input; `kotowari query` shows a marked test for each of these items
- Shown by: test — EX-core-466, EX-core-467, EX-core-468, EX-core-469, EX-core-470, EX-core-471, EX-core-474 as crate tests, one test per row of TBL-core-038 and TBL-core-039, one test each for REQ-core-292 and REQ-core-305, and one that valid data yields the four page names of the view
- Left to the implementer: the internal split between form, parts, references, marks and rendering
- Stop and hand back if: the form in TBL-core-038 cannot be written as a `kotowari-markdown-schema` schema, or a JSON Pointer in REQ-core-282 cannot be derived from what `jsonschema` reports

### S4: add overview findings to check and status

- Purpose: read the overview files in the kotowari library and pass their findings and numbers to core, so check, status and inspect report them
- Specification: `docs/ir/core/config.md#REQ-core-018`, `docs/ir/core/config.md#REQ-core-019`, `docs/ir/core/overview-data.md#REQ-core-278`, `docs/ir/core/overview-data.md#REQ-core-279`, `docs/ir/core/overview-data.md#REQ-core-280`, `docs/ir/core/overview-output.md#REQ-core-288`, `docs/ir/core/overview-output.md#REQ-core-289`, `docs/ir/core/overview-output.md#REQ-core-290`, `docs/ir/core/list.md#REQ-core-152`, `docs/ir/core/query.md#REQ-core-158`, `docs/ir/core/status.md#REQ-core-162`
- Prerequisites: S3
- May change: `crates/kotowari/`, root `src/`, root `tests/`, `Cargo.lock`
- Done when: EX-core-464, EX-core-465, EX-core-472 and EX-core-473 pass through the CLI; without the "overview" key check reads no overview data and reports the group as zeros; the check JSON always has the "overview" group and the text form never shows it; the walk follows REQ-core-018 and REQ-core-019 for `overview.files`; list and query neither read overview data nor stop on it; Project check, status and inspect return the same findings as the CLI
- Shown by: test — EX-core-464, EX-core-465, EX-core-472, EX-core-473 as CLI tests, the check half of EX-core-463, the existing tests of REQ-core-018, REQ-core-019, REQ-core-152, REQ-core-158 and REQ-core-162 extended with an overview case, and one library test that Project check includes overview findings
- Left to the implementer: where the overlap check lives inside the library
- Stop and hand back if: the overlap order of REQ-core-280 conflicts with the existing order of REQ-core-199

### S5: build the pages with overview build

- Purpose: add `kotowari overview build`, and `overview_prepare` and `overview_build` on Project and AsyncProject, writing only under `.kotowari/cache/overview/`
- Specification: `docs/ir/core/overview-data.md#REQ-core-279`, `docs/ir/core/overview-commands.md#REQ-core-293`, `docs/ir/core/overview-commands.md#REQ-core-294`, `docs/ir/core/overview-commands.md#REQ-core-295`, `docs/ir/core/overview-commands.md#REQ-core-296`, `docs/ir/core/cli.md#REQ-core-304`, `docs/ir/core/cli.md#REQ-core-001`, `docs/ir/core/cli.md#REQ-core-002`, `docs/ir/core/cli.md#REQ-core-004`, `docs/ir/core/cli.md#REQ-core-008`, `docs/ir/core/cli-scope.md#REQ-core-102`, `docs/ir/core/library-api.md#REQ-core-310`
- Prerequisites: S4
- May change: `crates/kotowari/`, root `src/`, root `tests/`, `agent/skills/kotowari/references/findings.md` (the Message table only), `Cargo.lock`
- Done when: EX-core-463, EX-core-475, EX-core-476 and EX-core-477 pass; build writes only changed files, removes pages no longer returned, stops with "overview error" and writes nothing when the data has errors, and stops with the config error of REQ-core-279 without the key; argument errors of REQ-core-304 and REQ-core-004 stop; `overview_prepare` writes nothing; Project and AsyncProject return the same written, removed and unchanged values; the REQ-core-127 test reads the wordings of core, the library and the CLI and passes
- Shown by: test — EX-core-463, EX-core-475, EX-core-476, EX-core-477, one CLI test per argument error of REQ-core-304, the existing tests of REQ-core-001, REQ-core-002, REQ-core-004, REQ-core-008 and REQ-core-102 kept passing or extended with the overview cases, a library test of `overview_prepare` and `overview_build` in both sync and async forms, and the REQ-core-127 test
- Left to the implementer: how the byte comparison against existing files is done
- Stop and hand back if: an unchanged page would have to be rewritten to keep the output of REQ-core-295 correct

### S6: show the pages with overview serve

- Purpose: add `kotowari overview serve`, which prepares, binds 127.0.0.1, writes, prints the URL and serves only the cache directory until interrupted
- Specification: `docs/ir/core/overview-commands.md#REQ-core-297`, `docs/ir/core/overview-commands.md#REQ-core-298`, `docs/ir/core/overview-commands.md#REQ-core-299`, `docs/ir/core/overview-commands.md#REQ-core-294`, `docs/ir/core/overview-commands.md#REQ-core-296`, `docs/ir/core/overview-data.md#REQ-core-279`, `docs/ir/core/overview-data.md#REQ-core-280`
- Prerequisites: S5
- May change: root `src/`, root `tests/`, root `Cargo.toml`, `Cargo.lock`, `agent/skills/kotowari/references/findings.md` (the Message table only)
- Done when: EX-core-478 and EX-core-479 pass; a request outside the cache, including through a symbolic link, gets 404; an interrupt ends the process with exit code 0; nothing is printed per request; serve on data with errors exits 2 with "overview error", binds no port and leaves the cache unchanged; serve without the "overview" key exits 2 with the config error of REQ-core-279; serve on an overlap of REQ-core-280 stops with its config error
- Shown by: test — EX-core-478 and EX-core-479 as process tests that start serve on a free port, read the URL line, send requests and interrupt it, plus one test each for a symbolic link that leaves the cache, the exit code after an interrupt, data with errors, the missing key and the overlap
- Left to the implementer: how the tests find a free port
- Stop and hand back if: `tiny_http` cannot refuse a path outside the cache before the file is opened, or `ctrlc` cannot make the interrupt exit with 0 on Linux and macOS

### S7: ship the two new packages like the others

- Purpose: put the two new crates under the same version, packaging and release checks as the existing packages
- Specification: `docs/ir/core/library-crates.md#REQ-core-306`, `docs/ir/core/library-crates.md#REQ-core-307`, `docs/ir/core/overview-data.md#REQ-core-287`
- Prerequisites: S1 to S6
- May change: `crates/kotowari-markdown-view/`, `crates/kotowari-overview/`, `Cargo.lock`, `scripts/check-versions.sh`, `scripts/release.sh`, `scripts/check-packages.py`, `scripts/tests/`, `.github/workflows/release.yml`, `.github/workflows/change-conformance.yml`, `README.md`, `PROJECT.md`, `CHANGELOG.md`, `crates/kotowari-markdown-schema/CHANGELOG.md`
- Done when: the normal dependency graph from `cargo tree` equals TBL-core-040; the view follows the schema-series version and the overview the kotowari-series version in `scripts/check-versions.sh`; `scripts/check-packages.py` builds both new packages from their own archives, builds an external usage example of each, and lists their embedded schemas as assets; no normal dependency of core, source-analysis or overview reaches file, network, process or Tokio code
- Shown by: check — `cargo tree -p <package> --edges normal --no-default-features --locked` for every package, `scripts/check-versions.sh`, `python3 -m unittest discover -s scripts/tests -p 'test_public_crate_*.py'`, `python3 scripts/check-packages.py --output "$PACKAGE_EVIDENCE"`, then a read of the call paths in `crates/kotowari-overview/src/` for file, network and environment access
- Left to the implementer: what the usage examples show, as long as each uses its package's public entry
- Stop and hand back if: the series assignment of record 2026-10-04 A4 cannot be expressed by the existing version automation without changing how the existing series are released

### S8: tell the skills when to draw the whole picture

- Purpose: write the brainstorm approval steps and the reference for writing overview data, and keep cycle and implement away from overview data
- Specification: `docs/ir/core/overview-workflow.md#REQ-core-300`, `docs/ir/core/overview-workflow.md#REQ-core-301`, `docs/ir/core/overview-workflow.md#REQ-core-302`, `docs/ir/core/overview-workflow.md#REQ-core-303`
- Prerequisites: S5
- May change: `agent/skills/kotowari-brainstorm/`, `agent/skills/kotowari/references/`, `agent/skills/kotowari/SKILL.md`, `agent/skills/kotowari-cycle/`, `agent/skills/kotowari-implement/`
- Done when: each how_to_verify of REQ-core-300 to REQ-core-303 is satisfied by the skill texts, the approval material and object of brainstorm are unchanged, and the steps run only in projects with the "overview" key
- Shown by: check — a separate-context review reads each how_to_verify against the changed skill texts and reports pass or fail per requirement, then `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references --locked` passes
- Left to the implementer: the wording and where in the references the overview scene sits
- Stop and hand back if: the approval steps would change what the person approves or the order of the approval material

### S9: measure build and check with many overview files

- Purpose: measure the time of check and build on synthetic data, as the overview record A40 asks instead of a numeric requirement
- Specification: `docs/ir/core/overview-commands.md#REQ-core-293`
- Prerequisites: S5
- May change: nothing in the repository; scratch files only
- Done when: the times of `kotowari check` and `kotowari overview build` on a copy of this repository with 100 generated overview files, and of check without them, are recorded with the machine and the command lines
- Shown by: artifact — a short measurement note in the session evidence, reported to the person with the numbers
- Left to the implementer: how the 100 files are generated, as long as they use every part kind and pass check
- Stop and hand back if: build or check with the data is so much slower than check without it that A40's replacement of the Markdown reader should be considered

### S10: reconcile the whole branch and hand it back

- Purpose: run the final gate on a fixed head and author both change records for the whole branch
- Specification: `docs/ir/core/overview-data.md#REQ-core-287`, `docs/ir/core/library-crates.md#REQ-core-306`
- Prerequisites: S1 to S9
- May change: `.kotowari/changes/implementation.yaml`, `.kotowari/changes/review.yaml`, `.kotowari/equivalents.yaml`
- Done when: every command of the Test command section exits 0 on the fixed head, except that a test-side finding outside this plan's items is reported rather than fixed; every item in the Verification map has a marked test or a passed review; the implementer record and a separately authored reviewer record cover every changed file since the base captured before S1; `changes --phase review` reports no finding
- Shown by: check — the commands of the Test command section in order, then `kotowari query ID` for each item of the Verification map
- Left to the implementer: none
- Stop and hand back if: the old public-crate records cannot be replaced without losing a review conclusion that is not repeated in the new reviewer record
