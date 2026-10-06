---
name: kotowari-plan
description: "Workflow station of the kotowari workflow: turn an approved specification into one Markdown plan that an implementer with no prior context can execute, referencing its requirements instead of copying them, with per-step completion evidence and stop conditions. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`). Use when asked to write or revise a kotowari plan from a specification. 日本語キーワード: 実装計画 手順書 計画を立てる 仕様から計画"
---

# Plan

Write the implementation plan for one approved specification. The reader is an LLM that knows
nothing of this conversation; it gets the plan path, the repository, and nothing else. Anything
the reader cannot recover from those must be in the plan.

## Inputs and outputs

In: the path of the IR store and the IDs of the requirements this plan covers. The specification
is approved only when the IR documents, the glossary, the problem record, and the decision
record are all committed and `kotowari check` reports no errors other than test-side findings
(requirement_without_test, scenario_without_test, and findings on test files); otherwise it is
unapproved — stop and say so. The plan itself runs neither `kotowari check` nor
`kotowari status`: the main session runs the check that judges approval. The one kotowari
command that looks at the plan file is `kotowari plan`, which checks only its form (Finishing
1). A topic with no IR takes the path of its committed specification instead (such as a
decision record whose decisions are the specification); uncommitted means unapproved.
Out: one Markdown file, `docs/plans/<name>.md`, approved by the person and committed. The
implementation branch will carry `<name>`; choose a name that reads well in a branch.

## What a plan is

Written in plain language, one file, no machine-oriented sections. It **references** each
requirement as `<document path>#REQ-nnn` — a topic with no IR, by path and section heading — and
never copies specification text: copies drift, and the implementer must read the requirements
anyway. What the plan adds is what only this plan
knows — why this order, why these files, where to stop.

Plan-level content: which requirements each step verifies; approach and its
rationale; the file scope that may change; step order and prerequisites; choices left to the
implementer; stop conditions.

Per-step content (see `references/step-template.md`): purpose and the requirements it
rests on; prerequisites; the files it may change; what "done" means and how it is shown (test /
check / artifact / external); choices left open; when to stop and hand back.

The form is fixed so that `kotowari plan` can check it against the schema bundled in kotowari
(`references/step-template.md` states the rules; `references/plan-example.md` is a whole plan
that passes). The plan-level content goes in its fixed `## ` sections, and the steps go under
`## Steps` as `### S<number>: <name>` headings, each followed only by its eight field lines
`- Name: value` in the template's order. Write each field's value on that one line. Put no
frontmatter at the top of the plan: it names no schema, and the check uses its own.

## Reading the requirements

Read a referenced requirement with `kotowari query REQ-nnn`, not by opening the document and
searching. In the one item it returns, `path` and `line` locate it, `body` is its text, `tests`
lists the tests already marked for it, and the `referenced_by` entries whose `via` is `about` are
its scenarios. An unknown ID stops with `argument error: unknown id:`. To read the output of
`kotowari list`, `query`, or `status`, read the kotowari skill's scene check
(`references/findings.md`).

The requirements and scenarios among `kotowari list`'s `items` whose `tests` is empty are what
this plan's cycle fills; build the Verification map from them. Leave out every item whose
`deferred` is true: a deferred requirement, and a scenario whose `@about` requirements are all
deferred or review with at least one deferred, is specified but not built now. Do not put a deferred requirement's ID in the plan —
not in a step, not in the Verification map — and do not plan a test for it. If building the
plan's requirements needs a deferred one, that is a specification question: hand it back to
brainstorm.

Do not read `kotowari list` whole: in a repository of 150 requirements its JSON exceeds 200KB and
its text form 1,000 lines. Take only the keys you need with `jq`. For example:

- IDs and locations of the requirements and scenarios with no marked test (a review requirement
  and a deferred item need no test, so they are left out):
  `kotowari list | jq -r '.items[] | select(.tests == [] and .verification != "review" and (.deferred | not)) | "\(.id) \(.path):\(.line)"'`
- one item's text only: `kotowari query REQ-001 | jq -r '.items[0].body[]'`
- its reverse references only: `kotowari query REQ-001 | jq -c '.items[0].referenced_by'`

`kotowari status` is a few hundred bytes; read it whole.

The last step's check commands confirm the plan's own work, not the whole repository: every
requirement and scenario the plan covers has a marked test (`kotowari query ID` shows a
nonempty `tests`; a requirement verified by review needs none), and `kotowari check` reports no
error on a file the branch changed or on an ID the plan covers. Do not require `kotowari status`
to be `complete true` or `kotowari check` to exit 0: both also count findings and problem records
that were there before the plan, and a committed problem record can keep them false forever.

## Boundaries

- A choice may be left to the implementer only if every option leaves the approved behavior
  unchanged. New input kinds, acceptance boundaries, and error handling need grounded authorization: a concrete addition within explicitly delegated scope may be recorded with its grounds and deciding role while keeping approved requirements unchanged; otherwise hand back to brainstorm.
  "The specification does not say" never means "the implementer decides".
- A step that changes only what the IR does not hold (CI, hooks, release, build configuration;
  the kotowari skill's **What the IR holds**) names its decisions instead of requirements, is
  shown by check or external, and adds no requirement and no test.
- A human check inside a step is written as an ordinary sentence in that step, and only for an
  irreversible operation, a privileged operation, or a dangerous target. Meaning-changing
  decisions go back to brainstorm; acceptance of the result belongs to the end of the cycle.
- When neither the project's instructions nor the ecosystem's standard tool fixes a test
  command, decide it here — the implementer must not invent one.

## Finishing

1. Form check, then self-check. Run `kotowari plan <plan path>` and read its exit code as the
   kotowari skill's scene check (`references/findings.md`) says. On 1, fix the plan where each
   invalid_plan finding points and run it again until it exits with 0; fixing the form is not a
   decision, so do not ask the person. On 2, show the person the stop reason from the first line
   of standard error and stop. Finishing is the only place in the workflow that checks the form,
   so the check must follow the plan's last edit; nothing checks it again after approval. Then
   self-check against `references/step-template.md`
   for what the form check cannot see: every field of every step has a non-empty value; every
   referenced requirement ID exists (`kotowari query` returns it; with no IR, every referenced
   heading exists in the specification); no step decides a specification question.
2. Adversarial review, only when the plan's own decisions can contradict each other — steps that
   depend on one another, or one requirement driving several steps. Say that reason,
   then launch one separate-context agent on the plan's own quality, and a second against the
   specification only when the match is one no check can make. A plan whose steps stand alone
   gets none. Findings that need no decision are fixed
   directly. Findings that need a decision: under the four stop conditions (missing meaning or
   departure from approved content; irreversible, privileged, or dangerous operation; spreading
   accident; no progress after a changed approach) stop and ask the person now; otherwise decide
   yourself and list the decision among step 3's judgment points. If this step changed the plan,
   run the form check again as in step 1, with the same handling of exit codes 1 and 2.
3. Approval, only after the form check has exited with 0 on the plan's last edit: stage only the
   plan, give the person the path, the command to view the diff, and
   the points needing their judgment. Do not paste the plan, and never let a summary be what
   they approve. The person commits, or tells you
   to. A plan is approved only once committed.

Finished plans are deleted by the main session after the person accepts the result and
merges the branch; the plan stays readable in git history. Do not delete it yourself.
