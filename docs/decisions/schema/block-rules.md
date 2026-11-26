# Rules for the lines under a heading

English | [日本語](block-rules.ja.md)

Covers whether the lines under a heading or the preamble are read as a field line, a statement, a bullet, a table or a code block, and the constraints a schema can impose on each kind of line. The skeleton of headings (title, sections and items) is decided in [The skeleton of a document](document-structure.md).

## A list line is read as a field line when its name matches a declared name, and as a bullet when it does not

Status: implemented

### Problem

Some list lines have the form "name and value". It needs to be decided when such a line is read as a field line and when as an ordinary bullet. Also, in Markdown the list markers "-", "*" and "+" all make the same list, and it would be a surprise if the meaning changed with which one the writer uses.

### Decision

List lines are told apart in the following order.

| Order | Shape of the line | Read as |
|---|---|---|
| 1 | The marker is followed by "name and value", and the name matches a declaration of the schema | Field line |
| 2 | Follows a marker but does not match 1 | Bullet |
| 3 | Starts with a number and a "." or ")" delimiter | Belongs to no rule kind ([A list line that starts with a number is not made a bullet](#a-list-line-that-starts-with-a-number-is-not-made-a-bullet)) |

The markers "-", "*" and "+" are treated the same. Whichever is used, the same line is read as the same rule kind.

### Confirmations

- C-schema-block-1 A "name and value" line whose name the schema declares is read as a field line, and a "name and value" line whose name is not declared is read as a bullet and enters the matching and the count of the bullet rules
- C-schema-block-2 Whether the list marker is "-", "*" or "+", and even when they are mixed line by line, the same line is read as the same rule kind and gives the same finding (property)

## A list line that starts with a number is not made a bullet

Status: implemented

### Problem

It needs to be decided whether a line of an ordered list starting with a number is read as a bullet, and also whether only "." is accepted as the delimiter or ")" as well. No reason survives other than the one about the ")" delimiter.

### Decision

A list line that starts with a number and a delimiter is not treated as a bullet. It belongs to no rule kind, and in the closed world ([The default is the closed world, and undeclared headings and lines are findings](closed-world.md#the-default-is-the-closed-world-and-undeclared-headings-and-lines-are-findings)) it is an undeclared_line finding.

Both "." and ")" are accepted as the delimiter, as in CommonMark. Whether reading by paragraph or by line, mds follows CommonMark and reads "1)" as a line of an ordered list.

### Rejected options

- Do not read a line with the ")" delimiter as a list line, and make it a statement. How CommonMark reads it when reading by paragraph and how it is read when reading by line would differ.

### Confirmations

- C-schema-block-3 A list line that starts with a number and "." or ")" enters neither the matching of the bullet rules, nor the count of bullets, nor the extraction of bullets, and in the closed world it is an undeclared_line finding

## A regular expression and an allow list can be imposed on the value of a field line

Status: implemented

### Problem

We want a schema to be able to constrain the shape of a field line's value. For a field line whose value is written as several elements divided by a separator, the unit of matching also needs to be decided. No reason survives.

### Decision

A regular expression and an allow list can be imposed on the value of a field line. A value that does not match the regular expression is a field_pattern_mismatch finding, and a value not in the allow list is a field_enum_invalid finding.

For a field line that declares a separator, each separated element is matched after the whitespace before and after it is removed. An element that is empty after separating is matched too.

### Confirmations

- C-schema-block-4 If the value of a field line does not match the regular expression it is a field_pattern_mismatch finding, and if it is not in the allow list it is a field_enum_invalid finding
- C-schema-block-5 A field line that declares a separator is matched per separated element with the whitespace before and after removed, and an empty element remains a target of matching and of the values

## A regular expression can be imposed on statements and bullet lines, and an allow list on statements too

Status: implemented

### Problem

We want a schema to be able to constrain the shape of statements and bullet lines too. No reason survives.

### Decision

A regular expression and an allow list can be imposed on a statement. A statement that does not satisfy the regular expression is a statement_pattern_mismatch finding, and a statement not in the allow list is a statement_enum_invalid finding.

A regular expression can be imposed on a bullet line. A line that does not satisfy it is a bullet_pattern_mismatch finding. Matching is done only on the part of the original marker line without the marker. Continuation paragraphs, and lines that follow the marker line with no blank line between, are not matched.

### Confirmations

- C-schema-block-6 If a statement does not match the regular expression it is a statement_pattern_mismatch finding, and if it is not in the allow list it is a statement_enum_invalid finding
- C-schema-block-7 If a bullet line does not match the regular expression it is a bullet_pattern_mismatch finding; matching is done only on the part of the marker line without the marker, and continuation paragraphs and lines following the marker line are not matched

## The order of field lines is matched only when enforcing it is declared

Status: implemented

### Problem

Some formats want field lines in a fixed order, and some do not care about the order. No reason survives.

### Decision

By default the order of field lines is free. When the schema declares that the order is enforced, field lines are required to appear in the order written in the schema, and a field line that does not appear in that order is a field_order_mismatch finding.

### Confirmations

- C-schema-block-8 In a schema that declares the order enforced, a field line that does not appear in the order written in the schema is a field_order_mismatch finding, and field lines in the written order get no finding

## The lines under a heading are read in the reading mode the schema chooses (paragraph or line)

Status: implemented

### Problem

mds is published, and reads the lines under a heading as CommonMark paragraphs. Changing that default would change the results of mds users.

On the other hand, the statements kotowari required of specification documents were one line each, a different reading from CommonMark paragraphs. Read as paragraphs, the line right after a field line was absorbed into its value and produced off-target findings, and quote and HTML lines silently disappeared, becoming neither statements nor findings. kotowari re-reading the raw lines itself would break the rule "kotowari does not read Markdown".

So the default reading is kept, and the reading needed can be chosen by a declaration in the schema. The declaration reads as a top-level setting like "open", and has a shape that needs no new key when a reading mode is added.

### Decision

The top-level "reading" of the schema chooses how the lines under headings and the preamble are read. "paragraph" (reading by paragraph) is the default, and is the CommonMark reading. "line" (reading by line) makes each non-blank line that is neither a list line nor a table line one statement. The values "reading" accepts are decided in [Paragraph is the default reading mode, and the schema can choose reading by line](schema-language.md#paragraph-is-the-default-reading-mode-and-the-schema-can-choose-reading-by-line).

In either reading mode, list lines themselves are told apart as in [A list line is read as a field line when its name matches a declared name, and as a bullet when it does not](#a-list-line-is-read-as-a-field-line-when-its-name-matches-a-declared-name-and-as-a-bullet-when-it-does-not), and child lists are as in [The child list of a bullet is checked against the declared child rules](#the-child-list-of-a-bullet-is-checked-against-the-declared-child-rules).

| Shape of the line | "paragraph" | "line" |
|---|---|---|
| A CommonMark ATX heading line ([Headings and tables are read as CommonMark and GFM specify in either reading mode](#headings-and-tables-are-read-as-commonmark-and-gfm-specify-in-either-reading-mode)) | Heading | Heading |
| Any other line starting with "#" ("#foo", a line of seven or more "#") | Statement | Statement |
| A line of only "===" or "---" following a statement | One heading together with the previous line | Both the previous line and that line are statements |
| Several consecutive lines, not separated by blank lines, that are neither a list nor a table | Together one statement | Each line one statement |
| A non-list line directly following a list line with no blank line between | Part of the preceding list line (for a field line, it goes into the value) | Statement |
| An indented non-list line following a list line after a blank line | Continuation paragraph | Statement |
| An indented list line following a list line | List line | List line |
| A quote, thematic break, HTML or image line | A quote, a thematic break and an HTML block are undeclared_line inside a node that does not declare "open: true" ([When reading by paragraph, block quotes, HTML blocks and thematic breaks inside a node that does not declare "open: true" are findings](#when-reading-by-paragraph-block-quotes-html-blocks-and-thematic-breaks-inside-a-node-that-does-not-declare-open-true-are-findings)); an image line is not counted as a statement, and not made a finding | Statement |
| A line indented by four or more spaces after a blank line, not following a list line | Indented code block | Statement (a line opening a fence is as in the next row) |
| A block enclosed in a "```" or "~~~" fence | Code block | Code block (whatever the indentation depth of the opening line) |
| A GFM table (a run with a header row and a delimiter row; it need not start with a vertical bar; it ends as GFM specifies) | Table | Table |
| A line starting with a vertical bar that has no delimiter row and does not form a table | Statement | Statement |

How following lines and continuation paragraphs are treated when reading by paragraph is decided in [When reading by paragraph, the lines following a list line and continuation paragraphs are part of that list line](#when-reading-by-paragraph-the-lines-following-a-list-line-and-continuation-paragraphs-are-part-of-that-list-line), and how statements are counted in [When reading by paragraph, a paragraph delimited by blank lines counts as one statement](#when-reading-by-paragraph-a-paragraph-delimited-by-blank-lines-counts-as-one-statement).

### Rejected options

- kotowari reads the raw lines and fills in the statements. This breaks "kotowari does not read Markdown".
- Change the engine's default reading to line by line. The results of mds users would change.

### Confirmations

- C-schema-block-9 In a "reading: line" schema, a line following a field line with no blank line between, and a line indented after a list line with a blank line between, both become statements and do not go into the list line's value (with "kotowari-mds values --format json" the value of the field line is only that line's value, and the next line is output as a statement element)
- C-schema-block-10 In a "reading: line" schema, lines of only a quote, a thematic break, HTML or an image are each read as a statement
- C-schema-block-11 In a "reading: line" schema, a line of only "---" or "===" after a statement makes no heading, and a line indented by four spaces after a blank line makes no indented code block; all of them become statements
- C-schema-block-12 A vertical-bar line such as "| a | b |" with no delimiter row becomes a statement under both "reading: paragraph" and "reading: line"
- C-schema-block-13 Nested list lines are read the same in either reading mode, and even under "reading: line" an indented list line stays a list line, not a statement
- C-schema-block-14 A block enclosed in a "```" or "~~~" fence, including one opened on a list marker line, is read as the same code block in either reading mode

## When reading by paragraph, the lines following a list line and continuation paragraphs are part of that list line

Status: implemented

### Problem

When reading by paragraph, the CommonMark reading is kept as the default ([The lines under a heading are read in the reading mode the schema chooses (paragraph or line)](#the-lines-under-a-heading-are-read-in-the-reading-mode-the-schema-chooses-paragraph-or-line)). In CommonMark, a line following a list line and a paragraph indented after a blank line belong to that list item. It needs to be decided whether these are counted as statements or are part of the list line.

### Decision

While reading by paragraph, a line that directly follows a list line with no blank line between is read as part of that list line. For a field line, it goes into the value. A paragraph that follows a list line after a blank line and is indented is read as a continuation paragraph, part of the preceding list line. Neither is counted as a statement, and neither is made a finding even in the closed world.

While reading by line, no continuation paragraph is created.

### Confirmations

- C-schema-block-15 In a schema that does not write "reading", a line following a field line with no blank line between goes into its value, and a continuation paragraph indented after a blank line attaches to the preceding list line; neither is counted as a statement, and neither is a finding even in the closed world (with "kotowari-mds check --format json" not a single finding is reported)

## When reading by paragraph, a paragraph delimited by blank lines counts as one statement

Status: implemented

### Problem

How statements are counted when reading by paragraph needs to be decided. In CommonMark, several consecutive lines not separated by blank lines are one paragraph, and quotes, thematic breaks, HTML and images are not paragraphs. The default reading of mds is kept in line with this. No reason survives for the content decided itself.

### Decision

While reading by paragraph, a paragraph delimited by blank lines counts as one statement. Lines of only an image are not counted as statements, and are not made findings even in the closed world. How quotes, thematic breaks and HTML blocks are treated is decided in [When reading by paragraph, block quotes, HTML blocks and thematic breaks inside a node that does not declare "open: true" are findings](#when-reading-by-paragraph-block-quotes-html-blocks-and-thematic-breaks-inside-a-node-that-does-not-declare-open-true-are-findings).

### Confirmations

- C-schema-block-16 When reading by paragraph, several consecutive lines not separated by blank lines count as one statement, and lines of only an image are not counted as statements and are not findings even in the closed world

## Headings and tables are read as CommonMark and GFM specify in either reading mode

Status: implemented

### Problem

mds is a generic engine not tied to kotowari; kotowari depends on mds, and there is no dependency the other way. When reading by line was added, its condition for a heading followed kotowari's earlier reading (whitespace right after the "#", no upper limit on the number of "#"), so "####### x" and "## x" with leading whitespace were read differently depending on the reading mode. Some tables, too, were tables when reading by paragraph but statements when reading by line.

### Decision

In either reading mode, a heading is only a CommonMark ATX heading line: a line with at most three leading spaces, one to six "#", and then whitespace or the end of the line. A line of only "#" is a heading too. "#foo" and a line of seven or more "#" are statements.

In either reading mode, a table is a GFM table: a run with a header row and a delimiter row, which need not start with a vertical bar. The end of a table also follows GFM.

### Rejected options

- Use the same condition as kotowari's earlier reading (whitespace right after the "#", no upper limit on the number of "#") only when reading by line. This would bring a kotowari-specific reading into mds.

### Confirmations

- C-schema-block-17 "  ## x" with two leading spaces and a line of only "#" are read as headings, and the lines "####### y" and "#z" are read as statements, with the same "kotowari-mds check --format json" output under "reading: paragraph" and "reading: line". Under "reading: line", a "## y" line indented by four spaces is a statement
- C-schema-block-18 A GFM table that does not start with a vertical bar, such as "a | b" and "--- | ---", and a GFM table whose header row or delimiter row lacks a vertical bar, are read as tables and extracted even under "reading: line"

## When reading by line, a line opening a fence opens it whatever its indentation depth

Status: implemented

### Problem

When reading by line, no indented code block is made, and list lines do not absorb the lines that follow them. So a fence written indented under a list can become a code block only if the indentation depth does not matter. If the indentation were limited to three spaces as in CommonMark, the opening and closing lines of an indented fence and its content would each become one statement per line.

### Decision

Under "reading: line", a "```" or "~~~" line opening a fence is read as the opening of a fence whatever its indentation depth, and the block is made a code block. The reading under "reading: paragraph" is not changed.

### Rejected options

- As in CommonMark, only a line indented by at most three spaces opens a fence. A fence under a list would break, and the mistake would be hard to notice.

### Confirmations

- C-schema-block-19 In a "reading: line" schema, a block enclosed by "```" lines indented by four spaces after a statement and a blank line is read as a code block, and the fence lines and the lines inside are not output as statement elements

## The child list of a bullet is checked against the declared child rules

Status: implemented

### Problem

Some formats write an indented child list under a bullet. The default of checking is the closed world, and its purpose is to fix the format, so silently letting undeclared child lines through would defeat that purpose. No reason survives other than the reason for the closed world.

### Decision

When a bullet has a child list, it is checked against the child rules "children" the schema declares. Child lines of a bullet or field line that declares no child rules are made findings, following the rule of the closed world ([The default is the closed world, and undeclared headings and lines are findings](closed-world.md#the-default-is-the-closed-world-and-undeclared-headings-and-lines-are-findings)).

Child rules can declare field lines and bullets. A child bullet can in turn have child rules. Child lines are told apart in the same way as list lines ([A list line is read as a field line when its name matches a declared name, and as a bullet when it does not](#a-list-line-is-read-as-a-field-line-when-its-name-matches-a-declared-name-and-as-a-bullet-when-it-does-not)), with no special treatment for any particular name.

A schema that declares extraction on a child bullet is a stop. Where the extracted values of a child field line are placed is as in [Values are placed where the placement path points, nested by the dot-separated names](extraction.md#values-are-placed-where-the-placement-path-points-nested-by-the-dot-separated-names).

### Confirmations

- C-schema-block-20 Child lines under a bullet or field line that declares no child rules (a child list, and a table or code block that follows indented) are undeclared_line findings
- C-schema-block-21 Child rules can declare field lines and bullets, a child bullet can in turn have child rules, child lines are read as children of their parent without entering the parent's count, and a child line with an undeclared name is matched as a child bullet
- C-schema-block-22 A schema that declares extraction on a child bullet is a stop

## The header of a table is matched only when it is declared

Status: implemented

### Problem

We want to be able to declare tables whose header differs from table to table. This aligns the treatment with the language of a code block, which can be omitted.

### Decision

"header" of a table rule can be omitted. Only when "header" is declared are the header and the number of columns of the document's table matched, and a header that does not match is a table_header_mismatch finding. When it is not declared, only whether the table exists and how many times it appears are looked at.

### Confirmations

- C-schema-block-23 With a table rule that declares no "header", a table with any header gets no header finding ("kotowari-mds check" reports no finding), and when there is no table a missing_table finding is reported
- C-schema-block-24 With a table rule that declares "header", a table whose header does not match the declaration gets a table_header_mismatch finding

## The number of columns of a table follows the document's header row, and extra cells are dropped

Status: implemented

### Problem

The GFM definition of tables drops cells beyond the header. Unlike this, mds made a row with extra cells a finding.

Also, if the number of columns is matched against the header the schema declared, then for a table whose header row in the document differs from the declaration, one cause (the header row) sets off a chain of findings, one per data row. The data rows do match the header of their own table. Dropping extra cells means reading rows against the document's header, so comparing again against the number of columns of the declared header would report findings on the dropped cells; the two cannot both hold.

### Decision

The number of columns of a table is matched against the number of columns of the document's header row, not against the header the schema declared.

A data row with more cells than the document's header row has the extra cells dropped and is not made a finding, regardless of the header declaration. A data row with fewer cells than the document's header row is made a finding.

For a table whose header row in the document differs from the declaration, only one finding on the header mismatch is reported, and no finding on the number of columns is reported per data row.

### Rejected options

- Also match data rows against the number of columns of the declared header. Checking again after the header is fixed reveals the same rows.

### Confirmations

- C-schema-block-25 In a table whose header has three cells, a data row with four cells gets no finding and its extracted value holds only the first three cells, and a data row with two cells gets one finding pointing at that row
- C-schema-block-26 For a table whose header row in the document differs from the declaration, only one header mismatch finding is reported, and data rows whose number of columns matches the document's header row get no finding

## A table rule can choose only the first table whose header matches, with "select: first"

Status: implemented

### Problem

Some formats place tables whose header matches and tables whose header does not match side by side in the same place (a table of terms after the title, with other tables allowed too, for example). The extracted values do not include the text of the table's header row, so the side reading the values cannot tell which table is the one whose header matches. Even so, the default of checking (a table whose header does not match is a finding) is not changed.

For a table rule with no header, "the header matches" is undefined for "select". If the range of values of "select" is not decided, whether an unknown value is a stop or is ignored would depend on how the implementation chooses its types. It is given the same treatment as an unknown value of "reading".

### Decision

When a table rule declares "select: first" together with "header", only the first table whose header matches the declaration, within the node where the rule is placed, is used as that rule's table for checking and extraction. Other tables, whether their header matches or not, are treated as undeclared tables.

A table rule that does not write "select" makes a table whose header does not match a shape-violation finding, as before.

A schema that writes "select" without "header", and a schema that writes a value other than "first" in "select", are a stop.

### Confirmations

- C-schema-block-27 With a table rule that declares "header" and "select: first", only the data rows of the first table whose header matches are extracted, other tables get an undeclared-table finding and no header shape-violation finding, and when the node holding the rule appears repeatedly, one table is chosen per occurrence
- C-schema-block-28 For a schema that declares "select: first" without writing "header", and a schema that writes a value other than "first" such as "select: last", the exit code of "kotowari-mds check" is 2

## The language of a code block is matched only when it is declared, and lines are matched with leading whitespace removed

Status: implemented

### Problem

We want a schema to be able to constrain the language of a code block and the shape of the lines inside it. No reason survives.

### Decision

A code block rule can declare a language and a per-line regular expression. The language is matched only when it is declared, and a mismatch is a codeblock_lang_mismatch finding.

The per-line regular expression is matched after leading whitespace is removed, and a line that is empty after removal is not matched. A line that does not match is a codeblock_line_mismatch finding.

### Confirmations

- C-schema-block-29 With a code block rule that declares a language, a code block of a different language gets a codeblock_lang_mismatch finding
- C-schema-block-30 With a code block rule that declares a per-line regular expression, lines are matched with leading whitespace removed, empty lines are not matched, and a line that does not match gets a codeblock_line_mismatch finding

## Table and code block rules can be declared on items the same as on sections

Status: implemented

### Problem

We want to be able to check items such as decision tables, which hold a table under their heading. It is asymmetric that tables and code blocks can be declared on sections but not on items.

### Decision

Table and code block rules can be declared on items too. The content of the rules, the default cardinality and the finding kinds are the same as when they are declared on a section. Which rule kinds can be placed where, as a whole, is decided in [There are nine rule kinds, each with the places it can be placed](schema-language.md#there-are-nine-rule-kinds-each-with-the-places-it-can-be-placed).

### Confirmations

- C-schema-block-31 A table and a code block declared on an item are not undeclared-line findings, and if the header of an item's table does not match the declaration it is a table_header_mismatch finding, the same as for a section

## The regular expression of a bullet is matched against the lead paragraph of the list item joined into one line

Status: proposed

### Problem

The regular expression of a bullet is matched only against the part of the marker line without the marker. For a bullet written wrapped over lines, neither the regular expression nor the named captures see the text from the second line on. The value of a field line also includes the lines that follow with no blank line between, so bullets and field lines are not read alike. kotowari's decision documents cut the ID and the name out of confirmation rows with named captures, so the name of a wrapped row is cut short.

### Decision

The regular expression of a bullet and its named captures ([Named captures are written with group as the value of a key of of, on the title and on bullets](extraction.md#named-captures-are-written-with-group-as-the-value-of-a-key-of-of-on-the-title-and-on-bullets)) are matched against the lead paragraph of the list item. The text matched is the lines of the lead paragraph, each with leading and trailing whitespace removed, joined with one space in place of each line break, without the marker. Continuation paragraphs and child lists are not matched.

### Rejected options

- Previous decision: a bullet is matched only against the part of the marker line without the marker, and lines that follow the marker line with no blank line between are not matched (the second paragraph of the Decision of [A regular expression can be imposed on statements and bullet lines, and an allow list on statements too](#a-regular-expression-can-be-imposed-on-statements-and-bullet-lines-and-an-allow-list-on-statements-too)). It was dropped because the regular expression and the captures do not see the rest of a wrapped bullet's text.
- Keep matching only the marker line, and have the user of mds (kotowari) join wrapped names. The user would be reading list items again.

### Confirmations

- C-schema-block-32 In a schema that does not write "reading", a bullet whose marker line "- a b" is followed with no blank line between by the line "  c" matches the regular expression "^a b c$" with no finding, and does not match the regular expression "^a b$", giving bullet_pattern_mismatch
- C-schema-block-33 A bullet whose lead paragraph is "a" and which has, after a blank line, an indented continuation paragraph "b" and a child list "- c" matches the regular expression "^a$" with no finding

## When reading by paragraph, block quotes, HTML blocks and thematic breaks inside a node that does not declare "open: true" are findings

Status: implemented

### Problem

When reading by paragraph, block quotes, HTML blocks and thematic breaks are not counted as statements and are not findings even in the closed world. Inside the closed world, which exists to fix the format, they pass silently without being checked. The reader of its own that kotowari used for decision documents made them errors in the place of the state line and in the "Open" section.

### Decision

When reading by paragraph, block quotes, HTML blocks and thematic breaks inside a node that does not declare "open: true" ([The preamble, a section, an item and the children of a bullet accept blocks that have no rule when they declare "open: true"](schema-language.md#the-preamble-a-section-an-item-and-the-children-of-a-bullet-accept-blocks-that-have-no-rule-when-they-declare-open-true)) are undeclared_line findings. Inside a node that declares "open: true" they are not findings. Inside an undeclared structure that the open world allows, they are allowed like any other line ([The open world allows only undeclared structures and what is inside them](closed-world.md#the-open-world-allows-only-undeclared-structures-and-what-is-inside-them)).

Image-only paragraphs, link reference definitions and blocks that hold only HTML comments are, inside any node, not counted as statements and not findings. An HTML comment is a note for the people and tools that write the document (kotowari markers are also written in HTML comments), not content of the document.

How lines are read when reading by line (block quote, thematic break, HTML and image lines are statements) is as in [The lines under a heading are read in the reading mode the schema chooses (paragraph or line)](#the-lines-under-a-heading-are-read-in-the-reading-mode-the-schema-chooses-paragraph-or-line), and this decision does not change it.

### Rejected options

- Previous decision: when reading by paragraph, lines with only a block quote, thematic break, HTML or image are not counted as statements and are not findings even in the closed world (the second sentence of the Decision of [When reading by paragraph, a paragraph delimited by blank lines counts as one statement](#when-reading-by-paragraph-a-paragraph-delimited-by-blank-lines-counts-as-one-statement), and the "paragraph" column of the row "Block quote, thematic break, HTML and image lines" in the table of [The lines under a heading are read in the reading mode the schema chooses (paragraph or line)](#the-lines-under-a-heading-are-read-in-the-reading-mode-the-schema-chooses-paragraph-or-line)). It was dropped for block quotes, HTML blocks and thematic breaks, because what is not declared passes silently in the closed world.

### Confirmations

- C-schema-block-34 In a schema that does not write "reading", block quotes, HTML blocks and thematic breaks inside a section that does not write "open" each get an undeclared_line finding at their opening line
- C-schema-block-35 In a schema that does not write "reading", image-only paragraphs and link reference definitions inside a section that does not write "open" get no finding
- C-schema-block-36 Block quotes, HTML blocks and thematic breaks inside a section that declares "open: true" get no finding
- C-schema-block-37 Even when "open: true" is written at the top level of the schema, a block quote inside a declared section gets an undeclared_line finding
- C-schema-block-41 In a schema that does not write "reading", a block that holds only HTML comments inside a section that does not write "open" gets no finding

## An unclosed fence is an unclosed_codeblock finding with every schema

Status: proposed

### Problem

When a fence opened with "```" or "~~~" is not closed before the document ends, CommonMark makes everything up to the end of the document the content of the code block. The headings and lines after it become content and are not checked. mds has no finding kind for this, and the document passes as one that satisfies the schema. kotowari searched for unclosed code blocks in decision documents with code of its own and made them errors.

### Decision

When a code block opened with a fence is not closed before the document ends, an unclosed_codeblock finding is reported. The line of the finding is the opening line of the fence, and the raw line is the text of that line.

This finding is reported with every schema. It is reported in the closed world and in the open world, inside a node that declares "open: true", inside an undeclared structure, and for a fence opened inside a list item. It is reported both when reading by paragraph and when reading by line. When reading by line, a fence opened as in [When reading by line, a line opening a fence opens it whatever its indentation depth](#when-reading-by-line-a-line-opening-a-fence-opens-it-whatever-its-indentation-depth) that reaches the end of the document with no closing line is a finding.

### Confirmations

- C-schema-block-38 In a document that, after a paragraph satisfying the schema, opens with "```" and does not close, "kotowari-mds check --format json" reports an unclosed_codeblock finding with the line number and the raw line of the opening line, and the exit code is 1
- C-schema-block-39 A fence opened indented inside a list item and not closed also gets an unclosed_codeblock finding, with an "open: true" schema and with a "reading: line" schema too
- C-schema-block-40 A document that has only closed fences gets no unclosed_codeblock finding
