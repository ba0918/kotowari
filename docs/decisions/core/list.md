# List of decisions and confirmations

English | [日本語](list.ja.md)

This document covers how "kotowari list" outputs the decisions and confirmation rows of the decision documents, together with the tests that point at them, in a form people and LLMs can read. What it reads is the same as check, and it reports no findings. Reading one entry in detail is decided in [Reading one entry](query.md), and counts and completeness in [Completeness summary](status.md).

## list reads the same things as check, reports no findings, and outputs every decision and confirmation row it could read

Status: implemented

### Problem

check already assembles the correspondence between the specification and test markers, but there was no opening that output it in a form people and LLMs can read. The correspondence between the specification and tests could only be kept in a hand-written table, and a table kept by hand rots. If it is output as another view of the same data, neither new state nor external dependencies are added.

If an LLM, after reading the list, calls another command again to get missing information, the round trips of reading increase. One run of list provides what is needed. A finding and an entry of the list are different things, so they are not mixed. Reporting findings is the job of check, and list is a read, so there is also no reason to stop the output because there are errors.

### Decision

"kotowari list" reads the decision documents and test files from the same configuration and locations as "kotowari check". It outputs every decision it could read, and every confirmation row with a well-formed ID, in "items" on standard output. It reports no findings, and makes the exit code 0. Even if the decision documents have check errors, the decisions and rows it could read are output.

One run of list provides what is needed to read the correspondence between decisions and tests, and further details are obtained with [Reading one entry](query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added). The form of an entry of "items" is a form separate from the findings of check ("findings").

### Rejected options

- Make trace, which outputs the correspondence between the specification and tests, a command separate from list. It would be the same as the information contained in the output of list.
- Keep the hand-written table of the correspondence between the specification and tests, unrelated to list. One table kept by hand would remain.

### Confirmations

- C-core-list-1 "kotowari list" outputs every decision of the decision documents in the configured location and every confirmation row with a well-formed ID, and does not output rows with malformed IDs
- C-core-list-2 Even if the decision documents have check errors, "kotowari list" outputs, with exit code 0, the decisions and rows it could read, and puts no findings in the output

## A confirmation row gets the tests that point at it, and a decision gets a fingerprint

Status: implemented

### Problem

So that one run of list provides what is needed, it had to be decided what one entry holds. check counts whether there are test markers, regardless of the language of the tests. In a language without a query, only the test name cannot be obtained. When the same test has two markers with the same ID, making one entry per test would need one more rule for which line to make "line".

A guide marker holds the heading of a decision and the fingerprint of that decision (how the staleness of guides is judged is decided in [A guide is stale when the Decision or Confirmations change](languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change)). An opening is needed to get the value to write.

### Decision

Every entry of "items" holds an ID ("id"), a kind ("kind"), a name ("name") and a position ("path" and "line"). The value of "kind" is "decision" for a decision and "confirmation" for a confirmation row. "path" is [relative to the base directory](base-directory.md#paths-in-configuration-values-and-the-path-of-the-output-are-relative-to-the-base-directory).

An entry of a confirmation row holds whether the row is a "(by reading)" row ("by_reading"), and the sequence of tests that have a marker containing the row's ID ("tests"). An entry of "tests" holds the "path" of the test file, the "line" of the line with the marker, and the "name" of the test. If the same test has several markers with the same ID, there is one entry per occurrence of a marker. Tests in a language without a query are listed the same way, and the "name" of a test whose name cannot be obtained is null ([A test's name is taken from the text the query captured](test-discovery.md#a-tests-name-is-taken-from-the-text-the-query-captured)).

An entry of a decision holds the fingerprint written in guide markers ("fingerprint").

### Confirmations

- C-core-list-3 The "tests" of a confirmation row hold, one entry per occurrence of a marker containing the row's ID, the test file, the line of the marker and the name of the test
- C-core-list-4 Tests in a language without a query, and tests whose name cannot be obtained, are listed in "tests" with "name" null
- C-core-list-5 The "by_reading" of a "(by reading)" row is true, and that of other confirmation rows is false
- C-core-list-6 The "fingerprint" of a decision entry is the same value as the fingerprint written when a guide marker points at that decision

## The order is by path, and by line within the same path

Status: implemented

### Problem

If the order is the same as the output of check, readers are not confused.

### Decision

The entries of "items" are ordered by "path" ascending, and within the same "path" by "line" ascending. The "tests" of an entry are also ordered by "path" ascending, and within the same "path" by "line" ascending.

### Confirmations

- C-core-list-7 "items" are ordered by "path" ascending, and within the same "path" by "line" ascending, and the "tests" of an entry are ordered the same way

## The output is json or text, and there are no filtering options

Status: implemented

### Problem

Each additional output format adds to the contract. A form easier for people to read was left to be considered separately, and the minimum form was chosen first. If the output is hard to read, users will ask for a change.

Having filtering options would depart from the aim of providing what is needed in one run of list. Narrowing down to one entry to read is the job of query.

### Decision

The "--format" of "kotowari list" accepts only "json" and "text", and the default is "json" (matching check). There are no filtering options. Every entry is output, and narrowing is done by the reader or with "kotowari query".

With "json", one JSON whose top level is "items" is output ("translations", added with two or more languages, is decided in [the next decision](#with-two-or-more-languages-the-blob-hash-of-each-pair-is-output-too)).

With "text", each entry of "items" is output as one line of the form "ID verification name path:line tests=count", separated by half-width spaces. The "verification" field is "reading" for a "(by reading)" row and "-" otherwise. Right after it, for each entry of "tests", a line "path:line name" indented by two half-width spaces follows. If the name is null, "-" is written.

### Rejected options

- Add the yaml format from the start. Each additional format adds to the contract. Add it if needed, after readability has been considered.

### Confirmations

- C-core-list-8 With one language, the only top-level key of the "json" output is "items"
- C-core-list-9 With "text", an entry is output as one line "ID verification name path:line tests=count", the verification field of a "(by reading)" row is "reading" and "-" otherwise, below it for each test entry an indented line "path:line name" follows, and if the name is null "-" is written

## With two or more languages, the blob hash of each pair is output too

Status: implemented

### Problem

The identifiers of the contents of both sides (blob hashes) are copied into the agreement record of a pair. We want an LLM to be able to get them in one run. However, list does not read guides and overview source data, so to output the hashes of the pairs in those locations, the locations had to be traversed.

### Decision

When the configuration's language list ("languages") has two or more languages, "translations" is added to the top level of "json". "translations" is a sequence of one entry per pair, and an entry holds the path of the first-language side ("path") and a sequence of one entry per language ("sides"). An entry per language holds the language tag ("language"), the path of that side ("path") and the blob hash of that side ("blob"). With one language, "translations" is not output.

With "text", after the lines of "items", each pair is output on one line: the path of the first-language side followed by "language-tag=hash" for each language.

For this, with two or more languages, the locations of guides, overview source data and the table of contents are traversed in the same way as check. If a location cannot be read, it stops in the same way as check. The contents are not checked.

### Confirmations

- C-core-list-10 With two or more languages, for each pair of guides, overview source data and the table of contents, the path and blob hash per language are output in "translations", and with "text" each pair is output on one line
- C-core-list-11 With two or more languages, under a configuration for which check stops on the guide location, "kotowari list" also stops with the same standard error as check, and with one language it does not stop under the same configuration

## Pairs of decision documents are output in translations too

Status: implemented

### Problem

Decision documents can also be kept as multilingual pairs ([Decision documents can also be kept as multilingual pairs](languages-and-guides.md#decision-documents-can-also-be-kept-as-multilingual-pairs)). The reason of getting, in one run, the hashes to copy into the agreement record of a pair applies as it is to pairs of decision documents too.

### Decision

When the language list has two or more languages, the pairs in the decision document location are also output in "translations", one entry each.

### Confirmations

- C-core-list-12 With two or more languages, for each pair of decision documents, the path and blob hash per language are output in "translations"

## list stops under the same conditions as check, and does not read guides or surfaces

Status: implemented

### Problem

list is the same read as check, so what it accepts and how it stops are the same as check. On the other hand, list does not need guides. The fingerprint of a decision is determined by the decision documents alone. Reading surfaces and outputting their counts is done by check and status.

list with one language does not need the overview source data either. Reading it would make even list, which only outputs the list of decisions and rows, stop on an overlap of the overview source data location or an unreadable source data file. The reason is not recorded.

### Decision

"kotowari list" accepts only four options: "--format", "--config", "--help" and "--version". When a condition under which "kotowari check" stops holds (a configuration error, an unreadable file, an argument error), it stops with the same reason and wording as check.

However, "kotowari list" does not read guides, and does not stop because of the guide location or its reading. It also does not read surface files, surface rule files or the list of unspecified surfaces, and does not stop because of reading them. With one language, it does not read the overview source data either, and does not stop because of its location or its reading. Traversing the locations with two or more languages is decided in [With two or more languages, the blob hash of each pair is output too](#with-two-or-more-languages-the-blob-hash-of-each-pair-is-output-too).

### Confirmations

- C-core-list-13 When the configuration file cannot be read as YAML, "kotowari list" exits with code 2, and the first line of standard error is the same as for "kotowari check"
- C-core-list-14 Adding "--tool" or a positional argument to "kotowari list" stops it with an argument error
- C-core-list-15 With one language, "kotowari list" does not stop even if a guide file cannot be read, or the guide location overlaps the test file location
- C-core-list-16 "kotowari list" does not stop even if the file of the list of unspecified surfaces does not exist, or a surface file cannot be read
- C-core-list-17 With one language, "kotowari list" does not stop even if the overview source data location overlaps the guide location, or a source data file is not UTF-8
