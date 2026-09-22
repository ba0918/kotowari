# 用語の書き方と用語集の表

二重引用符とバッククォートの書き分け、閉じないバッククォート、用語集の表の範囲を扱う。

## 要求

### REQ-core-104: 具体的な値は二重引用符で書く

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A31
- 検証: unit

`IR`の`文`は常に、具体的な値を二重引用符で書き、バッククォートでは`用語`と`ID`だけを囲む。

### REQ-core-116: 閉じないバッククォート

- 種類: event_driven
- 出典: docs/decision/records/records.md#A116, docs/decision/records/records.md#A138, docs/decision/records/records.md#A145
- 検証: unit

`対象の行`の二重引用符の外のバッククォートの数が奇数のとき、kotowari は行の文字を detail にして unclosed_backtick の`誤り`を出し、その行では`用語`と`ID`の参照の検査を行わず、`曖昧語`の検査は行う。

### REQ-core-117: 用語集の表の範囲

- 種類: event_driven
- 出典: docs/decision/records/records.md#A112, docs/decision/records/records.md#A141, docs/decision/records/records.md#A148, docs/decision/records/2026-09-16-ir-tree.md#A3
- 検証: unit

`用語集`の表は、各セルの前後の空白を除いて "用語"、"意味"、"出典" の3列と一致するヘッダの行と、各セルが3つ以上の "-"（前後に ":" があってもよい）の区切りの行から始まり、空行か表でない行で終わる。表の外の "|" で始まる行は`用語`にしない。`用語集`の文書があるのにこの形のヘッダと区切りの行が無いとき、kotowari は文書名を detail にして glossary_invalid の`誤り`を出し、その`用語集`の`用語`を0語として扱い、`連鎖`のほかの`用語集`の`用語`は見えたままで検査を続ける。ヘッダと区切りの行があれば、`用語`の行が0でも表はあるものとして扱う。

### REQ-core-122: 用語集の表の崩れた行

- 種類: event_driven
- 出典: docs/decision/records/records.md#A153, docs/decision/records/records.md#A163, docs/decision/records/2026-09-22-ir-engine.md#A83
- 検証: unit

`用語集`の表の中に、セル（行の先頭と末尾の "|" を除いて "|" で分けたもの）が3つ未満の行か`用語`のセルが空の行があるとき、kotowari は行の文字を detail にして invalid_glossary_row の`誤り`を出し、その行を`用語`にしない。両方に当たる行でも`誤り`は1件だけである。

### REQ-core-123: 用語の重複

- 種類: event_driven
- 出典: docs/decision/records/records.md#A154, docs/decision/records/records.md#A162, docs/decision/records/2026-09-16-ir-tree.md#A4
- 検証: unit

`用語集`の表の行の`用語`が、同じ`用語集`の前の行か、その`用語集`の`連鎖`の根に近い側の`用語集`にあるとき、kotowari はその行ごとに`用語`を detail にして duplicate_term の`誤り`を出し、重複した行は`用語`の定義に数えない。その語は根に近い側の1つ目の定義によって`用語`として見えたままで、新しい種類の`指摘`は作らない。重複した行は`項目`として扱わず、`出典`の検査（missing_source、source_invalid）も受けない。

## 具体例

```gherkin
@id=EX-core-027 @about=REQ-core-123 @source=docs/decision/records/2026-09-16-ir-tree.md#A4
Scenario: 連鎖の上と下で同じ用語を定義したら下の行に出る
  Given "docs/ir/CONTEXT.md" と "docs/ir/network/CONTEXT.md" の両方に "宛先" の行がある
  When "kotowari check" を実行する
  Then "docs/ir/network/CONTEXT.md" の行に duplicate_term の誤りが1件出て、"docs/ir/CONTEXT.md" には出ず、"docs/ir/network/" の文書で "宛先" を囲んでも unknown_term は出ない
```
