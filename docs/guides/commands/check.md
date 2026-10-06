# kotowari check

English | [日本語](check.ja.md)

<!-- @kotowari[REQ-core-001:f6b868d0] -->

This command reports errors in how the IR (the specification) is written, and gaps in the correspondence between the IR and the tests, as one finding each.
Run it after writing IR or tests, fix the findings, and repeat until there are zero errors.
This single command does both: it checks the form of the IR and checks its correspondence with the tests.

## Synopsis

<!-- @kotowari[REQ-core-002:f86e2efd] -->

```sh
kotowari check [--format json|text] [--config <path>] [--allow-test-findings]
```

It takes no positional arguments.

## Options and arguments

<!-- @kotowari[REQ-core-002:f86e2efd, REQ-core-021:ccedd28b, REQ-core-003:b4f59e48, REQ-core-004:2d3401d1] -->

| Name | Value | Default | Description |
|---|---|---|---|
| `--format` | `json` or `text` | `json` | The output format |
| `--config` | Path to a configuration file | `.kotowari/config.yaml` in the base directory | The configuration file to read. The path is read relative to the current directory |
| `--allow-test-findings` | None (takes no value) | Off | Leaves test-side findings out of the exit code ([Committing a specification before its tests](#committing-a-specification-before-its-tests)). Only `check` takes it; on any other command it stops with `argument error` |
| `--help` | None | — | Prints usage and exits with code 0 |
| `--version` | None | — | Prints the version and exits with code 0 |

The rules shared by all options (for example, that they can be written before or after the command) are in [cli.md](../cli.md#common-options).

## What it reads

<!-- @kotowari[TBL-core-004:322d11b1, REQ-core-325:ba483356, REQ-core-336:03710aae, REQ-core-337:40375497] -->

`check` reads the following from the locations the configuration file points to.

| What it reads | Configuration key that sets the location | Default |
|---|---|---|
| IR documents (including the glossary `CONTEXT.md` and the flag record `FLAGS.md`) | `ir` | `docs/ir` |
| Decision records (targets of sources) | `decisions.records` | `docs/decision/records` |
| ADRs (targets of sources) | `decisions.adr` | `docs/decision/adr` |
| Test files | `tests.files` | `src/**/*.rs`, `tests/**/*.rs` |
| Guides | `guides.files` | Empty (not read) |
| Surface files and surface rule files | `surface.files`, `surface.rules` | Empty (not read) |
| List of unspecified surfaces | `surface.unspecified` | None (zero entries) |
| Overview data | `overview.files` | None (not read when there is no `overview` key) |
| Overview table of contents | `overview.toc` | None (not read when there is no `overview` key; when the key exists it is required, and kotowari stops if the target is missing or unreadable) |

The three surface inputs are read only when `surface.rules` is not an empty list ([Surface checks](../surface.md)).
When `languages` lists two or more languages, kotowari also reads, for the IR, guides, overview data and table of contents, the other-language sides (`foo.en.md`) and the consistency records (`foo.i18n.yaml`), finding them by name in the same directory as the first-language side ([Languages and pairs](../config.md#languages-and-pairs--languages-and-labels)).
All the keys are in [config.md](../config.md).

### The configuration file

<!-- @kotowari[REQ-core-011:0b7f52a9, REQ-core-012:50c68e4b] -->

- Without `--config`, kotowari reads `.kotowari/config.yaml` in the base directory.
- If that file does not exist, everything is checked with default values.
- If the configuration file is empty (zero bytes, or comments only), default values are used, even for a file named with `--config`.

How the base directory is determined is described in [cli.md](../cli.md#the-base-directory).

### When a location is missing

<!-- @kotowari[REQ-core-018:6ea3e08f, REQ-core-019:178b0f0c] -->

The three locations `ir`, `decisions.records` and `decisions.adr` must exist as directories, even when left at their defaults.
If any of them is missing, is not a directory, or cannot be read, kotowari stops with `unreadable file`.
Create an empty directory even for a location you do not use.

`tests.files`, `guides.files`, `surface.files` and `overview.files` are lists of globs.
A glob that matches nothing is not an error.
`**` is read as recursive.
Hidden directories are not included even when a glob names them, and symbolic links to directories are not followed.
As exceptions, `changes.records` for change records and `overview.files` for overview data do read hidden directories named explicitly in a path component (such as `.kotowari/overview/*.md`). Hidden directories that are not named are excluded even by a broad `**`.

If a directory cannot be read during the scan, or a dangling symbolic link is encountered, kotowari stops with `unreadable file`.

### Encoding and unclosed code blocks

<!-- @kotowari[REQ-core-111:a41af8e5, REQ-core-112:2fc914d5] -->

Files that are read must be UTF-8.
A leading BOM is skipped, so UTF-8 with a BOM does not cause a stop.
If there is a file that is not UTF-8, kotowari stops with `non-UTF-8 file`.

If a code block in an IR document is never closed before the document ends, an `unclosed_code_block` error is reported on its opening line.
In that case, the part from the opening line to the end of the document is not checked.

## Output

### text

<!-- @kotowari[REQ-core-025:58025379, REQ-core-026:530a64e3] -->

Each line is one finding.
If there are no errors and no notices, nothing is printed.

```text
パス:行 [error] 種類 詳細
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
docs/ir/greet/greet.md:- [notice] too_many_lines 30
```

| Part | Content |
|---|---|
| Path (パス) | Path relative to the base directory |
| Line (行) | 1-based line number. `-` for a finding about the whole document |
| `[error]` / `[notice]` | Whether it is an error or a notice |
| Kind (種類) | The kind of finding. Its meaning and how to fix it are in [findings.md](../findings.md) |
| Detail (詳細) | A short string whose content is fixed per kind |

The text output does not include `files`, `lines`, `tests`, `guides` or `overview` from the JSON below.

When the configuration's `surface.rules` is not an empty list, a final line after the finding lines gives the number of surfaces excluded by the list of unspecified surfaces, in the form `surface: unspecified=数` (数 being the count).
It is printed even when there are zero findings and even when the count is 0 ([Surface checks](../surface.md#seeing-how-many-are-excluded)).

### JSON

<!-- @kotowari[REQ-core-022:8e221eee, TBL-core-005:73d0910b, PROP-core-002:35724b25, REQ-core-228:095b2109, REQ-core-288:0f3318d5] -->

This is the default output.
It prints a single JSON object to standard output.

| Key | Type | Description |
|---|---|---|
| `files` | number | The number of IR documents read (including glossaries and flag records; for a pair, other-language sides are counted too) |
| `lines` | number | The total number of lines in the IR documents (including glossaries and flag records; for a pair, other-language sides are counted too) |
| `findings` | array | The list of findings. Each one has the keys `kind`, `severity`, `path`, `line` and `detail` ([cli.md](../cli.md#findings-in-json)) |
| `counts` | object | The number of findings per kind. Kinds with zero findings are omitted entirely |
| `tests` | object | The number of test files read, per extension (see the section below) |
| `guides` | object | The number of guides read and the number of guide marks (see the section below) |
| `surface` | object | A single key, `unspecified` (the number of kind-and-name pairs of surfaces excluded by the list of unspecified surfaces). Omitted entirely when `surface.rules` is an empty list |
| `overview` | object | `files` (the number of overview data files read) and `marks` (the number of individual well-formed guide marks in them). Counted the same way as `guides`; both are output as 0 even when the configuration has no `overview` key |

The values in `counts` always match the number of findings of that kind in `findings`.

### `tests` and `guides` in the JSON

<!-- @kotowari[REQ-core-128:e6139f2c, TBL-core-021:d85afd29, REQ-core-206:e2f3dc5c] -->

`tests` and `guides` are counts for confirming that the globs in your configuration match where you expect.
A mistyped glob produces no finding, so check it here.

| Key | Type | Description |
|---|---|---|
| `tests.<extension>.files` | number | The number of test files read with that extension. Files that could not be parsed and became `unparsable_file` are counted too |
| `tests.<extension>.query` | boolean | Whether the language for that extension has a query for finding tests. For languages where it is false, only marks are picked up, without identifying test functions |
| `guides.files` | number | The number of guides read |
| `guides.marks` | number | The number of individual well-formed guide marks (each `ID:fingerprint` entry) |

- Extension keys do not include the `.` (`rs`, `ts`). The key for files without an extension is the empty string.
- With zero test files, `tests` is `{}`; with zero guides, `guides` is `{"files":0,"marks":0}`.
- Each file is counted once per path, even if it matches several globs.

Per-extension details are in [TBL-core-021 in output.md](../../ir/core/output.md).

### Finding order and lines

<!-- @kotowari[TBL-core-007:15954989, REQ-core-027:a594c5e0, TBL-core-019:d5c9adce] -->

Findings are ordered by `path`, `line` (null first), `kind` and `detail` ([cli.md](../cli.md#order-of-findings)).
Findings on the same line are ordered by the name of their kind.

`line` is fixed per kind.
It is null for findings about a whole document (`missing_title`, `missing_scope`, `too_many_lines` and so on), for findings about an entry in the list of unspecified surfaces, for the overview data findings `overview_lead_missing`, `overview_ir_missing`, `overview_ir_shared` and `overview_name_conflict`, for the table of contents findings `overview_toc_invalid`, `overview_toc_page_missing`, `overview_toc_page_unknown`, `overview_toc_page_duplicate` and `overview_toc_group_empty`, and for the pair findings `translation_missing`, `translation_record_invalid` and `translation_stale`. For findings about an item it is the line of the item's heading, for findings about a scenario the line of its tags, and for `surface_without_spec` the first line of the surface's section.
`overview_form_invalid` uses the line that does not fit the form (null when it concerns the whole document), and `overview_part_unknown`, `overview_part_invalid` and `overview_ref_unresolved` use the opening line of the part's fence.
However, for an `overview_part_invalid` whose part content cannot be read as YAML, if the YAML reader returns the position of the error, that line is used (converted to a line in the overview data file).
`translation_structure_mismatch` uses the line of the first element that differs (null when the counts differ or the element does not exist on that side), `translation_switcher_invalid` uses the first non-blank line after the title (the title line if there is none, or null if there is no title either), and `link_language_mismatch` and `link_to_record` use the line of the link.
The line for each kind is in the table in [findings.md](../findings.md#list-of-kinds).

## Exit codes

<!-- @kotowari[TBL-core-002:36817bf5] -->

| Code | Meaning |
|---|---|
| 0 | No errors (including when there are only notices). With `--allow-test-findings`, also when every error is a test-side finding |
| 1 | One or more errors. With `--allow-test-findings`, one or more errors that are not test-side findings |
| 2 | Stopped (the configuration could not be read, a location is missing, and so on) |

When kotowari stops, it prints nothing to standard output and prints the reason to standard error ([cli.md](../cli.md#stopping)).

## Committing a specification before its tests

<!-- @kotowari[REQ-core-357:352c971a, TBL-core-047:0a2c3aa9] -->

A specification is usually approved and committed before the tests that cover it are written.
At that commit every new requirement and scenario has no test yet, so `check` reports `requirement_without_test` and `scenario_without_test` and exits with 1.
With `--allow-test-findings`, `check` leaves these test-side findings out of the exit code.

Which errors are test-side findings is decided by kotowari, not by the caller:

| Kind | When it is a test-side finding |
|---|---|
| `requirement_without_test` | Always |
| `scenario_without_test` | Always |
| `test_without_id` | Always |
| `invalid_marker` | When its path is a test file |
| `unresolved_reference` | When its path is a test file |
| `unparsable_file` | When its path is a test file and not a surface file |

Every other error still makes the exit code 1: an error in the IR, a malformed guide mark, an unreadable surface file.
The option changes only the exit code. The findings, their order and the JSON and text output are the same as without it, so the test-side findings stay visible.

```console
$ kotowari check --format text --allow-test-findings
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
$ echo $?
0
```

### Where to use it in hooks

<!-- @kotowari[REQ-core-358:879777ff] -->

- In the pre-commit hook, run `kotowari check --allow-test-findings`, so that the commit approving a specification is not stopped before its tests exist.
- In the pre-push hook and in CI, run `kotowari check` without the option, so that nothing reaches the shared branch with a requirement or scenario left untested.

## Example

This is a scenario where you have written the IR for a small command that just returns a greeting.
You have placed the following five files and one empty directory.

```text
.kotowari/config.yaml                     設定（tests.files に "tests/**/*.rs"）
docs/decision/records/2026-09-24-greet.md 判断の記録（決定は A1 と A2 の2件）
docs/decision/adr/                        空のディレクトリ
docs/ir/greet/CONTEXT.md                  用語集（「挨拶文」だけ）
docs/ir/greet/greet.md                    IR の本体
tests/greet.rs                            テスト2本
```

(In order: the configuration with `"tests/**/*.rs"` in `tests.files`; a decision record with two decisions, A1 and A2; an empty directory; a glossary containing only "挨拶文" (greeting text); the main IR document; and two tests.)

### Checking freshly written IR

<!-- @kotowari[REQ-core-085:288046ea, REQ-core-086:8035b3f8, REQ-core-137:cb66f5a8] -->

The main IR document (`docs/ir/greet/greet.md`) and the tests are as follows.

````markdown
# 挨拶

挨拶のコマンドが名前を受けて返す文を扱う。

## Requirements

### REQ-greet-001: 名前を入れて挨拶する

- kind: event_driven
- source: docs/decision/records/2026-09-24-greet.md#A1
- verification: unit

名前を受けたとき、コマンドは「こんにちは、名前さん」の`挨拶文`を出す。

### REQ-greet-002: 空の名前は停止する

- kind: event_driven
- source: docs/decision/records/2026-09-24-greet.md#A3
- verification: unit

名前が空のとき、コマンドは`停止`し、必要に応じて理由を出す。

## Examples

```gherkin
@id=EX-greet-001 @about=REQ-greet-001 @source=docs/decision/records/2026-09-24-greet.md#A1
Scenario: 名前を挨拶文に入れる
  When "greet 太郎" を実行する
  Then "こんにちは、太郎さん" が出る
```
````

```rust
// @kotowari[REQ-greet-001]
#[test]
fn greets_with_name() {}

#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
docs/ir/greet/greet.md:21 [error] unknown_term 停止
docs/ir/greet/greet.md:21 [error] vague_word 必要に応じて
docs/ir/greet/greet.md:26 [error] scenario_without_test EX-greet-001
tests/greet.rs:6 [error] test_without_id rejects_empty_name
$ echo $?
1
$ kotowari check | jq '.findings[0]'
{
  "kind": "requirement_without_test",
  "severity": "error",
  "path": "docs/ir/greet/greet.md",
  "line": 15,
  "detail": "REQ-greet-002"
}
```

### Fixing the test marks

<!-- @kotowari[REQ-core-085:288046ea, REQ-core-137:cb66f5a8] -->

Add a mark to `rejects_empty_name`, and change the mark on `greets_with_name` to the scenario's ID.
A scenario mark also covers the requirement named in that scenario's `@about`.
If you have decided not to build a requirement for now, declaring a [deferral](../deferred.md) instead of adding a mark also clears this error.

```rust
// @kotowari[EX-greet-001]
#[test]
fn greets_with_name() {}

// @kotowari[REQ-greet-002]
#[test]
fn rejects_empty_name() {}
```

```console
$ kotowari check --format text
docs/ir/greet/greet.md:18 [error] source_invalid docs/decision/records/2026-09-24-greet.md#A3
docs/ir/greet/greet.md:21 [error] unknown_term 停止
docs/ir/greet/greet.md:21 [error] vague_word 必要に応じて
```

### Fixing the IR down to zero findings

<!-- @kotowari[REQ-core-058:0012abf7, REQ-core-064:75e8708a, REQ-core-066:79261ab9] -->

Change the source to `#A2`, which exists; add "停止" (stop) to the glossary; and rewrite "必要に応じて" (as needed) as a concrete statement.

```markdown
名前が空のとき、コマンドは`停止`し、理由を標準エラーに出す。
```

```console
$ kotowari check --format text
$ echo $?
0
$ kotowari check | jq -c .
{"files":2,"lines":36,"findings":[],"counts":{},"tests":{"rs":{"files":1,"query":true}},"guides":{"files":0,"marks":0}}
```

How to fix each finding is described in [findings.md](../findings.md).

### Only notices

<!-- @kotowari[REQ-core-038:170fdd3e, TBL-core-002:36817bf5] -->

If you try a configuration with `limits.lines` lowered to 20, a notice about the line count appears.
Since there are only notices, the exit code is 0.

```console
$ kotowari check --config notice.yaml --format text
docs/ir/greet/greet.md:- [notice] too_many_lines 30
$ echo $?
0
```

## Common pitfalls

### It stops with `unreadable file` even though there is no configuration file

<!-- @kotowari[REQ-core-018:6ea3e08f, REQ-core-012:50c68e4b] -->

```console
$ kotowari check --format text
unreadable file: docs/decision/adr: No such file or directory (os error 2)
```

Even without a configuration file, the default locations (`docs/ir`, `docs/decision/records`, `docs/decision/adr`) must exist.
If you do not use ADRs, create an empty `docs/decision/adr/`, or point the configuration's `decisions.adr` at another directory.

### `tests` stays `{}` even though tests are written

<!-- @kotowari[REQ-core-128:e6139f2c, TBL-core-021:d85afd29, REQ-core-015:44b9e418] -->

The globs in `tests.files` do not match your test files.
The defaults are only `src/**/*.rs` and `tests/**/*.rs`.
For tests in languages other than Rust, or tests in other places, write globs in the configuration's `tests.files`.
The list you write replaces the defaults, so if you still need the two defaults, list them as well.

### `guides.files` is 0 even though guides are written

<!-- @kotowari[REQ-core-206:e2f3dc5c] -->

The default for `guides.files` is an empty list, so no guides are read until you set it in the configuration.
Write globs in the configuration ([writing-guides.md](../writing-guides.md)).

### `test_without_id` does not go away even though a mark is written

<!-- @kotowari[REQ-core-086:8035b3f8] -->

The mark may be in the wrong place.
A mark goes in the comment block immediately before the test.
A mark inside the function body, or in a comment separated by a blank line, is ignored without any finding.

```console
$ kotowari check --format text      # 印を関数の本体の中に書いた
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

(The comment on the first line means "the mark was written inside the function body".)

Details are in [marks.md](../marks.md).

## Related

- Specification: [Output format](../../ir/core/output.md), [Finding order and lines](../../ir/core/finding-order.md), [Configuration](../../ir/core/config.md), [Encoding and code block boundaries](../../ir/core/ir-input.md)
- Rules shared by all commands: [cli.md](../cli.md)
- Finding kinds and how to fix them: [findings.md](../findings.md)
- Configuration keys: [config.md](../config.md)
- See whether everything is in place as a whole: [status](status.md)
- See items and tests one by one: [list](list.md)

When `changes.records` is configured, kotowari checks the format and current references of every change record. A broken reference is an error even for records with a different base. Hidden directories named in a path component of `changes.records` are read; hidden directories not named are not read even by a broad `**`. The usual hidden-directory exclusion for tests and guides does not change. No Git comparison base is needed; content freshness and change coverage are checked by [changes](changes.md).
