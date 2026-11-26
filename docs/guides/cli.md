# The kotowari CLI (common to all commands)

English | [日本語](cli.ja.md)

<!-- @kotowari[docs/decisions/core/cli.md#kotowari-has-seven-commands-and-the-single-command-check-does-both-the-check-of-the-specification-and-the-check-of-the-correspondence-with-tests:d77a0bd7] -->

This page collects the rules shared by kotowari's seven commands.
It covers the syntax, the common options, how findings are printed, the exit codes, the standard error output when kotowari stops, and the base for paths.
For the details of each command, see its own page.

## Syntax

<!-- @kotowari[docs/decisions/core/cli.md#kotowari-has-seven-commands-and-the-single-command-check-does-both-the-check-of-the-specification-and-the-check-of-the-correspondence-with-tests:d77a0bd7, docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd] -->

```sh
kotowari <command> [--format json|text] [--config <path>] [argument]
kotowari overview build [--format json|text] [--config <path>]
kotowari overview serve [--port <port>] [--config <path>]
kotowari --help
kotowari --version
```

There are exactly these seven commands.

| Command | What it does | Page |
|---|---|---|
| `check` | Checks both how the decision documents are written and how their confirmation rows correspond to the tests | [commands/check.md](commands/check.md) |
| `list` | Lists the decisions and confirmation rows one by one, next to the tests marked for each row | [commands/list.md](commands/list.md) |
| `query` | Prints the decision or row of one ID, with its body and back references | [commands/query.md](commands/query.md) |
| `status` | Answers whether the decision documents and the tests line up, with counts and one boolean | [commands/status.md](commands/status.md) |
| `mutants` | Reads a mutation testing result file and reports misses as findings | [commands/mutants.md](commands/mutants.md) |
| `plan` | Checks the form of one plan file with the bundled schema | [commands/plan.md](commands/plan.md) |
| `overview` | The subcommand `build` writes the overview pages under `.kotowari/cache/overview/`, and `serve` writes them and serves them on 127.0.0.1. When `languages` has two or more languages, the pages of every language but the first are written under `<language tag>/` | [Specification](../decisions/core/overview-commands.md) |

Options can go before or after the command.
`kotowari --format text check` and `kotowari check --format text` are the same.
The order of positional arguments and options does not matter for any command either (`kotowari query C-shop-discount-1 --format text` is accepted too).

## Common options

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/output.md#--format-accepts-json-and-text-with-json-as-the-default:811394a9, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47, docs/decisions/core/cli-environment.md#--help-and---version-are-taken-before-the-other-arguments-and-kotowari-outputs-the-usage-or-the-version-and-ends:f100d8c8, docs/decisions/core/overview-commands.md#overview-serve-writes-as-build-does-when-it-starts-and-serves-the-files-of-the-location-on-one-port-of-127001:dbf5bfdb] -->

| Name | Value | Default | Description | Accepted by |
|---|---|---|---|---|
| `--format` | `json` or `text` | `json` | The output format | Everything except `overview serve` |
| `--config` | Path to a configuration file | `.kotowari/config.yaml` in the base directory | Changes which configuration file is read. Relative to the **current directory** | `check`, `list`, `query`, `status`, `mutants`, `overview build`, `overview serve` |
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

`check`, `mutants` and `plan` print each problem they find as one "finding".
Findings have the same form in all of these commands.
What each kind means and how to fix it is in [findings.md](findings.md).

### Findings in JSON

<!-- @kotowari[docs/decisions/core/output.md#--format-accepts-json-and-text-with-json-as-the-default:811394a9, docs/decisions/core/output.md#a-finding-is-output-with-the-five-keys-kind-severity-path-line-and-detail:ab004221, docs/decisions/core/finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null:b98176aa] -->

With `--format json` (the default), one JSON value is printed to standard output.
One finding is an object with these five keys.

| Key | Type | Description |
|---|---|---|
| `kind` | String | The kind of finding (such as `confirmation_without_test`). The list is in [findings.md](findings.md) |
| `severity` | String | `error` or `notice` |
| `path` | String | Path relative to the base directory. For a finding on a decision document it is the document, for a finding on a test the test file, for `surface_without_spec` the surface file, and for a finding on one entry of a list the list file |
| `line` | Number or null | The line, starting from 1. null for a finding on a whole document |
| `detail` | String | A short string fixed per kind. Usually an ID, the offending word, or the line's text as is |

The top-level keys of the JSON differ per command.
See each command's page.

### Findings in text

<!-- @kotowari[docs/decisions/core/output.md#--format-text-outputs-one-finding-per-line:b093e38d] -->

With `--format text`, each finding is printed on one line.

```text
パス:行 [error] 種類 詳細
パス:行 [notice] 種類 詳細
```

(That is, `path:line [error] kind detail` and `path:line [notice] kind detail`.)

- The square brackets are printed literally.
- For a finding whose `line` is null, the line is written as `-` (`docs/decisions/shop/notitle.md:- [error] missing_title notitle.md`).
- Line breaks inside the path and detail are replaced by the two characters `\n` and `\r`. Whatever the file name, one finding is one line.
- When there are no findings, nothing is printed.

### Order of findings

<!-- @kotowari[docs/decisions/core/finding-order.md#findings-are-arranged-by-path-line-kind-and-detail-in-that-order:430be622] -->

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

<!-- @kotowari[docs/decisions/core/cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops:3dc67d73] -->

| Code | Meaning |
|---|---|
| 0 | No errors (including when there are only notices), every error of `check` with `--allow-test-findings` is a test-side finding, or it ended with `--help` or `--version` |
| 1 | One or more errors (for `check` with `--allow-test-findings`, one or more errors that are not test-side findings) |
| 2 | Stopped (the check could not start) |

Notices (`notice`) do not change the exit code.
In CI, the exit code alone tells you whether something needs fixing.
`status` uses its exit code, separately from this table, to say whether everything lines up ([commands/status.md](commands/status.md)).

## Stopping

<!-- @kotowari[docs/decisions/core/cli.md#when-kotowari-stops-it-outputs-nothing-to-standard-output-and-outputs-the-reason-and-the-details-on-the-first-line-of-standard-error:90b03313, docs/decisions/core/cli-environment.md#unreadable-input-and-broken-input-become-a-stop-or-an-error-finding-and-are-not-silently-skipped:f1577593] -->

When the input cannot be read as a whole, kotowari ends with exit code 2 without checking.
This is called a "stop".

- Nothing is printed to standard output.
- The first line of standard error has the form `reason text: detail`, in English.
- When the detail includes a path, it is relative to the base directory.

Unreadable input is never skipped silently.
Anything that breaks the premise of reading a file or the configuration as a whole (including syntax errors in the list of equivalents and the list of unspecified surfaces) is a stop; anything that could be read but where a part departs from the form (such as one malformed entry in a list) is an error finding at that place.

### Reasons for stopping

<!-- @kotowari[docs/decisions/core/cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording:383d0fd9, docs/decisions/core/cli-environment.md#the-details-of-a-stop-show-the-file-the-person-fixing-it-should-look-at-as-a-path-relative-to-the-base-directory:a832b365, docs/decisions/core/cli.md#when-kotowari-stops-it-outputs-nothing-to-standard-output-and-outputs-the-reason-and-the-details-on-the-first-line-of-standard-error:90b03313] -->

| Text on the first line | Reason | Detail | Typical situations |
|---|---|---|---|
| `argument error` | Argument error | An explanatory sentence and the offending argument | Unknown option, unknown value for `--format`, option without a value, second occurrence of the same option, extra positional argument, `--config` target missing or a directory, what follows `overview` is not exactly one of `build` or `serve`, `--port` value not an integer from 1 to 65535 |
| `config error` | Configuration error | The relative path of the configuration file and a description of the error (for errors in the rule files of `tests.rules` and `surface.rules`, the list of equivalents, or the list of unspecified surfaces, the relative path of that file; for overlapping locations, the overlapping file and `: matched by both ...`; when running `overview build` or `serve` with no `overview` key, only `overview is not configured`) | The configuration file cannot be read as YAML, unknown key, second occurrence of the same key, value of the wrong type, element that cannot be read as a glob, only one of `surface.files` and `surface.rules`, a rule file of `tests.rules` or `surface.rules` missing or unreadable, the list of unspecified surfaces cannot be read as YAML or its top level is not a sequence, `overview.files` overlaps the guide or test locations, `overview` has no `toc`, the table of contents file is among the files read by the scan of `overview.files`, `guides.files` or `tests.files`, `overview build` or `serve` run with no `overview` key |
| `unreadable file` | Unreadable file | The relative path and the OS error message | A location directory does not exist, a file or directory cannot be read, the target of `surface.unspecified` or `overview.toc` does not exist |
| `non-UTF-8 file` | File not in UTF-8 | The relative path | One of the files read is not UTF-8 (of the surface files, only those in a surface rule's language are read) |
| `results error` | Results error | The relative path of the result file and a description of the error | The result file for `mutants` is malformed |
| `mapping error` | Mapping error | A description of the finding kind or value that could not be mapped | An internal inconsistency in kotowari. Not expected to happen from user input |
| `overview error` | Overview data error | The number of errors and ` errors in overview data; run kotowari check` | In `overview build` or `serve`, the overview data or the table of contents has errors, or a pair of the overview data or the table of contents has `translation_missing` or `translation_structure_mismatch`. Nothing is written |
| `port error` | Port error | `127.0.0.1:<port>: ` and the OS error message | In `overview serve`, the given port cannot be used (no other port is tried), or accepting a connection failed while serving |
| `cache error` | Cache location error | The relative path of the problem path, followed by `: ` and the OS error message if there is one | In `overview build` or `serve`, one of `.kotowari`, `.kotowari/cache`, `.kotowari/cache/overview`, or (with two or more languages) `.kotowari/cache/overview/<language tag>` for another language is a symbolic link or a file that is not a directory (nothing is written or removed), or creating, writing to or removing from the location failed |

All the situations are in [the CLI decisions](../decisions/core/cli.md) and [the decisions on the usage display and target environments](../decisions/core/cli-environment.md).

### Argument errors

<!-- @kotowari[docs/decisions/core/cli.md#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error:11a07c33] -->

kotowari stops with `argument error` when any of the following applies (unless `--help` or `--version` is present).

- There are no arguments at all, or only options and no command
- The first positional argument is not one of the seven commands
- An unknown option, `--tool` on a command other than `mutants`, `--port` on a command other than `overview serve`, `--format` on `overview serve`, or `--allow-test-findings` on a command other than `check`
- After `overview` there is not exactly one positional argument, or it is not `build` or `serve`
- The value of `--port` is not a decimal integer from 1 to 65535
- A positional argument after `check`, `list` or `status`
- An unknown value for `--format`, an option without a value, or a second occurrence of the same option
- The target of `--config` does not exist or is a directory

The argument rules of `mutants` are in [commands/mutants.md](commands/mutants.md).

## The base directory

<!-- @kotowari[docs/decisions/core/base-directory.md#the-base-of-relative-paths-is-the-first-directory-with-kotowari-found-by-searching-upward:e421eeba, docs/decisions/core/base-directory.md#paths-in-configuration-values-and-the-path-of-the-output-are-relative-to-the-base-directory:712ec2ff] -->

The relative paths kotowari handles have exactly one base.
That is the "base directory".

| Order | Condition | Base directory |
|---|---|---|
| 1 | Searching upward from the current directory for a `.kotowari/` directory finds one | The first directory found |
| 2 | None is found | The current directory |

A **file** named `.kotowari` is ignored, and the search continues upward.

These three are relative to the base directory:

- Paths written in the configuration file (`decisions.documents`, `tests.files` and so on)
- The `path` in the output, and paths in the detail of a stop

`--config` only changes which configuration file is read; it does not change the base.
The `--config` path itself is read relative to the current directory.

### Path normalization

<!-- @kotowari[docs/decisions/core/base-directory.md#paths-in-configuration-values-are-normalized-before-comparing-and-before-output:a1dd7f03] -->

Paths in configuration values are tidied as follows before they are compared and before they are printed.

- A trailing `/` and a leading `./` are removed
- `/./` in the middle and runs of `/` become a single `/`
- `\` becomes `/`
- `a/..` is folded (a `..` with nothing to fold into is kept)

So `docs/guides/../decisions` and `docs/decisions` are treated as the same location.

## Target environments

<!-- @kotowari[docs/decisions/core/cli-environment.md#the-target-operating-systems-are-linux-and-macos-and-on-windows-only-the-normalization-of-path-separators-is-promised:03fcd5ae] -->

kotowari targets Linux and macOS.
On Windows it only converts the path separator `\` to `/`; nothing else about its behavior is promised.

## Examples

The examples were run in a small repository with a `.kotowari/config.yaml`.

### Showing usage and the version

<!-- @kotowari[docs/decisions/core/cli-environment.md#--help-and---version-are-taken-before-the-other-arguments-and-kotowari-outputs-the-usage-or-the-version-and-ends:f100d8c8] -->

```console
$ kotowari --version
kotowari 0.1.0
$ kotowari --help
Usage: kotowari [OPTIONS] <COMMAND> [ARGUMENT]

Commands:
  check      Check decision documents and test markers
  list       List decisions, confirmation rows and the tests marked for them
  mutants    Read a mutation testing result file and report survivors
  overview   build: write the overview pages; serve: write and show them locally
  plan       Check the form of one plan file against the bundled schema
  query      Show one decision or confirmation row with its body and back references
  status     Summarise the decision documents and tell whether they are complete

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

<!-- @kotowari[docs/decisions/core/cli.md#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error:11a07c33, docs/decisions/core/cli.md#when-kotowari-stops-it-outputs-nothing-to-standard-output-and-outputs-the-reason-and-the-details-on-the-first-line-of-standard-error:90b03313] -->

```console
$ kotowari
argument error: expected command: check, list, mutants, overview, plan, query or status
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
argument error: config file is a directory: docs
$ echo $?
2
```

### Stopping on file and configuration problems

<!-- @kotowari[docs/decisions/core/cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording:383d0fd9, docs/decisions/core/cli-environment.md#the-details-of-a-stop-show-the-file-the-person-fixing-it-should-look-at-as-a-path-relative-to-the-base-directory:a832b365] -->

In the example below, the comments mean, in order: `docs/decisions` does not exist; `docs/decisions/greet/bad.md` is not UTF-8; `files` under `tests` was mistyped as `file`; the `language` of `rules/c.yml` is `cobol`.

```console
$ kotowari check --format text      # docs/decisions が無い
unreadable file: docs/decisions: No such file or directory (os error 2)
$ kotowari check --format text      # docs/decisions/greet/bad.md が UTF-8 でない
non-UTF-8 file: docs/decisions/greet/bad.md
$ kotowari check --config bad.yaml  # tests の files を file と書き間違えた
config error: bad.yaml: error: line 2 column 3: unknown field `file`, expected one of files, rust, rules
 --> <input>:2:3
  |
1 | tests:
2 |   file: ["tests/**/*.rs"]
  |   ^ unknown field `file`, expected one of files, rust, rules
$ kotowari check --format text      # rules/c.yml の language が cobol
config error: invalid rule in tests.rules: rules/c.yml: unknown language: cobol
```

For configuration errors that come from reading the YAML (unknown keys, wrong types and so on), an excerpt of the relevant place follows the first line.
A second occurrence of the same key, and an unknown language in a `tests.rules` rule, give only one line.
The detail of an error in `tests.rules` points at the rule file, not the configuration file.
Only the form of the first line is fixed.

### Running from a subdirectory

<!-- @kotowari[docs/decisions/core/base-directory.md#the-base-of-relative-paths-is-the-first-directory-with-kotowari-found-by-searching-upward:e421eeba, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

Even when you run it in `tests/` below the directory that has `.kotowari/`, the base is the directory above that has `.kotowari/`.
Paths in the output are relative to the base, not to `tests/`.

```console
$ cd tests
$ kotowari check --format text
docs/decisions/greet/greet.md:35 [error] confirmation_without_test C-greet-greet-2
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

The `--config` path, on the other hand, is relative to the current directory.
To point at the base's configuration file from `tests/`, you need `../`.

```console
$ kotowari check --config .kotowari/config.yaml
argument error: config file not found: .kotowari/config.yaml
$ kotowari --format text check --config ../bad.yaml
config error: bad.yaml: error: line 2 column 3: unknown field `file`, expected one of files, rust, rules
…
```

The `bad.yaml` in the second detail is written as a path relative to the base directory.

## Common pitfalls

### Exit code 2 and nothing on standard output

<!-- @kotowari[docs/decisions/core/cli.md#when-kotowari-stops-it-outputs-nothing-to-standard-output-and-outputs-the-reason-and-the-details-on-the-first-line-of-standard-error:90b03313, docs/decisions/core/cli-environment.md#the-reason-for-a-stop-is-shown-with-fixed-english-wording:383d0fd9] -->

It stopped before starting the check.
Look at the first line of standard error, not standard output.
The text at the start of that line (such as `argument error`) tells you the reason.
When you pipe the JSON into `jq`, `jq` gets empty input and prints nothing, which makes this easy to miss.

### `config file not found` even though you passed `--config`

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47, docs/decisions/core/base-directory.md#the-base-of-relative-paths-is-the-first-directory-with-kotowari-found-by-searching-upward:e421eeba] -->

The `--config` path is read relative to the current directory, not the base directory.
If you are running in a subdirectory, add `../` or use an absolute path.

### A `.kotowari/` somewhere else has become the base

<!-- @kotowari[docs/decisions/core/base-directory.md#the-base-of-relative-paths-is-the-first-directory-with-kotowari-found-by-searching-upward:e421eeba] -->

The base directory is the first `.kotowari/` found searching upward from the current directory.
With nested repositories, or a `.kotowari/` in your home directory, an unexpected place can become the base.
If the `path` in the output does not look the way you expect, check for a `.kotowari/` in the directories above.

### Notices (`[notice]`) appear but the exit code is 0

<!-- @kotowari[docs/decisions/core/cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops:3dc67d73] -->

This is as specified.
Notices do not change the exit code.
If you want notices to fail CI, look at `counts` in the JSON with `jq` and decide from that.

## Related

- Specification: [Commands and exit](../decisions/core/cli.md), [Usage display and target environments](../decisions/core/cli-environment.md), [Base directory](../decisions/core/base-directory.md), [Output format](../decisions/core/output.md), [Finding order and lines](../decisions/core/finding-order.md)
- Configuration keys: [config.md](config.md)
- Finding kinds: [findings.md](findings.md)
- Each command: [check](commands/check.md), [list](commands/list.md), [query](commands/query.md), [status](commands/status.md), [mutants](commands/mutants.md), [plan](commands/plan.md)
