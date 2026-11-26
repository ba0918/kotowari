# Consistency phase

English | [日本語](consistency-phase.ja.md)

This document covers how an LLM, in a dedicated step inside the cycle (the consistency phase; that is also its English name), finds and resolves the disagreements between code that was implemented or changed and the decision documents, and the conflicts between decisions. A person is involved only as a last resort. That the implementing side decides, with grounds, the details and readings the decision documents do not write, and that the consistency phase writes no per-run record and writes the reasons for its fixes in the pull request body, are decided in the workflow topic's [Details found during implementation are decided by the implementing side](workflow.md#details-found-during-implementation-are-decided-by-the-implementing-side). How the merge stops when the phase leaves a finding awaiting a person's confirmation is decided in [When a person confirms, the pull request is left open without asking for the merge, and only its location and the cause of the stop are shown](unattended-merge.md#when-a-person-confirms-the-pull-request-is-left-open-without-asking-for-the-merge-and-only-its-location-and-the-cause-of-the-stop-are-shown).

## Agreement between code and specification is looked at by the consistency phase, a step separate from the quality review

Status: implemented

### Problem

The aim was to catch choices that are not in the specification entering as implementation runs ahead in the cycle. The earlier mechanism had the implementer and the reviewer write a record for each changed file, and the machine looked only at missing and stale records. In one change, the implementer implemented the surface file narrower than the glossary and wrote "within the existing specification" in the record, and that mechanism let it through. What caught it was the cycle's conformance review (an LLM). The past records also held no discovery that started from that mechanism, and there were 136 commits made for the records.

What must be kept is the specification, not the procedure. What records guarantee by machine is only that the change is covered by records and has not changed since the records (the procedure); they do not guarantee that what the records declare matches the specification. Agreement is checked against the code, not against records. Also, the conformance review was launched only under conditions, and looking at agreement was nobody's dedicated job. Not making kotowari too heavy is also a premise. Adding skills adds guidance at the entry and rules for handing work between skills as well.

When the implementer or the fixer runs the phase, it ratifies its own implementation by itself. The reason for limiting it to a separate context is not recorded.

### Decision

Agreement between code and specification is looked at by the consistency phase, a step inside the cycle separate from the quality review. The steps of the cycle are implementation, the consistency phase, then the quality review. No new skill is made, and the instructions handed to the agent that runs the phase are placed in one reference of kotowari-cycle.

The phase is run by an LLM in a context separate from the implementer, and is left to neither the implementer nor the fixer. Running it with a model of a different family from the implementer's is preferred, but not required.

### Rejected options

- Make agreement one perspective of the review. It would be read in the same seat as quality findings, and agreement would be diluted.

### Confirmations

- C-core-consist-1 (by reading) The kotowari-cycle skill says that the steps are implementation, the consistency phase, then the quality review
- C-core-consist-2 (by reading) The instructions handed to the agent that runs the phase are in one reference of kotowari-cycle, and there is no skill for the consistency phase
- C-core-consist-22 (by reading) The kotowari-cycle skill says that the consistency phase is run by a delegate in a context separate from the implementer, left to neither the implementer nor the fixer, and that a different model is preferred but not required

## Review is launched with the quality perspective only

Status: implemented

### Problem

Since agreement became a step separate from the review, keeping a seat that looks at agreement in the review too would pay twice for the same comparison.

### Decision

The reviewers of kotowari-review are launched with the quality perspective only. There is neither a conformance seat that looks at agreement between code and specification nor a rule that adds one. Whether a document contradicts the specification is looked at within the quality perspective.

### Confirmations

- C-core-consist-3 (by reading) The kotowari-review skill says that reviewers are launched with the quality perspective only, that there is neither a conformance seat nor a rule adding one, and that whether a document contradicts the specification is looked at within the quality perspective

## The consistency phase runs after implementation and after the quality review's fixes converge

Status: implemented

### Problem

Misses come from the range read and how it is divided, not from how many times it runs. Running it after every fix adds seats with each review round trip. A run that reads the diff of all the fixes has to be able to reproduce the same range after a resume. The state added should be as little as possible.

### Decision

The consistency phase runs once after implementation, and once over the diff of all the fixes after the quality review's fixes converge. After the phase's own fixes, it runs until no new finding comes out.

As the starting point of the diff of all the fixes, the cycle's findings file holds exactly one commit: the head the first quality review saw. This is the only state held for the phase.

### Rejected options

- Run it after each of the quality review's fixes. Seats increase with each review round trip.

### Confirmations

- C-core-consist-4 (by reading) The kotowari-cycle skill says that the consistency phase runs once after implementation and once over the diff of all the fixes after the quality review's fixes converge, and after the phase's own fixes runs until no new finding comes out
- C-core-consist-5 (by reading) The kotowari-cycle skill says that the only phase state the findings file holds is the one head commit the first quality review saw

## Whether to run it is decided by the role that arranges the steps, and only a diff that moves no behaviour visible to users is skipped, with a reason left

Status: implemented

### Problem

Each run has a real cost. Whether to run it may be decided each time, but only the role that arranges the steps can decide it. How readily a gap arises depends on whether behaviour moves, not on the number of lines in the diff. The change that prompted the mechanism to be decided again added behaviour with a small diff. In this repository the wording of a skill is itself behaviour visible to users. The more hurried a direct fix is, the more readily the check is missed.

### Decision

Whether to run it is decided by the cycle (the orchestrator). For a direct fix in the main session, the main session's agent decides by the same measure. kotowari-iterate goes on the cycle's flow, so it passes through the phase.

It may be skipped only when the diff neither adds nor changes behaviour visible to users (a diff of documents only (guides are an example), a diff of tests only, a refactoring that does not change behaviour). Skill text is treated as behaviour, not as a document, and is not skipped. When in doubt, it runs. When it is skipped, a one-line reason is written in the final report.

### Rejected options

- Count the fix of a single finding among what may be skipped. The fix of a finding can also change behaviour.

### Confirmations

- C-core-consist-6 (by reading) The kotowari-cycle skill says that the cycle decides whether to run it, that it may be skipped only for a diff that neither adds nor changes behaviour visible to users, that skill text is not skipped, that it runs when in doubt, and that a skip writes a one-line reason in the final report
- C-core-consist-7 (by reading) The kotowari-iterate skill says that it passes through the phase on the cycle's flow, and the kotowari-using-workflow skill says that for a direct fix the main session decides by the same measure whether to run it

## The consistency phase run on a direct fix also takes the decision documents as its counterpart

Status: implemented

### Problem

For a direct fix in the main session too, the consistency phase is run by the same measure as in the cycle ([Whether to run it is decided by the role that arranges the steps, and only a diff that moves no behaviour visible to users is skipped, with a reason left](#whether-to-run-it-is-decided-by-the-role-that-arranges-the-steps-and-only-a-diff-that-moves-no-behaviour-visible-to-users-is-skipped-with-a-reason-left)). Since the conformance review was removed, the comparison of a topic without a specification against the specification also has no place to remain other than the phase.

### Decision

The consistency phase run on a direct fix in the main session takes the place of the decision documents as its counterpart. The decisions in range are the decisions whose text the diff touched and the decisions they link to. For a topic without a specification, it takes the approved specification document as its counterpart, or the request text when there is none.

### Confirmations

- C-core-consist-8 (by reading) The section of the kotowari-using-workflow skill on the consistency phase in a direct fix says that the counterpart is the place of the decision documents, and for a topic without a specification the approved specification document or the request text

## The consistency phase reads the code of the diff and its surroundings, and the decisions in range

Status: implemented

### Problem

The case that prompted the mechanism to be decided again is caught by comparing the code in the diff with the glossary. Reading everything every time is heavy. On the other hand, when a disagreement arises from reusing an existing function or value, following only the calling and called relations does not read where the value is produced. In that case too, the collection of surface files was built in a different step. Since the conformance review was removed, the comparison of skill text and of topics without a specification against the specification also has to remain somewhere.

### Decision

What the consistency phase reads is the code itself, not records. The code includes text shipped as product. Skill text is an example.

The range is as follows.

- Code: the code the diff changed, its direct surroundings that it calls or that call it, and one step of where the values the diff uses are produced
- Decisions: the decisions the diff implemented (the decisions the cycle received) and the decisions whose text the diff touched. The decisions they link to, and the terms they use

For a small task without a specification, the request text is the counterpart. A comparison of the whole code with all the decision documents is done only when asked for separately.

### Confirmations

- C-core-consist-9 (by reading) The reference for the consistency phase says that what it reads is the code itself, not records, and that the code includes text shipped as product (skill text is an example)
- C-core-consist-10 (by reading) The reference for the consistency phase says that the code it reads is the code the diff changed, its direct calling and called surroundings, and one step of where the values the diff uses are produced
- C-core-consist-11 (by reading) The reference for the consistency phase says that the decisions it reads are the decisions the diff implemented (the decisions the cycle received), the decisions whose text it touched, the decisions they link to and the terms they use, and that for a small task without a specification the request text is the counterpart
- C-core-consist-12 (by reading) The reference for the consistency phase says that a comparison of the whole code with all the decision documents is done only when asked for separately

## The consistency phase finds behaviour different from a decision, details without a stated reason, and conflicts between decisions

Status: implemented

### Problem

What should be caught is whatever does something different from the specification, and the gaps and contradictions on the specification side. The case that prompted the mechanism to be decided again (the surface file implemented narrower than the glossary) is a real example of this, and many gaps become visible only once something is implemented.

### Decision

The consistency phase finds the following three.

- Behaviour different from a decision
- Details the decision documents do not write and the implementing side decided. They are not written in the decision documents; what is looked at is whether they disagree with no decision and whether their reason is stated
- Two sentences among the decisions in range that cannot both hold

### Confirmations

- C-core-consist-13 (by reading) The reference for the consistency phase says that what it finds is behaviour different from a decision, details the decision documents do not write, and sentences among the decisions in range that cannot both hold

## What it finds is resolved into a state where the decision documents and the code agree, and details not written down are not taken back out of the code

Status: implemented

### Problem

The conditions for resolving vary widely. Uniformly bringing the code to the specification discards correct details too. Putting the code back within the specification when the specification has a hole discards what is correct to match a specification that is missing something. What the user wants is to land in a good state where everything agrees. What separates this from ratifying one's own implementation is grounds and measurement.

### Decision

The consistency phase fixes what it finds by itself, and always resolves it into a state where the decision documents and the code agree. What observation settles is decided by checking it through measurement or reproduction.

A detail the decision documents do not write is kept in the code when it disagrees with no decision, and what was decided, why, and the evidence are written. The code is not put back because something is not written in the specification.

### Rejected options

- By default bring the code to the specification, and add to the specification only when an approved record gives grounds. Too narrow.
- When a matter of meaning that the grounds cannot settle comes up, put the code back within what the specification writes, remove the added behaviour, and wait for the person's judgment. The user pointed out that putting the code back when the specification has a hole is wrong.

### Confirmations

- C-core-consist-14 (by reading) The reference for the consistency phase says that what it finds is always resolved into a state where the decision documents and the code agree, that what observation settles is decided by measurement or reproduction, and that a detail that disagrees with no decision is kept in the code with what, why and the evidence written

## When code disagrees with a decision, the decision's reasons are read as constraints and the code is brought to the decision

Status: implemented

### Problem

What resolving rests on is grounds and measurement. A choice that disagrees with a decision has to be made with full regard to that disagreement.

If a decision's sentence could be corrected on the grounds that tests pass, merely passing tests written to match the implementation would rewrite a sentence the person decided toward the implementation side. The reason for excluding tests from the grounds is not recorded.

### Decision

When code disagrees with a decision, that decision's "Problem" and "Rejected options" are read as constraints, and the code is brought to the decision. A decision's sentence is corrected only when a reproduction shows it cannot hold literally and the direction the person decided does not change, and it is corrected as a rewrite of a fact ([Rewrite when a fact changes; when overturning a decision, write the old decision in Rejected options](decision-documents.md#rewrite-when-a-fact-changes-when-overturning-a-decision-write-the-old-decision-in-rejected-options)). Passing tests are not grounds for correcting a decision's sentence.

### Confirmations

- C-core-consist-15 (by reading) The reference for the consistency phase says that for a disagreement the decision's problem and rejected options are read as constraints and the code is brought to the decision, and that a decision's sentence is corrected only when a reproduction shows it cannot hold literally and the direction does not change
- C-core-consist-21 (by reading) The reference for the consistency phase says that passing tests are not grounds for correcting a decision's sentence

## Only when intents the person decided themselves conflict does it go on with a default, write it in "Open", and pass it to the final report as a finding awaiting the person's judgment

Status: implemented

### Problem

A finding awaiting a person's judgment is handled by passing it to the final report without stopping the loop. That fits the form of going on with a default without stopping and reporting it at the end together with the one word that reverses it. The user holds that a person's judgment should be kept to a minimum.

### Decision

Only when the decisions in range hold two sentences that cannot both hold, and either choice discards one of the things the person decided, does a person decide. The consistency phase takes a default that keeps as much of both decisions as it can, brings the code to it, and writes in the "Open" section of the topic document the two conflicting sentences, the default taken, and that the person decides.

That finding stays open as one awaiting a person's judgment (human_judgment) and is passed to the final report. The finding states the default and the one word that reverses it, the grounds and measurements tried, and why they could not settle it.

There is no path where the phase hands it back to a person and stops. A person's judgment is a last resort and is kept to a minimum.

### Confirmations

- C-core-consist-16 (by reading) The reference for the consistency phase says that a conflict of intent goes on with a default, is written in "Open" of the topic document, stays open as a human_judgment finding passed to the final report, and states the default and the one word that reverses it, the grounds and measurements tried, and why they could not settle it
- C-core-consist-17 (by reading) The reference for the consistency phase says that there is no path where the phase hands back to a person and stops

## On agreement with the specification, the phase's decision takes precedence over the quality review's fixes

Status: implemented

### Problem

The quality review is not the role that decides the specification. A case where a fix of the quality review and the phase's decision conflict is read again by the next phase run.

### Decision

On agreement with the specification, the consistency phase's decision takes precedence over the quality review's fixes.

### Confirmations

- C-core-consist-18 (by reading) The reference for the consistency phase says that on agreement with the specification the phase's decision takes precedence over the quality review's fixes

## The phase's findings go in the cycle's findings file and follow the cycle's existing way of stopping

Status: implemented

### Problem

When the phase was given a loop of its own apart from the cycle, holes came up with every fix, each needing the finding IDs, the way of stopping, the round trips, the handover of the person's answers, and resuming to be rebuilt for the phase. Putting it on the mechanism of the review loop needs no rebuilding and does not make kotowari heavy.

### Decision

What the consistency phase finds is put in the cycle's findings file in the same shape as review findings, with the perspective consistency. In each run, the phase returns still_present or no_longer_visible for the phase's findings up to the previous run, and returns what it newly found and the commits of what it fixed.

The way of stopping follows the cycle's existing rules as they are (still_present twice in a row, a closed cause returning, the user's limit, resuming). No state of the phase's own is held (the only thing held is the head commit of [The consistency phase runs after implementation and after the quality review's fixes converge](#the-consistency-phase-runs-after-implementation-and-after-the-quality-reviews-fixes-converge)).

### Rejected options

- Add state for the phase (a per-run record, marks for new findings and repeated findings) and fill the holes one by one.

### Confirmations

- C-core-consist-19 (by reading) The kotowari-cycle skill and the reference for the consistency phase say that the phase's findings go in the findings file in the same shape as review findings with the perspective consistency, and that each run returns still_present or no_longer_visible for the previous runs' findings, and returns its new findings and the commits of what it fixed
- C-core-consist-20 (by reading) The kotowari-cycle skill says that the phase stops by the cycle's existing rules (the endings), and the reference for the consistency phase says that the phase holds no state
