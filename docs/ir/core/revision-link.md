# Checking superseded_by links

English | [日本語](revision-link.ja.md)

Covers the form of the links in a superseded_by supplementary line of a decision record and the check that their targets exist. The scope of application (applied to every decision record, without looking at whether it has "## Context") is set by REQ-core-129 of record-form.md, and the check of the presence and names of supplementary lines is also covered by record-form.md.

## Requirements

### REQ-core-132: superseded_by links

- kind: algorithm
- source: docs/decision/records/2026-09-17-record-form.md#A4, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A18, docs/decision/records/2026-09-17-record-form.md#A19, docs/decision/records/2026-09-17-record-form.md#A20, docs/decision/records/2026-09-17-record-form.md#A21, docs/decision/records/2026-09-17-record-form.md#A22, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A29, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A36, docs/decision/records/2026-09-17-record-form.md#A37, docs/decision/records/2026-09-17-record-form.md#A38, docs/decision/records/2026-09-17-decision-log.md#A3, docs/decision/records/2026-09-17-decision-log.md#A10, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A42, docs/decision/records/2026-09-17-record-form.md#A43, docs/decision/records/2026-09-17-record-form.md#A41, docs/decision/records/2026-09-17-record-form.md#A44
- definition: TBL-core-023
- verification: unit

## Decision tables

### TBL-core-023: Judging superseded_by links

- source: docs/decision/records/2026-09-17-record-form.md#A4, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A18, docs/decision/records/2026-09-17-record-form.md#A19, docs/decision/records/2026-09-17-record-form.md#A20, docs/decision/records/2026-09-17-record-form.md#A21, docs/decision/records/2026-09-17-record-form.md#A25, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A29, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A36, docs/decision/records/2026-09-17-record-form.md#A37, docs/decision/records/2026-09-17-record-form.md#A38, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A42, docs/decision/records/2026-09-17-record-form.md#A43, docs/decision/records/2026-09-17-record-form.md#A41, docs/decision/records/2026-09-17-record-form.md#A44, docs/decision/records/2026-09-19-mutants-followup.md#A3, docs/decision/records/2026-09-19-mutants-followup.md#A8

A sequence of the form "[text](href)" in the value of a `supplementary line` whose name is "superseded_by" and whose value is not empty is a link. A link runs from a "[" of the value up to the first "]", and, when the character right after it is "(", up to the first ")"; the part between "(" and ")" is the href. The text may be empty. When there is no "]", when the character right after "]" is not "(", or when there is no ")" matching that "(", that "[" does not match this form and is skipped, and the scan continues from the character after that "[". When a link could be read, the scan continues from the character after its ")", and does not start reading another link from a "[" inside the link. The judgement is made per link, and for each invalid link one `error` of revision_link_invalid is raised with "line" set to that `supplementary line`. Even when the same href appears several times on the same line, one is raised per occurrence. The detail is the href (in step 1, the value of the line; the value has surrounding whitespace removed as in REQ-core-133, and is not the characters of the line as they are). The resolution in step 3 joins the part of the href before the first "#" literally to the directory of the record file, applies the normalization of REQ-core-110, and then resolves each ".." from left to right by removing the element just before it. When there is no element to remove, the result is outside the location.

| Step | Condition | Result |
|---|---|---|
| 1 | The value has no link at all | revision_link_invalid |
| 2 | The href has no "#", or what follows the first "#" is not of the form of a `decision number` | revision_link_invalid |
| 3 | The part before the first "#" starts with "/" or "\\", or the part before the first "#" is not empty and the result of resolving it is not inside decisions.records | revision_link_invalid |
| 4 | Taking as the target that record when the part before the first "#" is empty, and otherwise the file resulting from 3, the target is not a `decision record` that was read | revision_link_invalid |
| 5 | Neither the `decision section` entries nor the `Superseded section` of the target has a `numbered line` with that number | revision_link_invalid |
| 6 | None of 1 to 5 applies | valid |

## Examples

```gherkin
@id=EX-core-106 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A4,docs/decision/records/2026-09-17-record-form.md#A18,docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A21,docs/decision/records/2026-09-17-record-form.md#A22,docs/decision/records/2026-09-17-record-form.md#A27
Scenario: Both links of a Superseded line of an old record resolve and pass
  Given in the decision record "docs/decision/records/records.md" without a "## Context" heading, under the "- A54 " line of the Superseded section there is a "- superseded_by:" line with the two links "./2026-09-16-ir-tree.md#A21" and "./2026-09-16-ir-tree.md#A5", and the Agreements section of "docs/decision/records/2026-09-16-ir-tree.md" has lines starting with "- A21 " and "- A5 "
  When "kotowari check" is run
  Then no error of revision_link_invalid pointing at that line is raised

@id=EX-core-107 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A21,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A33
Scenario: A superseded_by without a link is an error
  Given under the "- A1 " line of the Agreements section of a decision record there is the line "- superseded_by: A24"
  When "kotowari check" is run
  Then an error of revision_link_invalid is raised whose "line" is that line and whose detail is "A24"

@id=EX-core-108 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A18,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A36
Scenario: A superseded_by pointing outside the location is an error
  Given "decisions.records" is "docs/decision/records", and under the "- A1 " line of the decision record "docs/decision/records/x.md" there is the line "- superseded_by: [A1](../../ir/example.md#A1)"
  When "kotowari check" is run
  Then an error of revision_link_invalid with detail "../../ir/example.md#A1" is raised

@id=EX-core-109 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: A superseded_by pointing at a number found only in Undecided is an error
  Given under the "- A1 " line of the decision record "docs/decision/records/a.md" there is the line "- superseded_by: [U1](./b.md#U1)", "docs/decision/records/b.md" has an Agreements section, and a line starting with "- U1 " is only in its Undecided section
  When "kotowari check" is run
  Then an error of revision_link_invalid with detail "./b.md#U1" is raised

@id=EX-core-112 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A19,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: A superseded_by pointing at a heading is an error
  Given "decisions.records" is "docs/decision/records", under the "- A1 " line of the decision record "docs/decision/records/a.md" there is the line "- superseded_by: [出典](./ir-form.md#出典)", and "docs/decision/records/ir-form.md" has the heading "## 出典"
  When "kotowari check" is run
  Then an error of revision_link_invalid with detail "./ir-form.md#出典" is raised

@id=EX-core-116 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A18,docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A36
Scenario: A link from a record in a subdirectory to a Superseded number of a record above passes
  Given "decisions.records" is "docs/decision/records", under the "- A1 " line of the decision record "docs/decision/records/sub/a.md" there is the line "- superseded_by: [A15](../records.md#A15)", and the Superseded section of "docs/decision/records/records.md" has a line starting with "- A15 "
  When "kotowari check" is run
  Then no error of revision_link_invalid pointing at that line is raised

@id=EX-core-117 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A20,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A29,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A15
Scenario: A link without a path targets the same record
  Given the Agreements section of the decision record "docs/decision/records/a.md" has the lines "- A1 " and "- A2 ", no section has a numbered line "- A9", and under the "- A1 " line there are the lines "- superseded_by: [A2](#A2)" and "- superseded_by: [A9](#A9)"
  When "kotowari check" is run
  Then no error pointing at the "[A2](#A2)" line is raised, and an error of revision_link_invalid with detail "#A9" is raised

@id=EX-core-119 @about=REQ-core-132 @source=docs/decision/records/2026-09-17-record-form.md#A4,docs/decision/records/2026-09-17-record-form.md#A25,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A41
Scenario: A superseded_by pointing at a file that is not a decision record is an error
  Given "decisions.records" is "docs/decision/records", under the "- A1 " line of the decision record "docs/decision/records/a.md" there is the line "- superseded_by: [A1](./ir-form.md#A1)", and "docs/decision/records/ir-form.md" has no heading of a decision section
  When "kotowari check" is run
  Then an error of revision_link_invalid with detail "./ir-form.md#A1" is raised
```
