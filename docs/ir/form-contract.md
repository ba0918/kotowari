# IR の形の固定

IR の形をどこで決めるかと、採らない形の決め方を扱う。

## 要求

### REQ-089: 形はコードに固定する

- 種類: ubiquitous
- 出典: experiments/003-cli/brainstorm/records.md#A25, experiments/003-cli/brainstorm/records.md#A52
- 検証: review

kotowari は常に、コードに固定した1つの形で`IR`を読む。

### REQ-090: スキーマのファイルを読まない

- 種類: prohibition
- 出典: experiments/003-cli/brainstorm/records.md#R4
- 検証: review

kotowari は、`IR`の形を宣言したスキーマのファイルを読んではならない。

### REQ-091: 外部の mdschema を使わない

- 種類: prohibition
- 出典: experiments/003-cli/brainstorm/records.md#R5
- 検証: review

kotowari は、外部の mdschema を検査の前段に使ってはならない。
