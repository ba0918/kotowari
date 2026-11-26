# Editing contract

The text cycle pastes into the prompt of an agent that edits for it. Cycle's **Delegations** says
who gets which section; everything below this paragraph is pasted as written.

## Editing contract

- Code: RED → GREEN → REFACTOR, with a test run at every transition. A failing test you write
  must meet the Evidence conditions (the kotowari-review skill's `references/oracle-evidence.md`,
  pasted below this contract when cycle delegates).
- Artifact: leave it judgeable by an independent review and pass its format check.
- Deletion: completion is all existing checks passing after the deletion; no failing test is
  needed.
- Stop and ask before an irreversible or privileged operation, a dangerous target, or an accident
  that would spread.
- One concern per commit; stage with `git add <path>` only; never disable hooks; never name a
  workflow station or a finding ID in a commit message.
- A question a throwaway run can answer is a fact: run it, keep it out of the commits, and report
  the command and its output.
- For a finding that is still present after an earlier fix (that fix's commits are given), report,
  before changing code, the one-sentence premise that fix assumed and the output of a command you
  ran to test it. Start the next fix from that result; another fix resting on the same premise is
  not a changed approach.

## Fixer only

- Before writing tests or changing their markers, read the kotowari skill's `references/mark.md`
  (its path is given in this prompt). Artifact-only work does not need it.
- For a check oracle, run the check commands the project's instructions give, else the
  language's standard tool, unedited.
- Decide a detail the decision documents do not write, or a reading of a sentence that reads two
  ways, yourself with grounds, and report it as a decision line. Hand back only a collision of
  intent: two sentences of the decision documents that cannot both hold, so that either choice
  drops something a person decided. Before handing it back, write it as an item under `## 未決` at
  the end of the topic document it concerns (the two sentences linked, and that the person decides
  it) and commit that item. When the decision documents are kept in language pairs, write the item
  on every side (`## Open` on an English side) and update the agreement record (the kotowari
  skill's `references/decision-documents.md`, **Languages: a pair of sides**).
- Apart from that item, never edit a decision document. If a hook stops a commit on a finding in a
  decision document, leave the change uncommitted and return the finding with the files it
  blocked.
- Return your commits and which finding each addresses, and each decision you made by yourself (a
  dependency you added included) as one line of what was decided, why, and the evidence, the same
  line written in the body of the commit that carries it; or a hand-back with its reason.
