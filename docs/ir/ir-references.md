# シナリオのタグと ID の参照

シナリオのタグの検査、ID の参照切れ、kotowari が見ない形の性質を扱う。

## 要求

### REQ-052: 知らないタグ

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A27, docs/decision/brainstorm/ir-form.md#検査の種類, docs/decision/brainstorm/records.md#A109, docs/decision/brainstorm/records.md#A143
- 検証: unit

gherkin のブロックの中のタグの行（`シナリオ`に結び付くかを問わない）に "@id"、"@about"、"@source" 以外のタグがあるとき、またはタグの行に "@" で始まらない語があるとき、kotowari はその名前か語を detail にして unknown_tag の`誤り`を出す。

### REQ-053: 無いタグ

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A42, docs/decision/brainstorm/ir-form.md#検査の種類, docs/decision/brainstorm/records.md#A97
- 検証: unit

`シナリオ`に "@id" か "@about" のタグが無いとき、kotowari は無いタグの名前を detail にして missing_tag の`誤り`を出す。値が空のタグ（"=" の後に何も無い）は、無いタグとして扱う。

### REQ-054: 参照切れ

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A21, docs/decision/brainstorm/records.md#A39, docs/decision/brainstorm/records.md#A52, docs/decision/brainstorm/records.md#A28, docs/decision/brainstorm/ir-form.md#検査の種類, docs/decision/brainstorm/records.md#A67, docs/decision/brainstorm/records.md#A119, docs/decision/brainstorm/records.md#A130, docs/decision/brainstorm/records.md#A145, docs/decision/brainstorm/records.md#A152
- 検証: unit

"- 定義:" の行、"@about" のタグ、`問題の記録`の "- 関係:" の行、`要求`と`性質`の`文`と gherkin のステップの行の中で、二重引用符の外でバッククォートで囲んだ`ID`、`印`（`問い合わせのある言語`で`テスト`の外にあるものを除く）のいずれかが存在しない`ID`を指すとき、または "- 定義:"、"@about"、"- 関係:" の値が`ID`の形でないとき、kotowari は出現ごとに1件の unresolved_reference の`誤り`を出す。

### REQ-055: EARS の型を見ない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#A42
- 検証: review

kotowari は、`要求`の`文`が EARS の型に沿うかを検査してはならない。

### REQ-056: 矛盾の読みの数を見ない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#A28
- 検証: review

kotowari は、`問題の記録`の矛盾の読みが2つ以上あるかを検査してはならない。

### REQ-113: gherkin のブロックの中の行

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A109, docs/decision/brainstorm/records.md#A132, docs/decision/brainstorm/records.md#A133, docs/decision/brainstorm/records.md#A155
- 検証: unit

gherkin の`コードブロック`の中の行を行頭の空白を除いて見て、タグの行（"@" で始まる）、"Scenario:" の行、ステップの行（"Given"、"When"、"Then"、"And"、"But" に半角空白1つ以上が続く行）、"#" で始まる注釈、空行のいずれでもない行（"Feature:"、"Background:"、"Scenario Outline:"、"Examples:"、データ表の行を含む）があるとき、kotowari は行の文字を detail にして invalid_gherkin_line の`誤り`を出す。直前に "Scenario:" もステップの行も無いステップの行、および直後が "Scenario:" でないタグの行も、同じ`誤り`を出す。タグの行は "Scenario:" の直前の行だけを結び付け、間にほかの行があれば結び付けない。

### REQ-114: ID の定義と形に合わない @id

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A110, docs/decision/brainstorm/records.md#A139, docs/decision/brainstorm/records.md#A151
- 検証: unit

"@id" の値が EX の`ID`の形でないとき、kotowari は値を detail、タグの行を "line" にして invalid_id の`誤り`を出し、"@id" の missing_tag は出さず（"@about" が無いときの missing_tag は出す）、その`シナリオ`の missing_source の detail は "Scenario:" の行の文字にする。存在する`ID`の集合には、形に合う見出しの`ID`と形に合う "@id" の値だけを数える。

## 具体例

```gherkin
@id=EX-009 @about=REQ-052 @source=docs/decision/brainstorm/records.md#A27,docs/decision/brainstorm/ir-form.md#検査の種類
Scenario: やめたタグは誤りになる
  Given `シナリオ`に "@requirement=REQ-001" のタグがある
  When "kotowari check" を実行する
  Then unknown_tag の誤りが1件出る

@id=EX-010 @about=REQ-054 @source=docs/decision/brainstorm/records.md#A52,docs/decision/brainstorm/records.md#A39,docs/decision/brainstorm/ir-form.md#検査の種類
Scenario: 無い要求を指す about は参照切れになる
  Given `シナリオ`の "@about" が "REQ-999" を指し、"REQ-999" はどこにも無い
  When "kotowari check" を実行する
  Then detail が "REQ-999" の unresolved_reference の誤りが出る
```
