# リリース変更の準備と確定

この操作仕様は[追加判断](../decision/records/2026-10-01-release-conformance.md)を、独立 quality/conformance の2観点 review を経て今回採用した具体化である。実リリースの実行は別途承認を要する。リリースは IR 外にあり、CLI の既存 IR を変更しない。

## 入力と開始

入口は `scripts/release.sh <製品> <版>` または `scripts/release.sh prepare <製品> <版>`。どちらも準備だけを行う。製品と版の文字列、2製品の生成対象と変更履歴、版の追随規則は既存スクリプトを維持する。新規操作は `finalize`、`status`、`abort` の後に同じ製品と版を取る。不正な引数は終了2、拒否・検査失敗は終了1、指定操作の成功は終了0。準備の終了0はタグを意味しない。

prepare は main、tracked・untracked を含めた clean-tree、同じ作業ツリーに未完了の準備がないことを要求する。無視対象のビルド生成物は従来通り許す。ローカル同名タグ・origin 同名タグがあれば拒否し、origin に到達できず不在を確認できない場合も拒否する。既存の Unreleased の非空検査を維持する。

## 既存入口からの移行

従来の2引数の終了0はタグ作成までを意味したが、今後は準備だけを意味する。自動化している呼び出し元は記録と独立 review の受け渡しを追加し、finalize を別に実行する必要がある。失敗時に何も残さず戻す動作も変更される。CHANGELOG の Unreleased に breaking な運用変更として記載し、PROJECT.md の手順・失敗時の復旧を同時に更新する。

## 準備状態

作業ツリーごとに `.agents/release/prepared.json` を1件保存する。このパスと `.agents/release/lock/` は ignore されていなければ準備開始前に停止する。状態は version 1 の JSON で、製品・版・タグ、開始 HEAD の完全 SHA、生成日、生成ファイル一覧と各ファイルの変更前後の mode・blob、開始時の index tree、段階、`planned_tree` と `candidate_commit` を持つ。欠落・未知の版・不正な型の状態を推測して修復しない。状態を shell の source/eval として実行しない。

段階は preparing、prepared、candidate、complete、aborted。`planned_tree` は staging が完了するまで null とし、生成ファイルと許された記録の追加・更新・削除を stage して予定 tree を確定した時点で、commit 実行前にその完全な tree object ID を保存する。`candidate_commit` は候補 commit を判定するまで null とし、commit の実行後または中断後の再開時に、開始 SHA・planned_tree・固定 commit メッセージとの一致を確認した時点で完全な commit SHA を保存する。candidate 段階は両方が確定した状態とする。その他の必須項目も省略しない。状態の保存は同じディレクトリの一時ファイルから rename して途中の JSON を読ませない。

prepare は書き換え前に全生成内容を計算し、変更前後を preparing として保存してから固定した生成内容を配置する。完了時に prepared にする。準備中断後は保存された前後の内容のどちらかであるファイルだけを今回の準備と認め、同じ製品・版の prepare が残りの配置を完了できる。前後のどちらでもない変更、HEAD の変化、index の変更、生成対象外の変更があれば停止して保存状態を保持する。準備では index を更新しない。

状態は内容固定と操作再開のためだけに使う。判断の理由、根拠、review の結果は tracked な判断記録・照合記録へ残し、ignored 状態へ隠さない。

## LLM への受け渡し

prepare の出力には製品・版・タグ、開始 SHA、生成ファイルと確定コマンドを示す。LLM は生成差分を独立に読み、変更が既存のリリース仕様に従うか照合する。実装側と別の review が `changes.records` に当たる現在の implementation.yaml と review.yaml を別々に作成または置換し、不要な commit.yaml を削除する。両者の base は開始 SHA、生成ファイルの before/after と関連 IR の識別値は現在の固定内容に対応させる。

release 固有仕様は IR 外にあるため、リリース生成変更の conclusion は `new` とし、既存リリース判断への decisions 参照と根拠を記す。ここで `new` は未承認仕様を自動採用する意味ではなく、IR の existing 要求へ対応させられない判断の記録で支える変更として使う。仕様を変えないリリース生成に無関係な IR を創作して `existing` を通してはならない。関連 IR がなければ ir は空でもよく、根拠の意味は独立 review が確認する。

この段階で生成内容を修正したり、判断記録・IR・製品コードへ新しい変更を足したりする必要があれば finalize は行わない。準備を安全に中止し、必要な通常変更を先に完了して clean-tree から再準備する。新しい判断記録が必要なのに根拠がない状態を、照合 YAML の reason だけで代替しない。

## 確定前の混入検査

finalize は main と準備状態を確認する。prepared では HEAD が開始 SHA のまま、生成ファイルは mode・blob とも保存した after と一致することを要求する。候補前に base が進んだ場合は停止し、自動 rebase や base の書き換えをしない。

許される tracked の差分は生成ファイルの固定変更と `changes.records` に当たる記録の更新・削除だけ。追加の untracked/index-added ファイルは設定された記録だけを許す。生成対象外かつ記録以外の tracked 変更、記録以外の untracked、symlink やリポジトリ外参照、競合 index は拒否する。stage 済みと未 stage の同じファイルの内容が異なる場合も拒否する。参照検査は追加・更新後に残る記録すべてについて通す。記録削除で必要な照合が欠ければ changes が拒否する。生成物用の既存 ignore は維持する。

確定は対象パスを個別に stage する。生成ファイルと許された記録の追加・更新・削除から予定 tree を固定し、その完全な tree ID を `planned_tree` に保存してから、`changes --base HEAD --staged --phase implementation` を実行する。`git commit` は既存フックを全て通す。フック後に tree が予定内容と異なればタグを作らず候補を保持して停止する。

## 候補の検証とタグ

コミットは開始 SHA を唯一の親とし、予定 tree と一致することを確認する。commit 実行前に保存した `planned_tree` を照合に使うため、commit の直後に中断しても、HEAD の親・tree と固定 commit メッセージ `chore: <製品> <版> をリリースする` から今回の候補か照合できる。合致しない新しい HEAD は採用しない。合致した候補の完全 SHA を `candidate_commit` に保存してから candidate 段階へ進む。

candidate では保存した候補 SHA が HEAD と一致し、clean-tree であることを要求する。同じ候補 checkout に対して、`scripts/check-versions.sh <tag>`、`CARGO_BUILD_JOBS=4 cargo run -q -p kotowari -- check --format text` の終了0、`CARGO_BUILD_JOBS=4 cargo test --workspace`、`changes --base <開始SHA> --head <候補SHA> --phase review` の終了0を要求する。実装側だけの記録、古いコード・IR の記録、deferred の記録、review の欠落は確定不可。

タグ直前にも main、HEAD、clean-tree、候補 tree と origin の同名タグ不在を再確認する。ローカルタグがないときだけ、その候補へ注釈付きタグを作る。検査からタグ作成まで別の作業をしない。origin の同名タグ不在は照会時点の事実であり、公開時の競合は従来の push 拒否と公開済みタグ不変のルールで扱う。

タグ作成後は complete と候補 SHA・タグ対象を保存し、push がまだであることと従来の公開手順を表示する。complete への finalize は同じ注釈付きローカルタグ・候補 HEAD を確認して結果だけを報告し、再作成しない。タグ作成直後の中断では candidate と一致する対象と注釈を確認して complete に進められる。タグの注釈本文は `<製品> <版>` とし、保存状態と異なる対象・種類・注釈の同名ローカルタグは削除せず停止する。

## 失敗・中止・再開

検査失敗は生成内容・記録・候補コミットを保持し、タグを作らず終了1。check/test/changes の失敗をフック無効化や除外追加で通さない。candidate の再検証は同じ HEAD と clean-tree のまま finalize を再実行する。候補を修正する必要があれば状態と異なる commit を自動受け入れず、通常の履歴修正と再準備の判断へ戻す。

commit 前の abort は main と開始 HEAD を確認し、生成対象の現在内容が保存した前後のどちらかであり、index が未変更または生成後内容だけを持つときに限って生成対象を変更前へ戻す。LLM が追加・更新・削除した記録は index/作業ツリーともそのまま保持し、復元・削除しない。生成ファイルに他者の変更が混ざった場合は何も戻さず停止する。abort 完了は aborted に保存する。再準備には保持した記録を利用者が保存・コミットするか片付け、clean-tree に戻す必要がある。

candidate/complete の abort は履歴・タグ・照合記録を一切変更せず、開始点・planned_tree・candidate_commit とタグ情報を保持して操作状態だけを aborted として保存し、明示中止の成功として終了0にする。保存候補と現在 HEAD を両方報告する。現在 HEAD が候補と違う場合も、無関係な HEAD を保存候補へ採用せず、Git の結果を合格・確定と扱わない。ローカルタグや公開済みタグは有無を問わず触らない。利用者が通常の修正や、公開済みかを確認した上で必要な手元履歴の処理を行い、clean-tree に戻した後に再準備する。ローカルまたは origin に同名タグがあれば prepare は既存規則どおり拒否する。自動 reset、タグ削除、force push は行わない。aborted への abort 再実行は Git に触れず既存の中止結果を報告して終了0とする。

status は保存状態と Git の照合結果を表示するだけで、生成・stage・commit・tag を行わない。complete または aborted の状態を確認した後は、新しい prepare がその終端状態を新しい開始状態で置き換えられる。置換前に終端状態の全情報を ignored な `.agents/release/history/` の新規ファイルへ保存し、履歴ファイルを上書きしない。保存に失敗したら新しい準備へ進まない。この履歴は操作の再確認に限り、tracked の根拠・照合記録を代替しない。未完了状態を上書きしない。tracked の判断・照合記録と Git のタグは状態置換でも保持する。全変更操作は作業ツリー内の mkdir ロックを取る。ロック取得失敗は停止する。通常終了は自分のロックだけを削除する。SIGKILL 等でロックが残った場合は勝手に奪わず、保持者が停止したことを確認してロックだけを解除してから status で状態を確認する。既存の準備状態や証拠をロック解除で消さない。共有作業ツリーへの別書込みは禁止し、混入は各境界で検出して停止する。

## 観測する受け入れ条件

実公開をしない一時 Git リポジトリとローカル bare origin で、両製品の準備、clean-tree/main 拒否、prepare の commit/tag 不在、受け渡し後の混入・base 前進拒否を観測する。
実際の Git index と pre-commit を使い、未照合・古い照合・独立 review 不在でタグができないことと、両役の適合記録で候補と注釈付きタグの対象が一致することを観測する。
origin 同名タグ、origin 到達失敗、チェック失敗、pre-commit 拒否、commit 前後・タグ前後の中断、再確定、共有作業の混入、abort 時の記録保持、candidate の HEAD が変わっていても状態だけを中止して参照・tree・index・記録を保持すること、中止後の通常修正と clean-tree からの再準備、および終端状態の ignored 履歴保存を観測する。
テスト fixture は一時領域の PATH とローカル Git remote で外部コマンドの成功・失敗を作る。製品スクリプトへ検査免除のオプションは足さない。スクリプトの文言・行順の一致を oracle とせず、Git の参照、tree、index、ファイル内容、終了状態で判定する。

既存の固定記録を含む clean な開始コミットから次の release を準備し、記録の更新・不要記録の削除で finalize が通ることを観測する。候補前・候補後 abort が追加・更新・削除済み記録の index と作業ツリーを維持することも観測する。記録以外の通常 tracked 変更は引き続き拒否する。
