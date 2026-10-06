---
name: kotowari-iterate
description: "Entry point beside the kotowari workflow for a task too small to need a specification or a plan: this session judges whether the request is a small task, delegating a read-only judge only when it cannot close the impact enumeration, then the cycle loop runs implementation, review, and fixing on it, adding only the stations the caller's one-line reason named; anything bigger is turned away with the next skill to call. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`). Use when asked to iterate, for one more fix, to fix this bit, to add this too, or to polish it a little more. 日本語キーワード: iterate ちょっと直して これも足して もう少し磨いて 小さいタスク"
---

# Iterate

Run cycle's loop on a **request** instead of a plan, once judged small. Delegate only what the
caller's reason named: review always in separate contexts, judgment and implementation only when
named. A person starts this skill from the conversation; cycle never calls it and hands nothing over
— what the main session learned there goes into the request; a findings file on the branch is
inherited state (see the loop). Required: the request, and the branch with its worktree path;
missing either, stop: the main session prepares them beforehand — never create a branch or a
worktree here. The branch name is short, for the request (no fixed prefix). Optional: cycle's four
optional inputs plus the specification path to match against; defaults are cycle's. A **request** is
the person's words completed by what the main session knows — named files, settled direction, a
preceding run's terminal report — into a self-contained text an implementer with no context can
build from. "Fix that thing from before" arrives expanded into the file and the change; if one or
two exchanges cannot, this skill does not run. Out: commits on the branch and a terminal report.
Stopped at the judgment: no commit, guidance only. Stopped by a hand-back during implementation: the
commits so far stay (never deleted or rewound), and the terminal report carries artifacts, commits,
and diff view plus the guidance.

## Small task and its judgment

A **small task** meets all four; file count is irrelevant.

1. One reading: the implementer writes the diff without choosing one.
2. No specification decision: no new input kind, acceptance boundary, or error handling is
   decided for the first time by this task. A change only to what the IR does not hold (CI, hooks,
   release, build configuration; the kotowari skill's **What the IR holds**) makes no
   specification decision.
3. If a specification exists, no contradiction with it.
4. The impact is readable: the judgment enumerates every file to change (files to be created included;
   for code, the tests the implementer adds test-first are among them) in a closed list — no "there
   may be others"; never a count — and says what changes in each without judgment (attached per
   entry; closure is judged per file); for an interface change, every caller is enumerated and its
   change said without judgment the same way. Not readable means large.

Undecidable on 1 or 4: not small. Thirty files in one spelling fix: small. A rename whose callers
all enumerate and change mechanically: small; four enumerated but one needing a choice on its return
value: not readable. "Make the error message clearer": one reading only with wording given.

The four conditions are judged here by default: this session already holds the context a judge
would re-read, so it enumerates the impact itself and records the verdict in one line. Delegate a
read-only **judge** (not an investigation, which starts from a symptom) only when the enumeration
cannot be closed — condition 4 in doubt — and say so; a judge over a closed enumeration is a
counter-example. When delegated, the prompt is self-contained: the request; the worktree
path; the specification path if given, else the duty to search the specification home the project's
instructions name and report one covering the files to change; the four conditions verbatim; the return
shape (the files and their changes, plus a verdict with grounds per condition); and, in full, these
restrictions: no editing, creating, overwriting, deleting, moving, or renaming any file, notebooks
included; allowed — the only commands run — are reading files, listing paths, searching, read-only
commands (a test only when known to update nothing, in the repo or outside), and following references;
forbidden, as examples (refuse anything else that changes state): `rm` `rmdir` `mv` `cp` `chmod` `chown`
`touch` `mkdir` `tee`, output redirection, in-place rewriting, state-changing git; secrets reported as
existing, never by value; the judge writes no file and never delegates further. Take `git status`
yourself before and after; a difference is a spreading accident: stop, show the person, ask; never
revert it. A found specification counts as given — for condition 3, review, and the guidance table's "if
a specification exists"; none found, go on without; passing condition 3 unsearched is a counter-example.
The verdict is a proposal; the decision is here, as with reviewers in cycle; a test file the implementer
needs test-first but the enumeration lacks is added here — that gap alone does not fail condition 4. No
verdict: re-delegate once, then report none was possible and stop.

Not small: stop with guidance (the failed condition with grounds, the destination, a ready-to-use
invocation); never start brainstorm or plan yourself. Several failed: list all, destination from
the highest row, since a clarified request may change verdicts 2 to 4. Guiding to plan with no
specification (it cannot run without one) or offering "continue here anyway" is a counter-example.

| Failed | Also handed back for | Destination | Ready-to-use form |
|---|---|---|---|
| 1 (ambiguous) | the request read two ways (implementer) | the person, via the main session | where the readings diverge, and the question to ask |
| 2 (specification decision), 3 (contradiction) | a missing design decision (2), a contradiction with the specification (3) — implementer or fixer | brainstorm | `/kotowari-brainstorm <topic>`, with the existing specification path if any |
| 4 (impact unreadable) | a file outside the enumeration (implementer only) | plan if a specification exists, else brainstorm | `/kotowari-plan <IR store path> <requirement IDs>` or `/kotowari-brainstorm <topic>` |

## The loop

Read the kotowari-cycle skill body and run all of it as written, its Inputs included (the review skill
read before the first review) and its consistency phase included; "cycle" there means this run. Only these substitutions apply:

| In cycle's body | Read here as |
|---|---|
| the required plan path | the request |
| the branch name contains the plan name | a short name for the request |
| the specification path read from the plan (the IR store path and the requirement IDs) | the given path, or the specification the judgment found; requirement IDs only when the request touches IR items |
| inferring done steps from the plan and `git log`, then delegating the rest to implement | no inference: the request goes to the implementer in one delegation, after the judgment |
| the implement delegation (plan path, branch, worktree path) | the implementer delegation (request, the judgment's enumeration, the specification path if any, branch, worktree path), with hand-back reasons added to the contract: a file outside the enumeration; a contradiction with the specification; or a request that reads two ways |
| the plan path in the fixer delegation | the request, and the specification path if any, with a hand-back reason added to the contract: a contradiction with the specification |
| the fixer contract's "the plan's commands in order, unedited" | check commands come from the project's instructions, then the ecosystem's standard tool |
| "run more" re-entering at step 1 when steps remain | at step 2 or step 6 when the ending was raised there, as in cycle; else the diff loop |
| the specification path and the file of IDs in the consistency phase delegation | the specification path and the request, both; the file of IDs only when the request touches IR items; neither path nor file when there is no specification, and the request is the consistency phase's counterpart |
| ending 4 (a hand-back to brainstorm or plan) and its "run more or accept the rest" choice | the destination is one of the guidance table's three; the choice is not offered — the report (as in Out above) adds the hand-back reason and the guidance, and the person restarts with a new request holding their answer |
| "names an ID the plan covers" in **kotowari check before the terminal report** | names an ID the request's enumeration covers; with none, only the files the branch changed count |
| any other plan word meaning the plan (one plan at once, out-of-plan changes) | the request (out-of-request changes); plan as a skill name, a destination, stays; sentences about plan steps (do not interpret its steps, if steps remain) do not apply — there is no plan |

The **implementer** is cycle's fixer contract pasted in full, the request replacing the visible
findings; the implement skill is not used. Where the reason did not name delegated implementation,
this session implements under that same contract, the rest of the loop unchanged. It returns
commits, evidence per completion kind, out-of-request changes with reasons — or a hand-back and why.
The enumeration goes along as reading material, marked as not an order — handing it as steps is a
counter-example — yet it caps the files to touch (edit, create, delete, rename; tests included): one
outside it — hand back, never touch or report it. In a listed file, changes the request did not name
(import tidying, tests following) are out-of-request; an entry whose change differs is review's to
catch. The cap binds only the implementer; the fixer follows findings into any file, reporting those
outside as out-of-request. With no specification the request is the counterpart; review launches the
quality reviewer only, and the consistency phase reads the code against the request.

Same branch right after a cycle or a run of this skill: reuse it, never re-cut; findings JSON still
there is cycle's resume — keep the findings (deleting or ignoring it is a counter-example) and
continue rounds from the inherited max (the first round is max+1), and set `first_review_head` to
null. Unless the person gave a
comparison base, it is the branch tip at start (already checked); inherited open findings are
evaluated in the diff review even outside it. Both of ending 3's streaks (`still_present` two rounds
running; new visible findings not shrinking) reset at a start of this skill (inherited evaluations
uncounted), not at "run more" after ending 2 or 3, as in cycle; a closed cause returning counts
across it. No default round-trip limit; one the person set counts this run's reviews from when set,
not the round numbers, and cycle's ending 2 applies. No consecutive-run counter.

Cycle's terminal report and "never" list apply, verification results from the implementer's evidence,
plus the guidance when not small or handed back. "Look into this" belongs to the investigate skill,
"check that it works" and "verify this" to review's diagnosis; implementing from them is a boundary
breach. "Fix it" or "add it" alone never starts this skill; ask for a reviewed loop.

