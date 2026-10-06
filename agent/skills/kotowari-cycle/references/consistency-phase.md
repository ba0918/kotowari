# Consistency phase

The instructions for the agent that runs the consistency phase. Cycle pastes this file in full into
that agent's prompt; the agent runs in a context separate from the implementer and reads no skill.

The phase reads the code a diff changed against the IR, finds where the two do not agree and where
the IR itself has a gap or a contradiction, and resolves every finding into a state where the IR and
the code agree. It reads the code itself, never a record about the code.

## Inputs

- The worktree path and the branch.
- The range: the base and head of the diff to read. When a hook stopped the implementer's (or, on
  a direct edit, the main session's) commit on an IR-side finding, the change it left uncommitted in the worktree is in range too. The phase
  commits its own fixes only, never that change; when a resolution changes that code, it edits it
  in place and leaves it uncommitted for whoever made it.
- The counterpart: the IR store path and a file listing the IDs the plan covers. For a topic with no
  IR, the approved specification document, or the request when there is none.
- The open findings of earlier runs that await a fix, with their IDs and commits; the open ones
  awaiting a person's judgment (`human_judgment`) as known findings, which it neither evaluates nor
  raises again, since they stay open for the final report; and the path of the kotowari-review
  skill's `references/finding-schema.md` (the shape of a finding).
- The kotowari skill's reference directory and its **What the IR holds** section pasted in full;
  the **Editing contract** of the kotowari-cycle skill's `references/editing-contract.md`, with
  the Evidence conditions. Read `findings.md` to interpret check;
  read `records.md` when recording a decision, `ir-form.md` when editing IR, `translations.md`
  when changing a language pair, and `mark.md` when writing tests. References are read on demand,
  not as a prerequisite to inspecting the diff.

## What it reads

- **Code**: the code the diff changed, its direct callers and callees, and one step of where the
  values the diff uses are produced (the function or step that builds a value the changed code
  reads, even when the diff reuses it unchanged). Code includes text shipped as product, such as
  skill text: read it as code, not as a document.
- **Items in range**: each item the diff touches, each item whose ID the plan covers, and each item
  whose ID a mark the diff adds carries.
- Around those items: the terms they use (the glossary), the relations between items (definition
  tables, `@about`, references), and the decision records that the items and terms in range cite.

Nothing outside that range. A comparison of the whole code base with the whole IR runs only when
asked for separately.

Every run starts with `kotowari check --format json` (read as `findings.md` says). Its IR-side
findings and its deferred_with_test and depends_on_deferred notices whose `path` is a file the
range changed or that name an ID in range are findings of this run, beside what the reading finds.

## What it looks for

- **Gap**: behavior in the code that the IR does not state.
- **Disagreement**: behavior different from what the IR states. A reading that widens or narrows
  the definition of a term or a table is a disagreement, not a concretisation.
- **Within the IR**: a gap or a contradiction among the items in range.

Apply the supplied **What the IR holds** definition before calling an absence a gap: a rule
outside that scope belongs in a decision record, never in an IR item.

## How a finding is resolved

Every finding ends in a state where the IR and the code agree. When the IR has a hole and the code
is right, never put the code back to match the IR. This replaces the cycle's former handling: the IR
is no longer left to the fixer, and a contradiction is no longer returned to the person; both are
resolved here as below.

- What observation can settle is settled by measuring or reproducing it: run the command or a
  throwaway probe, keep it out of the commits, and decide by its output.
- A concretisation consistent with the existing IR is added to the IR and to the decision record
  with its grounds.
- For a disagreement with the existing IR, read the reasons in the decision records the item cites
  as constraints. Revise the IR, keeping the reason, only when the IR is shown to be wrong;
  otherwise bring the code to the IR. Only a contradiction between parts of the IR, or a
  reproduction showing the IR cannot be realised, shows that it is wrong. Passing tests are not
  grounds.
- A question of meaning that no experiment can settle: choose a default that does not contradict
  the IR, make the IR and the code agree on it, and continue. Leave it in the flag record
  (`FLAGS.md`) and return it as a finding with the action `human_judgment`: its claim states the
  default and the one word that reverses it, and its `oracle.note` the grounds and measurements
  tried and why they could not settle it. It stays open for the final report; the phase never stops
  to hand it to a person. A person's judgment is the last resort, kept to as few findings as
  possible: only what the grounds and measurements cannot settle.
- On agreement with the specification, this phase's decision takes precedence over the quality
  review's fixes.

## Fixing and recording

- The phase fixes and commits the IR, decision record, flag record and code itself under the supplied
  Editing contract. The fixer's instruction to leave IR alone and return on a hook's IR-side
  finding does not apply: fixing IR is this phase's job. Resolve missing meaning by **How a finding
  is resolved**, not a hand-back. Check commands come from the project's instructions, then the
  ecosystem's standard tool; tests carry marks as `mark.md` says.
- What it decided is written to a new decision record for this run (`records.md` gives its place and
  form). Each decision names the consistency phase as the decider (`decided_by`), with the choice,
  the reason and pointers to the grounds: the measuring commands and their results, files and lines.
- When it fixes the IR, it keeps the language pairs and their consistency records aligned
  (`translations.md`).
- Every run ends with no IR-side finding of `kotowari check` left in its range. Test-side findings
  (such as an item the implementation has not reached yet) and findings outside the range do not
  count: they are the implementer's and the fixer's, or not this run's.

## Running again and what it returns

Cycle puts its findings in its findings file beside the review's, with the perspective
`consistency`, and decides reruns and when to stop by its own rules; the phase keeps no state. A
rerun has the head moved to the current head, so its range includes the fixes, and it also reads
whether the grounds recorded for each choice support it. A run returns:

- for each earlier finding it was given to evaluate (not a known one), `still_present` or `no_longer_visible`, with the commits of
  any further fix;
- each new finding in the finding shape, without `id`, `status` and `evaluations`: the perspective
  `consistency`; the action `fix_and_verify` for one it resolved (`human_judgment` only as above),
  the severity `critical` when it changed what a user observes and `warn` otherwise, and the profile
  from the file it concerns; `oracle.measured` `fails_now`, since the measurement showed it before
  the fix (`not_applicable` on a `human_judgment` one, which no measurement settled); the claim says whether it is a gap, a disagreement, or a gap or contradiction within
  the IR; the evidence is the file and line or the ID; the oracle is the measurement or check that
  decided it; `commits` are the commits that resolved it;
- the path of the decision record it wrote, if it decided anything.
