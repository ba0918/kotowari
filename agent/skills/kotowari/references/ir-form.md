Based on the kotowari specification (revised 2026-09-23; the version of kotowari itself is not pinned)

## Documents

kotowari walks the place for the IR (default `docs/ir`) with no limit on directory depth, and reads the files whose extension is lowercase `.md`.

- In every directory, `CONTEXT.md` is a glossary, `FLAGS.md` holds flags, and anything else is a topic document
- Do not place a `README.md` (if placed, it is read as a topic document, and without a scope line it becomes missing_scope). Describe a directory in the scope lines after the title of that directory's `CONTEXT.md`
- Name directories with lowercase English letters, digits and hyphens. kotowari does not restrict directory names, but a document name reference can point only at these characters
- Symbolic links to files are read. A leading BOM is skipped
- Not read: anything but `.md`, hidden directories, symbolic links to directories, anything that is neither a directory nor a regular file (sockets, named pipes, devices), and files named `.kotowari`. Hidden directories and symbolic links to directories are not followed at any depth. Empty directories raise no finding. If a directory cannot be read, it stops
- The `path` of a finding is relative to the base directory: the path relative to the place for the IR, joined with `/`. The `detail` of missing_title, missing_scope and glossary_invalid is the file name without the directory

Each document has exactly one `# ` title. If there is no title, missing_title; if there are two or more, a multiple_titles error per title after the first (detail is the text of that title).

Headings are only CommonMark ATX heading lines (up to three leading spaces, one to six `#`, then a space or the end of the line). A line of `#` alone is also a heading. A line with no space right after `#` like `#foo`, a line with seven or more `#`, and a line of only `---` or `===` are not headings; they are read as a `statement`.

The non-empty lines after the title and before the first `## ` or `### ` are "what the document covers" (the scope). A topic document has one or more (if none, a missing_scope error). `CONTEXT.md` and `FLAGS.md` need not have any.

A topic document has the following sections (leave out the sections it does not need).

- `## Requirements` — holds requirements
- `## Decision tables` — holds decision tables
- `## Properties` — holds properties
- `## Examples` — holds scenarios

A line ends at `\n`, at `\r\n` (counted as one line end) or at a lone `\r`. The last line counts as one line even without a newline. Lines start at 1. The character encoding is UTF-8.

Inside a code block (from a line starting with three or more `` ` `` or `~` to a line of the same character with the same number or more), everything except gherkin blocks is outside the check. If the document ends without closing it, an unclosed_code_block error.

## IDs

An ID is one of the following prefixes, followed by an optional name and `-`, followed by a number of three or more digits. With four or more digits, the first digit is not `0` (`REQ-001`, `REQ-1000` and `REQ-core-001` are of the correct form; `REQ-1`, `REQ-0001` and `REQ-Core-001` do not fit the form). The name starts with a lowercase English letter, and from the second character on uses only lowercase English letters, digits and `-`. When a name is given, it must match the first level of the document's place, or an id_domain_mismatch error is raised.

- `REQ-` — requirement
- `TBL-` — decision table
- `PROP-` — property
- `EX-` — scenario
- `FLAG-` — flag

IDs are unique across all documents, across directories. If the same ID appears in two or more places, a duplicate_id error.

## Items

| Item | Where it goes | Heading | Lines it has | Statement |
|---|---|---|---|---|
| Requirement | Under `## Requirements` | `### REQ-nnn: name` | `- kind:` (event_driven, state_driven, ubiquitous, prohibition, invariant, algorithm), `- source:`, `- verification:` (unit, property, proof, review), `- definition:` (an algorithm has it; others may have it), `- how_to_verify:` (required when verification is review; others may have it. Free text describing how a person or an LLM verifies it) | Has one unless algorithm. An algorithm has none |
| Decision table | Under `## Decision tables` | `### TBL-nnn: name` | `- source:` and a Markdown table | None (has a table) |
| Property | Under `## Properties` | `### PROP-nnn: name` | `- source:` | Has one |
| Scenario | A gherkin code block under `## Examples` | The `Scenario:` line | The tags on the preceding line `@id=EX-nnn`, `@about=ID,...`, `@source=source,...` | None (has step lines) |
| Flag | Under `## Flags` in `FLAGS.md`, or after the title with no section in between | `### FLAG-nnn: name` | `- kind:` (contradiction, gap, ambiguity), `- related:` (comma-separated IDs), `- source:` | Has a body |
| Term | `CONTEXT.md` | A row of the first table whose header has the three columns Term, Meaning and Source | None | None (has the Meaning column) |

When a `### ` heading is not of the form `### ID: name` (including an `EX-` ID used in a heading, and headings deeper than `#### `), an unknown_heading error. The lines under a heading that does not fit the form are also read by the item rules, and the applicable findings are raised.

The `- xxx:` lines allowed under a heading are only those in the "Lines it has" column of the table above for that kind of item. An unknown `- xxx:` line, or a list line not of the form `xxx:` (a line starting with `- `, `* `, `+ `, or digits and `.`, or a line of `-` alone), is an unknown_field error. The content of an unknown line is not read.

The `- ` lines under a heading may come in any order, with blank lines in between. The values of `- definition:`, `- related:` and `- source:` may be several, separated by commas. If the same `- xxx:` line appears twice or more, a duplicate_field error.

Errors when a required line is missing:

- No `- verification:` → verification_missing
- No `- source:`, or it is empty → missing_source
- Any other (`- kind:`, `- related:`) missing → missing_field

A `- kind:`, `- verification:`, `- definition:`, `- related:` or `- how_to_verify:` line with an empty value counts as present, and no finding for a missing line is raised. The empty values of `- kind:` and `- verification:` become the unknown_kind and verification_invalid errors below. If a decision table has no table, a missing_table error.

When the value of `- kind:` is not one of the values defined for the kind of item, an unknown_kind error. When the value of `- verification:` is not unit, property, proof or review, a verification_invalid error. When a requirement of kind algorithm has no `- definition:` pointing at a decision table or property, an algorithm_without_definition error.

A statement is a non-empty line under a heading that is neither a list nor a table; each line is read as one statement. Lines that follow without a blank line in between are also separate statements. A `- xxx:` line and any other list line end at one line; a line directly following it without a blank line, and an indented non-list line following it after a blank line, are statements and do not go into the list line's value (an indented list line is a list line). Quotes, thematic breaks, HTML, image lines, lines of only `---` or `===`, lines indented by four or more spaces after a blank line, and lines starting with `|` that have no delimiter row and do not form a table are also checked as statements. If a requirement other than algorithm (including one with no `- kind:` line), a property, or a flag item has no statement, a missing_statement error. Whether it follows the EARS patterns is not checked.

## Scenarios

Write them as `Scenario:` lines inside a gherkin code block.

Put the tags on the preceding line. The tags are only the three `@id=EX-nnn`, `@about=ID,...` and `@source=source,...`. A tag with an empty value is treated as absent. A tag line binds only to the `Scenario:` directly after it; if any other line comes in between, it does not bind. A word not starting with `@` is an unknown_tag error.

- No `@id` or `@about` → missing_tag
- No `@source` → missing_source
- The value of `@id` is not of the form `EX-nnn` → invalid_id (missing_tag is not raised)
- A tag other than the three → unknown_tag

Lines in the block are looked at without their leading spaces, and only the following are allowed.

- Tag lines (starting with `@`)
- `Scenario:` lines
- Step lines (`Given`, `When`, `Then`, `And`, `But` followed by one or more half-width spaces)
- Comments starting with `#`
- Blank lines

Any other line (including `Feature:`, `Background:`, `Scenario Outline:`, `Examples:` and data tables), a step line not following a `Scenario:`, and a tag line not directly followed by a `Scenario:` are invalid_gherkin_line errors.

A `Scenario:` line outside a gherkin block is not taken as a scenario.

## Glossary

A glossary `CONTEXT.md` can be placed in any directory. The terms visible from a document are the sum of the terms of every `CONTEXT.md` from that document's directory up to the root of the place for the IR (the chain). Terms of a `CONTEXT.md` in a neighbouring branch are not visible (unknown_term). A term is placed in only one file in the chain. Cut a directory when a term specific to that group appears.

In `CONTEXT.md`, place a table starting with the header `| Term | Meaning | Source |` and the delimiter row `|---|---|---|`.

The glossary table is the first table, after the title and before the first `## ` heading, whose header matches this. Tables before it whose header does not match, and tables after it, whether or not their header matches, become neither terms nor findings. A table ends at a blank line or a non-table line. Lines starting with `|` outside the table do not become terms. If there are a header and a delimiter row, the table counts as present even with 0 term rows.

- There is a `CONTEXT.md` but no header and delimiter row → glossary_invalid (that glossary is treated as 0 terms; the other glossaries in the chain remain visible)
- A row with fewer than three cells, or a row whose term cell is empty → invalid_glossary_row (it does not become a term). A row with four or more cells is not a broken row: it becomes a term from its first three cells, and the remaining cells are dropped
- The same term twice or more in the same glossary, or already present in a glossary nearer the root of the chain → duplicate_term (raised on the row farther from the root. That row does not become a term, and its source is not checked. The word stays visible through the definition nearer the root)

The Meaning column is not checked for terms or vague words. The Source column is checked as a source.

## Flags

Under the `## Flags` section of `FLAGS.md`, or after the title with no section in between, put a `### FLAG-nnn: name` heading (a document may have both), and under it the following lines and a body.

- `- kind:` — one of contradiction, gap and ambiguity
- `- related:` — the related IDs, comma-separated
- `- source:`

The body is not checked for terms or vague words.

## Sources

A source is written in the form `path#anchor`. It is split into path and anchor at the first `#`. A path cannot contain `#`. The path is written relative to the base directory (example: `docs/decision/records/2026-01-01-example.md#A1`). Paths are compared after normalisation: the trailing `/` and leading `./` are removed, `/./` and consecutive `/` in the middle are folded into one `/`, and `\` is changed to `/`.

How the target of a source is judged:

- When the path is a file inside the place for decision records (`decisions.records`) and that file is a decision record (it has one or more decision section headings outside code blocks): the anchor is a decision number (one uppercase English letter followed by one or more digits; of the form `A26`, `P1`). A decision section of that file (`## Agreements`, `## Prohibitions`, `## Delegated`, `## Rejected`) must have a line starting with `- A26 ` or a line of `- A26` alone. A decision section starts at a `## ` heading and ends at the next `## ` heading; a `### ` heading does not end the section. Decision numbers are looked up per file
- When the path is a file inside the place for decision records or the place for ADRs (`decisions.adr`) and that file is Markdown that is not a decision record (an ADR, a form contract, a supplementary document): the anchor is the text of a `## heading` outside code blocks, matched exactly after trimming surrounding spaces
- Neither of the above → source_invalid

The Source column of the glossary and the `@source` of scenarios are checked by the same rules. kotowari does not judge whether the source really states the content of the item.

When a definition, @about, related, or an ID in a statement points at an ID that does not exist, an unresolved_reference error.

## Terms and vague words

Anything enclosed in backticks must be a term of the glossary or an ID (otherwise an unknown_term error). When the chain of a document has no glossary at all, everything but IDs becomes unknown_term. The inside of double quotes is not looked at. Paths and code fragments enclosed in backticks are also checked as terms. Write concrete values in double quotes. Forgetting to enclose a term is not detected.

Vague words are checked by partial match against the words in the configuration's `vague_words` (a vague_word error).

What is checked:

| Line | Check |
|---|---|
| Statement of a requirement | Checked |
| Statement of a property | Checked |
| gherkin Given, When, Then, And, But lines | Checked |
| gherkin Scenario line | Not checked |
| "- " lines | Not checked |
| Tag lines | Not checked |
| Meaning column of the glossary | Not checked |
| Body of a flag | Not checked |

When the number of backticks outside double quotes on a checked line is odd, an unclosed_backtick error. On that line, terms and ID references are not checked, but vague words are.

One finding per occurrence. Occurrences of vague words are counted from the left of the line, longest match first, without overlap.

## Document name references

Sequences of document names outside code blocks (including gherkin) are checked as references. A reference has one of two forms.

- `xxx.md` without `/` points at a document in the same directory as the document that holds the reference. It does not search up through parent directories
- `network/dns/xxx.md` with `/` points at the document at that path relative to the place for the IR

If the target is not among the documents read, a missing_document error (documents under a symbolic link to a directory, which are not read, count as absent). A reference with a `.` or `..` element is not resolved and is missing_document. Writing a repository path outside the place for the IR without quotes makes it a reference of the form with `/` and it becomes missing_document, so write it in double quotes as a concrete value.

Conditions for being taken as a reference:

1. The character right before the start of the sequence is none of an English letter or digit, `_`, `-`, `/`, `.` and a backtick (including the start of the line. Spaces, punctuation and Japanese characters are boundaries, so `network-x.md` in 「設定はnetwork-x.mdで定義する」 is also a reference)
2. One or more elements (a run of lowercase English letters, digits and hyphens, `.`, or `..`) separated by `/`, the last element being a run of lowercase English letters, digits and hyphens, followed by `.md`
3. Right after `.md` there is none of an English letter or digit, `_`, `-`, `#` and `/` (the `path#anchor` of a source is not a reference)
4. It is not inside double quotes. When the number of double quotes on the line is odd, the text from the last quote to the end of the line is taken as inside quotes

## Exclusions

What kotowari does not read or look at, without raising a finding:

- Hidden directories
- Symbolic links to directories
- Anything that is neither a directory nor a regular file (sockets, named pipes, devices)
- Files other than `.md`
- Files named `.kotowari`
- The inside of code blocks (except gherkin). In decision records and in Markdown that is not a decision record (form contracts, supplementary documents, ADRs), gherkin is included too, and an unclosed block runs to the end of the document. `## ` headings inside it are not counted either
- In decision records, lines of the form of a numbered line and lines of the form of a supplementary line in a section that is none of the decision sections (`## Agreements`, `## Prohibitions`, `## Delegated`, `## Rejected`), `## Undecided` and `## Superseded` (including Revisions and `## Context`); lines of the form of a supplementary line before the first numbered line of a section; and lines that are neither numbered lines, supplementary lines nor headings
- `Scenario:` lines outside gherkin blocks
- Non-empty lines before the title
- Tables in a glossary document that are not the glossary table (tables before the glossary table whose header does not match, and tables after it)
- Marks outside tests and in the middle of a function body
- The inside of double quotes on a line
- From the last quote to the end of the line when the number of double quotes is odd
- Lines not checked for terms and vague words

Do not create skips that are not in this list.

## Limits and the unit of splitting

When the number of lines of one document exceeds the configuration's `limits.lines`, too_many_lines; when the number of requirements of a topic document exceeds `limits.requirements`, too_many_requirements. Both are notices (severity is `notice`; they do not change the exit code). Requirements are not counted in `CONTEXT.md` and `FLAGS.md`. The limits can be changed in the configuration, and the defaults are held by kotowari itself.

The unit of splitting is responsibility, not the number of lines or items. Read a notice as a signal to suspect mixed responsibilities.

- When a notice appears, reread the document. If requirements outside the scope line are mixed in, split into units the scope line can explain
- If none are mixed in, do not split; write the reason for keeping it in the decision record. Do not cut because of the number of lines or items
- A document with one or two requirements is merged with a neighbouring document the same scope line can explain
- Cut a directory when a term specific to that group appears
- For too_many_lines in a glossary, split it into the `CONTEXT.md` of each directory if the terms divide into per-directory groups; if they do not, raise `limits.lines` in the configuration and write that decision in the record
