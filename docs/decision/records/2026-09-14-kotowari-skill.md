# 壁打ちの進捗: kotowari スキル（LLM に kotowari と IR の知識と使い方を注入する）

目的: 既存の ba0918 のワークフロー（brainstorm → 仕様 → plan → cycle）で、kotowari を知らない LLM が IR を書き、check を回し、指摘に対処できるようにするスキルを作る。

## Agreements
- A1 知識の文書はスキルの references に置く。配布したら元本（kotowari リポジトリ）は参照できないと考える。現状の仕様を書き、変更があれば更新する（利用者、2026-09-14）
- A2 スキルは `kotowari` という単体スキル。分岐は `kotowari <sub-command>` の形。SKILL.md は薄く、必要なときに references を読ませる（利用者）
- A3 IR が仕様そのものになる（brainstorm の出力を最初から IR の形で書く）。ただし人間が今の IR を読むのは不可能に近い、という懸念がある（利用者）
- A4 スキルを作ってから kakoi-net で試用する（利用者）
- A5 スキルの元本は kotowari リポジトリの `skills/kotowari/`。手元へはコピーで入れる（推奨を採用）
- A6 sub-command は `setup`、`write`、`check`、`mark` の4つから始める。追加は摩擦が出てから（推奨を採用）
- A7 指摘の種類ごとに「対処」と「担当」の表を持つ。IR 側は brainstorm の席、テスト側は implementer、停止は人。除外の追加は仕様の変更なので brainstorm に戻す（推奨を採用）
- A8 references は `ir-form.md`（形の規則）、`findings.md`（種類ごとの意味と対処）、`config.md`（設定）の3つ。sub-command ごとに読むものを SKILL.md に書く（推奨を採用）
- A9 references の先頭に対象の kotowari の版を書き、SKILL.md の最初で `kotowari --version` を確かめ、違えば人に言う（推奨を採用）
- A11 判断の記録は `docs/decision/records/<name>.md` に永続化する。kotowari を使うときは brainstorm の「承認後に進捗ファイルを消す」約束を適用せず、kotowari スキル側にそう書く。既存の brainstorm スキル本体は変えない（推奨を採用）
- A12 対応言語は今は Rust だけと明記。`mark` は Rust の印だけ。他の言語では IR 側の検査だけ使い、テスト側は人が確かめる。言語の追加は kotowari 本体の壁打ち（U37）の後（推奨を採用）
- A13 check の関門は2か所。brainstorm の承認を求める直前（指摘0を承認の証拠に含める）と、cycle の終端報告の直前（requirement_without_test と unresolved_reference が0）。plan では走らせない（推奨を採用）
- A14 `setup` は置き場3つと空の用語集（ヘッダと区切りだけ）を作る。設定ファイルは既定と違うときだけ。既にあるものは上書きしない（推奨を採用）
- A15 LLM が読む出力は `--format json` を既定にする（text は指摘0のとき何も出さない。json は件数と種類が構造で取れる）（推奨を採用）
- A16 既存スキルには「仕様が IR の形なら kotowari スキルが次の手順を置き換える」の1行を足し、置き換える手順の中身は kotowari スキル側に書く。brainstorm: 出力は IR の文書群、判断の記録は最初から docs/decision/records に書いて消さない、用語集は IR の置き場の CONTEXT.md、承認時に記録も stage。plan: 入力は IR の置き場と対象の要求 ID の一覧、最後のステップの確認コマンドに kotowari check を列挙（plan 自身は走らせない）。cycle: implementer と fixer のプロンプトに印の規則を貼り、終端報告に check の結果を載せる。implement: plan が列挙した check を走らせる（推奨を採用）
- A17 分岐は「場面」と呼び、SKILL.md が文脈から選ぶ（人が名指ししてもよい）。frontmatter の description に発火語（kotowari、IR、docs/ir、@kotowari、印）を入れる（推奨を採用）
- A18 承認の関門は誤り0（終了コード0）。警告は承認の証拠に載せて人に見せる（推奨を採用）
- A19 テスト名は ID を小文字にしてハイフンを "_" に変えて先頭に付ける慣習。検査はしない（推奨を採用）
- A20 setup は .kotowari/config.yaml を必ず作る（既定と同じ中身でもよい）。基準のディレクトリを固定するため（推奨を採用）
- A21 問題の記録（FLAGS.md）は write の範囲。ADR は範囲外（置き場だけ作り、あれば出典に使えると書く）（推奨を採用）
- A22 判断の記録のファイル名は docs/decision/records/YYYY-MM-DD-<name>.md（REQ-093 にそろえる）（推奨を採用）
- A23 追認: SKILL.md は100行以内、版は完全一致、入れ先は ~/.claude/skills/kotowari/、kotowari コマンドが無いときは導入を求めて止まる（推奨を採用）
- A24 承認の前に照合レビューを入れる。write の承認前の手順は「check で誤り0 → 照合レビュー（別セッションの LLM が IR の各項目を判断の記録と突き合わせ、裏付けの無いものを挙げる）で0 → 人には判断の記録の差分と check の結果と照合の結果を見せる」。IR の差分は承認の対象のバイト列として添えるが読むことは求めない。盲検の判定は入れない（推奨を採用）
- A25 承認の関門は「requirement_without_test 以外の誤りが0」。requirement_without_test は cycle の終端で0にする。kotowari 本体には手を入れない（推奨を採用）
- A26 照合レビューは3回まで。3回目でも裏付けの無い項目が残れば問題の記録（FLAG）にして人に返す（推奨を採用）
- A27 既存の ba0918 のスキルには何も書かない。agentic-rules は kotowari に依存しない。結び付けは、kotowari を使うプロジェクトの AGENTS.md にルーティングとワークフローの規則を書くことで行い、まずはそれで様子を見る（利用者）
- A28 照合レビューが brainstorm の終わりの「記録への適合」のレビューを置き換える。「仕様の品質」のレビュー1本は残す（推奨を採用）
- A29 references を6つにする。mark.md（印の規則。cycle が implementer と fixer のプロンプトに貼る）と workflow.md（brainstorm・plan・cycle・implement の手順の置き換え）を足す（推奨を採用）
- A30 場面を5つにして workflow を足す。plan・cycle・implement の席は workflow として workflow.md の自分の節を読む。brainstorm の席は write のまま（計画のレビューで導線の欠落が見つかった。推奨を採用）
- A31 setup の手順（作るもの、AGENTS.md の節の雛形、既にあるものの扱い）は config.md に置く（推奨を採用）
- A32 references を書くとき docs/ir と実験の契約が食い違えば docs/ir が正。実測で確かめられるなら確かめ、決まらなければ止まって人に言う。references の中では kotowari 自身の ID（TBL-016 の類）と docs/ir のパスを引かない（配布先で解決できない。R10 の帰結。推奨を採用）
- A33 承認の関門は「IR の置き場のファイルへの誤りが0」。requirement_without_test を含むテスト側の指摘は cycle の終端で0にする（既にテストがあるプロジェクトで印の無いテストが承認を止めないため。cycle の human_judgment。推奨を採用）
- A34 用語集に too_many_lines が出たときの対処は、設定の limits.lines を上げてその判断を記録に書く（用語集は1ファイルで分割できない。警告なので承認は止めない。推奨を採用）
- A35 照合レビューで IR か記録を変えたら承認の手順1（check）に戻る。3回の上限は照合の回数のまま（推奨を採用）
- A10 このスキル自身の仕様は IR にせず、自由な Markdown（`docs/spec/kotowari-skill.md`）で書く。ただの文書に kotowari の規則を課すのは過剰（利用者）

## Prohibitions
- P1 このスキルのために kotowari 本体の振る舞いを変えない（A23 で追認）

## Undecided
- U3 段階的な導入の仕組み（PHPStan の level のように、設定で検査の厳しさや有効な検査を段階で選べるようにする。今は0か100しかない。利用者の着想。決める: kotowari 本体の次の壁打ち）
- U2 kakoi-net での試用の結果で setup の既定（置き場の名前）を変えるか（決める: 試用の後、利用者）
- U1 人間が IR を読める形にする方法（A3 の懸念。決める: 別の壁打ち。実験003では render を作らない P1 があった）

## Delegated
（なし）

## Rejected
- R1 知識を kotowari 本体（--help や init）に持たせる（配布の形が未定で、スキルの references なら道具を変えずに更新できる。A23 で追認）
- R2 このスキル自身の仕様を IR の形で書く（A10）

## Revisions
- A17 は A2 の「kotowari <sub-command> の形」を「場面」に改める（CLI のコマンドと紛れるため）
- A20 は A14 の「設定ファイルは既定と違うときだけ」を「必ず作る」に改める
- A24 は A8 の references 3つに、照合レビューの指示 `collate.md` を4つ目として足す
- A27 は A16 の「既存スキルに1行足す」を「AGENTS.md に書く」に改める（A11 の「既存の brainstorm スキル本体は変えない」はそのまま）
- A29 は A8 の references を6つ（ir-form、findings、config、collate、mark、workflow）に改める
- A30 は A6・A17 の場面4つを5つに改める
- A33 は A25 の「requirement_without_test 以外の誤り0」を「IR の置き場のファイルへの誤り0」に改める
- A25 は A13 の brainstorm 側の関門「指摘0」を「requirement_without_test 以外の誤りが0」に改める（A18 の「誤り0」も同じ）

## 敵対的レビュー（2026-09-14、下書き1版に対して）
- 仕様の品質: 35件（S1〜S35）。記録との適合: 決定の反映は A3 が missing、黙って決めた点15、既存スキルとの衝突10、用語のずれ6。
- 決定なしで直すもの（下書き2版で反映）: 指摘の種類の数（35）、停止の理由4つ、tests.files の既定は2つの glob、unresolved_reference の担当は path で分ける、停止の判別は終了コード2と標準エラーの1行目、引数の誤りは LLM が自分で直す、曖昧語（など）、印の置ける位置は TBL-016 のとおり（コメントの塊、属性を挟める、本体の先頭も可）、用語の定義を CONTEXT.md に合わせる（IR、判断の記録、出典、印、除外）、欄に定義と関係を足す、囲んでよいのは用語と ID、ir-form の内訳にシナリオと用語集の表と上限を足す、決定の節の見出し4種と行の形と「Undecided/Revisions は出典にできない」、setup の CONTEXT.md は題名と表のヘッダと区切り、references の禁止は experiments/ への参照に限る（決定の番号の例は架空のパス）、A2→A11 の出典、U37/U38 は実験003の記録の番号、R6 の成功の条件を項目ごとに、範囲の「作らないもの」から既存スキルへの追記を外す、R8 の確かめ方
- 決定が要るもの: 次のラウンド Q16〜Q23

## 次のラウンド
（Q16〜Q24 は推奨で決定済み。下書き2版 → 適合レビュー1本 → 承認）
- Q16 既存スキルとの結び付け方（1行では足りない。S3、S6、S15、S17、S18、適合レビューの衝突10件）
- Q17 分岐の名前（sub-command は CLI と紛れる。S7、S8）と発火語
- Q18 承認の関門は「誤り0」か「指摘0（警告も無し）」か（S12）
- Q19 テスト名の規則（S19。記録に無い）
- Q20 setup が `.kotowari/config.yaml` を必ず作るか（基準のディレクトリを固定する。S23）
- Q21 FLAGS.md と ADR を write の範囲に入れるか（S24、S34）
- Q22 判断の記録のファイル名に日付を付けるか（REQ-093。S33）
- Q23 黙って決めた点の追認（SKILL.md 100行、版は完全一致、入れ先 ~/.claude/skills/kotowari、却下「知識を道具に持たせる」、禁止「kotowari 本体の振る舞いを変えない」、コマンドが無いときは導入を求めて止まる）
