---
name: kotowari
description: "Read, scene by scene, how to write kotowari's IR and decision records, how to use check, and how to place marks. Trigger words: kotowari, IR, docs/ir, @kotowari, marks, mutants, mutation tests, guides, guide_stale, overview, overview build, overview serve, surface, surface_without_spec, changes, change_stale, change_record_invalid, languages, labels, translation, i18n.yaml, translation_stale. 日本語キーワード: 印 変異テスト ガイド 全体像 面 変更照合 対 翻訳 多言語"
---

## Scope

This skill governs IR, record and marker use and reading the tool results. Workflow decisions belong to the named workflow skills.

kotowari is a tool for writing a normalised specification (the IR) in Markdown and checking it with `kotowari check`. It also has the read commands `kotowari list` (the list of items), `kotowari query` (one item's body and its reverse references) and `kotowari status` (a tally of whether everything is in place), `kotowari mutants`, which reads the results of mutation tests, and `kotowari plan`, which checks the form of one plan file against the schema bundled in kotowari. `kotowari changes` compares caller-written records with a Git snapshot. `kotowari overview build` and `kotowari overview serve` render the overview data an LLM writes into pages that show the whole picture of a topic. This skill has you read a reference per scene and conveys how to write the IR and decision records, how to read the check results, and how to place marks.

First run `kotowari --version` to confirm the tool is there.

- The command is not found → ask for kotowari to be installed, and do not start the work
- No version can be read from the output → tell the person the version cannot be confirmed, and do not start the work

The target version is not pinned. The references are revised to follow the specification of kotowari itself, and carry their revision date at the top.

Choosing the scene: if the person names one, follow it. Otherwise choose from the context.

| Scene | When | Reference to read |
|---|---|---|
| changes | Writing or reconciling change records; the optional self-check before a commit, independent final review and CI | changes.md |
| setup | There is no place for the files yet; first use | config.md |
| write | Writing the IR and decision records during a brainstorm | ir-form.md and records.md |
| check | Reading the result of `kotowari check` or `kotowari plan`, or the output of `kotowari list`, `kotowari query` or `kotowari status` | findings.md |
| mark | Placing marks while writing tests | mark.md |
| mutants | Reading the result of `kotowari mutants`, investigating misses | mutants.md |
| surface | Writing surface rules (`surface.rules`) and the list of unspecified surfaces, or fixing surface_without_spec, surface_unspecified_invalid or surface_unspecified_stale | surface.md |
| overview | Writing or revising overview data at a brainstorm approval, choosing the unit of an overview, running `kotowari overview build` or `serve`, or reading an overview finding or guide_stale on overview data | overview.md |
| guide | Writing a guide (a user-facing document listed in `guides.files`) and placing its guide marks, or reviewing a guide after `kotowari check` raised guide_stale | guides.md |
| translate | A project with two or more `languages`: writing or changing any side of a pair, adding a language, or reading a translation_*, link_language_mismatch or link_to_record finding | translations.md |

Before the approval in write, also read collate.md.

Which work uses each scene:

- setup — once, at the start of a project
- write — when writing the IR and decision records in a brainstorm session
- check — brainstorm (before approval), plan (when reading requirements, and when checking the plan's form), cycle (before the final report) and implement (check commands)
- mark — when implement and the fixer write tests
- mutants — when a push is stopped by missed mutations, and when investigating misses while cleaning up after a cycle
- surface — when first writing surface rules for a project (adopt), and when `kotowari check` raises a surface finding
- overview — brainstorm (at approval, only in a project whose configuration has the `overview` key); never cycle or implement
- guide — when writing or revising a guide, and when `kotowari check` raises guide_stale
- translate — whenever a side of a pair changes, in any work, so that every side changes in the same change

## What the IR holds

The IR holds what the product delivers to the people who install and use it: the behavior they
can observe through its commands, configuration, output, and distributed files. The test for a
rule: if it changed, would someone using the product see a difference? If not, it is not IR.

Not IR, in every station:

- CI and workflow definitions, hooks, and the gates they enforce;
- the release procedure and the build configuration;
- the repository's own data being in some state (its IR, records, and documents that do not ship);
- rules about the project's own tests, checks, and fixtures: what verifies the product is not the
  product (a check the product ships to its users is product, and a test of product behavior still
  carries the mark of the requirement it verifies);
- the development process.

A decision about these goes in the decision record, and the rule is written where contributors
read it (the project's instructions, a comment in the file). It gets no requirement and no mark;
verify it by running it once or with the platform's own checker (a workflow linter, for
example), not with a test written for it.

When unsure, the default is not IR: record the decision as not IR with its reason, and show it to
the person at the next point they see the work. Never add a rule to the IR to make a test count
as evidence or to give a change record something to point at. What the IR may hold is settled
here first; whether an oracle meets the evidence conditions is asked only after that. Approved IR
written before this definition stays as it is until the person decides: never delete or move it
on your own.
