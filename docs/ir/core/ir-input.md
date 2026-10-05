# Character encoding and code block boundaries

English | [日本語](ir-input.ja.md)

Covers a BOM at the start of a file that is read, and the handling of an unclosed code block.

## Requirements

### REQ-core-111: A BOM at the start

- kind: event_driven
- source: docs/decision/records/records.md#A107
- verification: unit

When a file that is read has a UTF-8 BOM at its start, kotowari skips it and does not `stop` treating the file as not UTF-8.

### REQ-core-112: An unclosed code block

- kind: event_driven
- source: docs/decision/records/records.md#A108
- verification: unit

When a document ends without a `code block` being closed, kotowari raises an unclosed_code_block `error` with the start line as "line" and the characters of the start line as the detail, and removes the part from the start to the end of the document from what it checks.
