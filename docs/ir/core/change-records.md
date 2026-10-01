# 変更の照合記録と仕様の穴の処理先

照合記録の持つ情報、通常の check の静的検査、仕様の穴の処理先と、機械検査の保証の限界を扱う。保存形式は change-record-format.md、現在状態の検査は changes-results.md に定める。

## Requirements

### REQ-core-248: 照合記録の情報

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A4, docs/decision/records/2026-10-01-change-conformance.md#A5, docs/decision/records/2026-10-01-change-conformance.md#A11
- verification: unit

照合記録は常に、比較元、対象ファイル、対象内容の識別値、関連する`IR`とその内容の識別値、結論、理由、結論に応じた`判断の記録`への参照、照合した役を持つ。複数ファイルを変更意図ごとに1件へまとめられる。

### REQ-core-249: 通常の check の静的検査

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-current-change-records.md#A5, docs/decision/records/2026-10-01-current-change-records.md#A8
- verification: unit

kotowari は常に、"kotowari check" で、設定された照合記録すべての形式と、各件の`IR`・`判断の記録`・保留先への参照を検査し、必須情報や参照の欠落を出す。changes.records がパス成分で明示した隠しディレクトリ内の記録も読み、未指定の隠し配下は読まない。この静的検査のために Git の比較元を要求しない。

### REQ-core-250: 既存仕様の範囲という結論

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A5
- verification: unit

照合記録は常に、結論が既存仕様の範囲である件に、対応する`要求`への参照と、その範囲に収まる理由を持つ。

### REQ-core-251: 新しい判断という結論

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A1, docs/decision/records/2026-10-01-change-conformance.md#A5
- verification: unit

照合記録は常に、結論が新しい判断である件に、選択と根拠を記した`判断の記録`への参照を持つ。振る舞い・制約の仕様を変える選択は、変更後の`IR`とも対応する。

### REQ-core-252: 保留という結論

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-conformance.md#A5, docs/decision/records/2026-10-01-change-conformance.md#A10
- verification: unit

照合記録は常に、結論が保留である件に、理由と引き継ぎ先を持つ。仕様の穴を保留するときは、`判断の記録`にその保留を明示する。

### REQ-core-253: 記録は呼び出し側が書く

- kind: prohibition
- source: docs/decision/records/2026-10-01-current-change-records.md#A1
- verification: unit

kotowari は、照合記録や仕様の穴の処理先を、標準出力と標準エラーのほかへ書き出してはならない。これらは cycle または別の呼び出し側が共通の形式で書き、現在の変更に必要な照合記録だけをリポジトリに残す。更新で不要な過去記録は削除し、過去の証拠は Git 履歴から確認する。

### REQ-core-254: 機械検査の意味の限界

- kind: prohibition
- source: docs/decision/records/2026-10-01-change-conformance.md#A6, docs/decision/records/2026-10-01-change-conformance.md#P1
- verification: unit

kotowari は、照合記録の形式・対応・鮮度が検査を通ったことを、根拠が選択を支えること、仕様と実装の意味の一致、委譲範囲内であることを機械が証明した結果として出してはならない。

### REQ-core-255: 仕様の穴の分類と処理先

- kind: prohibition
- source: docs/decision/records/2026-10-01-change-conformance.md#A10, docs/decision/records/2026-10-01-change-conformance.md#A11
- verification: unit

仕様の穴の記録は常に、指摘の重要度・修正アクションとは別の分類と、`判断の記録`と`IR`への反映、仕様に合わせたコードの修正、`判断の記録`での明示的な保留のいずれかの処理先を持つ。"info" や "record_only" という扱いだけを処理済みの証拠にしてはならない。

## Examples

```gherkin
@id=EX-core-437 @about=REQ-core-249,REQ-core-250 @source=docs/decision/records/2026-10-01-change-conformance.md#A2,docs/decision/records/2026-10-01-change-conformance.md#A5,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: 既存仕様の参照がない
  Given 設定された照合記録の結論が既存仕様の範囲で、理由はあるが対応する`要求`への参照がない
  When "kotowari check" を実行する
  Then 参照の欠落が検出されて出る

@id=EX-core-438 @about=REQ-core-249,REQ-core-251 @source=docs/decision/records/2026-10-01-change-conformance.md#A1,docs/decision/records/2026-10-01-change-conformance.md#A2,docs/decision/records/2026-10-01-change-conformance.md#A5,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: 新判断の根拠を指す記録がない
  Given 設定された照合記録の結論が新しい判断で、`判断の記録`への参照がない
  When "kotowari check" を実行する
  Then 参照の欠落が検出されて出る

@id=EX-core-439 @about=REQ-core-249 @source=docs/decision/records/2026-10-01-change-conformance.md#A2
Scenario: Git の比較元なしで静的検査する
  Given 設定された照合記録の形式と参照が検査でき、Git の比較元を指定していない
  When "kotowari check" を実行する
  Then Git の比較元を要求せず、照合記録の形式と参照を検査する

@id=EX-core-440 @about=REQ-core-249,REQ-core-252 @source=docs/decision/records/2026-10-01-change-conformance.md#A2,docs/decision/records/2026-10-01-change-conformance.md#A5,docs/decision/records/2026-10-01-change-conformance.md#A6
Scenario: 保留先の欠落
  Given 設定された照合記録の結論が保留で、理由はあるが引き継ぎ先がない
  When "kotowari check" を実行する
  Then 参照の欠落が検出されて出る

```



```gherkin
@id=EX-core-457 @about=REQ-core-249,REQ-core-019 @source=docs/decision/records/2026-10-01-current-change-records.md#A8
Scenario: 名指しした隠し記録を静的検査する
  Given changes.records が .kotowari/changes/*.yaml を指定する
  And その配下に不正な形式または参照切れの記録がある
  When check と status を実行する
  Then 両方で change_record_invalid が出る
  And 未指定の .hidden 配下は検査しない

```
