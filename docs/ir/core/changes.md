# 変更に対する照合の検査

変更照合の専用コマンドが、指定された Git の変更と照合記録の対応・鮮度を検査する責務を扱う。具体的な引数、対象の設定と除外、指摘の出力は FLAGS.md の未決事項にある。

## Requirements

### REQ-core-240: 専用コマンド

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A2
- verification: unit

kotowari は常に、変更に対する照合の欠落と古さを検査する専用コマンド "kotowari changes" を持つ。

### REQ-core-241: 変更の独立した列挙

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A3
- verification: unit

変更照合の専用コマンドは常に、呼び出し元が指定した比較元と対象から Git の変更を取り出し、設定された検査対象ファイルの追加・変更・削除をファイル単位で列挙する。照合記録の対象ファイル一覧だけから検査対象を作ってはならない。

### REQ-core-242: 対象変更の照合漏れ

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A3, docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A6
- verification: unit

変更照合の専用コマンドは常に、列挙した対象変更のうち照合記録のどの件にも対応しないものを、照合の欠落として検出して出す。複数ファイルを1件で照合しても、対応するファイルの変更をすべて照合対象として明示する。

### REQ-core-243: コードとテストの鮮度

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A6
- verification: unit

変更照合の専用コマンドは常に、照合記録にある対象内容の識別値と検査対象の内容の識別値が一致しない件を、再照合が必要な件として検出して出す。

### REQ-core-244: 関連仕様の鮮度

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A6
- verification: unit

変更照合の専用コマンドは常に、照合記録にある関連する`IR`の内容の識別値と検査対象の`IR`の内容の識別値が一致しない件を、再照合が必要な件として検出して出す。

### REQ-core-245: テスト差分だけで対象を狭めない

- kind: prohibition
- source: docs/decision/records/2026-10-01-change-conformance.md#A7
- verification: unit

変更照合の専用コマンドは、`テスト`の期待値に変更が無いことだけを理由に、設定された検査対象ファイルに含まれる製品コードや補助関数の変更を検査対象から外してはならない。

### REQ-core-246: ステージされた対象

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A8
- verification: unit

変更照合の専用コマンドは常に、pre-commit の検査では HEAD とステージ済みの内容を比較し、ステージされていない作業ツリーの変更をコミット対象として数えない。

### REQ-core-247: 最終検査の対象

- kind: prohibition
- source: docs/decision/records/2026-10-01-change-conformance.md#A8, docs/decision/records/2026-10-01-change-conformance.md#A12
- verification: unit

変更照合の専用コマンドは常に、cycle の最終検査と CI では、呼び出し元が指定した比較元から対象ブランチ全体の変更を検査する。直前の1コミットだけを最終検査の対象にしてはならない。

## Examples

```gherkin
@id=EX-core-430 @about=REQ-core-241,REQ-core-242 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: 申告されない変更を拾う
  Given 設定された対象ファイル2つに変更があり、照合記録は片方だけを対象にしている
  When 変更照合の専用コマンドでその比較元と対象を検査する
  Then 記録されていない変更について照合の欠落が検出されて出る

@id=EX-core-431 @about=REQ-core-242 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A4
Scenario: 複数ファイルを意図ごとにまとめる
  Given 対象変更2つが、同じ変更意図の照合記録1件にどちらも明示されている
  When 変更照合の専用コマンドでその比較元と対象を検査する
  Then この2つについて照合の欠落は出ない

@id=EX-core-432 @about=REQ-core-243 @source=docs/decision/records/2026-10-01-change-conformance.md#A4,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: 照合後にコードが変わる
  Given 照合記録が対象ファイルの以前の内容の識別値を持つ
  And 検査対象の内容は照合後に変わっている
  When 変更照合の専用コマンドでその対象を検査する
  Then その件について再照合が必要な件として検出されて出る

@id=EX-core-433 @about=REQ-core-244 @source=docs/decision/records/2026-10-01-change-conformance.md#A4,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: コードが同じでも仕様が変わる
  Given 対象ファイルは照合時と同じで、関連する`IR`の内容が照合後に変わっている
  When 変更照合の専用コマンドでその対象を検査する
  Then その件について再照合が必要な件として検出されて出る

@id=EX-core-434 @about=REQ-core-241,REQ-core-245 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A7
Scenario: テストを変えない製品コードの変更
  Given 設定された対象の製品コードが変わり、`テスト`の期待値は変わっていない
  When 変更照合の専用コマンドでその比較元と対象を検査する
  Then 製品コードの変更は照合対象に含まれる

@id=EX-core-435 @about=REQ-core-246 @source=docs/decision/records/2026-10-01-change-conformance.md#A8
Scenario: コミット対象と作業ツリーを分ける
  Given ファイルにステージ済みの変更と、追加のステージされていない変更がある
  When pre-commit の変更照合を実行する
  Then ステージ済みの内容を対象に検査し、追加の変更をコミット対象として数えない

@id=EX-core-436 @about=REQ-core-247,REQ-core-242 @source=docs/decision/records/2026-10-01-change-conformance.md#A3,docs/decision/records/2026-10-01-change-conformance.md#A6,docs/decision/records/2026-10-01-change-conformance.md#A8,docs/decision/records/2026-10-01-change-conformance.md#A12
Scenario: 以前のコミットにある未照合変更
  Given ブランチの以前のコミットに照合されていない対象変更があり、直前のコミットには照合記録だけがある
  When 指定された比較元からブランチ全体を最終検査する
  Then 以前のコミットの未照合変更について照合の欠落が検出されて出る

```

