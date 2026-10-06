# 前提を疑う段の書き方を、壁打ちの記録と読み合わせて決める

## Context

前提を疑う段を足し工程の skill を校正した変更（[壁打ちの記録](./2026-10-06-premise-and-slimming.md)）を、整合のフェーズとしてその記録と読み合わせた。
実装は、記録が書いていない3つのこと（取り替えの1回の枠がいつ戻るか、重なりの判定に使えない修正をどう数えるか、「修正のすぐ後のレビュー」がどのラウンドか）を決めていた。
また、consistency の指摘で発火したときの渡し先が cycle の決まりとぶつかっていた。重なりの合図を記録ファイルだけで判定するには、指摘が上がったラウンドが記録ファイルに残る必要があるが、その書き方が決まっていなかった。
ここでそれぞれを、記録と食い違わない形に決める。

Position: 整合のフェーズの1回の実行で決めた（2026-10-06）。

## Agreements

- A1 前提の取り替えの1回の枠は、cycle の再開でも、終わり方の後の「run more」でも戻らない。戻るのは同じブランチで kotowari-iterate を新しく始めたときだけとする
  - why: [A4](./2026-10-06-premise-and-slimming.md#A4) は取り替えた後にまた止める条件が揃ったら止めると書き、再開や run more はその「取り替えた後」に当たる。[A20](./2026-10-06-premise-and-slimming.md#A20) は再開で同じ発火を2回試さないために試した記録を残すとし、枠が再開で戻る読みを支えない。iterate を新しく始めるのは新しい依頼で、終わり方 3 の続き具合もそこで数え直すので、記録とぶつからない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/premise-step.md の Once per firing、agent/skills/kotowari-cycle/SKILL.md の Resuming and running more、agent/skills/kotowari-iterate/SKILL.md の The loop の最後の段落）

- A2 見える指摘を直したと報告しなかった修正と、直した指摘の根拠に行範囲が無い修正は、重なりの判定を2回続けて数える列を切る。飛ばして前後をつながない
  - why: [A22](./2026-10-06-premise-and-slimming.md#A22) は重なりが修正をまたいで2回「続く」ことを合図とし、根拠の位置が空の指摘を判定に使わないとする。判定に使えない修正を飛ばすと、続いていない2回を続いたと数える。review-panel の記録に当てると、切る読みでも飛ばす読みでも #7 で発火し、#10 と #11 では発火しないので、記録が示す場面は変わらない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Overlap after a fix。.agents/artifacts/reviews/review-panel.json に書いたとおりの規則を当てる使い捨てのスクリプトの出力「fix of [5] -> review round 3: raised [7] overlaps [(7, 5)] run=2 FIRE」「fix of [9] -> review round 6: raised [10, 11] overlaps [] run=0」）

- A3 「修正のすぐ後のレビュー」は、修正が直した指摘が最後に still_present だったラウンドより後で、quality の指摘を評価したか上げた最初のラウンドとする。整合のフェーズの回は数えない
  - why: [A22](./2026-10-06-premise-and-slimming.md#A22) が言うのはレビューで、修正の後の最初のラウンドをそのまま取ると、フックで止まった修正の後に挟まる整合のフェーズの回を選び、その後のレビューで上がった重なる指摘を見落とす。フェーズの回は consistency の指摘しか評価も上げもしないので、ラウンドの種類を記録ファイルに足さずに見分けられる
  - decided_by: 整合のフェーズ（根拠: 使い捨ての記録ファイル（指摘 1 をラウンド 1 で上げて直し、ラウンド 2 がフェーズの回、ラウンド 3 のレビューが指摘 1 と重なる指摘 3 を上げる）に当てると、直す前の書き方は「fix of [1] -> review round 2: raised [2] overlaps [] run=0」、直した後は「fix of [1] -> review round 3: raised [3] overlaps [(3, 1)] run=1」。review-panel.json では直した後も #7 で発火し #10 と #11 で発火しない。直しはコミット a501af6 と 5b13c7e）
  - superseded_by: [A1（3回目）](./2026-10-06-premise-and-slimming-consistency-3.md#A1)

- A4 新しい指摘の evaluations の最初の項目は、その指摘を上げたラウンドの still_present とする
  - why: 重なりの合図は、指摘を上げたラウンドを最初の評価のラウンドから読む。.agents/artifacts/reviews/ の19の記録ファイルのうち16はこの形をとるが、mutants.json は評価が空、overview-index.json は raised と書き、parse-links-resume.json には最初が no_longer_visible のものがあり、書き方が決まっていないと [A22](./2026-10-06-premise-and-slimming.md#A22) の「記録ファイルだけで判定できる」が成り立たない。[A11](./2026-10-06-premise-and-slimming.md#A11) は review-panel の #4（上がったラウンドと次のラウンドで still_present）を2ラウンド続く例として読んでおり、この形と合う
  - decided_by: 整合のフェーズ（根拠: 各記録ファイルの最初の評価を数えた jq の出力。直しはコミット 8e962bd）

- A5 consistency の指摘で前提を疑う段に入ったときは、前提が実装の側にあっても IR や判断の記録の側にあっても、根拠と実測で決められるものは整合のフェーズに渡す。fixer には渡さない
  - why: agent/skills/kotowari-cycle/SKILL.md の Loop は consistency の指摘を整合のフェーズにだけ渡し、fixer にもレビューにも渡さないと書く。前提を疑う段の「実装の側なら fixer」をそのまま当てると、この決まりとぶつかる。[A2](./2026-10-06-premise-and-slimming.md#A2) は fixer と整合のフェーズの分け方を決めたが、フェーズの指摘で発火した場面は扱っていない。整合のフェーズはコードも直すので、渡し先を変えても取り替えは試せる
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Loop の後の段落と Consistency phase の What cycle does with the result、agent/skills/kotowari-cycle/references/premise-step.md の The step の 4。直しはコミット d2c75b6）
