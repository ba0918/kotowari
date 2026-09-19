# 揃っているかの集計

"kotowari status" が`IR`と`印`のある`テスト`と "kotowari check" の`指摘`を集計し、揃っているか（complete）を数と真偽で出すところを扱う。

## 要求

### REQ-162: status の読み取り

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A19
- 検証: unit

kotowari は常に、"kotowari status" で "kotowari check" と同じ設定と置き場から`IR`の文書と`テストのファイル`を読み、check と同じ検査を行い、`指摘`を出さず、`TBL-028` の鍵を持つ集計を1つ標準出力に出す。

### REQ-163: status の停止

- 種類: event_driven
- 出典: docs/decision/records/2026-09-20-query-status.md#A19
- 検証: unit

"kotowari status" で、"kotowari check" が`停止`する条件（設定の誤り、読めないファイル、引数の誤り）が成り立つとき、kotowari は check と同じ理由と文言で`停止`する。

### REQ-164: 集計の鍵

- 種類: algorithm
- 出典: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A12
- 定義: TBL-028
- 検証: unit

### REQ-165: complete と終了コード

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-20-query-status.md#A9, docs/decision/records/2026-09-20-query-status.md#A11
- 検証: unit

kotowari は常に、"kotowari check" の`誤り`が0件で、かつ`問題の記録`の`項目`が0件のときだけ "complete" を true にし、そのとき終了コードを0にし、false のとき終了コードを1にする。`停止`は2である。

### REQ-166: status の出力の形

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-20-query-status.md#A13, docs/decision/records/2026-09-20-query-status.md#A14, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-20-query-status.md#A20
- 検証: unit

kotowari は常に、"kotowari status" の "--format" の値として "json" と "text" の2つだけを受け、既定を "json" にする。"json" では最上位が `TBL-028` の群の鍵だけの JSON を1つ出す。"text" では `TBL-028` の群ごとに "群名 鍵=値 鍵=値" の形の1行を、`TBL-028` の表の順に出す。鍵の語は JSON と同じで、値の間は1つの半角空白で区切り、桁揃えの空白は入れない。"complete" の行は "complete true" か "complete false" である。

## 決定表

### TBL-028: status の鍵

- 出典: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A9, docs/decision/records/2026-09-20-query-status.md#A12, docs/decision/records/2026-09-20-query-status.md#A14, docs/decision/records/2026-09-20-query-status.md#A20, docs/decision/records/2026-09-17-scenario-tests.md#A9, docs/decision/records/2026-09-19-read-commands.md#A24

| 群 | 鍵 | 中身 |
|---|---|---|
| documents | files、lines | 読んだ`IR`の文書の数と行数の合計（"kotowari check" の "files" と "lines" と同じ） |
| items | requirement、table、property、scenario、flag | `ID` を持つ`項目`と`シナリオ`の種類ごとの数 |
| requirements | unit、property、proof、review | "- 検証:" の値ごとの`要求`の数。行の無い`要求`はどれにも数えない |
| requirements | with_tests、without_tests | "- 検証:" が review でない`要求`のうち、その `ID` を`印`に含む`テスト`があるか、その `ID` を "@about" に持つ`シナリオ`の `ID` を`印`に含む`テスト`があるものの数と、無いものの数 |
| requirements | review_with_how_to_verify、review_without_how_to_verify | "- 検証:" が review の`要求`のうち、"- 確かめ方:" の行があるものの数と、無いものの数 |
| requirements | without_examples | その `ID` を "@about" に持つ`シナリオ`が無い`要求`の数 |
| scenarios | with_tests、without_tests | その `ID` を`印`に含む`テスト`がある`シナリオ`の数と、無いものの数 |
| tests | marks | `印`の出現の数。1つの`印`に `ID` が複数あれば `ID` ごとに1つ（list の "tests" の1件と同じ数え方） |
| tests | files | "kotowari check" の "tests" と同じ（`TBL-021`）。"text" では拡張子ごとに "拡張子=ファイルの数" |
| findings | error、notice | "kotowari check" の`指摘`のうち severity が "error" のものと "notice" のものの数 |
| complete | （値だけ） | true か false（REQ-165） |

## 具体例

```gherkin
@id=EX-258 @about=REQ-162,REQ-165,TBL-028 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A11,docs/decision/records/2026-09-20-query-status.md#A20
Scenario: 揃っていれば complete で終了コード 0
  Given IR の文書は "docs/ir/a.md" の1つで、そこに検証が unit の要求 "REQ-001" と検証が review で "- 確かめ方:" の行のある要求 "REQ-002" と "@about=REQ-001" のシナリオ "EX-001" があり、"tests/a.rs" に印 "@kotowari[REQ-001, EX-001]" のテストがある
  And "kotowari check" の指摘は 0 件で、問題の記録は 0 件である
  When "kotowari status" を実行する
  Then 終了コードは 0 で、"requirements" は "unit" が 1、"review" が 1、"with_tests" が 1、"review_with_how_to_verify" が 1 で、"scenarios" の "with_tests" が 1、"tests" の "marks" が 2、"findings" の "error" が 0、"complete" が true である

@id=EX-259 @about=REQ-165,REQ-098 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A10,docs/decision/records/2026-09-20-query-status.md#A11,docs/decision/records/2026-09-20-query-status.md#A17
Scenario: 確かめ方の無い review の要求は誤りになり complete でない
  Given IR に検証が review で "- 確かめ方:" の行の無い要求 "REQ-002" がある
  When "kotowari status" を実行する
  Then 終了コードは 1 で、"findings" の "error" は 1 以上で、"complete" は false である
  And "kotowari check" は detail が "確かめ方" の missing_field の誤りを出す

@id=EX-260 @about=REQ-165 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A11
Scenario: 問題の記録があれば complete でない
  Given "kotowari check" の指摘が 0 件で、問題の記録に "FLAG-001" が1件ある
  When "kotowari status" を実行する
  Then 終了コードは 1 で、"items" の "flag" は 1 で、"complete" は false である

@id=EX-261 @about=REQ-166 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A13,docs/decision/records/2026-09-20-query-status.md#A14,docs/decision/records/2026-09-20-query-status.md#A20
Scenario: text は群ごとに1行
  Given EX-258 と同じ IR とテストがある
  When "kotowari status --format text" を実行する
  Then 出力の1行目は "documents files=1 lines=" で始まり、"requirements " で始まる行は "unit=1 property=0 proof=0 review=1 with_tests=1 without_tests=0 review_with_how_to_verify=1 review_without_how_to_verify=0 without_examples=1" を含み、最後の行は "complete true" である

@id=EX-262 @about=REQ-163 @source=docs/decision/records/2026-09-20-query-status.md#A19
Scenario: 設定が読めなければ check と同じく停止する
  Given 設定ファイルが YAML として読めない
  When "kotowari status" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "kotowari check" と同じ文言である

@id=EX-263 @about=TBL-028 @source=docs/decision/records/2026-09-20-query-status.md#A12,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-17-scenario-tests.md#A9
Scenario: シナリオ経由でテストのある要求も with_tests に数える
  Given IR に検証が unit の要求 "REQ-001" と "@about=REQ-001" のシナリオ "EX-001" があり、"tests/a.rs" に印 "@kotowari[EX-001]" のテストだけがある
  When "kotowari status" を実行する
  Then "requirements" の "with_tests" は 1 で "without_tests" は 0 である
```
