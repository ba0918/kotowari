# 項目の形

要求、決定表、性質、シナリオ、問題の記録の形と、見出しの下の行の検査を扱う。

## 要求

### REQ-042: 項目の形

- 種類: algorithm
- 出典: experiments/003-cli/brainstorm/records.md#A27, experiments/003-cli/brainstorm/records.md#A28, experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A52
- 定義: TBL-011
- 検証: unit

### REQ-043: 形に合わない見出し

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A52, experiments/003-cli/brainstorm/records.md#A82, experiments/003-cli/brainstorm/records.md#A110, experiments/003-cli/brainstorm/records.md#A111
- 検証: unit

"### " の見出しが、REQ、TBL、PROP、FLAG のいずれかの`ID`に名前を続けた "### ID: 名前" の形でないとき、kotowari は unknown_heading の`誤り`を出す。EX の`ID`を見出しに使ったとき、および "#### " より深い見出しのときも同じである。形に合わない見出しの下の行は`項目`として読まない（`除外`）。

### REQ-044: 知らない行

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A52, experiments/003-cli/brainstorm/records.md#A81, experiments/003-cli/brainstorm/ir-form.md#項目, experiments/003-cli/brainstorm/records.md#A87, experiments/003-cli/brainstorm/records.md#A111
- 検証: unit

見出しの下に知らない "- xxx:" の行、または "xxx:" の形でない一覧の行（"- "、"* "、"+ "、数字と "." で始まる行、および "-" だけの行）があるとき、kotowari は読んだ行の文字をそのまま detail にして unknown_field の`誤り`を出す。知らない行の中身は読まない（`ID` が書かれていても参照にしない）。知っている行は`項目`の種類ごとに TBL-011 の「持つ行」の列にあるものだけで、`性質`なら "- 出典:" だけである。

### REQ-045: 同じ行の重複

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A52, experiments/003-cli/brainstorm/records.md#A113
- 検証: unit

見出しの下に同じ知っている "- xxx:" の行が2つ以上あるとき、kotowari は2つ目以降ごとに1件の duplicate_field の`誤り`を出す。知らない行は重複しても unknown_field だけを出す。

### REQ-046: 見出しの下の行の読み方

- 種類: ubiquitous
- 出典: experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/records.md#A52, experiments/003-cli/brainstorm/ir-form.md#項目
- 検証: unit

kotowari は常に、見出しの下の "- " の行を順不同で読み、行の間の空行を許し、"- 定義:"、"- 関係:"、"- 出典:" の値をコンマで区切って読む。

### REQ-047: 文が無い

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/ir-form.md#検査の種類, experiments/003-cli/brainstorm/records.md#A131
- 検証: unit

種類が "algorithm" 以外の`要求`（"- 種類:" の行が無い`要求`を含む）、または`性質`に`文`が無いとき、kotowari は missing_statement の`誤り`を出す。

### REQ-048: 検証の行が無い

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A21, experiments/003-cli/brainstorm/ir-form.md#検査の種類
- 検証: unit

`要求`に "- 検証:" の行が無いとき、kotowari は verification_missing の`誤り`を出す。

### REQ-049: 検証の値の誤り

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A21, experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/ir-form.md#検査の種類
- 検証: unit

`要求`の検証の値が "unit"、"property"、"proof"、"review" のいずれでもないとき、kotowari は verification_invalid の`誤り`を出す。

### REQ-050: 種類の値の誤り

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A28, experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/ir-form.md#検査の種類
- 検証: unit

`要求`か`問題の記録`の`項目`の種類が TBL-011 で決めた値でないとき、kotowari は unknown_kind の`誤り`を出す。

### REQ-051: 定義の無い algorithm

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A21, experiments/003-cli/brainstorm/ir-form.md#項目, experiments/003-cli/brainstorm/ir-form.md#検査の種類
- 検証: unit

種類が "algorithm" の`要求`に、`決定表`か`性質`を指す "- 定義:" の行が無いとき、kotowari は algorithm_without_definition の`誤り`を出す。

## 決定表

### TBL-011: 項目の形

- 出典: experiments/003-cli/brainstorm/records.md#A27, experiments/003-cli/brainstorm/records.md#A28, experiments/003-cli/brainstorm/records.md#A42, experiments/003-cli/brainstorm/ir-form.md#項目, experiments/003-cli/brainstorm/ir-form.md#文書

| 項目 | 置く場所 | 見出し | 持つ行 | 文 |
|---|---|---|---|---|
| 要求 | ## 要求 の下 | ### REQ-nnn: 名前 | - 種類:（event_driven、state_driven、ubiquitous、prohibition、invariant、algorithm）、- 出典:、- 検証:（unit、property、proof、review）、- 定義:（algorithm では持ち、ほかはあってもよい） | algorithm 以外は持つ。algorithm は持たない |
| 決定表 | ## 決定表 の下 | ### TBL-nnn: 名前 | - 出典: と Markdown の表 | なし（表を持つ） |
| 性質 | ## 性質 の下 | ### PROP-nnn: 名前 | - 出典: | 持つ |
| シナリオ | ## 具体例 の下の gherkin のコードブロック | Scenario: の行 | 直前の行のタグ @id=EX-nnn、@about=ID,...、@source=出典,... | なし（ステップの行を持つ） |
| 問題の記録 | 問題の記録の文書 | ### FLAG-nnn: 名前 | - 種類:（contradiction、gap、ambiguity）、- 関係:（ID のコンマ区切り）、- 出典: | 本文を持つ |
| 用語 | 用語集の文書 | 用語、意味、出典の3列の表の行 | なし | なし（意味の列を持つ） |

## 具体例

```gherkin
@id=EX-008 @about=REQ-044 @source=experiments/003-cli/brainstorm/records.md#A42
Scenario: 知らない行は誤りになる
  Given `要求`の見出しの下に "- 優先度: 高" の行がある
  When "kotowari check" を実行する
  Then unknown_field の誤りが1件出る
```
