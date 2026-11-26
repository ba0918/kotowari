# unattended-merge の対応表

元: `docs/ir/core/unattended-merge.md`（日本語は `unattended-merge.ja.md`）。書き直した文書: `unattended-merge.md`。短い名前は `merge`。

kotowari-cycle の skill と `references/merge.md`、kotowari-brainstorm と kotowari-implement の skill は、すでに決定の文書を相手にする形に書き直されている。そこに書いてある振る舞いは「実装済み」にした。kotowari-iterate の skill と、kotowari-review の `references/finding-schema.md` は、指摘のファイルの `ids` をまだ要求の ID として説明しているので、その部分だけを「提案」の決定に分けた。

記録の decided_by が「主セッション（…から導き、見直しのラウンドで利用者に示した）」と「整合のフェーズ」の決定は、agent の細部として扱った。そのうち安全に関わるもの（マージの守り）は、利用者の決定（安全に関わる細部は、今のテストか skill の文が確かめているなら決定に戻す）に従い、22件すべてを決定と確認することの行に戻した（下の「戻した細部」）。decided_by が「利用者（代決を承認）」の決定は、利用者が個別に承認したものとして残した。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-364 | C-core-merge-1, C-core-merge-3, C-core-merge-4, C-core-merge-5, C-core-merge-6, C-core-merge-7, C-core-merge-8, C-core-merge-9, C-core-merge-10, C-core-merge-11, C-core-merge-12, C-core-merge-33, C-core-merge-34, C-core-merge-36, C-core-merge-37, C-core-merge-38 | cycle は、決定の文書の置き場と、今回作る決定の一覧と、マージ先を受け取る | そのまま（計画の段が無いこと A4・A27、承認から同じ流れで進むこと A22、テストのコマンド A33・merge-flow-fixes の A11、壁打ちの枝 merge-flow-fixes の A8、マージ先と範囲 merge-flow-fixes の A1）と、読み替え（実装済み。壁打ちが足した・変えた要求の ID を、今回作る決定の一覧に。仕様の置き場と判断の記録の置き場を、決定の文書の置き場に。まだ印の付いたテストが無い要求を、まだ提案の決定に）。iterate と指摘のファイルの形の資料の `ids` は読み替え（提案。C-10）。後回しの ID を外して実装役に渡すこと（A37）は消える。7つの決定に分けた。戻した細部: 壁打ちがコミットするときの止まり方（C-33）、枝の名前と始めと再開の守り（C-34、C-36〜C-38） |
| REQ-core-365 | C-core-merge-13, C-core-merge-14, C-core-merge-15, C-core-merge-16, C-core-merge-39, C-core-merge-40, C-core-merge-41, C-core-merge-42 | 線は、git だけで、差分が .kotowari/confirm-paths のパスに触れたかで判定する | そのまま（A9、A16、A19、A20、merge-flow-fixes の A6）。一覧に書かない置き場の「仕様と判断の記録の置き場」を、決定の文書の置き場に読み替えた（判断の記録は凍結した木にあり、check も線も読まない）。戻した細部: コミットを足すたびに判定し直すこと（C-39）、両方の一覧と1行ずつ照らすこと（C-40）、一覧のファイル自身と移したファイル（C-41）、手順の失敗を当たりにすること（C-42） |
| REQ-core-366 | C-core-merge-2, C-core-merge-17, C-core-merge-18, C-core-merge-19, C-core-merge-20, C-core-merge-21, C-core-merge-43, C-core-merge-44, C-core-merge-45, C-core-merge-46, C-core-merge-47, C-core-merge-48, C-core-merge-49, C-core-merge-50 | 線に当たらない回は、プルリクエストを作り、必須の検査が通ったらマージするようホスティングのサービスに頼む | そのまま（A12、A23、A24、A28、A35、merge-flow-fixes の A10）。3つの決定に分けた（頼むこと、収束した回だけと頼みの取り消し、見届けること）。戻した細部: 承認のレビューとプルリクエストの向き先と確かめられないときの止まり方（C-43〜C-45）、検査の名前を読めないときの見届け（C-46）、収束した回だけがマージへ進むこと（C-47）、頼みの取り消しと読めないときの止まり方（C-48〜C-50） |
| REQ-core-367 | C-core-merge-22, C-core-merge-23, C-core-merge-24 | 人が確認するときは、マージを頼まずにプルリクエストを開いたままにし、場所と止まった原因だけを見せる | そのまま（線に当たったとき、人の判断を待つ指摘が残ったとき。A3、A21、A26、A32）と、読み替え（実装済み。実装役が承認済みの要求を変える・消す必要があると返したときを、人の決めた意図がぶつかると返したときに。整合のフェーズが仕様に書き足した具体化を、実装する側やフェーズが決めた細部と事実に合わせて直した文に）。人の判断を待つ指摘の出どころを問わないこと（戻した細部）は C-22 の文に入れた。3つ目の場合（整合のフェーズが承認済みの要求を書き改めたとき。A46）は消える（置き換わった。下の「形に依存する要求」）。人が確認する場合を2つの決定に分けた（3種類の決定と、止まり方の決定） |
| REQ-core-368 | C-core-merge-25, C-core-merge-26, C-core-merge-27, C-core-merge-28, C-core-merge-29, C-core-merge-51 | レビューの指摘のうち、回の収束を止めるのは3種類だけにする | そのまま（A10、A11、A17、A25、merge-flow-fixes の A2、A3、A5）。2つの決定に分けた（収束を止める3種類と、人を待たないもの）。実装役の判断の1行をプルリクエストの本文に書くことは [workflow.ja.md の「実装の途中で見つかった細部は、実装する側が決める」](workflow.ja.md#実装の途中で見つかった細部は実装する側が決める)（C-core-flow-3）にあるので、最後の報告にも同じものを出すことだけを書いた。走らせて示せない意図とのずれを壁打ちの側の出戻りに数えることは、同じ決定が「書いていない細部は実装する側が決める」と置き換えたので消える（今の skill では record_only の細部になる）。戻した細部: セキュリティの指摘を直すことを提案するものに限り、人の判断を求めるものを開いたまま残すこと（C-51） |
| REQ-core-369 | C-core-merge-30, C-core-merge-31, C-core-merge-32, C-core-merge-35, C-core-merge-52, C-core-merge-53 | 回について、新しい記録は作らず、指摘のファイルに線の結果と手直しの元と終わり方を足す | そのまま（A13、A14、A30、A31、A47）。受け取った ID は「受け取った決定は指摘のファイルに書き、再開はそこから読む」に置いた。受け取ったマージ先を書くこと（merge-flow-fixes の A4 の細部、fixes-consistency の A13）は落とした細部。戻した細部: マージで終わった回のファイルを再開として読まないこと（C-35）、片付けの条件と指摘のファイルの置き場（C-52、C-53） |

元の文書に場面、TBL、PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|

この話題の ID を指しているテストは無い（`grep -rn "@kotowari\[" crates src tests` に当たりが無い）。ガイドのマーカーも無い。`agent/skills/kotowari-review/references/finding-schema.md` の例に "REQ-core-364" が出てくるが、マーカーではなく `ids` の値の例である（C-core-merge-10 の提案で直る）。確認することの行はどれも「（読んで確認）」である。

## 戻した細部

どれも、今それを確かめているのは skill の文で、確認することの行は「（読んで確認）」にした。下の「確かめている文」は、リポジトリ `agent/skills/` の該当する文を読んで確かめた。

| 細部（safety-dropped.md の #） | 新しい ID | 確かめている文 |
|---|---|---|
| マージへ進むのは収束した回だけ（#50） | C-core-merge-47 | kotowari-cycle の `references/merge.md` の冒頭 "Only ending 1 comes here: endings 2–4 report to the person as before, with no pull request and no request to merge."、kotowari-cycle の skill の Endings の "Only ending 1 goes on to step 7. Endings 2–4 ask for no merge" |
| コミットを足すたびに線を調べ直し、足す前に頼みを取り消し、新しい先頭で確かめてから頼み直す。条件が崩れたときと人に渡すときも取り消す（#51） | C-core-merge-39, C-core-merge-48, C-core-merge-49 | `references/merge.md` の The line の "Judge the line again each time cycle adds commits to the branch, always over the whole diff"、Withdrawing the request の箇条書き3つと "Ask again only after confirming, for the new head, …" |
| 取り消すかはホスティングのサービスが頼みを示しているかで決め、委譲の前と push の前に読む。再開、さらに回す、iterate の使い回しでも同じ（#52） | C-core-merge-48 | `references/merge.md` の Withdrawing the request の "whether this session asked or an earlier one did" と1つ目の箇条書き |
| 頼みを読めないときは push せず止まる。プルリクエストを作れないホスティングだけは進む（#53） | C-core-merge-50 | `references/merge.md` の "When the read before a push fails, do not push: stop and tell the person, with the error output. The one exception is a host where cycle cannot create a pull request …" |
| 頼む条件に、承認のレビューを必須にしていないこと（#54） | C-core-merge-43 | `references/merge.md` の Opening and asking の5 "and that the merge target requires no approving review" |
| 承認のレビューは3つの読みがそろったときだけ確かめたとする。枝の保護は GraphQL の requiresApprovingReviews（#55） | C-core-merge-43 | 同じ5の3つのコマンドと "Only these exact outputs confirm: all three are needed, and any other output, an empty one from the first two included, is not a confirmation." |
| プルリクエストが受け取ったマージ先へ向いていること。再開で別の枝へ向いていたら取り消して人に渡す（#56） | C-core-merge-44 | 同じ5の "that the pull request points at the merge target cycle received"、Resuming の最後の段落 |
| 頼む条件を確かめられなければプルリクエストで止まる（#57） | C-core-merge-45 | 同じ5の "When one of these does not hold or cannot be confirmed (a command fails), stop at the pull request …" |
| 必須の検査の読みが1つでも失敗したら確かめられなかったものとし、見えている検査を見届けてから人に渡す（#58） | C-core-merge-45, C-core-merge-46 | Opening and asking の3 "a failing command leaves it unconfirmed, whatever the other printed"、Watching の "unconfirmed (a read failed): … then hand the pull request to the person with the failure and its error output." |
| `.kotowari/confirm-paths` 自身はいつも当たり（#59） | C-core-merge-41 | The line の "A change to the file itself always counts, whatever it lists" |
| 両方の一覧を読み、1行ずつ照らし、失敗は当たったパスの無い当たり（#60） | C-core-merge-40, C-core-merge-42 | The line の "each on its own so that no line can take a hit away from another. It reads both the merge target's current list and the branch's; a hit under either is a hit." と "A nonzero exit is a failure of the procedure, … treat it as a hit, record no paths, and report the failure with its error output" |
| 移したファイルは両方のパスで数える（#61） | C-core-merge-41 | The line の "`--no-renames` counts a moved file under both its old and its new path." |
| 枝の名前は使い回さず、マージ済みか閉じたプルリクエストなら止まって名前を求め、開いたプルリクエストの先頭が手元に無ければ止まる（#62） | C-core-merge-34 | kotowari-cycle の skill の Resuming and running more の最初の段落 |
| マージ先も既定の枝も読めなければ人に聞く。開始でホスティングのサービスを読めなければ確かめを省いて push の前に確かめ、報告する（#63） | C-core-merge-36 | kotowari-cycle の skill の Inputs の "when that read fails, ask the person for the merge target, never guess it"、Resuming and running more の "When the read fails …"、`references/merge.md` の "When the start could not make its two checks …, make them here too, before the push." |
| 壁打ちがコミットし、承認したファイルがコミットされていないあいだは戻さず止まる（#64） | C-core-merge-33 | kotowari-brainstorm の skill の Finishing の4 "when the person said the staged files may be committed …, the brainstorm commits them there … until both hold, do not switch: tell the person which approved files are not committed on the new branch, and stop." |
| 再開とあとの依頼の区別、始めの順（#65） | C-core-merge-37 | kotowari-cycle の skill の Inputs の "after the reads at the head of **Resuming and running more** and before it creates or changes the findings file"、Resuming and running more の "A resume is a start that brings no request … is a later request on the same branch, not a resume" と表の「a later request on the same branch」の行 |
| プルリクエストを作ったあとの再開はホスティングのサービスから読む（#66） | C-core-merge-38 | `references/merge.md` の Resuming の最初の段落 |
| 人の判断を待つ指摘は出どころを問わない（#67） | C-core-merge-22（文を直した） | `references/merge.md` の When a person confirms の "a finding is still open as `human_judgment`, whatever raised it (the consistency phase or a review)" |
| セキュリティの指摘を直すことを提案するものに限り、人の判断を求めるものは開いたまま残る（#68） | C-core-merge-51 | kotowari-cycle の skill の Judgment stays here の "one proposing `auto_fix` or `fix_and_verify` is visible and goes to the fixer …; one proposing `human_judgment` stays open, which `references/merge.md` counts as a reason for a person to confirm; one proposing `record_only` stays recorded." |
| 指摘のファイルは作業用のディレクトリの外に置き、片付けの前に確かめる（#69） | C-core-merge-53 | kotowari-cycle の skill の "never in the run's worktree, so that **Cleanup** in `references/merge.md` does not remove it"、`references/merge.md` の Cleanup の1 |
| 片付けは自分でマージを見たときだけ。作業メモのほかにコミットしていない変更があれば残す。人があとでマージした回は片付けない（#70） | C-core-merge-52, C-core-merge-53 | `references/merge.md` の Cleanup の "Only when cycle saw the merge itself, in this run's watch. A run the person merges later is not cleaned up." と2 |
| マージで終わった回の指摘のファイルは再開として読まない（#71） | C-core-merge-35 | kotowari-cycle の skill の "A file whose `ending` is `merged` belongs to an earlier request on a branch of the same name and is not a resume" |

## 落とした細部

### 安全に関わるもの

無い（22件すべてを上の「戻した細部」に戻した）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-364: あとの差分のレビューと、修正のあとの整合のフェーズの回は、いまの範囲のまま。前の回がすでにレビューした枝で続ける小さい作業は、その回が終わったところから読む | 主セッション（merge-flow-fixes の A1 の細部） |
| REQ-core-364: 承認のあと主の作業ツリーを戻す先は、壁打ちを始めたときの枝。その枝はセッションの中で覚えるだけでよい | 主セッション（merge-flow-fixes の A8 の細部）、整合のフェーズ（fixes-consistency の A6） |
| REQ-core-364: 指摘のファイルに書く時点（始めるとき）、受け取ったマージ先も書くこと、0件のときも0件として書くこと、あとの依頼は足すこと | 主セッション（merge-flow-fixes の A4 の細部）。マージ先を書くことは fixes-consistency の A13（整合のフェーズ）も |
| REQ-core-364: 仕様の確かめの範囲を、枝が変えた文書か cycle が受け取った ID に絞る | 主セッション（merge-flow-fixes の A9 の細部）。今の skill は「枝が変えた決定の文書」と書いている |
| REQ-core-366: プルリクエストを作る回はどれも、マージ先が検査を必須にしているかを確かめる | 主セッション（merge-flow-fixes の A10 の細部） |
| REQ-core-366: プルリクエストで止まる回でも、必須の検査を見届け、落ちたら直す | 主セッション（A44） |
| REQ-core-366: 見届けの終わり方（マージを見た、検査が落ちて人に渡した、検査は通ったのにマージしないと返した、プルリクエストで止まる回で検査が通った、プルリクエストがマージされずに閉じられた、必須の検査が1つも無い）と、cycle は見届けに自分の時間切れを持たないこと | 主セッション（A43）、整合のフェーズ（unattended-merge-consistency の A1、A8）。名前を読めないときの終わり方だけは安全に関わるので戻した（C-46） |
| REQ-core-366: 検査が通ったあとホスティングのサービスがマージしないと読む状態から、BLOCKED を外す | 整合のフェーズ（fixes-consistency の A3） |
| REQ-core-365: いちど線に当たったあと、後の変更で差分が一覧に触れなくなったら、当たりでないとして扱う | 主セッション（A44）。差分の全体で判定し直すこと（C-39）から読めるが、この帰結は文にしなかった |
| REQ-core-367: 止まった原因は、プルリクエストを作ったらすぐ、見届けの前に見せる | 主セッション（merge-flow-fixes の A10 の細部） |
| REQ-core-369: 片付けの前に数えるコミットしていない変更に、git が無視するファイルを数えない | 整合のフェーズ（unattended-merge-consistency の A6）、主セッション（merge-flow-fixes の P1） |
| REQ-core-369: マージで終わった回の指摘のファイルの名前を変えて残す | 整合のフェーズ（fixes-consistency の A5） |
| 確かめ方を置き換えた指摘の oracle.measured を、置き換える前の値のまま持つ | 整合のフェーズ（unattended-merge-consistency の A4）。数えることそのものは利用者が merge-flow-fixes の A2 で決めたので書いた |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-364: 壁打ちが足した・変えた要求の ID | 読み替えて残る（実装済み）。今回作る決定の一覧（決定の文書のパスと見出しのアンカー）。壁打ちが一覧を渡すことは [confirmations.ja.md の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」](confirmations.ja.md#状態は実装する側が変えまだ作らない決定は提案のまま置く)（C-core-confirm-10）で決めている |
| REQ-core-364: cycle は後回しの ID を外してから実装役に渡す | 消える。後回しは「提案」の状態に置き換わり（同じ決定）、cycle は今回作る決定の一覧だけを受け取る |
| REQ-core-364: 仕様の置き場と判断の記録の置き場にコミットしていない変更が無い | 読み替えて残る（実装済み）。決定の文書の置き場だけ。判断の記録は凍結した木に移る |
| REQ-core-364: 実装の途中の再開で、まだ印の付いたテストが無い要求（読んで確かめる種類を除く） | 読み替えて残る（実装済み）。まだ状態が「提案」の決定 |
| REQ-core-364: 再開で、枝の差分のうち仕様の置き場から数え直す | 読み替えて残る（実装済み）。決定の文書の置き場の差分が足した・変えた、提案の決定 |
| REQ-core-364、REQ-core-369: 指摘のファイルの `ids` | cycle は読み替え済み（実装済み）。iterate と `finding-schema.md` は読み替え（提案。C-core-merge-10） |
| REQ-core-365: 一覧に書かないのは仕様と判断の記録の置き場 | 読み替えて残る（実装済み）。決定の文書の置き場 |
| REQ-core-367: 3つ目の場合（整合のフェーズが承認済みの要求を、矛盾か再現で誤りだと示して書き改めたら、人が確認する。A46、利用者） | 消える（置き換わった）。[decision-documents.ja.md の「事実が変わったら書き直し、決定を覆すときは古い決定を見送った案に書く」](decision-documents.ja.md#事実が変わったら書き直し決定を覆すときは古い決定を見送った案に書く)で、決めた向きを変えずに事実に合わせて文を直すことは事実の書き直しになり、[workflow.ja.md の「実装の途中で…」](workflow.ja.md#実装の途中で見つかった細部は実装する側が決める)で、人に戻すのは意図のぶつかりだけになった。向きが変わる書き換えは意図のぶつかりとして「未決」に書かれ、人の判断を待つ指摘になるので、1つ目と2つ目の場合で止まる。止まる条件が1つ減るので、安全に関わる変化として挙げる。あとの利用者の決定で覆ったので、決定には戻さない |
| REQ-core-367: 実装役が承認済みの要求を変える・消す必要があると返した | 読み替えて残る（実装済み）。人の決めた意図がぶつかると返した（cycle の終わり方の4） |
| REQ-core-367: 承認のあとに整合のフェーズが仕様や判断の記録に書き足した具体化 | 読み替えて残る（実装済み）。実装する側やフェーズが決めた細部と、事実に合わせて直した決定の文 |
| REQ-core-368: 走らせて示せない意図とのずれは、仕様の穴として壁打ちの側の出戻りに数える | 置き換わる（workflow.ja.md の「実装の途中で…」）。書いていない細部は実装する側が決め、record_only として報告される |

### テストの無い行

無い（どの行も「（読んで確認）」）。

## 誰が決めたか見分けられなかったもの

- 無い。decided_by の「利用者（代決を承認）」は、主セッションが代わりに決めた案を利用者が個別に承認したもので、細部をまとめて任せた承認ではないので、利用者の決定として扱った。1つの決定の中に「…は主セッションが足し、見直しのラウンドで利用者に示した」とある部分は、その部分だけを主セッションの決定として扱った。

## 理由や見送った案が記録に無かった決定

見送った案が記録に無いので、節を置かなかった決定:

- 「小さい作業と指摘のファイルの形の説明でも、受け取ったものは決定の一覧として扱う」（理由も記録に無い。「別の理由は残っていない」と書いた）
- 「人が確認するときは、マージを頼まずにプルリクエストを開いたままにし、場所と止まった原因だけを見せる」
- 「マージへ進むのは収束した回だけにし、マージの頼みは、枝へコミットを足す前と条件が崩れたときに取り消す」
- 「マージのあとは、手元の作業用のディレクトリと枝だけを片付け、ホスティングのサービスの側のものは消さない」

見送った案はあるが、採らなかった理由が記録に無いもの:

- 「計画を書く段とその承認を置かない」の「計画は書くが、人の承認を待たない」と「計画の承認を人に残す」（A4 の rejected）。「採らなかった理由は記録に残っていない」と書いた。
- 「レビューの指摘のうち、回の収束を止めるのは3種類だけにする」の「設計の判断を含む依頼を、人が確認する側にする」（A17 の rejected）。同じく書いた。
- 「線は…」の「差分を読んで扱いを返す kotowari の新しいコマンドを作る」（A9 の rejected）。理由が並んで書かれていないので、理由を付けずに書いた。

戻した細部のうち、理由が記録に無かったもの（問題に「記録に残っていない」と書いた）:

- 移したファイルを両方のパスで数えること（A20 の細部）
- 頼む条件を確かめられないときに止まること（A35 の「確かめられなければ止まる」）
- セキュリティの指摘を直すことを提案するものに限ること（merge-flow-fixes の A5 の細部）
- プルリクエストを作ったあとの再開でホスティングのサービスから読むこと（A33 の細部）

## 分けた所、まとめた所

- REQ-core-364 を7つの決定に分けた: 壁打ちと承認を人に残すこと（A1、A15、A22、A24）、計画の段を置かないこと（A4、A27）、cycle の入力と範囲と未承認の止まり方（A33、merge-flow-fixes の A1、A9）、指摘のファイルと再開（merge-flow-fixes の A4、A33）、枝の名前と始めと再開の守り（merge-flow-fixes の A7、fixes-consistency の A4、A9、A11、A45、A33 の細部）、テストのコマンド（A33、merge-flow-fixes の A11）、壁打ちの枝と作業用のディレクトリ（merge-flow-fixes の A8、fixes-consistency の A12）。iterate と形の資料の `ids` は提案の決定に分けた。
- 人が確認する3種類（A3）を、線の判定（REQ-core-365）と別の決定にした。人に戻すのは人でないと決められないことだけ、という原則（A15）は、最初の決定にまとめた。
- REQ-core-366 を、頼むこと（頼む条件の守りを含む）、収束した回だけと頼みの取り消し（A39、A41、unattended-merge-consistency の A7、fixes-consistency の A8）、見届けることの3つに分けた。頼みの取り消しは、取り消す場面と頼み直す条件と読めないときの止まり方が1つの決まりなので、1つの決定にまとめた。主セッションが直接編集する変更はコミットで終わること（A24）は、最初の決定に入れた。
- 線の守り（A19 の最後の2文、A20 の細部、A39 の前半、unattended-merge-consistency の A5）は、線の決定に文を足した。
- REQ-core-368 を、収束を止める3種類と、人を待たないもの（テストの削除、検査の抑制、依存、読みやすさ、判断の1行）の2つに分けた。
- 指摘のファイルに残す5つのうち、受け取った ID は指摘のファイルと再開の決定に、残りの3つは回の記録の決定に置いた（マージ先は落とした細部）。片付けの守り（A40、A43）は片付けの決定に文を足した。
- A18、A34、A42（このリポジトリの CI と一覧の中身。記録に「製品の振る舞いではないので IR に書かない」とある）、A2、A5〜A8、A29、A36、P1、P2 は、この話題の要求が引いていないか、製品の振る舞いでないので書かなかった。

## あとでリンクにする所

無い（工程の話題、決定の文書、確認することへの言及は、どれも見出しへのリンクにした）。

## 迷った判断と、どう決めたか

- 「主セッション（…見直しのラウンドで利用者に示した）」の決定は、agent の細部として扱った（書き直しの決まりのとおり）。そのうち安全に関わるものは、利用者の決定に従って決定に戻した。安全に関わらないもの（上の「そのほか」）は落としたままにした。
- 戻した守りは、22件を1件ずつの決定にせず、1つの決まりの面になっているものをまとめた。新しい決定は2つ（枝の名前と始めと再開の守り、収束した回だけと頼みの取り消し）で、ほかは属する既存の決定に文と行を足した。
- 人が確認する場合の2つ目を、出どころを問わない形に直した（C-core-merge-22 の文も直した）。記録の A32 は、本文が出どころを問わない形で、decided_by が「利用者（代決を承認）。広げたのは主セッション」である。広げた部分は安全に関わる細部として戻す対象になる。
- 既存の見出しは変えなかった。「線に当たらない回は…」の見出しに「収束した」を戻さず、収束した回だけがマージへ進むことは、新しい決定「マージへ進むのは収束した回だけにし、…」に書いた。ほかの話題の下書きから見出しへのリンクを壊さないためである。
- 3つ目の場合（A46）は、利用者の決定だが、あとの利用者の決定（決定の文書の形と工程）で置き換わったと判断した。止まる条件が減るので、形に依存する要求の表で安全に関わる変化として書き、決定には戻さなかった。
- セキュリティの指摘を「直すことを提案するもの」に限ったのは主セッションで、利用者が決めたのは「セキュリティの指摘では止めない。落ちるのを見たかにかかわらず修正役に渡す」である。限ったことと、人の判断を求めるものを開いたまま残すことは、安全に関わる細部として戻した（C-51）。
- 「人がマージを確認するのは…3種類にする」には確認することを置かなかった。3種類それぞれの振る舞いは、線と、人が確認するときの決定の確認することで確かめる。
- 自動のマージの設定を利用者が有効にしたこと（merge-flow-fixes の A15）は、このリポジトリの設定の事実で、製品の振る舞いではないので書かなかった。
- 「線に当たらない回は…頼む」に、マージ先が検査を必須にしているかの読み方（2つの読みのどちらかが空でない一覧なら必須、どちらも空なら必須でない）も書いた。#58 の「1つでも失敗したら確かめられない」は、この読み方の中でしか意味が通らないので、その前提として書いた（今の `references/merge.md` の Opening and asking の3の文）。「プルリクエストを作る回はどれも確かめる」は落としたままにした。
- 「枝の名前は…」の見送った案の「先頭のコミットで見分ける」は、利用者が代決で承認したあと取り替えられた案なので、「以前の決定:」と書いた。
- 戻した守りの「問題」には、記録の why から、落とすと何が起きるかを書いた。why が無いもの（上の「理由や見送った案が記録に無かった決定」の最後の一覧）は「記録に残っていない」と書いた。

## 実装する側が決める細部

無い。読み替えで要る名前（`ids`、`merge_target`、`line`、`fix_of`、`ending`、human_judgment、record_only）は、今の skill が使っている名前をそのまま使い、決定の文には `ids` だけを書かずに「受け取った決定」と書いた。

## 利用者に聞くこと

無い。

## 「提案」で書いた決定の一覧

| 決定 | 変えることになりそうな所 |
|---|---|
| 小さい作業と指摘のファイルの形の説明でも、受け取ったものは決定の一覧として扱う | `agent/skills/kotowari-iterate/SKILL.md` の「This run adds to `ids` the requirement IDs its request touches」と The loop の表の "requirement IDs" の行、`agent/skills/kotowari-review/references/finding-schema.md` の `ids` の説明と例 |

## 数

- 元: 要求 6、場面 0、決定表 0、性質 0
- 読んだ記録のファイル: 4（2026-11-24-unattended-merge.md、2026-11-24-unattended-merge-consistency.md、2026-11-24-merge-flow-fixes.md、2026-11-24-merge-flow-fixes-consistency.md）
- 読んだ skill の文: kotowari-cycle の SKILL.md と references の merge.md、consistency-phase.md、editing-contract.md、premise-step.md、kotowari-brainstorm の SKILL.md
- 出来上がり: 決定 18（実装済み 17、提案 1）、確認することの行 53（すべて「（読んで確認）」）
- テスト: 0
- 戻した細部: 22（新しい行 21、直した行 1）。戻せなかった細部: 0
- 直した「など」: 0（この文書の「決めたこと」と「確認すること」には無かった）
- 利用者に聞くこと: 0
