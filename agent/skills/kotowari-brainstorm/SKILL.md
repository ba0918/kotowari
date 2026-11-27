---
name: kotowari-brainstorm
description: "Workflow station of the kotowari workflow: interview the person until shared understanding is complete, then write the specification and get it approved. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/decisions/`). Use when asked to brainstorm, define requirements, extract a specification, or resolve an ambiguity in one. 日本語キーワード: 壁打ち 要件定義 仕様抽出 ブレインストーム 質問ラウンド ドメインモデリング"
---

# Brainstorm

Interview the person until nothing is left implicit, then write what they decided into the
decision documents. Only what the person alone can settle is asked; the rest you decide with
grounds and show at approval. This is domain modeling: the goal is shared understanding, not a
document. Compromise is not allowed, and the station never ends itself — it ends when the tree is exhausted,
the scene it started from is solved, and the person approves.

## Inputs and outputs

In: what the person wants to talk about; when revising, the existing decision documents of the
topic. Before the first round, look at the uncommitted changes under the place of the decision
documents (`git status --porcelain -- <place>` and `git diff -- <place>`): a brainstorm that
stopped part way left its decisions there, and resumes from them. Note the branch the main working
tree is on when the brainstorm begins; **Finishing** puts it back there. A brainstorm that resumes
after kotowari-cycle handed the work back writes in that run's worktree, on its branch, and cuts no
branch.
Out: the decision documents changed — new decisions written as proposed, overturned ones replaced,
open items under `## 未決` — approved and committed; and the list of the decisions to build this
time with the list of what was decided without asking, which a commit body on the branch also
carries, handed to kotowari-cycle.

Read the kotowari skill's scene decide (`references/decision-documents.md`) before writing a
decision document: the form check reads, the states, the rows to confirm and their IDs, the open
section, links and the pair of language sides are defined there, not here. Section names and
keywords in this skill are given in their Japanese spelling; an English side writes the English
spelling of the same keyword (that reference's table of keywords). Before approval, read its scene
check (`references/findings.md`).

## Before the first round

1. **The person's standing principles.** Read the principles file shared by every project of this
   person before recommending anything: `kotowari/principles.md` under the person's configuration
   directory (`$XDG_CONFIG_HOME` when it is set, else `~/.config`). Recommend nothing that
   contradicts it without saying which line it contradicts and why. When the file does not exist
   or cannot be read, say so at the start of the first round and go on without it. When the person
   corrects a recommendation because it went against a principle they hold across projects, add
   that correction to the file as one line, in their words, at that moment.
2. **Where the request came from.** Look into its origin before asking anything: the records of
   the person or repository that raised it, the issue, the earlier exchanges it points to. What is
   written there is grounds; do not ask the person what it already answers.
3. **The scene.** Build one concrete scene of the person in trouble: who does what, what happens
   now, and what should happen instead. When the origin holds the scene and the answers to what is
   to be decided, use them as grounds without asking. Only when it does not, build the scene from
   the person's words and ask in the first round whether it is right. This scene is what
   **Finishing** shows solved before approval.

## The tree and its rounds

Every decision has prerequisites (decisions made before it) and, once made, branches into
smaller decisions. Work the tree in rounds: collect the questions whose prerequisites are all
settled and that may be asked (**What goes in a round**); number them; attach a recommended
answer to each (the person should not have to answer from a blank page outside their expertise);
present the round; recompute the tree from the answers; repeat.

```
Q1: <question>
<background: what reading and trying showed>

A. <recommended answer>
```

A question left unanswered is not decided: its recommendation is **not** adopted, it stays an
implicit assumption, and it is asked again next round. Finish only when every branch is
walked and no implicit assumption remains.

### What goes in a round

A question is put before the person only when it is one of these kinds:

- the product's direction, preferences and trade-offs that neither reading nor trying settles;
- something whose change after it is decided means rebuilding what was built or the data that has
  accumulated;
- something the request's text, the sample or the records give a clue that the person cares about.

Before asking, read what reading answers, and try small what trying answers (a throwaway run, kept
out of the commits); write what they showed in the question's background. A question that is none
of the kinds is not put before the person in any form, wherever the rounds run: decide it yourself
with grounds, and it goes in the list of what was decided without asking (**Finishing**). A
question for which you cannot judge whether it is one of the kinds is asked.

This standard is the brainstorm's. It does not apply to a small task kotowari-iterate runs or to
taking in existing code (kotowari-adopt).

## Conduct

- Never propose a whole design and ask for approval; never fire yes/no questions to
  manufacture a "decided" state. Recommended answers on granular questions are different.
- Do not re-ask what was decided without a reason. Challenge the person's ideas and your own
  with counter-arguments and counter-examples; "no objection" is not agreement.
- Say which statements are verified facts, inferences from facts, and unverified hypotheses.
- Before proposing to replace something that exists, read it; never judge by name or summary.
  Read the decisions it rests on too: a decision not to build something binds the replacement.
- Delegate fact-finding to a separate-context agent and keep asking meanwhile.

### Showing a sample

When the request changes behavior a user can see (an output, a screen, a message, how a command
is used), do not settle it by asking in words. Make a sample and show it, and turn the person's
reaction into the decision. The sample is a working prototype, a mock, or a sample of the output;
which one to make, and whether to make one at all, you decide on the spot with a reason, without
asking the person. No step is added where a person tries the built product before the merge.

A working prototype is thrown away once it has done its job: it is never the base of the product
code. Hand its path to the implementer as something to compare with (with the list of decisions,
**Finishing**). What the prototype settled that a machine can check becomes a row to confirm, so
that a test carries it.

### Terms and concrete cases

When a word can be read two ways and its reading is one of the kinds **What goes in a round**
names, ask "does *X* mean this?" with your reading as the recommended answer, in the normal round;
otherwise decide the reading as that section says. A settled definition used by several decisions goes into
the decision that settled it, and the others link to it; the meaning of a term goes into the
project's glossary.

When deciding how concepts relate, work through concrete cases — normal and edge cases — so that
no boundary is left unconsidered. A case goes into a round, as "what happens here?", only when its
prerequisites are settled and it is one of the kinds **What goes in a round** names; decide the
others as that section says. A counter-example found this way becomes a row to
confirm, or a rejected option with its reason.

A question comes back here from implementation only when two decisions the person made cannot
both hold; it arrives as an item under `## 未決` of the topic. Settle it with the person, write the
decision, and remove the item. Change the decision documents after the person decides.

## Writing the decisions

Write each decision into its topic document as soon as the person makes it, in the form of the
kotowari skill's `references/decision-documents.md`, with the state `提案` (proposed). Keep no
record per brainstorm and no IR, and do not restate the decisions in another document: the topic documents
are the only place the decisions go, and a resume reads them back from the working tree.

When `languages` lists two or more tags, each topic document is a pair: write every side in the
same change, each in its own language and keywords, with the same headings, states and row IDs,
and bring the agreement record up to date before approval (that reference's **Languages: a pair of
sides**). The person approves the side in the language they read; the other sides say the same.

- **Only what a person decided.** A decision document holds the person's decisions. What you
  decided without asking is not written; it goes in the list (**Finishing**). A detail nobody
  raised in the brainstorm (a limit, an order, the wording of a message nobody specified) is not
  written either; it is decided during implementation and explained in the commit and pull request
  bodies. A recommendation the person did not answer is not a decision and is not
  written.
- **What was decided is the present rule.** Write no history into it. Why the decision was made goes
  in the problem and in the sentences of what was decided; options considered and dropped go in the
  rejected options, each with why it lost. Never invent a rejected option nobody considered.
- **A decision that concerns two topics** is written in the topic where the behavior is visible
  (where its rows to confirm can be placed); the other topic links to its heading.
- **A decision not to build something** is written as a decision of its topic (its heading says
  what is not built), or as a rejected option of the decision it concerns.
- **Open questions.** What the person has not decided yet, and what must be decided by someone at a
  later point, goes under `## 未決` at the end of the topic document: what is open, who decides it,
  and when. A detail the implementer may decide never goes there.
- **A fact that changed** (a value, a name, a path) is rewritten in place.
- **Overturning a decision.** Write a new decision. Put the old decision and why it was dropped in
  its rejected options as `以前の決定:`, move over the old reasons and rejected options that still
  mean something, and delete the old decision's text. Leave the tests that pointed at the old
  rows to the implementer: list those row IDs with the decision that replaces them in the handover
  (**Finishing**), so that the tests are pointed at the new rows or deleted.
- **Rows to confirm.** Give each decision that changes something observable the rows that show it
  is kept, worded for the person approving it; the exact inputs and outputs go in the tests later.
  Rows added to a decision already `実装済み` (implemented) leave its state as it is; the
  implementer writes their tests.
  A row only a reading can confirm (skill text, a document) starts with `（読んで確認）`. A row whose
  only oracle would fail the Evidence conditions (the kotowari-review skill's
  `references/oracle-evidence.md`) is reworded, made a row confirmed by reading, or left out, as
  that reference's **A row no oracle can meet** says. Require expensive model-running
  verification only when the person asks.
- A decision about the process, the build or the release is written in the same kind of document,
  without rows when nothing observes it.

Before calling the writing done, check for drafts, contradictions between decisions, vague words,
missing scope boundaries, related open items, and anything decided without asking that is missing
from the list; any of these sends you back to the dialogue.

## Finishing

1. Adversarial review, only when the decisions can contradict each other — several interdependent
   decisions, or one that two existing decisions could pull apart. Say that reason, then launch one
   separate-context agent on the decision documents' own quality. A specification of a few
   standalone decisions gets none. New findings become branches; keep asking.
2. Check the conditions for handing to cycle: one deliverable (one branch); result in one sentence;
   the list of decisions to build this time, every one of them proposed; built and unbuilt scope
   (a proposed decision not on the list stays proposed); stored state and its lifetime decided or
   confirmed absent; external dependencies accepted or rejected; human decision points and what
   they see.
3. Approve in this order:
   1. Run `kotowari check --format json`. Exit code 2 is a stop; handle it as the kotowari skill's
      `references/findings.md` says. Fix the errors among `findings` whose `path` is a decision
      document until none remain, test-side findings apart (that reference defines them: a marker
      left pointing at a row this brainstorm deleted, a row added to an implemented decision). They
      may remain at approval; the cycle brings them to zero at its end.
   2. Show the scene solved. Walk the scene from **Before the first round** through what was
      decided, and show the person what happens in it now. When it is not solved, do not go on to
      approval: go back to the rounds. When it cannot be judged without the real thing, decide when
      and by looking at what it will be judged, and write that under `## 未決` of the topic.
   3. Only in a project whose configuration has the `overview` key, rebuild the overview, as the
      kotowari skill's `references/overview.md` says. The overview is reference material, not
      something the person approves; it changes neither what is shown for approval nor the order
      in which the person reads it:
      1. From the decision documents this brainstorm changed, find the overviews involved through
         the `documents` lists of the overview data. For a changed decision document of a topic no
         overview lists, propose adding it to an existing overview or starting a new one, and let
         the person decide. The overview of an existing topic is made the first time a brainstorm
         touches its decision document.
      2. Create or revise that overview data, and place each new page in the table of contents
         without asking the person. Fix the guide_stale notices on that overview data here.
      3. Run `kotowari overview build` and fix the errors until it exits 0.
      4. Give the URL `kotowari overview serve` prints, as reference; whether to look is the
         person's choice. Where nothing can run in the background, give the command instead.
   4. Stage the decision documents, and the overview data and table of contents the step above revised. Show
      the person the diff of the decision documents (this is what they read and approve; with pairs, the diff of the side in the language they read; the
      overview files are not shown), the list of decisions to build, the check output (the count of
      test-side findings), and the paths approved with a content identifier for each — never a
      summary as the thing approved. With the diff, show the list of what was decided without
      asking: each question that was none of the kinds of **What goes in a round** and that you
      decided with grounds, one line each of what was decided and why; answers taken from the
      origin and the choice of a sample are not in it. When nothing was decided without asking,
      say there is none. An item the person says is wrong is something the person decided: write it
      in the decision documents, take it out of the list, and show the changed diff and identifiers
      again. A list is made at each approval; when the branch already carries one, the new list
      starts from it and keeps the items that still stand, since the newest replaces it whole.
      The person commits the staged files or says they may be committed, on a new branch named
      for the topic, which the brainstorm cuts in the main working tree from the head of the merge
      target (the branch the person names, else the repository's default branch):
      `git fetch origin <target>` and `git switch -c <branch> origin/<target>`, the staged files
      going with it.
4. The commit comes before leaving the new branch: when the person said the staged files may be
   committed rather than committing them, the brainstorm commits them there (`git commit`). The
   list of what was decided without asking goes in the body of that commit, under a line holding
   only `聞かずに決めたこと:` (`Decided without asking:` where the commit messages are in English),
   one line per item of what was decided and why, or `なし` (`None`) under it when the list is
   empty. When the person committed the staged files themselves, the brainstorm adds to the same branch, before leaving it, a commit
   with no change to files (`git commit --allow-empty`) whose body carries the list in the same
   way. Then
   `git status --porcelain -- <the staged paths>` must print nothing and
   `git diff --name-only origin/<target> HEAD -- <the staged paths>` must list every one of them;
   until both hold, do not switch: tell the person which approved files are not committed on the
   new branch, and stop. Then put the
   main working tree back on the branch noted at the start (`git switch <that branch>`) and check
   the new branch out in a worktree of its own (`git worktree add <path> <branch>`, where the
   project's instructions place worktrees); a brainstorm resumed after a hand-back commits in its
   worktree and already has both. When the main working tree cannot be switched, to cut the
   branch or to go back, stop and tell the person. Then go on to kotowari-cycle in the same flow
   unless the person says to put it off. Hand it the place of the decision documents, the list of
   decisions to build this time (each as the topic document's path and the heading's anchor,
   `docs/decisions/core/confirmations.md#<anchor>`), the list of what was decided without asking,
   the row IDs of overturned decisions with the decision replacing each, the path of any prototype
   to compare with, the merge target, and the branch with its worktree path.
