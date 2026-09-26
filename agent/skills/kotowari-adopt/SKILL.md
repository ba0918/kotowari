---
name: kotowari-adopt
description: "Workflow station of the kotowari workflow: bring one topic of an existing project into kotowari — sort its legacy documents, implementation, and tests against each other, write the IR, the glossary, the problem record, and one decision record, get approval, and hand the person one request text for the test work that follows. Use in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`) or is about to adopt it. Use when asked to bring an existing specification or existing code into the IR, to adopt kotowari into a project midway, or when it is unclear whether the IR matches the implementation. 日本語キーワード: 導入 取り込み 既存プロジェクト 途中から 旧資料 現状追認 仕分け IRと実装の突き合わせ"
---

# Adopt

Bring one topic of an existing project into kotowari. Legacy documents, the implementation, and
the tests are compared and each finding is sorted to a fixed destination: what agrees becomes
the specification as it stands, what disagrees or is missing is left undecided as a FLAG, and
nothing is decided on the spot. Deciding belongs to brainstorm, when the feature is next worked on.

**Legacy documents** (旧資料): documents older than the adoption that
state a specification or a decision — old specifications, existing decision records and ADRs, the
parts of a README that explain behavior, and, when re-adopting a topic that already has IR, that
topic's IR. Code comments and commit logs are not legacy documents. Use this term only.

## Inputs and outputs

In: the repository. Out, for one topic: IR documents in the IR store, rows in the glossary
`CONTEXT.md`, FLAGs in `FLAGS.md`, and exactly one decision record named
`docs/decision/records/YYYY-MM-DD-adopt-<topic>.md` — approved. Tests are never changed here: this
skill places no mark on a test, deletes no test, and moves no test.

The forms live in the kotowari skill; do not restate them. Before writing, read its scene write
(`references/ir-form.md` — Flags, Deferred requirements, Glossary, Sources — and
`references/records.md`); before approval, its scene check (`references/findings.md`) and
`references/collate.md`.

First, check whether the repository has `.kotowari/config.yaml`. If not, do the steps of setup in
the kotowari skill's `references/config.md` before anything else. Questions asked during setup do
not count toward the three below.

## The three times the person is asked

The person is asked exactly three times: the scope round, one list round, and approval. Never
interview item by item.

### 1. Scope round

- One topic per run. The person chooses it; the default offered is the feature they plan to change
  next. Topics not chosen stay as legacy documents, untouched; there is no project-wide inventory.
- In the same round, propose the topic's user entry points (the commands, subcommands, or screen
  operations a user touches), its code files and test files, and the candidate legacy documents to
  read, and have the person confirm them.
- This session reads and sorts everything itself. If the topic is too large to read in one go, do
  not hand the reading to another agent: propose, in this same round, splitting the topic smaller.

### 2. List round

After sorting (below), show one list. One row is one candidate requirement: behaviors under one rule
that differ only in a value or a wording (skip-reason messages, for example) are one row, a
candidate decision table, with the table's contents listed briefly in the implementation column.

- a behavior row: the legacy statement, the implementation's behavior, where the grounds test is
  (or "none"), and the verification value the requirement gets (below);
- a term row: the legacy definition, how the implementation uses the term, and the grounds.

The person answers two things only. For confirmation candidates, they remove only the rows that feel
wrong; a row they are unsure of stays and is confirmed. For legacy statements and existing
requirements with no implementation, they say whether it will be built. FLAG rows and the test
breakdown are shown, not asked. Also show, as a count only, the behaviors found outside the scope
(below), with the names of the topics they suggest next.

When the candidate requirements outnumber `limits.requirements` in the configuration, check whether
the entry points are mixed. If they are, propose in this same round to split the topic, and the
person picks the side for this run. If they are not, do not split for the count; split the IR into
documents by responsibility when writing it.

### 3. Approval

Write the IR and the decision record from the answers, then approve once by kotowari-brainstorm's
**Finishing**, step 3 ("Approve in this order"). The decision record's diff the person reads there is
the final version of the list.

## Sorting behaviors and terms

Only behaviors observable through the topic's entry points become rows. Code shared with another
entry point does not widen the scope: a behavior observed through another entry point (merge taking
a backup, when the topic is rollback) is outside it. Reading files outside the confirmed scope to
understand the topic is fine, but a behavior found there becomes no row and is not written in the
decision record; it is only counted for the list round.

Judge agreement by reading the implementation; whether a test exists is written in the list, not a
condition of agreement. "A legacy statement" below does not include the topic's existing IR
requirements: those are handled only by the "existing requirement" rows. Every FLAG, of any kind,
cites as its source a decision of its own.

| Subject | Situation | Destination |
|---|---|---|
| Legacy statement | A decision in the decision records that can be cited as a source agrees with the implementation | A requirement citing that decision |
| Legacy statement | Agrees with the implementation (no decision that can be cited) | Confirmation candidate |
| Legacy statement | Disagrees with the implementation | contradiction FLAG |
| Legacy statement | After reading, no confidence whether it agrees or disagrees | ambiguity FLAG |
| Legacy statement | No implementation; the person answered it will be built | Deferred requirement citing a decision to defer it |
| Legacy statement | No implementation; the person answered it will not be built | Rejected in the decision record |
| Behavior with no legacy statement | Visible to users, and a test checks it | Confirmation candidate marked "no legacy document" |
| Behavior with no legacy statement | Visible to users, no test | gap FLAG |
| Behavior with no legacy statement | Internal, not visible to users | Not put in the IR |
| Confirmation candidate | The person did not remove it in the list | A requirement or term citing that row's own confirmation decision |
| Confirmation candidate | The person removed it in the list | contradiction FLAG |
| Existing requirement | Agrees with the implementation | Keep the requirement as it is |
| Existing requirement | Disagrees with the implementation | Do not rewrite it; contradiction FLAG with its ID in related |
| Existing requirement | After reading, no confidence whether it agrees or disagrees | Do not rewrite it; ambiguity FLAG with its ID in related |
| Existing requirement | No implementation; the person answered it will be built | Add a `- deferred:` line citing a decision to defer it to that requirement |
| Existing requirement | No implementation; the person answered it will not be built | Do not delete it; contradiction FLAG with its ID in related |
| Legacy term definition | Agrees with how the implementation and tests use it | Confirmation candidate |
| Legacy term definition | Disagrees with how the implementation and tests use it | contradiction FLAG |
| Legacy term definition | Its reading splits | ambiguity FLAG |
| Term used in a requirement's statement | No legacy definition | Write a definition from how the implementation uses it; confirmation candidate |

Confirmation candidates include terms: a confirmed term goes into `CONTEXT.md`, a removed one becomes
a contradiction FLAG. When citing an existing decision that agrees, cite it even if it lacks a reason,
and do not rewrite the legacy document.

## Requirements written by confirmation

Decide each requirement's `- verification:` by what the behavior is, never by whether a test exists:
`unit` for a behavior of the implementation, `property` for a property that holds over the whole
input, `review` for something outside the code (a document, an operating procedure), with
`- how_to_verify:` saying how to check it. Making a requirement `review` because it has no test hides
the missing test in the specification; do not. The value is shown in the list, not asked.

Scenarios (success condition and counter-example) are not required for these requirements. Write one
only when a legacy document or a grounds test already holds a concrete case.

## Sorting tests

Sort every test in the topic's scope, each to exactly one of four destinations:

- **grounds of a requirement** — checks a row that becomes a requirement or a confirmation candidate
  (the candidates for a mark after approval);
- **test of a FLAG's behavior** — kept; its handling is decided when the FLAG is collected;
- **only traces implementation details** — a deletion candidate;
- **keep** — checks internal behavior that is not put in the IR, or cannot be judged.

Show the breakdown (the count per destination) in the list round and at approval, and write it as one
line in the decision record's Context.

## Writing the decision record

One decision record per run, `docs/decision/records/YYYY-MM-DD-adopt-<topic>.md`, form as in
`references/records.md`. One decision per line. Besides its fixed wording, each decision's body
states that row's own content: the approval's collation (`references/collate.md`) accepts an item
only when the decision it cites states the item's content.

- Each confirmed behavior row and each confirmed term (one word) gets its own decision: "the legacy
  document and the current implementation agree, so this is taken as the specification without
  review" (for a row marked "no legacy document", or a term defined from the implementation: the
  current implementation is taken as the specification without review), followed by the content —
  for a behavior, what the legacy document says and what the implementation does (only the latter
  for a row marked "no legacy document"); for a term, its definition. Its `- decided_by:` is `利用者（現状追認の一覧を承認）` (the person, approving the
  confirmation list). The requirement or term cites that row's decision.
- Each FLAG, whatever its kind, gets its own decision "leave this undecided as a FLAG", followed by
  what disagrees (contradiction), what is missing (gap), or what is unclear (ambiguity), and the FLAG
  cites it. Show where the legacy document is, in the decision's body or as a link. A FLAG born from an
  existing requirement has that requirement's ID in `- related:`, whatever its kind.
- Each deferral gets its own decision to defer, stating what is deferred, and each legacy statement
  that will not be built gets its own entry under `## Rejected`, stating what is not built — one per
  line, in the same record.

The confirmation, "leave this undecided as a FLAG", and deferral decisions go under `## Agreements`,
because they are cited as sources and the numbers of `## Undecided` cannot be. Only the entries for
what is not built go under `## Rejected`.

## After approval

Hand the person one request text, in the conversation only — do not write it to a file. It holds:

- review whether each grounds test checks its requirement sufficiently, and mark only the sufficient
  ones (the kotowari skill's `references/mark.md`);
- let the person decide in one batch whether to delete the tests that only trace implementation
  details;
- bring the tests to be marked into the range of `tests.files` in the configuration — move them in or
  widen `tests.files`, following the project's existing setting;
- write tests for the requirements that have no grounds test;
- in a project that runs mutation tests: compare the surviving mutants on the topic's code before and
  after the deletion, and if they increase, restore the deleted tests and put them in "keep".

Which station runs it is decided by kotowari-using-workflow, not here. The FLAGs are collected by
kotowari-brainstorm when that feature is next worked on.

Tell the person three things:

- tests outside `tests.files` are outside kotowari: a test with no mark there is not an error;
- a requirement with no grounds test keeps its requirement_without_test until its test is written, and
  a project whose push or CI requires `kotowari check` to pass stops on it — so run the adoption and the
  test work that follows on one branch;
- while FLAGs remain, `kotowari status` reports `complete` false. That is intended: it is the list of
  what awaits collection, not a breakage.
