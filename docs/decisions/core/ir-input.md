# Character encoding and code block boundaries

English | [日本語](ir-input.ja.md)

This document covers a BOM at the start of a file that is read, and how unclosed code blocks are treated. Not reading the inside of code blocks itself is decided in [The inside of a code block in a decision document is read neither as decision form nor as a quotation](ir-document.md#the-inside-of-a-code-block-in-a-decision-document-is-read-neither-as-decision-form-nor-as-a-quotation).

## A leading UTF-8 BOM is skipped, and is not a stop as a file that is not UTF-8

Status: implemented

### Problem

It had to be decided whether a UTF-8 BOM at the start of a file that is read is read as a character, made an encoding error, or skipped. The reason for deciding to skip it is not recorded.

### Decision

When a file kotowari reads has a UTF-8 BOM at its start, kotowari skips it and reads the file, and does not stop on it as a file that is not UTF-8. This applies to decision documents, the configuration file, test files and the sources of mutation outcomes alike.

### Confirmations

- C-core-input-1 Even if the configuration file, a test file or a source of mutation outcomes has a BOM at its start, check and mutants do not stop, and read it the same as without the BOM
- C-core-input-2 Even if a decision document has a BOM at its start, check does not stop, and reads the title on line 1 as the title

## An unclosed code block is an unclosed_code_block error, and from its start to the end of the document is not read

Status: implemented

### Problem

When a document ends with a code block not closed, the Markdown reader reads from its start to the end of the document as a code block. If nothing were reported, the decisions and confirmation rows after the unclosed code block would silently go uncounted, and check would pass even if a confirmation of an implemented decision has no test. That would silently skip input that does not fit the form ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)). The Markdown reader does not detect unclosed code blocks, so kotowari detects them on its side.

### Decision

When a code block of a decision document ends without being closed, an unclosed_code_block error is reported with the opening line as "line" and the characters of the opening line as they are (including indentation) as the detail. Because it is an error, the exit code of check is 1.

When a code block directly under the document ends without being closed and the document ends, everything from the opening line to the end of the document is excluded from the targets of checks. A heading inside it is not made a decision, and a line in list form is not made a confirmation row. The decisions and confirmations before the opening line are read as they are.

### Confirmations

- C-core-input-3 When a code block of a decision document ends without being closed, an unclosed_code_block error with the opening line as "line" and the characters of the opening line as the detail is reported, and the exit code of check is 1
- C-core-input-4 Headings and lines in confirmation form from the opening line of an unclosed code block directly under the document to the end of the document are not counted as decisions or confirmation rows, and the decisions and confirmation rows before the opening line are counted
