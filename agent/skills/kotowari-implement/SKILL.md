---
name: kotowari-implement
description: "Workflow station of the kotowari workflow: build the decisions of an approved specification on a branch, delegated by kotowari-cycle. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/decisions/`). Use when cycle delegates implementation of a kotowari specification. 日本語キーワード: 実装 仕様を実装 TDD"
---

# Implement

Build the decisions you were given. You decide the order, the files to touch, readability and
choices of structure, and every detail the decision documents do not write. Code is built
test-first; documents and small scripts may be written directly.

## Inputs and outputs

In: the worktree path, the branch, the place of the decision documents, and the list of decisions
to build (each a topic document's path and a heading's anchor); with them, when the brainstorm
overturned decisions, the row IDs of the old decisions and the decision replacing each, and the
path of a prototype to compare with, if one was made. Work only inside that worktree. Read each
decision in its topic document, and whatever else in the repository it needs. Nothing else is
handed to you — the repository is the context. Out: commits on that branch, with the states of the
decisions you finished turned to implemented.

Write working notes (the order you chose, what is left) only when you judge them necessary, under
`.agents/scratch/` in the worktree. They are never committed and no person reads them; they are
for your own resume.

## Deciding what the documents do not say

A decision document holds only what a person decided. A detail it does not write (a limit, an
order, the line an error points at, the wording of a message) you decide yourself, with grounds,
and never ask a person. So too when two decisions seem to collide or a sentence reads two ways:
decide the reading, with grounds, and go on.

Grounds are what can be shown: what a run in the worktree returned, the code and tests already
there, the decision documents' problems and rejected options, the project's conventions. Before
deciding, ask whether running something would answer it (what a library returns, whether a check
passes, how long a command takes). Such an answer is a fact: run the smallest throwaway probe, keep
it out of the commits, and act on what it showed.

Write each detail and reading you decided as one line of what was decided, why, and the evidence,
in the body of the commit that carries it and in your report, which goes into the pull request
body (kotowari-cycle's **Terminal report**). Never write it into a decision document: those hold
the person's decisions only.

Section names below are given in their Japanese spelling; an English side uses the English
spelling of the same keyword (the kotowari skill's `references/decision-documents.md`).

Hand a question back to a person only when the intent a person decided collides with itself:
two sentences of the decision documents cannot both hold, so that either choice drops one thing a
person decided. Then write it as an item under `## 未決` at the end of the topic document it
concerns — what collides, the two sentences linked, and that the person decides it — commit it,
and hand back to cycle with that item. Do not build either side of the collision meanwhile.

When building shows that a sentence of a decision does not match the facts (it cannot hold
literally) and the direction the person decided does not change, correct the sentence as a rewrite
of a fact (the kotowari skill's `references/decision-documents.md`, **Changing a decision**), and
write the correction — the old sentence, the new one, and the fact that forced it — in your report
for the pull request body. When the correction would change that direction, it is a collision of
intent: the open section above.

## Stop only for these

Hand back (stop, state the reason, do not guess) when:

- the intent a person decided collides with itself, as above → back to cycle with the open item;
- the work reaches an irreversible operation, a privileged operation, or a dangerous target
  (production data, configuration, external effects) → ask, then continue;
- continuing would spread damage (secret exposure, unintended publication, data loss) → stop;
- after diagnosing and changing approach once there is still no progress → question the premise
  once, as kotowari-cycle's `references/premise-step.md` says under **In implement**; hand back to
  cycle with the premise ladder only when that brings no progress either;
- a hook stops your commit on a finding in a decision document → back to cycle, as **Decisions and
  kotowari check** says;
- no test command can be determined (see below) → back to cycle, which asks the person, before
  writing product code;
- a check command cannot succeed as written → back to cycle, which asks the person, as
  `references/completion.md` says under **Check**.

Everything else you recover from yourself: an unforeseen but safe file to add, a flaky helper
tool, a hook failure, an ordinary command failure, a missing detail you decide as above. Never ask
for acceptance of the result along the way.

## Completing the work

Each piece of work is shown complete by one of four kinds: test (behavior a test can show), check
(commands, such as for a decision about the build with no rows), artifact (a document, or skill
text a row confirmed by reading describes: the row's text says what must be written), or
external. Before completing a piece, read that kind's section in `references/completion.md`. An
external piece hands back before running anything unsafe, privileged, or irreversible.

The test command comes from, in order: the project's own instructions, the standard tool of the
language. If neither settles it, hand back to cycle.

A prototype handed to you is something to compare with, not a base: build the product test-first
on its own, then set what you built beside the prototype and look at the behavior the person saw
in it. A difference you cannot ground in a decision or a fact is a detail to decide and report as
above.

## Decisions and kotowari check

Read each decision you were given in its topic document. Its rows to confirm are what you show
done; read one row and the tests whose markers point at it with `kotowari query <row ID>`, and take
only the keys you need with `jq` (the kotowari skill's `references/findings.md` has an example).
Never read `kotowari list` whole.

When writing a test, read the kotowari skill's scene mark (`references/mark.md`) and give the test
a marker with the row IDs it verifies. When the brainstorm overturned a decision, point the tests that
carried the old rows' IDs at the new rows, or delete them when nothing is left for them to confirm.
Run `kotowari check` and `kotowari status` before you return; to read their output, read the
kotowari skill's scene check (`references/findings.md`).

**Turning the state.** When every row of a decision on your list is met — each row checked by a
test has a passing test whose marker carries its ID, and each row confirmed by reading
(`（読んで確認）`, `(by reading)` on an English side) has been read and found true in what you
wrote — change the decision's state line from `状態: 提案` to `状態: 実装済み` (`Status: proposed`
to `Status: implemented` on an English side) on every side of its pair, and bring the agreement
record up to date, in the commit that finishes it. A decision with no rows
is turned when what it decides has been done. A decision not on your list stays as it is. Show,
per decision turned, the test names or the passages read for each row.

When `kotowari check` exits with 1, work only on this run's findings: those whose `path` is a
file you changed or that name a row of a decision you were given. Where each kind goes is in
kotowari-cycle's **kotowari check before the terminal report**. Fix yourself what it gives the
fixer or the implementer (test-side findings such as confirmation_without_test, and missed
mutations). List in your report, untouched, what it gives the consistency phase or only counts,
and every finding that is not this run's.

When the pre-commit hook stops your commit on a finding in a decision document, follow the
hook-stop rule in kotowari-cycle's `references/editing-contract.md` (**Fixer only**) and hand back.
Cycle runs the consistency phase and then delegates the remaining work to you again; resume from
the working diff as **Resuming** says.

## Committing

- Commit, stage, hook and deletion rules: the **Editing contract** section of kotowari-cycle's
  `references/editing-contract.md`. A test and the minimal code that makes it pass are one concern.
- Message: follow the repository's commit conventions; never name session chronology. The body
  carries the lines of each detail or reading you decided (**Deciding what the documents do not
  say**).
- Deleting a test, adding a suppression of a check, and adding or changing a dependency are not
  hand-backs: decide them and write the decision line.
- Changes beyond what the decisions need (a fix found on the way): commit them with the reason
  recorded, and list them in the final report. A change that would alter what a person decided is
  the collision of intent above, never an extra change.
- Do not invent verification of verification: tests whose subject is a check or test helper itself,
  and tests pinning workflow prose, are created only when a finding or a row requires them. This
  does not bar unit tests of product helpers for behavior they support. A change to CI, hooks,
  release, or build configuration gets no test; it is checked by running it.
- Keeping secrets out of commits is your responsibility; nothing scans for you.

## Resuming

There is no progress file. To resume, read `git log`, the working diff, `git status`, your
working notes if there are any, and which decisions on your list are still proposed and which of
their rows checked by a test still have no test with a marker (`tests` empty in `kotowari query`); build
what remains. Work that leaves no trace in git (check-only work, external checks) and
approved-but-unexecuted human decisions are redone or re-asked.

## Report at the end

Commits made; per decision, its state and the evidence for each row (test names run, check
commands, artifact passages, external summaries); changes beyond the decisions with reasons; each
detail or reading you decided by yourself (a dependency you added included) as one line of what was
decided, why, and the evidence; each sentence of a decision document you corrected to the facts,
with the fact; and anything handed back, with the open item it wrote.
