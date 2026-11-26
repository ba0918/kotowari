# kotowari list

[English](list.md) | 日本語

<!-- @kotowari[docs/decisions/core/list.md#list-reads-the-same-things-as-check-reports-no-findings-and-outputs-every-decision-and-confirmation-row-it-could-read:f863a2d6] -->

`kotowari list` は、決定の文書（仕様）の決定と確認することの行を、各行を指す印の付いたテストと一緒に一覧で出すコマンドです。
どの行にどのテストが付いているかを見たいとき、ガイドの印に書く決定の行き先と指紋を探すとき、LLM に仕様の全体を一度で渡したいときに使います。
指摘は出しません。指摘を見るときは [`kotowari check`](./check.ja.md) を使います。

## 書式

<!-- @kotowari[docs/decisions/core/cli.md#the-options-each-command-accepts-are-fixed-and-options-and-positional-arguments-may-be-written-in-any-order:94339fbd, docs/decisions/core/cli.md#an-option-not-accepted-an-unknown-command-an-extra-positional-argument-or-an-option-with-a-wrong-value-stops-with-an-argument-error:11a07c33] -->

```sh
kotowari list [--format json|text] [--config <path>]
kotowari list --help
kotowari list --version
```

オプションはコマンドの前にも後にも書けます（`kotowari --format text list` も同じ意味です）。
位置引数は取りません。

## オプションと引数

<!-- @kotowari[docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7, docs/decisions/core/config.md#the-configuration-file-is-placed-at-kotowariconfigyaml-in-the-base-directory-and---config-can-point-at-another-file:7c217b47] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `--format` | `json` か `text` | `json` | 出力の形式 |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイル。パスはカレントディレクトリからの相対で読みます |
| `--help` | なし | — | 使い方を出して終わる |
| `--version` | なし | — | 版を出して終わる |

絞り込みのオプションはありません。
1件を詳しく見るときは [`kotowari query`](./query.ja.md) を、条件で絞るときは JSON を `jq` に通します。
基準のディレクトリと共通のオプションの細部は [CLI の共通事項](../cli.ja.md) にあります。

## 出力

読むものは `kotowari check` と同じです（同じ設定、同じ置き場の決定の文書とテストのファイル）。
ただしガイドと全体像の元データは読みません。
`languages` に2つ以上の言語を書いたときだけ、`translations` の hash を出すために、ガイド、全体像の元データ、目次の置き場も辿ります（中身は検査しません）。
決定と行は、対の先頭の言語の側だけから取ります。

### text

<!-- @kotowari[docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7] -->

1件を1行で出し、その下に印の付いたテストを1件ずつ字下げして続けます。

```text
ID 検証 名前 パス:行 tests=数
  パス:行 テストの名前
```

| 欄 | 中身 |
|---|---|
| ID | 決定ではその行き先（`<パス>#<行き先の名前>`。ガイドの印に写す値）、行ではその ID |
| 検証 | `（読んで確認）` で始まる行は `reading`、ほかは `-` |
| 名前 | 決定ではその見出し、行では ID（と `（読んで確認）`）の後の文 |
| パス:行 | 決定の見出しの行か、行そのものの行 |
| tests=数 | その ID を印に含むテストの数。テストは行を指すので、決定ではいつも 0 |
| テストの行 | 2つの半角空白で字下げした `パス:行 名前`。パスと行は印の位置。名前が取れないときは `-` |

`languages` に2つ以上の言語を書いたときは、1件の行の後に、対ごとに1行で、先頭の言語の側のパスに続けて言語ごとに半角空白1つと `言語タグ=blob hash` を並べます。
無い側は `言語タグ=-` です。

```text
docs/decisions/shop/cart.md en=86fedceb857d3e80a36805d0a55533190c16ced5 ja=2246e3d2ff17d246033d32bcb65c8bb8717bd840
guides/a.md en=247728873932ff94b66350209bc6f7e598bad23c ja=-
```

### JSON

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7, docs/decisions/core/list.md#with-two-or-more-languages-the-blob-hash-of-each-pair-is-output-too:8b3ec16c, docs/decisions/core/list.md#pairs-of-decision-documents-are-output-in-translations-too:681f0a2a] -->

最上位は `items` と、`languages` に2つ以上の言語を書いたときだけ `translations` を持つオブジェクトです。
`items` は1件ずつのオブジェクトの並びで、種類によって持つ鍵が違います。

| 鍵 | 型 | 持つ種類 | 説明 |
|---|---|---|---|
| `id` | 文字列 | 両方 | 決定ではその行き先 `<パス>#<行き先の名前>`、行ではその ID |
| `kind` | 文字列 | 両方 | `decision` か `confirmation` |
| `name` | 文字列 | 両方 | 決定ではその見出し、行では ID と `（読んで確認）` の後の文。折り返した行は半角空白1つでつなぐ |
| `path` | 文字列 | 両方 | 決定の文書の、基準のディレクトリからの相対パス |
| `line` | 数 | 両方 | 決定の見出しの行か、行そのものの行 |
| `state` | 文字列か null | 両方 | `proposed` か `implemented`（行ではその決定の状態）。状態の行が誤っていれば null |
| `fingerprint` | 文字列 | 決定 | 指紋（16進8文字）。[ガイドの印](../writing-guides.ja.md)に写す値 |
| `decision` | 文字列 | 行 | その行を持つ決定の見出し |
| `by_reading` | 真偽値 | 行 | `（読んで確認）` で始まる行は true |
| `tests` | オブジェクトの並び | 行 | その ID を印に含むテスト。1件は `path`、`line`（印の行）、`name`（テストの名前。取れなければ null）を持つ |

同じテストに同じ ID の印が2つあれば、`tests` には2件出ます。
ID の形が誤った行は出ません。
細かい定義は [list の決定](../../decisions/core/list.ja.md) にあります。

`translations` は対ごとの1件の並びで、`path` の昇順です。決定の文書、ガイド、全体像の元データ、目次の対が入ります。
先頭の言語の側が無い対も1件になります。
1件は `path`（先頭の言語の側の基準のディレクトリからの相対パス）と `sides` を持ちます。
`sides` は `languages` の順の側の並びで、1つの側は `language`（言語タグ）、`path`、`blob`（その側の git の blob hash。側が無ければ null）を持ちます。
`blob` を一致の記録 `<幹>.i18n.yaml` に写します（[言語と対](../config.ja.md#言語と対--languages-と-labels)）。

```json
{"path":"guides/a.md","sides":[{"language":"en","path":"guides/a.md","blob":"247728873932ff94b66350209bc6f7e598bad23c"},{"language":"ja","path":"guides/a.ja.md","blob":null}]}
```

### 並び順

<!-- @kotowari[docs/decisions/core/list.md#the-order-is-by-path-and-by-line-within-the-same-path:37893e8e] -->

`items` は `path` の昇順、同じ `path` の中は `line` の昇順です。そのため、各決定の後にその行が続きます。
1件の `tests` も同じく `path`、`line` の順に並びます。

## 終了コード

<!-- @kotowari[docs/decisions/core/list.md#list-reads-the-same-things-as-check-reports-no-findings-and-outputs-every-decision-and-confirmation-row-it-could-read:f863a2d6, docs/decisions/core/list.md#list-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces:7fe55f1b] -->

| コード | 意味 |
|---|---|
| 0 | 読めた（決定の文書に誤りがあっても 0）。`--help` か `--version` で終わったときも 0 |
| 2 | 停止した（設定の誤り、読めないファイル、引数の誤りなど。理由は標準エラーに出ます） |

`list` は 1 を返しません。
誤りの有無で終了コードを変えるのは `check` と `status` です。
停止の理由と文言は `check` と同じです（[CLI の共通事項](../cli.ja.md)）。

## 例

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/list.md#the-output-is-json-or-text-and-there-are-no-filtering-options:ab50c4b7] -->

実装済みの決定2つと、そのうち1つが読んで確認する行の3つの行を持つ小さな決定の文書と、印を1つ付けたテストで実行した結果です。

決定の文書 `docs/decisions/shop/discount.md` の要点:

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

テスト `tests/discount.rs`:

```rust
// @kotowari[C-shop-discount-1]
#[test]
fn member_gets_ten_percent_off() {}
```

text で出すと次のとおりです。

```console
$ kotowari list --format text
docs/decisions/shop/discount.md#members-get-10-off-their-orders - Members get 10% off their orders docs/decisions/shop/discount.md:5 tests=0
C-shop-discount-1 - A member's order of 1000 is priced at 900 docs/decisions/shop/discount.md:19 tests=1
  tests/discount.rs:1 member_gets_ten_percent_off
docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted - Orders of non-members are not discounted docs/decisions/shop/discount.md:21 tests=0
C-shop-discount-2 - A guest's order of 1000 is priced at 1000 docs/decisions/shop/discount.md:35 tests=0
C-shop-discount-3 reading The price list page says that only members get the discount docs/decisions/shop/discount.md:36 tests=0
```

JSON（既定）では、同じ内容が鍵つきで出ます。最初の2件、決定と行を示します。

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

よく使う絞り込みは `jq` で書けます。まだテストの要る行は次のとおりです。

```console
$ kotowari list | jq -r '.items[] | select(.kind == "confirmation" and .tests == [] and (.by_reading | not)) | .id'
C-shop-discount-2
```

## よくあるつまずき

### 決定の文書に誤りがあるのに終了コードが 0 になる

<!-- @kotowari[docs/decisions/core/list.md#list-reads-the-same-things-as-check-reports-no-findings-and-outputs-every-decision-and-confirmation-row-it-could-read:f863a2d6] -->

`list` は読み取りのコマンドで、指摘を出さず、誤りがあっても読めた決定と行を出して 0 で終わります。
例えば例の最初の決定の状態の行を `Status: done` にすると、`check` は誤りを出して 1 で終わりますが、`list` はその決定と行をそのまま出して 0 で終わります。

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

CI で誤りを止めたいなら `kotowari check` か [`kotowari status`](./status.ja.md) を使います。

### テストの名前が `-`（JSON では null）になる

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66] -->

問い合わせの無い言語のテストのファイルでは、kotowari は印だけを拾い、テストの名前は取りません。
`path` と `line` は印の位置です。
どの拡張子に問い合わせがあるかは `kotowari check` の JSON の `tests` の `query` で分かります（[CLI の共通事項](../cli.ja.md)）。

### 決定はいつも `tests=0` になる

<!-- @kotowari[docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66, docs/decisions/core/confirmations.md#check-counts-per-confirmation-row-whether-a-test-points-at-it:f6a31de4] -->

テストが指すのは決定ではなく確認することの行なので、決定の行は `tests=0` で、JSON の1件は `tests` を持ちません。
決定が確かめられているかは、その下に並ぶ行で見ます。
`（読んで確認）` で始まる行（text では `reading`）にはテストが要らず、提案の決定の行にもまだ要りません。

### check や status は設定の誤りで止まるのに、list は動く

<!-- @kotowari[docs/decisions/core/list.md#list-stops-under-the-same-conditions-as-check-and-does-not-read-guides-or-surfaces:7fe55f1b, docs/decisions/core/guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them:05ee20e8, docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

`list` はガイドを読まないので、ガイドの置き場に関わる停止（`guides.files` と `tests.files` の重なりなど）はしません。
ただし `languages` に2つ以上の言語を書いたときは、ガイド、全体像の元データ、目次の置き場を辿るので、その置き場と読み込みによる停止は `check` と同じにします。
面のファイル、面の規則のファイル、未記載の面の一覧も読まないので、それらが無い、読めない、壊れているときの停止もしません。
全体像の元データも読まないので、その置き場と読み込みによる停止（`overview.files` とガイドかテストの置き場の重なりなど）もしません。
ただし `surface.files` と `surface.rules` の片方だけを書いたときなど、設定の鍵の組み合わせの誤りでは `list` も止まります（[設定](../config.ja.md)）。
それ以外の停止の条件は `check` と同じです。
設定を直すときは `kotowari check` で確かめてください。

## 関連

- 仕様: [list の決定](../../decisions/core/list.ja.md)
- 1件を本文と逆引きつきで見る: [`kotowari query`](./query.ja.md)
- 全体の数と揃っているかを見る: [`kotowari status`](./status.ja.md)
- 指摘を1件ずつ見る: [`kotowari check`](./check.ja.md)
- 共通のオプション、停止、基準のディレクトリ: [CLI の共通事項](../cli.ja.md)
- 設定ファイル: [設定](../config.ja.md)
