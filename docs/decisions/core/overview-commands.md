# Commands that build and show the overview

English | [日本語](overview-commands.ja.md)

Covers how "kotowari overview build" derives the reference table and the stale sections from the overview source data, passes them to the rendering engine (kotowari-markdown-view) and writes the files of the overview, and how "kotowari overview serve" shows them locally. The form and checks of the source data are decided in [Overview source data](overview-data.md), the options the two commands accept in [Commands and exit](cli.md), and the exception to where output may be written in [Users and construction](cli-scope.md), under [kotowari saves no state and writes nowhere other than standard output and standard error](cli-scope.md#kotowari-saves-no-state-and-writes-nowhere-other-than-standard-output-and-standard-error). The pages of languages other than the first language and the per-language bodies of references are decided in [Per-language overviews and UI strings](overview-languages.md), under [The overview is written for each language of the language list, from that language's side](overview-languages.md#the-overview-is-written-for-each-language-of-the-language-list-from-that-languages-side) and [On another language's page, the body of a reference is taken from the decision document on that page's language side](overview-languages.md#on-another-languages-page-the-body-of-a-reference-is-taken-from-the-decision-document-on-that-pages-language-side).

## overview build draws everything every time, writes only the files whose contents differ, and deletes files no longer produced

Status: implemented

### Problem

The overview has an index and a page per overview document. Putting everything in one file makes it hard to handle as it grows, and makes it impossible to output again only the files that changed. On the other hand, the index depends on every document, and a change to a referenced decision changes many pages, so the pages to output again cannot be determined correctly from only the source data that changed.

The drawn HTML can be rebuilt at any time from the source data by a fixed rendering. Committing it would mean managing it twice, and opening the HTML on GitHub does not render it. The pages are to be split, and the look is to be kept consistent in one place. The reason for making source data with overlapping names an error has not been kept.

If the scope of deletion were dropped, it would be undecided whether to delete a file under the location that was not made from source data. The reason for treating every file under the location as made by build is not on record.

### Decision

"kotowari overview build" draws all overview source data in memory every time, and rewrites only the files whose finished contents differ from the existing file. It deletes the files of pages that are no longer produced. It keeps no record of dependencies for skipping rendering, and so does without state.

Every file under ".kotowari/cache/overview/" is treated as made by build. A file that this build did not produce is deleted, even if it was not made from source data.

The place written to is ".kotowari/cache/overview/" of the base directory, and the drawn HTML is not committed. The index is "index.html", the page of each overview is "<topic>.html" with the same name as its source data, and the style is one shared "style.css" that every page refers to.

Source data whose file name without ".md" is the same as that of other source data, and source data whose name is "index" or "style", are made an overview_name_conflict error by check and status.

### Rejected options

- Commit the drawn HTML too. It can be rebuilt from the source data at any time, so it would be managed twice.
- Put the index and all pages into one HTML and output it to standard output. It becomes hard to handle as it grows, and only the changed files cannot be output again.
- Record the dependencies of each piece of source data and skip rendering. The pages to output again cannot be determined correctly.

### Confirmations

- C-core-ovcmd-1 The first "kotowari overview build" writes "index.html", a "<topic>.html" for each piece of source data, and "style.css" under ".kotowari/cache/overview/"
- C-core-ovcmd-2 The second build writes only the pages whose contents changed and does not rewrite the pages that did not change
- C-core-ovcmd-3 Running build after deleting a piece of source data deletes the file of that page
- C-core-ovcmd-24 For two pieces of source data with the same name and for source data named "index" or "style", check reports an overview_name_conflict error
- C-core-ovcmd-25 If there is a file under ".kotowari/cache/overview/" that was not made from source data ("stray.txt"), build deletes it and lists it in "removed"

## The overview commands write and delete nothing outside .kotowari/cache/overview

Status: implemented

### Problem

kotowari writes nothing besides standard output and standard error, but the commands that build the overview were made the only exception ([kotowari saves no state and writes nowhere other than standard output and standard error](cli-scope.md#kotowari-saves-no-state-and-writes-nowhere-other-than-standard-output-and-standard-error)). The place written to is to be fixed so that it does not change with the configuration, keeping the hole in the guarantee of writing nothing as small as possible.

### Decision

"kotowari overview build" and "kotowari overview serve" write and delete no file anywhere other than under ".kotowari/cache/overview/" of the base directory. The place written to cannot be changed by configuration.

### Confirmations

- C-core-ovcmd-4 "kotowari overview build" writes no file anywhere other than under ".kotowari/cache/overview/", and does not write outside by following a symbolic link inside the location either

## If the location or a directory above it is a symbolic link or not a directory, it writes nothing and stops with cache error

Status: implemented

### Problem

If the location of the overview, or a directory above it, is a symbolic link, writes and deletions reach the link's target, that is, outside the base directory. In a review, a file outside was actually deleted. A symbolic link can be committed to a cloned repository. If a failure to write to the location were stopped as an unreadable file, it would be mistaken for a failure to read (the reasons for a stop are decided in [The reason for a stop is shown with fixed English wording](cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording)).

### Decision

If any of ".kotowari", ".kotowari/cache" or ".kotowari/cache/overview" of the base directory is a symbolic link or a file of a kind other than a directory, "kotowari overview build" and "kotowari overview serve" write and delete no file, and stop with a location error (cache error) as the reason. The detail is that path.

When creating the location, enumerating the files under it, writing a file or deleting one fails, they also stop with a location error as the reason, and the detail is that path and the OS error text. Failures to create, write and delete in the location and failures to enumerate share the same reason.

### Confirmations

- C-core-ovcmd-5 If ".kotowari/cache/overview" or a directory above it is a symbolic link to an outside directory, build stops with cache error and does not change the contents of the outside directory
- C-core-ovcmd-6 If the location or a directory above it is a file that is not a directory, build stops with cache error and does not change that file
- C-core-ovcmd-7 When the files under the location cannot be enumerated and when a file cannot be written, build stops with cache error, and the detail shows that path and the OS error text

## If the source data or the table of contents has errors, it writes nothing and stops with overview error

Status: implemented

### Problem

Rather than output half-broken pages, it is to stop and have them fixed. check reports what the errors are, so build has no need to report the same thing twice.

If a side of the pair of the source data or the table of contents is missing, or the skeletons disagree, another language's page cannot be drawn from that language's side. If this stop were dropped, the pages could be written out anyway. If the order of checking, acquiring the port and writing in serve were dropped, a port error could come first for source data with errors, or the location could be written even though the port cannot be used. The reason for fixing the order is not on record.

### Decision

When the overview source data or the table of contents has one or more errors that check reports, "kotowari overview build" and "kotowari overview serve" write and delete no file, and stop with a source data error (overview error) as the reason. The detail is the number of errors and a pointer saying that "kotowari check" shows them in detail (in the form "overview error: 3 errors in overview data; run kotowari check"). build does not report what the errors are.

When a pair of the overview source data or the table of contents has a translation_missing or translation_structure_mismatch error, they likewise write nothing and stop with overview error.

"kotowari overview serve" checks the source data, acquires the port and writes, in that order. If the source data has errors, it stops before acquiring the port. If it cannot acquire the port, it stops without writing anything to the location.

What the errors of the table of contents are, and that errors of the table of contents also stop it, are decided in [The overview table of contents](overview-toc.md), under [Each overview page is placed exactly once in the table of contents](overview-toc.md#each-overview-page-is-placed-exactly-once-in-the-table-of-contents). The reason and detail of the stop follow this decision.

### Confirmations

- C-core-ovcmd-8 Running "kotowari overview build" with source data that has errors stops with exit code 2 and the first line of standard error "overview error: <count> errors in overview data; run kotowari check", and the list and contents of the files in the location are the same as before the run
- C-core-ovcmd-9 Running "kotowari overview serve" with source data that has errors stops with overview error, as build does
- C-core-ovcmd-26 With two or more languages, if the skeletons of the sides of the pair of the table of contents disagree, "kotowari overview build" stops with exit code 2 and does not create the location
- C-core-ovcmd-27 With source data that has errors, when the given port is used by another process, serve stops with overview error, not port error
- C-core-ovcmd-28 When serve stops because the port cannot be used, ".kotowari/cache/" is not created

## overview build reports the files written, the files deleted and the number of files not written

Status: implemented

### Problem

The LLM that ran build is to be able to confirm what changed.

### Decision

When "kotowari overview build" has finished writing, it reports, following "--format" (json by default, text otherwise), the files rewritten (written), the files deleted (removed) and the number of files that did not change (unchanged). Paths are relative to the base directory. json is a JSON with the keys "written", "removed" and "unchanged", and text puts one file on one line.

### Confirmations

- C-core-ovcmd-11 "kotowari overview build --format json" outputs a JSON with the keys "written", "removed" and "unchanged", and the paths are relative to the base directory
- C-core-ovcmd-12 With "--format text", the files written and the files deleted are output one per line

## The reference table holds, for each reference, the body of what it points at

Status: proposed

### Problem

The rendering engine does not know kotowari and does not read decision documents. To draw references, the kotowari side has to read the decision documents, make a mapping from the reference strings to display name, body and state (the reference table), and pass it to the engine. Also, kotowari does not know where the repository is published, and serve serves only what is under the location of the overview, so a reference on the page cannot link out to its target outside the page. Being able to see the body of the target is enough (how it is drawn: [References do not link outside the page; selecting one opens its body in place](../view/rendering.md#references-do-not-link-outside-the-page-selecting-one-opens-its-body-in-place)).

The state of a reference exists so that what is shown, including planned items, can be told apart on the page. In the new form, an overturned decision is deleted and moved to the rejected options, so there is nothing for "superseded" to come from, and there is no deferred declaration either.

### Decision

For each reference in the parts of all overview source data, "kotowari overview build" makes one entry of the reference table with display name, body and state, and passes it to the rendering engine. The target is a decision of a decision document or a confirmation row ([The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents)), and the body holds the contents of the target. The engine only looks up the table and draws.

The state in the reference table is set from the status of the target decision. A reference that points at a decision whose status is "proposed", or at one of its confirmation rows, is "deferred", and one that points at an "implemented" one is "current". No reference with the state "superseded" is made.

### Confirmations

- C-core-ovcmd-13 Each reference of a part puts one entry into the reference table, and the same reference appearing several times makes one entry
- C-core-ovcmd-14 When a reference of a part points at a decision's heading or a confirmation ID, the body of that entry of the reference table holds the contents of the target
- C-core-ovcmd-29 A reference that points at a decision whose status is "proposed" or at its confirmation rows has the deferred state in the reference table, a reference that points at an "implemented" decision or its rows has the current state, and there is no reference with the superseded state

## Sections whose markers are stale are passed to rendering as stale sections

Status: implemented

### Problem

The reader is to be able to tell apart the parts of the overview's sections that have not been reviewed yet after the decision they came from changed. The rendering engine does not know decision documents, so the kotowari side computes whether a section is stale and passes it. The processing is done efficiently. Fixing them is done not by the implementation loop but by the next brainstorm that touches that topic ([The process that writes the overview](overview-workflow.md), under [cycle and implementation do not fix the overview source data](overview-workflow.md#cycle-and-implementation-do-not-fix-the-overview-source-data)).

### Decision

For each section under a "## " heading of the overview source data, "kotowari overview build" passes that section to the rendering engine as a stale section if the section contains one or more markers that receive a guide_stale notice. When a marker becomes stale is decided in [The overview's source data points at decision documents](languages-and-guides.md#the-overviews-source-data-points-at-decision-documents).

### Confirmations

- C-core-ovcmd-15 Only sections that contain a marker receiving guide_stale are passed to the rendering engine as stale sections, and a section with only markers of the current fingerprint and a section with no marker are not

## overview serve writes as build does when it starts, and serves the files of the location on one port of 127.0.0.1

Status: implemented

### Problem

For a person to look at the built HTML, a tedious procedure is needed. A command that shows it is wanted as a pair with build. The effort of viewing is to be removed with the smallest construction. A rebuild can be reflected by restarting serve or by a separate build. serve is an entry point for viewing, and the only thing it needs to output is the URL.

### Decision

When it starts, "kotowari overview serve" performs the same rendering and writing as "kotowari overview build" once, and serves the files under ".kotowari/cache/overview/" over HTTP on one port of "127.0.0.1". It is not exposed to the outside network. The default port is "4590", and "--port" changes it.

Once started, it outputs only the one line of the URL "http://127.0.0.1:<port>/" to standard output, and keeps serving until an interrupt (Ctrl-C). It outputs no log per request.

### Confirmations

- C-core-ovcmd-16 Starting "kotowari overview serve --port 4591" makes standard output only the one line "http://127.0.0.1:4591/", and "/" and "/style.css" return the contents of the files in the location
- C-core-ovcmd-17 Without "--port", serve serves on port "4590"
- C-core-ovcmd-18 An interrupt (Ctrl-C) ends serve

## If the port of overview serve cannot be used, it stops with port error without trying other ports

Status: implemented

### Problem

If it silently escaped to another port when the port is in use, the URL shown and the reality would diverge. Also, the HTTP crate in use does not resume accepting once accepting a connection has failed, so it would stay alive serving nothing, and keeping on serving could not be kept. Rather than freezing silently, it is to stop and say so.

### Decision

When the given port of "127.0.0.1" cannot be used, "kotowari overview serve" stops with a port error (port error) as the reason without trying other ports (exit code 2). The detail is "127.0.0.1:<port>" and the OS error text. When accepting a connection fails while serving, it likewise stops with a port error as the reason.

### Rejected options

- When accepting fails, rebuild the server and continue.

### Confirmations

- C-core-ovcmd-19 Starting serve on a port another process is using stops it with exit code 2 and the first line of standard error starting with "port error: 127.0.0.1:<port>"
- C-core-ovcmd-20 When accepting a connection fails while serving, serve stops with port error

## overview serve returns neither files outside the location nor directory listings

Status: implemented

### Problem

serve is a server for viewing the overview locally, and lets nothing outside the location of the overview be read. Reading requests and checking request paths so that files outside the location cannot be read concern safety, so they are not built in-house and are left to a proven crate.

### Decision

"kotowari overview serve" serves only the files under ".kotowari/cache/overview/". For a request path that points outside it (a ".." component, an absolute path, or one whose target after following symbolic links is outside), it does not return the contents of the outside file and returns 404. It does not return directory listings. For the HTTP server, it uses one small HTTP server crate.

### Rejected options

- Build a simple server in-house with only the standard library. Reading requests and checking paths concern safety.

### Confirmations

- C-core-ovcmd-21 Requests for "/../config.yaml" and "/%2e%2e/config.yaml" return 404, and the contents of files outside the location are not returned
- C-core-ovcmd-22 Requests for a symbolic link pointing outside the location, a directory, or an absolute path return 404

## overview serve answers only requests with a local Host

Status: implemented

### Problem

serve is to let no request through that arrived under the name of an outside site. The URL it outputs is "127.0.0.1", but some people type "localhost" by hand. Both are local names, and not letting the names of outside sites through is enough for the purpose.

### Decision

"kotowari overview serve" answers only requests whose Host header value is "127.0.0.1:<port>" or "localhost:<port>" (the port being the one it listens on, regardless of the letter case of the name). To any other request (including one with no Host header), it returns 403 with no body before checking whether the file exists, and outputs nothing per request. With the same response, what files the location has does not leak outside either.

### Rejected options

- Answer only "127.0.0.1:<port>". Typing localhost by hand would not open it.
- Return 400 when there is no Host header and 403 when it differs. It only adds one more rule and protects nothing more.

### Confirmations

- C-core-ovcmd-23 Requests whose Host header is "127.0.0.1:<port>" and "LOCALHOST:<port>" return the contents of "index.html", and requests with another name, another port, or no Host header return 403 with no body
