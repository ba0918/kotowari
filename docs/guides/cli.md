# The kotowari CLI (common to all commands)

English | [日本語](cli.ja.md)

<!-- @kotowari[REQ-core-001:f6b868d0] -->

This page collects the rules shared by kotowari's eight commands.
It covers the syntax, the common options, how findings are printed, the exit codes, the standard error output when kotowari stops, and the base for paths.
For the details of each command, see its own page.

## Syntax

<!-- @kotowari[REQ-core-001:f6b868d0, REQ-core-002:f86e2efd, EX-core-380:f7f73b4e] -->

```sh
kotowari <command> [--format json|text] [--config <path>] [argument]
kotowari overview build [--format json|text] [--config <path>]
kotowari overview serve [--port <port>] [--config <path>]
kotowari --help
kotowari --version
```

There are exactly these eight commands.

| Command | What it does | Page |
|---|---|---|
| `changes` | Checks, at the given phase, the correspondence and freshness between Git changes and change records | [commands/changes.md](commands/changes.md) |
| `check` | Checks both how the IR is written and how the IR corresponds to the tests | [commands/check.md](commands/check.md) |
| `list` | Lists the IR items one by one, next to the tests marked for them | [commands/list.md](commands/list.md) |
| `query` | Prints the item or scenario of one ID, with its body and back references | [commands/query.md](commands/query.md) |
| `status` | Answers whether the IR and the tests line up, with counts and one boolean | [commands/status.md](commands/status.md) |
| `mutants` | Reads a mutation testing result file and reports misses as findings | [commands/mutants.md](commands/mutants.md) |
| `plan` | Checks the form of one plan file with the bundled schema | [commands/plan.md](commands/plan.md) |
| `overview` | The subcommand `build` writes the overview pages under `.kotowari/cache/overview/`, and `serve` writes them and serves them on 127.0.0.1. When `languages` has two or more languages, the pages of every language but the first are written under `<language tag>/` | [Specification](../ir/core/overview-commands.md) |

Options can go before or after the command.
`kotowari --format text check` and `kotowari check --format text` are the same.
The order of positional arguments and options does not matter for any command either (`kotowari query REQ-001 --format text` is accepted too).

## Common options

<!-- @kotowari[REQ-core-002:f86e2efd, REQ-core-021:ccedd28b, REQ-core-003:b4f59e48, REQ-core-107:7c6198ca, REQ-core-297:481b26d5] -->

| Name | Value | Default | Description | Accepted by |
|---|---|---|---|---|
| `--format` | `json` or `text` | `json` | The output format | Everything except `overview serve` |
| `--config` | Path to a configuration file | `.kotowari/config.yaml` in the base directory | Changes which configuration file is read. Normally relative to the **current directory**. For `changes` it is relative to the Git root, and the configuration of the target snapshot is read | `check`, `list`, `query`, `status`, `mutants`, `changes`, `overview build`, `overview serve` |
| `--help` | None | — | Prints usage to standard output and exits with code 0 | All |
| `--version` | None | — | Prints the version to standard output and exits with code 0 | All |
| `--tool` | `cargo-mutants` | None (required) | The tool that produced the result file | `mutants` only |
| `--port` | A decimal integer from 1 to 65535 | `4590` | The port on 127.0.0.1 to serve on | `overview serve` only |
| `--allow-test-findings` | None (takes no value) | Off | Leaves test-side findings out of the exit code ([commands/check.md](commands/check.md#committing-a-specification-before-its-tests)) | `check` only |

`--help` and `--version` take precedence over all other arguments.
When either is present, it is shown without looking at the other arguments, even if they contain mistakes, and no check is run.
The command can be omitted (`kotowari --version` alone works).

`plan` does not read the configuration file, so it does not accept `--config`.
The configuration keys are in [config.md](config.md).

## How findings are printed

`check`, `mutants`, `plan` and `changes` print each problem they find as one "finding".
Findings have the same form in all of these commands.
What each kind means and how to fix it is in [findings.md](findings.md).

### Findings in JSON

<!-- @kotowari[REQ-core-022:8e221eee, TBL-core-006:1b834d84, REQ-core-028:1c43e818] -->

With `--format json` (the default), one JSON value is printed to standard output.
One finding is an object with these five keys.

| Key | Type | Description |
|---|---|---|
| `kind` | String | The kind of finding (such as `missing_source`). The list is in [findings.md](findings.md) |
| `severity` | String | `error` or `notice` |
| `path` | String | Path relative to the base directory. For a finding on an IR document it is the document, for a finding on a test the test file, for `surface_without_spec` the surface file, and for a finding on one entry of a list the list file |
| `line` | Number or null | The line, starting from 1. null for a finding on a whole document |
| `detail` | String | A short string fixed per kind. Usually an ID, the offending word, or the line's text as is |

The top-level keys of the JSON differ per command.
See each command's page.

### Findings in text

<!-- @kotowari[REQ-core-025:58025379, REQ-core-026:530a64e3] -->

With `--format text`, each finding is printed on one line.

```text
パス:行 [error] 種類 詳細
パス:行 [notice] 種類 詳細
```

(That is, `path:line [error] kind detail` and `path:line [notice] kind detail`.)

- The square brackets are printed literally.
- For a finding whose `line` is null, the line is written as `-` (`docs/ir/misc/notitle.md:- [error] missing_title notitle.md`).
- Line breaks inside the path and detail are replaced by the two characters `\n` and `\r`. Whatever the file name, one finding is one line.
- When there are no findings, nothing is printed.

### Order of findings

<!-- @kotowari[TBL-core-007:15954989, PROP-core-003:0d661079] -->

Findings are sorted in the following order, the same in JSON and text.

| Order | Key | How it is sorted |
|---|---|---|
| 1 | `path` | Byte order |
| 2 | `line` | null first, then ascending |
| 3 | `kind` | Byte order |
| 4 | `detail` | Byte order |

The same input always gives output in the same order.
You can diff it or compare it with the previous output.

## Exit codes

<!-- @kotowari[REQ-core-007:0f08bddc, TBL-core-002:36817bf5] -->

| Code | Meaning |
|---|---|
| 0 | No errors (including when there are only notices), every error of `check` with `--allow-test-findings` is a test-side finding, or it ended with `--help` or `--version` |
| 1 | One or more errors (for `check` with `--allow-test-findings`, one or more errors that are not test-side findings) |
| 2 | Stopped (the check could not start) |

Notices (`notice`) do not change the exit code.
In CI, the exit code alone tells you whether something needs fixing.
`status` uses its exit code, separately from this table, to say whether everything lines up ([commands/status.md](commands/status.md)).

## Stopping

<!-- @kotowari[REQ-core-005:fee48254, REQ-core-109:4d28a72d] -->

When the input cannot be read as a whole, kotowari ends with exit code 2 without checking.
This is called a "stop".

- Nothing is printed to standard output.
- The first line of standard error has the form `reason text: detail`, in English.
- When the detail includes a path, it is relative to the base directory.

Unreadable input is never skipped silently.
Anything that breaks the premise of reading a file or the configuration as a whole (including syntax errors in the list of equivalents and the list of unspecified surfaces) is a stop; anything that could be read but where a part departs from the form (such as one malformed entry in a list) is an error finding at that place.

### Reasons for stopping

<!-- @kotowari[TBL-core-018:c435229b, TBL-core-020:d87686da, TBL-core-001:0d8e4c30] -->

| Text on the first line | Reason | Detail | Typical situations |
|---|---|---|---|
| `argument error` | Argument error | An explanatory sentence and the offending argument | Unknown option, unknown value for `--format`, option without a value, second occurrence of the same option, extra positional argument, `--config` target missing or a directory, what follows `overview` is not exactly one of `build` or `serve`, `--port` value not an integer from 1 to 65535 |
| `config error` | Configuration error | The relative path of the configuration file and a description of the error (for errors in the rule files of `tests.rules` and `surface.rules`, the list of equivalents, or the list of unspecified surfaces, the relative path of that file; for overlapping locations, the overlapping file and `: matched by both ...`; when running `overview build` or `serve` with no `overview` key, only `overview is not configured`) | The configuration file cannot be read as YAML, unknown key, second occurrence of the same key, value of the wrong type, element that cannot be read as a glob, only one of `surface.files` and `surface.rules`, a rule file of `tests.rules` or `surface.rules` missing or unreadable, the list of unspecified surfaces cannot be read as YAML or its top level is not a sequence, `overview.files` overlaps the guide or test locations, `overview` has no `toc`, the table of contents file is among the files read by the scan of `overview.files`, `guides.files` or `tests.files`, `overview build` or `serve` run with no `overview` key |
| `unreadable file` | Unreadable file | The relative path and the OS error message | A location directory does not exist, a file or directory cannot be read, the target of `surface.unspecified` or `overview.toc` does not exist |
| `non-UTF-8 file` | File not in UTF-8 | The relative path | One of the files read is not UTF-8 (of the surface files, only those in a surface rule's language are read) |
| `results error` | Results error | The relative path of the result file and a description of the error | The result file for `mutants` is malformed |
| `git error` | Git read stopped | A description of the history, target or index that could not be read | Git is missing, REV does not resolve to a commit, the configuration is not in the target, a conflicted index, an unsupported target |
| `mapping error` | Mapping error | A description of the finding kind or value that could not be mapped | An internal inconsistency in kotowari. Not expected to happen from user input |
| `overview error` | Overview data error | The number of errors and ` errors in overview data; run kotowari check` | In `overview build` or `serve`, the overview data or the table of contents has errors, or a pair of the IR, the overview data or the table of contents has `translation_missing` or `translation_structure_mismatch`. Nothing is written |
| `port error` | Port error | `127.0.0.1:<port>: ` and the OS error message | In `overview serve`, the given port cannot be used (no other port is tried), or accepting a connection failed while serving |
| `cache error` | Cache location error | The relative path of the problem path, followed by `: ` and the OS error message if there is one | In `overview build` or `serve`, one of `.kotowari`, `.kotowari/cache`, `.kotowari/cache/overview`, or (with two or more languages) `.kotowari/cache/overview/<language tag>` for another language is a symbolic link or a file that is not a directory (nothing is written or removed), or creating, writing to or removing from the location failed |

All the situations are in [TBL-core-001 of cli.md](../ir/core/cli.md) and [TBL-core-020 of cli-environment.md](../ir/core/cli-environment.md).

### Argument errors

<!-- @kotowari[REQ-core-004:2d3401d1, EX-core-219:f3123493, EX-core-241:38290f77, REQ-core-304:e9cd623d] -->

kotowari stops with `argument error` when any of the following applies (unless `--help` or `--version` is present).

- There are no arguments at all, or only options and no command
- The first positional argument is not one of the eight commands
- An unknown option, `--tool` on a command other than `mutants`, `--port` on a command other than `overview serve`, `--format` on `overview serve`, or `--allow-test-findings` on a command other than `check`
- After `overview` there is not exactly one positional argument, or it is not `build` or `serve`
- The value of `--port` is not a decimal integer from 1 to 65535
- A positional argument after `check`, `list` or `status`
- An unknown value for `--format`, an option without a value, or a second occurrence of the same option
- The target of `--config` does not exist or is a directory

The argument rules of `mutants` are in [commands/mutants.md](commands/mutants.md).

## The base directory

<!-- @kotowari[REQ-core-009:6b5a71a4, TBL-core-003:603e9601, PROP-core-001:9be33697, REQ-core-010:5217a2ba] -->

The relative paths kotowari handles have exactly one base.
That is the "base directory".

| Order | Condition | Base directory |
|---|---|---|
| 1 | Searching upward from the current directory for a `.kotowari/` directory finds one | The first directory found |
| 2 | None is found | The current directory |

A **file** named `.kotowari` is ignored, and the search continues upward.

These three are relative to the base directory:

- Paths written in the configuration file (`ir`, `tests.files` and so on)
- The paths of IR sources (`- source:` lines)
- The `path` in the output, and paths in the detail of a stop

`--config` only changes which configuration file is read; it does not change the base.
The `--config` path itself is read relative to the current directory.

### Path normalization

<!-- @kotowari[REQ-core-110:04555fb0] -->

Paths in configuration values and in sources are tidied as follows before they are compared and before they are printed.

- A trailing `/` and a leading `./` are removed
- `/./` in the middle and runs of `/` become a single `/`
- `\` becomes `/`
- `a/..` is folded (a `..` with nothing to fold into is kept)

So `docs/decision/../decision/records` and `docs/decision/records` are treated as the same location.

## Target environments

<!-- @kotowari[REQ-core-108:8e9073d4] -->

kotowari targets Linux and macOS.
On Windows it only converts the path separator `\` to `/`; nothing else about its behavior is promised.

## Examples

The examples were run in a small repository with a `.kotowari/config.yaml`.

### Showing usage and the version

<!-- @kotowari[REQ-core-107:7c6198ca] -->

```console
$ kotowari --version
kotowari 0.1.0
$ kotowari --help
Usage: kotowari [OPTIONS] <COMMAND> [ARGUMENT]

Commands:
  changes    Check change records against a Git base and target snapshot
  check      Check IR documents and test markers
  list       List IR items and the tests marked for them
  mutants    Read a mutation testing result file and report survivors
  overview   build: write the overview pages; serve: write and show them locally
  plan       Check the form of one plan file against the bundled schema
  query      Show one item or scenario with its body and back references
  status     Summarise the IR and tell whether it is complete

Changes: --base <REV> (--head <REV> | --staged) --phase <implementation|review>
Options:
  --format <FORMAT>      Output format: json (default) or text
  --config <PATH>        Path to configuration file
  --tool <TOOL>          Mutation testing tool of the result file: cargo-mutants
  --port <PORT>          Port of overview serve on 127.0.0.1 (default 4590)
  --allow-test-findings  check: exit 0 when every error is a test-side finding
  --help                 Show this help message
  --version              Show version
```

### Stopping on a wrong argument

<!-- @kotowari[REQ-core-004:2d3401d1, REQ-core-005:fee48254, EX-core-219:f3123493] -->

```console
$ kotowari
argument error: expected command: check, changes, list, mutants, overview, plan, query or status
$ kotowari check --verbose
argument error: unknown option: --verbose
$ kotowari check --format xml
argument error: unknown format: xml
$ kotowari check extra
argument error: unexpected argument: extra
$ kotowari check --format text --format json
argument error: repeated option: --format
$ kotowari status --allow-test-findings
argument error: unexpected option for status: --allow-test-findings
$ kotowari check --format
argument error: --format requires a value
$ kotowari check --config docs
argument error: --config is a directory: docs
$ echo $?
2
```

### Stopping on file and configuration problems

<!-- @kotowari[TBL-core-018:c435229b, TBL-core-020:d87686da] -->

In the example below, the comments mean, in order: `docs/decision/adr` does not exist; `docs/ir/greet/bad.md` is not UTF-8; `limits` was mistyped as `limit`; the `language` of `rules/c.yml` is `cobol`.

```console
$ kotowari check --format text      # docs/decision/adr が無い
unreadable file: docs/decision/adr: No such file or directory (os error 2)
$ kotowari check --format text      # docs/ir/greet/bad.md が UTF-8 でない
non-UTF-8 file: docs/ir/greet/bad.md
$ kotowari check --config bad.yaml  # limits を limit と書き間違えた
config error: bad.yaml: error: line 2 column 1: unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, changes, overview, vague_words, languages, labels
 --> <input>:2:1
  |
1 | ir: docs/ir
2 | limit:
  | ^ unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, changes, overview, vague_words, languages, labels
3 |   lines: 20
  |
$ kotowari check --format text      # rules/c.yml の language が cobol
config error: invalid rule in tests.rules: rules/c.yml: unknown language: cobol
```

For configuration errors that come from reading the YAML (unknown keys, wrong types and so on), an excerpt of the relevant place follows the first line.
A second occurrence of the same key, and an unknown language in a `tests.rules` rule, give only one line.
The detail of an error in `tests.rules` points at the rule file, not the configuration file.
Only the form of the first line is fixed.

### Running from a subdirectory

<!-- @kotowari[TBL-core-003:603e9601, PROP-core-001:9be33697, REQ-core-003:b4f59e48] -->

Even when you run it in `tests/` below the directory that has `.kotowari/`, the base is the directory above that has `.kotowari/`.
Paths in the output are relative to the base, not to `tests/`.

```console
$ cd tests
$ kotowari check --format text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
…
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

The `--config` path, on the other hand, is relative to the current directory.
To point at the base's configuration file from `tests/`, you need `../`.

```console
$ kotowari check --config .kotowari/config.yaml
argument error: config file not found: .kotowari/config.yaml
$ kotowari --format text check --config ../bad.yaml
config error: bad.yaml: error: line 2 column 1: unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, changes, overview, vague_words, languages, labels
…
```

The `bad.yaml` in the second detail is written as a path relative to the base directory.

## Common pitfalls

### Exit code 2 and nothing on standard output

<!-- @kotowari[REQ-core-005:fee48254, TBL-core-018:c435229b] -->

It stopped before starting the check.
Look at the first line of standard error, not standard output.
The text at the start of that line (such as `argument error`) tells you the reason.
When you pipe the JSON into `jq`, `jq` gets empty input and prints nothing, which makes this easy to miss.

### `config file not found` even though you passed `--config`

<!-- @kotowari[REQ-core-003:b4f59e48, PROP-core-001:9be33697] -->

The `--config` path is read relative to the current directory, not the base directory.
If you are running in a subdirectory, add `../` or use an absolute path.

### A `.kotowari/` somewhere else has become the base

<!-- @kotowari[REQ-core-009:6b5a71a4, TBL-core-003:603e9601] -->

The base directory is the first `.kotowari/` found searching upward from the current directory.
With nested repositories, or a `.kotowari/` in your home directory, an unexpected place can become the base.
If the `path` in the output does not look the way you expect, check for a `.kotowari/` in the directories above.

### Notices (`[notice]`) appear but the exit code is 0

<!-- @kotowari[TBL-core-002:36817bf5] -->

This is as specified.
Notices do not change the exit code.
If you want notices to fail CI, look at `counts` in the JSON with `jq` and decide from that.

## Related

- Specification: [Commands and exit](../ir/core/cli.md), [Usage display and target environments](../ir/core/cli-environment.md), [Base directory](../ir/core/base-directory.md), [Output format](../ir/core/output.md), [Finding order and lines](../ir/core/finding-order.md)
- Configuration keys: [config.md](config.md)
- Finding kinds: [findings.md](findings.md)
- Each command: [check](commands/check.md), [list](commands/list.md), [query](commands/query.md), [status](commands/status.md), [mutants](commands/mutants.md), [plan](commands/plan.md)

The base for `changes` is the root of the Git working tree that contains the starting location. It reads the target's configuration, IR, decision records and change records together from the commit or the index, and does not fill anything in from the working tree. For the comparison and the required phase arguments, see [changes](commands/changes.md).
