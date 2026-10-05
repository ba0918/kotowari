# 言語

[English](languages.md) | 日本語

view がページの言語を示すこと、view が書く文字を`UI の文字`から取ること、`ほかの言語`の同じ`ページ`へのリンク、本文の無い`参照`を扱う。view はどの言語の文字も中に持たず、言語ごとに1回ずつ描かれる。

## Requirements

### REQ-view-026: ページの言語

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A9
- verification: unit

view は常に、HTML の`ページ`の "html" の要素の lang の属性を、`描画の入力`の言語タグにする。

### REQ-view-027: 文字を中に持たない

- kind: prohibition
- source: docs/decision/records/2026-10-05-localization.md#A9
- verification: unit

view は、`ページ`に書く文字のうち、`文書`、`参照の表`、`目次`、`ほかの言語`から来ないものを、`UI の文字`のほかから取ってはならない。

### REQ-view-028: 数を入れる文字

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A8
- verification: unit

view は常に、TBL-view-002 で数を入れる鍵の`UI の文字`を描くとき、その中の "{n}" をその数の10進に置き換える。

### REQ-view-029: ほかの言語へのリンク

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A20, docs/decision/records/2026-10-05-localization.md#A32
- verification: unit

view は常に、HTML のすべての`ページ`に、`ほかの言語`の1件ごとに、その名前を文字とし、その置き場への相対パスに同じ`ページ`の名前を続けた先へのリンクを、`ほかの言語`の順に描く。`ほかの言語`が空のときはリンクを描かない。どの`ページ`にもスクリプトを入れず、選んだ言語を残さない。

### REQ-view-030: 本文の無い参照

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A34
- verification: unit

`参照の表`の1件が本文を持たないとき、view はその`参照`を表示名だけで描き、選んでも何も開かないようにする。

## Decision tables

### TBL-view-002: UI の文字の鍵

- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A32, docs/decision/records/2026-10-05-localization.md#D1

| 鍵 | 使う所 | 数を入れる |
|---|---|---|
| language_name | 使わない（`ほかの言語`の名前は view の外で決まる） | いいえ |
| index_link | REQ-view-019 の一覧へのリンク | いいえ |
| pages | REQ-view-017 の数 | はい |
| stale_sections | REQ-view-016 と REQ-view-017 の数 | はい |
| open_items | REQ-view-016 と REQ-view-017 の数 | はい |
| planned_items | REQ-view-016 の数 | はい |
| stale_mark | REQ-view-009 の印 | いいえ |
| outline_stale | REQ-view-022 の印 | いいえ |
| superseded | REQ-view-008 の置き換え済みの印 | いいえ |
| deferred | REQ-view-008 の後回しの印 | いいえ |
| compare_before | compare の`部品`の前の列の見出し | いいえ |
| compare_after | compare の`部品`の後の列の見出し | いいえ |
| compare_why | compare の`部品`の理由の列の見出し | いいえ |
| state_decided | status の札 "decided" | いいえ |
| state_planned | status の札 "planned" | いいえ |
| state_open | status の札 "open" | いいえ |
| state_dropped | status の札 "dropped" | いいえ |

## Examples

```gherkin
@id=EX-view-018 @about=REQ-view-026,REQ-view-027 @source=docs/decision/records/2026-10-05-localization.md#A5,docs/decision/records/2026-10-05-localization.md#A9,docs/decision/records/2026-10-05-localization.md#D1
Scenario: ページの言語と文字は入力のとおりになる
  Given 言語タグが "fr" で、UI の文字の "stale_mark" が "Pas encore relu" で、古いとされた節を持つ文書がある
  When view で描画する
  Then どの HTML のページの html の要素も lang の属性が "fr" で、古い節の近くに "Pas encore relu" がある

@id=EX-view-019 @about=REQ-view-027 @source=docs/decision/records/2026-10-05-localization.md#A9
Scenario: UI の文字を変えると view が書く文字が全部変わる
  Given 8種の部品をすべて使い、古い節と置き換え済みと後回しの参照を持つ文書があり、UI の文字の値がすべて "X" の後に鍵の名前を続けたもの（数を入れる鍵では末尾に " {n}"）である
  When view で描画する
  Then ページの文字のうち、文書、参照の表、目次、ほかの言語から来ないものは、すべて "X" で始まる UI の文字か、その "{n}" を数に置き換えたものである

@id=EX-view-020 @about=REQ-view-028 @source=docs/decision/records/2026-10-05-localization.md#A8,docs/decision/records/2026-10-05-localization.md#D1
Scenario: 数は語順の決まった文字に入る
  Given UI の文字の "pages" が "全{n}件" で、目次の群の下に文書が3つある
  When view で描画する
  Then その目次の群の見出しに "全3件" がある

@id=EX-view-021 @about=REQ-view-029 @source=docs/decision/records/2026-10-05-localization.md#A20,docs/decision/records/2026-10-05-localization.md#A32
Scenario: ほかの言語の同じページへリンクする
  Given ほかの言語が名前 "English" と置き場 "en/" の1件で、文書 "a" がある
  When view で描画する
  Then "a.html" に "en/a.html" への "English" のリンクがあり、"index.html" に "en/index.html" への "English" のリンクがあり、どのページにも script の要素は無い

@id=EX-view-022 @about=REQ-view-029 @source=docs/decision/records/2026-10-05-localization.md#A20
Scenario: ほかの言語が無ければリンクは無い
  Given ほかの言語が空で、文書 "a" がある
  When view で描画する
  Then "a.html" と "index.html" に、ほかの言語のページへのリンクは無い

@id=EX-view-023 @about=REQ-view-030 @source=docs/decision/records/2026-10-05-localization.md#A34
Scenario: 本文の無い参照は表示名だけで開かない
  Given 参照の表に、参照 "docs/x.md#A1" が表示名 "x A1"、本文なし、状態 "current" である1件があり、それを refs に持つ部品がある
  When view で描画する
  Then ページには表示名 "x A1" があり、選ぶと開く要素は無い
```
