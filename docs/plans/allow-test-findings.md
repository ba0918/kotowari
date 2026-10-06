# Plan: let check pass a commit whose tests are not written yet

## Goal

A project can put `kotowari check --allow-test-findings` in its pre-commit hook so that the commit approving a specification, made before its tests exist, is not stopped, while errors outside the test-side findings still stop it and the pre-push hook and CI keep running `kotowari check` without the option.

## Specification

The IR store is `docs/ir/`. Read every requirement with `kotowari query ID`, including its scenarios; never work from this plan's paraphrase. The governing decision record is [the spec-first-commit record](../decision/records/2026-10-06-spec-first-commit.md) (A1–A12, R1). The specification commit is `593e939` on branch `allow-test-findings`.

New items this plan must make tested:

| Document | Items |
|---|---|
| `docs/ir/core/test-side-findings.md` | REQ-core-357, TBL-core-047, EX-core-547 to EX-core-552 |

Requirement verified by review: `docs/ir/core/test-side-findings.md#REQ-core-358` (hook guidance in the kotowari skill, kotowari-adopt and the check guide). Its `how_to_verify` line says what to look at.

Existing items whose text changed in the specification commit (compare with `git show 593e939 -- docs/ir/core/cli.md`); their existing tests must keep passing:

- `docs/ir/core/cli.md#REQ-core-002` ("check" also accepts "--allow-test-findings", which takes no value)
- `docs/ir/core/cli.md#REQ-core-004` ("--allow-test-findings" on another command is an argument error)
- `docs/ir/core/cli.md#TBL-core-002` (the exit-code rows for "check" with the option)
- The glossary terms 誤り / error and テスト側の指摘 / test-side finding in `docs/ir/core/CONTEXT.md`

## Approach and why

Which error is a test-side finding is decided once, in the library, from TBL-core-047, so no hook and no CLI code carries a list of kinds (record A1, A9). The conditions on invalid_marker, unresolved_reference and unparsable_file need the set of test files and the set of surface files. Neither is kept in the result: `CheckResult` (`crates/kotowari-core/src/lib.rs`) holds test files only as a tally per extension, and when a file is both a test file and a surface file the surface side's unparsable_file is merged into the test side's (`crates/kotowari-core/src/surface.rs`), so the finding alone cannot tell them apart. The sets exist only while the check runs (the discovered test files and the surface analysis input, joined in `CheckPreparation` in `crates/kotowari-core/src/inputs.rs`), so the classification is computed or the sets are stored there, before `CheckReport` is built. `src/cli.rs` only passes the flag and turns the answer into the exit code (`exit_code_for` today counts every error). The findings themselves, their order and the JSON and text output stay byte-for-byte as without the option (`docs/ir/core/test-side-findings.md#REQ-core-357`, record A4).

Argument parsing in `src/cli.rs` (`parse_args`) already has one flag without a value, "--staged", with its own repeated-option check; "--allow-test-findings" follows that pattern. Its rejection on other commands follows the placement of the "--port" check, which runs before the early return for "changes" and so covers every command; the "--tool" check runs only for commands other than "mutants" and after "changes" has returned, so copying it would leave "changes" and "mutants" accepting the flag (record A12).

Once the option exists, this repository's own `lefthook.yml` pre-commit drops its jq filter, whose kind list was wider than TBL-core-047 (record A2, A8). The hook is not IR; its step names decisions instead of requirements.

The 48 guide_stale notices on the branch come from the specification commit changing REQ-core-002, REQ-core-004 and TBL-core-002 (and the glossary) that sixteen guides cite; each guide section citing them is reread, corrected where the changed text affects it, and its guide-mark fingerprint updated. The guides are language pairs with consistency records (`<stem>.i18n.yaml`); keep each pair aligned as the kotowari skill's `references/translations.md` says.

## Scope of change

- `crates/kotowari-core/src/` and `crates/kotowari-core/tests/` (the classification), `crates/kotowari/src/` only if the library facade needs to expose it
- `src/cli.rs` (parsing, help text, exit code) and `src/cli/tests.rs`
- `tests/step1_cli.rs` and one new process-level test file `tests/step22_test_side_findings.rs`
- `docs/guides/` (the sixteen guides with guide_stale today, the new hook section of `commands/check.md` and `commands/check.ja.md`) and their `.i18n.yaml` consistency records
- `agent/skills/kotowari/references/findings.md`, `agent/skills/kotowari-adopt/SKILL.md`
- `CHANGELOG.md` (one Unreleased entry under Added)
- `lefthook.yml`
- `.kotowari/changes/implementation.yaml` and `.kotowari/changes/review.yaml`, each by its own role
- `.kotowari/equivalents.yaml` only for equivalents that pass the existing challenge rule

The IR and the decision records are approved and do not change in this plan.

## Step order and prerequisites

S1 first (the flag must parse before its effect can be tested from the process); S2 needs S1; S3 needs S2 (the texts describe the finished behavior); S4 needs S2 (the hook uses the option from the binary built in the repository); S5 comes last. One writer works the steps in order on branch `allow-test-findings`.

Before S1, capture the branch-wide comparison base once with `git merge-base origin/main HEAD` (it was `b98994b849f4af524113f6a82c085066258f3695` when this plan was written) and keep it in session-local evidence; the final change records cover the specification commit, this plan and the implementation against that base. Delete the two records left in `.kotowari/changes/` from the previous branch before writing new ones in S5. Run the baseline: the test command must pass, and `kotowari check --format json` must report exactly 1 requirement_without_test and 6 scenario_without_test errors (on REQ-core-357 and EX-core-547 to EX-core-552), 48 guide_stale notices (all in `docs/guides/`), 9 size notices, and nothing else.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-002, REQ-core-004 | EX-core-552 |
| S2 | REQ-core-357, TBL-core-047, TBL-core-002 | EX-core-547 to EX-core-551 |
| S3 | REQ-core-358 (review) | none |
| S4 | none (record A8) | none |
| S5 | all of the above | all of the above |

## Left to the implementer

- The names of the classification function and of any type it returns, and whether `CheckReport` or the `kotowari` facade exposes it, within the layering above (the kind list lives in `kotowari-core`, never in `src/`).
- The wording of the help line for the option, of the guide section and of the skill texts, and the CHANGELOG wording.

## Stop conditions

- The check run does not keep enough to tell whether a finding's path is a test file or a surface file, and finding out would need reading the configuration or the file system again from the CLI.
- An existing test fails for a reason other than the text the specification commit replaced.
- A finding kind not in TBL-core-047 turns out to be raised on test files in a way that would stop the spec-first commit this plan is for: hand back to brainstorm rather than widening the table.
- The four general conditions: missing meaning or departure from approved content; an irreversible, privileged or dangerous operation; a spreading accident; no progress after a changed approach.

## Test command

`CARGO_BUILD_JOBS=4 cargo test --workspace --all-features --locked`. Format only the Rust files you touched with `rustfmt --edition 2024 <file>`; never run `cargo fmt --all`. Tests must not wait on real time. Pass `< /dev/null` when running the pre-push hook by hand.

## Out of scope

- Any stage or level of strictness for projects adopting kotowari midway (record R1).
- Changing `status`, `changes` or any command other than check (record A5).
- A field in the JSON output that classifies findings (record A1, rejected).

## Steps

### S1: accept the flag on check only

- Purpose: parse "--allow-test-findings" as a flag without a value on check, and stop with an argument error on every other command.
- Specification: docs/ir/core/cli.md#REQ-core-002, docs/ir/core/cli.md#REQ-core-004
- Prerequisites: the baseline above passes
- May change: src/cli.rs, src/cli/tests.rs, tests/step1_cli.rs
- Done when: `kotowari check --allow-test-findings` runs the check, a second occurrence of the flag stops as an argument error, `kotowari status --allow-test-findings` stops with exit code 2 and an argument error, and every existing argument test passes
- Shown by: test — a process-level test in tests/step1_cli.rs marked `@kotowari[EX-core-552]`, a parse test in src/cli/tests.rs marked `@kotowari[REQ-core-002]` for the flag being accepted on check without consuming the next argument, and parse tests marked `@kotowari[REQ-core-004]` that reject the flag on changes, mutants, plan, list, query, overview build and overview serve and reject a repeated flag on check, each written first and seen failing, then passing
- Left to the implementer: the help line's wording
- Stop and hand back if: accepting a flag without a value changes how any existing option or positional argument is parsed

### S2: leave test-side findings out of the exit code

- Purpose: classify each error by TBL-core-047 in the library and, when the flag is given, compute check's exit code from the errors that are not test-side findings, with the output unchanged.
- Specification: docs/ir/core/test-side-findings.md#REQ-core-357, docs/ir/core/test-side-findings.md#TBL-core-047, docs/ir/core/cli.md#TBL-core-002
- Prerequisites: S1
- May change: crates/kotowari-core/src/, crates/kotowari-core/tests/, crates/kotowari/src/, src/cli.rs, tests/step22_test_side_findings.rs
- Done when: the tests marked for EX-core-547 to EX-core-551 and for each row of TBL-core-047 pass, the JSON output with and without the flag is identical for the same input, and every existing test passes
- Shown by: test — process-level tests in tests/step22_test_side_findings.rs marked `@kotowari[EX-core-547]`, `@kotowari[EX-core-548]`, `@kotowari[EX-core-549]`, `@kotowari[EX-core-550]`, `@kotowari[EX-core-551]`, a core test marked `@kotowari[TBL-core-047]` that runs one case per row (the three "Always" rows on paths that are not test files, such as an IR document for requirement_without_test and scenario_without_test; invalid_marker and unresolved_reference on a test file and on a non-test path; unparsable_file on a file only in tests.files and on a file in both tests.files and surface.files), and a process-level test marked `@kotowari[REQ-core-357]` that runs check with and without the flag on one fixture holding a test-side finding and another error and asserts equal standard output in both json and text formats, each written first and seen failing, then passing
- Left to the implementer: none
- Stop and hand back if: an unresolved_reference or invalid_marker raised from a test file carries a path other than that test file

### S3: guides, skill references and changelog

- Purpose: describe the flag and the hook layout (pre-commit with the flag, pre-push and CI without) in the check guide, the kotowari skill's findings reference and kotowari-adopt, and bring the sixteen stale guides up to date.
- Specification: docs/ir/core/test-side-findings.md#REQ-core-358, docs/ir/core/cli.md#REQ-core-002, docs/ir/core/cli.md#TBL-core-002
- Prerequisites: S2
- May change: docs/guides/, agent/skills/kotowari/references/findings.md, agent/skills/kotowari-adopt/, CHANGELOG.md
- Done when: the three places hold every point of REQ-core-358's how_to_verify, every guide pair is aligned with its consistency record updated from `kotowari list`, and `kotowari check` reports no guide_stale, translation_stale or invalid_marker
- Shown by: check — `kotowari check --format json | jq '[.findings[] | select(.path | test("docs/guides/|agent/skills/"))]'` prints an empty list, and the test of REQ-core-125 passes
- Left to the implementer: wording, and where in each text the hook guidance sits
- Stop and hand back if: a stale guide section describes behavior that the changed requirements no longer state and no requirement says what replaces it

### S4: switch this repository's pre-commit hook to the flag

- Purpose: replace the jq filter of the kotowari-check command in lefthook.yml's pre-commit with `kotowari check --allow-test-findings`, and update the comment that explains it (record A8, A2).
- Specification: docs/decision/records/2026-10-06-spec-first-commit.md#A8
- Prerequisites: S2
- May change: lefthook.yml
- Done when: the pre-commit command holds no list of finding kinds, still stops on exit code 2 and on any error outside TBL-core-047, and the pre-push commands are unchanged
- Shown by: artifact — lefthook.yml: the pre-commit kotowari-check command runs `kotowari check --allow-test-findings` with no jq and no list of finding kinds and still exits 2 on a stop, the pre-push commands are byte-for-byte unchanged, and `lefthook run pre-commit --no-auto-install` exits 0; the stopping on other errors is shown by S2's tests for EX-core-549 to EX-core-551
- Left to the implementer: the comment's wording
- Stop and hand back if: the hook cannot run the binary with the flag without changing how cargo is invoked in the pre-push commands

### S5: change records and the final checks

- Purpose: record and independently review the branch's changes against the comparison base, and confirm the plan's own items are done.
- Specification: docs/ir/core/test-side-findings.md#REQ-core-357, docs/ir/core/test-side-findings.md#REQ-core-358, docs/ir/core/cli.md#REQ-core-002
- Prerequisites: S1 to S4
- May change: .kotowari/changes/implementation.yaml, .kotowari/changes/review.yaml, .kotowari/equivalents.yaml
- Done when: `kotowari query` shows a nonempty tests list for every requirement and scenario in the Verification map except REQ-core-358; `kotowari check` reports no error on a file the branch changed or on those IDs; a reviewer has confirmed REQ-core-358 by its how_to_verify; both records are written by their own roles and `kotowari changes --base <base> --head <head> --phase review` exits 0 at the committed head
- Shown by: check — the test command, then `kotowari check --format json`, then `kotowari changes --base <base> --head <head> --phase review --format text`
- Left to the implementer: none
- Stop and hand back if: the mutants run of the pull request finds survivors that no test of this plan's requirements can catch without pinning wording the specification does not declare
