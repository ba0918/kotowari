Based on the kotowari specification (revised 2026-10-05; the version of kotowari itself is not pinned)

An overview is a set of rendered pages, one per product topic, that lets the people who build the product — the person who brainstormed, the implementer and the reviewer — see the whole picture: what has been decided and why, how the decisions lead to one another, and how the product behaves as their result, as it stands now, including what is planned or deferred. It exists because a brainstorm settles decisions one question at a time, and with tens or hundreds of decisions nobody can say afterwards what was built and why. People read only the rendered pages. The overview data behind them is written by an LLM in a fixed form, is never read by people, and is kept in the repository and revised at each brainstorm approval. kotowari checks the data and renders it; it never writes the data.

## What an overview says, and what a guide says

An overview is read by the people who build the product; a guide (guides.md) is read by the people who use it. Both may cover the same feature, so write each for its own question and do not keep the same explanation in both.

- An overview says what has been decided and why, what is planned, deferred or undecided, and how the decisions lead to one another. It shows how the product behaves only briefly, as the result of those decisions.
- An overview does not say how to use the product: no commands, options, configuration examples or step-by-step usage. It names the feature; the guide shows how to use it.
- The boundary, by example: a flow part in which the implementer writes its change record, the reviewer writes its record and `kotowari changes` passes belongs in the overview; the command `kotowari changes --base <id> --head <id> --phase review` and its options belong in a guide.

## Where overview data is

Overview data is enabled by the `overview` key of the configuration, whose `files` is a required list of globs and whose `toc` is the required path of the table of contents (below):

```yaml
overview:
  files:
    - ".kotowari/overview/*.md"
  toc: ".kotowari/overview-toc.yaml"
```

Without the key nothing here applies: check and status read no overview data, and `kotowari overview build` and `serve` stop with `config error: overview is not configured`. Only files with the lowercase extension `.md` are read. The globs are read and walked like `guides.files`, except that a hidden directory named by a component of a glob (`.kotowari` above) is entered, as with `changes.records`; a broad `**` alone does not enter hidden directories. A file matched by both `overview.files` and `guides.files` or `tests.files` stops check, status, build and serve with a config error whose detail is that file followed by `: matched by both overview.files and guides.files` (or `tests.files`). Overview data is never read as a guide as well. After that, a table of contents that is one of the files the `overview.files`, `guides.files` or `tests.files` walks read (for `overview.files`, only `.md` files are read) stops them with a config error whose detail is its path followed by `: matched by both overview.toc and ` and the first of those keys in that order; keep it out of those globs, for example as a `.yaml` file. A missing or unreadable table of contents stops them as an unreadable file, and one that is not UTF-8 as a non-UTF-8 file.

`kotowari check` and `kotowari status` read overview data; `kotowari list` and `kotowari query` do not. The findings join check's findings, and the JSON of check and status always has an `overview` group with `files` (the overview data files read) and `marks` (the entries of well-formed guide marks in them), both 0 without the key.

## The unit of an overview

- One overview per product topic, a feature the user calls by one name (change records, guide marks, adoption, mutation tests). Something that cuts across features, such as the output format or the configuration, is not an overview of its own; mention it in the overview of each feature it touches.
- The frontmatter `ir` of each overview lists the topic documents of the IR it covers. One topic document belongs to at most one overview, so changing a topic document in a brainstorm decides mechanically which overview to revise.
- When a brainstorm changes a topic document that no overview's `ir` lists, propose either adding it to an existing overview or starting a new one, and let the person decide. The first time overviews are made, propose the units and let the person decide.
- Once decided, keep the units. Split or merge overviews only when a brainstorm decides to.
- Do not make overviews for topics the brainstorm did not touch. Existing topics get an overview the first time a brainstorm touches their IR, not all at once.
- Add the `overview` key to the configuration only together with the first overview data and its contents file. With the key and no overview data, no contents file passes check: a missing file stops, an empty outermost group is `overview_toc_group_empty`, and any name is `overview_toc_page_unknown`.

## The table of contents

The index page is drawn from the table of contents: one YAML file that declares how the pages are grouped and ordered. It is data for the index, not a document people read. The outermost level is itself a group, whose title is the heading of the index:

```yaml
title: kotowari
note: What the product is, one line
items:
  - workflow
  - title: Tests
    note: How tests are tied to the specification
    items:
      - marks
      - title: Mutation tests
        items:
          - mutants
          - equivalents
  - overview
```

| Part | Form |
|---|---|
| Group | A mapping with only `title` (required, a non-empty string), `note` (optional, one line without a line break) and `items` (required, a list of page names and groups; an empty list is the error overview_toc_group_empty, not a form error) |
| Page name | The name of an overview data file: its file name without `.md`, as a non-empty string |

- Every page is in the table of contents exactly once. A page missing from it, a name with no overview data, a name written twice and a group without items are errors.
- Within a level, put the pages and groups in the order a reader needs them: the order of use, or the order of the workflow. The order is shown as written; nothing is sorted.
- Nesting has no depth limit. Start with one level, and split a group into smaller groups only once it has grown too large to scan; a small product can stay at one level.
- Placing a new page, and adding or splitting a group, is decided at the brainstorm approval by the LLM writing the overview, without asking the person. The table of contents is not part of what the person approves; if a placement looks wrong when they see it, they say so and it is moved.

The index shows each group with its page count and the totals of sections not yet reviewed and of `open` statements; each page card shows its own counts of sections not yet reviewed and of `open` and `planned` statements. Each page shows its place in the table of contents above its title and links to the other pages of its group after its last section.

## The form of overview data

````markdown
---
ir:
  - docs/ir/core/changes.md
  - docs/ir/core/change-records.md
---

# Change records

```view lead
conclusion: One sentence that says what this topic is now
points:
  - What the user does with it
  - What is decided but not built yet
```

## How a change is reconciled
<!-- @kotowari[REQ-core-240:1a2b3c4d, EX-core-430:5e6f7a8b] -->

Markdown text: paragraphs, lists, numbered lists, tables, code blocks and `### ` headings.

```view flow
columns:
  - - title: Implementer record
      body: written before review
  - - title: Reviewer record
      tone: accent
```
````

| Part | Form |
|---|---|
| Frontmatter | YAML at the top with exactly one key, `ir`: a list of one or more paths of topic documents, relative to the base directory |
| Title | Exactly one `# ` heading |
| After the title | Before the first `## ` heading, only parts and HTML comments. The first block that is neither a comment nor a blank line must be a `lead` part. Parts after the lead are drawn after it, in order, before the first section |
| Sections | `## ` headings, with any Markdown text below: paragraphs, bullet lists (nested too), numbered lists, tables, code blocks, HTML comments and `### ` headings. Raw HTML is shown as text and HTML comments are not shown |
| Parts | A fenced code block whose info string is `view`, one or more spaces and the kind, holding YAML |

The kinds of part are fixed; a new kind is a change to the specification, decided in a brainstorm. `"width": "half"` in any part puts it beside the next half-width part of the same section; any other width is invalid.

| Kind | Draws | Fields |
|---|---|---|
| lead | The conclusion and its points, first in the document | `conclusion`, `points` |
| flow | Columns of boxes flowing from left to right, laid out on a grid | `columns` (a list of columns; a column is a list of boxes `title`, `body`, `tone`) |
| steps | A numbered sequence of stages | `items` (`title`, `body`, `refs`) |
| cards | Cards with a heading and a list of items | `cards` (`title`, `items`, `tone`) |
| status | Statements labelled `decided`, `planned`, `open` or `dropped`; the page shows each label in the UI text of its language | `items` (`state`, `text`, `refs`) |
| compare | Before, after and why; the before struck through | `items` (`before`, `after`, `why`, `refs`) |
| decisions | A tree of decisions with who decided each | `roots` (`ref`, `text`, `by`, `children`) |
| quiz | Questions whose answers open when chosen | `items` (`q`, `a`, `refs`) |

`tone` is one of `accent`, `good`, `warn`, `bad` and `muted`. The exact shape of each kind is its JSON Schema, which every object closes: an unknown key is an error.

## References

Put references only in fields named `refs` (a list of strings) or `ref` (one string), at any depth of a part. A reference is either an ID of an item or scenario in the IR (`REQ-core-240`, `EX-core-430`, `FLAG-core-001`), or a source in the same form as `- source:` (`docs/decision/records/2026-10-01-change-conformance.md#A2`, or a heading of an ADR or another Markdown file in the place for records). On the page a reference shows its label and, when chosen, opens its body in place: the statements of a requirement, the `Scenario:` line and steps of a scenario, or the text of the decision line. A decision with a non-empty `- superseded_by:` line is marked superseded, and a deferred requirement or deferred scenario is marked deferred. No reference links out of the page.

## Scene: writing overview data

Write it at the brainstorm approval, in the steps of kotowari-brainstorm, only in a project whose configuration has the `overview` key.

1. Find the overviews whose `ir` lists a topic document this brainstorm changed. For a changed topic document no overview lists, propose adding it to an existing overview or starting a new one, and let the person decide (the unit rules above).
2. Revise the previous overview data; do not write it again from nothing. Start from the file as it is, keep what still holds, and change what the brainstorm changed. A new overview starts from an empty file.
3. Put a `lead` part right after the title: the conclusion first, then its points.
4. Choose the parts and their order to suit the subject; there is no fixed layout. Use figures, tables and decorations rather than paragraphs of explanation. Show both how the product behaves as the result of the decisions, briefly (flow, steps, cards, compare), and how the decisions lead to one another (decisions). Leave usage to the guides (above).
5. Show the current state together with what is planned, deferred or undecided (status), so the page stays true right after the brainstorm.
6. Put references to the IR items and decisions each part rests on in its `refs` and `ref` fields.
7. Give each `## ` section a guide mark on the line right after its heading, on a line of its own, with an entry per IR item the section explains and the fingerprint from `kotowari query ID | jq -r '.items[0].fingerprint'` (guides.md).
8. Revise the table of contents: place each new overview where a reader would look for it, keeping the order of each level meaningful, and split a group only once it has grown large. Do not ask the person.
9. Run `kotowari overview build` and fix every error until it exits 0; the errors are listed by `kotowari check`.

## Commands

`kotowari overview build` checks the overview data, renders every page in memory, and writes under `.kotowari/cache/overview/` of the base directory: `index.html` (every overview's title and conclusion, nested and ordered as the table of contents says), `<name>.html` per overview data file (its file name with `.html` for `.md`) and `style.css`. Only pages whose bytes changed are written, and files no longer produced are removed. Nothing else is written, and the place cannot be changed. It takes `--format` (`json`, the default, or `text`) and `--config`. The JSON has exactly `written` and `removed` (paths relative to the base directory, in byte order) and `unchanged` (the number of pages not rewritten); text writes one `written <path>` line per written file, then one `removed <path>` line per removed file. Do not commit the pages: they are rebuilt from the data whenever needed.

With two or more languages in `languages`, every overview data file and the table of contents are pairs like the IR (translations.md): `a.md` and `a.en.md`, `toc.yaml` and `toc.en.yaml`, with a consistency record each. build and serve render one set of pages per language from that language's sides: the first language's pages where they always were, every other language's pages under `<tag>/` with the same names (`en/a.html`, `en/index.html`, `en/style.css`). Every page links the same page in each other language, named by that language's `language_name`. The text the pages write themselves (counts, marks, column headings, status labels) is that language's UI text: English is built in, other languages come from `labels.<tag>` (config.md). A reference to an IR item opens that language's side of the item; a reference to a decision record, an ADR or other Markdown in the place for records opens its text only on the first language's pages, and is a label only on the others. Write the other languages' overview data as translations of the first language's: only the sentences differ (the fields that are sentences are listed per part kind in translations.md).

`kotowari overview serve` does the same check, then takes the port `--port` (default 4590) on 127.0.0.1, writes the pages, prints one line `http://127.0.0.1:<port>/` and serves the files under `.kotowari/cache/overview/` until interrupted with Ctrl-C, which ends it with exit code 0. `/` is `index.html`. A path leaving that place (`..`, an absolute path, a symbolic link leading outside) and a directory get 404. It takes `--port` and `--config`, not `--format`, and prints nothing per request.

Both stop with exit code 2 and write nothing when the data or the table of contents has errors, or a pair of the IR, the overview data or the table of contents has translation_missing or translation_structure_mismatch: the first line of standard error is `overview error: N errors in overview data; run kotowari check`. Run `kotowari check`, fix the overview findings it reports, and build again. serve stops with `port error: 127.0.0.1:<port>: <OS error>` when the port cannot be used, without trying another one, and with the same error when it fails to accept a connection while serving. Both stop with `cache error: <path>` and write or remove nothing when `.kotowari`, `.kotowari/cache`, `.kotowari/cache/overview` or, with two or more languages, a language's `.kotowari/cache/overview/<tag>` is a symbolic link or not a directory, and with `cache error: <path>: <OS error>` when creating that place, writing a page or removing a file fails.

## Findings on overview data

The findings are in the kinds table of findings.md: overview_form_invalid, overview_part_unknown, overview_part_invalid, overview_lead_missing, overview_ir_missing, overview_ir_shared, overview_ref_unresolved and overview_name_conflict, plus invalid_marker and guide_stale on the guide marks of overview data, with `path` the overview data file. A file named `index.md` or `style.md`, or two files with the same name in different directories, give overview_name_conflict, because the page names would collide.

The table of contents has its own kinds, with `path` the table of contents and a null line: overview_toc_invalid when it cannot be read as YAML or does not have the form above (detail is the place, written as for overview_part_invalid), and, only when its form is correct, overview_toc_page_missing (detail is the page name), overview_toc_page_unknown, overview_toc_page_duplicate (the second and later places, in written order depth first) and overview_toc_group_empty (detail is the JSON Pointer of the item or group, `(root)` for the outermost group).

## guide_stale on overview data

guide_stale on an overview data file means the IR behind that section changed after the overview was written. The page marks the section as not reviewed since the IR changed. Cycle and implementation never edit overview data, even to clear these notices: the next brainstorm that touches the topic revises the overview and copies the current fingerprints, as in the scene above. A notice does not change the exit code.
