# Terms, vague words and document-name references

English | [日本語](terms.ja.md)

Covers the checks of words enclosed in backquotes, of vague words and of document-name references.

## Requirements

### REQ-core-063: Target lines

- kind: algorithm
- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A53, docs/decision/records/records.md#A56
- definition: TBL-core-013
- verification: unit

### REQ-core-064: Words not in the glossary

- kind: event_driven
- source: docs/decision/records/records.md#A31, docs/decision/records/records.md#A42, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A63, docs/decision/records/records.md#A116, docs/decision/records/records.md#A145, docs/decision/records/2026-09-24-review5-gaps.md#A1
- verification: unit

kotowari pairs the backquotes outside double quotes in a `target line` in order from the left of the line, and when the text between a pair (which may contain double quotes; the text with surrounding whitespace removed) is neither a `term` nor an `ID`, kotowari raises an unknown_term `error` with the text after removal as detail, even when it is a path or a code fragment. For an enclosure with empty contents the detail is "``".

### REQ-core-065: When there is no glossary

- kind: event_driven
- source: docs/decision/records/records.md#A56, docs/decision/records/records.md#A63, docs/decision/records/records.md#A53, docs/decision/records/2026-09-16-ir-tree.md#A3
- verification: unit

When the `chain` of a document has no `glossary` at all, kotowari makes every string enclosed in backquotes in a `target line` that is not an `ID` an unknown_term `error`.

### REQ-core-066: Vague words

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A41, docs/decision/records/records.md#A42, docs/decision/records/records.md#A53, docs/decision/records/ir-form.md#検査の種類
- verification: unit

When a `target line` contains a `vague word` as a substring match, kotowari raises a vague_word `error`.

### REQ-core-067: One per occurrence

- kind: ubiquitous
- source: docs/decision/records/records.md#A53, docs/decision/records/records.md#A56, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A117, docs/decision/records/records.md#A142
- verification: unit

kotowari always raises one unknown_term or vague_word `finding` per occurrence, and counts the occurrences of a `vague word` from the left of the line by longest match, without overlap.

### REQ-core-068: Forgotten enclosures are not detected

- kind: prohibition
- source: docs/decision/records/records.md#A31, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- verification: unit

kotowari must not detect that a `term` was not enclosed in backquotes.

### REQ-core-069: How document-name references are found

- kind: algorithm
- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A47, docs/decision/records/2026-09-16-ir-tree.md#A21, docs/decision/records/2026-09-22-ir-engine.md#A72
- definition: TBL-core-014
- verification: unit

### REQ-core-070: The referenced document does not exist

- kind: event_driven
- source: docs/decision/records/2026-09-16-ir-tree.md#A5, docs/decision/records/2026-09-16-ir-tree.md#A17, docs/decision/records/2026-09-16-ir-tree.md#A18, docs/decision/records/ir-form.md#検査の種類
- verification: unit

When a `document-name reference` contains no "/" and no document of that name is in the same directory as the document that wrote the reference, when it contains "/" and there is no document at that relative path from the location of the `IR`, or when the sequence contains a "." or ".." element, kotowari raises a missing_document `error` with the reference string as detail. Whether a document exists is judged by whether it is among the documents of the `IR` that were read (a document under a symbolic link to a directory, which is not read, counts as not existing). A document not in the same directory is not searched for by going up to parent directories.

## Decision tables

### TBL-core-013: What the term and vague word checks cover

- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A53, docs/decision/records/records.md#A56, docs/decision/records/records.md#A133

| Line | Check |
|---|---|
| Statement of a requirement | Covered |
| Statement of a property | Covered |
| Gherkin Given, When, Then, And and But lines | Covered |
| Gherkin Scenario line | Not covered |
| "- " line | Not covered |
| Tag line | Not covered |
| Meaning column of a glossary | Not covered |
| Body of a flag record | Not covered |

### TBL-core-014: Conditions of a document-name reference

- source: docs/decision/records/records.md#A47, docs/decision/records/ir-form.md#文書名の参照, docs/decision/records/records.md#A73, docs/decision/records/records.md#A118, docs/decision/records/2026-09-16-ir-tree.md#A5, docs/decision/records/2026-09-16-ir-tree.md#A12, docs/decision/records/2026-09-16-ir-tree.md#A14, docs/decision/records/2026-09-16-ir-tree.md#A17, docs/decision/records/2026-09-16-ir-tree.md#A21, docs/decision/records/2026-09-22-ir-engine.md#A72

| Order | Condition |
|---|---|
| 1 | It is inside a `statement` (including the lines of a `scope` and the body lines of a `flag record`). The `title` and heading lines, list lines starting with "- ", table lines and the inside of a `code block` (including gherkin blocks) are not covered |
| 2 | The character just before the start of the sequence is none of an alphanumeric character, "_", "-", "/", "." or a backquote (the line start included; whitespace, punctuation and Japanese characters are boundaries). A character that occurs inside the sequence (lowercase English letters, digits, hyphen, "." and "/") just before it is not a boundary, so a reference is never picked up from the middle of a sequence |
| 3 | One or more elements (each one of a run of lowercase English letters, digits and hyphens, ".", or "..") are separated by "/", the last element is a run of lowercase English letters, digits and hyphens followed by ".md", and the character just after ".md" is none of an alphanumeric character, "_", "-", "#" or "/" |
| 4 | It is not inside double quotes. When the double quotes in a line are odd in number, the part from the last quote to the end of the line counts as inside quotes |

## Examples

```gherkin
@id=EX-core-284 @about=REQ-core-064 @source=docs/decision/records/2026-09-24-review5-gaps.md#A1
Scenario: An enclosure with double quotes inside does not break
  Given a `statement` has an enclosure in which text enclosed in double quotes sits between words that are not a `term`, followed by an enclosure of a word that is not a `term`
  When "kotowari check" is run
  Then one unknown_term is raised with the whole contents of the first enclosure as detail, and one with the contents of the second enclosure as detail
  And the text between the two enclosures does not become an unknown_term

@id=EX-core-013 @about=REQ-core-064 @source=docs/decision/records/records.md#A42,docs/decision/records/records.md#A56,docs/decision/records/ir-form.md#検査の種類
Scenario: An enclosed path is an error
  Given a `statement` of a `requirement` writes "src/main.rs" enclosed in backquotes
  When "kotowari check" is run
  Then an unknown_term error whose detail is "src/main.rs" is raised

@id=EX-core-014 @about=REQ-core-069,REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A5,docs/decision/records/2026-09-16-ir-tree.md#A15,docs/decision/records/2026-09-16-ir-tree.md#A21,docs/decision/records/2026-09-22-ir-engine.md#A72
Scenario: A path outside the location is enclosed in quotes
  Given a scope line of a document writes "docs/decision/adr/0001-test-marker.md" without quotes
  When "kotowari check" is run
  Then it does not become a reference pointing only at "0001-test-marker.md", and a missing_document error for "docs/decision/adr/0001-test-marker.md" is raised

@id=EX-core-022 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A14
Scenario: The form of a source is not a reference
  Given a scope line of a document writes "docs/decision/records/records.md#A12" without quotes
  When "kotowari check" is run
  Then no missing_document error is raised

@id=EX-core-023 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A5,docs/decision/records/2026-09-22-ir-engine.md#A72
Scenario: A bare name looks only in the same directory
  Given a scope line of the document "docs/ir/network/dns/a.md" writes "b.md", and "docs/ir/network/b.md" exists but "docs/ir/network/dns/b.md" does not
  When "kotowari check" is run
  Then a missing_document error for "b.md" is raised

@id=EX-core-024 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A5,docs/decision/records/2026-09-22-ir-engine.md#A72
Scenario: A name containing a slash is looked up relative to the location
  Given a scope line of the document "docs/ir/network/dns/a.md" writes "network/publish/c.md", and "docs/ir/network/publish/c.md" exists
  When "kotowari check" is run
  Then no missing_document error is raised

@id=EX-core-025 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A17,docs/decision/records/2026-09-22-ir-engine.md#A72
Scenario: A sequence containing "." and ".." elements is not resolved
  Given a scope line of the document "docs/ir/network/dns/a.md" writes "../b.md" and "./c.md", and "docs/ir/network/b.md" and "docs/ir/network/dns/c.md" exist
  When "kotowari check" is run
  Then one missing_document error is raised for "../b.md" and one for "./c.md"

@id=EX-core-031 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A17,docs/decision/records/2026-09-16-ir-tree.md#A21,docs/decision/records/2026-09-22-ir-engine.md#A72
Scenario: A sequence where ".md" is followed by "/" is not a reference
  Given a scope line of a document writes "a.md/b.md" without quotes, and none of "a.md", "b.md" and "md/b.md" exists
  When "kotowari check" is run
  Then no missing_document error is raised

@id=EX-core-033 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A21,docs/decision/records/2026-09-22-ir-engine.md#A72
Scenario: A reference joined directly to Japanese characters is picked up too
  Given a scope line of a document writes "設定の形はtimeout-config.mdで定める", and "timeout-config.md" is not in the same directory
  When "kotowari check" is run
  Then a missing_document error for "timeout-config.md" is raised

@id=EX-core-034 @about=REQ-core-069 @source=docs/decision/records/2026-09-16-ir-tree.md#A21
Scenario: A path enclosed in backquotes is not a reference
  Given a `statement` of a document writes "`a.md`", "a.md" does not exist, and it is not in the `glossary` either
  When "kotowari check" is run
  Then an unknown_term error for "a.md" is raised, and no missing_document error is raised

@id=EX-core-032 @about=REQ-core-070 @source=docs/decision/records/2026-09-16-ir-tree.md#A18,docs/decision/records/2026-09-22-ir-engine.md#A72
Scenario: A reference to a document in a place that is not read is treated as not existing
  Given "docs/ir/link" is a symbolic link pointing at a directory outside the location, "d.md" is under it, and a scope line of "docs/ir/a.md" writes "link/d.md"
  When "kotowari check" is run
  Then a missing_document error for "link/d.md" is raised

@id=EX-core-026 @about=REQ-core-065 @source=docs/decision/records/2026-09-16-ir-tree.md#A3
Scenario: In a document whose chain has no glossary every enclosed word is an error
  Given "docs/ir/CONTEXT.md" does not exist, "docs/ir/network/CONTEXT.md" has a `term`, and a `statement` of "docs/ir/a.md" writes that `term` enclosed
  When "kotowari check" is run
  Then unknown_term errors are raised on "docs/ir/a.md", and none on the documents of "docs/ir/network/"
```
