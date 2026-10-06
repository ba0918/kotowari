# Commands that build and show the overview

English | [日本語](overview-commands.ja.md)

Covers how "kotowari overview build" derives the `reference table` and the stale sections from the `overview data`, passes them to the rendering engine (kotowari-markdown-view) and writes the files of the `overview`, and how "kotowari overview serve" shows them locally.

## Requirements

### REQ-core-291: Building the reference table

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A66, docs/decision/records/2026-10-02-whole-picture.md#A67, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-05-localization.md#A34
- definition: TBL-core-039
- verification: unit

kotowari always, on "kotowari overview build", makes, for each reference (REQ-core-285) in each `part` of all `overview data`, one entry of the `reference table` with the display name, body and state of TBL-core-039, and passes it to the rendering engine. The body for the page of each language is taken as REQ-core-354 says.

### REQ-core-292: Finding stale sections

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A39, docs/decision/records/2026-10-02-whole-picture.md#A68, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

kotowari always, on "kotowari overview build", passes to the rendering engine as stale sections those sections under a "## " heading of the `overview data` that contain one or more `guide mark` entries receiving a guide_stale `notice`. A `guide mark` before the first "## " heading belongs to no section and makes no stale section. HTML comments are removed from the heading text passed to the rendering engine.

### REQ-core-293: Writing the overview

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A16, docs/decision/records/2026-10-02-whole-picture.md#A26, docs/decision/records/2026-10-02-whole-picture.md#A29, docs/decision/records/2026-10-02-whole-picture.md#A56, docs/decision/records/2026-10-05-localization.md#A20
- verification: unit

kotowari always, on "kotowari overview build", renders all `overview data` every time, writes only those files of the `overview` returned by the rendering engine for which there is no file with the same name and the same bytes under ".kotowari/cache/overview/" of the `base directory`, and deletes the files under that location that were not returned this time. The file names are "index.html" for the index, for each `overview` the file name of the `overview data` of the `side` in the `first language` with ".md" replaced by ".html", and "style.css" for the style; the pages of a language other than the `first language` are written under "<tag>/" with the same names (REQ-core-353).

### REQ-core-294: Nothing is written when the data has errors

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A33, docs/decision/records/2026-10-02-whole-picture.md#A61, docs/decision/records/2026-10-02-whole-picture.md#A63, docs/decision/records/2026-10-02-whole-picture.md#A79, docs/decision/records/2026-10-05-overview-index.md#A12, docs/decision/records/2026-10-05-localization.md#A17, docs/decision/records/2026-10-05-localization.md#D2
- verification: unit

When, on "kotowari overview build" or "kotowari overview serve", the `overview data` or the `table of contents` has one or more `error` entries of REQ-core-278 to REQ-core-286, REQ-core-305 or REQ-core-327 to REQ-core-330, or a `pair` of the `IR`, the `overview data` or the `table of contents` has one or more translation_missing or translation_structure_mismatch `error` entries (REQ-core-338, REQ-core-345), kotowari writes no file and deletes none, makes a `stop` with an overview data error as the reason, and sets the detail to the number of `error` entries and "errors in overview data; run kotowari check".

### REQ-core-295: The output of build

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A62, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A78
- verification: unit

kotowari always, when "kotowari overview build" has finished writing, outputs, if "--format" is "json" (the default), one JSON with only the three keys "written" (the list of paths of the files written), "removed" (the list of paths of the files deleted) and "unchanged" (the number of files not written), and if "text", one line "written <path>" per file written and one line "removed <path>" per file deleted, outputting all written lines before the removed lines. Paths are relative to the `base directory`, and the entries of a list and the lines of the same kind are ordered by the bytes of the path.

### REQ-core-296: Writing only to the location of the overview

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A28
- verification: unit

kotowari shall not, on "kotowari overview build" and "kotowari overview serve", write or delete files anywhere other than under ".kotowari/cache/overview/" of the `base directory`. It shall not let the configuration change where it writes.

### REQ-core-297: Showing with serve

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A27, docs/decision/records/2026-10-02-whole-picture.md#A30, docs/decision/records/2026-10-02-whole-picture.md#A57, docs/decision/records/2026-10-02-whole-picture.md#A63, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A79
- verification: unit

kotowari always, on "kotowari overview serve", checks the `overview data`, acquires the port given by the value of "--port" (default "4590") on "127.0.0.1", and renders and writes the same way as "kotowari overview build", in that order; it then serves the files under ".kotowari/cache/overview/" over HTTP, outputs only the one line "http://127.0.0.1:<port>/" to standard output, keeps serving until an interrupt (Ctrl-C), and on the interrupt ends with exit code 0. It outputs nothing per request. It returns ".html" with the Content-Type "text/html; charset=utf-8" and ".css" with "text/css; charset=utf-8", and returns 404 for a request path to a file not in that location.

### REQ-core-298: When the port cannot be used

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A57, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-04-overview-on-public-api.md#A14
- verification: unit

When, on "kotowari overview serve", the given port on "127.0.0.1" cannot be used, kotowari does not try other ports, makes a `stop` with a port error as the reason (neither an unreadable file nor an argument error), and sets the detail to "127.0.0.1:<port>" and the OS error text. When accepting a connection fails while serving, it likewise makes a `stop` with a port error as the reason.

### REQ-core-299: What serve serves

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A59, docs/decision/records/2026-10-02-whole-picture.md#A72
- verification: unit

kotowari shall not, on "kotowari overview serve", return the content of a file outside ".kotowari/cache/overview/" for a request path that points outside that location (a ".." component, an absolute path, or one whose target after following symbolic links is outside); it returns 404. It shall not return a directory listing. When the path is "/", it returns "index.html".

### REQ-core-356: Hosts that serve answers

- kind: event_driven
- source: docs/decision/records/2026-10-05-serve-host.md#A1, docs/decision/records/2026-10-05-serve-host.md#A2
- verification: unit

When, on "kotowari overview serve", the value of the Host header of a request is neither "127.0.0.1:<port>" nor "localhost:<port>" with the port being served (the letter case of the name does not matter), or the request has no Host header, kotowari returns 403 with an empty body before it looks up the request path, and outputs nothing for that request.

### REQ-core-305: Overlapping page names

- kind: event_driven
- source: docs/decision/records/2026-10-02-whole-picture.md#A56, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A74, docs/decision/records/2026-10-05-localization.md#A31
- verification: unit

When the file name of the `overview data` of the `side` in the `first language`, with ".md" removed, is the same as that of other `overview data` of the `side` in the `first language`, or is "index" or "style", kotowari, on "kotowari check" and "kotowari status", outputs an overview_name_conflict `error` with "line" null and detail that name on the second and later overlapping `overview data` in the byte order of the path, and on the `overview data` named "index" or "style".

### REQ-core-324: When the location cannot be used

- kind: event_driven
- source: docs/decision/records/2026-10-04-overview-on-public-api.md#A9, docs/decision/records/2026-10-06-todo-zero.md#A5
- verification: unit

When, on "kotowari overview build" or "kotowari overview serve", any of ".kotowari", ".kotowari/cache" or ".kotowari/cache/overview" of the `base directory` is a symbolic link, or is neither a directory nor absent, kotowari writes no file and deletes none, makes a `stop` with a location error as the reason, and sets the detail to that path. When creating that location, enumerating the files under it, writing a file or deleting one fails, it likewise makes a `stop` with a location error as the reason, and sets the detail to that path and the OS error text.

## Decision tables

### TBL-core-039: One entry of the reference table

- source: docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A67, docs/decision/records/2026-10-02-whole-picture.md#A72, docs/decision/records/2026-10-02-whole-picture.md#A73, docs/decision/records/2026-10-02-whole-picture.md#A81, docs/decision/records/2026-10-02-whole-picture.md#A85, docs/decision/records/2026-10-02-whole-picture.md#A86

| Form of the reference | Display name | Body | State |
|---|---|---|---|
| the `ID` of a `requirement` of the `IR` | `ID` | the sequence of each `statement` of that `requirement` (for algorithm, the name and the value of "- definition:") | "deferred" if under `deferral`, otherwise "current" |
| the `ID` of a `decision table` or a `property` of the `IR` | `ID` | for a `decision table` the name, for a `property` the sequence of each `statement` | "current" |
| the `ID` of a `scenario` | `ID` | the sequence of the "Scenario:" line and the step lines | "deferred" for a `deferred scenario`, otherwise "current" |
| the form of a `source` pointing at a decision of a `decision record` | the file name with the leading "YYYY-MM-DD-" and the trailing ".md" removed, one space, and the `decision number` | the text of that `numbered line` after the `decision number` | "superseded" if the decision has a "- superseded_by:" line with a non-empty value, otherwise "current" |
| the `ID` of an `item` of the `flag record` | `ID` | the sequence of the body lines of that `item` | "current" |
| the form of a `source` pointing at a heading of a Markdown file that is not a decision record (an ADR, a form contract, a supplementary document) (4 of TBL-core-012) | the file name with the trailing ".md" removed, one space, and the heading text | the sequence of the sentences of that heading's section | "current" |

## Examples

```gherkin
@id=EX-core-474 @about=REQ-core-291 @source=docs/decision/records/2026-10-02-whole-picture.md#A65,docs/decision/records/2026-10-02-whole-picture.md#A67,docs/decision/records/2026-10-02-whole-picture.md#A72,docs/decision/records/2026-10-02-whole-picture.md#A81,docs/decision/records/2026-10-02-whole-picture.md#A85,docs/decision/records/2026-10-02-whole-picture.md#A66,docs/decision/records/2026-10-02-whole-picture.md#A86
Scenario: The states of a superseded decision and a deferred requirement go into the reference table
  Given the "refs" of a `part` has the decision "docs/decision/records/2026-01-01-x.md#A1", which has a "- superseded_by:" with a non-empty value, and the `requirement` "REQ-core-900" under `deferral`
  When "kotowari overview build" is run
  Then in the `reference table` passed to the rendering engine, the former has display name "x A1" and state "superseded", and the latter has display name "REQ-core-900" and state "deferred"

@id=EX-core-475 @about=REQ-core-293,REQ-core-295 @source=docs/decision/records/2026-10-02-whole-picture.md#A29,docs/decision/records/2026-10-02-whole-picture.md#A62,docs/decision/records/2026-10-02-whole-picture.md#A56,docs/decision/records/2026-10-02-whole-picture.md#A28,docs/decision/records/2026-10-02-whole-picture.md#A42,docs/decision/records/2026-10-02-whole-picture.md#A82
Scenario: The second build writes only the changed files
  Given after one build with the two `overview data` files "a.md" and "b.md", only a statement of "b.md" was changed
  When "kotowari overview build --format json" is run
  Then "written" is only ".kotowari/cache/overview/b.html", "removed" is empty, "unchanged" is 3, and the modification time of "a.html" does not change

@id=EX-core-476 @about=REQ-core-293 @source=docs/decision/records/2026-10-02-whole-picture.md#A29,docs/decision/records/2026-10-02-whole-picture.md#A56,docs/decision/records/2026-10-02-whole-picture.md#A62
Scenario: The page of data that no longer exists is deleted
  Given after a build with "a.md" and "b.md", "b.md" was deleted
  When "kotowari overview build --format json" is run
  Then "removed" is ".kotowari/cache/overview/b.html", and that file does not exist

@id=EX-core-477 @about=REQ-core-294,REQ-core-296 @source=docs/decision/records/2026-10-02-whole-picture.md#A33,docs/decision/records/2026-10-02-whole-picture.md#A61,docs/decision/records/2026-10-02-whole-picture.md#A38
Scenario: When the data has errors it stops without writing anything
  Given the only `error` is in `overview data` that has a "## " heading right after the `title` and no lead `part`, and ".kotowari/cache/overview/" has files from a previous build
  When "kotowari overview build" is run
  Then the exit code is 2, the first line of standard error is "overview error: 1 errors in overview data; run kotowari check", and the list and content of the files in the location are the same as before the run

@id=EX-core-478 @about=REQ-core-297,REQ-core-299 @source=docs/decision/records/2026-10-02-whole-picture.md#A30,docs/decision/records/2026-10-02-whole-picture.md#A59,docs/decision/records/2026-10-02-whole-picture.md#A63,docs/decision/records/2026-10-02-whole-picture.md#A72,docs/decision/records/2026-10-02-whole-picture.md#A56
Scenario: serve serves only what is in the location
  Given there is correct `overview data`, and port "4591" is free
  When "kotowari overview serve --port 4591" is started, and "/", "/style.css", "/../config.yaml" and "/%2e%2e/config.yaml" are requested
  Then standard output is the one line "http://127.0.0.1:4591/", "/" returns the content of "index.html", "/style.css" returns its content, and the remaining two return 404

@id=EX-core-543 @about=REQ-core-356 @source=docs/decision/records/2026-10-05-serve-host.md#A1,docs/decision/records/2026-10-05-serve-host.md#A2
Scenario: serve answers only local hosts
  Given there is correct `overview data`, and port "4593" is free
  When "kotowari overview serve --port 4593" is started, and "/" is requested with the Host header "127.0.0.1:4593", "LOCALHOST:4593", "evil.example:4593", "127.0.0.1:4594" and with no Host header
  Then the first two return the content of "index.html", and the other three return 403 with an empty body

@id=EX-core-479 @about=REQ-core-298 @source=docs/decision/records/2026-10-02-whole-picture.md#A57,docs/decision/records/2026-10-02-whole-picture.md#A72
Scenario: It stops on a port in use
  Given another process is using "127.0.0.1:4592"
  When "kotowari overview serve --port 4592" is run
  Then the exit code is 2, and the first line of standard error starts with "port error: 127.0.0.1:4592"

@id=EX-core-504 @about=REQ-core-324,REQ-core-296 @source=docs/decision/records/2026-10-04-overview-on-public-api.md#A9
Scenario: When the location is a symbolic link it stops without writing outside
  Given there is correct `overview data`, and ".kotowari/cache/overview" is a symbolic link to a directory outside the `base directory` that has the file "keep"
  When "kotowari overview build" is run
  Then the exit code is 2, the first line of standard error starts with "cache error: .kotowari/cache/overview", and the content of the outside directory is only "keep", unchanged

@id=EX-core-546 @about=REQ-core-324 @source=docs/decision/records/2026-10-06-todo-zero.md#A5,docs/decision/records/2026-10-04-overview-on-public-api.md#A9,docs/decision/records/records.md#A20,docs/decision/records/ir-form.md#出力
Scenario: A location whose files cannot be enumerated stops with a location error
  Given a directory under ".kotowari/cache/overview" cannot be read
  When "kotowari overview build" is run
  Then the exit code is 2 and the first line of standard error starts with "cache error: .kotowari/cache/overview"
```
