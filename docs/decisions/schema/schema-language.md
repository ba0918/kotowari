# The skeleton of the schema language

English | [日本語](schema-language.ja.md)

Covers which rule kinds a schema has, how cardinality and conditional rules are written, and in which reading mode the lines under a heading are read. How a document points at a schema is decided in [the "Schema resolution" topic](schema-resolution.md).

## A schema is written in YAML, with the title, the preamble, sections and items directly under the document at its root

Status: implemented

### Problem

We want schemas written in a form that can be read with types. Three levels of headings (title, section and item) are enough for the shapes of documents targeted. Nesting of any depth makes both the rules and the extraction complex.

If items can be declared only under a section, the shape of a document that lists items with no section between cannot be checked.

### Decision

A schema is written as a YAML mapping. The root nodes are four: the title, the preamble, sections and items directly under the document.

Items directly under the document are declared in "document.items", as a list of rules in the same shape as "items" under a section. In a schema that declares it, the preamble ends before the first "## " or "### " heading. How the document's headings are read when it is declared and when it is not is decided in [A schema can declare items directly under the document, with no section between, in "document.items"](document-structure.md#a-schema-can-declare-items-directly-under-the-document-with-no-section-between-in-documentitems) and [In a schema that declares no items directly under the document, a depth-3 heading before the first section is a finding only when the preamble is declared or in the closed world](document-structure.md#in-a-schema-that-declares-no-items-directly-under-the-document-a-depth-3-heading-before-the-first-section-is-a-finding-only-when-the-preamble-is-declared-or-in-the-closed-world).

### Rejected options

- Write the schema as rule lines in Markdown. Turning them into a form a machine can check makes a notation of its own. YAML can be read with types.

### Confirmations

- C-schema-lang-1 A YAML schema that declares a title, a preamble, sections and the rule kinds under them is loaded as declared

## There are ten rule kinds, each with the places it can be placed

Status: implemented

### Problem

We want to be able to check items such as decision tables, which hold a table under their heading. It is asymmetric that tables and code blocks can be declared on sections but not on items.

No reason survives for deciding the other rule kinds and where they can be placed.

### Decision

One element of the schema tree is called a node, and is the unit of validation and extraction. The kinds of node (rule kinds) are the following ten.

| Rule kind | What it validates | Where it can be placed |
|---|---|---|
| Title | The depth-1 heading | The root of the schema |
| Preamble | The part after the title and before the first section. When items directly under the document are declared, the part before the first section or item | The root of the schema |
| Section | A depth-2 heading | The root of the schema |
| Item | A depth-3 heading | Under a section; the root of the schema (an item directly under the document) |
| Field line | A list line of the name-and-value form | Preamble, section, item, child of a bullet |
| Statement | A non-blank line that is neither a list nor a table | Preamble, section, item |
| Bullet | A list line that is not a field line | Preamble, section, item, child of a bullet |
| Table | A Markdown table | Preamble, section, item |
| Code block | A block enclosed by a fence | Preamble, section, item |
| Subheading ("subheading") | Headings of depth 4 or more | Preamble, section, item |

A table and a code block declared on an item have the same rule content, default cardinality and error kinds as when declared on a section. That the preamble itself declares no extraction is decided in [For the preamble, rules and extraction are declared on the nodes inside it](document-structure.md#for-the-preamble-rules-and-extraction-are-declared-on-the-nodes-inside-it).

## A schema with an unknown key or a value that does not fit its shape stops without checking

Status: implemented

### Problem

Silently skipping input that cannot be read, is broken, or does not fit its shape makes a document that was not checked look as though it passed.

### Decision

When a schema has a key that the rule kind does not accept, mds stops without checking. A schema that cannot be read as YAML, and a schema whose value does not fit the shape of the rule kind, stop in the same way. How a stop is reported (the exit code and the name of the stop reason) is decided in [There are five reasons for a stop](cli.md#there-are-five-reasons-for-a-stop).

### Confirmations

- C-schema-lang-3 A schema with a key the rule kind does not accept, including a key under a bullet, is not loaded, as a schema error
- C-schema-lang-4 A schema that cannot be read as YAML, and a schema whose value does not fit the shape of the rule kind (for example, a wrong type, a regular expression that cannot be read, or a conditional rule that does not have exactly one comparison written), are not loaded, as a schema error

## Paragraph is the default reading mode, and the schema can choose reading by line

Status: implemented

### Problem

mds is published, and changing the default reading would change the results of mds users. On the other hand, we also want document shapes that count the lines under a heading as one statement per line to be checkable without anything outside mds reading the raw lines.

We want the reading-mode declaration to read as a top-level setting like "open", and to have a shape that needs no new key when a reading mode is added. A value that is not accepted is treated the same as [an unknown key](#a-schema-with-an-unknown-key-or-a-value-that-does-not-fit-its-shape-stops-without-checking).

### Decision

The top-level "reading" key of the schema is accepted as the declaration of the reading mode. Only the two values "paragraph" (read as CommonMark paragraphs) and "line" (read one line at a time) are accepted, and when the key is not written it is read as "paragraph". A schema whose value is neither "paragraph" nor "line" stops.

How the lines under a heading are told apart in each reading mode is decided in [The lines under a heading are read in the reading mode the schema chooses (paragraph or line)](block-rules.md#the-lines-under-a-heading-are-read-in-the-reading-mode-the-schema-chooses-paragraph-or-line).

### Rejected options

- Change the engine's default reading to line by line. The results of mds users would change.
- The side using mds (kotowari) reads the raw lines and fills in the statements. This breaks the rule that kotowari does not read Markdown itself.

### Confirmations

- C-schema-lang-5 A schema that does not write "reading" and the same schema with only "reading: paragraph" added return the same output for the same document in both check and values, and count a two-line paragraph with no blank line between as one statement
- C-schema-lang-6 For a schema whose "reading" value is neither "paragraph" nor "line" (for example "reading: word"), check stops

## Cardinality is declared in five ways

Status: implemented

### Problem

No reason survives.

### Decision

Cardinality is the lower and upper bounds on how many times a node may appear, and is declared in the following five ways.

| How it is written | Meaning |
|---|---|
| Default (nothing written) | Exactly one |
| The required declaration set to false | Zero or one |
| Only a lower bound written | That number or more |
| Only an upper bound written | From zero up to that number |
| Both a lower and an upper bound written | That range |

A range declaration overrides the required declaration. Not meeting the lower bound is a repeat_min_not_met finding, and exceeding the upper bound is a repeat_max_exceeded finding.

### Confirmations

- C-schema-lang-7 If a node with no cardinality written is absent from the document, a missing finding is reported
- C-schema-lang-8 A node whose required declaration is set to false gets no finding when absent, and gets a repeat_max_exceeded finding when there are two
- C-schema-lang-9 If a node with a lower bound written falls short of it, a repeat_min_not_met finding is reported, and if a node with an upper bound written exceeds it, a repeat_max_exceeded finding is reported. A node with only an upper bound written gets no repeat_min_not_met finding even when absent
- C-schema-lang-10 A cardinality whose lower and upper bounds are written as the same number is not a schema error

## A node that declares a cardinality range is extracted as an array even for a single value

Status: implemented

### Problem

No reason survives.

### Decision

The extraction of a node that declares a cardinality range is an array even when there is one value. The extraction of a node that declares no range is a single value. This correspondence applies to field lines that declare no separator, statements, sections, items, titles and code blocks.

The shape of the extraction of bullets, tables and field lines that declare a separator is decided in [A table declared per element that repeats gets a level per table, and everything else is joined into one array](extraction.md#a-table-declared-per-element-that-repeats-gets-a-level-per-table-and-everything-else-is-joined-into-one-array) and [A field line that declares a delimiter becomes an array regardless of cardinality](extraction.md#a-field-line-that-declares-a-delimiter-becomes-an-array-regardless-of-cardinality).

### Confirmations

- C-schema-lang-11 A field line or section that declares a cardinality range is extracted as an array even for one value, and a field line that declares no range is extracted as a single value

## A conditional rule applies the attached constraint only while its condition is true

Status: implemented

### Problem

No reason survives.

### Decision

The conditional rule "when" can be attached to any of "required", "pattern" and "enum", and applies that constraint only while the condition is true. While the condition is false, the constraint is not applied. There are two comparisons, equal ("eq") and not equal ("ne"), and the right-hand side accepts only a literal string.

The field line a condition refers to is looked for only under the same node. When it is not found, an equality condition is treated as false and an inequality condition as true.

### Confirmations

- C-schema-lang-12 The constraint "when" is attached to applies only while the condition is true (a missing required field line gets a missing finding). While the condition is false it does not apply, and "when" does not stop the matching other than the attached constraint
- C-schema-lang-13 The field line a condition refers to is looked for under the same node (for a child of a bullet, among the sibling field lines), and when it is not found, an "eq" condition is false and an "ne" condition is true

## Section and item rules select headings by a list of names or by a regular expression

Status: implemented

### Problem

A section rule selects only headings equal to one name. An item rule is one per section, and it requires the "ID: name" form of every depth-3 heading. So documents of the following shapes cannot be declared.

- A document in which the same section can be written in either of two languages' spellings ("未決" and "Open")
- A document in which the text of section headings is free per document, and every depth-2 heading is read with the same rule
- A document that puts several items told apart by name ("### 問題" and "### 決めたこと") under one section. With one item rule they cannot be told apart, and a heading not in the "ID: name" form is invalid_id

kotowari's decision documents have all three, and could not be declared.

### Decision

Section rules and item rules select headings in one of the following ways.

| How it is written | Headings it selects |
|---|---|
| A string or a list of strings in "name" | Headings whose text equals one of them |
| A regular expression in "pattern" | Headings whose text contains a part that matches the regular expression. "pattern: """ matches every heading |
| An item rule with neither "name" nor "pattern" written (the ID rule) | Every depth-3 heading that no earlier rule selected |

A heading belongs to the first rule, in declared order, that selects it. The text of a heading is read as in [A section is identified by the text of its heading](document-structure.md#a-section-is-identified-by-the-text-of-its-heading). How a heading the ID rule selected is read is decided in [Only headings selected by an item rule with neither a name nor a regular expression are split into the ID before ":" and the name after it](document-structure.md#only-headings-selected-by-an-item-rule-with-neither-a-name-nor-a-regular-expression-are-split-into-the-id-before--and-the-name-after-it).

A depth-2 heading that no section rule selects is an undeclared_heading finding. When the list of item rules has no ID rule, a depth-3 heading that no item rule selects is an undeclared_heading finding, and the lines under it are also undeclared_line findings, block by block (treated the same as in [A depth-3 heading inside a section that declares no item is a finding](document-structure.md#a-depth-3-heading-inside-a-section-that-declares-no-item-is-a-finding)).

### Rejected options

- Previous decision: a section is identified by whether it equals the one name the schema declared, and a section that does not match a declared name is a finding (the first sentence of the Decision of [A section is identified by the text of its heading](document-structure.md#a-section-is-identified-by-the-text-of-its-heading)). It was dropped because a document that writes the same section in two spellings, and a section whose heading text is free, cannot be declared.
- Have only selection by regular expression, and write the candidate spellings as "^(問題|Problem)$". Findings would carry a regular expression as the node name.
- Give rules a separate key "label" for the name shown in findings. The first name of the list of names becomes a stable node name, so it is not needed.

### Confirmations

- C-schema-lang-14 In a schema that declares a section rule "name: [未決, Open]", neither a "## 未決" section nor a "## Open" section is a finding
- C-schema-lang-15 In a schema that declares a section rule "pattern: """ after the name rules, the section of every depth-2 heading that matches no name rule is not undeclared_heading, and is checked as a section of the "pattern" rule
- C-schema-lang-16 When two rules match one heading, the rule declared first selects it (when a "pattern: """ rule is declared before a "name: 未決" rule, "## 未決" is counted as a section of the "pattern" rule, and the "name: 未決" rule gets a missing finding)
- C-schema-lang-17 When the list of item rules has only name rules and regular-expression rules, a depth-3 heading that matches none of them is not invalid_id, with or without ":", but undeclared_heading, and the paragraphs and bullets under it are each undeclared_line too

## Item rules are written as a list in "items", and the "item" key is not accepted

Status: implemented

### Problem

An item rule can be written, only one, in "item" under a section and in "document.item" directly under the document. To declare several items told apart by name in one section ([Section and item rules select headings by a list of names or by a regular expression](#section-and-item-rules-select-headings-by-a-list-of-names-or-by-a-regular-expression)), a list of rules is needed.

### Decision

Item rules are written as a list of rules, in "items" under a section and in "document.items" directly under the document.

The keys "item" and "document.item" are not accepted. A schema that writes them stops without checking, as in [A schema with an unknown key or a value that does not fit its shape stops without checking](#a-schema-with-an-unknown-key-or-a-value-that-does-not-fit-its-shape-stops-without-checking).

A list with only one rule that has neither "name" nor "pattern" ("items: [X]") is read the same as the rule written as "item: X".

### Rejected options

- Previous decision: an item rule is written, only one, in "item" under a section and in "document.item" directly under the document (the second paragraph of the Decision of [A schema is written in YAML, with the title, the preamble, sections and items directly under the document at its root](#a-schema-is-written-in-yaml-with-the-title-the-preamble-sections-and-items-directly-under-the-document-at-its-root)). It was dropped because several item rules cannot be written for one section.
- Accept both a single rule and a list of rules. The same thing would have two ways of being written. Rewriting existing schemas only needs "item: X" changed to "items: [X]".

### Confirmations

- C-schema-lang-18 A schema that writes "items" under a section and "document.items" directly under the document is loaded, and a schema that writes "item" under a section or "document.item" directly under the document is not loaded, as a schema error
- C-schema-lang-19 A schema in which "item: X" is rewritten as "items: [X]" returns, for the same document, the same findings and the same extracted values as before the rewrite

## The preamble, a section, an item and the children of a bullet accept blocks that have no rule when they declare "open: true"

Status: implemented

### Problem

Inside a declared preamble, section or item, a block that has no rule is a finding even in the open world. Shapes such as an item of free prose, or a bullet line under which a code block or a nested list can be written, cannot be fully written by declaring a rule for each kind of block. Ordered lists have no rule kind, so there is not even a means of declaring them.

### Decision

"open: true" can be declared on the preamble, a section, an item, and the child rules of a bullet ("children"). A node that declares "open: true" accepts, together with their contents, blocks of the kinds that have no rule on that node, and does not make them findings.

Even on a node that declares "open: true", the rules declared on that node keep all their constraints. Headings of depth 4 or more are not accepted ([Headings of depth 4 or more are read as lines of the preamble, section or item that contains them, and are findings unless "subheading" is declared](document-structure.md#headings-of-depth-4-or-more-are-read-as-lines-of-the-preamble-section-or-item-that-contains-them-and-are-findings-unless-subheading-is-declared)). The treatment of depth-2 and depth-3 headings does not change either.

"open: true" is something a node declares as its own content, and it is separate from the top-level "open" of the schema and `--open` of `check` ([The open world allows only undeclared structures and what is inside them](closed-world.md#the-open-world-allows-only-undeclared-structures-and-what-is-inside-them)). A node that does not write "open" makes blocks that have no rule findings.

### Rejected options

- Previous decision: undeclared structures and lines added inside a declared preamble, section or item are not allowed even in the open world (the first item of the second paragraph of the Decision of [The open world allows only undeclared structures and what is inside them](closed-world.md#the-open-world-allows-only-undeclared-structures-and-what-is-inside-them)). Since there was no means for a node to declare that it accepts content itself, and an item of free prose could not be declared, it does not apply to a node that declares "open: true".
- Add a rule for ordered lists, and a container of the same shape as the preamble under the child rules. That is three additions against "open", which is one.
- Have "open" on a node also accept headings of depth 4 or more. A node that accepts headings of depth 4 or more while making ordered lists findings could not be declared. An ordered list that is not a finding passes silently without being read as bullets.

### Confirmations

- C-schema-lang-20 On an item that declares "open: true", paragraphs, ordered lists, tables, code blocks and block quotes that have no rule are not findings, and on an item of the same shape that does not write "open", each is undeclared_line
- C-schema-lang-21 A line that does not match the regular expression of a bullet rule declared on an item that declares "open: true" is bullet_pattern_mismatch
- C-schema-lang-22 When "open: true" is declared on the child rules of a bullet, the child list, table and code block under a bullet line are not undeclared_line
- C-schema-lang-23 Even inside an item that declares "open: true", a heading of depth 4 or more is heading_level_mismatch unless "subheading" is declared
- C-schema-lang-24 Even when one item declares "open: true", a block that has no rule inside another item that does not write "open" is undeclared_line

## A rule for headings of depth 4 or more, "subheading", can be declared on the preamble, a section and an item

Status: implemented

### Problem

Headings of depth 4 or more have no rule kind, and are findings wherever they are. There is no means of declaring a document whose items hold subheadings. kotowari's decision documents are a form that allows headings of depth 4 or more inside the sections of a decision, and could not be declared.

### Decision

A rule for headings of depth 4 or more, "subheading", is added to the rule kinds. "subheading" can be declared on the preamble, a section and an item. A cardinality ([Cardinality is declared in five ways](#cardinality-is-declared-in-five-ways)) and a regular expression can be written on it; an extraction cannot.

The table of rule kinds and the places they can be placed ([There are ten rule kinds, each with the places it can be placed](#there-are-ten-rule-kinds-each-with-the-places-it-can-be-placed)) becomes ten rule kinds, with the following one row added.

| Rule kind | What it validates | Where it can be placed |
|---|---|---|
| Subheading ("subheading") | Headings of depth 4 or more | Preamble, section, item |

"subheading" is a rule that counts and checks the lines of headings of depth 4 or more, not a node that holds the lines under it. Which node a heading of depth 4 or more is read as a line of is decided in [Headings of depth 4 or more are read as lines of the preamble, section or item that contains them, and are findings unless "subheading" is declared](document-structure.md#headings-of-depth-4-or-more-are-read-as-lines-of-the-preamble-section-or-item-that-contains-them-and-are-findings-unless-subheading-is-declared).

### Rejected options

- Previous decision: there are nine rule kinds, with no rule for headings of depth 4 or more ([There are ten rule kinds, each with the places it can be placed](#there-are-ten-rule-kinds-each-with-the-places-it-can-be-placed)). It was dropped because subheadings inside an item cannot be declared.

### Confirmations

- C-schema-lang-25 (by reading) The public structs of `crates/kotowari-markdown-schema/src/schema.rs` correspond one to one to the ten rule kinds, the nine rule kinds plus the subheading, and the struct of "subheading" can be placed only on the preamble, a section and an item
- C-schema-lang-26 In a schema that declares "subheading: { repeat: { min: 1, max: 1 } }" on an item, an item with no heading of depth 4 or more gets repeat_min_not_met, and an item with two gets repeat_max_exceeded
- C-schema-lang-27 A schema that writes "extract" in "subheading" is not loaded, as a schema error
