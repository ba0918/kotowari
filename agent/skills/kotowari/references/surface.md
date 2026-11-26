Based on the kotowari specification (revised 2026-11-26; the version of kotowari itself is not pinned)

## What the surface check does

A surface is one piece of user-visible behavior taken from the code by a rule you write: a CLI subcommand or flag, a configuration key, an HTTP route. `kotowari check` takes every surface from the code and makes each one that the decision documents do not quote a surface_without_spec error. It is how a feature built without being written into a decision is caught. A project with no surface rules sees nothing of this.

Only `kotowari check` and `kotowari status` read the surface files, the surface rule files and the list of unspecified surfaces, and only they stop on those files. `kotowari list`, `query`, `plan` and `mutants` do not read them.

## Configuration

| Key | Value |
|---|---|
| surface.files | Globs of the files surfaces are taken from. Walked like `tests.files`: the same exclusions and the same stop for a dangling symbolic link. Only the files of a surface rule's language are read, and an unreadable or non-UTF-8 one among them stops as a test file does; other files are not read, so a broad glob such as `src/**` may match images or other binary files |
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
- If the captured text starts and ends with the same quote (`'`, `"` or a backquote), one quote is removed from each end. Nothing else is changed: kotowari adds no `--` and changes no case. Write the rule so that it captures the text a user sees, and quote that same text in the decision documents.
- `language` is matched without case and accepts ast-grep's aliases (`ts`, `py`). A rule applies only to surface files whose extension gives that language.
- `files` and `ignores` apply against the path relative to the base directory. `fix`, `message`, `severity`, `note` and `metadata` are not used; a rule with `severity: off` still applies.
- Only surface files of a language some surface rule has are read and parsed. A syntax error in one raises unparsable_file (once per path, even if the file is also a test file); other surface files are not read and raise nothing, even when they are unreadable or not UTF-8.
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

These take `flag verbose` and `subcommand check` (the root `Command::new("tool")` is not inside a `subcommand` call and is not taken). The flag's name is `verbose`, not `--verbose`: the decision documents must quote `"verbose"` for it, or the rule must capture text that already reads `--verbose`. With clap's derive API the names come from field and variant names, which a rule captures as written in the code (`verbose`, `Check`); choose rules and decision wording that agree.

Check a rule before adding it with `ast-grep scan -r rules/surface.yml <file>`, then run `kotowari check`.

## When a surface is in the decision documents

A surface is specified when its name is the whole content of a pair of double quotes or of backticks in a decision document, inside the `### 決めたこと` or `### 確認すること` section (`### Decision`, `### Confirmations`) of a decision of either state, on the first-language side of a pair: each line of the text, each table cell separately, code blocks not read. The problem, the rejected options and `## 未決` are not counted, nor is the glossary.

Double quotes pair from the left of the line; backticks pair only outside double quotes. The content must equal the name exactly, spaces included: `"--verbose true"` and `" --verbose"` do not contain the surface `--verbose`.

## Findings and how to fix them

| Kind | Fix |
|---|---|
| surface_without_spec | The surface is user-visible but no decision quotes it. A behavior missing from the decision documents is a change to the specification: the implementer returns it to brainstorm and never adds an entry to the list of unspecified surfaces. In kotowari-brainstorm, write it into the decision that specifies it, quoting the name exactly, or, if it is not to be specified now, add an entry for it to the list. Adding an entry is done only in kotowari-brainstorm or kotowari-adopt |
| surface_unspecified_invalid | Fix the entry to the form below. Until then it excludes nothing |
| surface_unspecified_stale | The entry is no longer needed: no surface with that kind and name is taken from the code, or the surface is now in the decision documents. Delete the entry (or change `name` if the surface was renamed) |

surface_without_spec is raised once per kind and name, at the first place the surface appears (path in byte order, then line); detail is the kind and the name separated by one space. surface_unspecified_stale is a notice and does not change the exit code.

## The list of unspecified surfaces

A YAML sequence; each entry has exactly the three keys `kind`, `name` and `why`, all strings, and `why` must not be blank. `kind` and `name` are compared with the surface's kind and name exactly, spaces included.

```yaml
- kind: "flag"
  name: "--legacy"
  why: "Kept for old scripts; to be specified when the output topic is adopted"
```

An empty file (0 bytes or comments only), or no `surface.unspecified` key, is an empty list. A missing or unreadable list file stops with unreadable file, a non-UTF-8 one with non-UTF-8 file, and a file that is not YAML or whose top level is not a sequence with a config error naming the list file. Two identical entries are not checked.

The list is a debt, not a place to leave surfaces for good. `kotowari check` prints the number of surfaces it excluded as its last text line, `surface: unspecified=N` (the `surface.unspecified` key of the JSON), and `kotowari status` prints `surface total=… specified=… unspecified=…`. When a project first writes its surface rules, put every surface raised by surface_without_spec into the list, and take entries out topic by topic as the surfaces are written into the decision documents (kotowari-adopt does this).
