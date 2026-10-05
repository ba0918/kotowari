# Skills, hooks and CI that run change conformance

English | [日本語](change-workflow.ja.md)

Covers recording during implementation, the independent review, the completion conditions for intermediate commits and before integration, confirmation with the person, and the division of work between hooks and CI. This is a contract on the distributed skills and the setup procedure, not a requirement whose meaning the CLI judges.

## Requirements

### REQ-core-256: Keep the decisions made during implementation

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A1, docs/decision/records/2026-10-01-change-conformance.md#A9
- verification: review
- how_to_verify: Read the plan, implement and cycle skills, and confirm that the conditions and information for recording, and the conditions for reflecting into the IR, match A1 and A9.

The kotowari skills always have a procedure by which the role that made a new decision during implementation or planning keeps the choice, the grounds and the deciding role in the `decision record`, and also reflects it into the `IR` when it changes the specification of behavior or constraints.

### REQ-core-257: A review separate from the implementing side

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A6, docs/decision/records/2026-10-01-conformance-at-integration.md#A1
- verification: review
- how_to_verify: Read the cycle and review skills, and confirm that the final conformance is not settled on the implementing side's claim alone, and that the three points checked match A6 and A1 of the decision that requires conformance only before integration.

The kotowari skills always have a procedure by which, in the final conformance check, a review separate from the implementing side confirms the validity of the grounds, the agreement in meaning between the specification and the implementation, and the delegated scope, and records what was checked and the result.

### REQ-core-258: Completion conditions for intermediate commits

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-conformance-at-integration.md#A1, docs/decision/records/2026-10-01-conformance-at-integration.md#A2
- verification: review
- how_to_verify: Read the setup procedure, the hook configuration, and the implement and cycle skills, and confirm that no procedure or hook requires a change record and a passing change conformance check for an intermediate commit, and that "--staged" is treated as an optional self-check.

The kotowari setup procedure and skills always require neither a change record nor a passing change conformance check for an intermediate commit, and do not put change conformance in pre-commit. A self-check by the implementing side with "--staged" is optional.

### REQ-core-259: Completion conditions before integration

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A2, docs/decision/records/2026-10-01-change-conformance.md#A12, docs/decision/records/2026-10-01-conformance-at-integration.md#A1
- verification: review
- how_to_verify: Read the setup procedure and the cycle skill, and confirm that the target is the whole branch, that both commands are required, and that check alone does not mean completion, matching A2, A12 and A1 of the decision that requires conformance only before integration.

The kotowari setup procedure and skills always put, in cycle's final inspection and in CI, a mechanical inspection that requires the whole branch to be checked up to the review's conformance, and require both check and changes before integration. The success of check alone is not taken as evidence that the current change has passed conformance.

### REQ-core-260: Boundary of confirmation with the person

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A9
- verification: review
- how_to_verify: Read the implement, cycle and review skills, and confirm that the conditions for autonomous updates and for returning to the person match A9, and that a change to an approved constraint is not allowed under the authority of recording work.

The kotowari skills always have a procedure that does not return to the person merely because a record is missing, and makes grounded decisions and record updates autonomously within the delegated scope. A change beyond the delegated scope, a case where the choice cannot be decided from the grounds, and an irreversible, dangerous or externally visible operation are returned to the person.

### REQ-core-261: CI decides what is compared

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A12
- verification: review
- how_to_verify: Read the CI setup example, and confirm that it states the target corresponding to the event explicitly, does not depend only on the comparison base inside the records, and runs changes with the same engine, matching A12.

The kotowari setup procedure always has a procedure in which the caller decides the comparison base and the target from the CI event, does not take what is compared only from the change record's self-report, and runs the same mechanical inspection engine as locally.

### REQ-core-262: A specification gap is not closed by recording alone

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A10, docs/decision/records/2026-10-01-change-conformance.md#A11
- verification: review
- how_to_verify: Read the review and cycle skills, and confirm that the classification and the three kinds of disposition match A10 and A11, and that a record-only treatment does not make recording the disposition unnecessary.

The kotowari review and cycle skills always have a procedure that classifies a specification gap separately from severity and fix action, and leaves its disposition in the common format. A specification gap is not treated as handled by "info" or "record_only" alone.

### REQ-core-275: Boundary of autonomous additions to the specification

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A13
- verification: review
- how_to_verify: Read the distributed skills and the setup examples, and confirm that they satisfy the inputs, responsibility boundaries and re-run conditions of source A13.

The kotowari skills and setup procedure always satisfy the following contract. implement and fixer may add to the IR, recording the grounds and the decider, only a concretization within the delegated scope that does not change the constraints of an approved requirement. A decision that changes or deletes an approved requirement, an addition that contradicts an existing choice, and a decision that cannot be made from the grounds are returned to the person. cycle itself makes no implementation or meaning decision; it delegates the recording to the implementing side and has a separate review confirm it. After the implementing side adds to the IR, check and the conformance check of the specification and implementation including that addition are always re-run. The review's findings JSON stays internal, and the implementing side and the review side each create a separate change record in the common YAML.

### REQ-core-276: Setup and re-checking conformance

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A14, docs/decision/records/2026-10-01-current-change-records.md#A2, docs/decision/records/2026-10-01-conformance-at-integration.md#A3, docs/decision/records/2026-10-01-current-change-records.md#A6, docs/decision/records/2026-10-01-current-change-records.md#A7, docs/decision/records/2026-10-01-current-change-records.md#A11
- verification: review
- how_to_verify: Read the distributed skills and the setup examples, and confirm that they satisfy the inputs, responsibility boundaries and re-run conditions of source A14.

The kotowari skills and setup procedure always satisfy the following contract. In the setup example, changes.files lists the product code, the tests, the distributed skills, and the build, hook and CI configuration, changes.records is ".kotowari/changes/*.yaml", and the fixed files implementation.yaml and review.yaml are used per role. This fixed layout is a setup convention and is not made a required path of the core. Generated files are left out only by an explicit exclude. In CI, for pull_request, the comparison base is the merge-base of the base SHA and the head SHA and the target is the head SHA; both histories are fetched and "changes --base <base> --head <target> --phase review" and check are run. The SHA for merging is not used as the target. The push setup example compares the event's before and after, and for a new branch whose before is all zeros, it stops and has the comparison base stated explicitly. For the final inspection, the implementing side's entry and the review side's entry are recreated with the branch's comparison base as base. If changes to new code, related IR or related decisions invalidate the final conformance, both final records are deleted and conformance is checked again. In a rebase that changes the comparison base, a cherry-pick, or a parallel integration, concatenating records or adopting one side alone does not complete the work; conformance is checked again for the comparison after integration. A change in the meaning of a related decision cannot be detected by the mechanical check that references exist, and is confirmed by the skills and the independent review. Re-checking conformance covers all files of the changed entries and the related IR. Ordinary document search excludes .kotowari/changes/ and reads only the needed entries. The original of a decision is in decisions and the specification is in the IR, and reason is limited to explaining the correspondence. Past records are read from the Git history when needed.


### REQ-core-277: Test changes to re-check first

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A7
- verification: review
- how_to_verify: Read the review skill and the conformance results, and confirm that the addition, expected-value change and deletion of tests with a requirement ID are checked first, and that changes to product code alone also stay in the target.

The kotowari review skill always has a procedure that re-checks first the addition, expected-value change and deletion of tests with a requirement ID, and does not drop changes to product code or helper functions from the target merely because those expected values do not change.

## Examples

```gherkin
@id=EX-core-461 @about=REQ-core-276 @source=docs/decision/records/2026-10-01-current-change-records.md#A6
Scenario: A change in the meaning of a related decision is re-checked
  Given the reference to a related decision exists, but the meaning of the choice has changed
  When the implementing side and the independent review check the current change for conformance
  Then they confirm the effect on the final conformance, delete both invalidated final records, and check conformance again

@id=EX-core-462 @about=REQ-core-276 @source=docs/decision/records/2026-10-01-current-change-records.md#A6
Scenario: Records are recreated after integrating parallel branches
  Given the fixed records of parallel branches conflict or correspond to different comparisons
  When the branches are integrated
  Then concatenating the records or adopting one side alone does not complete the work, and both roles check conformance again for the comparison after integration

```
