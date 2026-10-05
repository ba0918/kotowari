# 任意のTokio対応

[English](library-async.md) | 日本語

kotowariとMarkdownスキーマI/Oの高水準操作をTokioから待つ入口、同時実行数、待機をやめたときの動作を扱う。

## Requirements

### REQ-core-318: 同期操作を待つ非同期入口

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A22, docs/decision/records/2026-10-03-public-crate-api.md#A27, docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

"kotowari" と "kotowari-markdown-schema-io" は既定で無効な "tokio" feature で "AsyncProject" と "AsyncSchemaLoader" をそれぞれ提供する。対象は同期高水準操作と読込操作で、"spawn_blocking" に同期処理を渡し、同じ入力に対する値・指摘・通常の失敗分類を共用する。CLIは同期APIを使う。

### REQ-core-319: 実行枠の共有

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A23, docs/decision/records/2026-10-03-public-crate-api.md#A30, docs/decision/records/2026-10-03-public-crate-api.md#A31, docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

"AsyncOptions" は既定の同時実行数を1とし、変更値を "NonZeroUsize" で受ける。非同期実行用オブジェクトごとに上限を持ち、複製したオブジェクト同士は実行枠を共有する。

### REQ-core-320: キャンセルと実行枠の寿命

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A24, docs/decision/records/2026-10-03-public-crate-api.md#A32
- verification: unit

実行枠を待つ間にキャンセルされた処理は投入しない。投入後は呼出側が待機をやめても処理完了まで実行枠を保持する。開始済みのGit・HTTP・計算・キャッシュ書込みの停止は保証しない。

### REQ-core-321: ランタイムとスレッドの境界

- kind: ubiquitous
- source: docs/decision/records/2026-10-03-public-crate-api.md#A45
- verification: unit

非同期APIは利用者のTokioランタイム上で実行し、内部ランタイムを作らない。ランタイム不在とタスク実行失敗は型付きの失敗として返す。渡す入力・返す結果・公開Futureはスレッド間に移動できる型として提供する。

## Examples

```gherkin
@id=EX-core-492 @about=REQ-core-318,REQ-core-321 @source=docs/decision/records/2026-10-03-public-crate-api.md#A22,docs/decision/records/2026-10-03-public-crate-api.md#A45
Scenario: 同期と非同期で同じ指摘を得る
  Given 同じ入力を読む同期の入口とTokioランタイム上の非同期入口がある
  When 同じ検査を実行する
  Then 同じ値と指摘を返す
  And 非同期呼出のFutureと結果を別スレッドへ渡せる

@id=EX-core-493 @about=REQ-core-319,REQ-core-320 @source=docs/decision/records/2026-10-03-public-crate-api.md#A30,docs/decision/records/2026-10-03-public-crate-api.md#A31,docs/decision/records/2026-10-03-public-crate-api.md#A32
Scenario: 開始済み処理の待機中止で実行枠を空けない
  Given 既定上限1の実行用オブジェクトとその複製がある
  And 最初の処理は開始済みで完了していない
  When 最初の処理への待機をやめて複製から別の処理を要求する
  Then 別の処理は最初の処理が完了するまで投入されない

@id=EX-core-494 @about=REQ-core-320 @source=docs/decision/records/2026-10-03-public-crate-api.md#A32
Scenario: 枠待ちをやめた処理は後から投入しない
  Given 処理が実行枠を待っている
  When 枠を得る前に呼出をキャンセルする
  Then 枠が空いてもその処理を投入しない

@id=EX-core-495 @about=REQ-core-321 @source=docs/decision/records/2026-10-03-public-crate-api.md#A45
Scenario: Tokioランタイムのない場所で待つ
  Given 呼出側がTokioランタイム外で非同期APIをpollする
  When 操作の実行を要求する
  Then ランタイム不在を型付きの失敗で返し内部ランタイムを作らない
```
