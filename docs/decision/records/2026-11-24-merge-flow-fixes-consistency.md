# 人を待たずにマージする工程の手直しを IR と読み合わせる

## Context

[人を待たずにマージする工程の手直し](./2026-11-24-merge-flow-fixes.md)を skill の本文に書いたあと、その差分を整合のフェーズとして IR と読み合わせた。
skill の本文は製品の振る舞いなので、コードとして読む。
実装役が自分で決めた、仕様に触れる点が5つあった。
1つは IR と食い違っていたので skill を直し、残りの4つは IR と合っていることを根拠と実測で確かめた。
実装のあとの実行では IR を変えていない。
品質レビューの直しが収束したあとの実行では、直しが足した振る舞いのうち IR に無いものを IR に書き足し、IR と食い違う skill を直した。承認済みの要求の意味は変えていない。

Position: 整合のフェーズが決めた（2026-11-24）。A1〜A6 は実装のあとの実行で、A7 からは品質レビューの直しが収束したあとの実行で決めた。

## Agreements

- A1 マージ先が承認のレビューを必須にしていないことは、3つの読みがそろったときだけ確かめられたとする。枝の保護の読みが `false`、ルールセットの読みが `0`、プルリクエスト自身の `reviewDecision` が空。枝の保護の読みは、枝そのものが引けないときは何も出さない。ほかの出力は、空の出力も含めて、確かめられなかったものとして扱う。IR は変えず、skill を IR に合わせる
  - why: [REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) は、承認のレビューを必須にしていないと確かめたときだけ自動のマージを頼み、確かめられないときはプルリクエストで止まるとする。skill の枝の保護の読みは、値が無いときに `false` を出す書き方で、「分からない」を「必須でない」と読んでいた。枝の保護の規則は、それを見られないアカウントには無いものとして返るので、設定だけを読む2つのコマンドでは、承認のレビューが必須のマージ先でも `false` と `0` がそろう。プルリクエスト自身の `reviewDecision` は、どの設定が求めているかにかかわらず、ホスティングの側が判定した結果を返す。作った直後にこの値が何を返すかはまだ測っていない（[手直しの記録](./2026-11-24-merge-flow-fixes.md)の U2 と同じ）ので、設定の読みと置き換えず、3つめの条件として足す。枝の保護を見られないアカウントでは、設定の2つの読みは働かず、この値だけが頼りになる（U1）。そこで読み違えたときに起きるのは、頼みが残ったままの終わらない待ちで、レビューされていないものがマージされることではない
  - rejected: 設定の読みを外して `reviewDecision` だけにする（作った直後の値を測っていない）。値が無いときの扱いだけを直す（規則を見られないアカウントでは、枝が引けても規則が無いものとして返るので、読み違いが残る）
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の Opening and asking の5。直す前の枝の保護のコマンドは、無い枝の名前を渡すと `false` を出して終了コード 0 だった。直したあとは、このリポジトリのマージ先で `false`、無い枝で空を出す。読み取りの権限しか無い公開リポジトリで、REST の枝の情報が保護ありと必須の検査3つを返す枝に同じ問い合わせをすると、`branchProtectionRule` は null、ルールセットのコマンドは `0` を出し、枝の保護のコマンドは直す前後とも `false` を出した。その枝へのプルリクエストの `reviewDecision` は `REVIEW_REQUIRED` だった（いちばん新しいもので作ってから13時間あまり）。このリポジトリの開いているプルリクエストの `reviewDecision` は空。`reviewDecision` の値は `gh api graphql -f query='{__type(name:"PullRequestReviewDecision"){enumValues{name}}}'` で読み、CHANGES_REQUESTED、APPROVED、REVIEW_REQUIRED の3つ）

- A2 枝の保護が承認のレビューを必須にしているかは、GraphQL の `requiresApprovingReviews` で読む。REST の `required_pull_request_reviews` だけを返す問い合わせは使わない
  - why: その問い合わせは、承認のレビューを必須にしていないマージ先でも、必須の人数を 1 と返した。使うと、自動のマージがいつも使えないことになる
  - decided_by: 整合のフェーズ（根拠: このリポジトリのマージ先で4つを読み比べた。`branches/<target>/protection/required_pull_request_reviews` は `required_approving_review_count` が 1。`branches/<target>/protection` の `required_pull_request_reviews` は null。GraphQL の `requiresApprovingReviews` は false。直前にマージされたプルリクエストはレビューが0件で、`reviewDecision` は空だった）

- A3 検査が通ったあとホスティングの側がマージしないと読む状態は、衝突（DIRTY）と枝の遅れ（BEHIND）だけとし、BLOCKED は読まない。IR も skill も変えない
  - why: [REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) は「待っても変わらない状態」とし、[A10](./2026-11-24-merge-flow-fixes.md#A10) は衝突と遅れの2つを挙げる。BLOCKED は、検査が通った直後に一時的に返るのか、止まり続けるのかを、まだ測っていない（[手直しの記録](./2026-11-24-merge-flow-fixes.md)の U2）。BLOCKED のまま止まり続ける原因でいちばんありうるのは承認のレビューで、それは頼む前に A1 で除いている。読み違えたときに起きるのは終わらない待ちで、レビューされていないものがマージされることではない。[前の整合のフェーズの A8](./2026-11-24-unattended-merge-consistency.md#A8) は BLOCKED を決めておらず、そのときの skill が挙げていただけだった
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の Watching。読み取りの権限しか無い公開リポジトリの開いているプルリクエスト5件のうち、BLOCKED の4件は `reviewDecision` が REVIEW_REQUIRED か CHANGES_REQUESTED だった。ホスティングの側に書き込めないので、見届けそのものは走らせていない）

- A4 開いているプルリクエストの先頭が手元の枝にあるかは、新しい依頼の開始だけでなく、どの開始でも再開でも確かめる。IR も skill も変えない
  - why: [REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) のこの文には、マージ済みか閉じたプルリクエストの文と違って、「新しい依頼を受け取った開始で」という条件が付いていない
  - decided_by: 整合のフェーズ（根拠: REQ-core-364 の how_to_verify と [A7](./2026-11-24-merge-flow-fixes.md#A7)、agent/skills/kotowari-cycle/SKILL.md の Resuming and running more の最初の段）

- A5 マージで終わった回の指摘のファイルの名前を変える決まりは、枝の名前を使い回さない決まりと並べて残す。IR も skill も変えない
  - why: [REQ-core-369](../../ir/core/unattended-merge.md#REQ-core-369) が、マージで終わった回のファイルを再開として読まないことを skill に書くよう求めている。枝の名前を使い回さない決まりはホスティングの側を読めたときにだけ働くので、2つは食い違わない
  - decided_by: 整合のフェーズ（根拠: REQ-core-369 と REQ-core-364 の how_to_verify、agent/skills/kotowari-cycle/SKILL.md の Resuming and running more）

- A6 壁打ちを始めたときの枝は、セッションの中で覚えるだけでよい。IR も skill も変えない
  - why: 壁打ちが枝を切るのは承認のときで、それまで主の作業ツリーは始めたときの枝のままなので、途中で再開した壁打ちも同じ枝を覚える。枝を切ってから戻すまでのあいだにセッションが切れた場合は、新しい枝を作業用のディレクトリに出すところで git が断り、[REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) の「切り替えられないときは止まって人に伝える」に落ちる
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-brainstorm/SKILL.md の Inputs and outputs と Finishing。使い捨てのリポジトリで、別の枝にいてステージ済みの新しいファイルがあるとき、`git switch -c <branch> origin/<target>` はファイルを持ったまま切り替わった。その枝を出したまま `git worktree add <path> <branch>` を走らせると、すでに使われていると言って終了コード 128 で止まった。使い捨ての手順はコミットしていない）

- A7 必須の検査の読みが1つでも失敗したら、ほかの読みが何を返していても、マージ先が検査を必須にしているかは確かめられなかったものとする。そのとき見届けは、見えている検査を見届けてから、失敗を付けてプルリクエストを人に渡して終わる。[REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) に書き足す
  - why: REQ-core-366 は見届けの終わりを挙げるが、必須かどうかを確かめられなかった回の終わりが無く、検査が1つも現れないと進む手段の無い待ちになる。確かめられない回は自動のマージを頼まずプルリクエストで止まる回なので、人に渡すのは同じ要求の「確かめられないときはプルリクエストで止まり、人がマージする」と同じ向きになる。プルリクエストを開く手順は「どちらかが名前を出せば必須」と書き、見届けは「どちらも失敗していないこと」を求めていて、片方が失敗したとき、頼んでから人に渡す読みになっていた。失敗した読みが持っていた名前は分からないので、確かめられなかった側に寄せる
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の Opening and asking の3と Watching。このリポジトリのマージ先で、見届けが読む2つの名前の読みは、枝の保護が2つ、ルールセットが1つの名前を出し、開いているプルリクエストの `gh pr checks <pr> --required --json name` は同じ2つの名前を出した。ホスティングの側に書き込めないので、見届けそのものは走らせていない）

- A8 マージの頼みが残っているかをホスティングの側から読めないときは、枝へ push せず、止まって人に伝える。プルリクエストを作れないホスティングだけは、頼みが残りえないので、push へ進む。[REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) に書き足し、skill を合わせる
  - why: REQ-core-366 は、頼んだあとは枝へコミットを足す前に頼みを取り消すとする。直しのあとの skill は、開始のときに頼みを読めなければ進み、push の前の読み直しも失敗したときの扱いを書いていなかったので、すでにマージを頼んであるプルリクエストへ、線の判定もレビューも済んでいないコミットを push しうる（[A39](./2026-11-24-unattended-merge.md#A39) の why と同じ）。手元のコミットは push するまで害が無いので、開始のときは進んでよい。同じ要求は、プルリクエストを作れないホスティングでは枝を push して止まるとも言う。そこでは cycle が頼みを出す手段が無いので、読めないことを理由に止めると、この決まりが働かなくなる
  - rejected: 開始のときに読めなければ止まる（プルリクエストを作れないホスティングで何も始められない）
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Resuming and running more、agent/skills/kotowari-cycle/references/merge.md の Withdrawing the request と Opening and asking の2。使い捨てのリポジトリ（遠隔が手元の裸のリポジトリ）で `gh pr list` は失敗した。ネットワークを切った状態でも同じコマンドは終了コード 1 で、`gh auth status --hostname <host>` も、知らないホストとネットワークを切った状態のどちらでも終了コード 1 だった。終了コードでは2つを分けられないので、分け目は遠隔の URL のホストを gh が扱えるかどうかとして skill に書いた）

- A9 再開は、依頼を持たない開始とする。指摘のファイルがすでにある枝へ依頼を持ってきた開始は、再開ではなく、同じ枝へのあとの依頼とする。あとの依頼は、仕様が承認済みかを確かめ、ID を足し、実装からやり直し、整合のフェーズの回とフルレビューをマージ先との分かれ目から読む。cycle の始めは、枝のプルリクエストを読む、仕様を確かめる、指摘のファイルを作るか書き換える、の順にする。[REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) に書き足し、skill を合わせる
  - why: REQ-core-364 は、新しい依頼の開始、同じ枝へのあとの依頼、再開の3つを分けて書いている。skill は「指摘のファイルがあれば再開」としていたので、あとの依頼が再開に畳まれていた。差し戻しのあとに再開した壁打ちが仕様を変えて cycle に渡すと、承認の確かめが省かれ、最初のレビューが済んでいれば再開の表のどの行も新しい ID を実装役に渡さない。ファイルの有無で分けると、[REQ-core-369](../../ir/core/unattended-merge.md#REQ-core-369) が求める「人のための停止も終わり方として残す」と、「再開では確かめない」が両立しない。あとの依頼のレビューの範囲は、[A1](./2026-11-24-merge-flow-fixes.md#A1) の理由（レビューされないコミットがマージされないこと）から、マージ先との分かれ目からとする。前の回が終わったところから読むのは、A1 が挙げる小さい作業だけで、それは IR にも除くと書いた。始めの3つは、どれも「最初に」と書かれていて順が決まっていなかった。この順では、新しい枝の名前を求める停止と、未承認の停止は、指摘のファイルに何も書かない。どちらも回が始まる前の停止で、REQ-core-369 が残すと言う「回が着いた終わり方」には当たらない
  - rejected: 渡された ID がすでにファイルにあれば再開とする（要求を変えただけの依頼は同じ ID を持ってくるので、見分けられない）
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Inputs と Resuming and running more の表、agent/skills/kotowari-iterate/SKILL.md の The loop。iterate は「新しい依頼は判定のあと手順1から、その依頼の再開は cycle の表で」とすでに分けていて、この分け方と合う）

- A10 自動のマージを頼む条件に、プルリクエストが cycle の受け取ったマージ先へ向いていることを足す。再開のときに開いているプルリクエストが別の枝へ向いていたら、残っている頼みを取り消して人に渡す。[REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366) に書き足し、skill を合わせる
  - why: [REQ-core-365](../../ir/core/unattended-merge.md#REQ-core-365) は、マージ先は cycle が受け取ったもので、プルリクエストはそこへ向くとする。skill は、プルリクエストを新しく作るときだけマージ先を指定し、すでにあるプルリクエストの向き先を確かめていなかった。セッションのあいだに向き先を変えられたプルリクエストは、ある枝に対して線とレビューを読み、別の枝へマージされる。止め方は、REQ-core-366 の「確かめられないときはプルリクエストで止まる」に乗せる。[REQ-core-367](../../ir/core/unattended-merge.md#REQ-core-367) の人が確認する場合は3つだけと決まっているので、そこには足さない
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/references/merge.md の Opening and asking の2と5、Resuming の表。このリポジトリの開いているプルリクエストで `gh pr view <pr> --json baseRefName --jq .baseRefName` は向き先の枝の名前を出した。向き先を変えたときにホスティングの側が頼みを落とすかは測っていないので、それには頼らない）

- A11 マージ先を決めておらず、既定の枝も読めないときは、cycle は推測せずに人に聞く。開始のときにホスティングの側を読めなければ、枝の名前の使い回しとプルリクエストの先頭の2つの確かめを省いて進み、push の前に読めたらそこで確かめ、できなかった確かめを最後の報告に書く。[REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) に書き足す
  - why: 品質レビューの直しが skill に足した振る舞いで、REQ-core-364 は読めないときのことを言っていなかった。プルリクエストを作れないホスティングでも cycle は回る（[REQ-core-366](../../ir/core/unattended-merge.md#REQ-core-366)）ので、読めないことを理由に止めると、そこでは何も始められない。マージ先を推測すると、線とレビューの範囲が違う枝から読まれる。push の前には頼みを読み直す（A8）ので、省いた2つの確かめもそこでできる
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-cycle/SKILL.md の Inputs と Resuming and running more、Terminal report。使い捨てのリポジトリで `gh repo view --json defaultBranchRef` と `gh pr list` はどちらも失敗した）

- A12 人がコミットせずにコミットしてよいと言ったときは、壁打ちが新しい枝で仕様をコミットする。承認したファイルが新しい枝にコミットされていないあいだは、主の作業ツリーを戻さず、どのファイルかを人に伝えて止まる。[REQ-core-364](../../ir/core/unattended-merge.md#REQ-core-364) に書き足す
  - why: 品質レビューの直しが skill に足した振る舞いで、REQ-core-364 は、誰がコミットするかと、コミットされていないときのことを言っていなかった。「新しい枝で仕様をコミットし、承認のあと主の作業ツリーを戻す」という順と合う。ステージ済みのファイルは枝の切り替えに付いていくので、コミットの前に戻すと、仕様は元の枝へ持ち帰られ、新しい枝には何も入らないまま cycle に渡る
  - decided_by: 整合のフェーズ（根拠: agent/skills/kotowari-brainstorm/SKILL.md の Finishing の4。使い捨てのリポジトリで、ステージ済みの新しいファイルは `git switch -c <branch> origin/<target>` に付いていった（A6 の実測））

- A13 [REQ-core-369](../../ir/core/unattended-merge.md#REQ-core-369) の文に、指摘のファイルに残すものとして、受け取ったマージ先を足す
  - why: IR の中の抜け。同じ要求の how_to_verify は残すものを5つ挙げ、[A4](./2026-11-24-merge-flow-fixes.md#A4) もマージ先を書くと決めているのに、文は4つしか挙げていなかった。要求の意味は how_to_verify のとおりで変わらない
  - decided_by: 整合のフェーズ（根拠: docs/ir/core/unattended-merge.md の REQ-core-369 の文と how_to_verify を、両方の言語で読み比べた）

## Undecided

- U1 承認のレビューが必須のマージ先で、プルリクエストを作った直後の `reviewDecision` が、すぐに `REVIEW_REQUIRED` を返すか
  - decides: 承認のレビューを必須にしているリポジトリで、次にプルリクエストを作る回の主セッションが測り、判断の記録に残す。このリポジトリは承認のレビューを必須にしていないので、ここでは測れない
  - related: A1
