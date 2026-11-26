# Reading one entry

English | [日本語](query.ja.md)

This document covers how "kotowari query" receives one ID and outputs the decision or confirmation row with that ID in the form of an entry of [List of decisions and confirmations](list.md) with the body added. That passing a confirmation ID returns the file, line and name of the tests that point at it is itself decided in [Check counts, per confirmation row, whether a test points at it](confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it).

## query receives one ID and outputs the decision or confirmation row with that ID as a list entry with the body added

Status: implemented

### Problem

list outputs every entry one by one. As the place to get more detailed information, a command that reads one entry is needed. When an LLM reads one entry, we want it not to have to open the document again. That is the value query has that list does not.

If an entry of query is an entry of list with things added, query reads as "list narrowed down to one entry and made more detailed", and the explanation of the keys is not duplicated.

Two or more rows with the same ID are themselves a check error. There is no reason for query to hide that, and it is better that the person fixing the duplicate can see all of them.

### Decision

"kotowari query" reads the decision documents and test files from the same configuration and locations as "kotowari check", and outputs in "items" every decision and confirmation row that has the same ID as the positional argument. It reports no findings, and makes the exit code 0. Even if the decision documents have check errors, the entries it could read are output. If there are two or more confirmation rows with the same ID, all of them are output.

An entry of "items" has the same keys as [an entry of list](list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint), with the sequence of body lines ("body") added. The body of a confirmation row is the characters of that row, including wrapped lines, listed as they are. The range of the body when a decision is looked up is decided in [The body when a decision is looked up is the whole decision](#the-body-when-a-decision-is-looked-up-is-the-whole-decision), and the reverse references ("referenced_by") in [The reverse references of a decision hold the decisions that link to that decision's heading](#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading).

### Rejected options

- Name the command show. The decisions until then spoke of it by the name query.

### Confirmations

- C-core-query-1 Passing a confirmation ID returns that row as one entry with the same keys as a list entry plus "body", and "body" is the characters of the row, including wrapped lines, listed as they are
- C-core-query-2 Passing an ID that points at a decision returns that decision as one entry with the same keys as a list entry plus "body"
- C-core-query-3 If there are two or more confirmation rows with the same ID, "kotowari query" returns all of them, and the exit code is 0
- C-core-query-4 Even if the decision documents have check errors, "kotowari query" returns, with exit code 0, the entries it could read, and puts no findings in the output

## The body when a decision is looked up is the whole decision

Status: proposed

### Problem

That an LLM reading one entry does not have to open the document again is the value query has that list does not. The current query, when a decision is looked up, returns in "body" only the lines of that decision's "Decision" and "Confirmations" sections (the lines the guide fingerprint is taken from). What was decided can be read, but to read why it was decided so (Problem) and what was turned down (Rejected options), the document has to be opened again.

### Decision

When a decision is looked up with "kotowari query", "body" holds the lines of the whole decision (Problem, Decision, Rejected options, Confirmations).

### Confirmations

- C-core-query-14 Passing an ID that points at a decision puts in "body" the lines of that decision's Problem, Decision, Rejected options and Confirmations sections

## The reverse references of a decision hold the decisions that link to that decision's heading

Status: proposed

### Problem

Where a decision is pointed at from could be looked up with query when reading one entry (reverse references). Only the query side holds reverse references, and the side that points holds no reverse information. In the new form, a decision points at another decision with a link to its heading ([To refer to another decision, link to its heading](decision-documents.md#to-refer-to-another-decision-link-to-its-heading)). The current query returns empty reverse references for both decisions and confirmation rows.

### Decision

When a decision is looked up with "kotowari query", the "referenced_by" of the entry in "items" holds the decisions that link to that decision's heading. The order of "referenced_by" is, as in the order of list, by "path" ascending, and within the same "path" by "line" ascending.

### Confirmations

- C-core-query-15 If another decision links to the heading of the looked-up decision, the linking decision is in "referenced_by", and the "referenced_by" of a decision that no decision links to is an empty sequence
- C-core-query-16 The order of "referenced_by" is by "path" ascending, and within the same "path" by "line" ascending

## If the positional argument is not exactly one ID, or that ID does not exist, it is a stop with an argument error

Status: implemented

### Problem

Mistakes in the number and form of positional arguments stop in the same way as the positional arguments of "kotowari mutants". When the given ID exists nowhere, a non-zero exit code with a reason is less likely to be misread by an LLM than an empty "items" with exit code 0.

### Decision

"kotowari query" stops on the grounds of an argument error when there is not exactly one positional argument, when the positional argument has neither the form of a confirmation ID nor the form of an ID that points at a decision, or when neither a decision nor a confirmation row has the same ID as the positional argument. The detail when nothing has the ID is "unknown id: " followed by the characters of the positional argument. When "--help" or "--version" is present, [it is taken before the other arguments](cli-environment.md#--help-and---version-are-taken-before-the-other-arguments-and-kotowari-outputs-the-usage-or-the-version-and-ends).

That, when called from the library, the ID can be taken out of the failure for a missing ID is decided in [The ID can be taken from the failure of a query for an unknown ID](library-api.md#the-id-can-be-taken-from-the-failure-of-a-query-for-an-unknown-id).

### Confirmations

- C-core-query-5 When "kotowari query" has no positional argument, or has two or more, it exits with code 2 with an argument error
- C-core-query-6 When the positional argument is not of the ID form, "kotowari query" exits with code 2 with an argument error
- C-core-query-7 Passing an ID that is in no decision document makes "kotowari query" exit with code 2, standard output is empty, and the first line of standard error is "argument error: unknown id: " followed by the given ID

## query stops under the same conditions as check, and does not read guides or surfaces

Status: implemented

### Problem

query, like list, is the same read as check, so what it accepts and how it stops are the same as check. query does not need guides or surfaces.

query does not need the overview source data either. Reading it would make even query, which only reads one entry, stop on an overlap of the overview source data location or an unreadable source data file. The reason is not recorded.

### Decision

"kotowari query" accepts only the same four options as list ("--format", "--config", "--help", "--version"). When a condition under which "kotowari check" stops holds (a configuration error, an unreadable file, an argument error), it stops with the same reason and wording as check.

However, "kotowari query" does not read guides, and does not stop because of the guide location or its reading. It also does not read surface files, surface rule files or the list of unspecified surfaces, and does not stop because of reading them. It does not read the overview source data either, and does not stop because of its location or its reading.

### Confirmations

- C-core-query-8 When the configuration file cannot be read as YAML, "kotowari query" exits with code 2, and the first line of standard error is the same as for "kotowari check"
- C-core-query-9 "kotowari query" does not stop even if a guide file cannot be read, or the guide location overlaps the test file location
- C-core-query-10 "kotowari query" does not stop even if the file of the list of unspecified surfaces does not exist, or a surface file cannot be read
- C-core-query-13 "kotowari query" does not stop even if the overview source data location overlaps the guide location, or a source data file is not UTF-8

## The output is json or text, and text follows the list line with the body lines

Status: implemented

### Problem

If it is added on top of the "text" of list, there is no additional way of reading to learn. The body is the raw lines of the document, so indentation alone tells it apart.

The fixed words kotowari decides match the stop wording ([The reason for a stop is shown with fixed English wording](cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording)) and the JSON keys, which are English.

### Decision

The "--format" of "kotowari query" accepts only "json" and "text", and the default is "json". With "json", one JSON whose top level is only "items" is output.

With "text", for each entry, the same first line and test lines as [the "text" of list](list.md#the-output-is-json-or-text-and-there-are-no-filtering-options) are output, followed by each line of "body" indented by two half-width spaces.

The fixed words in the output that kotowari decides are English. Characters copied from decision documents and tests are output as they are.

### Confirmations

- C-core-query-11 The only top-level key of the "json" output is "items"
- C-core-query-12 With "text", after the same first line and test lines as the "text" of list, each line of "body" is output indented by two half-width spaces
