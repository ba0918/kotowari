Based on the kotowari specification (revised 2026-09-27; the version of kotowari itself is not pinned)

## What the surface check does

A surface is one piece of user-visible behavior taken from the code by a rule you write: a CLI subcommand or flag, a configuration key, an HTTP route. `kotowari check` takes every surface from the code and makes each one that the IR does not quote a surface_without_spec error. It is how a feature built without being written into the IR is caught. A project with no surface rules sees nothing of this.

Only `kotowari check` and `kotowari status` read the surface files, the surface rule files and the list of unspecified surfaces, and only they stop on those files. `kotowari list`, `query`, `plan` and `mutants` do not read them.

## Configuration

| Key | Value |
|---|---|
| surface.files | Globs of the files surfaces are taken from. Read like `tests.files`: the same walk, the same exclusions, and the same stops for an unreadable file, a file that is not UTF-8, and a dangling symbolic link |
| surface.rules | Paths of ast-grep rule YAML files, relative to the base directory. The check runs only when this list is not empty |
| surface.unspecified | Path of the list of unspecified surfaces (below). Optional |

`surface.files` and `surface.rules` are written together. Rules without files, files without rules, or `surface.unspecified` without rules stop every command that reads the configuration with a config error: otherwise the check would look configured while doing nothing.

```yaml
surface:
  files:
    - "src/**/*.rs"
  rules:
    - "rules/surface.yml"
  unspecified: "docs/surface-unspecified.yaml"
```

## Writing surface rules

A surface rule is an ast-grep rule, read as `tests.rules` reads its rules (mark.md, "Adding rules and languages without a query"), but kept apart: a surface rule is never added to the queries that find tests.

- Each node a rule matches is one surface. Its kind is the rule's `id`, and its name is the text captured by `$NAME`. A match that does not capture `$NAME` is not a surface.
- If the captured text starts and ends with the same quote (`'`, `"` or a backquote), one quote is removed from each end. Nothing else is changed: kotowari adds no `--` and changes no case. Write the rule so that it captures the text a user sees, and quote that same text in the IR.
- `language` is matched without case and accepts ast-grep's aliases (`ts`, `py`). A rule applies only to surface files whose extension gives that language.
- `files` and `ignores` apply against the path relative to the base directory. `fix`, `message`, `severity`, `note` and `metadata` are not used; a rule with `severity: off` still applies.
- Only surface files of a language some surface rule has are parsed. A syntax error in one raises unparsable_file (once per path, even if the file is also a test file); other surface files are not parsed and raise nothing.
- A missing rule file, a path that is not a file, an unreadable, non-UTF-8 or malformed rule file, the same path listed twice, or an unknown `language` stops check and status with a config error naming the rule file.
- kotowari bundles no surface rules.

Example: the flags and subcommands of a CLI built with clap's builder API.

```rust
Command::new("tool")
    .arg(Arg::new("verbose").long("verbose"))
    .subcommand(Command::new("check"))
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

These take `flag verbose` and `subcommand check` (the root `Command::new("tool")` is not inside a `subcommand` call and is not taken). The flag's name is `verbose`, not `--verbose`: the IR must quote `"verbose"` for it, or the rule must capture text that already reads `--verbose`. With clap's derive API the names come from field and variant names, which a rule captures as written in the code (`verbose`, `Check`); choose rules and IR wording that agree.

Check a rule before adding it with `ast-grep scan -r rules/surface.yml <file>`, then run `kotowari check`.

## When a surface is in the IR

A surface is in the IR when its name is the whole content of a pair of double quotes or of backticks in one of these places of a topic document:

- a statement of a requirement (a deferred requirement counts too);
- a cell of a decision table, the header row included;
- a step line of a scenario.

Double quotes pair from the left of the line; backticks pair only outside double quotes, as in the term check. The content must equal the name exactly, spaces included: `"--verbose true"` and `" --verbose"` do not contain the surface `--verbose`. A backticked name must still pass the term check, so a name in backticks has to be a glossary term; quote values with double quotes.

Not counted: statements of properties, the scope line, the lines starting with `- ` under an item heading (`- how_to_verify:` included), the flags (FLAGS.md), and the glossary.

## Findings and how to fix them

| Kind | Fix |
|---|---|
| surface_without_spec | The surface is user-visible but no requirement, decision table or scenario quotes it. A behavior missing from the IR is a change to the specification: the implementer returns it to brainstorm and never adds an entry to the list of unspecified surfaces. In kotowari-brainstorm, write it into the requirement that specifies it, quoting the name exactly, or, if it is not to be specified now, add an entry for it to the list. Adding an entry is done only in kotowari-brainstorm or kotowari-adopt |
| surface_unspecified_invalid | Fix the entry to the form below. Until then it excludes nothing |
| surface_unspecified_stale | The entry is no longer needed: no surface with that kind and name is taken from the code, or the surface is now in the IR. Delete the entry (or change `name` if the surface was renamed) |

surface_without_spec is raised once per kind and name, at the first place the surface appears (path in byte order, then line); detail is the kind and the name separated by one space. surface_unspecified_stale is a notice and does not change the exit code.

## The list of unspecified surfaces

A YAML sequence; each entry has exactly the three keys `kind`, `name` and `why`, all strings, and `why` must not be blank. `kind` and `name` are compared with the surface's kind and name exactly, spaces included.

```yaml
- kind: "flag"
  name: "--legacy"
  why: "Kept for old scripts; to be specified when the output topic is adopted"
```

An empty file (0 bytes or comments only), or no `surface.unspecified` key, is an empty list. A missing or unreadable list file stops with unreadable file, a non-UTF-8 one with non-UTF-8 file, and a file that is not YAML or whose top level is not a sequence with a config error naming the list file. Two identical entries are not checked.

The list is a debt, not a place to leave surfaces for good. `kotowari check` prints the number of surfaces it excluded as its last text line, `surface: unspecified=N` (the `surface.unspecified` key of the JSON), and `kotowari status` prints `surface total=… specified=… unspecified=…`. When a project first writes its surface rules, put every surface raised by surface_without_spec into the list, and take entries out topic by topic as the surfaces are written into the IR (kotowari-adopt does this).
