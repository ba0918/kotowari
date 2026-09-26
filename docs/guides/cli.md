# kotowari の CLI（全コマンド共通）

<!-- @kotowari[REQ-core-001:c859c183] -->

kotowari の6つのコマンドに共通する決まりをまとめたページです。
書式、共通のオプション、指摘の出し方、終了コード、止まったときの標準エラー、パスの基準を扱います。
コマンドごとの細部は、それぞれのページを見てください。

## 書式

<!-- @kotowari[REQ-core-001:c859c183, REQ-core-002:06abb59d, EX-core-380:3c688038] -->

```sh
kotowari <command> [--format json|text] [--config <path>] [argument]
kotowari --help
kotowari --version
```

コマンドは次の6つだけです。

| コマンド | 何をするか | ページ |
|---|---|---|
| `check` | IR の書き方の検査と、IR とテストの対応の検査を両方行う | [commands/check.md](commands/check.md) |
| `list` | IR の項目を1件ずつ、付いているテストと並べる | [commands/list.md](commands/list.md) |
| `query` | 1つの ID の項目かシナリオを、本文と逆参照つきで出す | [commands/query.md](commands/query.md) |
| `status` | IR とテストが揃っているかを、数と1つの真偽で答える | [commands/status.md](commands/status.md) |
| `mutants` | 変異テストの結果のファイルを読み、見逃しを指摘する | [commands/mutants.md](commands/mutants.md) |
| `plan` | 計画書のファイル1つの形を、同梱のスキーマで検査する | [commands/plan.md](commands/plan.md) |

オプションはコマンドの前にも後にも書けます。
`kotowari --format text check` と `kotowari check --format text` は同じです。
位置引数とオプションの順も、どのコマンドでも問いません（`kotowari query REQ-001 --format text` も受けます）。

## 共通のオプション

<!-- @kotowari[REQ-core-002:06abb59d, REQ-core-021:14bd7b25, REQ-core-003:ccf703c7, REQ-core-107:b0d16f0b] -->

| 名前 | 値 | 既定 | 説明 | 受けるコマンド |
|---|---|---|---|---|
| `--format` | `json` か `text` | `json` | 出力の形 | すべて |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイルを変える。パスは**カレントディレクトリ**からの相対で読む | `check`、`list`、`query`、`status`、`mutants` |
| `--help` | なし | — | 使い方を標準出力に出して終了コード0で終わる | すべて |
| `--version` | なし | — | 版を標準出力に出して終了コード0で終わる | すべて |
| `--tool` | `cargo-mutants` | なし（必須） | 結果のファイルを出した道具 | `mutants` だけ |

`--help` と `--version` は、ほかの引数より優先されます。
これらがあると、ほかの引数に誤りがあっても見ずに表示し、検査もしません。
コマンドが無くても構いません（`kotowari --version` だけで動きます）。

`plan` は設定ファイルを読まないので、`--config` を受けません。
設定ファイルのキーは [config.md](config.md) にあります。

## 指摘の出し方

`check`、`mutants`、`plan` は、見つけた問題を「指摘」として1件ずつ出します。
指摘の形は3つのコマンドで同じです。
種類ごとの意味と直し方は [findings.md](findings.md) にあります。

### JSON の指摘

<!-- @kotowari[REQ-core-022:e6e6163f, TBL-core-006:da871a1d, REQ-core-028:03dc4671] -->

`--format json`（既定）では、標準出力に JSON を1つ出します。
指摘の1件は次の5つの鍵を持つオブジェクトです。

| 鍵 | 型 | 説明 |
|---|---|---|
| `kind` | 文字列 | 指摘の種類（`missing_source` など）。一覧は [findings.md](findings.md) |
| `severity` | 文字列 | `error`（誤り）か `notice`（注意） |
| `path` | 文字列 | 基準のディレクトリからの相対パス。IR の文書への指摘は文書、テストへの指摘はテストのファイル、`surface_without_spec` は面のファイル、一覧の1件への指摘は一覧のファイル |
| `line` | 数か null | 1始まりの行。文書全体への指摘では null |
| `detail` | 文字列 | 種類ごとに決まった短い文字列。多くは ID、問題の語、行の文字そのまま |

JSON の最上位の鍵はコマンドごとに違います。
各コマンドのページを見てください。

### text の指摘

<!-- @kotowari[REQ-core-025:b4d331d2, REQ-core-026:70f612a2] -->

`--format text` では、1件を1行で出します。

```text
パス:行 [error] 種類 詳細
パス:行 [notice] 種類 詳細
```

- 角括弧もそのまま出ます。
- `line` が null の指摘は、行を `-` と書きます（`docs/ir/misc/notitle.md:- [error] missing_title notitle.md`）。
- パスと詳細の中の改行は、`\n` と `\r` の2文字に置き換えます。どんなファイル名でも1件は1行です。
- 指摘が0件のときは何も出しません。

### 指摘の並び

<!-- @kotowari[TBL-core-007:d8427397, PROP-core-003:a4dc21c1] -->

指摘は次の順に並びます。JSON でも text でも同じです。

| 順 | 鍵 | 並べ方 |
|---|---|---|
| 1 | `path` | バイト順 |
| 2 | `line` | null が先、その後は小さい順 |
| 3 | `kind` | バイト順 |
| 4 | `detail` | バイト順 |

同じ入力からは、いつも同じ順の出力になります。
差分を取ったり、前回の出力と比べたりできます。

## 終了コード

<!-- @kotowari[REQ-core-007:0f08bddc, TBL-core-002:46c482a8] -->

| コード | 意味 |
|---|---|
| 0 | 誤りが無い（注意だけのときを含む）。または `--help` か `--version` で終わった |
| 1 | 誤りが1件以上ある |
| 2 | 停止した（検査を始められなかった） |

注意（`notice`）は終了コードを変えません。
CI では終了コードだけで「直す必要があるか」を判定できます。
`status` は、この表と別に「揃っているか」を終了コードで表します（[commands/status.md](commands/status.md)）。

## 停止

<!-- @kotowari[REQ-core-005:948bc7d9, REQ-core-109:f299dcec] -->

入力を全体として読めないとき、kotowari は検査をせずに終了コード2で終わります。
これを「停止」と呼びます。

- 標準出力には何も出しません。
- 標準エラーの1行目は `理由の文言: 詳細` の形で、英語です。
- 詳細にパスを含めるときは、基準のディレクトリからの相対パスです。

読めない入力を黙って飛ばすことはありません。
ファイルや設定を全体として読む前提が崩れるもの（等価の一覧と未記載の面の一覧の構文の誤りを含む）は停止に、読めたが一部が形から外れるもの（一覧の1件の形の誤りなど）はその場所への誤りの指摘になります。

### 停止の理由

<!-- @kotowari[TBL-core-018:6994583a, TBL-core-020:c3b8e7f8, TBL-core-001:d6482e1a] -->

| 1行目の文言 | 理由 | 詳細 | 主な場面 |
|---|---|---|---|
| `argument error` | 引数の誤り | 説明の文と問題の引数 | 知らないオプション、`--format` の知らない値、値の無いオプション、同じオプションの2回目、余分な位置引数、`--config` の先が無いかディレクトリ |
| `config error` | 設定の誤り | 設定ファイルの相対パスと誤りの説明（`tests.rules` と `surface.rules` のルールのファイル、等価の一覧、未記載の面の一覧の誤りではそのファイルの相対パス） | 設定ファイルが YAML として読めない、知らないキー、同じキーの2回目、型の違う値、glob として読めない要素、`surface.files` と `surface.rules` の片方だけ、`tests.rules` か `surface.rules` のルールのファイルが無いか読めない、未記載の面の一覧が YAML として読めないか最上位が並びでない |
| `unreadable file` | 読めないファイル | 相対パスと OS の誤りの文 | 置き場のディレクトリが無い、ファイルやディレクトリが読めない、`surface.unspecified` の指す先が無い |
| `non-UTF-8 file` | UTF-8 でないファイル | 相対パス | 読むファイルのどれかが UTF-8 でない（面のファイルは面の規則の言語のものだけを読む） |
| `results error` | 結果の誤り | 結果のファイルの相対パスと誤りの説明 | `mutants` の結果のファイルの形が壊れている |
| `mapping error` | 写しの誤り | 写せなかった指摘の種類か値の説明 | kotowari の内部の不整合。利用者の入力では起きない想定 |

場面の全部は [cli.md の TBL-core-001](../ir/core/cli.md) と [cli-environment.md の TBL-core-020](../ir/core/cli-environment.md) にあります。

### 引数の誤り

<!-- @kotowari[REQ-core-004:e34832b6, EX-core-219:ca4b0dc6, EX-core-241:adaaea32] -->

次のどれかに当たると `argument error` で止まります（`--help` か `--version` があるときを除く）。

- 引数が1つも無い、またはオプションだけで、コマンドが無い
- 1つ目の位置引数が6つのコマンドのどれでもない
- 知らないオプション、または `mutants` でないコマンドに付けた `--tool`
- `check`、`list`、`status` の後の位置引数
- `--format` の知らない値、値の無いオプション、同じオプションの2回目
- `--config` の指す先が無いか、ディレクトリ

`mutants` の引数の決まりは [commands/mutants.md](commands/mutants.md) にあります。

## 基準のディレクトリ

<!-- @kotowari[REQ-core-009:6b5a71a4, TBL-core-003:4e72b7b4, PROP-core-001:7da90ad3, REQ-core-010:322a6380] -->

kotowari の扱う相対パスには、基準が1つだけあります。
それが「基準のディレクトリ」です。

| 順 | 条件 | 基準のディレクトリ |
|---|---|---|
| 1 | カレントディレクトリから上へ向かって `.kotowari/` のディレクトリを探し、見つかった | 最初に見つかったディレクトリ |
| 2 | 見つからない | カレントディレクトリ |

`.kotowari` という名前の**ファイル**は無視して、さらに上へ進みます。

基準のディレクトリからの相対になるのは、次の3つです。

- 設定ファイルに書くパス（`ir`、`tests.files` など）
- IR の出典（`- source:` の行）のパス
- 出力の `path` と、停止の詳細のパス

`--config` はどの設定ファイルを読むかを変えるだけで、基準は変えません。
`--config` のパスそのものは、カレントディレクトリからの相対で読みます。

### パスの正規化

<!-- @kotowari[REQ-core-110:f8ef194c] -->

設定の値のパスと出典のパスは、比べる前と出力の前に次のように整えます。

- 末尾の `/` と先頭の `./` を除く
- 途中の `/./` と、連続する `/` を1つの `/` にする
- `\` を `/` にする
- `a/..` を畳む（畳む相手の無い `..` は残す）

そのため `docs/decision/../decision/records` と `docs/decision/records` は同じ置き場として扱われます。

## 対象の環境

<!-- @kotowari[REQ-core-108:9188181d] -->

kotowari は Linux と macOS を対象にしています。
Windows では、パスの区切りの `\` を `/` に直すことだけを行い、それ以外の動作は約束していません。

## 例

例は、`.kotowari/config.yaml` を置いた小さなリポジトリで実行したものです。

### 使い方と版を見る

<!-- @kotowari[REQ-core-107:b0d16f0b] -->

```console
$ kotowari --version
kotowari 0.1.0
$ kotowari --help
Usage: kotowari [OPTIONS] <COMMAND> [ARGUMENT]

Commands:
  check      Check IR documents and test markers
  list       List IR items and the tests marked for them
  mutants    Read a mutation testing result file and report survivors
  plan       Check the form of one plan file against the bundled schema
  query      Show one item or scenario with its body and back references
  status     Summarise the IR and tell whether it is complete

Options:
  --format <FORMAT>  Output format: json (default) or text
  --config <PATH>    Path to configuration file
  --tool <TOOL>      Mutation testing tool of the result file: cargo-mutants
  --help             Show this help message
  --version          Show version
```

### 引数を間違えて止まる

<!-- @kotowari[REQ-core-004:e34832b6, REQ-core-005:948bc7d9, EX-core-219:ca4b0dc6] -->

```console
$ kotowari
argument error: expected command: check, list, mutants, plan, query or status
$ kotowari check --verbose
argument error: unknown option: --verbose
$ kotowari check --format xml
argument error: unknown format: xml
$ kotowari check extra
argument error: unexpected argument: extra
$ kotowari check --format text --format json
argument error: repeated option: --format
$ kotowari check --format
argument error: --format requires a value
$ kotowari check --config docs
argument error: --config is a directory: docs
$ echo $?
2
```

### ファイルと設定の問題で止まる

<!-- @kotowari[TBL-core-018:6994583a, TBL-core-020:c3b8e7f8] -->

```console
$ kotowari check --format text      # docs/decision/adr が無い
unreadable file: docs/decision/adr: No such file or directory (os error 2)
$ kotowari check --format text      # docs/ir/greet/bad.md が UTF-8 でない
non-UTF-8 file: docs/ir/greet/bad.md
$ kotowari check --config bad.yaml  # limits を limit と書き間違えた
config error: bad.yaml: error: line 2 column 1: unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, vague_words
 --> <input>:2:1
  |
1 | ir: docs/ir
2 | limit:
  | ^ unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, vague_words
3 |   lines: 20
  |
$ kotowari check --format text      # rules/c.yml の language が cobol
config error: invalid rule in tests.rules: rules/c.yml: unknown language: cobol
```

YAML の読み方に関わる設定の誤り（知らないキー、型の違いなど）では、1行目の後に該当箇所の抜粋が続きます。
同じキーの2回目と、`tests.rules` のルールの知らない言語は1行だけです。
`tests.rules` の誤りの詳細は、設定ファイルでなくルールのファイルを指します。
決まっているのは1行目の形だけです。

### サブディレクトリから実行する

<!-- @kotowari[TBL-core-003:4e72b7b4, PROP-core-001:7da90ad3, REQ-core-003:ccf703c7] -->

`.kotowari/` のあるディレクトリの下の `tests/` で実行しても、基準は上の `.kotowari/` のあるディレクトリです。
出力のパスは `tests/` からではなく、基準からの相対になります。

```console
$ cd tests
$ kotowari check --format text
docs/ir/greet/greet.md:15 [error] requirement_without_test REQ-greet-002
…
tests/greet.rs:6 [error] test_without_id rejects_empty_name
```

一方、`--config` のパスはカレントディレクトリからの相対です。
`tests/` から基準の設定ファイルを指すには `../` が要ります。

```console
$ kotowari check --config .kotowari/config.yaml
argument error: config file not found: .kotowari/config.yaml
$ kotowari --format text check --config ../bad.yaml
config error: bad.yaml: error: line 2 column 1: unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, vague_words
…
```

2つ目の詳細の `bad.yaml` は、基準のディレクトリからの相対パスで書かれています。

## よくあるつまずき

### 終了コードが2で、標準出力に何も出ない

<!-- @kotowari[REQ-core-005:948bc7d9, TBL-core-018:6994583a] -->

検査を始める前に止まっています。
標準出力ではなく、標準エラーの1行目を見てください。
1行目の先頭の文言（`argument error` など）で理由が分かります。
JSON を `jq` に渡しているときは、`jq` が空の入力を受けて何も出さないので気付きにくくなります。

### `--config` を付けたのに `config file not found` になる

<!-- @kotowari[REQ-core-003:ccf703c7, PROP-core-001:7da90ad3] -->

`--config` のパスは、基準のディレクトリではなくカレントディレクトリからの相対で読みます。
サブディレクトリで実行しているなら、`../` を付けるか絶対パスで指してください。

### 別の場所の `.kotowari/` が基準になっている

<!-- @kotowari[REQ-core-009:6b5a71a4, TBL-core-003:4e72b7b4] -->

基準のディレクトリは、カレントディレクトリから上へ探して最初に見つかった `.kotowari/` です。
入れ子のリポジトリや、ホームディレクトリに `.kotowari/` があると、思わぬ所が基準になります。
出力の `path` が思った形でないときは、上のディレクトリの `.kotowari/` を確かめてください。

### 注意（`[notice]`）が出ているのに終了コードが0

<!-- @kotowari[TBL-core-002:46c482a8] -->

仕様どおりです。
注意は終了コードを変えません。
注意を CI で止めたいときは、JSON の `counts` を `jq` で見て判定してください。

## なぜこういう作りか

- **既定の出力が JSON。**
  第一の利用者が LLM なので、構造化された形を基本にしました。
  人向けの text は、1指摘1行に留めています。
  （[records.md A7](../decision/records/records.md#A7)、[A18](../decision/records/records.md#A18)）
- **停止と指摘を分け、黙って飛ばさない。**
  読めない入力を黙って飛ばしたり既定の値にしたりすると、検査が通ったように見えてしまいます。
  全体を読めないものは停止、局所的に崩れたものはその場所への誤り、と必ずどちらかにします。
  （[records.md A100](../decision/records/records.md#A100)、[A101](../decision/records/records.md#A101)、[P2](../decision/records/records.md#P2)）
- **停止の文言は英語の固定文言。**
  汎用のツールの出力として英語にし、1行目の先頭で理由を機械的に見分けられるようにしました。
  （[records.md A104](../decision/records/records.md#A104)）
- **相対パスの基準は1つ。**
  基準が複数あると、どのパスが何からの相対かを覚える必要が出ます。
  `--config` は読む設定ファイルを変えるだけにしました。
  （[records.md A37](../decision/records/records.md#A37)）
- **`--help` と `--version` はほかの引数に勝つ。**
  引数を間違えているときこそ使い方を見たいからです。
  （[records.md A136](../decision/records/records.md#A136)）

## 関連

- 仕様: [コマンドと終了](../ir/core/cli.md)、[使い方の表示と対象の環境](../ir/core/cli-environment.md)、[基準のディレクトリ](../ir/core/base-directory.md)、[出力の形](../ir/core/output.md)、[指摘の並べ方と行](../ir/core/finding-order.md)
- 設定ファイルのキー: [config.md](config.md)
- 指摘の種類: [findings.md](findings.md)
- 各コマンド: [check](commands/check.md)、[list](commands/list.md)、[query](commands/query.md)、[status](commands/status.md)、[mutants](commands/mutants.md)、[plan](commands/plan.md)
