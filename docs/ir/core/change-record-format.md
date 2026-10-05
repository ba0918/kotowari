# Storage format and references of change records

English | [日本語](change-record-format.ja.md)

Defines the concrete inputs, records and pass/fail conditions of change conformance. The concrete contract and the grounds of the decisions are traced through the source records.

## Requirements

### REQ-core-268: Storage format

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-current-change-records.md#A4
- verification: unit

The change conformance check always satisfies the following contract. A change record is YAML in UTF-8, and one file holds only version: 1 and the list entries. entries may be empty. An unknown version, an unknown key, a duplicate YAML key and a wrong type are a change_record_invalid error. The keys of each entry are only these 11: id, base, role, files, ir, conclusion, reason, requirements, decisions, handoff and gaps, and all are required. id is a string matching "[A-Za-z0-9][A-Za-z0-9._-]*", reason is a string that is not only whitespace, base is a full Git object ID (40 or 64 lowercase hexadecimal digits), role is implementer or reviewer, and conclusion is existing, new or deferred. id is unique across all change records read. files is a list of one or more entries, ir, requirements, decisions and gaps are lists of zero or more entries, and handoff is null or a reference string to a decision record. The storage location is given by changes.records, and records of a different version are not converted automatically. The old state key from development is also rejected as an unknown key and is not migrated automatically.

### REQ-core-269: Content identification of files and IR

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A7
- verification: unit

The change conformance check always satisfies the following contract. Each entry of files holds only the 3 keys path, before and after. path is the normalized path relative to the base, before and after are null or "sha256:" followed by 64 lowercase hexadecimal digits, and not both are null. An addition has before null, a deletion has after null, and a modification has an identifier in both. The identifier is the SHA-256 of the concatenation, in order, of Git's 6-character mode, one NUL byte and all bytes of the blob. Each entry of ir has the 2 keys path and sha256, and sha256 is in the same notation, computed by SHA-256 over all bytes of the IR file only. path does not allow an empty path, an absolute path, or a component that is .., ., or contains a backslash. A duplicate path within either list, files or ir, is an error.

### REQ-core-270: References for each conclusion

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A8
- verification: unit

The change conformance check always satisfies the following contract. requirements is a list of strings that are existing requirement IDs, and decisions and handoff are in the notation "path#decision-number" within the same repository, the same as an existing source. existing holds one or more requirements and one or more ir, and ir includes every IR that defines a referenced requirement. new holds one or more decisions, and when the specification was changed, review confirms that the related IR reflects it. deferred holds one or more decisions and a handoff that is not null. check inspects that the decision records, requirements and IR exist and that the references are consistent. review confirms the meaning of the grounds and of the deferral.

### REQ-core-271: Structure of a specification gap

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A9
- verification: unit

The change conformance check always satisfies the following contract. Each entry of gaps holds only the 3 keys category, disposition and refs. category is one of missing_spec, spec_conflict and premise_conflict, and an unclassified value is an error. disposition is one of recorded, fixed and deferred, and refs holds one or more references to decision records. recorded references the choice that was reflected and its grounds, fixed records the decision to fix and how it corresponds to existing requirements, and deferred references an explicit deferral. The conclusion of an entry with gaps is deferred if there is even one deferred, otherwise new if there is even one recorded, and otherwise existing. An entry that includes fixed holds the corresponding requirements and the IR defining them even when its conclusion is new. recorded and fixed can coexist in the same entry. review confirms that each disposition supports the choice, and that recorded is reflected in the related IR.

## Examples

```gherkin
@id=EX-core-446 @about=REQ-core-268 @source=docs/decision/records/2026-10-01-change-details.md#A6,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: Unknown version
  Given the version of a configured record is 2
  When check is run
  Then a change_record_invalid error is raised

@id=EX-core-447 @about=REQ-core-269 @source=docs/decision/records/2026-10-01-change-details.md#A7,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: A change of the execute permission only
  Given the blob is the same but the file's execute permission changed, and the record has the previous mode
  When changes is run
  Then a change_stale error is raised

@id=EX-core-448 @about=REQ-core-270 @source=docs/decision/records/2026-10-01-change-details.md#A8,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: The IR of an existing requirement is missing
  Given the file defining a requirement referenced by existing is not in the ir list
  When check is run
  Then a change_record_invalid error is raised

@id=EX-core-449 @about=REQ-core-271 @source=docs/decision/records/2026-10-01-change-details.md#A9,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: The handling of a gap is unclassified
  Given the category of gaps is unknown
  When check is run
  Then a change_record_invalid error is raised

@id=EX-core-453 @about=REQ-core-271 @source=docs/decision/records/2026-10-01-change-details.md#A9,docs/decision/records/2026-10-01-change-details.md#A12
Scenario: A new specification and a fix to an existing specification are put in the same entry
  Given the gaps of the same file have recorded and fixed, and the references to both decisions and to the needed requirements and IR are all present
  And conclusion is new
  When check is run
  Then change_record_invalid is not raised because of the mixed dispositions
```


```gherkin
@id=EX-core-456 @about=REQ-core-268 @source=docs/decision/records/2026-10-01-current-change-records.md#A4
Scenario: The old state key is not converted implicitly
  Given an entry of version 1 has the old state key
  When check is run
  Then change_record_invalid is raised as an unknown key

```
