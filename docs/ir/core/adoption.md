# Adoption into an existing project

English | [日本語](adoption.ja.md)

Covers the procedure by which kotowari-adopt, the workflow station skill for bringing kotowari into an existing project midway, compares the `legacy material`, the implementation and the tests for one topic, sorts them, and writes them into the `IR`, the `decision record` and the `flag record`. This document holds only requirements checked by a person or an LLM reading the skill's text, and does not change the behavior of kotowari itself.

## Requirements

### REQ-core-214: The adoption skill and its entry

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A1, docs/decision/records/2026-09-26-adoption.md#A12
- verification: review
- how_to_verify: Confirm that "agent/skills/kotowari-adopt/" exists and says that its approval is done by naming kotowari-brainstorm's approval procedure, and that kotowari-using-workflow's entry has a row that routes to kotowari-adopt a request to bring an existing specification or existing code into the IR and a request where it is unclear whether the IR matches the implementation

The kotowari skills under "agent/skills/" always hold adoption as the workflow station skill kotowari-adopt, and do its approval by kotowari-brainstorm's approval procedure. kotowari-using-workflow routes to kotowari-adopt a request to bring an existing specification or existing code into the IR and a request where it is unclear whether the IR matches the implementation.

### REQ-core-216: Sorting behaviors and terms

- kind: algorithm
- source: docs/decision/records/2026-09-26-adoption.md#A4, docs/decision/records/2026-09-26-adoption.md#A5, docs/decision/records/2026-09-26-adoption.md#A8, docs/decision/records/2026-09-26-adoption.md#A10, docs/decision/records/2026-09-26-adoption.md#A20, docs/decision/records/2026-09-26-adoption.md#A21, docs/decision/records/2026-09-26-adoption.md#A26, docs/decision/records/2026-09-26-adoption.md#A30, docs/decision/records/2026-09-26-adoption.md#A31, docs/decision/records/2026-09-26-adoption.md#A32, docs/decision/records/2026-09-26-adoption.md#A35, docs/decision/records/2026-09-26-adoption.md#A37, docs/decision/records/2026-09-26-adoption.md#A40
- verification: review
- definition: TBL-core-037
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm that every row of TBL-core-037 is written with the same destination

### REQ-core-217: How the adoption's decision record is written

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A9, docs/decision/records/2026-09-26-adoption.md#A21, docs/decision/records/2026-09-26-adoption.md#A24, docs/decision/records/2026-09-26-adoption.md#A26, docs/decision/records/2026-09-26-adoption.md#A31, docs/decision/records/2026-09-26-adoption.md#A36
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm that all of the following are written. One adoption run creates one `decision record`, named "YYYY-MM-DD-adopt-<topic>.md". Each confirmed behavior and term gets one decision per line (per word), its "decided_by" is "利用者（現状追認の一覧を承認）" (the person, approving the confirmation list), and the `requirement` or term cites that line's decision as its `source`. Each FLAG, whatever its kind, gets its own decision "leave this undecided as a FLAG", which becomes that FLAG's `source`, and the location of the `legacy material` is shown in the decision's body or by a link. The decisions of `deferral` and the Rejected entries for what was in the `legacy material` but will not be built are also written in the same `decision record`, one per line

kotowari-adopt always writes the confirmed behaviors and terms, the matters left as FLAGs, the matters put under `deferral`, and the matters not to be built as decisions, one matter per line, in the one `decision record` of an adoption run, and the `requirement`, the term and the FLAG cite that line's decision as their `source`.

### REQ-core-218: Sorting tests

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adopt-verification.md#A2, docs/decision/records/2026-09-26-adoption.md#A29, docs/decision/records/2026-09-26-adoption.md#A34
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm it is written that every test in the scope is sorted to exactly one of four destinations: grounds of a `requirement` (those that check a row that becomes a `requirement` or a row that is a confirmation candidate), a test of a FLAG's behavior, only tracing implementation details (a deletion candidate), and keep (those that check internal behavior not put in the IR, and those that cannot be judged); that the breakdown is shown in the list round and at approval and written as one line in the `decision record`'s Context; and that kotowari-adopt itself places no `mark` on a test, deletes no test and moves no test

kotowari-adopt always sorts each existing test in the topic's scope to exactly one of the four destinations, shows the breakdown, and does not touch the tests themselves.

### REQ-core-219: How many times the person is asked

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A1, docs/decision/records/2026-09-26-adoption.md#A5, docs/decision/records/2026-09-26-adoption.md#A9, docs/decision/records/2026-09-26-adoption.md#A29, docs/decision/records/2026-09-26-adoption.md#A30, docs/decision/records/2026-09-26-adoption.md#A33, docs/decision/records/2026-09-26-adoption.md#A35, docs/decision/records/2026-09-26-adopt-verification.md#A1
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm that all of the following are written. The person is asked three times: the scope check, one list round, and approval; questions during the configuration setup do not count toward these. A behavior row of the list shows the `legacy material`'s statement, the implementation's behavior, the location of the grounds test (none if there is none), and the verification value the `requirement` would get; a term row shows the `legacy material`'s definition, how the implementation uses the term, and the grounds. For confirmation candidates, the person removes only the rows that feel wrong, and rows they are unsure of stay. For `legacy material` statements and existing `requirement` items with no implementation, the person is asked whether they will be built. The rows that become FLAGs and the test breakdown are only shown. Approval is done once by kotowari-brainstorm's approval procedure

kotowari-adopt always limits asking the person to three times, the scope check, one list round and approval, and does not interview item by item.

### REQ-core-220: The request after approval

- kind: event_driven
- source: docs/decision/records/2026-09-26-adopt-verification.md#A2, docs/decision/records/2026-09-26-adoption.md#A34, docs/decision/records/2026-09-26-adoption.md#A38
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm it says that after approval, the following work is put together into one request text handed to the person, with the workflow station decided by kotowari-using-workflow: reviewing whether each grounds test checks its `requirement` sufficiently and placing a `mark` only on the sufficient ones, the person deciding in one batch whether to delete the tests that only trace details, bringing the tests to be marked into the range of "tests.files" in the configuration, and writing tests for the `requirement` items with no grounds test; that the request text is not left in a file; and that the request text includes, for a project that runs mutation tests, comparing the surviving mutations on the topic's code before and after the deletion, and if they increase, restoring the deleted tests and putting them in keep

When approval is obtained, kotowari-adopt puts the work of placing the `mark`, the work of deciding whether to delete the detail tests, the work of relocating tests, and the work of writing tests for the `requirement` items with no grounds test into one request text handed to the person, and leaves which workflow station runs it to kotowari-using-workflow.

### REQ-core-221: Guidance on adoption

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adopt-verification.md#A2, docs/decision/records/2026-09-26-adoption.md#A27
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm it is written that tests outside "tests.files" in the configuration are outside kotowari and are not an `error` even without a `mark`; that while FLAGs remain, complete in "kotowari status" is false, and that this is intended as the list awaiting collection; and that requirement_without_test for a `requirement` with no grounds test remains until the test is written, and a project that requires "kotowari check" on push or in CI is stopped by it, so the adoption and the later test work proceed on the same branch

kotowari-adopt always tells the user the range in which a test with no `mark` is not an `error`, why complete stays false while FLAGs remain, and that because of the `error` left by a `requirement` with no test, the adoption and the later test work proceed on the same branch.

### REQ-core-222: Verification and scenarios of confirmed requirements

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adopt-verification.md#A1, docs/decision/records/2026-09-26-adopt-verification.md#A3
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm it is written that the verification value of a `requirement` written by confirmation is decided as "unit" for a behavior of the implementation, "property" for a property that holds over the whole input, and "review" for something outside the code (documents, operating procedures), with how_to_verify saying how to check it; that a `requirement` is not made "review" because it has no test; and that a success-condition and counter-example `scenario` is not required and is written only when the `legacy material` or a grounds test holds a concrete case

kotowari-adopt always decides the verification value of a `requirement` written by confirmation by what the behavior is, not by whether a test exists, and writes a `scenario` only when there is a concrete case.

### REQ-core-235: Reducing the list of unspecified surfaces

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-surface-check.md#A4, docs/decision/records/2026-09-27-surface-check.md#A11, docs/decision/records/2026-09-27-surface-check.md#A28, docs/decision/records/2026-09-27-surface-check.md#A15
- verification: review
- how_to_verify: Read "agent/skills/kotowari-adopt/" and confirm it is written that every surface raised as surface_without_spec when the surface rules are first written is put into the list of unspecified surfaces and reduced topic by topic through adoption; that when adoption makes a topic's surface a requirement, that surface's entry is removed from the list of unspecified surfaces; and that the entry of a surface that became a FLAG or that the person removed in the list round is kept

kotowari-adopt always puts every `surface` not in the `IR` into the `list of unspecified surfaces` when the `surface rule` is first written, and for each adoption topic makes that topic's `surface` a `requirement` and removes it from the `list of unspecified surfaces`.

## Decision tables

### TBL-core-037: Destinations of behaviors and terms

- source: docs/decision/records/2026-09-26-adoption.md#A4, docs/decision/records/2026-09-26-adoption.md#A5, docs/decision/records/2026-09-26-adoption.md#A8, docs/decision/records/2026-09-26-adoption.md#A10, docs/decision/records/2026-09-26-adoption.md#A20, docs/decision/records/2026-09-26-adoption.md#A21, docs/decision/records/2026-09-26-adoption.md#A26, docs/decision/records/2026-09-26-adoption.md#A30, docs/decision/records/2026-09-26-adoption.md#A31, docs/decision/records/2026-09-26-adoption.md#A32, docs/decision/records/2026-09-26-adoption.md#A35, docs/decision/records/2026-09-26-adoption.md#A37, docs/decision/records/2026-09-26-adoption.md#A40

"Legacy statement" does not include the topic's existing IR requirements. Existing IR requirements are handled only by the "Existing requirement" rows. Every FLAG, of any kind, cites as its source a decision of its own.

| Subject | Situation | Destination |
|---|---|---|
| Legacy statement | A decision in the decision records that can be cited as a source agrees with the implementation | A requirement citing that decision |
| Legacy statement | Agrees with the implementation (no decision that can be cited) | Confirmation candidate |
| Legacy statement | Disagrees with the implementation | contradiction FLAG |
| Legacy statement | After reading, no confidence whether it agrees or disagrees | ambiguity FLAG |
| Legacy statement | No implementation; the person answered it will be built | Deferred requirement citing a decision to defer it |
| Legacy statement | No implementation; the person answered it will not be built | Rejected in the decision record |
| Behavior with no legacy statement | Visible to users, and a test checks it | Confirmation candidate marked "no legacy material" |
| Behavior with no legacy statement | Visible to users, no test | gap FLAG |
| Behavior with no legacy statement | Internal, not visible to users | Not put in the IR |
| Confirmation candidate | The person did not remove it in the list | A requirement or term citing that row's own confirmation decision |
| Confirmation candidate | The person removed it in the list | contradiction FLAG |
| Existing requirement | Agrees with the implementation | Keep the requirement as it is |
| Existing requirement | Disagrees with the implementation | Do not rewrite the requirement; contradiction FLAG with its ID in related |
| Existing requirement | After reading, no confidence whether it agrees or disagrees | Do not rewrite the requirement; ambiguity FLAG with its ID in related |
| Existing requirement | No implementation; the person answered it will be built | Add to that requirement a "- deferred:" line citing a decision to defer it |
| Existing requirement | No implementation; the person answered it will not be built | Do not delete the requirement; contradiction FLAG with its ID in related |
| Legacy term definition | Agrees with how the implementation and tests use it | Confirmation candidate |
| Legacy term definition | Disagrees with how the implementation and tests use it | contradiction FLAG |
| Legacy term definition | Its reading splits | ambiguity FLAG |
| Term used in a requirement's statement | No definition in the legacy material | Make a definition written from how the implementation uses it a confirmation candidate |

## Examples

```gherkin
@id=EX-core-400 @about=REQ-core-216,REQ-core-219 @source=docs/decision/records/2026-09-26-adoption.md#A21,docs/decision/records/2026-09-26-adoption.md#A9
Scenario: A row the person is unsure of is confirmed
  Given the list has a row for a behavior whose `legacy material` statement agrees with the implementation, and the person cannot decide whether the row feels wrong
  When the person answers the list round
  Then the row is not removed and becomes a `requirement` citing that row's own confirmation decision as its `source`

@id=EX-core-401 @about=REQ-core-216,REQ-core-217 @source=docs/decision/records/2026-09-26-adoption.md#A21,docs/decision/records/2026-09-26-adoption.md#A31
Scenario: A row removed in the list becomes a FLAG
  Given the person removed in the list a row for a behavior whose `legacy material` statement agrees with the implementation
  When kotowari-adopt writes the IR and the `decision record`
  Then the row does not become a `requirement` but a FLAG of kind "contradiction", and that FLAG cites a decision of its own as its `source`

@id=EX-core-402 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A20
Scenario: A remaining decision is used before confirmation
  Given the place for each `decision record` holds a decision with a `decision number` that decided the topic's behavior, and it agrees with the implementation
  When kotowari-adopt sorts
  Then that behavior does not become a confirmation candidate but a `requirement` citing the existing decision as its `source`

@id=EX-core-403 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A30
Scenario: An existing IR requirement disagrees with the implementation
  Given the topic has an existing `requirement` "REQ-x-001", and the implementation behaves differently from its `statement`
  When kotowari-adopt sorts
  Then "REQ-x-001" is not rewritten, and a FLAG of kind "contradiction" with "REQ-x-001" in related is created

@id=EX-core-404 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A30
Scenario: An existing requirement with no implementation will be built
  Given the topic has an existing `requirement` "REQ-x-002" with no implementation, and the person answered in the list that it will be built
  When kotowari-adopt writes the IR and the `decision record`
  Then "REQ-x-002" is not deleted, and a "- deferred:" line citing a decision of `deferral` as its `source` is added

@id=EX-core-405 @about=REQ-core-216 @source=docs/decision/records/2026-09-26-adoption.md#A32,docs/decision/records/2026-09-26-adoption.md#A4
Scenario: The person removes a behavior not in the legacy material
  Given a behavior with no statement in the `legacy material`, visible to users and with a test, is in the list as a confirmation candidate marked "no legacy material", and the person removed it
  When kotowari-adopt writes the IR and the `decision record`
  Then the behavior does not become a `requirement` but a FLAG of kind "contradiction"

@id=EX-core-406 @about=REQ-core-218 @source=docs/decision/records/2026-09-26-adoption.md#A34,docs/decision/records/2026-09-26-adoption.md#A29,docs/decision/records/2026-09-26-adopt-verification.md#A2
Scenario: The difference in test counts is shown by the breakdown
  Given the topic's scope has 300 existing tests, of which 40 are grounds of a `requirement`
  When kotowari-adopt shows the list round
  Then the breakdown of destinations for all 300 is shown, no test gets a `mark`, and not one is deleted
```
