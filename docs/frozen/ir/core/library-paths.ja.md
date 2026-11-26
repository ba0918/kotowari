# ライブラリ入力のパス

[English](library-paths.md) | 日本語

メモリ入力の論理パスと、I/O入口の作業開始位置・キャッシュ基準を扱う。

## Requirements

### REQ-core-322: 論理パスの基準と同一性

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A51, docs/decision/records/2026-10-03-public-crate-api.md#A53, docs/decision/records/2026-10-03-public-crate-api.md#A54, docs/decision/records/2026-10-06-changes-rethink.md#A10
- verification: unit

メモリ入力の論理パスはプロジェクト基準からの相対パスとし、既存の区切り文字・先頭の "./"・途中の "/./"・重複区切り・末尾区切りを正規化する。既存の規則が許す親相対パスと残る ".." 成分を保持する。結果が空・絶対パスの場合、同じ群に正規化後の重複パスがある場合、または別の群の同一パスの元の本文が異なる場合は "InvalidInput" とする。テストと面の解析結果にも解析に用いたSourceTextの全文を保持して本文を比較する。同一内容の群間共有は既存の重なり規則で判定する。ファイルシステムで正規化せず、シンボリックリンクも解決しない。表示と出典照合は同じプロジェクト相対パスを使う。

### REQ-core-323: I/Oの位置を固定する

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A44, docs/decision/records/2026-10-03-public-crate-api.md#A52
- verification: unit

"ProjectOptions" と "LoaderOptions" の作業開始位置と明示するキャッシュ基準は絶対パスを受け、相対パスなら "InvalidInput" を返す。設定ファイルの相対パスは作業開始位置から解決する。キャッシュ基準の省略時は作業開始位置から既存の基準探索を行う。後のカレントディレクトリ変更で解決基準を変えない。

## Examples

```gherkin
@id=EX-core-498 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A51
Scenario: 別表記の重複パスを拒否する
  Given 同じ群に先頭の./だけが異なる同じ論理パスがある
  When メモリ入力を構築する
  Then 正規化後の重複としてInvalidInputを返す

@id=EX-core-499 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A51
Scenario: 存在しないファイル名でもメモリ文書を扱える
  Given 正常なプロジェクト相対パスと文字列があり対応するファイルはない
  When メモリ入力を構築する
  Then ファイルを探さず論理パスと内容を受け付ける

@id=EX-core-500 @about=REQ-core-323 @source=docs/decision/records/2026-10-03-public-crate-api.md#A52
Scenario: 相対の作業位置を暗黙に解決しない
  Given 作業開始位置に相対パスを渡す
  When I/O入口のオプションを構築する
  Then InvalidInputを返す

@id=EX-core-501 @about=REQ-core-323 @source=docs/decision/records/2026-10-03-public-crate-api.md#A52
Scenario: 呼出後のカレントディレクトリ変更で基準を変えない
  Given 絶対パスの作業開始位置を指定した入口がある
  When 呼出側がカレントディレクトリを変更して操作する
  Then 指定した作業開始位置を基準に解決する

@id=EX-core-502 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A54
Scenario: 親相対のIRを既存どおり扱う
  Given IRの置き場が "../spec/ir" で論理パスが "../spec/ir/topic.md" である
  When メモリ入力を解析する
  Then 親相対のパスを理由に拒否せず同じ基準で扱う

@id=EX-core-503 @about=REQ-core-322 @source=docs/decision/records/2026-10-03-public-crate-api.md#A51,docs/decision/records/2026-10-03-public-crate-api.md#A53
Scenario: 発見結果が同じでも元の内容の食い違いを検出する
  Given テストと面の解析結果が同じ論理パスを持ち元の本文は異なる
  When メモリ入力を構築する
  Then 発見結果と指摘の一致によらずInvalidInputを返す
```
