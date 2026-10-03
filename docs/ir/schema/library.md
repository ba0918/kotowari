# ライブラリの入口

純粋な "kotowari-markdown-schema" の公開入口と読み書きの境界を扱う。検証と抽出の結果は library-extraction.md、ファイル・HTTP操作は library-io.md で扱う。

## Requirements

### REQ-schema-049: ライブラリの入口

- kind: algorithm
- source: docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A43
- definition: TBL-schema-010
- verification: unit

### REQ-schema-050: 読み書きは呼び出し側の責務

- kind: prohibition
- source: docs/decision/records/2026-09-21-mds-spec.md#A18, docs/decision/records/2026-09-21-mds-spec.md#A57, docs/decision/records/2026-10-03-public-crate-api.md#A11, docs/decision/records/2026-10-03-public-crate-api.md#A17
- verification: unit

"kotowari-markdown-schema" は、`文書`と`スキーマ`のファイルを読まず、URL の`スキーマ`も取得しない。`スキーマ`の位置を決めるところまでを行い、読み書きは呼び出し側または別のI/Oクレートに残す。

### REQ-schema-051: 公開契約と内部実装

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A13, docs/decision/records/2026-10-03-public-crate-api.md#A16, docs/decision/records/2026-10-03-public-crate-api.md#A40, docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: review
- how_to_verify: rustdocの公開入口と入出力型を確認し、未検証の構築経路と依存先の内部型が公開されていないことを確認する。JSON値の公開は既存の抽出契約として許す。

"kotowari-markdown-schema" は TBL-schema-010 の入口とその入出力の公開型を互換性管理の対象とし、内部実装を非公開にする。"markdown::mdast::Node" と "regex::Captures" を公開署名に含めない。抽出値とASTのJSONは "serde_json::Value" で扱う。

## Decision tables

### TBL-schema-010: ライブラリの入口

- source: docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A43, docs/decision/records/2026-10-03-public-crate-api.md#A56

| 入口 | 何をするか | 返すもの |
|---|---|---|
| frontmatter_schema | `文書`の文字列から "$schema" の参照を読む | 参照、または無し。`frontmatter`が壊れていれば誤り |
| resolve_schema | `文書`の位置と参照から`スキーマ`の位置を決める | ファイルのパス、または URL |
| Schema::parse | `スキーマ`の YAML を解析し意味検証する | 不変のSchema、またはスキーマの失敗 |
| Document::parse | `文書`の文字列を読む | `文書`の構造 |
| validate | SchemaとDocumentをValidationOptionsで検証する | 文書の指摘 |
| extract_validated | 検証し、違反がない場合に抽出する | ValidatedValues、または文書の指摘 |
| extract_partial | 検証と取得可能な値の抽出を行う | 指摘とJSON値を持つPartialExtraction |
| ast_json | 既存の素のAST JSONを作る | JSON値、または実行失敗 |
| extract_typed_partial | 既存の型付き抽出JSONを作る | 型付き抽出のJSON値と指摘 |

## Examples

```gherkin
@id=EX-schema-015 @about=REQ-schema-049 @source=docs/decision/records/2026-10-03-public-crate-api.md#A42,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: 依存するクレートが入口だけで一通りを通せる
  Given `スキーマ`を宣言した`文書`の文字列がある
  When TBL-schema-010 の入口を順に呼ぶ
  Then `指摘`の並びと`抽出`の値の両方が得られる

@id=EX-schema-016 @about=REQ-schema-050 @source=docs/decision/records/2026-09-21-mds-spec.md#A24,docs/decision/records/2026-09-21-mds-spec.md#A57
Scenario: URL のスキーマは位置だけを返す
  Given URL の`スキーマ`を宣言した`文書`の文字列がある
  When resolve_schema を呼ぶ
  Then URL が返り、取得は行われない

@id=EX-schema-084 @about=REQ-schema-051 @source=docs/decision/records/2026-10-03-public-crate-api.md#A13,docs/decision/records/2026-10-03-public-crate-api.md#A16,docs/decision/records/2026-10-03-public-crate-api.md#A43
Scenario: 素の構文木型を公開した候補は契約に適合しない
  Given 候補の公開署名がmarkdown::mdast::Nodeを返す
  When 公開APIの依存を確認する
  Then 内部型が公開されているため適合と判定しない
```
