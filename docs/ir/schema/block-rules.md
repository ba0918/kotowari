---
$schema: ../../../.mds/schemas/ir.yaml
---
# 見出しの下の行の規則

この文書は、見出しや前置部の下に並ぶ行を、フィールド行、文、箇条書き、表、コードブロックのどれとして読むかを扱う。

## 要求

### REQ-schema-028: 一覧の行の読み分け

- 種類: algorithm
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10
- 定義: TBL-schema-007
- 検証: property

### REQ-schema-029: フィールド行の値の制約

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A29
- 検証: unit

mds は常に、`フィールド行`の値に正規表現と許可リストを課し、区切り文字を宣言したときは区切った要素ごとに課す。

### REQ-schema-030: 継続段落はその行の一部

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A11
- 検証: unit

mds は常に、`継続段落`を直前の一覧の行の一部として読み、`文`には数えず、`閉じた世界`でも`指摘`にしない。

### REQ-schema-031: 箇条書きの入れ子

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A2, docs/decision/records/2026-09-21-mds-spec.md#A10
- 検証: unit

`箇条書き`に子の一覧があるとき、mds は`スキーマ`が宣言した子の規則に照らし、宣言が無ければ子の行を`指摘`にする。

### REQ-schema-032: 文の数え方

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A30
- 検証: unit

mds は常に、空行で区切った段落を1つの`文`として数え、引用、水平線、画像だけの行は`文`に数えず、`閉じた世界`でも`指摘`にしない。

### REQ-schema-033: 表の検査

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A13
- 検証: unit

mds は常に、`表`のヘッダのセル列を宣言したときだけヘッダと列数を照合し、宣言しないときは`表`の有無と`出現回数`だけを見る。

### REQ-schema-034: コードブロックの検査

- 種類: ubiquitous
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A12, docs/decision/records/2026-09-21-mds-spec.md#A35
- 検証: unit

mds は常に、`コードブロック`の言語を宣言したときだけ言語を照合し、行ごとの正規表現を宣言したときは、行頭の空白を除いた空でない行だけを照合する。

### REQ-schema-041: フィールド行の並び順

- 種類: event_driven
- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A36
- 検証: unit

`フィールド行`の一覧に並び順の強制を宣言したとき、mds は`スキーマ`に書いた順で現れない`フィールド行`を`指摘`にする。宣言しないときの並びは順不同である。

## 決定表

### TBL-schema-007: 一覧の行の読み分け

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10, docs/decision/records/2026-09-21-mds-spec.md#A33

| 順 | 行の形 | 読み方 |
|---|---|---|
| 1 | マーカーに「名前と値」が続き、名前が`スキーマ`の宣言と一致する | `フィールド行` |
| 2 | マーカーに続くが、1 に当たらない | `箇条書き` |
| 3 | 数字と区切りの点で始まる | どの規則種別にも属さず、`閉じた世界`では`指摘` |

## 性質

### PROP-schema-006: マーカーの種類は読み分けを変えない

- 出典: docs/decision/records/2026-09-21-mds-spec.md#A10

一覧のマーカーが "-"、"*"、"+" のどれであっても、同じ行は同じ`規則種別`として読まれる。

## 具体例

```gherkin
@id=EX-schema-011 @about=REQ-schema-028 @source=docs/decision/records/2026-09-21-mds-spec.md#A10
Scenario: 宣言していない名前の行は箇条書きとして読む
  Given `フィールド行`の名前を宣言した`スキーマ`がある
  When 宣言していない名前の「名前と値」の行を持つ`文書`を検査する
  Then その行は`箇条書き`として読まれる

@id=EX-schema-012 @about=REQ-schema-033 @source=docs/decision/records/2026-09-21-mds-spec.md#A13
Scenario: ヘッダを宣言しない表はどのヘッダでも通る
  Given ヘッダのセル列を宣言しない`表`の規則を持つ`スキーマ`がある
  When 任意のヘッダを持つ`表`の`文書`で "mds check" を実行する
  Then ヘッダの`指摘`は出ない
```
