# Plan: remove change conformance and add the consistency phase

## Goal

kotowari no longer has the `changes` command, change records or the `changes` configuration section, this repository no longer runs or keeps them, and the kotowari skills run a consistency phase inside the cycle that reads the changed code against the IR and resolves gaps and disagreements on grounds and measurements, with review holding only the quality perspective.

## Specification

The IR store is `docs/ir/`. Read every requirement with `kotowari query ID`; never work from this plan's paraphrase. The governing decision record is [the changes-rethink record](../decision/records/2026-10-06-changes-rethink.md) (A1–A30; A1 and A7 are superseded and say so). The specification commit is `8cfff12` on branch `consistency-phase`.

New requirements, all verified by review (each `how_to_verify` line says what to look at):

| Document | Items |
|---|---|
| `docs/ir/core/consistency-phase.md` | REQ-core-359, REQ-core-360, REQ-core-361, REQ-core-362, REQ-core-363 |

Existing items whose text changed in the specification commit (compare with `git show 8cfff12 -- <file>`); their existing tests must keep passing, adjusted only where they pinned text the specification now replaces:

- `docs/ir/core/cli.md#REQ-core-001` (seven commands), `docs/ir/core/cli.md#REQ-core-003`, `docs/ir/core/cli.md#REQ-core-004`, and the scenarios EX-core-219 and EX-core-241 (seven command names)
- `docs/ir/core/cli-environment.md#TBL-core-020` (the expected-command message without `changes`)
- `docs/ir/core/config.md#REQ-core-016`, `docs/ir/core/config.md#REQ-core-019`, `docs/ir/core/config.md#TBL-core-004` (no `changes` keys) and EX-core-545 (now on `overview.files`)
- `docs/ir/core/library-api.md#TBL-core-041` and `docs/ir/core/library-api.md#REQ-core-312`, `docs/ir/core/library-inputs.md#REQ-core-316`, `docs/ir/core/library-inputs.md#REQ-core-317`, `docs/ir/core/library-inputs.md#TBL-core-042`, `docs/ir/core/library-crates.md#REQ-core-307` (verified by review), `docs/ir/core/library-crates.md#REQ-core-308`, `docs/ir/core/library-paths.md#REQ-core-322`, `docs/ir/core/library-async.md#REQ-core-320`

The removed items are REQ-core-240 to REQ-core-277 and EX-core-430 to EX-core-462 (record A23). Tests and code marks that point at them are deleted with the feature, not re-pointed.

## Approach and why

The repository's own use of change conformance goes first (S1), so that from then on this project is not "changes-enabled" and no skill station asks for change records while the rest of the plan runs. The product code goes next (S2): with the `changes` section gone from the configuration's schema, an old `changes:` section stops as an unknown key, which REQ-core-014's existing rule already covers (record A21: no transition). The skills follow (S3), then the guides, READMEs, site and changelog (S4), which describe the finished behavior.

Every check in this plan runs the repository's own binary, as the hooks do: `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- check --format json` (written `kotowari check` below), never an installed kotowari.

What S2 removes:

- `src/cli.rs`: the `changes` command, its options (`--base`, `--head`, `--phase`), `Cli::Changes`, `run_changes`, the help line, and the "git error" stop reason's use.
- `crates/kotowari-core`: `changes.rs`, `change_records.rs` (except the glob helper, below), `comparison.rs` if nothing else uses it, the change finding kinds (`FindingKind::Change*`), `StopReason::GitError` ("git error"), their match arms (for example in `inputs.rs`), the `#[path]` test modules in `src/lib.rs` that include the deleted core tests, and the change-record reading in check/status.
- `crates/kotowari`: `change_service.rs`, `git_snapshot.rs`, `git_snapshot_tests.rs`, `change_records.rs` (`read_texts`, called from `acquisition.rs` for check/status), the `changes` call in `acquisition.rs`, `ErrorKind::GitFailure`, and every public type and method that existed only for changes (`Project::changes`, `AsyncProject::changes`, `ChangesOptions`, `ChangesReport`, `Target`, `Phase`, `ChangesConfig`, `Comparison` and the like).
- Dependencies that become unused, in the crates' `Cargo.toml` and `Cargo.lock`.

The glob helper `change_records::glob` has two callers that stay: `crates/kotowari-core/src/test_side.rs` (test-side findings) and `crates/kotowari/src/test_files.rs` (`collect_named_hidden`, used for `overview.files`). Move it to a module that survives and drop the "changes.records" exception from its comment; keeping it for these callers is expected and does not trigger the first stop condition.

Two tests compare the code with the kotowari skill's `references/findings.md`: the test marked for REQ-core-125 (the Kind column against the finding kinds) and the test marked for REQ-core-127 (the stop table's Message column against the stop reasons, in `src/cli/tests.rs`). The rows of the removed finding kinds and the "git error" stop row leave that file in S2, in the same commit as the code.

Tests outside the deleted files also use the feature. In each, remove the changes-only cases and assertions and keep the cases for surviving requirements: `tests/native_paths.rs` (the "records" group writes a `changes:` configuration and expects change_record_invalid), `crates/kotowari/tests/project.rs` (`Project::changes`, `GitFailure`), `crates/kotowari/tests/async.rs` (`ChangesOptions`, `Phase`, `Target`, `AsyncProject::changes`), `src/cli/output.rs` (a status test with a `changes:` configuration and marks of removed IDs), `src/cli/tests.rs` (a "changes" argument list), `crates/kotowari-core/tests/admission.rs`. The only test marked EX-core-545 is in `crates/kotowari/tests/change_records.rs`, which is deleted: move it first into a surviving test file (for example `crates/kotowari/tests/project.rs`) and rewrite its example for `overview.files`. The three entries of `.kotowari/equivalents.yaml` that point at `crates/kotowari/src/git_snapshot.rs` are deleted with that file.

The consistency phase lives in `agent/skills/kotowari-cycle/` as steps of the cycle plus one reference file holding the instructions handed to the phase agent (record A17: no new skill). The cycle launches the phase as a delegate in a context separate from the implementer (record A29), in the order implementation, consistency phase, quality review. The phase agent itself edits and commits the IR, the decision record, the flag record and the code, writes a new decision record per run, and keeps the language pairs and `kotowari check` clean (REQ-core-363). `agent/skills/kotowari-review/` drops the conformance perspective and seat (REQ-core-362). Every skill's "Change conformance" section and the kotowari skill's `references/changes.md` go.

The source of the skills is `agent/skills/`; the copies installed under the user's home are not part of this plan.

## Scope of change

- `.kotowari/config.yaml` (the `changes` section), `.kotowari/changes/` (delete), `.ignore` (its `.kotowari/changes/` line), `.github/workflows/change-conformance.yml` (delete), `PROJECT.md` (the change conformance section and every mention of change records, the change-conformance workflow or guide, and `changes`)
- `src/cli.rs`, `src/cli/` (`output.rs`, `tests.rs`), `tests/change_cli.rs` (delete), `tests/native_paths.rs`, `tests/step1_cli.rs` and any other test that lists the commands or the expected-command message
- `crates/kotowari-core/src/`, `crates/kotowari-core/tests/` (delete `change_matching.rs` and `change_records.rs`; adjust `admission.rs`), `crates/kotowari-core/Cargo.toml`
- `crates/kotowari/src/`, `crates/kotowari/tests/` (delete `change_records.rs` after moving EX-core-545's test; adjust `project.rs` and `async.rs`), `crates/kotowari/Cargo.toml`, `crates/kotowari/README.md`, `Cargo.lock`
- `.kotowari/equivalents.yaml` (delete the entries for `git_snapshot.rs`; add others only if they pass the existing challenge rule)
- `agent/skills/` (kotowari and its `references/findings.md` and `references/overview.md`, kotowari-cycle, kotowari-review, kotowari-iterate, kotowari-using-workflow, kotowari-implement, kotowari-plan, kotowari-brainstorm, and `agent/skills/README.md` if it mentions changes)
- `docs/guides/` and their `.i18n.yaml` consistency records, `README.md`, `README.ja.md`, `CHANGELOG.md`
- `site/template.html`, `site/messages.json` (the `kotowari changes` card and its copy)

The IR and the decision records are approved and do not change in this plan, except what the consistency phase itself writes when it runs on this branch.

## Step order and prerequisites

S1 first; S2 needs S1; S3 needs S2; S4 needs S3; S5 comes last. One writer works the steps in order on branch `consistency-phase`.

Before S1, record the comparison base once with `git merge-base origin/main HEAD` (it was `d18bdae9c1fe9dcefc30f319e1f1ac5787239c6a` when this plan was written). Run the baseline: the test command must pass, and `kotowari check --format json` must report exactly 133 unresolved_reference errors (all in test files and `src/cli/output.rs`, on the removed IDs), 70 guide_stale notices (all in `docs/guides/`), 9 size notices, and nothing else.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | none (records A10, A22) | none |
| S2 | REQ-core-001, REQ-core-003, REQ-core-004, TBL-core-020, REQ-core-016, REQ-core-019, TBL-core-004, TBL-core-041, REQ-core-312, REQ-core-316, REQ-core-317, TBL-core-042, REQ-core-308, REQ-core-322, REQ-core-320 | EX-core-219, EX-core-241, EX-core-545 |
| S3 | REQ-core-359, REQ-core-360, REQ-core-361, REQ-core-362, REQ-core-363 (review) | none |
| S4 | REQ-core-001, TBL-core-004 (texts describing them) | none |
| S5 | all of the above, and REQ-core-307 (review) | all of the above |

## Left to the implementer

- Where the surviving glob helper lives, and module and type names, within the layering of `docs/ir/core/library-crates.md#REQ-core-307`.
- The surviving test file EX-core-545's test moves to.
- The name of the consistency phase's reference file under `agent/skills/kotowari-cycle/references/`, and the wording of every skill, guide and site text.
- The CHANGELOG wording, as long as the removal is marked BREAKING, names the removed command, configuration section and the removed public Rust API of `kotowari` and `kotowari-core`, and tells a user to delete the `changes` section and `.kotowari/changes/`.

## Stop conditions

- A public type or function of `kotowari` or `kotowari-core` that changes used turns out to be used by another command or by the overview beyond the glob helper named above, so removing it would change a requirement this plan does not list.
- An existing test fails for a reason other than the command count, the expected-command message, EX-core-545's new example, or the changes-only cases named in Approach.
- A skill text needs a rule that REQ-core-359 to REQ-core-363 and the decision record do not state (for example how the phase's findings enter the cycle's findings file beyond what REQ-core-363 says): hand back to brainstorm rather than inventing it.
- The four general conditions: missing meaning or departure from approved content; an irreversible, privileged or dangerous operation; a spreading accident; no progress after a changed approach.

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked`. Format only the Rust files you touched with `rustfmt --edition 2024 <file>`; never run `cargo fmt --all`. Tests must not wait on real time. Pass `< /dev/null` when running the pre-push hook by hand.

## Out of scope

- Fixing other repositories of the user that configure `changes` (record A21); the main session does that after this branch is merged.
- Undecided U1 of the record (a specification commit that removes behavior being stopped by the surface check).
- `docs/spec/public-crate-api.md`, a design proposal kept as it was written, not a current description.
- Pointing the site's guide links at a newer release tag (a separate item in TODO.md).
- Installing the changed skills into the user's home.

## Steps

### S1: stop using change conformance in this repository

- Purpose: remove this repository's change conformance setup so no station asks for change records from here on.
- Specification: docs/decision/records/2026-10-06-changes-rethink.md#A10, docs/decision/records/2026-10-06-changes-rethink.md#A22
- Prerequisites: the baseline above passes
- May change: .kotowari/config.yaml, .kotowari/changes/, .ignore, .github/workflows/change-conformance.yml, PROJECT.md
- Done when: the configuration has no `changes` key, `.kotowari/changes/` and the change-conformance workflow are gone, PROJECT.md mentions neither change records, the change-conformance workflow or guide, nor the `changes` command, and `kotowari check` reports the same counts as the baseline
- Shown by: check — `kotowari check --format json | jq -c .counts` equals the baseline counts, and `rg -n -i "change[ -]record|change[ -]conformance|\bchanges\b|\.kotowari/changes" PROJECT.md .kotowari .github .ignore` prints only hits reviewed by hand as the plain English word
- Left to the implementer: none
- Stop and hand back if: another workflow or script depends on the change-conformance workflow or on `.kotowari/changes/`

### S2: remove the changes command and its code

- Purpose: delete the `changes` command, change records, the `changes` configuration keys, the change finding kinds, the "git error" stop reason and the library code that served only them, and bring the remaining tests to the changed requirements.
- Specification: docs/ir/core/cli.md#REQ-core-001, docs/ir/core/cli.md#REQ-core-004, docs/ir/core/cli-environment.md#TBL-core-020, docs/ir/core/config.md#TBL-core-004, docs/ir/core/config.md#REQ-core-019, docs/ir/core/library-api.md#TBL-core-041, docs/ir/core/library-api.md#REQ-core-312, docs/ir/core/library-inputs.md#TBL-core-042, docs/ir/core/library-crates.md#REQ-core-308
- Prerequisites: S1
- May change: src/cli.rs, src/cli/, tests/, crates/kotowari-core/, crates/kotowari/ (its README in S4), Cargo.lock, agent/skills/kotowari/references/findings.md (only the Kind rows of removed finding kinds and the "git error" stop row), .kotowari/equivalents.yaml
- Done when: `kotowari changes` stops as an unknown command with the seven-name expected-command message, a configuration with a `changes:` section stops as an unknown key, no code or test refers to change records, git snapshots or the "git error" stop, `kotowari check` reports no unresolved_reference, and every remaining test passes
- Shown by: test — the existing tests marked for EX-core-219 and EX-core-241 updated to the seven names, each seen failing against the old code and then passing; the test marked for EX-core-545 moved and rewritten for `overview.files`, shown to fail when the brace-only exclusion in `collect_named_hidden` is removed and to pass with it; the tests of REQ-core-125 and REQ-core-127; then the test command
- Left to the implementer: where the surviving glob helper goes, and the file EX-core-545's test moves to
- Stop and hand back if: removing a type or function breaks a non-changes caller other than the glob helper's two callers

### S3: the consistency phase and a quality-only review in the skills

- Purpose: add the consistency phase to the cycle with its reference, launched as a separate-context delegate, make review quality-only, route iterate and the main session through the phase, and remove change conformance from every skill.
- Specification: docs/ir/core/consistency-phase.md#REQ-core-359, docs/ir/core/consistency-phase.md#REQ-core-360, docs/ir/core/consistency-phase.md#REQ-core-361, docs/ir/core/consistency-phase.md#REQ-core-362, docs/ir/core/consistency-phase.md#REQ-core-363
- Prerequisites: S2
- May change: agent/skills/
- Done when: every point of the how_to_verify lines of REQ-core-359 to REQ-core-363 is written in the named skill files, the cycle launches the phase as a delegate in a context separate from the implementer, `agent/skills/kotowari/references/changes.md` is gone, and no skill mentions change records, the `changes` command, changes-enabled projects or a conformance reviewer
- Shown by: artifact — the skill files under agent/skills/, judged against the five how_to_verify lines by an independent review; `rg -n -i "change[ -]record|changes-enabled|change[ -]conformance|\bchanges\b|conformance" agent/skills` prints only hits reviewed by hand as unrelated; the tests of REQ-core-125 and REQ-core-127 pass
- Left to the implementer: the reference file's name and the wording
- Stop and hand back if: a station's hand-off (what cycle passes to review, fixer or the phase) needs a rule the specification does not state

### S4: guides, READMEs, site and changelog

- Purpose: bring the guides, READMEs and site up to the seven commands, the configuration without `changes` and the stop reasons without "git error", and record the removal and the consistency phase in the changelog.
- Specification: docs/ir/core/cli.md#REQ-core-001, docs/ir/core/config.md#TBL-core-004, docs/ir/core/cli-environment.md#TBL-core-020
- Prerequisites: S3
- May change: docs/guides/, README.md, README.ja.md, crates/kotowari/README.md, site/template.html, site/messages.json, CHANGELOG.md
- Done when: no guide, README or site text mentions the `changes` command, change records or the "git error" stop; the unknown-field samples in the guides list the current configuration keys; every guide pair is aligned with its consistency record updated from `kotowari list`; the site check passes; and the changelog's Unreleased section has a BREAKING entry for the removal and an entry for the consistency phase
- Shown by: check — `kotowari check --format json | jq '[.findings[] | select(.path | test("docs/guides/|README"))]'` prints an empty list; `rg -n -i "\bchanges\b|change[ -]record|照合|git error|change-conformance" docs/guides README.md README.ja.md crates/*/README.md site` prints only hits reviewed by hand as unrelated; `python3 site/check.py` passes
- Left to the implementer: wording
- Stop and hand back if: a guide section describes behavior that no current requirement states and none replaces

### S5: final checks

- Purpose: confirm the plan's own items are done.
- Specification: docs/ir/core/consistency-phase.md#REQ-core-359, docs/ir/core/library-crates.md#REQ-core-307, docs/ir/core/cli.md#REQ-core-001
- Prerequisites: S1 to S4
- May change: nothing beyond fixes inside the scope above
- Done when: the test command passes; `kotowari query` shows a nonempty tests list for EX-core-219, EX-core-241 and EX-core-545; `kotowari check` reports no error and no guide_stale; an independent review has confirmed REQ-core-359 to REQ-core-363 and REQ-core-307 by their how_to_verify lines
- Shown by: check — the test command, then `kotowari check --format json`
- Left to the implementer: none
- Stop and hand back if: the mutants run of the pull request finds survivors that no remaining requirement's test can catch
