# kotowari list

<!-- @kotowari[REQ-core-151:4e40b186] -->

`kotowari list` は、IR（仕様）の項目とシナリオを、それを指す印の付いたテストと一緒に一覧で出すコマンドです。
どの要求にどのテストが付いているかを見たいときや、LLM に仕様の全体を一度で渡したいときに使います。
指摘は出しません。指摘を見るときは [`kotowari check`](./check.md) を使います。

## 書式

<!-- @kotowari[REQ-core-002:06abb59d, REQ-core-004:e34832b6] -->

```sh
kotowari list [--format json|text] [--config <path>]
kotowari list --help
kotowari list --version
```

オプションはコマンドの前にも後ろにも書けます（`kotowari --format text list` も同じ意味です）。
位置引数は受けません。

## オプションと引数

<!-- @kotowari[REQ-core-155:fbd06b39, REQ-core-003:ccf703c7, REQ-core-011:549c5c91] -->

| 名前 | 値 | 既定 | 説明 |
|---|---|---|---|
| `--format` | `json` か `text` | `json` | 出力の形 |
| `--config` | 設定ファイルのパス | 基準のディレクトリの `.kotowari/config.yaml` | 読む設定ファイル。パスはカレントディレクトリからの相対で読みます |
| `--help` | なし | — | 使い方を出して終わります |
| `--version` | なし | — | 版を出して終わります |

絞り込みのオプションはありません。
1件だけ詳しく見るなら [`kotowari query`](./query.md)、条件で絞るなら JSON を `jq` に通します。
基準のディレクトリと共通のオプションの詳細は [CLI の共通事項](../cli.md) にあります。

## 出力

何を読むかは `kotowari check` と同じです（同じ設定、同じ置き場の IR の文書とテストのファイル）。
ただしガイドは読みません。

### text

<!-- @kotowari[REQ-core-155:fbd06b39, EX-core-248:7651c98a, EX-core-399:bcb01e70] -->

1件を1行で出し、その下に印の付いたテストを1件ずつ字下げして続けます。

```text
ID 検証 名前 パス:行 tests=数
  パス:行 テストの名前
```

[後回し](../deferred.md)の要求と後回しのシナリオ（JSON の `deferred` が true の1件）では、行の末尾に ` deferred` が付きます。

```text
REQ-001 unit 例 docs/ir/a.md:7 tests=0 deferred
```

| 欄 | 中身 |
|---|---|
| 検証 | 要求の `- verification:` の値。要求以外と、`- verification:` の行の無い要求では `-` |
| パス:行 | 項目の見出しの行（シナリオは `Scenario:` の行） |
| tests=数 | その ID を印に含むテストの数 |
| テストの行 | 2つの半角空白で字下げした `パス:行 名前`。パスと行は印の位置。名前が取れないときは `-` |

### JSON

<!-- @kotowari[TBL-core-026:05d8938e, REQ-core-153:513617dc, EX-core-288:30fa9a16] -->

最上位は `items` だけのオブジェクトです。
`items` は1件ずつのオブジェクトの並びで、種類によって持つ鍵が違います。持たない鍵は出ません。

| 鍵 | 型 | 持つ種類 | 説明 |
|---|---|---|---|
| `id` | 文字列 | すべて | ID |
| `kind` | 文字列 | すべて | `requirement`、`table`、`property`、`scenario`、`flag` のいずれか |
| `name` | 文字列 | すべて | 見出しの名前。シナリオは `Scenario:` の後の文字 |
| `path` | 文字列 | すべて | 項目のある文書の、基準のディレクトリからの相対パス |
| `line` | 数 | すべて | 見出しの行。シナリオは `Scenario:` の行 |
| `type` | 文字列か null | 要求、問題の記録 | `- kind:` の値 |
| `verification` | 文字列か null | 要求 | `- verification:` の値 |
| `definition` | 文字列の並び | 要求 | `- definition:` の ID。無ければ空の並び |
| `examples` | 文字列の並び | 要求、決定表、性質 | その ID を `@about` に持つシナリオの ID。ID の昇順 |
| `how_to_verify` | 文字列か null | 要求 | `- how_to_verify:` の値 |
| `relations` | 文字列の並び | 問題の記録 | `- related:` の ID |
| `sources` | 文字列の並び | すべて | 出典 |
| `tests` | オブジェクトの並び | すべて | その ID を印に含むテスト。1件は `path`、`line`（印の行）、`name`（テストの名前か null） |
| `fingerprint` | 文字列 | すべて | 指紋（16進8桁）。[ガイドの印](../writing-guides.md)に写す値 |
| `deferred` | 真偽値 | すべて | [後回し](../deferred.md)の要求と後回しのシナリオは true、ほかは false |

同じテストに同じ ID の印が2つあれば、`tests` には2件出ます。
同じ ID のシナリオが2つあるときは、`examples` には1つ目だけを数えます。
細かい定義は [list の IR](../../ir/core/list.md) の TBL-core-026 にあります。

### 並び順

<!-- @kotowari[REQ-core-154:f5509315] -->

`items` は `path` の昇順、同じ `path` の中は `line` の昇順です。
1件の `tests` も同じく `path`、`line` の順に並びます。

## 終了コード

<!-- @kotowari[REQ-core-151:4e40b186, REQ-core-152:c0f8af67] -->

| コード | 意味 |
|---|---|
| 0 | 読めた（IR に誤りがあっても 0）。`--help` か `--version` で終わったときも 0 |
| 2 | 停止した（設定の誤り、読めないファイル、引数の誤りなど。理由は標準エラーに出ます） |

`list` は 1 を返しません。
IR の誤りの有無で終了コードを変えるのは `check` と `status` です。
停止の理由と文言は `check` と同じです（[CLI の共通事項](../cli.md)）。

## 例

<!-- @kotowari[EX-core-245:47ec1793, EX-core-248:7651c98a] -->

要求 2 つとシナリオ 1 つを持つ小さな IR と、印を1つ付けたテストで実行した結果です（2026-09-24 に実行）。

IR の文書 `docs/ir/greet.md` の要点:

```markdown
### REQ-001: 名前つきの挨拶
- verification: unit
...
### REQ-002: 挨拶の丁寧さ
- verification: review
- how_to_verify: 返す文を人が読み、丁寧な言葉であることを確かめる
...
@id=EX-001 @about=REQ-001 @source=docs/decision/records/records.md#A1
Scenario: 名前を受けて挨拶する
```

テスト `tests/greet.rs`:

```rust
// @kotowari[REQ-001, EX-001]
#[test]
fn greets_with_name() {}
```

text で出すと次のとおりです。

```console
$ kotowari list --format text
REQ-001 unit 名前つきの挨拶 docs/ir/greet.md:7 tests=1
  tests/greet.rs:1 greets_with_name
REQ-002 review 挨拶の丁寧さ docs/ir/greet.md:15 tests=0
EX-001 - 名前を受けて挨拶する docs/ir/greet.md:28 tests=1
  tests/greet.rs:1 greets_with_name
```

JSON（既定）では、同じ内容が鍵つきで出ます。1件目だけを示します。

```console
$ kotowari list | jq '.items[0]'
{
  "id": "REQ-001",
  "kind": "requirement",
  "name": "名前つきの挨拶",
  "path": "docs/ir/greet.md",
  "line": 7,
  "type": "ubiquitous",
  "verification": "unit",
  "definition": [],
  "examples": [
    "EX-001"
  ],
  "how_to_verify": null,
  "sources": [
    "docs/decision/records/records.md#A1"
  ],
  "tests": [
    {
      "path": "tests/greet.rs",
      "line": 1,
      "name": "greets_with_name"
    }
  ],
  "fingerprint": "ec0d8b1c",
  "deferred": false
}
```

よく使う絞り込みは `jq` で書けます。

```console
$ kotowari list | jq -r '.items[] | select(.kind == "requirement" and (.tests | length) == 0) | .id'
REQ-002
```

## よくあるつまずき

### IR に誤りがあるのに終了コードが 0 になる

<!-- @kotowari[REQ-core-151:4e40b186, EX-core-246:0d89a1ab] -->

`list` は読み取りのコマンドで、指摘を出さず、誤りがあっても読めた項目を出して 0 で終わります。
例えば例の IR から REQ-001 の `- verification:` の行を消すと、`check` は誤りを出して 1 で終わりますが、`list` は検証の欄を `-`（JSON では `null`）にして 0 で終わります（2026-09-24 に実行）。

```console
$ kotowari check --format text
docs/ir/greet.md:7 [error] verification_missing REQ-001
$ kotowari list --format text
REQ-001 - 名前つきの挨拶 docs/ir/greet.md:7 tests=1
  tests/greet.rs:1 greets_with_name
REQ-002 review 挨拶の丁寧さ docs/ir/greet.md:14 tests=0
EX-001 - 名前を受けて挨拶する docs/ir/greet.md:27 tests=1
  tests/greet.rs:1 greets_with_name
$ echo $?
0
```

CI で誤りを止めたいなら `kotowari check` か [`kotowari status`](./status.md) を使います。

### テストの名前が `-`（JSON では null）になる

<!-- @kotowari[TBL-core-026:05d8938e, EX-core-247:fccadbbf] -->

問い合わせの無い言語のテストのファイルでは、kotowari は印だけを拾い、テストの名前は取りません。
`path` と `line` は印の位置です。
どの拡張子に問い合わせがあるかは `kotowari check` の JSON の `tests` の `query` で分かります（[CLI の共通事項](../cli.md)）。

### `tests=0` の要求が status では「テストあり」に数えられている

<!-- @kotowari[TBL-core-026:05d8938e, TBL-core-028:6aa7ef3b] -->

`list` の `tests` は、その ID を直接印に含むテストだけです。
[`kotowari status`](./status.md) の `with_tests` は、その要求を `@about` に持つシナリオに付いたテストも数えます。
`tests=0` の要求を見つけたら、その要求の `examples` に並ぶシナリオの `tests` も確かめてください。
`deferred` が true の要求は、`status` では `with_tests` にも `without_tests` にも数えず、`deferred` に数えます。

### check や status は設定の誤りで止まるのに、list は動く

<!-- @kotowari[REQ-core-152:c0f8af67, REQ-core-198:ec0e5ea4, REQ-core-199:38cf396d] -->

`list` はガイドを読まないので、ガイドの置き場に関わる停止（`guides.files` と `tests.files` の重なりなど）はしません。
それ以外の停止の条件は `check` と同じです。
設定を直すときは `kotowari check` で確かめてください。

## なぜこういう作りか

- **list は指摘を出さず、誤りがあっても読めた範囲を出して 0 で終わる。**
  指摘は `check` の仕事で、読み取りのコマンドが誤りの有無で出力を止める理由が無いからです。
  （[決定の記録 A9](../../decision/records/2026-09-19-read-commands.md#A9)）
- **1回の list で必要な情報が揃うようにしている。**
  LLM が list を読んでから別のコマンドを呼び直すと、読み込みの往復が増えるからです。
  さらに詳しい本文と逆引きは `query` で取ります。
  （[A3](../../decision/records/2026-09-19-read-commands.md#A3)）
- **絞り込みのオプションを持たない。**
  全項目を出し、絞るのは `jq` か `query` の仕事にしています。
  （[A8](../../decision/records/2026-09-19-read-commands.md#A8)）
- **要求とテストの対応を手で保つ表をやめ、list の出力にした。**
  手で保つ表は腐るからです。trace は別のコマンドにせず list に含めました。
  （[A2](../../decision/records/2026-09-19-read-commands.md#A2)）
- **text は最低限の形で、JSON が既定。**
  形式は1つ増えるごとに契約が増えるので、yaml などは足していません。
  （[A7](../../decision/records/2026-09-19-read-commands.md#A7)、[A19](../../decision/records/2026-09-19-read-commands.md#A19)）
- **定義の逆向きの鍵（決定表から要求へ）を持たない。**
  片方向で足り、逆引きは `query` の `referenced_by` で行います。
  （[A14](../../decision/records/2026-09-19-read-commands.md#A14)）

## 関連

- 仕様: [list の IR](../../ir/core/list.md)
- 1件を本文と逆引きつきで見る: [`kotowari query`](./query.md)
- 全体の数と揃っているかを見る: [`kotowari status`](./status.md)
- 指摘を1件ずつ見る: [`kotowari check`](./check.md)
- 共通のオプション、停止、基準のディレクトリ: [CLI の共通事項](../cli.md)
- 設定ファイル: [設定](../config.md)
