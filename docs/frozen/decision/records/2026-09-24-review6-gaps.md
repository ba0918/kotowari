# 壁打ちの記録: 6回目の外部レビューで見つかった読み取りコマンドの仕様の穴

## Context

6回目の外部のレビュー（GPT、2026-09-24。範囲は `list.rs`、`query.rs`、`status.rs` と、変異テストの結果を読む3つのファイル）で、IR が決めていない振る舞いが2つ見つかった。同じ `ID` の`シナリオ`が2つ以上あるときに "kotowari list" の "examples" と "kotowari status" がどれを数えるか（[TBL-core-026](../../ir/core/list.md#TBL-core-026)）と、1つの`項目`が同じ `ID` を2回指したときの "kotowari query" の "referenced_by" の件数（[TBL-core-027](../../ir/core/query.md#TBL-core-027)）である。どちらも利用者が「ほぼ答えが決まっているものは推奨で決めてよい」とした範囲で決める。

Position: A1 と A2 を決めた。承認済み（2026-09-24）。

## Agreements

- A1 同じ `ID` の`シナリオ`が2つ以上あるとき、"kotowari list" の "examples" と、それに基づく "kotowari status" の数え上げは、[REQ-core-032](../../ir/core/findings.md#REQ-core-032) の1つ目の`シナリオ`だけを数える。"kotowari query" は今までどおり全部を出す。今の振る舞いを IR に書く
  - why: `ID` の重複はそれ自体が duplicate_id の`誤り`で、"kotowari check" の網羅の判定も1つ目だけを数えている。"list" と "status" もそれに揃える。"query" は重複を直す人が全部を見られるように全部出す（[REQ-core-156](../../ir/core/query.md#REQ-core-156)）
  - rejected: 2つ目以降の`シナリオ`の "@about" も数える。"check" の網羅の判定と食い違う
  - decided_by: 利用者（推奨を採用）

- A2 "kotowari query" の "referenced_by" は、1つの`項目`か`シナリオ`が同じ `ID` を同じ via で何度指しても1件にする
  - why: "referenced_by" はどこから指されているかを引くためのもので、同じ場所の同じ件が並んでも読み手に新しいことを伝えない。via が違えば指し方が違うので別の件のままにする
  - rejected: 指した出現ごとに1件出す。同じ行が重なって並ぶ
  - decided_by: 利用者（推奨を採用）
