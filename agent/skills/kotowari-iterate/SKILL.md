---
name: kotowari-iterate
description: "Entry point beside the kotowari workflow for a task too small to need a specification or a plan: judge whether the request is small and run the cycle loop on it; anything bigger is turned away with the next skill to call. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`). Use when asked to iterate, for one more fix, to fix this bit, to add this too, or to polish it a little more. 日本語キーワード: iterate ちょっと直して これも足して もう少し磨いて 小さいタスク"
---

# Iterate

Run cycle's loop on a **request** instead of a plan, once judged small. Delegate only what the
caller's reason named: review always runs in separate contexts, judgment and implementation only
when named. A person starts this skill from the conversation; cycle never calls it. Not for "look
into this" (kotowari-investigate) or "check that it works" and "verify this" (review's diagnosis):
implementing from those is a boundary breach. "Fix it" or "add it" alone never starts this skill;
ask for a reviewed loop.

Required: the request, and the branch with its worktree path. Missing either, stop: the main
session prepares them beforehand — never create a branch or a worktree here. The branch name is
short, for the request (no fixed prefix). Optional: cycle's optional inputs plus the specification
path to match against; defaults are cycle's.

A **request** is the person's words completed by what the main session knows — named files,
settled direction, a preceding run's terminal report — into a self-contained text an implementer
with no context can build from. "Fix that thing from before" arrives expanded into the file and the
change; if one or two exchanges cannot, this skill does not run.

Out: commits on the branch and a terminal report. Stopped at the judgment: no commit, guidance
only. Stopped by a hand-back during implementation: the commits so far stay (never deleted or
rewound), and the terminal report carries artifacts, commits, and diff view plus the guidance.

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
shape (the files and their changes, plus a verdict with grounds per condition); and, in full, the
restrictions kotowari-investigate's **Delegate exploration** puts into a subagent prompt (the judge
writes no file and never delegates further). Take `git status`
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

Run the kotowari-cycle skill as written, its consistency phase and reviews included, "cycle"
there meaning this run, with only these differences:

| In cycle | Here |
|---|---|
| the plan path; the branch name containing the plan name | the request; a short name for the request |
| the specification path read from the plan | the given path, or the specification the judgment found; requirement IDs only when the request touches IR items |
| step 1's delegation of the remaining plan steps to implement | after the judgment, the request goes to the implementer below in one delegation |
| the plan path in the fixer delegation | the request, and the specification path if any; the fixer also hands back a contradiction with the specification |
| **Fixer only**'s "the plan's commands in order, unedited" (kotowari-cycle's `references/editing-contract.md`) | check commands come from the project's instructions, then the ecosystem's standard tool |
| the row "a plan step left no git trace" in **Resuming and running more** | does not apply: there are no plan steps |
| the specification path and the file of IDs in the consistency phase delegation | the specification path and the request; the file of IDs only when the request touches IR items; with no specification, the request alone is the counterpart |
| ending 4 and its "run more or accept the rest" choice | the destination is one of the guidance table's three; no choice is offered — the report adds the hand-back reason and the guidance, and the person restarts with a new request holding their answer |
| "names an ID the plan covers" in **kotowari check before the terminal report** | names an ID the request's enumeration covers; with none, only the files the branch changed count |
| any other word meaning the plan (out-of-plan changes, one plan at once) | the request; sentences about plan steps do not apply. Plan as a skill name or a destination stays |

The **implementer** receives what the fixer receives — kotowari-cycle's
`references/editing-contract.md` whole, with the Evidence conditions — the request replacing the
visible findings, with the judgment's enumeration and the specification path if any; the
implement skill is not used. It also hands back a file outside the enumeration, a contradiction
with the specification, or a request that reads two ways. Where the reason did not name delegated
implementation, this session implements under that same contract, the rest of the loop
unchanged. It returns commits, evidence per completion kind, out-of-request changes with reasons —
or a hand-back and why. The enumeration goes along as reading material, marked as not an order —
handing it as steps is a counter-example — yet it caps the files to touch (edit, create, delete,
rename; tests included): one outside it — hand back, never touch or report it. In a listed file,
changes the request did not name (import tidying, tests following) are out-of-request; an entry
whose change differs is review's to catch. The cap binds only the implementer; the fixer follows
findings into any file, reporting those outside as out-of-request.

Same branch right after a cycle or a run of this skill: reuse it, never re-cut. A findings file
still there is cycle's resume — keep it; deleting or ignoring it is a counter-example — with these
differences: set `first_review_head` to null; unless the person gave a comparison base, it is the
branch tip at start, and inherited open findings are evaluated in the diff review even outside it;
ending 3's streaks reset at each start of this skill, inherited evaluations uncounted. Ending 3's
clauses enter the premise step as in cycle (kotowari-cycle's `references/premise-step.md`), whose
allowance of one attempt is renewed at each start of this skill too. No default
round-trip limit; one the person sets counts this run's reviews from when it was set, not the
round numbers. No counter of consecutive runs.

The terminal report takes its verification results from the implementer's evidence, and adds the
guidance when not small or handed back.
