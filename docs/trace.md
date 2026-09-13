# 要求とテストの対応の表

この表は、`docs/ir` の要求と、それを確かめるテスト（`tests/*.rs` の `// @kotowari[ID]` の印で結び付いた関数名）の対応を記す。unit と property の行はテストの印から機械的に作り、review の行は確認の手順を手で書く。

## 要求の確かめ方

| REQ | 種類 | 確かめ方 |
|---|---|---|
| REQ-001 | unit | `req_001_only_check_subcommand` |
| REQ-002 | unit | `req_002_only_format_and_config_options` |
| REQ-003 | unit | `req_003_config_path_is_relative_to_cwd` |
| REQ-004 | unit | `req_004_unknown_option_stops`, `req_004_positional_argument_stops`, `req_004_unknown_format_value_stops`, `req_004_missing_config_file_stops` |
| REQ-005 | unit | `req_004_unknown_option_stops`, `req_004_positional_argument_stops`, `req_005_stop_writes_nothing_to_stdout_and_reason_to_stderr`, `req_005_stderr_carries_the_stop_reason_text` |
| REQ-006 | unit | `req_006_non_utf8_config_stops`, `tbl_001_unreadable_test_file_stops`, `tbl_001_non_utf8_test_file_stops` |
| REQ-007 | unit | `req_004_unknown_option_stops`, `req_007_exit_codes_zero_two`, `req_007_exit_code_one_on_error_and_zero_on_warning_only` |
| REQ-008 | review | `src/main.rs` に render, trace, query のサブコマンドがないことを確認。`grep -c "render\|trace\|query" src/main.rs` が 0 |
| REQ-009 | unit | `req_009_base_is_the_dir_holding_dot_kotowari`, `req_009_falls_back_to_cwd` |
| REQ-010 | unit | `req_010_config_values_are_relative_to_base` |
| REQ-011 | unit | `req_011_reads_dot_kotowari_config_by_default` |
| REQ-012 | unit | `req_012_missing_config_uses_defaults` |
| REQ-013 | unit | `req_013_defaults_match_the_table` |
| REQ-014 | unit | `req_014_unknown_key_stops`, `req_014_wrong_type_stops`, `req_014_negative_limit_stops`, `req_014_zero_limit_stops`, `req_014_empty_vague_word_stops`, `req_014_empty_vague_word_stops_with_config_error` |
| REQ-015 | unit | `req_015_list_replaces_default` |
| REQ-016 | unit | `req_016_empty_list_means_none` |
| REQ-017 | unit | `req_017_nested_keys` |
| REQ-018 | unit | `req_018_unreadable_dir_stops`, `req_018_unreadable_records_dir_stops`, `req_018_unreadable_adr_dir_stops`, `req_018_missing_ir_dir_stops`, `req_005_stderr_carries_the_stop_reason_text`, `req_018_missing_records_dir_stops`, `req_018_missing_adr_dir_stops`, `req_018_unreadable_directory_under_tests_stops` |
| REQ-019 | unit | `req_019_glob_is_recursive_and_skips_hidden_dirs`, `req_019_hidden_directory_is_excluded_and_subdirectory_is_included` |
| REQ-020 | review | `src/lib.rs` で `.kotowari/config.yaml` だけを読み、`kotowari.toml` を読まないことを確認。`grep -c "kotowari.toml" src/lib.rs` が 0 |
| REQ-021 | unit | `req_021_default_format_is_json`, `req_021_format_values_are_json_and_text` |
| REQ-022 | unit | `req_022_json_is_one_document` |
| REQ-023 | unit | `req_023_files_counts_all_docs_and_lines_sums_them`, `req_023_finding_keys_match_the_table` |
| REQ-024 | property | `req_024_sorted_by_path_line_kind_detail`, `prop_003_findings_are_sorted`（PROP-003 の印） |
| REQ-025 | unit | `req_025_text_has_one_line_per_finding_with_bracketed_severity` |
| REQ-026 | unit | `req_026_null_line_prints_dash` |
| REQ-027 | unit | `req_027_document_wide_findings_have_null_line` |
| REQ-028 | unit | `req_028_lines_start_at_one` |
| REQ-029 | unit | `req_029_every_error_kind_has_the_detail_of_the_table` |
| REQ-030 | unit | `req_030_warning_kinds_have_the_detail_of_the_table` |
| REQ-031 | unit | `req_031_only_two_kinds_are_warnings` |
| REQ-032 | unit | `req_032_duplicate_id_on_each_later_place_with_its_line` |
| REQ-033 | unit | `req_033_only_direct_children` |
| REQ-034 | unit | `req_034_missing_title` |
| REQ-035 | unit | `req_035_multiple_titles` |
| REQ-036 | unit | `req_036_missing_scope`, `req_036_glossary_and_flags_need_no_scope` |
| REQ-037 | unit | `req_037_crlf_counts_as_one_line`, `req_037_crlf_title_and_requirement_line_numbers`, `req_037_empty_content_has_zero_lines` |
| REQ-038 | unit | `req_038_too_many_lines_is_a_warning`, `req_038_exactly_at_limit_no_warning_one_over_warns` |
| REQ-039 | unit | `req_039_too_many_requirements_skips_glossary_and_flags`, `req_039_unknown_heading_does_not_inflate_requirement_count` |
| REQ-040 | unit | `req_040_code_blocks_are_skipped_except_gherkin`, `req_040_gherkin_code_block_doc_ref_is_not_checked` |
| REQ-041 | review | `src/ir.rs` で scope_lines の中身を検査せず存在だけ確認。`check_documents` に範囲の内容検査がないことを確認 |
| REQ-042 | unit | `req_042_reads_every_item_kind_in_the_table`, `req_053_consecutive_scenarios_without_tags_each_get_missing_tag`, `req_042_gherkin_all_step_keywords_recognized`, `req_042_glossary_table_parses_terms`, `req_042_flag_relation_and_source_fields_read`, `req_042_scenario_source_tag_parsed_into_sources` |
| REQ-043 | unit | `req_043_unknown_heading`, `req_043_unknown_heading_detail_is_full_heading_text`, `req_043_unknown_heading_invalid_id_detail_is_full_heading_text`, `req_043_heading_without_colon_is_unknown` |
| REQ-044 | unit | `req_044_unknown_field`, `req_044_unknown_field_without_colon_has_line_text_as_detail`, `req_044_property_definition_field_is_unknown`, `req_044_prop_unknown_field_does_not_produce_unresolved_reference` |
| REQ-045 | unit | `req_045_duplicate_field` |
| REQ-046 | unit | `req_046_fields_in_any_order_with_blank_lines_and_commas` |
| REQ-047 | unit | `req_047_missing_statement`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-048 | unit | `req_048_verification_missing`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-049 | unit | `req_049_verification_invalid`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-050 | unit | `req_050_unknown_kind_of_requirement_and_flag`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-051 | unit | `req_051_algorithm_without_definition`, `req_051_algorithm_definition_must_point_to_tbl_or_prop`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-052 | unit | `req_052_unknown_tag`, `req_052_bare_tag_without_equals_is_unknown` |
| REQ-053 | unit | `req_053_consecutive_scenarios_without_tags_each_get_missing_tag`, `req_053_missing_tag`, `req_053_tag_with_empty_value_is_treated_as_missing`, `gherkin_tags_cleared_after_block_without_scenario` |
| REQ-054 | unit | `req_054_unresolved_reference_in_definition_about_relation_and_sentence`, `req_054_flag_relation_to_unknown_id_produces_unresolved_reference`, `req_054_backtick_id_known_no_finding_unknown_produces_unresolved`, `req_054_backtick_id_at_line_start_detected`, `req_054_two_backtick_ids_on_one_line_both_reported`, `req_054_backtick_non_id_not_reported_as_unresolved`, `req_054_property_statement_backtick_id_produces_unresolved`, `req_054_marker_to_known_id_is_not_unresolved`, `req_054_marker_to_unknown_id_is_unresolved`, `req_072_non_query_language_checks_invalid_marker`, `req_054_non_query_language_checks_unresolved_reference`, `req_054_marker_unresolved_reference_reports_fn_line` |
| REQ-055 | review | `src/ir.rs` に EARS の型検査がないことを確認。`grep -c "EARS" src/ir.rs` が 0 |
| REQ-056 | review | `src/ir.rs` に矛盾の読みの数の検査がないことを確認 |
| REQ-057 | unit | `req_057_source_splits_at_first_hash_and_allows_commas`, `req_060_glossary_trailing_comma_does_not_create_empty_source` |
| REQ-058 | unit | `req_058_number_anchor_looks_for_decision_line_and_other_anchor_for_heading`, `req_058_source_outside_places_is_invalid`, `req_058_source_path_equal_to_a_place_itself_is_invalid_without_crashing`, `tbl_012_decision_line_with_and_without_trailing_text`, `tbl_012_is_decision_number_rejects_invalid_forms`, `tbl_012_check_source_outside_records_and_adr_returns_err` |
| REQ-059 | unit | `req_053_tag_with_empty_value_is_treated_as_missing`, `req_059_scenario_without_id_missing_source_detail_is_scenario_text`, `req_059_missing_source_for_item_scenario_and_term`, `req_059_empty_source_value_produces_missing_source` |
| REQ-060 | unit | `req_060_glossary_trailing_comma_does_not_create_empty_source`, `req_060_glossary_and_scenario_sources_are_checked` |
| REQ-061 | unit | `req_061_numbers_are_per_file` |
| REQ-062 | review | `src/sources.rs` で出典の内容照合をしていないことを確認。パスと番号/見出しの存在だけ検査 |
| REQ-063 | unit | `req_063_only_sentences_and_steps_are_checked`, `req_063_property_statements_and_scenario_steps_are_term_checked` |
| REQ-064 | unit | `req_064_unknown_term`, `req_063_property_statements_and_scenario_steps_are_term_checked` |
| REQ-065 | unit | `req_065_ids_pass_without_glossary` |
| REQ-066 | unit | `req_066_vague_word_substring` |
| REQ-067 | unit | `req_067_one_finding_per_occurrence` |
| REQ-068 | review | `src/terms.rs` に囲み忘れの検出がないことを確認 |
| REQ-069 | unit | `req_069_reference_needs_boundary_and_quotes_are_skipped`, `req_069_quoted_text_ending_in_a_multibyte_character_is_split_at_the_quote`, `req_069_dot_md_at_the_start_of_a_line_is_skipped_and_scanning_continues`, `req_069_doc_ref_line_number_is_correct`, `req_069_doc_ref_at_line_end`, `req_069_doc_ref_at_line_start`, `req_069_doc_ref_after_punctuation`, `req_069_mdx_extension_not_matched_but_md_after_it_is` |
| REQ-070 | unit | `req_070_missing_document` |
| REQ-071 | unit | `req_071_marker_syntax_allows_spaces_around_commas`, `req_071_marker_line_number_is_reported` |
| REQ-072 | unit | `req_072_invalid_marker`, `req_072_invalid_marker_detail_is_line_text`, `req_072_invalid_marker_outside_test_is_ignored`, `req_072_invalid_marker_second_on_line_is_detected`, `req_072_non_query_language_checks_invalid_marker` |
| REQ-073 | unit | `req_072_invalid_marker_second_on_line_is_detected`, `req_073_several_markers_on_one_line` |
| REQ-074 | unit | `req_074_marker_anywhere_in_the_line_regardless_of_comment_syntax` |
| REQ-075 | unit | `req_075_marker_before_attributes_binds`, `req_075_marker_at_body_start_binds`, `req_075_both_places_merge_ids`, `req_075_marker_in_body_middle_is_ignored`, `tbl_016_blank_line_between_marker_and_test_breaks_binding`, `tbl_016_macro_function_boundary_breaks_marker_binding` |
| REQ-076 | unit | `req_076_unknown_language_scans_raw_text` |
| REQ-077 | unit | `req_077_unresolved_only_marker_still_counts`, `req_054_marker_to_known_id_is_not_unresolved`, `req_077_malformed_id_in_marker_is_unresolved_reference_rs`, `req_077_malformed_id_in_marker_is_unresolved_reference_non_rs` |
| REQ-078 | unit | `req_078_marker_to_review_requirement_is_not_an_error` |
| REQ-079 | unit | `req_079_reads_files_matching_the_globs`, `req_079_symlink_is_not_followed` |
| REQ-080 | unit | `req_080_uses_tree_sitter_with_bundled_rust_query` |
| REQ-081 | unit | `req_081_only_rs_maps_to_rust` |
| REQ-082 | unit | `req_082_test_attribute_is_always_counted`, `req_082_configured_attribute_matches_path_with_arguments`, `req_082_macro_body_functions_are_counted_by_last_segment`, `req_082_test_attribute_after_comment_is_recognized`, `req_082_function_without_configured_attribute_not_counted` |
| REQ-083 | unit | `req_083_unparsable_file_is_skipped` |
| REQ-084 | review | `src/tests_discovery.rs` に正規表現によるテスト検出がないことを確認。tree-sitter のみ使用 |
| REQ-085 | unit | `req_085_fixture_has_no_uncovered_requirement` |
| REQ-086 | unit | `req_086_fixture_has_no_unmarked_test_and_one_after_removal` |
| REQ-087 | unit | `req_087_unknown_language_only_feeds_coverage` |
| REQ-088 | unit | `req_088_empty_ir_still_checks_tests` |
| REQ-089 | review | `src/ir.rs` にコードで固定した形で IR を読むことを確認。parse_document 関数 |
| REQ-090 | review | スキーマファイルの読み込みがないことを確認。`grep -c "schema" src/` が 0 |
| REQ-091 | review | mdschema の使用がないことを確認。`grep -c "mdschema" src/` が 0 |
| REQ-092 | review | 判断の記録と ADR の両方のディレクトリを読むことを `src/sources.rs` で確認 |
| REQ-093 | review | 判断の記録の検査は出典の存在確認のみで、ファイルの上書きはしない |
| REQ-094 | review | ADR の節の検査は出典のための見出し照合のみ。5節の構造検査はしない |
| REQ-095 | review | ADR の決定の節の検査は出典のための見出し照合のみ |
| REQ-096 | review | kotowari は ADR だけの運用を禁止する検査をしない（設定に両方のパスが必要） |
| REQ-097 | review | kotowari は判断の記録だけの運用を禁止する検査をしない（同上） |
| REQ-098 | unit | `req_098_missing_field`, `req_098_required_lines_are_told_apart_from_empty_values`, `req_098_flag_without_relation_line_produces_missing_field`, `req_098_tbl_with_valid_source_no_missing_source`, `req_059_missing_source_for_item_scenario_and_term` |
| REQ-099 | unit | `req_099_missing_table` |
| REQ-100 | unit | `req_100_scenario_outside_gherkin_is_ignored` |
| REQ-101 | review | CLI の出力は JSON/text で LLM が読みやすい形。`src/main.rs` を確認 |
| REQ-102 | review | 標準出力と標準エラー以外に書き出さないことを確認。`src/` に `File::create`、`fs::write`、`OpenOptions` が無い。`write!` は `StopReason` の表示（`fmt::Formatter` 向け）にだけ使われ、ファイルには書かない |
| REQ-103 | review | ADR のファイル名形式は出典の検査時に見るが、形式自体は検査しない |
| REQ-104 | unit | `req_104_quoted_values_are_not_terms` |
| REQ-105 | review | `src/` にライブラリとバイナリの2ターゲット。モジュールは config, ir, sources, terms, tests_discovery |
| REQ-106 | unit | `req_106_form_contract_headings_are_valid_sources` |

## ミューテーションテスト

走らせる形は `CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true CARGO_BUILD_JOBS=4 cargo +nightly mutants -j 1 --no-config`（メモリの上限つきのサービスの中で。変異で無限ループになった子プロセスが孤児として残るので、60秒以上生きているものを殺す見張りを付ける）。安定版の cargo はファイルの更新時刻で再ビルドの要否を決めるため、変異を書き込んでも再コンパイルされず「見逃し」と記録される変異が混ざる。nightly の内容ハッシュによる判定で、すべての変異が再コンパイルされることをログ（`Compiling kotowari`）で確かめている。`.cargo/mutants.toml` は使わない（`--no-config`）。

| 時点 | 変異 | 殺した | 見逃し | ビルド不能 | 打ち切り |
|---|---|---|---|---|---|
| 8周目の後（`5ea4cca`、基準） | 503 | 365 | 113 | 16 | 9 |
| 9周目のテスト追加（1回目）の後（`0914705`） | 495 | 400 | 67 | 16 | 12 |
| レビューの指摘の修正の後（`140dee7`） | 495 | 403 | 65 | 16 | 11 |
| テスト追加（2回目、5件）の後（`07e4102`） | 495 | 407 | 60 | 16 | 12 |
| A96〜A97 の実装と到達不能コードの除去の後（`46adc55`、最終） | 502 | 413 | 59 | 16 | 14 |

打ち切りは、変異で無限ループになったものをテストの打ち切り（20秒）で止めた数。殺した側に数える。打ち切りになる変異は実行ごとに1〜2件ぶれる（メモリの上限に当たって先に殺されると caught に入る）。

`0914705` の見逃し67件は、1件ずつ差分を読んで次の3つに分けた（分類の記録は `experiments/003-cli/mutation/missed_classification.json`、差分は同じ場所の `missed_bundle.md`）。元のコードの方が仕様と食い違う疑い（defect_candidate）は0件だった。

| 分類 | `0914705` の時点 | 最終（`46adc55`）で残るもの |
|---|---|---|
| 等価（どんな入力でも振る舞いが変わらない） | 25 | 24 |
| 未検査（振る舞いが変わる入力があるが、テストに無い） | 42 | 35 |
| 元のコードの欠陥の疑い | 0 | 0 |

`0914705` から最終までに消えた見逃し8件: M01, M21, M26, M33, M41, M55, M56, M67。うち5件（M01、M26、M33、M41、M67）は、変異後の振る舞いが目立って壊れるのに通ってしまうテストの穴として、2回目のテスト追加で塞いだ。M21（性質の定義から参照を解く分岐）は到達不能コードとして除いたので変異そのものが無くなった。残る2件（M55, M56）はレビューの指摘の修正（弱いテストの正の断言、ADR の置き場が読めないときのテスト）で副次的に殺れた。

### 等価な変更（24件。M21 は到達不能コードの除去で変異が無くなった）

- **split_lines（ir.rs）**（6件: M02, M03, M04, M05, M06, M07）: 行末の `\r` を落とす処理の境界の変異。行を受け取る側はすべて `trim()` してから使うか、先頭の文字だけを見る（` ``` `、`|`、`.md` の探索）ので、`\r` が残っても結果は変わらない。
  - `ir.rs:173` replace > with ==
  - `ir.rs:173` replace > with <
  - `ir.rs:173` replace > with >=
  - `ir.rs:173` replace - with /
  - `ir.rs:174` replace - with +
  - `ir.rs:174` replace - with /
- **check_documents（ir.rs）**（1件: M13）: `> 1` を `>= 1` に変えても、続く繰り返しは `locations[1..]` を回すので、要素が1つのときは何も起きない。どちらでも指摘は出ない。
  - `ir.rs:836` replace > with >=
- **check_item（ir.rs）**（3件: M14, M15, M17）: `kind.is_none()` と「`種類` の行を見ていない」は、`ItemBuilder::build` の作りから常に同じ真偽になる。片方の条件を変えても結果は同じ。
  - `ir.rs:883` replace && with ||
  - `ir.rs:903` replace && with ||
  - `ir.rs:1123` replace && with ||
- **check_references（ir.rs）**（1件: M21）: 性質（PROP）の `definitions` は A87 の反映で常に空になったので、この分岐の中は到達しない。到達不能コードとして残っている（レビューの指摘でもある）。
  - `ir.rs:1277` delete !
- **SourceContext::check_source（sources.rs）**（1件: M27）: `records_files` に入るのは `is_records` が真のものだけなので、偽の側の分岐は到達しない。
  - `sources.rs:160` replace == with !=
- **split_outside_quotes（terms.rs）**（2件: M34, M36）: 引用符の後ろの断片の開始位置が1文字ずれて閉じ引用符が含まれるだけ。引用符は文書名の参照の境界の文字なので、参照の判定は変わらない。
  - `terms.rs:153` replace + with *
  - `terms.rs:157` replace < with <=
- **find_doc_refs（terms.rs）**（1件: M37）: `i == len` で1回余分に回っても、空の断片に `.md` は見つからずすぐ抜ける。
  - `terms.rs:177` replace < with <=
- **parse_markers_in_line（tests_discovery.rs）**（4件: M42, M43, M44, M45）: 印の `raw` フィールドの計算だけに影響する。`raw` はどこからも読まれていない（未使用フィールド）。
  - `tests_discovery.rs:46` replace + with -
  - `tests_discovery.rs:46` replace + with *
  - `tests_discovery.rs:52` replace + with -
  - `tests_discovery.rs:52` replace + with *
- **has_attribute（tests_discovery.rs）**（4件: M51, M52, M53, M54）: 関数ノードの直接の子を見る繰り返しの中の変異。tree-sitter の Rust 文法では `#[test]` は関数ノードの子ではなく兄弟なので、この繰り返しは何も見つけない。到達しても結果に影響しない。
  - `tests_discovery.rs:320` replace || with &&
  - `tests_discovery.rs:320` replace == with !=
  - `tests_discovery.rs:320` replace == with !=
  - `tests_discovery.rs:322` replace == with !=
- **collect_markers_before_line（tests_discovery.rs）**（2件: M62, M63）: `line_num` は捨てられる `Marker.line` の刻印にしか使われず、ID の抽出は行の文字列だけで決まる。
  - `tests_discovery.rs:449` replace + with -
  - `tests_discovery.rs:449` replace + with *

### 未検査の変更（残り35件）

振る舞いが変わる入力は存在するが、今のテストにその入力が無いもの。「殺す手間」は、low が既存テストに断言を1つ足す程度、medium が新しい入力のテストを1本書く程度。計画のステップ7が許すテストの追加は2回までで、1回目（9周目、37テスト）で113件を67件に、2回目（5テスト）で目立つ穴5件を塞いだ。残りは記録して残す（LOG の D-21）。

| 関数 | 件数 | 殺す手間 | 変異 |
|---|---|---|---|
| `parse_document` | 4 | medium 4 | M08 `ir.rs:311` delete !; M09 `ir.rs:313` replace && with ||; M10 `ir.rs:313` replace && with ||; M11 `ir.rs:320` replace == with != |
| `ItemBuilder::build` | 1 | low 1 | M12 `ir.rs:557` replace == with != |
| `check_item` | 4 | medium 4 | M18 `ir.rs:1123` delete !; M19 `ir.rs:1123` replace == with !=; M20 `ir.rs:1132` replace && with ||; M16 `ir.rs:1095` replace && with || |
| `check_backtick_ids` | 1 | medium 1 | M22 `ir.rs:1352` replace + with * |
| `split_source` | 1 | medium 1 | M23 `sources.rs:104` replace || with && |
| `SourceContext::check_source` | 4 | medium 4 | M24 `sources.rs:134` replace && with ||; M25 `sources.rs:137` replace && with ||; M28 `sources.rs:172` replace == with !=; M29 `sources.rs:187` replace == with != |
| `load_all_md` | 1 | medium 1 | M30 `sources.rs:257` replace && with || |
| `load_all_md_as_other` | 1 | medium 1 | M31 `sources.rs:303` replace && with || |
| `check_unknown_terms` | 1 | low 1 | M32 `terms.rs:68` replace + with * |
| `split_outside_quotes` | 1 | medium 1 | M35 `terms.rs:157` replace && with || |
| `find_doc_refs` | 3 | medium 3 | M38 `terms.rs:183` replace + with *; M39 `terms.rs:186` replace || with &&; M40 `terms.rs:203` replace < with <= |
| `discover_tests_in_node` | 2 | medium 2 | M46 `tests_discovery.rs:196` replace + with -; M47 `tests_discovery.rs:196` replace + with * |
| `discover_macro_functions` | 3 | medium 3 | M48 `tests_discovery.rs:247` replace + with -; M49 `tests_discovery.rs:247` replace + with *; M50 `tests_discovery.rs:247` replace + with * |
| `has_attribute` | 1 | medium 1 | M57 `tests_discovery.rs:336` replace != with == |
| `has_configured_attribute` | 3 | medium 3 | M58 `tests_discovery.rs:366` replace && with ||; M59 `tests_discovery.rs:366` replace != with ==; M60 `tests_discovery.rs:366` replace != with == |
| `collect_markers_from_siblings` | 1 | low 1 | M61 `tests_discovery.rs:389` replace + with * |
| `collect_body_start_markers` | 2 | medium 2 | M64 `tests_discovery.rs:486` replace + with -; M65 `tests_discovery.rs:486` replace + with * |
| `discover_and_check` | 1 | low 1 | M66 `tests_discovery.rs:576` replace + with * |

## kotowari 自身にかけた結果

リポジトリ直下（`docs/ir` とこのリポジトリの `tests/`）で `cargo run -- check` を実行した結果（`f4a0340`）:

- files: 20
- lines: 1401
- findings: 0
- 終了コード: 0
