# Surface check

English | [日本語](surface.ja.md)

<!-- @kotowari[docs/decisions/core/surface.md#surfaces-are-taken-out-of-the-code-with-surfacefiles-and-surfacerules-in-the-configuration-as-nodes-matched-by-ast-grep-rules:0cbadb0d, docs/decisions/core/surface.md#a-surface-not-in-the-specification-gets-one-surface_without_spec-error-at-the-place-in-the-code-where-it-is-first-found:210038a0] -->

This check finds features that were built without being written into the decision documents.
It takes user-visible behaviour (CLI subcommands and flags, configuration keys, HTTP routes and so on) out of the code using ast-grep rules you write in the configuration.
Each piece taken out is called a "surface".
If a surface's name does not appear quoted in a decision, `kotowari check` reports it as a `surface_without_spec` error.

In a project that writes no surface rules, this check does nothing.

## Configuration

<!-- @kotowari[docs/decisions/core/surface.md#surface-rules-and-surface-files-are-read-with-the-same-construction-as-testsrules-and-testsfiles:77fb35de, docs/decisions/core/surface.md#when-the-surface-configuration-keys-are-not-all-present-every-command-that-reads-the-configuration-stops:ab3a2fa6] -->

```yaml
surface:
  files:
    - "src/**/*.rs"
  rules:
    - "rules/surface.yml"
  unspecified: "docs/surface-unspecified.yaml"   # 書かなくてもよい
```

| Key | Contents |
|---|---|
| `surface.files` | Globs of the files to search for surfaces (surface files). Read and walked the same way as `tests.files`. Only files in a surface rule's language are read; if one cannot be read or is not UTF-8, kotowari stops, just as for test files |
| `surface.rules` | Paths of the surface rules (ast-grep rule YAML files). The check runs when this is not an empty list |
| `surface.unspecified` | The file of the list of unspecified surfaces ([section below](#list-of-unspecified-surfaces)) |

Write `surface.files` and `surface.rules` as a set.
If only one of them is written, or if `surface.unspecified` is written while `surface.rules` is empty, every command that reads the configuration stops with a configuration error.
This prevents a state where you think the check runs but it does nothing.
All keys are listed in [the configuration file](config.md).

## Writing surface rules

<!-- @kotowari[docs/decisions/core/surface.md#surfaces-are-taken-out-of-the-code-with-surfacefiles-and-surfacerules-in-the-configuration-as-nodes-matched-by-ast-grep-rules:0cbadb0d, docs/decisions/core/surface.md#surface-rules-and-surface-files-are-read-with-the-same-construction-as-testsrules-and-testsfiles:77fb35de, docs/decisions/core/surface.md#only-surface-files-in-the-languages-of-the-rules-are-read-and-parsed-into-syntax-trees-and-other-surface-files-are-not-read:82d0c416] -->

Surface rules are the same kind of ast-grep rule as `tests.rules`, which finds tests ([marking tests](marks.md)).
However, they are not added to the queries for tests; they are applied separately.

- One node matched by a rule is one surface. The surface's kind is the rule's `id`, and its name is the text captured by `$NAME`. A match that does not capture `$NAME` is not a surface.
- If the name begins and ends with the same quote (`'`, `"` or a backquote), one pair is removed. Nothing else is changed: no `--` is added and case is not changed.
- `language` is case-insensitive and also accepts ast-grep aliases (`ts`, `py`). A rule is applied only to files with its language's extensions.
- `files` and `ignores` narrow down the files a rule is applied to. `severity` is not looked at, so rules with `severity: off` are applied too.
- Only files in a rule's language are read and parsed into a syntax tree. If there is a syntax error, `unparsable_file` is reported and the file is skipped. Other files are not read, so kotowari does not stop even if they are unreadable or not UTF-8. A broad glob such as `src/**` that also matches images and the like is fine.
- If a rule file is missing, unreadable or broken, or the same path appears twice, `check` and `status` stop with a configuration error.
- kotowari bundles no surface rules.

Here is an example that takes out the flags and subcommands of a CLI written with clap's builder API.
The code is in `src/main.rs` and the rules in `rules/surface.yml`.

```rust
use clap::{Arg, ArgAction, Command};

fn cli() -> Command {
    Command::new("tool")
        .arg(Arg::new("verbose").long("verbose").action(ArgAction::SetTrue))
        .subcommand(Command::new("check").arg(Arg::new("format").long("format")))
        .subcommand(Command::new("list"))
}
```

```yaml
# rules/surface.yml
id: flag
language: rust
rule:
  pattern: $ARG.long($NAME)
---
id: subcommand
language: rust
rule:
  pattern: Command::new($NAME)
  inside:
    kind: arguments
    stopBy: end
    inside:
      pattern: $PARENT.subcommand($$$)
```

This yields four surfaces: `flag verbose`, `flag format`, `subcommand check` and `subcommand list`.
The root `Command::new("tool")` is not inside a `subcommand` call, so it is not taken out.
The flag's name is `format`, not `--format`.
In the decision, either write `"format"` exactly as the name is taken out, or write a rule that captures it in the `--format` form.

Checking a rule's matches with `ast-grep scan -r rules/surface.yml src/main.rs` before adding it to the configuration saves time.

## Where a surface counts as being in the specification

<!-- @kotowari[docs/decisions/core/surface.md#if-the-name-of-a-surface-is-quoted-in-decision-or-confirmations-of-a-decision-that-surface-is-in-the-specification:6c702ea9] -->

A surface is in the specification if its name appears, as the contents of a pair of double quotes or a pair of backquotes, in the Decision or Confirmations section of any decision of any decision document.

- Sentences and list items of those sections
- The cells of a table in them (cells of the header row count too)
- Decisions of either state: proposed decisions count too

It counts only when the enclosed contents are identical to the name, including leading and trailing whitespace.
`"--verbose true"` and `" --verbose"` do not contain the surface `--verbose`.
Double quotes are paired from left to right in the line, and only backquotes outside double quotes are paired.
A name does not count if it appears in the Problem or the Rejected options of a decision, in the `## Open` section, in the lead paragraph of a document, in a glossary, or inside a code block.
In a pair of decision documents, only the first language's side is looked at.

## Findings

<!-- @kotowari[docs/decisions/core/surface.md#a-surface-not-in-the-specification-gets-one-surface_without_spec-error-at-the-place-in-the-code-where-it-is-first-found:210038a0] -->

This is the result of combining the CLI from the example above with a decision document whose decision contains `"check"` and `"--format"`.

```console
$ kotowari check --format text
src/main.rs:5 [error] surface_without_spec flag verbose
src/main.rs:6 [error] surface_without_spec flag format
src/main.rs:7 [error] surface_without_spec subcommand list
surface: unspecified=0
$ echo $?
1
```

The detail of `surface_without_spec` is `kind name`.
However many places a surface with the same kind and name appears in, only one finding is reported, at the first place in byte order of path and then line order.
`flag format` is reported because the decision writes `"--format"`, which does not match the name `format`.

There are two ways to fix it.

- Quote the name in the decision that defines that surface. A surface missing from the decision documents is a gap in the specification, so decide it in brainstorming.
- If it is not going into the specification now, add it with a reason to the list of unspecified surfaces. Adding to the list is also a specification decision, so do it only during brainstorming or adoption. Do not add to the list midway through implementation to make the error go away; take any surface missing from the decision documents back to brainstorming.

## List of unspecified surfaces

<!-- @kotowari[docs/decisions/core/surface-unspecified.md#if-the-list-file-is-missing-or-cannot-be-read-check-and-status-stop:83dd3917, docs/decisions/core/surface-unspecified.md#a-surface-not-yet-made-part-of-the-specification-is-not-an-error-if-it-is-put-on-the-list-of-unspecified-surfaces-with-a-reason:0989b365, docs/decisions/core/surface-unspecified.md#a-malformed-entry-is-an-error-and-an-entry-no-longer-needed-is-a-notice:090bd4da] -->

This is the YAML file that `surface.unspecified` points at.
Each entry has exactly the three keys `kind`, `name` and `why`; every value is a string, and `why` cannot be empty.
When `kind` and `name` match a surface's kind and name as identical strings (including leading and trailing whitespace), that surface is not an error even though it is missing from the decision documents.

```yaml
- kind: "flag"
  name: "verbose"
  why: "出力の話題を導入するときに仕様にする"
- kind: "subcommand"
  name: "list"
  why: ""
- kind: "flag"
  name: "old"
  why: "消した"
```

```console
$ kotowari check --format text
docs/surface-unspecified.yaml:- [error] surface_unspecified_invalid subcommand list
docs/surface-unspecified.yaml:- [notice] surface_unspecified_stale flag old
src/main.rs:6 [error] surface_without_spec flag format
src/main.rs:7 [error] surface_without_spec subcommand list
surface: unspecified=1
```

| Finding | What happens | How to fix |
|---|---|---|
| `surface_unspecified_invalid` (error) | An entry has the wrong form (missing keys, extra keys, a non-string value, a whitespace-only `why`). That entry excludes no surface | Fix the form |
| `surface_unspecified_stale` (notice) | No surface in the code matches the entry, or the matching surface has been quoted in a decision | Delete the entry. If you renamed the surface, fix `name` |

The `line` of findings about the list is always null.
Two entries with the same content are not checked for.

If the key is absent, or the file is empty (zero bytes or only comments), the list has zero entries.
If the target does not exist or cannot be read, kotowari stops with `unreadable file`; if it is not UTF-8, with `non-UTF-8 file`; and if it cannot be read as YAML or its top level is not a sequence, with a `config error` naming the list's path.

## Seeing how many are excluded

<!-- @kotowari[docs/decisions/core/surface.md#the-number-of-surfaces-excluded-by-the-list-of-unspecified-surfaces-is-always-output-at-the-end-of-the-output-of-check:d8dacbf9, docs/decisions/core/surface.md#only-check-and-status-read-surfaces:77268ec4, docs/decisions/core/status.md#the-summary-outputs-per-group-the-counts-of-test-markers-guides-overview-surfaces-and-findings:d52c4092] -->

The list is a debt, not a storage place.
So that the amount borrowed is visible every time, in a project that sets `surface.rules`, `check` reports how many surfaces the list excluded.

- With `--format text`, the last line after the finding lines is `surface: unspecified=N`. It appears even when there are no findings and even when the number is 0.
- In JSON, a top-level `surface` with a single key, `unspecified`.

What is counted is kind-and-name pairs; a surface that is quoted in a decision is not counted even if it is in the list.

`status` reports three numbers in its `surface` group.
If `surface.rules` is an empty list, all three are 0.

```console
$ kotowari status --format text | grep '^surface'
surface total=4 specified=1 unspecified=1
```

| Key | Number |
|---|---|
| `total` | Number of surface kind-and-name pairs |
| `specified` | Of those, the ones quoted in decisions |
| `unspecified` | Those not quoted and excluded by the list |

`surface_without_spec` is an error, so while any remain, `complete` in `status` is false.

Only `check` and `status` read surface files, surface rule files and the list of unspecified surfaces.
`list`, `query`, `plan` and `mutants` do not read them and never stop because of them.

## Bringing it into an existing project

<!-- @kotowari[docs/decisions/core/surface.md#the-number-of-surfaces-excluded-by-the-list-of-unspecified-surfaces-is-always-output-at-the-end-of-the-output-of-check:d8dacbf9] -->

When you first write surface rules, every surface not yet quoted in a decision shows up as `surface_without_spec` at once.
In that case, put all the reported surfaces in the list of unspecified surfaces with reasons, and remove them from the list as you write the decisions for each topic.
`unspecified` in `kotowari status` shows how much is left.
