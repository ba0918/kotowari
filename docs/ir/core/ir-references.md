# Scenario tags and ID references

English | [日本語](ir-references.ja.md)

Covers the checks of scenario tags, broken ID references, and the properties of form that kotowari does not look at.

## Requirements

### REQ-core-052: Unknown tags

- kind: event_driven
- source: docs/decision/records/records.md#A27, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A109, docs/decision/records/records.md#A143
- verification: unit

When a tag line inside a gherkin block (whether or not it is attached to a `scenario`) has a tag other than "@id", "@about" and "@source", or when a tag line has a word that does not start with "@", kotowari raises an unknown_tag `error` with that name or word as the detail.

### REQ-core-053: Missing tags

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A97
- verification: unit

When a `scenario` has no "@id" or "@about" tag, kotowari raises a missing_tag `error` with the name of the missing tag as the detail. A tag whose value is empty (nothing after "=") is treated as a missing tag.

### REQ-core-054: Broken references

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A39, docs/decision/records/records.md#A52, docs/decision/records/records.md#A28, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A67, docs/decision/records/records.md#A119, docs/decision/records/records.md#A130, docs/decision/records/records.md#A145, docs/decision/records/records.md#A152, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-multi-language-tests.md#A15
- verification: unit

When any of a "- definition:" line, an "@about" tag, a "- related:" line of a `flag record`, an `ID` enclosed in backquotes outside double quotes in a `statement` of a `requirement` or a `property` or in a gherkin step line, or a `mark` (except one in a `language with a query` that is in no `test` node's `preceding comment block`) points at an `ID` that does not exist, or when the value of "- definition:", "@about" or "- related:" is not of the `ID` form, kotowari raises one unresolved_reference `error` per occurrence.

### REQ-core-055: Does not look at EARS patterns

- kind: prohibition
- source: docs/decision/records/records.md#A42, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari must not check whether the `statement` lines of a `requirement` follow an EARS pattern.

### REQ-core-056: Does not look at the number of readings of a contradiction

- kind: prohibition
- source: docs/decision/records/records.md#A28, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari must not check whether a contradiction in a `flag record` has two or more readings.

### REQ-core-113: Lines inside a gherkin block

- kind: event_driven
- source: docs/decision/records/records.md#A109, docs/decision/records/records.md#A132, docs/decision/records/records.md#A133, docs/decision/records/records.md#A155, docs/decision/records/2026-09-24-review3-gaps.md#A1
- verification: unit

Looking at the lines inside a gherkin `code block` with leading whitespace removed, when there is a line that is none of a tag line (starting with "@"), a "Scenario:" line, a step line (a line where "Given", "When", "Then", "And" or "But" is followed by one or more half-width spaces), a comment starting with "#", or a blank line (including "Feature:", "Background:", "Scenario Outline:", "Examples:" and data table lines), kotowari raises an invalid_gherkin_line `error` with the characters of the line as the detail. A step line with no "Scenario:" line before it in the same block, and a tag line not directly followed by "Scenario:", also raise the same `error`. However, step lines that follow on without a blank line or a comment in between are included in the `error` of the first step line. A step line that has a "Scenario:" line before it becomes a step of that scenario even when blank lines, comments or erroneous lines come in between. Tag lines are attached only when they are on the line just before "Scenario:", and are not attached when another line comes between.

### REQ-core-114: ID definitions and an @id that does not fit the form

- kind: event_driven
- source: docs/decision/records/records.md#A110, docs/decision/records/records.md#A139, docs/decision/records/records.md#A151
- verification: unit

When the value of "@id" is not of the form of an `ID` of EX, kotowari raises an invalid_id `error` with the value as the detail and the tag line as "line", does not raise missing_tag for "@id" (it does raise missing_tag when "@about" is missing), and makes the detail of missing_source for that `scenario` the characters of the "Scenario:" line. Only the `ID` values of headings that fit the form and the values of "@id" that fit the form are counted in the set of existing `ID` values.

### REQ-core-124: The form of an ID

- kind: ubiquitous
- source: docs/decision/records/records.md#A52, docs/decision/records/2026-09-16-ir-tree.md#A6, docs/decision/records/2026-09-16-ir-tree.md#A11, docs/decision/records/ir-form.md#ID, docs/decision/records/2026-09-22-id-namespace.md#A1, docs/decision/records/2026-09-22-id-namespace.md#A2
- verification: unit

An `ID` always has the form of one of "REQ-", "TBL-", "PROP-", "EX-" and "FLAG-", followed by an optional name and "-", then a number of three or more digits, which does not start with "0" when it has four or more digits. The name starts with a lowercase English letter, and from the second character on consists only of lowercase English letters, digits and "-". The "nnn" written in the forms of `item` headings and tags stands for this number. A heading that does not fit this form is handled as in REQ-core-043, an "@id" that does not fit is handled as in REQ-core-114, and an `ID` inside a `mark` that does not fit is handled as in the check of each `mark`.

### REQ-core-167: Agreement of the name of an ID with its location

- kind: event_driven
- source: docs/decision/records/2026-09-22-id-namespace.md#A3, docs/decision/records/2026-09-22-id-namespace.md#A5
- verification: unit

When the `ID` of a heading or the value of "@id" has a name, and that name differs from the first level of the document's path relative to the location, kotowari raises an id_domain_mismatch `error` with the `ID` as the detail. When the document is directly under the IR location and has no first level, it raises the same `error` if there is a name.

## Examples

```gherkin
@id=EX-core-283 @about=REQ-core-113 @source=docs/decision/records/2026-09-24-review3-gaps.md#A1
Scenario: Blank lines and comments inside a scenario do not cut the steps
  Given a gherkin block has a `scenario` with three step lines after the "Scenario:" line, with blank lines and comments in between
  When "kotowari check" is run
  Then invalid_gherkin_line is not raised, and the `scenario` has three steps

@id=EX-core-009 @about=REQ-core-052 @source=docs/decision/records/records.md#A27,docs/decision/records/ir-form.md#検査の種類
Scenario: A discontinued tag is an error
  Given a `scenario` has the tag "@requirement=REQ-001"
  When "kotowari check" is run
  Then one unknown_tag error is raised

@id=EX-core-010 @about=REQ-core-054 @source=docs/decision/records/records.md#A52,docs/decision/records/records.md#A39,docs/decision/records/ir-form.md#検査の種類
Scenario: An about that points at a missing requirement is a broken reference
  Given the "@about" of a `scenario` points at "REQ-999", and "REQ-999" exists nowhere
  When "kotowari check" is run
  Then an unresolved_reference error with the detail "REQ-999" is raised

@id=EX-core-028 @about=REQ-core-124 @source=docs/decision/records/2026-09-16-ir-tree.md#A6,docs/decision/records/2026-09-16-ir-tree.md#A11
Scenario: A four-digit ID passes, while four digits starting with 0 and two digits or fewer do not
  Given there are three headings: "### REQ-1000: 名前", "### REQ-0001: 名前" and "### REQ-1: 名前"
  When "kotowari check" is run
  Then "REQ-1000" is read as a `requirement`, and an unknown_heading error is raised on the headings of "REQ-0001" and "REQ-1"

@id=EX-core-029 @about=REQ-core-124 @source=docs/decision/records/2026-09-16-ir-tree.md#A6,docs/decision/records/2026-09-16-ir-tree.md#A11
Scenario: IDs in tags and marks are looked at with the same form
  Given there are the tag "@id=EX-1000" and the tag "@id=EX-0001", and a test has the `mark` "@kotowari[REQ-1000]"
  When "kotowari check" is run
  Then "EX-1000" becomes the `ID` of a `scenario`, an invalid_id error is raised on the tag "EX-0001", and "REQ-1000" in the `mark` is compared as the `ID` of a `requirement`

@id=EX-core-043 @about=REQ-core-124 @source=docs/decision/records/2026-09-22-id-namespace.md#A1,docs/decision/records/2026-09-22-id-namespace.md#A2
Scenario: IDs with a name and IDs without a name both pass
  Given "core/a.md" has three headings: "### REQ-core-001: 名前", "### REQ-002: 名前" and "### REQ-Core-003: 名前"
  When "kotowari check" is run
  Then "REQ-core-001" and "REQ-002" are read as each `requirement`, and an unknown_heading error is raised on the heading of "REQ-Core-003"

@id=EX-core-044 @about=REQ-core-167 @source=docs/decision/records/2026-09-22-id-namespace.md#A3,docs/decision/records/2026-09-22-id-namespace.md#A5
Scenario: A name that differs from the first level of the location is an error
  Given "core/a.md" has the heading "### REQ-schema-001: 名前", and "b.md" has the heading "### REQ-core-002: 名前"
  When "kotowari check" is run
  Then two id_domain_mismatch errors are raised, with the details "REQ-schema-001" and "REQ-core-002"
```
