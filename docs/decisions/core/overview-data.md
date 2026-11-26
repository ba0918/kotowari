# Overview source data

English | [日本語](overview-data.ja.md)

Covers where the overview source data (Markdown files of a fixed form that an LLM writes to draw the overview pages) is placed and how it is read, its form, the checks of the contents of parts, the opening lead, the list of documents covered, references inside parts, and markers inside the source data. That the list and references of the source data point at decision documents, and how staleness is judged, are decided in [The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents). How the findings found here are added to the output of check and status is decided in [Reporting the overview source data](overview-output.md), and the commands that build and show the overview from the source data in [Commands that build and show the overview](overview-commands.md).

## The overview source data is committed to the repository and maintained, and selected by the globs of overview.files

Status: implemented

### Problem

There is one overview per topic, and the overviews involved are revised at every brainstorm approval. Making them from scratch every time costs money and makes the look drift. Revising one needs the previous source data. The markers that find stale sections are also inside the source data.

The source data is not meant to be read by people. People read only the drawn pages. Data of tools that people do not read directly belongs with the configuration and the cache under ".kotowari/".

If the conditions for stopping while reading the source data were dropped, broken source data and files behind broken links would be skipped silently, and checking and writing would go on with those pages missing. The reason for stopping is not on record.

### Decision

The overview source data is maintained and committed to the repository. At every brainstorm approval, the previous source data is revised.

The source data is placed at ".kotowari/overview/<topic>.md", and the configuration's "overview.files" holds globs that point at the source data. check, status, "kotowari overview build" and "kotowari overview serve" read the files matched by these globs as overview source data. The key that turns the overview on and the rule that a hidden directory is read when a glob names it are decided in [The configuration keys are only those in the table, and a key not written takes the default value in the table](config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table) and [The glob "**" is read recursively, and hidden directories are entered only when named by a path component](config.md#the-glob--is-read-recursively-and-hidden-directories-are-entered-only-when-named-by-a-path-component).

When a source data file is not UTF-8, kotowari stops with a non-UTF-8 file as the reason. When the scan of "overview.files" meets a symbolic link with no target, it stops with an unreadable file as the reason.

### Rejected options

- Make it temporary data made from scratch at every approval. The cost and the drift of the look come back.
- Place it under the location of the specification documents. What is under the specification location falls under the rule that it is read as specification documents, and an exception for documents that are not specification would be needed.
- Place it in "docs/overview/". Data that people do not read would sit in the location of documents people read.

### Confirmations

- C-core-ovdata-1 check reads the files matched by the globs of "overview.files" as overview source data
- C-core-ovdata-18 If there is source data that is not UTF-8, check stops with exit code 2, and the first line of standard error starts with "non-UTF-8 file: " and that path
- C-core-ovdata-19 If the scan of "overview.files" meets a symbolic link with no target, check stops with exit code 2, and the first line of standard error starts with "unreadable file: "

## Without the overview key, overview build and overview serve stop with a configuration error

Status: implemented

### Problem

The overview is turned on only when the "overview" key is written in the configuration, and is not silently on by default ([The configuration keys are only those in the table, and a key not written takes the default value in the table](config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table)). When a command that builds the overview is run without the overview being turned on, it also stops instead of silently making something.

### Decision

When the configuration has no "overview" key, "kotowari overview build" and "kotowari overview serve" stop with a configuration error as the reason. check and status read no source data at all.

### Confirmations

- C-core-ovdata-2 If the configuration has no "overview" key, "kotowari overview build" and "kotowari overview serve" stop with a configuration error
- C-core-ovdata-17 If the configuration has no "overview" key, check does not read a malformed file in ".kotowari/overview/" and reports no finding pointing at it

## If the overview source data overlaps the location of guides or tests, it stops with a configuration error

Status: implemented

### Problem

Overview source data, guides and test files are all selected by globs, so they can match the same file. This is handled the same as the overlap of the guide and test locations ([When the guide and test locations overlap, it is a stop with a configuration error](guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error)).

If the way sides of a pair are counted were dropped, a side that matches the globs of "guides.files" but has no first-language side, and a side that does not match the globs but is read as a guide because it is in the same pair, would fall outside the overlap check. Such a side would be read both as source data and as a guide, and its markers would be counted twice silently. The reason for stopping on an overlap is not to allow this silently. If the order of the checks and the detail of the stop were dropped, which stop wording appears would be undecided. The reason for deciding the order of the checks and the wording of the detail is not on record.

### Decision

When one file matches both the globs of "overview.files" and the globs of "guides.files" or "tests.files", kotowari stops with a configuration error as the reason. This is the same for check, status, "kotowari overview build" and "kotowari overview serve". The markers of the source data are read once only, as overview source data.

The detail of the stop is the path, relative to the base directory, of the first of the overlapping files in the byte order of the path, followed by ": matched by both overview.files and guides.files" or ": matched by both overview.files and tests.files".

The overlap of the guide and test locations is checked before the overlap of the source data. For a file that matches all three locations, it stops on the overlap of guides and tests, and the detail ends with ": matched by both guides.files and tests.files".

The overlap with guides counts, besides the files matched by the globs of "guides.files", the sides of other languages read as guides. A side matched by the globs of "guides.files" is counted even if it has no first-language side. A side that does not match the globs but is read as a guide because it is in the same pair as a file the globs matched is also counted.

### Confirmations

- C-core-ovdata-3 If one file matches both "overview.files" and "guides.files", both check and "kotowari overview serve" stop with a configuration error
- C-core-ovdata-4 If one file matches both "overview.files" and "tests.files", both check and "kotowari overview build" stop with a configuration error
- C-core-ovdata-20 When the overlapping files are "tests/a.md" and "tests/b.md", the first line of standard error is "config error: tests/a.md: matched by both overview.files and tests.files", and for an overlap with guides it ends with ": matched by both overview.files and guides.files"
- C-core-ovdata-21 If one file matches all three of "overview.files", "guides.files" and "tests.files", both check and "kotowari overview build" stop with a configuration error whose detail ends with ": matched by both guides.files and tests.files"
- C-core-ovdata-22 With two or more languages, if only a side of another language that has no first-language side matches both "overview.files" and "guides.files", check, status and "kotowari overview build" stop with a configuration error

## Overview source data is written as frontmatter and a Markdown body, and parts are written in YAML inside fences marked view

Status: implemented

### Problem

The composition of an overview (the kinds of diagrams and tables used and how they are arranged) can vary per document, because what is best depends on the subject. On the other hand, making pages with a fixed rendering requires the form of the source data to be fixed. The conditions for how parts are written inside the source data were that they do not look broken when opened in other Markdown tools, and that the format of their contents can be checked thoroughly.

### Decision

Overview source data is written as frontmatter at the start and a Markdown body.

The frontmatter has one key only, "ir", which lists the paths of the documents that overview covers, relative to the base directory (what the list points at is decision documents, as decided in [The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents); the key's name is [to be renamed](#the-list-key-ir-of-the-overview-source-data-and-the-findings-overview_ir_missing-and-overview_ir_shared-are-renamed)). An unknown key is an error. The title is not put in the frontmatter; the "# " heading of the body is used.

A part is written in YAML inside a fenced code block whose info string is "view <kind>". Inside a fence, other Markdown tools see it as a plain code block. Markers are written, as in guides, in HTML comments next to section headings.

The check of the form as Markdown is done by kotowari-markdown-schema, and what does not fit the form is an overview_form_invalid error. The detail of overview_form_invalid is decided in [The detail of a finding is a short string that identifies that finding, decided per kind in a table](findings.md#the-detail-of-a-finding-is-a-short-string-that-identifies-that-finding-decided-per-kind-in-a-table).

### Confirmations

- C-core-ovdata-5 For source data whose frontmatter has a key other than "ir", check reports an overview_form_invalid error
- C-core-ovdata-6 For source data with no "# " heading for the title, check reports an overview_form_invalid error
- C-core-ovdata-7 check reads a fenced code block whose info string is "view" and a kind name as a part, and checks its kind and contents

## The contents of parts are checked by the kotowari side with view's part schemas

Status: implemented

### Problem

The kinds of parts were made a fixed list held by view ([The kinds of parts are a fixed list of eight held by view](../view/parts.md#the-kinds-of-parts-are-a-fixed-list-of-eight-held-by-view)). The shape of the contents of parts is held by view as the shape of its own input, in the part schemas ([The source of truth for the shape of parts is the part schemas that view embeds and publishes](../view/parts.md#the-source-of-truth-for-the-shape-of-parts-is-the-part-schemas-that-view-embeds-and-publishes)), but view does not check. Reading the source data and reporting errors is the job of the kotowari side.

kotowari-markdown-schema handles only the form of the source data as Markdown. The contents of a code block are not Markdown, so checking them is not part of kotowari-markdown-schema's responsibility.

### Decision

kotowari-markdown-schema checks the form of the source data as Markdown and takes out the code blocks of parts (info string and contents). The kotowari side reads the contents as YAML, checks them with the part schema that view publishes for the kind chosen by the info string, and turns the result into findings.

A part whose kind name is not among view's part kinds is an overview_part_unknown error. A part whose contents cannot be read as YAML, and a part that does not fit its part schema, are an overview_part_invalid error. The detail of overview_part_invalid is the kind of the part and the JSON Pointer of the place that did not fit, and does not include text made by the validation library. This is so that text that changes between versions is not put into the detail.

When the contents cannot be read as YAML, the line of overview_part_invalid is the line that the YAML reader returned as the position of the error, converted to a line of the source data file, and the detail is the kind name and "(yaml)". With only the opening line of the fence and "(yaml)", the place to fix cannot be found in a long part. When no position is returned, it is the opening line of the fence.

### Rejected options

- Add to kotowari-markdown-schema a general feature that checks the contents of code blocks as structure. The contents of a code block are not Markdown.

### Confirmations

- C-core-ovdata-8 A part of an unknown kind gets an overview_part_unknown error, and a part that does not fit its part schema gets an overview_part_invalid error
- C-core-ovdata-9 The detail of overview_part_invalid is the kind of the part and the JSON Pointer of the place that did not fit, and does not include text made by the validation library
- C-core-ovdata-10 The overview_part_invalid of a part that cannot be read as YAML points at the line of the YAML error converted to a line of the source data file, and its detail is the kind name and "(yaml)"

## The first part right after the title is the lead

Status: implemented

### Problem

A composition that puts the conclusion and summary at the start of the overview was the right one when forms were compared. Composition is free per document, but putting the conclusion and summary at the start is a rule that applies to every document, so it is enforced by machine.

### Decision

The first part right after the title of overview source data must be the part of conclusion and summary (lead). Otherwise, check reports an overview_lead_missing error.

### Confirmations

- C-core-ovdata-11 For source data whose first part right after the title is not lead, check reports an overview_lead_missing error

## One decision document is in the list of at most one piece of overview source data

Status: proposed

### Problem

When a brainstorm changes a decision document, the overview to fix is to be determined as one by machine.

### Decision

The list in the frontmatter of the source data (the key is now "ir") holds the decision documents that overview covers. One decision document is put in the list of at most one piece of overview source data. If it is in the lists of two or more pieces of source data, check reports an overview_ir_shared error. The case where an entry of the list points at a decision document that does not exist (overview_ir_missing) is decided in [The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents). The key and the names of the two findings are [to be renamed](#the-list-key-ir-of-the-overview-source-data-and-the-findings-overview_ir_missing-and-overview_ir_shared-are-renamed).

No check is made that reports a decision document that is in no overview's list.

### Confirmations

- C-core-ovdata-12 If one decision document is in the lists of two pieces of overview source data, check reports an overview_ir_shared error

## The list key ir of the overview source data and the findings overview_ir_missing and overview_ir_shared are renamed

Status: proposed

### Problem

After the switch, what the list of the overview source data points at is decision documents, and the IR goes away. The current key "ir" and the names of the finding kinds overview_ir_missing and overview_ir_shared remain words that point at the IR that goes away. No other reason for renaming them is on record.

### Decision

The list key "ir" in the frontmatter of the overview source data, the finding overview_ir_missing for when an entry of the list points at a decision document that does not exist, and the finding overview_ir_shared for when one decision document is in the lists of two or more pieces of source data, are renamed.

### Confirmations

- C-core-ovdata-23 The names of the list key of the overview source data, of the finding for pointing at a decision document that does not exist, and of the finding for a decision document in the lists of two pieces of source data are no longer "ir", overview_ir_missing and overview_ir_shared

## References inside parts are written in refs and ref fields

Status: implemented

### Problem

If which fields are references were decided per part kind, the way references are collected would change every time a part kind is added. Rather than draw with references unresolved, check is to stop and make them be fixed. Putting it in check also catches it in the checks before a commit.

### Decision

References inside the YAML of a part are written in fields named "refs" (a list of strings) or "ref" (one string). kotowari collects the fields with these names inside parts and reads them as references. What a reference can point at, and which references count as unresolvable, are decided in [The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents). check makes an unresolvable reference an overview_ref_unresolved error.

### Confirmations

- C-core-ovdata-13 check reads each entry of "refs" and the string of the "ref" field of a part as references, and reports an overview_ref_unresolved error for an unresolvable reference

## Markers in the overview source data are read by the same rules as guide markers

Status: implemented

### Problem

Which section of the overview copied which decision is held by markers, as in guides. After a decision changes, the sections not reviewed yet are to be found. Staleness of the chain of decisions is not made a finding; being visible on the page is enough. What triggers fixing an overview is the staleness of markers and the update at every brainstorm approval.

### Decision

check reads the markers inside the overview source data by the same rules as guide markers ([Guide markers](guides.md)) and matches them; a malformed one is an invalid_marker error and a stale one is a guide_stale notice. The "path" of the finding is that overview source data. The source data is not read a second time as a guide.

A section's marker is placed next to the "## " heading of that section. Passing whether a section is stale to rendering, from the staleness of its markers, is decided in [Sections whose markers are stale are passed to rendering as stale sections](overview-commands.md#sections-whose-markers-are-stale-are-passed-to-rendering-as-stale-sections).

### Confirmations

- C-core-ovdata-14 When one marker inside overview source data is stale, check reports a guide_stale notice with "path" set to that source data
- C-core-ovdata-15 For a malformed marker inside overview source data, check reports an invalid_marker error

## overview build and overview serve collect only the paths of test files and do not read their contents

Status: implemented

### Problem

The output of the overview does not use the results of test analysis. If writing stopped on a problem in an input it does not use, the user could not tell why it stopped. Problems on the test side are reported by check. Not reading them also makes writing faster (the time has not been measured).

### Decision

"kotowari overview build" and "kotowari overview serve" collect only the paths of test files, and read neither their contents nor "tests.rules". The paths are used for the check of [If the overview source data overlaps the location of guides or tests, it stops with a configuration error](#if-the-overview-source-data-overlaps-the-location-of-guides-or-tests-it-stops-with-a-configuration-error). The stops in the scan that collects the paths are kept the same as in check.

### Confirmations

- C-core-ovdata-16 Even if "tests.files" matches a file that is not UTF-8, "kotowari overview build" does not stop and writes the pages

## overview build and overview serve also read decision documents from the same location as check and stop under the same conditions

Status: implemented

### Problem

"kotowari overview build" and "kotowari overview serve" read decision documents to resolve the references of parts. If these stop conditions were dropped, build and serve could let through an error of the location that check would stop on (the location does not exist, cannot be read) and write anyway. The reason for making them the same as check is not on record.

### Decision

"kotowari overview build" and "kotowari overview serve" read decision documents from the same configuration and location as check. When the location of decision documents does not exist or cannot be read, they stop with the same reason and detail as check ([If the location of decision documents does not exist, is not a directory, or cannot be read, kotowari stops with unreadable file](config.md#if-the-location-of-decision-documents-does-not-exist-is-not-a-directory-or-cannot-be-read-kotowari-stops-with-unreadable-file)).

### Confirmations

- C-core-ovdata-24 If the location of decision documents does not exist, "kotowari overview build" stops with exit code 2, and the first line of standard error starts, as in check, with "unreadable file: " and the path of that location
