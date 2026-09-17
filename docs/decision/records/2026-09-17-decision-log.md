# 壁打ちの記録: 判断の記録に理由を必須にし、ADR の新規作成をやめる

## Context

判断の記録（brainstorm ごとの1行1決定のファイル）は、決定の行に理由が無いものが多い。
5本の記録の決定 364 行のうち、理由らしい語（理由、根拠、なぜ、ため、から、実測、経験）を含む行は約 30%（2026-09-17 の実測。語で当てる雑な指標で、「:」で理由を続ける行は拾えていない）。
一方で ADR は 0004（2026-09-13）を最後に書かれておらず、壁打ちの手順のどこにも「4つの基準に当てて ADR を切る」段が無い。
ADR が持っていた「採用理由の詳細」と「改訂の統合先」は、記録の行に理由を必須にし、改められた行から新しい決定へ辿れるようにすれば埋まる。
記録の前提だった「1ファイルで上書きされる」は、brainstorm ごとの別ファイルになった時点で消えている（[records.md#A22](./records.md#A22)）。
そこで、記録の形を人が読める形に寄せ、理由を必須にし、ADR を新しく書く義務をやめる。

Position: 第1〜第2ラウンドは承認・コミット済み。A19（2026-09-17）でゲートの置き場を改めた

## Agreements

- A1 Agreements と Prohibitions の1件は、1行目に決定の本文（1つの決定。文の数は問わないが、理由や経緯は混ぜない）だけを書き、その下に字下げの箇条書きで「理由」「却下」「決めた人」「改めた」を持つ。「理由」は必須、「却下」はその決定に固有の却下案があるときだけ、「決めた人」は、これまでの記録で決定の行の末尾に書いていた「（推奨を採用）」「（利用者の言葉）」の括弧書きを置き換える。Rejected と Delegated の1件は「理由」を必ず持ち（brainstorm スキルの記録の種類の規則どおり）、「決めた人」は任意。Undecided の1件は「決める人」を必ず持ち、「関係」は任意
  - why: 決定と理由が1行に混ざると、人が読むときに決定を拾えない。kotowari の出典の判定（[TBL-012](../../ir/sources.md#TBL-012)）は、行頭の空白を除いて「- 印 」で始まる行を見て、印が決定の番号の形（英大文字1文字に数字）のものだけを決定として読むので、「- 理由:」のような補足の行は決定として読まれない
  - decided_by: 利用者（推奨を採用）
  - superseded_by: [record-form の A27](./2026-09-17-record-form.md#A27)（補足の行の名前を英語に。理由 → why、却下 → rejected、決めた人 → decided_by、改めた → superseded_by、決める人 → decides、関係 → related）

- A2 理由が書けない決定は「- 理由: 記録なし」と書き、無いことを見えるようにする
  - why: 捏造しないための逃げ道。代替案や理由は記録するものであって発明するものではない（DeepSeek Harness という別のリポジトリの設計記録「Agent Note」の規則「代替案は記録するものであって発明するものではない」に倣う。kakoi 側のメモ 2026-09-16-deepseek-harness-harvest.md の C2）
  - decided_by: 利用者（推奨を採用）
  - superseded_by: [record-form の A27](./2026-09-17-record-form.md#A27)（逃げの値を `not recorded` に）

- A3 決定を改めたら、古い行の下に「- 改めた: リンク」を足す。別のファイルの決定で改めた場合も、古いファイルに書き足してよい。Revisions の節も時系列として残す
  - why: 古い行を読んだ人がその場で新しい決定に飛べる。href は A10 の形（文書からの相対パスと番号）。check にリンク先の実在を見せるには、出典の判定とは別に文書からの相対パスを解決する規則が要る（U2）
  - decided_by: 利用者（推奨を採用）

- A4 記録の節の見出しは英語のまま（決定の節の4つ Agreements、Prohibitions、Delegated、Rejected と、決定の節ではない Undecided、Revisions）
  - why: 見出しは出典の検査の契約で、変えるとコードと TBL-012 と用語集が動く。読みやすさは中身の側で取る
  - rejected: 見出しを日本語にする（R1）
  - decided_by: 利用者（推奨を採用）

- A5 ADR を新しく書く義務をやめる。既存の ADR 4本は消さない（0002 と 0003 は IR の出典の先。0001 と 0004 も履歴として残す）。IR の [REQ-097](../../ir/decision-records.md#REQ-097) を「既存の ADR を消さない」に、[REQ-092](../../ir/decision-records.md#REQ-092) を「判断の記録と、書かれた ADR が残る」に改め、[records.md#R2](./records.md#R2)（判断の記録だけにして ADR をやめる、の却下）を部分的に改める（新規作成をやめ、既存は残す）。[ADR-0003](../adr/0003-records-and-adr.md) の冒頭に改訂の注記を足す
  - why: R2 の理由「1ファイルに全部入り、粒度の考え方と衝突する」は、記録が brainstorm ごとの別ファイルになった時点で無くなった。ADR に残っていた固有の仕事（理由の詳細、改訂の統合先）は A1 と A3 が引き受ける
  - rejected: 4つの基準を壁打ちの手順に組み込んで ADR を書き続ける（R3）
  - decided_by: 利用者

- A6 記録の冒頭は「## 背景と目的」の節にし、初見の人向けに「何が問題で、なぜ今決めるか」を3〜6行で書く。再開のための Position 行はその節の本文の直後に残す
  - why: ADR の「状況」の節が担っていた入口の役を記録の冒頭に移す
  - decided_by: 利用者（推奨を採用）
  - superseded_by: [record-form の A27](./2026-09-17-record-form.md#A27)（見出しを `## Context` に）

- A7 「理由の無い決定」を kotowari の check で機械検査することは今回はしない。スキルの手順と承認前のレビューの観点に置き、機能の候補として U1 に残す
  - why: kotowari は今、記録を出典の先としてしか読んでおらず、記録の形の検査は新しい責務になる
  - decided_by: 利用者（推奨を採用）

- A8 既存の記録は新しい形に書き直さない。新しい記録から適用し、既存の記録には改訂が入ったときだけ A3 の参照を足す
  - why: 番号と行の意味を保つ。理由を掘り返すと捏造の誘惑が強い
  - rejected: 既存の記録を書き直す（R2）
  - decided_by: 利用者（推奨を採用）

- A9 変えるものは、スキルの [references/workflow.md](../../../skills/kotowari/references/workflow.md) の判断の記録の段落、[スキルの仕様](../../spec/kotowari-skill.md)の該当箇所、IR の [REQ-097](../../ir/decision-records.md#REQ-097) と [REQ-092](../../ir/decision-records.md#REQ-092)、[用語集](../../ir/CONTEXT.md)の ADR の意味、[records.md#R2](./records.md#R2) への「改めた」の参照、[ADR-0003](../adr/0003-records-and-adr.md) の注記。記録の最小の形（`- A1 ` の行と4つの見出し）は今のまま [TBL-012](../../ir/sources.md#TBL-012) が持つ。ba0918-brainstorm スキル本体（agentic-rules）は触らない。文書の改訂は主セッションが直接行い、レビューは読み取り専用の調査エージェント1体
  - why: 変更は文書だけで、cycle に見合わない
  - decided_by: 利用者（推奨を採用）

- A10 記録とスキルの文書で資料をまたいで参照するときは、必ず Markdown のリンクにする。href はその文書からの相対パスに `#番号` か `#見出し` を続けた形（例: 記録の中から `[A28（check-reach）](./2026-09-17-check-reach.md#A28)`）。基準のディレクトリからのパス（出典の形）をそのまま href に書くと、Markdown は文書の場所から解決するので辿れない。IR の文書の中の出典と文書名の参照は kotowari の契約どおり素の形のままで、この規則の対象外
  - why: パスの無い番号は人が咄嗟に追えない。DeepSeek Harness が番号だけの引用を禁じたのと同じ理由（利用者の言葉）
  - decided_by: 利用者

- A11 記録とスキルの文書で IR の ID（REQ-090 など）を挙げるときは、その項目を定義する文書へのリンクを必ず付ける（例: `[REQ-090](../../ir/form-contract.md#REQ-090)`）
  - why: ID だけでは内容を判断できない（利用者の言葉）
  - decided_by: 利用者

- A12 記録に `## Superseded` の節を足す。決定を置き換えたとき、置き換えられた決定の行を Agreements（または Prohibitions）からこの節へ移す。移す行は「改めた:」を必ず持ち、番号は変えない。Superseded は決定の節ではないので、そこの決定を指す出典は source_invalid になる（2026-09-17 に実測。コードの変更は不要）
  - why: Agreements を読んだ LLM や人に、現在の決定しか見えないようにする。古い決定を IR に写す事故を、指示ではなく出典の判定で止める
  - rejected: DeepSeek Harness のように話題ごとのノートを現在形に書き換え続ける（R4。IR が現在の真実を持つので二重になる）
  - decided_by: 利用者（推奨を採用）

- A13 Superseded へ移すのは「置き換え」だけ。Revisions の行に「X は Y を置き換える」か「X は Y に補足する」かを明記し、置き換えのときだけ Y の行を移す。補足のときは Y は元の節に残して「改めた:」を付ける
  - why: 補足まで移すと、有効な決定を指す出典が大量に無効になる。records.md の Revisions 33 行の多くは補足（実測: 改められた決定 29 件のうち 27 件を IR の出典が今も指している）
  - decided_by: 利用者（推奨を採用）

- A14 既存の記録の改められた決定（7本の記録で 38 件）を「置き換え / 補足」に分類し、利用者の承認の後、置き換えを Superseded へ移し、それを指す IR の出典を新しい決定に付け替える。付け替えは照合レビューで裏付けを取る。分類は読み取り専用の調査役が表を作り、利用者が承認する
  - why: 移行しないと Superseded の意味が薄い。IR の出典が古い決定を指したまま残るのは、まさに防ぎたい事故の形
  - decided_by: 利用者（推奨を採用）

- A15 lefthook を入れて、pre-commit で `kotowari check`（終了コード 0）、pre-push で `cargo test` を走らせる。既存の秘密情報の検査のフック（グローバルの pre-commit）はそのまま
  - why: いま commit 時に走るのは秘密情報の検査だけで、出典切れ・テストの無い要求・Superseded を指す出典を止めるものが無い。check は数百ミリ秒で commit 前に置ける。cargo test は時間がかかるので push 側
  - decided_by: 利用者（推奨を採用）
  - superseded_by: [A19](#A19)（テスト側の指摘は push で止める。補足）

- A16 kotowari 自身に記録の形の検査（Superseded の行が「改めた:」を持つ、「改めた:」のリンク先が実在して決定の行である、Revisions の「置き換え」と Superseded の整合、決定の行に理由がある）を足すかは、別の壁打ちに切る。U1 と U2 に統合して置く
  - why: 新しい指摘の種類とコードとテストが要り、文書だけの今回と混ぜない
  - decided_by: 利用者（推奨を採用）

- A17 第2ラウンドの決定はこの記録（decision-log）に足す
  - why: 同じ話題の続き
  - decided_by: 利用者（推奨を採用）

- A18 判断の記録の置き場を `docs/decision/brainstorm` から `docs/decision/records` に改名する。IR の出典、設定、スキルの references の既定値と例、仕様、kotowari 本体の既定（TBL-004 の decisions.records の既定値）をすべて置換し、`kotowari check` が 0 件であることで壊れていないことを確かめる。experiments/ の下（実験の記録）は触らない。既存の記録の本文に現れる旧パスも置換する（履歴の文言は変わるが、解決できる参照を優先する。元の文言は git の履歴が持つ）
  - why: 記録は brainstorm の産物から「判断の記録」（Superseded を持つ履歴）に性格が変わり、同じディレクトリに形の契約 ir-form.md もあって、brainstorm という名前が中身と合わない。records は用語集の「判断の記録」と設定の鍵 decisions.records に対応し、新しい語を増やさない。logs は「文字どおりの履歴」で、記録という語とずれる（利用者の言葉）
  - rejected: docs/decision/logs（利用者の言葉: 記録は履歴ではなく記録）。records.md の R7（置き場を移さない）は「出典の書き換えが要る」が理由で、今回は Superseded の移行で出典を触るのでその理由が消えた
  - decided_by: 利用者

- A19 pre-commit の `kotowari check` はテスト側の指摘（requirement_without_test、test_without_id、invalid_marker、unparsable_file、テストのファイルの unresolved_reference）では止めず、停止とそれ以外の誤りで止める。テスト側の指摘も含めた終了コード0の check は pre-push に置く
  - why: 仕様の承認のコミットは実装より先で、新しい unit の要求はその時点で必ず requirement_without_test を持つ。A15 の「テストの無い要求を止める」と承認の手順（テスト側の指摘は cycle の終端で0にする）が衝突し、2026-09-17 の record-form の承認のコミットが7件の requirement_without_test で止まった。止める場所を push に移せば「テストの無い要求を出荷しない」は保てる
  - rejected: `--no-verify` で飛ばす（ゲートを飛ばす癖がつく）。新しい要求を一旦 review で入れる（IR に嘘を書く）
  - decided_by: 利用者（推奨を採用）

## Prohibitions

（なし）

## Undecided

- U1 記録の形を kotowari の check で検査するか: 決定の行に「理由」があるか、Superseded の行が「改めた:」を持つか、Revisions の「置き換え」と Superseded が整合するか（字下げの行の有無なら機械で見られる）
  - decides: 利用者（別の壁打ち）
  - related: A7、A16

- U2 「改めた:」のリンク先の実在と、それが決定の行であることを kotowari の check で検査するか（文書からの相対パスの解決は出典の判定と別に要る）
  - decides: 利用者（別の壁打ち）
  - related: A3、A16

## Delegated

（なし）

## Rejected

- R1 決定の節の見出しを日本語にする
  - why: 出典の検査の契約が動き、コードと [TBL-012](../../ir/sources.md#TBL-012) と用語集の変更になる（A4）

- R2 既存の記録を新しい形に書き直す
  - why: 番号と行の意味が揺れる。理由の後付けは捏造になりやすい（A8）

- R3 4つの基準を壁打ちの手順に組み込んで ADR を書き続ける
  - why: 基準を当てる判断が毎回増え、LLM は甘くも厳しくも読める。ADR の固有の仕事は記録の側で引き受けられる（A5）

- R4 DeepSeek Harness のように話題ごとに1ファイルのノートを現在形で保つ
  - why: 「今どうなっているか」は IR が機械検査つきで持っているので、ノートにも同じ規律を課すと二重になる。読者が増えたら記録からノートを生成する道は残る（A5）

## Revisions

- A19 は A15 の pre-commit の check を、テスト側の指摘では止めない形に改める（補足）
- [record-form の A27](./2026-09-17-record-form.md#A27) が A1 の補足の行の名前、A2 の逃げの値、A6 の見出しを英語に改めた（2026-09-17。補足）
- A18 は [records.md#R7](./records.md#R7)（判断の記録と IR を docs/decision/ の新しい置き場に移さない）を置き換える。理由だった「出典の書き換えが要る」は、A14 の移行で出典を触るので消えた
- A5 は [records.md#R2](./records.md#R2) を部分的に改める（新しい ADR は書かず、既存は残す）。A5 は [records.md#A22](./records.md#A22) の「両方残す」も「書かれた ADR は残す」に改める
- A10 の href を「出典と同じ基準からのパス」から「その文書からの相対パス」に改めた（承認前。利用者が、基準からのパスでは Markdown のリンクが文書の場所から解決されて全部切れることに気づいたため）
- A1、A4、A5、A6、A9 の文言と A3 の理由を、承認前のレビュー（2026-09-17）の指摘で直した: A1 の適用範囲と「一文」の意味、A4 の「決定の節」の語、A5 の出典の先の事実（4本中2本）と REQ-092、A6 の Position の位置、A9 のリンク
