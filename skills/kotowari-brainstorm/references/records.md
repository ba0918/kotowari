# Record kinds

The decision record `docs/decision/records/YYYY-MM-DD-<name>.md` holds the items below from the
first round on, and its Position line holds the position in the tree and what the next round
covers. Update the Position line as the tree moves. The record's form — the `## Context` section,
the section headings, the indented lines each item carries, and links across documents — is
defined in the kotowari skill's `references/records.md`; follow it there.

| Kind | Holds | Goes to |
|---|---|---|
| agreement | what was decided, in the person's words when possible | `## Agreements`; the IR items built on it cite it as their source |
| prohibition | what will not be built | `## Prohibitions` |
| undecided | the open question and **who decides it** (person, or a later brainstorm — never the implementer: that is delegated) | `## Undecided` |
| delegated | a choice the person agreed to leave to implementation, and **why** every option keeps approved behavior — never filed before they answer | `## Delegated` |
| rejected | the alternative and **why** it lost | `## Rejected`, one line each, no mechanism description |
| revision | what replaced what, and why | `## Revisions`, and the replaced decision's link to its replacement |

Undecided has no answer yet. Delegated has an answerer chosen. Never file one as the other.

Recommended answers that the person did not answer are not agreements; keep them as undecided
until answered.
