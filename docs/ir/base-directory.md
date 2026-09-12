# 基準のディレクトリ

相対パスの基準になるディレクトリの決め方と、基準からの相対で扱うものを扱う。

## 要求

### REQ-009: 基準のディレクトリの決め方

- 種類: algorithm
- 出典: brainstorm/records.md#A37
- 定義: TBL-003, PROP-001
- 検証: unit

### REQ-010: 基準からの相対パス

- 種類: ubiquitous
- 出典: brainstorm/records.md#A13, brainstorm/records.md#A37
- 検証: unit

kotowari は常に、`設定ファイル`の値のパス、`出典`のパス、出力の "path" を`基準のディレクトリ`からの相対パスとして扱う。

## 決定表

### TBL-003: 基準のディレクトリを探す順

- 出典: brainstorm/records.md#A37

| 順 | 条件 | 基準のディレクトリ |
|---|---|---|
| 1 | カレントディレクトリから上に向かって ".kotowari/" のあるディレクトリが見つかる | 最初に見つかったディレクトリ |
| 2 | 1 で見つからない | カレントディレクトリ |

## 性質

### PROP-001: 設定のパスは基準を変えない

- 出典: brainstorm/records.md#A37

`基準のディレクトリ`は、"--config" に与えるパスによって変わらない。

## 具体例

```gherkin
@id=EX-002 @about=REQ-009 @source=brainstorm/records.md#A37
Scenario: 上のディレクトリの .kotowari を基準にする
  Given "/repo/.kotowari/" があり、"/repo/src/" に ".kotowari/" は無い
  When "/repo/src/" で "kotowari check" を実行する
  Then `基準のディレクトリ`は "/repo" である
```
