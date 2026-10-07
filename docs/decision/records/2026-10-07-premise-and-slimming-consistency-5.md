# 修正の近くに新しい指摘が続く合図を、cycle の判断として書き直す

## Context

[A25](./2026-10-06-premise-and-slimming.md#A25) は、前提を疑う段に入る合図を、記録ファイルから計算する手順から cycle の判断に取り替えた。
整合のフェーズの5回目の実行で、合図を書いた cycle の本文、記録ファイルの形、変更履歴をこの決定と読み合わせた。
本文はまだ計算の手順として書かれ、その手順のためだけの細則（1回目から4回目で足したもの）が残っていた。
ここで、何を消し何を残すかを決める。

1回目から4回目の決定の根拠を読み直すと、[A1（1回目）](./2026-10-06-premise-and-slimming-consistency.md#A1) と [A5（1回目）](./2026-10-06-premise-and-slimming-consistency.md#A5) の根拠は今の本文でも成り立つ（agent/skills/kotowari-cycle/references/premise-step.md の The step の 4 の最後の段落と Once per firing）。
2回目から4回目の根拠は、計算の手順を記録ファイルに当てる使い捨てのスクリプトの出力で、A25 の下では手順そのものが無くなるので、それらの決定は A25 で取り替えたと記録した。

Position: 整合のフェーズの5回目の実行で決めた（2026-10-07）。

## Agreements

- A1 cycle の本文の Overlap after a fix は、修正のすぐ後のレビューが、その修正を入れた場所（直した指摘の根拠と、コミットが変えた場所）の近くに新しい見える指摘を上げたかを、cycle が根拠の位置（ファイルと行）から判断すると書き、同じファイルで行範囲が重なることは例として示すだけにする。計算のための細則（上がった指摘を最初の評価のラウンドで選ぶ、見える指摘を開閉を問わず処置の種類で読む、行範囲の無い根拠は加わらない、列を切る3つの場合）は消す。終わり方 3 の条件も同じ言い方にする
  - why: A25 は合図を記録ファイルの項目から計算する手順にしないと決めた。細則はどれも計算のための端の場合で、判断として書けば要らない。規則を足すより消すほうを選ぶ（[A13](./2026-10-06-premise-and-slimming.md#A13)、[A15](./2026-10-06-premise-and-slimming.md#A15)）。判断する時点では新しい指摘は開いているので、見える指摘は Loop の後の段落の定義のまま読める
  - decided_by: 整合のフェーズ（根拠: ae3df95 の agent/skills/kotowari-cycle/SKILL.md 144〜152 行と 174〜175 行は記録ファイルから計算する手順だった。直した後の同じファイルの Overlap after a fix と Endings の 3。`CARGO_BUILD_JOBS=4 cargo test --test step7_skill_references` は 3 passed、`kotowari check` の指摘は範囲の外の IR の文書の too_many_lines と too_many_requirements の notice だけ。直しはコミット 16e370c と e2db932）

- A2 修正ごとの記録 `fixes` は、fixer の返りごとに、直した指摘、コミット、その後のレビューのラウンド（`reviewed_in`）を持つ事実の記録とし、再開と最後の報告に使う。フックで止まった返りの数え方、整合のフェーズの回を `reviewed_in` から除く但し書き、整合のフェーズ自身の修正を足さない但し書きは消す
  - why: A25 は修正ごとの記録を事実として残すと決めた。これらの但し書きは記録から合図を計算するときにだけ結果を変え、判断の材料として読むときには何も変えない。「レビュー」と「fixer の返り」という言い方が、整合のフェーズの回とその修正をすでに含まない。整合のフェーズの状態は記録ファイルに `first_review_head` だけという [REQ-core-363](../../ir/core/consistency-phase.md#REQ-core-363) とも、`fixes` が fixer の記録であることで食い違わない
  - decided_by: 整合のフェーズ（根拠: ae3df95 の agent/skills/kotowari-cycle/SKILL.md 87〜91 行と 133〜135 行、agent/skills/kotowari-review/references/finding-schema.md 62 行。直した後に `rg -n "fixes|reviewed_in|hook" agent/skills/kotowari-cycle/SKILL.md agent/skills/kotowari-review/references/finding-schema.md` で、フックで止まった返りを `fixes` で数える文が残っていないことを確かめた。直しはコミット 16e370c）

- A3 新しい指摘の evaluations の最初の項目をその指摘を上げたラウンドの still_present とする決まり（[A4（1回目）](./2026-10-06-premise-and-slimming-consistency.md#A4)）は残す
  - why: 合図の計算には使わなくなったが、終わり方 3 の最初の条件は上がったラウンドから still_present を数え、[A11](./2026-10-06-premise-and-slimming.md#A11) も review-panel の #4 をその読みで扱う。1回目が数えた記録ファイルの形の揃い方も A22 に依らない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-review/references/finding-schema.md の evaluations の行と agent/skills/kotowari-cycle/SKILL.md の Endings の 3）
