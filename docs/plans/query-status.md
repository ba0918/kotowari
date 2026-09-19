# 実装計画: query-status

## Goal

`kotowari query <id>` が1件の項目かシナリオを本文と逆引き付きで出し、`kotowari status` が IR の統計と complete の真偽を出して終了コードで揃っているかを伝え、検証が review の要求に `- 確かめ方:` が無ければ check が誤りにするようになる。

## Specification

正本は仕様 IR `docs/ir/*.md`（以下「IR」）。この計画が対象にする項目は次のとおり。

- query: `docs/ir/query.md#REQ-156`、`#REQ-157`、`#REQ-158`、`#REQ-159`、`#REQ-160`、`#REQ-161`、`#TBL-027`、具体例 EX-250〜EX-257
- status: `docs/ir/status.md#REQ-162`、`#REQ-163`、`#REQ-164`、`#REQ-165`、`#REQ-166`、`#TBL-028`、具体例 EX-258〜EX-263
- 確かめ方の必須化: `docs/ir/ir-missing.md#REQ-098`、`docs/ir/findings.md#TBL-008`（missing_field の行）、具体例 EX-259
- コマンドと引数: `docs/ir/cli.md#REQ-001`、`#REQ-002`、`#REQ-004`、`#REQ-008`、`#REQ-005`、`#TBL-002`、具体例 EX-219、EX-241
- 停止の文言と詳細: `docs/ir/cli-environment.md#TBL-018`、`#TBL-020`、`#REQ-107`
- 土台になる list の形: `docs/ir/list.md#REQ-154`、`#REQ-155`、`#TBL-026`（query の1件は TBL-026 の鍵をすべて持つ。list の振る舞いは変えない）
- 逆引きと集計が依存する既存の判定: `docs/ir/ir-references.md#REQ-054`（"text" の ID の見つけ方はこれと同じ）、`#REQ-124`（ID の形）、`docs/ir/coverage.md#REQ-085`（with_tests の判定）、`#REQ-137`、`docs/ir/findings.md#REQ-032`（同じ ID の1つ目）、`docs/ir/output.md#TBL-005`、`#TBL-021`、`#REQ-128`（check の形は変えない）
- スキルの references: `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`（既存の一致が保たれること）
- 用語: `docs/ir/CONTEXT.md` の「項目」「シナリオ」「印」「テスト」「問題の記録」「指摘」「誤り」「停止」

判断の記録は `docs/decision/records/2026-09-20-query-status.md`（A1〜A20、R1〜R3、U1。以下「記録」）。IR に書かないと決めたこと（REQ-150 に確かめ方の行を足すのは cycle で行う）は記録の A10 が根拠になる。

IR の読み方は前の計画と同じ。要求の `- 検証:` が `review` の要求はテストを求めない。各文書の `## 具体例` のシナリオは、その要求の成功の条件と反例で、テストの入力と期待の元にする。

IR と記録は実装の間は読み取り専用である。例外はステップ2で `docs/ir/mutants.md` の REQ-150 に `- 確かめ方:` の行を1行足すことだけ（記録の A10。文面はステップ2に書いてある）。それ以外の IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

この計画の中のテストの置き方と名前、モジュールの切り方の約束は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約 5,300 行、テスト 11 ファイル 541 件）を育てる。作り直さない。新しい依存は足さない。

層ごとの採否（探した順は、要るか → このコード → 標準ライブラリ → 環境 → 入っている依存 → 定番 → 数行 → 自作）:

- 設定・IR・テストの読み取りと検査 — 採用（このコード `load_all`。`Loaded` に文書、指摘、集計、印がすべてある）
- list の1件の組み立て — 採用（このコード `list::build`）。query の1件は list の1件に `body` と `referenced_by` を足したものなので、list の型を包むか拡張する
- ID の参照の検出（"- 定義:"、"- 関係:"、"@about"、文とステップの中のバッククォートの ID）— 採用（このコード `ir::check_references` と `check_backtick_ids` が REQ-054 のために同じ場所を読んでいる）。逆引きはこれと同じ判定で ID を拾う。二重引用符の外のバッククォートの中だけが "text" の参照で、地の文の ID は参照ではない（REQ-054 と同じ）
- テストのある要求の判定 — 採用（このコード `tests_discovery::discover_and_check` の REQ-085 の判定と `collect_scenarios`）。検証が unit / property / proof の要求に限れば、status の with_tests は requirement_without_test を出さない要求の数と同じ値になるはず
- 引数の読み取り — 採用（このコード `parse_args`）
- JSON の出力 — 採用（入っている依存 `serde_json`）
- 本文の切り出しと集計 — 数十行で書く（新しいモジュール）

作りの約束（記録の A4、A9、A19、A20 から）:

- query と status は `load_all` を通し、2つ目の読み取りの経路を作らない。check と list の JSON と文字の出力は1バイトも変わらない
- query は指摘を計算しても捨て、読めれば終了コード 0。status は指摘を捨てるが、その数（severity ごと）と complete の判定に使う
- 逆引きの `via` の値と検出の場所は、REQ-054 が列挙する5つの場所のうち`印`を除く4つと同じ。ID の参照を新しく解析しない
- status の数は check の同じ鍵と同じ値（documents、tests.files）、または check の指摘と同じ判定（with_tests は REQ-085、findings は severity）で作る。status 独自の判定規則を持たない（記録の A9）
- 「テストがある要求」の判定は REQ-085 と同じ。検証が unit / property / proof の要求に限れば、with_tests の数は requirement_without_test が出ない要求の数と一致する（"- 検証:" の行の無い要求は requirement_without_test の対象外だが、TBL-028 の with_tests / without_tests には「review でない要求」として入り、印の有無で数える）。両者が食い違ったら status の側が間違っている

作業はブランチ `query-status`（main から切る。worktree を使うならリポジトリの中の `.claude/worktrees/` の下）で行う。main の上で直接は作業しない。差分の変異の基準は `main`（origin/main は未 push の分だけ古い）。

読む順はステップ 1（コマンドと引数）→ 2（確かめ方の必須化）→ 3（query の組み立てと JSON）→ 4（query の text）→ 5（status の集計と JSON）→ 6（status の text）→ 7（スキルの reference）→ 8（終端の確認）。3 と 5 は 1 のコマンドの入口に、5 は 2 の指摘（EX-259 は確かめ方の無い要求で complete が false になることを見る）に、4 は 3 に、6 は 5 に依存する。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- ステップ1のテストは `tests/step1_cli.rs` に足す。ステップ2のテストは `tests/step2_ir.rs` に足す。ステップ3、4のテストは新しい `tests/step11_query.rs`、ステップ5、6のテストは新しい `tests/step12_status.rs` に置く。fixture の作り方は `tests/step10_list.rs` に倣う（一時ディレクトリに `.kotowari/config.yaml`、IR の文書、テストのファイルを置き、`assert_cmd` で CLI を起こす）
- シナリオの When が "kotowari query …" か "kotowari status …" を実行する場面のテストは、CLI を起こし、Given の各行を入力に置き、Then の各行を assert する
- 各テスト関数の直前の行に `// @kotowari[REQ-156, EX-250]` の形の印を置く（`skills/kotowari/references/mark.md` の形）。確かめる具体例の ID も印に含める
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる。「指摘が出ない」「停止しない」テストは、実装を入れた後に逆の期待を一時的に置いて RED を見てから戻し、その RED の出力を終端報告に載せる
- 既存のテストが新しい仕様と食い違うときは、新しい仕様に合わせて書き直し、コミットの本文に改めた決定を書く。落ちるものが3本、落ちないが書き直すものが1本ある: `tests/step1_cli.rs` の `req_004_no_arguments_names_all_three_commands`（129 行目）と `req_004_options_without_a_command_names_all_three_commands`（142 行目）は停止の文言が5つのコマンドに改まって落ちる（EX-219、EX-241）。`req_008_render_trace_query_are_argument_errors`（613 行目）は "query" がコマンドになるので落ちる（REQ-008 は render だけに改まった。"trace" はコマンドでないので今までどおり引数の誤りだが、REQ-008 の対象ではないので assert から外す）。`req_001_check_list_and_mutants_are_the_only_commands`（16 行目）は落ちないが REQ-001 がコマンド5つに改まったので、名前と assert を新しい文に合わせて書き直す（すべてステップ1）
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語。帰属の行は付けない

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `docs/ir/mutants.md`（REQ-150 の見出しの下に `- 確かめ方:` の行を1行足すことだけ。ステップ2）
- `skills/kotowari/SKILL.md`（場面の表の check の行と、冒頭の説明文だけ）、`skills/kotowari/references/findings.md`（ステップ7）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない。上に挙げた行以外の `docs/ir/`、`docs/decision/`、`docs/plans/`、`docs/spec/`、`.kotowari/`、`experiments/`、`fixtures/`、`scripts/`、`lefthook.yml`、`.gitignore` は変えない。インストール済みスキル（`~/.claude/skills/kotowari/`）と `~/.cargo/bin/kotowari` には触らない。

## Step order and prerequisites

ステップ1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 の順に行う。

## Step 1 — コマンドと引数

Purpose: "kotowari query" と "kotowari status" を4つ目と5つ目のコマンドとして受け、引数の誤りと停止の文言を仕様のとおりにし、REQ-008 を render だけにする。Specification: `docs/ir/cli.md#REQ-001`、`#REQ-002`、`#REQ-004`、`#REQ-008`、`docs/ir/query.md#REQ-157`（位置引数の数と形。ID の有無はステップ3）、`#REQ-158`、`docs/ir/status.md#REQ-163`、`docs/ir/cli-environment.md#TBL-020`、`#REQ-107`、`docs/ir/ir-references.md#REQ-124`。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check --format text` の出力を記録する。この時点の終了コードは 1 で、指摘は requirement_without_test 11 件（REQ-156〜REQ-166）と scenario_without_test 11 件（EX-250〜EX-263 のうち検証が unit の要求を about に持つもの）のはず。これを基準線にする（2026-09-20 に主セッションが実測した値。食い違えば実測を基準線にし、終端報告に書く）。
May change: `src/lib.rs`（`Cli`、`parse_args`、`run`、`print_help`）、`tests/step1_cli.rs`。
Done when: "query" と "status" が1つ目の位置引数として通り、`Cli` に両方の変種がある。コマンド無しと、オプションだけの停止の文言が "expected command: check, list, mutants, query or status"。"query" の位置引数が0個か2個以上、または ID の形（REQ-124）でないとき、"status" の後の位置引数、"query" か "status" に付けた "--tool" は引数の誤りで停止する（テストは終了コード 2 と標準エラーの1行目が "argument error: " で始まることまでを見る）。"render" は引数の誤りのまま（REQ-008）。両コマンドのオプションはコマンドの前後どちらでも受ける。"--help" と "--version" は両コマンドと一緒でも REQ-107 のとおり。使い方の表示に両コマンドを載せる（IR の契約ではなくこの計画の約束。テストでは固定しない）。正しい引数の "kotowari query" と "kotowari status" は、この時点では `load_all` を通してから仮の JSON（query は `{"items":[]}`、status は `{}`）を出して終了コード 0 で終わる（ステップ3、5までの仮の形。読み取りを通すのは、停止が check と同じであることをこのステップで観測するため）。
Shown by: test — `req_001_five_commands_only`（既存の書き直し）、`req_004_no_arguments_names_all_five_commands`（EX-219。既存の書き直し）、`req_004_options_without_a_command_names_all_five_commands`（EX-241。既存の書き直し）、`req_008_render_is_an_argument_error`（既存の書き直し。"render" 単独。"check render" は REQ-004 の場面なので、そこを見る assert は `req_004_positional_after_status_stops` と同じ形で REQ-004 の側に置く）、`req_157_two_positional_arguments_stop`（EX-252）、`req_157_query_with_no_positional_stops`、`req_157_positional_that_is_not_an_id_stops`、`req_004_positional_after_status_stops`、`req_004_tool_on_query_or_status_stops`、`req_002_query_and_status_options_before_or_after_the_command`（この2本は2つのコマンドを1本に畳む。REQ-002 と REQ-004 がコマンドを列挙で1つの文にしているのに合わせる。既存の `req_008_…` と同じ畳み方）、`req_158_unreadable_config_stops_like_check`（EX-256。"kotowari check" と "kotowari query REQ-001" を両方起こし、終了コード 2 と標準エラーの1行目が同じ文字であることを見る）、`req_163_unreadable_config_stops_like_check`（EX-262。同じ形で status）。
Left to the implementer: `Cli` の型の形、使い方の表示の文面、仮の JSON の出し方。
Stop and hand back if: REQ-002、REQ-004、REQ-157 のどれにも当たらない引数の並びで、停止か続行かを決められない。

## Step 2 — 確かめ方の必須化

Purpose: 検証が review の要求に "- 確かめ方:" の行が無いとき、check が detail "確かめ方" の missing_field を出す。あわせて REQ-150 に行を足し、このリポジトリの check を 0 件に保つ。Specification: `docs/ir/ir-missing.md#REQ-098`、`docs/ir/findings.md#TBL-008`（missing_field の行）、具体例 EX-259 の And の行。
Prerequisites: なし（順はステップ1の後）。
May change: `src/ir.rs`（要求の必須の行の検査）、`tests/step2_ir.rs`、`docs/ir/mutants.md`（REQ-150 の `- 検証: review` の行の直後に次の1行を足す。文字はこのとおり: `- 確かめ方: 指摘の種類名（mutant_survived、mutant_timeout、equivalent_stale、equivalent_invalid）と detail を組み立てる src/mutants.rs と src/lib.rs に、道具の結果の値の綴り（CaughtMutant、MissedMutant）が現れないことを rg -n 'CaughtMutant|MissedMutant' src/mutants.rs src/lib.rs が何も出さないことで確認。道具の値を写す src/cargo_mutants.rs は対象外`。2026-09-20 時点でこの rg は何も出さない）。
Done when: 検証が review で "- 確かめ方:" の行が無い要求に、その要求の見出しの行を指す detail "確かめ方" の missing_field が1件出る。行があるとき、値が空のとき（REQ-098 のとおり無い行として扱うので missing_field が出る）、検証が review でないときの振る舞いは仕様のとおり。このリポジトリの `kotowari check` に missing_field が出ない（REQ-150 の行を足した後。`CARGO_BUILD_JOBS=4 cargo run -q -- check --format text` の指摘が基準線と同じであることを見る）。
Shown by: test — `req_098_review_requirement_without_how_to_verify_is_a_missing_field`（行の無い review の要求と、行のある review の要求と、行の無い unit の要求を1つの fixture に置き、missing_field が review で行の無い1件だけに出ることを見る）。
Left to the implementer: 検査を置く関数（既存の必須の行の検査 `REQ-098` の隣）。
Stop and hand back if: missing_field の "line" の決まり（TBL-019）が見出しの行でない。

## Step 3 — query の組み立てと JSON

Purpose: 位置引数の ID を持つ項目とシナリオを list の1件の形に `body` と `referenced_by` を足して出し、無ければ停止する。Specification: `docs/ir/query.md#REQ-156`、`#REQ-157`（ID の有無）、`#REQ-159`、`#REQ-160`、`#REQ-161`（JSON の部分）、`#TBL-027`、`docs/ir/cli-environment.md#TBL-020`（"unknown id: "）、`docs/ir/ir-references.md#REQ-054`、`docs/ir/list.md#REQ-154`、`#TBL-026`。
Prerequisites: ステップ1。
May change: `src/` の下（新しいモジュール。`src/lib.rs` の query の入口。`src/list.rs` は query が使う型を公開する変更まで。`src/ir.rs` は逆引きの検出のために `check_references` と `check_backtick_ids` の ID の走査を「参照の一覧を返す関数」として切り出す変更まで。check の指摘は変えない）、`tests/step11_query.rs`（新規）。
Done when: 次のすべて。

- "items" は位置引数と同じ ID を持つ項目とシナリオの並びで、1件は TBL-026 のすべての鍵と `body`、`referenced_by` を持つ。同じ ID が複数あれば全部出て、順は REQ-154 と同じ
- `body` は TBL-027 の境界のとおり（項目は見出しの次の行から次の "### " か "## " の前まで、シナリオは "@id" のタグの行から最後のステップまで、先頭と末尾の空行を落とし、行の文字はそのまま）
- `referenced_by` の1件は id、kind、path、line、via を持ち、via は "definition"、"relations"、"about"、"text" のいずれか。"line" は TBL-026 と同じ意味で、指している側の項目の見出しの行、シナリオなら "Scenario:" の行（参照が書かれた行ではない）。"text" は REQ-054 と同じ判定（二重引用符の外のバッククォートの中の ID）で、地の文の ID は拾わない。並びは path → line
- 位置引数と同じ ID を持つものが無ければ "argument error: unknown id: <位置引数>" で停止する
- IR の文書に誤りがあっても読めた項目は出て終了コードは 0。指摘は標準出力にも標準エラーにも出ない
- check と list の JSON と文字の出力は、このステップの前後で変わらない

fixture の約束: body は先頭と末尾の空行を落とすので（TBL-027）、EX-250 と EX-254 の fixture は実物の IR と同じく見出しの直後に空行を置いてよく、body は "- 種類:" の行から始まる。EX-257 の fixture では、Given のとおり REQ-002 の文の中の TBL-001 をバッククォートで囲む。
Shown by: test — `req_156_item_has_body_and_referenced_by`（EX-250）、`req_157_unknown_id_stops`（EX-251）、`req_156_duplicate_ids_are_all_listed`（EX-253）、`tbl_027_scenario_body_starts_at_the_tag_line`（EX-255）、`tbl_027_definition_and_text_references_are_listed`（EX-257）、`tbl_027_text_reference_is_only_the_backticked_id`（バッククォート無しの ID と二重引用符の中の ID が via "text" に出ないこと）、`req_160_referenced_by_is_ordered_by_path_then_line`、`req_161_json_top_level_has_only_items`、`req_156_items_are_listed_despite_ir_errors_and_exit_zero`（"- 検証:" の無い要求を query して終了コード 0 で "findings" が無いこと）。
Left to the implementer: モジュールの名前と切り方、query の1件の型（list の型を包むか、鍵を平らに持つか。JSON の鍵の集合が TBL-027 のとおりならどちらでもよい）、参照の一覧を返す関数の形。
Stop and hand back if: check の JSON か文字の出力が変わる。`referenced_by` の "line" を、指している側の見出しの行（シナリオは "Scenario:" の行）にすると具体例が通らない。

## Step 4 — query の text

Purpose: "--format text" で list と同じ1行目とテストの行に続けて本文と逆引きの行を出す。Specification: `docs/ir/query.md#REQ-161`（text の部分）、`docs/ir/list.md#REQ-155`。
Prerequisites: ステップ3。
May change: `src/` の下（query の出力の関数）、`tests/step11_query.rs`。
Done when: 1件ごとに、list の text と同じ1行目と "tests" の行、次に `body` の各行を2つの半角空白で字下げした行、最後に `referenced_by` の1件ごとの "  <- ID via パス:行" の行。固定の文言（"<-"、via の値）は英語。
Shown by: test — `req_161_text_prints_body_and_referenced_by_lines`（EX-254。1行目、2行目、3行目、最後の行を比べる）。
Left to the implementer: 整形の関数の切り方。
Stop and hand back if: EX-254 の行を REQ-161 の文から組み立てた出力と1バイトでも違う（文と具体例が食い違っている）。

## Step 5 — status の集計と JSON

Purpose: check と同じ検査を走らせて TBL-028 の数と complete を JSON で出し、終了コードで伝える。Specification: `docs/ir/status.md#REQ-162`、`#REQ-164`、`#REQ-165`、`#REQ-166`（JSON の部分）、`#TBL-028`、`docs/ir/coverage.md#REQ-085`、`#REQ-137`、`docs/ir/findings.md#REQ-032`、`docs/ir/output.md#TBL-021`、`docs/ir/ir-missing.md#REQ-098`。
Prerequisites: ステップ1、2。
May change: `src/` の下（新しいモジュール。`src/lib.rs` の status の入口。`src/tests_discovery.rs` は REQ-085 の判定を status から再利用できる形にする変更まで。check の指摘は変えない）、`tests/step12_status.rs`（新規）。
Done when: 次のすべて。

- JSON の最上位は TBL-028 の群の鍵だけ（documents、items、requirements、scenarios、tests、findings、complete）で、各群の鍵と値は TBL-028 のとおり
- with_tests の「テストがある」は REQ-085 の判定と同じ（ID を含む印か、"@about" にその ID を持つシナリオの ID を含む印。同じ ID のシナリオが複数なら REQ-032 の1つ目）。検証が review の要求は with_tests にも without_tests にも入らない。"- 検証:" の行の無い要求は review でないので with_tests か without_tests に入り、unit / property / proof / review のどれにも数えない（TBL-028 の字面どおり）
- tests.marks は印の出現を ID ごとに1つと数える（`Loaded.markers` の件数と同じ）
- findings.error と findings.notice は check の指摘の severity ごとの数。complete は誤りが 0 かつ FLAG が 0 のときだけ true
- 終了コードは complete なら 0、そうでなければ 1
- 指摘は標準出力にも標準エラーにも出ない。check と list の出力は変わらない

Shown by: test — `req_162_complete_project_counts_and_exit_zero`（EX-258）、`req_165_review_requirement_without_how_to_verify_is_not_complete`（EX-259）、`req_165_flag_makes_it_not_complete`（EX-260）、`tbl_028_requirement_covered_through_a_scenario_counts_as_with_tests`（EX-263）、`tbl_028_review_requirements_are_not_in_with_or_without_tests`、`tbl_028_requirement_without_a_verification_line_is_counted_by_marks_only`（by_verification のどれにも入らず、with_tests か without_tests には入る）、`tbl_028_items_are_counted_by_kind`（5種類の数）、`tbl_028_documents_and_test_files_match_check`（同じ fixture の check の files、lines、tests と同じ値）、`req_166_json_top_level_has_only_the_groups`、`req_162_status_writes_no_finding`（誤りのある IR で終了コードが 1、標準出力の "findings" は "error" と "notice" の2つの鍵だけを持ち、指摘の並び（"kind" や "detail" を持つ要素）がどこにも無く、標準エラーが空であること）。
Left to the implementer: モジュールの名前、集計の型の形、REQ-085 の判定を共有する形（関数を切り出すか、check の指摘の集合から数えるか）。
Stop and hand back if: 検証が unit / property / proof の要求に限った with_tests の数が、同じ入力の check で requirement_without_test が出ない要求の数と一致しない。check の出力が変わる。

## Step 6 — status の text

Purpose: "--format text" で群ごとに1行を出す。Specification: `docs/ir/status.md#REQ-166`（text の部分）、`#TBL-028`。
Prerequisites: ステップ5。
May change: `src/` の下（status の出力の関数）、`tests/step12_status.rs`。
Done when: TBL-028 の表の順に、群ごとに "群名 鍵=値 鍵=値" の1行。鍵は JSON と同じ語、値の間は半角空白1つ、桁揃え無し。tests の行は "marks=数" に続けて拡張子ごとの "拡張子=数"。最後の行は "complete true" か "complete false"。
Shown by: test — `req_166_text_prints_one_line_per_group`（EX-261。1行目の始まり、requirements の行の中身、最後の行を比べる）。
Left to the implementer: 整形の関数の切り方。
Stop and hand back if: EX-261 の行を REQ-166 と TBL-028 から組み立てた出力と1バイトでも違う。

## Step 7 — スキルの reference の追従

Purpose: スキル kotowari が query と status の使い方と出力の読み方を伝え、references の値が本体と一致したままである。Specification: `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`。節の中身の根拠は記録の A2、A3、A7、A8、A9、A10、A11、A14。
Prerequisites: ステップ6。
May change: `skills/kotowari/references/findings.md`、`skills/kotowari/SKILL.md`（場面 check の「いつ」の欄に `kotowari query` と `kotowari status` の出力を読むことを足し、冒頭の説明文に読み取りのコマンドを足す。新しい場面と発火語は足さない。list のときと同じ扱い）。
Done when: `findings.md` に query と status の短い段落があり、次が書いてある: query は list の1件に `body`（本文の生の行）と `referenced_by`（via の4つの値の意味）を足したもので、無い ID は "argument error: unknown id: "。status は群ごとの数と complete で、complete は check の誤り 0 かつ FLAG 0、終了コードは complete なら 0 でなければ 1。検証が review の要求は `- 確かめ方:` が必須で、無ければ missing_field（detail "確かめ方"）。`findings.md` の先頭の改訂日を実装の日付にする（同じ日なら変わらない）。`SKILL.md` の場面 check の行が両コマンドの出力を読む場面を含み、冒頭の説明文（check と mutants を挙げている文）に list、query、status が読み取りのコマンドとして載る。既存の `req_125_findings_reference_kinds_match_the_code`、`req_126_config_reference_setup_yaml_parses_to_the_defaults`、`req_127_findings_reference_stop_wordings_match_the_code` が通る。
Shown by: artifact — `skills/kotowari/references/findings.md` の足した段落と `skills/kotowari/SKILL.md` の変えた行を、終端報告に引用する。新しいテストは足さない。
Left to the implementer: 段落の文面と置く位置。
Stop and hand back if: 既存の3本のテストを通すために、references ではなくテストの突き合わせの仕方を変える必要がある。

## Step 8 — 終端の確認

Purpose: 全体が仕様どおりで、このリポジトリで check が 0 件、status が complete、このブランチの差分の見逃しが 0 件であることを確かめる。Specification: 上の全部。
Prerequisites: ステップ1〜7。
May change: `tests/` の下（見逃しを捕まえるテストの追加）、`src/` の下（等価を見つけたとき、コードを単純にして変異を無くす直し）。等価の一覧（`.kotowari/equivalents.yaml`）には足さない（足す必要があると判断したら、その変異と理由を添えて止まって返す）。
Done when: `CARGO_BUILD_JOBS=4 cargo test` が全件通り、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の終了コードが 0 で "findings" が空で、`CARGO_BUILD_JOBS=4 cargo run -q -- status` の終了コードが 0 で "complete" が true で、`CARGO_BUILD_JOBS=4 cargo run -q -- query REQ-156` の "items" が1件で "referenced_by" に EX-250 と EX-253 が含まれ、`scripts/mutants.sh diff main` の集計の5つの値の合計が 1 以上で終了コードが 0（"survived" が 0）。見逃しにテストを足すときは、先にその変異が壊す振る舞いを IR の要求か具体例で言えることを確かめる。言えないときはテストを足さず、その変異と理由を添えて止まって返す。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test`、`CARGO_BUILD_JOBS=4 cargo run -q -- check`（終了コード 0）、`CARGO_BUILD_JOBS=4 cargo run -q -- status --format text`（最後の行が "complete true"。全行を終端報告に載せる）、`CARGO_BUILD_JOBS=4 cargo run -q -- query --format text REQ-156`（全行を載せる）、`scripts/mutants.sh diff main`（終了コード 0。最後の行の集計を載せる。対象を `-- --file` で分けて走らせたときは、分けた全部の集計を載せ、差分に入るソースのファイルを漏れなく回したことを `git diff --stat main...HEAD -- src` と並べて示す）。
Left to the implementer: なし。
Stop and hand back if: 種類を問わず、ステップ1の基準線に無い指摘が残る。差分の見逃しのうち、IR の要求でも具体例でも壊れる振る舞いを言えないものがある。ファイルごとに分けても 10 分で終わらない対象がある。

## Verification map

| 仕様の項目 | 証拠を出すステップ |
|---|---|
| cli.md REQ-001、REQ-002、REQ-004、REQ-008、EX-219、EX-241、cli-environment.md TBL-020（文言）、REQ-107、ir-references.md REQ-124、query.md REQ-157（引数の数と形）、REQ-158、EX-252、EX-256、status.md REQ-163、EX-262 | ステップ1 |
| ir-missing.md REQ-098（確かめ方）、findings.md TBL-008、EX-259 の And の行、記録の A10（REQ-150 の行） | ステップ2 |
| query.md REQ-156、REQ-157（ID の有無）、REQ-159、REQ-160、REQ-161 の JSON の部分、TBL-027、EX-250、EX-251、EX-253、EX-255、EX-257、cli-environment.md TBL-020（unknown id）、ir-references.md REQ-054、list.md REQ-154、TBL-026 | ステップ3 |
| query.md REQ-161 の text の部分、EX-254、list.md REQ-155 | ステップ4 |
| query.md REQ-161 と status.md REQ-166 の "--format" の値と既定の部分 | 既存の引数の解析（コマンドの分岐の前）で成り立つ。ステップ1で観測し、新しいテストは足さない |
| status.md REQ-162、REQ-164、REQ-165、REQ-166 の JSON の部分、TBL-028、EX-258〜EX-260、EX-263、coverage.md REQ-085、REQ-137、findings.md REQ-032、output.md TBL-021 | ステップ5 |
| status.md REQ-166 の text の部分、EX-261 | ステップ6 |
| skill-references.md REQ-125〜REQ-127（既存の一致の維持） | ステップ7 |
| output.md TBL-005、REQ-128（check の形が変わらないこと） | ステップ3、5 の Stop 条件 |
| このリポジトリの check 0 件、status complete、差分の見逃し 0 件 | ステップ8 |

## Left to the implementer

各ステップの欄のとおり。共通: Rust の型・関数・モジュールの名前、fixture の文面。

## Stop conditions

一般の4条件に加えて、各ステップの「Stop and hand back if」。ステップ2の1行以外で IR・記録を変えたくなったら、変えずに止まって返す。変異テストを含め、長いコマンドを背景に逃がして応答を終えない。

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。check の確認は `CARGO_BUILD_JOBS=4 cargo run -q -- check`（JSON）と `--format text`。差分の変異は `scripts/mutants.sh diff main`。

## Out of scope

- "render" の禁止を残すかどうか（記録の U1）
- list の振る舞いの変更（query は list の型を使うが、list の出力は変えない）
- インストール済みスキル `~/.claude/skills/kotowari/` と `~/.cargo/bin/kotowari` への同期（マージ後に主セッションが行う）
- リリース（この実装の後に主セッションが行う。版の数字は未決）
- ステップ2の1行以外の IR、記録の変更
- 全体の変異テスト（`scripts/mutants.sh full`）
- `kotowari list | head` で出る Broken pipe の panic（既存の振る舞い。別の話題）
