# kotowari の CLI（全コマンド共通）

[English](cli.md) | 日本語

<!-- @kotowari[REQ-core-001:f6b868d0] -->

kotowari の8つのコマンドに共通する決まりをまとめたページです。
書式、共通のオプション、指摘の出し方、終了コード、止まったときの標準エラー、パスの基準を扱います。
コマンドごとの細部は、それぞれのページを見てください。

## 書式

<!-- @kotowari[REQ-core-001:f6b868d0, REQ-core-002:f86e2efd, EX-core-380:f7f73b4e] -->

```sh
kotowari <command> [--format json|text] [--config <path>] [argument]
kotowari overview build [--format json|text] [--config <path>]
kotowari overview serve [--port <port>] [--config <path>]
kotowari --help
kotowari --version
```

コマンドは次の8つだけです。

| コマンド | 何をするか | ページ |
|---|---|---|
| `changes` | Git の変更と記録の対応・鮮度を指定した段階で検査する | [commands/changes.md](commands/changes.ja.md) |
| `check` | IR の書き方の検査と、IR とテストの対応の検査を両方行う | [commands/check.md](commands/check.ja.md) |
| `list` | IR の項目を1件ずつ、付いているテストと並べる | [commands/list.md](commands/list.ja.md) |
| `query` | 1つの ID の項目かシナリオを、本文と逆参照つきで出す | [commands/query.md](commands/query.ja.md) |
| `status` | IR とテストが揃っているかを、数と1つの真偽で答える | [commands/status.md](commands/status.ja.md) |
| `mutants` | 変異テストの結果のファイルを読み、見逃しを指摘する | [commands/mutants.md](commands/mutants.ja.md) |
| `plan` | 計画書のファイル1つの形を、同梱のスキーマで検査する | [commands/plan.md](commands/plan.ja.md) |
| `overview` | 下位のコマンド `build` で全体像のページを `.kotowari/cache/overview/` の下に書き、`serve` で書いて 127.0.0.1 で配る。`languages` に2つ以上の言語があれば、先頭以外の言語のページを `<言語タグ>/` の下に書く | [仕様](../ir/core/overview-commands.ja.md) |

オプションはコマンドの前にも後にも書けます。
`kotowari --format text check` と `kotowari check --format text` は同じです。
位置引数とオプションの順も、どのコマンドでも問いません（`kotowari query REQ-001 --format text` も受けます）。

## 共通のオプション

<!-- @kotowari[REQ-core-002:f86e2efd, REQ-core-021:ccedd28b, REQ-core-003:b4f59e48, REQ-core-107:7c6198ca, REQ-core-297:481b26d5] -->

| 名前 | 値 | 既定 | 説明 | 受けるコマンド |
|---|---|---|---|---|
| `--format` | `json` か `text` | `json` | 出力の形 | `overview serve` 以外のすべて |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイルを変える。通常は**カレントディレクトリ**からの相対。`changes` は Git ルートからの相対で、対象 snapshot の設定を読む | `check`、`list`、`query`、`status`、`mutants`、`changes`、`overview build`、`overview serve` |
| `--help` | なし | — | 使い方を標準出力に出して終了コード0で終わる | すべて |
| `--version` | なし | — | 版を標準出力に出して終了コード0で終わる | すべて |
| `--tool` | `cargo-mutants` | なし（必須） | 結果のファイルを出した道具 | `mutants` だけ |
| `--port` | 1から65535までの10進の整数 | `4590` | 配る 127.0.0.1 のポート | `overview serve` だけ |
| `--allow-test-findings` | なし（値を取らない） | 付けない | テスト側の指摘を終了コードに数えない（[commands/check.md](commands/check.ja.md#テストより先に仕様をコミットする)） | `check` だけ |

`--help` と `--version` は、ほかの引数より優先されます。
これらがあると、ほかの引数に誤りがあっても見ずに表示し、検査もしません。
コマンドが無くても構いません（`kotowari --version` だけで動きます）。

`plan` は設定ファイルを読まないので、`--config` を受けません。
設定ファイルのキーは [config.md](config.ja.md) にあります。

## 指摘の出し方

`check`、`mutants`、`plan`、`changes` は、見つけた問題を「指摘」として1件ずつ出します。
指摘の形はこれらのコマンドで同じです。
種類ごとの意味と直し方は [findings.md](findings.ja.md) にあります。

### JSON の指摘

<!-- @kotowari[REQ-core-022:8e221eee, TBL-core-006:1b834d84, REQ-core-028:1c43e818] -->

`--format json`（既定）では、標準出力に JSON を1つ出します。
指摘の1件は次の5つの鍵を持つオブジェクトです。

| 鍵 | 型 | 説明 |
|---|---|---|
| `kind` | 文字列 | 指摘の種類（`missing_source` など）。一覧は [findings.md](findings.ja.md) |
| `severity` | 文字列 | `error`（誤り）か `notice`（注意） |
| `path` | 文字列 | 基準のディレクトリからの相対パス。IR の文書への指摘は文書、テストへの指摘はテストのファイル、`surface_without_spec` は面のファイル、一覧の1件への指摘は一覧のファイル |
| `line` | 数か null | 1始まりの行。文書全体への指摘では null |
| `detail` | 文字列 | 種類ごとに決まった短い文字列。多くは ID、問題の語、行の文字そのまま |

JSON の最上位の鍵はコマンドごとに違います。
各コマンドのページを見てください。

### text の指摘

<!-- @kotowari[REQ-core-025:58025379, REQ-core-026:530a64e3] -->

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

<!-- @kotowari[TBL-core-007:15954989, PROP-core-003:0d661079] -->

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

<!-- @kotowari[REQ-core-007:0f08bddc, TBL-core-002:36817bf5] -->

| コード | 意味 |
|---|---|
| 0 | 誤りが無い（注意だけのときを含む）、`--allow-test-findings` を付けた `check` の誤りがどれもテスト側の指摘、または `--help` か `--version` で終わった |
| 1 | 誤りが1件以上ある（`--allow-test-findings` を付けた `check` では、テスト側の指摘でない誤りが1件以上ある） |
| 2 | 停止した（検査を始められなかった） |

注意（`notice`）は終了コードを変えません。
CI では終了コードだけで「直す必要があるか」を判定できます。
`status` は、この表と別に「揃っているか」を終了コードで表します（[commands/status.md](commands/status.ja.md)）。

## 停止

<!-- @kotowari[REQ-core-005:fee48254, REQ-core-109:4d28a72d] -->

入力を全体として読めないとき、kotowari は検査をせずに終了コード2で終わります。
これを「停止」と呼びます。

- 標準出力には何も出しません。
- 標準エラーの1行目は `理由の文言: 詳細` の形で、英語です。
- 詳細にパスを含めるときは、基準のディレクトリからの相対パスです。

読めない入力を黙って飛ばすことはありません。
ファイルや設定を全体として読む前提が崩れるもの（等価の一覧と未記載の面の一覧の構文の誤りを含む）は停止に、読めたが一部が形から外れるもの（一覧の1件の形の誤りなど）はその場所への誤りの指摘になります。

### 停止の理由

<!-- @kotowari[TBL-core-018:c435229b, TBL-core-020:d87686da, TBL-core-001:0d8e4c30] -->

| 1行目の文言 | 理由 | 詳細 | 主な場面 |
|---|---|---|---|
| `argument error` | 引数の誤り | 説明の文と問題の引数 | 知らないオプション、`--format` の知らない値、値の無いオプション、同じオプションの2回目、余分な位置引数、`--config` の先が無いかディレクトリ、`overview` の後が `build` か `serve` の1つでない、`--port` の値が1から65535の整数でない |
| `config error` | 設定の誤り | 設定ファイルの相対パスと誤りの説明（`tests.rules` と `surface.rules` のルールのファイル、等価の一覧、未記載の面の一覧の誤りではそのファイルの相対パス。置き場の重なりでは重なったファイルと `: matched by both ...`。`overview` の鍵が無いまま `overview build` か `serve` を実行したときは `overview is not configured` だけ） | 設定ファイルが YAML として読めない、知らないキー、同じキーの2回目、型の違う値、glob として読めない要素、`surface.files` と `surface.rules` の片方だけ、`tests.rules` か `surface.rules` のルールのファイルが無いか読めない、未記載の面の一覧が YAML として読めないか最上位が並びでない、`overview.files` がガイドかテストの置き場と重なる、`overview` に `toc` が無い、目次のファイルが `overview.files`、`guides.files`、`tests.files` のどれかの走査で読むファイルに入る、`overview` の鍵が無いまま `overview build` か `serve` を実行した |
| `unreadable file` | 読めないファイル | 相対パスと OS の誤りの文 | 置き場のディレクトリが無い、ファイルやディレクトリが読めない、`surface.unspecified` か `overview.toc` の指す先が無い |
| `non-UTF-8 file` | UTF-8 でないファイル | 相対パス | 読むファイルのどれかが UTF-8 でない（面のファイルは面の規則の言語のものだけを読む） |
| `results error` | 結果の誤り | 結果のファイルの相対パスと誤りの説明 | `mutants` の結果のファイルの形が壊れている |
| `git error` | Git の読み取り停止 | 読めない履歴・対象・index の説明 | Git が無い、REV が commit に解決できない、設定が対象に無い、競合した index、不対応の対象 |
| `mapping error` | 写しの誤り | 写せなかった指摘の種類か値の説明 | kotowari の内部の不整合。利用者の入力では起きない想定 |
| `overview error` | 元データの誤り | 誤りの件数と ` errors in overview data; run kotowari check` | `overview build` か `serve` で全体像の元データか目次に誤りがある、または IR、全体像の元データ、目次の対に `translation_missing` か `translation_structure_mismatch` がある。何も書かない |
| `port error` | ポートの誤り | `127.0.0.1:<ポート>: ` と OS の誤りの文 | `overview serve` で指定のポートを使えない（ほかのポートは試さない）か、配っている間に接続の受け付けに失敗した |
| `cache error` | 置き場の誤り | 問題のパスの相対パスと、OS の誤りがあれば `: ` と OS の誤りの文 | `overview build` か `serve` で `.kotowari`、`.kotowari/cache`、`.kotowari/cache/overview`、言語が2つ以上のときのほかの言語の `.kotowari/cache/overview/<言語タグ>` のどれかがシンボリックリンクかディレクトリでないファイル（何も書かず消さない）、または置き場の作成・書き込み・削除に失敗した |

場面の全部は [cli.md の TBL-core-001](../ir/core/cli.ja.md) と [cli-environment.md の TBL-core-020](../ir/core/cli-environment.ja.md) にあります。

### 引数の誤り

<!-- @kotowari[REQ-core-004:2d3401d1, EX-core-219:f3123493, EX-core-241:38290f77, REQ-core-304:e9cd623d] -->

次のどれかに当たると `argument error` で止まります（`--help` か `--version` があるときを除く）。

- 引数が1つも無い、またはオプションだけで、コマンドが無い
- 1つ目の位置引数が8つのコマンドのどれでもない
- 知らないオプション、`mutants` でないコマンドに付けた `--tool`、`overview serve` でないコマンドに付けた `--port`、`overview serve` に付けた `--format`、または `check` でないコマンドに付けた `--allow-test-findings`
- `overview` の後の位置引数がちょうど1つでないか、`build` か `serve` でない
- `--port` の値が1から65535までの10進の整数でない
- `check`、`list`、`status` の後の位置引数
- `--format` の知らない値、値の無いオプション、同じオプションの2回目
- `--config` の指す先が無いか、ディレクトリ

`mutants` の引数の決まりは [commands/mutants.md](commands/mutants.ja.md) にあります。

## 基準のディレクトリ

<!-- @kotowari[REQ-core-009:6b5a71a4, TBL-core-003:603e9601, PROP-core-001:9be33697, REQ-core-010:5217a2ba] -->

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

<!-- @kotowari[REQ-core-110:04555fb0] -->

設定の値のパスと出典のパスは、比べる前と出力の前に次のように整えます。

- 末尾の `/` と先頭の `./` を除く
- 途中の `/./` と、連続する `/` を1つの `/` にする
- `\` を `/` にする
- `a/..` を畳む（畳む相手の無い `..` は残す）

そのため `docs/decision/../decision/records` と `docs/decision/records` は同じ置き場として扱われます。

## 対象の環境

<!-- @kotowari[REQ-core-108:8e9073d4] -->

kotowari は Linux と macOS を対象にしています。
Windows では、パスの区切りの `\` を `/` に直すことだけを行い、それ以外の動作は約束していません。

## 例

例は、`.kotowari/config.yaml` を置いた小さなリポジトリで実行したものです。

### 使い方と版を見る

<!-- @kotowari[REQ-core-107:7c6198ca] -->

```console
$ kotowari --version
kotowari 0.1.0
$ kotowari --help
Usage: kotowari [OPTIONS] <COMMAND> [ARGUMENT]

Commands:
  changes    Check change records against a Git base and target snapshot
  check      Check IR documents and test markers
  list       List IR items and the tests marked for them
  mutants    Read a mutation testing result file and report survivors
  overview   build: write the overview pages; serve: write and show them locally
  plan       Check the form of one plan file against the bundled schema
  query      Show one item or scenario with its body and back references
  status     Summarise the IR and tell whether it is complete

Changes: --base <REV> (--head <REV> | --staged) --phase <implementation|review>
Options:
  --format <FORMAT>      Output format: json (default) or text
  --config <PATH>        Path to configuration file
  --tool <TOOL>          Mutation testing tool of the result file: cargo-mutants
  --port <PORT>          Port of overview serve on 127.0.0.1 (default 4590)
  --allow-test-findings  check: exit 0 when every error is a test-side finding
  --help                 Show this help message
  --version              Show version
```

### 引数を間違えて止まる

<!-- @kotowari[REQ-core-004:2d3401d1, REQ-core-005:fee48254, EX-core-219:f3123493] -->

```console
$ kotowari
argument error: expected command: check, changes, list, mutants, overview, plan, query or status
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
argument error: --config is a directory: docs
$ echo $?
2
```

### ファイルと設定の問題で止まる

<!-- @kotowari[TBL-core-018:c435229b, TBL-core-020:d87686da] -->

```console
$ kotowari check --format text      # docs/decision/adr が無い
unreadable file: docs/decision/adr: No such file or directory (os error 2)
$ kotowari check --format text      # docs/ir/greet/bad.md が UTF-8 でない
non-UTF-8 file: docs/ir/greet/bad.md
$ kotowari check --config bad.yaml  # limits を limit と書き間違えた
config error: bad.yaml: error: line 2 column 1: unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, changes, overview, vague_words, languages, labels
 --> <input>:2:1
  |
1 | ir: docs/ir
2 | limit:
  | ^ unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, changes, overview, vague_words, languages, labels
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

<!-- @kotowari[TBL-core-003:603e9601, PROP-core-001:9be33697, REQ-core-003:b4f59e48] -->

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
config error: bad.yaml: error: line 2 column 1: unknown field `limit`, expected one of ir, decisions, tests, guides, mutants, surface, limits, changes, overview, vague_words, languages, labels
…
```

2つ目の詳細の `bad.yaml` は、基準のディレクトリからの相対パスで書かれています。

## よくあるつまずき

### 終了コードが2で、標準出力に何も出ない

<!-- @kotowari[REQ-core-005:fee48254, TBL-core-018:c435229b] -->

検査を始める前に止まっています。
標準出力ではなく、標準エラーの1行目を見てください。
1行目の先頭の文言（`argument error` など）で理由が分かります。
JSON を `jq` に渡しているときは、`jq` が空の入力を受けて何も出さないので気付きにくくなります。

### `--config` を付けたのに `config file not found` になる

<!-- @kotowari[REQ-core-003:b4f59e48, PROP-core-001:9be33697] -->

`--config` のパスは、基準のディレクトリではなくカレントディレクトリからの相対で読みます。
サブディレクトリで実行しているなら、`../` を付けるか絶対パスで指してください。

### 別の場所の `.kotowari/` が基準になっている

<!-- @kotowari[REQ-core-009:6b5a71a4, TBL-core-003:603e9601] -->

基準のディレクトリは、カレントディレクトリから上へ探して最初に見つかった `.kotowari/` です。
入れ子のリポジトリや、ホームディレクトリに `.kotowari/` があると、思わぬ所が基準になります。
出力の `path` が思った形でないときは、上のディレクトリの `.kotowari/` を確かめてください。

### 注意（`[notice]`）が出ているのに終了コードが0

<!-- @kotowari[TBL-core-002:36817bf5] -->

仕様どおりです。
注意は終了コードを変えません。
注意を CI で止めたいときは、JSON の `counts` を `jq` で見て判定してください。

## 関連

- 仕様: [コマンドと終了](../ir/core/cli.ja.md)、[使い方の表示と対象の環境](../ir/core/cli-environment.ja.md)、[基準のディレクトリ](../ir/core/base-directory.ja.md)、[出力の形](../ir/core/output.ja.md)、[指摘の並べ方と行](../ir/core/finding-order.ja.md)
- 設定ファイルのキー: [config.md](config.ja.md)
- 指摘の種類: [findings.md](findings.ja.md)
- 各コマンド: [check](commands/check.ja.md)、[list](commands/list.ja.md)、[query](commands/query.ja.md)、[status](commands/status.ja.md)、[mutants](commands/mutants.ja.md)、[plan](commands/plan.ja.md)

`changes` の基準は起動位置を含む Git 作業ツリーのルートです。対象の設定・IR・判断の記録・照合記録を commit または index から一緒に読み、作業ツリーでは補いません。比較と段階の必須引数は [changes](commands/changes.ja.md) を見てください。
