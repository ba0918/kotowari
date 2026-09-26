# 揃っているかの集計

"kotowari status" が`IR`と`印`のある`テスト`と "kotowari check" の`指摘`を集計し、揃っているか（complete）を数と真偽で出すところを扱う。

## Requirements

### REQ-core-162: status の読み取り

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A19, docs/decision/records/2026-09-24-doc-marks.md#A22, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A3, docs/decision/records/2026-09-27-surface-check.md#A24
- verification: unit

kotowari は常に、"kotowari status" で "kotowari check" と同じ設定と置き場から`IR`の文書と`テストのファイル`と`ガイド`を読み、"surface.rules" が空の一覧でなければ`面のファイル`と`面の規則`のファイルと`未記載の面の一覧`も読み、check と同じ検査を行い、`指摘`を出さず、`TBL-core-028` の鍵を持つ集計を1つ標準出力に出す。

### REQ-core-163: status の停止

- kind: event_driven
- source: docs/decision/records/2026-09-20-query-status.md#A19
- verification: unit

"kotowari status" で、"kotowari check" が`停止`する条件（設定の誤り、読めないファイル、引数の誤り）が成り立つとき、kotowari は check と同じ理由と文言で`停止`する。

### REQ-core-164: 集計の鍵

- kind: algorithm
- source: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A12
- definition: TBL-core-028
- verification: unit

### REQ-core-165: complete と終了コード

- kind: ubiquitous
- source: docs/decision/records/2026-09-20-query-status.md#A9, docs/decision/records/2026-09-20-query-status.md#A11
- verification: unit

kotowari は常に、"kotowari check" の`誤り`が0件で、かつ`問題の記録`の`項目`が0件のときだけ "complete" を true にし、そのとき終了コードを0にし、false のとき終了コードを1にする。`停止`は2である。

### REQ-core-166: status の出力の形

- kind: ubiquitous
- source: docs/decision/records/2026-09-19-read-commands.md#A7, docs/decision/records/2026-09-20-query-status.md#A13, docs/decision/records/2026-09-20-query-status.md#A14, docs/decision/records/2026-09-20-query-status.md#A18, docs/decision/records/2026-09-20-query-status.md#A20
- verification: unit

kotowari は常に、"kotowari status" の "--format" の値として "json" と "text" の2つだけを受け、既定を "json" にする。"json" では最上位が `TBL-core-028` の群の鍵だけの JSON を1つ出し、群の順は `TBL-core-028` の表の順で "complete" が最後である。"text" では `TBL-core-028` の群ごとに "群名 鍵=値 鍵=値" の形の1行を、`TBL-core-028` の表の順に出す。鍵の語は JSON と同じで、値の間は1つの半角空白で区切り、桁揃えの空白は入れない。"complete" の行は "complete true" か "complete false" である。

## Decision tables

### TBL-core-028: status の鍵

- source: docs/decision/records/2026-09-20-query-status.md#A8, docs/decision/records/2026-09-20-query-status.md#A9, docs/decision/records/2026-09-20-query-status.md#A12, docs/decision/records/2026-09-20-query-status.md#A14, docs/decision/records/2026-09-20-query-status.md#A20, docs/decision/records/2026-09-17-scenario-tests.md#A9, docs/decision/records/2026-09-19-read-commands.md#A24, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-doc-marks.md#A17, docs/decision/records/2026-09-25-deferred-items.md#A8, docs/decision/records/2026-09-25-deferred-items.md#A15, docs/decision/records/2026-09-25-deferred-items.md#A16, docs/decision/records/2026-09-25-deferred-items.md#A24, docs/decision/records/2026-09-25-deferred-items.md#A26, docs/decision/records/2026-09-27-surface-check.md#A10, docs/decision/records/2026-09-27-surface-check.md#A14, docs/decision/records/2026-09-27-surface-check.md#A17, docs/decision/records/2026-09-27-surface-check.md#A16, docs/decision/records/2026-09-27-surface-check.md#A7

| 群 | 鍵 | 中身 |
|---|---|---|
| documents | files、lines | 読んだ`IR`の文書の数と行数の合計（"kotowari check" の "files" と "lines" と同じ） |
| items | requirement、table、property、scenario、flag | `ID` を持つ`項目`と`シナリオ`の種類ごとの数 |
| requirements | unit、property、proof、review | "- verification:" の値ごとの`要求`の数。行の無い`要求`はどれにも数えない |
| requirements | with_tests、without_tests | "- verification:" が review でなく`後回し`でない`要求`のうち、その `ID` を`印`に含む`テスト`があるか、その `ID` を "@about" に持つ`シナリオ`の `ID` を`印`に含む`テスト`があるものの数と、無いものの数 |
| requirements | review_with_how_to_verify、review_without_how_to_verify | "- verification:" が review の`要求`のうち、"- how_to_verify:" の行があるものの数と、無いものの数 |
| requirements | without_examples | その `ID` を "@about" に持つ`シナリオ`が無い`要求`の数。`後回し`の`要求`も数える |
| requirements | deferred | `後回し`の`要求`の数（"unit" などと同じく`項目`の出現ごとに数え、同じ `ID` の`要求`は REQ-core-032 の1つ目で`後回し`かどうかを決める） |
| scenarios | with_tests、without_tests | `後回しのシナリオ`でない`シナリオ`のうち、その `ID` を`印`に含む`テスト`があるものの数と、無いものの数 |
| scenarios | deferred | `後回しのシナリオ`の数 |
| tests | marks | `印`の出現の数。1つの`印`に `ID` が複数あれば `ID` ごとに1つ（list の "tests" の1件と同じ数え方） |
| tests | files | "kotowari check" の "tests" と同じ（`TBL-core-021`）。"text" では拡張子ごとに "拡張子=ファイルの数" |
| guides | files、marks | "kotowari check" の "guides" と同じ（`TBL-core-005`） |
| surface | total、specified、unspecified | `面`の種類と名前の組の数、そのうち`IR`にあるものの数、`IR`になく`未記載の面の一覧`の形の正しい1件に一致したものの数（REQ-core-229）。"surface.rules" が空の一覧なら3つとも 0 |
| findings | error、notice | "kotowari check" の`指摘`のうち severity が "error" のものと "notice" のものの数 |
| complete | （値だけ） | true か false（REQ-core-165） |

## Examples

```gherkin
@id=EX-core-258 @about=REQ-core-162,REQ-core-165,TBL-core-028 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A11,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 揃っていれば complete で終了コード 0
  Given IR の文書は "docs/ir/a.md" の1つで、そこに検証が unit の要求 "REQ-001" と検証が review で "- how_to_verify:" の行のある要求 "REQ-002" と "@about=REQ-001" のシナリオ "EX-001" があり、"tests/a.rs" に印 "@kotowari[REQ-001, EX-001]" のテストがある
  And "kotowari check" の指摘は 0 件で、問題の記録は 0 件である
  When "kotowari status" を実行する
  Then 終了コードは 0 で、"requirements" は "unit" が 1、"review" が 1、"with_tests" が 1、"review_with_how_to_verify" が 1 で、"scenarios" の "with_tests" が 1、"tests" の "marks" が 2、"findings" の "error" が 0、"complete" が true である

@id=EX-core-259 @about=REQ-core-165,REQ-core-098 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A10,docs/decision/records/2026-09-20-query-status.md#A11,docs/decision/records/2026-09-20-query-status.md#A17,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-23-ir-english-tokens.md#A2,docs/decision/records/2026-09-23-ir-english-tokens.md#A7
Scenario: 確かめ方の無い review の要求は誤りになり complete でない
  Given IR に検証が review で "- how_to_verify:" の行の無い要求 "REQ-002" がある
  When "kotowari status" を実行する
  Then 終了コードは 1 で、"findings" の "error" は 1 以上で、"complete" は false である
  And "kotowari check" は detail が "how_to_verify" の missing_field の誤りを出す

@id=EX-core-260 @about=REQ-core-165 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A9,docs/decision/records/2026-09-20-query-status.md#A11
Scenario: 問題の記録があれば complete でない
  Given "kotowari check" の指摘が 0 件で、問題の記録に "FLAG-001" が1件ある
  When "kotowari status" を実行する
  Then 終了コードは 1 で、"items" の "flag" は 1 で、"complete" は false である

@id=EX-core-261 @about=REQ-core-166 @source=docs/decision/records/2026-09-20-query-status.md#A8,docs/decision/records/2026-09-20-query-status.md#A13,docs/decision/records/2026-09-20-query-status.md#A14,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-25-deferred-items.md#A8
Scenario: text は群ごとに1行
  Given EX-core-258 と同じ IR とテストがある
  When "kotowari status --format text" を実行する
  Then 出力の1行目は "documents files=1 lines=" で始まり、"requirements " で始まる行は "unit=1 property=0 proof=0 review=1 with_tests=1 without_tests=0 review_with_how_to_verify=1 review_without_how_to_verify=0 without_examples=1 deferred=0" を含み、"scenarios " で始まる行は "with_tests=1 without_tests=0 deferred=0" で終わり、最後の行は "complete true" である

@id=EX-core-262 @about=REQ-core-163 @source=docs/decision/records/2026-09-20-query-status.md#A19
Scenario: 設定が読めなければ check と同じく停止する
  Given 設定ファイルが YAML として読めない
  When "kotowari status" を実行する
  Then 終了コードは 2 で、標準エラーの1行目は "kotowari check" と同じ文言である

@id=EX-core-263 @about=TBL-core-028 @source=docs/decision/records/2026-09-20-query-status.md#A12,docs/decision/records/2026-09-20-query-status.md#A20,docs/decision/records/2026-09-17-scenario-tests.md#A9
Scenario: シナリオ経由でテストのある要求も with_tests に数える
  Given IR に検証が unit の要求 "REQ-001" と "@about=REQ-001" のシナリオ "EX-001" があり、"tests/a.rs" に印 "@kotowari[EX-001]" のテストだけがある
  When "kotowari status" を実行する
  Then "requirements" の "with_tests" は 1 で "without_tests" は 0 である

@id=EX-core-398 @about=TBL-core-028,REQ-core-165 @source=docs/decision/records/2026-09-25-deferred-items.md#A4,docs/decision/records/2026-09-25-deferred-items.md#A8,docs/decision/records/2026-09-25-deferred-items.md#A15,docs/decision/records/2026-09-25-deferred-items.md#A16
Scenario: 後回しは件数に出て complete を左右しない
  Given IR に検証が unit の要求 "REQ-001" と "@about=REQ-001" のシナリオ "EX-001" があり、"REQ-001" は後回しで、どちらの ID を含む印も無く、ほかに指摘も問題の記録も無い
  When "kotowari status" を実行する
  Then "requirements" の "deferred" は 1、"with_tests" と "without_tests" と "without_examples" は 0 で、"scenarios" の "deferred" は 1、"with_tests" と "without_tests" は 0 で、"complete" は true である
```
