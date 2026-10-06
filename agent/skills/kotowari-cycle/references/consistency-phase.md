# Consistency phase

The instructions for the agent that runs the consistency phase. Cycle pastes this file in full into
that agent's prompt; the agent runs in a context separate from the implementer and reads no skill.

The phase reads the code a diff changed against the IR, finds where the two do not agree and where
the IR itself has a gap or a contradiction, and resolves every finding into a state where the IR and
the code agree. It reads the code itself, never a record about the code. Grounds and measurements
decide; a person is involved only as a last resort.

## What it is given

- The worktree path and the branch.
- The range: the base and head of the diff to read. When a hook stopped the implementer's (or, on
  a direct edit, the main session's) commit on an IR-side finding, the change it left uncommitted in the worktree is in range too. The phase
  commits its own fixes only, never that change; when a resolution changes that code, it edits it
  in place and leaves it uncommitted for whoever made it.
- The counterpart: the IR store path and a file listing the IDs the plan covers. For a topic with no
  IR, the approved specification document, or the request when there is none.
- The findings the previous run with the same base returned, if any.
- The paths of the kotowari skill's references it follows when it writes: `ir-form.md` and
  `records.md` (the IR and decision records), `translations.md` (language pairs),
  `findings.md` (reading `kotowari check`) and `mark.md` (marks on tests); the kotowari skill's
  **What the IR holds** section; and cycle's fixer contract for changes to code.

## What it reads

- **Code**: the code the diff changed, its direct callers and callees, and one step of where the
  values the diff uses are produced (the function or step that builds a value the changed code
  reads, even when the diff reuses it unchanged). Code includes text shipped as product, such as
  skill text: read it as code, not as a document.
- **Items in range**: each item the diff touches, each item whose ID the plan covers, and each item
  whose ID a mark the diff adds carries.
- Around those items: the terms they use (the glossary), the relations between items (definition
  tables, `@about`, references), and the decision records that the items and terms in range cite.
- For a topic with no IR, the counterpart is the approved specification document, or the request
  when there is none.

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

The IR holds only what a user of the product can observe (the kotowari skill's **What the IR
holds**, which the phase is given). CI and workflow definitions, hooks, the release procedure, the
build configuration, rules about the project's own tests and checks, and the development process
are not IR: behavior there that no IR states is not a gap. A decision about it goes to the decision
record, never to an IR item.

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
  (`FLAGS.md`) as awaiting a person, and return the default with the one word that reverses it.
- Only what still needs a judgment after the grounds and measurements are exhausted goes back to a
  person: hand it back with its reason.
- On agreement with the specification, this phase's decision takes precedence over the quality
  review's fixes.

## Fixing and recording

- The phase itself fixes the IR, the decision record, the flag record and the code, and commits
  them. A change to code follows the fixer contract it was given, and a test it writes carries marks
  as `mark.md` says. Of that contract, these apply: test-first for code, with a failing test
  meeting the Evidence conditions; the artifact and deletion rules; commit hygiene; and stopping
  before an irreversible or privileged operation. Its rule to hand back a missing design decision
  does not: **How a finding is resolved** replaces it. There is no plan, so its check commands come
  from the project's instructions, then the ecosystem's standard tool.
- What it decided is written to a new decision record for this run (`records.md` gives its place and
  form). Each decision names the consistency phase as the decider (`decided_by`), with the choice,
  the reason and pointers to the grounds: the measuring commands and their results, files and lines.
- When it fixes the IR, it keeps the language pairs and their consistency records aligned
  (`translations.md`).
- Every run ends by passing `kotowari check`, with none of the range's findings left: an IR item it
  adds gets a mark on an existing test or a test it writes, as `mark.md` says.
- One concern per commit; `git add <path>` only; never disable hooks; never name a station or a
  finding ID in a commit message.

## Running again and ending

- One delegation is one run. After a run that fixed something, cycle delegates another run with the
  same base and the head moved to the current head, so the range includes the fixes; that run also
  reads whether the grounds recorded for each choice support it.
- Each finding a rerun returns is marked `new`, or `repeat` when it is the same gap or disagreement
  at the same place as a finding of the previous run it was given.
- A `repeat` is no progress, the same finding twice in a row: cycle ends the loop by its ending for
  no progress.
- Otherwise, a run that raises no new finding is convergence.

## What it returns

- Each finding of this run: what it is (gap, disagreement, or a gap or contradiction within the IR),
  where (file and line, or ID), `new` or `repeat`, how it was resolved, and its commits.
- The path of the decision record it wrote, if it decided anything.
- Each default left awaiting a person: the flag ID, the default, and the word that reverses it.
- Anything handed back to a person, with its reason.
