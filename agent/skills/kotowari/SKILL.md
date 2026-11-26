---
name: kotowari
description: "Read, scene by scene, how to write kotowari's decision documents, how to use check, and how to place marks. Trigger words: kotowari, decision documents, docs/decisions, @kotowari, marks, mutants, mutation tests, guides, guide_stale, overview, overview build, overview serve, surface, surface_without_spec, languages, labels, translation, i18n.yaml, translation_stale. 日本語キーワード: 決定の文書 確認すること 読んで確認 未決 印 変異テスト ガイド 全体像 面 対 翻訳 多言語"
---

## Scope

This skill governs decision document and marker use and reading the tool results. Workflow decisions belong to the named workflow skills.

kotowari is a tool for writing a specification as decision documents in Markdown and checking it with `kotowari check`. It also has the read commands `kotowari list` (the list of decisions and rows to confirm), `kotowari query` (one decision or row with its body and reverse references) and `kotowari status` (a tally of whether everything is in place), `kotowari mutants`, which reads the results of mutation tests, and `kotowari plan`, which checks the form of one plan file against the schema bundled in kotowari. `kotowari overview build` and `kotowari overview serve` render the overview data an LLM writes into pages that show the whole picture of a topic. This skill has you read a reference per scene and conveys how to write decision documents, how to read the check results, and how to place marks.

First run `kotowari --version` to confirm the tool is there.

- The command is not found → ask for kotowari to be installed, and do not start the work
- No version can be read from the output → tell the person the version cannot be confirmed, and do not start the work

The target version is not pinned. The references are revised to follow the specification of kotowari itself, and carry their revision date at the top.

Choosing the scene: if the person names one, follow it. Otherwise choose from the context.

| Scene | When | Reference to read |
|---|---|---|
| setup | There is no place for the files yet; first use | config.md |
| decide | Writing or changing a decision document (a topic file under `decisions.documents`): a new decision, a state, the rows to confirm, an overturned decision, the open section | decision-documents.md |
| check | Reading the result of `kotowari check` or `kotowari plan`, or the output of `kotowari list`, `kotowari query` or `kotowari status`; putting `kotowari check` into hooks or CI | findings.md |
| mark | Placing marks while writing tests | mark.md |
| mutants | Reading the result of `kotowari mutants`, investigating misses | mutants.md |
| surface | Writing surface rules (`surface.rules`) and the list of unspecified surfaces, or fixing surface_without_spec, surface_unspecified_invalid or surface_unspecified_stale | surface.md |
| overview | Writing or revising overview data at a brainstorm approval, choosing the unit of an overview, running `kotowari overview build` or `serve`, or reading an overview finding or guide_stale on overview data | overview.md |
| guide | Writing a guide (a user-facing document listed in `guides.files`) and placing its guide marks, or reviewing a guide after `kotowari check` raised guide_stale | guides.md |
| translate | A project with two or more `languages`: writing or changing any side of a pair, adding a language, or reading a translation_*, link_language_mismatch or link_to_record finding | translations.md |

Which work uses each scene:

- setup — once, at the start of a project
- decide — whenever a decision document is written or changed: a brainstorm writing decisions, an implementer turning a state or correcting a sentence to the facts, and whoever writes an item of the open section
- check — brainstorm (before approval), cycle (when reading requirements, and before the final report), implement (check commands), and whoever checks a plan file's form with `kotowari plan`
- mark — when implement and the fixer write tests
- mutants — when a push is stopped by missed mutations, and when investigating misses while cleaning up after a cycle
- surface — when first writing surface rules for a project (adopt), and when `kotowari check` raises a surface finding
- overview — brainstorm (at approval, only in a project whose configuration has the `overview` key); never cycle or implement
- guide — when writing or revising a guide, and when `kotowari check` raises guide_stale
- translate — whenever a side of a pair changes, in any work, so that every side changes in the same change

## What the decision documents hold

A decision document holds what a person decided about the product and its making, one topic per
file (decision-documents.md). Product behaviour — what the people who install and use it can
observe through its commands, configuration, output and distributed files — is written as a
decision with rows to confirm, and each row checked by tests gets tests that carry its ID.

A decision that is not product behaviour (CI and workflow definitions, hooks and the gates they
enforce, the release procedure, the build configuration, rules about the project's own tests and
fixtures, the development process) is written in the same documents as a decision without rows to
confirm. It gets no mark; verify it by running it once or with the platform's own checker (a
workflow linter, for example), not with a test written for it. Never add a row to confirm to make
a test count as evidence.

Details the implementing side decided with a reason (a limit, an order, the line an error points
at, the wording of a message) are not written in a decision document: the code and tests hold
them, and the reasons are in the commit and pull request bodies.
