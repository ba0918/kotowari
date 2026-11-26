# kotowari query

English | [日本語](query.ja.md)

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

`kotowari query` takes one ID, a decision's target or a confirmation row's ID, and prints that entry together with its body and, for a decision, the decisions that link to it.
Use it when you want to read a single decision without opening the decision document, to see which tests confirm a row, or to get the fingerprint to copy into a guide mark.

## Synopsis

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

```sh
kotowari query [--format json|text] [--config <path>] <ID>
kotowari query --help
kotowari query --version
```

The positional argument is exactly one ID.
Options are accepted before the command or after the ID (`kotowari query C-shop-discount-1 --format text` means the same thing).

## Options and arguments

<!-- @kotowari[docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

| Name | Value | Default | Description |
|---|---|---|---|
| `<ID>` | A decision's target (`docs/decisions/shop/discount.md#members-get-10-off-their-orders`) or a row's ID (`C-shop-discount-1`) | None (required) | The entry to read. Case-sensitive |
| `--format` | `json` or `text` | `json` | The output format |
| `--config` | Path to a configuration file | `.kotowari/config.yaml` in the base directory | The configuration file to read. The path is read relative to the current directory |
| `--help` | None | — | Prints usage and exits |
| `--version` | None | — | Prints the version and exits |

Details on the shared options and the base directory are in [CLI common rules](../cli.md).

## Output

What it reads is the same as `kotowari check` (the same configuration, and the decision documents and test files in the same locations).
However, it does not read guides.
If several rows have the same ID, all of them are printed.

### text

<!-- @kotowari[docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5] -->

For each entry, the body lines follow the same first line and test lines as the text output of [`kotowari list`](./list.md).

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
| `<-` lines | For a decision, one per entry in `referenced_by`: `  <- target link path:line` |
| Last line of a row | `  decision state heading`: the state and the heading of the decision holding the row (`-` for the state when the state line is wrong) |

### JSON

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#the-body-when-a-decision-is-looked-up-is-the-whole-decision:39fb387c, docs/decisions/core/query.md#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading:90363085] -->

The top level is an object with only `items`, the same shape as `list`.
Each entry has all of [the keys of a `kotowari list` entry](./list.md#json), plus the following two keys.

| Key | Type | Description |
|---|---|---|
| `body` | array of strings | The body lines, as written in the decision document |
| `referenced_by` | array of objects | For a decision, the decisions that link to its heading, each with `id`, `kind` (`decision`), `path`, `line` (the linking decision's heading) and `via` (`link`). Always empty for a row |

The extent of `body` depends on the kind.

| Kind | Extent of `body` |
|---|---|
| Decision | Every line of the decision from its heading to its last section (the state line, Problem, Decision, Rejected options and Confirmations) |
| Row | The row's lines as written, including wrapped lines |

A decision that links to the same heading several times appears only once in `referenced_by`.
Decision documents point at decisions by links to headings, not by row IDs, so nothing points at a row.
The detailed definition is in [the query decisions](../../decisions/core/query.md).

### Ordering

<!-- @kotowari[docs/decisions/core/query.md#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading:90363085] -->

`items` is in the same order as `list` (ascending `path`, and within the same `path`, ascending `line`).
`referenced_by` is also ordered by `path`, then `line`.

## Exit codes

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c, docs/decisions/core/query.md#query-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces:4414244f] -->

| Code | Meaning |
|---|---|
| 0 | Read successfully (0 even if the decision documents have errors). Also 0 when it exits after `--help` or `--version` |
| 2 | Stopped (the ID does not exist, the argument is not of ID form, the wrong number of arguments, a configuration error, an unreadable file, and so on; the reason is printed to standard error) |

`query` never returns 1.
The commands whose exit code depends on whether there are errors are `check` and `status`.
`query` does not read guides, surface files, surface rule files, the list of unspecified surfaces or overview data, so it never stops because of them.

## Example

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5] -->

These are results of running it on the same small decision document and test as in [the `kotowari list` example](./list.md#example), where the Problem of the second decision links to the first one.

Read a decision as text.
The `<-` line after the body shows that the guest decision links to this decision's heading.

```console
$ kotowari query 'docs/decisions/shop/discount.md#members-get-10-off-their-orders' --format text
docs/decisions/shop/discount.md#members-get-10-off-their-orders - Members get 10% off their orders docs/decisions/shop/discount.md:5 tests=0
  ## Members get 10% off their orders
  
  Status: implemented
  
  ### Problem
  
  Members asked for a benefit for ordering often.
  
  ### Decision
  
  An order placed by a member is priced at 10% off.
  
  ### Confirmations
  
  - C-shop-discount-1 A member's order of 1000 is priced at 900
  <- docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted link docs/decisions/shop/discount.md:21
```

Read a row as text. Its tests come first, and the last line names its decision.

```console
$ kotowari query C-shop-discount-1 --format text
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
  - C-shop-discount-1 A member's order of 1000 is priced at 900
  decision implemented Members get 10% off their orders
```

In JSON, an entry of `list` gains `body` and `referenced_by`.

```console
$ kotowari query C-shop-discount-1
{"items":[{"id":"C-shop-discount-1","kind":"confirmation","name":"A member's order of 1000 is priced at 900","path":"docs/decisions/shop/discount.md","line":19,"decision":"Members get 10% off their orders","state":"implemented","by_reading":false,"tests":[{"path":"tests/discount.rs","line":1,"name":"member_gets_ten_percent_off"}],"body":["- C-shop-discount-1 A member's order of 1000 is priced at 900"],"referenced_by":[]}]}
```

### Getting the fingerprint to copy into a guide mark

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/guides.md#a-stale-section-is-found-by-comparing-the-fingerprint-written-in-the-marker-with-the-current-fingerprint-of-the-decision:c21ccd0f] -->

The fingerprint you write in a guide mark is the `fingerprint` value of the decision, copied as is. Do not compute it by hand.

```console
$ kotowari query 'docs/decisions/shop/discount.md#members-get-10-off-their-orders' | jq -r '.items[0].fingerprint'
e3adc927
```

How to write guides is described in [Writing guides](../writing-guides.md).

## Common pitfalls

### It stops with `argument error: unknown id: ...`

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

No decision or row with that ID exists.
Because it stops instead of returning an empty result, you notice a mistyped ID right away.
A target whose heading was renamed is unknown too.

```console
$ kotowari query C-shop-discount-9
argument error: unknown id: C-shop-discount-9
$ echo $?
2
$ kotowari query 'docs/decisions/shop/discount.md#no-such-heading'
argument error: unknown id: docs/decisions/shop/discount.md#no-such-heading
```

If you are not sure of the ID, look for it with `kotowari list --format text`.

### It stops with `argument error: not an id: ...`

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

The positional argument is neither of the row ID form nor of the target form.
IDs are case-sensitive, so writing one in lowercase also makes it stop, and a document path without `#` is not a target.

```console
$ kotowari query c-shop-discount-1
argument error: not an id: c-shop-discount-1
$ kotowari query docs/decisions/shop/discount.md
argument error: not an id: docs/decisions/shop/discount.md
```

### It stops with `argument error: query expects exactly one id, got N`

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

You passed no ID, or two or more.
`query` reads only one ID at a time. To look at several, run it once per ID, or filter the JSON of `kotowari list` with `jq`.

```console
$ kotowari query C-shop-discount-1 C-shop-discount-2
argument error: query expects exactly one id, got 2
$ kotowari query
argument error: query expects exactly one id, got 0
```

### Two entries come out for the same ID

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

Two rows have the same ID (a duplicate_id error in `check`).
`query` does not hide duplicates; it prints all of them and exits with 0.

```console
$ kotowari query C-shop-discount-1 --format text
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
  - C-shop-discount-1 A member's order of 1000 is priced at 900
  decision implemented Members get 10% off their orders
C-shop-discount-1 reading The price list page says that only members get the discount docs/decisions/shop/discount.md:36 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
  - C-shop-discount-1 (by reading) The price list page says that only members get the discount
  decision implemented Orders of non-members are not discounted
$ kotowari check --format text
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
docs/decisions/shop/discount.md:36 [error] duplicate_id C-shop-discount-1
```

Give the later row the next unused number of the topic.

### A decision mentioned in a sentence does not appear in `referenced_by`

<!-- @kotowari[docs/decisions/core/query.md#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading:90363085, docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75] -->

Only Markdown links to the decision's heading count as reverse references.
A heading quoted in plain text, or a row ID written in a sentence, is not counted.
Write a reference you want listed as a link, like `[the member discount](discount.md#members-get-10-off-their-orders)`.

## Related

- Specification: [the query decisions](../../decisions/core/query.md)
- See all decisions and rows as a list: [`kotowari list`](./list.md)
- See overall counts and whether everything is in place: [`kotowari status`](./status.md)
- See findings one by one: [`kotowari check`](./check.md)
- Guide marks and fingerprints: [Writing guides](../writing-guides.md)
- Shared options, stopping, the base directory: [CLI common rules](../cli.md)
