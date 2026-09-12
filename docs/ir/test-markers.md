# テストの印

テストに書く印の構文と、印をテストに結び付ける規則を扱う。

## 要求

### REQ-071: 印の構文

- 種類: algorithm
- 出典: brainstorm/records.md#A14, brainstorm/records.md#A57
- 定義: TBL-015
- 検証: unit

### REQ-072: 形の誤った印

- 種類: event_driven
- 出典: brainstorm/records.md#A57, brainstorm/records.md#A67, brainstorm/records.md#A39
- 検証: unit

`問い合わせのある言語`で`テスト`の外にあるものを除く`印`について、その中が空のとき、またはその`印`に閉じ括弧が無いとき、kotowari は invalid_marker の`誤り`を出す。

### REQ-073: 1行に複数の印

- 種類: ubiquitous
- 出典: brainstorm/records.md#A57
- 検証: unit

kotowari は常に、1行の中の`印`をすべて拾う。

### REQ-074: コメント記号を見ない

- 種類: ubiquitous
- 出典: brainstorm/records.md#A14
- 検証: unit

kotowari は常に、`印`を行のどの位置からも拾い、コメント記号を見ない。

### REQ-075: 印の結び付け

- 種類: algorithm
- 出典: brainstorm/records.md#A26, brainstorm/records.md#A34, brainstorm/records.md#A39, brainstorm/records.md#A57
- 定義: TBL-016
- 検証: unit

### REQ-076: 問い合わせの無い言語の印

- 種類: event_driven
- 出典: brainstorm/records.md#A39, brainstorm/records.md#A57
- 検証: unit

`問い合わせの無い言語`の`テストのファイル`を読むとき、kotowari はコメントかどうかを問わず、ファイルの文字の中の`印`をすべて拾う。

### REQ-077: 存在しない ID だけを指す印

- 種類: event_driven
- 出典: brainstorm/records.md#A57
- 検証: unit

`印`が存在しない`ID`だけを指すとき、kotowari はその`印`の結び付いた`テスト`を`印`のあるものと数える。

### REQ-078: review の要求を指す印

- 種類: event_driven
- 出典: brainstorm/records.md#A39
- 検証: unit

`印`が検証の値 "review" の`要求`を指すとき、kotowari はそれを`誤り`にしない。

## 決定表

### TBL-015: 印の構文

- 出典: brainstorm/records.md#A14, brainstorm/records.md#A57

| 部分 | 形 |
|---|---|
| 始まり | @kotowari[ |
| 中身 | ID をコンマで区切って並べる。コンマの前後に空白を置いてよい |
| 終わり | ] |

### TBL-016: 印の結び付け（問い合わせのある言語）

- 出典: brainstorm/records.md#A26, brainstorm/records.md#A34, brainstorm/records.md#A39, brainstorm/records.md#A57, brainstorm/records.md#A67

| 印の位置 | 扱い |
|---|---|
| 関数とその属性の直前に続くコメントの塊（属性を挟んでよく、空行を挟んだら切れる） | その関数のテストに結び付ける |
| 関数の本体の先頭で、どの文よりも前にあるコメントの塊 | その関数のテストに結び付ける |
| 上の2つの両方 | 両方の ID を合わせて結び付ける |
| 関数の本体の途中 | 無視する |
| テストの外 | 無視し、invalid_marker も unresolved_reference も出さない |

## 具体例

```gherkin
@id=EX-015 @about=REQ-073 @source=brainstorm/records.md#A57,brainstorm/records.md#A26,brainstorm/records.md#A39
Scenario: 1行の2つの印を両方拾う
  Given `テスト`の直前のコメントに "@kotowari[REQ-001] @kotowari[TBL-002]" がある
  When "kotowari check" を実行する
  Then その`テスト`は "REQ-001" と "TBL-002" に結び付く

@id=EX-016 @about=REQ-075 @source=brainstorm/records.md#A39,brainstorm/records.md#A47,brainstorm/ir-form.md#検査の種類,brainstorm/records.md#A26,brainstorm/records.md#A49
Scenario: 空行を挟んだコメントの印は結び付かない
  Given "@kotowari[REQ-001]" のコメントと "#[test]" の関数の間に空行がある
  When "kotowari check" を実行する
  Then その関数に test_without_id の誤りが出る
```
