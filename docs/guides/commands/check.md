# kotowari check

English | [日本語](check.ja.md)

<!-- @kotowari[docs/decisions/core/cli.md#kotowari-has-seven-commands-and-the-single-command-check-does-both-the-check-of-the-specification-and-the-check-of-the-correspondence-with-tests:d77a0bd7] -->

This command reports errors in how the decision documents (the specification) are written, and gaps in the correspondence between their confirmation rows and the tests, as one finding each.
Run it after writing decision documents or tests, fix the findings, and repeat until there are zero errors.
This single command does both: it checks the form of the decision documents and checks their correspondence with the tests.

## Synopsis

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd] -->

```sh
kotowari check [--format json|text] [--config <path>] [--allow-test-findings]
```

It takes no positional arguments.

## Options and arguments

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/output.md#--format-accepts-json-and-text-with-json-as-the-default:811394a9, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47, docs/decisions/core/cli.md#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error:11a07c33] -->

| Name | Value | Default | Description |
|---|---|---|---|
| `--format` | `json` or `text` | `json` | The output format |
| `--config` | Path to a configuration file | `.kotowari/config.yaml` in the base directory | The configuration file to read. The path is read relative to the current directory |
| `--allow-test-findings` | None (takes no value) | Off | Leaves test-side findings out of the exit code ([Committing a specification before its tests](#committing-a-specification-before-its-tests)). Only `check` takes it; on any other command it stops with `argument error` |
| `--help` | None | — | Prints usage and exits with code 0 |
| `--version` | None | — | Prints the version and exits with code 0 |

The rules shared by all options (for example, that they can be written before or after the command) are in [cli.md](../cli.md#common-options).

## What it reads

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-keys-are-only-those-in-the-table-and-a-key-not-written-takes-the-default-value-in-the-table:122a3403, docs/decisions/core/config.md#the-location-of-decision-documents-is-pointed-at-by-decisionsdocuments-with-docsdecisions-as-the-default-and-the-keys-for-the-locations-of-the-old-form-are-removed:4a2eacb8, docs/decisions/core/overview-toc.md#how-the-index-is-bundled-and-ordered-is-written-in-the-one-table-of-contents-file-that-overviewtoc-points-at:0883f676, docs/decisions/core/translation-pairs.md#guides-overview-source-data-and-the-table-of-contents-have-a-side-for-every-language-of-the-language-list:a581e9d7, docs/decisions/core/translation-pairs.md#the-sides-of-a-pair-are-told-apart-by-file-names-and-configuration-only:3c4056af] -->

`check` reads the following from the locations the configuration file points to.

| What it reads | Configuration key that sets the location | Default |
|---|---|---|
| Decision documents (the glossary `CONTEXT.md` of each group is not read as one) | `decisions.documents` | `docs/decisions` |
| Test files | `tests.files` | `src/**/*.rs`, `tests/**/*.rs` |
| Guides | `guides.files` | Empty (not read) |
| Surface files and surface rule files | `surface.files`, `surface.rules` | Empty (not read) |
| List of unspecified surfaces | `surface.unspecified` | None (zero entries) |
| Overview data | `overview.files` | None (not read when there is no `overview` key) |
| Overview table of contents | `overview.toc` | None (not read when there is no `overview` key; when the key exists it is required, and kotowari stops if the target is missing or unreadable) |

The three surface inputs are read only when `surface.rules` is not an empty list ([Surface checks](../surface.md)).
When `languages` lists two or more languages, kotowari also reads, for decision documents, guides, overview data and table of contents, the other-language sides (`foo.ja.md`) and the consistency records (`foo.i18n.yaml`), finding them by name in the same directory as the first-language side ([Languages and pairs](../config.md#languages-and-pairs--languages-and-labels)).
All the keys are in [config.md](../config.md).

### The configuration file

<!-- @kotowari[docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47, docs/decisions/core/config.md#when-the-configuration-file-is-missing-or-empty-kotowari-runs-with-the-default-values:d78fb03e] -->

- Without `--config`, kotowari reads `.kotowari/config.yaml` in the base directory.
- If that file does not exist, everything is checked with default values.
- If the configuration file is empty (zero bytes, or comments only), default values are used, even for a file named with `--config`.

How the base directory is determined is described in [cli.md](../cli.md#the-base-directory).

### When a location is missing

<!-- @kotowari[docs/decisions/core/config.md#if-the-location-of-decision-documents-does-not-exist-is-not-a-directory-or-cannot-be-read-kotowari-stops-with-unreadable-file:a2064f46, docs/decisions/core/config.md#the-glob--is-read-recursively-and-hidden-directories-are-entered-only-when-named-by-a-path-component:6f67f22b, docs/decisions/core/coverage.md#the-check-of-tests-is-performed-even-when-there-is-no-decision-document-at-all:8dedcbac] -->

The location of decision documents (`decisions.documents`) must exist as a directory, even when left at its default.
If it is missing, is not a directory, or cannot be read, kotowari stops with `unreadable file`.
An empty directory is fine: with no decision documents, the tests are still checked, and tests without marks are reported.

`tests.files`, `guides.files`, `surface.files` and `overview.files` are lists of globs.
A glob that matches nothing is not an error.
`**` is read as recursive.
Hidden directories are not included even when a glob names them, and symbolic links to directories are not followed.
As an exception, `overview.files` for overview data does read hidden directories named explicitly in a path component (such as `.kotowari/overview/*.md`). Hidden directories that are not named are excluded even by a broad `**`, and a hidden directory named only inside braces (`{.overview,other}/*.md`) does not count as named.

If a directory cannot be read during the scan, or a dangling symbolic link is encountered, kotowari stops with `unreadable file`.

### Encoding and unclosed code blocks

<!-- @kotowari[docs/decisions/core/ir-input.md#a-leading-utf-8-bom-is-skipped-and-is-not-a-stop-as-a-file-that-is-not-utf-8:52a6c07c, docs/decisions/core/ir-input.md#an-unclosed-code-block-is-an-unclosed_code_block-error-and-from-its-start-to-the-end-of-the-document-is-not-read:c1db707e] -->

Files that are read must be UTF-8.
A leading BOM is skipped, so UTF-8 with a BOM does not cause a stop.
If there is a file that is not UTF-8, kotowari stops with `non-UTF-8 file`.

If a code block in a decision document is never closed before the document ends, an `unclosed_code_block` error is reported on its opening line.
In that case, the part from the opening line to the end of the document is not read as decisions or rows.

## Output

### text

<!-- @kotowari[docs/decisions/core/output.md#--format-text-outputs-one-finding-per-line:b093e38d] -->

Each line is one finding.
If there are no errors and no notices, nothing is printed.

```text
パス:行 [error] 種類 詳細
docs/decisions/greet/greet.md:31 [error] vague_word など
guides/greet.md:5 [notice] guide_stale docs/decisions/greet/greet.md#the-command-greets-the-name-it-is-given 00000000 b08a343a
```

| Part | Content |
|---|---|
| Path (パス) | Path relative to the base directory |
| Line (行) | 1-based line number. `-` for a finding about the whole document |
| `[error]` / `[notice]` | Whether it is an error or a notice |
| Kind (種類) | The kind of finding. Its meaning and how to fix it are in [findings.md](../findings.md) |
| Detail (詳細) | A short string whose content is fixed per kind |

The text output does not include `tests`, `guides`, `overview` or `decisions` from the JSON below.

When the configuration's `surface.rules` is not an empty list, a final line after the finding lines gives the number of surfaces excluded by the list of unspecified surfaces, in the form `surface: unspecified=数` (数 being the count).
It is printed even when there are zero findings and even when the count is 0 ([Surface checks](../surface.md#seeing-how-many-are-excluded)).

### JSON

<!-- @kotowari[docs/decisions/core/output.md#--format-accepts-json-and-text-with-json-as-the-default:811394a9, docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7, docs/decisions/core/surface.md#the-number-of-surfaces-excluded-by-the-list-of-unspecified-surfaces-is-always-output-at-the-end-of-the-output-of-check:d8dacbf9, docs/decisions/core/overview-output.md#the-json-of-check-carries-an-overview-group-of-the-same-form-as-guides:c1d88d3a] -->

This is the default output.
It prints a single JSON object to standard output.

| Key | Type | Description |
|---|---|---|
| `findings` | array | The list of findings. Each one has the keys `kind`, `severity`, `path`, `line` and `detail` ([cli.md](../cli.md#findings-in-json)) |
| `counts` | object | The number of findings per kind. Kinds with zero findings are omitted entirely |
| `tests` | object | The number of test files read, per extension (see the section below) |
| `guides` | object | The number of guides read and the number of guide marks (see the section below) |
| `overview` | object | `files` (the number of overview data files read) and `marks` (the number of individual well-formed guide marks in them). Counted the same way as `guides`; both are output as 0 even when the configuration has no `overview` key |
| `surface` | object | A single key, `unspecified` (the number of kind-and-name pairs of surfaces excluded by the list of unspecified surfaces). Omitted entirely when `surface.rules` is an empty list |
| `decisions` | object | `documents` (the number of decision documents read), `decisions` (the number of decisions) and `confirmations` (the number of confirmation rows with a well-formed ID) |

The values in `counts` always match the number of findings of that kind in `findings`.

### `tests` and `guides` in the JSON

<!-- @kotowari[docs/decisions/core/output.md#the-tests-of-checks-json-counts-the-test-files-read-per-extension:834f2486, docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7] -->

`tests` and `guides` are counts for confirming that the globs in your configuration match where you expect.
A mistyped glob produces no finding, so check it here.

| Key | Type | Description |
|---|---|---|
| `tests.<extension>.files` | number | The number of test files read with that extension. Files that could not be parsed and became `unparsable_file` are counted too |
| `tests.<extension>.query` | boolean | Whether the language for that extension has a query for finding tests. For languages where it is false, only marks are picked up, without identifying test functions |
| `guides.files` | number | The number of guides read |
| `guides.marks` | number | The number of individual well-formed guide marks (each `target:fingerprint` entry) |

- Extension keys do not include the `.` (`rs`, `ts`). The key for files without an extension is the empty string.
- With zero test files, `tests` is `{}`; with zero guides, `guides` is `{"files":0,"marks":0}`.
- Each file is counted once per path, even if it matches several globs.

Per-extension details are in [the output decisions](../../decisions/core/output.md#the-tests-of-checks-json-counts-the-test-files-read-per-extension).

### Finding order and lines

<!-- @kotowari[docs/decisions/core/finding-order.md#findings-are-arranged-by-path-line-kind-and-detail-in-that-order:430be622, docs/decisions/core/finding-order.md#line-is-counted-from-1-and-a-finding-that-covers-the-whole-document-is-null:b98176aa, docs/decisions/core/finding-order.md#the-line-of-findings-that-have-a-line-is-decided-per-kind-in-one-table:848f58fc] -->

Findings are ordered by `path`, `line` (null first), `kind` and `detail` ([cli.md](../cli.md#order-of-findings)).
Findings on the same line are ordered by the name of their kind.

`line` is fixed per kind.
It is null for findings about a whole document or file (`missing_title`, `multiple_titles`, `unparsable_file` and so on), for findings about the list of equivalents and the list of unspecified surfaces, for the overview data findings `overview_lead_missing`, `overview_document_missing`, `overview_document_shared` and `overview_name_conflict`, for the table of contents findings, and for the pair findings `translation_missing`, `translation_record_invalid` and `translation_stale`.
For findings about a decision it is the line of the decision's heading or of its state line, for findings about a row the line of the row, and for `surface_without_spec` the first line of the surface's node.
The line for each kind is in the table in [findings.md](../findings.md#list-of-kinds).

## Exit codes

<!-- @kotowari[docs/decisions/core/cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops:3dc67d73] -->

| Code | Meaning |
|---|---|
| 0 | No errors (including when there are only notices). With `--allow-test-findings`, also when every error is a test-side finding |
| 1 | One or more errors. With `--allow-test-findings`, one or more errors that are not test-side findings |
| 2 | Stopped (the configuration could not be read, a location is missing, and so on) |

When kotowari stops, it prints nothing to standard output and prints the reason to standard error ([cli.md](../cli.md#stopping)).

## Committing a specification before its tests

<!-- @kotowari[docs/decisions/core/test-side-findings.md#with---allow-test-findings-check-does-not-count-test-side-findings-toward-the-exit-code:b75bbdf7, docs/decisions/core/test-side-findings.md#test-side-findings-are-limited-to-errors-caused-by-test-files-and-their-range-is-set-in-a-table:09dd0c43, docs/decisions/core/confirmations.md#the-implementing-side-changes-the-status-and-decisions-not-built-yet-stay-proposed:436adde2] -->

A specification is usually approved and committed before the tests that cover it are written.
A decision written as proposed asks for no test, so it does not stop that commit.
A row added to an implemented decision, however, has no test yet, so `check` reports `confirmation_without_test` and exits with 1.
With `--allow-test-findings`, `check` leaves these test-side findings out of the exit code.

Which errors are test-side findings is decided by kotowari, not by the caller:

| Kind | When it is a test-side finding |
|---|---|
| `confirmation_without_test` | Always |
| `test_without_id` | Always |
| `invalid_marker` | When its path is a test file |
| `unresolved_reference` | When its path is a test file |
| `unparsable_file` | When its path is a test file and not a surface file |

Every other error still makes the exit code 1: an error in a decision document, a malformed guide mark, an unreadable surface file.
The option changes only the exit code. The findings, their order and the JSON and text output are the same as without it, so the test-side findings stay visible.

```console
$ kotowari check --format text --allow-test-findings
docs/decisions/greet/greet.md:35 [error] confirmation_without_test C-greet-greet-2
tests/greet.rs:6 [error] test_without_id rejects_empty_name
$ echo $?
0
```

### Where to use it in hooks

<!-- @kotowari[docs/decisions/core/test-side-findings.md#how-to-build-the-hooks-is-guided-in-the-skill-and-in-the-guide-of-check:3bde432b] -->

- In the pre-commit hook, run `kotowari check --allow-test-findings`, so that the commit approving a specification is not stopped before its tests exist.
- In the pre-push hook and in CI, run `kotowari check` without the option, so that nothing reaches the shared branch with a confirmation row left untested.

## Example

This is a scenario where you have written the decision document for a small command that just returns a greeting.
You have placed the following three files.

```text
.kotowari/config.yaml           設定（tests.files に "tests/**/*.rs"）
docs/decisions/greet/greet.md   決定の文書
tests/greet.rs                  テスト2本
```

(In order: the configuration with `"tests/**/*.rs"` in `tests.files`; the decision document; and two tests.)

### Checking a freshly written decision document

<!-- @kotowari[docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

The decision document (`docs/decisions/greet/greet.md`) and the tests are as follows.

```markdown
# Greeting

This topic decides the text the greet command prints for a name.

## The command greets the name it is given

Status: implemented

### Problem

Users wanted to see their own name in the greeting.

### Decision

Given a name, the command prints "Hello, <name>!".

### Confirmations

- C-greet-greet-1 "greet Taro" prints "Hello, Taro!"

## An empty name stops the command

Status: implemented

### Problem

An empty name printed "Hello, !", which looked like a bug. The rule for names is in [names](#names-are-trimmed).

### Decision

When the name is empty, the command stops and prints the reason など.

### Confirmations

- C-greet-greet-2 "greet ''" exits with 2
```

```rust
// @kotowari[C-greet-greet-1]
#[test]
fn greets_with_name() {}

#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/decisions/greet/greet.md:27 [error] link_target_missing #names-are-trimmed
docs/decisions/greet/greet.md:31 [error] vague_word など
docs/decisions/greet/greet.md:35 [error] confirmation_without_test C-greet-greet-2
tests/greet.rs:6 [error] test_without_id rejects_empty_name
$ echo $?
1
$ kotowari check | jq '.findings[0]'
{
  "kind": "link_target_missing",
  "severity": "error",
  "path": "docs/decisions/greet/greet.md",
  "line": 27,
  "detail": "#names-are-trimmed"
}
```

### Fixing the test marks

Add a mark to `rejects_empty_name` with the ID of the row it confirms.

```rust
// @kotowari[C-greet-greet-1]
#[test]
fn greets_with_name() {}

// @kotowari[C-greet-greet-2]
#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/decisions/greet/greet.md:27 [error] link_target_missing #names-are-trimmed
docs/decisions/greet/greet.md:31 [error] vague_word など
```

### Fixing the decision document down to zero findings

<!-- @kotowari[docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75, docs/decisions/core/decision-documents.md#keep-the-glossary-as-a-document-for-people-and-drop-the-term-check:270bb62b] -->

Remove the link to a heading that does not exist, and rewrite "など" (etc.) as a concrete statement.

```markdown
When the name is empty, the command stops and prints the reason to standard error.
```

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari check | jq -c '{findings, counts, tests, guides, decisions}'
{"findings":[],"counts":{},"tests":{"rs":{"files":1,"query":true}},"guides":{"files":0,"marks":0},"decisions":{"documents":1,"decisions":2,"confirmations":2}}
```

How to fix each finding is described in [findings.md](../findings.md).

### Only notices

<!-- @kotowari[docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce, docs/decisions/core/cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops:3dc67d73] -->

If you try a configuration that also reads a guide whose mark has an old fingerprint, a `guide_stale` notice appears.
Since there are only notices, the exit code is 0.

```console
$ kotowari check --config notice.yaml --format text
guides/greet.md:5 [notice] guide_stale docs/decisions/greet/greet.md#the-command-greets-the-name-it-is-given 00000000 b08a343a
$ echo $?
0
```

## Common pitfalls

### It stops with `unreadable file` even though there is no configuration file

<!-- @kotowari[docs/decisions/core/config.md#if-the-location-of-decision-documents-does-not-exist-is-not-a-directory-or-cannot-be-read-kotowari-stops-with-unreadable-file:a2064f46, docs/decisions/core/config.md#when-the-configuration-file-is-missing-or-empty-kotowari-runs-with-the-default-values:d78fb03e] -->

```console
$ kotowari check --format text
unreadable file: docs/decisions: No such file or directory (os error 2)
```

Even without a configuration file, the default location of decision documents (`docs/decisions`) must exist.
Create the directory, or point the configuration's `decisions.documents` at the directory where your decision documents are.

### `tests` stays `{}` even though tests are written

<!-- @kotowari[docs/decisions/core/output.md#the-tests-of-checks-json-counts-the-test-files-read-per-extension:834f2486, docs/decisions/core/config.md#a-list-key-accepts-only-a-list-the-list-written-replaces-the-default-and-an-empty-list-is-a-list-with-no-elements:7c2ef13f] -->

The globs in `tests.files` do not match your test files.
The defaults are only `src/**/*.rs` and `tests/**/*.rs`.
For tests in languages other than Rust, or tests in other places, write globs in the configuration's `tests.files`.
The list you write replaces the defaults, so if you still need the two defaults, list them as well.

### `guides.files` is 0 even though guides are written

<!-- @kotowari[docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7] -->

The default for `guides.files` is an empty list, so no guides are read until you set it in the configuration.
Write globs in the configuration ([writing-guides.md](../writing-guides.md)).

### `test_without_id` does not go away even though a mark is written

<!-- @kotowari[docs/decisions/core/coverage.md#a-test-without-a-marker-gets-a-test_without_id-error:2f609678] -->

The mark may be in the wrong place.
A mark goes in the comment block immediately before the test.
A mark inside the function body, or in a comment separated by a blank line, is ignored without any finding.

```console
$ kotowari check --format text      # 印を関数の本体の中に書いた
docs/decisions/greet/greet.md:35 [error] confirmation_without_test C-greet-greet-2
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

(The comment on the first line means "the mark was written inside the function body".)

Details are in [marks.md](../marks.md).

## Related

- Specification: [Output format](../../decisions/core/output.md), [Finding order and lines](../../decisions/core/finding-order.md), [Configuration](../../decisions/core/config.md), [Encoding and code block boundaries](../../decisions/core/ir-input.md), [Test-side findings](../../decisions/core/test-side-findings.md)
- Rules shared by all commands: [cli.md](../cli.md)
- Finding kinds and how to fix them: [findings.md](../findings.md)
- How to write decision documents: [decision-documents.md](../decision-documents.md)
- Configuration keys: [config.md](../config.md)
- See whether everything is in place as a whole: [status](status.md)
- See decisions, rows and tests one by one: [list](list.md)
