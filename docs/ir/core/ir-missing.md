# 項目の欠けた行と表

項目に必須の行と表が無いときの検査と、gherkin のブロックの外にある Scenario: の行を扱う。

## 要求

### REQ-core-098: 必須の行が無い

- 種類: event_driven
- 出典: docs/decision/records/records.md#A68, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A92, docs/decision/records/2026-09-22-ir-engine.md#A75, docs/decision/records/records.md#A157, docs/decision/records/2026-09-19-read-commands.md#A23, docs/decision/records/2026-09-20-query-status.md#A10, docs/decision/records/2026-09-20-query-status.md#A17
- 検証: unit

`要求`に "- 種類:" の行が無いとき、"- 検証:" が review の`要求`に "- 確かめ方:" の行が無いとき、または`問題の記録`の`項目`に "- 種類:" か "- 関係:" の行が無いとき、kotowari は無い行の名前を detail にして missing_field の`誤り`を出す。"- 検証:" の行が無いときは verification_missing だけ、"- 出典:" の行が無いときは missing_source だけを出し、missing_field は出さない。"- 種類:"、"- 検証:"、"- 定義:"、"- 関係:"、"- 確かめ方:" の値が空の行は、行が在るものとして扱い、値の誤りの`指摘`を出す。

### REQ-core-099: 決定表に表が無い

- 種類: event_driven
- 出典: docs/decision/records/records.md#A68, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`決定表`に Markdown の表が無いとき、kotowari はその`決定表`の`ID`を detail にして missing_table の`誤り`を出す。

### REQ-core-100: gherkin のブロックの外の Scenario

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A68, docs/decision/records/ir-form.md#項目
- 検証: unit

kotowari は常に、gherkin のコードブロックの外にある "Scenario:" の行を`シナリオ`と見なさずに無視する。
