# In-memory input and already-read results

English | [日本語](library-inputs.ja.md)

Covers the parsing of partial IR, the groups of input needed for checks, and the lifetime and reuse of immutable read results.

## Requirements

### REQ-core-314: Reading partial parses

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A12, docs/decision/records/2026-10-03-public-crate-api.md#A38, docs/decision/records/2026-10-03-public-crate-api.md#A51, docs/decision/records/2026-10-03-public-crate-api.md#A54
- verification: unit

"ir::parse" builds an "IrDocument" from the logical path and string of a "SourceText" and from "IrOptions", which specifies the IR location. The default location is "docs/ir", and a SourceText has a path under it. It returns, read-only, the items, references and findings it could obtain. Even when an ID is missing or invalid, it does not drop the items it could obtain, and returns their 1-based line in the original document and their end line when there is one. It does not fill in IDs or positions that do not exist, and does not change the existing listing conditions of list/query.

### REQ-core-315: Not provided and empty sets

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A6, docs/decision/records/2026-10-03-public-crate-api.md#A29, docs/decision/records/2026-10-03-public-crate-api.md#A39, docs/decision/records/2026-10-03-public-crate-api.md#A49, docs/decision/records/2026-10-03-public-crate-api.md#A50
- definition: TBL-core-042
- verification: unit

"ReadModel::build" and "Inspection::build" receive the configuration and the input of TBL-core-042. When a required group is not provided they return "InputMissing", and an empty set that was provided is checked by the ordinary rules. A group whose condition is false may be left unprovided, and is not used for judgement even if provided. They keep the findings from parsing and the file lists, and reproduce the existing suppression of duplicate findings, counts and the check of overlapping locations. Completeness of individual files is the caller's responsibility, and InputMissing detects groups that are not provided. Files are not fetched automatically from in-memory input.

### REQ-core-316: Reuse of already-read results

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A18, docs/decision/records/2026-10-03-public-crate-api.md#A19, docs/decision/records/2026-10-03-public-crate-api.md#A37, docs/decision/records/2026-10-06-changes-rethink.md#A10
- verification: unit

"Project::read" returns an immutable "ReadModel", and "Project::inspect" an immutable "Inspection". list/query of the former, check/status of the latter, and references to the read results do no additional I/O. Owned values are kept until the user drops them, and are neither saved nor watched automatically. To reflect changes, read again. The six convenience operations of Project read the needed input anew on each call. Consistency at a single point in time while a set of files is being read is not guaranteed.

### REQ-core-317: Read scope by operation

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A19, docs/decision/records/2026-10-03-public-crate-api.md#A29, docs/decision/records/2026-10-03-public-crate-api.md#A37, docs/decision/records/2026-10-03-public-crate-api.md#A39, docs/decision/records/2026-10-06-changes-rethink.md#A10
- verification: unit

Even when already-read results are provided, the input for list/query does not require the guides and surfaces that only check/status use. Parsing only the IR does not require the other groups of input.

## Decision tables

### TBL-core-042: Contents and required conditions of input groups

- source: docs/decision/records/2026-10-03-public-crate-api.md#A39, docs/decision/records/2026-10-03-public-crate-api.md#A49, docs/decision/records/2026-10-03-public-crate-api.md#A50, docs/decision/records/2026-10-03-public-crate-api.md#A53, docs/decision/records/2026-10-04-overview-on-public-api.md#A2, docs/decision/records/2026-10-06-changes-rethink.md#A10

| Input group | Contents | Required in ReadInputs | Required in CheckInputs |
|---|---|---|---|
| IR | Logical paths and document contents | Always | Always |
| Decision record location | Paths and contents of the decision records and of the ordinary Markdown that sources point at | Always, whether or not referenced | Same as left |
| ADR | Paths and contents | Always, whether or not referenced | Same as left |
| Test information | The SourceText of target files, the language and whether it has a query, already-discovered tests and marks, and findings from parsing | When tests.files is not empty | Same as left |
| Guides | Paths and contents | Not required | When guides.files is not empty |
| Surface analysis results | The SourceText of target files, the language and whether it has a query, discovery results, and findings from parsing | Not required | When surface.rules is not empty |
| List of unspecified surfaces | The path and contents of the configured list | Not required | When surface.rules is not empty and surface.unspecified is specified |
| Additional finding groups | The name of the group, the number of files read, the number of marks, and the findings. core does not know their meaning, orders and counts them together with the other findings, and includes each group's counts in the result | Not required | Not required (added only when passed) |

## Examples

```gherkin
@id=EX-core-487 @about=REQ-core-314,REQ-core-317 @source=docs/decision/records/2026-10-03-public-crate-api.md#A12,docs/decision/records/2026-10-03-public-crate-api.md#A29,docs/decision/records/2026-10-03-public-crate-api.md#A38
Scenario: A parsed item without an ID can be referred to
  Given the IR string has an obtainable item lacking an ID
  When ir::parse is called without passing other input
  Then that item, its original line and the findings can be referred to
  And no ID is filled in, and the item is not dropped by the listing conditions

@id=EX-core-488 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A29,docs/decision/records/2026-10-03-public-crate-api.md#A39
Scenario: When required input is not provided, the check is not completed
  Given the group of test information required by the configuration is not provided
  When a check result is built from in-memory input
  Then InputMissing is returned, and no completed check result is returned

@id=EX-core-489 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A29,docs/decision/records/2026-10-03-public-crate-api.md#A39
Scenario: Explicitly empty test information is checked
  Given the required input is provided, and the test information is a provided empty set
  When a requirement that needs tests is checked
  Then a result containing the ordinary test correspondence findings is returned, not InputMissing

@id=EX-core-490 @about=REQ-core-316 @source=docs/decision/records/2026-10-03-public-crate-api.md#A18,docs/decision/records/2026-10-03-public-crate-api.md#A37
Scenario: Changes to files after reading do not change the kept result
  Given the original files were changed after the ReadModel was obtained
  When the same ReadModel is asked for a list and a query
  Then it returns results from the contents at the time of reading
  And only a result read again explicitly from Project reflects the changes

@id=EX-core-491 @about=REQ-core-317 @source=docs/decision/records/2026-10-03-public-crate-api.md#A19,docs/decision/records/2026-10-03-public-crate-api.md#A37,docs/decision/records/2026-10-03-public-crate-api.md#A39
Scenario: A failure to read guides does not stop the list
  Given the input needed for list can be read, but the guides cannot be read
  When a list is obtained from read of Project
  Then it returns the list without reading the guides

@id=EX-core-496 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A49,docs/decision/records/2026-10-03-public-crate-api.md#A50
Scenario: An empty discovery result from a failed parse is not mistaken for success
  Given the required test information has a finding of a failed parse and an empty discovery result
  When the in-memory input is checked
  Then the finding of the failed parse is carried over into the result

@id=EX-core-497 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A50
Scenario: Input groups with no configured targets are not required
  Given tests.files is empty and the other required input is provided
  When a ReadModel is built with the test information left unprovided
  Then InputMissing for the test information is not returned
```
