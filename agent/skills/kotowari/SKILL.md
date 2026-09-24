---
name: kotowari
description: "Read, scene by scene, how to write kotowari's IR and decision records, how to use check, and how to place marks. Trigger words: kotowari, IR, docs/ir, @kotowari, marks, mutants, mutation tests. 日本語キーワード: 印 変異テスト"
---

kotowari is a tool for writing a normalised specification (the IR) in Markdown and checking it with `kotowari check`. It also has the read commands `kotowari list` (the list of items), `kotowari query` (one item's body and its reverse references) and `kotowari status` (a tally of whether everything is in place), `kotowari mutants`, which reads the results of mutation tests, and `kotowari plan`, which checks the form of one plan file against the schema bundled in kotowari. This skill has you read a reference per scene and conveys how to write the IR and decision records, how to read the check results, and how to place marks.

First run `kotowari --version` to confirm the tool is there.

- The command is not found → ask for kotowari to be installed, and do not start the work
- No version can be read from the output → tell the person the version cannot be confirmed, and do not start the work

The target version is not pinned. The references are revised to follow the specification of kotowari itself, and carry their revision date at the top.

Choosing the scene: if the person names one, follow it. Otherwise choose from the context.

| Scene | When | Reference to read |
|---|---|---|
| setup | There is no place for the files yet; first use | config.md |
| write | Writing the IR and decision records during a brainstorm | ir-form.md and records.md |
| check | Reading the result of `kotowari check` or `kotowari plan`, or the output of `kotowari list`, `kotowari query` or `kotowari status` | findings.md |
| mark | Placing marks while writing tests | mark.md |
| mutants | Reading the result of `kotowari mutants`, investigating misses | mutants.md |

Before the approval in write, also read collate.md.

Which work uses each scene:

- setup — once, at the start of a project
- write — when writing the IR and decision records in a brainstorm session
- check — brainstorm (before approval), plan (when reading requirements, and when checking the plan's form), cycle (before the final report) and implement (check commands)
- mark — when implement and the fixer write tests
- mutants — when a push is stopped by missed mutations, and when investigating misses while cleaning up after a cycle
