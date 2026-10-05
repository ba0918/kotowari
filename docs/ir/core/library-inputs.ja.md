# メモリ入力と読み込み済みの結果

[English](library-inputs.md) | 日本語

部分的なIRの解析、検査に必要な入力群、不変の読込結果の寿命と再利用を扱う。

## Requirements

### REQ-core-314: 部分解析の読み取り

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A12, docs/decision/records/2026-10-03-public-crate-api.md#A38, docs/decision/records/2026-10-03-public-crate-api.md#A51, docs/decision/records/2026-10-03-public-crate-api.md#A54
- verification: unit

"ir::parse" は "SourceText" の論理パスと文字列、およびIRの置き場を指定する "IrOptions" から "IrDocument" を作る。置き場の既定は "docs/ir" とし、SourceTextはその配下のパスを持つ。取得できた項目・参照・指摘を読み取り専用で返す。IDが欠落または不正でも取得できた項目を除かず、元の文書の1始まりの行と存在する場合の終端行を返す。存在しないIDと位置は補わず、list/queryの既存の掲載条件は変更しない。

### REQ-core-315: 未提供と空集合

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A6, docs/decision/records/2026-10-03-public-crate-api.md#A29, docs/decision/records/2026-10-03-public-crate-api.md#A39, docs/decision/records/2026-10-03-public-crate-api.md#A49, docs/decision/records/2026-10-03-public-crate-api.md#A50
- definition: TBL-core-042
- verification: unit

"ReadModel::build" と "Inspection::build" は設定と TBL-core-042 の入力を受ける。必須の群が未提供なら "InputMissing" を返し、提供済みの空集合は通常の規則で検査する。条件が偽の群は未提供でもよく、提供されても判定に使わない。解析時の指摘とファイル一覧を保持し、既存の指摘重複抑制・件数・置き場の重なり検査を再現する。個別ファイルの網羅性は呼出側が担い、InputMissingは未提供の群を検出する。メモリ入力からファイルを自動取得しない。

### REQ-core-316: 読み込み済み結果の再利用

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A18, docs/decision/records/2026-10-03-public-crate-api.md#A19, docs/decision/records/2026-10-03-public-crate-api.md#A37
- verification: unit

"Project::read" は不変の "ReadModel" を、"Project::inspect" は不変の "Inspection" を返す。前者のlist/queryと後者のcheck/statusおよび読み取り結果への参照は追加I/Oを行わない。所有値は利用者が破棄するまで保持され、保存や自動監視はしない。変更を反映するには再読込する。簡便なProjectの7操作は呼出ごとに必要な入力を新しく読む。ファイル群の読込中の同時点性は保証しない。

### REQ-core-317: 操作ごとの読込範囲

- kind: invariant
- source: docs/decision/records/2026-10-03-public-crate-api.md#A19, docs/decision/records/2026-10-03-public-crate-api.md#A29, docs/decision/records/2026-10-03-public-crate-api.md#A37, docs/decision/records/2026-10-03-public-crate-api.md#A39
- verification: unit

読み込み済み結果を提供するときも、list/query用の入力にはcheck/statusだけが使うガイド・面・照合記録を要求しない。IRだけの解析は他の入力群を要求しない。

## Decision tables

### TBL-core-042: 入力群の内容と必須条件

- source: docs/decision/records/2026-10-03-public-crate-api.md#A39, docs/decision/records/2026-10-03-public-crate-api.md#A49, docs/decision/records/2026-10-03-public-crate-api.md#A50, docs/decision/records/2026-10-03-public-crate-api.md#A53, docs/decision/records/2026-10-04-overview-on-public-api.md#A2

| 入力群 | 内容 | ReadInputsで必須 | CheckInputsで必須 |
|---|---|---|---|
| IR | 論理パスと文書内容 | 常に | 常に |
| 判断の記録の置き場 | 判断の記録と出典対象の通常Markdownのパス・内容 | 参照の有無によらず常に | 同左 |
| ADR | パスと内容 | 参照の有無によらず常に | 同左 |
| テスト情報 | 対象ファイルのSourceText、言語・問い合わせの有無、発見済みテスト・印、解析時の指摘 | tests.filesが空でない | 同左 |
| ガイド | パスと内容 | 不要 | guides.filesが空でない |
| 面の解析結果 | 対象ファイルのSourceText、言語・問い合わせの有無、発見結果、解析時の指摘 | 不要 | surface.rulesが空でない |
| 未記載の面の一覧 | 設定された一覧のパスと内容 | 不要 | surface.rulesが空でなくsurface.unspecifiedが指定されている |
| 照合記録 | パスと内容 | 不要 | changesが設定されている |
| 追加の指摘の群 | 群の名前、読んだファイルの数、印の数、指摘。coreは意味を知らず、ほかの指摘と合わせて並べて数え、群ごとの数を結果に持たせる | 不要 | 不要（渡したときだけ加える） |

## Examples

```gherkin
@id=EX-core-487 @about=REQ-core-314,REQ-core-317 @source=docs/decision/records/2026-10-03-public-crate-api.md#A12,docs/decision/records/2026-10-03-public-crate-api.md#A29,docs/decision/records/2026-10-03-public-crate-api.md#A38
Scenario: IDがない解析項目を参照できる
  Given IR文字列にIDが欠けた取得可能な項目がある
  When 他の入力を渡さずir::parseを呼ぶ
  Then その項目と元の行と指摘を参照できる
  And IDを補ったり項目を一覧の掲載条件で除いたりしない

@id=EX-core-488 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A29,docs/decision/records/2026-10-03-public-crate-api.md#A39
Scenario: 必須の入力が未提供なら検査完了にしない
  Given 設定で必要なテスト情報の群が未提供である
  When メモリ入力から検査結果を構築する
  Then InputMissingを返し完了した検査結果を返さない

@id=EX-core-489 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A29,docs/decision/records/2026-10-03-public-crate-api.md#A39
Scenario: 明示的な空のテスト情報は検査する
  Given 必須入力が提供されテスト情報は提供済みの空集合である
  When テストを必要とする要求を検査する
  Then InputMissingではなく通常のテスト対応の指摘を含む結果を返す

@id=EX-core-490 @about=REQ-core-316 @source=docs/decision/records/2026-10-03-public-crate-api.md#A18,docs/decision/records/2026-10-03-public-crate-api.md#A37
Scenario: 読込後のファイル変更は保持した結果を変えない
  Given ReadModelを取得した後に元のファイルを変更した
  When 同じReadModelに一覧と問い合わせを求める
  Then 読込時の内容から結果を返す
  And 明示的にProjectから再読込した結果だけが変更を反映する

@id=EX-core-491 @about=REQ-core-317 @source=docs/decision/records/2026-10-03-public-crate-api.md#A19,docs/decision/records/2026-10-03-public-crate-api.md#A37,docs/decision/records/2026-10-03-public-crate-api.md#A39
Scenario: ガイドの読込失敗で一覧を止めない
  Given listに必要な入力は読めるがガイドは読めない
  When Projectのreadから一覧を取得する
  Then ガイドを読まず一覧を返す

@id=EX-core-496 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A49,docs/decision/records/2026-10-03-public-crate-api.md#A50
Scenario: 解析失敗の空の発見結果を成功と混同しない
  Given 必須のテスト情報に解析失敗の指摘と空の発見結果がある
  When メモリ入力を検査する
  Then 解析失敗の指摘を結果に引き継ぐ

@id=EX-core-497 @about=REQ-core-315 @source=docs/decision/records/2026-10-03-public-crate-api.md#A50
Scenario: 対象を設定しない入力群は要求しない
  Given tests.filesが空で他の必須入力は提供されている
  When テスト情報を未提供のままReadModelを構築する
  Then テスト情報のInputMissingを返さない
```
