# IR の形の固定

IR の形をどこで決めるかと、採らない形の決め方を扱う。

## 要求

### REQ-core-089: 形はコードに固定する

- 種類: ubiquitous
- 出典: docs/decision/records/records.md#A25, docs/decision/records/records.md#A52, docs/decision/records/records.md#A98
- 検証: review
- 確かめ方: `crates/kotowari-core/src/ir.rs` にコードで固定した形で IR を読むことを確認。parse_document 関数

kotowari は常に、コードに固定した1つの形で`IR`を読む。

### REQ-core-090: スキーマのファイルを読まない

- 種類: prohibition
- 出典: docs/decision/records/records.md#R4, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4
- 検証: unit

kotowari は、`IR`の形を宣言したスキーマのファイルを読んではならない。

### REQ-core-091: 外部の mdschema を使わない

- 種類: prohibition
- 出典: docs/decision/records/records.md#R5, docs/decision/records/2026-09-17-check-reach.md#A3, docs/decision/records/2026-09-17-check-reach.md#A4, docs/decision/records/2026-09-17-check-reach.md#A18
- 検証: unit

kotowari は、外部の mdschema を検査の前段に使ってはならない。

## 具体例

```gherkin
@id=EX-core-040 @about=REQ-core-090 @source=docs/decision/records/2026-09-17-check-reach.md#A4,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: スキーマのファイルを置いても読まれない
  Given ".kotowari/schema.yaml" に壊れた YAML がある
  When "kotowari check" を実行する
  Then `停止`せず、置く前と同じ終了コードと標準出力になる

@id=EX-core-041 @about=REQ-core-091 @source=docs/decision/records/2026-09-17-check-reach.md#A18,docs/decision/records/2026-09-17-check-reach.md#A28
Scenario: 外部のコマンドが見つからなくても結果は変わらない
  Given 環境変数 "PATH" が空のディレクトリだけを指す
  When "kotowari check" を実行する
  Then "PATH" をそのままにしたときと同じ終了コード、標準出力、標準エラーになる
```
