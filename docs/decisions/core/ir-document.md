# How documents are read, and checks on a whole document

English | [日本語](ir-document.ja.md)

This document covers which files are read from the decision document location, how far the title of one document and the paragraph after the title are examined, how lines are counted, removing the notices for the limits on line count and requirement count, and how the inside of code blocks is treated. The stop when the location itself is missing is decided in the configuration topic, in [If the location of decision documents does not exist, is not a directory, or cannot be read, kotowari stops with unreadable file](config.md#if-the-location-of-decision-documents-does-not-exist-is-not-a-directory-or-cannot-be-read-kotowari-stops-with-unreadable-file), and a leading BOM and unclosed code blocks are decided in [Character encoding and code block boundaries](ir-input.md).

## The decision document location is traversed to any depth, and only files with a lowercase .md are read

Status: implemented

### Problem

How the location is divided depends on the requirements. We want to divide a domain with many concepts deeply and a simple one shallowly. If the traversal depth is limited, the division is decided by the tool's convenience.

On the other hand, the inputs that are not read are only those enumerated, and nothing is silently skipped ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)). Symbolic links to directories can create cycles, so they are not followed. Symbolic links to files do not create cycles, and if they were not read, documents the user shared would silently disappear.

Trying to read something that is neither a directory nor a regular file (a socket, a named pipe, a device) can make check hang without the read returning. Of the reasons for deciding not to read them, this is the only one left in the records.

### Decision

The directories under the decision document location are traversed to any depth. Only files whose extension is a lowercase ".md" are read; ".MD" files are not read. Nothing is reported for an empty directory.

Hidden directories and symbolic links to directories are not traversed at any depth. Symbolic links to files are read. On meeting a symbolic link without a target, or an entry whose type cannot be obtained, kotowari stops on the grounds of unreadable file. Something that is neither a directory nor a regular file (a socket, a named pipe, a device) is not read even if its name ends with ".md", and neither a finding nor a stop is reported for it.

How the glossary "CONTEXT.md" placed in a group folder is treated is decided in [Keep the glossary as a document for people, and drop the term check](decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check).

### Confirmations

- C-core-document-1 Decision documents are read by traversing the subdirectories under the location to any depth, and nothing is reported for an empty directory
- C-core-document-2 Files whose extension is an uppercase ".MD" are not read
- C-core-document-3 Documents under hidden directories and under symbolic links to directories are not read at any depth, and neither a finding nor a stop is reported
- C-core-document-4 Symbolic links to files are read, and if there is a symbolic link without a target, check stops on the grounds of unreadable file
- C-core-document-13 Even if there is a socket whose name ends with ".md" under the location, check does not read it, reports neither a finding nor a stop, and reads the other documents

## A decision document has one title, and each title from the second on is a multiple_titles error

Status: implemented

### Problem

Silently skipping the second and later "# " titles would silently pass input that does not fit the form ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)). What the person reading the finding wants to know is which title should be deleted.

### Decision

When a decision document has two or more titles, one multiple_titles error is reported for each title from the second on.

How a document without a title is treated is decided in [Write one decision as text in Problem, Decision, Rejected options and Confirmations](decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations) and in [missing_title on a decision document without a title has the document name as its detail](findings.md#missing_title-on-a-decision-document-without-a-title-has-the-document-name-as-its-detail).

### Confirmations

- C-core-document-5 For a decision document with two or more titles, one multiple_titles error is reported for each title from the second on

## For the paragraph after the title, only its presence is checked, not its content, line count or separation of responsibilities

Status: implemented

### Problem

Separation of a document's responsibilities is a judgement of meaning. What matters for granularity is not the amount of text but whether responsibilities are separated; when they are not separated, there is more to decide, and the text grows. What the CLI can see goes only as far as whether it is written what the document covers. The separation itself is examined in a review that looks for problems, by asking whether things outside the scope of that document are mixed in.

### Decision

check only examines whether there is a paragraph after the title saying what the topic covers ([Write one decision as text in Problem, Decision, Rejected options and Confirmations](decision-documents.md#write-one-decision-as-text-in-problem-decision-rejected-options-and-confirmations)). The content and the line count of that paragraph are not checked. The separation of a document's responsibilities is not judged.

### Confirmations

- C-core-document-6 (by reading) The code of check only examines whether there is a paragraph after the title, does not check its content or line count, and does not judge the separation of a document's responsibilities

## kotowari counts "\n", "\r\n" and a lone "\r" each as one line ending

Status: implemented

### Problem

kotowari outputs line numbers in findings. The Markdown reader that reads documents counts a lone "\r" as a line separator, following CommonMark. If kotowari's own counting differed from this, the two line numbers would drift apart in a document containing a lone "\r", and the line of a finding and the range of an unclosed code block would shift (it was measured that items before an unclosed code block were dropped). Even if only test files were counted differently, the line numbers of markers would shift.

### Decision

When kotowari counts lines, it counts "\n", "\r\n" (one line ending) and a lone "\r" each as one line ending. Even if the last line has no line break, that line is counted as one line. A document with empty content is counted as 0 lines.

This counting is used for every file for which kotowari outputs line numbers. It applies to decision documents, test files and the sources of mutation outcomes ([An entry of the list identifies a mutation by the file, the description of the change, and the text of the line](equivalents.md#an-entry-of-the-list-identifies-a-mutation-by-the-file-the-description-of-the-change-and-the-text-of-the-line)).

### Confirmations

- C-core-document-7 Line separation makes "\n", "\r\n" and a lone "\r" each one line ending, does not count "\r\n" as two lines, and counts the last line even if it has no line break
- C-core-document-8 Even if the lines of a test file are separated by "\r\n" or a lone "\r", the line number of a finding on a marker is the line after separation, and no "\r" remains in the detail
- C-core-document-9 Even if the lines of a decision document are separated by a lone "\r", the line number of a finding on the decision document is the line after separation

## The confirmation-row body and the fingerprint of a decision in a decision document are taken with the same line counting

Status: implemented

### Problem

If the body of a confirmation row and the fingerprint of a decision's "Decision" and "Confirmations" were taken with another way of splitting that does not count a lone "\r" as a line separator, then in a document separated by lone "\r", the body of query would be empty and the fingerprint would be the fingerprint of empty content. If the fingerprint stays empty, the guide is not reported as stale even when the Decision is rewritten.

### Decision

The body of a confirmation row and the fingerprint of a decision are also taken from the lines split as in [kotowari counts "\n", "\r\n" and a lone "\r" each as one line ending](#kotowari-counts-n-rn-and-a-lone-r-each-as-one-line-ending). The state line is also read with the same splitting.

Outputting the total line count of the decision documents in status is decided in [The documents group counts the number of decision documents read and their lines](status.md#the-documents-group-counts-the-number-of-decision-documents-read-and-their-lines).

### Confirmations

- C-core-document-10 Even in a decision document whose lines are separated by lone "\r", the body of a confirmation row (the "body" of query) and the fingerprint of a decision are the same as for the same document separated by "\n"
- C-core-document-14 Even in a decision document whose lines are separated by lone "\r", the state line is read and the states are counted

## The line-count and requirement-count limit notices too_many_lines and too_many_requirements are removed

Status: proposed

### Problem

In the old form, a too_many_lines notice was reported when the line count of a document exceeded a limit, and a too_many_requirements notice when the number of requirements exceeded a limit. For decision documents it was decided not to report a notice for the line-count limit, and the notice for the requirement-count limit is also removed, since requirements disappear. The reason is not recorded.

The code that reads documents in the old form still reports these two notices.

### Decision

check does not report the too_many_lines and too_many_requirements notices, however many lines and decisions a decision document has. These two finding kinds are removed together with the code that reads documents in the old form. Removing the configuration keys that hold the limits is decided in [The keys for the limits on line count and number of requirements, limits.lines and limits.requirements, are removed](config.md#the-keys-for-the-limits-on-line-count-and-number-of-requirements-limitslines-and-limitsrequirements-are-removed).

### Confirmations

- C-core-document-11 check does not report the too_many_lines and too_many_requirements notices even for a decision document with many lines and many decisions

## The inside of a code block in a decision document is read neither as decision form nor as a quotation

Status: implemented

### Problem

It had to be decided whether a code block in a decision document is read as part of the document's form or as a target of checks. In the specification documents until now as well, the inside of code blocks was excluded from the targets of checks. The reason is not recorded.

### Decision

The inside of a code block in a decision document is excluded from the targets of checks. A heading inside a code block is not made a decision heading, and a line in list form is not made a confirmation row. It is not counted as a quotation of a surface name either ([A surface is in the specification when its name is quoted in the "If the name of a surface is quoted in "Decision" or "Confirmations" of a decision, that surface is in the specification" or "Confirmations" of a decision](surface.md#if-the-name-of-a-surface-is-quoted-in-decision-or-confirmations-of-a-decision-that-surface-is-in-the-specification)). A code block written under a confirmation row is not included in the name of that row.

How an unclosed code block is treated is decided in [An unclosed code block is an unclosed_code_block error, and from its start to the end of the document is not read](ir-input.md#an-unclosed-code-block-is-an-unclosed_code_block-error-and-from-its-start-to-the-end-of-the-document-is-not-read).

### Confirmations

- C-core-document-12 A "## " heading and a line in confirmation form written inside a code block are not counted as a decision or as a confirmation row
- C-core-document-15 The content of a code block written under a confirmation row is not included in the "name" of that row in list and query
