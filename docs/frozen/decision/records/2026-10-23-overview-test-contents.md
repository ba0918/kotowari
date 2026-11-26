# 全体像の書き出しでテストのファイルの中身を読まない

## Context

"kotowari overview build" と "kotowari overview serve" は check と同じ読み込みを通り、"tests.files" に当たるファイルをすべて読んで解析していた。
全体像が使うのはテストのファイルのパスだけで（ガイドと全体像の元データとの重なりの判定、[REQ-core-280](../../ir/core/overview-data.md#REQ-core-280)）、解析の結果は全体像の出力に使っていない。
そのため、UTF-8 でないテストのファイルのようにテストの側に問題があると、全体像を書き出せずに止まっていた。この停止は [REQ-core-278](../../ir/core/overview-data.md#REQ-core-278) に書かれていない。
この停止を仕様にするか、中身を読まないようにするかを、[issue #41](https://github.com/ba0918/kotowari/issues/41) で決める。

Position: 利用者が推奨の案を採って決めた（2026-10-23）。

## Agreements

- A1 "kotowari overview build" と "kotowari overview serve" は、`テストのファイル`のパスだけを集め、その中身と "tests.rules" を読まない。パスは [REQ-core-199](../../ir/core/guides.md#REQ-core-199) と [REQ-core-280](../../ir/core/overview-data.md#REQ-core-280) の重なりの判定に使う。パスを集める走査での`停止`（[REQ-core-018](../../ir/core/config.md#REQ-core-018)）は今のまま残す
  - why: 全体像の出力はテストの解析の結果を使わない。使わない入力の問題で書き出しが止まると、利用者には止まる理由が分からない。テストの側の問題は check が指摘する。読まない分だけ書き出しも速くなる（時間は測っていない）
  - decided_by: 利用者（推奨を採用）

## Rejected

- A2 テストの側の問題で止まることを IR に書き、今の振る舞いを仕様にする
  - why: 使わない入力のために書き出しを止める理由が無い。仕様にすると、使わない解析の時間も残り続ける
