# Plan: the next release bundle — config directories, the unknown ID, the typed-extraction wording and a narrower public API

## Goal

A library user gets the same failure kind for a configuration file that is a directory as for one that does not exist, can take the unknown ID out of a query failure, sees stop messages that no longer name a CLI subcommand inside the schema engine, and finds only the public API that external users rely on, with every CLI output kept except the one directory message.

## Specification

The IR is in `docs/ir/`. The approved decision record is [2026-10-23-next-release-bundle](../decision/records/2026-10-23-next-release-bundle.md), committed on this branch (`feat/next-release-bundle`, commit `7f97b96`). Read the whole record before S1; this plan points to its decisions as `#A<n>` and never restates them. A19 is a prohibition that holds for every step.

This plan covers:

- `docs/ir/core/library-api.md#REQ-core-312`, with its scenarios EX-core-554, EX-core-555, EX-core-556 and EX-core-557 (no test yet)
- `docs/ir/core/library-crates.md#REQ-core-309` and its scenario EX-core-558 (verified by review)
- `docs/ir/schema/library.md#REQ-schema-051` (verified by review)
- `docs/ir/schema/cli.md#TBL-schema-009` (already tested; only the wording of one condition changed)
- `docs/ir/core/cli.md#REQ-core-004`, whose behaviour must stay as it is (S1 moves where the directory check runs)

Decisions that change code but are deliberately not in the IR (#A16): #A3, #A10, #A11, #A13, and the error-precedence consequence #A14. They are built here and written in the changelogs.

## Approach and why

The library becomes the one place that judges a `--config` target (#A2, #A14): `load_config` in `crates/kotowari/src/acquisition.rs` already stops on a missing file with an argument error, which the library maps to `ErrorKind::InvalidInput`; S1 adds the directory case next to it with the path as given (#A17) and removes the CLI's own check in `src/cli/args.rs`. The CLI then prints whatever the library returns, so its message changes and nothing else does. The default location (`None` branch) is left alone (#A15); EX-core-555 pins that existing behaviour, so its test passes from the start.

The two string round trips are replaced by carrying the value itself. For #A4 and #A7, `kotowari::Error` keeps the unknown ID when it is built from `InputError::UnknownQuery`, and `src/cli.rs` builds its `unknown id: <ID>` message from `unknown_id()` instead of stripping `unknown query: `. `ErrorKind` stays exactly as it is (#A4 is non-breaking), and `crates/kotowari/src/asynchronous.rs`, which builds `Error` values behind the `tokio` feature, must keep compiling. For #A3, `Analyzer::new` puts the bare detail into `AnalysisError::InvalidRules`, so `detail()` no longer strips anything; its returned values must stay exactly as they are today, because `crates/kotowari/src/test_files.rs` and `crates/kotowari/src/surface.rs` turn them into the CLI's config errors. The rule loader returns only configuration errors, so the bare detail loses nothing.

The API narrowing follows #A6 within the scope of #A8, one crate family per step: the `kotowari` facade (S5), `kotowari-core` and the `kotowari-source-analysis` code that fills its types (S6), then `kotowari-markdown-schema` (S7). "A public signature of `kotowari`" below means the items `kotowari` defines itself (`Project`, `AsyncProject`, `ProjectOptions`, `MutantsOptions`, `Error` and the like); core types that `kotowari` re-exports keep their own methods, such as `ReadModel::build` taking `kotowari_core::ReadInputs`, which is how in-memory users already work (#A12). Facts found while writing this plan that the steps rely on:

- `crates/kotowari/examples/identity.rs` proves re-export identity with `kotowari::SourceText` and `kotowari::ReadInputs`, both removed by #A12. `scripts/check-packages.py` copies this example into an independent consumer, so S5 rewrites it against a type that stays re-exported and reachable (for example `kotowari::Config` or `kotowari::IrDocument`), keeping what it demonstrates: a value of the `kotowari` name is accepted where the `kotowari_core` type is taken.
- `Tool` reaches `crates/kotowari/src/acquisition.rs` through its `use kotowari_core::*` glob and is used in `run_mutants`; once S5 defines `Tool` in `kotowari`, that file imports it from the crate itself.
- Thirteen tests in `tests/step3_sources_terms.rs` call `kotowari_core::terms::check_document_references` directly; all thirteen mark TBL-core-014, and seven of them also mark REQ-core-069. S6 makes that function crate-private, so those tests move into `kotowari-core` with their assertions and marks unchanged. Core keeps its internal contract tests in `crates/kotowari-core/tests/` and includes them as `#[cfg(test)] #[path = "../tests/<file>.rs"] mod …;` in `crates/kotowari-core/src/lib.rs`; the moved tests follow that convention. `tests.files` in `.kotowari/config.yaml` includes `crates/kotowari-core/**/*.rs`, so the marks keep counting. Today no test of TBL-core-014 lives under `crates/kotowari-core/`.
- `GlossaryDuplicates`, which #A11 makes crate-private, is a parameter of `pub fn check_sources_with_duplicates` in the public module `kotowari_core::sources`, and that function is called only from `crates/kotowari-core/src/inputs/read.rs`. A public function cannot take a crate-private type without the `private_interfaces` lint, which the gates treat as an error. The function is the same kind of intermediate part as #A11's `check_documents_with_duplicates` (its public counterpart for the outside is the in-memory check), so this plan makes it crate-private as well, as the same-reason sibling that #A8's scope includes.
- The unused parameter of `check_unknown_terms` (#A8) is passed down from `check_terms_and_vague_words_with_duplicates`, whose core-internal callers in `crates/kotowari-core/src/inputs/read.rs` and `crates/kotowari-core/src/translations/ir_sides.rs` compute an ID set for it. S6 removes the parameter along that chain, and the computation wherever nothing else uses it.
- `TestAnalysis` and `SurfaceAnalysis` are built with struct literals in `crates/kotowari-source-analysis/src/lib.rs` and in several tests under `crates/kotowari-core/tests/`; S6 removes the `language` field from all of them.
- `docs/ir/core/library-inputs.md#TBL-core-042` still lists "the language" among the contents of the test and surface input groups. After S6 that wording no longer matches the types. This plan does not edit the IR; the cycle's consistency phase brings TBL-core-042 (both language sides) in line, with #A10 as its grounds.
- `docs/guides/cli.md` and `docs/guides/cli.ja.md` show the old CLI directory message as an example; S1 updates both sides and their consistency record.

Every pull request runs the mutation tests on the diff (`.github/workflows/mutants.yml`). Changing a function's signature line, for example from `pub fn` to `pub(crate) fn`, puts the whole function in that diff. The functions touched here are already exercised by the workspace's tests, but a survivor is possible; the cycle handles it like any other mutation finding.

Changelog entries go under `## [Unreleased]` on this branch, in English, as PROJECT.md requires: `CHANGELOG.md` for the kotowari product (CLI, kotowari, core, source-analysis, overview) and `crates/kotowari-markdown-schema/CHANGELOG.md` for kotowari-mds. The categories follow #A18. Each step writes the entries for its own change.

## Scope of change

- `crates/kotowari/src/acquisition.rs`
  - `load_config`, and the import of `Tool`
- `crates/kotowari/src/lib.rs`
  - the re-exports, `Error`, and the new home of `Tool`
- `crates/kotowari/src/asynchronous.rs`
  - only if its `Error` values need the new field
- `crates/kotowari/examples/identity.rs`
- `crates/kotowari/tests/project.rs`
- `crates/kotowari-source-analysis/src/lib.rs`, and `src/test_queries.rs` only if the bare detail is easier to produce there
- `crates/kotowari-core/src/`
  - `lib.rs` (Tool, the `terms` module, the new test module), `terms.rs`, `ir.rs`, `ir/fence.rs`, `ir/check.rs`, `ir/model.rs`, `sources.rs`, `inputs/types.rs`, `inputs/read.rs`, `translations/ir_sides.rs`, and any file that only needs its imports adjusted
- `crates/kotowari-core/tests/`
  - a new file for the moved tests, and removing the `language` field from struct literals
- `crates/kotowari-core/examples/`
  - removing the `language` field from struct literals only
- `crates/kotowari-markdown-schema/src/finding.rs` and `src/schema/check.rs`
- `src/cli/args.rs`, `src/cli.rs`
- `tests/step3_sources_terms.rs`
  - removing the thirteen moved tests only
- `docs/guides/cli.md`, `docs/guides/cli.ja.md`, `docs/guides/cli.i18n.yaml`
- `CHANGELOG.md`, `crates/kotowari-markdown-schema/CHANGELOG.md`

## Step order and prerequisites

S1 to S4 are independent of each other. S1 and S2 both edit `crates/kotowari/tests/project.rs`, and S1 to S3 all write to `CHANGELOG.md`, so they run in order; S4 touches only the schema crate and its changelog and is placed after them for a simple sequence. S5 and S6 both edit `crates/kotowari-core/src/lib.rs` (S5 removes `Tool` from it, S6 makes `terms` private and adds the test module), so S5 comes first and S6 follows; neither needs the other's result. S7 is independent and comes after S6 only to keep one crate family per step. S8 runs last.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-312 (REQ-core-004 unchanged) | EX-core-554, EX-core-555 |
| S2 | REQ-core-312 | EX-core-556, EX-core-557 |
| S3 | none in the IR (#A3) | none |
| S4 | TBL-schema-009 | none new |
| S5 | REQ-core-309 (review) | EX-core-558 (review) |
| S6 | REQ-core-309 (review) | EX-core-558 (review) |
| S7 | REQ-schema-051 (review) | none |
| S8 | all of the above | EX-core-554 to EX-core-557 |

## Left to the implementer

- How `kotowari::Error` stores the unknown ID (a private field of `Error`; never a change to `ErrorKind`) and how the existing constructors fill it
- Whether the bare detail for #A3 is produced in `Analyzer::new` or returned by `test_queries`
- Which reachable re-exported type the identity example uses
- How the crate-private items are imported inside `kotowari-core` once the `pub use` globs no longer carry them, and the name of the new test file
- The wording of each changelog entry, within the categories of #A18, and the category of the library's directory classification change in S1

## Stop conditions

- Removing a re-export in S5 makes an item that `kotowari` defines itself take or return a type that is no longer nameable from `kotowari`: that type is reachable from an entry point, which contradicts #A12
- An item that S6 or S7 makes crate-private is used by another crate's non-test code, or making it crate-private would force another public item to change beyond those this plan names: #A8, #A9 and #A11 rested on them being used only inside their own crate
- A test moved in S6 cannot be expressed inside `kotowari-core` without changing what it asserts
- `AnalysisError::detail()` returns a different value after S3 for any input, or a CLI config-error output for a broken `tests.rules` or `surface.rules` changes
- Any CLI output other than the `--config` directory message changes, apart from which message comes first when argument errors overlap (#A14)

## Out of scope

- The releases themselves: version bumps, tags, `scripts/release.sh`, and arranging the kotowari-mds changelog into its pre-import 0.1.0 and post-import sections (#A1, #A19)
- A full audit of the public API of the nine crates (#A34)
- Editing the IR, including TBL-core-042 (the consistency phase owns it)
- A test that pins the CLI directory message or the schema engine's collision message: neither wording is declared as a contract, so such a test would not meet the Evidence conditions
- `scripts/check-packages.py` itself; running it is not part of this plan
- The notices `kotowari check` already reported before this plan

## Steps

### S1: classify a configuration file that is a directory in the library

- Purpose: make a `ProjectOptions.config` that names a directory fail with `InvalidInput` like a missing one, and let the CLI print the library's message instead of checking on its own
- Specification: `docs/ir/core/library-api.md#REQ-core-312`, `docs/ir/core/cli.md#REQ-core-004`; decisions #A2, #A14, #A15, #A17
- Prerequisites: none
- May change: `crates/kotowari/src/acquisition.rs`, `crates/kotowari/tests/project.rs`, `src/cli/args.rs`, `docs/guides/cli.md`, `docs/guides/cli.ja.md`, `docs/guides/cli.i18n.yaml`, `CHANGELOG.md`
- Done when: a project whose `ProjectOptions.config` names a directory returns an error of kind `InvalidInput` from `check`; a project with no configuration file given and a directory at `.kotowari/config.yaml` returns `ReadFailure`; `kotowari check --config .` still exits 2 with empty standard output and its first standard-error line is `argument error: config file is a directory: .`; `src/cli/args.rs` no longer checks whether the `--config` target is a directory; both guide sides show the new message and `kotowari check` reports no translation finding for them; `CHANGELOG.md` has entries for the CLI message and the library classification
- Shown by: test — EX-core-554 (a directory given through `ProjectOptions.config` fails with `InvalidInput`, the same kind as a missing file; red before the change) and EX-core-555 (a directory at the default location fails with `ReadFailure`; pins existing behaviour, green from the start), both in `crates/kotowari/tests/project.rs`; the existing tests `req_004_config_pointing_to_directory_stops` (`tests/step1_cli.rs`) and `req_core_190_config_pointing_to_a_directory_on_plan_stops` (`tests/step14_plan.rs`) keep passing; the message is observed with `cd fixtures/valid-project && CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- check --config . 2>&1 >/dev/null | head -1`
- Left to the implementer: none
- Stop and hand back if: a CLI path reads the configuration without going through `load_config`, so removing the CLI check would let a directory target through unchecked

### S2: carry the unknown ID in the library's error

- Purpose: add `kotowari::Error::unknown_id` and build the CLI's `unknown id: <ID>` message from it instead of stripping a prefix
- Specification: `docs/ir/core/library-api.md#REQ-core-312`; decisions #A4, #A7
- Prerequisites: S1
- May change: `crates/kotowari/src/lib.rs`, `crates/kotowari/src/asynchronous.rs`, `crates/kotowari/tests/project.rs`, `src/cli.rs`, `CHANGELOG.md`
- Done when: `query` with an ID no item or scenario has returns an error of kind `UnknownQuery` whose `unknown_id()` is `Some` of that ID; an error of any other kind returns `None`; `ErrorKind` is unchanged; `src/cli.rs` no longer calls `strip_prefix("unknown query: ")`; `kotowari query REQ-999` in a project without that ID still prints `argument error: unknown id: REQ-999` as its first standard-error line; `CHANGELOG.md` lists `unknown_id` under Added
- Shown by: test — EX-core-556 (the unknown ID is taken from a failed query) and EX-core-557 (a read failure provides no unknown ID), both in `crates/kotowari/tests/project.rs`; the existing CLI tests for REQ-core-157 keep passing; `CARGO_BUILD_JOBS=4 cargo build --workspace --all-features` compiles the `tokio` path
- Left to the implementer: none
- Stop and hand back if: `InputError::UnknownQuery` can carry something other than the queried ID, so the value to return is not determined

### S3: put the bare detail into `AnalysisError`

- Purpose: stop prefixing the rule error with `config error: ` only to strip it again, so `detail()` returns the stored detail as it is
- Specification: `docs/decision/records/2026-10-23-next-release-bundle.md#A3` (not in the IR, #A16)
- Prerequisites: S2
- May change: `crates/kotowari-source-analysis/src/lib.rs`, `crates/kotowari-source-analysis/src/test_queries.rs`, `CHANGELOG.md`
- Done when: `AnalysisError::detail()` contains no `strip_prefix`; its values are the same as before for every input; `Display` gives `invalid rules: ` followed by the detail; `CHANGELOG.md` lists the `Display` change under Changed
- Shown by: check — `rg -n 'strip_prefix\("config error: "\)' crates/kotowari-source-analysis/src` prints nothing; `CARGO_BUILD_JOBS=4 cargo test --workspace` passes, including `crates/kotowari-source-analysis/tests/analyzer.rs` and the CLI tests for broken `tests.rules` and `surface.rules` (REQ-core-189, REQ-core-225)
- Left to the implementer: none
- Stop and hand back if: the rule loader turns out to return a stop reason other than a configuration error, so a bare detail would lose which reason it was

### S4: word the type collision by the typed extraction

- Purpose: make the schema engine's collision message name the typed extraction instead of the CLI's `ast --schema`
- Specification: `docs/ir/schema/cli.md#TBL-schema-009`; decision #A5
- Prerequisites: S3
- May change: `crates/kotowari-markdown-schema/src/schema/check.rs`, `crates/kotowari-markdown-schema/CHANGELOG.md`
- Done when: the collision message in `src/schema/check.rs` is the one #A5 gives; doc comments elsewhere in the crate that mention `ast --schema` are left as they are; `crates/kotowari-markdown-schema/CHANGELOG.md` has an entry under Changed
- Shown by: check — `rg -n 'that ast --schema sets' crates/kotowari-markdown-schema/src` prints nothing; `CARGO_BUILD_JOBS=4 cargo test -p kotowari-markdown-schema` and `CARGO_BUILD_JOBS=4 cargo test -p kotowari-mds` pass
- Left to the implementer: none
- Stop and hand back if: a test outside the schema crate asserts the old message text

### S5: narrow and complete the `kotowari` facade

- Purpose: stop re-exporting the six core input types no entry point reaches, re-export `GuideReader`, and move `Tool` into `kotowari`
- Specification: `docs/ir/core/library-crates.md#REQ-core-309`; decisions #A6, #A8, #A12, #A13
- Prerequisites: S4
- May change: `crates/kotowari/src/lib.rs`, `crates/kotowari/src/acquisition.rs`, `crates/kotowari/examples/identity.rs`, `crates/kotowari/tests/project.rs`, `crates/kotowari-core/src/lib.rs`, files under `crates/kotowari/src/` and `src/` that only need their imports adjusted, `CHANGELOG.md`
- Done when: `kotowari` no longer re-exports `CheckInputs`, `ReadInputs`, `IrOptions`, `SourceText`, `TestAnalysis` or `SurfaceAnalysis`; `kotowari::GuideReader` is the same type as the one `ReadModel::guide_reader` returns, shown by adding it to the existing facade test `facade_names_are_the_same_lower_result_types` in `crates/kotowari/tests/project.rs`; `Tool` is defined in `kotowari` and no longer exists in `kotowari-core`, and `kotowari::Tool::CargoMutants` still names it; the identity example uses only re-exports that remain and still passes a `kotowari` value where the `kotowari_core` type is taken; `CHANGELOG.md` lists the removed re-exports and the move of `Tool` out of `kotowari-core` as BREAKING (Rust API) and the `GuideReader` re-export as Added
- Shown by: check — `rg -n '\b(CheckInputs|ReadInputs|IrOptions|SourceText|TestAnalysis|SurfaceAnalysis)\b' crates/kotowari/src/lib.rs` shows no re-export of them; `rg -n 'pub enum Tool' crates/kotowari-core/src` prints nothing; `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets --all-features -- -D warnings`; `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari --example identity` exits 0; `CARGO_BUILD_JOBS=4 cargo test --workspace`
- Left to the implementer: none
- Stop and hand back if: an item that `kotowari` defines itself takes or returns one of the six types, or `kotowari-core` itself uses `Tool`

### S6: make core's internal parts crate-private and drop the unread fields

- Purpose: apply #A6 to `kotowari-core` within the scope of #A8, and stop filling the `language` fields nobody reads
- Specification: `docs/ir/core/library-crates.md#REQ-core-309`; decisions #A6, #A8, #A10, #A11
- Prerequisites: S5
- May change: `crates/kotowari-core/src/`, `crates/kotowari-core/tests/`, `crates/kotowari-core/examples/`, `crates/kotowari-source-analysis/src/lib.rs`, `tests/step3_sources_terms.rs`, `CHANGELOG.md`
- Done when: the `terms` module and its four functions, `CodeFence`, `parse_opening_fence`, `is_closing_fence`, `check_documents_with_duplicates`, `GlossaryDuplicates` and `check_sources_with_duplicates` are not nameable from outside `kotowari-core`; `check_documents` stays public; the unused parameter of `check_unknown_terms` is gone along the chain described in Approach, and `CodeFence` has no unread `lang` field; `TestAnalysis` and `SurfaceAnalysis` have no `language` field and `kotowari-source-analysis` no longer computes one; the thirteen tests that called `check_document_references` from `tests/step3_sources_terms.rs` live inside `kotowari-core` with the same assertions and marks; `CHANGELOG.md` lists the removed core items and fields as BREAKING (Rust API)
- Shown by: check — `rg -n '^pub (mod terms|fn (check_unknown_terms|check_vague_words|check_document_references|check_terms_and_vague_words_with_duplicates|check_documents_with_duplicates|check_sources_with_duplicates|parse_opening_fence|is_closing_fence)|struct (CodeFence|GlossaryDuplicates))\b' crates/kotowari-core/src` prints nothing; `rg -n 'pub language' crates/kotowari-core/src` prints nothing; `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets --all-features -- -D warnings`; `CARGO_BUILD_JOBS=4 cargo test --workspace`
- Left to the implementer: none
- Stop and hand back if: `kotowari-overview`, `kotowari`, `kotowari-source-analysis` or the CLI uses one of these items in non-test code

### S7: let `Finding` of the schema crate be made only inside it

- Purpose: make the constructors of the schema crate's `Finding` crate-private, so external code only reads findings from validation and extraction results
- Specification: `docs/ir/schema/library.md#REQ-schema-051`; decisions #A6, #A9
- Prerequisites: S6
- May change: `crates/kotowari-markdown-schema/src/finding.rs`, `crates/kotowari-markdown-schema/CHANGELOG.md`
- Done when: `Finding::new`, `at`, `maybe_at`, `of_node` and `of_rule` of `kotowari-markdown-schema` are not callable from outside the crate; `crates/kotowari-markdown-schema/CHANGELOG.md` lists them as BREAKING (Rust API)
- Shown by: check — `rg -n 'pub fn (new|at|maybe_at|of_node|of_rule)\b' crates/kotowari-markdown-schema/src/finding.rs` prints nothing; `CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets --all-features -- -D warnings`; `CARGO_BUILD_JOBS=4 cargo test --workspace`
- Left to the implementer: none
- Stop and hand back if: `kotowari-markdown-schema-io`, `kotowari-mds`, `kotowari-core` or `kotowari-overview` constructs a schema `Finding` in non-test code

### S8: confirm the plan's own work

- Purpose: confirm that every scenario this plan covers has a marked test and that the branch leaves no error of its own
- Specification: `docs/ir/core/library-api.md#REQ-core-312`, `docs/ir/core/library-crates.md#REQ-core-309`, `docs/ir/schema/library.md#REQ-schema-051`, `docs/ir/schema/cli.md#TBL-schema-009`
- Prerequisites: S1 to S7
- May change: nothing beyond fixes inside the files the earlier steps may change
- Done when: `kotowari query` for EX-core-554, EX-core-555, EX-core-556 and EX-core-557 each shows a nonempty `tests`; `kotowari check` reports no error on a file this branch changed or on an ID this plan covers; `scripts/gates.sh` passes
- Shown by: check — `for id in EX-core-554 EX-core-555 EX-core-556 EX-core-557; do CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- query $id --format json | jq -e '.items[0].tests | length > 0' || exit 1; done`, then `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- check --format json`, then `scripts/gates.sh`
- Left to the implementer: none
- Stop and hand back if: `scripts/gates.sh` fails on a file this branch did not change
