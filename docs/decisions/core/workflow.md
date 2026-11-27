# A workflow that fills gaps as it goes

English | [日本語](workflow.ja.md)

This document decides, for the way work proceeds from the brainstorm through implementation to the merge, who decides and where when something the specification does not say is found. It does not try to complete the specification first; it proceeds on the premise that gaps will appear. What is decided here is written in the text of the workflow skills. The form of decision documents is decided in [Decision documents](decision-documents.md), and the correspondence with tests in [Confirmations and tests](confirmations.md).

From the work records of nine repositories that use kotowari, 65 cases where a gap in the specification appeared were read and classified. In 22 the person's policy or premise differed from the agent's recommendation or the decided specification and the person corrected it; 16 were undecided details that came up during implementation or review; 15 were matters of appearance or usability found only through use; 10 were contradictions between sentences of the specification, staleness, or faulty sources; and 2 were other. The cases were selected by the side that did the survey, and the classification is one person's reading. The decisions below are the remedy for each of these kinds.

## Details found during implementation are decided by the implementing side

Status: implemented

### Problem

During implementation or review, details that the specification does not say are found. Until now, the person was asked, and the answer was written into the specification before proceeding. In 12 of the 16 cases examined, the answer the person gave was to adopt the recommendation. Stopping to ask hardly changes the answer.

It also turns out during implementation that decisions conflict with each other, or can be read in two ways. In the cases where the confirmation of the reading was handed back to the person, the answer was also to adopt the recommendation.

### Decision

A detail found during implementation or review that the decision documents do not say is decided by the implementing side, with grounds. The person is not asked. When decisions conflict with each other, or can be read in two ways, the implementing side also decides the reading, with grounds.

It is handed back to the person only when intents the person decided themselves conflict (when either choice discards one of the things the person decided). In that case, it is written in the "Open" section of the related topic document. Whether intents conflict is told by two sentences written in the decision documents being unable to hold together.

For the details and readings the implementing side decided, what, why and the evidence are written in the commit body and in the list of judgements in the pull request body. They are not written in the decision documents. This keeps the decision documents to what the person decided.

The step in the implementation loop that fixes the specification side (the consistency step) works the same way. No per-run record is written; the reasons for the fixes are written in the pull request body.

### Rejected options

- Previous decision: details are also asked of the person, and written into the specification before proceeding. The implementing side must not decide what the specification is silent about. Dropped because the answer was almost always to adopt the recommendation, so stopping the person meant little.
- Previous decision: the consistency step writes a new decision record for each run. Dropped because records that accumulate are no longer kept.
- Ask the person only about details that concern behaviour visible to users. "Is it visible to users" is a matter of degree; no line can be drawn by looking at the documents, and it can swing to asking too much or too little.
- Write the details the implementing side decided under the related decision in the decision documents. They become indistinguishable from what the person decided, and the things to rewrite each time the code changes increase.

### Confirmations

- C-core-flow-1 (by reading) The implementation skill says that details and readings not written down are decided by the implementing side with grounds, without asking the person
- C-core-flow-2 (by reading) The implementation skill says that only when intents the person decided themselves conflict is it written in "Open" of the topic document and handed back to the person
- C-core-flow-3 (by reading) The implementation and merge skills say that what, why and the evidence of decided details are written in the commit body and the pull request body
- C-core-flow-4 (by reading) The procedure of the consistency step says that no per-run record is written and the reasons for the fixes are written in the pull request body

## The brainstorm reads the user-wide policy document before giving a recommendation

Status: implemented

### Problem

Of the 22 cases where the person's policy and the agent's recommendation differed and were corrected, about 10 were ones where the agent's recommendation went against a settled policy of the user that spans projects. Corrections of the same form were repeated in different repositories. The agent's memory is per project and cannot be read from a brainstorm in another repository.

### Decision

One user-wide policy document is kept. Its location is `kotowari/principles.md` under the user's configuration location (following `XDG_CONFIG_HOME` if it is set). The brainstorm reads it before giving a recommendation. When a recommendation goes against the policy and is corrected, one line with what the user corrected is added to this document on the spot.

When the document does not exist or cannot be read, the brainstorm says at its start that it could not read it, and proceeds without it.

### Rejected options

- Keep a policy document per project. Policies that span projects would be rewritten in each repository.
- Put it in the agent's instruction file or the agent's memory location. The location differs by kind of agent, and it cannot be read across projects.
- Keep none, and only fix corrections on the spot. The same corrections are repeated.

### Confirmations

- C-core-flow-5 (by reading) The brainstorm skill says that the user-wide policy document is read before giving a recommendation, and one line is added when corrected
- C-core-flow-6 (by reading) The brainstorm skill says that when the policy document cannot be read, it says at the start that it could not read it and proceeds

## The brainstorm first builds one scene where someone is stuck, and before approval shows what happens to that scene

Status: implemented

### Problem

Something that did not solve the requested scene was built after passing approval, with its scope written correctly in the records. Each question could be judged on its own, but whether they together reached the goal was not visible.

Also, when the person was asked in order to build the scene, the user did not remember that repository and could not answer. The answer was written in the records of the side that made the request.

### Decision

At the start of the brainstorm, one scene where a person is stuck is built concretely. Before building it, the origin of the request (the records of the person or repository that made the request, issues, past exchanges) is investigated. If the scene and the answers to what is to be decided are written in the origin, they are used as the grounds without asking the user. Only when they are not written is the scene built from the user's words and checked with the user for correctness.

Before approval, what happens to that scene with what was decided is shown. If it is not solved, it does not proceed to approval. When there is no real thing and whether it is solved cannot be judged, when and by looking at what it will be judged is decided, and it is left as open.

What the user reads at approval is the diff of the decision documents.

### Rejected options

- Before approval, only show the whole of what was decided on one page (no scene is built). Even when the whole is visible, whether it reaches the target scene is not known.
- A drift in the goal is noticed and fixed by the person after it is built. That comes after the cost of building has been paid.
- Previous decision: what the user reads at approval is the diff of the decision records (one line per decision), and the IR diff is not shown. Since records are no longer kept and the decision documents themselves become readable prose, it was decided to read those.

### Confirmations

- C-core-flow-7 (by reading) The brainstorm skill says that one scene where someone is stuck is built first, and the origin of the request is investigated before building it
- C-core-flow-8 (by reading) The brainstorm skill says that before approval what happens to that scene with what was decided is shown, and if it is not solved it does not proceed to approval
- C-core-flow-9 (by reading) The brainstorm skill says that what is shown at approval is the diff of the decision documents

## Requests that change visible behaviour are decided in the brainstorm by showing a sample

Status: implemented

### Problem

Almost all of the 15 cases about appearance and usability were found by the person after the merge, in production, or on a real device. What is decided by asking in words turns out to be wrong only when it is touched. The rounds that proceeded after agreeing on a sample were rated highly by the user.

### Decision

For a request that changes behaviour visible to users, in the brainstorm a sample is made and shown instead of asking in words, and the reaction becomes the decision. The sample is one of a working prototype, a mock, or a sample of output. What to make (including making nothing) is decided on the spot, with reasons, by the agent running the brainstorm, and the person is not asked. No step where the person touches and checks it before the merge is added.

A working prototype is throwaway, for deciding, and is not used as the base of product code. It is handed to the implementing side as something to compare against. Of the behaviours decided with the prototype, the ones that can be checked by machine become confirmation rows and tests.

### Rejected options

- Before the merge, the person touches the working thing and then it is merged. Requests that wait for the person increase.
- Find it by use after the merge and fix it with a small request. This is the way so far, and things are found late.
- The person chooses what to make each time. The questions increase.
- Use the prototype as the base and grow it as is into the product. Code that has not gone through the test-first flow and review enters the product.

### Confirmations

- C-core-flow-10 (by reading) The brainstorm skill says that for a request that changes visible behaviour a sample is made and shown, and what to make is decided by the agent with reasons
- C-core-flow-11 (by reading) The brainstorm and implementation skills say that a prototype is throwaway, not the base of the product, and is something for the implementing side to compare against

## The questions put in a round of the brainstorm are narrowed to three kinds, and the agent running the brainstorm decides the rest

Status: implemented

### Problem

There was no standard for what may be put in a round of the brainstorm, and every question whose prerequisites were settled was asked of the person with a recommendation. In a trial where an LLM played the person and three requests were carried through, the person had no answer to 11 of the 29 questions of the brainstorm. How to handle a day with only one of the break punches, how to handle punches in a wrong order, and the exit code when there are warnings were such questions, and the person's answer was "I don't know. I leave it to you." The person pays the effort of reading a question they have no answer to and replying that they leave it.

Four of the 11 were questions where no option changes the agreed behaviour and a later change is cheap. The other seven were behaviour at the edges that the person had not mentioned and that neither the sample nor the records gave a clue about. They are neither self-evident nor found by looking, so a standard that only says not to ask what reading answers leaves them.

On the other hand, widening what is decided without asking raises the risk of silently deciding a point the person cares about. In the same trial, the request to add a column of overtime had the question whether the part over 40 hours a week counts as overtime, and deciding it silently would keep a reading that bears on the calculation of pay from reaching the person.

### Decision

A question put in a round of the brainstorm is only one that is one of the following three kinds.

- The product's direction, preferences and trade-offs that neither reading nor trying settles
- Something whose change after it is decided means rebuilding what was built or the data that has accumulated
- Something for which the request's text, the sample or the records give a clue that the person cares about it

A fact that reading answers is confirmed by reading, what trying answers is tried small, and the result is written in the background of the question.

A question that is none of them is decided, with grounds, by the agent running the brainstorm ([What the brainstorm decided without asking is shown as a list at approval, written in the body of the commit of the specification, and carried to the implementing side](#what-the-brainstorm-decided-without-asking-is-shown-as-a-list-at-approval-written-in-the-body-of-the-commit-of-the-specification-and-carried-to-the-implementing-side)). A question for which it cannot be judged whether it is one of them is asked.

This standard is applied to the brainstorm. It is not applied to a small task kotowari-iterate runs or to taking in existing code.

### Rejected options

- Put every question whose prerequisites are settled in a round, with a recommendation. Questions the person has no answer to line up too, and the person pays the effort of replying that they leave it.
- Leave behaviour at the edges untouched in the brainstorm, and let the implementing side decide it during implementation. The person would see how it was decided only after the merge.
- Draw no line for what stays asked, and decide also the questions that cannot be judged. The list shown at approval grows long, and what the person does not catch there is built as it is.
- Ask about all behaviour visible to the user. Nearly all the edge questions the person had no answer to in the trial would stay asked.

### Confirmations

- C-core-flow-15 (by reading) The brainstorm skill says that a question put in a round is only one of three kinds: the product's direction, preferences and trade-offs that neither reading nor trying settles, something whose change after being decided means rebuilding what was built or the accumulated data, and something for which the request's text, the sample or the records give a clue that the person cares
- C-core-flow-16 (by reading) The brainstorm skill says that a fact reading answers is read, that what trying answers is tried small and the result written in the background of the question, that a question that is none of the three kinds is decided with grounds, and that one that cannot be judged is asked
- C-core-flow-19 (by reading) The brainstorm skill says that this standard is applied to the brainstorm, and not to a small task kotowari-iterate runs or to taking in existing code

## What the brainstorm decided without asking is shown as a list at approval, written in the body of the commit of the specification, and carried to the implementing side

Status: implemented

### Problem

If what the agent running the brainstorm decided without asking goes on without being shown to the person, a point the person cared about can be missed and not be known until after the merge. A decision document holds only what a person decided, so there is no place for what was decided without asking. Kept only in the conversation, it is lost when the cycle is started in another session after the brainstorm, and what the implementing side decides later can differ from the way of deciding the person saw at approval.

### Decision

At approval, together with the diff of the decision documents, the user reads the list of what the brainstorm decided without asking. What goes in the list is each question that is none of [the three kinds put in a round](#the-questions-put-in-a-round-of-the-brainstorm-are-narrowed-to-three-kinds-and-the-agent-running-the-brainstorm-decides-the-rest) and that the agent running the brainstorm decided with grounds, written one line each of what and why. When nothing was decided without asking, it says there is none. What the person says is wrong is written in the decision documents as something the person decided.

The rest is not written in the decision documents; it is written, in the same one line each, in the body of the commit that commits the decision documents. When the person committed by themselves, the brainstorm adds to the same branch a commit with no change to files and writes it in that body. A list is made at each approval, and when one branch has several, that of a later approval wins over an earlier one.

The cycle receives, in addition to [what it receives from the brainstorm](unattended-merge.md#the-cycle-receives-the-place-of-the-decision-documents-the-list-of-decisions-to-build-this-time-and-the-merge-target), the list of what was decided without asking, and passes it to the implementing side. The implementing side follows it. When it turns out during implementation that a different way of deciding is needed, the implementing side decides again and, as with [details found during implementation](#details-found-during-implementation-are-decided-by-the-implementing-side), writes what, why and the evidence in the commit body and the pull request body.

### Rejected options

- Add to the decision documents a section for what an agent decided. It overturns the rule that they hold only what a person decided, and changes the form check reads and the guides. It also adds something to rewrite each time the code changes.
- Only show it at approval, and do not carry it to the implementing side. What is built can differ from the way of deciding the person saw at approval.

### Confirmations

- C-core-flow-17 (by reading) The brainstorm skill says that at approval, together with the diff of the decision documents, what was decided without asking is shown one line each of what and why, that when there is none it says so, that what the person says is wrong is written as a decision, and that the rest is written in the body of the commit that commits the decision documents, or, when the person committed by themselves, in the body of a commit with no change to files added to the same branch
- C-core-flow-18 (by reading) The brainstorm skill says that the list of what was decided without asking goes into the handover to the cycle, the cycle skill says that it receives the list and passes it to the implementing side, and the implement skill says that it follows the list and, when a different way of deciding turns out to be needed, decides again and writes what, why and the evidence in the commit body and the pull request body

## When taking in existing code, behaviour the person has ratified is written as "implemented"

Status: implemented

### Problem

When taking existing code without a specification into kotowari, until now one decision record was written per intake, and behaviour without tests was sent to the issue records. Since neither is kept any more, a destination is needed. A decision in the legacy decision records is not asked for ratification in an intake, so unless it counts as already ratified, it contradicts the rule that behaviour the person has not ratified is not written. Of the 6 cases where tests were added because of check's "no test" finding (the survey in [check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it)), 4 were existing code that was given a specification later.

### Decision

In an intake, behaviour the person has ratified is written in the topic document as a decision whose state is "implemented". Confirmation rows without tests come out as check errors, and those become the list of work for writing tests. No per-intake record is written. Until the tests are in place, check keeps reporting errors. For errors about missing tests, the current practice of not stopping at commit and stopping at push (`--allow-test-findings`) is kept.

Behaviour that the code merely has and the person has not ratified is not written. A decision in the legacy decision records that agrees with the implementation is something a person decided before, so it counts as already ratified ([A decision in the legacy decision records that agrees with the implementation counts as ratified, and the reason the record gives goes in its "Problem"](adoption.md#a-decision-in-the-legacy-decision-records-that-agrees-with-the-implementation-counts-as-ratified-and-the-reason-the-record-gives-goes-in-its-problem)). If why it is that way is not known, "no reason remains" is written in the Problem section. No reason is invented.

### Rejected options

- Write behaviour without tests in the "proposed" state. "proposed" would then mean both something not yet built and something built but without tests.
- Previous decision: one decision record is written per intake, and behaviour without tests is sent to the issue records. Dropped because neither the records nor the issue records are kept any more.

### Confirmations

- C-core-flow-12 (by reading) The intake skill says that behaviour the person has ratified is written as an "implemented" decision, and no per-intake record is written
- C-core-flow-13 (by reading) The intake skill says that behaviour the person has not ratified is not written, and if the reason is not known that is written, without inventing a reason
- C-core-flow-14 (by reading) The intake skill says that a decision in the legacy decision records that agrees with the implementation counts as already ratified

## Open

- Whether, after the questions the brainstorm asks are narrowed, a run appears that silently decides a point the person cares about. The user decides. After running a trial that includes rules only the person knows, answers that disagree, and a change of mind at approval.
