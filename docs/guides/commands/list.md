# kotowari list

English | [日本語](list.ja.md)

<!-- @kotowari[REQ-core-151:c8db3df3] -->

`kotowari list` lists the items and scenarios of the IR (the specification), together with the tests whose marks point at them.
Use it when you want to see which tests are attached to which requirement, or when you want to hand the whole specification to an LLM in one go.
It reports no findings. To see findings, use [`kotowari check`](./check.md).

## Synopsis

<!-- @kotowari[REQ-core-002:f86e2efd, REQ-core-004:7719a0bd] -->

```sh
kotowari list [--format json|text] [--config <path>]
kotowari list --help
kotowari list --version
```

Options can be written before or after the command (`kotowari --format text list` means the same thing).
It takes no positional arguments.

## Options and arguments

<!-- @kotowari[REQ-core-155:586b389e, REQ-core-003:8ac6759c, REQ-core-011:0b7f52a9] -->

| Name | Value | Default | Description |
|---|---|---|---|
| `--format` | `json` or `text` | `json` | The output format |
| `--config` | Path to a configuration file | `.kotowari/config.yaml` in the base directory | The configuration file to read. The path is read relative to the current directory |
| `--help` | None | — | Prints usage and exits |
| `--version` | None | — | Prints the version and exits |

There are no filtering options.
To look at a single entry in detail, use [`kotowari query`](./query.md); to filter by a condition, pipe the JSON through `jq`.
Details on the base directory and the shared options are in [CLI common rules](../cli.md).

## Output

What it reads is the same as `kotowari check` (the same configuration, and the IR documents and test files in the same locations).
However, it does not read guides or overview data.
Only when `languages` lists two or more languages does it also walk the locations of guides, overview data and the table of contents, to output the hashes in `translations` (it does not check their contents).
Items come only from the first-language side of each pair.

### text

<!-- @kotowari[REQ-core-155:586b389e, EX-core-248:6b0b85d0, EX-core-399:663dc658] -->

Each entry is printed on one line, followed by its marked tests, one per line, indented.

```text
ID 検証 名前 パス:行 tests=数
  パス:行 テストの名前
```

For a [deferred](../deferred.md) requirement or a deferred scenario (an entry whose `deferred` is true in the JSON), ` deferred` is appended to the end of the line.

```text
REQ-001 unit 例 docs/ir/a.md:7 tests=0 deferred
```

| Field | Content |
|---|---|
| Verification (検証) | The value of the requirement's `- verification:`. `-` for anything other than a requirement, and for a requirement with no `- verification:` line |
| Path:line (パス:行) | The line of the item's heading (for a scenario, the `Scenario:` line) |
| tests=count (tests=数) | The number of tests whose marks include that ID |
| Test lines | `path:line name`, indented by two half-width spaces. The path and line are the mark's position. `-` when the name cannot be obtained |

When `languages` lists two or more languages, after the item lines comes one line per pair: the path of the first-language side, followed, for each language, by a single half-width space and `language-tag=blob hash`.
A missing side is shown as `language-tag=-`.

```text
docs/ir/a.md ja=78981922613b2afb6025042ff6bd878ac1994e85 en=-
```

### JSON

<!-- @kotowari[TBL-core-026:382b0b95, REQ-core-153:513617dc, EX-core-288:4229c3fc, REQ-core-155:586b389e] -->

The top level is an object with `items`, plus `translations` only when `languages` lists two or more languages.
`items` is an array of objects, one per entry, and the keys each one has depend on its kind. Keys an entry does not have are omitted.

| Key | Type | Kinds that have it | Description |
|---|---|---|---|
| `id` | string | All | The ID |
| `kind` | string | All | One of `requirement`, `table`, `property`, `scenario`, `flag` |
| `name` | string | All | The heading's name. For a scenario, the text after `Scenario:` |
| `path` | string | All | Path of the document containing the item, relative to the base directory |
| `line` | number | All | The heading's line. For a scenario, the `Scenario:` line |
| `type` | string or null | Requirement, flag record | The value of `- kind:` |
| `verification` | string or null | Requirement | The value of `- verification:` |
| `definition` | array of strings | Requirement | The IDs in `- definition:`. An empty array if there are none |
| `examples` | array of strings | Requirement, decision table, property | The IDs of scenarios that have this ID in `@about`, in ascending ID order |
| `how_to_verify` | string or null | Requirement | The value of `- how_to_verify:` |
| `relations` | array of strings | Flag record | The IDs in `- related:` |
| `sources` | array of strings | All | The sources |
| `tests` | array of objects | All | The tests whose marks include this ID. Each has `path`, `line` (the mark's line) and `name` (the test's name, or null) |
| `fingerprint` | string | All | The fingerprint (8 hex digits). The value you copy into a [guide mark](../writing-guides.md) |
| `deferred` | boolean | All | true for a [deferred](../deferred.md) requirement or a deferred scenario, otherwise false |

If the same test has two marks with the same ID, it appears twice in `tests`.
If there are two scenarios with the same ID, only the first one is counted in `examples`.
The detailed definition is in TBL-core-026 in [the list IR](../../ir/core/list.md).

`translations` is an array with one entry per pair, in ascending `path` order.
A pair whose first-language side is missing still gets an entry.
Each entry has `path` (the path of the first-language side, relative to the base directory) and `sides`.
`sides` is an array of sides in `languages` order; each side has `language` (the language tag), `path`, and `blob` (that side's git blob hash, or null if the side does not exist).
Copy `blob` into the consistency record `<stem>.i18n.yaml` ([Languages and pairs](../config.md#languages-and-pairs--languages-and-labels)).

```json
{"path":"docs/ir/a.md","sides":[{"language":"ja","path":"docs/ir/a.md","blob":"78981922613b2afb6025042ff6bd878ac1994e85"},{"language":"en","path":"docs/ir/a.en.md","blob":null}]}
```

### Ordering

<!-- @kotowari[REQ-core-154:1d379703] -->

`items` is in ascending `path` order, and within the same `path`, in ascending `line` order.
The `tests` of each entry are likewise ordered by `path`, then `line`.

## Exit codes

<!-- @kotowari[REQ-core-151:c8db3df3, REQ-core-152:04c37477] -->

| Code | Meaning |
|---|---|
| 0 | Read successfully (0 even if the IR has errors). Also 0 when it exits after `--help` or `--version` |
| 2 | Stopped (a configuration error, an unreadable file, an argument error, and so on; the reason is printed to standard error) |

`list` never returns 1.
The commands whose exit code depends on whether the IR has errors are `check` and `status`.
The reasons and messages for stopping are the same as for `check` ([CLI common rules](../cli.md)).

## Example

<!-- @kotowari[EX-core-245:37c957d4, EX-core-248:6b0b85d0] -->

This is the result of running it on a small IR with two requirements and one scenario, and a test with one mark (run on 2026-09-24).

The key parts of the IR document `docs/ir/greet.md`:

```markdown
### REQ-001: 名前つきの挨拶
- verification: unit
...
### REQ-002: 挨拶の丁寧さ
- verification: review
- how_to_verify: 返す文を人が読み、丁寧な言葉であることを確かめる
...
@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1
Scenario: 名前を受けて挨拶する
```

The test `tests/greet.rs`:

```rust
// @kotowari[REQ-001, EX-001]
#[test]
fn greets_with_name() {}
```

As text, the output is:

```console
$ kotowari list --format text
REQ-001 unit 名前つきの挨拶 docs/ir/greet.md:7 tests=1
  tests/greet.rs:1 greets_with_name
REQ-002 review 挨拶の丁寧さ docs/ir/greet.md:15 tests=0
EX-001 - 名前を受けて挨拶する docs/ir/greet.md:28 tests=1
  tests/greet.rs:1 greets_with_name
```

In JSON (the default), the same content comes out with keys. Only the first entry is shown.

```console
$ kotowari list | jq '.items[0]'
{
  "id": "REQ-001",
  "kind": "requirement",
  "name": "名前つきの挨拶",
  "path": "docs/ir/greet.md",
  "line": 7,
  "type": "ubiquitous",
  "verification": "unit",
  "definition": [],
  "examples": [
    "EX-001"
  ],
  "how_to_verify": null,
  "sources": [
    "docs/decision/records/records.md#A1"
  ],
  "tests": [
    {
      "path": "tests/greet.rs",
      "line": 1,
      "name": "greets_with_name"
    }
  ],
  "fingerprint": "ec0d8b1c",
  "deferred": false
}
```

Common filters can be written with `jq`.

```console
$ kotowari list | jq -r '.items[] | select(.kind == "requirement" and (.tests | length) == 0) | .id'
REQ-002
```

## Common pitfalls

### The exit code is 0 even though the IR has errors

<!-- @kotowari[REQ-core-151:c8db3df3, EX-core-246:9b303051] -->

`list` is a read-only command: it reports no findings, and even when there are errors it outputs the items it could read and exits with 0.
For example, if you delete the `- verification:` line of REQ-001 from the example IR, `check` reports an error and exits with 1, but `list` shows the verification field as `-` (`null` in JSON) and exits with 0 (run on 2026-09-24).

```console
$ kotowari check --format text
docs/ir/greet.md:7 [error] verification_missing REQ-001
$ kotowari list --format text
REQ-001 - 名前つきの挨拶 docs/ir/greet.md:7 tests=1
  tests/greet.rs:1 greets_with_name
REQ-002 review 挨拶の丁寧さ docs/ir/greet.md:14 tests=0
EX-001 - 名前を受けて挨拶する docs/ir/greet.md:27 tests=1
  tests/greet.rs:1 greets_with_name
$ echo $?
0
```

To fail CI on errors, use `kotowari check` or [`kotowari status`](./status.md).

### The test name is `-` (null in JSON)

<!-- @kotowari[TBL-core-026:382b0b95, EX-core-247:e5b61ce9] -->

In test files of a language without a query, kotowari picks up only the marks and does not obtain test names.
`path` and `line` are the mark's position.
Which extensions have a query is shown by `query` under `tests` in the JSON of `kotowari check` ([CLI common rules](../cli.md)).

### A requirement with `tests=0` is counted as "with tests" in status

<!-- @kotowari[TBL-core-026:382b0b95, TBL-core-028:9645c008] -->

The `tests` in `list` are only the tests whose marks directly include that ID.
`with_tests` in [`kotowari status`](./status.md) also counts tests attached to scenarios that have that requirement in `@about`.
When you find a requirement with `tests=0`, also check the `tests` of the scenarios listed in that requirement's `examples`.
A requirement whose `deferred` is true is counted in neither `with_tests` nor `without_tests` in `status`, but in `deferred`.

### check and status stop on a configuration error, but list runs

<!-- @kotowari[REQ-core-152:04c37477, REQ-core-198:253e79fd, REQ-core-199:55922b15] -->

`list` does not read guides, so it does not stop for reasons tied to the guide locations (such as `guides.files` overlapping `tests.files`).
However, when `languages` lists two or more languages, it walks the locations of guides, overview data and the table of contents, so stops caused by those locations and by reading them behave the same as in `check`.
It also does not read surface files, surface rule files or the list of unspecified surfaces, so it does not stop when they are missing, unreadable or broken.
It does not read overview data either, so it does not stop for reasons tied to its location or reading it (such as `overview.files` overlapping the guide or test locations).
However, `list` does stop on errors in how configuration keys are combined, such as writing only one of `surface.files` and `surface.rules` ([Configuration](../config.md)).
The other conditions for stopping are the same as for `check`.
When you fix the configuration, confirm it with `kotowari check`.

## Related

- Specification: [the list IR](../../ir/core/list.md)
- See one entry with its body and reverse references: [`kotowari query`](./query.md)
- See overall counts and whether everything is in place: [`kotowari status`](./status.md)
- See findings one by one: [`kotowari check`](./check.md)
- Shared options, stopping, the base directory: [CLI common rules](../cli.md)
- The configuration file: [Configuration](../config.md)
