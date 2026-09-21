# 要求とテストの対応

要求と具体例に印のあるテストがあるか、テストに印があるかの検査を扱う。

## 要求

### REQ-core-085: テストのない要求

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/records.md#A39, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A120, docs/decision/records/2026-09-17-scenario-tests.md#A4, docs/decision/records/2026-09-17-scenario-tests.md#A9, docs/decision/records/2026-09-17-scenario-tests.md#A15, docs/decision/records/2026-09-17-scenario-tests.md#A16
- 検証: unit

検証が "review" 以外の`要求`で、その`ID`を含む`印`も、"@about" にその`ID`を持つ`シナリオ`の`ID`を含む`印`も1つも無いとき、kotowari は requirement_without_test の`誤り`を出す。同じ`ID`の`シナリオ`が2か所以上にあるときの "@about" は REQ-core-032 の1つ目の`シナリオ`のもの。"- 検証:" の行が無い`要求`には verification_missing だけを出し、requirement_without_test は出さない。

### REQ-core-086: 印の無いテスト

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/records.md#A24, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`問い合わせのある言語`の`テスト`に`印`が無いとき、kotowari はその関数の名前を detail にして test_without_id の`誤り`を出す。

### REQ-core-087: 問い合わせの無い言語の対応

- 種類: event_driven
- 出典: docs/decision/records/records.md#A24, docs/decision/records/records.md#A39, docs/decision/adr/0002-tree-sitter.md#結果, docs/decision/records/records.md#A51, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-17-scenario-tests.md#A10
- 検証: unit

`問い合わせの無い言語`の`テストのファイル`を読むとき、kotowari は拾った`印`を requirement_without_test と scenario_without_test を消す側に数え、test_without_id を出さない。

### REQ-core-137: テストのない具体例

- 種類: event_driven
- 出典: docs/decision/records/2026-09-17-scenario-tests.md#A2, docs/decision/records/2026-09-17-scenario-tests.md#A3, docs/decision/records/2026-09-17-scenario-tests.md#A6, docs/decision/records/2026-09-17-scenario-tests.md#A8, docs/decision/records/2026-09-17-scenario-tests.md#A11, docs/decision/records/2026-09-17-scenario-tests.md#A14, docs/decision/records/2026-09-17-scenario-tests.md#A15, docs/decision/records/2026-09-17-scenario-tests.md#A16, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

"@about" の`ID`のうち`要求`として解決できたものの中に検証の値が "review" 以外のものが1つでもある`シナリオ`で、その`ID`を含む`印`が1つも無いとき、kotowari は detail をその`ID`にして scenario_without_test の`誤り`を出す。"@about" に`要求`が無い`シナリオ`と、"@about" の`要求`がすべて "review" の`シナリオ`には出さない。"- 検証:" の行が無い`要求`と検証の値が4つ以外の`要求`は数えない。"@id" が無い`シナリオ`と invalid_id の`シナリオ`には出さない。同じ`ID`の`シナリオ`が2か所以上にあるときは、REQ-core-032 の1つ目の`シナリオ`の "@about" を使い、`誤り`は1つ目の`シナリオ`のタグの行に1件だけ出す。同じ`ID`を複数の`印`が挙げても、1つの`印`が複数の`ID`を挙げても数は見ない。

### REQ-core-088: IR に文書が無いとき

- 種類: event_driven
- 出典: docs/decision/records/records.md#A41, docs/decision/records/records.md#A51
- 検証: unit

`IR`に文書が無いとき、kotowari は`IR`の検査の`指摘`を0件にし、`テスト`との対応の検査を行う。

## 具体例

```gherkin
@id=EX-core-019 @about=REQ-core-088 @source=docs/decision/records/records.md#A51,docs/decision/records/records.md#A29,docs/decision/records/records.md#A26
Scenario: IR が空でも印の無いテストは挙がる
  Given `IR`の置き場に文書が無い
  And "#[test]" の付いた関数が1つあり、`印`が無い
  When "kotowari check" を実行する
  Then test_without_id の誤りが1件出る
  And requirement_without_test の誤りは出ない
  And 終了コードは 1 である

@id=EX-core-121 @about=REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A2,docs/decision/records/2026-09-17-scenario-tests.md#A3,docs/decision/records/2026-09-17-scenario-tests.md#A6,docs/decision/records/2026-09-17-scenario-tests.md#A4
Scenario: 印に挙がっていない具体例は誤りになる
  Given "docs/ir/a.md" に検証が "unit" の "REQ-001" と "@id=EX-201 @about=REQ-001" のシナリオがあり、"@kotowari[REQ-001]" の印を持つテストはあるが、"EX-201" を含む印がどのテストにも無い
  When "kotowari check" を実行する
  Then "line" がそのタグの行で detail が "EX-201" の scenario_without_test の誤りが出る

@id=EX-core-122 @about=REQ-core-085,REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A4,docs/decision/records/2026-09-17-scenario-tests.md#A9,docs/decision/records/2026-09-17-scenario-tests.md#A2,docs/decision/records/2026-09-17-scenario-tests.md#A6,docs/decision/records/records.md#A26
Scenario: 具体例の印は要求の分も満たす
  Given "docs/ir/a.md" に検証が "unit" の "REQ-001" と "@id=EX-201 @about=REQ-001" のシナリオがあり、"#[test]" の付いた関数に印 "@kotowari[EX-201]" があり、"REQ-001" を含む印は無い
  When "kotowari check" を実行する
  Then "REQ-001" を detail にする requirement_without_test の誤りは出ず、"EX-201" を detail にする scenario_without_test の誤りも出ない

@id=EX-core-123 @about=REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A3,docs/decision/records/2026-09-17-scenario-tests.md#A8
Scenario: review の要求だけの具体例には求めない
  Given "REQ-002" の検証が "review" で、"@id=EX-202 @about=REQ-002" のシナリオがあり、"EX-202" を含む印がどのテストにも無い
  When "kotowari check" を実行する
  Then scenario_without_test の誤りは出ない

@id=EX-core-124 @about=REQ-core-087 @source=docs/decision/records/2026-09-17-scenario-tests.md#A10,docs/decision/records/2026-09-17-scenario-tests.md#A6,docs/decision/records/records.md#A24,docs/decision/records/records.md#A39,docs/decision/records/records.md#A36,docs/decision/records/records.md#A47
Scenario: 問い合わせの無い言語のファイルの具体例の印も数える
  Given "tests.files" が "tests/**/*.py" を含み、"docs/ir/a.md" に検証が "unit" の "REQ-001" と "@id=EX-201 @about=REQ-001" のシナリオがあり、"tests/a.py" が "@kotowari[EX-201]" を含み、Rust のテストには "EX-201" を含む印が無い
  When "kotowari check" を実行する
  Then "EX-201" を detail にする scenario_without_test の誤りは出ない

@id=EX-core-125 @about=REQ-core-137 @source=docs/decision/records/2026-09-17-scenario-tests.md#A8,docs/decision/records/2026-09-17-scenario-tests.md#A14
Scenario: 要求を挙げない具体例には求めない
  Given "docs/ir/a.md" に "TBL-001" の決定表と "@id=EX-203 @about=TBL-001" のシナリオがあり、"EX-203" を含む印がどのテストにも無い
  When "kotowari check" を実行する
  Then scenario_without_test の誤りは出ない
```
