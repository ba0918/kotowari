# Decision documents

English | [日本語](decision-documents.ja.md)

Decides the form of the documents that kotowari reads as the specification. It covers what to write in one document, how to write one decision, how to rewrite a decision when it changes, and where to put the documents. The correspondence between confirmations and tests is decided in [Confirmations and tests](confirmations.md), translations and guides in [Languages of decision documents and guides](languages-and-guides.md), the way of working from brainstorm to implementation in [A workflow that fills gaps as it goes](workflow.md), and the switch from the current form in [Switching from the old form](form-switch.md).

## The specification lives only in decision documents, one per topic, holding the decisions now in force

Status: implemented

### Problem

Until now the specification was held in two layers: decision records added with each brainstorm, and the IR that restated them in the form now in force. The same decision was written by hand in two places, each IR item pointed with a source line at the decision in the records it was based on, and an LLM in a separate session was made to check that their contents matched.

To change one rule, you wrote the decision in a new record, put a revised mark on the decision in the old record, rewrote the IR sentence, changed the source, and checked them against each other. To learn why some requirement was the way it was, you ended up opening five decisions in three records.

We examined the work logs of nine repositories that use kotowari. Of 10 cases of conflict, staleness or defective sources among the specification's sentences, 6 arose from holding the IR as a separate layer (the IR lagged behind the implementation, a source was missing). In an experiment (43 runs) in which we made two copies of a repository, one with the IR removed and one with it kept, and had an LLM do the same task, no difference in results was visible between having the IR and not.

The other problem is on the reading side. The user opens the documents and reads what was decided and why at approval time and when coming back to a repository after some time away. The decision records are a history, so to learn the rules now in force you end up reading old decisions too. The IR consists of requirement statements in a fixed sentence pattern and scenarios, and is not written on the assumption that people read it.

### Decision

The specification is held only in decision documents, one per topic. One file is one topic, and it lists that topic's decisions now in force. Records added with each brainstorm, and an IR that restates them, are not kept. Source lines and the cross-check are not kept either.

A decision document contains only what people decided. Details that the implementing side decided with a reason (for example numeric limits, the order of output, the position of the line where an error is reported) are not written. The source of truth for details is the code and the tests, and the reasons for them are in the commit body and the pull request body ([Details found during implementation are decided by the implementing side](workflow.md#details-found-during-implementation-are-decided-by-the-implementing-side)).

"Decision" always describes the current state. History such as "previously" or "changed to ..." is not written. The history is in git. Only the history needed to convey why it was decided that way is written, in "Problem" and "Rejected options".

### Rejected options

- Previous decision: The decision records are kept in a separate file per brainstorm and are not overwritten. Revised decisions are added, and the old line gets a link to its revision. The form now in force is written in the IR, and each IR item cites a decision in the records as its source. This was dropped because it required writing twice and the cross-check, and because learning the current rules meant reading old decisions too. At that time an option to keep one note file per topic in the present tense was also considered, and it was rejected on the grounds that the IR played the same role with machine checks, so it would be a duplicate. Because the IR is the one being dropped, that reason for rejection no longer holds, and this time that form was adopted.
- Make it one layer while keeping the form with requirement statements in a fixed sentence pattern and scenarios, and add reasons and rejected options. One small repository was moved to this form as a trial. The counting worked, but when the user opened it and read it, they could not read what had been decided.
- Hold decisions as text in an accumulating history, put the current source of truth in another document, and point to it through an index. Writing two layers by hand remains, and the problem of reading old decisions merely turns into the effort of looking things up in the index.

### Confirmations

- C-core-docs-1 check reads the topic documents under the location of decision documents, and counts the decisions and confirmations written in them
- C-core-docs-2 check does not require source lines or decision records per brainstorm
- C-core-docs-3 (by reading) The brainstorm skill says that what was decided is written directly in the topic document, and that records per brainstorm and the IR are not written
- C-core-docs-4 (by reading) The brainstorm skill says that a decision document contains only what people decided, and that details decided by the implementing side are not written

## Write one decision as text in Problem, Decision, Rejected options and Confirmations

Status: implemented

### Problem

A decision document must be text that people can read as it is. At the same time, a machine needs to be able to count whether what was decided is actually confirmed. Requirement statements in a fixed sentence pattern and Given / When / Then scenarios are easy for a machine to count, but hard for people to read and grasp what was decided. One requirement bundled several decisions, and there were items with 10 reasons and 9 rejected options attached.

### Decision

A topic document starts with a title and a paragraph on what the topic covers. After that, the decisions are listed one by one under headings. A heading is a sentence that states the decision in a word. One heading is one decision; several decisions are not bundled into one.

One decision is written in the following order.

| Part | What to write |
|---|---|
| Status | "proposed" (decided but not built yet) or "implemented" (built, and the current behaviour) |
| Problem | What was troubling. Written in sentences that can be read without knowing what was decided. Why it was decided that way is also written here and in the sentences of "Decision" |
| Decision | The rules now in force, in plain sentences |
| Rejected options | Options that were considered and not adopted, and why they were not adopted. If there are none, this section is omitted. Options that were not actually considered are not made up and written |
| Confirmations | Results that can be checked by looking, showing that what was decided is kept, one per line |

Requirement statements in a fixed sentence pattern and Given / When / Then scenarios are not written. A confirmation row is a sentence that people reading it at approval time can recognise as "I want this checked". Strict conditions such as input values or the detailed shape of output are placed in the test that points at that line.

The "Open" heading placed at the end of the document ([Write questions handed back to people in the "Open" section of the topic document](#write-questions-handed-back-to-people-in-the-open-section-of-the-topic-document)) is not treated as a decision.

A decision table (a table of condition and result pairs) is written as a Markdown table inside "Decision". A property (something that holds for any input) is written as a line in "Confirmations". A decision with no behaviour to confirm, such as a decision on a workflow or a policy, may be written in the same document without "Confirmations". No dedicated section or dedicated kind of ID is created for decision tables or properties.

### Rejected options

- Previous decision: Requirements are written in a fixed sentence pattern and carry a kind, a confirmation method and a source line. Scenarios are written as Given / When / Then and point at requirements. Decision tables and properties have dedicated headings and kinds of ID. This was dropped because people could not read it.
- Previous decision: Decisions that are not product behaviour (rules on the workflow, the build, releases) are not written in the IR and are kept only in the decision records. Because the records are no longer kept, these are now written in the same document as decisions without confirmations.

### Confirmations

- C-core-docs-5 check reports an error for a decision whose status line is neither "proposed" nor "implemented"
- C-core-docs-6 check reports an error for a decision that lacks the "Problem" or "Decision" section
- C-core-docs-7 check reports no error for a decision that lacks the "Rejected options" and "Confirmations" sections
- C-core-docs-8 check reports an error for a document that has no paragraph on what the topic covers after the title

## Rewrite when a fact changes; when overturning a decision, write the old decision in Rejected options

Status: proposed

### Problem

Once it is decided to keep only the decisions now in force, what to do with the old one when a decision changes becomes a problem. If it is rewritten silently, why it changed remains only in git, and the same option gets considered again. If the old one is kept, learning the current rules means reading the old ones too.

### Decision

When a fact in a decision (a value, a name, a path) changes, the decision is rewritten in place.

When the decision itself is overturned, a new decision is set up. In the new decision's "Rejected options", the old decision and why it was dropped are written as a "Previous decision". Of the old decision's reasons and rejected options, those that still mean something are moved to the new decision. The body of the old decision is deleted. Tests that pointed at the old decision's confirmations are changed to point at the new decision's confirmations, or deleted if nothing is left to check.

When implementation shows that the sentences of a decision do not match the facts (they do not hold literally, for example), and the direction decided does not change, the sentences are corrected as a rewrite of facts.

### Rejected options

- Move the overturned old decision as it is to the frozen tree. The frozen tree keeps growing, and so does the amount an agent picks up by searching.
- Write "overturned" on the overturned old decision and keep it in the same document. Reading old ones to learn the current rules happens inside the same file.
- Keep neither status nor history, rewrite in place, and leave the history to git alone. What was decided can be rewritten silently, and the reasons remain only in git.

### Confirmations

- C-core-docs-9 (by reading) The brainstorm skill says that when overturning a decision, a new decision is set up, the old decision and why it was dropped are written in Rejected options, and the old body is deleted
- C-core-docs-10 (by reading) The implementation skill says that when the facts found by implementing do not match the sentences of a decision, the sentences are corrected as long as the direction decided does not change, and this is written in the pull request body

## To refer to another decision, link to its heading

Status: implemented

### Problem

A decision sometimes relies on a decision in another topic, or on a definition used by several decisions. Pointing at them by listing IDs in the sentences makes the text hard to read.

### Decision

To refer to another decision, a Markdown link is placed from within the sentence to the heading of that decision. A definition is written in the decision that decided it, and the side that uses it links there. No section consisting only of definitions is created. The meanings of words stay in the glossary, as before.

How links to headings are written follows GitHub's rules for rendering Markdown. If the document or heading a link points to does not exist, check treats it as an error. When the text of a heading is changed, the links and guide markers that point at it are fixed in the same change.

### Rejected options

- Give decisions IDs and point at them by writing IDs in the sentences. This does not break when a heading changes, but IDs line up in the sentences and make them hard to read.

### Confirmations

- C-core-docs-11 check reports an error when a link in a decision document points at a document or heading that does not exist

## When the same decision appears in two topics, write it on the side where the behaviour is visible

Status: proposed

### Problem

One decision sometimes concerns two topics. If it is written in both, one gets fixed and not the other, and they conflict.

### Decision

A decision that spans two topics is written in the topic on the side where the behaviour is visible (the side where confirmation rows can be placed). From the other one, [a link is placed to the heading](#to-refer-to-another-decision-link-to-its-heading).

### Confirmations

- C-core-docs-17 (by reading) The brainstorm skill says that a decision spanning two topics is written on the side where the behaviour is visible, and linked from the other one

## Keep the glossary as a document for people, and drop the term check

Status: implemented

### Problem

Until now, check read words enclosed in backquotes in the specification's sentences as terms, and treated them as errors if they were not in the glossary. Decision documents are text that reads as plain sentences, so terms are not enclosed. Without enclosure, a machine cannot tell which words are terms. The check that finds vague words ("appropriately", "as needed", and so on) only compares against a word list, so it works without enclosure.

### Decision

The term check (treating words not in the glossary as errors) is not done on decision documents. The vague-word check is applied to a decision's "Decision" and "Confirmations".

The glossary is kept as a document that people read. Its location is `CONTEXT.md` in the group's folder (`docs/decisions/<group>/CONTEXT.md`). check does not read it as a topic document and does not check its contents.

### Rejected options

- Previous decision: Terms in the specification's sentences are enclosed in backquotes, and words not in the glossary are treated as errors. This was dropped because enclosures would come back into the sentences of decision documents and make them hard to read as plain sentences.
- Drop the vague-word check too. Even when a vague word gets in, check reports nothing, and only people see it at approval.
- Keep no glossary file, and write the meaning of a word in the decision that decided it. Looking for a word's meaning means looking for the decision.

### Confirmations

- C-core-docs-18 check reports a finding when a decision's "Decision" or "Confirmations" contains a vague word
- C-core-docs-19 check reports no finding even when a decision's "Problem" or "Rejected options" contains a vague word
- C-core-docs-20 check does not read `CONTEXT.md` in a group's folder under the location of decision documents as a topic document, and does not require a paragraph after the title or the form of decisions from it

## Write questions handed back to people in the "Open" section of the topic document

Status: proposed

### Problem

Until now, questions handed back to people were written in a separate document (the issue record), and the consistency phase inside the implementation loop wrote a new decision record on each run. Because accumulating records are no longer kept, they need somewhere to go.

### Decision

What is not decided yet, and questions handed back to people, are written in the "Open" section at the end of the related topic document, stating what is not decided and who decides it and when. No separate document such as the issue record is kept.

Only what people decide is written in "Open". Details that the implementing side may decide are not written.

"Complete" (complete) means that check has zero errors and no topic document has any item in "Open". Decisions remaining in the "proposed" status do not change whether it is complete.

### Rejected options

- Previous decision: Questions handed back to people are written as items in the issue record (FLAGS.md) in the IR's location. This was dropped because one document in a form different from decision documents would remain.

### Confirmations

- C-core-docs-12 If the "Open" section has even one item, complete in status is false
- C-core-docs-13 If check has zero errors and no document has any "Open" item, complete in status is true

## Put decision documents in docs/decisions, and move documents in the old form to the frozen tree

Status: implemented

### Problem

It is necessary to decide where to put decision documents, and how to handle documents in the old form from before the switch (the IR, decision records, ADRs). Reading old documents as the basis for the current rules leads to mix-ups.

### Decision

Decision documents are placed under `docs/decisions/`, one file per topic. The groups per crate (core, schema, view) are directories under it. The location can be changed in kotowari's configuration file.

Documents in the old form from before the switch are not deleted but moved to `docs/frozen/`. At the top, a notice saying "Do not read this as the basis for the current rules. Do not edit it" is placed, and the tree is added to the settings that search tools skip by default. check does not read the frozen tree. Only documents in the old form from before the switch go into the frozen tree; nothing is added later.

### Rejected options

- Delete documents in the old form and read the history in git. When you want to check details or reasons dropped during the move, you end up digging through git.

### Confirmations

- C-core-docs-14 check does not read documents under the frozen tree, and does not count old-form IDs or errors there
- C-core-docs-15 When the location of decision documents is changed in the configuration, check reads that location
- C-core-docs-16 (by reading) At the top of the frozen tree there is a notice saying not to read it as the basis for the current rules and not to edit it
