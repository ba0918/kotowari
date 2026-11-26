# 用語の書き方と用語集の表

[English](terms-form.md) | 日本語

二重引用符とバッククォートの書き分け、閉じないバッククォート、用語集の表の範囲を扱う。

## Requirements

### REQ-core-104: 具体的な値は二重引用符で書く

- kind: ubiquitous
- source: docs/decision/records/records.md#A31
- verification: unit

`IR`の`文`は常に、具体的な値を二重引用符で書き、バッククォートでは`用語`と`ID`だけを囲む。

### REQ-core-116: 閉じないバッククォート

- kind: event_driven
- source: docs/decision/records/records.md#A116, docs/decision/records/records.md#A138, docs/decision/records/records.md#A145
- verification: unit

`対象の行`の二重引用符の外のバッククォートの数が奇数のとき、kotowari は行の文字を detail にして unclosed_backtick の`誤り`を出し、その行では`用語`と`ID`の参照の検査を行わず、`曖昧語`の検査は行う。

### REQ-core-117: 用語集の表の範囲

- kind: event_driven
- source: docs/decision/records/records.md#A112, docs/decision/records/records.md#A141, docs/decision/records/records.md#A148, docs/decision/records/2026-09-16-ir-tree.md#A3, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-engine-gaps.md#A27, docs/decision/records/2026-09-23-ir-engine-gaps.md#A39, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

`用語集`の表は、各セルの前後の空白を除いて "Term"、"Meaning"、"Source" の3列と一致するヘッダの行と、各セルが3つ以上の "-"（前後に ":" があってもよい）の区切りの行から始まる表のうち、`用語集`の文書の題名の後、最初の "## " の見出しより前で最初に現れるものであり、空行か表でない行で終わる。それより前にあるヘッダの合わない表も、後にある表も、ヘッダが合うかどうかに依らず`用語`にも`指摘`にもしない（`除外`）。表の外の "|" で始まる行は`用語`にしない。`用語集`の文書があるのにこの形のヘッダと区切りの行が無いとき、kotowari は文書名を detail にして glossary_invalid の`誤り`を出し、その`用語集`の`用語`を0語として扱い、`連鎖`のほかの`用語集`の`用語`は見えたままで検査を続ける。ヘッダと区切りの行があれば、`用語`の行が0でも表はあるものとして扱う。

### REQ-core-122: 用語集の表の崩れた行

- kind: event_driven
- source: docs/decision/records/records.md#A153, docs/decision/records/records.md#A163, docs/decision/records/2026-09-23-ir-engine-gaps.md#A6, docs/decision/records/2026-09-23-ir-engine-gaps.md#A16
- verification: unit

`用語集`の表の中に、セル（行の先頭と末尾の "|" を除いて "|" で分けたもの）が3つ未満の行か`用語`のセルが空の行があるとき、kotowari は行の文字を detail にして invalid_glossary_row の`誤り`を出し、その行を`用語`にしない。セルが4つ以上の行は崩れた行とせず、先頭の3つのセルで`用語`にし、残りのセルを捨てる。

### REQ-core-123: 用語の重複

- kind: event_driven
- source: docs/decision/records/records.md#A154, docs/decision/records/records.md#A162, docs/decision/records/2026-09-16-ir-tree.md#A4
- verification: unit

`用語集`の表の行の`用語`が、同じ`用語集`の前の行か、その`用語集`の`連鎖`の根に近い側の`用語集`にあるとき、kotowari はその行ごとに`用語`を detail にして duplicate_term の`誤り`を出し、重複した行は`用語`の定義に数えない。その語は根に近い側の1つ目の定義によって`用語`として見えたままで、新しい種類の`指摘`は作らない。重複した行は`項目`として扱わず、`出典`の検査（missing_source、source_invalid）も受けない。

## Examples

```gherkin
@id=EX-core-027 @about=REQ-core-123 @source=docs/decision/records/2026-09-16-ir-tree.md#A4
Scenario: 連鎖の上と下で同じ用語を定義したら下の行に出る
  Given "docs/ir/CONTEXT.md" と "docs/ir/network/CONTEXT.md" の両方に "宛先" の行がある
  When "kotowari check" を実行する
  Then "docs/ir/network/CONTEXT.md" の行に duplicate_term の誤りが1件出て、"docs/ir/CONTEXT.md" には出ず、"docs/ir/network/" の文書で "宛先" を囲んでも unknown_term は出ない

@id=EX-core-270 @about=REQ-core-117,REQ-core-174 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A25,docs/decision/records/2026-09-23-ir-engine-gaps.md#A27
Scenario: ヘッダの合う最初の表を用語集の表にする
  Given "CONTEXT.md" に、ヘッダが "a"、"b"、"c" の表と、"宛先" の行を持つヘッダの合う表と、"経路" の行を持つヘッダの合う2つ目の表が、それぞれ空行を挟んでこの順にある
  And 別の文書の`要求`の`文`でバッククォートで囲んだ "宛先" と "経路" がある
  When "kotowari check --format json" を実行する
  Then "CONTEXT.md" には glossary_invalid も invalid_glossary_row も unknown_line も出ない
  And "宛先" には unknown_term が出ず、"経路" には unknown_term が出る

@id=EX-core-271 @about=REQ-core-117 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A25
Scenario: ヘッダの合う表が1つも無ければ用語集は無効になる
  Given "CONTEXT.md" に、ヘッダが "a"、"b"、"c" の表だけがある
  When "kotowari check --format json" を実行する
  Then "CONTEXT.md" に glossary_invalid の誤りが1件出る

@id=EX-core-272 @about=REQ-core-122 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A6,docs/decision/records/2026-09-23-ir-engine-gaps.md#A16
Scenario: 4列の行は先頭の3列で用語になる
  Given "CONTEXT.md" の`用語集`の表に、`用語`のセルが "宛先" で正しい出典のセルの後に4つ目のセルを持つ行と、セルが2つの行がある
  When "kotowari check --format json" を実行する
  Then 4つのセルの行には`指摘`が出ず、"宛先" は`用語`として見える
  And セルが2つの行に invalid_glossary_row の誤りが1件出る
```
