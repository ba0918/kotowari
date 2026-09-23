# kotowari スキル

kotowari を知らない LLM が、どの工程の中でも、仕様を IR の形で書き、`kotowari check` を回し、指摘に対処できるようにする Claude Code のスキル。判断の記録は `docs/decision/records/2026-09-14-kotowari-skill.md`（A1〜A35、P1、U1〜U2、R1〜R2、Revisions）。この仕様自身は IR ではなく kotowari の検査を受けないが、記録は参照できるように残す。承認は 2026-09-15 に会話の中で得た。計画のレビューで見つかった穴3つ（A30〜A32）は承認の後に足した。2026-09-16 の記録（`docs/decision/records/2026-09-16-skill-follow.md`）で、版の固定をやめ、置き場の再帰・注意・責務で分ける指針に追従した。2026-09-17 の記録（[decision-log](../decision/records/2026-09-17-decision-log.md)）で、判断の記録に理由を必須にし、ADR の新規作成をやめた。2026-09-23 の記録（[workflow-split](../decision/records/2026-09-23-workflow-split.md)）で、brainstorm・plan・cycle・implement の手順を kotowari-* の工程の skill に分け、このスキルから場面 workflow と `workflow.md` を外し、判断の記録の形を reference `records.md` に移し、手元へはコピーでなくシンボリックリンクで入れることにした（A27 を同記録の A1、A2 で改めた）。2026-09-23 の記録（[ir-english-tokens](../decision/records/2026-09-23-ir-english-tokens.md)）で、IR の型の語（節の見出し、項目の欄の名前、用語集の題名と列、問題の記録の節）を英語にし、このスキルの SKILL.md と references の本文を英語にした（A1〜A6）。

## 結果を一文で

`kotowari` という1つのスキルを入れると、LLM は場面ごとに IR と判断の記録の書き方、`check` の使い方、指摘の直し方、印の置き方を読めるようになり、人は IR そのものを読まずに、判断の記録と検査の結果で承認できる。

## 用語

kotowari の用語集（`docs/ir/core/CONTEXT.md`）の意味をそのまま使う。この文書で出てくるものだけ、要点を写す。

- **IR**: 正規化した仕様の Markdown の文書の集まり。設定の `ir` の置き場（既定 `docs/ir`）の下に、ディレクトリの深さに制限なく置く。話題ごとの文書、用語集 `CONTEXT.md`、問題の記録 `FLAGS.md` から成る
- **項目**: 要求（`### REQ-nnn: 名前`）、決定表（`TBL-`）、性質（`PROP-`）、問題の記録（`FLAG-`）。要求は `- kind:`、`- source:`、`- verification:` を持ち、`algorithm` の要求は `- definition:` も持つ。問題の記録は `- kind:`、`- related:`、`- source:` を持つ
- **シナリオ**: `## Examples` の下の gherkin のコードブロックの中の `Scenario:`。直前の行にタグ `@id=EX-nnn`、`@about=ID,...`、`@source=出典`
- **判断の記録**: brainstorm で決めたことを1行1決定で並べたファイル。設定の `decisions.records` の置き場（既定 `docs/decision/records`）の下に置き、決定の節の見出し（`## Agreements`、`## Prohibitions`、`## Delegated`、`## Rejected`）を1つ以上持つ
- **決定の番号**: 決定の節の行の先頭 `- A26 ` の `A26`。英大文字1文字に1桁以上の数字
- **出典**: 項目の元になった決定か ADR の節を指す `パス#印` の文字列。パスは基準のディレクトリからの相対
- **印**: テストに書く `@kotowari[ID, ...]` の並び。コメント記号は見ない。ID は要求に限らず IR の ID
- **除外**: 仕様が列挙した、kotowari が指摘を出さずに読まないか見ないもの。列挙に無い読み飛ばしは作らない
- **指摘**: `check` が出す誤り（終了コード1）と注意（終了コードを変えない。severity は `notice`。行数と要求数の上限超えの2種類）。**停止**は検査に入れないこと（終了コード2。設定の誤り、引数の誤り、読めないファイル、UTF-8 でないファイル）
- **場面**: このスキルの分岐。`setup`、`write`、`check`、`mark`、`mutants` の5つ（A30、[A12（mutants-followup）](../decision/records/2026-09-19-mutants-followup.md#A12)、[A5（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A5)）。シェルで実行するコマンドではなく、SKILL.md が文脈から選んで対応する reference を読む
- **reference**: スキルの `references/` にある文書7つ。SKILL.md から場面に応じて読む

## 範囲

作るもの:

- スキル `kotowari`: `SKILL.md` と references 7つ（`ir-form.md`、`findings.md`、`config.md`、`collate.md`、`mark.md`、`mutants.md`、`records.md`）。元本は kotowari リポジトリの `agent/skills/kotowari/`
- kotowari を使うプロジェクトの `AGENTS.md` に足す、仕様を IR で管理するという一文の雛形（`setup` が書く。A27、[A8（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A8)）

作らないもの:

- 既存の ba0918 のスキル（brainstorm、plan、cycle、implement）への変更。agentic-rules は kotowari に依存しない（A27）
- 工程の手順（brainstorm、plan、cycle、implement などの席が何をどの順で行うか）。リポジトリの `agent/skills/` に kotowari-* の工程の skill として別に置き、このスキルはそれを名指ししない（[A4（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A4)、[A5（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A5)）
- 人間が IR を読むための描画（U1）
- Rust 以外の言語の印の規則（A12。kotowari 本体に問い合わせが足されてから）
- 配布の仕組み（crates.io、CI、ライセンス。実験003の記録の U38）
- ADR の書き方（A21。置き場は作り、あれば出典に使えると書くだけ。2026-09-17 の記録 [A5](../decision/records/2026-09-17-decision-log.md#A5) で、新しい ADR を書く義務そのものをやめた）
- 盲検の判定（記録を隠して IR から決定を復元させる検査。A24）
- kotowari 本体の振る舞いの変更（P1）

## 要求

### R1: SKILL.md は薄く、場面の振り分けだけを持つ

SKILL.md は、frontmatter（`name: kotowari`、`description` に発火語 kotowari、IR、`docs/ir`、`@kotowari`、印、mutants、変異テスト）、目的、道具の有無の確認の手順、場面の選び方（人が名指ししたらそれ。なければ文脈から: 置き場が無い → `setup`、brainstorm の途中 → `write`、`check` の結果を読むとき → `check`、テストを書くとき → `mark`、`kotowari mutants` の結果を読むときと見逃しを調べるとき → `mutants`）、場面ごとに読む reference の表（`setup` → `config.md`、`write` → `ir-form.md` と `records.md`、`check` → `findings.md`、`mark` → `mark.md`、`mutants` → `mutants.md`。`collate.md` は `write` の承認前）、この5つの場面をどの作業で使うか、だけを持つ。IR の形の規則、判断の記録の形、指摘の対処、設定のキー、印の規則は本文に書かず、references に置く。

- 成功の条件: SKILL.md が100行以内。本文に `### REQ-` の形の規則、指摘の種類の名前（`docs/ir/core/findings.md` の TBL-core-008 と TBL-core-009 の値）、設定のキー（`docs/ir/core/config.md` の TBL-core-004 の値）が、reference の表の中以外に現れない。frontmatter に上の発火語がある
- 反例: SKILL.md に「要求は `### REQ-nnn: 名前` の見出しの下に…」の規則が書いてある
- 確かめ方: 人が行数を数え、`rg -n 'REQ-|missing_|decisions\.' SKILL.md` の当たりが frontmatter と表の中だけであることを見る

### R2: 道具の有無の確認で始まり、道具が無ければ止まる

どの場面も、最初に `kotowari --version` を走らせる。コマンドが見つからないか、出力から版が読めないときは、作業を始めずに止まり、導入を求める。対象の版は固定しない（2026-09-16 の記録 A1。本体はローカルで育てていて常に最新。タグとして公開したあとの追従の書き方は同 U1）。references の先頭には版の写しでなく改訂日を書く。

- 成功の条件: SKILL.md の最初の手順が道具の確認で、コマンド無し・版が読めない、の2つの分岐が書いてある。SKILL.md に版の固定が無い。references 7つの先頭に改訂日がある
- 反例: 出力から版が読めないのに `write` の手順に進む。SKILL.md に `0.1.0` のような版が書いてある
- 確かめ方: 人が SKILL.md と references の先頭を読む。PATH から `kotowari` を外してスキルを呼び、止まることを見る

### R3: `setup` は置き場と設定と空の用語集と AGENTS.md の規則を作り、既にあるものを壊さない

`setup` の手順は reference `config.md` に置く（A31）。`setup` は、リポジトリ直下で次を作る。

- `.kotowari/` と `.kotowari/config.yaml`。`.kotowari/` があることで基準のディレクトリがリポジトリ直下に固定される（サブディレクトリから `check` を走らせても同じ基準になる）。設定ファイルは既定と同じ値でも必ず書く（A20。既定を目に見える形にするため）。値を既定から変えるのは、テストの glob（既定 `src/**/*.rs` と `tests/**/*.rs`）か置き場を変えるときだけ
- `docs/ir/`、`docs/decision/records/`、`docs/decision/adr/`
- `docs/ir/CONTEXT.md`（題名の行 `# Glossary`、空行、表のヘッダ `| Term | Meaning | Source |`、区切り行 `|---|---|---|` の4行）
- `AGENTS.md` の節（A27、[A8（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A8)）: 「This project manages its specification as an IR (`docs/ir/`).」の一文だけ。工程の skill には触れない（工程の skill を入れる人が足す1行は、その入れ方の説明にある）。節の見出しは `## kotowari`。`AGENTS.md` が無ければ作り、あれば節を末尾に足す。`## kotowari` の見出しが既にあれば足さない

既にあるファイルやディレクトリは上書きせず、あることを人に言う（`AGENTS.md` への節の追加だけは、上書きでなく追記なので行う）。

- 成功の条件: 空のリポジトリで `setup` の後に `kotowari check` を走らせると、終了コード0で `findings` が空（実測済み: `files` 1、`lines` 4）。サブディレクトリから走らせても同じ。既に `docs/ir/CONTEXT.md` があるリポジトリで実行しても、そのファイルの中身が変わらない。`AGENTS.md` に kotowari の節が1つだけある
- 反例: 既にある `CONTEXT.md` を4行の表で置き換える。`setup` を2回呼ぶと `AGENTS.md` に同じ節が2つできる
- 確かめ方: 人が空のリポジトリと既存のリポジトリで `setup` を呼び、`kotowari check` の出力と `git status` と `AGENTS.md` を見る

### R4: `write` は IR の形の規則と判断の記録の形を丸ごと読ませる

`write` は IR と判断の記録を書くときに使う（brainstorm の席のほか、別の工程で書くときも）。reference `ir-form.md` と `records.md` を読ませる。

`ir-form.md` は、現在の kotowari の仕様どおりに、次の節を持つ: 文書（置き場の再帰、題名、範囲の行、節、行の数え方、コードブロック、BOM、README を置かない、ディレクトリ名の文字、path と detail の作り方）、ID、項目（4種類と持つ行）、シナリオ（タグ、許されるステップの行）、用語集の表の形と連鎖、問題の記録、出典（`パス#印`、判断の記録の決定の番号、ADR の `## ` の見出し）、用語と曖昧語（バッククォートで囲んでよいのは用語と ID だけ、二重引用符の中は見ない）、文書名の参照（同じディレクトリを指す素の形と、置き場からの `/` の形）、除外の一覧、上限と分ける単位（数字を持たず、too_many_lines と too_many_requirements の読み方と、責務で分ける指針。2026-09-16 の記録 A2、A3）。

`records.md` は、判断の記録の形と、中身の種類ごとの置き場を持つ（A11、A21、A22 のうち形の部分。[A11（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A11)）。brainstorm の手順（出力を IR にすること、記録を最初から置き場に書くこと、未コミットの記録からの再開、終わりのレビューの差し替え、stage するもの）は工程の skill の側にあり、このスキルは持たない。

- 判断の記録の形: 置き場とファイル名（`docs/decision/records/YYYY-MM-DD-<name>.md`。ファイル名は最初に決めて改名しない。承認後も消さない）、冒頭の `## Context` の節と Position 行、決定の節の見出し4種の下に `- A1 本文` の形で並べること（番号は使い回さない）、決定ごとの字下げの補足の行6つ（`- why:`、`- rejected:`、`- decided_by:`、`- superseded_by:`、`- decides:`、`- related:`）とその必須、補足の行が決定として読まれない理由、`Undecided` と `Revisions` の番号は出典に使えないこと、資料をまたぐ参照と IR の ID へのリンクの規則、ADR の扱い。2026-09-17 の記録（[decision-log](../decision/records/2026-09-17-decision-log.md) の A1〜A6、A10、A11）で足した規則を、今の英語の見出しと補足の行の名前で持つ。必須の補足の行とリンクの検査の指摘は `findings.md` で引く
- 成功の条件と反例の置き場: 各要求に求める観測できる成功の条件と反例は、IR では `## Examples` のシナリオ（成功の条件は通る場面、反例は指摘か停止が出る場面）として書く。要求の見出しの下には持てる行しか置けない
- 禁止・却下・未決・委譲の置き場: IR の話題ごとの文書には要求・決定表・性質・具体例しか置けないので、これらは判断の記録の節に置く。矛盾・欠落・曖昧は `FLAGS.md` の `### FLAG-nnn: 名前` に `- kind:`（`contradiction`、`gap`、`ambiguity`）、`- related:`（関係する ID）、`- source:` を付けて書く（A21）
- 用語集の連鎖: 用語集はどのディレクトリにも置け、文書から見えるのは自分のディレクトリから置き場の根までの `CONTEXT.md` の用語。用語は連鎖の中で1つのファイルにだけ置く
- 成功の条件: `ir-form.md` を読んだ LLM が、新しい要求1件、決定表か性質1件、シナリオ1件、用語1語、問題の記録1件を書き、それらが `ir-form.md` の項目の形（見出し、持つ行、タグ）に一致して、`kotowari check` の誤りがテスト側の指摘だけになる。判断の記録が `docs/decision/records/` にあり、出典がその決定の番号を指している。`ir-form.md` の節の一覧が上の列挙と一致し、`records.md` に上の4つの項目があり、各節の内容が `docs/ir/core/` の対応する文書（ir-document、ir-items、ir-references、terms-form、terms、sources、CONTEXT の除外）と食い違わない
- 反例: 出典が `.agents/tmp/` の進捗ファイルを指している（`decisions.records` の外を指す出典は、ファイルがあっても `source_invalid` になる）。`U3` を出典に使う。要求の見出しの下に `- 反例:` の行を置く
- 確かめ方: 人が kotowari を知らない別セッションの LLM に `write` を読ませて上の5つと判断の記録を書かせ、`check` の出力と、書き上がったものの形を見る。人が `ir-form.md` の節を `docs/ir/` の文書と突き合わせる

### R5: 承認の関門は IR 側の誤り0と照合レビューの裏付け無し0

承認を求める直前に次を順に行う（A13、A18、A24、A25、A26、A33、A35）。この手順は工程の skill の brainstorm の側が持ち（[A11（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A11)）、このスキルは手順に要る reference（`findings.md`、`collate.md`）を持って、`write` の承認の前に `collate.md` を読ませる。

1. `kotowari check --format json` を走らせる。終了コードを先に見て、2なら停止として R6 のとおり扱う。`findings` のうち `path` が IR の置き場か判断の記録の置き場のファイルである誤りが0になるまで直す（テスト側の指摘、つまり `requirement_without_test`、`scenario_without_test` とテストのファイルへの指摘は、承認の時点では残ってよく、cycle の終端で0にする。A33）。注意は残してよい（残す理由を記録に書く）
2. 照合レビュー: reference `collate.md` の指示で照合の範囲（その壁打ちで足したか変えた項目と、変えた決定を出典に持つ既存の項目）を数え、件数と照合するかどうかを1行書く。0件なら照合しない。1件以上なら別セッションの LLM に範囲の項目（要求、決定表、性質、シナリオ、用語、問題の記録）とその出典の決定を渡し（[A1（collate-tiers）](../decision/records/2026-09-23-collate-tiers.md#A1)、[A2（collate-tiers）](../decision/records/2026-09-23-collate-tiers.md#A2)）、出典の決定が項目の内容を裏付けていないものを挙げさせる。挙がったものは、出典を足すか、記録に決定を足すか、項目を直すかして、手順1（check）に戻ってから再び照合する（A35）。2回目と3回目は前の回で挙がった項目と直したものだけを渡す（[A3（collate-tiers）](../decision/records/2026-09-23-collate-tiers.md#A3)）。3回目の照合でも残るものは問題の記録（FLAG）にして人に返す
3. 人に見せるもの: 判断の記録の差分（1行1決定で人が読む対象）、`check` の出力（テスト側の指摘の件数と、注意を残した文書とその理由）、照合レビューの結果、承認の対象のパスと内容の識別子（IR の文書、用語集、問題の記録、判断の記録）。IR の差分を読むことは求めない

- 成功の条件: 承認の依頼の文に、check の終了コード、テスト側の指摘の件数、注意を残した文書と理由、照合レビューで挙がった件数（0か、FLAG にした件数）、承認の対象のパスと識別子がある。IR 側の誤りが残っている状態で承認を求めていない。照合の後に IR を変えたら check をやり直している
- 反例: 照合レビューを飛ばして「check が通ったので承認してください」と言う。IR の差分を読んで承認するよう求める。4回目の照合レビューを回す
- 確かめ方: 人が承認の依頼の文を読む

### R6: `check` は json で走らせ、指摘の種類ごとに対処と担当を引き、停止は人に返す

`check` は `kotowari check --format json` を走らせ（A15）、終了コードを先に見る。0か1なら標準出力の JSON を読み、`findings` の各件を reference `findings.md` の表で引く。表は現在の kotowari の指摘の種類すべて（誤り33、注意2）を行に持ち、各行に「Meaning」「Action」「Owner」の列がある。表の1列目の見出しは `Kind`、停止の文言の表の1列目の見出しは `Message` で、このリポジトリのテストがこの見出しで表を探す（[A6（ir-english-tokens）](../decision/records/2026-09-23-ir-english-tokens.md#A6)）。

- 担当は表の「Owner」の列を正とする（A7）。IR の置き場のファイルへの指摘は brainstorm の席が直す。テストのファイルへの指摘（`test_without_id`、`invalid_marker`、`unparsable_file`、印からの `unresolved_reference`）は implementer が直す。`unresolved_reference` は IR 側からも出るので、表は `path` で分けて2行持つ。`requirement_without_test` は `path` が IR の文書だが、直すのはテストを書く implementer なので、表で例外として明記する
- 除外の追加や規則の緩めは仕様の変更なので、スキルの中で決めず brainstorm に戻す（A7）
- `too_many_lines` と `too_many_requirements` の対処は分割の指示ではなく、責務の混在を疑う読み直し（2026-09-16 の記録 A3）。用語集はディレクトリごとに分けられなければ設定の `limits.lines` を上げ、その判断を記録に書く（A34）
- 停止は終了コード2で、理由は標準エラーの1行目の文言（`config error`、`argument error`、`unreadable file`、`non-UTF-8 file`）で判別する。引数の誤りは LLM 自身のコマンドの誤りなので自分で直す。残り3つは人に返す
- 成功の条件: `findings.md` の行が `docs/ir/core/findings.md` の TBL-core-008 と TBL-core-009 の種類と過不足なく対応し（`unresolved_reference` は2行）、各行に対処と担当がある。停止の4つの文言と対処が別の表にある。手順の最初が終了コードの確認である
- 反例: `unknown_term` を「バッククォートを外す」で対処する（用語集に足すか、言い換えるかは仕様の判断で、外すだけでは意味が変わる）。設定の誤りを LLM が設定ファイルを書き換えて黙って直す。停止のときに空の標準出力を JSON として読もうとする
- 確かめ方: 人が `findings.md` の表と `docs/ir/core/findings.md` を突き合わせる

### R7: `mark` は Rust の印の規則を持ち、その本文は `mark.md` にある

`mark` は implementer と fixer が使う。reference `mark.md`（A29）に次を書く。

- 印の形: `@kotowari[REQ-core-001, TBL-core-002]`。カンマ区切りで複数の ID。コメント記号は問わない
- 印の置ける位置（kotowari の TBL-core-016 のとおり）: テスト関数とその属性の直前に続くコメントの塊（属性を挟んでよい。空行を挟むと切れる）、または関数の本体の先頭でどの文よりも前にあるコメントの塊。本体の途中と、テストの外の印は結び付かない
- テストの見分け方（TBL-core-017 のとおり）: 属性のパスの末尾の要素が `test` の関数（`#[test]`、`#[tokio::test]`）、設定の `tests.rust.attributes` に挙げた属性の関数、`tests.rust.macros` に挙げたマクロの中の関数
- テスト名の慣習（A19。kotowari は検査しない）: 確かめる ID を小文字にしてハイフンを `_` に変え、先頭に付ける（`req_001_...`）
- Rust 以外の言語では、IR 側の検査だけを使い、テスト側は人が確かめる（A12）
- 成功の条件: `mark.md` を読んだ LLM が書いたテストに `check` が `test_without_id` と `invalid_marker` を出さない。`mark.md` の本文に、上の5つの項目がある
- 反例: 印のコメントとテスト関数の間に空行がある。印を関数の本体の途中に置く。`#[tokio::test]` の関数に印を付けない
- 確かめ方: 人が別セッションの LLM に `mark.md` を読ませてテストを1本書かせ、`check` の出力を見る。人が `mark.md` の本文に5つの項目があることを見る

### R8: 工程の手順はこのスキルに置かない

brainstorm、plan、cycle、implement の各席が何をどの順で行うかは、このスキルではなく、リポジトリの `agent/skills/` にある kotowari-* の工程の skill の本文にある（[A4（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A4)）。依存は工程の skill からこのスキルへの一方向で、工程の skill が場面ごとにこのスキルの reference を読み、このスキルは工程の skill を名指ししない（[A5（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A5)）。このスキルだけを入れて別の工程で使っても、IR と判断の記録を書き、`check` の結果を読み、印を置ける。

- 成功の条件: SKILL.md の場面の表に工程の手順を読ませる行が無く、references に席ごとの手順の節を持つ文書が無い。`agent/skills/kotowari/` の中に kotowari- で始まる工程の skill の名前が無い
- 反例: SKILL.md が「plan の席では工程の skill の plan を読め」と書く。references に cycle の席の終端の手順の節がある
- 確かめ方: 人が SKILL.md と references を読み、`rg -n 'kotowari-[a-z]' agent/skills/kotowari/` の当たりが無いことを見る

### R9: references は現在の kotowari の仕様を、実験の記録を参照せずに書く

references 7つ（`ir-form.md`、`findings.md`、`config.md`、`collate.md`、`mark.md`、`mutants.md`、`records.md`）は、kotowari の IR（`docs/ir/`）と形の契約から作るが、kotowari リポジトリの実験の記録（`experiments/` の下）への参照を含めない（A1。配布先で元本は読めない）。決定の番号や出典の例は架空のパス（`docs/decision/records/2026-01-01-example.md#A1`）で書く。kotowari 自身の IR の ID（`TBL-core-016` の類）を根拠として引かず、`docs/ir/` の文書名も引かない（配布先で解決できない。A32。例に使う ID は形の説明なので可）。元にする `docs/ir/` と形の契約が食い違えば `docs/ir/` が正で、実測で確かめられるなら確かめ、決まらなければ止まって人に言う（A32）。kotowari の仕様が変わったら references を更新し、先頭の改訂日を改める。SKILL.md と references の本文は工程の skill と同じ英語で書き、IR の型の語は英語の語で説明する。設定の `vague_words` の既定は本文の言語の話で型の語ではないので、日本語の4語のまま写す（[A4（ir-english-tokens）](../decision/records/2026-09-23-ir-english-tokens.md#A4)、[A5（ir-english-tokens）](../decision/records/2026-09-23-ir-english-tokens.md#A5)）。

- 成功の条件: `rg -n 'experiments/|docs/ir/[a-z-]+\.md' agent/skills/kotowari/` が0件（置き場の名前 `docs/ir` と、利用者側に作る `docs/ir/CONTEXT.md`・`docs/ir/FLAGS.md` は書いてよい）。人が読んで、kotowari 自身の IR の ID を「〜のとおり」のように根拠として引いている箇所が無い。`config.md` に設定ファイルの全キー（`ir`、`decisions.records`、`decisions.adr`、`tests.files`、`tests.rust.attributes`、`tests.rust.macros`、`vague_words`、`limits.lines`、`limits.requirements`）と既定値がある。`collate.md` に、渡す入力（項目と出典の対）、判定の基準（出典の決定が項目の内容を裏付けるか）、返す形（裏付けの無い項目の一覧）、回数の上限（3回）がある
- 反例: `ir-form.md` に `experiments/003-cli/brainstorm/records.md#A145` が残っている。`mark.md` に「TBL-core-016 のとおり」と書いてある
- 確かめ方: 上の `rg`。人が `config.md` のキーを `docs/ir/core/config.md` の TBL-core-004 と突き合わせ、`collate.md` の4つの要素を見る

### R10: 元本の置き場と入れ方

元本は kotowari リポジトリの `agent/skills/kotowari/`（`SKILL.md` と `references/` の7つ）。手元へは `~/.claude/skills/kotowari` にリポジトリの `agent/skills/kotowari/` へのシンボリックリンクを置いて入れる（A5、A23 を [A7（工程の分離）](../decision/records/2026-09-23-workflow-split.md#A7) で改め、置き場を [A1（配布）](../decision/records/2026-09-23-skill-distribution.md#A1) で改めた）。他の人へは APM・gh skill・npx skills のどれか1行で入れる（[A2（配布）](../decision/records/2026-09-23-skill-distribution.md#A2)）。

- 成功の条件: kotowari リポジトリ以外のプロジェクトで、`~/.claude/skills/kotowari` のリンクからスキルを呼んだとき、スキルのディレクトリの外にある kotowari リポジトリのファイルを1つも読まずに5つの場面が動く
- 反例: SKILL.md が `../../docs/ir/...` を読ませる
- 確かめ方: 人が別のプロジェクトでスキルを呼び、読んだファイルの一覧を見る

## 決めていないこと

- U1 人間が IR を読める形にする方法（実験003では描画を作らない禁止があった。決める: 別の壁打ち）
- U2 kakoi-net での試用の結果で `setup` の既定（置き場の名前）を変えるか（決める: 試用の後、利用者）

## 禁止

- P1 このスキルのために kotowari 本体の振る舞いを変えない

## 却下したもの

- R1 知識を kotowari 本体（`--help` や `init`）に持たせる（配布の形が未定で、スキルの references なら道具を変えずに更新できる）
- R2 このスキル自身の仕様を IR の形で書く（ただの文書に kotowari の規則を課すのは過剰。A10）
- 既存の ba0918 のスキルに kotowari への1行を足す（agentic-rules は kotowari に依存しない。A27）

## 人が判断する場面

- R2 の道具の不在（導入するか）
- R6 の停止のうち設定の誤り、読めないファイル、UTF-8 でないファイル
- 指摘への対処が仕様の変更を伴うとき（除外の追加、規則の緩め）は brainstorm に戻す。cycle の途中で IR 側の指摘が出たときも同じ
- R5 の承認（判断の記録の差分と検査の結果を見て決める）
- R5 の照合レビューが3回で収束しなかった項目（FLAG）
