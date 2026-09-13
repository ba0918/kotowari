# 出力の形

"--format" で選ぶ出力の形を扱う。

## 要求

### REQ-021: 出力の形の値

- 種類: ubiquitous
- 出典: experiments/003-cli/brainstorm/records.md#A7, experiments/003-cli/brainstorm/records.md#A18
- 検証: unit

kotowari は常に、"--format" の値として "json" と "text" の2つを受け、既定を "json" にする。

### REQ-022: JSON を1つ出す

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A40, experiments/003-cli/brainstorm/ir-form.md#出力
- 検証: unit

"--format" が "json" のとき、kotowari は標準出力に1つの JSON を出す。

### REQ-023: JSON の中身

- 種類: algorithm
- 出典: experiments/003-cli/brainstorm/records.md#A40, experiments/003-cli/brainstorm/records.md#A56
- 定義: TBL-005, TBL-006, PROP-002
- 検証: unit

### REQ-025: 文字の出力

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A40, experiments/003-cli/brainstorm/records.md#A50, experiments/003-cli/brainstorm/records.md#A18, experiments/003-cli/brainstorm/ir-form.md#出力
- 検証: unit

"--format" が "text" のとき、kotowari は1つの`指摘`を1行で "パス:行 [error] 種類 詳細" か "パス:行 [warning] 種類 詳細" の形で出し、角括弧も出す。

### REQ-026: 行の無い指摘の文字の出力

- 種類: event_driven
- 出典: experiments/003-cli/brainstorm/records.md#A47
- 検証: unit

"--format" が "text" で`指摘`の "line" が null のとき、kotowari は行を "-" と書く。

## 決定表

### TBL-005: JSON の最上位

- 出典: experiments/003-cli/brainstorm/records.md#A40, experiments/003-cli/brainstorm/records.md#A56, experiments/003-cli/brainstorm/ir-form.md#出力

| 鍵 | 中身 |
|---|---|
| files | 読んだ IR の文書の数（用語集と問題の記録を含む） |
| lines | IR の文書の行数の合計（用語集と問題の記録を含む） |
| findings | 指摘の一覧 |
| counts | 種類ごとの指摘の数 |

### TBL-006: 指摘の鍵

- 出典: experiments/003-cli/brainstorm/records.md#A40, experiments/003-cli/brainstorm/records.md#A61, experiments/003-cli/brainstorm/records.md#A106

| 鍵 | 中身 |
|---|---|
| kind | 指摘の種類（TBL-008、TBL-009） |
| severity | error か warning |
| path | 基準のディレクトリからの相対パス。正規化した置き場と文書名を "/" でつなぐ（REQ-110） |
| line | 行（1始まり）。文書全体への指摘は null |
| detail | 種類ごとに TBL-008、TBL-009 で決めた文字列 |

## 性質

### PROP-002: counts と findings の一致

- 出典: experiments/003-cli/brainstorm/records.md#A40, experiments/003-cli/brainstorm/ir-form.md#出力

"counts" の各種類の値は "findings" の中のその種類の`指摘`の数に等しく、"findings" に1件も無い種類は "counts" に無い。

## 具体例

```gherkin
@id=EX-004 @about=REQ-025,REQ-026 @source=experiments/003-cli/brainstorm/records.md#A47,experiments/003-cli/brainstorm/records.md#A40,experiments/003-cli/brainstorm/records.md#A50,experiments/003-cli/brainstorm/ir-form.md#検査の種類
Scenario: 題名の無い文書を文字で出す
  Given "docs/ir/a.md" に題名が無い
  When "kotowari check --format text" を実行する
  Then "docs/ir/a.md:- [error] missing_title a.md" の行が出る
```
