# Extraction and the plain syntax tree

English | [日本語](extraction.ja.md)

This covers the behavior of assembling values from the extraction a schema declares, and the behavior of outputting the plain syntax tree separately from validation. The entry point that takes extracted values out of the library is decided in [the "Validation and value extraction in the library"](library-extraction.md) topic, in [The CLI's values and ast --schema output values as before even for a document with violations](library-extraction.md#the-clis-values-and-ast---schema-output-values-as-before-even-for-a-document-with-violations), that a value becomes an array under a range of cardinality in [A node that declares a cardinality range is extracted as an array even for a single value](schema-language.md#a-node-that-declares-a-cardinality-range-is-extracted-as-an-array-even-for-a-single-value), which lines become field lines, statements, and bullets in [the "Rules for the lines under a heading"](block-rules.md) topic, and collisions between placement paths and the reasons for a stop in [A schema whose placement paths collide stops without reading documents](cli.md#a-schema-whose-placement-paths-collide-stops-without-reading-documents) and [There are five reasons for a stop](cli.md#there-are-five-reasons-for-a-stop).

## Extracted values are output as strings, with no type conversion

Status: implemented

### Problem

If values taken from a document were converted to dates or numbers, mds would decide on the user's behalf how a value is interpreted. As strings, the values can be used in common by every user.

### Decision

The extracted values taken from a document are output as strings. Even a value that looks like a date or a number is not converted.

Values derived from positions rather than from the text of the document (the line number and the last line of the element among the derived values) are outside this rule. How they are decided is decided in [Derived values are five words](#derived-values-are-five-words).

### Rejected options

- Convert extracted values to dates or numbers. This fixes the user's interpretation, and strings are the common denominator.

### Confirmations

- C-schema-extract-1 For any document, including one with values that look like numbers or dates, every leaf of the extracted values is a string

## Values are placed where the placement path points, nested by the dot-separated names

Status: implemented

### Problem

Where an extracted value is placed is decided by the placement path written in the schema. The reason for placing values nested by dot separation is not recorded.

### Decision

mds places an extracted value nested along the dot-separated names of the placement path. `values` and `ast --schema` output values in this nested shape (the shape of `ast --schema` is decided in [ast --schema adds the schema's name as type to the same values as values --format json](cli.md#ast---schema-adds-the-schemas-name-as-type-to-the-same-values-as-values---format-json)).

The stop for a schema in which a separated name of a placement path is empty is decided in [A placement path with an empty dot-separated name stops](cli.md#a-placement-path-with-an-empty-dot-separated-name-stops), and the stop for a schema whose placement paths overlap is decided in [A schema whose placement paths collide stops without reading documents](cli.md#a-schema-whose-placement-paths-collide-stops-without-reading-documents).

### Confirmations

- C-schema-extract-2 When a placement path containing dots is declared, the values of `kotowari-mds values --format json` are output nested by the dot-separated names

## When the target of an extraction is absent from the document, the key of its placement path is not output

Status: implemented

### Problem

What the output should be when a node that declares an extraction is absent from the document had to be decided. The reason is not recorded.

### Decision

When the node targeted by an extraction is absent from the document, mds does not output the key of its placement path.

### Confirmations

- C-schema-extract-4 When a node that declares an extraction (any of a field line, a bullet, or a table, whether repeated or not) is absent from the document, the key of its placement path is not in the output

## An extraction is declared as one nested mapping holding path, value, and of

Status: implemented

### Problem

The declaration of an extraction used to list, in one list, the place to put a value (an absolute placement path) and the keys inside an element (relative placement paths). Which one an entry was could be told only from an unwritten clue: the absence of a derived value declaration. The rule on the number of places also had no home in the shape of a list; with zero places the output went silently empty, and with two the same content was duplicated. From the user's remark that the mistakes come easily because the way of writing the declaration is not intuitive, it was decided to make the zero-place and two-place forms impossible to write, instead of rejecting them with a stop.

If the value of the element were given an implicit unnamed place, it would compete for names with the derived values at the same level. If the value of the element is also given a declared key, the competition itself disappears. A declaration that wants only derived values (no cells) is a legitimate intent, and since a place always exists, the result is also as predicted.

The short form is used by 13 of the 22 declarations, and it is not ambiguous in itself.

### Decision

The extraction of a node is declared as one nested mapping. `path` holds exactly one placement path, `value` the key under which the value of the element is placed, and `of` the mapping from keys of derived values to words. `of` may be written alone without `value`, and then the element has only the keys of the derived values. A declaration without `path` and a declaration written as a list of forms are a stop for a schema that does not match the form.

The short form `extract: <name>` is read as an abbreviation of `{ path: <name> }`.

How a named capture is written is decided in [Named captures are written with group as the value of a key of of, on the title and on bullets](#named-captures-are-written-with-group-as-the-value-of-a-key-of-of-on-the-title-and-on-bullets).

### Rejected options

- Keep the extraction as a list of forms, and enforce exactly one place with `schema_invalid`. The difficulty of reading places and keys inside elements side by side in the same list would remain.

### Confirmations

- C-schema-extract-5 For a schema that declares an extraction without `path` or an extraction written as a list of forms, `values` stops, and the exit code is 2
- C-schema-extract-6 The element object of an extraction that has only `of` written, without `value`, holds only the keys of the derived values and the placement paths that inner nodes declared
- C-schema-extract-7 The short form `extract: <name>` is read the same as a declaration with only `path` written

## An object per element is assembled only when value or a derived value is declared

Status: implemented

### Problem

There is no reason for the output of an extraction that declares no derived value to change. On the other hand, every node that has a collection of elements should allow the value of the element and the derived values to be taken out in the same way of writing. Unless one table decides, per node, what becomes one element and which line the raw line points at, the shape when they are declared is not settled.

### Decision

For any node, if neither `value` nor a derived value is declared, the shorthand shape in the table below is output as is, without splitting into elements. If either is declared, one object is assembled per unit of element, with the value of the element under the `value` key and the derived values under the `of` keys. Even for a node that is not split, declaring them makes one object under the placement path.

| Node | Unit of element (when `value` or a derived value is declared) | Value | Line the raw line and the line number point at |
|---|---|---|---|
| Title | Not split | The heading text | The heading line of the title |
| Field line | Not split | The value string after the name. When a delimiter is declared, the array of split strings | The line of that field line |
| Statement | Lines, only when a derived value is declared. Not split if none is declared | When not split, the body string. When split into lines, the text of the line with leading and trailing whitespace removed | That line |
| Section | Not split | The body string joining only the lines of statements and bullets. Field lines, tables, code blocks, and items are not included | The heading line of the section |
| Item | Item | The string joining the heading and the body with a newline. The heading is reduced to the ID and the name. The body includes field lines, statements, and bullets, and does not include tables and code blocks. A declared field line contributes its line and its continuation paragraph but not its child bullets, and an undeclared line contributes its child lines as well | The heading line of the item |
| Bullet | Line | The original lines starting at the marker (including the lines of continuation paragraphs and child bullets). In the shorthand shape, an array of those strings | The marker line of that line |
| Table | Data row | The values of the data row. The keys are decided in [The keys of table rows come from the header declaration or the column positions, not from the text of the document's header row](#the-keys-of-table-rows-come-from-the-header-declaration-or-the-column-positions-not-from-the-text-of-the-documents-header-row). In the shorthand shape, an array of rows | The line of that data row |
| Code block | Block | The string of the whole block | The opening line of the fence |

How the lines of a value are taken and joined is decided in [The lines of a value are used as the original lines of the document](#the-lines-of-a-value-are-used-as-the-original-lines-of-the-document). The handling of the cells of a table row beyond the document's header row is decided in [The number of columns of a table follows the document's header row, and extra cells are dropped](block-rules.md#the-number-of-columns-of-a-table-follows-the-documents-header-row-and-extra-cells-are-dropped).

### Confirmations

- C-schema-extract-10 When only the shorthand `extract: text` and `extract: rows` are declared, the value of a statement becomes one string and the value of a table becomes a list of rows, and no object per element is made
- C-schema-extract-11 When `value` or a derived value is declared, a table becomes one object per data row, a bullet one object per line, and a code block one object per block, and a field line, which is not split, becomes one object under the placement path. The value of the element goes under the `value` key, and the derived values under the `of` keys
- C-schema-extract-12 The value of a bullet is the original line, keeping the original marker and the whitespace after it, and includes the lines of continuation paragraphs and child bullets with their indentation
- C-schema-extract-13 The value of a section is the body joining only the lines of statements and bullets, and does not include field lines, tables, code blocks, or items
- C-schema-extract-14 The shorthand value of an item is the string joining the ID and name of the heading and the body with a newline, and the body does not include tables or code blocks
- C-schema-extract-15 In the body of an item, a declared field line contributes only its line and its continuation paragraph and not the lines of its child bullets, and an undeclared list line contributes the lines of its child bullets as well
- C-schema-extract-16 The value of an element of a code block is the string of the whole content of the block

## If there is an extraction inside an item, the item becomes an object keyed by the inner placement paths

Status: implemented

### Problem

The side that checks across documents should be able to read items as structured values. If the person who writes the schema decides the keys, the code that uses the values does not change when the format changes, as long as the placement paths are kept. If the words of the document (the names of headings and field lines) were used as keys, merely rewording a heading would break the output contract. The user adopted this shape on the condition that it can be used as a generic engine not fixed to a particular consumer.

### Decision

While a node inside an item declares an extraction, mds assembles one object per item and resolves the inner placement paths as relative paths within that object. The keys are decided by the inner placement paths, and the names of the document's headings and field lines are not used as output keys.

If there is no extraction inside and the item's extraction declares neither `value` nor a derived value, no object is assembled, and the shorthand shape (the string joining the heading and the body) is used.

### Rejected options

- Use the names of the declared fields as keys as they are. The words of the Markdown would become the output contract, and changing the format would break the consumers.

### Confirmations

- C-schema-extract-17 When a node inside an item declares an extraction, even if the item's extraction is only `path`, an object keyed by the inner placement paths is made per item, and the keys are the same even for documents whose heading and field line names differ
- C-schema-extract-18 If there is no extraction inside an item and the item's extraction declares neither `value` nor a derived value, the item's value stays the string joining the heading and the body

## A schema that declares extraction only inside an item, without the item itself declaring one, stops

Status: implemented

### Problem

The values inside an item are placed in the item's object, as in [If there is an extraction inside an item, the item becomes an object keyed by the inner placement paths](#if-there-is-an-extraction-inside-an-item-the-item-becomes-an-object-keyed-by-the-inner-placement-paths). If the item itself has no extraction, there is no place to put the inner values. The reason is not recorded.

### Decision

When a field line, statement, bullet, table, or code block inside an item declares an extraction and that item itself declares no extraction, mds stops for a schema that does not match the form (`schema_invalid`). A field line that is a child of a bullet also counts as inside the item.

### Confirmations

- C-schema-extract-19 A schema in which the item itself declares no extraction and only a field line, table, or code block inside the item declares an extraction stops with `schema_invalid`. If the item declares an extraction, the same inner declarations do not stop
- C-schema-extract-20 A schema that declares an extraction only on a field line that is a child of a bullet inside an item (however deep the nesting) also stops with `schema_invalid`

## Derived values are five words

Status: implemented

### Problem

Position information is not a value taken from the document but something the engine derives. As a string, it would only leave consumers the chore of always parsing it again. Since only the users who need it declare it, the output of users who do not need it does not change.

If the raw line were always held under an implicit key, the implementation would decide its name without the specification deciding it, and it would compete for names with the declared derived values (declaring the line number silently overwrote the raw line). As one of the words, it can be taken out with a name only when needed. If the range on which the raw line can be declared differed from that of the line number, the difference would have to be remembered, so they are the same. The reason for allowing the ID and the name of the item heading to be declared only on items is not recorded.

### Decision

The derived values are five: the line number `line`, the ID of the item heading `id`, the name of the item heading `name`, the raw line `raw`, and the last line of the element `end`. A schema that declares any other word is a stop.

The line number and the last line of the element are output as JSON numbers.

The nodes on which each word can be declared are decided in [The derived value name can be declared on sections too, and end on bullets too](#the-derived-value-name-can-be-declared-on-sections-too-and-end-on-bullets-too).

The line that the line number and the raw line point at follows the table in [An object per element is assembled only when value or a derived value is declared](#an-object-per-element-is-assembled-only-when-value-or-a-derived-value-is-declared). For a table row, the header row and the delimiter row are not counted, and the data row is pointed at. The raw line outputs the text of that line as it is, including indentation and trailing whitespace, and does not reassemble it.

### Rejected options

- Wrap every value in a pair of value and line number with a flag. Consumers would have to handle two shapes.

### Confirmations

- C-schema-extract-21 The values of `line` and `end` are output as JSON numbers
- C-schema-extract-22 For a schema that declares as a derived value a word that is none of the five, `values` stops, and the exit code is 2
- C-schema-extract-25 `line` and `raw` point at the heading line for a title, at that data row for a table, at that line for a field line, and at the opening line of the fence for a code block
- C-schema-extract-26 `raw` outputs the text of the heading or line as written, including indentation, trailing whitespace, and the symbols of inline markup

## The last line of an element is the line before the next heading of the same depth or shallower

Status: implemented

### Problem

The result of extraction did not include where an item ends. To cut out the range of an item's body, its last line is needed.

### Decision

The `end` of an item or a section is the line number of the line just before the next heading, after its own heading, that is of the same depth or shallower. If there is no such heading, it is the line number of the last line of the document. Blank lines at the end of the range are also included in the last line.

### Confirmations

- C-schema-extract-27 The `end` of an item or a section is the line just before the next heading of the same depth or shallower (that blank line, if there is a blank line between), and if there is no heading after it, the last line of the document

## Declaring a derived value on a statement makes an element per line, and the element's value has leading and trailing whitespace removed

Status: implemented

### Problem

kotowari needed the line number and the original text of each line, to output the line and the content of a finding per line of a statement. The extraction until then joined the following lines into one string and returned only the starting line, so these could not be restored.

If the value of a statement line were the original line itself, the value of the element and the raw line would point at the same thing, and which one to use would be a question every time. By separating them, the user can choose by purpose: the value of the element for matching words, and the raw line for the content of a finding. Since the value is one line, removing the indentation does not break the relation between lines.

### Decision

When a derived value is declared on a statement, one element is made per line of the statement. The lines of paragraphs that continue after a blank line also go into the same list. A continuation paragraph of a list line does not go into the statement's elements.

The value of a line element is the text of the line with leading and trailing whitespace removed. The original line is taken with the raw line.

### Rejected options

- Leave in kotowari the checks that need the raw line per line, and have kotowari read Markdown. The exceptions to the policy of not reading Markdown would grow, and the policy would become a formality.
- Accept that the lines and contents of findings change. This goes against the policy of not changing the behavior users see.

### Confirmations

- C-schema-extract-28 When a derived value is declared on a statement, as many elements as there are lines are output, the lines of paragraphs that continue after a blank line are in the list, and a continuation paragraph of a list line is not
- C-schema-extract-29 The value of a line element of a statement is the text with leading and trailing whitespace removed, and the raw line of the same line includes indentation and trailing whitespace

## The keys of table rows come from the header declaration or the column positions, not from the text of the document's header row

Status: implemented

### Problem

The cells of a Markdown table's header row may be empty, and the same name may repeat. Making keys from them made values silently disappear (in a table with an empty header, the values of the first column were lost, and when there were two columns with the same name, one of them disappeared). Even when `header` was declared, the matching finding was raised, but extraction continued with the text on the document's side. If the source of keys is limited to the schema and positions, this path becomes not something to detect but something that cannot happen. This option came from the user.

If only position has meaning, an array is the natural form, and the very idea of key collisions disappears. Since the keys come from the declaration side, the basis for the number of columns must also be on the declaration side, or where the keys of extra columns come from would go back to the document side.

### Decision

The keys of a table row are the declared names when the schema declares `header`, and the column positions when it does not. When taken by column position, the row is an array. In either case, the text of the document's header row is not used as keys. This rule applies the same way in the shorthand shape and in the object per element.

When the number of names in `header` differs from the number of columns in the document, the number of declared names is authoritative. If the document has too few columns, those keys are omitted, and if it has too many, the extra ones are discarded.

### Rejected options

- Require a `header` declaration on tables that use derived values. Tables whose column names and count differ per table (decision tables) cannot declare `header`, and could not use derived values at all.
- Always use column positions as keys. The glossary tables read their columns by name.
- Take keys from the document's header row, and detect empty cells, repeated column names, and collisions with derived values, raising a finding or a stop. This stops at detection, and the ease of making mistakes itself remains.
- Declare a separate mapping between positions and names in the schema. The `header` declaration is that mapping as it is, so a separate mechanism would be a duplicate.

### Confirmations

- C-schema-extract-30 The rows of a table that declares `header` become objects keyed by the declared names, and the rows of a table that does not declare it become arrays by column position, and no column value is lost even when a header cell is empty or there are two columns with the same name
- C-schema-extract-31 If the document has fewer columns than the names in `header`, the keys of the missing columns are omitted, and if it has more, the values of the extra columns are discarded

## A table declared per element that repeats gets a level per table, and everything else is joined into one array

Status: implemented

### Problem

A placement path is a place to put values, and if a table repeats, it is natural for there to be several places. This also matches the existing convention of making an array of arrays when a range of cardinality is also declared. The shape of the element itself does not change with whether it repeats. The reason for joining the other tables and bullets into one array is not recorded.

### Decision

When a table that declares `value` or a derived value also declares a range of cardinality, a level per table is made directly under the placement path, and the row objects are listed inside it. Other tables and bullets, even when several appear at the same place, are joined into one array in the order they appear in the document.

### Confirmations

- C-schema-extract-32 When there are two tables that declare `value` or a derived value and a range of cardinality, directly under the placement path there is a level per table, and the line number of each row points at that table's data row
- C-schema-extract-33 Tables in the shorthand shape join the rows of the tables at the same place into one array in order of appearance, even when a range of cardinality is declared

## A field line that declares a delimiter becomes an array regardless of cardinality

Status: implemented

### Problem

The shape in which the value of a field line that declares a delimiter is output had to be decided. The reason is not recorded.

### Decision

A field line that declares a delimiter is extracted into an array of split strings, regardless of its cardinality declaration. When a range of cardinality is also declared, it becomes an array of arrays. When `value` or a derived value is also declared, this array is placed under the `value` key.

Only the value that does not include continuation paragraphs (including lines that follow without a blank line between) is split. Continuation paragraphs are not split, and are attached to the last element of the split.

### Confirmations

- C-schema-extract-34 The value of a field line that declares a delimiter becomes an array, and when a range of cardinality is also declared, an array of arrays
- C-schema-extract-35 Lines that follow without a blank line between are included in what is split, and continuation paragraphs are not split and are attached to the last element of the split
- C-schema-extract-36 For a field line that declares a delimiter and `value`, the split array goes under the `value` key

## The lines of a value are used as the original lines of the document

Status: implemented

### Problem

The indentation and blank lines of values spanning several lines were broken. Only the first line of a paragraph lost its indentation, it remained from the second line on, and the blank lines between paragraphs disappeared. Bullets, field lines, statements, sections, and items all broke the same way. This was not a rule someone decided, but an error in where paragraphs were cut out.

Reassembling the elements of a bullet loses the whitespace and markup the writer put there, and the original lines can no longer be recovered from the extracted value. This is made to hold for the whole value, not only the first line of a bullet, leaving no form whose handling changes from place to place. The name of a field line and the indentation before a marker are not part of the value. If the value is the original lines of the range with the excluded lines taken out, no rule per kind of part for joining is needed. Until then it differed from place to place: blank lines only between statements, newlines elsewhere, and blank lines between continuation paragraphs. Between the lead paragraph after a marker-only line and a continuation paragraph, there is always a blank line in the original document, and this rule keeps it, so no rule per form is needed.

### Decision

The lines that go into a value are used as the original lines of the document. Indentation and the trailing whitespace of lines in the middle of the value are kept, and the trailing whitespace and blank lines at the end of the value are removed.

Where a value starts is decided per node. A bullet starts at the marker of its line (including the whitespace after the marker), a field line after the colon and whitespace following the name, and a statement at the first non-whitespace character of the paragraph. From the second line on, lines are taken from the start of the line.

A value made of several parts (the lead paragraph, continuation paragraphs, and child bullets of a bullet; the lead paragraph and continuation paragraphs of a field line; a sequence of statements; the body of a section; the body of an item) joins the included parts with one blank line if there was a blank line between them in the original document, and with one newline if there was not. The lines of excluded parts are taken out, and the gap they leave and blank lines that were consecutive in the original document are merged into one blank line. A line of only whitespace is treated as a blank line. Between the lead paragraph after a marker-only line and a continuation paragraph, the original blank line also remains, following this rule.

The value of an element of a statement split into lines is outside this rule, and follows [Declaring a derived value on a statement makes an element per line, and the element's value has leading and trailing whitespace removed](#declaring-a-derived-value-on-a-statement-makes-an-element-per-line-and-the-elements-value-has-leading-and-trailing-whitespace-removed).

### Rejected options

- Remove the indentation of list contents from every line. This conflicts with the rule that the original lines can be recovered, and would need an exception that keeps indentation only on the lines of child bullets.
- Cut out each part and join them with one newline (the way before the fix). Blank lines remain inside parts and disappear between parts.
- In every form, join the lead paragraph and continuation paragraphs with one newline. The original paragraph breaks disappear from the value.

### Confirmations

- C-schema-extract-37 A value spanning several lines keeps, as in the original, the indentation from the second line on and the trailing whitespace of lines in the middle
- C-schema-extract-38 The value of a field line starts after the whitespace following the name, and the value of a bullet starts at the marker
- C-schema-extract-39 Included parts are joined with one blank line if there was a blank line between them in the original document, and with one newline if not, and the blank lines left where excluded parts were taken out are merged into one. Even when the reading mode is line (`reading: line`), no blank line absent from the original document is added between the lines of statements
- C-schema-extract-40 The value of a bullet whose lead paragraph comes after a marker-only line does not lack the lead paragraph, and the original blank line remains between the lead paragraph and the continuation paragraph

## Heading names and table cell values are read as the text with the inline markup symbols removed

Status: implemented

### Problem

What is used as the name of a heading or the value of a cell is the text, not the symbols. The user ratified the current behavior.

### Decision

mds reads heading names and the values of table cells as the text with the inline markup symbols removed. Emphasis, strong emphasis, links, and reference links join the text inside them, and inline code keeps its contents. Matching against the heading names declared in the schema is also done with this text with the symbols removed.

### Confirmations

- C-schema-extract-41 The names of item headings and the values of table cells that contain strong emphasis, links, reference links, inline code, and emphasis become the text with the symbols removed and the text inside joined, and inline code keeps its contents
- C-schema-extract-42 When a document with the heading "## **Req** x" is given to `kotowari-mds check` with a schema that declares the section name "Req", the heading is read as "Req x" and becomes a finding of an undeclared heading

## The result of extraction is the same whether validated in the closed world or the open world

Status: implemented

### Problem

Whether changing the validation style (closed world or open world) also changes the result of extraction had to be decided. The user ratified it after checking the actual implementation. The reason is not recorded.

### Decision

Extraction does not read the closed-world setting. For the same document and the same schema, the result of extraction is the same whether the schema's "open" or "--open" is given. "open" and "--open" are decided in [The open world allows only undeclared structures and what is inside them](closed-world.md#the-open-world-allows-only-undeclared-structures-and-what-is-inside-them).

### Confirmations

- C-schema-extract-43 For any document and schema, the result of extraction is equal whether the schema's "open" is true or false

## ast outputs the plain syntax tree of a document as JSON, separately from validation

Status: implemented

### Problem

Whether to have a means of viewing a Markdown document as a tree as it is, separately from schemas, had to be decided. The user ratified that the CLI accepts `ast`. The reason is not recorded.

### Decision

`kotowari-mds ast` outputs the plain syntax tree of a document as JSON, separately from validation. The list of commands the CLI accepts is decided in [The mds CLI is named kotowari-mds and accepts check, ast, values, and --version](cli.md#the-mds-cli-is-named-kotowari-mds-and-accepts-check-ast-values-and---version).

### Confirmations

- C-schema-extract-44 Passing a document to `kotowari-mds ast` outputs the JSON of a syntax tree whose nodes are headings, paragraphs, and lists

## The derived value name can be declared on sections too, and end on bullets too

Status: implemented

### Problem

The extraction of a section cannot take out the text of its heading. For sections whose heading text is free (sections selected by "pattern" in [Section and item rules select headings by a list of names or by a regular expression](schema-language.md#section-and-item-rules-select-headings-by-a-list-of-names-or-by-a-regular-expression)), it cannot be told which heading's section a value belongs to. The extraction of bullets cannot take out the last line of a row, so the range of a row's body cannot be cut out. kotowari needs the last line of a confirmation row to return the row's body.

### Decision

The nodes on which the words of derived values can be declared are as follows. A schema that declares a word on a node that does not accept it is a stop.

| Word | Nodes on which it can be declared |
|---|---|
| `line`, `raw` | Any node |
| `id` | Item |
| `name` | Item, section |
| `end` | Item, section, bullet |

The `name` of a section is the text of the section's heading (read as in [Heading names and table cell values are read as the text with the inline markup symbols removed](#heading-names-and-table-cell-values-are-read-as-the-text-with-the-inline-markup-symbols-removed)). The `end` of a bullet is the line number of the last line of its list item (including continuation paragraphs and child lists).

### Rejected options

- Previous decision: `id` and `name` can be declared only on items, and `end` only on items and sections (the third paragraph of the Decision of [Derived values are five words](#derived-values-are-five-words)). It was dropped because the text of a section's heading and the last line of a bullet's row cannot be taken out.

### Confirmations

- C-schema-extract-45 When `name` is declared in `of` of a section's extraction, the `name` key of each section's element holds the text of that section's heading
- C-schema-extract-46 When `end` is declared in `of` of a bullet's extraction, the `end` of each row's element is the line number (a JSON number) of the last line of its list item, including continuation paragraphs and child lists
- C-schema-extract-47 For a schema that declares `id` on a node other than an item, `name` on a node other than items and sections (for example a bullet), or `end` on a node other than items, sections and bullets (for example a table), `values` stops with `schema_invalid`

## Named captures are written with group as the value of a key of of, on the title and on bullets

Status: implemented

### Problem

The named capture `group` can be written only on the extraction of the title, next to `path`. The value is then only the captured text, and it cannot be taken out side by side with other derived values such as the line number. Nor can a bullet's row be cut into parts with a regular expression and taken out. The confirmation rows of kotowari's decision documents ("- C-x-1 （読んで確認）<sentence>") need to be cut into the ID, whether "(by reading)" is present, and the name, and this could not be declared.

### Decision

When `{ group: <name> }` is written as the value of a key of `of`, that key holds the text captured by that named capture of the node's regular expression. It can be declared on the title and on bullets. For the title, the captures of the title's regular expression are used; for bullets, those of the bullets' regular expression ([The regular expression of a bullet is matched against the lead paragraph of the list item joined into one line](block-rules.md#the-regular-expression-of-a-bullet-is-matched-against-the-lead-paragraph-of-the-list-item-joined-into-one-line)).

The key of a named capture that did not take part in the match is not output in that element. The value of the title's element (the `value` key) stays the text of the heading even when captures are declared.

The form that writes `group` next to `path` is not accepted, and a schema that writes it is a stop. A schema that writes `group` on a node other than the title and bullets, and a schema in which the node with `group` has no regular expression or its regular expression does not contain that named capture, are also a stop.

### Rejected options

- Previous decision: the named capture `group` is written next to `path`, not under `of`, and can be declared only on the title (the third paragraph of the Decision of [An extraction is declared as one nested mapping holding path, value, and of](#an-extraction-is-declared-as-one-nested-mapping-holding-path-value-and-of)). It was dropped because captures could not be used on bullets and could not be taken out side by side with other derived values.

### Confirmations

- C-schema-extract-48 When the bullet regular expression "^(?P<code>[A-Z]-[0-9]+) (?P<text>.*)$" and `of: { code: { group: code }, text: { group: text } }` are declared, the `code` and `text` of each row's element hold the text each capture captured
- C-schema-extract-49 When a named capture that may not take part in the match (for example "(?P<flag>!)?") is declared in `of`, the element of a row in which that capture did not take part has no such key
- C-schema-extract-50 For a bullet that has a line following its marker line with no blank line between, the captured text is taken from the lead paragraph joined with one space
- C-schema-extract-51 When `{ group: <name> }` is declared in `of` of the title, that key holds the captured text of the title's regular expression, and the `value` key holds the text of the heading
- C-schema-extract-52 For a schema that declares any of an extraction that writes `group` next to `path`, an extraction that writes `group` in `of` of a node other than the title and bullets, or an extraction that writes `group` on a node with no regular expression or with a regular expression lacking the named capture, `values` stops with `schema_invalid`
