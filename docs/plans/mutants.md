# 実装計画: mutants

## Goal

変異テストの道具の結果のファイルを読んで見逃しを指摘にするコマンド `kotowari mutants` が入り、このリポジトリでは push の前のフックが差分に入る変異を（タグの push では全体を）走らせて、見逃しが残る push を止めるようになる。

## Specification

正本は仕様 IR `docs/ir/*.md`（以下「IR」）。この計画が対象にする項目は次のとおり。

- 結果の読み取りと停止: `docs/ir/mutants-input.md#REQ-138`、`#REQ-144`、`#TBL-024`、具体例 EX-207、EX-208、EX-222〜EX-227
- 指摘と集計: `docs/ir/mutants.md#REQ-139`、`#REQ-140`、`#REQ-145`、`#REQ-146`、`#REQ-147`、`#REQ-150`（検証は review）、`#TBL-025`、`#PROP-005`、具体例 EX-204〜EX-206、EX-209、EX-210、EX-229〜EX-231
- 等価の一覧: `docs/ir/equivalents.md#REQ-141`、`#REQ-142`、`#REQ-143`、`#REQ-148`、具体例 EX-211〜EX-217、EX-220、EX-221、EX-232〜EX-239、EX-243
- コマンドと引数: `docs/ir/cli.md#REQ-001`、`#REQ-002`、`#REQ-004`、`#REQ-149`、`#TBL-001`、具体例 EX-218、EX-219、EX-240〜EX-242、EX-244
- 停止の文言と詳細: `docs/ir/cli-environment.md#TBL-018`、`#TBL-020`、`#REQ-109`（検証は review）
- 設定の鍵: `docs/ir/config.md#TBL-004`（"mutants.equivalents" の行）、`#REQ-014`（値の検査は既存のまま掛かる）、`#REQ-018`（"kotowari check" に限定）
- 指摘の形: `docs/ir/findings.md#REQ-031`、`#TBL-008`、`#TBL-009`、`docs/ir/finding-order.md#REQ-027`、`#TBL-007`、`#TBL-019`、`docs/ir/output.md#TBL-006`、`#TBL-005`、`#REQ-128`（後ろの2つは "kotowari check" に限定）
- 既存の要求で境界を決めているもの: `docs/ir/base-directory.md#REQ-110`（パスの正規化）、`docs/ir/ir-document.md#TBL-010`（行の数え方）、`docs/ir/cli.md#REQ-003`、`#REQ-005`、`#TBL-002`、`docs/ir/cli-environment.md#REQ-107`、`docs/ir/output.md#REQ-021`、`#REQ-022`、`#REQ-025`、`#REQ-026`、`#PROP-002`、`docs/ir/finding-order.md#PROP-003`、`docs/ir/skill-references.md#REQ-125`〜`#REQ-127`（REQ-126 は記録の A59 で「既定のある鍵」に改訂）
- 用語: `docs/ir/CONTEXT.md` の「変異」「変異の結果」「見逃し」「等価」「等価の一覧」

判断の記録は `docs/decision/records/2026-09-17-mutation-tests.md`（A1〜A60。以下「記録」）。IR に書かないと決めたもの（実行のスクリプト、フック、スキルの手順書）は、記録の A4、A19、A21〜A23、A25〜A29 がこの計画の根拠になる。

IR の読み方は前の計画と同じ。要求の `- 検証:` が `review` の要求はテストを求めない。IR の各文書の `## 具体例` のシナリオは、その要求の成功の条件と反例で、テストの入力と期待の元にする。

IR と記録は実装の間は読み取り専用である。IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

この計画の中のテストの置き方と名前、スクリプトの引数と置き場、フックの書き方は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約4,300行、テスト 470 件）を育てる。作り直さない。新しい依存は足さない。

層ごとの採否（探した順は、要るか → このコード → 標準ライブラリ → 環境 → 入っている依存 → 定番 → 数行 → 自作）:

- 結果のファイルの JSON の読み取り — 採用（入っている依存 `serde_json`）。出力で使っている
- 等価の一覧の YAML の読み取り — 採用（入っている依存 `serde-saphyr`）。設定ファイルで使っている
- パスの正規化 — 採用（このコード `normalize_path`）。REQ-110 の実装そのもの
- 結果のファイルと一覧の UTF-8 の読み取りと停止 — 採用（このコード `read_utf8_file`）
- 変異の結果か一覧の1件が指すソースの読み取り — 数行で書く。無い、読めない、UTF-8 でないのどれでも停止しない（REQ-141、REQ-142）ので、停止を返す `read_utf8_file` は使えない
- 引数の読み取り — 採用（このコード `parse_args`）。引数の解析の crate は以前に外している
- 指摘の並べ方と JSON / text の出力 — 採用（このコードの check の出力）
- 変異テストの実行 — 採用（入っている道具 cargo-mutants 27.1.0）。kotowari の外（スクリプト）から呼ぶ
- メモリの上限 — 採用（環境 `systemd-run --user`）
- フック — 採用（入っている道具 lefthook）
- 居残った子プロセスの見張り — 数行の bash で書く

作りの約束（記録の A12、A13、A54 から）:

- 道具に依存する部分は「結果のファイルを変異の結果に写す」1か所だけにし、道具ごとに1つの関数かモジュールに閉じる。変異の結果の型、指摘、集計、一覧との一致は、cargo-mutants の語（missed、CaughtMutant など）を知らない
- 一致と集計の判定は、読んだ文字列を引数で受ける関数にし、ファイルの読み取りは呼び出す側に寄せる（テストで入力を置きやすくするため）
- 等価の一覧の読み取りは1か所に閉じる

作業はブランチ `mutants`（main から切る。worktree を使うならリポジトリの中の `.claude/worktrees/` の下）で行う。main の上で直接は作業しない。差分の変異の基準は、フックと同じ `origin/main` に揃える（main の上だと差分が空になり、終端の確認が変異を1件も走らせずに通ってしまう）。

読む順はステップ 1（コマンドと引数）→ 2（結果の読み取りと停止）→ 3（指摘と集計）→ 4（等価の一覧）→ 5（スキルの references）→ 6（スクリプトとフック）→ 7（終端の確認）。2〜4 は 1 のコマンドの入口に、3 は 2 の変異の結果に、4 は 3 の指摘に、6 は 1〜4 のコマンドに依存する。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- ステップ1のテストは `tests/step1_cli.rs` に足す。ステップ2〜4のテストは新しい `tests/step9_mutants.rs` に置く。fixture の作り方は既存のテストに倣う（一時ディレクトリに `.kotowari/config.yaml`、結果のファイル、ソース、等価の一覧を置き、`assert_cmd` で CLI を起こす）
- シナリオの When が "kotowari mutants …" か "kotowari check" を実行する場面のテストは、CLI を起こし、Given の各行を入力に置き、Then の各行を assert する
- 各テスト関数の直前の行に `// @kotowari[REQ-139, EX-204]` の形の印を置く（`skills/kotowari/references/mark.md` の形）。確かめる具体例の ID も印に含める
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる。「指摘が出ない」「停止しない」テストは、実装を入れた後に逆の期待を一時的に置いて RED を見てから戻し、その RED の出力を終端報告に載せる
- 既存のテストが新しい仕様と食い違うときは、新しい仕様に合わせて書き直し、コミットの本文に改めた決定を書く。落ちなくても要求の文が変わったテストが2本ある: `tests/step1_cli.rs` の `req_001_only_check_subcommand`（REQ-001 はコマンド2つに改まった）と `tests/step5_findings.rs` の `req_031_only_two_kinds_are_notices`（REQ-031 は注意4つに改まった）。名前と assert を新しい文に合わせて書き直す（前者はステップ1、後者はステップ3）
- `tests/step7_skill_references.rs` の `req_127_findings_reference_stop_wordings_match_the_code` はステップ2で停止の文言を足した時点で、`req_125_findings_reference_kinds_match_the_code` はステップ3で種類を足した時点で落ちる。落とさないために、そのステップの中で `skills/kotowari/references/findings.md` の文言の表と種類の表だけを先に追従させる（残りの references はステップ5）
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語。帰属の行は付けない

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `skills/kotowari/SKILL.md`、`skills/kotowari/references/` の下（ステップ5）
- `scripts/mutants.sh`（新規。ステップ6）
- `lefthook.yml`（ステップ6）
- `.cargo/mutants.toml`（削除。ステップ6）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない。`docs/ir/`、`docs/decision/`、`docs/plans/`、`docs/trace.md`、`docs/spec/`、`.kotowari/`、`experiments/`、`fixtures/`、`.gitignore` は変えない。インストール済みスキル（`~/.claude/skills/kotowari/`）と `~/.cargo/bin/kotowari` には触らない。

## Step order and prerequisites

ステップ1 → 2 → 3 → 4 → 5 → 6 → 7 の順に行う。

## Step 1 — コマンドと引数

Purpose: "kotowari mutants" を2つ目のコマンドとして受け、引数の誤りを仕様のとおりに停止にする。Specification: `docs/ir/cli.md#REQ-001`、`#REQ-002`、`#REQ-004`、`#REQ-149`、`docs/ir/cli-environment.md#TBL-020`、`#REQ-107`。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の出力を記録する。この時点の終了コードは1で、"findings" は requirement_without_test と scenario_without_test だけのはず（この計画の対象の要求と具体例の分）。これを基準線にする。
May change: `src/lib.rs`（`Cli`、`parse_args`）、`src/main.rs`、`tests/step1_cli.rs`。
Done when: REQ-004 と REQ-149 の場面がすべて引数の誤りで停止し、詳細が TBL-020 のとおり。"mutants" の引数が正しいとき、結果のファイルのパスと道具の名前がコマンドの処理に渡る（この時点の処理の中身は、結果のファイルが無いときの停止だけでよい）。"--help" と "--version" は "mutants" と一緒でも REQ-107 のとおり。使い方の表示に "mutants" と "--tool" を載せる（IR の契約ではなくこの計画の約束。テストでは固定しない）。
Shown by: test — `req_149_mutants_without_tool_is_an_argument_error`（EX-218）、`req_149_unknown_tool_is_an_argument_error`（EX-240）、`req_149_two_result_paths_is_an_argument_error`（EX-242）、`req_004_no_arguments_names_both_commands`（EX-219）、`req_004_options_without_a_command_names_both_commands`（EX-241）、`req_004_tool_on_check_is_an_argument_error`、`req_002_mutants_options_can_come_before_the_command_and_after_the_path`（EX-244。ステップ3の後でないと終了コード0を観測できないので、このステップでは引数の誤りにならないことまでを見て、ステップ3で Then を足す）、`req_144_missing_result_file_is_an_unreadable_file`（正しい引数のとき、結果のファイルのパスが処理に渡ることの観測。無いパスを渡すと "unreadable file: " で停止する）。
Left to the implementer: `Cli` の型の形、使い方の表示の文面（IR に文面の契約は無い）。
Stop and hand back if: REQ-004 の列挙と REQ-149 の列挙のどちらにも当たらない引数の並びで、停止か続行かを決められない。

## Step 2 — 結果の読み取りと停止

Purpose: cargo-mutants の結果のファイルを、道具を知らない変異の結果に写し、写せない結果で停止する。Specification: `docs/ir/mutants-input.md#REQ-138`、`#REQ-144`、`#TBL-024`、`docs/ir/cli.md#TBL-001`、`docs/ir/cli-environment.md#TBL-018`、`#TBL-020`、`docs/ir/base-directory.md#REQ-110`。
Prerequisites: ステップ1。
May change: `src/` の下（新しいモジュール。`src/lib.rs` の停止の理由に「結果の誤り」）、`tests/step9_mutants.rs`、`skills/kotowari/references/findings.md`（停止の文言の表に "results error" の1行だけ）。
Done when: REQ-144 の停止の場面がすべて "results error: " で始まる停止になり、結果のファイルが無い、読めない、UTF-8 でないときは既存の理由で停止する。TBL-024 に無い鍵と、基準の実行が無い結果では停止しない。写した変異の結果のファイルは REQ-110 の正規化が掛かっている。
Shown by: test — `req_144_unknown_outcome_value_stops`（EX-207）、`req_144_failed_baseline_stops`（EX-208）、`req_144_broken_json_stops`（EX-222）、`req_144_wrong_key_type_stops`（EX-223）、`req_144_name_without_the_location_prefix_stops`（EX-224）、`req_144_line_zero_stops`（EX-225）、`req_144_path_outside_the_base_stops`（EX-226）、`req_144_results_without_baseline_and_with_unknown_keys_are_read`（EX-227。終了コード0と "caught" の観測はステップ3の後に足す）。
Left to the implementer: モジュールの名前と分け方（道具ごとの写しは1か所に閉じる、という約束の中で）、停止の詳細の誤りの説明の文面（TBL-020 は「相対パスと、誤りの説明」までを契約にしている）、JSON を型に落とすか値のまま辿るか。
Stop and hand back if: 実物の結果のファイルの鍵の形が TBL-024 と合わない。実物はこのステップの最初に1回作る: `CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true CARGO_BUILD_JOBS=4 cargo +nightly mutants -j 1 --no-config --file src/record_form.rs -o <リポジトリの外の一時ディレクトリ>`（2026-09-17 の実測で変異23件、44秒。同期で走らせる）。その `mutants.out/outcomes.json` の全件で、"name" の先頭が "file"、"span.start.line"、"span.start.column" から組み立てた前置きと一致することを確かめ、一致しない変異があれば止まって返す。実物のファイルはリポジトリに置かない（テストの fixture は TBL-024 の鍵だけを持つ手書きの JSON にする）。

## Step 3 — 指摘と集計

Purpose: 変異の結果から mutant_survived と mutant_timeout を出し、集計を JSON と文字で出す（等価の一覧は0件として）。Specification: `docs/ir/mutants.md#REQ-139`、`#REQ-140`、`#REQ-145`、`#REQ-146`、`#REQ-147`、`#TBL-025`、`#PROP-005`、`docs/ir/findings.md#REQ-031`、`#TBL-008`、`#TBL-009`、`docs/ir/finding-order.md#TBL-007`、`#TBL-019`、`docs/ir/output.md#TBL-006`、`#TBL-005`、`#REQ-128`、`docs/ir/config.md#REQ-018`。
Prerequisites: ステップ2。
May change: `src/` の下（`finding_kinds!` に4つの種類。重大度の判定に mutant_timeout と equivalent_stale）、`tests/step9_mutants.rs`、`tests/step1_cli.rs`（EX-244 の Then）、`tests/step5_findings.rs`（`req_031_only_two_kinds_are_notices` の書き直し。mutants の注意2種類も観測する）、`skills/kotowari/references/findings.md`（種類の表に4行だけ）。
Done when: REQ-139、REQ-140 の文のとおりに指摘が出て、JSON の最上位が TBL-025 の3つの鍵だけで、文字の出力の最後の1行が REQ-146 の形。終了コードは TBL-002 のとおり（見逃しがあれば1、時間切れだけなら0）。"ir" の指す先が無くても停止しない。
Shown by: test — `req_139_survived_mutant_is_an_error`（EX-204）、`req_139_caught_and_unviable_mutants_yield_nothing`（EX-205）、`req_140_timeout_is_a_notice_even_when_listed`（EX-206。一覧を置く部分はステップ4の後に足す）、`req_145_no_mutants_exits_zero_with_all_zero_counts`（EX-209）、`req_147_missing_ir_directory_does_not_stop_mutants`（EX-210）、`req_139_duplicate_mutants_yield_one_finding_each`（EX-229）、`req_146_summary_is_the_last_line_after_findings`（EX-230）、`req_147_unmarked_test_is_not_reported_and_json_has_three_keys`（EX-231）。
Left to the implementer: 集計の型の形、check と mutants の出力の共通化の仕方。
Stop and hand back if: "kotowari check" の JSON か文字の出力が、このステップの変更で変わる（TBL-005、REQ-128、REQ-025 は check の出力を変えていない）。

## Step 4 — 等価の一覧

Purpose: 設定の "mutants.equivalents" が指す一覧を読み、一致した見逃しを外し、一覧の1件への指摘を出す。Specification: `docs/ir/equivalents.md#REQ-141`、`#REQ-142`、`#REQ-143`、`#REQ-148`、`docs/ir/config.md#TBL-004`、`#REQ-014`、`docs/ir/cli-environment.md#TBL-020`、`docs/ir/ir-document.md#TBL-010`、`docs/ir/finding-order.md#REQ-027`。
Prerequisites: ステップ3。
May change: `src/config.rs`（鍵 "mutants.equivalents"）、`src/` の下、`tests/step9_mutants.rs`、`tests/step1_config.rs`（鍵の型と絶対パスの値の検査）。
Done when: REQ-141 の一致の定義のとおりに見逃しが "equivalent" に数えられて指摘から外れ、REQ-142 と REQ-143 の指摘が1件ごとに出る（"line" は null、detail は一覧に書かれたままの値）。REQ-148 の停止と続行の場面が文のとおりで、一覧が YAML として読めないか最上位が並びでないときの詳細は一覧のファイルの相対パスで始まる。"kotowari check" は一覧の指す先が無くても停止しない。
Shown by: test — `req_141_listed_survivor_is_counted_as_equivalent`（EX-211）、`req_141_moved_line_still_matches`（EX-212）、`req_141_rewritten_line_no_longer_matches_and_entry_goes_stale`（EX-213）、`req_141_line_beyond_the_file_does_not_match`（EX-214）、`req_143_blank_why_is_invalid_and_suppresses_nothing`（EX-215）、`req_143_class_other_than_equivalent_is_invalid`（EX-216）、`req_148_missing_list_file_stops`（EX-217）、`req_143_entry_without_file_has_an_empty_detail_prefix`（EX-220）、`req_148_check_ignores_a_missing_list_file`（EX-221）、`req_141_one_entry_matches_every_line_with_the_same_text`（EX-232）、`req_141_entry_for_another_file_does_not_match`（EX-233）、`req_141_path_spelling_tab_indent_and_crlf_still_match`（EX-234）、`req_141_non_utf8_source_does_not_stop`（EX-235）、`req_143_entry_outside_the_base_is_invalid`（EX-236）、`req_143_duplicate_invalid_entries_yield_one_finding_each`（EX-237）、`req_148_empty_list_file_is_zero_entries`（EX-238）、`req_148_list_that_is_not_a_sequence_stops_with_the_list_path`（EX-239）、`req_142_stale_entry_detail_keeps_the_written_path`（EX-243）。
Left to the implementer: YAML を型に落とすか値のまま辿るか（1件ごとに形の誤りを指摘にする必要があるので、ファイル全体を1つの型に落として失敗させる形は使えない）、一致の判定の関数の切り方。
Stop and hand back if: `serde-saphyr` で、注釈だけのファイルと0バイトのファイルを「空」として見分けられない。1件が鍵と値の組でないときや値が文字列でないときを、ファイル全体の読み取りの失敗と区別して1件ごとに扱えない。

## Step 5 — スキルの references の追従

Purpose: スキル kotowari が新しいコマンドと指摘と手順を伝え、references の値が本体と一致する。Specification: `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`。手順の中身の根拠は記録の A5、A17、A19、A21、A22、A25、A26、A28。
Prerequisites: ステップ4。
May change: `skills/kotowari/SKILL.md`、`skills/kotowari/references/findings.md`、`config.md`、`workflow.md`、新しい `skills/kotowari/references/mutants.md`、`tests/step7_skill_references.rs`（既存の3本のテストが新しい種類と文言と鍵で落ちるときの追従だけ）。
Done when: `findings.md` に4つの種類（重大度と detail）と停止の文言 "results error" があり（表の行はステップ2、3で追従済み。ここでは読み方の説明を足す）、`config.md` に鍵 "mutants.equivalents" の説明（既定は無い。setup の手順1の YAML のブロックには書かない。REQ-126 がそのブロックに求めるのは既定のある鍵だけ）があり、`mutants.md` に次が書いてある: `kotowari mutants` の使い方と出力の読み方、等価の一覧の形と一致の取り方、見逃しを調べる3分類（等価、未検査、欠陥の疑い）と「未検査は、要求がその変異を区別できるほど具体的かを先に確かめ、曖昧なら IR へ戻し、具体的ならテストを足す」、等価を見つけたら一覧に書く前にコードを単純にして変異を無くせないかを先に見ること、一覧に1件足す前に別の文脈の LLM にその変異を落とすテストを書かせ、落とせたらそのテストを採用し、落とせなかった試みを "why" に書くこと、一覧の変更は人の普段の差分のレビューに混ざること。`SKILL.md` の場面の表に mutants の行があり、`workflow.md` の cycle と implement の節に「push の前のフックが差分の変異を走らせる。見逃しはテスト側の指摘と同じく fixer か実装者が直す」がある。このステップで変えた reference の先頭の改訂日が 2026-09-17 以降。
Shown by: artifact — `skills/kotowari/SKILL.md` の場面の表、`skills/kotowari/references/mutants.md` の全体、`workflow.md` と `config.md` と `findings.md` の変えた節を、終端報告に見出しつきで引用する。前提として、既存の `req_125_findings_reference_kinds_match_the_code`、`req_126_config_reference_setup_yaml_parses_to_the_defaults`、`req_127_findings_reference_stop_wordings_match_the_code` が通ること（新しいテストは足さない。references の文面を固定するテストは、IR が契約にしていない文面を固定することになる）。
Left to the implementer: references の文面と節の切り方。
Stop and hand back if: 既存の3本のテストを通すために、references ではなくテストの突き合わせの仕方を変える必要がある。

## Step 6 — 実行のスクリプトとフック

Purpose: 変異テストの実行から `kotowari mutants` までを1コマンドにし、push の前のフックから呼ぶ。根拠は記録の A4、A19、A23、A25、A26、A27、A29 と、Context の実走の観測。IR の要求は無い（製品の振る舞いではない）。
Prerequisites: ステップ4。cargo-mutants 27.1.0、nightly の toolchain、`systemd-run --user` が入っていること（入っていなければ止まって返す）。
May change: `scripts/mutants.sh`（新規）、`lefthook.yml`、`.cargo/mutants.toml`（削除）。
Done when: 次のすべて。

- `scripts/mutants.sh diff <base>` は、`git diff <base>...HEAD` を一時ファイルに書き、cargo-mutants を `--in-diff` で走らせる。`scripts/mutants.sh full` は全体を走らせる。どちらも、`--` の後ろの引数をそのまま cargo-mutants に渡す（`scripts/mutants.sh diff origin/main -- --file src/config.rs` のように、対象のファイルを絞って10分に収まる単位に分けて走らせるため）。`<base>` が解決できないとき（`origin/main` が無いなど）は、その旨を標準エラーに出して失敗で終わる。フックの中で fetch はしない
- `scripts/mutants.sh plan` は、標準入力から git の pre-push の行（`<ローカルの参照> <ローカルの SHA> <リモートの参照> <リモートの SHA>`）を読み、何も走らせずに、選ぶ副コマンド（"full" か "diff origin/main"）を1行で標準出力に出す。`scripts/mutants.sh hook` は同じ判定をして、そのまま実行する。フックは `hook` を呼ぶ
- cargo-mutants の呼び方は `CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true CARGO_BUILD_JOBS=4 cargo +nightly mutants -j 1 --no-config -o .`（結果はリポジトリの直下の `mutants.out/outcomes.json` にできる。`mutants.out/` は `.gitignore` に入っている。走らせる前に前回の `mutants.out/` を消す。`kotowari mutants` にはカレントディレクトリからの相対パス `mutants.out/outcomes.json` を渡す）。`--in-place` は使わない。安定版の cargo だと更新時刻の判定で変異が再コンパイルされず、幻の見逃しが出る
- 実行は `systemd-run --user` のサービス（`MemoryMax=12G`、`MemorySwapMax=0`、`OOMPolicy=continue`）の中で同期に行い、終わるまで待つ。`OOMPolicy=continue` が無いと、上限に当たった時点でサービスごと止まる
- 走っている間、変異済みの実行ファイル（`/tmp/cargo-mutants-` で始まるディレクトリの下の `target/debug/kotowari`）のうち60秒以上生きているものを殺す見張りを回す。cargo-mutants はテストを20秒で打ち切るので、60秒生きている変異済みの実行ファイルは、打ち切られたテストが起こしたまま居残った子プロセスしか無い（1つずつは上限未満なので OOM の仕組みでは死なず、溜まると次の変異のコンパイルがメモリ待ちで止まる）。殺した件数をスクリプトの終わりに標準エラーへ1行で出す。見張りは `trap`（EXIT、INT、TERM）で必ず止める。`pgrep -f` のパターンは `^/tmp/cargo-mutants-` から始めて固定する（固定しないと見張り自身の bash に当たる）
- 走らせる前に `src/` を探し、`mutants::skip` があれば失敗で終わる
- cargo-mutants が結果のファイルを作らずに終了コード0で終わったとき（差分に Rust のソースが無いとき）は、終了コード0で終わる。結果のファイルが無く終了コードが0でないときは失敗で終わる
- 結果のファイルがあれば `CARGO_BUILD_JOBS=4 cargo run -q -- mutants --tool cargo-mutants --format text <結果のファイル>` を走らせ、その終了コードで終わる（cargo-mutants 自身の終了コードは、見逃しや時間切れで0以外になるので使わない）
- `lefthook.yml` の pre-push は、既存の cargo-test と kotowari-check の後に、直列でこのスクリプトを呼ぶ。push される参照にタグ（`refs/tags/` で始まるもの）が1つでもあれば `full`、無ければ `diff origin/main`。参照は git が pre-push フックの標準入力に渡す行から読む
- `.cargo/mutants.toml` を消す（記録の A60。スクリプトは `--no-config` で読まない。中の除外は信頼できない計測の頃の判断で、等価の判断の置き場は等価の一覧に移った）。これで `src/main.rs` も変異の対象に戻る。`src/main.rs` は薄いまま保ち、分岐は `src/lib.rs` の側の関数に寄せて CLI のテストで捕まえる

Shown by: check — この順に走らせ、出力を終端報告に載せる。(1) `bash -n scripts/mutants.sh`。(2) `scripts/mutants.sh diff HEAD`（差分が空）が終了コード0で終わる。2026-09-17 の実測で、cargo-mutants は空の差分でも文書だけの差分でも、結果のファイルを作らずに終了コード0で終わる。(2b) `printf 'refs/heads/main 1 refs/heads/main 2\n' | scripts/mutants.sh plan` が "diff origin/main" を、`printf 'refs/tags/v0.2.0 1 refs/tags/v0.2.0 2\n' | scripts/mutants.sh plan` が "full" を出す。(2c) `src/` のどれか1つの関数に `#[mutants::skip]` を一時的に置いて `scripts/mutants.sh diff HEAD` が失敗で終わることを見て、置いた行を戻す（戻した後 `git diff --stat` が空であることを見る）。(3) `scripts/mutants.sh diff origin/main` がこのブランチの差分の変異を走らせ、最後の行に "mutants: caught=" で始まる集計が出る（終了コードはステップ7で0にする。このステップでは1でもよい）。1コマンドの上限（10分）に収まらないときは、`-- --file <パス>` で対象を分けて順に走らせる。分けても収まらないファイルがあれば、背景に逃がさず、止まって返す。(4) `lefthook run pre-push` は走らせない（cargo test と重なって長い）。代わりに `lefthook validate` が通ること。`full` は走らせない（十数分以上かかる。主セッションがマージの前に切り離して走らせる）。
Left to the implementer: スクリプトの中の関数の切り方、一時ディレクトリの作り方と消し方、見張りの実装（上の約束の中で）、lefthook で標準入力を受ける書き方（`use_stdin` など。lefthook 2.1.12 の文書で確かめる）と直列にする書き方。
Stop and hand back if: lefthook の pre-push で、push される参照の行をスクリプトに渡せない。`systemd-run --user` がこの環境で同期に待てない。見張りが変異済みでないプロセスに当たる。

## Step 7 — 終端の確認

Purpose: 全体が仕様どおりで、このリポジトリで check が 0 件、このブランチの差分の見逃しが 0 件であることを確かめる。Specification: 上の全部。
Prerequisites: ステップ1〜6。
May change: `tests/` の下（見逃しを捕まえるテストの追加）、`src/` の下（等価を見つけたとき、コードを単純にして変異を無くす直し）。等価の一覧のファイルと設定の鍵は足さない（足す必要があると判断したら、その変異の一覧と理由を添えて止まって返す。一覧に書くには別の文脈の反証が要り、実装者は自分の判定を自分で通せない）。
Done when: 作業ブランチの差分に Rust のソースが含まれていて、`scripts/mutants.sh diff origin/main` の集計の5つの値の合計が1以上で（変異が1件も走らずに通ったのではないこと）、`CARGO_BUILD_JOBS=4 cargo test` が全件通り、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の終了コードが0で "findings" が空で、`scripts/mutants.sh diff origin/main` の終了コードが0（集計の "survived" が0）。見逃しにテストを足すときは、先にその変異が壊す振る舞いを IR の要求か具体例で言えることを確かめる。言えないとき（要求が曖昧で区別できないとき）はテストを足さず、その変異と理由を添えて止まって返す。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test`、`CARGO_BUILD_JOBS=4 cargo run -q -- check`（終了コード0）、`CARGO_BUILD_JOBS=4 cargo run -q -- check --format text`（何も出ない）、`scripts/mutants.sh diff origin/main`（終了コード0。最後の行の集計を終端報告に載せる。対象を `-- --file` で分けて走らせたときは、分けた全部の集計を載せ、差分に入るソースのファイルを漏れなく回したことを `git diff --stat origin/main...HEAD -- src` と並べて示す）。
Left to the implementer: なし。
Stop and hand back if: 種類を問わず、ステップ1の基準線に無い指摘が残る。差分の見逃しのうち、IR の要求でも具体例でも壊れる振る舞いを言えないものがある。ファイルごとに分けても10分で終わらない対象がある。`src/main.rs` の見逃しを、分岐を `src/lib.rs` に寄せても捕まえられない。

## Verification map

| 仕様の項目 | 証拠を出すステップ |
|---|---|
| cli.md REQ-001、REQ-002、REQ-004、REQ-149、EX-218、EX-219、EX-240〜EX-242、EX-244 | ステップ1（EX-244 の Then はステップ3） |
| mutants-input.md REQ-138、REQ-144、TBL-024、EX-207、EX-208、EX-222〜EX-227、cli.md TBL-001、cli-environment.md TBL-018、TBL-020 の結果の誤り | ステップ2 |
| mutants.md REQ-139、REQ-140、REQ-145、REQ-146、REQ-147、TBL-025、PROP-005、EX-204〜EX-206、EX-209、EX-210、EX-229〜EX-231、findings.md REQ-031、TBL-008、TBL-009、finding-order.md TBL-019、output.md TBL-006、TBL-005 と REQ-128 の check への限定（EX-231）、config.md REQ-018 | ステップ3 |
| equivalents.md REQ-141〜REQ-143、REQ-148、EX-211〜EX-217、EX-220、EX-221、EX-232〜EX-239、EX-243、config.md TBL-004、finding-order.md REQ-027、cli-environment.md TBL-020 の等価の一覧の誤り | ステップ4 |
| skill-references.md REQ-125〜REQ-127 | ステップ5 |
| mutants.md REQ-150、cli-environment.md REQ-109（どちらも検証は review） | cycle のレビュー |
| 記録の A4、A19、A23、A25〜A27、A29（スクリプトとフック） | ステップ6、7 |
| このリポジトリの check 0 件と差分の見逃し 0 件 | ステップ7 |

## Left to the implementer

各ステップの欄のとおり。共通: Rust の型・関数・モジュールの名前、fixture の文面。

## Stop conditions

一般の4条件に加えて、各ステップの「Stop and hand back if」。IR・記録を変えたくなったら、変えずに止まって返す。変異テストを含め、長いコマンドを背景に逃がして応答を終えない（応答で終わる実行の形では、背景の処理はそこで殺される）。

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。check の確認は `CARGO_BUILD_JOBS=4 cargo run -q -- check`（JSON）と `--format text`。差分の変異は `scripts/mutants.sh diff origin/main`。

## Out of scope

- 全体の実走（`scripts/mutants.sh full`）の基準値の計測と、そこで出る既存のコードの見逃しの調べ（主セッションがマージの前に切り離して走らせ、結果を人に見せる。既存のコードの見逃しが残る間、タグの push はフックで止まる）
- 等価の一覧のファイルの新設と設定の鍵の追加（全体の実走の結果を調べてから、主セッションが反証の手順を踏んで行う）
- インストール済みスキル `~/.claude/skills/kotowari/` と `~/.cargo/bin/kotowari` への同期（マージ後に主セッションが行う）
- IR、記録の変更。具体例の文言の直し（観測できない場面は止まって返す）
- cargo-mutants 以外の道具の写し方
- フックを無視する push（`--no-verify`）への対処（記録の A29）
