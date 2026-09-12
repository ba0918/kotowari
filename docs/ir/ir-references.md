# シナリオのタグと ID の参照

シナリオのタグの検査、ID の参照切れ、kotowari が見ない形の性質を扱う。

## 要求

### REQ-052: 知らないタグ

- 種類: event_driven
- 出典: brainstorm/records.md#A27, brainstorm/ir-form.md#検査の種類
- 検証: unit

`シナリオ`に "@id"、"@about"、"@source" 以外のタグがあるとき、kotowari は unknown_tag の`誤り`を出す。

### REQ-053: 無いタグ

- 種類: event_driven
- 出典: brainstorm/records.md#A42, brainstorm/ir-form.md#検査の種類
- 検証: unit

`シナリオ`に "@id" か "@about" のタグが無いとき、kotowari は無いタグの名前を detail にして missing_tag の`誤り`を出す。

### REQ-054: 参照切れ

- 種類: event_driven
- 出典: brainstorm/records.md#A21, brainstorm/records.md#A39, brainstorm/records.md#A52, brainstorm/records.md#A28, brainstorm/ir-form.md#検査の種類, brainstorm/records.md#A67
- 検証: unit

"- 定義:" の行、"@about" のタグ、`問題の記録`の "- 関係:" の行、`要求`と`性質`の`文`の中でバッククォートで囲んだ`ID`、`印`（`問い合わせのある言語`で`テスト`の外にあるものを除く）のいずれかが存在しない`ID`を指すとき、kotowari は unresolved_reference の`誤り`を出す。

### REQ-055: EARS の型を見ない

- 種類: prohibition
- 出典: brainstorm/records.md#A42
- 検証: review

kotowari は、`要求`の`文`が EARS の型に沿うかを検査してはならない。

### REQ-056: 矛盾の読みの数を見ない

- 種類: prohibition
- 出典: brainstorm/records.md#A28
- 検証: review

kotowari は、`問題の記録`の矛盾の読みが2つ以上あるかを検査してはならない。

## 具体例

```gherkin
@id=EX-009 @about=REQ-052 @source=brainstorm/records.md#A27,brainstorm/ir-form.md#検査の種類
Scenario: やめたタグは誤りになる
  Given `シナリオ`に "@requirement=REQ-001" のタグがある
  When "kotowari check" を実行する
  Then unknown_tag の誤りが1件出る

@id=EX-010 @about=REQ-054 @source=brainstorm/records.md#A52,brainstorm/records.md#A39,brainstorm/ir-form.md#検査の種類
Scenario: 無い要求を指す about は参照切れになる
  Given `シナリオ`の "@about" が "REQ-999" を指し、"REQ-999" はどこにも無い
  When "kotowari check" を実行する
  Then detail が "REQ-999" の unresolved_reference の誤りが出る
```
