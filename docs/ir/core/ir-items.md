# 項目の形

要求、決定表、性質、シナリオ、問題の記録の形と、見出しの下の行の検査を扱う。

## 要求

### REQ-core-042: 項目の形

- 種類: algorithm
- 出典: docs/decision/records/records.md#A27, docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/records.md#A52
- 定義: TBL-core-011
- 検証: unit

### REQ-core-043: 形に合わない見出し

- 種類: event_driven
- 出典: docs/decision/records/records.md#A52, docs/decision/records/records.md#A82, docs/decision/records/records.md#A111, docs/decision/records/2026-09-22-ir-engine.md#A84
- 検証: unit

"### " の見出しが、REQ、TBL、PROP、FLAG のいずれかの`ID`に名前を続けた "### ID: 名前" の形でないとき、kotowari は unknown_heading の`誤り`を出す。EX の`ID`を見出しに使ったとき、および "#### " より深い見出しのときも同じである。形に合わない見出しの下の行も`項目`の規則で読み、当てはまる`指摘`を出す。

### REQ-core-044: 知らない行

- 種類: event_driven
- 出典: docs/decision/records/records.md#A42, docs/decision/records/records.md#A52, docs/decision/records/records.md#A81, docs/decision/records/ir-form.md#項目, docs/decision/records/records.md#A87, docs/decision/records/records.md#A111
- 検証: unit

見出しの下に知らない "- xxx:" の行、または "xxx:" の形でない一覧の行（"- "、"* "、"+ "、数字と "." で始まる行、および "-" だけの行）があるとき、kotowari は読んだ行の文字をそのまま detail にして unknown_field の`誤り`を出す。知らない行の中身は読まない（`ID` が書かれていても参照にしない）。知っている行は`項目`の種類ごとに TBL-core-011 の「持つ行」の列にあるものだけで、`性質`なら "- 出典:" だけである。

### REQ-core-045: 同じ行の重複

- 種類: event_driven
- 出典: docs/decision/records/records.md#A52, docs/decision/records/records.md#A113, docs/decision/records/2026-09-22-ir-engine.md#A75
- 検証: unit

見出しの下に同じ知っている "- xxx:" の行が2つ以上あるとき、kotowari は2つ目の行に1件の duplicate_field の`誤り`を出す。3本以上あっても1件である。知らない行は重複しても unknown_field だけを出す。

### REQ-core-046: 見出しの下の行の読み方

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A42, docs/decision/records/records.md#A52, docs/decision/records/ir-form.md#項目
- 検証: unit

kotowari は常に、見出しの下の "- " の行を順不同で読み、行の間の空行を許し、"- 定義:"、"- 関係:"、"- 出典:" の値をコンマで区切って読む。

### REQ-core-178: 文を1行ずつ読む

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-23-ir-engine-gaps.md#A1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A2, docs/decision/records/2026-09-23-ir-engine-gaps.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A21, docs/decision/records/2026-09-23-ir-engine-gaps.md#A22, docs/decision/records/2026-09-23-ir-engine-gaps.md#A26, docs/decision/records/2026-09-23-ir-engine-gaps.md#A35, docs/decision/records/2026-09-23-ir-engine-gaps.md#A38
- 検証: unit

kotowari は常に、取り込んだスキーマの宣言（REQ-core-179）によって、見出しの下の、一覧の行でも表の行でもない空でない行を1行ずつ`文`として読む。"- 名前:" の行とほかの一覧の行は1行で終わり、その直後に空行なしで続く行も、空行の後に字下げして続く一覧でない行も`文`として読み、一覧の行の値に含めない。字下げした一覧の行は一覧の行として読む。引用、水平線、HTML、画像の行と、区切りの行を持たず表にならない "|" で始まる行も`文`として読む。見出しは1つ以上の "#" の直後に空白が続く行だけで、"---" か "===" だけの行はその前の行とともに`文`である。`コードブロック`は囲みの行で始まるものだけで、空行の後に4つ以上の空白で字下げした行は`文`である。

### REQ-core-047: 文が無い

- 種類: event_driven
- 出典: docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A131, docs/decision/records/2026-09-22-ir-engine.md#A75
- 検証: unit

種類が "algorithm" 以外の`要求`（"- 種類:" の行が無い`要求`を含む）、`性質`、または`問題の記録`の`項目`に`文`が無いとき、kotowari は missing_statement の`誤り`を出す。

### REQ-core-048: 検証の行が無い

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`要求`に "- 検証:" の行が無いとき、kotowari は verification_missing の`誤り`を出す。

### REQ-core-049: 検証の値の誤り

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`要求`の検証の値が "unit"、"property"、"proof"、"review" のいずれでもないとき、kotowari は verification_invalid の`誤り`を出す。

### REQ-core-050: 種類の値の誤り

- 種類: event_driven
- 出典: docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類
- 検証: unit

`要求`か`問題の記録`の`項目`の種類が TBL-core-011 で決めた値でないとき、kotowari は unknown_kind の`誤り`を出す。

### REQ-core-051: 定義の無い algorithm

- 種類: event_driven
- 出典: docs/decision/records/records.md#A21, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-22-ir-engine.md#A71
- 検証: unit

種類が "algorithm" の`要求`に、`決定表`か`性質`を指す "- 定義:" の行が無いとき（行そのものが無いときを含む）、kotowari は algorithm_without_definition の`誤り`を出す。

## 決定表

### TBL-core-011: 項目の形

- 出典: docs/decision/records/records.md#A27, docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#文書, docs/decision/records/2026-09-19-read-commands.md#A5, docs/decision/records/2026-09-19-read-commands.md#A11, docs/decision/records/2026-09-20-query-status.md#A10, docs/decision/records/2026-09-23-ir-engine-gaps.md#A4, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25

| 項目 | 置く場所 | 見出し | 持つ行 | 文 |
|---|---|---|---|---|
| 要求 | ## 要求 の下 | ### REQ-nnn: 名前 | - 種類:（event_driven、state_driven、ubiquitous、prohibition、invariant、algorithm）、- 出典:、- 検証:（unit、property、proof、review）、- 定義:（algorithm では持ち、ほかはあってもよい）、- 確かめ方:（"- 検証:" が review なら持つ。ほかはあってもよい。人か LLM が確かめる手順の自由文） | algorithm 以外は持つ。algorithm は持たない |
| 決定表 | ## 決定表 の下 | ### TBL-nnn: 名前 | - 出典: と Markdown の表 | なし（表を持つ） |
| 性質 | ## 性質 の下 | ### PROP-nnn: 名前 | - 出典: | 持つ |
| シナリオ | ## 具体例 の下の gherkin のコードブロック | Scenario: の行 | 直前の行のタグ @id=EX-nnn、@about=ID,...、@source=出典,... | なし（ステップの行を持つ） |
| 問題の記録 | 問題の記録の文書の ## 問題の記録 の下か、節を挟まずに文書の直下（同じ文書に両方があってもよい） | ### FLAG-nnn: 名前 | - 種類:（contradiction、gap、ambiguity）、- 関係:（ID のコンマ区切り）、- 出典: | 本文を持つ |
| 用語 | 用語集の文書 | 用語、意味、出典の3列のヘッダを持つ最初の表の行（REQ-core-117） | なし | なし（意味の列を持つ） |

## 具体例

```gherkin
@id=EX-core-008 @about=REQ-core-044 @source=docs/decision/records/records.md#A42
Scenario: 知らない行は誤りになる
  Given `要求`の見出しの下に "- 優先度: 高" の行がある
  When "kotowari check" を実行する
  Then unknown_field の誤りが1件出る

@id=EX-core-273 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A1,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
Scenario: 行の直後に続く文は行の値に入らない
  Given 検証が "review" の`要求`で、"- 確かめ方: 見る" の次の行に空行を挟まずに`文`があり、"- 検証: review" の次の行にも空行を挟まずに`文`がある
  When "kotowari check --format json" を実行する
  Then その`要求`に missing_statement も verification_invalid も requirement_without_test も出ない

@id=EX-core-274 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A2,docs/decision/records/2026-09-23-ir-engine-gaps.md#A5,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13
Scenario: 字下げした行と引用と HTML の行の閉じないバッククォートは誤りになる
  Given `要求`の "- 出典:" の行の後に空行を挟んで字下げした行があり、その下に引用の行と HTML の行があり、3つの行にはどれも閉じないバッククォートがある
  When "kotowari check --format json" を実行する
  Then 3つの行のそれぞれに unclosed_backtick の誤りが1件ずつ出る

@id=EX-core-275 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A26
Scenario: 表にならない縦棒の行は文として検査を受ける
  Given `要求`の下に、区切りの行を持たず閉じないバッククォートを含む "| a |" で始まる行がある
  When "kotowari check --format json" を実行する
  Then その行に unclosed_backtick の誤りが1件出て、unknown_line は出ない

@id=EX-core-276 @about=REQ-core-042,TBL-core-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A4,docs/decision/records/2026-09-23-ir-engine-gaps.md#A15
Scenario: 節の無い問題の記録の項目も読む
  Given "FLAGS.md" に "## 問題の記録" の節が無く、`題名`の後に "### FLAG-001: 例" の`項目`と、その "- 種類:"、"- 関係:"、"- 出典:" の行と本文がある
  And 別の "FLAGS.md" に、節の無い "### FLAG-002: 例" と、"## 問題の記録" の下の "### FLAG-003: 例" がある
  When "kotowari check --format json" を実行する
  Then unknown_heading も unknown_field も unknown_line も出ない
  And "kotowari status" の問題の記録の項目の数は3である
```
