# overview-workflow の対応表

元: `docs/ir/core/overview-workflow.md`（日本語は `overview-workflow.ja.md`）。書き直した文書: `overview-workflow.md`。短い名前は `ovflow`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-300 | C-core-ovflow-7, C-core-ovflow-8, C-core-ovflow-9 | 壁打ちの承認の手順に全体像を作り直す段を置き、全体像は承認の対象にしない | 読み替え（提案）。壁打ちの承認で全体像を作って示す手順（2026-10-02-whole-picture.md の A2、A8、A34、A52、A84、2026-10-05-overview-index.md の A19）。利用者の答え（BRIEF-C の2: 段を戻す。設定に全体像のキーがあるプロジェクトでだけ行い、全体像は承認の対象にしない）で決めた。「変えた IR の話題ごとの文書」を変えた決定の文書に、「元データの "ir" の一覧」を元データの一覧に、「今の check と照合レビューの後」を承認の手順の中に（照合は持たない）、承認の材料を決定の文書の差分に読み替えた。壁打ちの skill に段がまだ無いので提案 |
| REQ-core-301 | C-core-ovflow-4 | 全体像の単位は、使う人が1つの機能として名前で呼べるものにし、人が決める | 読み替え（提案）。A18、A20、A35（利用者）。「どの全体像の元データの "ir" にも無い話題ごとの文書」を、どの全体像にも属さない決定の文書の話題に読み替えた。案を出して人が決める時（壁打ちの承認のとき）と、「壁打ちがその話題に触れたときに初めて作る」の時は、承認の段の決定（C-core-ovflow-8）に書いた |
| REQ-core-302 | C-core-ovflow-1, C-core-ovflow-2, C-core-ovflow-3 | 全体像の元データは、冒頭の結論から始め、題材に合わせた部品で、決定のつながりと振る舞いと今の状態を表す | 読み替え（提案）。A1、A6、A7、A8、A9、A14、A15、A38、A54（利用者）、A66（LLM が導き、利用者が第21ラウンドで確認した）、2026-10-05-guide-overview-roles.md の A2、A3、A4、A6（利用者）。「後回しの予定」を状態が「提案」の決定に、「IR の項目を指すガイドの印」を決定を指すガイドのマーカーに読み替えた。今の skill の文は IR と後回しを前提にしているので提案にした |
| REQ-core-303 | C-core-ovflow-6 | cycle と実装は、全体像の元データを直さない | そのまま（cycle と実装が直さない部分。A39、利用者）。「次にその話題に触れた壁打ちが直す」の部分は、承認の段の決定（C-core-ovflow-8、提案）に書き、この決定の問題からリンクした |
| REQ-core-333 | C-core-ovflow-5 | 目次の置き場所は、全体像を書く LLM が人に聞かずに決め、承認の対象にしない | そのまま（LLM が決めて人に聞かないこと、承認の対象にしないこと、群を分ける時、並びの順。overview-index の A4、A6、A19、利用者）。「承認のときに」の時は、承認の段の決定（C-core-ovflow-7 の目次を直す手順）に書いた |

元の文書に場面、TBL、PROP は無い。

## テストごとの表

この話題の ID を指すテストは無い（どの要求も、確かめ方が読んで確認）。ガイドのマーカーも無い。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| （無い） | A84（LLM）の「設定に "overview" のキーを持つプロジェクトでだけ行う」は、利用者の答えが同じことを決めたので、承認の段の決定に書いた |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-300 | 読み替えて残る（提案）。「変えた IR の話題ごとの文書」を変えた決定の文書に、「全体像の元データの "ir" の一覧」を全体像の元データが指す決定の文書の一覧に、「今の check と照合レビューの後」を check の後に（照合は持たない。`decision-documents.md` の「仕様は、話題ごとの、今有効な決定の文書だけで持つ」）、承認の材料を決定の文書の差分に（workflow の「壁打ちは、最初に困っている場面を1つ組み、承認の前にその場面がどうなるかを見せる」）読み替えた |
| REQ-core-301 | 読み替えて残る（提案）。IR の話題ごとの文書を決定の文書の話題に |
| REQ-core-302 | 読み替えて残る（提案）。後回しの予定を「提案」の状態の決定に、IR の項目と判断の記録を指す参照とマーカーを決定を指すものに |

### テストの無い行

無い。この話題の行はどれも「（読んで確認）」。

## 誰が決めたか見分けられなかったもの

- 無い。whole-picture の A84 は LLM、A66 は LLM が導いて利用者が確認したもの、ほかの A 行は利用者。

## 理由や見送った案が記録に無かった決定

- 「目次の置き場所は、…承認の対象にしない」: 見送った案が記録に無い。
- 「cycle と実装は、全体像の元データを直さない」: 見送った案が記録に無い。理由は A39 の why（読む人が古い部分を見分けられ、cycle の作業を増やさない）だけで、それを書いた。
- 「全体像の単位は、…人が決める」: 見送った案は A35 のものだけ。

## 分けた所、まとめた所

- REQ-core-302 と guide-overview-roles の書き分けの指針（A3、A4、A6）を1つの決定にまとめた。どちらも全体像の元データを書く場面の指針で、guide-overview-roles の A4 が「書き分けの指針は全体像の書き方の references に書く」と決めている。
- REQ-core-301、REQ-core-303、REQ-core-333 から、壁打ちの承認の段に依存する時の部分を外し、残りをそれぞれ1つの決定にした。外した部分は、REQ-core-300 を読み替えた承認の段の決定に入れ、元の決定から見出しへリンクした。

## あとでリンクにする所

- 無い。overview-data、overview-commands、languages-and-guides の「全体像の元データは、決定の文書を指す」は、冒頭の段落でリンクにした。
- `docs/decisions/view/rendering.md`（古い節のマーカー）: 「cycle と実装は、全体像の元データを直さない」の問題（読む人がページのマーカーで古い部分を見分けられる）

## 迷った判断と、どう決めたか

- 目次の置き場所の決定と cycle の決定は、今の skill の文（`agent/skills/kotowari/references/overview.md` と `agent/skills/kotowari-cycle/SKILL.md`）がそのまま満たしているので、実装済みにした。行は、今の文で成り立つように、承認の時を含めずに書いた。
- 全体像の元データを書く場面の決定は、今の references の文が IR の項目、判断の記録、後回しを前提にしているので、提案にした。
- 今の kotowari の skill（`agent/skills/kotowari/SKILL.md` の場面の表の「overview — brainstorm (at approval…)」と、`references/overview.md` の「Write it at the brainstorm approval, in the steps of kotowari-brainstorm」と guide_stale の節の「the next brainstorm that touches the topic revises the overview」）は、壁打ちの skill から外された段を今も前提にしている。壁打ちの skill（`agent/skills/kotowari-brainstorm/SKILL.md`）には全体像の段が無い。利用者の答え（段を戻す）で、承認の段を提案の決定にした。
- 承認の段の決定の問題には、記録の理由（A2 の「結局どうなるのか」と途中で作る費用、A34 の承認が重くなる懸念）と、段が無いと誰も直さないという事実を書いた。A34 の rejected（最初に全体像を開かせる、全体像も承認の対象にする）と A2 の「途中で作ることは扱わない」を見送った案にした。A34 の rejected のうち「照合レビューにかける」は、照合が無くなるので書かなかった。
- 単位の決定の目安の例（A20 の「変更照合、ガイドの印、導入、変異テストなど」）は、BRIEF-C の3に合わせ、例だと分かる書き方にした。前の下書きで例から外れていた変更照合は、外したままにした（外した理由は前の下書きの対応表に書かれていない）。

## 実装する側が決める細部

- 全体像の元データの frontmatter で、扱う決定の文書の一覧を書くキーの名前（今は "ir"）。`languages-and-guides.md` の決定は「扱う文書の一覧は、決定の文書のパスで書く」だけを決めている。
- 単位の決定でいう「どの全体像にも属さない話題の文書」を、話題の文書のパスで判定するか、まとまりと話題の名前で判定するか。

## 利用者に聞くこと

- 無い。壁打ちの承認の段を戻すかの問いは、BRIEF-C の2の答え（戻す。提案）で消した。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えることになりそうか |
|---|---|
| 全体像の元データは、冒頭の結論から始め、題材に合わせた部品で、決定のつながりと振る舞いと今の状態を表す | `agent/skills/kotowari/references/overview.md` の「Scene: writing overview data」と冒頭の段落、frontmatter と参照の節を、決定の文書の一覧、決定を指す参照とマーカー、「提案」の決定と未決に書き直す |
| 全体像の単位は、使う人が1つの機能として名前で呼べるものにし、人が決める | `agent/skills/kotowari/references/overview.md` の「The unit of an overview」を、IR の話題ごとの文書から決定の文書の話題に書き直す |
| 壁打ちの承認の手順に全体像を作り直す段を置き、全体像は承認の対象にしない | `agent/skills/kotowari-brainstorm/SKILL.md` の承認の手順に段を足し、`agent/skills/kotowari/references/overview.md` の「Write it at the brainstorm approval」をその段に合わせる |

## 数

- 元: 要求 5、場面 0、決定表 0、性質 0
- 読んだ記録のファイル: 3（2026-10-02-whole-picture.md、2026-10-05-overview-index.md、2026-10-05-guide-overview-roles.md）。ほかに `agent/skills/kotowari-brainstorm/SKILL.md`、`agent/skills/kotowari-cycle/SKILL.md`、`agent/skills/kotowari/SKILL.md`、`agent/skills/kotowari/references/overview.md` と、壁打ちの skill から全体像の段を外したコミットの本文を読んだ
- 出来上がり: 決定 5（実装済み 2、提案 3）、確認することの行 9（うち「（読んで確認）」9）
- テスト: 0
- 利用者に聞くこと: 0（答えの出た1つを消した）
