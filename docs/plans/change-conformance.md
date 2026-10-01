# Plan: 変更の照合漏れと判断の書き戻しを検出する

## Goal

今回の実行対象は S9、S10、S12。S1〜S7 は完了済みの履歴であり再実行しない。旧 steps の docs/changes・active/archived の記述より、今回改訂した IR と追加 steps の現在状態契約を優先する。S12 が今回の最終検証を更新する。

実装側の記録を pre-commit で、独立 review の照合を取り込み前と CI で要求し、未申告の変更・古い照合・処理先の無い仕様の穴を検出する。

## Specification

IR は docs/ir/。基礎となる仕様コミットは e2159ac。コマンド一覧表記の補足と REQ-core-277 は今回の文書レビューで採用し、計画の実行前にコミットする。対象要求は次の通り。

- docs/ir/core/cli.md#REQ-core-001
- docs/ir/core/cli.md#REQ-core-003
- docs/ir/core/cli.md#REQ-core-004
- docs/ir/core/config.md#REQ-core-016
- docs/ir/core/config.md#REQ-core-019
- docs/ir/core/decision-records.md#REQ-core-093
- docs/ir/core/changes.md#REQ-core-240
- docs/ir/core/changes.md#REQ-core-241
- docs/ir/core/changes.md#REQ-core-242
- docs/ir/core/changes.md#REQ-core-243
- docs/ir/core/changes.md#REQ-core-244
- docs/ir/core/changes.md#REQ-core-245
- docs/ir/core/changes.md#REQ-core-246
- docs/ir/core/changes.md#REQ-core-247
- docs/ir/core/change-records.md#REQ-core-248
- docs/ir/core/change-records.md#REQ-core-249
- docs/ir/core/change-records.md#REQ-core-250
- docs/ir/core/change-records.md#REQ-core-251
- docs/ir/core/change-records.md#REQ-core-252
- docs/ir/core/change-records.md#REQ-core-253
- docs/ir/core/change-records.md#REQ-core-254
- docs/ir/core/change-records.md#REQ-core-255
- docs/ir/core/change-workflow.md#REQ-core-256
- docs/ir/core/change-workflow.md#REQ-core-257
- docs/ir/core/change-workflow.md#REQ-core-258
- docs/ir/core/change-workflow.md#REQ-core-259
- docs/ir/core/change-workflow.md#REQ-core-260
- docs/ir/core/change-workflow.md#REQ-core-261
- docs/ir/core/change-workflow.md#REQ-core-262
- docs/ir/core/changes-inputs.md#REQ-core-263
- docs/ir/core/changes-inputs.md#REQ-core-264
- docs/ir/core/changes-inputs.md#REQ-core-265
- docs/ir/core/changes-inputs.md#REQ-core-266
- docs/ir/core/changes-inputs.md#REQ-core-267
- docs/ir/core/change-record-format.md#REQ-core-268
- docs/ir/core/change-record-format.md#REQ-core-269
- docs/ir/core/change-record-format.md#REQ-core-270
- docs/ir/core/change-record-format.md#REQ-core-271
- docs/ir/core/changes-results.md#REQ-core-272
- docs/ir/core/changes-results.md#REQ-core-273
- docs/ir/core/changes-results.md#REQ-core-274
- docs/ir/core/change-workflow.md#REQ-core-275
- docs/ir/core/change-workflow.md#REQ-core-276
- docs/ir/core/change-workflow.md#REQ-core-277
- docs/ir/core/config.md#TBL-core-004
- docs/ir/core/cli-environment.md#TBL-core-020

## Approach and why

記録の解析・参照検査、Git の読み取り、照合の純粋な判定、CLI の接続を分ける。既存の serde-saphyr、serde_json、globset、sha2 と Git の plumbing を使い、Git の履歴解析やパーサーを自作しない。既存の sources・finding・status に接続する。副作用は adapter に閉じ、照合には取得済みの値を渡す。新しい依存が必要なら理由と固定する契約を返す。自分自身のフックは新コマンドと記録が用意できた最後に有効化する。スキルの意味は独立 review で検証する。

## Scope of change

今回追加する改訂は [.kotowari/changes/ の判断](../decision/records/2026-10-01-current-change-records.md)に従う。旧 docs/changes/ を削除し、新配置と記録形式へ置換する。.ignore と changes.files の対象追加を含む。docs/release/ の場所は今回変更しない。

- crates/kotowari-core/src の記録解析・照合・Git adapter の新モジュール、config、lib、status、finding の接続
- src/main.rs、tests とコアの単体テスト
- agent/skills の kotowari、implement、cycle、review、plan、brainstorm、iterate、using-workflow と直接参照する資料
- docs/guides、agent/skills/README.md、PROJECT.md、CHANGELOG.md
- .kotowari/config.yaml、lefthook.yml、docs/changes、必要最小限の scripts
- .github/workflows/change-conformance.yml の PR 検査をローカルに作るところまで

## Step order and prerequisites

S1 → S2 → S3 → S4 → S5 → S6 → S7。S1〜S7 の初回検証は 5d2dcec までに完了済みで、以下に過去の実行計画を保持する。現在状態の改訂は S9 → S10 → S12 を追加実行し、最終検証を更新する。S7 は追加改訂と照合記録も含めた最終対象 SHA でやり直す。実装開始時に branch と worktree を宣言し、計画と仕様コミットを引き継ぐ。全コード変更は ba0918-tdd に従い RED と GREEN の実行結果を残す。

## Verification map

| Step | Requirements and tables | Examples |
|---|---|---|
| S1 | REQ-core-016, REQ-core-248, REQ-core-249, REQ-core-250, REQ-core-251, REQ-core-252, REQ-core-254, REQ-core-255, REQ-core-264, REQ-core-268, REQ-core-269, REQ-core-270, REQ-core-271, REQ-core-273、TBL-core-004 | EX-core-437〜440、442、446、448〜449、453、455 |
| S2 | REQ-core-003, REQ-core-019, REQ-core-241, REQ-core-245, REQ-core-246, REQ-core-247, REQ-core-263, REQ-core-265, REQ-core-266, REQ-core-267, REQ-core-269 | EX-core-434〜435、441、443〜445 |
| S3 | REQ-core-242, REQ-core-243, REQ-core-244, REQ-core-247, REQ-core-272, REQ-core-273, REQ-core-274 | EX-core-430〜433、436、447、450〜452、454 |
| S4 | REQ-core-001, REQ-core-003, REQ-core-004, REQ-core-240, REQ-core-249, REQ-core-253, REQ-core-254, REQ-core-263, REQ-core-264, REQ-core-274、TBL-core-020 | EX-core-219、241 と上記例の CLI 接続 |
| S5 | REQ-core-093, REQ-core-256, REQ-core-257, REQ-core-258, REQ-core-259, REQ-core-260, REQ-core-261, REQ-core-262, REQ-core-275, REQ-core-276, REQ-core-277 | review 要求は手順と独立 review で確認 |
| S6 | REQ-core-258, REQ-core-259, REQ-core-261, REQ-core-276 | 未ステージの記録・複数コミット・イベント比較元の実行検証 |
| S7 | REQ-core-240, REQ-core-248, REQ-core-259, REQ-core-274 | 計画の全要求と全例のテスト対応と照合 |

## Left to the implementer

内部の型・モジュール・テストの名前と helper の抽出。承認された入出力・保存形式・合否を保つ選択に限る。Git の引数は shell の文字列へ連結せず渡す。

## Stop conditions

仕様に無い入力・合否・保存形式・停止理由が必要なら実装前に戻す。承認済み要求の変更・削除と矛盾する追加は自律追記に含めない。フックや既存チェックを弱めない。不可逆・危険・権限が必要な操作と公開・push は人へ戻す。

## Out of scope

意味や本人性の機械的な証明、関数単位の Git 照合、symlink・submodule の新しい対応、別版の自動移行、リリース・タグ・push。計画を承認するだけでリモートに CI を公開しない。

## Steps

### S1: 記録の静的検査

- Purpose: 記録の静的検査ことで次の段階が必要とする判定と証拠を用意する
- Specification: docs/ir/core/config.md#REQ-core-016, docs/ir/core/change-records.md#REQ-core-248, docs/ir/core/change-records.md#REQ-core-249, docs/ir/core/change-records.md#REQ-core-250, docs/ir/core/change-records.md#REQ-core-251, docs/ir/core/change-records.md#REQ-core-252, docs/ir/core/change-records.md#REQ-core-254, docs/ir/core/change-records.md#REQ-core-255, docs/ir/core/changes-inputs.md#REQ-core-264, docs/ir/core/change-record-format.md#REQ-core-268, docs/ir/core/change-record-format.md#REQ-core-269, docs/ir/core/change-record-format.md#REQ-core-270, docs/ir/core/change-record-format.md#REQ-core-271, docs/ir/core/changes-results.md#REQ-core-273
- Prerequisites: なし。仕様コミット e2159ac と末尾の補足を確認する
- May change: コアの記録解析・参照検査・config・lib・status、記録と設定のテスト
- Done when: active/archived、欠落した必須情報・型・版・参照、混在する穴の処理先と未設定の互換性を観察できる
- Shown by: test — 対応する例を check と status の読み取り経路で検証する。CARGO_BUILD_JOBS=4 cargo test -p kotowari-core と該当する CLI テスト
- Left to the implementer: 型とモジュールの名前、既存 sources 検査の再利用
- Stop and hand back if: 保存契約や静的検査の合否を追加する必要が出る

### S2: Git の snapshot と差分を読む

- Purpose: Git の snapshot と差分を読むことで次の段階が必要とする判定と証拠を用意する
- Specification: docs/ir/core/cli.md#REQ-core-003, docs/ir/core/config.md#REQ-core-019, docs/ir/core/changes.md#REQ-core-241, docs/ir/core/changes.md#REQ-core-245, docs/ir/core/changes.md#REQ-core-246, docs/ir/core/changes.md#REQ-core-247, docs/ir/core/changes-inputs.md#REQ-core-263, docs/ir/core/changes-inputs.md#REQ-core-265, docs/ir/core/changes-inputs.md#REQ-core-266, docs/ir/core/changes-inputs.md#REQ-core-267, docs/ir/core/change-record-format.md#REQ-core-269
- Prerequisites: S1、Git を実行できるローカル環境
- May change: コアの Git adapter・snapshot 型、Git を使うテスト
- Done when: staged と unstaged、commit、追加・削除・移動・mode の変更、対象外と不対応入力、設定の基準を区別できる
- Shown by: test — 一時 Git リポジトリに実際の変更と index を作って内容と識別値を検査する。CARGO_BUILD_JOBS=4 cargo test -p kotowari-core
- Left to the implementer: plumbing の選択と引数、取得済みデータの内部型
- Stop and hand back if: Git の取得結果が mode と全バイトの識別契約を満たせない

### S3: 対応と鮮度と段階を判定する

- Purpose: 対応と鮮度と段階を判定することで次の段階が必要とする判定と証拠を用意する
- Specification: docs/ir/core/changes.md#REQ-core-242, docs/ir/core/changes.md#REQ-core-243, docs/ir/core/changes.md#REQ-core-244, docs/ir/core/changes.md#REQ-core-247, docs/ir/core/changes-results.md#REQ-core-272, docs/ir/core/changes-results.md#REQ-core-273, docs/ir/core/changes-results.md#REQ-core-274
- Prerequisites: S1、S2
- May change: コアの照合の純粋な判定と単体テスト
- Done when: 申告漏れ、before/after/IR の古さ、両役のまとめ方の差、関連 IR の包含、結論の不一致と段階別の保留を検出する
- Shown by: test — 対応する例と reviewer の関連 IR が不足する対照を判定する。CARGO_BUILD_JOBS=4 cargo test -p kotowari-core
- Left to the implementer: 内部データ構造と純粋関数の分け方
- Stop and hand back if: 複数件の対応に仕様に無い優先規則が必要になる

### S4: CLI と利用案内を接続する

- Purpose: CLI と利用案内を接続することで次の段階が必要とする判定と証拠を用意する
- Specification: docs/ir/core/cli.md#REQ-core-001, docs/ir/core/cli.md#REQ-core-003, docs/ir/core/cli.md#REQ-core-004, docs/ir/core/changes.md#REQ-core-240, docs/ir/core/change-records.md#REQ-core-249, docs/ir/core/change-records.md#REQ-core-253, docs/ir/core/change-records.md#REQ-core-254, docs/ir/core/changes-inputs.md#REQ-core-263, docs/ir/core/changes-inputs.md#REQ-core-264, docs/ir/core/changes-results.md#REQ-core-274
- Prerequisites: S1〜S3
- May change: src/main.rs、コアの公開接続と停止理由、CLI テスト、docs/guides、CHANGELOG.md
- Done when: 引数・JSON/text・終了コード・一覧表記を実行で確認し、旧コマンドの契約を維持し、今回古くなったガイド印を内容確認後に更新する
- Shown by: test — 一時リポジトリで changes を実行し、欠落・不正な引数・Git の停止・設定基準を観察する。CARGO_BUILD_JOBS=4 cargo test --workspace
- Left to the implementer: 引数の内部型と既存の出力処理の再利用
- Stop and hand back if: 停止の表記や入力境界が仕様に足りない

### S5: 判断の書き戻しと独立 review の手順を整える

- Purpose: 判断の書き戻しと独立 review の手順を整えることで次の段階が必要とする判定と証拠を用意する
- Specification: docs/ir/core/decision-records.md#REQ-core-093, docs/ir/core/change-workflow.md#REQ-core-256, docs/ir/core/change-workflow.md#REQ-core-257, docs/ir/core/change-workflow.md#REQ-core-258, docs/ir/core/change-workflow.md#REQ-core-259, docs/ir/core/change-workflow.md#REQ-core-260, docs/ir/core/change-workflow.md#REQ-core-261, docs/ir/core/change-workflow.md#REQ-core-262, docs/ir/core/change-workflow.md#REQ-core-275, docs/ir/core/change-workflow.md#REQ-core-276, docs/ir/core/change-workflow.md#REQ-core-277
- Prerequisites: S4
- May change: agent/skills の関連スキルと参照資料、agent/skills/README.md、docs/guides、CHANGELOG.md
- Done when: 途中用と最終用の記録、優先確認するテスト変更、自律追記と戻す条件、穴の処理先、独立 review と再照合をスキル間で矛盾なく追える
- Shown by: artifact — 変更スキルを要求と照合する独立 review の結果を残す。形式・参照は kotowari check で確認し、文章一致のテストは作らない
- Left to the implementer: 説明の配置と共通参照の切り出し
- Stop and hand back if: 承認済み要求の変更を許す必要や、仕様の無い権限が出る

### S6: フックと CI を導入する

- Purpose: フックと CI を導入することで次の段階が必要とする判定と証拠を用意する
- Specification: docs/ir/core/change-workflow.md#REQ-core-258, docs/ir/core/change-workflow.md#REQ-core-259, docs/ir/core/change-workflow.md#REQ-core-261, docs/ir/core/change-workflow.md#REQ-core-276
- Prerequisites: S4、S5、新コマンドを実行でき、今回の途中用と最終用の記録を用意できること
- May change: 設定、lefthook.yml、必要最小限の scripts、.github/workflows/change-conformance.yml、docs/changes、docs/guides、PROJECT.md
- Done when: 整形後の index を検査し、イベント由来の比較元と対象を CI に渡し、自己導入を検査の免除なしに行える
- Shown by: check — 一時 Git リポジトリでフックと比較元を作る処理を実行し、未ステージの記録と以前の未照合コミットで停止することを確認する。CI 構成は actionlint、呼び出し対象と段階は独立 review で確認する
- Left to the implementer: フックのシェル構成と補助処理の分け方
- Stop and hand back if: 自己導入にフック無効化や未規定の免除が必要になる、または CI の比較元を確定できない

### S7: 計画の対象を検証して引き渡す

- Purpose: 計画の対象を検証して引き渡すことで次の段階が必要とする判定と証拠を用意する
- Specification: docs/ir/core/changes.md#REQ-core-240, docs/ir/core/change-records.md#REQ-core-248, docs/ir/core/change-workflow.md#REQ-core-259, docs/ir/core/changes-results.md#REQ-core-274
- Prerequisites: S1〜S6、仕様と実装の独立 review とその指摘への対応が完了。必要な修正と両役の最終記録をコミットし、引き渡す対象 commit の完全な SHA を固定済み
- May change: docs/changes の照合記録、今回の検証で修正が必要と分かった Scope of change 内の箇所
- Done when: 計画の全要求と例に必要なテストがあり、固定した対象 SHA と一致する変更でテストと check を検証し、変更ファイルと対象 ID に check の誤りがなく、review 段階の changes が終了0となる。取り込み可能とするには check 全体の終了0も必要。対象外の誤りがあれば範囲を広げず、実装範囲の結果と取り込みゲート未通過を区別して引き渡す
- Shown by: check — 対象 SHA の checkout で git status --porcelain に今回の未コミット変更が無いことを確認し、CARGO_BUILD_JOBS=4 cargo test --workspace、kotowari query の対象テスト対応、kotowari check --format json の対象内の誤り0と全体の終了コード、kotowari changes --base <比較元の完全な SHA> --head <対象の完全な SHA> --phase review --format json の終了0を記録する。比較元・対象 SHA・実行結果を引き渡し、変更を追加したら記録・コミット・再照合・検証をやり直す。無関係な誤りと通知は種類別に報告する
- Left to the implementer: なし
- Stop and hand back if: 記録を追加すると再照合が循環する、または独立 review に意味の不一致が残る

### S9: 履歴区分を外して現在の記録を検査する

- Purpose: 過去 entries を保持する契約を外し、全件の静的検査と比較元ごとの照合へ揃える
- Specification: docs/ir/core/change-record-format.md#REQ-core-268, docs/ir/core/change-records.md#REQ-core-249, docs/ir/core/change-records.md#REQ-core-253, docs/ir/core/changes-results.md#REQ-core-272, docs/ir/core/changes-results.md#REQ-core-273, docs/ir/core/config.md#REQ-core-019
- Prerequisites: 利用者採用済みの現在状態方針と今回文書 draft の読取り。追加承認は不要
- May change: 記録 parser・純粋照合・作業ツリー記録 walker と対応テスト、今回の仕様 draft の確定
- Done when: 11必須鍵を受け、旧 state を未知鍵で拒否する。全件の参照を検査し、異なる base は現在 coverage に使わない。明示された隠し記録配下を check/status が検査し、未指定の隠し配下と他の文書探索は従来通り除外する
- Shown by: test — EX-core-451・455 の旧 archived 前提テストを新契約へ更新する。EX455 の成功から失敗への変更理由は現在記録に参照検査免除が不要になったため。EX451 は別 base を coverage に使わない対照へ変更する。EX456・457・460 を RED→GREEN で観測し、state 欠落成功・state 存在失敗・隠し記録の形式/参照切れを check と status の双方で検査する
- Left to the implementer: 型・内部 helper と既存 glob の再利用。隠し配下の名指しは正規化した glob のパス成分で判断し、一般 walker は拡張しない
- Stop and hand back if: 開発中 version 1 以外の公開データの移行が必要、隠し配下を全面探索する必要がある

### S10: 固定記録と探索範囲へ導入を揃える

- Purpose: 現在の変更に必要な一式だけを保持し、通常探索から機械用記録を外す
- Specification: docs/ir/core/change-workflow.md#REQ-core-276, docs/ir/core/changes-inputs.md#REQ-core-265, docs/ir/core/changes-inputs.md#REQ-core-267, docs/ir/core/change-records.md#REQ-core-253
- Prerequisites: S9。自己フックを旧記録から新形式へ移行できる順序を宣言する
- May change: agent/skills の直接関連する手順・参照、docs/guides と確認後のガイド印、PROJECT.md、CHANGELOG.md、.kotowari/config.yaml、.ignore、docs/changes の削除と .kotowari/changes の現在記録
- Done when: commit.yaml は HEAD→整形後 index の現在照合だけへ毎回置換され、最終記録は branch base→target の両役で作り直す。最終時 commit.yaml を削除する。無効化した両最終記録を削除し再照合する。通常 rg 探索は記録を除外し、Git と機械検査は読める。.ignore 自体を照合対象に含める
- Shown by: test — EX458・459 を実 Git fixture で RED→GREEN。固定ファイル更新・削除後の現在比較と過去コミット再検証、次コード変更の change_stale、次関連 IR 変更の change_ir_stale、base 変更の EX460 を観測する。artifact — EX461・462 は独立 review が判断意味変更・rebase・cherry-pick・並行統合後の再照合手順を確認し、仕様文面一致のテストは作らない。check — rg の通常探索除外と明示読取可能な限界を確認する
- Left to the implementer: 既存 rg の .ignore を使う。README や過去の理由一覧を機械用記録ディレクトリへ増やさない。コアへ固定3ファイル名の制約を加えない
- Stop and hand back if: フック免除が必要、過去の Git 履歴を書換える必要、判断の意味を機械証明する必要がある

### S12: 現在記録で独立レビューと最終検証を更新する

- Purpose: 追加改訂を含めた同じ対象 snapshot に両役の現在記録と検証を揃える
- Specification: docs/ir/core/change-workflow.md#REQ-core-257, docs/ir/core/change-workflow.md#REQ-core-259, docs/ir/core/change-workflow.md#REQ-core-276, docs/ir/core/changes-results.md#REQ-core-274
- Prerequisites: S9、S10。実装とは別の review が今回の仕様・実装・スキルの照合を実施済み
- May change: .kotowari/changes/ の現在記録、今回発見の修正を既存 Scope 内へ委譲する箇所
- Done when: 過去データを蓄積せず両最終記録が全ブランチ差分を覆い、commit.yaml が存在せず、固定 SHA の check と changes の review 段階が終了0となる。対象の必要な要求・例のテスト対応が揃う
- Shown by: check — CARGO_BUILD_JOBS=4 cargo test --workspace、kotowari plan docs/plans/change-conformance.md、kotowari check、対象 ID の query、kotowari changes --base <完全SHA> --head <完全SHA> --phase review を固定対象で実行する。通常探索と機械検査の境界、変更path・ID・未確認事項を引き渡す。ファイル追加後は内容と対象 SHA を再検証する
- Left to the implementer: なし。独立 review の記録は review 側が作成する
- Stop and hand back if: 未確認の意味の不一致、自己照合の循環、作業ツリーに過去データを残す必要が出る
