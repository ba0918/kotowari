# 言語ごとの全体像と UI の文字

`言語の一覧`の言語ごとに`全体像`を書くこと、言語ごとの`参照の表`、描画のエンジンに渡す言語と`UI の文字`、設定の "labels" から`UI の文字`を決めることとその誤りを扱う。

## Requirements

### REQ-core-353: 言語ごとに書く

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A20, docs/decision/records/2026-10-05-localization.md#A26, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A22
- verification: unit

kotowari は常に、"kotowari overview build" と "kotowari overview serve" で、`言語の一覧`の言語ごとに、その言語の`側`の`全体像の元データ`と`目次`を描画のエンジンで描き、`先頭の言語`のページを REQ-core-293 の名前で、ほかの言語のページを ".kotowari/cache/overview/<言語タグ>/" の下に同じ名前で書く。ある言語のページの古い節は、その言語の`側`の`全体像の元データ`の`ガイドの印`から REQ-core-292 のとおりに求める。`言語の一覧`の言語が1つのときは、その言語のページだけを REQ-core-293 の名前で書く。

### REQ-core-354: 言語ごとの参照の表

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A34, docs/decision/records/2026-10-05-localization.md#A38, docs/decision/records/2026-10-05-localization.md#D2
- verification: unit

kotowari は常に、ある言語のページの`参照の表`で、`IR`の`項目`、`シナリオ`、`問題の記録`の`項目`を指す参照の本文を、その`ID`を持つ`IR`の`対`のその言語の`側`の同じ`ID`の`項目`か`シナリオ`から TBL-core-039 のとおりに取る（その`側`があり同じ`ID`を持つことは REQ-core-294 が保つ）。`判断の記録`、ADR、判断の記録でない Markdown を指す参照は、`先頭の言語`のページでは TBL-core-039 の本文を持ち、ほかの言語のページでは本文を持たない。

### REQ-core-355: 描画のエンジンに言語を渡す

- kind: ubiquitous
- source: docs/decision/records/2026-10-05-localization.md#A9, docs/decision/records/2026-10-05-localization.md#A20, docs/decision/records/2026-10-05-localization.md#A32, docs/decision/records/2026-10-05-localization.md#D1
- verification: unit

kotowari は常に、ある言語のページを描くとき、描画のエンジンに、その言語タグ、その言語の`UI の文字`（REQ-core-351）、ほかの言語ごとの名前（その言語の`UI の文字`の "language_name"）とその言語のページの置き場への相対パス（`言語の一覧`の順）を渡す。

### REQ-core-351: UI の文字を決める

- kind: algorithm
- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A7, docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D1
- definition: TBL-core-046
- verification: unit

### REQ-core-352: UI の文字の誤り

- kind: event_driven
- source: docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D3
- verification: unit

`設定ファイル`の "labels" に`言語の一覧`に無い言語タグの鍵があるとき、"labels.<言語タグ>" に TBL-core-046 に無い鍵があるとき、"en" でない`言語の一覧`の言語の "labels.<言語タグ>" が無いか TBL-core-046 の鍵のどれかを欠くとき、または TBL-core-046 で数を入れる鍵の値が "{n}" をちょうど1つ含まないとき、kotowari は設定の誤りを理由に`停止`する。

## Decision tables

### TBL-core-046: UI の文字の鍵と英語の文字

- source: docs/decision/records/2026-10-05-localization.md#A5, docs/decision/records/2026-10-05-localization.md#A7, docs/decision/records/2026-10-05-localization.md#A8, docs/decision/records/2026-10-05-localization.md#A21, docs/decision/records/2026-10-05-localization.md#A32, docs/decision/records/2026-10-05-localization.md#A40, docs/decision/records/2026-10-05-localization.md#D1, docs/decision/records/2026-10-05-localization.md#D3

言語 "en" の`UI の文字`は、この表の英語の文字に "labels.en" で書いた鍵の値を上書きしたものである。"en" でない言語の`UI の文字`は "labels.<言語タグ>" の値そのものである。"{n}" は描くときに数に置き換わる。

| 鍵 | 使う所 | 数を入れる | 英語の文字 |
|---|---|---|---|
| language_name | `切り替えの行`と`全体像`のほかの言語へのリンクに書くその言語の名前 | いいえ | English |
| index_link | `目次`に名前の無い`全体像`のページから一覧へのリンク | いいえ | Overview |
| pages | 一覧の`目次の群`の`全体像`の数 | はい | {n} pages |
| stale_sections | 一覧の古い節の数 | はい | {n} sections to review |
| open_items | 一覧の札が open の項目の数 | はい | {n} open |
| planned_items | 一覧の札が planned の項目の数 | はい | {n} planned |
| stale_mark | 古い節の見出しの近くの印 | いいえ | Not reviewed since the IR changed |
| outline_stale | アウトラインの古い節の印 | いいえ | not reviewed |
| superseded | 置き換え済みの参照の印 | いいえ | (superseded) |
| deferred | 後回しの参照の印 | いいえ | (deferred) |
| compare_before | compare の`部品`の前の列の見出し | いいえ | Before |
| compare_after | compare の`部品`の後の列の見出し | いいえ | After |
| compare_why | compare の`部品`の理由の列の見出し | いいえ | Why |
| state_decided | status の札 decided の表示 | いいえ | Decided |
| state_planned | status の札 planned の表示 | いいえ | Planned |
| state_open | status の札 open の表示 | いいえ | Open |
| state_dropped | status の札 dropped の表示 | いいえ | Dropped |

## Examples

```gherkin
@id=EX-core-540 @about=REQ-core-353,REQ-core-355 @source=docs/decision/records/2026-10-05-localization.md#A20,docs/decision/records/2026-10-05-localization.md#A32,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 英語のページは en の下に書かれ互いにリンクする
  Given 設定の "languages" が "[ja, en]" で、"labels.ja.language_name" が "日本語" であり、`全体像の元データ` "a.md" と "a.en.md" と、その`目次`の`対`がある
  When "kotowari overview build" を実行する
  Then ".kotowari/cache/overview/a.html" と ".kotowari/cache/overview/en/a.html" が書かれ、前者には "en/a.html" への "English" のリンクが、後者には "../a.html" への "日本語" のリンクがある

@id=EX-core-541 @about=REQ-core-353 @source=docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#A20
Scenario: 言語が1つなら下のディレクトリを作らない
  Given 設定の "languages" が "[ja]" で、"labels.ja" に TBL-core-046 のすべての鍵がある
  When "kotowari overview build" を実行する
  Then ".kotowari/cache/overview/" の下に "ja" のディレクトリは無く、一覧は "index.html" である

@id=EX-core-542 @about=REQ-core-354 @source=docs/decision/records/2026-10-05-localization.md#A34
Scenario: 英語のページでは IR の本文は英語の側から取り、判断の記録の本文は持たない
  Given 設定の "languages" が "[ja, en]" で、"REQ-001" の文が "a.md" では "文。"、"a.en.md" では "Text." であり、`全体像の元データ`の`部品`が "REQ-001" と "docs/decision/records/r.md#A1" を参照する
  When "kotowari overview build" を実行する
  Then "en/" の下のページの "REQ-001" の本文は "Text." で、"r A1" は本文を持たず、`先頭の言語`のページの "REQ-001" の本文は "文。" で、"r A1" は決定の文を本文に持つ
@id=EX-core-536 @about=REQ-core-351 @source=docs/decision/records/2026-10-05-localization.md#A7,docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1
Scenario: 英語は kotowari の文字に上書きを重ねる
  Given 設定の "languages" が無く、"labels.en.pages" が "{n} docs" である
  When "kotowari overview build" を実行する
  Then 一覧のページの`目次の群`の数は "<数> docs" の形で、ほかの`UI の文字`は TBL-core-046 の英語の文字である

@id=EX-core-537 @about=REQ-core-352 @source=docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1
Scenario: 英語でない言語の鍵が欠ければ止まる
  Given 設定の "languages" が "[ja]" で、"labels.ja" に "stale_mark" の鍵が無い
  When "kotowari check" を実行する
  Then 終了コードは 2 で、設定の誤りで`停止`する

@id=EX-core-538 @about=REQ-core-352 @source=docs/decision/records/2026-10-05-localization.md#A8,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D1
Scenario: 数を入れる文字に "{n}" が無ければ止まる
  Given 設定の "languages" が無く、"labels.en.pages" が "pages" である
  When "kotowari check" を実行する
  Then 終了コードは 2 で、設定の誤りで`停止`する

@id=EX-core-539 @about=REQ-core-352 @source=docs/decision/records/2026-10-05-localization.md#A21,docs/decision/records/2026-10-05-localization.md#A40,docs/decision/records/2026-10-05-localization.md#D3
Scenario: 一覧に無い言語の文字を書けば止まる
  Given 設定の "languages" が無く、"labels.ja" がある
  When "kotowari check" を実行する
  Then 終了コードは 2 で、設定の誤りで`停止`する
```
