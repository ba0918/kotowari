# kotowari query

English | [日本語](query.ja.md)

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

`kotowari query` takes one ID and prints that item or scenario together with its body and its reverse references (the items that point at that ID).
Use it when you want to read a single requirement without opening the IR document, or to get the fingerprint to copy into a guide mark.

## Synopsis

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

```sh
kotowari query [--format json|text] [--config <path>] <ID>
kotowari query --help
kotowari query --version
```

The positional argument is exactly one ID.
Options are accepted before the command or after the ID (`kotowari query REQ-001 --format text` means the same thing).

## Options and arguments

<!-- @kotowari[docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

| Name | Value | Default | Description |
|---|---|---|---|
| `<ID>` | An ID (of the form `REQ-core-001`) | None (required) | The ID of the item or scenario to read. Case-sensitive |
| `--format` | `json` or `text` | `json` | The output format |
| `--config` | Path to a configuration file | `.kotowari/config.yaml` in the base directory | The configuration file to read. The path is read relative to the current directory |
| `--help` | None | — | Prints usage and exits |
| `--version` | None | — | Prints the version and exits |

Details on the shared options and the base directory are in [CLI common rules](../cli.md).

## Output

What it reads is the same as `kotowari check` (the same configuration, and the IR documents and test files in the same locations).
However, it does not read guides.
If several items or scenarios have the same ID, all of them are printed.

### text

<!-- @kotowari[docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5] -->

For each entry, the body lines and the reverse reference lines follow the same first line and test lines as the text output of [`kotowari list`](./list.md).

```text
ID 検証 名前 パス:行 tests=数
  パス:行 テストの名前
  本文の1行目
  本文の2行目
  ...
  <- 指している ID via パス:行
```

| Line | Content |
|---|---|
| First line and test lines | Same as `kotowari list --format text` |
| Body lines | Each line of `body`, indented by two half-width spaces. A blank line becomes a line of just two spaces |
| `<-` lines | One per entry in `referenced_by`: `  <- ID via path:line`. `via` takes a value from the table below |

### JSON

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

The top level is an object with only `items`, the same shape as `list`.
Each entry has all of [the keys of a `kotowari list` entry](./list.md#json) (`id`, `kind`, `name`, `path`, `line`, `type`, `verification`, `definition`, `examples`, `how_to_verify`, `relations`, `sources`, `tests`, `fingerprint`, `deferred`), plus the following two keys.

| Key | Type | Description |
|---|---|---|
| `body` | array of strings | The body lines. Leading and trailing blank lines are excluded; the text of each line is kept as is |
| `referenced_by` | array of objects | The items and scenarios that point at this ID. Each has `id`, `kind`, `path`, `line` and `via` |

The extent of `body` depends on the kind.

| Kind | Extent of `body` |
|---|---|
| Item (requirement, decision table, property, flag record) | From the line after the heading up to the line before the next heading of the same or shallower depth. Lines inside code blocks that look like headings do not end it |
| Scenario | From the `@id` tag line to the last step line |

`via` tells where the reference comes from.

| `via` | Where it points from |
|---|---|
| `definition` | A `- definition:` line |
| `relations` | A `- related:` line |
| `about` | A scenario's `@about` tag |
| `text` | An ID enclosed in backquotes (outside double quotes) in a requirement's statement, a property's statement, or a scenario's step |

Even if one item points at the same ID several times with the same `via`, it appears only once in `referenced_by`.
The detailed definition is in TBL-core-027 in [the query IR](../../ir/core/query.md).

### Ordering

<!-- @kotowari[docs/decisions/core/query.md#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading:90363085] -->

`items` is in the same order as `list` (ascending `path`, and within the same `path`, ascending `line`).
`referenced_by` is also ordered by `path`, then `line`.

## Exit codes

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c, docs/decisions/core/query.md#query-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces:4414244f] -->

| Code | Meaning |
|---|---|
| 0 | Read successfully (0 even if the IR has errors). Also 0 when it exits after `--help` or `--version` |
| 2 | Stopped (the ID does not exist, the argument is not of ID form, the wrong number of arguments, a configuration error, an unreadable file, and so on; the reason is printed to standard error) |

`query` never returns 1.
The commands whose exit code depends on whether the IR has errors are `check` and `status`.
`query` does not read guides, surface files, surface rule files, the list of unspecified surfaces or overview data, so it never stops because of them.

## Example

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5] -->

These are results of running it on the same small IR and tests as in [the `kotowari list` example](./list.md#example) (run on 2026-09-24).

Read a requirement as text.
The `<-` line after the body shows that scenario EX-001 points at this requirement through `@about`.

```console
$ kotowari query REQ-001 --format text
REQ-001 unit 名前つきの挨拶 docs/ir/greet.md:7 tests=1
  tests/greet.rs:1 greets_with_name
  - kind: ubiquitous
  - source: docs/decision/records/records.md#A1
  - verification: unit
  
  システムは常に、名前を受けたら "こんにちは、" に名前を続けた文を返す。
  <- EX-001 about docs/ir/greet.md:28
```

In JSON, an entry of `list` gains `body` and `referenced_by`. Only those two added keys are shown.

```console
$ kotowari query REQ-001 | jq '.items[0] | {body, referenced_by}'
{
  "body": [
    "- kind: ubiquitous",
    "- source: docs/decision/records/records.md#A1",
    "- verification: unit",
    "",
    "システムは常に、名前を受けたら \"こんにちは、\" に名前を続けた文を返す。"
  ],
  "referenced_by": [
    {
      "id": "EX-001",
      "kind": "scenario",
      "path": "docs/ir/greet.md",
      "line": 28,
      "via": "about"
    }
  ]
}
```

A scenario's body starts at the `@id` tag line.

```console
$ kotowari query EX-001 --format text
EX-001 - 名前を受けて挨拶する docs/ir/greet.md:28 tests=1
  tests/greet.rs:1 greets_with_name
  @id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1
  Scenario: 名前を受けて挨拶する
    Given 名前 "花子" がある
    When 挨拶を求める
    Then "こんにちは、花子" が返る
```

### Getting the fingerprint to copy into a guide mark

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/guides.md#a-stale-section-is-found-by-comparing-the-fingerprint-written-in-the-marker-with-the-current-fingerprint-of-the-decision:c21ccd0f] -->

The fingerprint you write in a guide mark is the `fingerprint` value, copied as is. Do not compute it by hand.

```console
$ kotowari query REQ-001 | jq -r '.items[0].fingerprint'
ec0d8b1c
```

How to write guides is described in [Writing guides](../writing-guides.md).

## Common pitfalls

### It stops with `argument error: unknown id: ...`

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

Neither an item nor a scenario with that ID exists in the IR.
Because it stops instead of returning an empty result, you notice a mistyped ID right away.

```console
$ kotowari query REQ-999
argument error: unknown id: REQ-999
$ echo $?
2
```

If you are not sure of the ID, look for it with `kotowari list --format text`.

### It stops with `argument error: not an id: ...`

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

The positional argument is not of ID form.
IDs are case-sensitive, so writing one in lowercase also makes it stop.

```console
$ kotowari query req-001
argument error: not an id: req-001
```

### It stops with `argument error: query expects exactly one id, got N`

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

You passed no ID, or two or more.
`query` reads only one ID at a time. To look at several, run it once per ID, or filter the JSON of `kotowari list` with `jq`.

```console
$ kotowari query REQ-001 REQ-002
argument error: query expects exactly one id, got 2
$ kotowari query
argument error: query expects exactly one id, got 0
```

### Two entries come out for the same ID

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

The IR has two items with the same ID (a duplicate_id error in `check`).
`query` does not hide duplicates; it prints all of them and exits with 0.

```console
$ kotowari query REQ-001 --format text
REQ-001 unit 別れの挨拶 docs/ir/farewell.md:7 tests=1
  tests/greet.rs:1 greets_with_name
  - kind: ubiquitous
  - source: docs/decision/records/records.md#A1
  - verification: unit
  
  システムは常に、"さようなら" を返す。
  <- EX-001 about docs/ir/greet.md:28
REQ-001 unit 名前つきの挨拶 docs/ir/greet.md:7 tests=1
  tests/greet.rs:1 greets_with_name
  - kind: ubiquitous
  - source: docs/decision/records/records.md#A1
  - verification: unit
  
  システムは常に、名前を受けたら "こんにちは、" に名前を続けた文を返す。
  <- EX-001 about docs/ir/greet.md:28
$ kotowari check --format text
docs/ir/greet.md:7 [error] duplicate_id REQ-001
```

Give one of them a new ID.

### An ID mentioned in a statement does not appear in `referenced_by`

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

Only IDs enclosed in backquotes count as reverse references with `via` `text`.
IDs inside double quotes, or IDs not enclosed at all, are not counted.
Write a reference you want listed as a reverse reference like `` `REQ-001` ``.

## Related

- Specification: [the query IR](../../ir/core/query.md)
- See all items as a list: [`kotowari list`](./list.md)
- See overall counts and whether everything is in place: [`kotowari status`](./status.md)
- See findings one by one: [`kotowari check`](./check.md)
- Guide marks and fingerprints: [Writing guides](../writing-guides.md)
- Shared options, stopping, the base directory: [CLI common rules](../cli.md)
