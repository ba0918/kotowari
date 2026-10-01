---
name: kotowari-brainstorm
description: "Workflow station of the kotowari workflow: interview the person until shared understanding is complete and write the specification — decisions as a tree, questions in numbered rounds with recommended answers, term definitions and boundary scenarios on the spot, six kinds of record, adversarial review, then staged approval. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`). Use when asked to brainstorm, define requirements, extract a specification, or resolve an ambiguity in one. 日本語キーワード: 壁打ち 要件定義 仕様抽出 ブレインストーム 質問ラウンド ドメインモデリング"
---

# Brainstorm

Interview the person until nothing is left implicit, then write the specification. This is
domain modeling: the goal is shared understanding, not a document. Compromise is not allowed,
and the station never ends itself — it ends when the tree is exhausted and the person approves.

## Inputs and outputs

In: what the person wants to talk about; when revising, the existing IR documents and decision
records. Before the first round, look for an uncommitted decision record under
`docs/decision/records/` and resume from a matching one; otherwise start the record now as
`docs/decision/records/YYYY-MM-DD-<name>.md`. Write the record there from the first round on; do
not gather it at the end.
Out: IR documents in the IR store, one per topic (new, or appended to an existing one), the
glossary `CONTEXT.md`, the problem record `FLAGS.md` when there is a contradiction, a gap, or an
ambiguity to record, and the decision record — approved and committed. The decision record is
kept after approval.

Read the kotowari skill's scene write (`references/ir-form.md` and `references/records.md`)
before writing the IR or the decision record: the IR's form, the record's form, and where each
kind of content belongs are defined there, not here. Before approval, read its scene check
(`references/findings.md`) and its `references/collate.md`.

## The tree and its rounds

Every decision has prerequisites (decisions made before it) and, once made, branches into
smaller decisions. Work the tree in rounds: collect the questions whose prerequisites are all
settled; number them; attach a recommended answer to each (the person should not have to answer
from a blank page outside their expertise); present the round; recompute the tree from the
answers; repeat.

```
Q1: <question>

A. <recommended answer>
```

A question left unanswered is not decided: its recommendation is **not** adopted, it stays an
implicit assumption, and it is asked again next round. Finish only when every branch is
walked and no implicit assumption remains.

## Conduct

- Never propose a whole design and ask for approval; never fire yes/no questions to
  manufacture a "decided" state. Recommended answers on granular questions are different.
- Do not re-ask what was decided without a reason. Challenge the person's ideas and your own
  with counter-arguments and counter-examples; "no objection" is not agreement.
- Say which statements are verified facts, inferences from facts, and unverified hypotheses.
- Before proposing to replace something that exists, read it; never judge by name or summary.
- Delegate fact-finding to a separate-context agent and keep asking meanwhile.

### Terms and scenarios

When a word can be read two ways, ask "does *X* mean this?" with your reading as the
recommended answer, in the normal round. Settled definitions go into the glossary (a
`CONTEXT.md` in the IR store; which one follows the glossary chain in the kotowari skill's
`references/records.md`): what the term *is*, and the words not to use for it. Only words
read two ways whose meaning here is this project's — no general vocabulary in its general sense.

When deciding how concepts relate, present concrete scenarios — normal and edge cases — and ask
"what happens here?" so the person meets boundaries they had not considered. Only scenarios
whose prerequisites are settled go into a round. Counter-examples found this way become the
counter-examples attached to requirements.

When a requirement does not change observable product behavior (release automation, licensing,
CI), ask whether it belongs here, in a separate specification, or is not built. Never add it silently.

Whoever notices an ambiguous term or a boundary that disagrees with the code asks it there, records
what becomes clear, and hands it back here. Change the glossary and specification after the person
decides.

## Records

Six kinds, kept in the decision record and defined in `references/records.md`: agreement, prohibition,
undecided (with who decides), delegated (with reason), rejected (with reason), revision (what replaced
what). Never merge undecided with delegated. When meaning changes, record the revision instead of
rewriting the decision; resume from the record.

## Writing the specification

Before writing, check for drafts, contradictions, vague words, missing scope boundaries,
related undecided items, and anything decided silently; any of these sends you back to the
dialogue. Specification silence never means "implementer decides".

Each requirement has an observable success condition and a counter-example, written as scenarios
under the IR's `## Examples` (the kotowari skill's `references/records.md` says how). Test its verification against **Evidence conditions**. On failure, express a non-code requirement
as human or platform inspection; drop a code behavior into an already reachable generic error path,
recording it as rejected with its missing conditions. Agreements go into the IR documents, each item
citing the decisions it rests on; prohibitions and rejected / undecided / delegated items stay in the
decision record's own sections, since an IR document holds only requirements, decision tables,
properties, and scenarios. Require
expensive model-running verification only when the person asks.

A requirement that is specified now but will not be built now (an upstream constraint, a later
phase) is deferred rather than left out: write the decision to defer it, with its reason, in the
decision record, and cite that decision as the source on a `- deferred:` line — under the
requirement, or once after the title of a topic document to defer all its requirements (the
kotowari skill's `references/ir-form.md`). Deferring keeps its form and reference checks and its
ID; it only stops check from asking for its tests. When the person decides to build it, remove
the line in the same way, with the decision recorded. A depends_on_deferred notice means an item
that will be built relies on a deferred one; settle it with the person — defer that item too,
bring the requirement back, or drop the reference.

### Evidence conditions

An oracle — a test, a check, or a fixture — counts as evidence only when the condition it
produces has a named operational producer in a supported environment (untrusted input arriving
at a boundary is one), its subject is the product or a check rather than the oracle itself, the
rule it enforces is stated by the specification, and every wording, file layout, or internal
name it pins is declared there as a contract. An oracle that fails any of these is a cost: do
not add it, keep it in a change under review, or demand it.

A requirement whose only oracle would fail these conditions is not mechanically verifiable:
when it is not code, verify it by a human-run check or by the platform's own checker; when it
is code, drop the requirement and let the failure join a generic error path a reachable
failure already proves — never resolve it by having the implementer build the fixture.

Source: `ba0918-verification`, agentic-rules v0.8.0.

## Finishing

1. Adversarial review, only when the specification's own requirements can contradict each other —
   several interdependent requirements, or one that two decisions in the record could pull apart.
   Say that reason, then launch one separate-context agent on the IR's own quality. Conformance
   to the decision record is not a review here: the collation review in step 3 replaces it. A
   specification of a few standalone requirements gets none. New findings become branches; keep
   asking.
2. Check the conditions for handing to plan: one deliverable (one branch); result in one sentence;
   built and unbuilt scope; stored state and its lifetime decided or confirmed absent; external
   dependencies accepted or rejected; human decision points and what they see; an ID for every
   requirement.
3. Approve in this order:
   1. Run `kotowari check --format json`. Exit code 2 is a stop; handle it as the kotowari skill's
      `references/findings.md` says. Fix the errors among `findings` whose `path` is a file in the
      IR store or in the decision records until none remain. Test-side findings
      (requirement_without_test, scenario_without_test, and findings on test files) may remain at
      approval; the cycle brings them to zero at its end. Notices (too_many_lines,
      too_many_requirements) may remain; to keep one, reread the document as the kotowari skill's
      `references/ir-form.md` section "Limits and the unit of splitting" says, and write the reason for keeping
      it in the decision record.
   2. Collation review: count the collation scope as the kotowari skill's
      `references/collate.md` says — the items this brainstorm added or changed, and the existing
      items citing a decision it changed. Write one line with the count and the resulting choice
      before acting. With a scope of zero, do not collate. Otherwise give a separate-session LLM on
      the most capable model tier available (never a lower tier to save cost) the items in scope
      and the decisions they cite, as `collate.md` directs, and have it list
      the items whose cited decisions do not support their content. For each item in the returned JSON's `unsupported`,
      add a source, add a decision to the record, or fix the item. After changing the IR or the
      record, go back to step 1 (check) before collating again. The second and third collations take only
      the items flagged last time and what was fixed since. What still remains at the third
      collation becomes a problem record (FLAG) and goes back to the person.
   3. Stage the IR documents, the glossary, the problem record, and the decision record. Show the
      person the decision record's diff (one decision per line; this is what they read), the check
      output (the count of test-side findings, and each document whose notice was kept, with the
      reason), the collation result, and the paths approved with a content identifier for each —
      never the full text, never the IR's diff as something they must read, and never a summary as
      the thing approved. The person commits or says to.

## Decisions carried into change conformance

For changes-enabled projects read the kotowari skill's changes scene. Make delegated concrete choices and their limits explicit in the decision record and IR, so that implementation can record additions within them; changes to approved requirements come back here.
