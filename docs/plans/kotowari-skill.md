# 実装計画: kotowari スキル

Goal: `skills/kotowari/`（SKILL.md と references 6つ）を作り、手元の `~/.claude/skills/kotowari/` に入れて、kotowari を知らない LLM が5つの場面（setup、write、check、mark、workflow）を読めるようにする。

Specification: `docs/spec/kotowari-skill.md`（承認済み。A30〜A32 と細部の訂正を含む `9da9731`）。判断の記録は `docs/decision/brainstorm/2026-09-14-kotowari-skill.md`。この計画で仕様の節を指すときは `docs/spec/kotowari-skill.md#R番号` と書き、見出しの残りの文字列は省く（見出しの文字列にバッククォートが混ざるため）。

## Approach and why

成果物はすべて Markdown で、コードは無い。references は kotowari の仕様（`docs/ir/*.md`）と形の契約（`experiments/003-cli/brainstorm/ir-form.md`）から写して作るが、配布先で元本が読めない前提（仕様の R9）なので、写しは自己完結させ、実験の記録への参照と kotowari 自身の IR の文書名を落とし、kotowari 自身の ID を根拠として引かない。references を先に作り、SKILL.md は最後に作る（SKILL.md は references の表と場面の振り分けだけなので、references の名前と節が決まってからの方が短く書ける）。手元への導入と人の確認は最後の1ステップにまとめる。

各 reference の書式は、機械で確かめられるように次で固定する（仕様の R2、R6、R9 の確かめ方を check にするため）。

- 1行目は `kotowari 0.1.0 の仕様に基づく` の1行（見出しにしない）
- 節の見出しを Done when に列挙したステップ（1、4、5、6）では、`## ` で始まる行はその列挙の見出しだけ（コードブロックの中を含む。数を数える check があるため）。列挙の無いステップ（2、3）の節の構成は実装役が決める
- 表は1列目に種類の名前かキーを書き、バッククォートで囲まない

## Scope of change

- 作る: `skills/kotowari/SKILL.md`、`skills/kotowari/references/{ir-form,findings,config,collate,mark,workflow}.md`
- ステップ8だけ: `.agents/artifacts/kotowari-skill-check.md`（人の確認の記録。コミットしない）、`~/.claude/skills/kotowari/`（リポジトリの外。人がコピーする）
- 変えない: `src/`、`tests/`、`docs/ir/`、`docs/spec/`、`docs/decision/`、`experiments/`、`~/.claude/skills/ba0918-*/`（仕様の A27、P1）

## Step order and prerequisites

1〜5 は互いに独立で順はどれでもよい。6 は 4 と 5 の後（cycle の節が `mark.md` を、brainstorm の節が `collate.md` の返す形を名指しする）。7 は 1〜6 の後。8 は 7 の後。

## Verification map

| 仕様の節 | 実装するステップ | 証明するステップ |
|---|---|---|
| R1 SKILL.md は薄く | 7 | 7（check）、8(a) |
| R2 版の確認 | 7、1〜6（先頭の版） | 7（check）、8(a) |
| R3 setup | 3 | 3（check）、8(b) |
| R4 write | 1、6 | 1・6（check）、8(c) |
| R5 承認の関門 | 5、6 | 5・6（check）、8(c) |
| R6 check | 2 | 2（check）、8(d) |
| R7 mark | 4 | 4（check）、8(e) |
| R8 plan・cycle・implement の置き換え | 6 | 6（check）、8(f)、8(g) |
| R9 references は実験を参照しない | 1〜6 | 1〜6（check）、7（check、全体）、8(d)(e)(f) の人の読み |
| R10 元本と入れ方 | 1〜7 | 8(h) |

## Left to the implementer（計画全体）

- references の中の節の下の構成（小見出し、表か箇条書きか）。ただし節の見出しの名前と表の1列目は上の書式に従う
- 日本語の言い回し。ただし kotowari の用語（`docs/ir/CONTEXT.md`）はそのまま使い、言い換えない。TBL-004 の既定の列だけは1文字も変えずに写す（check が文字列で比べる）
- 例に使う架空のパスと ID の値（仕様の R9 の形に従う）

## Stop conditions（計画全体）

一般の4つの条件（意味の欠落か承認された内容からの逸脱、不可逆・特権・危険な操作、広がる事故、やり方を変えても進まない）に加えて:

- 元にする `docs/ir/` と `experiments/003-cli/brainstorm/ir-form.md` が食い違うとき: `docs/ir/` が正（仕様の A32）。実測（`kotowari check` を小さな入力で走らせる）で確かめられるなら確かめて記録し、実測も `docs/ir/` と食い違えば止まって人に言う
- 仕様の R1〜R10 が求める内容のうち、この計画のどのステップの May change にも置き場が無いものを見つけたとき

## Test command

コードが無いので `cargo test` は使わない。各ステップの Shown by に check のコマンドを列挙する。実行はリポジトリ直下で、bash で行う。括弧内は期待する出力で、実行者が突き合わせる（`rg -n` の「0件」は出力が空で終了コード1）。

## Out of scope

- kotowari 自身のリポジトリへの `setup` の適用（設定と置き場が実験003の形のまま。別途）
- kakoi-net での試用（仕様の A4。この計画の取り込み後に別途）
- 既存の ba0918 のスキルの変更（A27）

---

## Step 1 — `references/ir-form.md`（IR の形の規則）

Purpose: brainstorm の席が IR を書けるだけの形の規則を、1つの文書に自己完結して持たせる。Specification: `docs/spec/kotowari-skill.md#R4`、`docs/spec/kotowari-skill.md#R9`、`docs/spec/kotowari-skill.md#R2`（先頭の版の写し）。
Prerequisites: なし。元にするのは `experiments/003-cli/brainstorm/ir-form.md`（契約。見出しは 文書・ID・項目・出典・用語と曖昧語・文書名の参照・検査の種類・出力 の8つで、残りは `docs/ir/` から取る）、`docs/ir/CONTEXT.md`（用語と除外の一覧）、`docs/ir/ir-document.md`、`docs/ir/ir-input.md`、`docs/ir/ir-items.md`、`docs/ir/ir-missing.md`（必須の行）、`docs/ir/ir-references.md`、`docs/ir/terms-form.md`、`docs/ir/terms.md`、`docs/ir/sources.md`、`docs/ir/decision-records.md`、`docs/ir/form-contract.md`、`docs/ir/config.md`（TBL-004 の上限の既定 120 と 10）。
May change: `skills/kotowari/references/ir-form.md` だけ。
Done when: 1行目が版の行。節の見出しが `## 文書`、`## ID`、`## 項目`、`## シナリオ`、`## 用語集`、`## 問題の記録`、`## 出典`、`## 用語と曖昧語`、`## 文書名の参照`、`## 除外`、`## 上限` の11個だけ（仕様の R4 の列挙との対応: 用語集 = 用語集の表の形、除外 = 除外の一覧、上限 = 1文書の上限）で、各節の内容が元の文書と食い違わない（人の突き合わせはステップ8(c)）。決定の番号と出典の例は架空のパス。`experiments/` と kotowari 自身の IR の文書名（`docs/ir/ir-items.md` の類）が無い。
Shown by: check — `head -1 skills/kotowari/references/ir-form.md`（`kotowari 0.1.0 の仕様に基づく`）、`rg -c '^## (文書|ID|項目|シナリオ|用語集|問題の記録|出典|用語と曖昧語|文書名の参照|除外|上限)$' skills/kotowari/references/ir-form.md`（11）、`rg -c '^## ' skills/kotowari/references/ir-form.md`（11）、`rg -n 'experiments/|docs/ir/[a-z-]+\.md' skills/kotowari/references/ir-form.md`（0件）。
Left to the implementer: 節の順序。契約の表を写すか箇条書きに直すか。例に使う ID の値。
Stop and hand back if: 計画全体の Stop conditions の1つ目（契約と `docs/ir/` の食い違い）。

## Step 2 — `references/findings.md`（指摘の種類ごとの意味・対処・担当と、停止の表）

Purpose: `check` の結果を読んだ LLM が、指摘ごとに何を誰が直すかを引けるようにする。Specification: `docs/spec/kotowari-skill.md#R6`、`docs/spec/kotowari-skill.md#R9`。
Prerequisites: なし。元は `docs/ir/findings.md`（TBL-008、TBL-009）、`docs/ir/finding-order.md`（TBL-019 の line）、`docs/ir/output.md`（JSON の形、TBL-006 の path）、`docs/ir/cli-environment.md`（TBL-018 停止の文言、TBL-020 詳細）、`docs/ir/cli.md`（TBL-001、TBL-002）。
May change: `skills/kotowari/references/findings.md` だけ。
Done when: 1行目が版の行。本文の最初の手順が「終了コードを見る。2なら停止、0か1なら JSON を読む」。誤り33種類と警告2種類の表（1列目が種類の名前。`unresolved_reference` は path で2行。`requirement_without_test` は担当が implementer である旨の例外つき）に、意味・対処・担当がある。停止の表（1列目が英語の文言 `config error`、`argument error`、`unreadable file`、`non-UTF-8 file`）に対処（引数の誤りは自分で直す、他の3つは人に返す）がある。「除外の追加や規則の緩めは brainstorm に戻す」がある。`experiments/` と kotowari 自身の IR の文書名が無い。
Shown by: check — `head -1 skills/kotowari/references/findings.md`（`kotowari 0.1.0 の仕様に基づく`）、`rg -n 'experiments/|docs/ir/[a-z-]+\.md' skills/kotowari/references/findings.md`（0件）、`python3 -c 'import re;a=set(re.findall(r"^\| ([a-z_]+) \|", open("docs/ir/findings.md").read(), re.M));b=set(re.findall(r"^\| ([a-z_]+) \|", open("skills/kotowari/references/findings.md").read(), re.M));print(sorted(a-b), sorted(b-a))'`（`[] []`）、`rg -c '^\| unresolved_reference \|' skills/kotowari/references/findings.md`（2）、`rg -c '^\| (config error|argument error|unreadable file|non-UTF-8 file) \|' skills/kotowari/references/findings.md`（4）、`rg -n '終了コード' skills/kotowari/references/findings.md | head -1`（版の行の直後の手順の中にある）。
Left to the implementer: 節の構成。2列目以降の列の順。対処の文の長さ。
Stop and hand back if: ある指摘の対処が、仕様の R6 の担当の分け方（表の担当の列が正。IR 側は brainstorm の席、テスト側は implementer、`requirement_without_test` は例外）で決まらないとき。

## Step 3 — `references/config.md`（設定と置き場と `setup` の手順）

Purpose: `setup` が作るものと設定ファイルの全キーを1か所に持つ（仕様の A31）。Specification: `docs/spec/kotowari-skill.md#R3`、`docs/spec/kotowari-skill.md#R9`。
Prerequisites: なし。元は `docs/ir/config.md`（TBL-004 のキー・値・既定、REQ-012 空の設定は既定）、`docs/ir/base-directory.md`（TBL-003 基準のディレクトリ）、`docs/ir/terms-form.md`（REQ-117 用語集の表）、仕様の R3（`AGENTS.md` の節の趣旨）。
May change: `skills/kotowari/references/config.md` だけ。
Done when: 1行目が版の行。TBL-004 と同じ3列（キー、値、既定）の表で、1列目のキーと3列目の既定が TBL-004 と1文字も違わない。`setup` の手順: `.kotowari/config.yaml` を既定の値で書く（値を既定から変えるのはテストの glob か置き場を変えるときだけ）、置き場3つ、`CONTEXT.md` の4行（題名、空行、ヘッダ、区切り）、`AGENTS.md` の節（見出し `## kotowari`。この見出しが既にあれば足さない。無ければ末尾に足す。ファイルが無ければ作る。雛形は本文に示す）、既にあるファイルとディレクトリは上書きせず人に言う。基準のディレクトリの決まり方（`.kotowari/` を上に探す。無ければカレントディレクトリ）。`experiments/` と kotowari 自身の IR の文書名が無い。
Shown by: check — `head -1 skills/kotowari/references/config.md`（`kotowari 0.1.0 の仕様に基づく`）、`rg -n 'experiments/|docs/ir/[a-z-]+\.md' skills/kotowari/references/config.md`（0件）、`python3 -c 'import re;f=lambda p:set((m[0],m[2]) for m in re.findall(r"^\| ([a-z_.]+) \| ([^|]*) \| ([^|]*) \|", open(p).read(), re.M));a=f("docs/ir/config.md");b=f("skills/kotowari/references/config.md");print(sorted(a-b), sorted(b-a))'`（`[] []`。キーと既定の対が一致）、`rg -c '## kotowari' skills/kotowari/references/config.md`（1以上。雛形の見出し。コードブロックの中でよい）。
Left to the implementer: 節の構成。`AGENTS.md` の節の本文の文言（趣旨は仕様の R3: 仕様は IR、brainstorm・plan・cycle・implement では kotowari スキルを読む）。設定ファイルの雛形のコメントの有無。
Stop and hand back if: TBL-004 の既定と、手元の `kotowari check` が空の設定で実際に使う値が食い違うとき。

## Step 4 — `references/mark.md`（印の規則）

Purpose: implementer と fixer が印を正しく置けるようにし、cycle がプロンプトに貼れる1枚にする。Specification: `docs/spec/kotowari-skill.md#R7`、`docs/spec/kotowari-skill.md#R9`。
Prerequisites: なし。元は `docs/ir/test-markers.md`（TBL-015 印の形、TBL-016 位置、REQ-074 コメント記号、REQ-076 問い合わせの無い言語の印）、`docs/ir/test-discovery.md`（TBL-017 テストの見分け方、REQ-081 Rust だけ）、`docs/ir/coverage.md`（REQ-085、REQ-087 問い合わせの無い言語の対応）、仕様の R7（テスト名の慣習。kotowari は検査しない）。
May change: `skills/kotowari/references/mark.md` だけ。
Done when: 1行目が版の行。節の見出しが `## 印の形`、`## 置ける位置`、`## テストの見分け方`、`## テスト名`、`## Rust 以外` の5個だけで、内容が元の文書と食い違わない（人の突き合わせはステップ8(e)）。kotowari 自身の ID を「〜のとおり」のように根拠として引かない（例の ID は可）。`experiments/` と kotowari 自身の IR の文書名が無い。
Shown by: check — `head -1 skills/kotowari/references/mark.md`（`kotowari 0.1.0 の仕様に基づく`）、`rg -c '^## (印の形|置ける位置|テストの見分け方|テスト名|Rust 以外)$' skills/kotowari/references/mark.md`（5）、`rg -c '^## ' skills/kotowari/references/mark.md`（5）、`rg -n 'experiments/|docs/ir/[a-z-]+\.md' skills/kotowari/references/mark.md`（0件）。
Left to the implementer: 例に使うテストの中身。長さ（プロンプトに貼るので短い方がよいが、上限は決めない）。
Stop and hand back if: 印の位置の規則（属性を挟む、空行で切れる）を例で示そうとして、`docs/ir/` の具体例と食い違う読み方が2つ以上あるとき。

## Step 5 — `references/collate.md`（照合レビューの指示）

Purpose: 承認の前に、IR の各項目が判断の記録に裏付けられているかを別セッションの LLM に確かめさせる指示を持つ。Specification: `docs/spec/kotowari-skill.md#R5`、`docs/spec/kotowari-skill.md#R9`。
Prerequisites: なし。参考は `experiments/003-cli/review/instructions.md`（実験003の照合レビューの指示。写すのではなく、同じ判定の基準を実験の文脈を外して書き直す）。
May change: `skills/kotowari/references/collate.md` だけ。
Done when: 1行目が版の行。節の見出しが `## 入力`（項目と出典の対。IR の置き場の文書と判断の記録のパス）、`## 基準`（出典の決定が項目の内容を裏付けるか。裏付けの無い項目の挙げ方）、`## 返す形`（JSON。`{"unsupported":[{"id":"…","source":"…","reason":"…"}]}` の形に固定。この形は `workflow.md` の brainstorm の節が反映の手順で前提にする）、`## 上限`（3回。残れば FLAG にして人に返す）の4個だけ。頼む側の手順（別セッションに渡す、結果を反映する）は `workflow.md` に置き、ここには書かない。`experiments/` と kotowari 自身の IR の文書名が無い。
Shown by: check — `head -1 skills/kotowari/references/collate.md`（`kotowari 0.1.0 の仕様に基づく`）、`rg -c '^## (入力|基準|返す形|上限)$' skills/kotowari/references/collate.md`（4）、`rg -c '^## ' skills/kotowari/references/collate.md`（4）、`rg -n '"unsupported"' skills/kotowari/references/collate.md`（1件以上）、`rg -n 'experiments/|docs/ir/[a-z-]+\.md' skills/kotowari/references/collate.md`（0件）。
Left to the implementer: 基準の言い回し。
Stop and hand back if: 実験003の指示の判定の基準が、仕様の R5 の「出典の決定が項目の内容を裏付けるか」と食い違うとき。

## Step 6 — `references/workflow.md`（brainstorm・plan・cycle・implement の手順の置き換え）

Purpose: 既存のスキルを変えずに、IR を使うときの手順の違いを1か所に持つ。Specification: `docs/spec/kotowari-skill.md#R4`（brainstorm の節）、`docs/spec/kotowari-skill.md#R5`（承認の手順は brainstorm の節に含める）、`docs/spec/kotowari-skill.md#R8`、`docs/spec/kotowari-skill.md#R9`。
Prerequisites: ステップ4（cycle の節が `mark.md` を貼ると書く）とステップ5（brainstorm の節が `collate.md` の返す形 `unsupported` を反映する手順を書く）。
May change: `skills/kotowari/references/workflow.md` だけ。
Done when: 1行目が版の行。節の見出しが `## brainstorm`、`## plan`、`## cycle`、`## implement` の4個だけ。brainstorm の節に仕様の R4 の8項目（出力、判断の記録、再開、成功の条件と反例の置き場、禁止・却下・未決・委譲の置き場、用語集、終わりのレビュー、stage するもの）と R5 の3手順（check、照合レビューの頼み方と `unsupported` の反映と3回の上限、人に見せるもの）がある。plan の節に、入力（IR の置き場のパスと要求 ID の一覧）、承認済みの判定、要求の参照の書式 `文書のパス#REQ-nnn` と実在の確かめ方（`### REQ-nnn:` で始まる見出し）、最後のステップの確認コマンドに `kotowari check` を列挙すること、plan 自身は走らせないこと、がある。cycle の節に、review に渡す仕様のパス（IR の置き場）、implementer と fixer のプロンプトに `mark.md` を貼ること、終端報告の直前の `kotowari check` と出力の掲載、テスト側の指摘の扱い（fixer へ。直らなければ進捗なしの終わり方）、IR 側の指摘の扱い（人の判断として終端報告に載せ brainstorm に戻す）、がある。implement の節に、plan が列挙した `kotowari check` を確認コマンドとして走らせること、終了コード1のときの扱い（テスト側は自分で直す、IR 側は差し戻す）、がある。既存スキルへの言及は「置き換える手順の名前」だけで、既存スキルのファイルを変える指示が無い。判断の記録の節の見出し（`## Agreements` の類）を示すときは行の中でバッククォートで囲む。`experiments/` と kotowari 自身の IR の文書名が無い。
Shown by: check — `head -1 skills/kotowari/references/workflow.md`（`kotowari 0.1.0 の仕様に基づく`）、`rg -c '^## (brainstorm|plan|cycle|implement)$' skills/kotowari/references/workflow.md`（4）、`rg -c '^## ' skills/kotowari/references/workflow.md`（4）、`rg -n 'experiments/|docs/ir/[a-z-]+\.md' skills/kotowari/references/workflow.md`（0件）、`rg -n 'mark\.md|collate\.md|unsupported|#REQ-nnn' skills/kotowari/references/workflow.md`（4語とも1件以上）。
Left to the implementer: 節の中の項目の順。
Stop and hand back if: 既存スキルの手順のうち、仕様の R4・R8 が置き換えを書いていないのに IR では成り立たないものを見つけたとき（仕様の穴。brainstorm に戻す）。

## Step 7 — `SKILL.md`（薄い振り分け）

Purpose: 場面を選び、対応する reference を読ませる入口を作る。Specification: `docs/spec/kotowari-skill.md#R1`、`docs/spec/kotowari-skill.md#R2`、`docs/spec/kotowari-skill.md#R9`（版の正本と全体の検査）。
Prerequisites: ステップ1〜6（reference の名前と節）。
May change: `skills/kotowari/SKILL.md` だけ。
Done when: frontmatter（`name: kotowari`、`description` は1行で、発火語 kotowari、IR、`docs/ir`、`@kotowari`、印を含む）。本文に、目的、対象の版 `0.1.0`（正本）、版の確認の手順（`kotowari --version`。不一致・コマンド無し・版が読めない、の3分岐で止まる）、場面の選び方（仕様の R1 の対応。5つ）、場面ごとに読む reference の表（仕様の R1 のとおり: setup → config、write → ir-form と workflow の brainstorm の節、check → findings、mark → mark、workflow → workflow の自分の席の節。collate は write の承認前）、5つの場面がワークフローのどこに当たるか。100行以内。
Shown by: check — `wc -l skills/kotowari/SKILL.md`（100以下）、`rg -n '^description:.*kotowari' skills/kotowari/SKILL.md`（1件）、`rg -n 'REQ-|missing_|decisions\.' skills/kotowari/SKILL.md`（当たりが frontmatter と表の中だけ）、`rg -n 'experiments/|docs/ir/[a-z-]+\.md' skills/kotowari/`（0件）、`for f in skills/kotowari/references/*.md; do head -1 "$f"; done`（6行すべて `kotowari 0.1.0 の仕様に基づく`）、`rg -n '見つからない|読めない|一致しない' skills/kotowari/SKILL.md`（3分岐に当たる行がある）。
Left to the implementer: description の文。表の形。
Stop and hand back if: 100行に収めると仕様の R1 が求める内容のどれかが落ちるとき。

## Step 8 — 手元への導入と人の確認

Purpose: 元本をコピーで入れ、仕様の人が確かめる条件を実際に確かめる。Specification: `docs/spec/kotowari-skill.md#R10`、および R1〜R9 の確かめ方のうち人が行うもの。
Prerequisites: ステップ7。`kotowari 0.1.0` が PATH にあること（`kotowari --version` が `kotowari 0.1.0` を出す。無ければ人が `cargo install --path .` を行う）。
May change: `~/.claude/skills/kotowari/`（リポジトリの外。人がコピーする。既にあれば人が上書きを決める）、`.agents/artifacts/kotowari-skill-check.md`（確認の記録。コミットしない）。リポジトリの他のファイルは変えない。
Done when: `~/.claude/skills/kotowari/` が `skills/kotowari/` と同じ内容（`diff -r skills/kotowari ~/.claude/skills/kotowari` が空）。人が次を (a)、(b)、(c)、(d)、(e)、(f)、(g)、(h) の順に確かめて記録した。(b)〜(c) と (g)〜(h) は同じ一時的なリポジトリ（kotowari リポジトリの外）を使い回す。
- (a) R1・R2: SKILL.md と references 6つの先頭を読み、版の行、場面の対応、版の確認の3分岐がある。`--version` の出力を `kotowari 9.9.9` にした偽の `kotowari` を PATH の先頭に置いてスキルを呼ぶと作業を始めずに止まる。PATH から `kotowari` を外して呼ぶと導入を求めて止まる
- (b) R3: kotowari を知らない別セッションの LLM に、空の一時リポジトリで `setup` を行わせる。`.kotowari/config.yaml`、置き場3つ、`CONTEXT.md`（4行）、`AGENTS.md`（`## kotowari` の節）ができ、`kotowari check` が終了コード0で `files` 1、`lines` 4、`findings` 空。サブディレクトリから走らせても同じ。同じセッションに `setup` をもう一度行わせても `AGENTS.md` の節は1つで、`CONTEXT.md` は変わらない（`git status` と `git diff` で見る）
- (c) R4・R5: 同じ一時リポジトリで、別セッションの LLM に `write` を読ませ、判断の記録（`docs/decision/brainstorm/` に日付と題のファイル、決定の節に番号つきの行）と、要求・決定表か性質・シナリオ・用語・問題の記録を各1件書かせる。`check` の誤りが `requirement_without_test` だけで、出典が記録の決定の番号を指し、その決定が項目の内容を言っている。続けて照合レビューを1回実際に走らせ（`collate.md` の指示で別セッションを起こす）、返る JSON が `unsupported` の形である。続けて承認の依頼の文を作らせ、check の終了コードと `requirement_without_test` の件数と警告の件数、照合レビューの件数、承認の対象のパスと識別子があり、IR の差分を読むよう求めていない。その後、IR と記録をその一時リポジトリにコミットする（(g) の前提）。人が `ir-form.md` の11の節を `docs/ir/` の文書と突き合わせて食い違いが無い
- (d) R6・R9: `findings.md` の表を `docs/ir/findings.md` の TBL-008・TBL-009 と突き合わせ、対処と担当の列を読む。手順の最初が終了コードの確認である。kotowari 自身の ID を根拠として引いていない
- (e) R7・R9: 別セッションの LLM に `mark.md` でテストを1本書かせ、`check` が `test_without_id` と `invalid_marker` を出さない。`mark.md` の5つの節を TBL-015〜017、REQ-076、REQ-087、仕様の R7 と突き合わせる。kotowari 自身の ID を根拠として引いていない
- (f) R8・R9: `workflow.md` の4つの節を仕様の R4・R5・R8 と突き合わせる。`find ~/.claude/skills/ba0918-* -newer docs/plans/kotowari-skill.md` が空（この計画の後に既存スキルのファイルが変わっていない）。kotowari 自身の ID を根拠として引いていない
- (g) R8（導線）: (c) でコミットした一時リポジトリで plan の席のセッションを起こし、`AGENTS.md` の規則から `workflow.md` の plan の節を読むことを、そのセッションの記録（読み込みの tool 呼び出しの一覧）で見る
- (h) R10: 同じ一時リポジトリで5つの場面を呼び、セッションの記録（読み込みの tool 呼び出しの一覧）に kotowari リポジトリのファイルが無い
Shown by: external — 人が (a)〜(h) を行い、結果を `.agents/artifacts/kotowari-skill-check.md` に書く。(b)、(c)、(e)、(g)、(h) は LLM のセッションを新しく起こすので、費用がかかることを人に言ってから行う。
Left to the implementer: 一時的なリポジトリの場所。セッションの記録の取り方（会話の tool 呼び出しの一覧を見る）。
Stop and hand back if: (a)〜(h) のどれかが通らず、原因が references の写し間違いでなく仕様の穴のとき。
