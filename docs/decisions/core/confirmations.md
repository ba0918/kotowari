# Confirmations and tests

English | [日本語](confirmations.ja.md)

Decides the correspondence between the lines in a decision's "Confirmations" and tests. It covers what check requires tests for, how to write lines that are not checked by tests, and who changes a decision's status and when. The form of decision documents itself is decided in [Decision documents](decision-documents.md).

## Check counts, per confirmation row, whether a test points at it

Status: implemented

### Problem

Requirement statements in a fixed sentence pattern and scenarios disappear from decision documents. Until now check counted, per requirement ID and scenario ID, whether a test marker pointed at it, so the unit needs to be decided anew.

What a marker guarantees goes only as far as "there is at least one test that points at what was decided"; it does not guarantee that the test really checks what was decided. Quality is watched by review and by mutation testing. Even so, the guarantee of existence was not decoration. From the work logs, we picked 40 cases in which check actually reported "there is no test", and followed what happened next. 31 only appeared for things that were planned to be implemented later. In 6, a test went in that, by our reading, would not have been written without the finding, and in 1 of those the test that was written failed and revealed a mistake in the implementation. Of the rest, 2 only added a marker to an existing test, and 1 deleted the specification side.

Markers have one more use: when a decision changes, the tests that need fixing can be looked up by ID (`kotowari query`).

### Decision

Confirmation lines get IDs. An ID has the form `C-<short name of the group and topic>-<number>`. The sequence number is the one after the largest number used in that topic; the numbers of deleted lines are not closed up or reused for another line. The marker on the test side, `@kotowari[...]`, points at this ID.

check counts, for every confirmation row of a decision whose status is "implemented", whether a test points at it. If none does, it is an error. This is not required of decisions whose status is "proposed". A line written as a property is pointed at by a property test (a test that checks something holds for any input).

### Rejected options

- Previous decision: Count per requirement ID and scenario ID. Scenarios are written in the document as Given / When / Then, and their IDs are written in tests. Because scenarios are no longer written in documents, the unit was changed to confirmation rows.
- Use the decision as the unit, and count whether an implemented decision has at least one test pointing at it. When there are several confirmations, a single test is enough to pass. When a decision changes, the tests cannot be narrowed down to which confirmations change.
- Do not count. Whether tests exist is left to review and the tests themselves. Even if what was decided becomes implemented with no test, no machine stops it.
- Make the ID a per-decision number with a sub-number. The ID does not change when the topic's name changes, but looking at the ID does not tell you which topic it belongs to.

### Confirmations

- C-core-confirm-1 If no test points at a confirmation row of an implemented decision, check reports an error on that line
- C-core-confirm-2 If tests point at every confirmation row of an implemented decision, check reports no error for that decision
- C-core-confirm-3 If there are two confirmation rows with the same ID, check reports an error
- C-core-confirm-4 If a test marker points at an ID that is in no document, check reports an error
- C-core-confirm-5 Passing a confirmation ID to `kotowari query` returns the file, line and name of the tests that point at it

## Write "(by reading)" at the head of a line not checked by tests

Status: implemented

### Problem

Some confirmations cannot be checked by tests and are checked by people or an LLM reading (the skill's text says such-and-such, for example). If they cannot be told apart from lines checked by tests, check ends up requiring tests for them.

### Decision

A line that people or an LLM check by reading has "(by reading)" written at its head (after the ID). check does not require a test for that line. Those who check it by reading are the implementing side and the review.

### Rejected options

- Split Confirmations into two sections, "checked by tests" and "checked by reading". Most decisions have no lines checked by reading, so a separate rule not to write empty sections would be needed.
- Previous decision: Each requirement has a line for the confirmation method (test or by reading), and those checked by reading get an additional line on how to check. Because the split is per confirmation row rather than per decision, it is now written on the line itself.

### Confirmations

- C-core-confirm-6 For a line with "(by reading)" at its head, check reports no error even if no test points at it

## The implementing side changes the status, and decisions not built yet stay "proposed"

Status: implemented

### Problem

If the specification is written and committed first, check stops because there are no tests yet. To get past this, a flag that overlooked findings on the test side had been added. Also, a requirement that was decided but not built now had a "deferred" declaration written to remove it from checking, and warnings were reported for a deferred item that had a test and for something depending on a deferred item. Both handled the same state, "decided but not built yet", with separate mechanisms.

### Decision

A decision that has been decided but not built yet is written in the document with the status "proposed". Tests are not required for proposed decisions, so check does not stop even if only the specification is committed first.

The brainstorm hands the list of decisions to build this time to the implementation loop. The implementing side changes the status to "implemented" when every confirmation row of that decision is satisfied (lines checked by tests have tests attached, and lines checked by reading have been checked by reading). A decision with no confirmations is changed when what was decided has actually been done (for example, written in a skill or a setting changed). Proposed decisions not on the list stay as they are. status counts and shows the number of decisions still remaining as proposed.

When confirmation rows are added to an implemented decision, its status stays "implemented". That an added line has no test is reported as an error, and the implementation loop brings it to zero. This is because returning it to "proposed" would make check stop requiring tests for the lines that were there from the start too.

Even if a marker points at a proposed decision, check reports nothing (because this always happens partway through writing tests first). The deferred declaration and the warnings about it are not kept.

### Rejected options

- Previous decision: A requirement not built now gets a "deferred" declaration line, citing the decision giving the reason as its source. Warnings are reported for markers pointing at deferred items and for references depending on deferred requirements. This was dropped because the "proposed" status plays the same role.
- Have three statuses (proposed, in progress, implemented). The document's status would be rewritten twice for every piece of work.

### Confirmations

- C-core-confirm-7 Even if no test points at a confirmation row of a proposed decision, check reports no error
- C-core-confirm-8 Even if a marker points at a confirmation row of a proposed decision, check reports neither an error nor a warning
- C-core-confirm-9 status reports the number of proposed decisions and the number of implemented decisions separately
- C-core-confirm-10 (by reading) The brainstorm skill says that the list of decisions to build this time is handed to the implementation loop
- C-core-confirm-11 (by reading) The implementation skill says that the status is changed to "implemented" when every confirmation row is satisfied
- C-core-confirm-12 (by reading) The brainstorm skill says that when confirmation rows are added to an implemented decision, the status stays "implemented"
