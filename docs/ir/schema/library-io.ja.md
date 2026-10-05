# スキーマと文書を読み込むライブラリ

[English](library-io.md) | 日本語

ファイル・URLからスキーマと文書を取得する別クレートの入口と、既存の読込動作の再利用を扱う。

## Requirements

### REQ-schema-072: 読み込みの入口

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A17, docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A44, docs/decision/records/2026-10-03-public-crate-api.md#A52
- verification: unit

"kotowari-markdown-schema-io" は "SchemaLoader::new" と "LoaderOptions" で絶対パスの作業開始位置と任意のキャッシュ基準を受ける。位置の契約は REQ-core-323 に従う。"load" は再利用可能なスキーマと文書、"check" は単一ファイルまたはディレクトリの結果、"extract_validated" と "extract_partial" はそれぞれの抽出結果を返す。

### REQ-schema-073: I/Oと表示の責務

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A9, docs/decision/records/2026-10-03-public-crate-api.md#A17, docs/decision/records/2026-10-03-public-crate-api.md#A44
- verification: unit

I/Oクレートは既存CLIの探索・URL解決・キャッシュ・取得上限・タイムアウトの規則を維持し、カレントディレクトリを変更しない。キャッシュは既存の場所と寿命に従う。結果を標準出力・標準エラーへ書かず、表示と終了コード決定をCLIに残す。

### REQ-schema-074: 読み込み失敗と文書の指摘

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A8, docs/decision/records/2026-10-03-public-crate-api.md#A40, docs/decision/records/2026-10-03-public-crate-api.md#A44
- verification: unit

I/Oクレートは、完了した文書検査の指摘と、読み込みや入力形式の失敗を区別する。実行できない失敗は種類を判別できる "Result" の "Err" とし、失敗型は "std::error::Error" と "Display" を実装する。

### REQ-schema-075: 任意の非同期入口

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A22, docs/decision/records/2026-10-03-public-crate-api.md#A24, docs/decision/records/2026-10-03-public-crate-api.md#A27, docs/decision/records/2026-10-03-public-crate-api.md#A30, docs/decision/records/2026-10-03-public-crate-api.md#A31, docs/decision/records/2026-10-03-public-crate-api.md#A32, docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

"kotowari-markdown-schema-io" は既定で無効な "tokio" feature で "AsyncSchemaLoader" を提供する。非同期の操作範囲・ランタイム・実行枠・待機キャンセルの契約は REQ-core-318、REQ-core-319、REQ-core-320、REQ-core-321 に従う。

## Examples

```gherkin
@id=EX-schema-090 @about=REQ-schema-072,REQ-schema-073 @source=docs/decision/records/2026-10-03-public-crate-api.md#A17,docs/decision/records/2026-10-03-public-crate-api.md#A44
Scenario: 位置を渡して文書とスキーマを読み込む
  Given 作業開始位置とキャッシュ基準を指定したSchemaLoaderがある
  When スキーマ参照のある文書をloadする
  Then 再利用可能なスキーマと文書を返す
  And カレントディレクトリを変更せず結果を端末へ表示しない

@id=EX-schema-091 @about=REQ-schema-073,REQ-schema-074 @source=docs/decision/records/2026-10-03-public-crate-api.md#A8,docs/decision/records/2026-10-03-public-crate-api.md#A40,docs/decision/records/2026-10-03-public-crate-api.md#A44
Scenario: スキーマの取得失敗を文書の違反と混同しない
  Given スキーマを既存の取得規則に従って読み込めない
  When 文書のcheckを呼ぶ
  Then 完了した文書検査の結果ではなく型付きの実行失敗を返す

@id=EX-schema-092 @about=REQ-schema-075 @source=docs/decision/records/2026-10-03-public-crate-api.md#A22,docs/decision/records/2026-10-03-public-crate-api.md#A27,docs/decision/records/2026-10-03-public-crate-api.md#A45
Scenario: Tokio対応を選んだ呼出でも同期と同じ結果を得る
  Given tokio featureを有効にした利用者が同じ入力を用意する
  When AsyncSchemaLoaderをTokio上で待つ
  Then 同期のSchemaLoaderと同じ値と指摘を返す
```
