# IR の形の固定

IR の形をどこで決めるかと、採らない形の決め方を扱う。

## 要求

### REQ-089: 形はコードに固定する

- 種類: ubiquitous
- 出典: docs/decision/brainstorm/records.md#A25, docs/decision/brainstorm/records.md#A52, docs/decision/brainstorm/records.md#A98
- 検証: review

kotowari は常に、コードに固定した1つの形で`IR`を読む。

### REQ-090: スキーマのファイルを読まない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#R4, docs/decision/brainstorm/2026-09-17-check-reach.md#A3, docs/decision/brainstorm/2026-09-17-check-reach.md#A4
- 検証: unit

kotowari は、`IR`の形を宣言したスキーマのファイルを読んではならない。

### REQ-091: 外部の mdschema を使わない

- 種類: prohibition
- 出典: docs/decision/brainstorm/records.md#R5, docs/decision/brainstorm/2026-09-17-check-reach.md#A3, docs/decision/brainstorm/2026-09-17-check-reach.md#A4
- 検証: unit

kotowari は、外部の mdschema を検査の前段に使ってはならない。
