# kotowari plan

English | [日本語](plan.ja.md)

Reads one plan (a Markdown document of implementation steps) and checks its form against the schema bundled in the kotowari binary.
Use it in the kotowari-plan station when you have finished writing a plan, before asking for approval.

## Synopsis

<!-- @kotowari[REQ-core-190:b4b9f2e6, REQ-core-002:d9acbe9c] -->

```sh
kotowari plan [--format json|text] <計画書のファイル>
```

Options may appear before the command or after the file.
`--config` is not accepted (passing it stops with an argument error).

## Options and arguments

<!-- @kotowari[REQ-core-190:b4b9f2e6, REQ-core-002:d9acbe9c, REQ-core-021:ccedd28b] -->

| Name | Value | Default | Description |
|---|---|---|---|
| `<計画書のファイル>` (plan file) | path | none (required) | The plan to check. Exactly one. Read as a path relative to the current directory |
| `--format` | `json` or `text` | `json` | The output format |
| `--help` / `--version` | none | | Print the usage or the version and exit |

## What it reads

<!-- @kotowari[REQ-core-196:283ed4cd, REQ-core-191:162de9ae, EX-core-359:67cce6be, EX-core-345:bdb6abae] -->

`plan` reads only the plan file you pass.
It does not read the configuration file, the IR, decision records or test files.
So `plan` does not stop even if the configuration file is broken.

The rules for the form (the schema) are compiled into the binary; no schema file is read at run time.
A frontmatter block at the top of the plan is skipped without being read, whatever it contains.
Pointing at a schema with `$schema` in the frontmatter has no effect, and causes neither a finding nor a stop.

## The form of a plan

<!-- @kotowari[REQ-core-192:e10bf1b8, EX-core-337:5f0d07c9] -->

A plan is read line by line, and has no errors only when it has the following form.

```markdown
# <題名>                       ← ちょうど1つ。題名と最初の ## の間は空行だけ

## Goal
## Specification
## Approach and why
## Scope of change
## Step order and prerequisites
## Verification map
## Left to the implementer
## Stop conditions
## Test command                 ← この節だけ 0 個か 1 個
## Out of scope
## Steps

### S1: <このステップで作るもの>
- Purpose: ...
- Specification: ...
- Prerequisites: ...
- May change: ...
- Done when: ...
- Shown by: test | check | artifact | external ...
- Left to the implementer: ...
- Stop and hand back if: ...
```

In the block above, the title (`# <題名>`) must appear exactly once, with only blank lines between it and the first `##`; each step heading names what that step produces.
The ten sections other than `## Test command` must each appear exactly once (`## Test command` may appear zero times or once), and no other `## ` section is allowed.
A complete example is in [Example](#example).

### Plan-wide sections

<!-- @kotowari[REQ-core-192:e10bf1b8, EX-core-339:c1d921ec, EX-core-353:6b1ad7b6, EX-core-348:33e00663] -->

What may and may not go under the sections other than `## Steps`:

| Allowed | Not allowed |
|---|---|
| Sentences | Numbered lists (`1. ...`) |
| Unnumbered lists (`-`) and their child lists | `### ` and deeper headings |
| Tables | |
| Code blocks | |

### Steps

<!-- @kotowari[REQ-core-192:e10bf1b8, EX-core-333:7fc19aa4, EX-core-334:37e6d2a2, EX-core-340:bd7dd866] -->

The rules for the `## Steps` section:

| Rule | Details |
|---|---|
| Step heading | `### S`, then one or more digits, then `:` (`### S1: read the input`). At least one is required |
| Before the first step | No non-blank lines and no other headings |
| Fields | `Purpose`, `Specification`, `Prerequisites`, `May change`, `Done when`, `Shown by`, `Left to the implementer` and `Stop and hand back if`, each written once, in this order, as a list line of the form `- Name: value`. The list marker may be `-`, `*` or `+` |
| Other lines in a step | Under a step, no non-blank lines other than the eight field lines are allowed. Write each value on one line |
| Value of `Shown by` | Starts with one of the words `test`, `check`, `artifact` or `external`, followed by whitespace or the end of the value |

### What is not checked

<!-- @kotowari[REQ-core-192:e10bf1b8, EX-core-341:274e4805] -->

The following are not checked:

- The order of the sections
- Whether step numbers are consecutive, or whether two steps share a number
- The name after the `:` in a step heading
- The contents of the fields (whether requirement IDs exist, whether `Done when` is an observable condition, and so on)

Use `kotowari query` to confirm that requirement IDs exist.

## Output

### text

<!-- @kotowari[REQ-core-193:ee3eed54, REQ-core-207:1f5ede36, REQ-core-025:58025379, EX-core-381:a4b8c124, EX-core-382:a36d81e7] -->

Prints one finding per line in the form `path:line [error] invalid_plan detail`, and nothing else.
When there are no findings, nothing is printed (there is no tally line).

```text
docs/plans/broken.md:69 [error] invalid_plan field_pattern_mismatch: value "manual — read the output" does not match pattern "^(test|check|artifact|external)(\s|$)"
```

When a step is missing a required field, the finding points at that step's heading line.
The following is the output for a plan whose step `### S1: read the input` on line 45 has no `- Done when:` line.

```text
docs/plans/a.md:45 [error] invalid_plan missing_required_field: field "Done when" is required but missing
```

| Part | Contents |
|---|---|
| path | The path of the plan file relative to the base directory. Normalized; contains `../` if the file is outside the base directory |
| line | The line reported by the schema side. When a step is missing a required field, the line of that step's heading. `-` if there is no line |
| detail | The schema side's kind and detail joined by `: ` (`field_pattern_mismatch: ...`) |

There is only one finding kind, `invalid_plan`, and its severity is error.
Tell what is wrong from the first half of the detail (the schema side's kind).

### JSON

<!-- @kotowari[REQ-core-194:beb92c09, EX-core-346:ecaa0604] -->

The top level has exactly two keys: `findings` and `counts`.
The `files`, `lines`, `tests` and `guides` keys of `check` are not present.

| Key | Type | Description |
|---|---|---|
| `findings` | array | The list of findings. Each one has the keys `kind`, `severity`, `path`, `line` and `detail` (same as `check`). `line` is null when there is no line |
| `counts` | object | The number of findings per kind (`{"invalid_plan": 4}`). `{}` when there are none |

## Exit codes

<!-- @kotowari[TBL-core-002:14c565f2, REQ-core-193:ee3eed54] -->

| Code | Meaning |
|---|---|
| 0 | The plan's form has no errors |
| 1 | One or more `invalid_plan` |
| 2 | Stopped (argument error; the plan does not exist, is a directory or cannot be read; not UTF-8) |

On a stop, nothing is written to standard output and the reason is written on the first line of standard error.

| Situation | Start of the first line of standard error |
|---|---|
| No plan passed, two or more passed, or `--config` given | `argument error: ` |
| The plan does not exist, is a directory or cannot be read | `unreadable file: ` |
| The plan is not UTF-8 | `non-UTF-8 file: ` |

## Example

<!-- @kotowari[EX-core-332:680b57dc, EX-core-333:7fc19aa4, EX-core-334:37e6d2a2, EX-core-348:33e00663] -->

A plan in the correct form (the same as the [example plan](../../../agent/skills/kotowari-plan/references/plan-example.md) in the kotowari-plan skill).

````markdown
# Plan: report the words a document uses too often

## Goal

A person who runs `wordcount check <path>` sees each word used more than the configured limit, with the line of its first use.

## Specification

The IR is in `docs/ir/`. This plan covers:

- `docs/ir/core/count.md#REQ-core-010`, `#REQ-core-011`
- `docs/ir/core/output.md#REQ-core-020`
- Examples: EX-core-030, EX-core-031

## Approach and why

Counting and reporting are split: a pure function counts words per document, and the command only reads the file and prints. The existing `read_text` helper already stops on unreadable and non-UTF-8 files, so the command reuses it instead of opening the file itself.

## Scope of change

- `src/count.rs` (new)
- `src/main.rs`
  - the `check` arm only
- `tests/count.rs` (new)

## Step order and prerequisites

S1 before S2: S2 prints what S1 counts.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-010, REQ-core-011 | EX-core-030 |
| S2 | REQ-core-020 | EX-core-031 |

## Left to the implementer

- The names of the counting function and its result type

## Stop conditions

- The limit turns out to need a per-word setting, which the specification does not define
- An existing test fails for a reason other than the new command

## Test command

```sh
cargo test --workspace
```

## Out of scope

- Stemming or case folding beyond what REQ-core-011 states

## Steps

### S1: count the words of one document

- Purpose: count each word of a document and remember the line of its first use
- Specification: `docs/ir/core/count.md#REQ-core-010`, `docs/ir/core/count.md#REQ-core-011`
- Prerequisites: none
- May change: `src/count.rs`, `tests/count.rs`
- Done when: counting a document with "a" three times and the limit 2 returns "a" with the line of its first use, and a document within the limit returns nothing
- Shown by: test — EX-core-030, one test per rule of REQ-core-011
- Left to the implementer: none
- Stop and hand back if: REQ-core-011 and EX-core-030 disagree on what counts as a word

### S2: print the words over the limit

- Purpose: wire the count into `wordcount check` and print one line per word over the limit
- Specification: `docs/ir/core/output.md#REQ-core-020`
- Prerequisites: S1
- May change: `src/main.rs`, `tests/count.rs`
- Done when: `wordcount check a.md` prints `a.md:3 a` for EX-core-031 and exits with 1, and exits with 0 when nothing is over the limit
- Shown by: test — EX-core-031
- Left to the implementer: none
- Stop and hand back if: the exit code for a document with no words is not decided by REQ-core-020
````

```console
$ kotowari plan docs/plans/word-limit.md --format text
$ echo $?
0
$ kotowari plan docs/plans/word-limit.md
{"findings":[],"counts":{}}
```

Now feed it `broken.md`, which breaks this plan in three places:

- Added a `## Notes` section with one sentence before `## Out of scope`
- Changed S1's `Shown by` to `manual — read the output`
- Deleted S2's `Done when` line

```console
$ kotowari plan docs/plans/broken.md --format text
docs/plans/broken.md:52 [error] invalid_plan undeclared_heading: undeclared section heading "Notes"
docs/plans/broken.md:54 [error] invalid_plan undeclared_line: undeclared statement "Counting is case-sensitive."
docs/plans/broken.md:69 [error] invalid_plan field_pattern_mismatch: value "manual — read the output" does not match pattern "^(test|check|artifact|external)(\s|$)"
docs/plans/broken.md:73 [error] invalid_plan missing_required_field: field "Done when" is required but missing
$ echo $?
1
```

The line for the missing field (73) points at the line of that step's heading, `### S2: ...`.

Here is the JSON for the same input.

```console
$ kotowari plan docs/plans/broken.md | jq .
{
  "findings": [
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 52,
      "detail": "undeclared_heading: undeclared section heading \"Notes\""
    },
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 54,
      "detail": "undeclared_line: undeclared statement \"Counting is case-sensitive.\""
    },
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 69,
      "detail": "field_pattern_mismatch: value \"manual — read the output\" does not match pattern \"^(test|check|artifact|external)(\\s|$)\""
    },
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 73,
      "detail": "missing_required_field: field \"Done when\" is required but missing"
    }
  ],
  "counts": {
    "invalid_plan": 4
  }
}
```

## Common pitfalls

### Wrapping a field value onto two lines gives `undeclared_line`

<!-- @kotowari[REQ-core-192:e10bf1b8, EX-core-340:bd7dd866] -->

A plan is read line by line, so a continuation line of a field value does not become part of the field.
It is an error as a line that is not allowed under a step.
Write each value on one line, including any commands.

```console
$ kotowari plan docs/plans/wrapped.md --format text
docs/plans/wrapped.md:28 [error] invalid_plan undeclared_line: undeclared ordered list "S1 before S2: S2 prints what S1 counts."
docs/plans/wrapped.md:66 [error] invalid_plan undeclared_line: undeclared statement "one test per rule of REQ-core-011"
```

(Line 66 is `one test per rule of REQ-core-011`, indented and continued on the line after `- Shown by: test — EX-core-030,`.)

### A numbered list becomes `undeclared ordered list`

<!-- @kotowari[REQ-core-192:e10bf1b8, EX-core-353:6b1ad7b6] -->

Numbered lists are not allowed in the plan-wide sections (line 28 in the example above is `1. S1 before S2: ...`).
Rewrite it as a `-` bullet list or as sentences.
To express an order, write it as sentences in `## Step order and prerequisites`.

### `argument error: unexpected option for plan: --config`

<!-- @kotowari[REQ-core-190:b4b9f2e6, EX-core-343:af29bce2] -->

`plan` does not read the configuration, so it does not accept `--config`.
If you passed it, remove it.

```console
$ kotowari plan docs/plans/word-limit.md --config .kotowari/config.yaml
argument error: unexpected option for plan: --config
```

### `argument error: plan expects exactly one plan file path`

<!-- @kotowari[REQ-core-190:b4b9f2e6, EX-core-342:7e4f1631, EX-core-357:d71d499d] -->

You can pass only one plan at a time.
To check several, run the command once for each.

```console
$ kotowari plan
argument error: plan expects exactly one plan file path, got 0
$ kotowari plan docs/plans/word-limit.md docs/plans/broken.md
argument error: plan expects exactly one plan file path, got 2
```

### `unreadable file: ...`

<!-- @kotowari[REQ-core-197:2fbfa51a, EX-core-344:9b05eb1b] -->

The path is read relative to the current directory,
not relative to the base directory (the directory that contains `.kotowari/`).

```console
$ kotowari plan docs/plans/none.md
unreadable file: docs/plans/none.md: No such file or directory (os error 2)
```

The `path` in the output, on the other hand, is relative to the base directory.
Even when you run the command from a subdirectory, `path` comes out in the same form.

```console
$ cd docs && kotowari plan plans/broken.md --format text | head -1
docs/plans/broken.md:52 [error] invalid_plan undeclared_heading: undeclared section heading "Notes"
```

## Related

- Specification: [Checking plans (the plan IR)](../../ir/core/plan.md), [Arguments](../../ir/core/cli.md#REQ-core-190)
- Common format and stops: [cli.md](../cli.md)
- The list of finding kinds: [findings.md](../findings.md)
- How to write a plan: [step-template.md](../../../agent/skills/kotowari-plan/references/step-template.md) in the kotowari-plan skill
- Confirming requirement IDs: [kotowari query](query.md)
