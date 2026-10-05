# Checking the supplementary lines of decision records

English | [日本語](record-form.ja.md)

Covers how the numbered lines and supplementary lines of a decision record are read (applied to every decision record), and, in decision records that have a "## Context" heading, the check of the presence and names of supplementary lines. The check of superseded_by links is covered by revision-link.md.

## Requirements

### REQ-core-129: What the form check applies to

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A1, docs/decision/records/2026-09-17-record-form.md#A22, docs/decision/records/2026-09-17-record-form.md#A26, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A39, docs/decision/records/2026-09-17-record-form.md#A41, docs/decision/records/2026-09-17-decision-log.md#A6, docs/decision/records/records.md#A134
- verification: unit

kotowari always applies REQ-core-130 and REQ-core-131 only to a `decision record` that has a heading whose text after "## ", with surrounding whitespace removed, exactly matches "Context", and does not apply them to a `decision record` without one. REQ-core-132 is applied to every `decision record`, whether or not it has "## Context". A file without a heading of a `decision section` is not a `decision record` and undergoes neither check.

### REQ-core-130: Required supplementary lines

- kind: algorithm
- source: docs/decision/records/2026-09-17-record-form.md#A2, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A13, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-decision-log.md#A1, docs/decision/records/2026-09-17-decision-log.md#A12
- definition: TBL-core-022
- verification: unit

### REQ-core-131: Supplementary lines with unknown names

- kind: event_driven
- source: docs/decision/records/2026-09-17-record-form.md#A1, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A14, docs/decision/records/2026-09-17-record-form.md#A27
- verification: unit

When the name of a `supplementary line` of a `numbered line` in a section of TBL-core-022 is none of the names TBL-core-022 allows, kotowari raises an `error` of record_field_unknown with "line" set to that line and detail set to that name.

### REQ-core-133: Names and values of supplementary lines

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A23, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-record-form.md#A37, docs/decision/records/2026-09-17-decision-log.md#A2, docs/decision/records/2026-09-17-record-form.md#A1, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A35
- verification: unit

kotowari always reads the name of a `supplementary line` as the characters from just after "- " up to the first ":", and the value as the characters from after the first ":" to the end of the line, with surrounding whitespace removed. The name is at least one character long and contains neither whitespace nor ":"; a line that does not match is not a `supplementary line`. A `supplementary line` with an empty value is counted as absent in the judgement of TBL-core-022, while the name check (REQ-core-131) applies even when the value is empty. A superseded_by line with an empty value is not subject to the judgement of TBL-core-023. In a `decision record` without "## Context", a superseded_by line with an empty value receives no `finding` (the other judgements of REQ-core-132 apply in every `decision record`). A line that does not match the form of a name is an `exclusion`, and its name is not counted as present. A line whose value is "not recorded" is counted as present.

### REQ-core-134: The number of supplementary lines is not looked at

- kind: prohibition
- source: docs/decision/records/2026-09-17-record-form.md#A15
- verification: unit

kotowari must not raise a `finding` because two or more `supplementary line` entries with the same name are under one `numbered line`.

### REQ-core-135: Lines not read in decision records

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A14, docs/decision/records/2026-09-17-record-form.md#A24, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A34, docs/decision/records/2026-09-17-record-form.md#A35, docs/decision/records/records.md#A115, docs/decision/records/2026-09-17-record-form.md#A45
- verification: unit

kotowari always leaves unread, as an `exclusion`, the following in a `decision record`: lines of the `numbered line` form and lines of the `supplementary line` form in sections not in the table of TBL-core-022 (including Revisions and "## Context"); lines of the `supplementary line` form before the first `numbered line` of a section; lines that are neither a `numbered line`, a `supplementary line` nor a heading (including non-bullet body lines and lines before the first "## " heading); and the inside of a `code block` (gherkin included; "## " headings inside are not counted either; when it is left unclosed until the document ends, everything up to the end of the document is inside, and no `finding` is raised). Headings are read to divide the sections and to judge "## Context", and are not subject to any `finding`. The list of sections is the "Section" column of the table of TBL-core-022, and it is used in every `decision record`, apart from the scope to which REQ-core-130 applies.

### REQ-core-136: Reading the records is one function

- kind: ubiquitous
- source: docs/decision/records/2026-09-17-record-form.md#A10, docs/decision/records/2026-09-17-record-form.md#A11, docs/decision/records/2026-09-17-record-form.md#A16
- verification: review
- how_to_verify: Confirm that there is one function that reads decision records (parse_records_file in `crates/kotowari-core/src/sources.rs`, or the module that reads the records), and that the form check and the judgement of sources read only the structure it returns. With `rg "lines\(\)" crates/kotowari-core/src/sources.rs`, confirm that no place outside the reading function reads the lines of a record file directly

kotowari always reads a `decision record` (sections, `numbered line` entries, `supplementary line` entries, links) in one function, and the form check and the judgement of a `source` read only the structure that function returns.

## Decision tables

### TBL-core-022: Required supplementary lines by section

- source: docs/decision/records/2026-09-17-record-form.md#A2, docs/decision/records/2026-09-17-record-form.md#A6, docs/decision/records/2026-09-17-record-form.md#A7, docs/decision/records/2026-09-17-record-form.md#A12, docs/decision/records/2026-09-17-record-form.md#A13, docs/decision/records/2026-09-17-record-form.md#A14, docs/decision/records/2026-09-17-record-form.md#A27, docs/decision/records/2026-09-17-record-form.md#A32, docs/decision/records/2026-09-17-record-form.md#A33, docs/decision/records/2026-09-17-decision-log.md#A1, docs/decision/records/records.md#A115

The allowed names are the six "why", "rejected", "decided_by", "superseded_by", "decides" and "related". The judgement of a `numbered line` is made before that of a `supplementary line`, and a `numbered line` is not taken as a `supplementary line`. When a `numbered line` of a section lacks a `supplementary line` with a required name, an `error` of record_field_missing is raised with "line" set to that `numbered line` and detail set to that name. Lines of sections not in the table are not read. A section starts at a "## " heading and ends at the next "## " heading; a "### " heading does not end a section.

| Section | Required name |
|---|---|
| Agreements | why |
| Prohibitions | why |
| Delegated | why |
| Rejected | why |
| Undecided | decides |
| Superseded | superseded_by |

## Examples

```gherkin
@id=EX-core-101 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A2,docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: In a record with Context, a decision without why is an error
  Given in the Agreements section of a decision record with a "## Context" heading there is a line starting with "- A1 ", and under it there is no "- why:" line
  When "kotowari check" is run
  Then an error of record_field_missing is raised whose "line" is the "- A1 " line and whose detail is "why"

@id=EX-core-102 @about=REQ-core-129 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A22,docs/decision/records/2026-09-17-record-form.md#A27
Scenario: A record without Context does not undergo the form check
  Given in the Agreements section of a decision record without a "## Context" heading there is a line starting with "- A1 ", and under it there is no "- why:" line
  When "kotowari check" is run
  Then no error of record_field_missing or record_field_unknown pointing at that record is raised

@id=EX-core-103 @about=REQ-core-133 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A23,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-decision-log.md#A2,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: not recorded passes, and a value of only whitespace counts as absent
  Given in the Agreements section of a decision record with a "## Context" heading, under the "- A1 " line there is "- why: not recorded", and under the "- A2 " line there is the line "- why:   "
  When "kotowari check" is run
  Then no record_field_missing pointing at the "- A1 " line is raised, and an error of record_field_missing pointing at the "- A2 " line with detail "why" is raised

@id=EX-core-104 @about=REQ-core-131 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A12,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: A supplementary line with an unknown name is an error
  Given in the Agreements section of a decision record with a "## Context" heading, under the "- A1 " line there are the lines "- why: x" and "- reason: x"
  When "kotowari check" is run
  Then an error of record_field_unknown is raised whose "line" is the "- reason: x" line and whose detail is "reason", and no error pointing at the "- A1 " line is raised

@id=EX-core-105 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A2,docs/decision/records/2026-09-17-record-form.md#A7,docs/decision/records/2026-09-17-record-form.md#A13,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A32
Scenario: A Superseded line without superseded_by is an error
  Given in the Superseded section of a decision record with a "## Context" heading there is a "- A3 " line, and under it there is only "- why: x"
  When "kotowari check" is run
  Then an error of record_field_missing is raised whose "line" is the "- A3 " line and whose detail is "superseded_by"

@id=EX-core-110 @about=REQ-core-135 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A14,docs/decision/records/2026-09-17-record-form.md#A24,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A35,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: Numbered lines of sections not in the table, supplementary lines without a parent, and lines of other forms are not read
  Given in a decision record with a "## Context" heading, the Revisions section has the line "- A21 は A5 を置き換える", the Agreements section has a "- why: x" line before its first "- A1 " line, and under the "- A1 " line there are the lines "- why: x", "- (i) x", "- why : x" and "（なし）"
  When "kotowari check" is run
  Then no error pointing at any line is raised

@id=EX-core-111 @about=REQ-core-134 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A12,docs/decision/records/2026-09-17-record-form.md#A15,docs/decision/records/2026-09-17-record-form.md#A27
Scenario: Two supplementary lines with the same name pass
  Given in the Agreements section of a decision record with a "## Context" heading, under the "- A1 " line there are two "- why: x" lines
  When "kotowari check" is run
  Then no error pointing at the "- A1 " line or the lines under it is raised

@id=EX-core-113 @about=REQ-core-135 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A34,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: Numbered lines inside a code block are not read
  Given in the Agreements section of a decision record "docs/decision/records/x.md" with a "## Context" heading there are a "- A1 " line and "- why: x" under it, and in a code block after them there is the line "- A9 x"
  When "kotowari check" is run
  Then no error pointing at the "- A9 x" line is raised, and no record_field_missing pointing at the "- A1 " line is raised either

@id=EX-core-118 @about=REQ-core-058 @source=docs/decision/records/2026-09-17-record-form.md#A34,docs/decision/records/records.md#A38,docs/decision/records/ir-form.md#検査の種類
Scenario: A number inside a code block is not the target of a source
  Given "decisions.records" is "docs/decision/records", the line "- A9 x" is only inside a code block in the Agreements section of the decision record "docs/decision/records/x.md", and a requirement of the IR writes the source "docs/decision/records/x.md#A9"
  When "kotowari check" is run
  Then an error of source_invalid with detail "docs/decision/records/x.md#A9" is raised

@id=EX-core-114 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A5,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: An unindented supplementary line belongs to the decision too
  Given in the Agreements section of a decision record with a "## Context" heading, on the line after the "- A1 " line there is "- why: x" without indentation
  When "kotowari check" is run
  Then no record_field_missing pointing at the "- A1 " line is raised

@id=EX-core-115 @about=REQ-core-130 @source=docs/decision/records/2026-09-17-record-form.md#A1,docs/decision/records/2026-09-17-record-form.md#A27,docs/decision/records/2026-09-17-record-form.md#A33,docs/decision/records/2026-09-17-record-form.md#A2
Scenario: A decision line whose text contains a colon is not taken as a supplementary line
  Given in the Agreements section of a decision record with a "## Context" heading there are the line "- A1 定義を機械的にする: 節にある行" and "- why: x" under it
  When "kotowari check" is run
  Then no error pointing at the "- A1 " line is raised
```
