---
name: kotowari-cycle
description: >-
  Workflow station of the kotowari workflow: the orchestrator that runs an approved specification
  on a branch through implementation, the consistency phase, review, and fixing until the findings
  converge, then has the hosting service merge it or leaves the pull request to the person. Use
  only in a repository that uses
  kotowari (one that has `.kotowari/` or `docs/decisions/`). Use when asked to run a kotowari cycle on a
  specification, or to resume one. 日本語キーワード:
  サイクル 実装ループ 改善ループ オーケストレータ 仕様を実装する
---

# Cycle

Run implement → consistency phase → review → fix on one approved specification until the visible
findings are gone.
Delegate everything; cycle itself never implements, reviews, or fixes. A converged run merges
without waiting for a person unless `references/merge.md` says a person confirms; the loop does not
stop for them except in the cases below.

## Inputs

The specification is the decision documents, one topic per file under the place the
configuration's `decisions.documents` names (the kotowari skill's
`references/decision-documents.md`).

Required: the place of the decision documents, the list of decisions to build (each a topic
document's path and a heading's anchor, `docs/decisions/core/confirmations.md#<anchor>`), and the
branch with its worktree path. The brainstorm hands over all of them, with the merge target, as
it goes on to cycle (its **Finishing**), together with the row IDs of the decisions it overturned
(each with the decision replacing it) and the path of any prototype to compare with; for any other
start the main session prepares the branch, which carries the commit of the approved decisions,
and its worktree first. Cycle takes the list as it comes: it adds no decision to it.

A start that brings a request with the place of the decision documents confirms in the worktree
that the decisions are approved, after the reads at the head of **Resuming and running more** and
before it creates or changes the findings file. A resume, which brings no request, does not. The
place of the decision documents holds no uncommitted change:
`git status --porcelain -- <place>` prints nothing. And
`kotowari check --format json` exits 0 or 1 with no error whose `path` is a decision document the
branch changed (`git diff --name-only <comparison base> HEAD`), test-side findings (the kotowari
skill's `references/findings.md`) apart. When either does not hold, say the decisions are not
approved and stop. A small task without a specification has no place to confirm.

Then cycle creates the findings file (**Resuming and running more**) and writes into it the
decisions and the merge target it received (`ids`, one `path#anchor` entry per decision, an empty
list included, and `merge_target`); a later request on the same branch adds its decisions to
`ids`. A resume reads both from the file; only when the file has no `ids` does it count the
decisions again from the part of the branch's diff that lies in the place of the decision documents
(`git diff <comparison base> HEAD -- <place>`): the proposed decisions whose text that diff adds or
changes. "The decisions cycle received" below means this list; "a row of a decision cycle
received" means a row under `### 確認すること` of one of them. Section names in this skill are
given in their Japanese spelling (`### 確認すること`, `## 未決`); an English side of a decision
document uses the English spelling (`### Confirmations`, `## Open`), as the kotowari skill's
`references/decision-documents.md` lists.

Optional: round-trip limit (default none: loop until convergence), review strength (the
person's choice, default `standard`), merge target (the branch the brainstorm or the person
decided, else the repository's default branch, `gh repo view --json defaultBranchRef`; when that
read fails, ask the person for the merge target, never guess it),
comparison base (default: the commit where the branch parted from the merge target,
`git merge-base origin/<merge target> HEAD`), profiles (default: chosen from changed paths by the
review skill's **Profiles from paths**), optional seats (the person's word for this run; otherwise
as the review skill's **Optional seats** says), review items (what the person or the caller asks
every review to look at, beyond the profiles).

The first consistency phase run (step 2) and the first full review (step 3) read from the
comparison base, and the line (`references/merge.md`) always from the commit where the branch
parted from the merge target; the pull request points at the merge target. The later diff reviews
and the phase run after the fixes keep the ranges **Loop** and **Consistency phase** give them.

Cycle runs only what the caller's one-line reason named; delegating more is a counter-example.
Whether a review launches a reviewer is the review skill's gate, and a second full review happens
only under step 5's condition. Optional seats are the person's standing choice, not an addition,
and the consistency phase is decided by cycle's own measure (**Consistency phase**), not by the
reason.

Where to start is in **Resuming and running more**.

Before the first review, read the kotowari-review skill's `SKILL.md` and the references its
**Reviewer setup** names; read the reference its **Optional seats** names only when this run has
optional seats. **Reviewer setup** owns the self-contained reviewer prompt; use that contract for
both full and diff reviews.

## Loop

1. Delegate the place of the decision documents, the decisions cycle received, the overturned row
   IDs and the prototype path when the brainstorm gave them, the branch, and the worktree path to
   an implement agent, all remaining work in one delegation.
2. Consistency phase (**Consistency phase**).
3. Full review: base..head, profiles, strength, counterpart, plus the **known findings** (open
   `record_only` / `human_judgment`, closed `accepted`), never visible or fixed ones; a match is not raised again.
   Then launch the optional seats yourself, as the review skill's **Optional seats** says.
4. Diff loop: delegate the **visible findings** to a fixer; then diff review (changes since the
   last review, the open findings with IDs, profiles, strength, counterpart). Repeat until
   no visible finding remains.
5. A second full review only when a fix could spread beyond where it was made; name that reason
   before running it; it gets the optional seats as in step 3. Its visible findings → one more
   diff loop until none remain.
   With no such reason, the diff loop clearing every visible finding ends the quality review.
6. Consistency phase once more (**Consistency phase**); then converged.
7. Merge, as `references/merge.md` says.

Visible findings = open findings whose final action is `auto_fix` or `fix_and_verify`. Findings with
`human_judgment` or `record_only` stay open for the terminal report. A `consistency` finding goes
only to the consistency phase, never to a fixer or a review. When
it runs, the second full review cancels the taint a diff review carries from seeing prior findings.
A **round trip** is one review invocation (any number of reviewers, full or diff, the first one
included) or one consistency phase run. The limit, when the person set one, counts round trips.

## Delegations

Every prompt is self-contained; never assume a delegate loaded a skill or read the conversation.

- **implement:** carry what step 1 names. It returns commits, the state of each decision with the
  evidence per row, changes beyond the decisions with their reasons, the lines of each detail or
  reading it decided by itself, and each sentence of a decision document it corrected to the facts;
  or a hand-back with the open item it wrote.
- **review (full):** carry the base and head, worktree path, known findings, the review items from
  **Inputs** if any, and the reviewer prompt of the review skill's **Reviewer setup** (it carries the Evidence
  conditions). It returns findings JSON.
- **review (diff):** carry the diff since the last review, worktree path, open findings with IDs,
  and the same review items and reviewer prompt. It returns per-finding `still_present` or
  `no_longer_visible` and new findings.
- **consistency phase:** use **Consistency phase** below.
- **fixer:** carry the visible findings, the place of the decision documents, branch, worktree path, the path of the kotowari
  skill's `references/mark.md`, and the contract below; for a finding that is `still_present`
  after a fix, also that fix's commits.

Paste `references/editing-contract.md` as written: its **Editing contract** to the fixer and the
consistency phase, its **Fixer only** to the fixer alone. Immediately below the contract, paste
the **Conditions** section of the kotowari-review skill's `references/oracle-evidence.md`. Keep no
copy of either in this file.

When implement or the fixer returns a commit a hook stopped on a finding in a decision document,
run the consistency phase as in step 2, the blocked change in the worktree included in its range,
then repeat the same delegation; after implement, step 2 still runs.

A review's counterpart is the place of the decision documents, read as the review skill's
**Inputs** reads a specification. Only the consistency phase gets the decisions cycle received:
write them to a file, one `path#anchor` per line, and pass that file. For a small task without a
specification, pass the request and no file.

## Consistency phase

The consistency phase owns agreement between code and the decision documents, separately from
quality review. Its agent instructions and input contract live in `references/consistency-phase.md`.
It writes no record per run: the reason for each thing it fixed comes back as a line, which goes
into the pull request body.

- **When.** Once after implementation (step 2), over base..head, and once after the quality
  review's fixes converge (step 6), over the diff of all those fixes: from the head step 3 reviewed
  (`first_review_head`) to the current head. One delegation is one run. After a run that returned a
  new finding or a visible one `still_present`, cycle delegates a rerun with the same base and the head moved to
  the current head; the step ends at a run that returns neither.
- **Whether.** Cycle decides. Skip a run only when its diff neither adds nor changes behavior a user
  can observe: documents such as guides only, tests only, or a refactoring that keeps behavior.
  Skill text is behavior, never skipped as a document. When in doubt, run it. A skipped run writes a
  one-line reason in the terminal report.
- **Who.** A delegate in a context separate from the implementer, never the implement agent or the
  fixer. A model different from the implementer's is preferred, not required.
- **What it carries.** Paste the reference in full and supply its **Inputs**, using the place of
  the decision documents and the file of decisions prepared above, and the contract as
  **Delegations** says.
- **What cycle does with the result.** The phase fixes and commits by itself; cycle only records the
  run in the findings file under **Judgment stays here**, keeps the reason lines it returned for the
  pull request body (**Terminal report**), and applies the same **Endings**.
  Cycle never fixes the phase's findings.

## Judgment stays here

Cycle alone writes the findings file (shape: the review skill's `finding-schema.md`, with `ids`,
`merge_target`, `base`, `last_reviewed_head`, `first_review_head`, `premise_attempts`, `fixes`,
`line`, `fix_of` and `ending`), creating it at its start as **Inputs** says. It writes no other
record file and records no cost. It sets `line` each time it judges the line
(`references/merge.md`), with the paths that were hit; `fix_of` to the branch of the earlier request this request fixes, only when
the request says it is a fix of an earlier one, else null; and `ending` when the run ends, with the
ending reached and its reason, a hand-back and a stop for the person included. Each
time step 3 runs it sets `first_review_head` to the head it reviews; that head is the only state
kept for the phase. After every review or phase run
it overwrites the file: sets `last_reviewed_head` after a review, assigns
IDs to new findings, merges reviewers and groups same-cause findings, finalizes each proposed action
before classifying visible findings. For each non-`security` finding, in order: force `info` to
`record_only`; force a claim stating no defect to `warn` and `record_only`; for a defect demanding a
new test or fixture without showing it qualifies, keep its action but replace its oracle with all
existing checks passing; replace a `warn` oracle even when it qualifies; a replaced oracle keeps the
`measured` the reviewer reported, and its note says that value was measured on the proposal that
was replaced, so a `fails_now` kept this way counts as shown by running; force `human_judgment`
proposed for readability, a choice of structure, a deleted test, an added suppression of a check,
or an added or changed dependency to `record_only`, since the implementer decides those; force an
`auto_fix` or `fix_and_verify` whose `oracle.measured` is not `fails_now` to `record_only`, except a
proposal to delete verification the diff added because it does not meet the Evidence conditions
(the review skill's **How a reviewer works**), which stays a fix without having been seen failing.

Three kinds of review finding keep a run from converging: a mistake that running something shows,
a `security` finding whose proposed action is a fix, and such a proposal to delete verification;
the stops of `references/merge.md` are apart from this. A gap from the intent that no run can show
is `record_only`: a detail the decision documents leave to the implementer, reported in the
terminal report. When it shows two sentences of the decision documents that cannot both hold, it is
a collision of intent: hand it to the next consistency phase run, which writes it under `## 未決`
as its reference says. A `security` finding keeps the action it proposes and
never pauses the run: one proposing `auto_fix` or `fix_and_verify` is visible and goes to the fixer
whether or not its oracle was seen failing, and when it cannot be fixed ending 3 hands it to the
person; one proposing `human_judgment` stays open, which `references/merge.md` counts as a reason
for a person to confirm; one proposing `record_only` stays recorded. For each `security` finding a
fix closed, write a decision line: what was fixed, why (its claim), and the evidence (its commits).

Adopt any proposal that matched none of these checks unchanged. Never move a finding out
of `human_judgment` without asking the person. Append numbered verdicts and update state; after every
fix record its reported commits on each finding and append to `fixes` the findings it addressed and
its commits; the round of the review after it goes into that entry's `reviewed_in`. `fixes` is a
record of facts for resuming and for the terminal report.
`no_longer_visible` → closed (`fixed`); accepted by the person at the end → closed (`accepted`). If
reviewers disagree, one `still_present` means still present. New findings carry no IDs, whether a
full review, a diff review, or a phase run raised them: match each by evidence location and
oracle — a match with an open finding reuses its ID and appends `still_present`; a match with a
closed finding is "same cause returned" under **Endings** and reopens it unless it was closed
`accepted`. Reviewers only evaluate; the fixer only reports commits; the consistency phase does
both for its own findings. Record each premise attempt as `references/premise-step.md` says.

**Overlap after a fix** is cycle's judgment, read from the evidence locations (file and lines):
did the review right after a fix raise a new visible finding next to where that fix was made — the
evidence of the findings it addressed and the places its commits changed? Evidence naming the same
file with overlapping line ranges is one example; judge what the locations show rather than
computing it from the findings file.

## Stopping inside the loop

- A delegate stops on an irreversible operation, a privileged operation, a dangerous target, or a
  spreading accident: relay the question verbatim, then resume or re-delegate with the answer.
- A delegate hands back a collision of intent — two sentences of the decision documents that
  cannot both hold — with the item it wrote under `## 未決`: ending 4. A detail the documents do not
  write is never a hand-back: the delegate decides it with grounds.
- Implement finds no test command in the project's instructions or the language's standard tool,
  or returns that a check command cannot succeed as written: ask the person, then re-delegate with
  the answer.

Not a stop: a `security` finding (**Judgment stays here**), and a change that deletes a test, adds
a suppression of a check, or adds or changes a dependency. Readability and choices of structure are the implementer's to decide; they neither stop
the loop nor add a review seat.

## Endings

1. Converged: the last review returned no visible finding, or the diff loop after it cleared them,
   and the consistency phase after it ended its step or was skipped.
2. The person's round-trip limit was reached.
3. No progress, when one of these holds:
   - a visible finding is `still_present` in two consecutive rounds that evaluated it (the second
     after a changed approach);
   - a closed finding's cause returns;
   - two consecutive post-fix diff reviews have at least as many finalized new visible findings as
     visible findings marked `no_longer_visible` (full reviews are excluded from this comparison);
   - for two fixes in a row, the review right after the fix raised a new visible finding next to
     where the fix was made (**Overlap after a fix**);
   - a review still cannot succeed after one re-delegation.

   Every clause but the last first enters the premise step (`references/premise-step.md`), and the
   loop ends here only when that step says so. A review that cannot succeed ends the loop at once.
4. A delegate handed back a collision of intent, which goes to the person through brainstorm.

Only ending 1 goes on to step 7. Endings 2–4 ask for no merge; they add to the terminal report the
choice "run more or accept the rest and finish" and any hand-back reason.

## Resuming and running more

A branch name is not used again after its request ended. Before anything else at a start or a
resume, read the branch's pull requests from the hosting service
(`gh pr list --head <branch> --state all --json number,state,headRefOid`). When a start that
received a new request finds one merged or closed, stop and ask the person for a new branch name.
When one is open and its head is not on the local branch
(`git merge-base --is-ancestor <headRefOid> HEAD` fails), stop and tell the person. When the read
fails (on a host the `gh` commands do not reach, for one), go on without these two checks and say in
the terminal report that they could not be made. A failed read of a request to merge, below, is
handled the same way: the commits stay local until the push of step 2 of **Opening and asking**,
before which **Withdrawing the request** in `references/merge.md` reads it again, makes the two
checks the start could not make, and does not push while the request cannot be read.

The findings file is `.agents/artifacts/reviews/<branch>.json` (a `/` in the branch name is a
directory) in the main working tree (the first entry of `git worktree list`), never in the run's
worktree, so that **Cleanup** in `references/merge.md` does not remove it. It is never committed and
never deleted by the workflow. A file whose `ending` is `merged` belongs to an earlier request on a
branch of the same name and is not a resume: rename it beside itself to
`<branch>.merged-<n>.json`, with the lowest free `n`, and start a new file. A resume is a start
that brings no request: cycle was asked only to go on. A start that brings a request (the place of
the decision documents with its decisions, or a kotowari-iterate request) to a branch whose findings
file is already there is a later request on the same branch, not a resume: it makes the
confirmation of **Inputs**, adds its decisions to `ids`, and sets `first_review_head` to null. If another file exists
when cycle starts, a resume and a later request alike keep its findings, continue round
numbers from its maximum, and read its `premise_attempts` as `references/premise-step.md` says, so
that a run that already made its attempt does not make another. Ending 3's streaks (`still_present` twice running; new visible findings
not shrinking; overlap after a fix) count from this start only; a closed cause returning counts
across starts. "Run more" after endings 2–4 continues the same run: streaks kept, findings still open, a
new limit only if the person sets one.

Either way, go on at the first row that holds. Cycle creates no new file for resuming. Before a
row that adds a commit to the branch, read from the hosting service whether the branch's pull
request still carries a request to merge an earlier session made, and withdraw it, as
**Withdrawing the request** in `references/merge.md` says.

| Situation | Go on at |
|---|---|
| no findings file yet | step 1 (implement decides what remains from `git log` and the decisions still proposed), then step 2 |
| a later request on the same branch | step 1 as above, with all of `ids`, then step 2 and on: the phase run and the full review read from the comparison base again |
| "run more" after an ending a phase rerun raised | that step: 2 (then on to 3) or 6 |
| "run more" otherwise | step 4 |
| resume, `first_review_head` null | step 1 as above (implement returns at once when nothing remains), then step 2 |
| resume, a visible quality finding open | step 4 |
| resume, the branch has a pull request | **Resuming** in `references/merge.md` |
| resume otherwise | step 6 |

Whether to skip a phase run is decided again by its measure.

## kotowari check before the terminal report

After the run converges and before step 7, run `kotowari check` and
`kotowari status --format text`, and put both outputs in the terminal report. Read them as the kotowari skill's scene check
(`references/findings.md`) says. When the last line of the status is `complete false`, the reason
is in the check's findings if `findings` shows a nonzero `error`, and otherwise in the items under
`## 未決` of the topic documents. The `decisions` line counts the proposed and implemented
decisions; a decision cycle received that is still proposed after convergence is named in the
terminal report with the rows that are not met.

Only this run's findings are worked on. A finding is this run's when its `path` is a file the
branch changed (`git diff --name-only <base>..HEAD`) or it names a row of a decision cycle received; narrow the
JSON with `jq` rather than reading it whole. Every other finding was there before the run: count
it in the terminal report by kind, and never delegate it — fixing what the run did not touch
spends a loop on work nobody asked for. The same holds for a `complete false` whose reason is
only such findings or open items that were already committed.

- This run's test-side findings (defined in the kotowari skill's `references/findings.md`) are
  findings for the fixer: make them visible and run the diff loop. If they do not go away, end as
  ending 3 (no progress).
- A proposed decision is not among what the run brings to zero unless cycle received it: check
  raises no test-side finding for its rows, and a marker pointing at one raises nothing.
- Overview data (the files in the configuration's `overview.files`) is not changed by the run, and
  its findings and guide_stale notices are never delegated. Count them in the terminal report.
- Findings in decision documents (a `path` under the place of the decision documents) among this
  run's findings are the consistency phase's. Every phase run starts by taking them from
  `kotowari check` and ends with none left in its range, so one left here means a run was skipped
  or its step did not end: run the phase over the branch's diff. Cycle never edits the decision
  documents itself.
- Missed mutations: the project's mutation gate (in kotowari itself, the pull request CI) runs mutations on the diff. A miss is fixed by the fixer or
  the implementer, like a test-side finding; how to investigate one is in the kotowari skill's
  `references/mutants.md`.

## Terminal report

Always: artifacts and commits, verification results from the implement report, the
`kotowari check` and `kotowari status` output, how to view the diff, and the decision lines, the
same lines as the pull request body. The decision lines are, one line each of what was decided, why,
and the evidence: each detail or reading implement or the fixer decided by itself, a dependency it
added included; each `security` finding a fix closed; each sentence of a decision document
implement or the consistency phase corrected to the facts, with the old and new sentence and the
fact; and each fix the consistency phase made, with the reason it returned. When present: fixed
findings, forwarded observations, changes beyond the decisions with their reasons, open findings
needing the person, the decisions cycle received that are still proposed, details a review found
left to the implementer (`record_only`), and the checks at the start that could not be made
(**Resuming and running more**).
From the consistency phase: its commits, each open `human_judgment` finding with the item it wrote
under `## 未決`, and the one-line reason for each skipped run.
When a premise attempt ran or a premise went to the person: the premise ladder of each attempt in
the findings file. When a full review ran optional seats: their attendance, as the review skill's
`references/optional-seats.md` says under **Merging and reporting**.
After step 7: where the pull request is and whether the merge was asked for; for a stop, its cause
as `references/merge.md` shows it, with no summary of the change; when merging by itself is not
available, that and why.
Cycle never performs a merge (the hosting service merges what step 7 asks for), publishes, changes
a repository setting, deletes anything on the hosting service, removes a local branch or worktree
except as **Cleanup** in `references/merge.md` says, edits the decision documents itself (the
implementer turns states and corrects sentences to the facts, and the consistency phase fixes them
as its reference says), manages issues, or runs two requests at once.
