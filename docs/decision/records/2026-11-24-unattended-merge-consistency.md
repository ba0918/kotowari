# 人を待たずにマージする工程の skill を IR と読み合わせる

## Context

[仕様の承認からマージまでを人なしで通す決定](./2026-11-24-unattended-merge.md)を skill の本文に書いたあと、その差分を整合のフェーズとして IR と読み合わせた。
skill の本文は製品の振る舞いなので、コードとして読む。
IR に書かれていない振る舞いと、IR の中で食い違っている文が見つかったので、根拠と実測で決められるものをここで決める。
決められなかった意味の問いは、既定を置いて進め、問題の記録に残す。

Position: 整合のフェーズが決めた（2026-11-24）。A1〜A4 は実装のあとの実行で、A5 からは品質レビューの直しが収束したあとの実行で決めた。

## Agreements

- A1 見届けは、プルリクエストで止まる回では必須の検査が通ったときにも終わる。マージ先に必須の検査が1つも無いときは、見届けるものが無いので、すぐに終わる。[REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) にこの2つを書き足す
  - why: REQ-core-366 と [A43](./2026-11-24-unattended-merge.md#A43) は見届けの終わりを3つ挙げるが、3つとも、自動のマージを頼んだ回か検査が落ちた回のものだった。プルリクエストで止まる回も検査を見届ける（[A44](./2026-11-24-unattended-merge.md#A44)）ので、検査が通ったあとの終わりが無いと、進む手段の無い待ちになる。cycle は見届けに時間切れを持たず、人があとでマージした回は片付けない（[REQ-core-369](../../ir/core/unattended-merge.md#REQ-core-369)）ので、人のマージを待ち続ける読みは取れない。必須の検査が無いマージ先は、自動のマージを使えずプルリクエストで止まる場合（[A35](./2026-11-24-unattended-merge.md#A35)）として想定されていて、そこでは検査の結果が来ない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の Watching の終わりの一覧に4つ目があり、IR に無かった。A43、A44、A35 と矛盾しない具体化）

- A2 [REQ-core-213](../../ir/core/deferred.md#REQ-core-213) の文から「計画」を外す。skill が書いているのは、実装のループ、レビュー、壁打ちでの後回しの扱いとする
  - why: IR どうしが食い違う。[REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) は計画を書く段が無いとし、REQ-core-213 の how_to_verify は [A38](./2026-11-24-unattended-merge.md#A38) で計画の skill を挙げる部分を cycle に書き換え済みなのに、文だけが計画での扱いを skill が書いていると言い続けていた。A38 は計画の段の名前を挙げている文を意味を変えずに書き換えると決めていて、この文はその書き換えの取りこぼしにあたる。要求の意味（後回しの ID を実装に渡す前に外す）は how_to_verify のとおりで変わらない
  - decided_by: 整合のフェーズ（根拠: docs/ir/core/deferred.md の REQ-core-213 の文と how_to_verify、REQ-core-364 の how_to_verify、A37 と A38。agent/skills/ の下に計画を書く skill が無いことは `ls agent/skills` で確かめた）

- A3 security の指摘で止まって人に渡したとき、人が「いまは直さず、開いたまま残す」と答えたら、cycle はその指摘のアクションを human_judgment にする。IR は変えず、skill の本文を IR に合わせる
  - why: [REQ-core-367](../../ir/core/unattended-merge.md#REQ-core-367) と [A32](./2026-11-24-unattended-merge.md#A32) は、人の判断を待つ指摘の出どころに「人が残すと決めたもの」を挙げ、それが開いていればマージを頼まないとする。収束する回の中で人が指摘を開いたまま残すと決める場面は、security の指摘で止まったときの答えだけだった。ところが skill は、その指摘のアクションを変えず、マージを頼まない条件を「human_judgment で開いている指摘」と書いていたので、人が残すと決めた security の指摘が開いたまま、人なしでマージを頼む読みになっていた。アクションが fix_and_verify のままなら見える指摘として残り、収束もしない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Stopping inside the loop の1項目めと Loop の見える指摘の定義、agent/skills/kotowari-cycle/references/merge.md の When a person confirms の2項目め、REQ-core-367 の how_to_verify。ほかに人が出る場面も読んだ。前提を人に渡すとき（agent/skills/kotowari-cycle/references/premise-step.md の The step の4）は進まないときの終わり方で終わり、収束しないのでマージへ進まない。終わりに人が受け入れた指摘は閉じる（SKILL.md の Judgment stays here）。kotowari-iterate は cycle のループをそのまま使う）

- A4 オラクルを「既存の検査がすべて通ること」に置き換えた指摘は、レビュアーが報告した oracle.measured をそのまま持ち、その値が置き換える前の提案で測ったものだと note に書く。fails_now の指摘は直す指摘に残り、それ以外は記録だけになる。これは既定で、意味の問いは [FLAG-core-012](../../ir/core/FLAGS.md#FLAG-core-012) に残し、人の判断を待つ（U1）
  - why: skill は置き換えのあと oracle.measured をどうするかを書いておらず、提案が「既存の検査がすべて通ること」で measured が fails_now という、読んだだけでは食い違う記録になっていた。[REQ-core-368](../../ir/core/unattended-merge.md#REQ-core-368) と [A17](./2026-11-24-unattended-merge.md#A17) は「走らせて示せる間違い」とだけ言い、置き換えられたオラクルで落ちた結果を数えるかは言っていない。IR と判断の記録を `rg -n -i 'oracle|evidence condition|existing checks|fails_now' docs/ir` と `rg -n 'オラクル|証拠の条件|既存の検査|fails_now|oracle' docs/decision/records` で探したが、置き換えの規則そのものがどちらにも無かった。測る相手も無い。measured はレビュアーが報告する値で、どちらの読みでも同じ値が来るので、走らせても読みは分かれない。既定を「数える」にしたのは、レビュアーが実際に走らせて落ちるのを見ていること、置き換えの規則が以前から「アクションは保つ」としていて、measured を測り直す読みではその句が働かなくなること、warn の指摘は証拠の条件を満たすオラクルでも置き換えるのでその実測は証拠として数えられること、逆の既定では走らせて落ちた間違いが直されないまま人なしでマージされうることによる
  - rejected: 置き換えたら measured を新しいオラクルで測り直す（既存の検査はその時点で通っているので、置き換えた指摘がぜんぶ記録だけになり、「アクションは保つ」が働かなくなる）
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Judgment stays here の順に2つの見本をたどった。新しいテストを求め fails_now の critical は、アクションを保ち、オラクルを置き換え、最後の規則で直す指摘に残る。同じ指摘が not_run なら最後の規則で記録だけになる。agent/skills/kotowari-review/SKILL.md の Writing a finding は、証拠の条件を満たすと示さない検証の要求を「記録された提案」とするが、指摘そのものは落とさない）

- A5 線の手順について、次の3つを [REQ-core-365](../../ir/core/unattended-merge.md#REQ-core-365) に書き足す。パスの指定は、マージ先の先頭にある一覧と枝の先頭にある一覧の両方から読み、どちらかで当たれば当たりとする。1行ずつ別々に照らし、パスを除く行はほかの行の当たりを消さない。手順のコマンドが失敗したときは、当たったパスの無い当たりとし、失敗として報告する。止まった原因として人に見せるものにも、この失敗を足す（[REQ-core-367](../../ir/core/unattended-merge.md#REQ-core-367)）
  - why: REQ-core-365 は「1行に1つ書いたパスの指定と照らす」とだけ言い、どちらの一覧を読むか、行をまとめて照らすか別々に照らすか、手順が失敗したらどうするかを言っていなかった。枝の一覧だけを読むと、枝が分かれたあとにマージ先の一覧へ足された行が効かない。行をまとめて照らすと、パスを除く行を1つ足すだけで、ほかの行の当たりを消せる（一覧のファイル自身は当たるが、当たったパスの一覧からは消える）。失敗を当たりにするのは、ファイルが無いときを当たりと同じにする決まり（[A19](./2026-11-24-unattended-merge.md#A19)）と同じ向きで、結果が2つの値だけという決まりとも合う。失敗には当たったパスが無いので、原因として見せるのは失敗そのものになる
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の The line の手順を、使い捨てのリポジトリで sh（dash）から走らせた。マージ先の一覧にあとから足した行に当たる変更は当たり。一覧が `scripts` と `:!scripts/gate.sh` のとき、scripts/gate.sh の変更は当たりで、同じ2行を1回の git diff にまとめると何も出ない。取得の失敗と、知らない書き方の行（`:(nosuchmagic)x`）は、どちらも終了コード 128。使い捨ての手順はコミットしていない）

## Undecided

- U1 証拠の条件を満たすと示されないまま置き換えられたオラクルが落ちたことを、「走らせて示せる間違い」に数えるか。いまの既定は A4 のとおり「数える」
  - decides: 利用者
  - related: A4
