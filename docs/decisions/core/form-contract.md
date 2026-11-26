# Fixing the form of documents

English | [日本語](form-contract.ja.md)

Covers where the form of decision documents is decided, which engine reads decision documents, and not using external tools before the check. The form of a decision document itself (the title, the sections of a decision, the state line, the confirmation rows) is decided in [Write one decision as text in Problem, Decision, Rejected options and Confirmations](decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations).

## The form of decision documents is fixed in code, and no file that declares a form is read

Status: implemented

### Problem

If the form of documents could be changed with configuration or a schema file, kotowari would have a small schema language of its own, which affects the upper limit on how much is built. There is only one form now. Without fixing the form, the meaning of the check "the form is broken" also wavers. The question of how to provide schema files where kotowari is distributed also arises.

### Decision

kotowari reads decision documents in one form fixed in code. A file that declares a form (for example ".kotowari/schema.yaml") is not read even if it is present. There is no way to change the form through configuration. Documents do not declare a form either.

### Confirmations

- C-core-formc-1 Even when content that cannot be read as YAML is placed in ".kotowari/schema.yaml", check does not stop, and gives the same exit code and standard output as before placing it

## No external tool is used before the check

Status: implemented

### Problem

Using an external tool written in another language as a step before the check of the form of documents would ask users to install that tool.

### Decision

kotowari does not use external processes as a step before the check. Reading documents and checking their form are done inside kotowari.

### Confirmations

- C-core-formc-2 Even when the environment variable "PATH" points only at an empty directory, the exit code, standard output and standard error of check do not change

## Decision documents are read by the mds engine with a schema embedded in the binary

Status: implemented

### Problem

kotowari reads decision documents with code of its own. It walks the tree that results from reading the Markdown, and tells apart state lines, sections and confirmation rows by hand. The mechanism that reads the structure of Markdown documents is in two places: the generic engine mds (kotowari-markdown-schema) and kotowari.

When the specification was kept as the IR, it had been decided that kotowari does not read the Markdown structure of documents itself and leaves it to the engine that reads schemas. The form of decision documents could not be declared in mds's schema language, because section names were limited to one string, item headings were required to have the "ID: name" form, and headings of depth 4 or more and unclosed code blocks could not be expressed. So a reader of its own was written.

### Decision

kotowari reads decision documents with the mds engine. The schema used for reading is one schema embedded in the binary, which declares the form of decision documents (the title, the topic paragraph, decision headings, the state line, the four sections, confirmation rows, and the "Open" section) with the form words of both Japanese and English. A document written with Japanese and English form words mixed is also read with the same schema. The embedded schema is part of the binary, and the form stays fixed in the binary as in [The form of decision documents is fixed in code, and no file that declares a form is read](#the-form-of-decision-documents-is-fixed-in-code-and-no-file-that-declares-a-form-is-read).

kotowari has no code of its own that tells apart the form of decision documents. The title, decisions, states, sections, and the ID, whether "(by reading)" is present, the name and the line range of confirmation rows are taken from the engine's extracted values. Form errors are reported by mapping the engine's findings to kotowari's findings. An engine finding with no mapping target stops as in [A mismatch in kotowari's internal mapping stops with mapping error, kept apart from errors in the user's input](cli-environment.md#a-mismatch-in-kotowaris-internal-mapping-stops-with-mapping-error-kept-apart-from-errors-in-the-users-input).

The following five are handled by kotowari outside the engine.

- The anchors of headings, and the check of links inside decision documents
- The comparison of the skeleton between the sides of a pair
- The fingerprints of guide markers
- The quotation of surface names
- The matching of confirmation IDs with test markers

### Rejected options

- Keep the reader of its own. The mechanism that reads the structure of Markdown documents would stay in two places, kotowari and mds.
- Have a separate schema for each language of the form words. Choosing which one to use would need a guess of the language, and documents with Japanese and English form words mixed could no longer pass.
- Take out confirmation IDs with a loose capture, and have kotowari check their form. The engine would no longer report IDs of the wrong form.

### Confirmations

- C-core-formc-3 (by reading) `crates/kotowari-core/src/decisions.rs` has no function that tells apart the title of a decision document, the headings of decisions and sections, the state line or confirmation rows from raw lines or from the Markdown tree. The only functions that read raw lines or the Markdown tree are those for the five listed in the Decision, and those that cut out and use line ranges or lines the engine returned (the body of a confirmation row, the detail of an error)
- C-core-formc-4 The embedded schema can be loaded as an mds schema, and its state values ("提案", "実装済み", "proposed", "implemented"), the heads of the state line ("状態:", "Status:"), the Japanese and English names of the four sections, and the names of the "Open" section ("未決", "Open") equal the words kotowari uses to tell states and sections apart
- C-core-formc-5 check reports no form error for a decision document written with Japanese and English form words mixed in one document
- C-core-formc-6 For decision documents that have an unclosed code block, a second title, a decision with no state line, a state that is neither "proposed" nor "implemented", a decision with no "Decision" section, and a confirmation ID of the wrong form, the errors unclosed_code_block, multiple_titles, decision_state_invalid, decision_state_invalid, decision_section_missing and invalid_id are reported respectively

## A depth-3 heading with an unknown name in a decision is an error, and so is the content under it, block by block

Status: implemented

### Problem

When a decision has a depth-3 heading whose name is none of the four sections, the current reader makes only that heading an unknown_heading error, and does not make the content under it an error. mds makes the content under an undeclared heading undeclared-line findings, block by block. That content is read neither into the fingerprints of guide markers, nor into the quotation of surface names, nor into confirmation rows.

### Decision

A depth-3 heading with an unknown name inside a decision, or before the first decision heading, is an unknown_heading error, and the content under it is also an unknown_line error, block by block.

### Rejected options

- Make only the heading an error (the behaviour of the current reader). Mapping the engine's findings would need code that drops the findings on the content.
- Change the rule of mds so that the content under an undeclared heading is not a finding. The behaviour would change for every user of mds.

### Confirmations

- C-core-formc-7 In a document that has, inside a decision, a "### Notes" heading with a paragraph and a bullet under it, an unknown_heading error is reported on the "### Notes" line, and unknown_line errors on the opening lines of the paragraph and of the bullet
- C-core-formc-8 A "### " heading before the first "## " heading, and the paragraph under it, also get unknown_heading and unknown_line errors
