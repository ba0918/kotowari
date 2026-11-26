# ガイドを書く — ガイドの印と指紋

[English](writing-guides.md) | 日本語

ガイドは、決定の文書を読んだ人か LLM が書く、利用者向けの使い方の文書です（このページもその1つです）。
kotowari はガイドを生成しません。
その代わり、ガイドの節ごとに「どの決定を説明しているか」と「書いたときの各決定の指紋」を印として持たせ、`kotowari check` が今の決定の文書と照らします。
決定が変わって古くなった節は、いつ実行しても `guide_stale` の注意として挙がります。

## 書式

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

ガイドの印は HTML のコメントの中に書くので、表示したガイドには出ません。

```markdown
## Member discount

<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:e3adc927] -->

A member's order is 10% off.
```

| 部分 | 形 |
|---|---|
| 始まり | `@kotowari[` |
| 中身 | `行き先:指紋` の形の1件を、カンマで区切って並べる |
| 行き先 | 決定の文書のパスと、決定の見出しの行き先の名前を `#` でつないだもの（`<パス>#<行き先の名前>`）。パスは先頭の言語の側で、基準のディレクトリからの相対 |
| 指紋 | 16進の小文字8文字（`0`〜`9` と `a`〜`f`） |
| 終わり | 始まりと同じ行の `]` |

- 行き先の名前は、決定の文書の中で見出しへリンクを張るときと同じ規則で作ります（[決定の文書を書く](decision-documents.ja.md#決定を変える)）。自分で組み立てる必要はなく、`kotowari list` が各決定の `id` として出します。
- 括弧の内側や `:` の前後に空白を置いてかまいません。
- 同じ行き先の1件を2つ書いても、1件ずつ照合します。
- テストの印（`@kotowari[C-shop-discount-1]`）とは別の規則です。テストの印は [テストに印を付ける](marks.ja.md) を見てください。

## 印を読む場所

<!-- @kotowari[docs/decisions/core/guides.md#guide-markers-are-read-only-from-inside-html-comments:6ad1c700] -->

ガイドは CommonMark として読みます。
ガイドの印として読むのは、HTML のコメントの `<!--` から `-->` までの中にある `@kotowari[` だけです。

| 場所 | 読むか |
|---|---|
| HTML のコメントの中 | 読む。1つのコメントの中と1行の中の印は、すべて拾う |
| コメントの外の本文 | 読まない |
| コードブロックの中（囲みの形と字下げの形） | 読まない |
| コードスパンの中 | 読まない |
| 同じ行の `-->` より後ろ | 読まない |

そのため、印の書き方の例はコードブロックかコードスパンに書けば、印として読まれません。
このページの例もすべてコードブロックの中にあります。

次のガイドでは、読まれる印は最後の1件だけです。

````markdown
# How to write marks

Write a mark as `<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:e3adc927] -->`, not as @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:e3adc927].

```markdown
<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:00000000] -->
```

<!-- note --> @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders:00000000]

<!-- @kotowari[ docs/decisions/shop/discount.md#members-get-10-off-their-orders : e3adc927 ] -->
````

## 置き場の設定

<!-- @kotowari[docs/decisions/core/guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them:05ee20e8, docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

設定ファイルの `guides.files` に、ガイドの置き場を glob で並べます。

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
| 決定の文書の置き場のファイルに当たる | 止まらない。そのファイルをガイドとしても読む |

- ガイドを読むのは `kotowari check` と `kotowari status` だけです。`list` と `query` は読みません。
- glob の読み方、走査、読めないファイルでの停止は `tests.files` と同じです（[設定ファイル](config.ja.md#glob-の読み方)）。

## 指紋

<!-- @kotowari[docs/decisions/core/guides.md#a-stale-section-is-found-by-comparing-the-fingerprint-written-in-the-marker-with-the-current-fingerprint-of-the-decision:c21ccd0f, docs/decisions/core/languages-and-guides.md#a-guide-is-stale-when-the-decision-or-confirmations-change:de5a25a2, docs/decisions/core/list.md#a-confirmation-row-gets-the-tests-that-point-at-it-and-a-decision-gets-a-fingerprint:64ab7c66] -->

指紋は、決定の決めたことと確認することの節から取る、16進の小文字8文字の値です。
`kotowari list` は決定ごとに種類が `decision` の1件を出し、その `id` が行き先、`fingerprint` が写す値です。`kotowari query '<行き先>'` は1つの決定について同じものを出します。
手で計算せず、そこから写してください。

```console
$ kotowari list | jq -r '.items[] | select(.kind == "decision") | "\(.id) \(.fingerprint)"'
docs/decisions/shop/discount.md#members-get-10-off-their-orders e3adc927
docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted 55ce4da7
$ kotowari query 'docs/decisions/shop/discount.md#members-get-10-off-their-orders' | jq -r '.items[0].fingerprint'
e3adc927
```

指紋が変わる変更と変わらない変更は次のとおりです。

| 変更 | 指紋 |
|---|---|
| 決めたことの節（文、表）を変える | 変わる |
| 確認することの節を変える（行を足す、消す、言い換える） | 変わる |
| 状態の行を変える（提案から実装済みへ） | 変わらない |
| 問題か見送った案を変える | 変わらない |
| 決定の見出しの文を変える | 行き先が変わるので、古い行き先を指す印は今の指紋が `-` の `guide_stale` になる |
| 改行の形（`\r\n` と `\n`）を変える | 変わらない |

指紋が追うのは、ガイドが説明するもの、つまり規則と、それが守られていることを示す結果だけです。
決定を実装済みにしても、理由を書き足しても、ガイドの見直しは求めません。

## 出力

### `guides` の群

<!-- @kotowari[docs/decisions/core/output.md#the-top-level-of-checks-json-shows-the-findings-the-count-per-kind-and-the-counts-of-what-was-read:bcec81c7, docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

`kotowari check` の JSON の一番上と `kotowari status` に、`guides` の群が出ます。

| 鍵 | 型 | 説明 |
|---|---|---|
| `files` | 数 | 読んだガイドの数 |
| `marks` | 数 | 形の正しいガイドの印の1件の数（行き先を2つ持つ印なら 2） |

`check --format text` には出ません。`status --format text` では `guides` の行に出ます。

3つの節を持ち、最後の節が2つの決定に印を付けたガイドでは、次のようになります。

```console
$ kotowari check | jq -c .guides
{"files":1,"marks":4}
$ kotowari status --format text | grep -E '^(guides|findings|complete)'
guides files=1 marks=4
findings error=0 notice=0
complete true
```

glob の書き間違いでガイドが0件になっても、`files=0` で気づけます。

### `guide_stale`（注意）

<!-- @kotowari[docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce] -->

形の正しい印の1件について、次のときに1件ずつ出ます。

- 行き先がどの決定も指していない（文書か見出しが無い、見出しが題名か節のもの）
- 決定の今の指紋が、書かれた指紋と違う

| 欄 | 値 |
|---|---|
| path | ガイドの基準のディレクトリからの相対パス |
| line | その印が始まる行 |
| detail | `行き先 書かれた指紋 今の指紋`（半角空白で区切る）。行き先がどの決定も指していないときの今の指紋は `-` |

会員の決定を「10% off」から「20% off」に変えると、次のようになります。

```console
$ kotowari check --format text
guides/discount.md:5 [notice] guide_stale docs/decisions/shop/discount.md#members-get-10-off-their-orders e3adc927 6db223da
guides/discount.md:17 [notice] guide_stale docs/decisions/shop/discount.md#members-get-10-off-their-orders e3adc927 6db223da
$ echo $?
0
```

客の決定の見出しの文を変えると、今の指紋は `-` になります。

```console
$ kotowari check --format text
guides/discount.md:11 [notice] guide_stale docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted 55ce4da7 -
```

`guide_stale` は注意なので、終了コードも `status` の `complete` も変えません。
直すまで `check` と `status` の数に残り続けます。

### `invalid_marker`（誤り）

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

次のときに、1つの印につき1件出ます（終了コード 1）。

- 中が空か、区切りだけ
- 始まりと同じ行に `]` が無い
- 1件に `:` が無い（指紋の書き忘れ）
- `:` の前が行き先の形でない（パスが無い、`#行き先の名前` が無い、確認することの ID を書いた）
- `:` の後が16進の小文字8文字でない（大文字を含む）

detail は印のある行の全体です。
形の誤った印は、その中の正しい1件も含めてどれも照合しません。

```markdown
## Member discount

<!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders] -->

## Orders from guests

<!-- @kotowari[docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55CE4DA7] -->

## Example

<!-- @kotowari[C-shop-discount-1:e3adc927, docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55ce4da7] -->
```

```console
$ kotowari check --format text
guides/bad.md:5 [error] invalid_marker <!-- @kotowari[docs/decisions/shop/discount.md#members-get-10-off-their-orders] -->
guides/bad.md:9 [error] invalid_marker <!-- @kotowari[docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55CE4DA7] -->
guides/bad.md:13 [error] invalid_marker <!-- @kotowari[C-shop-discount-1:e3adc927, docs/decisions/shop/discount.md#orders-of-non-members-are-not-discounted:55ce4da7] -->
```

## ガイドを書く手順

<!-- @kotowari[docs/decisions/core/guides.md#the-occasions-for-writing-and-reviewing-guides-are-placed-in-the-kotowari-skill:cfbcc10a] -->

1. 決定の文書を読んで、読み手に向けて節を書きます。
2. その節が説明している決定を挙げます。`kotowari query '<行き先>'` でそれぞれを読み、節が本当にその決定に基づいているかを確かめます。行き先が分からなければ `kotowari list` で探します。
3. 節の見出しのすぐ下に印を1つ置き、決定ごとに1件、`kotowari query '<行き先>' | jq -r '.items[0].fingerprint'` の指紋を写します。
4. 1つの印の決定は数個までにします。もっと要る節はたいてい複数のことを説明しているので、節を割ります（kotowari はこれを検査しません）。
5. `kotowari check` を実行し、ガイドへの指摘が無いことと、`guides` がガイドと1件の数を数えていることを確かめます。

節が説明している決定を印に書き忘れても、kotowari には分かりません。手順 2 が唯一の歯止めです。

`## 関連` のように決定に結び付かない節には、印を付けなくてかまいません。

## ガイドを見直す手順

<!-- @kotowari[docs/decisions/core/guides.md#the-occasions-for-writing-and-reviewing-guides-are-placed-in-the-kotowari-skill:cfbcc10a, docs/decisions/core/guides.md#a-stale-marker-is-a-guide_stale-notice-and-does-not-change-the-exit-code-or-complete:7b8dc3ce] -->

`guide_stale` が出たら、1件ごとに次のようにします。

1. 印の属する節を読み、`kotowari query '<行き先>'` で今の決定を読みます。
2. 節が決定と合わなくなっていれば、先に節を直します。ついでに、節が説明しているのに印に無い決定が無いかを `kotowari list` と `kotowari query` で確かめ、あれば足します。
3. 節を見直してから、detail の今の指紋を印の1件に写します。
4. 今の指紋が `-` なら、行き先は今どの決定も指していません。その中身を持つ決定（見出しの文を変えた決定や、ほかの話題の決定）を探し、節の記述を書き直すか消し、1件をその決定に付け替えるか、1件を消します。
5. もう一度 `kotowari check` を実行し、`guide_stale` が消えたことを確かめます。

節を読まずに指紋だけ写すと、古い節を直さずに注意だけを消すことになります（[つまずき](#注意を消したのに節が古いまま)）。

## よくあるつまずき

### 印を書いたのに `marks` が 0

<!-- @kotowari[docs/decisions/core/guides.md#guide-markers-are-read-only-from-inside-html-comments:6ad1c700] -->

印が HTML のコメントの外にあるか、コードブロックかコードスパンの中にあります。
`<!--` と `-->` で囲み、コードの外に置きます。
`-->` と同じ行の後ろに書いた印も読まれません。

### `guides` の `files` が 0

<!-- @kotowari[docs/decisions/core/guides.md#guides-are-chosen-by-the-globs-of-guidesfiles-and-only-check-and-status-read-them:05ee20e8] -->

設定に `guides.files` が無いか、glob がどのファイルにも当たっていません。
`guides.files` が無いときは、ガイドに古い印があっても何も出ません。

```console
$ kotowari status --format text | grep '^guides'
guides files=0 marks=0
```

### 1件だけ直したのに、同じ印の他の1件も照合されない

<!-- @kotowari[docs/decisions/core/guides.md#one-entry-of-a-guide-marker-joins-the-target-of-a-decision-and-a-fingerprint-with-a-colon:7ba923d7] -->

1つの印の中に形の誤った1件が1つでもあると、その印は丸ごと `invalid_marker` になり、正しい1件も照合されません。
`invalid_marker` を先に直してから、`guide_stale` を見ます。

### ガイドとテストの置き場が重なって止まる

<!-- @kotowari[docs/decisions/core/guides.md#when-the-guide-and-test-locations-overlap-it-is-a-stop-with-a-configuration-error:4b7987f7] -->

`tests.files` が広すぎて（`**/*` など）ガイドにも当たっています。
問い合わせの無い言語のファイルでは `@kotowari[` を全部拾うので、ガイドの印がテストの印として読まれるのを防ぐために止まります。
詳しくは [設定ファイル](config.ja.md#ガイドとテストの置き場が重なって止まる) を見てください。

### 注意を消したのに節が古いまま

指紋を写す前に節を見直していません。
指紋の写しは「この節は今の決定に合っている」という申告です。[見直す手順](#ガイドを見直す手順) の 1 と 2 を先に行います。

## 関連

- 仕様: [ガイド](../decisions/core/guides.ja.md)、[ガイドが古いとき](../decisions/core/languages-and-guides.ja.md)
- 置き場の設定: [設定ファイル](config.ja.md)
- テストに書く印: [テストに印を付ける](marks.ja.md)
- 指紋を取る: [kotowari list](commands/list.ja.md)、[kotowari query](commands/query.ja.md)
- 揃っているかを見る: [kotowari status](commands/status.ja.md)
- 指摘の種類: [指摘の一覧](findings.ja.md)
