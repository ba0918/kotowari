# Guide markers

English | [日本語](guides.ja.md)

This document covers where guides (documents on usage, read by the people who use the product) are placed and how they are read, the form and fingerprint of guide markers, and how stale sections are found. A guide is a document that a person or an LLM writes after reading the specification; kotowari does not generate it from the specification. Which changes in the decision a marker points at make it stale is decided in [A guide is stale when the Decision or Confirmations change](languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change). The "guides" group in the JSON of check and in status is decided in [The top level of check's JSON shows the findings, the count per kind, and the counts of what was read](output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read) and [The summary outputs, per group, the counts of test markers, guides, overview, surfaces and findings](status.md#the-summary-outputs-per-group-the-counts-of-test-markers-guides-overview-surfaces-and-findings).

## A stale section is found by comparing the fingerprint written in the marker with the current fingerprint of the decision

Status: implemented

### Problem

Reviewing the whole guide every time the specification is fixed is a large effort. We want the machine to narrow the review down to the sections that need it. If the narrowing is done only inside the change that altered the specification, there is no way to notice later when that change forgot to fix a guide.

### Decision

A guide section has, as a marker, the decision the section explains and the fingerprint of that decision at the time of writing. check compares the written fingerprint with the current fingerprint of the decision, and keeps reporting a finding as long as a stale section exists. The narrowing can be done at any time, not only when the specification is changed. This matches kotowari's policy of checking by machine instead of relying on procedures being followed.

The fingerprint is output in the "fingerprint" of an entry of kotowari list and kotowari query, and the person or LLM who reviewed the section copies it into the marker. kotowari does not rewrite guides. With copying, kotowari needs no command that writes files. kotowari list outputs one entry per decision, and its "id" is the target written in the marker. Passing that target to kotowari query returns the fingerprint of that decision.

The fingerprint is the first 8 characters, in lowercase hexadecimal, of the SHA-256 of the string that joins the lines the fingerprint is taken from with "\n". Line-ending characters (including the "\r" of "\r\n") are not part of a line. Without a fixed algorithm, the value could change just from upgrading the Rust version. 8 characters is a length chosen to make copying easy, and with the few hundred entries of one project, accidental collisions need not be a concern. If line-ending characters were included, the fingerprint of the same content would change with the line-ending form of the checkout.

### Rejected options

- Hold no fingerprint, and narrow down by taking the fixed items from the diff of the change that altered the specification. Forgotten fixes remain.

### Confirmations

- C-core-guides-1 Passing the target of a decision to kotowari query returns one entry whose kind is "decision", with the fingerprint of that decision in "fingerprint"
- C-core-guides-2 kotowari list outputs, for each decision, one entry whose id is the target written in the marker
- C-core-guides-3 check does not report guide_stale for a marker that copied the fingerprint from kotowari list or kotowari query
- C-core-guides-4 The fingerprint of a decision with the same content is the same in a decision document with "\r\n" line endings and one with "\n" line endings

## Guides are chosen by the globs of guides.files, and only check and status read them

Status: implemented

### Problem

Some projects have no guides. If the way of writing the guide location differs from that of the test location, there is more to remember. If each location has its own reading rules, they are hard to remember.

### Decision

A list of globs is written in the configuration's "guides.files", and the files they match are read as guides. It is written the same way as "tests.files". If the key is absent or the list is empty, no guide is read and no guide check is done. Projects without guides are not affected.

Failures in reading guide files are treated the same as for "tests.files" ([Test files are selected with the globs of tests.files](test-discovery.md#test-files-are-selected-with-the-globs-of-testsfiles)). An element that cannot be read as a glob is a configuration error; an unreadable file, a file that is not UTF-8 and a symbolic link without a target are a stop; a glob that matches nothing is not an error.

Even if a glob of "guides.files" matches a file in the decision document location, it is not a stop. That file is read both as a decision document and as a guide. The reason for stopping an overlap with the test location ([When the guide and test locations overlap, it is a stop with a configuration error](#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error)) does not apply to decision documents, and reading the file as both does no harm.

Only check and status read guides, because status runs the same checks as check. kotowari list and kotowari query do not read guides, and do not stop because of the guide location or its reading. The fingerprints of list are determined by the decision documents alone, and need no guides.

### Confirmations

- C-core-guides-5 When the configuration has no "guides.files" key, check reads no guide, and does not report guide_stale even if a file with markers exists
- C-core-guides-6 When a file matched by "guides.files" is not UTF-8 or cannot be read, check stops
- C-core-guides-7 Even if a glob of "guides.files" matches a file in the decision document location, check does not stop, and that file is counted both as a decision document and as a guide
- C-core-guides-8 kotowari list and kotowari query do not stop even if a file matched by "guides.files" is not UTF-8

## When the guide and test locations overlap, it is a stop with a configuration error

Status: implemented

### Problem

In a test file of a language without a query, every "@kotowari[" in the file is picked up as a marker ([In files of a language without a query, all markers in the file are picked up](test-markers.md#in-files-of-a-language-without-a-query-all-markers-in-the-file-are-picked-up)). If a guide is also matched as a test file, the guide's markers are read as test markers. Neither one is silently given priority ([Unreadable input and broken input become a stop or an error finding, and are not silently skipped](cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped)).

With two or more languages, there are sides that are read as guides because they are in the same pair as a matched file, even though they are not matched by the globs of "guides.files". If such a side is also matched as a test file, it is read both as a guide and as a test, and the guide's markers are silently counted as test markers. Also, if a side matched by a glob were left out of the overlap judgement because the first-language side is missing, the configuration error of overlapping globs would be hidden behind the error of a missing paired side.

### Decision

When one file is matched by the globs of both "guides.files" and "tests.files", check and status stop on the grounds of a configuration error.

With two or more languages, the guides examined for overlap include, in addition to the files matched by the globs of "guides.files", the sides read as guides because they are in the same pair as a matched file (even when not matched by a glob) (how the sides of a pair are told apart is decided in [The sides of a pair are told apart by file names and configuration only](translation-pairs.md#the-sides-of-a-pair-are-told-apart-by-file-names-and-configuration-only)). A file matched by a glob is counted even if the first-language side is missing.

The detail of the stop is the path of the overlapping file relative to the base directory, followed by ": matched by both guides.files and tests.files". A relative path alone would look the same as other configuration errors (the path of the configuration file and an explanation), and the file would read like a broken configuration file. If two or more files overlap, only the first one in byte order of path is output. The detail of a stop is one line; fixing it and running again shows the next one.

### Confirmations

- C-core-guides-9 When one file is matched by both "guides.files" and "tests.files", check and status stop with exit code 2, and the detail of the stop is the relative path of that file followed by ": matched by both guides.files and tests.files"
- C-core-guides-10 When two files overlap, the detail of the stop shows only the one that comes first in byte order of path
- C-core-guides-23 With two or more languages, when a side in the same pair as a guide matched by "guides.files" is not matched by "guides.files" but is matched by "tests.files", check makes the same stop with the relative path of that side
- C-core-guides-24 With two or more languages, when a side whose first-language side is missing is matched by all three of "guides.files", "tests.files" and "overview.files", check and status stop with the detail that is the relative path of that side followed by ": matched by both guides.files and tests.files"

## Guide markers are read only from inside HTML comments

Status: implemented

### Problem

If markers appear in the rendered guide, readers are confused. HTML comments do not appear in the rendered guide. On the other hand, a guide that explains how to write markers puts its examples in code blocks. If every "@kotowari[" in the file were picked up, the examples would be read as markers.

Also, a CommonMark HTML block continues to the end of the line that contains "-->", so judging per block would read characters outside the comment.

### Decision

A guide marker is written inside an HTML comment, in a form that starts with "@kotowari[", the same as a test marker. It looks the same as a test marker.

check reads a guide as CommonMark, and reads as a marker only an "@kotowari[" inside the span from "<!--" to "-->". It does not read "@kotowari[" inside a CommonMark code block (the indented form and the fenced form) or a code span, or in the characters after "-->" (including the rest of the same line of an HTML block that starts with "<!--").

### Confirmations

- C-core-guides-11 check reads markers inside an HTML comment within a paragraph and inside an HTML comment that spans lines
- C-core-guides-12 check does not read as a marker an "@kotowari[" inside a code block (the indented form and the fenced form), inside a code span or in body text outside a comment, and reports neither invalid_marker nor guide_stale
- C-core-guides-13 check does not read as a marker an "@kotowari[" after "-->", even on the same line

## One entry of a guide marker joins the target of a decision and a fingerprint with a colon

Status: implemented

### Problem

One section sometimes explains several decisions. We want it to be clear which decision changed. Also, there is no reason to make it an error just because there is whitespace inside the brackets or around the colon. On the other hand, a broken form itself is a writing mistake, so we want to stop on it instead of silently skipping it. Test markers also make form mistakes errors.

### Decision

A guide marker points at a decision of a decision document. Its form is as in the following table. Each entry holds its own fingerprint, so it is clear which decision changed.

| Part | Form |
|---|---|
| Start | @kotowari[ |
| Contents | Each entry is written in the form "<target>:<fingerprint>", and entries are listed separated by commas. Whitespace may be placed around an entry, around the target and around the fingerprint. Even when there are two or more entries with the same target, each is treated as an entry of its own |
| Target | The path of the decision document and the anchor name of the link to the heading of that decision, joined with "#" ("<document path>#<heading anchor>") |
| Fingerprint | 8 lowercase hexadecimal characters ("0" to "9" and "a" to "f") |
| End | "]" on the same line as the start |

The anchor name of a heading is made by the same rule as when linking to a heading inside a decision document ([To refer to another decision, link to its heading](decision-documents.md#to-refer-to-another-decision-link-to-its-heading)).

When the form of a marker is wrong (the inside is empty or only separators, there is no "]" on the same line, there is an entry without ":", the part before ":" is not of the target form, or the part after ":" is not of the fingerprint form), check reports an invalid_marker error. When the target has the right form and only the decision does not exist now, it is treated not as a form error but as a stale marker ([A stale marker is a guide_stale notice, and does not change the exit code or complete](#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete)).

### Confirmations

- C-core-guides-14 check reads a marker with whitespace inside the brackets and around ":" the same as one without whitespace
- C-core-guides-15 check reports an invalid_marker error for a marker that has an entry without a fingerprint, an uppercase fingerprint, or an entry whose part before ":" is not of the target form (for example, one without the path or without the heading anchor name)
- C-core-guides-16 check reports an invalid_marker error for a marker whose inside is empty or only separators, and for a marker with no "]" on the same line
- C-core-guides-17 check compares the two entries listed in one marker one by one, and reports guide_stale only for the entry whose fingerprint differs

## A stale marker is a guide_stale notice, and does not change the exit code or complete

Status: implemented

### Problem

It is a problem if CI fails just because a guide lags behind. On the other hand, we want neglected guides to be visible at all times. Also, if a decision is deleted from the specification, the section that explained it is stale.

### Decision

For one entry of a marker of the right form, when the target decision does not exist, or when the current fingerprint of the target decision differs from the fingerprint written in the entry, check reports a guide_stale notice for each such entry. The finding points at the line of the marker. kotowari does not decide the range (section) a marker covers.

The detail of guide_stale is the string of the target, the fingerprint written in the entry and the current fingerprint, separated by spaces. When the target decision does not exist, the current fingerprint is "-". The single line of the notice tells which target to rewrite and to what.

Because it is a notice, it does not change the exit code or the complete of status. What decides complete is kept unchanged. The notices keep remaining in the counts of check and status, so neglected guides are visible at all times.

### Rejected options

- Make stale markers errors. CI fails just because a guide lags behind.
- Make a marker whose target decision does not exist an unresolved_reference error, the same as a test marker. CI fails just because a decision was deleted from the specification, which contradicts making stale markers notices.

### Confirmations

- C-core-guides-18 For an entry whose fingerprint differs from the current decision, one guide_stale notice is reported on the line of the marker, and its detail is the target, the written fingerprint and the current fingerprint, in that order
- C-core-guides-19 For an entry that points at a document that does not exist, a heading that does not exist, or a heading that is not a decision (the title, a section), a guide_stale notice with "-" as the current fingerprint is reported, and unresolved_reference is not reported
- C-core-guides-20 When there is no other error and only guide_stale exists, the exit code of check is 0, and the complete of status stays true

## The occasions for writing and reviewing guides are placed in the kotowari skill

Status: implemented

### Problem

The granularity of markers is not a problem big enough to stop on by machine. In a trial, sections about one symptom worked with 1 to 4 entries in one marker, and sections that cut across topics grew to 7 to 11 entries. Splitting finely is a matter of how guides are written. A machine cannot judge which decision the text of a guide explains, so kotowari cannot find a forgotten marker. The review procedure is to read the section a marker belongs to, so unless the way of writing fixes how a marker is tied to a section, the range to review is not fixed.

### Decision

kotowari does not check the range a marker covers, the granularity of markers, where markers are placed, or forgotten markers. Instead, the kotowari skill has an occasion for writing guides and one for reviewing them, with the following rules and procedures.

- One marker is placed next to the heading of the section it covers
- The guideline for the number of entries in one marker is up to a few, and a section that needs more is split
- When writing and when reviewing, the decisions that correspond to the content of the section are confirmed with kotowari query and added to the marker
- Before the current fingerprint in the detail of guide_stale is copied into the marker, the section is reviewed

### Confirmations

- C-core-guides-21 (by reading) The kotowari skill has an occasion for writing guides and one for reviewing them, and says that one marker is placed next to the heading of the section it covers and that the guideline for the number of entries in one marker is up to a few
- C-core-guides-22 (by reading) The kotowari skill has the procedure of confirming with kotowari query the decisions that correspond to the content of a section and adding them to the marker, and the procedure of reviewing the section before copying the current fingerprint of guide_stale
