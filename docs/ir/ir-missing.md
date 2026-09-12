# 項目の欠けた行と表

項目に必須の行と表が無いときの検査と、gherkin のブロックの外にある Scenario: の行を扱う。

## 要求

### REQ-098: 必須の行が無い

- 種類: event_driven
- 出典: brainstorm/records.md#A68, brainstorm/ir-form.md#項目, brainstorm/ir-form.md#検査の種類
- 検証: unit

`要求`に "- 種類:"、"- 出典:"、"- 検証:" のいずれかの行が無いとき、`決定表`か`性質`に "- 出典:" の行が無いとき、または`問題の記録`の`項目`に "- 種類:"、"- 関係:"、"- 出典:" のいずれかの行が無いとき、kotowari は無い行の名前を detail にして missing_field の`誤り`を出す。

### REQ-099: 決定表に表が無い

- 種類: event_driven
- 出典: brainstorm/records.md#A68, brainstorm/ir-form.md#検査の種類
- 検証: unit

`決定表`に Markdown の表が無いとき、kotowari はその`決定表`の`ID`を detail にして missing_table の`誤り`を出す。

### REQ-100: gherkin のブロックの外の Scenario

- 種類: ubiquitous
- 出典: brainstorm/records.md#A68, brainstorm/ir-form.md#項目
- 検証: unit

kotowari は常に、gherkin のコードブロックの外にある "Scenario:" の行を`シナリオ`と見なさずに無視する。
