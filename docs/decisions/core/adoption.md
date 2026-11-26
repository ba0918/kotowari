# Adoption into an existing project

English | [日本語](adoption.ja.md)

This document covers how the workflow skill kotowari-adopt, which brings kotowari into an existing project midway, compares the legacy material, the implementation and the tests for one topic, sorts them, and writes them into the decision documents. How the scope is decided and how the rows of the list are made is decided in [Adoption scope and how rows are made](adoption-scope.md). Writing behaviour the person has ratified as a decision whose state is "implemented", writing no per-intake record, and not writing behaviour the person has not ratified and not inventing reasons are decided in the workflow topic's [When taking in existing code, behaviour the person has ratified is written as "implemented"](workflow.md#when-taking-in-existing-code-behaviour-the-person-has-ratified-is-written-as-implemented). This document holds only decisions that a person or an LLM checks by reading the skill's text.

## Adoption is done by the workflow skill kotowari-adopt, and approval follows the brainstorm's approval procedure

Status: implemented

### Problem

The brainstorm skill kotowari-brainstorm does not end until the question tree is exhausted, and does not compromise. Adoption is the opposite: its core is to leave undecided what it does not decide and move on. Putting both in the same skill would hollow out the rules of one of them. A procedure with scenes that ask the person belongs to a workflow skill, and the kotowari skill is a dictionary of how to write.

### Decision

The adoption procedure is held as the workflow skill kotowari-adopt. Approval uses kotowari-brainstorm's approval procedure, by naming it.

### Rejected options

- Add a scene for adoption to the references of the kotowari skill. This would bring a procedure that asks the person into the dictionary side.

### Confirmations

- C-core-adopt-1 (by reading) `agent/skills/kotowari-adopt/` exists, and that skill says that approval is done by naming kotowari-brainstorm's approval procedure

## A request to bring in an existing specification or existing code is routed to kotowari-adopt at the entry

Status: implemented

### Problem

Without routing at the entry, adoption runs as a brainstorm, and splitting adoption into a separate skill loses its point.

### Decision

kotowari-using-workflow routes to kotowari-adopt a request to bring an existing specification or existing code into the decision documents, and a request where it is unclear whether the decision documents match the implementation.

### Confirmations

- C-core-adopt-2 (by reading) The kotowari-using-workflow skill has a row that routes to kotowari-adopt a request to bring an existing specification or existing code into the decision documents and a request where it is unclear whether the decision documents match the implementation

## Agreement is judged by reading the implementation, and whether tests exist is not a condition of agreement

Status: implemented

### Problem

Checking by running everything whether the legacy material's statements agree with the implementation is too heavy a cost for adoption.

### Decision

Whether a statement of the legacy material agrees with the implementation is judged by reading the implementation. Whether a test checks it is not a condition of agreement; it is written in the list.

### Confirmations

- C-core-adopt-3 (by reading) The kotowari-adopt skill says that agreement is judged by reading the implementation, and that whether a test exists is not a condition of agreement but is written in the list

## The destination of a row is decided by the result of comparing the legacy material with the implementation

Status: implemented

### Problem

In an existing project, what agrees between the legacy material and the implementation, what disagrees, and what exists on only one side are mixed together. Trying to decide all of it on the spot during adoption inflates the interview, so what is not decided has to be left undecided where it can be seen, and the work has to move on.

For behaviour where the legacy material and the implementation agree, the result the user expected has itself faded from memory over the gap in time, and asking item by item what was expected yields no answer. The risk of fixing in place what two sources agree on is smaller than the loss of adoption stalling to review everything. Behaviour that has no statement in the legacy material but is visible to users and checked by a test carries evidence that someone has kept it on purpose. A decision in the remaining decision records weighs more than a ratification. For behaviour that exists only in the legacy material, if the person separates what will be built from what will not, what will be built can be placed in the specification in a form that asks for no test. Checking everything by running it is too heavy a cost, so the possibility of a misreading is left where it can be seen, as rows that could not be read through.

Even for a topic that already has a decision document, the way its decisions are divided can be doubtful. Rewriting decisions on the spot during adoption hides their relation to what the person decided. The reason for not writing in the decision documents the legacy statements the person answered will not be built, and the reason for making behaviour with no legacy statement and no test a ratification candidate, are not recorded. The legacy material is not rewritten, so the statements not to be built remain in the legacy material.

### Decision

The rows of the list are sorted to the destinations in the following table. "Legacy statement" includes the decisions in the decision records inside the legacy material, and does not include the decisions of a decision document the topic already has. Existing decisions are handled only by the "Existing decision" rows of the table.

| Subject | Situation | Destination |
|---|---|---|
| Legacy statement | A decision in the legacy decision records that agrees with the implementation | Not asked for ratification in the list; written as a decision whose state is "implemented", as that decision says |
| Legacy statement | Agrees with the implementation (no decision record decided it) | Ratification candidate |
| Legacy statement | Disagrees with the implementation | An "Open" item |
| Legacy statement | After reading, no confidence whether it agrees or disagrees with the implementation | An "Open" item |
| Legacy statement | No implementation; the person answered it will be built | A decision whose state is "proposed" |
| Legacy statement | No implementation; the person answered it will not be built | Not written in the decision documents |
| Behaviour with no legacy statement | Visible to users, and a test checks it | A ratification candidate marked as having no legacy material |
| Behaviour with no legacy statement | Visible to users, and no test checks it | A ratification candidate marked as having no legacy material and no grounds test |
| Behaviour with no legacy statement | Internal behaviour not visible to users | Not written |
| Ratification candidate | The person did not remove it in the list | A decision whose state is "implemented" |
| Ratification candidate | The person removed it in the list | An "Open" item |
| Existing decision | Agrees with the implementation | Kept as it is |
| Existing decision | Disagrees with the implementation, or no confidence whether it agrees | Neither rewritten nor deleted; made an "Open" item that links to that decision |

Each "Open" item is written, one per item, in the "Open" section of the topic document. It states what disagrees (for a row the person removed, the disagreement between the person's expectation and the implementation; for a row with no confidence, what could not be read through) and where the legacy material is, and that the user decides it, when that feature is next brainstormed. It is not decided on the spot during adoption.

When a decision in the legacy decision records is used, it is used as it is even when it states no reason, and the legacy material is not rewritten.

### Confirmations

- C-core-adopt-4 (by reading) The kotowari-adopt skill has every row of the table above written with the same destination
- C-core-adopt-5 (by reading) The kotowari-adopt skill says that disagreements and rows that could not be read through are not decided during adoption, and are written in the "Open" section of the topic document with what disagrees (or what could not be read through), where the legacy material is, and that the user decides it when that feature is brainstormed
- C-core-adopt-23 (by reading) The kotowari-adopt skill says that a decision the topic already has is neither rewritten nor deleted, and a disagreement with the implementation is made an "Open" item that links to that decision

## Of the ratification candidates, the person removes only the rows that feel wrong, and rows they are unsure of stay and are ratified

Status: implemented

### Problem

Asking a person whose expectations are vague for a judgment item by item yields no answer. The result the user expected may itself have faded from memory over the gap in time.

### Decision

Of the ratification candidates in the list, the person removes only the rows that feel wrong. Rows the person is unsure of are not removed, and are ratified.

### Confirmations

- C-core-adopt-6 (by reading) The kotowari-adopt skill says that of the ratification candidates the person removes only the rows that feel wrong, and rows they are unsure of stay and are ratified

## The person is asked only three times: the scope round, one list round, and approval

Status: implemented

### Problem

Without fixing in advance how many times the person's help is used, the work falls back into interviews item by item.

### Decision

kotowari-adopt asks the person only three times: the scope round, one list round, and approval. There is no interview item by item. Questions during the configuration setup do not count toward these three ([When there is no configuration, the kotowari skill's setup procedure is done first, and its questions do not count toward the times the person is asked](adoption-scope.md#when-there-is-no-configuration-the-kotowari-skills-setup-procedure-is-done-first-and-its-questions-do-not-count-toward-the-times-the-person-is-asked)).

In the list round the person answers only two things: which of the ratification candidates to remove, and whether a legacy statement with no implementation will be built. What is written from the answers is approved once by kotowari-brainstorm's approval procedure.

### Confirmations

- C-core-adopt-7 (by reading) The kotowari-adopt skill says that the person is asked three times, the scope round, one list round and approval, and that questions during the configuration setup do not count toward these
- C-core-adopt-8 (by reading) The kotowari-adopt skill says that in the list round the person answers only two things: the ratification candidates to remove, and whether a legacy statement with no implementation will be built

## A row of the list sets out the legacy statement, the implementation's behaviour and the grounds test, and rows that become "Open" are only shown

Status: implemented

### Problem

For the person to find the rows in the list that feel wrong, what the legacy material says, what the implementation does, and what checks it have to be set out side by side in each row. To keep the person's questions at three, what the person does not need to answer has to be shown apart from what they answer.

### Decision

A behaviour row of the list sets out the legacy statement, the implementation's behaviour, where the grounds test is ("none" if there is none), and how that row is checked ([A confirmation row written by ratification is not made "(by reading)" because it has no test](#a-confirmation-row-written-by-ratification-is-not-made-by-reading-because-it-has-no-test)).

The rows that become "Open" items and the breakdown of test destinations are only shown in the list; the person is not asked about them. The diff of the decision documents the person reads at approval is the final version of the list.

### Confirmations

- C-core-adopt-9 (by reading) The kotowari-adopt skill says that a behaviour row of the list sets out the legacy statement, the implementation's behaviour, where the grounds test is (none if there is none), and how it is checked
- C-core-adopt-10 (by reading) The kotowari-adopt skill says that the rows that become "Open" and the test breakdown are only shown, and that the diff of the decision documents read at approval is the final version of the list

## A confirmation row written by ratification is not made "(by reading)" because it has no test

Status: implemented

### Problem

Since whether a test exists is not used to judge agreement, ratified decisions with no grounds test come out. If such a row is made a row a person or an LLM checks by reading because it has no test, the missing test is hidden on the specification side and cannot be seen.

### Decision

How a confirmation row of a decision written by ratification is checked is decided by what the behaviour is. A behaviour of the implementation, and a property that holds over the whole input, become rows checked by tests. Something outside the code (a document, an operating procedure) becomes a "(by reading)" row. A row is not made "(by reading)" because it has no test.

How each row is checked is shown in the list, and the person is not asked.

### Confirmations

- C-core-adopt-11 (by reading) The kotowari-adopt skill says that how a confirmation row written by ratification is checked is decided by what the behaviour is, and that a row is not made "(by reading)" because it has no test
- C-core-adopt-12 (by reading) The kotowari-adopt skill says that how each row is checked is shown in the list and the person is not asked

## The "Problem" of a decision written by ratification states in one sentence that it was taken as the specification without review

Status: implemented

### Problem

A ratification is not something the person judged item by item on its content. If the decision does not pretend its content was judged and honestly shows its weight, people who read it later will not misjudge it.

### Decision

The "Problem" of a decision written by ratification states in one sentence that the legacy material and the implementation agreed, so it was taken as the specification without review.

### Confirmations

- C-core-adopt-24 (by reading) The kotowari-adopt skill says that the "Problem" of a decision written by ratification states in one sentence that the legacy material and the implementation agreed, so it was taken as the specification without review

## Terms are not sorted in an intake

Status: implemented

### Problem

Terms used to be sorted like behaviour because a word not in the glossary was an error of the term check, and the specification could not be written without deciding how to handle it. The term check was dropped ([Keep the glossary as a document for people, and drop the term check](decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check)), so that reason is gone.

### Decision

kotowari-adopt does not sort terms by comparing the legacy material's definitions of terms with how they are used in the implementation and the tests. The list has no term rows.

### Confirmations

- C-core-adopt-25 (by reading) The kotowari-adopt skill has no sorting of terms and no term rows in the list

## An intake places no marker on a test, deletes no test, and moves no test

Status: implemented

### Problem

What an intake produces is limited to the specification documents, and work that touches code is left to the implementation workflow.

### Decision

kotowari-adopt places no marker on a test, deletes no test, and moves no test.

### Confirmations

- C-core-adopt-13 (by reading) The kotowari-adopt skill says that it places no marker on a test, deletes no test, and moves no test

## Each test in the scope is sorted to one of four destinations, and the breakdown is shown in the list round and at approval

Status: implemented

### Problem

In the adoption of one existing project, there were 174 markers against 2,821 existing tests. The number of tests with a marker was far smaller than the original number of tests, and the user became uneasy. Comparing the overall numbers gives no answer; only when the breakdown of the destination of each test explains it is it clear that tests did not decrease.

Behaviour not visible to users also has important tests, so those are not made deletion candidates.

### Decision

Every existing test in the scope of the topic is assigned, one by one, to exactly one of the following.

- Grounds of a decision: those that check a row that becomes a decision or a row that is a ratification candidate. They become candidates for a marker after approval
- Tests of a behaviour made "Open": kept; how they are handled is decided when that "Open" item is decided
- Those that only trace implementation details: deletion candidates
- Keep: those that check internal behaviour not written in the decision documents, and those that cannot be judged

The breakdown of the number per destination is shown in the list round and at approval.

### Rejected options

- Add to `kotowari status` an overall number, such as the number of tests without a marker. The way of comparing by setting out overall numbers itself creates the unease, and the only meaningful number is the breakdown per topic.

### Confirmations

- C-core-adopt-14 (by reading) The kotowari-adopt skill says that every test in the scope is assigned to exactly one of four: grounds of a decision, tests of a behaviour made "Open", those that only trace implementation details (deletion candidates), and keep, and that the breakdown is shown in the list round and at approval

## The work after approval is handed over in the conversation as one request text, and the entry decides which workflow station runs it

Status: implemented

### Problem

The numbers in the breakdown of test destinations (for example 40 grounds, 250 details) can exceed the workflow for small tasks. It suits better for the entry to decide which workflow station runs it according to the numbers. The candidates can be derived again from the list and the breakdown, so they need not be kept in a file.

Tests that only trace details set the implementation in place and hinder rebuilding it, and in mutation tests the time of the whole test run is multiplied by the number of mutations. Since git can restore them, the risk of deciding in one batch whether to delete them is small. The only means to check mechanically that no important test was deleted is the comparison by mutation tests. Requiring a project that does not run mutation tests to introduce them as well inflates the cost of adoption. The deletion work is done by a workflow after the intake, so unless the comparison is written in the request text, the procedure does not reach it.

### Decision

After approval, kotowari-adopt hands the person one request text in the conversation. The request text is not kept in a file. kotowari-using-workflow decides which workflow station runs it.

The request text holds the following.

- The work of the person deciding in one batch whether to delete the tests that only trace details
- The work of bringing the tests to receive a marker into the range of "tests.files" in the configuration. Whether to move them in or widen "tests.files" follows the project's existing setting
- In a project that runs mutation tests, the procedure of comparing the surviving mutations on the topic's code before and after the deletion, and if they increase, restoring the deleted tests and putting them in "keep". It is not required of a project that does not run them

### Rejected options

- Previous decision: the placing of markers after approval, the deletion of detail tests, and the relocation of tests are run with kotowari-iterate. Since the numbers can exceed the workflow for small tasks, it was changed so that the entry decides the workflow station.

### Confirmations

- C-core-adopt-15 (by reading) The kotowari-adopt skill says that the work after approval is put into one request text handed over in the conversation, not kept in a file, and that kotowari-using-workflow decides which workflow station runs it
- C-core-adopt-16 (by reading) The request text of the kotowari-adopt skill holds the work of the person deciding in one batch whether to delete the detail tests, the work of bringing tests into the range of "tests.files", and, in a project that runs mutation tests, the comparison of surviving mutations

## In the request text after approval, reviewing the grounds tests and writing tests are done per confirmation row

Status: implemented

### Problem

Since whether a test exists is not used to judge agreement, ratified decisions with no grounds test come out. The work of placing markers alone does not remove the error of such a row. Putting such a decision in the "proposed" state because it has no test would use the state meant for a decision not built yet for behaviour that is built, which is a different meaning.

### Decision

The request text after approval holds the following two.

- The work of reviewing whether a grounds test checks its confirmation row sufficiently, and placing a marker with that row's ID only on the sufficient ones
- The work of writing tests for the confirmation rows that have no grounds test

### Confirmations

- C-core-adopt-17 (by reading) The request text of the kotowari-adopt skill holds the work of reviewing whether a grounds test checks its confirmation row sufficiently and placing a marker only on the sufficient ones, and the work of writing tests for the confirmation rows that have no grounds test

## The user is told that tests outside "tests.files" are outside kotowari and are not an error even without a marker

Status: implemented

### Problem

In one adoption "tests.files" was narrowed to some directories, with 146 tests inside and 2,680 outside. The user should be able to understand why a large number of tests without a marker are not an error.

### Decision

kotowari-adopt tells the user that tests outside "tests.files" in the configuration are outside kotowari and are not an error even without a marker.

### Confirmations

- C-core-adopt-18 (by reading) The kotowari-adopt skill says that it tells the user that tests outside "tests.files" are outside kotowari and are not an error even without a marker

## The user is told that the errors left until tests are written, and complete while "Open" items remain, are intended

Status: implemented

### Problem

A confirmation row with no grounds test remains a check error until its test is written. A project that requires check on push or in CI is stopped by it. Also, while "Open" items remain, the work is not complete. For neither of these to be read as a breakage, their intent has to be told in advance.

### Decision

kotowari-adopt tells the user the following two things.

- A confirmation row with no grounds test keeps a confirmation_without_test error until its test is written. A project that requires `kotowari check` on push or in CI is stopped by it, so the adoption and the later test work proceed on the same branch. Commits pass with the pre-commit hook that runs `kotowari check --allow-test-findings` (this handling is decided in [When taking in existing code, behaviour the person has ratified is written as "implemented"](workflow.md#when-taking-in-existing-code-behaviour-the-person-has-ratified-is-written-as-implemented))
- While "Open" items remain, complete in `kotowari status` is false. That is the list of what awaits a decision, and it is intended

### Confirmations

- C-core-adopt-19 (by reading) The kotowari-adopt skill says that it tells the user that for a confirmation row with no grounds test confirmation_without_test remains until the test is written and stops push or CI, so the adoption and the later test work proceed on the same branch
- C-core-adopt-20 (by reading) The kotowari-adopt skill says that it tells the user that while "Open" remains, complete in `kotowari status` is false, and that this is intended

## Every surface raised when the surface rules are first written goes into the list of unspecified surfaces, and the list is reduced topic by topic through adoption

Status: implemented

### Problem

When surface rules are written in an existing project, a large number of surface_without_spec come out from the start. So that adoption is still possible, there is a mechanism that puts the surfaces not yet specified into a list and does not make them errors ([A surface not yet made part of the specification is not an error if it is put on the list of unspecified surfaces with a reason](surface-unspecified.md#a-surface-not-yet-made-part-of-the-specification-is-not-an-error-if-it-is-put-on-the-list-of-unspecified-surfaces-with-a-reason)). The work of reducing that list is put on the flow of adoption.

### Decision

Every surface for which `kotowari check` raises surface_without_spec when the surface rules are first written is put into the list of unspecified surfaces, one entry each. At that time, no decision is written for those surfaces. The list is reduced topic by topic through adoption.

### Confirmations

- C-core-adopt-21 (by reading) The kotowari-adopt skill says that every surface for which surface_without_spec comes out when the surface rules are first written is put into the list of unspecified surfaces, and the list is reduced topic by topic through adoption

## When adoption writes a topic's surface into a decision, that surface's entry is removed from the list of unspecified surfaces

Status: implemented

### Problem

An entry is removed from the list only when the surface is written into the specification. A surface that did not become specification stays in the list as a consequence.

### Decision

When, in adoption, the "Decision" or the "Confirmations" of a ratified decision quotes the name of a surface of the topic, that surface's entry is removed from the list of unspecified surfaces in the same run. The entry of a surface that became "Open", and of a surface the person removed in the list round, stays in the list.

### Confirmations

- C-core-adopt-22 (by reading) The kotowari-adopt skill says that when a ratified decision quotes the name of a surface of the topic, that surface's entry is removed from the list in the same run, and the entry of a surface that became "Open" or that the person removed stays

## Open

- What the "Problem" of a ratified decision says when the candidate has no legacy material. The one sentence decided for ratified decisions says that the legacy material and the implementation agreed, which is not true for such a candidate. The user decides, at the next brainstorm of this topic.
- How a decision taken from the legacy decision records that agrees with the implementation is written: what its "Problem" holds, how its confirmation rows are checked, and whether its row appears in the list. It is written as "implemented" without being asked for ratification, while the workflow topic says behaviour the person has not ratified is not written; whether such a decision counts as already ratified is not stated. The user decides, at the next brainstorm of this topic.
- Where a row goes when the topic already has a decision for it and that decision is still "proposed". The table has no row for this case. The user decides, at the next brainstorm of this topic.
- Which steps of the brainstorm's approval procedure an adoption follows. The procedure has steps made for a brainstorm (showing the scene, rebuilding the overview, the list of decisions to build, cutting the branch and the working tree). The user decides, at the next brainstorm of this topic.
