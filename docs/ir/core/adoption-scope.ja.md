# 導入の範囲と行の作り方

[English](adoption-scope.md) | 日本語

kotowari-adopt が1回の導入で扱う話題の範囲をどう決め、`旧資料`と実装とテストを読んで一覧の行をどう作るかを扱う。行の行き先は adoption.md の TBL-core-037 が、人に聞く回数は adoption.md の REQ-core-219 が扱う。スキルの文面を人か LLM が読んで確かめる要求だけを持つ。

## Requirements

### REQ-core-215: 導入の範囲と準備

- kind: ubiquitous
- source: docs/decision/records/2026-09-26-adoption.md#A2, docs/decision/records/2026-09-26-adoption.md#A19, docs/decision/records/2026-09-26-adoption.md#A23, docs/decision/records/2026-09-26-adoption.md#A27, docs/decision/records/2026-09-26-adoption.md#A28, docs/decision/records/2026-09-26-adoption.md#A33, docs/decision/records/2026-09-27-adopt-scope.md#A1, docs/decision/records/2026-09-27-adopt-scope.md#A5
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、次のすべてが書いてあることを確かめる。最初に ".kotowari/config.yaml" があるかを確かめ、無ければ kotowari スキルの "references/config.md" の準備の手順を名指しして先に行うこと。1回で扱う話題は1つで、範囲の確認のラウンドで人が選び、既定は次に変更する予定の機能であること。同じラウンドで、話題の利用者の入口（利用者が触るコマンドやサブコマンド、画面の操作）と、範囲のコードファイルとテストファイルの案と、読む`旧資料`の候補を人に確かめること。読んで仕分けるのは主セッションで、1回で読み切れない大きさなら別の agent に読ませず、範囲の確認のラウンドで話題を小さく割ることを人に提案すること

kotowari-adopt は常に、設定があることを確かめてから、範囲の確認のラウンドで話題1つとその利用者の入口と範囲のコードファイルとテストファイルと読む`旧資料`を人に確かめ、主セッションで読んで仕分ける。

### REQ-core-237: 行にする振る舞いの範囲

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-adopt-scope.md#A1, docs/decision/records/2026-09-27-adopt-scope.md#A2
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、次のすべてが書いてあることを確かめる。行にするのは話題の利用者の入口から観測できる振る舞いだけで、同じコードを共有していても別の入口から観測される振る舞いは範囲の外であること。理解のために範囲の外のファイルを読んでよいが、範囲の外の振る舞いは行にせず`判断の記録`にも書かないこと。一覧のラウンドで、範囲の外で見つかった振る舞いの件数と次の話題の候補の名前だけを見せること

kotowari-adopt は常に、話題の利用者の入口から観測できる振る舞いだけを行にし、範囲の外で見つけた振る舞いは件数と次の話題の候補の名前だけを見せる。

### REQ-core-238: 行の粒度

- kind: ubiquitous
- source: docs/decision/records/2026-09-27-adopt-scope.md#A3
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、一覧の1行が`要求`1つの候補であること、同じ規則で値や文言だけが違うものは1行にまとめて`決定表`で書く候補にし、一覧の実装の振る舞いの欄に表の中身を短く並べることが書いてあることを確かめる

kotowari-adopt は常に、一覧の1行を`要求`1つの候補にし、値や文言だけが違うものを1行にまとめる。

### REQ-core-239: 行が多いときの見直し

- kind: event_driven
- source: docs/decision/records/2026-09-27-adopt-scope.md#A4
- verification: review
- how_to_verify: "agent/skills/kotowari-adopt/" を読み、`要求`の候補の数が設定の "limits.requirements" を超えたら利用者の入口が混ざっていないかを見直すこと、混ざっていれば一覧のラウンドの中で話題を分けることを提案して人に今回の側を選んでもらうこと、混ざっていなければ数を理由に分けず`IR`を書く段で責務ごとに文書を分けることが書いてあることを確かめる

`要求`の候補の数が "limits.requirements" を超えたとき、kotowari-adopt は利用者の入口が混ざっていないかを見直し、混ざっていれば一覧のラウンドの中で話題を分けることを提案する。
