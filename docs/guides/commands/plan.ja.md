# kotowari plan

[English](plan.md) | 日本語

計画書（実装の手順を書いた Markdown の文書）を1つ読み、kotowari の本体に同梱したスキーマで形を検査します。
計画書を書き終えたとき、その形を確かめるのに使います。

## 書式

<!-- @kotowari[docs/decisions/core/plan.md#kotowari-plan-reads-no-configuration-and-handles-arguments-and-unreadable-files-in-the-same-way-as-mutants:b92c7955, docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd] -->

```sh
kotowari plan [--format json|text] <計画書のファイル>
```

オプションはコマンドの前にも、ファイルの後にも書けます。
`--config` は受けません（付けると引数の誤りで止まります）。

## オプションと引数

<!-- @kotowari[docs/decisions/core/plan.md#kotowari-plan-reads-no-configuration-and-handles-arguments-and-unreadable-files-in-the-same-way-as-mutants:b92c7955, docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/output.md#--format-accepts-json-and-text-with-json-as-the-default:811394a9] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `<計画書のファイル>` | パス | なし（必須） | 検査する計画書。ちょうど1つ。カレントディレクトリからの相対パスとして読む |
| `--format` | `json` か `text` | `json` | 出力の形 |
| `--help` / `--version` | なし | | 使い方か版を出して終わる |

## 読むもの

<!-- @kotowari[docs/decisions/core/plan.md#kotowari-plan-reads-no-configuration-and-handles-arguments-and-unreadable-files-in-the-same-way-as-mutants:b92c7955, docs/decisions/core/plan.md#kotowari-plan-checks-the-form-of-a-plan-against-the-schema-bundled-in-the-binary:8c47a326] -->

`plan` が読むのは、渡した計画書のファイルだけです。
設定ファイル、IR、判断の記録、テストのファイルは読みません。
そのため、設定ファイルが壊れていても `plan` は止まりません。

形の決まり（スキーマ）はコンパイル時に本体へ取り込んだもので、実行時にスキーマのファイルを読みません。
計画書の先頭の frontmatter は、中身を問わず読まずに飛ばします。
frontmatter の `$schema` でスキーマを指しても使われず、指摘にも停止にもなりません。

## 計画書の形

<!-- @kotowari[docs/decisions/core/plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields:f10ab09f] -->

計画書は1行ずつ読まれ、次の形を満たすときだけ誤りがありません。

```markdown
# <題名>                       ← ちょうど1つ。題名と最初の ## の間は空行だけ

## Goal
## Specification
## Approach and why
## Scope of change
## Step order and prerequisites
## Verification map
## Left to the implementer
## Stop conditions
## Test command                 ← この節だけ 0 個か 1 個
## Out of scope
## Steps

### S1: <このステップで作るもの>
- Purpose: ...
- Specification: ...
- Prerequisites: ...
- May change: ...
- Done when: ...
- Shown by: test | check | artifact | external ...
- Left to the implementer: ...
- Stop and hand back if: ...
```

`## Test command` を除く10の節はちょうど1つずつ必要で、ほかの `## ` の節は置けません。
全体の例は [例](#例) にあります。

### 計画全体の節

<!-- @kotowari[docs/decisions/core/plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields:f10ab09f, docs/decisions/core/plan.md#under-a-step-only-field-lines-are-placed-and-plan-wide-sections-can-hold-statements-lists-tables-and-code-blocks:12358175] -->

`## Steps` 以外の節の下に置けるもの・置けないものです。

| 置ける | 置けない |
|---|---|
| 文 | 番号付きの一覧（`1. ...`） |
| 番号の無い一覧（`-`）とその子の一覧 | `### ` とそれより深い見出し |
| 表 | |
| コードブロック | |

### ステップ

<!-- @kotowari[docs/decisions/core/plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields:f10ab09f, docs/decisions/core/plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail:1608f12c, docs/decisions/core/plan.md#under-a-step-only-field-lines-are-placed-and-plan-wide-sections-can-hold-statements-lists-tables-and-code-blocks:12358175] -->

`## Steps` の節の決まりです。

| 決まり | 内容 |
|---|---|
| ステップの見出し | `### S` と1桁以上の数字の後に `:` がある（`### S1: 入力を読む`）。1つ以上必要 |
| 最初のステップより前 | 空でない行もほかの見出しも置けない |
| 欄 | `Purpose`、`Specification`、`Prerequisites`、`May change`、`Done when`、`Shown by`、`Left to the implementer`、`Stop and hand back if` を、`- 名前: 値` の一覧の行でこの順に1回ずつ書く。一覧の記号は `-`、`*`、`+` のどれでもよい |
| 欄のほかの行 | ステップの下には、8つの欄の行のほかに空でない行を置けない。値は1行に書く |
| `Shown by` の値 | `test`、`check`、`artifact`、`external` のどれかの語で始まり、その語の後は空白か値の終わり |

### 検査しないこと

<!-- @kotowari[docs/decisions/core/plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields:f10ab09f, docs/decisions/core/plan.md#the-order-of-sections-the-step-numbers-and-the-names-in-step-headings-are-not-checked:172d9d8f] -->

次のことは検査しません。

- 節の並び順
- ステップの番号が連番か、同じ番号のステップが2つあるか
- ステップの見出しの `:` の後の名前
- 欄の中身（要求の ID が実在するか、`Done when` が観測できる条件か、など）

要求の ID の実在は `kotowari query` で確かめます。

## 出力

### text

<!-- @kotowari[docs/decisions/core/plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail:1608f12c, docs/decisions/core/plan.md#the-output-of-kotowari-plan-uses-the-same-finding-keys-as-check-and-the-json-has-only-findings-and-counts:b227d6e0, docs/decisions/core/output.md#--format-text-outputs-one-finding-per-line:b093e38d] -->

指摘を1件1行で `パス:行 [error] invalid_plan 詳細` の形で出し、ほかの行は出しません。
指摘が0件のときは何も出しません（集計の行はありません）。

```text
docs/plans/broken.md:69 [error] invalid_plan field_pattern_mismatch: value "manual — read the output" does not match pattern "^(test|check|artifact|external)(\s|$)"
```

ステップに必須の欄が無いときは、そのステップの見出しの行を指します。
次は、45行目の `### S1: 入力を読む` のステップに `- Done when:` の行が無い計画書の出力です。

```text
docs/plans/a.md:45 [error] invalid_plan missing_required_field: field "Done when" is required but missing
```

| 部分 | 中身 |
|---|---|
| パス | 計画書のファイルの、基準のディレクトリからの相対パス。正規化し、基準の外なら `../` を含む |
| 行 | スキーマの側が出した行。ステップに必須の欄が無いときはそのステップの見出しの行。行が無ければ `-` |
| 詳細 | スキーマの側の種類と詳細を `: ` でつないだもの（`field_pattern_mismatch: ...`） |

指摘の種類は `invalid_plan` の1つだけで、重さは誤りです。
何が悪いかは詳細の前半（スキーマの側の種類）で見分けます。

### JSON

<!-- @kotowari[docs/decisions/core/plan.md#the-output-of-kotowari-plan-uses-the-same-finding-keys-as-check-and-the-json-has-only-findings-and-counts:b227d6e0] -->

最上位は `findings` と `counts` の2つの鍵だけです。
`check` の `files`、`lines`、`tests`、`guides` はありません。

| 鍵 | 型 | 説明 |
|---|---|---|
| `findings` | 配列 | 指摘の一覧。1件の鍵は `kind`、`severity`、`path`、`line`、`detail`（`check` と同じ）。`line` は行が無ければ null |
| `counts` | オブジェクト | 種類ごとの指摘の数（`{"invalid_plan": 4}`）。0件なら `{}` |

## 終了コード

<!-- @kotowari[docs/decisions/core/cli.md#the-exit-code-is-0-when-there-is-no-error-1-when-there-is-an-error-and-2-when-kotowari-stops:3dc67d73, docs/decisions/core/plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail:1608f12c] -->

| コード | 意味 |
|---|---|
| 0 | 計画書の形に誤りが無い |
| 1 | `invalid_plan` が1件以上ある |
| 2 | 停止した（引数の誤り、計画書が無い・ディレクトリ・読めない、UTF-8 でない） |

停止したときは標準出力に何も出さず、標準エラーの1行目に理由を出します。

| 場面 | 標準エラーの1行目の始まり |
|---|---|
| 計画書を渡さない・2つ以上渡す・`--config` を付けた | `argument error: ` |
| 計画書が無い・ディレクトリ・読めない | `unreadable file: ` |
| 計画書が UTF-8 でない | `non-UTF-8 file: ` |

## 例

<!-- @kotowari[docs/decisions/core/plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields:f10ab09f, docs/decisions/core/plan.md#findings-on-the-form-are-gathered-into-the-single-kind-invalid_plan-with-the-schema-sides-kind-and-details-in-detail:1608f12c] -->

形の揃った計画書です。

````markdown
# Plan: report the words a document uses too often

## Goal

A person who runs `wordcount check <path>` sees each word used more than the configured limit, with the line of its first use.

## Specification

The IR is in `docs/ir/`. This plan covers:

- `docs/ir/core/count.md#REQ-core-010`, `#REQ-core-011`
- `docs/ir/core/output.md#REQ-core-020`
- Examples: EX-core-030, EX-core-031

## Approach and why

Counting and reporting are split: a pure function counts words per document, and the command only reads the file and prints. The existing `read_text` helper already stops on unreadable and non-UTF-8 files, so the command reuses it instead of opening the file itself.

## Scope of change

- `src/count.rs` (new)
- `src/main.rs`
  - the `check` arm only
- `tests/count.rs` (new)

## Step order and prerequisites

S1 before S2: S2 prints what S1 counts.

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-010, REQ-core-011 | EX-core-030 |
| S2 | REQ-core-020 | EX-core-031 |

## Left to the implementer

- The names of the counting function and its result type

## Stop conditions

- The limit turns out to need a per-word setting, which the specification does not define
- An existing test fails for a reason other than the new command

## Test command

```sh
cargo test --workspace
```

## Out of scope

- Stemming or case folding beyond what REQ-core-011 states

## Steps

### S1: count the words of one document

- Purpose: count each word of a document and remember the line of its first use
- Specification: `docs/ir/core/count.md#REQ-core-010`, `docs/ir/core/count.md#REQ-core-011`
- Prerequisites: none
- May change: `src/count.rs`, `tests/count.rs`
- Done when: counting a document with "a" three times and the limit 2 returns "a" with the line of its first use, and a document within the limit returns nothing
- Shown by: test — EX-core-030, one test per rule of REQ-core-011
- Left to the implementer: none
- Stop and hand back if: REQ-core-011 and EX-core-030 disagree on what counts as a word

### S2: print the words over the limit

- Purpose: wire the count into `wordcount check` and print one line per word over the limit
- Specification: `docs/ir/core/output.md#REQ-core-020`
- Prerequisites: S1
- May change: `src/main.rs`, `tests/count.rs`
- Done when: `wordcount check a.md` prints `a.md:3 a` for EX-core-031 and exits with 1, and exits with 0 when nothing is over the limit
- Shown by: test — EX-core-031
- Left to the implementer: none
- Stop and hand back if: the exit code for a document with no words is not decided by REQ-core-020
````

```console
$ kotowari plan docs/plans/word-limit.md --format text
$ echo $?
0
$ kotowari plan docs/plans/word-limit.md
{"findings":[],"counts":{}}
```

この計画書を3か所壊した `broken.md` を読ませます。

- `## Out of scope` の前に `## Notes` の節と1行の文を足した
- S1 の `Shown by` を `manual — read the output` にした
- S2 の `Done when` の行を消した

```console
$ kotowari plan docs/plans/broken.md --format text
docs/plans/broken.md:52 [error] invalid_plan undeclared_heading: undeclared section heading "Notes"
docs/plans/broken.md:54 [error] invalid_plan undeclared_line: undeclared statement "Counting is case-sensitive."
docs/plans/broken.md:69 [error] invalid_plan field_pattern_mismatch: value "manual — read the output" does not match pattern "^(test|check|artifact|external)(\s|$)"
docs/plans/broken.md:73 [error] invalid_plan missing_required_field: field "Done when" is required but missing
$ echo $?
1
```

欄が欠けたときの行（73）は、そのステップの見出し `### S2: ...` の行を指しています。

同じ入力の JSON です。

```console
$ kotowari plan docs/plans/broken.md | jq .
{
  "findings": [
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 52,
      "detail": "undeclared_heading: undeclared section heading \"Notes\""
    },
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 54,
      "detail": "undeclared_line: undeclared statement \"Counting is case-sensitive.\""
    },
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 69,
      "detail": "field_pattern_mismatch: value \"manual — read the output\" does not match pattern \"^(test|check|artifact|external)(\\s|$)\""
    },
    {
      "kind": "invalid_plan",
      "severity": "error",
      "path": "docs/plans/broken.md",
      "line": 73,
      "detail": "missing_required_field: field \"Done when\" is required but missing"
    }
  ],
  "counts": {
    "invalid_plan": 4
  }
}
```

## よくあるつまずき

### 欄の値を2行に折り返したら `undeclared_line` が出る

<!-- @kotowari[docs/decisions/core/plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields:f10ab09f, docs/decisions/core/plan.md#under-a-step-only-field-lines-are-placed-and-plan-wide-sections-can-hold-statements-lists-tables-and-code-blocks:12358175] -->

計画書は1行ずつ読まれるので、欄の値の続きの行は欄の一部になりません。
ステップの下に置けない行として誤りになります。
値は、コマンドを含めて1行に書きます。

```console
$ kotowari plan docs/plans/wrapped.md --format text
docs/plans/wrapped.md:28 [error] invalid_plan undeclared_line: undeclared ordered list "S1 before S2: S2 prints what S1 counts."
docs/plans/wrapped.md:66 [error] invalid_plan undeclared_line: undeclared statement "one test per rule of REQ-core-011"
```

（66行目は `- Shown by: test — EX-core-030,` の次の行に字下げして続けた `one test per rule of REQ-core-011`）

### 番号付きの一覧が `undeclared ordered list` になる

<!-- @kotowari[docs/decisions/core/plan.md#a-plan-is-written-with-fixed-plan-wide-sections-and-steps-that-have-eight-fields:f10ab09f, docs/decisions/core/plan.md#under-a-step-only-field-lines-are-placed-and-plan-wide-sections-can-hold-statements-lists-tables-and-code-blocks:12358175] -->

計画全体の節には番号付きの一覧を置けません（上の例の28行目は `1. S1 before S2: ...`）。
`-` の箇条書きか文に書き換えます。
順序を表したいときは `## Step order and prerequisites` に文で書きます。

### `argument error: unexpected option for plan: --config`

<!-- @kotowari[docs/decisions/core/plan.md#kotowari-plan-reads-no-configuration-and-handles-arguments-and-unreadable-files-in-the-same-way-as-mutants:b92c7955] -->

`plan` は設定を読まないので、`--config` を受けません。
付けていたら外します。

```console
$ kotowari plan docs/plans/word-limit.md --config .kotowari/config.yaml
argument error: unexpected option for plan: --config
```

### `argument error: plan expects exactly one plan file path`

<!-- @kotowari[docs/decisions/core/plan.md#kotowari-plan-reads-no-configuration-and-handles-arguments-and-unreadable-files-in-the-same-way-as-mutants:b92c7955] -->

計画書は1回に1つだけ渡せます。
複数を検査するときは、1つずつ実行します。

```console
$ kotowari plan
argument error: plan expects exactly one plan file path, got 0
$ kotowari plan docs/plans/word-limit.md docs/plans/broken.md
argument error: plan expects exactly one plan file path, got 2
```

### `unreadable file: ...`

<!-- @kotowari[docs/decisions/core/plan.md#kotowari-plan-reads-no-configuration-and-handles-arguments-and-unreadable-files-in-the-same-way-as-mutants:b92c7955] -->

パスはカレントディレクトリからの相対で読みます。
基準のディレクトリ（`.kotowari/` のあるディレクトリ）からではありません。

```console
$ kotowari plan docs/plans/none.md
unreadable file: docs/plans/none.md: No such file or directory (os error 2)
```

一方、出力の `path` は基準のディレクトリからの相対です。
サブディレクトリから実行しても、`path` は同じ形で出ます。

```console
$ cd docs && kotowari plan plans/broken.md --format text | head -1
docs/plans/broken.md:52 [error] invalid_plan undeclared_heading: undeclared section heading "Notes"
```

## 関連

- 仕様: [計画書の検査（plan の IR）](../../ir/core/plan.ja.md)、[引数](../../ir/core/cli.ja.md#REQ-core-190)
- 共通の書式と停止: [cli.md](../cli.ja.md)
- 指摘の種類の一覧: [findings.md](../findings.ja.md)
- 要求の ID を確かめる: [kotowari query](query.ja.md)
