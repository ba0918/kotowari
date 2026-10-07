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

- Before writing tests or changing their marks, read the kotowari skill's `references/mark.md`
  (its path is given in this prompt). Artifact-only work does not need it.
- For a check oracle, run the plan's commands in order, unedited.
- Hand back a missing design decision.
- If a hook stops a commit on an IR-side finding, never edit the IR: leave the change uncommitted
  and return the finding with the files it blocked.
- Return your commits and which finding each addresses, or a hand-back with its reason.
