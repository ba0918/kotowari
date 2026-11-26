# kotowari query

[English](query.md) | 日本語

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

`kotowari query` は、ID を1つ（決定の行き先か確認することの行の ID）受け取り、その1件を本文と一緒に出すコマンドです。決定なら、その決定へリンクしている決定も出します。
決定の文書を開かずに1つの決定を読みたいとき、ある行をどのテストが確かめているかを見たいとき、ガイドの印に写す指紋を取りたいときに使います。

## 書式

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

```sh
kotowari query [--format json|text] [--config <path>] <ID>
kotowari query --help
kotowari query --version
```

位置引数はちょうど1つの ID です。
オプションはコマンドの前にも ID の後にも書けます（`kotowari query C-shop-discount-1 --format text` も同じ意味です）。

## オプションと引数

<!-- @kotowari[docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `<ID>` | 決定の行き先（`docs/decisions/shop/discount.md#members-get-10-off-their-orders`）か行の ID（`C-shop-discount-1`） | なし（必須） | 読む1件。大文字と小文字を区別します |
| `--format` | `json` か `text` | `json` | 出力の形式 |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイル。パスはカレントディレクトリからの相対で読みます |
| `--help` | なし | — | 使い方を出して終わる |
| `--version` | なし | — | 版を出して終わる |

共通のオプションと基準のディレクトリの細部は [CLI の共通事項](../cli.ja.md) にあります。

## 出力

読むものは `kotowari check` と同じです（同じ設定、同じ置き場の決定の文書とテストのファイル）。
ただしガイドは読みません。
同じ ID の行がいくつもあれば、すべてを出します。

### text

<!-- @kotowari[docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5] -->

1件ごとに、[`kotowari list`](./list.ja.md) の text と同じ1行目とテストの行の後に、本文の行が続きます。

```text
ID 検証 名前 パス:行 tests=数
  パス:行 テストの名前
  本文の1行目
  本文の2行目
  ...
  <- 指している ID via パス:行
```

| 行 | 中身 |
|---|---|
| 1行目とテストの行 | `kotowari list --format text` と同じ |
| 本文の行 | `body` の各行を、2つの半角空白で字下げしたもの。空の行は空白2つだけの行になる |
| `<-` の行 | 決定では `referenced_by` の1件ごとに `  <- 行き先 link パス:行` |
| 行の最後の行 | `  decision 状態 見出し`。その行を持つ決定の状態と見出し（状態の行が誤っていれば状態は `-`） |

### JSON

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#the-body-when-a-decision-is-looked-up-is-the-whole-decision:39fb387c, docs/decisions/core/query.md#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading:90363085] -->

最上位は `items` だけを持つオブジェクトで、`list` と同じ形です。
1件は [`kotowari list` の1件の鍵](./list.ja.md#json)をすべて持ち、次の2つの鍵が加わります。

| 鍵 | 型 | 説明 |
|---|---|---|
| `body` | 文字列の並び | 本文の行。決定の文書に書かれたまま |
| `referenced_by` | オブジェクトの並び | 決定では、その見出しへリンクしている決定。1件は `id`、`kind`（`decision`）、`path`、`line`（リンクしている決定の見出しの行）、`via`（`link`）を持つ。行ではいつも空 |

`body` の範囲は種類で決まります。

| 種類 | `body` の範囲 |
|---|---|
| 決定 | 見出しから最後の節までの、決定のすべての行（状態の行、問題、決めたこと、見送った案、確認すること） |
| 行 | 書かれたままの行。折り返した行も含む |

同じ見出しに何度もリンクしている決定も、`referenced_by` には1回だけ出ます。
決定の文書は、行の ID ではなく見出しへのリンクで決定を指すので、行を指すものはありません。
細かい定義は [query の決定](../../decisions/core/query.ja.md) にあります。

### 並び順

<!-- @kotowari[docs/decisions/core/query.md#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading:90363085] -->

`items` は `list` と同じ並び（`path` の昇順、同じ `path` の中は `line` の昇順）です。
`referenced_by` も `path`、`line` の順に並びます。

## 終了コード

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c, docs/decisions/core/query.md#query-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces:4414244f] -->

| コード | 意味 |
|---|---|
| 0 | 読めた（決定の文書に誤りがあっても 0）。`--help` か `--version` で終わったときも 0 |
| 2 | 停止した（ID が無い、引数が ID の形でない、引数の数が違う、設定の誤り、読めないファイルなど。理由は標準エラーに出ます） |

`query` は 1 を返しません。
誤りの有無で終了コードを変えるのは `check` と `status` です。
`query` はガイド、面のファイル、面の規則のファイル、未記載の面の一覧、全体像の元データを読まないので、それらを理由に止まることはありません。

## 例

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7, docs/decisions/core/query.md#the-output-is-json-or-text-and-text-follows-the-list-line-with-the-body-lines:d37342f5] -->

[`kotowari list` の例](./list.ja.md#例)と同じ小さな決定の文書とテストで実行した結果です。2つ目の決定の問題から、1つ目の決定へリンクを張っています。

決定を text で読みます。
本文の後の `<-` の行は、客の決定がこの決定の見出しへリンクしていることを示します。

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

行を text で読みます。テストが先に出て、最後の行がその決定を示します。

```console
$ kotowari query C-shop-discount-1 --format text
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
  - C-shop-discount-1 A member's order of 1000 is priced at 900
  decision implemented Members get 10% off their orders
```

JSON では、`list` の1件に `body` と `referenced_by` が加わります。

```console
$ kotowari query C-shop-discount-1
{"items":[{"id":"C-shop-discount-1","kind":"confirmation","name":"A member's order of 1000 is priced at 900","path":"docs/decisions/shop/discount.md","line":19,"decision":"Members get 10% off their orders","state":"implemented","by_reading":false,"tests":[{"path":"tests/discount.rs","line":1,"name":"member_gets_ten_percent_off"}],"body":["- C-shop-discount-1 A member's order of 1000 is priced at 900"],"referenced_by":[]}]}
```

### ガイドの印に写す指紋を取る

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/guides.md#a-stale-section-is-found-by-comparing-the-fingerprint-written-in-the-marker-with-the-current-fingerprint-of-the-decision:c21ccd0f] -->

ガイドの印に書く指紋は、決定の `fingerprint` の値をそのまま写します。手で計算しないでください。

```console
$ kotowari query 'docs/decisions/shop/discount.md#members-get-10-off-their-orders' | jq -r '.items[0].fingerprint'
e3adc927
```

ガイドの書き方は [ガイドを書く](../writing-guides.ja.md) にあります。

## よくあるつまずき

### `argument error: unknown id: ...` で止まる

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

その ID の決定も行もありません。
空の結果を返さずに止まるので、ID の書き間違いにすぐ気づけます。
見出しの文を変えた決定の行き先も、無い ID になります。

```console
$ kotowari query C-shop-discount-9
argument error: unknown id: C-shop-discount-9
$ echo $?
2
$ kotowari query 'docs/decisions/shop/discount.md#no-such-heading'
argument error: unknown id: docs/decisions/shop/discount.md#no-such-heading
```

ID が分からなければ `kotowari list --format text` で探します。

### `argument error: not an id: ...` で止まる

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

位置引数が、行の ID の形でも行き先の形でもありません。
ID は大文字と小文字を区別するので、小文字で書いても止まります。`#` の無い文書のパスも行き先ではありません。

```console
$ kotowari query c-shop-discount-1
argument error: not an id: c-shop-discount-1
$ kotowari query docs/decisions/shop/discount.md
argument error: not an id: docs/decisions/shop/discount.md
```

### `argument error: query expects exactly one id, got N` で止まる

<!-- @kotowari[docs/decisions/core/query.md#if-the-positional-argument-is-not-exactly-one-id-or-that-id-does-not-exist-it-is-a-stop-with-an-argument-error:bbaa030c] -->

ID を渡していないか、2つ以上渡しています。
`query` が読むのは1回に1つの ID です。いくつも見たいときは ID ごとに実行するか、`kotowari list` の JSON を `jq` で絞ります。

```console
$ kotowari query C-shop-discount-1 C-shop-discount-2
argument error: query expects exactly one id, got 2
$ kotowari query
argument error: query expects exactly one id, got 0
```

### 同じ ID で2件出てくる

<!-- @kotowari[docs/decisions/core/query.md#query-receives-one-id-and-outputs-the-decision-or-confirmation-row-with-that-id-as-a-list-entry-with-the-body-added:4b2043e7] -->

2つの行が同じ ID を持っています（`check` では duplicate_id の誤り）。
`query` は重複を隠さず、すべてを出して 0 で終わります。

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

後の行に、その話題でまだ使っていない次の番号を付けます。

### 文の中で決定に触れているのに `referenced_by` に出ない

<!-- @kotowari[docs/decisions/core/query.md#the-reverse-references-of-a-decision-hold-the-decisions-that-link-to-that-decisions-heading:90363085, docs/decisions/core/decision-documents.md#to-refer-to-another-decision-link-to-its-heading:41ba3d75] -->

逆引きに数えるのは、決定の見出しへの Markdown のリンクだけです。
見出しを地の文に書いただけのものや、文に書いた行の ID は数えません。
逆引きに出したい参照は、`[会員の割引](discount.ja.md#会員は注文が1割引きになる)` のようにリンクで書きます。

## 関連

- 仕様: [query の決定](../../decisions/core/query.ja.md)
- 決定と行をすべて一覧で見る: [`kotowari list`](./list.ja.md)
- 全体の数と揃っているかを見る: [`kotowari status`](./status.ja.md)
- 指摘を1件ずつ見る: [`kotowari check`](./check.ja.md)
- ガイドの印と指紋: [ガイドを書く](../writing-guides.ja.md)
- 共通のオプション、停止、基準のディレクトリ: [CLI の共通事項](../cli.ja.md)
