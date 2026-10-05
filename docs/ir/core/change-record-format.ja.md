# 照合記録の保存形式と参照

[English](change-record-format.md) | 日本語

変更照合の具体的な入力、記録または合否を定める。具体的な契約と判断の根拠は出典の記録で追う。

## Requirements

### REQ-core-268: 保存形式

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-current-change-records.md#A4
- verification: unit

変更照合の検査は常に、次の契約を満たす。照合記録は UTF-8 の YAML とし、1ファイルは version: 1 と entries の一覧だけを持つ。entries は空でもよい。未知の版、未知の鍵、重複した YAML の鍵、誤った型は change_record_invalid の誤りとする。各件の鍵は id、base、role、files、ir、conclusion、reason、requirements、decisions、handoff、gaps の11個だけとし、すべて必須とする。id は "[A-Za-z0-9][A-Za-z0-9._-]*" に合う文字列、reason は空白だけでない文字列、base は Git の完全な object ID（40桁または64桁の小文字16進）、role は implementer または reviewer、conclusion は existing、new または deferred とする。id は読む照合記録全体で一意とする。files は1件以上、ir・requirements・decisions・gaps は0件以上の一覧、handoff は null または判断の記録への参照文字列とする。保存場所は changes.records が指定し、版の異なる記録を自動変換しない。開発中の旧 state 鍵も未知の鍵として拒否し、自動移行しない。

### REQ-core-269: ファイルと IR の内容識別

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A7
- verification: unit

変更照合の検査は常に、次の契約を満たす。files の各件は path、before、after の3鍵だけを持つ。path は基準からの正規化した相対パス、before と after は null または "sha256:" と64桁の小文字16進で、両方 null にしない。追加は before を null、削除は after を null とし、変更は両方に識別値を持つ。識別値は Git の6文字の mode、NUL 1バイト、blob の全バイトを順に連結した値の SHA-256 とする。ir の各件は path と sha256 の2鍵で、sha256 は IR ファイルの全バイトだけを SHA-256 で計算した同じ表記とする。path は空、絶対パス、..、.、バックスラッシュを含む成分を認めない。files と ir の各一覧内の重複パスは誤りとする。

### REQ-core-270: 結論ごとの参照

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A8
- verification: unit

変更照合の検査は常に、次の契約を満たす。requirements は既存の要求 ID の文字列一覧、decisions と handoff は既存の出典と同じリポジトリ内の "パス#決定番号" の表記とする。existing は requirements を1件以上、ir を1件以上持ち、参照要求を定義する IR のすべてを ir に含める。new は decisions を1件以上持ち、仕様を変更した場合の関連 IR への反映を review が確認する。deferred は decisions を1件以上と null でない handoff を持つ。判断の記録・要求・IR の存在と参照の整合を check で検査する。根拠と保留の意味は review が確認する。

### REQ-core-271: 仕様の穴の構造

- kind: ubiquitous
- source: docs/decision/records/2026-10-01-change-details.md#A9
- verification: unit

変更照合の検査は常に、次の契約を満たす。gaps の各件は category、disposition、refs の3鍵だけを持つ。category は missing_spec、spec_conflict、premise_conflict のいずれかで、未分類の値は誤りとする。disposition は recorded、fixed、deferred のいずれかで、refs は1件以上の判断の記録への参照を持つ。recorded は反映した選択と根拠を参照し、fixed は修正の判断と既存要求への対応を記録し、deferred は明示的な保留を参照する。gaps がある件の conclusion は、deferred が1件でもあれば deferred、それがなく recorded が1件でもあれば new、それ以外は existing とする。fixed を含む件は conclusion が new でも対応する要求とその定義 IR を持つ。recorded と fixed は同じ件に共存できる。各処理先が選択を支えることと、recorded の関連 IR への反映は review が確認する。

## Examples

```gherkin
@id=EX-core-446 @about=REQ-core-268 @source=docs/decision/records/2026-10-01-change-details.md#A6,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 未知の版
  Given 設定された記録の version が 2 である
  When check を実行する
  Then change_record_invalid の誤りが出る

@id=EX-core-447 @about=REQ-core-269 @source=docs/decision/records/2026-10-01-change-details.md#A7,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 実行権だけの変更
  Given blob は同じだがファイルの実行権が変わり、記録は以前の mode である
  When changes を実行する
  Then change_stale の誤りが出る

@id=EX-core-448 @about=REQ-core-270 @source=docs/decision/records/2026-10-01-change-details.md#A8,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 既存要求の IR が欠ける
  Given existing の参照要求の定義ファイルが ir の一覧にない
  When check を実行する
  Then change_record_invalid の誤りが出る

@id=EX-core-449 @about=REQ-core-271 @source=docs/decision/records/2026-10-01-change-details.md#A9,docs/decision/records/2026-10-01-change-details.md#A12,docs/decision/records/2026-10-01-change-details.md#A11
Scenario: 穴の扱いが未分類
  Given gaps の category が unknown である
  When check を実行する
  Then change_record_invalid の誤りが出る

@id=EX-core-453 @about=REQ-core-271 @source=docs/decision/records/2026-10-01-change-details.md#A9,docs/decision/records/2026-10-01-change-details.md#A12
Scenario: 新仕様と既存仕様への修正を同じ件にまとめる
  Given 同じファイルの gaps に recorded と fixed があり、両方の判断と必要な要求と IR の参照が揃う
  And conclusion は new である
  When check を実行する
  Then 処理先の混在を理由に change_record_invalid を出さない
```


```gherkin
@id=EX-core-456 @about=REQ-core-268 @source=docs/decision/records/2026-10-01-current-change-records.md#A4
Scenario: 旧 state 鍵を暗黙変換しない
  Given version 1 の件に旧 state 鍵がある
  When check を実行する
  Then 未知の鍵として change_record_invalid が出る

```
