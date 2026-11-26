# Glossary

English | [日本語](CONTEXT.ja.md)

Terms used in the specification IR of mds. Only terms whose meaning differs from general usage are listed.

| Term | Meaning | Source |
|---|---|---|
| schema | A YAML file that declares the format of a document (structure and constraints) and the extraction rules | docs/decision/records/2026-09-21-mds-spec.md#A1 |
| document | A Markdown file that mds checks and extracts from | docs/decision/records/2026-09-21-mds-spec.md#A1 |
| frontmatter | A YAML block between "---" lines at the start of a document | docs/decision/records/2026-09-21-mds-spec.md#A1 |
| closed world | A validation style that treats headings and lines not declared in the schema as errors | docs/decision/records/2026-09-21-mds-spec.md#A2 |
| open world | A validation style that allows undeclared structures and the lines inside them | docs/decision/records/2026-09-21-mds-spec.md#A2 |
| rule kind | A kind of node by which the schema describes the structure of a document | docs/decision/records/2026-09-21-mds-spec.md#A39 |
| node | One element of the schema tree. The unit of validation and extraction | docs/decision/records/2026-09-21-mds-spec.md#A39 |
| title | The depth-1 heading of a document | docs/decision/records/2026-09-21-mds-spec.md#A5 |
| preamble | The part after the title and before the first section. In a schema that declares items directly under the document, the part before the first section or item | docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-21-mds-spec.md#A38, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23 |
| section | A depth-2 heading | docs/decision/records/2026-09-21-mds-spec.md#A5 |
| item | A depth-3 heading with an ID and a name. Placed under a section or, when the schema declares it, directly under the document with no section between | docs/decision/records/2026-09-21-mds-spec.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A23 |
| field line | A line of the "name and value" form following a list marker, whose name matches a declaration of the schema | docs/decision/records/2026-09-21-mds-spec.md#A10 |
| bullet | A list line that starts with a list marker and is not a field line | docs/decision/records/2026-09-21-mds-spec.md#A10 |
| statement | A non-blank line under a heading or the preamble that is neither a list nor a table. When the reading mode is paragraph, a paragraph spanning several lines counts as one statement; when it is line, each line counts as one statement | docs/decision/records/2026-09-21-mds-spec.md#A30, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20 |
| continuation paragraph | When the reading mode is paragraph, a paragraph that follows a list line after a blank line and is a child of that line. Not created when the reading mode is line | docs/decision/records/2026-09-21-mds-spec.md#A11, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13 |
| table | A Markdown table. The header cells, the number of columns, and a selection that makes only the first table with a matching header the rule's table can be specified | docs/decision/records/2026-09-21-mds-spec.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A28 |
| code block | A fenced block. The language and per-line rules can be specified | docs/decision/records/2026-09-21-mds-spec.md#A12 |
| extraction | A rule, declared by the schema on each node, that takes out values | docs/decision/records/2026-09-21-mds-spec.md#A4 |
| placement path | A dot-separated name pointing at where an extracted value is placed | docs/decision/records/2026-09-21-mds-spec.md#A4 |
| finding | One result produced by a check | docs/decision/records/2026-09-21-mds-spec.md#A17 |
| stop | Ending with exit code 2 when the check cannot be performed | docs/decision/records/2026-09-21-mds-spec.md#A15 |
| base directory | The first directory containing ".kotowari/" found by searching upward from the current directory. If none is found, the current directory | docs/decision/records/2026-09-21-mds-spec.md#A14, docs/decision/records/2026-09-23-mutants-gaps.md#A9, docs/decision/records/2026-09-23-mutants-gaps.md#A12, docs/decision/records/2026-09-24-kotowari-dir.md#A1 |
| cardinality | The lower and upper bounds on how many times a node may appear | docs/decision/records/2026-09-21-mds-spec.md#A40 |
| conditional rule | A condition clause that switches whether another rule applies according to the value of a field line | docs/decision/records/2026-09-21-mds-spec.md#A28 |
| derived value | A value that an extraction declaration derives from a position or an identifier rather than from the text of the document. Five of them: the line number, the ID of the item heading, the name of the item heading, the raw line, and the last line of the element | docs/decision/records/2026-09-22-ir-engine.md#A49, docs/decision/records/2026-09-21-mds-spec.md#A23, docs/decision/records/2026-09-23-ir-engine-gaps.md#A18, docs/decision/records/2026-09-23-ir-engine-gaps.md#A24 |
| element value | A value that extraction takes from the element itself. Distinguished from derived values | docs/decision/records/2026-09-22-ir-engine.md#A52 |
| node name | The name of a node declared by the schema. Only sections and field lines have one; some nodes, such as the title, have no declared name | docs/decision/records/2026-09-22-ir-engine.md#A27, docs/decision/records/2026-09-21-mds-spec.md#A45 |
| raw line | The text of the line a finding points at, or of the line of an extracted element, exactly as it is. Includes indentation and trailing whitespace, and is not reassembled | docs/decision/records/2026-09-22-ir-engine.md#A49, docs/decision/records/2026-09-22-ir-engine.md#A29, docs/decision/records/2026-09-22-ir-engine.md#A57, docs/decision/records/2026-09-22-ir-engine.md#A62 |
| reading mode | The unit, declared at the top level of the schema, by which the lines under a heading are split into statements and list lines. Two of them: paragraph ("paragraph"; the default, read as CommonMark paragraphs) and line ("line"; read one line at a time) | docs/decision/records/2026-09-23-ir-engine-gaps.md#A12, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A20 |
