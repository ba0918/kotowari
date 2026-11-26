# Glossary

English | [日本語](CONTEXT.ja.md)

Terms used in the decision documents of mds. Only terms whose meaning differs from general usage are listed.

| Term | Meaning |
|---|---|
| schema | A YAML file that declares the format of a document (structure and constraints) and the extraction rules |
| document | A Markdown file that mds checks and extracts from |
| frontmatter | A YAML block between "---" lines at the start of a document |
| closed world | A validation style that treats headings and lines not declared in the schema as errors |
| open world | A validation style that allows undeclared structures and the lines inside them |
| rule kind | A kind of node by which the schema describes the structure of a document |
| node | One element of the schema tree. The unit of validation and extraction |
| title | The depth-1 heading of a document |
| preamble | The part after the title and before the first section. In a schema that declares items directly under the document, the part before the first section or item |
| section | A depth-2 heading |
| item | A depth-3 heading with an ID and a name. Placed under a section or, when the schema declares it, directly under the document with no section between |
| field line | A line of the "name and value" form following a list marker, whose name matches a declaration of the schema |
| bullet | A list line that starts with a list marker and is not a field line |
| statement | A non-blank line under a heading or the preamble that is neither a list nor a table. When the reading mode is paragraph, a paragraph spanning several lines counts as one statement; when it is line, each line counts as one statement |
| continuation paragraph | When the reading mode is paragraph, a paragraph that follows a list line after a blank line and is a child of that line. Not created when the reading mode is line |
| table | A Markdown table. The header cells, the number of columns, and a selection that makes only the first table with a matching header the rule's table can be specified |
| code block | A fenced block. The language and per-line rules can be specified |
| extraction | A rule, declared by the schema on each node, that takes out values |
| placement path | A dot-separated name pointing at where an extracted value is placed |
| finding | One result produced by a check |
| stop | Ending with exit code 2 when the check cannot be performed |
| base directory | The first directory containing ".kotowari/" found by searching upward from the current directory. If none is found, the current directory |
| cardinality | The lower and upper bounds on how many times a node may appear |
| conditional rule | A condition clause that switches whether another rule applies according to the value of a field line |
| derived value | A value that an extraction declaration derives from a position or an identifier rather than from the text of the document. Five of them: the line number, the ID of the item heading, the name of the item heading, the raw line, and the last line of the element |
| element value | A value that extraction takes from the element itself. Distinguished from derived values |
| node name | The name of a node declared by the schema. Only sections and field lines have one; some nodes, such as the title, have no declared name |
| raw line | The text of the line a finding points at, or of the line of an extracted element, exactly as it is. Includes indentation and trailing whitespace, and is not reassembled |
| reading mode | The unit, declared at the top level of the schema, by which the lines under a heading are split into statements and list lines. Two of them: paragraph ("paragraph"; the default, read as CommonMark paragraphs) and line ("line"; read one line at a time) |
