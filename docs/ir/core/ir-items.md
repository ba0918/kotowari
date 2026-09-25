# 項目の形

要求、決定表、性質、シナリオ、問題の記録の形と、見出しの下の行の検査を扱う。

## Requirements

### REQ-core-042: 項目の形

- kind: algorithm
- source: docs/decision/records/records.md#A27, docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/records.md#A52
- definition: TBL-core-011
- verification: unit

### REQ-core-043: 形に合わない見出し

- kind: event_driven
- source: docs/decision/records/records.md#A52, docs/decision/records/records.md#A82, docs/decision/records/records.md#A111, docs/decision/records/2026-09-22-ir-engine.md#A84, docs/decision/records/2026-09-24-review8-gaps.md#A2
- verification: unit

"### " の見出しが、REQ、TBL、PROP、FLAG のいずれかの`ID`に名前を続けた "### ID: 名前" の形でないとき（":" の後に名前が無いときを含む）、kotowari は unknown_heading の`誤り`を出す。EX の`ID`を見出しに使ったとき、および "#### " より深い見出しのときも同じである。形に合わない見出しの下の行も`項目`の規則で読み、当てはまる`指摘`を出す。

### REQ-core-044: 知らない行

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A52, docs/decision/records/records.md#A81, docs/decision/records/ir-form.md#項目, docs/decision/records/records.md#A87, docs/decision/records/records.md#A111, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-review3-gaps.md#A2
- verification: unit

見出しの下に知らない "- xxx:" の行、または "xxx:" の形でない一覧の行（"- "、"* "、"+ "、数字と "." か ")" で始まる行、および "-" だけの行）があるとき、kotowari は読んだ行の文字をそのまま detail にして unknown_field の`誤り`を出す。知らない行の中身は読まない（`ID` が書かれていても参照にしない）。知っている行は`項目`の種類ごとに TBL-core-011 の「持つ行」の列にあるものだけで、`性質`なら "- source:" だけである。

### REQ-core-045: 同じ行の重複

- kind: event_driven
- source: docs/decision/records/records.md#A52, docs/decision/records/records.md#A113, docs/decision/records/2026-09-22-ir-engine.md#A75, docs/decision/records/2026-09-24-review8-gaps.md#A3
- verification: unit

見出しの下に同じ知っている "- xxx:" の行が2つ以上あるとき、kotowari は2つ目の行に1件の duplicate_field の`誤り`を出す。3本以上あっても1件である。読むのは1つ目の行の値で、2つ目以降の行の値は読まない。知らない行は重複しても unknown_field だけを出す。

### REQ-core-046: 見出しの下の行の読み方

- kind: ubiquitous
- source: docs/decision/records/records.md#A42, docs/decision/records/records.md#A52, docs/decision/records/ir-form.md#項目, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-25-deferred-items.md#A2
- verification: unit

kotowari は常に、見出しの下の "- " の行を順不同で読み、行の間の空行を許し、"- definition:"、"- related:"、"- source:"、"- deferred:" の値をコンマで区切って読む。

### REQ-core-178: 文を1行ずつ読む

- kind: ubiquitous
- source: docs/decision/records/2026-09-23-ir-engine-gaps.md#A1, docs/decision/records/2026-09-23-ir-engine-gaps.md#A2, docs/decision/records/2026-09-23-ir-engine-gaps.md#A5, docs/decision/records/2026-09-23-ir-engine-gaps.md#A13, docs/decision/records/2026-09-23-ir-engine-gaps.md#A21, docs/decision/records/2026-09-23-ir-engine-gaps.md#A22, docs/decision/records/2026-09-23-ir-engine-gaps.md#A26, docs/decision/records/2026-09-23-ir-engine-gaps.md#A35, docs/decision/records/2026-09-23-ir-engine-gaps.md#A38, docs/decision/records/2026-09-23-ir-engine-gaps.md#A43
- verification: unit

kotowari は常に、取り込んだスキーマの宣言（REQ-core-179）によって、見出しの下の、一覧の行でも表の行でもない空でない行を1行ずつ`文`として読む。"- 名前:" の行とほかの一覧の行は1行で終わり、その直後に空行なしで続く行も、空行の後に字下げして続く一覧でない行も`文`として読み、一覧の行の値に含めない。字下げした一覧の行は一覧の行として読む。引用、水平線、HTML、画像の行と、区切りの行を持たず表にならない "|" で始まる行も`文`として読む。見出しは CommonMark の ATX 見出しの行（行頭の空白は3つまで、"#" は1〜6個、その後が空白か行末）だけで、"---" か "===" だけの行はその前の行とともに`文`である。`コードブロック`は囲みの行で始まるものだけで、空行の後に4つ以上の空白で字下げした行は`文`である。

### REQ-core-047: 文が無い

- kind: event_driven
- source: docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/records.md#A131, docs/decision/records/2026-09-22-ir-engine.md#A75, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

種類が "algorithm" 以外の`要求`（"- kind:" の行が無い`要求`を含む）、`性質`、または`問題の記録`の`項目`に`文`が無いとき、kotowari は missing_statement の`誤り`を出す。

### REQ-core-048: 検証の行が無い

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

`要求`に "- verification:" の行が無いとき、kotowari は verification_missing の`誤り`を出す。

### REQ-core-049: 検証の値の誤り

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類
- verification: unit

`要求`の検証の値が "unit"、"property"、"proof"、"review" のいずれでもないとき、kotowari は verification_invalid の`誤り`を出す。

### REQ-core-050: 種類の値の誤り

- kind: event_driven
- source: docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#検査の種類
- verification: unit

`要求`か`問題の記録`の`項目`の種類が TBL-core-011 で決めた値でないとき、kotowari は unknown_kind の`誤り`を出す。

### REQ-core-051: 定義の無い algorithm

- kind: event_driven
- source: docs/decision/records/records.md#A21, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#検査の種類, docs/decision/records/2026-09-22-ir-engine.md#A71, docs/decision/records/2026-09-23-ir-english-tokens.md#A2
- verification: unit

種類が "algorithm" の`要求`に、`決定表`か`性質`を指す "- definition:" の行が無いとき（行そのものが無いときを含む）、kotowari は algorithm_without_definition の`誤り`を出す。

## Decision tables

### TBL-core-011: 項目の形

- source: docs/decision/records/records.md#A27, docs/decision/records/records.md#A28, docs/decision/records/records.md#A42, docs/decision/records/ir-form.md#項目, docs/decision/records/ir-form.md#文書, docs/decision/records/2026-09-19-read-commands.md#A5, docs/decision/records/2026-09-19-read-commands.md#A11, docs/decision/records/2026-09-20-query-status.md#A10, docs/decision/records/2026-09-23-ir-engine-gaps.md#A4, docs/decision/records/2026-09-23-ir-engine-gaps.md#A15, docs/decision/records/2026-09-23-ir-engine-gaps.md#A25, docs/decision/records/2026-09-23-ir-english-tokens.md#A2, docs/decision/records/2026-09-24-review8-gaps.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A1, docs/decision/records/2026-09-25-deferred-items.md#A2

| 項目 | 置く場所 | 見出し | 持つ行 | 文 |
|---|---|---|---|---|
| 要求 | ## Requirements の下 | ### REQ-nnn: 名前 | - kind:（event_driven、state_driven、ubiquitous、prohibition、invariant、algorithm）、- source:、- verification:（unit、property、proof、review）、- definition:（algorithm では持ち、ほかはあってもよい）、- how_to_verify:（"- verification:" が review なら持つ。ほかはあってもよい。人か LLM が確かめる手順の自由文）、- deferred:（`後回し`にするときだけ持つ。出典のコンマ区切り） | algorithm 以外は持つ。algorithm は持たなくてよく、持ってもよい |
| 決定表 | ## Decision tables の下 | ### TBL-nnn: 名前 | - source: と Markdown の表 | 持たなくてよく、持ってもよい（表を持つ） |
| 性質 | ## Properties の下 | ### PROP-nnn: 名前 | - source: | 持つ |
| シナリオ | ## Examples の下の gherkin のコードブロック | Scenario: の行 | 直前の行のタグ @id=EX-nnn、@about=ID,...、@source=出典,... | なし（ステップの行を持つ） |
| 問題の記録 | 問題の記録の文書の ## Flags の下か、節を挟まずに文書の直下（同じ文書に両方があってもよい） | ### FLAG-nnn: 名前 | - kind:（contradiction、gap、ambiguity）、- related:（ID のコンマ区切り）、- source: | 本文を持つ |
| 用語 | 用語集の文書 | Term、Meaning、Source の3列のヘッダを持つ最初の表の行（REQ-core-117） | なし | なし（意味の列を持つ） |

## Examples

```gherkin
@id=EX-core-290 @about=REQ-core-043 @source=docs/decision/records/2026-09-24-review8-gaps.md#A2
Scenario: 名前の無い見出しは形に合わない
  Given "### REQ-001:" の見出しの下に、必要な行と文を持つ`要求`がある
  When "kotowari check" を実行する
  Then その見出しの行に unknown_heading が出て、"REQ-001" は定義に数えない

@id=EX-core-291 @about=TBL-core-011 @source=docs/decision/records/2026-09-24-review8-gaps.md#A1
Scenario: algorithm の要求と決定表は文を持ってもよい
  Given 文を持つ algorithm の`要求`と、表の前に文を持つ`決定表`がある
  When "kotowari check" を実行する
  Then 形の`指摘`は出ない

@id=EX-core-292 @about=REQ-core-045 @source=docs/decision/records/2026-09-24-review8-gaps.md#A3
Scenario: 同じ行が2つあるときは1つ目の値を読む
  Given "- kind: algorithm" の後に "- kind: ubiquitous" を持ち、文も定義も無い`要求`がある
  When "kotowari check" を実行する
  Then duplicate_field と algorithm_without_definition が出て、missing_statement は出ない

@id=EX-core-008 @about=REQ-core-044 @source=docs/decision/records/records.md#A42
Scenario: 知らない行は誤りになる
  Given `要求`の見出しの下に "- 優先度: 高" の行がある
  When "kotowari check" を実行する
  Then unknown_field の誤りが1件出る

@id=EX-core-273 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A1,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13,docs/decision/records/2026-09-23-ir-english-tokens.md#A2,docs/decision/records/ir-form.md#検査の種類
Scenario: 行の直後に続く文は行の値に入らない
  Given 検証が "review" の`要求`で、"- how_to_verify: 見る" の次の行に空行を挟まずに`文`があり、"- verification: review" の次の行にも空行を挟まずに`文`がある
  When "kotowari check --format json" を実行する
  Then その`要求`に missing_statement も verification_invalid も requirement_without_test も出ない

@id=EX-core-274 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A2,docs/decision/records/2026-09-23-ir-engine-gaps.md#A5,docs/decision/records/2026-09-23-ir-engine-gaps.md#A13,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 字下げした行と引用と HTML の行の閉じないバッククォートは誤りになる
  Given `要求`の "- source:" の行の後に空行を挟んで字下げした行があり、その下に引用の行と HTML の行があり、3つの行にはどれも閉じないバッククォートがある
  When "kotowari check --format json" を実行する
  Then 3つの行のそれぞれに unclosed_backtick の誤りが1件ずつ出る

@id=EX-core-275 @about=REQ-core-178 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A26
Scenario: 表にならない縦棒の行は文として検査を受ける
  Given `要求`の下に、区切りの行を持たず閉じないバッククォートを含む "| a |" で始まる行がある
  When "kotowari check --format json" を実行する
  Then その行に unclosed_backtick の誤りが1件出て、unknown_line は出ない

@id=EX-core-276 @about=REQ-core-042,TBL-core-011 @source=docs/decision/records/2026-09-23-ir-engine-gaps.md#A4,docs/decision/records/2026-09-23-ir-engine-gaps.md#A15,docs/decision/records/2026-09-23-ir-english-tokens.md#A2
Scenario: 節の無い問題の記録の項目も読む
  Given "FLAGS.md" に "## Flags" の節が無く、`題名`の後に "### FLAG-001: 例" の`項目`と、その "- kind:"、"- related:"、"- source:" の行と本文がある
  And 別の "FLAGS.md" に、節の無い "### FLAG-002: 例" と、"## Flags" の下の "### FLAG-003: 例" がある
  When "kotowari check --format json" を実行する
  Then unknown_heading も unknown_field も unknown_line も出ない
  And "kotowari status" の問題の記録の項目の数は3である
```
