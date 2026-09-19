# 実装計画: list

## Goal

`kotowari list` が入り、IR の項目（要求・決定表・性質・シナリオ・問題の記録）とそれを指す印のあるテストを JSON と文字で出せるようになり、手で保っていた `docs/trace.md` が各要求の `- 確かめ方:` の行に移って消える。

## Specification

正本は仕様 IR `docs/ir/*.md`（以下「IR」）。この計画が対象にする項目は次のとおり。

- list の振る舞いと出力: `docs/ir/list.md#REQ-151`、`#REQ-152`、`#REQ-153`、`#REQ-154`、`#REQ-155`、`#TBL-026`、具体例 EX-245〜EX-249
- コマンドと引数: `docs/ir/cli.md#REQ-001`、`#REQ-002`、`#REQ-004`、`#REQ-005`、`#TBL-001`、`#TBL-002`、具体例 EX-219、EX-241
- 停止の文言と詳細: `docs/ir/cli-environment.md#TBL-018`、`#TBL-020`、`#REQ-107`
- 出力の形の値: `docs/ir/output.md#REQ-021`（"--format" の値は全コマンド共通）、`#TBL-005`、`#REQ-128`（どちらも "kotowari check" に限定。list で変えない）
- 項目の形: `docs/ir/ir-items.md#TBL-011`（要求の持つ行に "- 確かめ方:"）、`#REQ-044`（知っている行の集合）、`#REQ-045`、`docs/ir/ir-missing.md#REQ-098`（値が空の "- 確かめ方:" は無い行）
- 印とテスト: `docs/ir/test-markers.md#REQ-076`、`#TBL-016`、`docs/ir/test-discovery.md#REQ-081`、`docs/ir/coverage.md#REQ-087`（問い合わせの無い言語の印の拾い方。list の "tests" の元）
- パス: `docs/ir/base-directory.md#REQ-010`、`#REQ-110`
- スキルの references: `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`（既存の一致が保たれること）
- 用語: `docs/ir/CONTEXT.md` の「項目」「シナリオ」「印」「テスト」「テストのファイル」「問い合わせの無い言語」「基準のディレクトリ」「停止」

判断の記録は `docs/decision/records/2026-09-19-read-commands.md`（A1〜A24 と Revisions。以下「記録」）。IR に書かないと決めたもの（trace.md の移し替えと削除の時期、スキルの節、concept.md の表）は、記録の A2、A5、A15、A17 がこの計画の根拠になる。

IR の読み方は前の計画と同じ。要求の `- 検証:` が `review` の要求はテストを求めない。各文書の `## 具体例` のシナリオは、その要求の成功の条件と反例で、テストの入力と期待の元にする。

IR と記録は実装の間は読み取り専用である。例外はステップ6の、`docs/trace.md` の表の 16 行を対応する要求の `- 確かめ方:` の行として移すことだけで（記録の A5、A17）、それ以外の IR の書き換えが必要だと思ったら、書き換えずに止まって返す。IR の沈黙は実装者が決めてよい意味ではない（`docs/ir/cli-environment.md#REQ-120`）。

list の引数の誤りは REQ-004（"list" を含む列挙。記録の Revisions で改めた）、REQ-152（check と同じ条件で同じ理由と文言で停止する）、REQ-002（list が受けるのは4つのオプションだけ）が決めている。

この計画の中のテストの置き方と名前、モジュールの切り方の約束は、IR ではなくこの計画の約束である。

## Approach and why

既存の実装（`src/` 約 5,000 行、テスト 10 ファイル）を育てる。作り直さない。新しい依存は足さない。

層ごとの採否（探した順は、要るか → このコード → 標準ライブラリ → 環境 → 入っている依存 → 定番 → 数行 → 自作）:

- IR の読み取りと項目の解析 — 採用（このコード `ir::load_and_check`、`ir::Item`）。list は check と同じ読み取りを使う（REQ-151）
- 印とテストの発見 — 採用（このコード `tests_discovery::discover_and_check`）。ただし今は Rust のテストの印ごとの (ID, 行) を `DiscoveredTest.marker_ids` に持ちながら集計だけを返して捨て、問い合わせの無い言語では ID しか集合に残していないので、両方から (ID, パス, 印の行, 名前か無し) を集めて返すように広げる
- 引数の読み取り — 採用（このコード `parse_args`）
- JSON の出力 — 採用（入っている依存 `serde_json`）
- 並べ替え — 標準ライブラリ（`sort_by` と `BTreeMap`）
- 項目の組み立てと text の整形 — 数十行で書く（新しいモジュール）

作りの約束（記録の A9、A12、A17 と REQ-151、REQ-152 から）:

- list は check と同じ関数で設定を読み、基準のディレクトリを決め、IR とテストのファイルを読む。2つ目の読み取りの経路を作らない。`run_check` を「全部読んで (文書, 印, 指摘) を返す」部分と「check の結果に組む」部分に分け、list はその前半だけを使う形が最も素直だが、分け方は実装者に任せる。分けた後も check の JSON と文字の出力は1バイトも変わらない
- list は指摘を計算しても捨てる。標準出力にも標準エラーにも出さず、終了コードは読めれば 0（`exit_code_for` を使わない）。停止だけを check と同じ理由と文言で伝える
- 項目の "path" は check の指摘と同じ表示のパス（`join_display_path(cfg.ir, doc.relative_path)`。EX-245 の "docs/ir/a.md"）。tests の "path" はテストのファイルの相対パス（指摘の path と同じ）
- "examples" は全文書のシナリオの "@about" を正引きして作る（`collect_scenarios` が同じ走査をしている。ただしその `line` はタグの行なので examples の元にだけ使い、シナリオの項目の "line" は `Item::Scenario` の `line`（"Scenario:" の行）を使う）
- "tests" の "line" は印のある行（`marker_ids` の行）で、テストの関数の行ではない。問い合わせの無い言語では印の行と "name" null

作業はブランチ `list`（main から切る。worktree を使うならリポジトリの中の `.claude/worktrees/` の下）で行う。main の上で直接は作業しない。差分の変異の基準は `main` にする（main は origin/main より 20 コミット先で push されていない。`origin/main` を基準にするとこの計画と無関係な 20 コミットの変異まで走る。push の前のフックは `origin/main` を基準にするので、時間がかかることは主セッションが承知している）。

読む順はステップ 1（コマンドと引数）→ 2（"- 確かめ方:" の行）→ 3（項目の組み立てと JSON）→ 4（text）→ 5（スキルの reference）→ 6（trace.md の移し替え）→ 7（終端の確認）。3 は 1 のコマンドの入口と 2 の行に、4 は 3 の項目に、6 は 2 の行（check がその行を知らないうちに移すと unknown_field が 16 件出る。記録の A17）と 3（移した行が list に出ることの確認）に依存する。

テストの置き方と名前（この計画の約束。前の計画と同じ）:

- ステップ1のテストは `tests/step1_cli.rs` に足す。ステップ2のテストは `tests/step2_ir.rs` に足す。ステップ3、4のテストは新しい `tests/step10_list.rs` に置く。fixture の作り方は既存のテストに倣う（一時ディレクトリに `.kotowari/config.yaml`、IR の文書、テストのファイルを置き、`assert_cmd` で CLI を起こす）
- シナリオの When が "kotowari list …" を実行する場面のテストは、CLI を起こし、Given の各行を入力に置き、Then の各行を assert する
- 各テスト関数の直前の行に `// @kotowari[REQ-151, EX-245]` の形の印を置く（`skills/kotowari/references/mark.md` の形）。確かめる具体例の ID も印に含める
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case
- テストは RED → GREEN → REFACTOR の順で書き、各段で `CARGO_BUILD_JOBS=4 cargo test` を走らせる。「指摘が出ない」「停止しない」テストは、実装を入れた後に逆の期待を一時的に置いて RED を見てから戻し、その RED の出力を終端報告に載せる
- 既存のテストが新しい仕様と食い違うときは、新しい仕様に合わせて書き直し、コミットの本文に改めた決定を書く。落ちるものが2本、落ちないが書き直すものが1本ある: `tests/step1_cli.rs` の `req_004_no_arguments_names_both_commands`（123 行目）と `req_004_options_without_a_command_names_both_commands`（136 行目）は停止の文言が "check, list or mutants" に改まって落ちる（EX-219、EX-241）。`req_001_check_and_mutants_are_the_only_commands`（16 行目）は落ちないが REQ-001 がコマンド3つに改まったので、名前と assert を新しい文に合わせて書き直す（すべてステップ1）
- コミットは意図ごとに分け、`git add <path>` でファイルを1つずつ。件名は `<type>: <件名>` の形で日本語。帰属の行は付けない

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `src/` の下のすべて
- `tests/` の下のすべて
- `skills/kotowari/SKILL.md`（場面 check の「いつ」の欄だけ）、`skills/kotowari/references/findings.md`（ステップ5）
- `docs/ir/*.md` のうち、`docs/trace.md` の表に載る 16 の要求の見出しの下に `- 確かめ方:` の行を1行ずつ足すことだけ（ステップ6）
- `docs/trace.md`（削除。ステップ6）
- `docs/spec/concept.md`（置き場の表の `docs/trace.md` の行だけ。ステップ6）
- `Cargo.lock`（依存の更新はしない。ビルドで変わった分だけ）

`Cargo.toml` は変えない。上に挙げた行以外の `docs/ir/`、`docs/decision/`、`docs/plans/`、`docs/spec/`、`.kotowari/`、`experiments/`、`fixtures/`、`scripts/`、`lefthook.yml`、`.gitignore` は変えない。インストール済みスキル（`~/.claude/skills/kotowari/`）と `~/.cargo/bin/kotowari` には触らない。

## Step order and prerequisites

ステップ1 → 2 → 3 → 4 → 5 → 6 → 7 の順に行う。

## Step 1 — コマンドと引数

Purpose: "kotowari list" を3つ目のコマンドとして受け、引数の誤りと停止の文言を仕様のとおりにする。Specification: `docs/ir/cli.md#REQ-001`、`#REQ-002`、`#REQ-004`、`docs/ir/list.md#REQ-152`、`docs/ir/cli-environment.md#TBL-020`、`#REQ-107`、`docs/ir/output.md#REQ-021`。
Prerequisites: なし。始める前に `CARGO_BUILD_JOBS=4 cargo test` を1回走らせて全件通ることと、`CARGO_BUILD_JOBS=4 cargo run -q -- check --format text` の出力を記録する。この時点の終了コードは 1 で、指摘は `docs/ir/list.md` の requirement_without_test 5件（REQ-151〜REQ-155）と scenario_without_test 5件（EX-245〜EX-249）の計 10 件のはず。これを基準線にする。
May change: `src/lib.rs`（`Cli`、`parse_args`、`run`、`print_help`）、`src/main.rs`、`tests/step1_cli.rs`。
Done when: "list" が1つ目の位置引数として通り、`Cli` に list の変種がある。コマンド無しと、オプションだけの停止の文言が "expected command: check, list or mutants"。"list" に付けた "--tool" と "list" の後の位置引数は、check と同じく引数の誤りで停止する（テストは終了コード 2 と標準エラーの1行目が "argument error: " で始まることまでを見る。詳細の文は TBL-020 の「説明の文と、問題の引数の文字」の範囲で実装者が書く）。"list" のオプションはコマンドの前後どちらでも受ける。"--help" と "--version" は "list" と一緒でも REQ-107 のとおり。使い方の表示に "list" を載せる（IR の契約ではなくこの計画の約束。テストでは固定しない）。正しい引数の "kotowari list" は、この時点では check と同じ読み取り（`run_check`）を通してから最上位が "items" だけで中身が空の JSON を出して終了コード 0 で終わる（ステップ3までの仮の形。読み取りを通すのは、停止が check と同じであることをこのステップで観測するため）。
Shown by: test — `req_001_check_list_and_mutants_are_the_only_commands`（既存の書き直し）、`req_004_no_arguments_names_all_three_commands`（EX-219。既存の書き直し）、`req_004_options_without_a_command_names_all_three_commands`（EX-241。既存の書き直し）、`req_152_tool_on_list_is_an_argument_error`、`req_152_positional_after_list_is_an_argument_error`、`req_002_list_options_can_come_before_or_after_the_command`、`req_152_unreadable_config_stops_like_check`（EX-249。YAML として読めない設定で "kotowari check" と "kotowari list" を両方起こし、終了コードが 2 で標準エラーの1行目が同じ文字であることを見る）。
Left to the implementer: `Cli` の型の形（list と check を1つの変種にまとめるか分けるか）、使い方の表示の文面、仮の JSON の出し方（`run_check` を通す約束の中で）。
Stop and hand back if: REQ-002、REQ-004、REQ-152 のどれにも当たらない引数の並びで、停止か続行かを決められない。

## Step 2 — "- 確かめ方:" を要求の知っている行にする

Purpose: 要求の見出しの下の "- 確かめ方:" の行を check が知っている行として読み、値を項目に持たせる。Specification: `docs/ir/ir-items.md#TBL-011`、`#REQ-044`、`#REQ-045`、`docs/ir/ir-missing.md#REQ-098`。
Prerequisites: なし（ステップ1と独立だが、順はステップ1の後）。
May change: `src/ir.rs`（`Item::Requirement` に確かめ方の欄、要求の知っている行の一覧）、`tests/step2_ir.rs`。
Done when: 要求の見出しの下の "- 確かめ方: 自由文" の行が unknown_field にならず、値が `Item::Requirement` に残る。決定表・性質・問題の記録の下の同じ行は今までどおり unknown_field（TBL-011 は要求だけに持たせる）。同じ行が2つあれば REQ-045 のとおり duplicate_field（既存の `check_fields` が知っている行に共通に掛ける）。値が空の行は REQ-098 のとおり無い行として扱う（`is_blankable_field` に足す。check ではこの行は必須でないので指摘は出ず、無い扱いはステップ3の list の null で観測する）。行の値の中身は読まない（用語と曖昧語の検査の対象は `docs/ir/terms.md#TBL-013` のとおり "- " の行を含まない。文書名の参照の検査はコードブロックの外の全行を見るので、fixture の値に ".md" を含めない）。
Shown by: test — `req_044_how_to_verify_is_a_known_line_of_a_requirement`（"- 確かめ方:" の行を持つ要求で unknown_field が出ないこと。値が項目に残ることはステップ3の `req_153_how_to_verify_is_the_value_of_the_line_or_null` で観測する）。
Left to the implementer: 欄の名前と、`Item::Requirement` を組み立てる箇所の書き方。
Stop and hand back if: "- 確かめ方:" の行を要求以外にも認めないと既存のテストが通らない。

## Step 3 — 項目の組み立てと JSON

Purpose: check と同じ読み取りから TBL-026 の鍵を持つ項目の並びを組み立て、"items" だけの JSON を出す。Specification: `docs/ir/list.md#REQ-151`、`#REQ-153`、`#REQ-154`、`#REQ-155`、`#TBL-026`、`docs/ir/test-markers.md#REQ-076`、`#TBL-016`、`docs/ir/coverage.md#REQ-087`、`docs/ir/base-directory.md#REQ-010`、`#REQ-110`、`docs/ir/output.md#TBL-005`、`#REQ-128`。
Prerequisites: ステップ1、2。
May change: `src/` の下（新しいモジュール。`src/lib.rs` の `run_check` の分割と list の入口。`src/tests_discovery.rs` の `discover_and_check` の返す値に印ごとの (ID, テストのファイルのパス, 印の行, テストの名前か無し) を足す）、`tests/step10_list.rs`（新規）。
Done when: 次のすべて。

- "kotowari list" の JSON の最上位が "items" だけで、"items" の1件が TBL-026 の「持つ項目」の列のとおりの鍵だけを持つ（要求に "relations" は無く、決定表に "type" は無い、など）。値の中身は TBL-026 の「中身」の列のとおり
- 項目は文書の "path" の昇順、同じ "path" の中は "line" の昇順。"tests" も "path" → "line" の順
- IR の文書に誤り（"- 検証:" の無い要求、知らない行など）があっても、読めた項目は出て終了コードは 0。指摘は標準出力にも標準エラーにも出ない
- 問い合わせの無い言語のテストのファイルの印は "name" が null で "tests" に載る
- 同じテストに同じ ID の印が複数の行にあれば、"tests" は印の出現ごとに1件（TBL-026 の tests の行）
- シナリオの "name" は "Scenario:" の後の文字から前後の半角空白とタブを除いたもの、"line" は "Scenario:" の行（TBL-026 の name と line の行）
- ID の無い項目（"@id" の無いシナリオ、形に合わない見出し、用語）は "items" に出さない（記録の A6）
- 鍵の有無は種類で決まり、値が無いときは鍵ごと消さずに null か空の並びを出す（`skip_serializing_if` を使うなら種類ごとの鍵の有無にだけ使う）
- "kotowari check" の JSON と文字の出力は、このステップの前後で変わらない（TBL-005、REQ-128 は check の形のまま）

Shown by: test — `req_151_requirement_with_a_marked_test_is_listed`（EX-245）、`req_151_items_are_listed_despite_ir_errors_and_exit_zero`（EX-246）、`req_153_test_in_a_language_without_a_query_has_a_null_name`（EX-247）、`req_153_table_property_scenario_and_flag_carry_their_keys`（TBL-026 の「持つ項目」の列を1本で縛る。要求以外の4種類がそれぞれの鍵だけを持つことと、シナリオの "line" が "Scenario:" の行で "name" がその後の文字であること）、`req_153_examples_are_the_scenarios_about_the_item`（要求・決定表・性質の "examples"）、`req_153_how_to_verify_is_the_value_of_the_line_or_null`（行がある要求、行が無い要求、値が空の要求の3つを1本で見る。REQ-098 の「空は無い行」の観測はここだけ）、`req_153_tests_have_one_entry_per_marker_occurrence`、`req_154_items_and_tests_are_ordered_by_path_then_line`、`req_155_json_top_level_has_only_items`。
Left to the implementer: モジュールの名前と切り方、`run_check` の分け方（check の出力を変えない約束の中で）、`discover_and_check` の返す値の形（印の一覧を集計と一緒に返すか、発見と検査を分けるか）、項目の型を `serde::Serialize` の構造体にするか `serde_json::Value` で組むか（鍵の有無が種類で変わるので、種類ごとに構造体を分けるか `skip_serializing_if` を使うかは任せる）。
Stop and hand back if: check の JSON か文字の出力が変わる。TBL-026 の「中身」の列で決まらない値の形が出る（例: "sources" の1件の文字をどこまで正規化するか。仕様は「出典の並び」としか言わないので、IR に書かれた文字のまま出す。それで足りない場面が出たら止まる）。

## Step 4 — text の形

Purpose: "--format text" で1項目1行とテストの行を出す。Specification: `docs/ir/list.md#REQ-155`。
Prerequisites: ステップ3。
May change: `src/` の下（list の出力の関数）、`tests/step10_list.rs`。
Done when: 1つの項目が "ID 検証 名前 パス:行 tests=数" の1行で、要求以外は "検証" の欄が "-"。その直後に "tests" の1件ごとに半角空白2つで字下げした "パス:行 名前" の行が続き、"name" が null なら "-"。順はステップ3の JSON と同じ。名前と検証の値はエスケープせずそのまま出す（半角空白を含む見出しの名前も。REQ-155 は形だけを決めていて、区切りの規則を足すのは仕様の判断）。
Shown by: test — `req_155_text_prints_one_line_per_item_and_indented_test_lines`（EX-248。2行の文字を丸ごと比べる）、`req_155_text_writes_dash_for_a_null_test_name`（問い合わせの無い言語のテストの行）、`req_155_text_writes_dash_for_the_verification_of_a_non_requirement`（決定表の行）。
Left to the implementer: 整形の関数の切り方。
Stop and hand back if: EX-248 の2行を丸ごと比べて、REQ-155 の文から組み立てた出力と1バイトでも違う（文と具体例が食い違っている）。

## Step 5 — スキルの reference の追従

Purpose: スキル kotowari が list の出力の読み方を伝え、references の値が本体と一致したままである。Specification: `docs/ir/skill-references.md#REQ-125`、`#REQ-126`、`#REQ-127`。節の中身の根拠は記録の A2、A5、A15。
Prerequisites: ステップ4。
May change: `skills/kotowari/references/findings.md`、`skills/kotowari/SKILL.md`（場面 check の「いつ」の欄に "kotowari list" の出力を読むことを足す。新しい場面と発火語は足さない。記録の A15）。
Done when: `findings.md` に「list の読み方」の短い節（見出しの無い文書なので段落でよい）があり、次が書いてある: `kotowari list` は check と同じ読み取りで、終了コードは読めれば 0 で停止だけ 2、指摘は出ない。JSON の最上位は "items" だけで、1件の鍵は TBL-026 の種類ごとの集合（要求の "how_to_verify" は "- 確かめ方:" の行で、検証が review の要求の確かめ方はここにある。手で保つ表は無い）。この「表は無い」はステップ6で `docs/trace.md` を消して初めて事実になる。同じブランチの中で整合するので、ステップ5の時点で食い違っていてよい。"tests" の "name" が null の件は問い合わせの無い言語の印。text の1行の形。`findings.md` の先頭の改訂日がこの変更の日付。`SKILL.md` の場面 check の行が list の出力を読む場面を含む。
Shown by: artifact — `skills/kotowari/references/findings.md` の足した段落と `skills/kotowari/SKILL.md` の場面の表を、終端報告に引用する。前提として、既存の `req_125_findings_reference_kinds_match_the_code`、`req_126_config_reference_setup_yaml_parses_to_the_defaults`、`req_127_findings_reference_stop_wordings_match_the_code` が通ること（新しいテストは足さない。references の文面を固定するテストは、IR が契約にしていない文面を固定することになる）。
Left to the implementer: 段落の文面と置く位置。
Stop and hand back if: 既存の3本のテストを通すために、references ではなくテストの突き合わせの仕方を変える必要がある。

## Step 6 — trace.md の移し替えと削除

Purpose: `docs/trace.md` の表の 16 行を、対応する要求の `- 確かめ方:` の行として IR に移し、`docs/trace.md` を消し、置き場の表から外す。根拠は記録の A2、A5、A17。Specification: `docs/ir/ir-items.md#TBL-011`。
Prerequisites: ステップ2（check がその行を知っていること）、ステップ3（移した行が list の "how_to_verify" に出ること）。
May change: `docs/ir/*.md`（表の 16 の要求の見出しの下。REQ-041、REQ-084、REQ-089、REQ-092〜REQ-097、REQ-101、REQ-103、REQ-105、REQ-108、REQ-109、REQ-120、REQ-136。それぞれの文書は `rg -n '^### REQ-041:' docs/ir` のように見出しで探す）、`docs/trace.md`（削除）、`docs/spec/concept.md`（置き場の表）。
Done when: 16 の要求のそれぞれの見出しの下に、既存の "- 検証:"（と "- 定義:" があればその後）の行に続けて `- 確かめ方: <表の行の3列目の文字そのまま>` の1行がある。3列目は、表の行の生のテキストのうち `| review | ` の後から行末の ` |` の前までを丸ごと取る（REQ-097 と REQ-109 の行はバッククォートの中に "|" を含むので、Markdown の列の区切りで切らない）。表の文字は変えない（バッククォートも `rg` のコマンドもそのまま。REQ-105 の行のモジュールの列挙のように今の実体と合っていない文もそのまま移す。内容の更新はこの cycle の範囲外。"- " の行は用語の検査の対象外で、文書名の参照の条件の ".md" を含む行は無い）。`docs/trace.md` が無い。`docs/spec/concept.md` の置き場の表から `docs/trace.md` の行が消え、`docs/ir/` の行の「担当」に「検証が review の要求の確かめ方（"- 確かめ方:" の行）」が足されている。ほかの行と文は変えない。
Shown by: check — この順に走らせ、出力を終端報告に載せる。(1) `CARGO_BUILD_JOBS=4 cargo run -q -- check --format text` の指摘に unknown_field も duplicate_field も無い（ステップ1の基準線に無い指摘が1件も無い）。(2) `CARGO_BUILD_JOBS=4 cargo run -q -- list | jq '[.items[] | select(.how_to_verify != null) | .id]'` が 16 の ID をすべて出し、それ以外を出さない。(3) `test ! -e docs/trace.md`。(4) `rg -n 'trace\.md' docs/spec docs/ir skills` が何も出さない（`docs/decision/` の記録は経緯なので残る）。
Left to the implementer: なし（文字はそのまま移す）。
Stop and hand back if: 表の行のうち、対応する見出しが IR に無いものがある。移した行が (1) で unknown_field 以外の新しい指摘を出す（用語か文書名の参照が掛かった。行の文字を変えずに止まって返す）。

## Step 7 — 終端の確認

Purpose: 全体が仕様どおりで、このリポジトリで check が 0 件、このブランチの差分の見逃しが 0 件であることを確かめる。Specification: 上の全部。
Prerequisites: ステップ1〜6。
May change: `tests/` の下（見逃しを捕まえるテストの追加）、`src/` の下（等価を見つけたとき、コードを単純にして変異を無くす直し）。等価の一覧（`.kotowari/equivalents.yaml`）には足さない（足す必要があると判断したら、その変異と理由を添えて止まって返す。一覧に書くには別の文脈の反証が要り、実装者は自分の判定を自分で通せない）。
Done when: `CARGO_BUILD_JOBS=4 cargo test` が全件通り、`CARGO_BUILD_JOBS=4 cargo run -q -- check` の終了コードが 0 で "findings" が空で、`CARGO_BUILD_JOBS=4 cargo run -q -- list` の終了コードが 0 で "items" に REQ-151〜REQ-155 と EX-245〜EX-249 の 10 件があり、そのそれぞれの "tests" が 1 件以上で、`scripts/mutants.sh diff main` の集計の5つの値の合計が 1 以上（変異が1件も走らずに通ったのではないこと）で終了コードが 0（"survived" が 0）。見逃しにテストを足すときは、先にその変異が壊す振る舞いを IR の要求か具体例で言えることを確かめる。言えないとき（要求が曖昧で区別できないとき）はテストを足さず、その変異と理由を添えて止まって返す。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo test`、`CARGO_BUILD_JOBS=4 cargo run -q -- check`（終了コード 0）、`CARGO_BUILD_JOBS=4 cargo run -q -- check --format text`（何も出ない）、`CARGO_BUILD_JOBS=4 cargo run -q -- list --format text | rg '^(REQ-15[1-5]|EX-24[5-9]) '`（10 行）、`scripts/mutants.sh diff main`（終了コード 0。最後の行の集計を終端報告に載せる。差分に `src/terms.rs` が入らなければ時間切れは出ないはず。対象を `-- --file` で分けて走らせたときは、分けた全部の集計を載せ、差分に入るソースのファイルを漏れなく回したことを `git diff --stat main...HEAD -- src` と並べて示す）。
Left to the implementer: なし。
Stop and hand back if: 種類を問わず、ステップ1の基準線に無い指摘が残る。差分の見逃しのうち、IR の要求でも具体例でも壊れる振る舞いを言えないものがある。ファイルごとに分けても 10 分で終わらない対象がある。

## Verification map

| 仕様の項目 | 証拠を出すステップ |
|---|---|
| cli.md REQ-001、REQ-002、REQ-004、EX-219、EX-241、cli-environment.md TBL-020、REQ-107、output.md REQ-021、list.md REQ-152、EX-249、REQ-155 のうち "--format" の値と既定と絞り込みを持たない部分（既定の json はステップ3のテストが通ることで観測し、新しいテストは足さない） | ステップ1 |
| ir-items.md TBL-011（"- 確かめ方:"）、REQ-044、REQ-045、ir-missing.md REQ-098 の "- 確かめ方:" の部分（観測はステップ3） | ステップ2 |
| list.md REQ-151、REQ-153、REQ-154、TBL-026、EX-245〜EX-247、REQ-155 の JSON の部分、test-markers.md REQ-076、TBL-016、coverage.md REQ-087、base-directory.md REQ-010、REQ-110（"tests" と "path" の元）、output.md TBL-005 と REQ-128 の check への限定 | ステップ3 |
| list.md REQ-155 の text の部分、EX-248 | ステップ4 |
| skill-references.md REQ-125〜REQ-127（既存の一致の維持） | ステップ5 |
| 記録の A2、A5、A15、A17（trace.md の移し替えと削除、スキルの節、concept.md の表） | ステップ5、6 |
| このリポジトリの check 0 件と差分の見逃し 0 件 | ステップ7 |

## Left to the implementer

各ステップの欄のとおり。共通: Rust の型・関数・モジュールの名前、fixture の文面。

## Stop conditions

一般の4条件に加えて、各ステップの「Stop and hand back if」。ステップ6の 16 行以外で IR・記録を変えたくなったら、変えずに止まって返す。変異テストを含め、長いコマンドを背景に逃がして応答を終えない（応答で終わる実行の形では、背景の処理はそこで殺される）。

## Test command

`CARGO_BUILD_JOBS=4 cargo test`。check の確認は `CARGO_BUILD_JOBS=4 cargo run -q -- check`（JSON）と `--format text`。list の確認は `CARGO_BUILD_JOBS=4 cargo run -q -- list`。差分の変異は `scripts/mutants.sh diff main`。

## Out of scope

- `query <id>` と `status`（記録の A4。それぞれ小さい壁打ちの後）。この計画は list だけで、query の形を先取りしない
- yaml の出力と絞り込みのオプション（記録の R3、A8）
- "- 確かめ方:" の行の有無の検査（記録の A11。status の壁打ちで決める）
- インストール済みスキル `~/.claude/skills/kotowari/` と `~/.cargo/bin/kotowari` への同期（マージ後に主セッションが行う）
- リリース（list の実装の後に主セッションが行う。版の数字は未決）
- ステップ6の 16 行以外の IR、記録の変更
- 全体の変異テスト（`scripts/mutants.sh full`）
