# 壁打ちの記録: kotowari の工程を ba0918 の workflow から切り出す

## Context

kotowari 方式の brainstorm / plan / cycle / implement は今、ba0918 の workflow の各 skill（ソースは agentic-workflow のリポジトリ）に、kotowari スキルの `skills/kotowari/references/workflow.md` で上書きをかぶせて動いている。この結び付け方は [A27（kotowari-skill）](./2026-09-14-kotowari-skill.md#A27) で「既存の ba0918 のスキルには何も書かず、AGENTS.md の規則で結ぶ。まずはそれで様子を見る」と決めたものである。

様子を見た結果、上書きは「先に reference を読めば効く」という運用に頼っていて、2026-09-22 に `ba0918-brainstorm` がそのまま回り、出力が `docs/spec/` に出て IR へ移し直す手戻りが出た。基本の考え方は同じでも細部の方針が食い違うので、相乗りをやめて kotowari 専用の工程の skill に分けるかを決める。

Position: 第2ラウンドまでで構造の論点を決め（A1〜A7）、計画を立てる前に見つかった3つを第3ラウンドで決めた（A8〜A10）。A4 と A5 の食い違いを A11 で解いた。実装のレビューで見つかった2つを A12〜A14 で決めた。承認済み（2026-09-23）。

## Agreements

- A1 agentic-workflow の工程の skill 8つ（brainstorm、plan、cycle、implement、review、iterate、investigate、using-workflow）をすべて kotowari 用に分ける
  - why: 1つでも ba0918 の skill を名指しで使うと、kotowari を使うプロジェクトに agentic-workflow の導入が必須になる。kotowari 用の skill は推奨であって、それ無しでも kotowari が使えるようにしたい
  - rejected: 上書きのある4つ（brainstorm、plan、cycle、implement）だけを分け、review、iterate、investigate、using-workflow は ba0918 のものを使う。iterate が ba0918-cycle の loop を名指しし、入口の表も ba0918-brainstorm を名指しするので、agentic-workflow への依存が残る
  - decided_by: 利用者

- A2 kotowari を使うプロジェクトの入口は kotowari-using-workflow が決め、そのプロジェクトの AGENTS.md は ba0918-using-workflow の代わりにこれを読ませる
  - why: agentic-workflow と同じ流れにする。入口が ba0918-using-workflow のままだと、入口の表が ba0918-brainstorm を名指しして、2026-09-22 の事故の道が残る
  - rejected: AGENTS.md に「ba0918-* の代わりに kotowari-* を使う」と書くだけにする。先に読めば効くという運用に頼る点で、今の上書きと同じ弱さを持つ
  - decided_by: 利用者（推奨を採用）

- A3 kotowari 用の skill は agentic-workflow の skill を書き写して独立させ、以後 ba0918 側に追従させない。kotowari の skill から ba0918 の skill を読ませることも、道具で写して同期することもしない
  - why: 共有させると、本格的に袂を分かったときに困る
  - rejected: 共通部分は ba0918 の skill を読ませ、差分だけを持つ。今の上書きと同じ構造に戻る
  - rejected: agentic-skill-vendor で ba0918 の文書を写して追従させる。共有の一形態で、袂を分かったときに外せなくなる。また、kotowari 版は本文の途中が違うので文書を丸ごと写す形に合わない
  - decided_by: 利用者

- A4 分けた skill はリポジトリの `skills/` に `kotowari-brainstorm` のような名前で8つ並べ、kotowari スキルと一緒に配る。`skills/kotowari/references/workflow.md` は中身を各工程の skill へ移して消す
  - why: kotowari スキルと同じく他のプロジェクトへ配るもので、元本を1か所に置く。workflow.md が担っていた上書きは、各工程の skill の本文そのものになるので要らなくなる
  - decided_by: 利用者（推奨を採用）

- A5 依存は工程の skill から kotowari スキルへの一方向にする。工程の skill は場面ごとに kotowari スキルを読み、kotowari スキルは工程の skill を名指しせず、場面 workflow を外す。工程の skill の間で名指しし合うことは妨げない
  - why: 工程の skill は kotowari を前提にするので、kotowari に縛られても困らない。逆向きを断つことで、kotowari スキルだけを入れて別の工程で使う道が残り、A1 の「工程の skill は推奨」が成り立つ
  - decided_by: 利用者（推奨を採用）

- A6 この決定のために新しい IR は作らない。[A27（kotowari-skill）](./2026-09-14-kotowari-skill.md#A27) は A1 と A2 で改める。kotowari スキルとその仕様 `docs/spec/kotowari-skill.md` は、工程の分離に合わせて書き換えてよい
  - why: 8つの skill の中身は、写す元の agentic-workflow の skill と今の workflow.md の差分で既に決まっている。kotowari スキルは場面 workflow を外すなど、分離で少し変わる
  - decided_by: 利用者（推奨を採用。仕様の書き換えを許すのは利用者の言葉）

- A7 手元へは `~/.claude/skills/` にリポジトリの `skills/` の各ディレクトリへのシンボリックリンクを置いて入れる。他の人への配布の仕組みはこの記録では決めない
  - why: コピーで入れた kotowari スキルは既にリポジトリより古くなっていた（手元の改訂日が 2026-09-20 と 2026-09-17、リポジトリは 2026-09-22）。skill が9つに増えるとずれやすくなる。ローカルで使う間は常に最新を使う方針とも合う
  - decided_by: 利用者（推奨を採用。「まずは」の扱いで、配布は別の件）
  - superseded_by: [A1（skill-distribution）](./2026-09-23-skill-distribution.md#A1)、[A2（skill-distribution）](./2026-09-23-skill-distribution.md#A2)

- A8 kotowari スキルの setup が AGENTS.md に書く雛形は「仕様は IR で管理する」の一文だけにする。kotowari-using-workflow を読ませる一文は、工程の skill の入れ方の説明に「AGENTS.md に足す1行」として書く。このリポジトリの AGENTS.md には今回その一文を足す
  - why: A5 で kotowari スキルは工程の skill を名指ししないと決めたので、setup の雛形には書けない。工程の skill を入れる人だけがその一文を要る
  - decided_by: 利用者（推奨を採用）

- A9 工程の skill の description に「kotowari を使うリポジトリ（`.kotowari/` か `docs/ir/` がある）で使う」という条件を書く。ba0918 の skill は変えない
  - why: 両方が入った環境では description の似た2つが同じ依頼の候補になる。入口は A2 の一文で kotowari-using-workflow に向くので、残るのは利用者が直接呼んだときの取り違えだけになる
  - decided_by: 利用者（推奨を採用）

- A10 工程の skill の本文は英語で揃える。写した本文はそのまま、workflow.md が担っていた kotowari 固有の手順は英語に訳して溶かし込む
  - why: 写す元との差分が kotowari 固有の部分だけになって読みやすく、1つの skill の中で言語が混ざらない
  - rejected: 全部を日本語にする。kotowari スキルとは揃うが、写す元との差分が全部の行になる
  - decided_by: 利用者（推奨を採用）

- A11 workflow.md の brainstorm の節のうち、判断の記録の形（決定の節の見出し、補足の行、資料をまたぐ参照をリンクにする規則）は kotowari スキルの新しい reference に残し、場面 write から読ませる。手順（記録からの再開、承認の前の check と照合レビュー、stage するもの）は kotowari-brainstorm に移す。工程の skill は kotowari スキルやほかの工程の skill を名指しして「これを読め」と書いてよい
  - why: A4 を文字どおりに読むと記録の形も工程の skill に移り、A5 の「kotowari スキルだけを入れて別の工程で使う」道で記録の書き方が分からなくなる。記録の形は check が読む kotowari の契約で、工程ではない。kotowari の skill の間の依存は認める
  - decided_by: 利用者（推奨を採用。名指しを認めるのは利用者の言葉）

- A12 kotowari の工程は、利用者がトークンの費用を払い切れない前提で作る。やるべきことはやり、理由の無い席、委譲、繰り返しは足さない
  - why: agentic-workflow は小さな作業にも壁打ち、subagent、実装後のレビューと改善のループを求め、富豪でないと成り立たなかったので、理由が無ければ走らせない形に直した。kotowari も同じ方向に舵を切り、効率よく最大の見返りを出す
  - decided_by: 利用者

- A13 cycle、iterate、implement が扱う kotowari check の指摘は、その回のものだけにする。その回のものとは、`path` がブランチで変えたファイルであるか、計画（iterate では依頼の列挙）が覆う ID を名指しする指摘。それ以外は始める前からあったものとして、終端報告に種類ごとの件数を載せるだけで委譲しない。既にあった指摘かコミット済みの問題の記録だけが理由の `complete false` も同じ扱い
  - why: リポジトリ全体の指摘を直させると、依頼に無い作業に修正のループを回すことになる（A12）。仕様の無い小さな作業で `complete false` をどう扱うかも、これで決まる
  - rejected: 起点のコミットで check を走らせて、差分で新しく出た指摘だけを選ぶ。正確だが起点を別に取り出す手間が要り、ファイルと ID で絞れば足りる
  - decided_by: 主セッション（利用者が A12 の方針で任せた）

- A14 計画の最後の確認は、計画が覆う要求と具体例にテストの印があることと、ブランチで変えたファイルか計画が覆う ID への check の誤りが無いこととする。`kotowari status` の `complete true` と `kotowari check` の終了コード0は求めない
  - why: どちらも計画の前からある指摘と問題の記録を数えるので、FLAG をコミット済みのリポジトリでは最後の確認が永遠に通らない。計画が確かめるのは自分の仕事だけでよい（A12、A13）
  - decided_by: 主セッション（利用者が A12 の方針で任せた）

## Undecided

## Revisions

- A7 の置き場を [A1（skill-distribution）](./2026-09-23-skill-distribution.md#A1) で `agent/skills/` に改め、決めていなかった他の人への配り方を [A2（skill-distribution）](./2026-09-23-skill-distribution.md#A2) で決めた。手元へのシンボリックリンクの入れ方は変えていない
