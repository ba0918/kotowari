# ガイドを書く — ガイドの印と指紋

[English](writing-guides.md) | 日本語

ガイドは、IR を読んだ人か LLM が書く、利用者向けの使い方の文書です（このページもその1つです）。
kotowari はガイドを生成しません。
その代わり、ガイドの節ごとに「どの IR の項目を説明しているか」と「書いたときの項目の指紋」を印として持たせ、`kotowari check` が今の IR と照らします。
IR が変わって古くなった節は、いつ実行しても `guide_stale` の注意として挙がります。

## 書式

<!-- @kotowari[REQ-core-201:82ef697c, TBL-core-036:96778f03] -->

ガイドの印は、HTML のコメントの中に書きます。描画したガイドには出ません。

```markdown
## 会員の割引

<!-- @kotowari[REQ-001:874e14a4, EX-001:54af105a] -->

会員の注文は1割引きになります。
```

| 部分 | 形 |
|---|---|
| 始まり | `@kotowari[` |
| 中身 | 1件を `ID:指紋` の形で書き、コンマで区切って並べる |
| 指紋 | 16進の小文字（`0`〜`9` と `a`〜`f`）の8文字 |
| 終わり | 始まりと同じ行の `]` |

- 1件ごとに前後の空白を除き、最初の `:` で ID と指紋に分けます。括弧の内側や `:` の前後に空白を置いてかまいません。
- 同じ ID の1件を2つ書いても、1件ずつ照合します。
- テストの印（`@kotowari[REQ-001]`）とは別の規則です。テストの印は [テストに印を付ける](marks.ja.md) を見てください。

## 印を読む場所

<!-- @kotowari[REQ-core-200:97b3eb9b, EX-core-366:c019e492, EX-core-374:4cdeb739] -->

ガイドは CommonMark として読みます。
ガイドの印として読むのは、HTML のコメントの `<!--` から `-->` までの中にある `@kotowari[` だけです。

| 場所 | 読むか |
|---|---|
| HTML のコメントの中 | 読む。1つのコメントの中、1行の中の印はすべて拾う |
| コメントの外の本文 | 読まない |
| コードブロック（フェンスの形と字下げの形）の中 | 読まない |
| コードスパンの中 | 読まない |
| 同じ行の `-->` の後 | 読まない |

そのため、印の書き方の例はコードブロックかコードスパンに書けば、印として読まれません。
このページの例もすべてコードブロックの中にあります。

次のガイドでは、読まれる印は最後の1件だけです。

````markdown
# 印の書き方

印は @kotowari[REQ-001] の形ではなく、`<!-- @kotowari[REQ-001:874e14a4] -->` の形で書きます。

```markdown
<!-- @kotowari[REQ-001] -->
```

<!-- メモ --> @kotowari[REQ-001:00000000]

<!-- @kotowari[REQ-001 : 874e14a4] -->
````

```console
$ kotowari check | jq -c .guides
{"files":1,"marks":1}
```

## 置き場の設定

<!-- @kotowari[REQ-core-198:253e79fd, REQ-core-199:55922b15, EX-core-369:248c0ebf] -->

ガイドの置き場は、設定ファイルの `guides.files` に glob の一覧で書きます。

```yaml
guides:
  files:
    - "guides/**/*.md"
```

| 状態 | 動き |
|---|---|
| `guides.files` が無いか空の一覧 | ガイドを1つも読まない。ガイドの検査は何もしない |
| glob が何にも当たらない | 誤りにしない（`guides` の `files` が 0 になる） |
| `tests.files` と同じファイルに当たる | 設定の誤りで止まる（終了コード 2） |
| IR や判断の記録の置き場のファイルに当たる | 止まらない。そのファイルをガイドとしても読む |

- ガイドを読むのは `kotowari check` と `kotowari status` だけです。`list` と `query` は読みません。
- glob の読み方、走査、読めないファイルでの停止は `tests.files` と同じです（[設定ファイル](config.ja.md#glob-の読み方)）。

## 指紋

<!-- @kotowari[REQ-core-203:1fdc0443, TBL-core-026:382b0b95, EX-core-364:cc1080a1, EX-core-375:6d9bd3bb] -->

指紋は、項目かシナリオの本文から取る16進の小文字8文字の値です。
`kotowari list` と `kotowari query` の JSON の `fingerprint` に出ます。
手で計算せず、ここから写します。

```console
$ kotowari query REQ-001 | jq -r '.items[0].fingerprint'
874e14a4
$ kotowari list | jq -r '.items[] | "\(.id) \(.fingerprint)"'
REQ-001 874e14a4
REQ-002 76709f33
EX-001 54af105a
```

指紋が変わる変更と変わらない変更は次のとおりです。

| 変更 | 指紋 |
|---|---|
| 項目の本文（`- kind:`、`- verification:` の行、文、表）を変える | 変わる |
| シナリオのステップの行（Given、When、Then、And）を変える | 変わる |
| 項目の見出しの名前を変える | 変わらない |
| `- source:` の行（出典）を変える | 変わらない |
| 要求の見出しの下の `- deferred:` の行を足す、消す、変える | 変わる |
| 文書単位の `- deferred:` の行（題名の後）を足す、消す、変える | その文書の要求は変わる。決定表とシナリオは変わらない |
| シナリオの `Scenario:` の行の名前、タグの行（`@source` など）を変える | 変わらない |
| 改行の形（`\r\n` と `\n`）を変える | 変わらない |

出典は IR を直すたびに足されるので、指紋に入れるとそのたびに注意が出てうるさくなります。
[後回し](deferred.ja.md)の宣言は指紋に入れます。後回しを外すことはその機能を出すことなので、ガイドの見直しを促すためです。
文書単位の宣言は要求の本文の外にあるので、その1つ目の行を、その文書の要求の指紋の先頭に加えます。
計算の定義は [REQ-core-203](../ir/core/guides.ja.md#REQ-core-203) にあります。

## 出力

### `guides` の群

<!-- @kotowari[REQ-core-206:e2f3dc5c, EX-core-371:f354c476] -->

`kotowari check` の JSON の最上位と `kotowari status` に、`guides` の群が出ます。

| 鍵 | 型 | 説明 |
|---|---|---|
| `files` | 数 | 読んだガイドの数 |
| `marks` | 数 | 形の正しいガイドの印の1件の数（`REQ-001:…, EX-001:…` なら 2） |

`check --format text` には出ません。`status --format text` には `guides` の行で出ます。

```console
$ kotowari check | jq -c .guides
{"files":1,"marks":3}
$ kotowari status --format text | grep -E '^(guides|findings|complete)'
guides files=1 marks=3
findings error=0 notice=0
complete true
```

glob の書き間違いでガイドが0件になっても、`files=0` で気づけます。

### `guide_stale`（注意）

<!-- @kotowari[REQ-core-204:fa82249e, EX-core-363:9bff9f93, EX-core-365:542b2c69] -->

形の正しい印の1件について、次のときに1件ずつ出ます。

- その ID の項目かシナリオが IR に無い
- その ID のどの項目の指紋も、書かれた指紋と同じでない

| 欄 | 値 |
|---|---|
| path | ガイドの基準のディレクトリからの相対パス |
| line | その印が始まる行 |
| detail | `ID 書かれた指紋 今の指紋`（半角空白で区切る）。IR に無いときの今の指紋は `-` |

REQ-001 の文を「1割」から「2割」に変えると、次のように出ます。

```console
$ kotowari check --format text
guides/discount.md:5 [notice] guide_stale REQ-001 874e14a4 19e1ff5a
$ echo $?
0
```

REQ-002 を IR から消すと、今の指紋は `-` になります。

```console
$ kotowari check --format text
guides/discount.md:11 [notice] guide_stale REQ-002 76709f33 -
```

`guide_stale` は注意なので、終了コードも `status` の `complete` も変えません。
直すまで `check` と `status` の数に残り続けます。

同じ ID の項目が IR に2か所以上あるときは、どちらかの指紋と一致すれば古くないとします。

### `invalid_marker`（誤り）

<!-- @kotowari[REQ-core-202:907dd989, EX-core-367:68e78a4a, EX-core-373:acd8d951] -->

次のときに、1つの印につき1件出ます（終了コード 1）。

- 中が空か、区切りだけ
- 始まりと同じ行に `]` が無い
- 1件に `:` が無い（指紋の書き忘れ）
- `:` の前が ID の形でない（空を含む）
- `:` の後が16進の小文字8文字でない（大文字を含む）

detail は印のある行の全体です。
形の誤った印は、その中の正しい1件も含めてどれも照合しません。

```markdown
## 会員の割引

<!-- @kotowari[REQ-001] -->

## 会員でない注文

<!-- @kotowari[REQ-002:76709F33] -->

## 例

<!-- @kotowari[REQ-001:874e14a4, EX-001] -->
```

```console
$ kotowari check --format text
guides/discount.md:5 [error] invalid_marker <!-- @kotowari[REQ-001] -->
guides/discount.md:9 [error] invalid_marker <!-- @kotowari[REQ-002:76709F33] -->
guides/discount.md:13 [error] invalid_marker <!-- @kotowari[REQ-001:874e14a4, EX-001] -->
$ kotowari check | jq -c .guides
{"files":1,"marks":0}
```

## ガイドを書く手順

<!-- @kotowari[REQ-core-205:7110f061] -->

1. IR を読んで、読み手に向けて節を書きます。
2. その節が説明している項目とシナリオを挙げます。`kotowari query ID` で本文を読み、節が本当にその項目に基づいているかを確かめます。ID が分からなければ `kotowari list` で探します。
3. 節の見出しのすぐ下に印を1つ置き、ID ごとに1件、`kotowari query ID | jq -r '.items[0].fingerprint'` の指紋を写します。
4. 1つの印の ID は数個までにします。もっと要る節はたいてい複数のことを説明しているので、節を割ります（kotowari はこれを検査しません）。
5. `kotowari check` を実行し、ガイドへの指摘が無いことと、`guides` がガイドと1件の数を数えていることを確かめます。

節が説明している項目を印に書き忘れても、kotowari には分かりません。手順 2 が唯一の歯止めです。

`## 関連` のように IR の項目に結び付かない節には、印を付けなくてかまいません。

## ガイドを見直す手順

<!-- @kotowari[REQ-core-205:7110f061, REQ-core-204:fa82249e] -->

`guide_stale` が出たら、1件ごとに次のようにします。

1. 印の属する節を読み、`kotowari query ID` で今の本文を読みます。
2. 節が IR と合わなくなっていれば、先に節を直します。ついでに、節が説明しているのに印に無い項目が無いかを `kotowari query` で確かめ、あれば足します。
3. 節を見直してから、detail の今の指紋を印の1件に写します。
4. 今の指紋が `-` なら、項目は IR から消えています。節のその項目についての記述を書き直すか消し、1件を置き換えた ID に変えるか、1件を消します。
5. もう一度 `kotowari check` を実行し、`guide_stale` が消えたことを確かめます。

節を読まずに指紋だけ写すと、古い節を直さずに注意だけを消すことになります（[つまずき](#注意を消したのに節が古いまま)）。

## よくあるつまずき

### 印を書いたのに `marks` が 0

<!-- @kotowari[REQ-core-200:97b3eb9b, EX-core-366:c019e492] -->

印が HTML のコメントの外にあるか、コードブロックかコードスパンの中にあります。
`<!--` と `-->` で囲み、コードの外に置きます。
`-->` と同じ行の後ろに書いた印も読まれません。

### `guides` の `files` が 0

<!-- @kotowari[REQ-core-198:253e79fd, EX-core-369:248c0ebf] -->

設定に `guides.files` が無いか、glob がどのファイルにも当たっていません。
`guides.files` が無いときは、ガイドに古い印があっても何も出ません。

```console
$ kotowari status --format text | grep '^guides'
guides files=0 marks=0
```

### 1件だけ直したのに、同じ印の他の1件も照合されない

<!-- @kotowari[REQ-core-202:907dd989, EX-core-373:acd8d951] -->

1つの印の中に形の誤った1件が1つでもあると、その印は丸ごと `invalid_marker` になり、正しい1件も照合されません。
`invalid_marker` を先に直してから、`guide_stale` を見ます。

### ガイドとテストの置き場が重なって止まる

<!-- @kotowari[REQ-core-199:55922b15, EX-core-368:d4ff36cf] -->

`tests.files` が広すぎて（`**/*` など）ガイドにも当たっています。
問い合わせの無い言語のファイルでは `@kotowari[` を全部拾うので、ガイドの印がテストの印として読まれるのを防ぐために止まります。
詳しくは [設定ファイル](config.ja.md#ガイドとテストの置き場が重なって止まる) を見てください。

### 注意を消したのに節が古いまま

指紋を写す前に節を見直していません。
指紋の写しは「この節は今の IR に合っている」という申告です。[見直す手順](#ガイドを見直す手順) の 1 と 2 を先に行います。

## 関連

- 仕様: [ガイドの印の IR](../ir/core/guides.ja.md)
- 置き場の設定: [設定ファイル](config.ja.md)
- テストに書く印: [テストに印を付ける](marks.ja.md)
- 指紋を取る: [kotowari list](commands/list.ja.md)、[kotowari query](commands/query.ja.md)
- 揃っているかを見る: [kotowari status](commands/status.ja.md)
- 指摘の種類: [指摘の一覧](findings.ja.md)
