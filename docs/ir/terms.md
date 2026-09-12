# 用語と曖昧語と文書名の参照

バッククォートで囲んだ語、曖昧語、文書名の参照の検査を扱う。

## 要求

### REQ-063: 対象の行

- 種類: algorithm
- 出典: experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A53, experiments/003-cli/brainstorm/records.md#A56
- 定義: TBL-013
- 検証: unit

### REQ-064: 用語集に無い語

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A31, experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A56, experiments/003-cli/brainstorm/ir-form.md#検査の種類, experiments/003-cli/brainstorm/records.md#A63
- 検証: unit

`対象の行`でバッククォートで囲んだものが`用語`でも`ID`でもないとき、kotowari は、それがパスかコード片であっても unknown_term の`誤り`を出す。

### REQ-065: 用語集が無いとき

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A56, experiments/003-cli/brainstorm/records.md#A63
- 検証: unit

`用語集`が無いとき、kotowari は`対象の行`でバッククォートで囲んだもののうち、`ID`でないものをすべて unknown_term の`誤り`にする。

### REQ-066: 曖昧語

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A21, experiments/003-cli/brainstorm/records.md#A41, experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A53, experiments/003-cli/brainstorm/ir-form.md#検査の種類
- 検証: unit

`対象の行`に`曖昧語`が部分一致で含まれるとき、kotowari は vague_word の`誤り`を出す。

### REQ-067: 出現ごとに1件

- 種類: ubiquitous
- 出典: experiments/003-cli/brainstorm/records.md#A53, experiments/003-cli/brainstorm/records.md#A56, experiments/003-cli/brainstorm/ir-form.md#検査の種類
- 検証: unit

kotowari は常に、unknown_term と vague_word の`指摘`を出現ごとに1件出す。

### REQ-068: 囲み忘れを検出しない

- 種類: prohibition
- 出典: experiments/003-cli/brainstorm/records.md#A31
- 検証: review

kotowari は、`用語`をバッククォートで囲み忘れたことを検出してはならない。

### REQ-069: 文書名の参照の見つけ方

- 種類: algorithm
- 出典: experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A47, experiments/003-cli/brainstorm/records.md#A54
- 定義: TBL-014
- 検証: unit

### REQ-070: 参照された文書が無い

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A54
- 検証: unit

`文書名の参照`の名前の文書が`IR`の置き場の直下に無いとき、kotowari は missing_document の`誤り`を出す。

### REQ-104: 具体的な値は二重引用符で書く

- 種類: ubiquitous
- 出典: experiments/003-cli/brainstorm/records.md#A31
- 検証: unit

`IR`の`文`は常に、具体的な値を二重引用符で書き、バッククォートでは`用語`と`ID`だけを囲む。

## 決定表

### TBL-013: 用語と曖昧語の検査の対象

- 出典: experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A53, experiments/003-cli/brainstorm/records.md#A56

| 行 | 検査 |
|---|---|
| 要求の文 | 対象 |
| 性質の文 | 対象 |
| Gherkin の Given、When、Then、And、But の行 | 対象 |
| Gherkin の Scenario、Feature の行 | 対象外 |
| "- " の行 | 対象外 |
| タグの行 | 対象外 |
| 用語集の意味の列 | 対象外 |
| 問題の記録の本文 | 対象外 |

### TBL-014: 文書名の参照の条件

- 出典: experiments/003-cli/brainstorm/records.md#A47, experiments/003-cli/brainstorm/records.md#A54, experiments/003-cli/brainstorm/ir-form.md#文書名の参照, experiments/003-cli/brainstorm/records.md#A64, experiments/003-cli/brainstorm/records.md#A65, experiments/003-cli/brainstorm/records.md#A73

| 順 | 条件 |
|---|---|
| 1 | コードブロックの外にある |
| 2 | 直前が行頭、空白、句読点のいずれか（"/"、"_"、英字の続きは当たらない）。句読点は、半角の ","、"."、":"、";"、"("、")"、二重引用符、一重引用符と、全角の "、"、"。"、"，"、"．"、"（"、"）"、"「"、"」"、"『"、"』"、"“"、"”" |
| 3 | 英小文字と数字とハイフンの並びに ".md" が続く |
| 4 | 二重引用符の中にない |

## 具体例

```gherkin
@id=EX-013 @about=REQ-064 @source=experiments/003-cli/brainstorm/records.md#A42,experiments/003-cli/brainstorm/records.md#A56,experiments/003-cli/brainstorm/ir-form.md#検査の種類
Scenario: 囲んだパスは誤りになる
  Given `要求`の`文`に "src/main.rs" をバッククォートで囲んで書いている
  When "kotowari check" を実行する
  Then detail が "src/main.rs" の unknown_term の誤りが出る

@id=EX-014 @about=REQ-069 @source=experiments/003-cli/brainstorm/records.md#A54
Scenario: スラッシュの後の文書名は拾わない
  Given 文書に "experiments/003-cli/adr/0001-test-marker.md" と書いている
  When "kotowari check" を実行する
  Then "0001-test-marker.md" に missing_document の誤りは出ない
```
