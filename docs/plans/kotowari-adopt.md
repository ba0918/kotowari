# Plan: 既存プロジェクトへの導入のスキル kotowari-adopt を作る

## Goal

既存のプロジェクトに途中から kotowari を入れる人が、kotowari-using-workflow の案内で kotowari-adopt に入り、話題1つずつ旧資料と実装とテストを仕分けて IR と判断の記録と FLAG に書き、承認後の作業を依頼文で受け取れる。

## Specification

IR は `docs/ir/`。決定は `docs/decision/records/2026-09-26-adoption.md`（A1〜A40。superseded_by の付いた A3、A6、A11、A14、A15、A16、A17、A22 は後の決定に置き換わっているので、置き換え先を読む）。この計画が扱うのは次のとおり。

- 要求: `docs/ir/core/adoption.md#REQ-core-214`、`#REQ-core-215`、`#REQ-core-216`、`#REQ-core-217`、`#REQ-core-218`、`#REQ-core-219`、`#REQ-core-220`、`#REQ-core-221`
- 決定表: `docs/ir/core/adoption.md#TBL-core-037`（REQ-core-216 の定義）
- シナリオ: EX-core-400〜EX-core-406（すべて adoption.md）
- 用語: `docs/ir/core/CONTEXT.md` の「旧資料」

要求の本文は `kotowari query <ID>` で読む（`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- query <ID>`）。本文と how_to_verify の行（`jq -r '.items[0].how_to_verify'`）の両方が、書く内容の出どころである。要求はすべて検証が review なので、テストも印も要らない。できあがりは how_to_verify の行に書かれたことがスキルの文面にあるかを読んで確かめる。

## Approach and why

- 新しいスキルは `agent/skills/kotowari-adopt/SKILL.md` に書く。ほかの工程のスキル（`agent/skills/kotowari-brainstorm/SKILL.md` など）と同じ形にする。frontmatter は name と description の2つで、description は英語で書き、「Use only in a repository that uses kotowari (one that has `.kotowari/` or `docs/ir/`).」と「日本語キーワード:」を含める。本文も英語で書く。工程のスキルがすべてそうなっているため。
- スキルは IR の書き方、判断の記録の書き方、check の読み方を自分では持たず、kotowari スキルの references（`ir-form.md`、`records.md`、`findings.md`、`config.md`）を場面ごとに名指しして読ませる。工程のスキルは kotowari スキルに一方向に依存し、書き方を二か所に持たない（`agent/skills/README.md` の「関係」の節）。
- 承認は kotowari-brainstorm の「Finishing」の手順3（「Approve in this order」: check、照合レビュー、ステージして提示）を名指しして使う（REQ-core-214）。brainstorm の手順1（敵対的レビュー）と手順2（plan に渡す条件）は名指ししない。決定 A1 が名指しするのは承認の手順だけで、adopt は決めない件を FLAG に残して進むため、brainstorm の「木を尽くす」規則と噛み合わない。
- TBL-core-037 の20行は、行き先を変えずにスキルに写す。表のまま写すか、同じ行き先の文で書くかは実装者が選んでよい。どちらにしても、表の上の2文（「旧資料の記述」は既存の IR の要求を含まないこと、FLAG はどの kind でもその FLAG だけの決定を出典にすること）も写す。
- 本文がスキルの読み込みの費用として長すぎると感じたら、一覧の列や判断の記録の例のような細部を `agent/skills/kotowari-adopt/references/` に分けてよい。SKILL.md から名指しして読ませる。
- 入口の表（`agent/skills/kotowari-using-workflow/SKILL.md` の「The entry table」）に1行足す。振るのは REQ-core-214 の2種類の依頼で、既存の行より前に置くか後に置くかは表の読み方が変わらない位置にする。
- スキルの数を書いている箇所（`agent/skills/README.md` の「9つ」「8つ」と表、`PROJECT.md` の「nine」「eight」と一覧）を10と9に直し、表に kotowari-adopt の行を足す。仕様の要求ではなく、数と一覧が実物と食い違わないようにするこの計画の判断である。

## Scope of change

- `agent/skills/kotowari-adopt/`（新設。SKILL.md と、分けるなら references/）
- `agent/skills/kotowari-using-workflow/SKILL.md`
  - 「The entry table」の節と、その説明の文だけ
- `agent/skills/README.md`
- `PROJECT.md`
  - 「What this is」の節のスキルの数と一覧だけ

## Step order and prerequisites

S1 が先。S2 は S1 で作ったスキルの名前を入口と一覧に載せる。スキルの名前は仕様で "kotowari-adopt" と決まっているので、S1 と S2 の間で名前が食い違うことはない。S3 が全体を確かめる。

## Verification map

| Step | Requirements | Examples |
|---|---|---|
| S1 | REQ-core-214（スキルの側）、REQ-core-215、REQ-core-216、TBL-core-037、REQ-core-217、REQ-core-218、REQ-core-219、REQ-core-220、REQ-core-221 | EX-core-400〜EX-core-406 |
| S2 | REQ-core-214（入口の側） | なし |
| S3 | この表のすべて | この表のすべて |

## Left to the implementer

- SKILL.md の節の分け方と見出し、references に分けるかどうかとその名前
- TBL-core-037 を表で写すか文で写すか（行き先は変えない）
- 入口の表に足す行の位置と言い回し（振る依頼の2種類は変えない）

## Stop conditions

- how_to_verify の行と要求の本文が食い違っていて、どちらに合わせるか決められない
- kotowari スキルの references に、adopt が名指しする手順（`config.md` の準備の手順、`records.md` の判断の記録の形）が見つからない、または仕様の前提と違う
- 仕様に書かれていない振る舞い（人に聞く回数を増やす、テストに手を加える、kotowari 本体を変える）が要るように見える

## Test command

```sh
CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text
```

## Out of scope

- kotowari 本体（CLI）の変更（決定 A7、A18）
- kotowari-brainstorm の変更（決定 A12）
- 手元の `~/.claude/skills/` へのシンボリックリンクの追加。リポジトリの外なので、マージの後に主セッションが `agent/skills/README.md` の手順で行う
- remote-merge での試用

## Steps

### S1: kotowari-adopt のスキルを書く

- Purpose: 話題1つの導入を、範囲の確認、仕分け、一覧のラウンド、IR と判断の記録の書き出し、承認、依頼文の受け渡しの順で進めるスキルを書く
- Specification: `docs/ir/core/adoption.md#REQ-core-214`, `docs/ir/core/adoption.md#REQ-core-215`, `docs/ir/core/adoption.md#REQ-core-216`, `docs/ir/core/adoption.md#REQ-core-217`, `docs/ir/core/adoption.md#REQ-core-218`, `docs/ir/core/adoption.md#REQ-core-219`, `docs/ir/core/adoption.md#REQ-core-220`, `docs/ir/core/adoption.md#REQ-core-221`
- Prerequisites: none
- May change: `agent/skills/kotowari-adopt/`
- Done when: `agent/skills/kotowari-adopt/SKILL.md` があり、REQ-core-214（スキルの側の2点: スキルがあること、承認を kotowari-brainstorm の承認の手順を名指しして行うこと）と REQ-core-215〜REQ-core-221 の how_to_verify の行に挙がる事柄がすべて書かれていて、TBL-core-037 の20行が同じ行き先で書かれ、EX-core-400〜EX-core-406 の状況をスキルどおりに進めるとシナリオの Then になる
- Shown by: artifact — `agent/skills/kotowari-adopt/SKILL.md`（と references）。要求ごとに how_to_verify の行の事柄をスキルの文面の該当箇所と対にして挙げ、TBL-core-037 の行ごとに写した箇所を挙げる
- Left to the implementer: 節の分け方、references に分けるかどうか、表で写すか文で写すか
- Stop and hand back if: how_to_verify の事柄のどれかが、ほかの要求や TBL-core-037 と両立しない書き方でしか書けない

### S2: 入口とスキルの一覧に kotowari-adopt を載せる

- Purpose: kotowari-using-workflow が導入の依頼を kotowari-adopt に振り、スキルの数と一覧が実物と合うようにする
- Specification: `docs/ir/core/adoption.md#REQ-core-214`
- Prerequisites: S1
- May change: `agent/skills/kotowari-using-workflow/SKILL.md`, `agent/skills/README.md`, `PROJECT.md`
- Done when: 入口の表に、既存の仕様やコードを IR に取り込みたい依頼と IR が実装と合っているか分からない依頼を kotowari-adopt に振る行があり、`agent/skills/README.md` と `PROJECT.md` のスキルの数が10（工程のスキルは9）で、両方の一覧に kotowari-adopt がある
- Shown by: check — `rg -n 'kotowari-adopt' agent/skills/kotowari-using-workflow/SKILL.md agent/skills/README.md PROJECT.md` で3つのファイルに当たりがあり、`rg -n '9つ|8つ' agent/skills/README.md` と `rg -n 'nine|eight workflow' PROJECT.md` で古い数が残っていない
- Left to the implementer: 入口の表の行の位置と言い回し
- Stop and hand back if: 入口の表の既存の行が導入の依頼を既に別の工程に振っていて、足す行と食い違う

### S3: 全体を確かめる

- Purpose: 仕様の検査とリポジトリのテストがこの変更で壊れていないことを確かめる
- Specification: `docs/ir/core/adoption.md#REQ-core-214`, `docs/ir/core/adoption.md#REQ-core-221`
- Prerequisites: S1, S2
- May change: none
- Done when: `kotowari check` がこのブランチの変えたファイルとこの計画の ID に誤りを出さず、`cargo test --workspace` が通る
- Shown by: check — `CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の後に `CARGO_BUILD_JOBS=4 cargo test --workspace`
- Left to the implementer: none
- Stop and hand back if: `cargo test --workspace` がこの変更と関係の無い理由で落ちる
