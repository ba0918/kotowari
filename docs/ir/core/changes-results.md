# Conformance phases and output

English | [日本語](changes-results.ja.md)

Defines the concrete inputs, records and pass/fail conditions of change conformance. The concrete contract and the grounds of the decisions are traced through the source records.

## Requirements

### REQ-core-272: Pass/fail per phase

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-current-change-records.md#A5
- verification: unit

The change conformance check always satisfies the following contract. The implementation phase requires, for each file, an implementer entry corresponding to the same comparison base and to its before and after; the review phase requires both an implementer entry and a reviewer entry. The unit of coverage is each file, and the two roles need not group files the same way or use the same entry id. The set of related IR of a reviewer entry includes all the related IR of the corresponding implementer entry, and may include additional IR. A reviewer entry that does not include the required related IR does not satisfy that file's review coverage, and change_uncovered is raised. The conclusion deferred can pass in the implementation phase when its format and references are complete, but does not pass in the review phase, as a change_deferred error. The same change appearing in several entries is allowed in itself, but if entries corresponding to the same base and the same pair of a file's before and after have different conclusions, it is a change_conclusion_conflict error. Missing conformance and the count of covered are per file, and the freshness inspection is per entry; if even one in-target file or related IR of an entry containing target changes does not match, none of that entry's files is counted as covered. To formally adopt a deferral, the allowance is stated explicitly in the decision and the IR, and conformance is checked again as within the existing specification or as a new decision. The role in a record is not a proof of independence; the running of a separate review is guaranteed by the skills.

### REQ-core-273: Current records and inspection per comparison base

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-current-change-records.md#A5, docs/decision/records/2026-10-01-current-change-records.md#A8
- verification: unit

The change conformance check always inspects the format of all change records and the existence and consistency of the references of all entries. Coverage and freshness are inspected only for entries that match the given base. check and status also inspect the format and references of all entries, and read under hidden directories that changes.records names explicitly in a path component. Entries of another base are not used for the current coverage, and the staleness of their contents alone is not an error. A missing corresponding entry is change_uncovered, a mismatch of before or after is change_stale, and a mismatch of the IR is change_ir_stale. When an entry matching the comparison base has a file that is no longer a target, that file's coverage and freshness are not inspected. There is no state category that accumulates past completed entries.

### REQ-core-274: Output and stopping

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A12
- verification: unit

The change conformance check always satisfies the following contract. The JSON of changes has the 6 keys base, target, phase, files, covered and findings. base is the full resolved object ID, target is the commit's object ID or "index", phase is the given value, files is the number of enumerated target changes, and covered is the number of changes whose records and freshness for that phase are complete and whose conclusion conforms. The shape of findings and the text notation of one finding are the same as check. A static inconsistency of format, conclusion, required information or references is change_record_invalid. Every new finding here has severity error, path is the relative path of the change record, line is null, and detail shows the id of the related entry and the target path. change_record_invalid does not require an id or path that cannot be obtained; an invalid whole file outputs "file: " and an explanation, and an invalid entry outputs "entry ", the 0-based position within entries, ": " and an explanation. The id is included in the explanation only when it can be obtained. However, change_uncovered has the target file's path, line null, and the required role as detail. Findings are sorted by the byte order of path, kind and detail. Exit 0 with 0 errors, exit 1 with errors, and exit 2 when execution or input stops. The stop reason for reading from Git is "git error". check and status count the errors of the static inspection of records in the existing findings, and add no new top-level count key. complete in status does not guarantee the final conformance of the difference.

## Examples

```gherkin
@id=EX-core-450 @about=REQ-core-272 @source=docs/decision/records/2026-10-01-change-details.md#A10,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: Deferral at the final inspection
  Given the records of both roles are present, but the conclusion is deferred
  When changes is run in the review phase
  Then a change_deferred error is raised and it exits with 1

@id=EX-core-451 @about=REQ-core-273 @source=docs/decision/records/2026-10-01-current-change-records.md#A5
Scenario: Another comparison base is not used as current evidence
  Given for the current target changes there are only records of another base, with correct format and references
  When changes is run with the current base
  Then the staleness of the other base's contents alone is not an error, and the current target changes are change_uncovered

@id=EX-core-452 @about=REQ-core-274 @source=docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: The target and the phase are output
  Given one target change has conforming records from both roles
  When changes is run in the review phase with JSON
  Then the given base, target and phase, files 1, covered 1 and empty findings are output, and it exits with 0

@id=EX-core-454 @about=REQ-core-272 @source=docs/decision/records/2026-10-01-change-details.md#A10,docs/decision/records/2026-10-01-change-details.md#A12
Scenario: The two roles group files differently
  Given one implementer entry has 2 files, and the reviewer puts each file in a separate entry
  And they correspond to the same base and contents, have the same conclusion and complete freshness, and the reviewer has checked all the related IR of the implementer
  When changes is run in the review phase
  Then it succeeds with files 2 and covered 2

@id=EX-core-455 @about=REQ-core-273 @source=docs/decision/records/2026-10-01-current-change-records.md#A5
Scenario: A reference target of a current change record is missing
  Given the entry's format is correct, but the referenced requirement does not exist in the target's IR
  When check is run
  Then a change_record_invalid error is raised
```


```gherkin
@id=EX-core-460 @about=REQ-core-273,REQ-core-242 @source=docs/decision/records/2026-10-01-current-change-records.md#A6
Scenario: Records with a changed comparison base do not pass the current change
  Given after a rebase the comparison base differs from the base of the records
  And there is no record whose base matches the current target changes
  When changes is run in the review phase
  Then change_uncovered is raised

```
