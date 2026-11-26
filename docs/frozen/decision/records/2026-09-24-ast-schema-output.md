# 壁打ちの記録: "ast --schema" の出力の形を IR に書く

## Context

"kotowari-mds ast --schema" は判断の記録（[A46（mds-spec）](./2026-09-21-mds-spec.md#A46)、[A56（mds-spec）](./2026-09-21-mds-spec.md#A56)）と README にあるが、IR の [REQ-schema-005](../../ir/schema/cli.md#REQ-schema-005) は "ast" を素の構文木としてしか書いておらず、出力の形も、スキーマの最上位の "name" の鍵も要求に無かった。2回目の外部のレビュー（2026-09-24）で見つかり、TODO に積んでいた。今のエンジンの振る舞いを IR に書く。

Position: A1 と A2 を決めた。承認済み（2026-09-24）。

## Agreements

- A1 "kotowari-mds ast" に "--schema" を付けたとき、mds は`文書`が宣言した`スキーマ`で "values --format json" と同じ値を組み立て、JSON で出す。`スキーマ`が最上位の "name" を宣言していれば、根の "type" の鍵にその値を置く。"name" を宣言しないときは "type" を置かず、出力は "values --format json" と同じになる。今の振る舞いを書く
  - why: "ast --schema" は抽出の値に型の名前を添えたものとして作られていて（README、A46）、"values" と別の組み立て方を持たない。同じと書けば、抽出の規則（[REQ-schema-035](../../ir/schema/extraction.md#REQ-schema-035) など）がそのまま "ast --schema" にも効く
  - decided_by: 利用者（推奨を採用）

- A2 `スキーマ`の最上位の "name" は文字列の型の名前として受け、"ast --schema" の "type" にだけ使う。"check" と "values" の結果には影響しない
  - why: エンジンは "name" を "ast --schema" の "type" にしか使っていない。IR にこの鍵が無かったので、書かないと何のための鍵か分からない。"type" の鍵との衝突の停止（[A2（review2-gaps）](./2026-09-24-review2-gaps.md#A2)）もこの鍵を前提にしている
  - decided_by: 利用者（推奨を採用）
