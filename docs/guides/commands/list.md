# kotowari list

English | [日本語](list.ja.md)

<!-- @kotowari[docs/decisions/core/list.md#list-reads-the-same-things-as-check-reports-no-findings-and-outputs-every-decision-and-confirmation-row-it-could-read:f863a2d6] -->

`kotowari list` lists the decisions and confirmation rows of the decision documents (the specification), together with the tests whose marks point at each row.
Use it when you want to see which tests are attached to which row, to find a decision's target and fingerprint for a guide mark, or to hand the whole specification to an LLM in one go.
It reports no findings. To see findings, use [`kotowari check`](./check.md).

## Synopsis

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/cli.md#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error:11a07c33] -->

```sh
kotowari list [--format json|text] [--config <path>]
kotowari list --help
kotowari list --version
```

Options can be written before or after the command (`kotowari --format text list` means the same thing).
It takes no positional arguments.

## Options and arguments

<!-- @kotowari[docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

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

What it reads is the same as `kotowari check` (the same configuration, and the decision documents and test files in the same locations).
However, it does not read guides or overview data.
Only when `languages` lists two or more languages does it also walk the locations of guides, overview data and the table of contents, to output the hashes in `translations` (it does not check their contents).
Decisions and rows come only from the first-language side of each pair.

### text

<!-- @kotowari[docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7] -->

Each entry is printed on one line, followed by its marked tests, one per line, indented.

```text
ID 検証 名前 パス:行 tests=数
  パス:行 テストの名前
```

| Field | Content |
|---|---|
| ID | For a decision, its target (`<path>#<anchor>`, the value a guide mark copies); for a row, its ID |
| Verification (検証) | `reading` for a row that starts with `(by reading)`, `-` otherwise |
| Name (名前) | For a decision, its heading; for a row, its text after the ID (and after `(by reading)`) |
| Path:line (パス:行) | The line of the decision's heading, or of the row |
| tests=count (tests=数) | The number of tests whose marks include that ID. Always 0 for a decision, since tests point at rows |
| Test lines | `path:line name`, indented by two half-width spaces. The path and line are the mark's position. `-` when the name cannot be obtained |

When `languages` lists two or more languages, after the entry lines comes one line per pair: the path of the first-language side, followed, for each language, by a single half-width space and `language-tag=blob hash`.
A missing side is shown as `language-tag=-`.

```text
docs/decisions/shop/cart.md en=86fedceb857d3e80a36805d0a55533190c16ced5 ja=2246e3d2ff17d246033d32bcb65c8bb8717bd840
guides/a.md en=247728873932ff94b66350209bc6f7e598bad23c ja=-
```

### JSON

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7, docs/decisions/core/list.md#with-two-or-more-languages-the-blob-hash-of-each-pair-is-output-too:8b3ec16c, docs/decisions/core/list.md#pairs-of-decision-documents-are-output-in-translations-too:681f0a2a] -->

The top level is an object with `items`, plus `translations` only when `languages` lists two or more languages.
`items` is an array of objects, one per entry, and the keys each one has depend on its kind.

| Key | Type | Kinds that have it | Description |
|---|---|---|---|
| `id` | string | Both | For a decision, its target `<path>#<anchor>`; for a row, its ID |
| `kind` | string | Both | `decision` or `confirmation` |
| `name` | string | Both | For a decision, its heading; for a row, its text after the ID and after `(by reading)`, with wrapped lines joined by one space |
| `path` | string | Both | Path of the decision document, relative to the base directory |
| `line` | number | Both | The line of the decision's heading, or of the row |
| `state` | string or null | Both | `proposed` or `implemented` (for a row, the state of its decision); null when the state line is wrong |
| `fingerprint` | string | Decision | The fingerprint (8 hex digits). The value you copy into a [guide mark](../writing-guides.md) |
| `decision` | string | Row | The heading of the decision holding the row |
| `by_reading` | boolean | Row | true for a row that starts with `(by reading)` |
| `tests` | array of objects | Row | The tests whose marks include this ID. Each has `path`, `line` (the mark's line) and `name` (the test's name, or null) |

If the same test has two marks with the same ID, it appears twice in `tests`.
A row whose ID is malformed is not listed.
The detailed definition is in [the list decisions](../../decisions/core/list.md).

`translations` is an array with one entry per pair, in ascending `path` order: the pairs of decision documents, guides, overview data and the table of contents.
A pair whose first-language side is missing still gets an entry.
Each entry has `path` (the path of the first-language side, relative to the base directory) and `sides`.
`sides` is an array of sides in `languages` order; each side has `language` (the language tag), `path`, and `blob` (that side's git blob hash, or null if the side does not exist).
Copy `blob` into the consistency record `<stem>.i18n.yaml` ([Languages and pairs](../config.md#languages-and-pairs--languages-and-labels)).

```json
{"path":"guides/a.md","sides":[{"language":"en","path":"guides/a.md","blob":"247728873932ff94b66350209bc6f7e598bad23c"},{"language":"ja","path":"guides/a.ja.md","blob":null}]}
```

### Ordering

<!-- @kotowari[docs/decisions/core/list.md#the-order-is-by-path-and-by-line-within-the-same-path:37893e8e] -->

`items` is in ascending `path` order, and within the same `path`, in ascending `line` order, so each decision is followed by its rows.
The `tests` of each entry are likewise ordered by `path`, then `line`.

## Exit codes

<!-- @kotowari[docs/decisions/core/list.md#list-reads-the-same-things-as-check-reports-no-findings-and-outputs-every-decision-and-confirmation-row-it-could-read:f863a2d6, docs/decisions/core/list.md#list-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces:7fe55f1b] -->

| Code | Meaning |
|---|---|
| 0 | Read successfully (0 even if the decision documents have errors). Also 0 when it exits after `--help` or `--version` |
| 2 | Stopped (a configuration error, an unreadable file, an argument error, and so on; the reason is printed to standard error) |

`list` never returns 1.
The commands whose exit code depends on whether there are errors are `check` and `status`.
The reasons and messages for stopping are the same as for `check` ([CLI common rules](../cli.md)).

## Example

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7] -->

This is the result of running it on a small decision document with two implemented decisions and three rows, one of them confirmed by reading, and a test with one mark.

The key parts of the decision document `docs/decisions/shop/discount.md`:

```markdown
## Members get 10% off their orders

Status: implemented
...
- C-shop-discount-1 A member's order of 1000 is priced at 900

## Orders of non-members are not discounted

Status: implemented
...
- C-shop-discount-2 A guest's order of 1000 is priced at 1000
- C-shop-discount-3 (by reading) The price list page says that only members get the discount
```

The test `tests/discount.rs`:

```rust
// @kotowari[C-shop-discount-1]
#[test]
fn member_gets_ten_percent_off() {}
```

As text, the output is:

```console
$ kotowari list --format text
docs/decisions/shop/discount.md#members-get-10-off-their-orders - Members get 10% off their orders docs/decisions/shop/discount.md:5 tests=0
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted - Orders of non-members are not discounted docs/decisions/shop/discount.md:21 tests=0
C-shop-discount-2 - A guest's order of 1000 is priced at 1000 docs/decisions/shop/discount.md:35 tests=0
C-shop-discount-3 reading The price list page says that only members get the discount docs/decisions/shop/discount.md:36 tests=0
```

In JSON (the default), the same content comes out with keys. The first two entries, a decision and a row, are shown.

```console
$ kotowari list | jq '.items[0], .items[1]'
{
  "id": "docs/decisions/shop/discount.md#members-get-10-off-their-orders",
  "kind": "decision",
  "name": "Members get 10% off their orders",
  "path": "docs/decisions/shop/discount.md",
  "line": 5,
  "state": "implemented",
  "fingerprint": "e3adc927"
}
{
  "id": "C-shop-discount-1",
  "kind": "confirmation",
  "name": "A member's order of 1000 is priced at 900",
  "path": "docs/decisions/shop/discount.md",
  "line": 19,
  "decision": "Members get 10% off their orders",
  "state": "implemented",
  "by_reading": false,
  "tests": [
    {
      "path": "tests/discount.rs",
      "line": 1,
      "name": "member_gets_ten_percent_off"
    }
  ]
}
```

Common filters can be written with `jq`. The rows that still need a test:

```console
$ kotowari list | jq -r '.items[] | select(.kind == "confirmation" and .tests == [] and (.by_reading | not)) | .id'
C-shop-discount-2
```

## Common pitfalls

### The exit code is 0 even though the decision documents have errors

<!-- @kotowari[docs/decisions/core/list.md#list-reads-the-same-things-as-check-reports-no-findings-and-outputs-every-decision-and-confirmation-row-it-could-read:f863a2d6] -->

`list` is a read-only command: it reports no findings, and even when there are errors it outputs the decisions and rows it could read and exits with 0.
For example, if you change the state line of the first decision in the example to `Status: done`, `check` reports an error and exits with 1, but `list` still lists that decision and its row and exits with 0.

```console
$ kotowari check --format text
docs/decisions/shop/discount.md:7 [error] decision_state_invalid done
docs/decisions/shop/discount.md:35 [error] confirmation_without_test C-shop-discount-2
$ kotowari list --format text
docs/decisions/shop/discount.md#members-get-10-off-their-orders - Members get 10% off their orders docs/decisions/shop/discount.md:5 tests=0
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
...
$ echo $?
0
```

To fail CI on errors, use `kotowari check` or [`kotowari status`](./status.md).

### The test name is `-` (null in JSON)

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66] -->

In test files of a language without a query, kotowari picks up only the marks and does not obtain test names.
`path` and `line` are the mark's position.
Which extensions have a query is shown by `query` under `tests` in the JSON of `kotowari check` ([CLI common rules](../cli.md)).

### A decision always shows `tests=0`

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

Tests point at confirmation rows, not at decisions, so the line of a decision shows `tests=0` and its JSON entry has no `tests`.
To see whether a decision is confirmed, look at the rows listed under it.
A row that starts with `(by reading)` (`reading` in text) needs no test, and the rows of a proposed decision do not need one yet.

### check and status stop on a configuration error, but list runs

<!-- @kotowari[docs/decisions/core/list.md#list-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces:7fe55f1b, docs/decisions/core/guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them:05ee20e8, docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

`list` does not read guides, so it does not stop for reasons tied to the guide locations (such as `guides.files` overlapping `tests.files`).
However, when `languages` lists two or more languages, it walks the locations of guides, overview data and the table of contents, so stops caused by those locations and by reading them behave the same as in `check`.
It also does not read surface files, surface rule files or the list of unspecified surfaces, so it does not stop when they are missing, unreadable or broken.
It does not read overview data either, so it does not stop for reasons tied to its location or reading it (such as `overview.files` overlapping the guide or test locations).
However, `list` does stop on errors in how configuration keys are combined, such as writing only one of `surface.files` and `surface.rules` ([Configuration](../config.md)).
The other conditions for stopping are the same as for `check`.
When you fix the configuration, confirm it with `kotowari check`.

## Related

- Specification: [the list decisions](../../decisions/core/list.md)
- See one entry with its body and reverse references: [`kotowari query`](./query.md)
- See overall counts and whether everything is in place: [`kotowari status`](./status.md)
- See findings one by one: [`kotowari check`](./check.md)
- Shared options, stopping, the base directory: [CLI common rules](../cli.md)
- The configuration file: [Configuration](../config.md)
