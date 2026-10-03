# ライブラリの検証と値抽出

再利用するスキーマと文書の型、検証済みの値と部分抽出の区別、CLIの抽出との関係を扱う。

## Requirements

### REQ-schema-068: 検証を迂回しないスキーマ

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A21, docs/decision/records/2026-10-03-public-crate-api.md#A42
- verification: unit

"Schema::parse" は意味検証を通した不変の "Schema" を返す。外からのフィールド変更や未検証の直接Deserializeで検証を迂回できない。解析済みの "Schema" と "Document" を検証・抽出で再利用できる。

### REQ-schema-069: 検証済みと部分抽出

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A20, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: unit

"extract_validated" は検証し、文書に違反がなければ "ValidatedValues" を返し、違反があればその指摘を返して検証済み値を返さない。"extract_partial" は指摘と取得可能なJSON値を持つ "PartialExtraction" を返す。値は "serde_json::Value" で扱う。

### REQ-schema-070: スキーマのopenを尊重する

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A42
- verification: unit

"ValidationOptions" の既定はスキーマの "open" を尊重する。利用者が緩和を指定した場合、スキーマの値との論理和を検証に使う。

### REQ-schema-071: CLI抽出の意味を維持する

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A9, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: unit

CLIの "values" と "ast --schema" は部分抽出の値を使い、文書の違反による既存の終了コード・出力を変えない。"ast_json" と "extract_typed_partial" は既存の素のAST JSONと型付き抽出JSONを提供する。

## Examples

```gherkin
@id=EX-schema-085 @about=REQ-schema-068,REQ-schema-069 @source=docs/decision/records/2026-10-03-public-crate-api.md#A21,docs/decision/records/2026-10-03-public-crate-api.md#A42,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: 解析済みの値を繰り返し検証して抽出する
  Given 意味検証を通ったSchemaと適合するDocumentがある
  When 同じ解析済みの値で検証とextract_validatedを呼ぶ
  Then ValidatedValuesを返し再解析を要求しない

@id=EX-schema-086 @about=REQ-schema-068 @source=docs/decision/records/2026-10-03-public-crate-api.md#A42
Scenario: 不正なスキーマを検証済みの型にしない
  Given 意味検証に違反するスキーマのYAMLがある
  When Schema::parseを呼ぶ
  Then 失敗を返しSchemaを返さない

@id=EX-schema-087 @about=REQ-schema-069 @source=docs/decision/records/2026-10-03-public-crate-api.md#A20,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: 部分的に抽出できても検証済みとは扱わない
  Given 文書に違反と抽出可能な値がある
  When extract_validatedとextract_partialを呼ぶ
  Then 前者は指摘を返してValidatedValuesを返さない
  And 後者は指摘と取得可能な値を返す

@id=EX-schema-088 @about=REQ-schema-070 @source=docs/decision/records/2026-10-03-public-crate-api.md#A42
Scenario: 既定のオプションでスキーマの緩和を打ち消さない
  Given スキーマのopenが真である
  When 既定のValidationOptionsで検証する
  Then openが真として検証する

@id=EX-schema-089 @about=REQ-schema-071 @source=docs/decision/records/2026-10-03-public-crate-api.md#A9,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: 文書の違反で既存のvaluesコマンドを止めない
  Given 読み込める文書に検証上の違反と抽出可能な値がある
  When valuesコマンドを実行する
  Then 既存の契約どおり値を出力し文書の違反だけで終了コードを変更しない
```
