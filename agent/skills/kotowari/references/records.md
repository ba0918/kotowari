Based on the kotowari specification (revised 2026-09-23; the version of kotowari itself is not pinned)

## The form of a decision record

Place and file name: a decision record goes in `docs/decision/records/YYYY-MM-DD-<name>.md`. Decide the file name first and do not rename it (the paths of sources would break). Do not delete it after approval either.

At the top, place a `## Context` section, and write "what the problem is and why it is being decided now" in three to six lines for a first-time reader. The Position line for resuming goes right after the body of that section.

Place one or more decision section headings (`## Agreements`, `## Prohibitions`, `## Delegated`, `## Rejected`), and list under them in the form `- A1 the body of the decision`. Numbers are not reused. The numbers of `Undecided` and `Revisions` cannot be used as sources (they are not decision sections).

For one entry of Agreements and Prohibitions, the first line holds only the body of the decision (one decision; the number of sentences does not matter, but reasons and history are not mixed in), and under it, as an indented bulleted list, it has:

- `- why:` Required. When it cannot be written, write `- why: not recorded` so that its absence is visible (do not invent it afterwards)
- `- rejected:` Only when there is a rejected alternative specific to that decision (keep the `Rejected` section too)
- `- decided_by:` Who decided, such as the user, or the user (took the recommendation)
- `- superseded_by:` When revised by a later decision, a Markdown link to the new decision (even when revised by a decision in another file, it may be added to the old file). Keep the `Revisions` section as the chronology too

One entry of Rejected and Delegated always has `- why:`; `- decided_by:` is optional. One entry of Undecided always has `- decides:`; `- related:` (the numbers of related decisions) is optional. The names of the supplementary lines and the headings are fixed in English so as not to depend on the user's language, and the values may be free text. The names of the supplementary lines are only these six.

This reference states the form; `kotowari check` checks whether the required supplementary lines are present and their names, and that the link of `- superseded_by:` exists. Look up its findings in the rows record_field_missing, record_field_unknown and revision_link_invalid of the table in [findings.md](./findings.md).

Supplementary lines are not read as decisions because kotowari's judgement of sources reads as decisions only the lines that, after leading spaces are removed, start with `- anchor ` where the anchor has the form of a decision number (one uppercase English letter followed by digits) (the anchor of `- why:` does not have the form of a number). The indentation itself plays no part in the judgement.

## References across documents

When referring across documents in records and in these references, always make it a Markdown link. The href is the relative path from that document followed by `#number` or `#heading` (example: from inside a record, `[A1 (example)](./2026-01-01-example.md#A1)`). If a path from the base directory (the form of a source) is written as the href, Markdown resolves it from the document's location, so it cannot be followed. When listing an IR ID, attach a link to the document that defines that item (example: from inside a record, `[REQ-001](../../ir/example.md#REQ-001)`). The ID alone does not let the reader judge the content. Sources and document name references inside IR documents are written in the plain form of the kotowari contract, and are outside this rule.

## ADRs

There is no obligation to write new ones. Existing ADRs are kept as targets of sources. The detail of reasons and the integration of revisions that ADRs used to carry are taken over by `- why:` and `- superseded_by:` above.

## Where each kind of content goes

Success conditions and counterexamples: write the observable success conditions and counterexamples required of each requirement as scenarios under the IR's `## Examples`. Write a success condition as a scene that passes, and a counterexample as a scene where a finding or a stop appears. Only the allowed lines can be placed under a requirement's heading.

Prohibitions, rejections, undecided matters and delegations: a topic document of the IR can hold only requirements, decision tables, properties and examples, so these go in sections of the decision record.

Contradictions, gaps and ambiguities: write them as `### FLAG-nnn: name` in `FLAGS.md` with `- kind:`, `- related:` and `- source:`.

Glossary: a glossary `CONTEXT.md` can be placed in any directory, and what a document sees is the terms of the `CONTEXT.md` files from its own directory up to the root of the place for the IR (the chain). A term is placed in only one file in the chain.
