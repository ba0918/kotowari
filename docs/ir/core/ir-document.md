# Reading documents and whole-document checks

English | [日本語](ir-document.ja.md)

Covers how documents of the IR are chosen, the title and scope lines, how lines are counted, and the notices on the number of lines and of requirements.

## Requirements

### REQ-core-033: Documents that are read

- kind: ubiquitous
- source: docs/decision/records/records.md#A32, docs/decision/records/records.md#A102, docs/decision/records/records.md#A165, docs/decision/records/2026-09-16-ir-tree.md#A1, docs/decision/records/2026-09-16-ir-tree.md#A8, docs/decision/records/2026-09-16-ir-tree.md#A13, docs/decision/records/2026-10-05-localization.md#A22, docs/decision/records/2026-10-05-localization.md#A11, docs/decision/records/2026-10-05-localization.md#A15, docs/decision/records/2026-10-05-localization.md#A21
- verification: unit

kotowari always traverses the directories under the `IR` location with no limit on depth, reads only files whose extension is a lowercase ".md", and does not read ".MD" documents or entries that are neither directories nor regular files (sockets, named pipes, devices) (`exclusion`). It does not traverse hidden directories or symbolic links to directories at any depth (`exclusion`), and raises no `finding` for an empty directory. In every directory, "CONTEXT.md" is a `glossary` and "FLAGS.md" is a `flag record`, and when the `language list` has two or more languages, each `side` of that `pair` ("CONTEXT.<tag>.md", "FLAGS.<tag>.md") is likewise the `glossary` and the `flag record` of that language (REQ-core-337). Symbolic links to files are read. When there is an entry whose type cannot be obtained, kotowari does a `stop` on the grounds of an unreadable file.

### REQ-core-034: No title

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A56
- verification: unit

When a document of the `IR` has no `title`, kotowari raises a missing_title `error`.

### REQ-core-035: More than one title

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/2026-09-23-ir-engine-gaps.md#A7, docs/decision/records/2026-09-23-ir-engine-gaps.md#A17
- verification: unit

When a document of the `IR` has two or more of each `title`, kotowari raises one multiple_titles `error` for each `title` from the second on.

### REQ-core-036: No scope lines

- kind: event_driven
- source: docs/decision/records/records.md#A30, docs/decision/records/records.md#A41, docs/decision/records/records.md#A55, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#検査の種類
- verification: unit

When a `topic document` has not a single line of its `scope`, kotowari raises a missing_scope `error`.

### REQ-core-037: How lines are counted

- kind: algorithm
- source: docs/decision/records/records.md#A33, docs/decision/records/2026-09-23-mutants-gaps.md#A1
- definition: TBL-core-010
- verification: unit

### REQ-core-038: Upper limit on lines

- kind: event_driven
- source: docs/decision/records/records.md#A17, docs/decision/records/records.md#A29, docs/decision/records/records.md#A41, docs/decision/records/records.md#A56, docs/decision/records/records.md#A47, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2
- verification: unit

When the number of lines of a document of the `IR` exceeds "limits.lines", kotowari raises a too_many_lines `notice`.

### REQ-core-039: Upper limit on requirements

- kind: event_driven
- source: docs/decision/records/records.md#A17, docs/decision/records/records.md#A29, docs/decision/records/records.md#A41, docs/decision/records/records.md#A47, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-16-notice.md#A2
- verification: unit

When the number of each `requirement` of a `topic document` exceeds "limits.requirements", kotowari raises a too_many_requirements `notice`.

### REQ-core-040: Inside code blocks

- kind: ubiquitous
- source: docs/decision/records/records.md#A52, docs/decision/records/records.md#A88, docs/decision/records/records.md#A108, docs/decision/records/records.md#A140
- verification: unit

kotowari always removes the inside of a `code block` from what it checks (`exclusion`). An unclosed `code block` is removed even when it is gherkin. The lines inside a closed gherkin block are subject to the checks of `scenario` tags, of each `term` and of vague words, and are not subject to the check of document-name references.

### REQ-core-041: Does not look at the contents of the scope or the separation of responsibilities

- kind: prohibition
- source: docs/decision/records/records.md#A17, docs/decision/records/records.md#A30, docs/decision/records/2026-09-24-kotowari-dir.md#A2
- verification: review
- how_to_verify: Confirm that "crates/kotowari-core/schemas/ir.yaml" declares only a lower bound on the number of occurrences for the scope statements, and that "check_documents" in "crates/kotowari-core/src/ir.rs" does not check the contents or the number of lines of scope_lines

kotowari must not check the contents or the number of lines of the `scope`, nor judge the separation of responsibilities of a document.

## Decision tables

### TBL-core-010: How lines are counted

- source: docs/decision/records/records.md#A33, docs/decision/records/records.md#A129, docs/decision/records/2026-09-23-mutants-gaps.md#A1, docs/decision/records/2026-09-24-review3-gaps.md#A3

This way of counting is used not only for documents of the `IR` but for every file for which kotowari outputs line numbers (each `decision record`, ADRs, each `test file`, and the sources of each `mutation outcome`).

| Case | How it is counted |
|---|---|
| "\n" | The end of one line |
| "\r\n" | The end of one line (counted as one line) |
| A lone "\r" | The end of one line |
| The last line has no line break | That line is counted as one line too |
| A document with empty contents | Counted as 0 lines (it has no title, so missing_title is raised) |

## Examples

```gherkin
@id=EX-core-006 @about=REQ-core-036 @source=docs/decision/records/records.md#A41,docs/decision/records/ir-form.md#検査の種類
Scenario: A glossary may have no scope lines
  Given a `glossary` has only a `title` and a table
  When "kotowari check" is run
  Then no missing_scope error is raised for the `glossary`

@id=EX-core-007 @about=REQ-core-037 @source=docs/decision/records/records.md#A33
Scenario: Differences in line breaks do not change the number of lines
  Given there is a document written as "a\r\nb"
  When the lines of that document are counted
  Then the number of lines is 2

@id=EX-core-281 @about=REQ-core-037 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A1,docs/decision/records/records.md#A33
Scenario: A lone CR is also counted as the end of one line
  Given there are a document written as "a\rb" and a document written as "a\rb\r\nc"
  When the lines of each document are counted
  Then the former has 2 lines and the latter has 3 lines

@id=EX-core-282 @about=REQ-core-037 @source=docs/decision/records/2026-09-23-mutants-gaps.md#A1
Scenario: A finding on a line after a lone CR has the line number after the split
  Given there is a document whose `title` on line 1, scope line on line 2, "## Requirements" on line 3 and "### foo" on line 4 are separated by lone "\r"
  When "kotowari check" is run
  Then an unknown_heading error with line 4 is raised for that document

@id=EX-core-020 @about=REQ-core-033 @source=docs/decision/records/2026-09-16-ir-tree.md#A1,docs/decision/records/2026-09-16-ir-tree.md#A13,docs/decision/records/records.md#A21
Scenario: Documents in deep directories are read too
  Given "docs/ir/network/dns/timeout.md" has one `requirement` whose verification is "unit" and that has no `mark`, and "docs/ir/network/empty/" is an empty directory
  When "kotowari check" is run
  Then a requirement_without_test error whose path is "docs/ir/network/dns/timeout.md" is raised, and no `finding` is raised for the empty directory

@id=EX-core-030 @about=REQ-core-033 @source=docs/decision/records/2026-09-16-ir-tree.md#A13
Scenario: Even inside deep directories, hidden directories and symbolic links to directories are not traversed
  Given there is "docs/ir/network/.draft/a.md", and "docs/ir/network/link" is a symbolic link to a directory pointing at "docs/ir/"
  When "kotowari check" is run
  Then "docs/ir/network/.draft/a.md" and the documents under "docs/ir/network/link/" are not read, and neither a `finding` nor a `stop` occurs

@id=EX-core-021 @about=REQ-core-033 @source=docs/decision/records/2026-09-16-ir-tree.md#A8,docs/decision/records/records.md#A41,docs/decision/records/records.md#A56
Scenario: CONTEXT.md and FLAGS.md in subdirectories are also a glossary and a flag record
  Given there are "docs/ir/network/CONTEXT.md" and "docs/ir/network/FLAGS.md"
  When "kotowari check" is run
  Then no missing_scope error is raised for either
```
