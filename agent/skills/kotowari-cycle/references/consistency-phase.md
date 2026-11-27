# Consistency phase

The instructions for the agent that runs the consistency phase. Cycle pastes this file in full into
that agent's prompt; the agent runs in a context separate from the implementer and reads no skill.

The phase reads the code a diff changed against the decision documents, finds where the two do not
agree and where the decisions themselves collide, and resolves every finding into a state where
the decision documents and the code agree. It reads the code itself, never a record about the code.

The decision documents hold only what a person decided, one topic per file under the place of the
decision documents (the form is the kotowari skill's `references/decision-documents.md`). A detail
they do not write is the implementer's to decide with grounds; it is not a hole to fill in them.

## Inputs

- The worktree path and the branch.
- The range: the base and head of the diff to read. When a hook stopped the implementer's (or, on
  a direct edit, the main session's) commit on a finding in a decision document, the change it left
  uncommitted in the worktree is in range too. The phase commits its own fixes only, never that
  change; when a resolution changes that code, it edits it in place and leaves it uncommitted for
  whoever made it. The range leaves out the branch's plan, `docs/plans/<branch>.md`: it is not
  read, and no finding is raised on it. Any other file under `docs/plans/` is in range.
- The counterpart: the place of the decision documents and a file listing the decisions the cycle
  received, one `path#anchor` per line. For a small task without a specification, the request.
- The open findings of earlier runs that await a fix, with their IDs and commits; the open ones
  awaiting a person's judgment (`human_judgment`) as known findings, which it neither evaluates nor
  raises again, since they stay open for the final report; and the path of the kotowari-review
  skill's `references/finding-schema.md` (the shape of a finding).
- When cycle sends a premise to replace: the premise ladder and the replaced premise, as one more
  finding to resolve by **How a finding is resolved**.
- The kotowari skill's reference directory; the **Editing contract** of the kotowari-cycle skill's
  `references/editing-contract.md`, with the Evidence conditions. Read `findings.md` to interpret
  check, `decision-documents.md` before editing a decision document, `translations.md` when
  changing a language pair, and `mark.md` when writing tests. References are read on demand, not
  as a prerequisite to inspecting the diff.

## What it reads

- **Code**: the code the diff changed, its direct callers and callees, and one step of where the
  values the diff uses are produced (the function or step that builds a value the changed code
  reads, even when the diff reuses it unchanged). Code includes text shipped as product, such as
  skill text: read it as code, not as a document.
- **Decisions in range**: each decision the cycle received, each decision whose text the diff
  touches, and each decision holding a row whose ID a marker the diff adds carries.
- Around those decisions: the decisions they link to, and the terms they use.

Nothing outside that range. A comparison of the whole code base with all the decision documents
runs only when asked for separately.

Every run starts with `kotowari check --format json` (read as `findings.md` says). Its findings
whose `path` is a decision document the range changed, or that name a row of a decision in range,
are findings of this run, beside what the reading finds.

## What it looks for

- **Disagreement**: behavior different from what a decision states. A reading that widens or
  narrows a decision's definition of a term or a table is a disagreement, not a detail.
- **A detail without its reason**: behavior the code has that no decision states and that the
  implementer decided (a limit, an order, a message). It is not written into the decision
  documents; what is checked is that it contradicts no decision and that its reason is stated.
- **A collision of intent**: two sentences of the decision documents in range that cannot both
  hold, so that either choice drops something a person decided.

## How a finding is resolved

Every finding ends in a state where the decision documents and the code agree.

- What observation can settle is settled by measuring or reproducing it: run the command or a
  throwaway probe, keep it out of the commits, and decide by its output.
- A detail without its reason: when it contradicts no decision, keep it and write the line of what
  was decided, why, and the evidence. When it contradicts one, it is a disagreement.
- For a disagreement, read the decision's problem and rejected options as constraints, and bring
  the code to the decision. Only when a reproduction shows that a sentence of the decision cannot
  hold literally, and the direction the person decided does not change, correct the sentence as a
  rewrite of a fact (`decision-documents.md`, **Changing a decision**). Passing tests are not
  grounds.
- A question of reading that no experiment can settle: choose, with grounds, a reading that
  contradicts no decision, make the code agree with it, and write the line of what was chosen,
  why, and the evidence. Do not hand it to a person.
- A collision of intent is the one thing a person decides. Choose a default that keeps as much of
  both decisions as can be kept, make the code agree with it, and write the collision as an item
  under `## 未決` at the end of the topic document: the two sentences linked, the default taken, and
  that the person decides when that topic is next brainstormed. Return it as a finding with the action `human_judgment`: its claim states
  the default and the one word that reverses it, and its `oracle.note` the grounds and measurements
  tried and why they could not settle it. It stays open for the final report; the phase never stops
  to hand it to a person.
- On agreement with the decision documents, this phase's decision takes precedence over the
  quality review's fixes.

## Fixing and recording

- The phase fixes and commits the code, the tests and the decision documents itself, under the
  supplied Editing contract. The fixer's instruction to leave the decision documents alone and
  return on a hook's finding in a decision document does not apply: fixing them is this phase's
  job. Check commands come from the project's instructions, then the ecosystem's standard tool;
  tests carry markers as `mark.md` says.
- It writes no record per run. The reason for each fix — what was decided, why, and the evidence
  (the measuring commands and their results, files and lines) — goes in the body of the commit that
  carries the fix and in the line the run returns, which cycle puts in the pull request body.
- It writes into a decision document only a sentence corrected to the facts and an item under
  `## 未決`; it never writes a detail it or the implementer decided, and never turns a state.
  When the decision documents are kept in language pairs, every edit it makes goes to every side
  in its own language's keywords, with the agreement record updated (the kotowari skill's
  `references/decision-documents.md`, **Languages: a pair of sides**).
- When it fixes a decision document that has a language pair, it keeps the pair and its
  consistency record aligned (`translations.md`).
- Every run ends with no finding of `kotowari check` in a decision document left in its range.
  Test-side findings (such as a row the implementation has not reached yet) and findings outside
  the range do not count: they are the implementer's and the fixer's, or not this run's.

## Running again and what it returns

Cycle puts its findings in its findings file beside the review's, with the perspective
`consistency`, and decides reruns and when to stop by its own rules; the phase keeps no state. A
rerun has the head moved to the current head, so its range includes the fixes, and it also reads
whether the grounds given for each choice support it. A run returns:

- for each earlier finding it was given to evaluate (not a known one), `still_present` or
  `no_longer_visible`, with the commits of any further fix;
- each new finding in the finding shape, without `id`, `status` and `evaluations`: the perspective
  `consistency`; the action `fix_and_verify` for one it resolved (`human_judgment` only as above),
  the severity `critical` when it changed what a user observes and `warn` otherwise, and the profile
  from the file it concerns; `oracle.measured` `fails_now`, since the measurement showed it before
  the fix (`not_applicable` on a `human_judgment` one, which no measurement settled); the claim says
  whether it is a disagreement, a detail without its reason, or a collision of intent; the evidence
  is the file and line, or the decision's `path#anchor`; the oracle is the measurement or check that
  decided it; `commits` are the commits that resolved it;
- for each finding it resolved, the line of what was decided, why, and the evidence, and for each
  sentence it corrected to the facts, the old and new sentence and the fact that forced it.
