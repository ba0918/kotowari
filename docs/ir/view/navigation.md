# 一覧と移動

一覧の`ページ`に添える状態の数と`目次の群`の描き方と、`文書`の`ページ`から`目次`の中のほかの場所へどう移るかと、`文書`の`ページ`の中で`アウトライン`から`節`へどう移るかを扱う。`目次`が`文書`と食い違うときも、view は検査せずに決まった形で描く。

## Requirements

### REQ-view-016: 文書の状態の数

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A2, docs/decision/records/2026-10-05-overview-index.md#A13, docs/decision/records/2026-10-05-overview-index.md#A24, docs/decision/records/2026-10-05-overview-index.md#A35
- verification: unit

view は常に、一覧の`ページ`の`文書`の項目に、古いとされた`節`の数を "見直していない節 <数>"、status の`部品`の項目のうち札が "未決" のものの数を "未決 <数>"、札が "予定" のものの数を "予定 <数>" として、それぞれ数が0でないときだけ添える。札を数えるのは、その`文書`の lead に続く冒頭の`部品`と、`節`の`ブロック`の中のすべての status の`部品`である。

### REQ-view-017: 目次の群の数

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A13, docs/decision/records/2026-10-05-overview-index.md#A15, docs/decision/records/2026-10-05-overview-index.md#A20, docs/decision/records/2026-10-05-overview-index.md#A24, docs/decision/records/2026-10-05-overview-index.md#A35
- verification: unit

view は常に、一覧の`ページ`の`目次の群`（`目次`そのものを含む）の見出しに、その`目次の群`の下に入れ子の深さを問わず描かれる`文書`の項目の数を "<数> ページ" として添え、それらの`文書`の古い`節`の数の合計を "見直していない節 <数>"、札が "未決" のものの数の合計を "未決 <数>" として、合計が0でないときだけ添える。

### REQ-view-018: 目次の群を畳む

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#D1, docs/decision/records/2026-10-05-overview-index.md#A31
- verification: unit

view は常に、一覧の`ページ`の`目次`の中のすべての`目次の群`（`目次`そのものを除く）を開いた状態で描き、その見出しを選ぶと中身を畳めるようにする。畳んだ状態を残さず、view が返すどの`ページ`にもスクリプトを入れない。

### REQ-view-019: 目次の中の位置

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A14, docs/decision/records/2026-10-05-overview-index.md#A18, docs/decision/records/2026-10-05-overview-index.md#A25
- verification: unit

view は常に、`目次`に名前がある`文書`の`ページ`の題名より上に、`目次`からその名前の項目までにたどる`目次の群`の題名を外側から順に描き、それぞれを一覧の`ページ`の中のその`目次の群`の場所へリンクする。`目次`に2回以上出てくる名前では、`目次`を書かれた順に深さ優先でたどって最初に出てくる項目までをたどる。`目次`に名前の無い`文書`の`ページ`には、一覧の`ページ`へのリンクだけを描く。

### REQ-view-020: 同じ目次の群の文書へのリンク

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A14, docs/decision/records/2026-10-05-overview-index.md#A25, docs/decision/records/2026-10-05-overview-index.md#A30, docs/decision/records/2026-10-05-overview-index.md#A36, docs/decision/records/2026-10-05-overview-index.md#A6, docs/decision/records/2026-10-02-whole-picture.md#A26
- verification: unit

view は常に、`目次`に名前がある`文書`の`ページ`の最後の`節`の後に、REQ-view-019 でたどった項目が直接属する`目次の群`の直下の名前の項目のうち、その`文書`と違う名前のものを、`目次`に書かれた順に、その`文書`の題名で描き、その`文書`の`ページ`へ相対パスでリンクする。入れ子の`目次の群`の中の項目と、`描画の入力`に`文書`の無い名前は描かない。

### REQ-view-021: 目次と食い違う入力

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-index.md#A18, docs/decision/records/2026-10-02-whole-picture.md#A22, docs/decision/records/2026-10-05-overview-index.md#A25, docs/decision/records/2026-10-05-overview-index.md#A35
- verification: unit

view は常に、`目次`にあって`描画の入力`に`文書`の無い名前を、一覧の`ページ`に描かず数にも入れない。`目次`に名前の無い`文書`は、その`ページ`を返すが一覧の`ページ`には描かない。`目次`に2回以上出てくる名前は、出てくるたびに描いて数に入れる。view はこれらを誤りとして報告しない。

### REQ-view-022: 節のアウトライン

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A1, docs/decision/records/2026-10-05-overview-page-reading.md#A2, docs/decision/records/2026-10-05-overview-page-reading.md#A3, docs/decision/records/2026-10-05-overview-page-reading.md#A4, docs/decision/records/2026-10-05-overview-page-reading.md#A5, docs/decision/records/2026-10-05-overview-page-reading.md#A6, docs/decision/records/2026-10-05-overview-page-reading.md#A7, docs/decision/records/2026-10-05-overview-page-reading.md#A14, docs/decision/records/2026-10-05-overview-page-reading.md#A17, docs/decision/records/2026-10-05-overview-page-reading.md#D1
- verification: unit

view は常に、`節`を1つ以上持つ`文書`の`ページ`に`アウトライン`を描き、`節`の見出しの文字を`節`の並びの順に並べ、それぞれをその`ページ`の中のその`節`へのリンクにする。古いとされた`節`の項目には、見出しの文字の後に REQ-view-009 の印と同じ意味の印を付ける。`アウトライン`には lead と lead に続く冒頭の`部品`への項目、`節`の中の "### " 以下の見出し、ほかの`ページ`へのリンクを入れない。節へのリンクの場所は1つの`ページ`の中で重ならず、同じ見出しの`節`が2つあっても別の場所にする。一覧の`ページ`と、`節`の無い`文書`の`ページ`には`アウトライン`を描かない。

### REQ-view-023: アウトラインの置き場所

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-overview-page-reading.md#A12, docs/decision/records/2026-10-05-overview-page-reading.md#A13, docs/decision/records/2026-10-05-overview-page-reading.md#A15, docs/decision/records/2026-10-05-overview-page-reading.md#D2
- verification: review
- how_to_verify: `節`を十分に持つ`文書`の`ページ`を描いてブラウザで開き、広い幅では`アウトライン`が本文の左にあって本文をスクロールしても見え続けること、狭い幅では`アウトライン`が題名の下で lead より上にあってスクロールに追従しないことを人が見て確かめる

view は常に、広い画面では`アウトライン`を本文の左に置いて本文をスクロールしても見え続けるように描き、狭い画面では`アウトライン`を題名の下、lead より上に置いてスクロールに追従させない。

## Examples

```gherkin
@id=EX-view-011 @about=REQ-view-016 @source=docs/decision/records/2026-10-05-overview-index.md#A13
Scenario: 0でない状態の数だけがカードに添えられる
  Given 古い節が2つあり、status の部品に札 "未決" の項目が1つあり、札 "予定" の項目が無い文書 "a" がある
  When view で描画する
  Then "index.html" の "a" の項目に "見直していない節 2" と "未決 1" があり、"予定" の文字は無い

@id=EX-view-012 @about=REQ-view-017 @source=docs/decision/records/2026-10-05-overview-index.md#A13,docs/decision/records/2026-10-05-overview-index.md#A20,docs/decision/records/2026-10-05-overview-index.md#A35,docs/decision/records/2026-10-05-overview-index.md#A24
Scenario: 目次の群の数は子孫の文書をすべて数える
  Given 目次の群 "テスト" の項目が "a" と、"b" と "c" を持つ目次の群 "変異テスト" で、"b" に札 "未決" の項目が2つあり、古い節はどの文書にも無い
  When view で描画する
  Then "テスト" の見出しに "3 ページ" と "未決 2" があり、"変異テスト" の見出しに "2 ページ" と "未決 2" があり、どちらにも "見直していない節" の文字は無い

@id=EX-view-013 @about=REQ-view-018 @source=docs/decision/records/2026-10-05-overview-index.md#D1
Scenario: 目次の群は開いた状態で描かれスクリプトを使わない
  Given 入れ子の目次の群を2段持つ目次がある
  When view で描画する
  Then "index.html" のすべての目次の群は開いた状態の、見出しを選ぶと中身を畳める要素で描かれ、どのページにも script の要素は無い

@id=EX-view-014 @about=REQ-view-019,REQ-view-020 @source=docs/decision/records/2026-10-05-overview-index.md#A14,docs/decision/records/2026-10-05-overview-index.md#A25,docs/decision/records/2026-10-05-overview-index.md#A30,docs/decision/records/2026-10-05-overview-index.md#A36,docs/decision/records/2026-10-05-overview-index.md#A15
Scenario: 文書のページに目次の中の位置と同じ群の文書が出る
  Given 目次の題名が "kotowari" で、その項目が目次の群 "テスト"（項目は "a"、"z"、"b"）であり、文書は "a" と "b" だけである
  When view で描画する
  Then "a.html" の題名より上に "kotowari" と "テスト" がこの順にあり、それぞれ "index.html" の中のその目次の群の場所へリンクする
  And "a.html" の最後の節の後に "b" の題名の "b.html" へのリンクがあり、"z" と "a" 自身へのリンクは無い

@id=EX-view-015 @about=REQ-view-021,REQ-view-019 @source=docs/decision/records/2026-10-05-overview-index.md#A18,docs/decision/records/2026-10-05-overview-index.md#A25,docs/decision/records/2026-10-05-overview-index.md#A35,docs/decision/records/2026-10-05-overview-index.md#A24,docs/decision/records/2026-10-05-overview-index.md#A13,docs/decision/records/2026-10-05-overview-index.md#A36
Scenario: 目次と食い違う入力も誤りにせず描く
  Given 目次の項目が "a"、"z"、"a" の順で、文書は "a" と "c" である
  When view で描画する
  Then 返るページに "a.html" と "c.html" があり、"index.html" には "a" の題名が2回あり、"c" の題名と "z" は無く、目次の見出しに "2 ページ" がある
  And "c.html" の題名より上には "index.html" へのリンクだけがある

@id=EX-view-016 @about=REQ-view-022 @source=docs/decision/records/2026-10-05-overview-page-reading.md#A1,docs/decision/records/2026-10-05-overview-page-reading.md#A4,docs/decision/records/2026-10-05-overview-page-reading.md#A2,docs/decision/records/2026-10-05-overview-page-reading.md#A3,docs/decision/records/2026-10-05-overview-page-reading.md#A5,docs/decision/records/2026-10-05-overview-page-reading.md#A14,docs/decision/records/2026-10-05-overview-page-reading.md#A17,docs/decision/records/2026-10-05-overview-page-reading.md#D1
Scenario: アウトラインは節を順に並べ、古い節に印を付ける
  Given 文書 "a" に、古くない節 "読む"、古いとされた節 "書く"、古くない節 "読む" がこの順にあり、節 "書く" の中に "### 細部" の見出しがある
  When view で描画する
  Then "a.html" のアウトラインに "読む"、"書く"、"読む" がこの順にあり、"書く" の文字の後にだけ見直していない節の印がある
  And 3つの項目は "a.html" の中の互いに違う場所へリンクし、それぞれの場所はその節であり、アウトラインに "細部" と lead の結論は無い

@id=EX-view-017 @about=REQ-view-022 @source=docs/decision/records/2026-10-05-overview-page-reading.md#A1,docs/decision/records/2026-10-05-overview-page-reading.md#A4
Scenario: 節の無い文書と一覧にはアウトラインが無い
  Given 節を持たない文書 "a" と、節を1つ持つ文書 "b" がある
  When view で描画する
  Then "b.html" にはアウトラインがあり、"a.html" と "index.html" にはアウトラインが無い
```
