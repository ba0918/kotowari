# Surface check

English | [日本語](surface.ja.md)

<!-- @kotowari[REQ-core-223:418e5055, REQ-core-227:9203538c] -->

This check finds features that were built without being written into the IR.
It takes user-visible behaviour (CLI subcommands and flags, configuration keys, HTTP routes and so on) out of the code using ast-grep rules you write in the configuration.
Each piece taken out is called a "surface".
If a surface's name does not appear quoted in the IR, `kotowari check` reports it as a `surface_without_spec` error.

In a project that writes no surface rules, this check does nothing.

## Configuration

<!-- @kotowari[REQ-core-224:f13b971e, REQ-core-225:be4cdd0b] -->

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

<!-- @kotowari[REQ-core-223:418e5055, REQ-core-224:f13b971e, REQ-core-236:45fa5c5f] -->

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
In the IR, either write `"format"` exactly as the name is taken out, or write a rule that captures it in the `--format` form.

Checking a rule's matches with `ast-grep scan -r rules/surface.yml src/main.rs` before adding it to the configuration saves time.

## Where a surface counts as being in the IR

<!-- @kotowari[REQ-core-226:86d88ed4, EX-core-410:d4987877, EX-core-411:0e1012fa, EX-core-429:1a617a65] -->

A surface is in the IR if its name appears, as the contents of a pair of double quotes or a pair of backquotes, in any of the following places in a topic document.

- A requirement's statements (statements of deferred requirements count too)
- The cells of a decision table (cells of the header row count too)
- The step lines of a scenario

It counts only when the enclosed contents are identical to the name, including leading and trailing whitespace.
`"--verbose true"` and `" --verbose"` do not contain the surface `--verbose`.
Double quotes are paired from left to right in the line, and only backquotes outside double quotes are paired.
Backquoted text is also subject to the term check, so enclose names that are not in the glossary in double quotes.

A name does not count if it appears in a property's statements, in the scope lines of a document, in lines starting with `- ` under an item's heading (including `- how_to_verify:`), in a problem record (`FLAGS.md`), or in a glossary.

## Findings

<!-- @kotowari[REQ-core-227:9203538c, EX-core-408:eaffb4f4, EX-core-409:4eb051b9] -->

This is the result of combining the CLI from the example above with an IR whose requirement statements contain `"check"` and `"--format"`.

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
`flag format` is reported because the IR writes `"--format"`, which does not match the name `format`.

There are two ways to fix it.

- Quote the name in the requirement that defines that surface. A surface missing from the IR is a gap in the specification, so decide the requirement in brainstorming.
- If it is not going into the specification now, add it with a reason to the list of unspecified surfaces. Adding to the list is also a specification decision, so do it only during brainstorming or adoption. Do not add to the list midway through implementation to make the error go away; take any surface missing from the IR back to brainstorming.

## List of unspecified surfaces

<!-- @kotowari[REQ-core-231:d03e71cc, REQ-core-232:3989b803, REQ-core-233:f788681d, REQ-core-234:8a2d9f6e] -->

This is the YAML file that `surface.unspecified` points at.
Each entry has exactly the three keys `kind`, `name` and `why`; every value is a string, and `why` cannot be empty.
When `kind` and `name` match a surface's kind and name as identical strings (including leading and trailing whitespace), that surface is not an error even though it is missing from the IR.

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
| `surface_unspecified_stale` (notice) | No surface in the code matches the entry, or the matching surface has been written into the IR | Delete the entry. If you renamed the surface, fix `name` |

The `line` of findings about the list is always null.
Two entries with the same content are not checked for.

If the key is absent, or the file is empty (zero bytes or only comments), the list has zero entries.
If the target does not exist or cannot be read, kotowari stops with `unreadable file`; if it is not UTF-8, with `non-UTF-8 file`; and if it cannot be read as YAML or its top level is not a sequence, with a `config error` naming the list's path.

## Seeing how many are excluded

<!-- @kotowari[REQ-core-228:095b2109, REQ-core-229:7909d808, TBL-core-028:9645c008] -->

The list is a debt, not a storage place.
So that the amount borrowed is visible every time, in a project that sets `surface.rules`, `check` reports how many surfaces the list excluded.

- With `--format text`, the last line after the finding lines is `surface: unspecified=N`. It appears even when there are no findings and even when the number is 0.
- In JSON, a top-level `surface` with a single key, `unspecified`.

What is counted is kind-and-name pairs; a surface that is in the IR is not counted even if it is in the list.

`status` reports three numbers in its `surface` group.
If `surface.rules` is an empty list, all three are 0.

```console
$ kotowari status --format text | grep '^surface'
surface total=4 specified=1 unspecified=1
```

| Key | Number |
|---|---|
| `total` | Number of surface kind-and-name pairs |
| `specified` | Of those, the ones in the IR |
| `unspecified` | Those not in the IR and excluded by the list |

`surface_without_spec` is an error, so while any remain, `complete` in `status` is false.

Only `check` and `status` read surface files, surface rule files and the list of unspecified surfaces.
`list`, `query`, `plan` and `mutants` do not read them and never stop because of them.

## Bringing it into an existing project

<!-- @kotowari[REQ-core-228:095b2109] -->

When you first write surface rules, every surface not yet written into the IR shows up as `surface_without_spec` at once.
In that case, put all the reported surfaces in the list of unspecified surfaces with reasons, and remove them from the list as you write the requirements for each topic.
`unspecified` in `kotowari status` shows how much is left.
