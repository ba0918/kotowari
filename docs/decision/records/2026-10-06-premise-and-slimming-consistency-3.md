# 修正のすぐ後のレビューを、推し量らずに記録から読む

## Context

重なりの合図（[A22](./2026-10-06-premise-and-slimming.md#A22)）の「修正のすぐ後のレビュー」は、これまで指摘の評価のラウンドから推し量っていた。
整合のフェーズの1回目（[A3（1回目）](./2026-10-06-premise-and-slimming-consistency.md#A3)）と2回目（[A1（2回目）](./2026-10-06-premise-and-slimming-consistency-2.md#A1)）は、どちらもこの推し量りに例外を足す直しだった。
3つ目の例外を足す前に、2つの直しが共通に置いた前提「修正のすぐ後のレビューは指摘の評価のラウンドから推し量れる」を試した。
前提が成り立たないことが実測で分かったので、推し量りをやめ、cycle が記録した事実を読む形に取り替える。

Position: 整合のフェーズの3回目の実行で決めた（2026-10-06）。

## Agreements

- A1 cycle は修正の報告を記録ファイルに書くとき、`fixes` に1件足し、その修正が報告した指摘とコミットを持たせる。次のレビュー（整合のフェーズの回は除く）を記録するとき、そのラウンドを `reviewed_in` が空の各件に書く。重なりの合図は `fixes` を順に読み、修正のすぐ後のレビューを `reviewed_in` のラウンドとする。整合のフェーズが自分で直した分は `fixes` に足さない
  - why: 記録ファイルは指摘ごとのコミットを持つが、どのコミットが1回の修正のものかを持たない。そのため、推し量る規則は記録ファイルの外にある fixer の報告を要し、[A22](./2026-10-06-premise-and-slimming.md#A22) の本文が求める「記録ファイルだけで判定できる」が成り立たない。評価のラウンドだけから修正を分ける読みは、1回目と2回目の例外を両方足さないと同じ場面を捕まえない。A22 の why はラウンド番号を足さないほうを選んでいたが、それは判定できると見た上での理由で、本文は記録ファイルだけで判定できることを求める。修正ごとに1件、ラウンドを1つ記録するだけで、推し量りとその例外が要らなくなる
  - rejected: 推し量りを残し、修正をラウンドで分けると本文に書く（評価のラウンドだけで修正を分ける読みは、下の根拠の synth.json でフェーズの回を、failoverlap.json で直しきらなかった修正を取り違えるので、1回目と2回目の例外を残したままになる）
  - rejected: 指摘ごとの `commits` を修正ごとの組に変える（fixer に前の修正のコミットを渡す決まりや、整合のフェーズが返すコミットの形まで変わる。`fixes` を足すほうが変える範囲が小さい）
  - decided_by: 整合のフェーズ（根拠: 1回目と2回目に使った使い捨てのスクリプト overlap.py は、修正の分け方を記録ファイルの外から引数で受け取っていた。これに .agents/artifacts/reviews/review-panel.json と、そのラウンド1の後のコミット e0778d3、9b91917、f5a8601 の分け方を3通り渡すと、1回の修正とみると「fix of [5] -> review round 3: raised [7] overlaps [(7, 5)] run=2 FIRE」、3回の修正とみると「fix of [5] -> review round 3: raised [7] overlaps [(7, 5)] run=1」「fix of [7] -> review round 4: raised [8] overlaps [(8, 7)] run=2 FIRE」で、同じ記録ファイルから発火する場所が #7 と #8 に分かれた。評価のラウンドだけで修正を分ける使い捨てのスクリプト replay.py は、review-panel.json では #7 で発火するが、フェーズの回を挟む合成の記録 synth.json では「fix after round 1: addressed [1]; raised at 2: [2]; overlaps []」で重なりを見落とした。取り替えた後の規則を `fixes` だけから読む使い捨てのスクリプトでは、review-panel.json に `fixes`（[1,2,3] はラウンド2、[5] は3、[7] は4、[9] は6、[10,11] は7。ラウンド1の3つのコミットは27秒の間に作られ、1回の修正とみた）を足した写しで「fix of [5] -> round 3: raised [7] overlaps [(7, 5)] run=2 FIRE」「fix of [9] -> round 6: raised [10, 11] overlaps [] run=0」「fix of [10, 11] -> round 7: raised [] overlaps [] run=0」となり、#7 で発火し #10 と #11 では発火しない。synth.json の写しは「fix of [1] -> round 3: raised [3] overlaps [(3, 1)] run=1」。直しはコミット f782a39）
  - superseded_by: [A25](./2026-10-06-premise-and-slimming.md#A25)

- A2 修正が直した指摘のどれかが `reviewed_in` のラウンドで still_present なら、その修正は重なりの判定を2回続けて数える列を切る
  - why: [A1（2回目）](./2026-10-06-premise-and-slimming-consistency-2.md#A1) の選んだこと（直しきらなかった修正は列を切る）は変えず、見分け方だけを、コミットの並びから推し量る形から、記録したラウンドの評価を読む形に替える。直しきらなかった指摘は、終わり方 3 の still_present が2回続く条件で捕まる
  - decided_by: 整合のフェーズ（根拠: 同じ使い捨てのスクリプトで、1つの指摘を2回の修正で直す合成の記録 twofix.json の写しは「fix of [1] -> round 2: an addressed finding still_present run=0」「fix of [1] -> round 3: raised [2] overlaps [(2, 1)] run=1」で、重なりを2回と数えない。failoverlap.json の写しも「fix of [1] -> round 2: an addressed finding still_present run=0」「fix of [1, 2] -> round 3: raised [3] overlaps [(3, 1), (3, 2)] run=1」。直しはコミット f782a39）
  - superseded_by: [A25](./2026-10-06-premise-and-slimming.md#A25)

## Revisions

- 1回目の A3 の「修正のすぐ後のレビュー」の推し量り方を、記録した `reviewed_in` を読む A1 で取り替えた
- 2回目の A1 の、直しきらなかった修正の見分け方を、`reviewed_in` のラウンドの評価を読む A2 で取り替えた。列を切ることは変わらない
- A1 と A2 の、`fixes` から重なりの合図を読む手順を、[A25](./2026-10-06-premise-and-slimming.md#A25) で cycle の判断に取り替えた。修正ごとの記録は、再開と最後の報告に使う事実として残る
