---
name: kotowari-adopt
description: "Workflow station of the kotowari workflow: bring one topic of an existing project into kotowari by sorting its legacy material, implementation, and tests into the decision documents. Use in a repository that uses kotowari (one that has `.kotowari/` or `docs/decisions/`) or is about to adopt it. Use when asked to bring an existing specification or existing code into the decision documents, to adopt kotowari into a project midway, or when it is unclear whether the decision documents match the implementation. 日本語キーワード: 導入 取り込み 既存プロジェクト 途中から 旧資料 現状追認 仕分け 決定の文書と実装の突き合わせ"
---

# Adopt

Bring one topic of an existing project into kotowari. The legacy material, the implementation and
the tests are compared, and each row is sorted to a fixed destination: what agrees becomes the
specification as it stands, and what disagrees or could not be read through is left undecided
under the open section of the topic document. Nothing is decided on the spot; deciding belongs to
kotowari-brainstorm, when the feature is next worked on.

**Legacy material** (旧資料): documents that existed before the adoption and write down a
specification or decisions — old specifications, existing decision records and ADRs, the parts of
a README that explain behavior. Code comments and commit logs are not legacy material. The
decisions of a decision document the topic already has are not legacy material either: they are
**existing decisions**, handled only by the rows of that name below. Use these terms only.

## Inputs and outputs

In: the repository. Out, for one topic: its topic document under the place of the decision
documents — the decisions the person ratified as `状態: 実装済み` (implemented), the legacy
statements the person answered will be built as `状態: 提案` (proposed), and one item under
`## 未決` per row left undecided — approved. No record is written per adoption. Tests are never
changed here: this skill places no marker on a test, deletes no test, and moves no test.

The form lives in the kotowari skill; do not restate it. Before writing, read its scene decide
(`references/decision-documents.md`); before approval, its scene check (`references/findings.md`).
Section names and keywords below are in their Japanese spelling; an English side writes the English
spelling of the same keyword (`Status: implemented`, `## Open`), as that reference lists.

First, check whether the repository has `.kotowari/config.yaml`. If not, do the steps of setup in
the kotowari skill's `references/config.md` before anything else. Questions asked during setup do
not count toward the three below.

## The three times the person is asked

The person is asked exactly three times: the scope round, one list round, and approval. Never
interview item by item.

### 1. Scope round

- One topic per run. The person chooses it; the default offered is the feature they plan to change
  next. Topics not chosen stay as legacy material, untouched; there is no project-wide inventory.
- In the same round, propose the topic's user entry points (the commands, subcommands, or screen
  operations a user touches), its code files and test files, and the candidate legacy material to
  read, and have the person confirm them.
- This session reads and sorts everything itself. If the topic is too large to read in one go, do
  not hand the reading to another agent: propose, in this same round, splitting the topic smaller.

### 2. List round

After sorting (below), show one list. One behavior row is a candidate for one decision. Behaviors
under one rule that differ only in a value or a wording (skip-reason messages, for example) are one
row, a candidate decision table, with the table's contents listed briefly in the implementation
column.

A behavior row sets out the legacy statement, the implementation's behavior, where the grounds
test is (or "none"), and how the row is checked (**Decisions written by ratification**). The list
has no rows for terms: terms are not sorted.

The person answers two things only. Of the ratification candidates, they remove only the rows that
feel wrong; a row they are unsure of stays and is ratified. For each legacy statement with no
implementation, they say whether it will be built. The rows that become items under `## 未決`, how
each row is checked, and the test breakdown are shown, not asked. Also show, as a count only, the
behaviors found outside the scope (below), with the names of the topics they suggest next.

### 3. Approval

Write the topic document from the answers, then approve once by kotowari-brainstorm's
**Finishing**, step 3 ("Approve in this order"). The diff of the decision documents the person reads
there is the final version of the list. Of that step, 3.2 (showing the scene) does not apply: an
adoption builds nothing. In 3.4, show the proposed decisions written, when there are any, in place
of the list of decisions to build. Step 4 applies as written (the commit, and the branch with its
worktree); the request text of **After approval** names that branch, its worktree, and the decisions
written, each as `path#anchor`.

## Sorting behaviors

Only behaviors observable through the topic's entry points become rows. Code shared with another
entry point does not widen the scope: a behavior observed through another entry point (merge taking
a backup, when the topic is rollback) is outside it. Reading files outside the confirmed scope to
understand the topic is fine, but a behavior found there becomes no row; it is only counted for the
list round.

Judge agreement by reading the implementation; whether a test exists is written in the list, not a
condition of agreement. "Legacy statement" below includes the decisions in the decision records of
the legacy material, and does not include existing decisions.

| Subject | Situation | Destination |
|---|---|---|
| Legacy statement | A decision in the legacy decision records that agrees with the implementation | Not asked for ratification in the list; written as an implemented decision, as that decision says |
| Legacy statement | Agrees with the implementation (no decision record decided it) | Ratification candidate |
| Legacy statement | Disagrees with the implementation | An item under `## 未決` |
| Legacy statement | After reading, no confidence whether it agrees or disagrees | An item under `## 未決` |
| Legacy statement | No implementation; the person answered it will be built | A proposed decision |
| Legacy statement | No implementation; the person answered it will not be built | Not written in the decision documents |
| Behavior with no legacy statement | Visible to users, and a test checks it | Ratification candidate marked "no legacy material" |
| Behavior with no legacy statement | Visible to users, no test checks it | Ratification candidate marked "no legacy material, no grounds test" |
| Behavior with no legacy statement | Internal, not visible to users | Not written |
| Ratification candidate | The person did not remove it in the list | An implemented decision |
| Ratification candidate | The person removed it in the list | An item under `## 未決` |
| Existing decision | Agrees with the implementation | Kept as it is |
| Existing decision | Disagrees with the implementation, or no confidence whether it agrees | Neither rewritten nor deleted; an item under `## 未決` that links to that decision |

Disagreements and rows that could not be read through are not decided during adoption. Write each
as one item under `## 未決` at the end of the topic document: what disagrees (for a row the person
removed, the disagreement between their expectation and the implementation; for a row with no
confidence, what could not be read through), where the legacy material is, and that the person
decides it when that feature is next brainstormed.

A decision taken from the legacy decision records is used as it is, even when it states no reason;
the legacy material is never rewritten.

## Decisions written by ratification

Write each ratified row as one decision whose state is `実装済み`, in the form of the kotowari
skill's `references/decision-documents.md`. Behavior the person has not ratified is not written.

- **Problem**: one sentence saying that the legacy material and the implementation agreed, so it
  was taken as the specification without review. When why the behavior is that way is not known,
  also write that no reason remains; never invent one. Write no rejected options nobody considered.
- **Decision**: what the implementation does, as the row set it out (a decision table for a row
  that is one).
- **Confirmations**: decide how each row is checked by what the behavior is, never by whether a
  test exists. A behavior of the implementation, and a property that holds over the whole input,
  become rows checked by tests; something outside the code (a document, an operating procedure)
  becomes a `（読んで確認）` row. Making a row `（読んで確認）` because it has no test hides the
  missing test in the specification; do not.

## Sorting tests

Sort every test in the topic's scope, each to exactly one of four destinations:

- **grounds of a decision** — checks a row that becomes a decision or a ratification candidate
  (the candidates for a marker after approval);
- **test of a behavior left under `## 未決`** — kept; its handling is decided when that item is
  decided;
- **only traces implementation details** — a deletion candidate;
- **keep** — checks internal behavior that is not written in the decision documents, or cannot be
  judged.

Show the breakdown (the count per destination) in the list round and at approval.

## Surfaces

When the project checks surfaces (`surface.rules` in the configuration; the kotowari skill's
`references/surface.md`), the list of unspecified surfaces carries what adoption has not reached yet.

- When surface rules are first written, the first `kotowari check` raises surface_without_spec for
  every user-visible surface no decision quotes yet. Put every one of them into the list of
  unspecified surfaces, one entry each, with a `why` that names it as not yet adopted. Write no
  decisions for them then; that is the work of each topic's adoption.
- In each run, when the Decision or the Confirmations of a ratified decision quotes the name of a
  surface of the topic, remove that surface's entry from the list in the same run;
  `kotowari check` raises surface_unspecified_stale for an entry left behind.
- A surface of the topic that became an item under `## 未決`, or that the person removed in the list
  round, keeps its entry.

## After approval

Hand the person one request text, in the conversation only — do not write it to a file. It holds:

- for each confirmation row with a grounds test, review whether the test checks the row
  sufficiently, and place a marker with that row's ID only on the sufficient ones (the kotowari
  skill's `references/mark.md`);
- write tests for the confirmation rows that have no grounds test;
- let the person decide in one batch whether to delete the tests that only trace implementation
  details;
- bring the tests to receive a marker into the range of `tests.files` in the configuration — move
  them in or widen `tests.files`, following the project's existing setting;
- in a project that runs mutation tests: compare the surviving mutants on the topic's code before and
  after the deletion, and if they increase, restore the deleted tests and put them in "keep". A
  project that does not run them is not asked to.

Which station runs it is decided by kotowari-using-workflow, not here. The items under `## 未決` are
decided by kotowari-brainstorm when that feature is next worked on.

Tell the person three things:

- tests outside `tests.files` are outside kotowari: a test with no marker there is not an error;
- a confirmation row with no grounds test keeps a confirmation_without_test error until its test is
  written, and a project whose push or CI requires `kotowari check` to pass stops on it — so run the
  adoption and the test work that follows on one branch. Commits on that branch pass when the
  pre-commit hook runs `kotowari check --allow-test-findings`, which leaves test-side findings out of
  the exit code; the pre-push hook and CI run `kotowari check` without the option, so the branch is
  stopped there until the tests are written;
- while items remain under `## 未決`, `kotowari status` reports `complete` false. That is intended:
  it is the list of what awaits a decision, not a breakage.
