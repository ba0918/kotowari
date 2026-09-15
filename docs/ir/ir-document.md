# 文書の読み方と文書全体の検査

IR の文書の選び方、題名と範囲の行、行の数え方、行数と要求の数の警告を扱う。

## 要求

### REQ-033: 読む文書

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A32, docs/decision/brainstorm/records.md#A102, docs/decision/brainstorm/records.md#A165
- 検証: unit

kotowari は常に、`IR`の置き場の直下の、拡張子が小文字の ".md" のファイルだけを読み、サブディレクトリの文書と ".MD" の文書、ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）を読まない（`除外`）。ファイルのシンボリックリンクは読む。種類を取れない要素があるときは読めないファイルを理由に`停止`する。

### REQ-034: 題名が無い

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A42, docs/decision/brainstorm/records.md#A56
- 検証: unit

`IR`の文書に`題名`が無いとき、kotowari は missing_title の`誤り`を出す。

### REQ-035: 題名が複数

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A42
- 検証: unit

`IR`の文書に`題名`が2つ以上あるとき、kotowari は multiple_titles の`誤り`を出す。

### REQ-036: 範囲の行が無い

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A30, docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A55, docs/decision/brainstorm/records.md#A56, docs/decision/brainstorm/ir-form.md#検査の種類
- 検証: unit

`話題ごとの文書`に`文書が扱う範囲`の行が1行も無いとき、kotowari は missing_scope の`誤り`を出す。

### REQ-037: 行の数え方

- 種類: algorithm
- 出典: docs/decision/brainstorm/records.md#A33
- 定義: TBL-010
- 検証: unit

### REQ-038: 行数の上限

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A17, docs/decision/brainstorm/records.md#A29, docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A56, docs/decision/brainstorm/records.md#A47, docs/decision/brainstorm/ir-form.md#検査の種類
- 検証: unit

`IR`の文書の行数が "limits.lines" を超えるとき、kotowari は too_many_lines の`警告`を出す。

### REQ-039: 要求の数の上限

- 種類: event_driven
- 出典: docs/decision/brainstorm/records.md#A17, docs/decision/brainstorm/records.md#A29, docs/decision/brainstorm/records.md#A41, docs/decision/brainstorm/records.md#A47, docs/decision/brainstorm/ir-form.md#検査の種類
- 検証: unit

`話題ごとの文書`の`要求`の数が "limits.requirements" を超えるとき、kotowari は too_many_requirements の`警告`を出す。

### REQ-040: コードブロックの中

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A52, docs/decision/brainstorm/records.md#A88, docs/decision/brainstorm/records.md#A108, docs/decision/brainstorm/records.md#A140
- 検証: unit

kotowari は常に、`コードブロック`の中を検査の対象から外す（`除外`）。閉じていない`コードブロック`は gherkin でも対象から外す。閉じた gherkin のブロックの中の行は、`シナリオ`のタグと`用語`と曖昧語の検査の対象にし、文書名の参照の検査では対象にしない。

### REQ-041: 範囲の中身と責務の分離を見ない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#A17, docs/decision/brainstorm/records.md#A30
- 検証: review

kotowari は、`文書が扱う範囲`の中身と行数を検査すること、文書の責務の分離を判定することをしてはならない。

## 決定表

### TBL-010: 行の数え方

- 出典: docs/decision/brainstorm/records.md#A33, docs/decision/brainstorm/records.md#A129

| 場面 | 数え方 |
|---|---|
| "\n" | 1つの行の終わり |
| "\r\n" | 1つの行の終わり（1行に数える） |
| 最後の行に改行が無い | その行も1行に数える |
| 中身が空の文書 | 0行に数える（題名が無いので missing_title を出す） |

## 具体例

```gherkin
@id=EX-006 @about=REQ-036 @source=docs/decision/brainstorm/records.md#A41,docs/decision/brainstorm/ir-form.md#検査の種類
Scenario: 用語集は範囲の行が無くてもよい
  Given `用語集`に`題名`と表だけがある
  When "kotowari check" を実行する
  Then `用語集`に missing_scope の誤りは出ない

@id=EX-007 @about=REQ-037 @source=docs/decision/brainstorm/records.md#A33
Scenario: 改行の違いで行数は変わらない
  Given "a\r\nb" と書いた文書がある
  When その文書の行数を数える
  Then 行数は 2 である
```
