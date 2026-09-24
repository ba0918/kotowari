# 計画: レビューに別のモデルの任意の席を足す

## Goal

kotowari-review の full review で、利用者が自分の環境に書いた別のモデルの席が quality 観点の同じプロンプトで追加に立ち、落ちた席は欠席として報告に残ってレビューは続き、席の数は実行ごとに一言で上書きできる。

## Specification

この件に IR は無い（review-panel A1）。仕様は判断の記録 `docs/decision/records/2026-09-24-review-panel.md` の Agreements（A1〜A14）で、節の見出しは `## Agreements`。各ステップは決定の番号で指す。

## Approach and why

席の一覧は利用者スコープの指示ファイルに書かれる（A2）ので、kotowari-review の本文は「一覧があれば従う、無ければ1席」とだけ書き、ツールや ba0918 の skill を名指ししない（A2、A5）。振る舞いの大半（どのレビューに付けるか、渡すプロンプト、落ちたときと書き換えたときの扱い、時間の上限、統合）は review の呼び出し側の決まりなので、kotowari-review の `SKILL.md` に1つの節としてまとめて置く。cycle は full review を委譲する側なので、席の指定を引き継ぐことと終端報告に出欠を載せることだけを足す（A3、A8、A9）。

変更は工程の skill の本文だけで、コードとテストは変えない。レビューの本文が固まってから cycle がそれを参照するので、review → cycle → 案内の順にする。

## Scope of change

- `agent/skills/kotowari-review/SKILL.md`
- `agent/skills/kotowari-cycle/SKILL.md`
- `agent/skills/README.md`（利用者スコープの一覧の書き方の案内）

`references/finding-schema.md` は変えない（A9）。IR、判断の記録、コードは変えない。

## Step order and prerequisites

S1 → S2 → S3 → S4。S2 は S1 で足した節の名前を参照する。S3 は S1 の一覧の中身の説明を案内に写す。

## Verification map

| ステップ | 確かめる決定 |
|---|---|
| S1 | A2〜A14 |
| S2 | A3、A4、A8、A9、A13 |
| S3 | A2、A5、A6、A7 |
| S4 | A2、A5（名指しの禁止）、A9（finding の形を変えない）と、変更の範囲 |

IR が無いので、テストの印の検査は無い。各ステップは本文（artifact）で示し、独立したレビューが決定と突き合わせる。

## Left to the implementer

- 新しい節の見出しの名前と、`SKILL.md` の中の置き場所
- 利用者スコープの一覧の例の書き方（1席ぶんの例を示すこと。行の形は、席の名前、起動の手段、任意の時間の上限が読み取れれば問わない）
- 英語の言い回し（工程の skill の本文は英語。workflow-split A10）

## Stop conditions

- 本文に、特定のツール（codex、opencode など）や ba0918 の skill の名前を書かないと説明できないと分かったとき（A2、A5）
- 任意の席の扱いが、今の「Reviewers only evaluate」（kotowari-cycle）などの既存の規則と矛盾し、どちらかの意味を変えないと直せないとき。kotowari-review の「Two are never the default」は観点（quality と conformance）の数を指す文と読み、任意の席は quality の観点の追加（A10）なので矛盾しない。そう読めるように言い回しを直してよい
- finding の JSON の形（`references/finding-schema.md`）に手を入れないと統合できないと分かったとき（A9）

## Out of scope

- plan と brainstorm が Finishing で立てるレビュー、brainstorm の照合レビューに席を足すこと（A11）
- diff review に席を足すこと（A3）
- 利用枠の残りを事前に調べる仕組み
- 計画書の検査の件（別の計画 `docs/plans/plan-schema.md`）

## Steps

## Step 1 — kotowari-review に任意の席の節を足す

Purpose: 呼び出し側が任意の席を立て、落ちたときや書き換えたときにどう扱うかを1つの節に書く。Specification: `docs/decision/records/2026-09-24-review-panel.md` の A2〜A14。
Prerequisites: なし。
May change: `agent/skills/kotowari-review/SKILL.md`。
Done when: 本文から次が読み取れる。席の一覧は利用者スコープの指示ファイルにあり、無ければ1席（A2）。任意の席は full review にだけ付け（A3）、quality 観点と同じプロンプトを渡し、conformance には付けない（A10）。起動の手段はコマンドか skill で、本文は名指ししない（A5）。1回落ちたら試し直さず欠席とし理由を報告に残す。必須の席が落ちたら既存の規則のまま（A4）。任意の席は1つずつ順に起動し（A12）、その前後に `git status` を取り、差があればその席の指摘を捨てて欠席とし、書き換えは戻さずループを止めて人に見せる（A6、A13）。出力が finding の JSON として読めなければ「出力を読めない」で欠席（A14）。時間の上限は一覧に書かれたときだけ（A7）。席の数は人の一言か呼び出し側の1行の理由で上書きでき、無ければ一覧のとおり（A8）。指摘は今の統合と重複の除去でまとめ、finding の形は変えず、出席と欠席の席を報告に載せる（A9）。人が直接 review を呼んだときも同じ（A11）。Reviewer setup の「Launch one reviewer …」と矛盾しない。
Shown by: artifact — `agent/skills/kotowari-review/SKILL.md`。形式の検査は無い。独立したレビューが上の決定と突き合わせて判断できる状態にする。
Left to the implementer: 節の見出しと置き場所、Inputs の表に「席の指定」の行を足すかどうか。
Stop and hand back if: 上の Stop conditions に当たったとき。

## Step 2 — cycle が席の指定を引き継ぎ、出欠を報告する

Purpose: cycle が full review で席の指定に従って任意の席を立て、終端報告に出席と欠席の席を載せる。Specification: `docs/decision/records/2026-09-24-review-panel.md` の A3、A4、A8、A9、A13。
Prerequisites: S1。
May change: `agent/skills/kotowari-cycle/SKILL.md`。
Done when: Inputs の任意の入力に「席の指定（人の一言。既定は一覧のとおり）」があり、full review（1回目と2回目）では cycle 自身が呼び出し側として S1 の節に従って任意の席を立て（レビュー役の委譲に席の指定や S1 の節を渡さない）、diff review では立てないことが読み取れ、Stopping inside the loop に「任意の席の書き換えが見つかったら、security の指摘と同じくループを止めて人に見せ、返事で戻して続けるかそのまま続けるかを決める」（A13）があり、Terminal report に出席と欠席の席（欠席は理由つき）が載る。
Shown by: artifact — `agent/skills/kotowari-cycle/SKILL.md`。独立したレビューが決定と突き合わせる。
Left to the implementer: 本文の中の書き足す位置。
Stop and hand back if: 欠席を ending（終わり方）の条件に数えないと成り立たない記述が必要になったとき（A4 は必須の席だけを既存の規則に任せている）。

## Step 3 — 一覧の書き方を案内する

Purpose: 利用者が自分の指示ファイルに席の一覧を書けるよう、置き場と中身を案内する。Specification: `docs/decision/records/2026-09-24-review-panel.md` の A2、A5、A6、A7。
Prerequisites: S1。
May change: `agent/skills/README.md`。
Done when: README に、利用者スコープの指示ファイルに書くこと、書かなければ1席であること、1席ぶんに書くもの（名前、起動の手段としてのコマンドか skill、任意の時間の上限、読み取り専用で動かす約束）と、1席の例がある。
Shown by: artifact — `agent/skills/README.md`。
Left to the implementer: 例に使う起動の手段の書き方（実在のツール名を例に出してもよいが、kotowari-review の本文には書かない）。
Stop and hand back if: なし（Stop conditions のほか）。

## Step 4 — 変更の範囲を確かめる

Purpose: 名指しの禁止と変更の範囲が守られていることを確かめる。Specification: `docs/decision/records/2026-09-24-review-panel.md` の A2、A5、A9。
Prerequisites: S1〜S3。
May change: なし（見つかったものは該当するステップのファイルで直す）。
Done when: 下のコマンドがすべて期待どおりになる。
Shown by: check — `rg -n -i 'ba0918|codex|opencode' agent/skills/kotowari-review/SKILL.md agent/skills/kotowari-cycle/SKILL.md` が何も出さない。`git diff --name-only main...HEAD` が Scope of change の3ファイルだけを出す。`git diff main...HEAD -- agent/skills/kotowari-review/references/` が空。
Left to the implementer: なし。
Stop and hand back if: なし。
