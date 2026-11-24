# 人を待たずにマージする工程の手直しを IR と読み合わせる

## Context

[人を待たずにマージする工程の手直し](./2026-11-24-merge-flow-fixes.md)を skill の本文に書いたあと、その差分を整合のフェーズとして IR と読み合わせた。
skill の本文は製品の振る舞いなので、コードとして読む。
実装役が自分で決めた、仕様に触れる点が5つあった。
1つは IR と食い違っていたので skill を直し、残りの4つは IR と合っていることを根拠と実測で確かめた。
IR は変えていない。

Position: 整合のフェーズが決めた（2026-11-24）。実装のあとの実行で決めた。

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

## Undecided

- U1 承認のレビューが必須のマージ先で、プルリクエストを作った直後の `reviewDecision` が、すぐに `REVIEW_REQUIRED` を返すか
  - decides: 承認のレビューを必須にしているリポジトリで、次にプルリクエストを作る回の主セッションが測り、判断の記録に残す。このリポジトリは承認のレビューを必須にしていないので、ここでは測れない
  - related: A1
