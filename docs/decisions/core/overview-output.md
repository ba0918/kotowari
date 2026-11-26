# Reporting the overview source data

English | [日本語](overview-output.ja.md)

Covers how the results of checking the overview source data are added to the output of "kotowari check" and "kotowari status". The checks themselves are decided in [Overview source data](overview-data.md). The "overview" group of status is decided in [Completeness summary](status.md), under [The summary outputs, per group, the counts of test markers, guides, overview, surfaces and findings](status.md#the-summary-outputs-per-group-the-counts-of-test-markers-guides-overview-surfaces-and-findings), and the form in which the library passes check results to core in [In-memory input and already-read results](library-inputs.md), under [Named additional finding groups are received, and ordered and counted together with the other findings](library-inputs.md#named-additional-finding-groups-are-received-and-ordered-and-counted-together-with-the-other-findings).

## The JSON of check carries an overview group of the same form as guides

Status: implemented

### Problem

How to put the results of checking the overview source data into the output of check needed deciding. It is handled the same as guides, and no new rule is made.

### Decision

At the top level of the JSON of "kotowari check", an "overview" group of the same form as "guides" is output. "files" is the number of overview source data files read, and "marks" is the number of well-formed markers inside the source data. status outputs the same group.

### Confirmations

- C-core-ovout-1 The top-level "overview" of "kotowari check --format json" shows the number of source data files read as "files" and the number of well-formed markers inside the source data as "marks"

## Findings of the source data are ordered and counted like other findings, and affect the exit code and complete

Status: implemented

### Problem

check reports malformations of the source data as errors at their places. Putting it in check also catches them in the checks before a commit. If only the overview findings had a separate ordering or separate pass rules, the rules for the order of output would split in two.

### Decision

Findings of the overview source data are added to the findings of "kotowari check" and "kotowari status". Together with the other findings, they are ordered as in [Order and lines of findings](finding-order.md), under [Findings are arranged by path, line, kind and detail, in that order](finding-order.md#findings-are-arranged-by-path-line-kind-and-detail-in-that-order), and are counted, like the other findings, in "counts", the exit code and status's "complete". status's complete requires check to have zero errors, so it is false if the source data has errors.

### Confirmations

- C-core-ovout-2 If the source data has errors, they go into "findings" and "counts" of "kotowari check --format json", and the exit code becomes 1
- C-core-ovout-3 Findings of the source data are placed in "findings" by the same ordering rules as other findings
- C-core-ovout-4 Even with no other errors, if the source data has errors, "complete" of "kotowari status" becomes false
