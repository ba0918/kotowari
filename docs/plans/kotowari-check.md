# 実装計画: kotowari-check

## Goal

仕様IR（`docs/ir/`）の要求を満たす Rust の CLI `kotowari check` が、要求 ID の印の付いたテスト、性質のテスト、ミューテーションテスト、要求とテストの対応の表とともに、リポジトリ直下にできている。

## Specification

正本は仕様IR `docs/ir/*.md`（以下「IR」）で、形は `experiments/003-cli/brainstorm/ir-form.md`（以下「契約」）に従う。
人間向けの仕様書 `experiments/003-cli/docs/spec/kotowari-cli.md` は読んでよいが、IR と食い違えば IR が正しい。

IR の読み方:

- 文書ごとに `### REQ-nnn: 名前` の要求、`### TBL-nnn: 名前` の決定表、`### PROP-nnn: 名前` の性質、Gherkin のシナリオ（`@id=EX-nnn`）がある。この計画では項目を `docs/ir/cli.md#REQ-001` のように文書のパスと ID で指す
- 要求の `- 種類:` が `prohibition` の文は起きてはならないこと。`algorithm` の中身は `- 定義:` が指す決定表と性質にある。`- 検証:` が `unit` か `property` の要求はテストで、`review` の要求はコードを読んで確かめる（`docs/ir/ir-items.md#REQ-042`）
- 用語は `docs/ir/CONTEXT.md` にある。文の中のバッククォートで囲んだ語は用語集の語か ID である
- `docs/ir/FLAGS.md` に問題の記録の項目は無い（すべて解決済み）

IR と契約は実装の間は読み取り専用である。IR の書き換えが必要だと思ったら、書き換えずに止まって返す。

この計画の中の版の値、時間の上限、テストの置き方と名前の付け方は、IR ではなくこの計画の約束である。IR の振る舞いを変えるものではない。

## Approach and why

読む順に作る。引数と設定、IR の文書と項目、出典と用語、テストと印、出力の細部、の順である。後のステップは前のステップの結果（設定、読んだ項目）を使うので、この順なら各ステップのテストをそのステップの終わりで GREEN にできる。

package はリポジトリ直下に1つ置き、ライブラリ（`src/lib.rs`）とバイナリ（`src/main.rs`）の2つのターゲットを持たせる。`tests/` の結合テストはライブラリの公開する関数と型を通し、バイナリは引数と入出力だけを持つ。層（文書を読む、項目を検査する、テストを見つける、指摘を並べる）が crate として切り出せる形になったら `crates/` に足す（`docs/ir/cli-scope.md#REQ-105`）。切り出すかどうかと時期は実装者に委ねる（判断の記録 D1）。純粋な部分は `String` と構造体だけを受け取る関数にして、ファイル無しでテストできるようにする。

再利用の判断（層ごと。理由を1行で。この計画の約束として、各層の探索は10分を上限にし、超えたらその旨を記録して自作にする）:

| 層 | 判断 | 理由 |
|---|---|---|
| コマンド行の引数 | 採用 `clap`（定番） | `--format`、`--config`、知らないオプションと位置引数の拒否（`docs/ir/cli.md#REQ-004`）を宣言で書ける |
| 設定ファイルの YAML | 採用 `serde` + `serde-saphyr`（定番の後継） | `serde_yaml` は2024年3月に非推奨になり、`serde_yml` は未保守で健全性の問題が RUSTSEC-2025-0068 として報告されている。`serde-saphyr` は2026年8月に更新があり、`deny_unknown_fields` で知らないキーを落とせる（`docs/ir/config.md#REQ-014`）。設定の形を型で固定する。解決できない、または `deny_unknown_fields` が効かないときは止まって返す |
| IR の Markdown | 自作（最小） | 契約は行の形（見出し、`- xxx:` の行、表の行、フェンス）で決まっていて、指摘に行番号が要る。Markdown の構文木を作る crate（`pulldown-cmark`）は行番号と生の行を別に持ち直す必要があり、契約の形に対して遠回りになる |
| Gherkin のブロック | 自作（数行） | フェンスの中の `@` の行と `Scenario:` の行と `Given` などの行を読むだけ |
| テストの発見 | 採用 `tree-sitter` + `tree-sitter-rust`（定番。`docs/ir/test-discovery.md#REQ-080`） | 利用者が選び ADR-0002 で決めた。`#[test]` の属性、設定の属性、マクロの中身の読み直しを構文木で行う。この環境で確かめた版は `tree-sitter` 0.26.13 と 0.27.0、`tree-sitter-rust` 0.24.2（2026-09-13、crates.io）。組み合わせが取れなければ止まって返す。文法の crate は C コンパイラを要し、この環境には `cc` 13.3 がある |
| テストのファイルの glob | 採用 `globset` + `walkdir`（定番） | `**` の再帰と隠しディレクトリの除外（`docs/ir/config.md#REQ-019`）を自作すると端の扱いを誤りやすい |
| JSON の出力 | 採用 `serde_json`（定番） | 出力の形（`docs/ir/output.md#TBL-005`）を型から作る |
| 判断の記録と ADR の読み込み | 自作（数行） | `- A26 ` で始まる行と `## 見出し` を探すだけ（`docs/ir/sources.md#TBL-012`） |
| UTF-8 の判定 | 標準ライブラリ | `String::from_utf8` の失敗で足りる（`docs/ir/cli.md#TBL-001`） |
| 指摘の並び | 標準ライブラリ | バイト順の比較は `Ord` で足りる（`docs/ir/finding-order.md#TBL-007`） |
| CLI の結合テスト | 採用 `assert_cmd`（定番、dev） | 終了コードと標準出力を確かめる |
| 性質のテスト | 採用 `proptest`（実験002で使用済み、dev） | PROP-001、PROP-002、PROP-003 |

直接の実行時の依存はこの表のものだけにし、ほかを足す必要が出たら止まって返す。推移的な依存は数えない。

テストの置き方と名前（この計画の約束）:

- テストは `tests/` に置き（`src/` の中の単体テストは置かない）、ライブラリの公開する関数と型だけを通す
- 各テスト関数の直前（`#[test]` の直前の行）に `// @kotowari[REQ-001, TBL-002]` の形の印を置き、そのテストが確かめる IR の ID を書く（`docs/ir/test-markers.md#REQ-071`、`docs/ir/test-markers.md#TBL-016`）。これは kotowari 自身の印であり、ステップ8で kotowari 自身にかける
- テスト関数の名前は、確かめる IR の ID を先頭に付けた snake_case にする（例: `req_014_unknown_key_stops`）
- テストは RED → GREEN → REFACTOR の順で書き、各段で `cargo test` を走らせる
- 各ステップの終わりに `src/` の行数（コメントと空行を含む）を数えて記録し、1,500行を超える見込みになったら理由を書いて止まって返す（判断の記録 A9）

固定の入力（判断の記録 A76）は `fixtures/allowlist/` に置く。`tests/` の下に置かないのは、kotowari 自身の既定の `tests.files`（`docs/ir/config.md#TBL-004`）が `tests/**/*.rs` を含み、ステップ8で kotowari 自身にかけるときに固定の入力の印が混ざるためである。

## Scope of change

変えてよいのは、リポジトリ直下の次のものだけである。

- `Cargo.toml`、`Cargo.lock`、`.gitignore`（`target/` と `mutants.out/` を無視する）
- `.cargo/mutants.toml`（ステップ7）
- `src/` の下のすべて
- `tests/` の下のすべて
- `fixtures/` の下のすべて（固定の入力。中に `.kotowari/config.yaml`、`docs/ir/`、`decision/`、`src/`、`tests/` を持つ）
- `docs/trace.md`（ステップ8）

`docs/ir/`、`docs/plans/`、リポジトリ直下の `.kotowari/`、`experiments/` は変えない。

## Step order and prerequisites

ステップ1〜6は順に行う。ステップ7はステップ6の後、ステップ8はステップ7の後に行う。

## Test command

`cargo test`。ミューテーションテストは `cargo mutants`（ステップ7）。

## Step 1 — 引数、基準のディレクトリ、設定、停止

Purpose: `kotowari check` が引数を受け、基準のディレクトリを決め、設定ファイルを読み、誤りで停止する。Specification: `docs/ir/cli.md#REQ-001`〜`docs/ir/cli.md#REQ-008`、`docs/ir/cli.md#TBL-001`、`docs/ir/cli.md#TBL-002`、`docs/ir/base-directory.md#REQ-009`、`docs/ir/base-directory.md#REQ-010`、`docs/ir/base-directory.md#TBL-003`、`docs/ir/base-directory.md#PROP-001`、`docs/ir/config.md#REQ-011`〜`docs/ir/config.md#REQ-020`、`docs/ir/config.md#TBL-004`、`docs/ir/cli-scope.md#REQ-102`、`docs/ir/cli-scope.md#REQ-105`。
Prerequisites: なし。この環境で確かめた道具は `cargo` 1.98.1、`cc` 13.3（2026-09-13）。
May change: `Cargo.toml`、`Cargo.lock`、`.gitignore`、`src/`、`tests/`（`clap`、`serde`、`serde-saphyr`、`globset`、`walkdir`、dev の `assert_cmd` と `proptest`）。
Done when: 引数の誤り（知らないオプション、位置引数、`--format` の知らない値、`--config` の指すファイルが無い）と設定の誤り（知らないキー、型の違い、負の数、0）と置き場の無さ（`ir`、`decisions.records`、`decisions.adr`）と UTF-8 でない設定で終了コード2、標準出力は空、理由は標準エラーに出る。誤りが無ければ、基準のディレクトリと設定（既定と上書き）が決まり、この時点では IR に文書が無いときと同じ空の結果（`docs/ir/output.md#TBL-005` の形で `files` が0、`findings` が空）を既定の `json` で出して終了コード0になる。基準はサブディレクトリからの起動と `--config` で変わらない。
Shown by: test — `req_001_only_check_subcommand`、`req_002_only_format_and_config_options`、`req_003_config_path_is_relative_to_cwd`、`req_004_unknown_option_stops`、`req_004_positional_argument_stops`、`req_004_unknown_format_value_stops`、`req_004_missing_config_file_stops`、`req_005_stop_writes_nothing_to_stdout_and_reason_to_stderr`、`req_006_non_utf8_config_stops`、`req_007_exit_codes_zero_two`、`req_009_base_is_the_dir_holding_dot_kotowari`、`req_009_falls_back_to_cwd`、`req_010_config_values_are_relative_to_base`、`prop_001_config_path_does_not_move_the_base`、`req_011_reads_dot_kotowari_config_by_default`、`req_012_missing_config_uses_defaults`、`req_013_defaults_match_the_table`、`req_014_unknown_key_stops`、`req_014_wrong_type_stops`、`req_014_negative_limit_stops`、`req_014_zero_limit_stops`、`req_015_list_replaces_default`、`req_016_empty_list_means_none`、`req_017_nested_keys`、`req_018_missing_ir_dir_stops`、`req_018_missing_records_dir_stops`、`req_018_missing_adr_dir_stops`、`req_019_glob_is_recursive_and_skips_hidden_dirs`、`req_021_default_format_is_json`。`review` の要求（REQ-008、REQ-020、REQ-102、REQ-105）はステップ8の表で読む場所を書く。
Left to the implementer: モジュールの分け方、引数と設定の構造体の名前、既定値の持ち方。
Stop and hand back if: `clap` か `serde-saphyr` の版が解決できない。`serde-saphyr` で `deny_unknown_fields` が効かない（代わりの crate を選ばずに止まる）。

## Step 2 — IR の文書と項目を読み、形を検査する

Purpose: `ir` の直下の文書を読み、題名、範囲、項目、行、タグ、ID の重複、参照切れ（印を除く）の指摘を出す。Specification: `docs/ir/ir-document.md#REQ-033`〜`docs/ir/ir-document.md#REQ-041`、`docs/ir/ir-document.md#TBL-010`、`docs/ir/ir-items.md#REQ-042`〜`docs/ir/ir-items.md#REQ-051`、`docs/ir/ir-items.md#TBL-011`、`docs/ir/ir-missing.md#REQ-098`〜`docs/ir/ir-missing.md#REQ-100`、`docs/ir/ir-references.md#REQ-052`〜`docs/ir/ir-references.md#REQ-056`、`docs/ir/findings.md#REQ-032`、`docs/ir/form-contract.md#REQ-089`〜`docs/ir/form-contract.md#REQ-091`。
Prerequisites: ステップ1（設定の `ir` と上限）。
May change: `src/`、`tests/`。
Done when: 契約の形を守る文書の集まりで指摘が0になり、種類ごとの最小の文書でその種類の指摘が1件出る。要求、決定表、性質、シナリオ、用語、問題の記録のすべての形（`docs/ir/ir-items.md#TBL-011`）を読める。
Shown by: test — `req_033_only_direct_children`、`req_034_missing_title`、`req_035_multiple_titles`、`req_036_missing_scope`、`req_036_glossary_and_flags_need_no_scope`、`req_037_crlf_counts_as_one_line`、`req_038_too_many_lines_is_a_warning`、`req_039_too_many_requirements_skips_glossary_and_flags`、`req_040_code_blocks_are_skipped_except_gherkin`、`req_042_reads_every_item_kind_in_the_table`、`req_043_unknown_heading`、`req_044_unknown_field`、`req_045_duplicate_field`、`req_046_fields_in_any_order_with_blank_lines_and_commas`、`req_047_missing_statement`、`req_048_verification_missing`、`req_049_verification_invalid`、`req_050_unknown_kind_of_requirement_and_flag`、`req_051_algorithm_without_definition`、`req_098_missing_field`、`req_099_missing_table`、`req_100_scenario_outside_gherkin_is_ignored`、`req_052_unknown_tag`、`req_053_missing_tag`、`req_054_unresolved_reference_in_definition_about_relation_and_sentence`、`req_032_duplicate_id_on_each_later_place_with_its_line`。`review` の要求（REQ-041、REQ-055、REQ-056、REQ-089〜REQ-091）はステップ8の表で読む場所を書く。
Left to the implementer: 読んだ項目を表す構造体の形、行ごとの読み方の関数の切り方。
Stop and hand back if: 契約が定めていない行の形が IR の文書に現れ、指摘の種類を選べない。

## Step 3 — 出典、用語、曖昧語、文書名の参照

Purpose: 出典の実在、バッククォートの語、曖昧語、文書名の参照を検査する。Specification: `docs/ir/sources.md#REQ-057`〜`docs/ir/sources.md#REQ-062`、`docs/ir/sources.md#REQ-106`、`docs/ir/sources.md#TBL-012`、`docs/ir/terms.md#REQ-063`〜`docs/ir/terms.md#REQ-070`、`docs/ir/terms.md#REQ-104`、`docs/ir/terms.md#TBL-013`、`docs/ir/terms.md#TBL-014`、`docs/ir/decision-records.md#REQ-092`〜`docs/ir/decision-records.md#REQ-097`、`docs/ir/decision-records.md#REQ-103`。
Prerequisites: ステップ2。判断の記録と ADR の見本として `experiments/003-cli/brainstorm/` と `experiments/003-cli/adr/` を `fixtures/` に写してよい。出典の印が決定の番号（`A26`、`P1` の形）なら判断の記録として `- 番号 ` の行を、それ以外なら `## 見出し` を探す（判断の記録 A78）。
May change: `src/`、`tests/`。
Done when: 実在する出典は通り、場所の外、書式違い、無い番号、無い見出しが `source_invalid` になり、用語と曖昧語と文書名の参照が契約の対象行と条件で検査される。
Shown by: test — `req_057_source_splits_at_first_hash_and_allows_commas`、`req_058_number_anchor_looks_for_decision_line_and_other_anchor_for_heading`、`req_058_source_outside_places_is_invalid`、`req_059_missing_source_for_item_scenario_and_term`、`req_060_glossary_and_scenario_sources_are_checked`、`req_061_numbers_are_per_file`、`req_106_form_contract_headings_are_valid_sources`、`req_063_only_sentences_and_steps_are_checked`、`req_064_unknown_term`、`req_065_ids_pass_without_glossary`、`req_066_vague_word_substring`、`req_067_one_finding_per_occurrence`、`req_069_reference_needs_boundary_and_quotes_are_skipped`、`req_070_missing_document`、`req_104_quoted_values_are_not_terms`。`review` の要求（REQ-062、REQ-068、REQ-092〜REQ-097、REQ-103）はステップ8の表で読む場所を書く。
Left to the implementer: 出典の解析結果の型、用語集の持ち方。
Stop and hand back if: なし（ステップ固有のものはない）。

## Step 4 — テストの発見と印の結び付け

Purpose: tree-sitter で Rust のテストを見つけ、印を結び付け、要求とテストの対応の指摘を出す。Specification: `docs/ir/test-discovery.md#REQ-079`〜`docs/ir/test-discovery.md#REQ-084`、`docs/ir/test-discovery.md#TBL-017`、`docs/ir/test-markers.md#REQ-071`〜`docs/ir/test-markers.md#REQ-078`、`docs/ir/test-markers.md#TBL-015`、`docs/ir/test-markers.md#TBL-016`、`docs/ir/coverage.md#REQ-085`〜`docs/ir/coverage.md#REQ-088`。
Prerequisites: ステップ2（要求の ID と検証の値）。固定の入力（判断の記録 A76）を `fixtures/allowlist/` に作る: `experiments/002-allowlist/allowlist/src/` と `tests/` を `fixtures/allowlist/src/` と `tests/` に写し、`// spec: X` を `// @kotowari[X]` に書き換え、`src/proofs.rs` の `#[kani::proof]` の関数2つにも印を足す（`proptest!` の中の関数は既に `#[test]` と印を持つ）。印が指す ID の集合（REQ 14、TBL 7、PROP 14、EX 33）をちょうど持つ IR を `fixtures/allowlist/docs/ir/` に機械で作る。要求は `- 種類: ubiquitous`、`- 検証: unit`、決定表は1行の表、性質は1文、シナリオは gherkin のブロックの中に `@id`、`@about`、`@source` を持ち、出典はすべて `fixtures/allowlist/decision/records.md` の決定 `A1` を指す。`fixtures/allowlist/.kotowari/config.yaml` に `ir: docs/ir`、`decisions.records: decision`、`decisions.adr: decision/adr`（空のディレクトリ）、`tests.rust.attributes: [kani::proof]`、`tests.rust.macros: [proptest]` を書く。固定の入力は CLI で、`fixtures/allowlist/` をカレントディレクトリにして走らせる（基準はそこの `.kotowari/` になる）。
May change: `src/`、`tests/`、`fixtures/`、`Cargo.toml`（`tree-sitter`、`tree-sitter-rust`）。
Done when: 固定の入力で `requirement_without_test` と `test_without_id` がともに0件になり、印を1つ外すと `test_without_id` が1件になる。問い合わせの無い拡張子のファイルでは印だけ拾う。IR に文書が無くてもテストの検査は行われる。
Shown by: test — `req_079_reads_files_matching_the_globs`、`req_080_uses_tree_sitter_with_bundled_rust_query`、`req_081_only_rs_maps_to_rust`、`req_082_test_attribute_is_always_counted`、`req_082_configured_attribute_matches_path_with_arguments`、`req_082_macro_body_functions_are_counted_by_last_segment`、`req_083_unparsable_file_is_skipped`、`req_071_marker_syntax_allows_spaces_around_commas`、`req_072_invalid_marker`、`req_073_several_markers_on_one_line`、`req_074_marker_anywhere_in_the_line_regardless_of_comment_syntax`、`req_075_marker_before_attributes_binds`、`req_075_marker_at_body_start_binds`、`req_075_both_places_merge_ids`、`req_075_marker_in_body_middle_is_ignored`、`req_076_unknown_language_scans_raw_text`、`req_077_unresolved_only_marker_still_counts`、`req_078_marker_to_review_requirement_is_not_an_error`、`req_054_marker_to_unknown_id_is_unresolved`、`req_085_fixture_has_no_uncovered_requirement`、`req_086_fixture_has_no_unmarked_test_and_one_after_removal`、`req_087_unknown_language_only_feeds_coverage`、`req_088_empty_ir_still_checks_tests`。`review` の要求（REQ-084）はステップ8の表で読む場所を書く。
Left to the implementer: 問い合わせの文（tree-sitter の query）の書き方、マクロの中身を読み直す関数の切り方、固定の入力の IR を作るスクリプトの形（`fixtures/` の中に置く）。
Stop and hand back if: `tree-sitter` と `tree-sitter-rust` の版の組み合わせがビルドできない（正規表現に落とさずに止まる。判断の記録 A50）。固定の入力で、契約の規則で結び付かないテストが見つかった（規則の不足）。

## Step 5 — 指摘の種類と `detail`、並び、行

Purpose: すべての指摘の種類が契約の `detail` を持ち、契約の順に並び、行が契約どおりになる。Specification: `docs/ir/findings.md#REQ-029`〜`docs/ir/findings.md#REQ-031`、`docs/ir/findings.md#TBL-008`、`docs/ir/findings.md#TBL-009`、`docs/ir/finding-order.md#REQ-024`、`docs/ir/finding-order.md#REQ-027`、`docs/ir/finding-order.md#REQ-028`、`docs/ir/finding-order.md#TBL-007`、`docs/ir/finding-order.md#PROP-003`。
Prerequisites: ステップ4。
May change: `src/`、`tests/`。
Done when: 誤りの種類ごとに `detail` が契約の表のものになり、警告は2種類だけで、文書全体への指摘は `line` が null、ほかは1始まりの行、並びは契約の順になる。
Shown by: test — `req_029_every_error_kind_has_the_detail_of_the_table`、`req_030_warning_kinds_have_the_detail_of_the_table`、`req_031_only_two_kinds_are_warnings`、`req_027_document_wide_findings_have_null_line`、`req_028_lines_start_at_one`、`req_024_sorted_by_path_line_kind_detail`、`prop_003_findings_are_sorted`。
Left to the implementer: 指摘の型の名前、並べ替えの関数の置き場。
Stop and hand back if: なし。

## Step 6 — 出力と終了コード

Purpose: 指摘を JSON と文字で出し、終了コードを決める。Specification: `docs/ir/output.md#REQ-021`、`docs/ir/output.md#REQ-022`、`docs/ir/output.md#REQ-023`、`docs/ir/output.md#REQ-025`、`docs/ir/output.md#REQ-026`、`docs/ir/output.md#TBL-005`、`docs/ir/output.md#TBL-006`、`docs/ir/output.md#PROP-002`、`docs/ir/cli.md#REQ-007`、`docs/ir/cli.md#TBL-002`、`docs/ir/cli-scope.md#REQ-101`。
Prerequisites: ステップ5。
May change: `src/`、`tests/`、`Cargo.toml`（`serde_json`）。
Done when: `json` は1つの JSON で `files`（用語集と問題の記録を含む文書の数）、`lines`（行数の合計）、`findings`、`counts`（0件の種類を含まない）を持ち、`text` は1指摘1行で重大度を角括弧ごと出し、行の無い指摘は `-` になり、終了コードは誤りなし（警告だけを含む）で0、誤りありで1、停止で2になる。
Shown by: test — `req_021_format_values_are_json_and_text`、`req_022_json_is_one_document`、`req_023_files_counts_all_docs_and_lines_sums_them`、`req_023_finding_keys_match_the_table`、`prop_002_counts_match_findings`、`req_025_text_has_one_line_per_finding_with_bracketed_severity`、`req_026_null_line_prints_dash`、`req_007_exit_code_one_on_error_and_zero_on_warning_only`。`review` の要求（REQ-101）はステップ8の表で読む場所を書く。
Left to the implementer: 出力の関数の切り方。
Stop and hand back if: なし。

## Step 7 — ミューテーションテスト

Purpose: テストが実装の変更を捕まえることを確かめる。Specification: ステップ1〜6の Specification に挙げた項目のすべて。
Prerequisites: ステップ6。この環境で確かめた `cargo-mutants` は 27.1.0（2026-09-13）。`CARGO_BUILD_JOBS=4` で並びを絞る（実験002の O-2）。
May change: `.cargo/mutants.toml`、`tests/`、`src/`（見逃しを捕まえるテストの追加と、その修正だけ）。
Done when: `cargo mutants` の見逃しが0件になる。等価な変更は理由を `docs/trace.md` に書いて除く（表そのものはステップ8で書く）。
Shown by: check — `CARGO_BUILD_JOBS=4 cargo mutants` を走らせ、出力の missed が0（等価な変更を除く）であること。
Left to the implementer: `mutants.toml` の除外の書き方。
Stop and hand back if: 見逃しを捕まえるために IR に無い振る舞いを決める必要が出た。`cargo mutants` が環境の都合で走らない（実験002の O-2、O-3）。2回の追加で見逃しが減らない。

## Step 8 — 要求とテストの対応の表と、kotowari 自身への適用

Purpose: 要求ごとの確かめ方を表にし、`review` の要求の読む場所を書き、kotowari 自身の IR とテストにかけた結果を記録する。Specification: ステップ1〜6の Specification に挙げた項目のすべて。`review` の要求は REQ-008、REQ-020、REQ-041、REQ-055、REQ-056、REQ-062、REQ-068、REQ-084、REQ-089〜REQ-097、REQ-101〜REQ-103、REQ-105 の21件。
Prerequisites: ステップ7。
May change: `docs/trace.md`。
Done when: `docs/trace.md` に IR の要求106件すべての確かめ方（テストの関数名、または `review` なら読む場所と確認コマンド）と、等価な変更の一覧と、`kotowari check` をリポジトリ直下で走らせた結果（指摘の数と種類。通ることは求めない。実験003の記録項目）がある。
Shown by: artifact — `docs/trace.md`。要求 ID の行が106あることを `grep -c '^| REQ-' docs/trace.md` で数える。
Left to the implementer: 表の並び。
Stop and hand back if: kotowari 自身にかけた結果が、この計画の約束（印の置き方）と契約の食い違いを示した（例: `#[test]` の直前の印が結び付かない）。

## Verification map

| IR の文書 | 確かめるステップ |
|---|---|
| `cli.md`、`cli-scope.md`、`base-directory.md`、`config.md` | 1（REQ-007 と REQ-101 は 6 も） |
| `ir-document.md`、`ir-items.md`、`ir-missing.md`、`ir-references.md`、`form-contract.md` | 2 |
| `sources.md`、`terms.md`、`decision-records.md` | 3 |
| `test-discovery.md`、`test-markers.md`、`coverage.md` | 4 |
| `findings.md`、`finding-order.md` | 5（REQ-032 は 2） |
| `output.md` | 6 |
| すべて | 7、8 |

## Left to the implementer

- 公開する関数と型の名前、`src/` の中のモジュールの分け方、crate に切り出すかどうか（判断の記録 D1、`docs/ir/cli-scope.md#REQ-105`）
- 内部の誤りの型の設計
- tree-sitter の問い合わせの文
- 固定の入力の IR を作るスクリプトの形

## Stop conditions

計画全体で、次のときは作業を止めて返す。

- IR か契約に書かれていない振る舞いを決めないと進めない（IR の沈黙は実装者が決めてよい意味ではない）
- IR と契約が食い違う、または IR の中で食い違う
- 各ステップの終わりに数えた `src/` の行数から、1,500行を超える見込みになった。超える理由を書いて返す（判断の記録 A9。行数だけでは切らない）
- 再利用の判断の表に無い直接の実行時の依存が要る
- `tree-sitter` の版の組み合わせが取れない（判断の記録 A50）

## Out of scope

- `render`、`trace`、`query`（`docs/ir/cli.md#REQ-008`）
- `--format pretty`（`docs/ir/output.md#REQ-021`）、ID だけの出典（`docs/ir/sources.md#REQ-057`）、`docs/ir/` のサブディレクトリ（`docs/ir/ir-document.md#REQ-033`）、スキーマのファイル（`docs/ir/form-contract.md#REQ-090`）
- Rust 以外の言語の問い合わせ（`docs/ir/test-discovery.md#REQ-081`）
- 形式手法による証明（実験003の D-17）
