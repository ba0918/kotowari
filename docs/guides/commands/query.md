# kotowari query

<!-- @kotowari[REQ-core-156:31e36c67] -->

`kotowari query` は、ID を1つ受けて、その項目かシナリオを本文と逆引き（その ID を指している項目）つきで出すコマンドです。
1つの要求を IR の文書を開かずに読みたいときや、ガイドの印に写す指紋を取りたいときに使います。

## 書式

<!-- @kotowari[REQ-core-002:06abb59d, REQ-core-157:f0e9d719, EX-core-380:3c688038] -->

```sh
kotowari query [--format json|text] [--config <path>] <ID>
kotowari query --help
kotowari query --version
```

位置引数はちょうど1つの ID です。
オプションはコマンドの前に書いても、ID の後に書いても受けます（`kotowari query REQ-001 --format text` も同じ意味です）。

## オプションと引数

<!-- @kotowari[REQ-core-161:bf9675f4, REQ-core-157:f0e9d719, REQ-core-003:ccf703c7, REQ-core-011:549c5c91] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `<ID>` | ID（`REQ-core-001` のような形） | なし（必須） | 読む項目かシナリオの ID。大文字小文字を区別します |
| `--format` | `json` か `text` | `json` | 出力の形 |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイル。パスはカレントディレクトリからの相対で読みます |
| `--help` | なし | — | 使い方を出して終わります |
| `--version` | なし | — | 版を出して終わります |

共通のオプションと基準のディレクトリの詳細は [CLI の共通事項](../cli.md) にあります。

## 出力

何を読むかは `kotowari check` と同じです（同じ設定、同じ置き場の IR の文書とテストのファイル）。
ただしガイドは読みません。
同じ ID を持つ項目やシナリオが複数あれば、全部出します。

### text

<!-- @kotowari[REQ-core-161:bf9675f4, EX-core-254:01d21c93] -->

1件ごとに、[`kotowari list`](./list.md) の text と同じ1行目とテストの行に、本文と逆引きの行を続けます。

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
| 本文の行 | `body` の各行を2つの半角空白で字下げしたもの。空の行は空白2つだけの行になります |
| `<-` の行 | `referenced_by` の1件ごとに `  <- ID via パス:行`。`via` は下の表の値 |

### JSON

<!-- @kotowari[TBL-core-027:62b02eff, REQ-core-159:4b71c072] -->

最上位は `items` だけのオブジェクトで、`list` と同じ形です。
1件は [`kotowari list` の1件の鍵](./list.md#json)（`id`、`kind`、`name`、`path`、`line`、`type`、`verification`、`definition`、`examples`、`how_to_verify`、`relations`、`sources`、`tests`、`fingerprint`、`deferred`）をすべて持ち、次の2つの鍵が増えます。

| 鍵 | 型 | 説明 |
|---|---|---|
| `body` | 文字列の並び | 本文の行。先頭と末尾の空の行は含めず、行の文字はそのまま |
| `referenced_by` | オブジェクトの並び | その ID を指している項目とシナリオ。1件は `id`、`kind`、`path`、`line`、`via` |

`body` の範囲は種類で違います。

| 種類 | `body` の範囲 |
|---|---|
| 項目（要求、決定表、性質、問題の記録） | 見出しの次の行から、同じ深さかそれより浅い次の見出しの前の行まで。コードブロックの中の見出しの形の行では切りません |
| シナリオ | `@id` のタグの行から最後のステップの行まで |

`via` は、どこから指しているかを表します。

| `via` | 指している場所 |
|---|---|
| `definition` | `- definition:` の行 |
| `relations` | `- related:` の行 |
| `about` | シナリオの `@about` のタグ |
| `text` | 要求の文、性質の文、シナリオのステップの中で、バッククォートで囲んだ ID（二重引用符の外のもの） |

1つの項目が同じ ID を同じ `via` で何度指しても、`referenced_by` には1件だけ出ます。
細かい定義は [query の IR](../../ir/core/query.md) の TBL-core-027 にあります。

### 並び順

<!-- @kotowari[REQ-core-160:aad6e692] -->

`items` は `list` と同じ順（`path` の昇順、同じ `path` の中は `line` の昇順）です。
`referenced_by` も `path`、`line` の順に並びます。

## 終了コード

<!-- @kotowari[REQ-core-156:31e36c67, REQ-core-157:f0e9d719, REQ-core-158:57b9da70] -->

| コード | 意味 |
|---|---|
| 0 | 読めた（IR に誤りがあっても 0）。`--help` か `--version` で終わったときも 0 |
| 2 | 停止した（ID が無い、ID の形でない、引数の数が違う、設定の誤り、読めないファイルなど。理由は標準エラーに出ます） |

`query` は 1 を返しません。
IR の誤りの有無で終了コードを変えるのは `check` と `status` です。

## 例

<!-- @kotowari[EX-core-250:3f505f2e, EX-core-254:01d21c93, EX-core-255:f2fb4c9e] -->

[`kotowari list` の例](./list.md#例)と同じ小さな IR とテストで実行した結果です（2026-09-24 に実行）。

要求を text で読みます。
本文の後の `<-` の行から、シナリオ EX-001 がこの要求を `@about` で指していると分かります。

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

JSON では、`list` の1件に `body` と `referenced_by` が増えます。増えた2つだけを示します。

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

シナリオの本文は、`@id` のタグの行から始まります。

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

### ガイドの印に写す指紋を取る

<!-- @kotowari[TBL-core-026:05d8938e, REQ-core-203:e195dec1] -->

ガイドの印に書く指紋は、`fingerprint` の値をそのまま写します。手で計算しません。

```console
$ kotowari query REQ-001 | jq -r '.items[0].fingerprint'
ec0d8b1c
```

書き方は [ガイドを書く](../writing-guides.md) にあります。

## よくあるつまずき

### `argument error: unknown id: ...` で止まる

<!-- @kotowari[REQ-core-157:f0e9d719, EX-core-251:0b347986] -->

その ID を持つ項目もシナリオも IR にありません。
空の結果を返さずに止まるので、ID の打ち間違いにすぐ気づけます。

```console
$ kotowari query REQ-999
argument error: unknown id: REQ-999
$ echo $?
2
```

ID がうろ覚えなら、`kotowari list --format text` で探します。

### `argument error: not an id: ...` で止まる

<!-- @kotowari[REQ-core-157:f0e9d719] -->

位置引数が ID の形ではありません。
ID は大文字小文字を区別するので、小文字で書いても止まります。

```console
$ kotowari query req-001
argument error: not an id: req-001
```

### `argument error: query expects exactly one id, got N` で止まる

<!-- @kotowari[REQ-core-157:f0e9d719, EX-core-252:fe70e3eb] -->

ID を渡していないか、2つ以上渡しています。
`query` は1回に1つの ID だけを読みます。複数を見たいときは1つずつ実行するか、`kotowari list` の JSON を `jq` で絞ります。

```console
$ kotowari query REQ-001 REQ-002
argument error: query expects exactly one id, got 2
$ kotowari query
argument error: query expects exactly one id, got 0
```

### 同じ ID で2件出てくる

<!-- @kotowari[REQ-core-156:31e36c67, EX-core-253:a0ac53a2] -->

IR に同じ ID の項目が2つあります（`check` では duplicate_id の誤り）。
`query` は重複を隠さず、全部出して 0 で終わります。

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

どちらかの ID を付け直してください。

### 文の中で ID に触れているのに `referenced_by` に出ない

<!-- @kotowari[TBL-core-027:62b02eff, EX-core-257:373de661] -->

`via` が `text` の逆引きに数えるのは、バッククォートで囲んだ ID だけです。
二重引用符の中の ID や、囲んでいない ID は数えません。
逆引きに載せたい参照は `` `REQ-001` `` のように書きます。

## なぜこういう作りか

- **list の1件に `body` を足した形にしている。**
  LLM が1件を読むときに IR の文書を開き直さなくて済むからです。それが list に無い query の値打ちです。
  最上位も list と同じ `items` なので、鍵の説明が二重になりません。
  （[決定の記録 A2](../../decision/records/2026-09-20-query-status.md#A2)、[A4](../../decision/records/2026-09-20-query-status.md#A4)）
- **逆引きは query だけが持つ。**
  list は定義の逆向きの鍵を持たず、逆引きは query で行うと決めています。拾う場所は check が参照の検査のために ID を読んでいる場所と同じです。
  （[A3](../../decision/records/2026-09-20-query-status.md#A3)）
- **無い ID は空の結果でなく停止にする。**
  LLM が読むとき、空の `items` と終了コード 0 より、0 でない終了コードと理由が出るほうが取り違えないからです。
  一方で重複は check の指摘なので、query は隠さず全部出します。
  （[A6](../../decision/records/2026-09-20-query-status.md#A6)）
- **text は list の text の上に足す形。**
  読み方が1つ増えないようにしています。本文は IR の生の行なので、字下げだけで区別できます。
  （[A7](../../decision/records/2026-09-20-query-status.md#A7)）
- **`<-` や `via` の値など、kotowari が決める文言は英語。**
  停止の文言と JSON の鍵が英語なので揃えています。IR やテストから写す文字はそのままです。
  （[A13](../../decision/records/2026-09-20-query-status.md#A13)）

## 関連

- 仕様: [query の IR](../../ir/core/query.md)
- 全項目を一覧で見る: [`kotowari list`](./list.md)
- 全体の数と揃っているかを見る: [`kotowari status`](./status.md)
- 指摘を1件ずつ見る: [`kotowari check`](./check.md)
- ガイドの印と指紋: [ガイドを書く](../writing-guides.md)
- 共通のオプション、停止、基準のディレクトリ: [CLI の共通事項](../cli.md)
