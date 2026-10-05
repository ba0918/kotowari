# Change records and the dispositions of specification gaps

English | [日本語](change-records.ja.md)

Covers the information a change record holds, the static inspection of the ordinary check, the dispositions of a specification gap, and the limits of what mechanical inspection guarantees. The storage format is defined in change-record-format.md, and the inspection of the current state in changes-results.md.

## Requirements

### REQ-core-248: Information in a change record

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A5, docs/decision/records/2026-10-01-change-conformance.md#A11
- verification: unit

A change record always holds the comparison base, the target files, the identifiers of the target contents, the related `IR` and the identifiers of its contents, the conclusion, the reason, references to the `decision record` according to the conclusion, and the role that did the conformance check. Several files can be put together into one entry per change intent.

### REQ-core-249: Static inspection by the ordinary check

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-current-change-records.md#A5, docs/decision/records/2026-10-01-current-change-records.md#A8
- verification: unit

kotowari always, in "kotowari check", inspects the format of every configured change record and each entry's references to the `IR`, to the `decision record` and to the deferral target, and reports missing required information and missing references. It also reads records inside a hidden directory that changes.records names explicitly in a path component, and does not read under hidden directories not named. This static inspection does not require a Git comparison base.

### REQ-core-250: The conclusion "within the existing specification"

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A5
- verification: unit

A change record always holds, for an entry whose conclusion is within the existing specification, references to the corresponding `requirement` and the reason the change falls within that range.

### REQ-core-251: The conclusion "a new decision"

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A1, docs/decision/records/2026-10-01-change-conformance.md#A5
- verification: unit

A change record always holds, for an entry whose conclusion is a new decision, a reference to the `decision record` stating the choice and its grounds. A choice that changes the specification of behavior or constraints also corresponds to the `IR` after the change.

### REQ-core-252: The conclusion "deferred"

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A5, docs/decision/records/2026-10-01-change-conformance.md#A10
- verification: unit

A change record always holds, for an entry whose conclusion is deferred, the reason and the handoff target. When a specification gap is deferred, the deferral is stated explicitly in the `decision record`.

### REQ-core-253: The caller writes the records

- kind: prohibition
- source: docs/decision/records/2026-10-01-current-change-records.md#A1
- verification: unit

kotowari must not write change records or the dispositions of specification gaps anywhere other than standard output and standard error. They are written by cycle or another caller in the common format, and only the change records needed for the current change are kept in the repository. Past records no longer needed are deleted on update, and past evidence is checked from the Git history.

### REQ-core-254: Limits of the meaning of mechanical inspection

- kind: prohibition
- source: docs/decision/records/2026-10-01-change-conformance.md#A6, docs/decision/records/2026-10-01-change-conformance.md#P1
- verification: unit

kotowari must not present the passing of the inspection of a change record's format, correspondence and freshness as a machine proof that the grounds support the choice, that the specification and the implementation agree in meaning, or that the change is within the delegated scope.

### REQ-core-255: Classification and dispositions of specification gaps

- kind: prohibition
- source: docs/decision/records/2026-10-01-change-conformance.md#A10, docs/decision/records/2026-10-01-change-conformance.md#A11
- verification: unit

A record of a specification gap always holds a classification separate from a finding's severity and fix action, and one of these dispositions: reflection in the `decision record` and the `IR`, a fix of the code to match the specification, or an explicit deferral in the `decision record`. A treatment of "info" or "record_only" alone must not be taken as evidence that the gap was handled.

## Examples

```gherkin
@id=EX-core-437 @about=REQ-core-249,REQ-core-250 @source=docs/decision/records/2026-10-01-change-conformance.md#A2,docs/decision/records/2026-10-01-change-conformance.md#A5,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: No reference to the existing specification
  Given the conclusion of a configured change record is within the existing specification, and it has a reason but no reference to the corresponding `requirement`
  When "kotowari check" is run
  Then the missing reference is detected and reported

@id=EX-core-438 @about=REQ-core-249,REQ-core-251 @source=docs/decision/records/2026-10-01-change-conformance.md#A1,docs/decision/records/2026-10-01-change-conformance.md#A2,docs/decision/records/2026-10-01-change-conformance.md#A5,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: No record points to the grounds of a new decision
  Given the conclusion of a configured change record is a new decision, and it has no reference to a `decision record`
  When "kotowari check" is run
  Then the missing reference is detected and reported

@id=EX-core-439 @about=REQ-core-249 @source=docs/decision/records/2026-10-01-change-conformance.md#A2
Scenario: Static inspection without a Git comparison base
  Given the format and references of the configured change records can be inspected, and no Git comparison base is specified
  When "kotowari check" is run
  Then it does not require a Git comparison base, and inspects the format and references of the change records

@id=EX-core-440 @about=REQ-core-249,REQ-core-252 @source=docs/decision/records/2026-10-01-change-conformance.md#A2,docs/decision/records/2026-10-01-change-conformance.md#A5,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: Missing deferral target
  Given the conclusion of a configured change record is deferred, and it has a reason but no handoff target
  When "kotowari check" is run
  Then the missing reference is detected and reported

```



```gherkin
@id=EX-core-457 @about=REQ-core-249,REQ-core-019 @source=docs/decision/records/2026-10-01-current-change-records.md#A8
Scenario: Hidden records named explicitly are inspected statically
  Given changes.records specifies .kotowari/changes/*.yaml
  And under it is a record with an invalid format or a broken reference
  When check and status are run
  Then change_record_invalid is raised in both
  And the unnamed .hidden directory is not inspected

```
