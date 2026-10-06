# 重なりの合図の見える指摘と、フックで止まった修正の記録を決める

## Context

整合のフェーズの4回目の実行で、3回目に決めた修正ごとの記録（[A1（3回目）](./2026-10-06-premise-and-slimming-consistency-3.md#A1)）を、記録ファイルを読み書きする各箇所と、その根拠に挙げた使い捨てのスクリプトに突き合わせた。
3回目の根拠のスクリプトは、見える指摘を最終の処置だけで選び、開いているかどうかを見ていなかった。cycle の本文は見える指摘を開いている指摘と定義しており、本文どおりに読むと合図が成り立たない。
また、フックで止まった修正を修正の記録に何件と数えるかが決まっていなかった。
ここでこの2つを、[A22](./2026-10-06-premise-and-slimming.md#A22) が示す場面（review-panel の #7 で発火し、#10 と #11 で発火しない）を保つ形に決める。

Position: 整合のフェーズの4回目の実行で決めた（2026-10-06）。

## Agreements

- A1 重なりの合図で言う見える指摘は、最終の処置が auto_fix か fix_and_verify の指摘とし、開いているか閉じているかを問わない
  - why: [A9](./2026-10-06-premise-and-slimming.md#A9) と [A22](./2026-10-06-premise-and-slimming.md#A22) は見える指摘を「直す対象の指摘」、つまり処置の種類で言う。合図は記録ファイルから読み直すので、開いている指摘に限ると、修正が直した指摘はすぐ後のレビューで閉じ、そのレビューが上げた指摘も次の修正で閉じて、重なる組がどちらの側にも残らない。3回目の根拠（[A1（3回目）](./2026-10-06-premise-and-slimming-consistency-3.md#A1)、[A2（3回目）](./2026-10-06-premise-and-slimming-consistency-3.md#A2)）は処置の種類で読んだときにだけ成り立つので、本文をその読みに合わせる
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Loop の後の段落の Visible findings の定義と Overlap after a fix。3回目の使い捨てのスクリプト recorded.py を、見える指摘に開いていることも求めるよう2行だけ変えた literal.py と並べて走らせた。3回目の review-panel.json の写し（`fixes` 付き）では、recorded.py が「fix of [5] -> round 3: raised [7] overlaps [(7, 5)] run=2 FIRE」、literal.py はすべての修正で「raised [] overlaps [] run=0」。ラウンド 3 を記録した直後の状態に戻した写し（#5 がラウンド 3 で no_longer_visible で閉じ、#7 が開いている）でも、recorded.py は「fix of [5] -> round 3: raised [7] overlaps [(7, 5)] run=2 FIRE」、literal.py は「fix of [1, 2, 3] -> round 2: raised [] overlaps [] run=0」「fix of [5] -> round 3: raised [7] overlaps [] run=0」。3回目の合成の記録 twofix、failoverlap、synth の写しを recorded.py で読み直すと、3回目の記録に書いた出力と同じだった。直しはコミット a9c9299）

- A2 修正の委譲の返りがフックで止まったときは、それを修正に数えず `fixes` に足さない。整合のフェーズの後にやり直した同じ委譲の返りを1回の修正とし、それに1件だけ足す
  - why: agent/skills/kotowari-cycle/references/editing-contract.md の Fixer only は、フックで止まった fixer がコミットを持たずに指摘を返すとし、cycle の Delegations はその後に整合のフェーズを挟んで同じ委譲をやり直すとする。止まった返りを直した指摘の無い修正として足すと列が切れて A22 の場面を逃し、直した指摘付きで足すと1回の重なりを2回と数えて誤って発火する。どちらも [A22](./2026-10-06-premise-and-slimming.md#A22) の「修正をまたいで2回続く」と合わない
  - decided_by: 整合のフェーズ（根拠: ラウンド 3 を記録した直後の review-panel.json の写しの `fixes` の間に、止まった返りの件を挟んだ2つの合成の記録を recorded.py で読んだ。直した指摘の無い件を挟むと「fix of []: reported no finding run=0」「fix of [5] -> round 3: raised [7] overlaps [(7, 5)] run=1」で発火しない。最初の修正を [1, 3] にして重なりを1回だけにし、[5] の件を2つ並べると「fix of [5] -> round 3: raised [7] overlaps [(7, 5)] run=1」「fix of [5] -> round 3: raised [7] overlaps [(7, 5)] run=2 FIRE」で、1回の重なりで発火する。直しはコミット d772571）
