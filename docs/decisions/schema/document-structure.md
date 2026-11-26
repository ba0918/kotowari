# The skeleton of a document

English | [日本語](document-structure.ja.md)

Covers how the schema validates the skeleton of a document: the title, the preamble, sections and items. How the lines under a heading are told apart and constrained is decided in [Rules for the lines under a heading](block-rules.md), whether undeclared headings and lines are allowed in [the "Closed world and open world" topic](closed-world.md), and that a cardinality finding carries the rule kind it counted in [Cardinality findings hold the rule kind of the counted node](cli.md#cardinality-findings-hold-the-rule-kind-of-the-counted-node).

## The skeleton of headings is closed at three levels: title, section and item

Status: implemented

### Problem

It needs to be decided how deep headings may nest. Three levels are enough for the shapes of documents targeted. Nesting of any depth makes both the rules and the extraction complex.

### Decision

The heading depths a schema can declare are only three levels: the title (depth 1), sections (depth 2) and items (depth 3). There is no recursive nesting of any depth, and there is no means to declare a further heading node under an item.

### Confirmations

- C-schema-doc-2 (by reading) The schema language has no means to declare a heading node under an item

## Exactly one title is required only when the schema declares a title

Status: implemented

### Problem

If "exactly one title is always required", then in the closed world the title of a schema that declares no title would be an undeclared heading, and the two rules could not both hold. We also want documents of a form that has no title to be writable.

### Decision

When the schema declares a title, the document is required to have exactly one title. When there is no title, one missing_title finding is reported.

A schema that declares no title does not require one. If the document has a title, it is an undeclared heading ([The default is the closed world, and undeclared headings and lines are findings](closed-world.md#the-default-is-the-closed-world-and-undeclared-headings-and-lines-are-findings)).

### Rejected options

- Require a title even in a schema that declares none. A form with no title could not be expressed.

### Confirmations

- C-schema-doc-3 In a schema that declares a title, a document with no title gets one missing_title finding, and that finding has no line number
- C-schema-doc-4 In a schema that declares no title, the exit code of "kotowari-mds check" is 0 for a document with no title, and for a document with a title, that title is an undeclared-heading finding in the closed world

## Each title from the second on is one finding

Status: implemented

### Problem

What a person reading the finding wants to know is the title to delete. If only one finding is reported however many titles there are, the titles from the third on are not reported.

### Decision

When there are two or more titles, one multiple_titles finding is reported for each title from the second on. The finding carries the line number and the raw line of that title.

### Confirmations

- C-schema-doc-5 In a document with two or more titles, one multiple_titles finding is reported for each title from the second on (two for three titles), and the line number and raw line of each are that title's line and its text exactly as written

## Even with two or more titles, the first title is matched against the regular expression

Status: implemented

### Problem

Having several titles and a title not matching the regular expression are separate problems. There is no reason to hide one when reporting the other. Hiding it means the mistake in the first title is noticed only after the second title is deleted.

### Decision

When the schema declares a regular expression for the title, the first title is matched even when there are two or more titles, and if it does not match, a title_pattern_mismatch finding is reported as well.

### Rejected options

- Do not match against the regular expression when there are several titles.

### Confirmations

- C-schema-doc-6 In a schema that declares a regular expression for the title, if the first title does not match, a title_pattern_mismatch finding is reported on that line even when there are two or more titles, and a multiple_titles finding is reported on the line of the second title

## A title from the second on ends the sections and items before it

Status: implemented

### Problem

The range of an extracted element ([The last line of an element is the line before the next heading of the same depth or shallower](extraction.md#the-last-line-of-an-element-is-the-line-before-the-next-heading-of-the-same-depth-or-shallower)) already used the title as a boundary, and the last line of an item was the line before the title. Even so, only when building the document tree were the lines after the title put into the previous item. The way of dividing is made one.

### Decision

A title from the second on ends the sections and items before it. The lines after it are read as lines of the preamble until the next section.

### Rejected options

- Keep the current building, which puts the following lines into the previous item, and change the range of elements instead. The lines after the title would be output as the body of the previous item.

### Confirmations

- C-schema-doc-7 For a document that has a second title and lines after it following an item's statement, with "kotowari-mds values --format json" the item's statement element holds only the lines before the second title

## For the preamble, rules and extraction are declared on the nodes inside it

Status: implemented

### Problem

The preamble is the part after the title and before the first section, and can hold field lines, statements, bullets, tables and code blocks. When the nodes that can declare extraction were put in a table, the preamble was listed too, but the implemented preamble had no extraction key, and a schema that wrote one was a schema error as soon as it was written. This was corrected as a mistake of listing by analogy without checking the real thing.

### Decision

The preamble itself has no extraction key. Extraction and rules for the preamble are declared on the nodes inside it (field lines, statements, bullets, tables and code blocks).

### Confirmations

- C-schema-doc-8 A table and a code block declared inside the preamble are not undeclared-line findings, and extraction declared on a table of the preamble returns per-row values keyed by the declared header names
- C-schema-doc-9 A schema that writes extraction on the preamble itself is a stop

## A section is identified by the text of its heading

Status: implemented

### Problem

The default of checking is the closed world, and its purpose is to fix the format, so silently letting undeclared sections through would defeat that purpose. Whether a section is the same as a declaration is told by the text of its heading. However, while the text inside emphasis, bold, links and code was kept in the name, only strikethrough was discarded together with the text inside it. So a heading with the same name as a declared one became an undeclared section, and a missing required section was reported too.

### Decision

A section is identified by the text of its heading, and a section that matches no name the schema declares is made a finding. The text of a heading is the text with inline markup removed, and the text inside strikethrough is kept too.

### Confirmations

- C-schema-doc-10 A section heading that matches no name the schema declares is an undeclared-heading finding, and if a declared section is absent, a finding that the section is missing is reported
- C-schema-doc-11 In a schema that declares the section "Ax", a document with a section whose heading consists of "A" and "x" enclosed in strikethrough gets exit code 0 from "kotowari-mds check"

## An item heading is read with the part before ":" as the ID and the part after it as the name

Status: implemented

### Problem

An item is a depth-3 heading with an ID and a name. It needs to be decided how much of the heading is the ID and how the shape of the ID is constrained. No reason survives.

### Decision

An item heading is read as follows.

| Order | Shape of the heading | Read as |
|---|---|---|
| 1 | No separating colon | A finding that the ID does not fit its shape |
| 2 | Has a colon, and the part before it matches the regular expression of the schema | From the start up to just before ":" as the ID, and the rest as the name |
| 3 | Has a colon, and the part before it does not match the regular expression | A finding that the ID does not fit its shape |

A finding that the ID does not fit its shape is invalid_id.

### Confirmations

- C-schema-doc-12 An item heading with no ":" and an item heading whose ID does not match the schema's regular expression are invalid_id findings

## A depth-3 heading inside a section that declares no item is a finding

Status: implemented

### Problem

The default of checking is the closed world, and its purpose is to fix the format, so silently letting undeclared headings through would defeat that purpose. No reason survives other than the reason for the closed world.

### Decision

When there is a depth-3 heading inside a section that declares no item, that heading and the lines inside it are made findings. The same holds in the open world ([The open world allows only undeclared structures and what is inside them](closed-world.md#the-open-world-allows-only-undeclared-structures-and-what-is-inside-them)).

### Confirmations

- C-schema-doc-13 A depth-3 heading inside a section that declares no item is an undeclared-heading finding even in the open world, and the lines inside it are undeclared-line findings too

## A schema can declare items directly under the document, with no section between, in "document.item"

Status: implemented

### Problem

We want to read documents that list items with no section between (kotowari's problem records were like this) in that shape as they are. In the schema language, items could be declared only under a section. The behavior of a schema that does not declare them is not changed.

### Decision

A schema can declare items directly under the document in "document.item", in the same shape as "item" under a section (accepting it as a root node is decided in [A schema is written in YAML, with the title, the preamble, sections and items directly under the document at its root](schema-language.md#a-schema-is-written-in-yaml-with-the-title-the-preamble-sections-and-items-directly-under-the-document-at-its-root)). When it is declared, a depth-3 heading before the first section is read as that item, and the preamble ends before the first section or item. The same document may have both items under a section and items directly under the document.

### Confirmations

- C-schema-doc-14 In a schema that declares both "document.item" and "item" under a section, both an item of a depth-3 heading after the preamble's statement with no section between and an item under a section get no finding with "kotowari-mds check --format json" and appear in the values of "kotowari-mds values --format json", and the value of the preamble's statement does not include the lines of the item directly under the document

## In a schema that declares no items directly under the document, a depth-3 heading before the first section is a finding only when the preamble is declared or in the closed world

Status: implemented

### Problem

It needs to be decided how a schema that declares no items directly under the document treats a depth-3 heading before the first section. We want the same treatment as an undeclared item in a section ([A depth-3 heading inside a section that declares no item is a finding](#a-depth-3-heading-inside-a-section-that-declares-no-item-is-a-finding)), but bringing it over as it is would bring along the rule of the open world too. The open world allows undeclared structures and what is inside them, except inside a declared preamble, section or item, where it does not allow them ([The open world allows only undeclared structures and what is inside them](closed-world.md#the-open-world-allows-only-undeclared-structures-and-what-is-inside-them)).

A heading before the title, including one in a document with no title, is outside the preamble. A document with no title gets a separate finding on the missing title, so no harm is done.

### Decision

In a schema that declares no items directly under the document, when the preamble is declared or in the closed world, a depth-3 heading before the first section and the lines inside it (field lines and statements alike) are made findings. In the open world without a declared preamble, they are allowed as undeclared structures.

A depth-3 heading before the title, including one in a document with no title, is taken to be outside the preamble, and in the open world that heading and the lines inside it are allowed as undeclared structures even when the preamble is declared.

### Confirmations

- C-schema-doc-15 In a schema that declares items only under sections, a depth-3 heading before the first section gets a finding, and in a schema that declares the preamble, a depth-3 heading inside the preamble and the lines inside it get findings even in the open world
- C-schema-doc-16 In a schema with "open: true" that declares neither items directly under the document nor the preamble, neither the heading "### X-1: a" before the first section nor the lines under it get a finding
- C-schema-doc-17 In a schema with "open: true" that declares the title and the preamble and declares no items directly under the document, the heading "### a" before the first section of a document with no title and the lines under it get no finding, and the only finding is the one on the missing title

## When items are lacking, it is a finding of falling below the lower bound, whether or not a cardinality is written

Status: implemented

### Problem

There is no finding kind that expresses a missing item. The default cardinality is also a cardinality, "exactly one", so expressing it with a lower-bound finding is consistent. The implementation also always counted items by cardinality.

### Decision

When items are lacking, it is a repeat_min_not_met finding as falling below the lower bound, whether or not a cardinality is written. When no cardinality is written, the lower bound is 1, from the default of exactly one ([Cardinality is declared in five ways](schema-language.md#cardinality-is-declared-in-five-ways)). In this case the missing-section finding missing_required_section is not reported. missing_required_section is reported only for a missing section.

### Confirmations

- C-schema-doc-18 In a schema that declares on a section an item with no cardinality written, only one repeat_min_not_met finding is reported on the line of a section that has no item, and no missing_required_section finding is reported

## Headings of depth 4 or more are read as lines of the preamble, section or item that contains them, and are findings unless "subheading" is declared

Status: implemented

### Problem

A heading of depth 4 or more is a heading_level_mismatch finding wherever it is. A document that puts subheadings in an item has no way through except deleting the subheadings. If a subheading ended the item, the lines after the subheading would fall out of the item's check and extraction.

### Decision

A heading of depth 4 or more is read as a line inside the preamble, section or item that contains it. It does not end that node, and the lines after it are also read as lines of the same node. The last line of items and sections is as in [The last line of an element is the line before the next heading of the same depth or shallower](extraction.md#the-last-line-of-an-element-is-the-line-before-the-next-heading-of-the-same-depth-or-shallower), and is not cut by headings of depth 4 or more.

If the node that contains a heading of depth 4 or more declares "subheading" ([A rule for headings of depth 4 or more, "subheading", can be declared on the preamble, a section and an item](schema-language.md#a-rule-for-headings-of-depth-4-or-more-subheading-can-be-declared-on-the-preamble-a-section-and-an-item)), the heading is checked against that rule. If it does not, the heading is a heading_level_mismatch finding. The same holds inside a node that declares "open: true", and inside an undeclared structure that the open world allows.

### Rejected options

- Previous decision: a heading of depth 4 or more is a finding (the second paragraph of the Decision of [The skeleton of headings is closed at three levels: title, section and item](#the-skeleton-of-headings-is-closed-at-three-levels-title-section-and-item)). It was dropped because there is no way to let subheadings inside an item through.

### Confirmations

- C-schema-doc-20 In a schema that does not declare "subheading", a heading of depth 4 or more is heading_level_mismatch, even inside an item that declares "open: true" and inside an undeclared section in the open world
- C-schema-doc-21 Bullets after a "####" heading inside an item are matched by the bullet rule of the same item and enter that item's extraction, and that item's "end" is the line before the next "###", "##" or "#" heading
- C-schema-doc-22 A heading of depth 4 or more inside an item that declares "subheading" gets no heading_level_mismatch

## Only headings selected by an item rule with neither a name nor a regular expression are split into the ID before ":" and the name after it

Status: proposed

### Problem

Every item heading is read with the part before ":" as the ID and the part after it as the name, and a heading without ":" is invalid_id. A heading such as "### 問題" selected by an item rule that selects by name ([Section and item rules select headings by a list of names or by a regular expression](schema-language.md#section-and-item-rules-select-headings-by-a-list-of-names-or-by-a-regular-expression)) would be required to have the ID form too.

### Decision

Among item rules, only headings selected by a rule that has neither "name" nor "pattern" (the ID rule) are read as follows.

| Order | Shape of the heading | How it is read |
|---|---|---|
| 1 | No separating colon | A finding that it does not fit the ID form |
| 2 | A colon, and the part before it matches the schema's regular expression | The part from the start up to just before ":" is read as the ID, and the rest as the name |
| 3 | A colon, and the part before it does not match the regular expression | A finding that it does not fit the ID form |

The finding that a heading does not fit the ID form is invalid_id.

A heading selected by a "name" or "pattern" rule is not required to have the ID form. The name of that item is the whole text of the heading, and the item has no ID.

### Rejected options

- Previous decision: every item heading is read with the part before ":" as the ID and the part after it as the name ([An item heading is read with the part before ":" as the ID and the part after it as the name](#an-item-heading-is-read-with-the-part-before--as-the-id-and-the-part-after-it-as-the-name)). Since the ID form would also be required of the headings of items selected by name, it applies only to the ID rule.

### Confirmations

- C-schema-doc-23 An item heading without ":" selected by the ID rule, and an item heading whose ID does not match the regular expression of that rule, are invalid_id findings
- C-schema-doc-24 "### 問題" selected by an item rule "name: 問題" gets no invalid_id, and when "id" and "name" are declared in "of" of that item's extraction, the value of "name" is "問題" and there is no "id" key

## A schema can declare items directly under the document, with no section between, in "document.items"

Status: proposed

### Problem

We want to read a document that lists items with no section between (kotowari's issue records were like this) in that shape. Item rules are written in the list "items" under a section ([Item rules are written as a list in "items", and the "item" key is not accepted](schema-language.md#item-rules-are-written-as-a-list-in-items-and-the-item-key-is-not-accepted)). Items directly under the document should be writable in the same shape. The behaviour of schemas that do not declare it does not change.

### Decision

A schema can declare the rules for items directly under the document in "document.items", as a list of the same shape as "items" under a section. When it is declared, a depth-3 heading before the first section is read as an item that a rule of that list selects, and the preamble ends before the first section or item. The same document may have both items under sections and items directly under the document.

### Rejected options

- Previous decision: items directly under the document are declared with one rule in "document.item" ([A schema can declare items directly under the document, with no section between, in "document.item"](#a-schema-can-declare-items-directly-under-the-document-with-no-section-between-in-documentitem)). It was dropped because item rules were made a list in "items".

### Confirmations

- C-schema-doc-25 In a schema that declares both "document.items" and "items" under a section, neither the item of a depth-3 heading with no section between, after a statement of the preamble, nor the item under a section gets a finding in "kotowari-mds check --format json"; both appear in the values of "kotowari-mds values --format json", and the value of the preamble's statement does not include the lines of the item directly under the document
