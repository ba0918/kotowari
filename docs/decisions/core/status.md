# Completeness summary

English | [日本語](status.ja.md)

This document covers how "kotowari status" summarizes the results of the same reading and checks as check, and outputs whether things are complete (complete) as counts and a boolean. What complete is determined by is decided in [Write questions handed back to people in the "Write questions handed back to people in the "Open" section of the topic document" section of the topic document](decision-documents.md#write-questions-handed-back-to-people-in-the-open-section-of-the-topic-document), and outputting the count per decision state in [The implementing side changes the status, and decisions not built yet stay "The implementing side changes the status, and decisions not built yet stay "proposed""](confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed).

## status reads the same things as check and runs the same checks, and outputs counts and complete instead of findings

Status: implemented

### Problem

check outputs findings one by one, and list outputs decisions and rows one by one. On top of these, a command is needed that answers "is this specification complete now" with counts and a state that can be grasped at once (the boolean complete). The user agreed on this form after seeing a sample made from real data. Which counts to add or drop is judged while using it.

The counts are made from the findings of check, so they cannot be output without running the same checks as check. If the rules for judging completeness were in two places, check and status, they would diverge.

### Decision

"kotowari status" reads the decision documents, test files, guides and overview source data from the same configuration and locations as "kotowari check". If "surface.rules" is not an empty list, it also reads the surface files, the surface rule files and the list of unspecified surfaces. It then runs the same checks as check, reports no findings, and outputs one summary to standard output.

How complete is determined is only what is decided in [Write questions handed back to people in the "Write questions handed back to people in the "Open" section of the topic document" section of the topic document](decision-documents.md#write-questions-handed-back-to-people-in-the-open-section-of-the-topic-document) (check has zero errors, and no topic document has an item in "Open"), and status has no judgement rules of its own. Notices of check are not errors, so they do not prevent complete. If a criterion not there is needed, it is added to check.

"kotowari status" stops, when a condition under which check stops holds (a configuration error, an unreadable file, an argument error), with the same reason and wording as check.

### Confirmations

- C-core-status-1 "kotowari status" does not output findings one by one, and the "findings" group holds only the counts of errors and notices
- C-core-status-2 If check has errors, the "complete" of "kotowari status" is false
- C-core-status-3 When the configuration file cannot be read as YAML, "kotowari status" exits with code 2, and the first line of standard error is the same as for "kotowari check"
- C-core-status-4 When the guide location overlaps the test file location, "kotowari status" stops with a configuration error, as check does

## The exit code is 0 when complete, and 1 otherwise

Status: implemented

### Problem

We want CI to be able to judge completeness just by running one "kotowari status".

### Decision

"kotowari status" makes the exit code 0 if "complete" is true, and 1 if it is false. A stop is 2.

### Rejected options

- As a read-only command, always make the exit code 0. Unlike list, status holds a judgement, so the judgement is reflected in the exit code.

### Confirmations

- C-core-status-5 If "complete" is true, the exit code of "kotowari status" is 0, and if false, 1

## The summary outputs, per group, the counts of test markers, guides, overview, surfaces and findings

Status: implemented

### Problem

It had to be decided what status counts. For guides and overview source data, the number read is output so that they do not go silent when a glob mistake makes them zero. The overview source data is treated the same as guides. A surface not in the specification is an error, so complete becomes false because of it, but the counts of surfaces are also output to make visible how much is relying on the list of unspecified surfaces.

### Decision

The summary has the following groups. The group names and keys are English, and the same words are used in JSON and "text".

| Group | Keys | Contents |
|---|---|---|
| tests | marks, files | The number of test markers, and the number of test files read per extension |
| guides | files, marks | The number of guides read, and the number of guide markers |
| overview | files, marks | The number of overview source data files read, and the number of their markers |
| surface | total, specified, unspecified | The number of surfaces, the number of those written in decision documents, and the number listed in the list of unspecified surfaces |
| findings | error, notice | The number of check errors, and the number of notices |
| complete | (value only) | true or false |

The group of counts per decision state is decided in [The implementing side changes the status, and decisions not built yet stay "The implementing side changes the status, and decisions not built yet stay "proposed""](confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed), and the counts of confirmation rows in [The number of confirmation rows is output split into those with tests, those without, and those by reading](#the-number-of-confirmation-rows-is-output-split-into-those-with-tests-those-without-and-those-by-reading).

### Confirmations

- C-core-status-6 The "tests" group shows the number of test markers and the number of test files read per extension
- C-core-status-7 The "guides" group and the "overview" group each show the number of files read and the number of markers
- C-core-status-8 The "surface" group shows the number of surfaces, the number of surfaces written in decision documents, and the number of surfaces listed in the list of unspecified surfaces
- C-core-status-9 The "findings" group shows the number of check errors and the number of notices separately

## The number of confirmation rows is output split into those with tests, those without, and those by reading

Status: implemented

### Problem

The old summary output, per requirement, the numbers with and without tests, and the number of requirements not checked by tests that had a way of checking. The user agreed on this form after seeing a sample made from real data, and decided to judge which counts to add or drop while using it. In the new form, the unit for counting tests is the confirmation row ([Check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it)), and a row not checked by tests has "(by reading)" written at its head ([Write "Write "(by reading)" at the head of a line not checked by tests" at the head of a line not checked by tests](confirmations.md#write-by-reading-at-the-head-of-a-line-not-checked-by-tests)). Before this decision, what status output about the new form was only the count per decision state, and how many confirmation rows had tests was not visible as a count.

### Decision

The summary of "kotowari status" outputs the number of confirmation rows split into the following three.

- The number of rows checked by tests that have a test pointing at them
- The number of rows checked by tests that have no test pointing at them
- The number of rows with "(by reading)" written at their head

### Confirmations

- C-core-status-13 The summary of "kotowari status" shows, separately, the number of confirmation rows with tests, the number of rows without tests and the number of "(by reading)" rows, and a "(by reading)" row is counted neither among the rows with tests nor among those without

## The documents group counts the number of decision documents read and their lines

Status: proposed

### Problem

status outputs the number of documents read and their line count. Since the specification is held only in decision documents, what is counted needs to be the decision documents.

### Decision

The "files" and "lines" of the "documents" group are the number of decision documents read and the total of their line counts.

### Confirmations

- C-core-status-10 The "files" and "lines" of the "documents" group are the number of decision documents read and the total of their line counts

## The output is json or text, and text has one line per group

Status: implemented

### Problem

We want readers not to have to memorize the correspondence between "text" and JSON. Alignment spaces change with the number of digits of the values, and are hard to make part of the contract. The fixed words kotowari decides match the stop wording and the JSON keys, which are English.

### Decision

The "--format" of "kotowari status" accepts only "json" and "text", and the default is "json". With "json", one JSON whose top level is only the keys of the groups is output.

With "text", one line of the form "group key=value key=value" is output per group. The key words are the same as in JSON, the values are separated by one half-width space, and no alignment spaces are inserted. The line of "complete" is "complete true" or "complete false".

### Confirmations

- C-core-status-11 With "text", one line per group is output with "group key=value" separated by one half-width space, and the line of "complete" is "complete true" or "complete false"
- C-core-status-12 The top-level keys of the "json" output are only the group names and "complete"
