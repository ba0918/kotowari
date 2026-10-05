# Conformance inspection of changes

English | [日本語](changes.ja.md)

Covers the responsibility of the change conformance command to inspect the correspondence and freshness between the given Git changes and the change records. The concrete arguments and targets are defined in changes-inputs.md, and the output of findings in changes-results.md.

## Requirements

### REQ-core-240: Dedicated command

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A2
- verification: unit

kotowari always has a dedicated command "kotowari changes" that inspects changes for missing and stale conformance.

### REQ-core-241: Independent enumeration of changes

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A3
- verification: unit

The change conformance command always takes the Git changes from the comparison base and the target given by the caller, and enumerates, per file, the additions, modifications and deletions of the configured files to inspect. It must not build the set of files to inspect only from the list of target files in the change records.

### REQ-core-242: Target changes missing conformance

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A3, docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A6
- verification: unit

The change conformance command always detects and reports, as missing conformance, each enumerated target change that corresponds to no entry of any change record. Even when several files are checked in one entry, the changes of every corresponding file are stated explicitly as conformance targets.

### REQ-core-243: Freshness of code and tests

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A6
- verification: unit

The change conformance command always detects and reports, as an entry that needs conformance checked again, an entry whose identifier of the target contents in the change record does not match the identifier of the inspected contents.

### REQ-core-244: Freshness of the related specification

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A6
- verification: unit

The change conformance command always detects and reports, as an entry that needs conformance checked again, an entry whose identifier of the contents of the related `IR` in the change record does not match the identifier of the contents of the inspected `IR`.

### REQ-core-245: The target is not narrowed by test differences alone

- kind: prohibition
- source: docs/decision/records/2026-10-01-change-conformance.md#A7
- verification: unit

The change conformance command must not drop changes to product code or helper functions included in the configured files to inspect from the inspection merely because the expected values of a `test` did not change.

### REQ-core-246: Staged target

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-conformance-at-integration.md#A2
- verification: unit

The change conformance command always, in a "--staged" inspection, compares HEAD with the staged contents, and does not count unstaged changes in the working tree as part of the commit.

### REQ-core-247: Target of the final inspection

- kind: prohibition
- source: docs/decision/records/2026-10-01-conformance-at-integration.md#A1, docs/decision/records/2026-10-01-change-conformance.md#A12
- verification: unit

The change conformance command always, in cycle's final inspection and in CI, inspects the changes of the whole target branch from the comparison base given by the caller. It must not make only the last single commit the target of the final inspection.

## Examples

```gherkin
@id=EX-core-430 @about=REQ-core-241,REQ-core-242 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: Changes not declared are picked up
  Given two configured target files have changes, and the change record targets only one of them
  When the change conformance command inspects that comparison base and target
  Then missing conformance is detected and reported for the unrecorded change

@id=EX-core-431 @about=REQ-core-242 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A4
Scenario: Several files are grouped per intent
  Given two target changes are both stated explicitly in one change record entry for the same change intent
  When the change conformance command inspects that comparison base and target
  Then no missing conformance is reported for these two

@id=EX-core-432 @about=REQ-core-243 @source=docs/decision/records/2026-10-01-change-conformance.md#A4,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: The code changes after the conformance check
  Given a change record holds the identifier of a target file's previous contents
  And the inspected contents changed after the conformance check
  When the change conformance command inspects that target
  Then that entry is detected and reported as an entry that needs conformance checked again

@id=EX-core-433 @about=REQ-core-244 @source=docs/decision/records/2026-10-01-change-conformance.md#A4,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: The specification changes while the code stays the same
  Given the target file is the same as at the conformance check, and the contents of the related `IR` changed after it
  When the change conformance command inspects that target
  Then that entry is detected and reported as an entry that needs conformance checked again

@id=EX-core-434 @about=REQ-core-241,REQ-core-245 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A7
Scenario: A product code change that does not change tests
  Given configured product code changed, and the expected values of the `test` did not change
  When the change conformance command inspects that comparison base and target
  Then the product code change is included in the conformance targets

@id=EX-core-435 @about=REQ-core-246 @source=docs/decision/records/2026-10-01-conformance-at-integration.md#A2
Scenario: What is committed is separated from the working tree
  Given a file has staged changes and additional unstaged changes
  When change conformance is run with "--staged"
  Then the staged contents are inspected, and the additional changes are not counted as part of the commit

@id=EX-core-436 @about=REQ-core-247,REQ-core-242 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A6,docs/decision/records/2026-10-01-change-conformance.md#A12,docs/decision/records/2026-10-01-conformance-at-integration.md#A1
Scenario: Unchecked changes in an earlier commit
  Given an earlier commit of the branch has a target change not checked for conformance, and the last commit has only change records
  When the whole branch is given the final inspection from the given comparison base
  Then missing conformance is detected and reported for the unchecked change of the earlier commit

```

