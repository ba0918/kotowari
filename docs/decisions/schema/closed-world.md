# Closed world and open world

English | [日本語](closed-world.ja.md)

Covers how checking treats headings and lines not declared in the schema, and how far that can be relaxed. The form of findings and the CLI options other than `--open` are decided in [the "The CLI and how results are output" topic](cli.md), and the combination of the library's validation setting and the schema's `open` in [Validation in the library does not cancel the schema's open](library-extraction.md#validation-in-the-library-does-not-cancel-the-schemas-open).

## The default is the closed world, and undeclared headings and lines are findings

Status: implemented

### Problem

mds is a tool for mechanically fixing the format of Markdown written by an LLM. Since the purpose is to fix the format, silently letting through what is not declared in the schema would defeat that purpose.

### Decision

The default of checking is the closed world. A heading not declared in the schema is an `undeclared_heading` finding, and an undeclared line is an `undeclared_line` finding.

A title in a document whose schema declares no title, and a heading placed before the title, are also made findings as undeclared headings. The lines inside an undeclared heading are also made findings as undeclared lines. How the child lines of a bullet are declared is decided in [The child list of a bullet is checked against the declared child rules](block-rules.md#the-child-list-of-a-bullet-is-checked-against-the-declared-child-rules).

### Confirmations

- C-schema-closed-1 In the closed world, an undeclared section, item, title or heading before the title is `undeclared_heading`, and the lines inside it and the other undeclared lines (for example, lines in the preamble or a section, and lines inside a bullet) are `undeclared_line`

## The open world allows only undeclared structures and what is inside them

Status: implemented

### Problem

Silently letting through what is not declared defeats the purpose of fixing the format. Even with a means of relaxing, if it also removed the rules that were declared, it would defeat the purpose in the same way. No reason survives for having a means of relaxing at all.

### Decision

There are two means of relaxing: the top-level `open: true` of the schema, and `--open` of `check`. When either is present, checking is done in the open world, and undeclared structures and all the lines inside them are allowed. A heading before the title is also allowed, as one of the undeclared structures.

The means of relaxing work only in the direction of relaxing. Even in the open world, the following are not allowed.

- Undeclared structures added inside a declared preamble, section or item, and undeclared lines added inside one that does not declare "open: true" ([The preamble, a section, an item and the children of a bullet accept blocks that have no rule when they declare "open: true"](schema-language.md#the-preamble-a-section-an-item-and-the-children-of-a-bullet-accept-blocks-that-have-no-rule-when-they-declare-open-true))
- A missing required node
- A cardinality violation, and a shape violation

How a depth-3 heading before the first section is treated in a schema that declares no preamble is decided in [In a schema that declares no items directly under the document, a depth-3 heading before the first section is a finding only when the preamble is declared or in the closed world](document-structure.md#in-a-schema-that-declares-no-items-directly-under-the-document-a-depth-3-heading-before-the-first-section-is-a-finding-only-when-the-preamble-is-declared-or-in-the-closed-world). That the extraction result does not change between the closed world and the open world is decided in [The result of extraction is the same whether validated in the closed world or the open world](extraction.md#the-result-of-extraction-is-the-same-whether-validated-in-the-closed-world-or-the-open-world).

### Confirmations

- C-schema-closed-2 When the schema declares `open: true` (including checking a directory), and when `--open` is given to `check`, undeclared sections, lines of the preamble and headings before the title, and the lines inside them, get no finding. The same holds for the title of a schema that declares no title
- C-schema-closed-3 Even in the open world, undeclared headings added inside a declared preamble, section or item, and undeclared lines added inside one that does not declare "open: true", are findings
- C-schema-closed-4 Even in the open world, a missing required node is a finding
- C-schema-closed-5 For any document, the set of findings reported in the open world is contained in the set of findings reported when the same document is checked in the closed world

## A finding on an undeclared line carries the kind the line was read as

Status: implemented

### Problem

If findings on undeclared lines have only one kind, the caller has to tell whether the line was a field line or a bullet by reading the wording of the finding's detail, or by re-reading the characters of the raw line. Relying on the wording of the detail makes the caller break silently when the wording is changed. Re-reading the raw line puts the decision of telling lines apart in two places, mds and the caller. mds knows how it read the line at the point it makes the finding.

### Decision

A finding on an undeclared line carries, as structure, which of these the line was read as: a list line of the name-and-value form (a field line), a bullet, an ordered list, a statement, a table, a code block, a block quote, an HTML block or a thematic break.

### Rejected options

- The caller decides the kind by looking at the first characters of the raw line. The decision of telling lines apart would be in two places.

### Confirmations

- C-schema-closed-6 An `undeclared_line` finding carries the kind the line was read as: an undeclared field line has the field line kind, a bullet line the bullet kind, a line of an ordered list the ordered list kind, and a table the table kind (the same in the output of `check --format json`)
