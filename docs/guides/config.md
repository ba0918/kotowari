# Configuration file — `.kotowari/config.yaml`

English | [日本語](config.ja.md)

This is the YAML file that states where kotowari reads from (the IR, decision records, tests, guides, surface files, overview data) and the values the checks use.
kotowari runs with default values when the file is absent, so you write only the keys you want to change from the defaults.

## Format

<!-- @kotowari[REQ-core-013:f8877aec, REQ-core-017:4a231543, TBL-core-004:3b95b98e] -->

Written out with every key at its default value, it looks like this.
Keys are written nested (`records:` under `decisions:`).

```yaml
ir: docs/ir
decisions:
  records: docs/decision/records
  adr: docs/decision/adr
tests:
  files:
    - "src/**/*.rs"
    - "tests/**/*.rs"
  rust:
    attributes: []
    macros: []
  rules: []
guides:
  files: []
mutants:
  equivalents: .kotowari/equivalents.yaml   # 既定は「無し」。書いたときだけ読む
surface:
  files: []
  rules: []
  # unspecified: docs/surface-unspecified.yaml   # 既定は「無し」。surface.rules が空の一覧のときは書けない
# overview:                                     # 既定は「無し」。overview build と serve を使うときは書く
#   files:
#     - ".kotowari/overview/*.md"
#   toc: .kotowari/overview-toc.yaml            # 全体像の目次。overview を書くときは必須
# languages: [ja, en]                           # 既定は「無し」（英語だけ）。2つ以上なら文書を言語ごとの対で持つ
# labels:                                       # 既定は「無し」。英語でない言語の UI の文字（鍵はすべて書く）
#   ja:
#     language_name: 日本語
#     pages: "{n} ページ"
limits:
  lines: 200
  requirements: 10
vague_words: [適切に, 必要に応じて, 通常は, など]
```

The comments in the example say, in order: `mutants.equivalents` has no default and is read only when written; `surface.unspecified` has no default and cannot be written while `surface.rules` is an empty list; `overview` has no default and must be written to use `overview build` and `serve`; `overview.toc` is the overview's table of contents and is required when `overview` is written; `languages` has no default (English only), and with two or more languages documents are kept as pairs, one side per language; `labels` has no default and holds the UI text of non-English languages (all keys must be written).
The default `vague_words` are Japanese words meaning "appropriately", "as needed", "usually" and "etc.".

`mutants.equivalents`, `surface.unspecified`, `overview`, `languages` and `labels` default to "key absent".
The values in the example above show how to write them; they are not defaults.
`surface.unspecified` stops kotowari if written while `surface.rules` is an empty list, so the example comments it out with `#` ([`surface.*`](#surfacefiles-surfacerules-surfaceunspecified)).

## Key list

<!-- @kotowari[TBL-core-004:3b95b98e, REQ-core-325:ba483356] -->

| Key | Value type | Default | Description |
|---|---|---|---|
| `ir` | Path (string) | `docs/ir` | The directory holding the IR |
| `decisions.records` | Path (string) | `docs/decision/records` | The directory holding decision records. The decisions in the files under it can be pointed at as sources |
| `decisions.adr` | Path (string) | `docs/decision/adr` | The directory holding ADRs |
| `tests.files` | List of globs | `src/**/*.rs`, `tests/**/*.rs` | Where the test files are |
| `tests.rust.attributes` | List of attribute paths | Empty | Rust attributes counted as tests besides `#[test]` (e.g. `kani::proof`) |
| `tests.rust.macros` | List of macro names | Empty | Rust macros whose inner functions are counted as tests. Written without the `!` (e.g. `proptest`) |
| `tests.rules` | List of paths | Empty | YAML files of ast-grep rules that find tests. Globs are not allowed |
| `guides.files` | List of globs | Empty | Where the guides are. If empty, no guides are read |
| `mutants.equivalents` | Path (string) | None | The file of the list of equivalents. If absent, the list of equivalents has 0 entries |
| `surface.files` | List of globs | Empty | Where the code files that surfaces are taken from (surface files) are |
| `surface.rules` | List of paths | Empty | YAML files of ast-grep rules that take out surfaces. Globs are not allowed. If not an empty list, the surface check runs |
| `surface.unspecified` | Path (string) | None | The file of the list of unspecified surfaces. If absent, the list has 0 entries |
| `overview.files` | List of globs | `overview` may be omitted; required when written | Where the overview data is. Without `overview`, no overview data is read, and `overview build` and `serve` stop |
| `overview.toc` | Path (string) | `overview` may be omitted; required when written | The overview's table of contents file. It sets the nesting and order of the listing page. When `overview` is present, `check`, `status`, `overview build` and `overview serve` read it, and stop if its target is missing or unreadable |
| `limits.lines` | Positive integer | `200` | The maximum number of lines in an IR document. Exceeding it gives a `too_many_lines` notice |
| `limits.requirements` | Positive integer | `10` | The maximum number of requirements in one document. Exceeding it gives a `too_many_requirements` notice |
| `vague_words` | List of words | `適切に`, `必要に応じて`, `通常は`, `など` | Vague words. Their appearance in an IR statement gives a `vague_word` error |
| `languages` | List of language tags | None (English `en` only) | The language list. The first language is the language of files without a suffix. With two or more, the IR, guides, overview data and table of contents are kept as pairs, one side per language ([Languages and pairs](#languages-and-pairs--languages-and-labels)) |
| `labels` | Map from language tag to a map from UI text key to string | None (kotowari holds the English text) | The UI text per language. Non-English languages write every key |

All paths are written relative to the base directory ([Base directory](#base-directory)).
The full definition is TBL-core-004 in [the config IR](../ir/core/config.md).

## Languages and pairs — `languages` and `labels`

<!-- @kotowari[REQ-core-334:c2dc1692, REQ-core-335:b51997f0, REQ-core-336:03710aae, REQ-core-337:40375497, REQ-core-339:81c3b742, REQ-core-351:bfccf5f2, REQ-core-352:7b5a3a7f, TBL-core-046:105022e3] -->

`languages` is a sequence of language tags (e.g. `[ja, en]`).
A language tag consists only of lowercase English letters, digits and `-`; an empty string, a language tag containing other characters, or a second occurrence of the same language tag stops kotowari with a configuration error.
If the key is absent or an empty list, it is taken as English `en` only.
The first language is called the "first language", and its files have names without a suffix (`foo.md`).

With two or more languages, the documents in the IR location (including `CONTEXT.md` and `FLAGS.md`), the guides, the overview data and the table of contents are read as sets of files, one per language (pairs).
Files in the decision record and ADR locations are not paired.

- The files of other languages go in the same directory, named with the language tag inserted before the extension (`foo.en.md`, `toc.en.yaml`). They are found by name, whether or not they match the globs of `guides.files` or `overview.files`
- `foo.ja.md` carrying the first language's tag, or `foo.fr.md` for a language not in the list, is read as a plain document `foo.ja.md`
- Next to the first language's file, put a consistency record `foo.i18n.yaml`. It is a map whose keys are each language's file name (without the directory) and whose values are the git blob hash of that file (40 lowercase hexadecimal characters, the same as `git hash-object`) as of the last time it was confirmed to have the same content. Copy the values from `translations` in `kotowari list` ([list](commands/list.md))
- A missing side, an error in the consistency record, a hash different from the record, a mismatch in the skeleton (the parts other than sentences), an error in the switcher line, and a link to another language's side or to a decision record are each errors ([Findings](findings.md))

```yaml
# docs/ir/a.i18n.yaml（値は kotowari list の translations から写す。ここでは例）
a.md: 78981922613b2afb6025042ff6bd878ac1994e85
a.en.md: 0c3b7c0b5b2f3a4d1e8f6a9b2c4d5e6f7a8b9c0d
```

(The comment says the values are copied from `translations` in `kotowari list`, and these are examples.)

`labels` is the UI text per language.
UI text is text kotowari writes into overview pages and switcher lines that is in neither the overview data nor the IR.
kotowari holds the UI text for English `en`, and only the keys written in `labels.en` replace it.
For a non-English language, write every one of the following keys in `labels.<language tag>`.
A missing key, an unknown key, a language tag not in `languages`, or a value of a key that takes a number that does not contain exactly one `{n}` stops kotowari with a configuration error.
Even a project that does not use overviews needs `labels` for non-English languages, because `language_name` is used in the switcher line.

| Key | Where it is used | Takes a number | English text |
|---|---|---|---|
| `language_name` | The language's name written in the switcher line and in the overview's links to other languages | No | `English` |
| `index_link` | The link to the listing from a page not named in the table of contents | No | `Overview` |
| `pages` | The number of pages in a group of the listing's table of contents | Yes | `{n} pages` |
| `stale_sections` | The number of stale sections in the listing | Yes | `{n} sections to review` |
| `open_items` | The number of items with the `open` tag in the listing | Yes | `{n} open` |
| `planned_items` | The number of items with the `planned` tag in the listing | Yes | `{n} planned` |
| `stale_mark` | The marker near the heading of a stale section | No | `Not reviewed since the IR changed` |
| `outline_stale` | The marker of a stale section in the outline | No | `not reviewed` |
| `superseded` | The marker of a superseded reference | No | `(superseded)` |
| `deferred` | The marker of a deferred reference | No | `(deferred)` |
| `compare_before`, `compare_after`, `compare_why` | Column headings of the compare part | No | `Before`, `After`, `Why` |
| `state_decided`, `state_planned`, `state_open`, `state_dropped` | How the status part's tags `decided`, `planned`, `open`, `dropped` are shown | No | `Decided`, `Planned`, `Open`, `Dropped` |

## Where the configuration file is

<!-- @kotowari[REQ-core-011:0b7f52a9, REQ-core-003:8ac6759c, REQ-core-020:2b67aa66] -->

| What you specify | File read |
|---|---|
| No `--config` | `.kotowari/config.yaml` in the base directory |
| `--config <path>` | `<path>`, read as a path relative to the current directory |

- `kotowari plan` does not read the configuration file. Passing `--config` stops it with an argument error.
- A `kotowari.toml` at the top of the repository is not read.

When the target of `--config` does not exist, kotowari stops with an argument error.

```console
$ kotowari check --config nope.yaml
argument error: config file not found: nope.yaml
$ echo $?
2
```

## Base directory

<!-- @kotowari[REQ-core-009:6b5a71a4, TBL-core-003:603e9601, PROP-core-001:9be33697, REQ-core-010:5217a2ba] -->

Paths in configuration values, paths in sources, and the `path` in the output are all relative to the base directory.
The base directory is determined in this order.

| Order | Condition | Base directory |
|---|---|---|
| 1 | Going upward from the current directory, a directory containing a `.kotowari/` directory is found | The first directory found |
| 2 | None is found | The current directory |

A file named `.kotowari` is ignored and the search continues upward.
Passing a different file to `--config` does not change the base directory.

Even when run in a subdirectory, paths are printed relative to the base directory.

```console
$ cd tests
$ kotowari list --format text
REQ-001 unit 会員の割引 docs/ir/discount.md:7 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
REQ-002 unit 会員でない注文 docs/ir/discount.md:15 tests=1
  tests/discount.rs:5 non_member_pays_full_price
EX-001 - 1000円の注文は900円になる docs/ir/discount.md:27 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
```

### How to write paths

<!-- @kotowari[REQ-core-110:04555fb0, REQ-core-010:5217a2ba] -->

Paths in configuration values are normalized before they are compared and before they are printed.
A trailing `/` and a leading `./` are removed, `//` and `/./` are folded into a single `/`, `\` becomes `/`, and `a/..` is folded.
So `./docs/ir/` and `docs/ir` are the same location.
Absolute paths starting with `/` cannot be written ([Configuration errors](#configuration-errors)).

## When the file is missing or empty

<!-- @kotowari[REQ-core-012:50c68e4b] -->

| Situation | Behavior |
|---|---|
| No `--config` and no `.kotowari/config.yaml` | Checks with the default values |
| The file is empty (0 bytes or only comments) | Checks with the default values. The same for a file named by `--config` |
| The target of `--config` does not exist | Stops with an argument error (exit code 2) |

## List keys

<!-- @kotowari[REQ-core-015:44b9e418, REQ-core-016:cca736db] -->

List keys (`tests.files`, `guides.files`, `vague_words` and so on) take only lists.
A list you write **replaces** the default list rather than being added to it.

```yaml
tests:
  files: ["spec/**/*.rs"]   # 既定の src/**/*.rs と tests/**/*.rs は読まれなくなる
```

(The comment says the default `src/**/*.rs` and `tests/**/*.rs` are no longer read.)

An empty list (`[]`) is accepted and means "no elements".
With `vague_words: []`, the vague word check reports nothing.

## How globs are read

<!-- @kotowari[REQ-core-019:10278003, REQ-core-079:589c548b] -->

The globs of `tests.files`, `guides.files`, `surface.files` and `overview.files` are read as follows.

| Target | Handling |
|---|---|
| `**` | Recursive (directories at any depth) |
| Hidden directories (starting with `.`) | Not included, even when the glob names them |
| Hidden files | Read if the glob matches them |
| Symbolic links to directories | Not followed |
| Symbolic links to files | Read. Stops if the target does not exist |
| Sockets, named pipes, devices | Not read |

As an exception, `overview.files` for overview data reads hidden directories named explicitly by a path component (such as `.kotowari` in `.kotowari/overview/*.md`). Hidden paths that are not named are excluded even by a broad `**`, and a hidden directory named only inside braces (`{.overview,other}/*.md`) does not count as named.

The scan walks the whole base directory (except hidden directories) and then selects with the globs.
So even an unreadable directory in a place no glob matches stops kotowari.

## Notes per key

### `ir`, `decisions.records`, `decisions.adr`

<!-- @kotowari[REQ-core-018:6ea3e08f] -->

`kotowari check` stops with the unreadable file reason when the target does not exist, is not a directory, or cannot be read.

```console
$ cat .kotowari/config.yaml
ir: docs/nothing
$ kotowari check --format text
unreadable file: docs/nothing: No such file or directory (os error 2)
$ echo $?
2
```

If the location exists but simply has no documents in it, the check runs without stopping.

### `tests.rust.attributes` and `tests.rust.macros`

<!-- @kotowari[TBL-core-017:aff9f804, EX-core-017:d7ca524e, EX-core-018:0b4c0d06] -->

| Key | How to write it | How it matches |
|---|---|---|
| `tests.rust.attributes` | The path without `#[`, `]` and the arguments (`kani::proof`) | Exact match of the path. `#[kani::proof(unwind = 3)]` matches too |
| `tests.rust.macros` | The name without `!` (`proptest`) | Match of the last element of the name. `proptest::proptest!` matches too |

`#[test]` (an attribute whose path ends in `test`) is always counted regardless of the configuration, so you do not need to write it.
For usage, see [Marking tests](marks.md#tests-inside-a-macro-are-not-counted).

### `tests.rules`

<!-- @kotowari[REQ-core-121:6502b5d5, REQ-core-186:cfb141ec, REQ-core-187:a464f692, REQ-core-188:e1781d9c] -->

Adds, through ast-grep rules, tests of shapes the bundled queries do not find.
The listed paths are relative to the base directory, and globs are not allowed.
One file may hold several rules separated by `---`.

| Rule field | How kotowari treats it |
|---|---|
| `language` | The language the rule is added to. Case-insensitive, and aliases such as `ts` and `py` are accepted |
| `rule` | Each matched syntax-tree node counts as one test. The text captured by the metavariable `$NAME` becomes the test's name |
| `files`, `ignores` | Read the same way as in ast-grep, and narrow by matching against the test file's relative path |
| `fix`, `message`, `severity`, `note`, `metadata` | Not used. Rules with `severity: off` are applied too |

The bundled queries cannot be removed. Rules are only added to the bundled queries.

As an example, add a rule that counts TypeScript's `bench(...)` as a test.

```yaml
# .kotowari/config.yaml
tests:
  files:
    - "tests/**/*.rs"
    - "tests/**/*.ts"
  rules:
    - rules/bench.yml
```

```yaml
# rules/bench.yml
id: bench
language: typescript
rule:
  pattern: bench($NAME, $$$)
```

`tests/discount.bench.ts`:

```ts
// @kotowari[REQ-001]
bench('member discount is fast', () => {});

bench('non-member is fast', () => {});
```

```console
$ kotowari check --format text
tests/discount.bench.ts:4 [error] test_without_id non-member is fast
```

The second `bench`, which has no mark, is found as a test, and `test_without_id` is reported.
You can try the rule file as is with `ast-grep scan -r rules/bench.yml`.

### `guides.files`

<!-- @kotowari[REQ-core-198:253e79fd, REQ-core-199:55922b15] -->

Where the guides are. For how to write them, see [Writing guides](writing-guides.md).

- If the key is absent or an empty list, no guide is read.
- Only `kotowari check` and `kotowari status` read guides.
- If one file matches both `guides.files` and `tests.files`, kotowari stops with a configuration error ([pitfall](#stopping-because-the-guide-and-test-locations-overlap)).
- Overlapping the IR or decision record locations is fine. Such files are also read as guides.

### `surface.files`, `surface.rules`, `surface.unspecified`

<!-- @kotowari[REQ-core-224:f13b971e, REQ-core-225:be4cdd0b, REQ-core-229:7909d808] -->

Settings for the check that takes user-visible surfaces (CLI subcommands and flags, configuration keys and so on) out of the code and confirms they are written in the IR.
For usage, see [Surface check](surface.md).

- The globs of `surface.files` are read and scanned the same way as `tests.files`. Only files in a surface rule's language are read, and if such a file is unreadable or not UTF-8, kotowari stops the same way as for test files. Other files are not read, so writing something broad like `src/**` that matches images and the like does not stop it.
- `surface.rules` is written the same way as `tests.rules`, and stops on file errors under the same conditions. Surface rules, however, are not added to the queries that find tests.
- Write `surface.files` and `surface.rules` as a pair. When only one of them is written, or when `surface.unspecified` is written while `surface.rules` is empty, every command that reads the configuration stops with a configuration error.
- Only `kotowari check` and `kotowari status` read surface files, surface rule files and the list of unspecified surfaces.

```console
$ cat .kotowari/config.yaml
surface:
  rules: ["rules/surface.yml"]
$ kotowari check --format text
config error: .kotowari/config.yaml: surface.rules is set but surface.files is empty
```

### `mutants.equivalents`

<!-- @kotowari[REQ-core-148:9dc7ec0a] -->

Points at the file of the list of equivalents. Only `kotowari mutants` reads it.
`kotowari check` only checks the form of the value (that it is a string and not an absolute path), not whether the file exists.
For how to write the list, see [kotowari mutants](commands/mutants.md).

### `limits.lines`, `limits.requirements`, `vague_words`

<!-- @kotowari[REQ-core-038:170fdd3e, REQ-core-039:f3d75ba1, REQ-core-066:79261ab9] -->

| Key | Finding when exceeded or matched | Severity |
|---|---|---|
| `limits.lines` | `too_many_lines` (number of lines in an IR document) | Notice |
| `limits.requirements` | `too_many_requirements` (number of requirements in a topic document) | Notice |
| `vague_words` | `vague_word` (a target line contains the word as a substring) | Error |

```console
$ cat .kotowari/config.yaml
limits:
  lines: 20
vague_words: [割引]
$ kotowari check --format text
docs/ir/discount.md:- [notice] too_many_lines 31
docs/ir/discount.md:13 [error] vague_word 割引
docs/ir/discount.md:21 [error] vague_word 割引
$ echo $?
1
```

## Configuration errors

<!-- @kotowari[REQ-core-014:4f851c96, EX-core-003:a69bc60c, EX-core-383:dac73d9d, REQ-core-225:be4cdd0b, REQ-core-280:1b9f50df, TBL-core-004:3b95b98e, REQ-core-326:fb819a40, REQ-core-335:b51997f0, REQ-core-352:7b5a3a7f] -->

If any of the following is present, kotowari stops without checking (exit code 2).
The first line of standard error starts with `config error: `, followed by the configuration file's path and the reason.
For errors in the files of `tests.rules` and `surface.rules`, errors in the list of unspecified surfaces, and overlapping locations, the path shown is that file's, not the configuration file's (see the examples below).

| Situation | Example |
|---|---|
| Cannot be read as YAML | `ir: [` |
| Unknown key | `limit:` (should be `limits:`) |
| Second occurrence of the same key | `ir:` on two lines |
| Key whose value is null | A line with only `ir:` |
| Value of the wrong type | `files: "tests/**/*"` (a string, not a list) |
| Negative number, 0 | `lines: 0` |
| Absolute path (starting with `/`) | `ir: /docs/ir` |
| Empty string, or second occurrence of the same word, in `vague_words` | `vague_words: [""]` |
| An element of `tests.files`, `guides.files`, `surface.files` or `overview.files` that cannot be read as a glob | `files: ["tests/[a"]` |
| Only one of `surface.files` and `surface.rules`, or `surface.unspecified` without `surface.rules` | [`surface.*`](#surfacefiles-surfacerules-surfaceunspecified) |
| The guide and test locations overlap | [pitfall](#stopping-because-the-guide-and-test-locations-overlap) |
| The overview data location overlaps the guide or test location | `config error: docs/a.md: matched by both overview.files and guides.files` (`tests.files` for tests) |
| `overview` has no `toc` | `config error: .kotowari/config.yaml: overview.toc is required` |
| The table of contents file is among the files read by the scan of `overview.files`, `guides.files` or `tests.files` | `config error: .kotowari/overview/toc.md: matched by both overview.toc and overview.files` (of the keys that matched, the first in this order. Put the table of contents outside the globs, for example by making it `.yaml`) |
| An empty string in `languages`, a language tag containing characters other than lowercase English letters, digits and `-`, or a second occurrence of the same language tag | `config error: .kotowari/config.yaml: invalid language tag: "EN"` |
| An error in `labels` (a language tag not in `languages`, an unknown key, a missing key for a non-English language, or text for a number that does not contain exactly one `{n}`) | `config error: .kotowari/config.yaml: labels.ja.stale_mark: missing` |

An empty list (`files: []`) is not an error.

Here are examples of the actual output.

```console
$ cat .kotowari/config.yaml
ir:
$ kotowari check --format text
config error: .kotowari/config.yaml: null value for key: ir
```

```console
$ cat .kotowari/config.yaml
ir: /docs/ir
$ kotowari check --format text
config error: .kotowari/config.yaml: absolute path not allowed for ir: /docs/ir
```

```console
$ cat .kotowari/config.yaml
tests:
  files: ["tests/[a"]
$ kotowari check --format text
config error: .kotowari/config.yaml: invalid glob pattern: tests/[a
```

```console
$ cat .kotowari/config.yaml
ir: docs/ir
ir: docs/ir
$ kotowari check --format text
config error: .kotowari/config.yaml: duplicate key: ir
```

A second occurrence of the same key shows only that key's name, on one line.
Other errors that come from reading the YAML (unknown key, wrong type, 0) are followed by an explanation with line and column ([pitfall](#stopping-on-an-unknown-key)).

### Errors in `tests.rules` files

<!-- @kotowari[REQ-core-189:5b4f081a, EX-core-315:92402027, EX-core-378:e7924545, EX-core-379:8e774593] -->

kotowari also stops with a configuration error when:

- The file does not exist, is not a file, cannot be read, or is not UTF-8
- The same path is listed twice
- It cannot be read as YAML, or as an ast-grep rule
- `language` is a language kotowari does not know (the language list is in [Marking tests](marks.md#which-tests-are-found))

```console
$ cat .kotowari/config.yaml
tests:
  rules: [rules/missing.yml]
$ kotowari check --format text
config error: unreadable file in tests.rules: rules/missing.yml: No such file or directory (os error 2)
```

```console
$ cat rules/c.yml
id: c
language: cobol
rule:
  pattern: x
$ kotowari check --format text
config error: invalid rule in tests.rules: rules/c.yml: unknown language: cobol
```

Duplicate rule `id`s are not an error.

## Common pitfalls

### Stopping on an unknown key

<!-- @kotowari[REQ-core-014:4f851c96, EX-core-003:a69bc60c] -->

```console
$ cat .kotowari/config.yaml
ir: docs/ir
limit:
  lines: 100
$ kotowari check --format text
config error: .kotowari/config.yaml: error: line 2 column 1: unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, overview, vague_words, languages, labels
 --> <input>:2:1
  |
1 | ir: docs/ir
2 | limit:
  | ^ unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, overview, vague_words, languages, labels
3 |   lines: 100
  |
$ echo $?
2
```

The key is misspelled. After `expected one of` come the keys you can write at the top level.
Writing a nested key (`decisions.records`) on one line as `decisions.records:` also makes it an unknown key.

### Adding a glob for tests stopped the original tests from being read

<!-- @kotowari[REQ-core-015:44b9e418] -->

Writing `tests.files` removes the defaults `src/**/*.rs` and `tests/**/*.rs`.
If you want the original locations read as well, list them all.

```console
$ cat .kotowari/config.yaml
tests:
  files: ["spec/**/*.rs"]
$ kotowari check --format text
docs/ir/discount.md:7 [error] requirement_without_test REQ-001
docs/ir/discount.md:15 [error] requirement_without_test REQ-002
docs/ir/discount.md:26 [error] scenario_without_test EX-001
$ kotowari status --format text | grep '^tests'
tests marks=0
```

The `tests` line of `status` lets you check the number of test files read, per extension.
If it says `marks=0` with no per-extension counts, the globs matched nothing.

### Stopping because the guide and test locations overlap

<!-- @kotowari[REQ-core-199:55922b15, EX-core-368:d4ff36cf] -->

```console
$ cat .kotowari/config.yaml
tests:
  files: ["**/*"]
guides:
  files: ["guides/**/*.md"]
$ kotowari check --format text
config error: guides/a.md: matched by both guides.files and tests.files
$ echo $?
2
```

Of the overlapping files, the first one in byte order of the path is shown.
Narrow `tests.files`, for example to `tests/**/*`, so that it does not match the guides.
After fixing and running again, the next file is shown if there are other overlaps.

### `kotowari.toml` is not read

<!-- @kotowari[REQ-core-020:2b67aa66] -->

The configuration is `.kotowari/config.yaml` only.
A `kotowari.toml` at the top is not read even if its content has errors, and the check runs with the default values.

## Related

- Specification: [config IR](../ir/core/config.md), [base directory IR](../ir/core/base-directory.md), [IR of queries added through configuration](../ir/core/query-rules.md)
- Syntax, exit codes and stops common to all commands: [CLI](cli.md)
- How tests are found, and marks: [Marking tests](marks.md)
- Guide locations and marks: [Writing guides](writing-guides.md)
- Finding kinds: [List of findings](findings.md)
