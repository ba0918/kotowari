---
name: kotowari-implement
description: "Workflow station of the kotowari workflow: build the requirements of an approved specification on a branch, delegated by kotowari-cycle. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`). Use when cycle delegates implementation of a kotowari specification. 日本語キーワード: 実装 仕様を実装 TDD"
---

# Implement

Build the requirements you were given. You decide the order and the files to touch. Code is
built test-first; documents and small scripts may be written directly.

## Inputs and outputs

In: the worktree path, the branch, the place of the specification (the IR store path, or for a
topic with no IR the path of its committed specification), and the IDs of the requirements to
build, deferred ones already removed; work only inside that worktree. Read the requirements as
**Requirements and kotowari check** below says (with no IR, the specification's sections), and
whatever else in the repository they need. Nothing else is handed to you — the repository is the
context. Out: commits on that branch.

Write working notes (the order you chose, what is left) only when you judge them necessary, under
`.agents/scratch/` in the worktree. They are never committed and no person reads them; they are
for your own resume.

## Stop only for these

Hand back (stop, state the reason, do not guess) when:

- a consequential decision lacks grounds, exceeds the delegation, changes or deletes an approved requirement, or contradicts an existing choice → back to brainstorm;
- the work reaches an irreversible operation, a privileged operation, or a dangerous target
  (production data, configuration, external effects) → ask, then continue;
- continuing would spread damage (secret exposure, unintended publication, data loss) → stop;
- after diagnosing and changing approach once there is still no progress → question the premise
  once, as kotowari-cycle's `references/premise-step.md` says under **In implement**; hand back to
  cycle with the premise ladder only when that brings no progress either;
- a hook stops your commit on an IR-side finding → back to cycle, as **Requirements and kotowari
  check** says;
- no test command can be determined (see below) → back to cycle, which asks the person, before
  writing product code.

Before handing back a missing decision, ask whether running something in the worktree would answer
it (what a library returns, whether a check passes, how long a command takes). Such an answer is a
fact, not a decision: run the smallest throwaway probe, keep it out of the commits, act on what it
showed, and put the command and its output in the report. Hand back only what no run can settle —
what the specification should mean.

Everything else you recover from yourself: an unforeseen but safe file to add, a flaky helper
tool, a hook failure, an ordinary command failure, a missing bit of record you can reconstruct.
Never ask for acceptance of the result along the way.

## Completing the work

Each piece of work is shown complete by one of four kinds: test (behavior a test can show), check
(commands, such as for a change the IR does not hold), artifact (a document, or skill text a
requirement verified by review describes: its `how_to_verify` says what must be written), or
external. Before completing a piece, read that kind's section in `references/completion.md`. An
external piece hands back before running anything unsafe, privileged, or irreversible.

The test command comes from, in order: the project's own instructions, the standard tool of the
language. If neither settles it, hand back to cycle.

## Requirements and kotowari check

Read the requirements you were given, and their scenarios, with `kotowari query ID`: one call gives the
text, the tests already marked for it, and its reverse references. Take only the keys you need
with `jq` (the kotowari skill's `references/findings.md` has an example). Never read
`kotowari list` whole.

When writing a test, read the kotowari skill's scene mark (`references/mark.md`) and mark the test
with the IDs it verifies. Run `kotowari check` and `kotowari status` before you return; to read their output, read the kotowari skill's scene check (`references/findings.md`).

When `kotowari check` exits with 1, work only on this run's findings: those whose `path` is a
file you changed or that name an ID you were given. Where each kind goes is in kotowari-cycle's
**kotowari check before the terminal report**. Fix
yourself what it gives the fixer or the implementer (test-side findings, missed mutations, and a
deferred_with_test your own mark raised, by removing that mark). List in your report, untouched,
what it gives the consistency phase or only counts (IR-side findings, the other deferred notices,
overview data), and every finding that is not this run's.

A deferred requirement or deferred scenario (`deferred` true in `kotowari query`) is not built
now: write no test for it and put its ID in no mark.

When the pre-commit hook stops your commit on an IR-side finding, follow the hook-stop rule in
kotowari-cycle's `references/editing-contract.md` (**Fixer only**) and hand back. Cycle runs the
consistency phase and then delegates the remaining work to you again; resume from the working
diff as **Resuming** says.

## Committing

- Commit, stage, hook and deletion rules: the **Editing contract** section of kotowari-cycle's
  `references/editing-contract.md`. A test and the minimal code that makes it pass are one concern.
- Message: follow the repository's commit conventions; a body only when the *why* needs it; never
  name session chronology.
- Changes beyond what the requirements need (a fix found on the way): commit them with the
  reason recorded, and list them in the final report. Anything that changes what a requirement
  means is a hand-back.
- Do not invent verification of verification: tests whose subject is a check or test helper itself,
  and tests pinning workflow prose, are created only when a finding or the specification
  requires them. This does not bar unit tests of product helpers for behavior they support.
  A change to CI, hooks, release, or build configuration gets no test and no IR of its own; the
  kotowari skill's **What the IR holds** lists what the IR leaves out.
- Keeping secrets out of commits is your responsibility; nothing scans for you.

## Resuming

There is no progress file. To resume, read `git log`, the working diff, `git status`, your
working notes if there are any, and which of the given requirements, other than those verified by
review, still have no marked test (`tests` empty in `kotowari query`); build what remains. Work
that leaves no trace in git (check-only work, external checks) and approved-but-unexecuted human
decisions are redone or re-asked.

## Report at the end

Commits made, verification evidence per requirement (test names run, check commands, artifact
paths, external summaries), changes beyond the requirements with reasons, anything handed back and
why.
