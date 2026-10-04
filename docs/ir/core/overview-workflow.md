# 全体像を作る工程

"agent/skills/" の下の kotowari のスキルが、壁打ちの承認で`全体像`を作るか直して見せるまでの手順を扱う。スキルの文面を人か LLM が読んで確かめる要求だけを持ち、kotowari 本体の振る舞いは overview-data.md と overview-commands.md が扱う。

## Requirements

### REQ-core-300: 承認で全体像を作って示す

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A2, docs/decision/records/2026-10-02-whole-picture.md#A8, docs/decision/records/2026-10-02-whole-picture.md#A34, docs/decision/records/2026-10-02-whole-picture.md#A52, docs/decision/records/2026-10-02-whole-picture.md#A84
- verification: review
- how_to_verify: "agent/skills/kotowari-brainstorm/SKILL.md" の承認の手順を読み、check と照合レビューの後に、関わる`全体像`を "ir" の一覧で決めること、`全体像の元データ`を作るか直すこと、"kotowari overview build" を走らせて誤りを直すこと、serve を起動して URL を参考として示し裏で起動できなければコマンドを示すことが、この順で書いてあり、承認の対象と承認の材料を変えていないことを確かめる

kotowari-brainstorm は常に、"overview" の鍵を持つプロジェクトの承認で、今の check と照合レビューの後に、変えた`IR`の`話題ごとの文書`から関わる`全体像`を`全体像の元データ`の "ir" の一覧で決め、その`全体像の元データ`を作るか直し、"kotowari overview build" を走らせて誤りを直し、"kotowari overview serve" を起動して URL を参考として示す。`全体像`は承認の対象にせず、見るかどうかは人に委ねる。

### REQ-core-301: 全体像の単位を決める

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A18, docs/decision/records/2026-10-02-whole-picture.md#A20, docs/decision/records/2026-10-02-whole-picture.md#A35
- verification: review
- how_to_verify: "agent/skills/kotowari-brainstorm/SKILL.md" か kotowari のスキルの references を読み、どの`全体像の元データ`の "ir" にも無い`話題ごとの文書`を変えたときに、既存の`全体像`に足すか新しく作るかの案を出して人に決めてもらうこと、単位の目安が使う人が1つの機能として名前で呼べるもので横断的なものを単独にしないこと、決めた単位を壁打ちで決めない限り変えないこと、触れていない話題の`全体像`をまとめて作らないことが書いてあることを確かめる

kotowari のスキルは常に、どの`全体像の元データ`の "ir" にも無い`話題ごとの文書`を壁打ちで変えたとき、既存の`全体像`に足すか新しく作るかの案を出して人に決めてもらい、単位を使う人が1つの機能として名前で呼べるものにし、決めた単位は壁打ちで決めない限り変えず、壁打ちが触れていない話題の`全体像`を作らない。

### REQ-core-302: 元データの書き方

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A1, docs/decision/records/2026-10-02-whole-picture.md#A6, docs/decision/records/2026-10-02-whole-picture.md#A7, docs/decision/records/2026-10-02-whole-picture.md#A8, docs/decision/records/2026-10-02-whole-picture.md#A9, docs/decision/records/2026-10-02-whole-picture.md#A14, docs/decision/records/2026-10-02-whole-picture.md#A15, docs/decision/records/2026-10-02-whole-picture.md#A54
- verification: review
- how_to_verify: kotowari のスキルの references に`全体像の元データ`を書く場面があり、冒頭に結論と要約の lead を置くこと、`部品`の種類と並べ方を題材に合わせて選ぶこと、出来上がるものの姿と判断のつながりの両方を表すこと、今の状態と`後回し`の予定を表すこと、前回の`全体像の元データ`を直して書くこと、節の見出しの隣に`ガイドの印`を付けること、`部品`の参照を "refs" か "ref" の欄に書くことが書いてあることを確かめる

kotowari のスキルは常に、`全体像の元データ`を書く場面を持ち、その場面で、冒頭に結論と要約を置き、図・表・装飾の`部品`を題材に合わせて選んで並べ、出来上がるものの姿と判断のつながりの両方と、今の状態と予定を表し、前回の`全体像の元データ`を直して書き、節ごとに`ガイドの印`を付けることを求める。

### REQ-core-303: cycle は全体像を直さない

- kind: ubiquitous
- source: docs/decision/records/2026-10-02-whole-picture.md#A39
- verification: review
- how_to_verify: "agent/skills/kotowari-cycle/" と "agent/skills/kotowari-implement/" を読み、`全体像の元データ`を直す手順が無いこと、`全体像の元データ`の guide_stale を次にその話題に触れた壁打ちが直すと kotowari のスキルの references に書いてあることを確かめる

kotowari のスキルは常に、cycle と実装の工程で`全体像の元データ`を直さず、`全体像の元データ`の guide_stale を次にその話題に触れた壁打ちで直す。
