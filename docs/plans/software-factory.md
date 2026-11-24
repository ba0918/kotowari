# Plan: carry a request from specification approval to merge without waiting for a person

## Goal

A person who approves a specification sees the request implemented, reviewed and merged without being asked again, unless the diff touches the confirmation list, a finding awaits their judgment, or an approved requirement was revised.

## Specification

The IR is in `docs/ir/`. This plan covers:

- `docs/ir/core/unattended-merge.md#REQ-core-364`, `#REQ-core-365`, `#REQ-core-366`, `#REQ-core-367`, `#REQ-core-368`, `#REQ-core-369`
- `docs/ir/core/consistency-phase.md#REQ-core-360`, `#REQ-core-362` (reworded parts only)
- `docs/ir/core/deferred.md#REQ-core-213` (reworded clause only)
- Decisions that are not IR: `docs/decision/records/2026-11-24-unattended-merge.md#A36`, `#A38`, `#A42`

All of these requirements are verified by review: the proof is the skill text, read as each requirement's `how_to_verify` line says. No new test is planned.

## Approach and why

The change is almost entirely skill text under `agent/skills/`. The plan station is removed first, because every later step rewrites sentences that mention it. Then the cycle's text is changed in the order a run meets it: input and resuming, the line and the merge, what does not stop, the record and cleanup. The procedure for the line, the merge request and the watch goes in one new reference under `agent/skills/kotowari-cycle/references/`, so that the path this repository lists in `.kotowari/confirm-paths` (`agent/skills/kotowari-cycle`) covers it and `SKILL.md` stays readable. The repository's own `.kotowari/confirm-paths` is added last, after the text that reads it exists.

The skill text follows the project's rule that a workflow skill adds no seat, delegation or loop without a stated reason: nothing in these steps adds a reviewer or a new loop.

## Scope of change

- `agent/skills/kotowari-plan/` (deleted)
- `agent/skills/kotowari-cycle/SKILL.md` and `agent/skills/kotowari-cycle/references/`
  - one new reference for the line, the merge request, the watch and the cleanup
  - `consistency-phase.md`, `editing-contract.md`, `premise-step.md` only where they name the plan
- `agent/skills/kotowari-implement/`, `agent/skills/kotowari-iterate/`, `agent/skills/kotowari-brainstorm/`, `agent/skills/kotowari-using-workflow/`, `agent/skills/kotowari-investigate/`, `agent/skills/kotowari-review/`, `agent/skills/kotowari-adopt/`, `agent/skills/kotowari/` only where they name the plan station or the terminal report's "one check"
- `agent/skills/README.md`, `README.md`, `README.ja.md`, `PROJECT.md`
- `tests/step7_skill_references.rs` (the REQ-core-195 test only)
- `docs/guides/commands/plan.md`, its `.ja.md` side and its `.i18n.yaml`
- `.kotowari/confirm-paths` (new)

## Step order and prerequisites

S1 before all others: later steps rewrite text that S1 leaves without a plan station. S2 before S3: the merge starts from the run S2 describes. S3 before S4 and S5: both refer to the stops and the pull request S3 defines. S6 after S3: the file is read by the procedure S3 writes. S7 last.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-364 (no plan station), A36, A38 | none |
| S2 | REQ-core-364, REQ-core-213, REQ-core-360, REQ-core-362 | none |
| S3 | REQ-core-365, REQ-core-366, REQ-core-367 | none |
| S4 | REQ-core-368 | none |
| S5 | REQ-core-369 | none |
| S6 | A42 | none |
| S7 | all of the above | none |

## Left to the implementer

- The name of the new reference under `agent/skills/kotowari-cycle/references/`, and how the text is divided between it and `SKILL.md`
- The names of the three keys added to the findings file
- The wording of the skill text, as long as every clause of the `how_to_verify` lines is written

## Stop conditions

- A `how_to_verify` clause cannot be written without deciding something the requirement and its cited decisions do not say (hand back to brainstorm; do not invent it)
- Two requirements among REQ-core-359 to REQ-core-369 turn out to demand contradicting skill text
- A test other than the removed REQ-core-195 test fails
- Removing the plan station would require removing or changing the `kotowari plan` command or its requirements (`docs/ir/core/plan.md`); the command stays

## Out of scope

- The `kotowari plan` command, its schema and `docs/ir/core/plan.md`
- A dependency licence and advisory check in CI (`docs/decision/records/2026-11-24-unattended-merge.md#A18`), which is a separate request
- Changing any repository setting on the hosting service; enabling merging by itself is the person's operation
- Reading the record to improve the workflow
- `docs/spec/` documents

## Steps

### S1: the plan station is gone

- Purpose: remove the station that writes a plan, and everything that points at it, while the `kotowari plan` command stays
- Specification: `docs/ir/core/unattended-merge.md#REQ-core-364`, `docs/decision/records/2026-11-24-unattended-merge.md#A36`, `docs/decision/records/2026-11-24-unattended-merge.md#A38`
- Prerequisites: none
- May change: `agent/skills/kotowari-plan/`, `tests/step7_skill_references.rs`, `docs/guides/commands/plan.md`, `docs/guides/commands/plan.ja.md`, `docs/guides/commands/plan.i18n.yaml`, `agent/skills/README.md`, `README.md`, `README.ja.md`, `PROJECT.md`, `agent/skills/kotowari/SKILL.md`, `agent/skills/kotowari/references/findings.md`, `agent/skills/kotowari-using-workflow/SKILL.md`, `agent/skills/kotowari-investigate/SKILL.md`, `agent/skills/kotowari-adopt/SKILL.md`
- Done when: `agent/skills/kotowari-plan/` does not exist; the REQ-core-195 test and its mark are gone; the guide of the plan command holds its example inline and links to no file under the removed skill; no file under `agent/skills/`, `README.md`, `README.ja.md` or `PROJECT.md` names a kotowari-plan skill or routes a request to it; the guide pair's sides agree and its consistency record is updated
- Shown by: check — `test ! -e agent/skills/kotowari-plan`, then `rg -n 'kotowari-plan' agent README.md README.ja.md PROJECT.md docs/guides tests` (no hit), then `cargo test --workspace`, then `kotowari check --format json` (no error on a file this step changed, and no unresolved_reference for REQ-core-195)
- Left to the implementer: how much of the removed example plan the guide keeps inline
- Stop and hand back if: a test other than the REQ-core-195 test reads a file under `agent/skills/kotowari-plan/`, or the guide cannot explain the form of a plan without the removed step template

### S2: the cycle starts from the specification and resumes without a plan

- Purpose: make the cycle, the implementer, iterate, the brainstorm and the entry decider work from the specification's place and requirement IDs instead of a plan
- Specification: `docs/ir/core/unattended-merge.md#REQ-core-364`, `docs/ir/core/deferred.md#REQ-core-213`, `docs/ir/core/consistency-phase.md#REQ-core-360`, `docs/ir/core/consistency-phase.md#REQ-core-362`
- Prerequisites: S1
- May change: `agent/skills/kotowari-cycle/`, `agent/skills/kotowari-implement/`, `agent/skills/kotowari-iterate/SKILL.md`, `agent/skills/kotowari-brainstorm/SKILL.md`, `agent/skills/kotowari-using-workflow/SKILL.md`, `agent/skills/kotowari-investigate/SKILL.md`, `agent/skills/kotowari-review/`, `agent/skills/README.md`
- Done when: every clause of the `how_to_verify` line of REQ-core-364 is written in the skills it names; the changed clauses of REQ-core-213, REQ-core-360 and REQ-core-362 are written; no skill text tells a reader to read, write, approve or return to a plan
- Shown by: check — read each named skill against the `how_to_verify` line of REQ-core-364 clause by clause and list the place of each clause in the report, then `rg -n -i 'plan' agent/skills --glob '!kotowari/**'` and confirm every remaining hit is the `kotowari plan` command or the ordinary word
- Left to the implementer: none
- Stop and hand back if: the cycle's resume table cannot be rewritten without a rule for which requirement an implementer continues from that REQ-core-364 does not give

### S3: the line, the merge and the stops

- Purpose: write the procedure that judges the line with git, asks the hosting service to merge a converged run, withdraws that request when its conditions stop holding, watches the required checks, and stops with an open pull request in the three cases a person confirms
- Specification: `docs/ir/core/unattended-merge.md#REQ-core-365`, `docs/ir/core/unattended-merge.md#REQ-core-366`, `docs/ir/core/unattended-merge.md#REQ-core-367`
- Prerequisites: S2
- May change: `agent/skills/kotowari-cycle/`, `agent/skills/kotowari-iterate/SKILL.md`
- Done when: every clause of the `how_to_verify` lines of REQ-core-365, REQ-core-366 and REQ-core-367 is written; the git command the procedure gives reports a moved file under both paths and takes the pathspecs of `.kotowari/confirm-paths` as they are; the sentence that the terminal report is the person's one check and that the cycle never merges is replaced
- Shown by: check — read the cycle skill against the three `how_to_verify` lines clause by clause and list the place of each clause in the report, then run the procedure's git command in a scratch worktree on a commit that moves a file out of a listed path and on one that touches no listed path, and report both outputs
- Left to the implementer: the exact git invocation, as long as it is git alone, judges the whole diff from the commit where the branch parted from the merge target, and counts a moved file under both paths
- Stop and hand back if: the hosting service's command line cannot report whether merging by itself is allowed or whether the merge target requires a check, or cannot withdraw a request to merge; or REQ-core-367 and REQ-core-361 demand contradicting text for a requirement the consistency phase revised

### S4: what does not wait for a person

- Purpose: write that only a mistake running shows keeps a run from converging, that deleted tests, suppressions and dependency changes do not call a person, and that the implementer's own decisions are left as one line each
- Specification: `docs/ir/core/unattended-merge.md#REQ-core-368`
- Prerequisites: S3
- May change: `agent/skills/kotowari-cycle/`, `agent/skills/kotowari-implement/`, `agent/skills/kotowari-review/`
- Done when: every clause of the `how_to_verify` line of REQ-core-368 is written in the three skills it names, and the implementer's report and the cycle's final report carry the decision lines in the same form as the pull request body
- Shown by: check — read the three skills against the `how_to_verify` line clause by clause and list the place of each clause in the report
- Left to the implementer: none
- Stop and hand back if: the review skill's existing rule for which findings are visible disagrees with "only a mistake that running something shows keeps a run from converging" in a way the requirement does not settle

### S5: the record of a run and the cleanup

- Purpose: add the three things the findings file keeps, keep that file out of the working directory that is cleaned up, tell a merged run's file from a resume, and clean up only after a merge the cycle saw
- Specification: `docs/ir/core/unattended-merge.md#REQ-core-369`
- Prerequisites: S3
- May change: `agent/skills/kotowari-cycle/`, `agent/skills/kotowari-review/references/finding-schema.md`, `agent/skills/kotowari-iterate/SKILL.md`
- Done when: every clause of the `how_to_verify` line of REQ-core-369 is written in the cycle skill and the finding schema
- Shown by: check — read both against the `how_to_verify` line clause by clause and list the place of each clause in the report
- Left to the implementer: none
- Stop and hand back if: keeping the findings file outside the working directory needs a place the cycle skill does not already name

### S6: this repository's confirmation list

- Purpose: give this repository the list the line reads, so that a request here can merge without a person
- Specification: `docs/decision/records/2026-11-24-unattended-merge.md#A42`
- Prerequisites: S3
- May change: `.kotowari/confirm-paths`
- Done when: `.kotowari/confirm-paths` holds exactly the nine pathspecs of A42, one per line
- Shown by: check — run the git command of S3's procedure with this file on this branch and report that it is a hit (the branch changes `agent/skills/kotowari-cycle` and the file itself)
- Left to the implementer: none
- Stop and hand back if: one of the nine paths does not exist in the repository

### S7: the whole agrees

- Purpose: confirm the skill text, the IR and the repository's checks agree after all steps
- Specification: `docs/ir/core/unattended-merge.md#REQ-core-364`, `docs/ir/core/unattended-merge.md#REQ-core-365`, `docs/ir/core/unattended-merge.md#REQ-core-366`, `docs/ir/core/unattended-merge.md#REQ-core-367`, `docs/ir/core/unattended-merge.md#REQ-core-368`, `docs/ir/core/unattended-merge.md#REQ-core-369`
- Prerequisites: S1, S2, S3, S4, S5, S6
- May change: files already changed by S1 to S6
- Done when: the repository's gates pass and `kotowari check` reports no error on a file the branch changed or on an ID this plan covers
- Shown by: check — `scripts/gates.sh`, then `kotowari check --format json` filtered to the files the branch changed and the IDs REQ-core-213, REQ-core-360, REQ-core-362 and REQ-core-364 to REQ-core-369
- Left to the implementer: none
- Stop and hand back if: a gate fails for a reason outside the files this plan changes
