# プロジェクトを使うRust API

[English](library-api.md) | 日本語

既存6操作を外部のRustアプリから呼ぶ入口、結果と失敗、CLI互換性を扱う。入力の保持は library-inputs.md、Tokio対応は library-async.md で扱う。

## Requirements

### REQ-core-310: プロジェクト操作の入口

- kind: algorithm
- source: docs/decision/records/2026-10-03-public-crate-api.md#A10, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A41
- definition: TBL-core-041
- verification: unit

### REQ-core-311: 呼出開始位置を明示する

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A52
- verification: unit

"Project::new" は "ProjectOptions" で絶対パスの呼出開始位置と任意の設定ファイル指定を受け取る。プロセスのカレントディレクトリを変更せず、既存の設定探索と相対パス解決を行う。CLIが現在位置を取得して渡す。入力パスの契約は REQ-core-323 に従う。

### REQ-core-312: 指摘と実行失敗を区別する

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A7, docs/decision/records/2026-10-03-public-crate-api.md#A8, docs/decision/records/2026-10-03-public-crate-api.md#A40, docs/decision/records/2026-10-06-changes-rethink.md#A10
- verification: unit

完了した検査は文書の指摘を持つ結果を返し、実行できない場合は "Result" の "Err" を返す。失敗は種類を判別でき、"std::error::Error" と "Display" を実装する。入力不足、設定・入力形式不正、未知の問い合わせID、読込失敗、内部対応不能を区別する。結果は指摘の種類・重要度・パス・存在する場合の行を読み取り専用で提供し、表示を行わない。

### REQ-core-313: CLI契約を保持する

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A9, docs/decision/records/2026-10-03-public-crate-api.md#A14, docs/decision/records/2026-10-03-public-crate-api.md#A33, docs/decision/records/2026-10-03-public-crate-api.md#A40, docs/decision/records/2026-10-03-public-crate-api.md#A43
- verification: unit

再構成は現在実装されている両バイナリのコマンド・引数・終了コード・出力・設定・IR・スキーマと検査結果の意味を維持する。CLIはライブラリの結果を既存のJSON/textと停止理由に写す。ソースからのインストール対象パッケージ名の変更は実行コマンド名を変えない。未実装のoverviewは別計画で追加する。

## Decision tables

### TBL-core-041: Projectの操作

- source: docs/decision/records/2026-10-03-public-crate-api.md#A10, docs/decision/records/2026-10-03-public-crate-api.md#A36, docs/decision/records/2026-10-03-public-crate-api.md#A41, docs/decision/records/2026-10-03-public-crate-api.md#A55, docs/decision/records/2026-10-04-overview-on-public-api.md#A3, docs/decision/records/2026-10-04-overview-on-public-api.md#A7, docs/decision/records/2026-10-06-changes-rethink.md#A10

| メソッド | 入力と操作 | 結果 |
|---|---|---|
| check | 設定に従ってプロジェクトを検査する。`全体像の元データ`の指摘と "overview" の群を含める | 指摘と集計 |
| list | 読み取れた項目を一覧にする | 型付き一覧 |
| query | 指定したIDを問い合わせる | 本文・逆参照を含む型付き結果 |
| status | checkと同じ範囲を検査する | 集計と完了状態 |
| plan | 指定した計画書を検査し、IRの読込は要求しない | 指摘と集計 |
| mutants | 指定した変異結果と等価情報を検査し、IRの読込は要求しない | 指摘と変異の集計 |
| overview_prepare | `全体像の元データ`を検査して描画し、ファイルを書かない | 書く前の描画の結果。書く操作を持つ |
| overview_build | overview_prepare に続けて、その結果を ".kotowari/cache/overview/" の下へ書く（REQ-core-293） | 書いたファイル・消したファイルの一覧と書かなかったファイルの数 |

## Examples

```gherkin
@id=EX-core-483 @about=REQ-core-310,REQ-core-311,REQ-core-312 @source=docs/decision/records/2026-10-03-public-crate-api.md#A7,docs/decision/records/2026-10-03-public-crate-api.md#A10,docs/decision/records/2026-10-03-public-crate-api.md#A36,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: 文書の違反は検査結果として受け取る
  Given 呼出開始位置を明示したプロジェクトに指摘の出るIRがある
  When Projectのcheckを呼ぶ
  Then 指摘を型で判別できる完了した検査結果を返す
  And 標準出力へ結果を書かずカレントディレクトリも変更しない

@id=EX-core-484 @about=REQ-core-310,REQ-core-312 @source=docs/decision/records/2026-10-03-public-crate-api.md#A36,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: 読めない入力を指摘なしの結果にしない
  Given 検査に必要なファイルが読めない
  When Projectのcheckを呼ぶ
  Then 読込失敗を判別できるErrを返す

@id=EX-core-485 @about=REQ-core-313 @source=docs/decision/records/2026-10-03-public-crate-api.md#A9,docs/decision/records/2026-10-03-public-crate-api.md#A40
Scenario: CLIの出力はRust内部型の変更から独立する
  Given 既存CLIの契約テストが入力と期待するJSON・text・終了コードを定めている
  When 再構成後のCLIを同じ入力で実行する
  Then 既存の期待する出力と終了コードに一致する

@id=EX-core-486 @about=REQ-core-310 @source=docs/decision/records/2026-10-03-public-crate-api.md#A41
Scenario: IRがなくても計画書と変異結果の検査は利用できる
  Given 計画書または変異結果に必要な入力がありIRの置き場はない
  When 対応するplanまたはmutants操作を呼ぶ
  Then IRの読込を要求せず結果を返す
```
