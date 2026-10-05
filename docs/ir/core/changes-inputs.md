# Inputs and targets of change conformance

English | [日本語](changes-inputs.ja.md)

Defines the concrete inputs, records and pass/fail conditions of change conformance. The concrete contract and the grounds of the decisions are traced through the source records.

## Requirements

### REQ-core-263: What is compared, and the phase

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A1
- verification: unit

The change conformance check always satisfies the following contract. changes requires "--base REV", exactly one of "--head REV" and "--staged", and "--phase implementation" or "--phase review". No default comparison or phase is set. --staged is accepted only in combination with --base HEAD and implementation. The existing --config, --format, --help and --version are kept, and passing the dedicated options to another command stops with an argument error. The precedence rules for --help and --version are the same as the existing ones.

### REQ-core-264: Configuration

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A2
- verification: unit

The change conformance check always satisfies the following contract. An optional changes is added to the configuration, with the keys files, exclude and records. Each value is a list of strings that are globs relative to the base directory, and the default of exclude is empty. When changes is written, files and records are stated explicitly and neither is empty. An unknown key, null, an empty string and an invalid glob stop with a configuration error. When changes is omitted, check and status do not inspect change records, and the changes command stops with a configuration error. The glob syntax is the same as the existing tests.files.

### REQ-core-265: Boundary of reading from Git

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A3
- verification: unit

The change conformance check always satisfies the following contract. Instead of the usual configuration search, changes finds the root of the Git working tree from where it is started, and makes that root the base directory. The comparison base and target REVs are resolved to commits, and the base of a change record must match the full object ID of the resolved comparison base. The target of --head is that commit's tree, and that of --staged is the contents of the index; the target files, the related IR, the decision records, the change records and the configuration file are all read from the same target. changes' --config is a path relative to the Git root, and only a configuration file that exists in the target is accepted. When omitted, the target's ".kotowari/config.yaml" is read. For changes only, this rule replaces the existing rule that interprets the configuration path from the current directory and the existence check in the working tree. The starting location is the Git root or below it, and every path written in the configuration is based on the Git root. Uncommitted contents of the working tree are not used to fill in. A missing Git, an unreadable history, target or configuration, and a conflicted index stop. check and status read the working tree as before.

### REQ-core-266: Enumerating the targets

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A4
- verification: unit

The change conformance check always satisfies the following contract. The difference between the Git comparison base's tree and the target's tree is enumerated as additions, modifications and deletions, without rename detection. A move is a deletion of the old path and an addition of the new path, and a type change is also treated as a difference. Paths matched by files and not matched by exclude are inspected. Paths in hidden directories contained in Git are also included when a glob matches them. Untracked and unstaged changes are outside the target. A symlink, a submodule or a non-UTF-8 path chosen as a target stops as an unsupported input. A change of a file's execute permission is also included in the target changes, and the identifier includes Git's mode and all bytes of the blob.

### REQ-core-267: The records themselves and what is excluded

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A5
- verification: unit

The change conformance check always satisfies the following contract. The change records matched by records are removed from the enumeration of target changes. The IR location, the locations of decisions.records and decisions.adr, and the configuration file in use are also removed from the enumeration of conformance targets. The related IR and decision records are handled by the reference inspection, and the IR is also inspected for freshness. A narrowing of the target range by changes to files and exclude is confirmed by review. When there are 0 target changes, changes can succeed, but the format and references of the configured change records are still inspected.

## Examples

```gherkin
@id=EX-core-441 @about=REQ-core-263 @source=docs/decision/records/2026-10-01-change-details.md#A1,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: No phase is given
  Given only the comparison base and the target are given
  When changes is run
  Then it exits with 2 on an argument error

@id=EX-core-442 @about=REQ-core-264 @source=docs/decision/records/2026-10-01-change-details.md#A2,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: A project that has not set it up
  Given the configuration has no changes
  When changes is run
  Then it exits with 2 on a configuration error

@id=EX-core-443 @about=REQ-core-265 @source=docs/decision/records/2026-10-01-change-details.md#A3,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: Unstaged records are not used to fill in
  Given the index has target changes, and the change record exists only in the working tree
  When changes is run on the staged contents
  Then the record in the working tree is not taken as passing conformance

@id=EX-core-444 @about=REQ-core-266 @source=docs/decision/records/2026-10-01-change-details.md#A4,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: A move is a deletion and an addition
  Given a regular file in the target has moved to another path
  When changes is run
  Then the deletion of the old path and the addition of the new path are the targets

@id=EX-core-445 @about=REQ-core-267 @source=docs/decision/records/2026-10-01-change-details.md#A5,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: Avoiding a cycle through the records themselves
  Given only the configured record files were added, and their format and references are correct
  When changes is run
  Then it succeeds with 0 target changes
```


```gherkin
@id=EX-core-458 @about=REQ-core-265,REQ-core-267 @source=docs/decision/records/2026-10-01-conformance-at-integration.md#A3,docs/decision/records/2026-10-01-change-details.md#A5
Scenario: Updating the fixed records does not make self-conformance cyclic
  Given only the configured records in .kotowari/changes/ are updated or deleted, and the remaining records are correct
  When changes is run on that index
  Then it succeeds with 0 target changes

@id=EX-core-459 @about=REQ-core-265,REQ-core-273 @source=docs/decision/records/2026-10-01-current-change-records.md#A1
Scenario: Past commits can still be checked after the current records are replaced
  Given the current fixed records were replaced, and past entries are not in the current files
  And the past target commit has the configuration and records of that time
  When the past comparison base and target commit are inspected with changes
  Then they can be inspected with the records of that time, and the current records are not used to fill in

```
