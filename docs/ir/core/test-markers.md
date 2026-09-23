# テストの印

テストに書く印の構文と、印をテストに結び付ける規則を扱う。

## Requirements

### REQ-core-071: 印の構文

- kind: algorithm
- source: docs/decision/records/records.md#A14, docs/decision/records/records.md#A57
- definition: TBL-core-015
- verification: unit

### REQ-core-072: 形の誤った印

- kind: event_driven
- source: docs/decision/records/records.md#A57, docs/decision/records/records.md#A67, docs/decision/records/records.md#A39, docs/decision/records/records.md#A121, docs/decision/records/records.md#A111
- verification: unit

`問い合わせのある言語`で`テスト`の外にあるものを除く`印`について、その中が空か区切りだけのとき、またはその`印`に同じ行の閉じ括弧が無いとき、kotowari は行の文字を detail にして invalid_marker の`誤り`を出す。

### REQ-core-073: 1行に複数の印

- kind: ubiquitous
- source: docs/decision/records/records.md#A57
- verification: unit

kotowari は常に、1行の中の`印`をすべて拾う。

### REQ-core-074: コメント記号を見ない

- kind: ubiquitous
- source: docs/decision/records/records.md#A14
- verification: unit

kotowari は常に、`印`を行のどの位置からも拾い、コメント記号を見ない。

### REQ-core-075: 印の結び付け

- kind: algorithm
- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A34, docs/decision/records/records.md#A39, docs/decision/records/records.md#A57
- definition: TBL-core-016
- verification: unit

### REQ-core-076: 問い合わせの無い言語の印

- kind: event_driven
- source: docs/decision/records/records.md#A39, docs/decision/records/records.md#A57
- verification: unit

`問い合わせの無い言語`の`テストのファイル`を読むとき、kotowari はコメントかどうかを問わず、ファイルの文字の中の`印`をすべて拾う。

### REQ-core-077: 存在しない ID だけを指す印

- kind: event_driven
- source: docs/decision/records/records.md#A57, docs/decision/records/records.md#A89
- verification: unit

`印`が存在しない`ID`だけを指すとき、kotowari はその`印`の結び付いた`テスト`を`印`のあるものと数える。`印`の角括弧の中の`ID`の形でない要素（"REQ001" のように区切りの無いもの）は、存在しない`ID`を指したものとして unresolved_reference の`誤り`を出す。

### REQ-core-078: review の要求を指す印

- kind: event_driven
- source: docs/decision/records/records.md#A39
- verification: unit

`印`が検証の値 "review" の`要求`を指すとき、kotowari はそれを`誤り`にしない。

### REQ-core-118: 印の指摘の行

- kind: ubiquitous
- source: docs/decision/records/records.md#A121
- verification: unit

kotowari は常に、`印`から出す unresolved_reference と invalid_marker の "line" を`印`のある行（行をまたぐ`印`なら "@kotowari[" のある行）にする。

## Decision tables

### TBL-core-015: 印の構文

- source: docs/decision/records/records.md#A14, docs/decision/records/records.md#A57

| 部分 | 形 |
|---|---|
| 始まり | @kotowari[ |
| 中身 | ID をコンマで区切って並べる。コンマの前後に空白を置いてよい |
| 終わり | ] |

### TBL-core-016: 印の結び付け（問い合わせのある言語）

- source: docs/decision/records/records.md#A26, docs/decision/records/records.md#A34, docs/decision/records/records.md#A39, docs/decision/records/records.md#A57, docs/decision/records/records.md#A67, docs/decision/records/records.md#A121

| 印の位置 | 扱い |
|---|---|
| 関数とその属性の直前に続くコメントの塊（属性を挟んでよく、空行を挟んだら切れる） | その関数のテストに結び付ける |
| 関数の本体の先頭で、どの文よりも前にあるコメントの塊 | その関数のテストに結び付ける |
| 上の2つの両方 | 両方の ID を合わせて結び付ける |
| 関数の本体の途中 | 無視する |
| テストの外 | 無視し、invalid_marker も unresolved_reference も出さない |
| マクロの中の関数 | 上と同じ規則を適用する |

## Examples

```gherkin
@id=EX-core-015 @about=REQ-core-073 @source=docs/decision/records/records.md#A57,docs/decision/records/records.md#A26,docs/decision/records/records.md#A39
Scenario: 1行の2つの印を両方拾う
  Given `テスト`の直前のコメントに "@kotowari[REQ-001] @kotowari[TBL-002]" がある
  When "kotowari check" を実行する
  Then その`テスト`は "REQ-001" と "TBL-002" に結び付く

@id=EX-core-016 @about=REQ-core-075 @source=docs/decision/records/records.md#A39,docs/decision/records/records.md#A47,docs/decision/records/ir-form.md#検査の種類,docs/decision/records/records.md#A26,docs/decision/records/records.md#A49
Scenario: 空行を挟んだコメントの印は結び付かない
  Given "@kotowari[REQ-001]" のコメントと "#[test]" の関数の間に空行がある
  When "kotowari check" を実行する
  Then その関数に test_without_id の誤りが出る
```
