# Plan: question the premise when the loop does not converge, and proofread the workflow skills

## Goal

The kotowari workflow skills carry no wasted, redundant, outdated, duplicated or needlessly up-front instructions, and a cycle (and an implementer) caught in a loop where each fix raises a new finding next to the last one writes down the premise its fixes shared and tries once with that premise replaced before handing the loop back to a person.

## Specification

There is no IR for this topic. The specification is the approved decision record [2026-10-06-premise-and-slimming](../decision/records/2026-10-06-premise-and-slimming.md), committed on branch `premise-and-slimming`. Read the whole record before S1; this plan points to its decisions as `#A<n>` and never restates them. A7 is superseded by A13 and A9 by A22; follow the superseding decisions. A12 is a prohibition that holds for every step, the commit messages included.

The skills are partly bound by existing IR requirements, all verified by review. Their `how_to_verify` lines name text in the skills that must stay true after every step. Read each with `kotowari query ID` before touching the files it names:

| Requirement | Skill files it reads |
|---|---|
| `docs/ir/core/consistency-phase.md#REQ-core-359` to `#REQ-core-363` | kotowari-cycle (SKILL.md and `references/consistency-phase.md`), kotowari-iterate, kotowari-using-workflow, kotowari-review |
| `docs/ir/core/deferred.md#REQ-core-213` | kotowari-plan, kotowari-cycle, kotowari-review, kotowari-brainstorm |
| `docs/ir/core/overview-workflow.md#REQ-core-300`, `#REQ-core-301`, `#REQ-core-303` | kotowari-brainstorm, kotowari-cycle, kotowari-implement |
| `docs/ir/core/adoption.md#REQ-core-214`, `#REQ-core-216` to `#REQ-core-222`, `#REQ-core-235`; `docs/ir/core/adoption-scope.md#REQ-core-215`, `#REQ-core-237` to `#REQ-core-239` | kotowari-adopt, kotowari-using-workflow |
| `docs/ir/core/test-side-findings.md#REQ-core-358` | kotowari-adopt |
| `docs/ir/core/surface.md#REQ-core-230` | kotowari-brainstorm, kotowari-adopt, kotowari-implement (who adds an entry to the unspecified surfaces) |
| `docs/ir/core/skill-references.md#REQ-core-195` | `agent/skills/kotowari-plan/references/plan-example.md` (pinned by a test) |

The IR and the decision record do not change in this plan, except what the consistency phase writes when it runs on this branch.

## Approach and why

Subtract first, then add (#A1): S1 to S4 proofread, restructure and separate the nine workflow skills; S5 adds the premise step to the loop that S1 has already cleaned; S6 writes the changelog. Adding last means the new text lands in a cycle skill whose duplicates are gone, so it is written once.

What counts as bloat is #A13, read with #A15, #A18 and #A19. Length is not the measure: no step is done because a file got shorter, and no step reports word or line counts. Use the `ba0918-skill-authoring` skill as the writing yardstick (#A16); the skills themselves never name it.

The rules for single sources (#A18): every rule has one home among the skill files and every other place points to it by file and section. A contract that cycle pastes into a delegate's prompt at run time (the Editing contract, the Evidence conditions, the reviewer setup) keeps one home, and the skill tells cycle what to paste from where; a pasted copy at run time is not a duplicate, a copy kept in a second file is. Pointers across skills are allowed (record [A11 of workflow-split](../decision/records/2026-09-23-workflow-split.md#A11)).

How a pointer resolves: a bare `references/<file>.md` belongs to the skill named in the same sentence, and otherwise to the skill whose file holds the pointer. A pointer to a section names the target's heading or bold label, and that label must exist in the target file.

A survey made while the record was being written found these duplicates. They are input to S1 to S4, not a checklist; verify each before acting on it:

- The Evidence conditions text is in `kotowari-review/references/oracle-evidence.md`, `kotowari-brainstorm/SKILL.md` (section "Evidence conditions") and `kotowari-plan/references/step-template.md` (section "Evidence conditions"); the reading of conditions 3 and 4 is in both `kotowari-review/SKILL.md` and `kotowari-cycle/SKILL.md`.
- What happens when a hook stops a commit on an IR-side finding is written in `kotowari-cycle/SKILL.md` (Delegations, twice; Editing contract), `kotowari-implement/SKILL.md`, `kotowari-cycle/references/consistency-phase.md` and `kotowari-using-workflow/SKILL.md`.
- When the consistency phase runs and reruns: `kotowari-cycle/SKILL.md` (Loop steps 2 and 6, and **When**) and `references/consistency-phase.md`.
- Optional seats appear in about seven places in `kotowari-cycle/SKILL.md`; their home is `kotowari-review/SKILL.md` **Optional seats**.
- Resume and "run more" re-entry: `kotowari-cycle/SKILL.md` (Inputs, Endings) and `kotowari-iterate/SKILL.md` (the substitution table and the resume paragraph).
- "Run only what the reason named": `kotowari-cycle/SKILL.md` Inputs and `kotowari-using-workflow/SKILL.md`; whether a review is needed: `kotowari-review/SKILL.md` and `kotowari-using-workflow/SKILL.md`.
- The small-task definition: `kotowari-iterate/SKILL.md` (its home) and `kotowari-investigate/SKILL.md`; the read-only constraints: `kotowari-iterate/SKILL.md` (judge prompt) and `kotowari-investigate/SKILL.md`.
- This run's findings, test-side kinds, deferred handling, overview and missed mutations are written in both `kotowari-cycle/SKILL.md` and `kotowari-implement/SKILL.md`.
- The six record kinds: `kotowari-brainstorm/SKILL.md` and `kotowari-brainstorm/references/records.md`; the latter is referred to as `references/records.md`, the same spelling the skill uses for the kotowari skill's `references/records.md`.
- What a plan holds: `kotowari-plan/SKILL.md` and `references/step-template.md`; test completion: `kotowari-implement/SKILL.md` and `references/completion.md`.
- The frontmatter descriptions of kotowari-iterate, kotowari-using-workflow and kotowari-investigate carry run-time mechanics beyond when to use the skill.

S5 writes the premise step once, as a new reference under `agent/skills/kotowari-cycle/references/` (#A5), and each place that stops a loop points to it by name: cycle's **Endings** (ending 3), kotowari-iterate's own ending 3 paragraph, kotowari-implement's stop rule, and kotowari-using-workflow's direct-edit path (#A24). Which ending 3 clauses enter the premise step is #A23. It also makes the changes to ending 3 and finding matching (#A22, #A10, #A11) and adds the attempt record to the findings file's shape in `kotowari-review/references/finding-schema.md` (#A20), since cycle writes that file in the shape the schema states. The premise step keeps cycle as the one judge (#A3) and adds no agent or seat.

How the premise step and REQ-core-363 fit: REQ-core-363 says the consistency phase stops by the cycle's rules and keeps no state of its own. The premise step is part of the cycle's rules and its record lives in the findings file that cycle alone writes, so the phase still keeps no state; the phase is one of the destinations a replaced premise can go to (#A2).

Every check runs this repository's own binary: `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari-cli --bin kotowari -- check --format json` (written `kotowari check` below). It reads no skill file and not `CHANGELOG.md`; in this plan it only confirms that the IR and the decision records are unchanged, by giving the baseline's exit status and finding counts.

## Scope of change

- `agent/skills/kotowari-cycle/` (SKILL.md, `references/consistency-phase.md`, and new references)
- `agent/skills/kotowari-iterate/SKILL.md`
- `agent/skills/kotowari-review/` (SKILL.md and `references/`)
- `agent/skills/kotowari-brainstorm/` (SKILL.md, `references/records.md`, which may be deleted)
- `agent/skills/kotowari-plan/` (SKILL.md, `references/step-template.md`; `references/plan-example.md` only if REQ-core-195's test still passes)
- `agent/skills/kotowari-implement/` (SKILL.md, `references/completion.md`)
- `agent/skills/kotowari-investigate/SKILL.md`, `agent/skills/kotowari-using-workflow/SKILL.md`, `agent/skills/kotowari-adopt/SKILL.md`
- `agent/skills/README.md`, only where a skill's role line no longer matches
- `CHANGELOG.md` (the Unreleased section)

The `kotowari` skill (`agent/skills/kotowari/`) is not a workflow skill and is out of scope. So are the IR, the decision records and the copies of the skills installed under a user's home.

## Step order and prerequisites

S1, S2, S3 and S4 in this order, then S5, then S6; one writer on branch `premise-and-slimming`. S1 goes first because cycle is where most duplicates have their home and the other skills point to it. Each of S1 to S4 replaces the copies inside its own **May change** with pointers to the chosen home, and a pointer may name a home a later step finishes. A rule has one home overall only after the step that removes its last copy: the Evidence conditions after S3, the hook-stop rule after S4. S5 needs S1 to S4 so that the premise step is added once to the cleaned text.

Before S1, record the comparison base with `git merge-base main HEAD` (`b89cf15` when this plan was written) and run the baseline: `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` passes, and `kotowari check` exits 0 with nine size notices (too_many_lines, too_many_requirements) on IR documents and nothing else. Every later step must give the same result.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | record #A13, #A15, #A16, #A18, #A19; keeps REQ-core-213, REQ-core-303, REQ-core-359 to REQ-core-363 (review) | none |
| S2 | record #A13, #A15, #A16, #A18, #A19; keeps REQ-core-213, REQ-core-362 (review) | none |
| S3 | record #A13, #A15, #A16, #A18, #A19; keeps REQ-core-195, REQ-core-213, REQ-core-230, REQ-core-300, REQ-core-301 | none |
| S4 | record #A13, #A15, #A16, #A18, #A19; keeps REQ-core-214 to REQ-core-222, REQ-core-230, REQ-core-235, REQ-core-237 to REQ-core-239, REQ-core-303, REQ-core-358, REQ-core-359 (review) | none |
| S5 | record #A2 to #A5, #A10, #A11, #A20 to #A24; keeps REQ-core-213, REQ-core-303, REQ-core-359 to REQ-core-363 (review) | none |
| S6 | none (changelog) | none |

## Left to the implementer

- Which file is the home of a duplicated rule, as long as every other place points to it and every IR requirement in the table above still finds its text where its `how_to_verify` looks.
- Splitting a section into a new reference or merging a short reference back, when the text is read only on some paths (#A13, "needlessly up-front") or a reference holds nothing its one reader lacks. `kotowari-cycle/references/consistency-phase.md` is not split: REQ-core-359 requires the phase's instructions in one reference.
- The new premise reference's file name and section names.
- Wording, order of sections and the commit split inside a step, one concern per commit.

## Stop conditions

- A rule in two places says two different things and neither the record nor an IR requirement decides which holds: hand back with both locations quoted.
- Removing or rewording text would leave an IR requirement's `how_to_verify` unsatisfied: keep the text and go on; if keeping it contradicts a decision in the record, hand back.
- An instruction looks outdated but nothing shows whether the failure it guards against still happens: keep it in a shorter form (#A19); never delete it on the ground that a current model would not need it without a reason grounded in the record.
- `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` fails after a change to `plan-example.md`: revert that change.

## Test command

`CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references`. No step adds a test: the skills are verified by review (#A17). Cycle does not read this plan's sections, so the main session that starts the cycle passes the five kinds of #A13, read as #A15 says, as review items of the full review.

## Out of scope

- Any change to the `kotowari` skill, the IR, the CLI or the decision records.
- Running a model to compare behavior before and after (#A14).
- Ideas from outside the record, however useful they look; the record lists what was adopted (#A6).

## Steps

### S1: kotowari-cycle and kotowari-iterate proofread, with one home per rule

- Purpose: remove the bloat #A13 defines from the cycle skill, its consistency-phase reference and the iterate skill, and give each rule they share one home.
- Specification: docs/decision/records/2026-10-06-premise-and-slimming.md#A13, docs/decision/records/2026-10-06-premise-and-slimming.md#A15, docs/decision/records/2026-10-06-premise-and-slimming.md#A16, docs/decision/records/2026-10-06-premise-and-slimming.md#A18, docs/decision/records/2026-10-06-premise-and-slimming.md#A19, docs/ir/core/deferred.md#REQ-core-213, docs/ir/core/overview-workflow.md#REQ-core-303, docs/ir/core/consistency-phase.md#REQ-core-359, docs/ir/core/consistency-phase.md#REQ-core-360, docs/ir/core/consistency-phase.md#REQ-core-361, docs/ir/core/consistency-phase.md#REQ-core-362, docs/ir/core/consistency-phase.md#REQ-core-363
- Prerequisites: the baseline in **Step order and prerequisites** passes.
- May change: agent/skills/kotowari-cycle/, agent/skills/kotowari-iterate/SKILL.md
- Done when: each duplicate the survey lists has its copies inside these files replaced by pointers to one home; the text cycle pastes into a delegate's prompt (the Editing contract and what goes with it) is separated from the rules cycle itself follows; iterate's **The loop** section states only how it differs from cycle's loop; and every requirement in Specification still finds its text.
- Shown by: artifact — agent/skills/kotowari-cycle/, agent/skills/kotowari-iterate/SKILL.md; format check: `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` passes; `kotowari check` gives the baseline; every pointer in the changed files resolves as **Approach and why** says; each requirement in Specification, read with `kotowari query ID`, still finds the text its `how_to_verify` names.
- Left to the implementer: whether the Editing contract moves to a reference; whether the resume and re-entry rules become one table.
- Stop and hand back if: cycle's and iterate's resume or "run more" rules disagree in a way the record does not settle.

### S2: kotowari-review proofread, home of the Evidence conditions and optional seats

- Purpose: remove the bloat #A13 defines from the review skill and its references, and make `references/oracle-evidence.md` the home of the Evidence conditions with the reading of conditions 3 and 4.
- Specification: docs/decision/records/2026-10-06-premise-and-slimming.md#A13, docs/decision/records/2026-10-06-premise-and-slimming.md#A15, docs/decision/records/2026-10-06-premise-and-slimming.md#A16, docs/decision/records/2026-10-06-premise-and-slimming.md#A18, docs/decision/records/2026-10-06-premise-and-slimming.md#A19, docs/ir/core/consistency-phase.md#REQ-core-362, docs/ir/core/deferred.md#REQ-core-213
- Prerequisites: S1
- May change: agent/skills/kotowari-review/, agent/skills/kotowari-cycle/SKILL.md (its pointers and its instruction on what to paste from oracle-evidence.md)
- Done when: the review skill keeps no second copy of the Evidence conditions or of the reading of conditions 3 and 4; what cycle and review paste into a delegate's or reviewer's prompt still includes that reading; the rules only a caller of the review follows (such as optional seats) are separated from what a reviewer is given; and every requirement in Specification still finds its text.
- Shown by: artifact — agent/skills/kotowari-review/, agent/skills/kotowari-cycle/SKILL.md; format check: `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` passes; `kotowari check` gives the baseline; every pointer in the changed files resolves as **Approach and why** says; each requirement in Specification, read with `kotowari query ID`, still finds the text its `how_to_verify` names.
- Left to the implementer: whether optional seats move to their own reference.
- Stop and hand back if: the user-scope instructions this skill reads for optional seats would have to change for the skill to work.

### S3: kotowari-brainstorm and kotowari-plan proofread

- Purpose: remove the bloat #A13 defines from the brainstorm and plan skills and their references, pointing to the Evidence conditions' home instead of copying it.
- Specification: docs/decision/records/2026-10-06-premise-and-slimming.md#A13, docs/decision/records/2026-10-06-premise-and-slimming.md#A15, docs/decision/records/2026-10-06-premise-and-slimming.md#A16, docs/decision/records/2026-10-06-premise-and-slimming.md#A18, docs/decision/records/2026-10-06-premise-and-slimming.md#A19, docs/ir/core/overview-workflow.md#REQ-core-300, docs/ir/core/overview-workflow.md#REQ-core-301, docs/ir/core/deferred.md#REQ-core-213, docs/ir/core/surface.md#REQ-core-230, docs/ir/core/skill-references.md#REQ-core-195
- Prerequisites: S2
- May change: agent/skills/kotowari-brainstorm/, agent/skills/kotowari-plan/
- Done when: neither skill keeps a copy of the Evidence conditions, so they have one home overall; brainstorm's record kinds have one home and no reference name in it can be mistaken for the kotowari skill's `references/records.md`; plan's list of what a plan holds has one home; and every requirement in Specification still finds its text.
- Shown by: artifact — agent/skills/kotowari-brainstorm/, agent/skills/kotowari-plan/; format check: `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` passes; `kotowari check` gives the baseline; every pointer in the changed files resolves as **Approach and why** says; each requirement in Specification, read with `kotowari query ID`, still finds the text its `how_to_verify` names.
- Left to the implementer: whether `kotowari-brainstorm/references/records.md` is deleted after its content moves.
- Stop and hand back if: brainstorm's approval steps would need reordering to remove a duplicate (REQ-core-300 and REQ-core-301 fix their order).

### S4: kotowari-implement, -investigate, -using-workflow and -adopt proofread

- Purpose: remove the bloat #A13 defines from the remaining workflow skills, pointing to the homes S1 to S3 settled and trimming each description to when the skill is used.
- Specification: docs/decision/records/2026-10-06-premise-and-slimming.md#A13, docs/decision/records/2026-10-06-premise-and-slimming.md#A15, docs/decision/records/2026-10-06-premise-and-slimming.md#A16, docs/decision/records/2026-10-06-premise-and-slimming.md#A18, docs/decision/records/2026-10-06-premise-and-slimming.md#A19, docs/ir/core/consistency-phase.md#REQ-core-359, docs/ir/core/overview-workflow.md#REQ-core-303, docs/ir/core/adoption.md#REQ-core-214, docs/ir/core/adoption-scope.md#REQ-core-215, docs/ir/core/adoption.md#REQ-core-216, docs/ir/core/adoption.md#REQ-core-217, docs/ir/core/adoption.md#REQ-core-218, docs/ir/core/adoption.md#REQ-core-219, docs/ir/core/adoption.md#REQ-core-220, docs/ir/core/adoption.md#REQ-core-221, docs/ir/core/adoption.md#REQ-core-222, docs/ir/core/adoption.md#REQ-core-235, docs/ir/core/adoption-scope.md#REQ-core-237, docs/ir/core/adoption-scope.md#REQ-core-238, docs/ir/core/adoption-scope.md#REQ-core-239, docs/ir/core/test-side-findings.md#REQ-core-358, docs/ir/core/surface.md#REQ-core-230
- Prerequisites: S3
- May change: agent/skills/kotowari-implement/, agent/skills/kotowari-investigate/SKILL.md, agent/skills/kotowari-using-workflow/SKILL.md, agent/skills/kotowari-adopt/SKILL.md, agent/skills/kotowari-iterate/SKILL.md (description and the judge prompt's read-only list only), agent/skills/README.md
- Done when: each duplicate the survey lists has its copies inside these files replaced by pointers, so the hook-stop rule has one home overall; each description says when to use the skill and not how it runs; and every requirement in Specification still finds its text.
- Shown by: artifact — the files in May change; format check: `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` passes; `kotowari check` gives the baseline; every pointer in the changed files resolves as **Approach and why** says; each requirement in Specification, read with `kotowari query ID`, still finds the text its `how_to_verify` names.
- Left to the implementer: how far adopt is proofread, given that most of its text is bound by requirements.
- Stop and hand back if: a description change would change which requests start a skill (the entry table in kotowari-using-workflow decides that).

### S5: the premise step in the loop

- Purpose: add the premise step with its trigger and entry clauses, the matching and ending 3 changes, the attempt record and its report, and the premise step in implement and in the direct-edit path.
- Specification: docs/decision/records/2026-10-06-premise-and-slimming.md#A2, docs/decision/records/2026-10-06-premise-and-slimming.md#A3, docs/decision/records/2026-10-06-premise-and-slimming.md#A4, docs/decision/records/2026-10-06-premise-and-slimming.md#A5, docs/decision/records/2026-10-06-premise-and-slimming.md#A10, docs/decision/records/2026-10-06-premise-and-slimming.md#A11, docs/decision/records/2026-10-06-premise-and-slimming.md#A20, docs/decision/records/2026-10-06-premise-and-slimming.md#A21, docs/decision/records/2026-10-06-premise-and-slimming.md#A22, docs/decision/records/2026-10-06-premise-and-slimming.md#A23, docs/decision/records/2026-10-06-premise-and-slimming.md#A24, docs/ir/core/deferred.md#REQ-core-213, docs/ir/core/overview-workflow.md#REQ-core-303, docs/ir/core/consistency-phase.md#REQ-core-359, docs/ir/core/consistency-phase.md#REQ-core-360, docs/ir/core/consistency-phase.md#REQ-core-361, docs/ir/core/consistency-phase.md#REQ-core-362, docs/ir/core/consistency-phase.md#REQ-core-363
- Prerequisites: S4
- May change: agent/skills/kotowari-cycle/, agent/skills/kotowari-iterate/SKILL.md, agent/skills/kotowari-implement/SKILL.md, agent/skills/kotowari-using-workflow/SKILL.md, agent/skills/kotowari-review/references/finding-schema.md
- Done when: one reference under `kotowari-cycle/references/` holds the premise step (#A2 to #A4) and cycle's **Endings**, iterate's ending 3 paragraph, implement's stop rule and using-workflow's direct-edit path each name it; cycle states the trigger of #A22, the entry clauses of #A23, the matching of #A10 and the scope of #A11; `finding-schema.md` gives the attempt record of #A20 and cycle's resume reads it; cycle's **Terminal report** lists the premise list from the attempt record whenever a premise attempt ran or a premise was handed to the person; implement's stop rule follows #A21; and every requirement in Specification still finds its text.
- Shown by: artifact — the files in May change; format check: `CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` passes; `kotowari check` gives the baseline; every pointer in the changed files resolves as **Approach and why** says; each requirement in Specification, read with `kotowari query ID`, still finds the text its `how_to_verify` names; and, when `.agents/artifacts/reviews/review-panel.json` exists (it is gitignored), the written trigger applied by hand to its findings and their fixes fires after the fix of finding 5 (finding 7 overlapping it) and not for findings 10 or 11.
- Left to the implementer: the attempt record's field names in the findings file.
- Stop and hand back if: the trigger as written cannot be decided from the findings file alone (for example, a fix's addressed finding is not recorded).

### S6: changelog

- Purpose: tell someone who installed the skills what changed for them.
- Specification: docs/decision/records/2026-10-06-premise-and-slimming.md#A2, docs/decision/records/2026-10-06-premise-and-slimming.md#A13, docs/decision/records/2026-10-06-premise-and-slimming.md#A21, docs/decision/records/2026-10-06-premise-and-slimming.md#A22, docs/decision/records/2026-10-06-premise-and-slimming.md#A24
- Prerequisites: S5
- May change: CHANGELOG.md
- Done when: the Unreleased section of `CHANGELOG.md` has entries, in English and Keep a Changelog form, for the premise step in cycle, iterate, implement and the direct-edit path and for the proofread workflow skills, written for a user of the skills and naming no outside material (#A12).
- Shown by: artifact — `CHANGELOG.md`, Unreleased section.
- Left to the implementer: none
- Stop and hand back if: none beyond the plan's stop conditions.
