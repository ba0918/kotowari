# 基準のディレクトリ

相対パスの基準になるディレクトリの決め方と、基準からの相対で扱うものを扱う。

## 要求

### REQ-core-009: 基準のディレクトリの決め方

- 種類: algorithm
- 出典: docs/decision/records/records.md#A37
- 定義: TBL-core-003, PROP-core-001
- 検証: unit

### REQ-core-010: 基準からの相対パス

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A13, docs/decision/records/records.md#A37
- 検証: unit

kotowari は常に、`設定ファイル`の値のパス、`出典`のパス、出力の "path" を`基準のディレクトリ`からの相対パスとして扱う。

### REQ-core-110: パスの正規化

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A106, docs/decision/records/2026-09-16-ir-tree.md#A13
- 検証: unit

kotowari は常に、`設定ファイル`の値のパスと`出典`のパスを、比べる前と出力の前に、末尾の "/" と先頭の "./" を除き、途中の "/./" と連続する "/" を1つの "/" に畳み、"\\" を "/" に直して正規化し、出力の "path" を正規化した置き場と、置き場からの文書の相対パスを "/" でつないで作る。

## 決定表

### TBL-core-003: 基準のディレクトリを探す順

- 出典: docs/decision/records/records.md#A37, docs/decision/records/records.md#A124

| 順 | 条件 | 基準のディレクトリ |
|---|---|---|
| 1 | カレントディレクトリから上に向かって ".kotowari/" のディレクトリのあるディレクトリが見つかる（".kotowari" という名前のファイルは無視して上に進む） | 最初に見つかったディレクトリ |
| 2 | 1 で見つからない | カレントディレクトリ |

## 性質

### PROP-core-001: 設定のパスは基準を変えない

- 出典: docs/decision/records/records.md#A37

`基準のディレクトリ`は、"--config" に与えるパスによって変わらない。

## 具体例

```gherkin
@id=EX-core-002 @about=REQ-core-009 @source=docs/decision/records/records.md#A37
Scenario: 上のディレクトリの .kotowari を基準にする
  Given "/repo/.kotowari/" があり、"/repo/src/" に ".kotowari/" は無い
  When "/repo/src/" で "kotowari check" を実行する
  Then `基準のディレクトリ`は "/repo" である
```
