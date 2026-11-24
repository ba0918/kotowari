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
  - superseded_by: [A5（セキュリティの指摘では止めずに直す）](./2026-11-24-merge-flow-fixes.md#A5)

- A4 オラクルを「既存の検査がすべて通ること」に置き換えた指摘は、レビュアーが報告した oracle.measured をそのまま持ち、その値が置き換える前の提案で測ったものだと note に書く。fails_now の指摘は直す指摘に残り、それ以外は記録だけになる。これは既定で、意味の問いは [FLAG-core-012](../../ir/core/FLAGS.md#FLAG-core-012) に残し、人の判断を待つ（U1）
  - why: skill は置き換えのあと oracle.measured をどうするかを書いておらず、提案が「既存の検査がすべて通ること」で measured が fails_now という、読んだだけでは食い違う記録になっていた。[REQ-core-368](../../ir/core/unattended-merge.md#REQ-core-368) と [A17](./2026-11-24-unattended-merge.md#A17) は「走らせて示せる間違い」とだけ言い、置き換えられたオラクルで落ちた結果を数えるかは言っていない。IR と判断の記録を `rg -n -i 'oracle|evidence condition|existing checks|fails_now' docs/ir` と `rg -n 'オラクル|証拠の条件|既存の検査|fails_now|oracle' docs/decision/records` で探したが、置き換えの規則そのものがどちらにも無かった。測る相手も無い。measured はレビュアーが報告する値で、どちらの読みでも同じ値が来るので、走らせても読みは分かれない。既定を「数える」にしたのは、レビュアーが実際に走らせて落ちるのを見ていること、置き換えの規則が以前から「アクションは保つ」としていて、measured を測り直す読みではその句が働かなくなること、warn の指摘は証拠の条件を満たすオラクルでも置き換えるのでその実測は証拠として数えられること、逆の既定では走らせて落ちた間違いが直されないまま人なしでマージされうることによる
  - rejected: 置き換えたら measured を新しいオラクルで測り直す（既存の検査はその時点で通っているので、置き換えた指摘がぜんぶ記録だけになり、「アクションは保つ」が働かなくなる）
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Judgment stays here の順に2つの見本をたどった。新しいテストを求め fails_now の critical は、アクションを保ち、オラクルを置き換え、最後の規則で直す指摘に残る。同じ指摘が not_run なら最後の規則で記録だけになる。agent/skills/kotowari-review/SKILL.md の Writing a finding は、証拠の条件を満たすと示さない検証の要求を「記録された提案」とするが、指摘そのものは落とさない）

- A5 線の手順について、次の3つを [REQ-core-365](../../ir/core/unattended-merge.md#REQ-core-365) に書き足す。パスの指定は、マージ先の先頭にある一覧と枝の先頭にある一覧の両方から読み、どちらかで当たれば当たりとする。1行ずつ別々に照らし、パスを除く行はほかの行の当たりを消さない。手順のコマンドが失敗したときは、当たったパスの無い当たりとし、失敗として報告する。止まった原因として人に見せるものにも、この失敗を足す（[REQ-core-367](../../ir/core/unattended-merge.md#REQ-core-367)）
  - why: REQ-core-365 は「1行に1つ書いたパスの指定と照らす」とだけ言い、どちらの一覧を読むか、行をまとめて照らすか別々に照らすか、手順が失敗したらどうするかを言っていなかった。枝の一覧だけを読むと、枝が分かれたあとにマージ先の一覧へ足された行が効かない。行をまとめて照らすと、パスを除く行を1つ足すだけで、ほかの行の当たりを消せる（一覧のファイル自身は当たるが、当たったパスの一覧からは消える）。失敗を当たりにするのは、ファイルが無いときを当たりと同じにする決まり（[A19](./2026-11-24-unattended-merge.md#A19)）と同じ向きで、結果が2つの値だけという決まりとも合う。失敗には当たったパスが無いので、原因として見せるのは失敗そのものになる
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の The line の手順を、使い捨てのリポジトリで sh（dash）から走らせた。マージ先の一覧にあとから足した行に当たる変更は当たり。一覧が `scripts` と `:!scripts/gate.sh` のとき、scripts/gate.sh の変更は当たりで、同じ2行を1回の git diff にまとめると何も出ない。取得の失敗と、知らない書き方の行（`:(nosuchmagic)x`）は、どちらも終了コード 128。使い捨ての手順はコミットしていない）

- A6 片付けの前に数える「コミットしていない変更」は、git の status が出すもの（追跡しているファイルの変更と、追跡していないファイル）とし、git が無視するファイルは数えない。IR は変えず、無視するファイルも数えるようにした skill の直しを IR に戻す。片付けで無視するファイルも消えることは skill に書く
  - why: [REQ-core-369](../../ir/core/unattended-merge.md#REQ-core-369) と [A31](./2026-11-24-unattended-merge.md#A31) は、見たマージのあとに手元の作業用のディレクトリと枝を片付けるとし、残すのは「実装役の作業メモのほかにコミットしていない変更」があるときだけとする（[A43](./2026-11-24-unattended-merge.md#A43)）。無視するファイルまで数えると、何かをビルドした作業用のディレクトリはビルドの出力でいつも残る側になり、IR が約束する片付けが起きない。「コミットしていない変更」の意味を広げる読みで、具体化ではなく食い違いにあたる。IR が誤りだと示すもの（IR どうしの矛盾、実現できないことの再現）は無いので、コードを IR に合わせる
  - rejected: 無視するファイルも数えて残す（品質レビューの直し。強制の片付けで無視されたローカルのファイルが黙って消えることを防ぐが、ビルドの出力と見分ける手段が無く、片付けそのものが働かなくなる。A43 が守るのは失うと困る作業で、無視するファイルをそれに数える根拠は記録に無い）
  - decided_by: 整合のフェーズ（根拠: この回の作業用のディレクトリでもある主の作業ツリーで `git status --porcelain --untracked-files=all --ignored` を数えた。無視されるファイルは 76,605 件で、うち 75,123 件が target/ の下。`--ignored` を付けないと 0 件。同じ木で、IR は片付けると言い、直しのあとの skill は残すと言っていた）

- A7 マージの頼みを取り消す条件は、この回が頼んだかどうかではなく、ホスティングの側がプルリクエストに頼みを示しているかどうかで決める。枝へコミットを足す委譲の前と、枝を push する前に、ホスティングの側から読んで、頼みがあれば取り消す。再開のどの行から入るときも、「さらに回す」ときも、kotowari-iterate が枝を使い回して新しい依頼を始めるときも同じ。IR は変えず、skill を IR に合わせる
  - why: [REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) は、頼んだあとは枝へコミットを足す前に頼みを取り消すとし、[REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) は、プルリクエストを作ったあとの再開は自動のマージを頼んであるかをホスティングの側から読むとする。skill の取り消しは「頼んだあと」という、その回の記憶に掛かった条件で、再開の表は「プルリクエストがある」の行より上の行（最初のレビューがまだ、見える指摘が開いている）から入るとホスティングの側を読まずに実装や修正を委譲していた。iterate が枝を使い回す新しい依頼は表を通らず実装から始まる。見届けには時間切れが無いので、頼んだ回が見届けの途中で終わることは起こる。その頼みが生きたまま足したコミットを push すると、線の判定もレビューも済まないうちに、検査が通った時点でマージされうる（[A39](./2026-11-24-unattended-merge.md#A39) の why と同じ）
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Resuming and running more の表、agent/skills/kotowari-cycle/references/merge.md の Opening and asking の2と Withdrawing the request、agent/skills/kotowari-iterate/SKILL.md の枝を使い回す段を、3つの入り方でたどった。ホスティングの側に書き込めないので、走らせてはいない）

- A8 見届けは、プルリクエストがマージされずに閉じられたときにも終わり、cycle は人に報告する。[REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) に書き足し、回の記録の終わり方に「閉じられた」を足す。検査は通ったのにホスティングの側がマージしない場合には、マージ先が枝の最新化を求めていて枝が遅れている状態も含める。これは IR がすでに言っていることで、skill の挙げ方が狭かった。必須の検査がいつまでも結果を返さない場合の終わりは足さない
  - why: 閉じられたプルリクエストはマージされず、検査の結果を待つ意味も無くなるが、REQ-core-366 と [A43](./2026-11-24-unattended-merge.md#A43) の終わりのどれにも当たらず、進む手段の無い待ちになる。再開のときに閉じられていれば人に報告する、という扱いは skill にすでにあり、それと同じにする。開いたままでもマージ済みでもないので、終わり方は別の値にする。枝が遅れている状態は「検査は通ったのにホスティングのサービスがマージしないと返したとき」にあたるのに、skill は頼みが消えた場合と2つの状態だけを挙げていた。結果を返さない検査に終わりを足すと、cycle が見届けに自分の時間切れを持つことになり、A43 に反する
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の Watching と Resuming の表。状態の名前は `gh api graphql -f query='{__type(name:"MergeStateStatus"){enumValues{name description}}}'` で読んだ。DIRTY、UNKNOWN、BLOCKED、BEHIND、UNSTABLE、HAS_HOOKS、CLEAN の7つで、BEHIND の説明は「先頭が古い」。ホスティングの側に書き込めないので、見届けそのものは走らせていない）

## Undecided

- U1 証拠の条件を満たすと示されないまま置き換えられたオラクルが落ちたことを、「走らせて示せる間違い」に数えるか。いまの既定は A4 のとおり「数える」
  - decides: 利用者
  - related: A4
