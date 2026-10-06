---
name: kotowari-implement
description: "Workflow station of the kotowari workflow: execute an approved plan step by step, test-first for code, committing one concern at a time, and hand back instead of guessing when a design decision is missing. Invoked by kotowari-cycle with a plan path, a branch, and a worktree path. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`). Use when cycle delegates implementation of a kotowari plan. 日本語キーワード: 実装 手順書を実行 TDD 実装計画"
---

# Implement

Carry out the plan you were given, in the order written. Code is built test-first; documents
and small scripts may be written directly. The person sees the result at the end of the cycle,
not per step.

## Inputs and outputs

In: the worktree path, the plan path, and the branch; work only inside that worktree. Out:
commits on that branch. A plan step names the
requirements it rests on as `<document path>#REQ-nnn` (read them as **Requirements and kotowari
check** below says), or, for a topic with no IR, the specification sections; read those and
whatever else in the repository the step needs. Nothing else is handed to you — the repository is the context.

## Stop only for these

Hand back (stop, state the reason, do not guess) when:

- a consequential decision lacks grounds, exceeds the delegation, changes or deletes an approved requirement, or contradicts an existing choice → back to brainstorm;
- the plan step asks for confirmation before an irreversible operation, a privileged operation,
  or a dangerous target (production data, configuration, external effects) → ask, then continue;
- continuing would spread damage (secret exposure, unintended publication, data loss) → stop;
- after diagnosing and changing approach once there is still no progress → stop;
- a hook stops your commit on an IR-side finding → back to cycle, as **Requirements and kotowari
  check** says;
- no test command can be determined (see below) → back to plan before writing product code.

Before handing back a missing decision, ask whether running something in the worktree would answer
it (what a library returns, whether a check passes, how long a command takes). Such an answer is a
fact, not a decision: run the smallest throwaway probe, keep it out of the commits, act on what it
showed, and put the command and its output in the report. Hand back only what no run can settle —
what the specification should mean.

Everything else you recover from yourself: an unplanned but safe file to add, a flaky helper
tool, a hook failure, an ordinary command failure, a missing bit of record you can reconstruct.
Never ask for acceptance of the result step by step; that happens once, at the end of the cycle.

## Completing a step

Each plan step says how its completion is shown: test, check, artifact, or external. Before
completing a step, read that kind's section in `references/completion.md`. An external step hands
back before running anything unsafe, privileged, or irreversible.

The test command comes from, in order: the plan, the project's own instructions, the standard
tool for the ecosystem. If none decides it, hand back to plan.

## Requirements and kotowari check

Read the requirements and scenarios the plan names with `kotowari query ID`: one call gives the
text, the tests already marked for it, and its reverse references. Take only the keys you need
with `jq` (the kotowari-plan skill's **Reading the requirements** has examples). Never read
`kotowari list` whole.

When writing a test, read the kotowari skill's scene mark (`references/mark.md`) and mark the test
with the IDs it verifies. Run the `kotowari check` and `kotowari status` the plan lists as check
commands; to read their output, read the kotowari skill's scene check (`references/findings.md`).

When `kotowari check` exits with 1, work only on this run's findings, as kotowari-cycle's
**kotowari check before the terminal report** defines them and says where each kind goes. Fix
yourself what it gives the fixer or the implementer (test-side findings, missed mutations, and a
deferred_with_test your own mark raised, by removing that mark). List in your report, untouched,
what it gives the consistency phase or only counts (IR-side findings, the other deferred notices,
overview data), and every finding that is not this run's.

A deferred requirement or deferred scenario (`deferred` true in `kotowari query`) is not built
now: write no test for it and put its ID in no mark.

When the pre-commit hook stops your commit on an IR-side finding, follow the hook-stop rule in
kotowari-cycle's `references/editing-contract.md` (**Fixer only**) and hand back. Cycle runs the
consistency phase and then delegates the remaining steps to you again; resume from the working
diff as **Resuming** says.

## Committing

- One concern per commit. A test and the minimal code that makes it pass are one concern.
- Stage with `git add <path>`; never `git add .` or `-A`. Never disable or bypass hooks.
- Message: follow the repository's commit conventions; a body only when the *why* needs it.
  Never name a workflow station (brainstorm / plan / cycle / implement / review), a finding ID,
  or session chronology.
- Fixes outside the plan that do not change its thrust: commit them with the reason recorded,
  and list them in the final report. Anything that changes the thrust is a hand-back.
- Do not invent verification of verification: tests whose subject is a check or test helper itself,
  and tests pinning workflow prose, are created only when the plan, finding, or specification
  requires them. This does not bar unit tests of product helpers for behavior they support.
  A change to CI, hooks, release, or build configuration gets no test and no IR of its own; the
  kotowari skill's **What the IR holds** lists what the IR leaves out.
- For a deletion finding, no failing test is needed. Completion evidence is all existing checks
  passing after deletion.
- Keeping secrets out of commits is your responsibility; nothing scans for you.

## Resuming

There is no progress file. To resume, read the plan, `git log`, the working diff, and
`git status`, and infer where you are. Steps that leave no trace in git (check-only steps,
external checks) and approved-but-unexecuted human decisions are redone or re-asked.

## Report at the end

Commits made, verification evidence per step (test names run, check commands, artifact paths,
external summaries), out-of-plan changes with reasons, anything handed back and why.
