# 計画: kotowari の工程を ba0918 の workflow から切り出す

## Goal

kotowari を使うリポジトリで、ba0918 の工程の skill を1つも入れずに、kotowari 用の工程の skill 8つと kotowari スキルだけで brainstorm から cycle までが回る。

## Specification

`docs/decision/records/2026-09-23-workflow-split.md`（A1〜A11。この計画は IR を持たない題目で、仕様は判断の記録の決定そのもの。見出しの代わりに決定の番号 `#A1` の形で参照する）

## Approach and why

写す元は `/home/mizumi/develop/agentic-workflow/skills/` の `ba0918-*` の8つで、コミット `ce35193` の時点のものに固定する（A3 で以後は追従しないので、どの版から分けたかを1つに決めておく）。手元の `~/.claude/skills/ba0918-*` は写す元にしない（iterate が元と食い違っている）。

最初に8つを1バイトも変えずに写してコミットし、そのあとの変更をすべて差分として見えるようにする（A10 の「写す元との差分が kotowari 固有の部分だけ」を、レビューで確かめられる形にするため）。次に名前を置き換え、そのあと `skills/kotowari/references/workflow.md` の4つの節を1つずつ各工程の skill へ溶かし込む。`workflow.md` を消すのは、中身がすべて行き先を得た最後の段。

溶かし込むときの行き先は次のとおり。`workflow.md` の段落はどれか1つの行き先を持つ。

| workflow.md の節 | 中身 | 行き先 |
|---|---|---|
| brainstorm | 判断の記録の形（置き場とファイル名の形、ファイル名を変えないこと、`## Context` の節と Position の行、決定の節の見出し、1件の書き方、補足の行6つとその必須、補足の行が決定として読まれない理由、決定の節でない節の番号、資料をまたぐ参照をリンクにする規則、ADR の扱い）と、中身の種類ごとの置き場（成功の条件と反例は IR のシナリオ、禁止・却下・未決・委譲は記録の節、矛盾・欠落・曖昧は `FLAGS.md`、用語集の連鎖） | kotowari スキルの新しい reference（A11） |
| brainstorm | 流れ（出力が `docs/spec/` でなく IR と用語集と問題の記録であること、記録を最後にまとめず最初から置き場に書くこと、未コミットの記録からの再開、終わりのレビューの差し替え、stage するもの、承認の手順1〜3） | kotowari-brainstorm（A4、A11） |
| 全体 | 先頭の改訂日の行と、各節の最初の「既存の〜スキルの手順のうち、次を置き換える。」の4文 | 捨てる（上書きの形そのものの説明で、分けたあとは意味を持たない） |
| plan | すべて | kotowari-plan |
| cycle | すべて | kotowari-cycle（review に渡すものの段落は kotowari-review の入力の書き方にも合わせる） |
| implement | すべて | kotowari-implement |

線引きは「ファイルの形と、中身の種類ごとの置き場」を kotowari スキルへ、「いつ・誰が・どの順で」を工程の skill へ、とする。A11 の理由（kotowari スキルだけを入れて別の工程で使う人も記録と IR を正しく書ける）から、`check` が読む形や、kotowari スキルだけの人が知らないと書けない置き場は形の側に寄せる。

本文は英語（A10）。写した本文の文はそのまま残し、kotowari 固有の手順は英語に訳して置き換える箇所に書く。kotowari スキル（`skills/kotowari/`）は今までどおり日本語。

工程の skill から kotowari スキルの reference を読ませるときは、skill の名前と reference のファイル名で名指しする（A11 で認めた）。逆向き、つまり kotowari スキルから `kotowari-*` の工程の skill を名指しすることはしない（A5）。

Step 3〜7 の突き合わせで読む `workflow.md` は、Step 8 で消えたあとも、ブランチの起点（この計画をコミットしたコミット）の版を `git show <起点>:skills/kotowari/references/workflow.md` で読む。

## Scope of change

- `skills/kotowari-brainstorm/`、`skills/kotowari-plan/`、`skills/kotowari-cycle/`、`skills/kotowari-implement/`、`skills/kotowari-review/`、`skills/kotowari-iterate/`、`skills/kotowari-investigate/`、`skills/kotowari-using-workflow/`（新規）
- `skills/README.md`（新規。工程の skill の入れ方）
- `skills/kotowari/`（SKILL.md、references の追加と `workflow.md` の削除、`config.md`）
- `docs/spec/kotowari-skill.md`（A6 で書き換えを許されている）
- `AGENTS.md`、`PROJECT.md`（このリポジトリ自身の入口と説明）
- `~/.claude/skills/` の下のリンク（最後の段。人が実行する）

これ以外、とくに `/home/mizumi/develop/agentic-workflow/` と `~/.claude/skills/ba0918-*`、`docs/ir/`、Rust のソースとテストは変えない。

## Step order and prerequisites

1 → 2 → 3 → 4〜7 → 8 → 9 → 10。1 と 2 は写したものと名前を固定する土台で、3 は 4 が名指しする reference を先に作る。4〜7 は互いに独立で、どの順でもよい。8 は `workflow.md` を消すので 4〜7 がすべて終わってから。10 は手元の環境に触るので最後。

## Step 1 — 8つの skill を写す元のまま写す

Purpose: 以後の変更をすべて写す元との差分として見せる土台を作る。Specification: `docs/decision/records/2026-09-23-workflow-split.md#A1`、`#A3`、`#A4`。
Prerequisites: `/home/mizumi/develop/agentic-workflow` の HEAD が `ce35193` で、作業ツリーがきれいなこと。
May change: `skills/kotowari-*/`（新規の8つ）。
Done when: `ba0918-<席>/` の中身が `skills/kotowari-<席>/` に同じバイトで入っていて、それだけを1コミットにしている。コミットの本文に写す元のリポジトリとコミット `ce35193` を書く。
Shown by: check — `bash -c 'for s in brainstorm plan cycle implement review iterate investigate using-workflow; do diff -r /home/mizumi/develop/agentic-workflow/skills/ba0918-$s skills/kotowari-$s || echo DIFF $s; done'` が何も出さない（利用者のシェルは fish なので `bash -c` で包む）。
Left to the implementer: 写す手段（`cp -r`、`git archive` など）。
Stop and hand back if: agentic-workflow の HEAD が `ce35193` でない、または作業ツリーに変更がある（写す元の版が決まらない）。

## Step 2 — 名前と description を kotowari 用にする

Purpose: 工程の skill が ba0918 の skill を1つも名指ししない状態にする。Specification: `#A1`、`#A2`、`#A3`、`#A9`。
Prerequisites: Step 1。
May change: `skills/kotowari-*/` の8つ。
Done when: 各 SKILL.md の `name` が `kotowari-<席>`。本文と description の `ba0918-<席>` の名指し（`/ba0918-brainstorm <topic>` のようなスラッシュコマンドの形を含む。iterate と investigate の案内の表にある）がすべて `kotowari-<席>` になり、「ba0918 workflow」「a ba0918 plan」「a ba0918 cycle」「a ba0918 review」のような言い方が kotowari に置き換わっている。各 description に「kotowari を使うリポジトリ（`.kotowari/` か `docs/ir/` がある）で使う」の条件が英語で入っている。例外として、`Source: \`ba0918-verification\`, agentic-rules v0.8.0.` の出典の行は写した文の出どころの表示なので残す。
Shown by: check — `rg -n 'ba0918' skills/kotowari-*/` の結果が `Source:` の出典の行だけ。`rg -n '^name:' skills/kotowari-*/SKILL.md` が8行で、すべて `kotowari-` で始まる。
Left to the implementer: description の条件の英語の言い回しと、description の中での位置。description にある日本語のキーワードの行は残してよい。
Stop and hand back if: `ba0918` を含む名指しのうち、工程の skill でも出典の行でもないもの（規範の skill を実行時に読ませる指示など）が見つかった。A1 の「agentic-workflow を必須にしない」の外の依存で、扱いが決まっていない。

## Step 3 — 判断の記録の形を kotowari スキルの reference にする

Purpose: kotowari スキルだけを入れた人も判断の記録を書けるようにする。Specification: `#A5`、`#A11`。
Prerequisites: Step 2（順序の都合だけ。内容は独立）。
May change: `skills/kotowari/references/`（新しい reference を1つ足す）、`skills/kotowari/SKILL.md`。
Done when: 新しい reference に、上の行き先の表で「kotowari スキルの新しい reference」とした中身が日本語で入っていて、`workflow.md` の該当の文と意味が同じ。ただし `workflow.md` の「必須の補足の行の有無と名前、`- superseded_by:` のリンクの実在は、判断の記録の形の契約（ir-form.md の「判断の記録の形」の節）で決まる。kotowari の check にこの検査が入ったら、指摘は findings.md の表で引く。」の文は、ir-form.md にその節が無く、検査も既に `findings.md` の record_field_missing、record_field_unknown、revision_link_invalid として入っているので、今の姿（この reference が形を述べ、検査の指摘は `findings.md` で引く）に書き直す。SKILL.md の場面 write の行がこの reference を読ませる。先頭に他の references と同じ形の改訂日の行がある。
Shown by: artifact — 新しい reference のファイル。レビューで `workflow.md` の brainstorm の節と段落ごとに突き合わせる。
Left to the implementer: reference のファイル名（`records.md` を推奨）、中の段落の並べ方。
Stop and hand back if: 記録の形の段落と手順の段落が1つの文の中で分けられない（行き先の表の線引きが当てはまらない）。

## Step 4 — kotowari-brainstorm に brainstorm の手順を溶かし込む

Purpose: brainstorm の出力が IR と判断の記録に行き、`docs/spec/` に行く道を無くす。Specification: `#A4`、`#A10`、`#A11`。
Prerequisites: Step 3。
May change: `skills/kotowari-brainstorm/`（SKILL.md と `references/records.md`）。
Done when: 行き先の表で kotowari-brainstorm とした中身が英語で入っていて、`workflow.md` の該当の文と意味が同じ。本文から `docs/spec/<name>.md` への出力と `.agents/tmp/brainstorm-*.md` の進捗ファイル、承認後の削除が無くなり、代わりに記録を `docs/decision/records/YYYY-MM-DD-<name>.md` に最初から書いて残すことになっている。記録の形は kotowari スキルの Step 3 の reference を名指しして読ませ、形の規則そのものは複製しない。IR を書くときは kotowari スキルの場面 write を、承認の前には場面 check と `collate.md` を読ませる。写した `references/records.md`（進捗ファイルの記録の種類）は、記録の種類の区別（未決と委譲の違いなど）を残したまま、置き場が判断の記録の節になる形に直す。その中の進捗ファイルの雛形のコードブロック（`(decides: person)` や `(why: ...)` を行の中に書く形）は kotowari の記録の形と食い違うので、雛形を消して Step 3 の reference を読ませる。
Shown by: check — `rg -n 'docs/spec|\.agents/tmp' skills/kotowari-brainstorm/` が0件。あわせて artifact としてレビューで `workflow.md` の brainstorm の節と突き合わせる。
Left to the implementer: 英語の言い回し、写した節のどこに差し込むか、写した文のうち置き換えで意味を失った文を消すか書き直すか。
Stop and hand back if: 写した本文の規則と `workflow.md` の規則が食い違い、どちらを採るか `workflow.md` に書いていない（例: 記録の種類 revision の置き場）。

## Step 5 — kotowari-plan に plan の手順を溶かし込む

Purpose: plan が IR の要求の ID を入力にして、`kotowari query` と `kotowari list` で要求を読むようにする。Specification: `#A4`、`#A5`、`#A10`。
Prerequisites: Step 2。
May change: `skills/kotowari-plan/`。
Done when: `workflow.md` の plan の節の段落（入力、承認済みの判定、要求の参照、対象の要求の一覧、読む量と `jq` の例、確認コマンド、plan 自身は check を走らせないこと）がすべて英語で入っていて、意味が同じ。写した本文の「specification の見出しを参照する」は、IR の要求を `文書のパス#REQ-nnn` で参照する形に置き換わっている。同じ記法を持つ `references/step-template.md` の `Specification: <path>#<heading>` と、SKILL.md の Finishing の自己点検（参照した見出しが仕様にあること）も同じ形に合わせる。`kotowari list`、`query`、`status` の出力を読むときは kotowari スキルの場面 check（`findings.md`）を読ませる。IR を持たない題目（この計画のように判断の記録が仕様のもの）の扱いは写した本文のまま残す。
Shown by: artifact — レビューで `workflow.md` の plan の節と突き合わせる。
Left to the implementer: 英語の言い回しと差し込む位置。
Stop and hand back if: 写した本文が求める「committed specification」の判定と、`workflow.md` の「承認済みの判定」が両立しない。

## Step 6 — kotowari-cycle と kotowari-review に cycle の手順を溶かし込む

Purpose: cycle の終端で `kotowari check` と `kotowari status` を走らせ、指摘の行き先を kotowari の分け方にする。Specification: `#A4`、`#A10`。
Prerequisites: Step 2。
May change: `skills/kotowari-cycle/`、`skills/kotowari-review/`、`skills/kotowari-iterate/`。
Done when: `workflow.md` の cycle の節の段落（review に渡す仕様のパスと `kotowari list` を絞った出力のファイル、implementer と fixer のプロンプトに `mark.md` を貼ること、終端報告の直前の check と status、テスト側の指摘と IR 側の指摘の行き先、変異の見逃し）がすべて英語で kotowari-cycle に入っていて、意味が同じ。`mark.md` と `mutants.md` は kotowari スキルの reference として名指しする。kotowari-review の入力の説明が、仕様が IR の置き場のパスで、差分が覆う ID の一覧がファイルで渡ることと矛盾しない。kotowari-iterate は kotowari-cycle の本文を読んで回す作りなので、その置き換えの表（「the specification path read from the plan」「the specification path in review delegations」の行）を、cycle が IR の置き場のパスと ID の一覧のファイルを渡すことに合わせる。仕様のパスが無い依頼では今までどおり渡さず、ID の一覧のファイルは依頼が IR の項目に触れるときだけ渡す。
Shown by: artifact — レビューで `workflow.md` の cycle の節と突き合わせる。
Left to the implementer: kotowari-review に書き足すか、kotowari-cycle の委譲の説明だけで足りるとするか（どちらでも review 役が受け取るものは同じ）。
Stop and hand back if: iterate の置き換えの表を上のとおり合わせても、cycle の kotowari 固有の手順のうち iterate で意味を持たないものが残る（例: 終端の `kotowari status` の `complete false` を、仕様の無い小さな作業でどう扱うか）。

## Step 7 — kotowari-implement に implement の手順を溶かし込む

Purpose: implement が `kotowari query` で要求を読み、check の指摘をテスト側と IR 側に分けて扱うようにする。Specification: `#A4`、`#A5`、`#A10`。
Prerequisites: Step 2。
May change: `skills/kotowari-implement/`。
Done when: `workflow.md` の implement の節の段落（`kotowari query` で読むこと、`kotowari list` を丸ごと読まないこと、確認コマンド、check の終了コード1のときの分け方、変異の見逃し）がすべて英語で入っていて、意味が同じ。テストを書くときは kotowari スキルの場面 mark（`mark.md`）を、check の出力を読むときは場面 check（`findings.md`）を読ませる。
Shown by: artifact — レビューで `workflow.md` の implement の節と突き合わせる。
Left to the implementer: 英語の言い回しと差し込む位置。
Stop and hand back if: なし（一般の停止条件だけ）。

## Step 8 — kotowari スキルから工程を外し、workflow.md を消す

Purpose: kotowari スキルを工程から独立させる。Specification: `#A4`、`#A5`、`#A8`。
Prerequisites: Step 3〜7。
May change: `skills/kotowari/`（SKILL.md、`references/workflow.md` の削除、`references/config.md`）。
Done when: `references/workflow.md` が無い。SKILL.md の場面の表と「場面とワークフローの対応」から場面 workflow が消え、残りの場面の説明が `workflow.md` を指さない。description と冒頭の段落の「ワークフローの手順を読む／伝える」の言い方も外す。場面の説明に残る「brainstorm の席」「cycle」などの一般の言い方はそのままでよいが、`kotowari-` で始まる skill の名前は出さない。`config.md` の setup の手順4の雛形が「このプロジェクトの仕様は IR（`docs/ir/`）で管理する」の趣旨の一文だけになり、工程の skill に触れない。
Shown by: check — `test ! -e skills/kotowari/references/workflow.md`、`rg -n 'workflow\.md|kotowari-(brainstorm|plan|cycle|implement|review|iterate|investigate|using-workflow)' skills/kotowari/` が0件、`rg -n 'ワークフロー' skills/kotowari/SKILL.md` が0件、`CARGO_BUILD_JOBS=4 cargo test --workspace --test step7_skill_references` が通る（`config.md` の YAML と `findings.md` の表を本体と突き合わせる既存のテスト）。
Left to the implementer: SKILL.md の文の直し方。
Stop and hand back if: `step7_skill_references` が `config.md` の変更で落ち、落ちた理由が雛形の一文の変更と関係ない（検査の対象を読み違えている）。

## Step 9 — 入れ方の説明、このリポジトリの入口、仕様の文書を直す

Purpose: 工程の skill を入れる人が AGENTS.md に足す1行を知り、このリポジトリ自身も kotowari-using-workflow から入るようにする。Specification: `#A2`、`#A6`、`#A7`、`#A8`。
Prerequisites: Step 8。
May change: `skills/README.md`（新規）、`AGENTS.md`、`PROJECT.md`、`docs/spec/kotowari-skill.md`。
Done when: `skills/README.md` に、工程の skill 8つと kotowari スキルの関係（工程の skill は kotowari スキルに依存し、逆は無い）、手元への入れ方（`~/.claude/skills/` にリポジトリの各ディレクトリへのシンボリックリンクを置く）、kotowari を使うプロジェクトの AGENTS.md に足す1行（新しい依頼の入口は kotowari-using-workflow が決め、ba0918-using-workflow は使わない）が書いてある。このリポジトリの `AGENTS.md` にその1行が入っている。`PROJECT.md` の skill の説明が `skills/` の9つを指す。`docs/spec/kotowari-skill.md` のうち `workflow.md`、場面 workflow、工程の手順の置き換え、コピーで入れること、AGENTS.md の節の中身に触れる箇所（「結果を一文で」、用語の「場面」と「reference」、「範囲」、R1、R2、R3、R4、R8、R9、R10）が今回の決定に合わせて書き換わり、冒頭の経緯の段落に今回の記録へのリンクがある。references は `workflow.md` が抜けて記録の形の reference が入るので7つのまま、場面は5つになる。
Shown by: artifact — 4つのファイル。`rg -n 'workflow|コピーで入れ|6つの場面|場面.*6つ' docs/spec/kotowari-skill.md` が、改めた経緯を述べる文だけになる。
Left to the implementer: README の構成と言語（このリポジトリの docs に合わせて日本語でも、skill に合わせて英語でもよい）、AGENTS.md の Rule Routing の表の中での行の位置と言い回し。
Stop and hand back if: なし（一般の停止条件だけ）。`AGENTS.md` は ba0918-scaffold の雛形の骨格を持つが、既にこのリポジトリ固有の kotowari の行を Rule Routing の表に持つので、同じ表に1行を足す。

## Step 10 — 手元にシンボリックリンクを張る

Purpose: 手元の環境で常にリポジトリの最新の skill を使う。Specification: `#A7`。
Prerequisites: Step 9 までがブランチでレビュー済みで、利用者が結果を受け入れていること。
May change: `~/.claude/skills/kotowari`、`~/.claude/skills/kotowari-*`（リポジトリの外）。
Done when: `~/.claude/skills/` に、`skills/` の9つのディレクトリそれぞれへのシンボリックリンクがある。
Shown by: external — `ls -l ~/.claude/skills/ | rg kotowari` で9つがリンクで、リンク先がこのリポジトリの `skills/` の下。今ある `~/.claude/skills/kotowari` は実ディレクトリなので、消してからリンクを張る。消すのはリポジトリの外のディレクトリで、プロジェクトのフックも `rm` を止めるので、人が `! rm -r ~/.claude/skills/kotowari` で実行する。消す前に、その中にリポジトリに無いファイルが無いことを `diff -rq ~/.claude/skills/kotowari skills/kotowari` で確かめて人に見せる。
Left to the implementer: なし。
Stop and hand back if: `diff -rq` が、Step 8 で消した `references/workflow.md` 以外に、リポジトリに無いファイル（`Only in /home/mizumi/.claude/skills/kotowari`）を出す。

## Verification map

| 決定 | 確かめる Step |
|---|---|
| A1（8つをすべて分ける） | 1、2 |
| A2（入口は kotowari-using-workflow） | 2、9 |
| A3（写して独立させる） | 1（写す元の版の固定）、2（ba0918 を名指ししない） |
| A4（置き場と workflow.md を消す） | 1、4〜8 |
| A5（依存の向き） | 3、8 |
| A6（IR を作らない、仕様の文書の書き換え） | 9。`docs/ir/` を変えないことはすべての Step の範囲で守る |
| A7（手元はシンボリックリンク） | 9（説明）、10（実施） |
| A8（setup の雛形と AGENTS.md の1行） | 8、9 |
| A9（description の条件） | 2 |
| A10（本文は英語） | 4〜7 |
| A11（記録の形は kotowari スキル、手順は工程の skill） | 3、4 |

## Left to the implementer

- 英語の言い回し全般。写した文は、置き換えで意味を失ったときだけ書き直す
- コミットの分け方は Step ごとを基本にする。Step 1 は写しただけの1コミットにする（ほかと混ぜない）

## Stop conditions

一般の4つ（承認した内容からの逸脱か意味の欠落、不可逆・特権・危険な操作、広がる事故、やり方を変えても進まない）に加えて:

- `workflow.md` の段落のうち、上の行き先の表のどれにも当てはまらないものが見つかった
- 写した本文の規則を消さないと kotowari 固有の手順が入らないが、`workflow.md` がその規則の置き換えを述べていない

## Test command

skill の文書の変更なので、新しいテストは足さない。確かめるのは各 Step の check と、最後に次の3つ。

- `CARGO_BUILD_JOBS=4 cargo test --workspace`（`step7_skill_references` を含む既存のテストが通る）
- `kotowari check`（終了コード0。IR と判断の記録を変えていないことの確かめ）
- `lefthook run pre-commit --no-auto-install`

## Out of scope

- 他の人へ配る仕組み（`TODO.md` の判断待ち）
- `/home/mizumi/develop/agentic-workflow/` と `~/.claude/skills/ba0918-*` の変更
- kotowari 固有でない部分の写した本文の改善
- `tests/library.rs` の `cargo fmt` のずれ
