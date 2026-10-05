# 描画

view が`描画の入力`を受け取り、`ページ`の並びを返すまでを扱う。`部品`の種類とそのスキーマは parts.md が、一覧の`ページ`に添える数と`ページ`の間の移動は navigation.md が扱う。view は kotowari の IR も判断の記録も知らない。

## Requirements

### REQ-view-001: 描画の入力

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A42, docs/decision/records/2026-10-02-whole-picture.md#A68, docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A81, docs/decision/records/2026-10-02-whole-picture.md#A82, docs/decision/records/2026-10-04-overview-on-public-api.md#A10, docs/decision/records/2026-10-05-overview-index.md#A7, docs/decision/records/2026-10-05-overview-index.md#A10, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A33
- verification: unit

view は常に、`描画の入力`として`文書`の並びと`参照の表`と`目次`を受け取る。`文書`は名前、題名、冒頭の lead の`部品`、lead に続く冒頭の`部品`の並び、`節`の並びを持ち、`節`は見出しの文字、古いかどうかの真偽、`ブロック`の並びを持つ。`参照の表`の1件は`参照`の文字列、表示名、本文、状態（"current"、"superseded"、"deferred" のいずれか）を持つ。`目次`は1つの`目次の群`で、`目次の群`は題名、省いてよい一行の説明、項目の並びを持ち、項目は`文書`の名前か`目次の群`のどちらかである。

### REQ-view-002: ページの並びを返す

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A26, docs/decision/records/2026-10-02-whole-picture.md#A56, docs/decision/records/2026-10-05-overview-index.md#A18
- verification: unit

view は常に、1回の描画で、一覧の`ページ` "index.html"、`文書`ごとの`ページ` "<名前>.html"、共通のスタイルの`ページ` "style.css" を返し、それ以外の`ページ`を返さない。`文書`ごとの`ページ`と一覧の`ページ`は "style.css" を相対パスで参照し、互いに相対パスでリンクする。ただし一覧の`ページ`から`文書`の`ページ`へのリンクは、`目次`に名前のある`文書`に限る（REQ-view-021）。

### REQ-view-003: ファイルを書かない

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-02-whole-picture.md#A25, docs/decision/records/2026-10-02-whole-picture.md#A43, docs/decision/records/2026-10-02-whole-picture.md#A45, docs/decision/records/2026-10-02-whole-picture.md#A47
- verification: unit

view は、ファイルの読み書き、ネットワークへの接続、`描画の入力`の検査と誤りの報告をしてはならない。

### REQ-view-004: 同じ入力から同じページ

- kind: invariant
- source: docs/decision/records/2026-10-02-whole-picture.md#A10, docs/decision/records/2026-10-02-whole-picture.md#A29
- verification: property

view は常に、同じ`描画の入力`から、名前と中身のバイト列がすべて同じ`ページ`の並びを返す。時刻、乱数、環境変数を`ページ`に入れない。

### REQ-view-005: 一覧のページ

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A24, docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A42, docs/decision/records/2026-10-05-overview-index.md#A2, docs/decision/records/2026-10-05-overview-index.md#A4, docs/decision/records/2026-10-05-overview-index.md#A6, docs/decision/records/2026-10-05-overview-index.md#A8, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A26
- verification: unit

view は常に、一覧の`ページ`に、`目次`の題名を見出しとして描き、`目次`に説明があればその下に描き、その下に`目次`の項目を書かれた順に描く。項目が`目次の群`なら、その題名と説明を見出しにして、その中にその`目次の群`の項目を書かれた順に入れ子で描く。項目が`文書`の名前なら、その`文書`の題名と冒頭の lead の結論を描き、その`文書`の`ページ`へ相対パスでリンクする。

### REQ-view-006: 冒頭の結論

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A38, docs/decision/records/2026-10-04-overview-on-public-api.md#A10, docs/decision/records/2026-10-05-overview-page-reading.md#A17, docs/decision/records/2026-10-05-overview-page-reading.md#A19
- verification: unit

view は常に、`文書`の`ページ`の題名の後に、REQ-view-022 で`アウトライン`を描くときはその`アウトライン`を描き、その後に冒頭の lead の`部品`を描き、続けて lead に続く冒頭の`部品`を並びの順に描き、その後に`節`を並びの順に描く。

### REQ-view-007: Markdown の文章

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A36, docs/decision/records/2026-10-02-whole-picture.md#A42, docs/decision/records/2026-10-02-whole-picture.md#A70, docs/decision/records/2026-10-02-whole-picture.md#A82
- verification: unit

view は常に、Markdown の文章の塊の`ブロック`を CommonMark と GFM の表として HTML にする。文章の中の生の HTML は解釈せず文字として出し、HTML のコメントは出さない。

### REQ-view-008: 参照を開いて見せる

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A65, docs/decision/records/2026-10-02-whole-picture.md#A67, docs/decision/records/2026-10-02-whole-picture.md#A31, docs/decision/records/2026-10-02-whole-picture.md#A73
- verification: unit

view は常に、`参照`を`参照の表`の表示名で描き、それを選ぶとページを移らずにその`参照`の本文を開いて見せる。状態が "superseded" の`参照`には置き換え済みの印を、"deferred" の`参照`には後回しの印を付ける。view はどの`参照`もページの外へのリンクにしない。

### REQ-view-009: 古い節の印

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A39, docs/decision/records/2026-10-02-whole-picture.md#A68
- verification: unit

view は常に、古いとされた`節`の見出しの近くに、IR が変わった後にまだ見直していない`節`であることを示す印を描く。

### REQ-view-010: 外から読み込まない

- kind: prohibition
- source: docs/decision/records/2026-10-02-whole-picture.md#A37, docs/decision/records/2026-10-02-whole-picture.md#A16, docs/decision/records/2026-10-02-whole-picture.md#A82
- verification: unit

view は、`ページ`の中で、フォント、スクリプト、スタイル、画像を、view が返す`ページ`の外から読み込ませてはならない。

## Examples

```gherkin
@id=EX-view-001 @about=REQ-view-002 @source=docs/decision/records/2026-10-02-whole-picture.md#A24,docs/decision/records/2026-10-02-whole-picture.md#A56,docs/decision/records/2026-10-02-whole-picture.md#A42,docs/decision/records/2026-10-02-whole-picture.md#A26,docs/decision/records/2026-10-02-whole-picture.md#A82
Scenario: 2つの文書から4つのページが返る
  Given 名前が "changes" と "guides" の2つの文書を持つ描画の入力がある
  When view で描画する
  Then 返るページの名前は "changes.html"、"guides.html"、"index.html"、"style.css" の4つだけである

@id=EX-view-002 @about=REQ-view-004 @source=docs/decision/records/2026-10-02-whole-picture.md#A10,docs/decision/records/2026-10-02-whole-picture.md#A29
Scenario: 同じ入力を2回描くと同じバイト列になる
  Given 任意の描画の入力がある
  When view で2回描画する
  Then 2回の結果のページの名前と中身のバイト列がすべて等しい

@id=EX-view-003 @about=REQ-view-007 @source=docs/decision/records/2026-10-02-whole-picture.md#A70
Scenario: 文章の中の生の HTML は文字として出る
  Given 節の Markdown の文章に "<script>x</script>" の行と "<!-- @kotowari[REQ-core-001:00000000] -->" の行がある
  When view で描画する
  Then その文書のページに "<script>" の要素は無く、"&lt;script&gt;" の文字があり、"@kotowari[" の文字は無い

@id=EX-view-004 @about=REQ-view-008 @source=docs/decision/records/2026-10-02-whole-picture.md#A65,docs/decision/records/2026-10-02-whole-picture.md#A67
Scenario: 置き換え済みの参照は本文と印付きで描かれ外へリンクしない
  Given 参照の表に、参照 "docs/x.md#A1" が表示名 "x A1"、本文 "古い決定"、状態 "superseded" である1件があり、それを refs に持つ部品がある
  When view で描画する
  Then ページには表示名 "x A1" と置き換え済みの印と、選ぶと開く本文 "古い決定" があり、"docs/x.md" への href は無い

@id=EX-view-005 @about=REQ-view-009 @source=docs/decision/records/2026-10-02-whole-picture.md#A39,docs/decision/records/2026-10-02-whole-picture.md#A68
Scenario: 古い節にだけ印が付く
  Given 古いとされた節 "A" と古くない節 "B" を持つ文書がある
  When view で描画する
  Then 節 "A" の見出しの近くにだけ見直していない節の印がある

@id=EX-view-006 @about=REQ-view-010,REQ-view-003 @source=docs/decision/records/2026-10-02-whole-picture.md#A37,docs/decision/records/2026-10-02-whole-picture.md#A22,docs/decision/records/2026-10-02-whole-picture.md#A82,docs/decision/records/2026-10-02-whole-picture.md#A65,docs/decision/records/2026-10-02-whole-picture.md#A73
Scenario: ページは外のものを読み込まない
  Given 8種の部品をすべて使う文書を持つ描画の入力がある
  When view で描画する
  Then どのページにも "http://" か "https://" で始まる src、href、"@import"、"url(" の参照は無い

@id=EX-view-010 @about=REQ-view-005 @source=docs/decision/records/2026-10-05-overview-index.md#A4,docs/decision/records/2026-10-05-overview-index.md#A6,docs/decision/records/2026-10-05-overview-index.md#A15
Scenario: 一覧は目次に書いた順と入れ子で描かれる
  Given 名前が "a" と "b" の2つの文書があり、目次の題名が "kotowari" で、項目が題名 "テスト" と説明 "テストとの対応" の目次の群（項目は "b"）と、"a" の順である
  When view で描画する
  Then "index.html" の見出しは "kotowari" で、"テスト" の目次の群の中に "b" の題名と lead の結論があり、その後に "a" の題名と lead の結論がある
  And "b" の題名は "b.html" へ、"a" の題名は "a.html" へ相対リンクする
```
