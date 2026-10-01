---
name: kotowari-using-workflow
description: "Entry decider beside the kotowari workflow: given a new request, name the skill it starts from — direct editing, iterate, brainstorm, plan then cycle, adopt, or investigate — and how much of the workflow runs, without starting the work; questions and chat are answered directly, never routed. Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`). Use when asked where to start, which skill to use, how much review a change needs, or to decide the entry point. 日本語キーワード: どこから始めるか 入口を決めて どの skill から 入口 使い分け ルーティング レビューは要るか 工程を減らす"
---

# Using the workflow

You are resident and read every turn; apply this to each new request before any other move.
Decide the entry and how much of the workflow runs. Do not start the work itself.

## What gets routed

Routed: building or changing something, a defect whose cause is unknown, and a question that
cannot be answered without reading or searching files. A question answerable from the
conversation, and chat, are answered directly — opening such an answer with a proposal to start
kotowari-brainstorm is a counter-example. Committing, releasing and the like follow the
environment's own rules. The boundary is the need to read.

## Add nothing by default

Each station — brainstorm, plan, plan review, delegated implementation, the small-task
judgment, review, the loop — costs a separate-context agent that re-reads the repository. None
is added by default; adding one needs a reason. State the line below, then act. The person
overrides it; never wait for approval.

```
outside reach · interdependent change sites · what would notice a mistake → stations added
```

| Station | Added when |
|---|---|
| brainstorm | the request reads two ways and one or two exchanges cannot settle it |
| plan | more than an implementer holds at once, or a person takes over midway |
| delegated implementation | enough volume to pollute this session's context |
| review | several interdependent change sites, so the implementation can contradict itself |
| the loop | a fix can spread beyond where it was made |

Reviewers are one per perspective: one, or two when a specification must be matched and no
machine check sees that match. A single site, independent changes, and mistakes a machine check
catches all take zero.

Three things are never traded away, and machine checks or the existing stop rules carry all
three, so none adds a station: secrets and credentials, and publishing, distribution or version,
go through the project's checks; irreversible, privileged and dangerous targets stop and ask.
Review itself is never mandatory. With no machine check, add one when that costs less than the
change; otherwise write "nothing would notice" in the line and go on — reviewers never
substitute for a missing check.

A review left out can be asked for afterwards: the line is also how the person notices none ran,
and kotowari-review called directly, or another kotowari-iterate run on the same branch, adds it then.

Outside reach shows in four signs: a publishing, distribution or version declaration;
credentials or secrets; reaching outside (network, deploy, migration); a deletion whose users
cannot be enumerated. A fifth needs a real incident behind it.

## The entry table

| Request | Entry |
|---|---|
| The line added no station | this session edits directly |
| A small task | kotowari-iterate |
| A medium-or-larger change with no specification, no grounds to judge by, or one that reads two ways | kotowari-brainstorm |
| A medium-or-larger change with a specification | kotowari-plan, then kotowari-cycle |
| Bringing an existing specification or existing code into the IR, or not knowing whether the IR matches the implementation | kotowari-adopt |
| A defect whose cause is unknown, or a question needing reading or searching | kotowari-investigate |

The kotowari-adopt row is chosen by the kind of request, not by the stations the line named, and
it comes before every other row, the first included: a request of its kind goes to kotowari-adopt
even when the line added no station. The other rows below the first fire only for what the line
named. A small task is kotowari-iterate's own
definition; an unreadable impact or a specification decision makes a change medium or larger. A
request to change a specification goes to kotowari-brainstorm with its path, not the plan row;
re-sorting a topic's existing IR against its implementation, without deciding changes, is
kotowari-adopt. A bare "fix it" leaves the person the path of this session fixing it directly. "Make
the error message clearer" with no wording given reads two ways: kotowari-brainstorm, not a small
task. A defect sent to kotowari-investigate continues from its report's recommended next action,
which names the next entry.

## Exceptions

Work with an approved plan or in-progress records is not rewound; it enters from where it left
off, and its skill's resume rules own the records. "Implement it" with an approved plan is
kotowari-cycle — implementing by hand there is a counter-example. A skill outside the table fires
on its own description.

## Change conformance routing

For a changes-enabled project, the entry route does not replace its commit and integration gates. Route caller-written records, stale entries, intermediate index checks and independent final review to the kotowari skill's changes scene. A missing record alone is repaired within delegation; unsupported consequential meaning, changing/deleting approved requirements or contradictory choices returns to brainstorm. All implementation routes retain check and branch-wide review-phase changes before integration; the route selector itself does not implement, author another role's judgment or declare machine-verified meaning.
