# ライブラリのクレート境界

公開するRustライブラリとCLIの責務、依存方向、利用者が選ぶ入口を扱う。公開作業と版管理の手順は判断の記録に置く。

## Requirements

### REQ-core-306: 用途ごとのパッケージ

- kind: algorithm
- source: docs/decision/records/2026-10-03-public-crate-api.md#A14, docs/decision/records/2026-10-03-public-crate-api.md#A25, docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A35, docs/decision/records/2026-10-04-overview-on-public-api.md#A4
- definition: TBL-core-040
- verification: review
- how_to_verify: Cargoのパッケージ一覧・通常依存グラフ・各公開入口の外部利用例を表と比較する。

### REQ-core-307: 計算側に環境操作を置かない

- kind: prohibition
- source: docs/decision/records/2026-10-03-public-crate-api.md#A11, docs/decision/records/2026-10-03-public-crate-api.md#A25, docs/decision/records/2026-10-03-public-crate-api.md#A35, docs/decision/records/2026-10-04-overview-on-public-api.md#A1
- verification: review
- how_to_verify: coreとsource-analysisとoverviewの通常依存、公開入口からの呼出経路、ファイル・環境変数・プロセス・端末・ネットワーク操作の所在を確認する。

"kotowari-core"、"kotowari-source-analysis"、"kotowari-overview" は、入力をファイル・環境変数・Git・HTTPから取得せず、標準出力・標準エラーへ書かず、TokioとCLI用依存を持たない。coreはIR解析・出典・テスト対応・変異結果・変更照合・計画書・読み取り結果を責務別モジュールに分ける。

### REQ-core-308: 解析と判定の依存方向

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A26, docs/decision/records/2026-10-03-public-crate-api.md#A41
- verification: unit

coreのテスト対応判定は発見済みのテスト・印を受け取り、ソース解析を呼ばない。変更照合の "Comparison" と識別値計算はcoreが持ち、Git取得側が同じ型を構築する。"Analyzer" は既存の組込み規則と文字列の追加規則でソース文字列を解析し、テスト・印・面と指摘を返す。規則と未対応言語の既存の意味を維持する。

### REQ-core-309: 内部の型を公開契約に混ぜない

- kind: prohibition
- source: docs/decision/records/2026-10-03-public-crate-api.md#A13, docs/decision/records/2026-10-03-public-crate-api.md#A16, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A40
- verification: review
- how_to_verify: rustdocの公開署名と下位クレートの外部利用例を確認する。再公開した型が元の型と同じ型として受け渡せることをコンパイルで確認する。

公開署名はMarkdown解析とソース検索エンジンの内部型を含まない。公開する下位クレートのAPIも互換性管理の対象とし、実装詳細は非公開にする。"kotowari" は入口で使う下位の入力型・結果型を複製せず同じ型として再公開する。

## Decision tables

### TBL-core-040: パッケージと通常依存

- source: docs/decision/records/2026-10-03-public-crate-api.md#A34, docs/decision/records/2026-10-03-public-crate-api.md#A35, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A41, docs/decision/records/2026-10-03-public-crate-api.md#A42, docs/decision/records/2026-10-03-public-crate-api.md#A44, docs/decision/records/2026-10-04-overview-on-public-api.md#A1, docs/decision/records/2026-10-04-overview-on-public-api.md#A4

| パッケージ | 使う目的 | workspace内の通常依存 |
|---|---|---|
| kotowari | プロジェクトを読み、型付きの操作を呼ぶ | kotowari-core、kotowari-source-analysis、kotowari-overview |
| kotowari-core | メモリ上のIR解析・検査・結果の組立て | kotowari-markdown-schema |
| kotowari-source-analysis | メモリ上のソースからテスト・印・面を発見する | kotowari-core |
| kotowari-overview | メモリ上の全体像の元データを検査し、参照の表・古い節・描画の入力を作る | kotowari-core、kotowari-markdown-schema、kotowari-markdown-view |
| kotowari-cli | kotowariコマンドの引数・表示・終了コード、全体像を配る serve | kotowari |
| kotowari-markdown-schema | メモリ上のMarkdownスキーマ検証・抽出 | なし |
| kotowari-markdown-schema-io | 文書・スキーマの読込と検査・抽出 | kotowari-markdown-schema |
| kotowari-markdown-view | メモリ上の描画の入力から全体像のページを作る | なし |
| kotowari-mds | kotowari-mdsコマンドの引数・表示・終了コード | kotowari-markdown-schema-io、kotowari-markdown-schema |

## Examples

```gherkin
@id=EX-core-480 @about=REQ-core-306,REQ-core-307,REQ-core-309 @source=docs/decision/records/2026-10-03-public-crate-api.md#A11,docs/decision/records/2026-10-03-public-crate-api.md#A13,docs/decision/records/2026-10-03-public-crate-api.md#A25,docs/decision/records/2026-10-03-public-crate-api.md#A35,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: IR専用の利用に環境操作の依存を持ち込まない
  Given 外部アプリがメモリ上のIRを扱うためにkotowari-coreだけに依存する
  When 公開APIの利用例と通常依存グラフを確認する
  Then ast-grepとTokioとCLI・HTTP用依存を必要としない
  And 内部の構文木型を利用者が扱う必要がない

@id=EX-core-481 @about=REQ-core-308 @source=docs/decision/records/2026-10-03-public-crate-api.md#A26,docs/decision/records/2026-10-03-public-crate-api.md#A41
Scenario: 発見済みのテストだけで対応を判定する
  Given IRと発見済みテスト・印がメモリ上にある
  When coreでテスト対応を判定する
  Then ソースファイルやソース解析を要求せず判定結果を返す

@id=EX-core-482 @about=REQ-core-306,REQ-core-307,REQ-core-309 @source=docs/decision/records/2026-10-03-public-crate-api.md#A13,docs/decision/records/2026-10-03-public-crate-api.md#A35,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: 内部依存を公開した候補は境界の検証に通らない
  Given coreがGitを実行するか公開署名が検索エンジンの内部型を返す候補がある
  When 依存方向と公開APIを確認する
  Then その候補をクレート境界の適合とは判定しない
```
