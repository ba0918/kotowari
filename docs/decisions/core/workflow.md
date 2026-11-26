# A workflow that fills gaps as it goes

English | [日本語](workflow.ja.md)

This document decides, for the way work proceeds from the brainstorm through implementation to the merge, who decides and where when something the specification does not say is found. It does not try to complete the specification first; it proceeds on the premise that gaps will appear. What is decided here is written in the text of the workflow skills. The form of decision documents is decided in [Decision documents](decision-documents.md), and the correspondence with tests in [Confirmations and tests](confirmations.md).

From the work records of nine repositories that use kotowari, 65 cases where a gap in the specification appeared were read and classified. In 22 the person's policy or premise differed from the agent's recommendation or the decided specification and the person corrected it; 16 were undecided details that came up during implementation or review; 15 were matters of appearance or usability found only through use; 10 were contradictions between sentences of the specification, staleness, or faulty sources; and 2 were other. The cases were selected by the side that did the survey, and the classification is one person's reading. The decisions below are the remedy for each of these kinds.

## Details found during implementation are decided by the implementing side

Status: proposed

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

Status: proposed

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

Status: proposed

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

Status: proposed

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

## When taking in existing code, behaviour the person has ratified is written as "implemented"

Status: proposed

### Problem

When taking existing code without a specification into kotowari, until now one decision record was written per intake, and behaviour without tests was sent to the issue records. Since neither is kept any more, a destination is needed. Of the 6 cases where tests were added because of check's "no test" finding (the survey in [check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it)), 4 were existing code that was given a specification later.

### Decision

In an intake, behaviour the person has ratified is written in the topic document as a decision whose state is "implemented". Confirmation rows without tests come out as check errors, and those become the list of work for writing tests. No per-intake record is written. Until the tests are in place, check keeps reporting errors. For errors about missing tests, the current practice of not stopping at commit and stopping at push (`--allow-test-findings`) is kept.

Behaviour that the code merely has and the person has not ratified is not written. If why it is that way is not known, "no reason remains" is written in the Problem section. No reason is invented.

### Rejected options

- Write behaviour without tests in the "proposed" state. "proposed" would then mean both something not yet built and something built but without tests.
- Previous decision: one decision record is written per intake, and behaviour without tests is sent to the issue records. Dropped because neither the records nor the issue records are kept any more.

### Confirmations

- C-core-flow-12 (by reading) The intake skill says that behaviour the person has ratified is written as an "implemented" decision, and no per-intake record is written
- C-core-flow-13 (by reading) The intake skill says that behaviour the person has not ratified is not written, and if the reason is not known that is written, without inventing a reason
