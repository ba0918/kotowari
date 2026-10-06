# 指摘を直しきらなかった修正を、重なりの合図でどう数えるかを決める

## Context

整合のフェーズの2回目の実行で、1回目が決めた「修正のすぐ後のレビュー」の決め方（[A3（1回目）](./2026-10-06-premise-and-slimming-consistency.md#A3)）を、1つの指摘を2回の修正で直す場面に当てた。
その決め方は、修正が直した指摘が最後に still_present だったラウンドの後を見るので、1回目の修正と2回目の修正のどちらも、2回目の修正の後のレビューに当たる。
重なりが1回しか起きていないのに、修正をまたいで2回続いたと数え、前提を疑う段の後なら終わり方 3 で止まる。
ここで、指摘を直しきらなかった修正の数え方を決める。

Position: 整合のフェーズの2回目の実行で決めた（2026-10-06）。

## Agreements

- A1 修正が直した指摘のどれかの `commits` に、その修正の後に別の修正のコミットが続くとき、その修正は重なりの判定を2回続けて数える列を切る
  - why: [A22](./2026-10-06-premise-and-slimming.md#A22) は「その修正が直した見える指摘」との重なりを合図とし、修正の後も still_present のままの指摘はその修正が直していない。[A2（1回目）](./2026-10-06-premise-and-slimming-consistency.md#A2) は判定に使えない修正で列を切ると決めており、同じ扱いに加える。記録ファイルの `commits` の並びだけで判定でき、ラウンド番号を足さない。直しきらなかった指摘は終わり方 3 の still_present が2回続く条件で捕まるので、列を切っても合図の取りこぼしにはならない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Overlap after a fix。重なりの規則を当てる使い捨てのスクリプトで、指摘 1 をラウンド 1 で上げ、修正 1 の後のラウンド 2 で still_present、修正 2 の後のラウンド 3 で no_longer_visible と重なる指摘 2 が上がる記録を読むと、直す前の書き方は「fix of [1] -> review round 3: raised [2] overlaps [(2, 1)] run=1」「fix of [1] -> review round 3: raised [2] overlaps [(2, 1)] run=2 FIRE」、直した後は「fix of [1]: an addressed finding stayed still_present -> run=0」「fix of [1] -> review round 3: raised [2] overlaps [(2, 1)] run=1」。.agents/artifacts/reviews/review-panel.json では直す前も後も「fix of [5] -> review round 3: raised [7] overlaps [(7, 5)] run=2 FIRE」「fix of [9] -> review round 6: raised [10, 11] overlaps [] run=0」で、#7 で発火し #10 と #11 で発火しない。修正 1 の後のレビューも重なる指摘を上げた記録では、直した後は列が1で止まるが、指摘 1 がラウンド 1 と 2 で still_present なので still_present が2回続く条件がラウンド 2 で成り立つ。直しはコミット 10573bf）
