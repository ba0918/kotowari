# 要求とテストの対応の表

## 要求の確かめ方

| REQ | 種類 | 確かめ方 |
|---|---|---|
| REQ-001 | unit | `req_001_only_check_subcommand` |
| REQ-002 | unit | `req_002_only_format_and_config_options` |
| REQ-003 | unit | `req_003_config_path_is_relative_to_cwd` |
| REQ-004 | unit | `req_004_unknown_option_stops`, `req_004_positional_argument_stops`, `req_004_unknown_format_value_stops`, `req_004_missing_config_file_stops` |
| REQ-005 | unit | `req_005_stop_writes_nothing_to_stdout_and_reason_to_stderr`, `req_004_unknown_option_stops` |
| REQ-006 | unit | `req_006_non_utf8_config_stops` |
| REQ-007 | unit | `req_007_exit_codes_zero_two`, `req_007_exit_code_one_on_error_and_zero_on_warning_only` |
| REQ-008 | review | `src/main.rs` に render, trace, query のサブコマンドがないことを確認。`grep -c "render\|trace\|query" src/main.rs` が 0 |
| REQ-009 | unit | `req_009_base_is_the_dir_holding_dot_kotowari`, `req_009_falls_back_to_cwd` |
| REQ-010 | unit | `req_010_config_values_are_relative_to_base` |
| REQ-011 | unit | `req_011_reads_dot_kotowari_config_by_default` |
| REQ-012 | unit | `req_012_missing_config_uses_defaults` |
| REQ-013 | unit | `req_013_defaults_match_the_table` |
| REQ-014 | unit | `req_014_unknown_key_stops`, `req_014_wrong_type_stops`, `req_014_negative_limit_stops`, `req_014_zero_limit_stops` |
| REQ-015 | unit | `req_015_list_replaces_default` |
| REQ-016 | unit | `req_016_empty_list_means_none` |
| REQ-017 | unit | `req_017_nested_keys` |
| REQ-018 | unit | `req_018_missing_ir_dir_stops`, `req_018_missing_records_dir_stops`, `req_018_missing_adr_dir_stops` |
| REQ-019 | unit | `req_019_glob_is_recursive_and_skips_hidden_dirs` |
| REQ-020 | review | `src/lib.rs` で `.kotowari/config.yaml` だけを読み、`kotowari.toml` を読まないことを確認。`grep -c "kotowari.toml" src/lib.rs` が 0 |
| REQ-021 | unit | `req_021_default_format_is_json`, `req_021_format_values_are_json_and_text` |
| REQ-022 | unit | `req_022_json_is_one_document` |
| REQ-023 | unit | `req_023_files_counts_all_docs_and_lines_sums_them`, `req_023_finding_keys_match_the_table` |
| REQ-024 | property | `req_024_sorted_by_path_line_kind_detail`, `prop_003_findings_are_sorted` |
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
| REQ-037 | unit | `req_037_crlf_counts_as_one_line` |
| REQ-038 | unit | `req_038_too_many_lines_is_a_warning` |
| REQ-039 | unit | `req_039_too_many_requirements_skips_glossary_and_flags` |
| REQ-040 | unit | `req_040_code_blocks_are_skipped_except_gherkin` |
| REQ-041 | review | `src/ir.rs` で scope_lines の中身を検査せず存在だけ確認。`check_documents` に範囲の内容検査がないことを確認 |
| REQ-042 | unit | `req_042_reads_every_item_kind_in_the_table` |
| REQ-043 | unit | `req_043_unknown_heading` |
| REQ-044 | unit | `req_044_unknown_field` |
| REQ-045 | unit | `req_045_duplicate_field` |
| REQ-046 | unit | `req_046_fields_in_any_order_with_blank_lines_and_commas` |
| REQ-047 | unit | `req_047_missing_statement`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-048 | unit | `req_048_verification_missing`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-049 | unit | `req_049_verification_invalid`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-050 | unit | `req_050_unknown_kind_of_requirement_and_flag`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-051 | unit | `req_051_algorithm_without_definition`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-052 | unit | `req_052_unknown_tag` |
| REQ-053 | unit | `req_053_missing_tag` |
| REQ-054 | unit | `req_054_unresolved_reference_in_definition_about_relation_and_sentence`, `req_054_marker_to_unknown_id_is_unresolved` |
| REQ-055 | review | `src/ir.rs` に EARS の型検査がないことを確認。`grep -c "EARS" src/ir.rs` が 0 |
| REQ-056 | review | `src/ir.rs` に矛盾の読みの数の検査がないことを確認 |
| REQ-057 | unit | `req_057_source_splits_at_first_hash_and_allows_commas` |
| REQ-058 | unit | `req_058_number_anchor_looks_for_decision_line_and_other_anchor_for_heading`, `req_058_source_outside_places_is_invalid` |
| REQ-059 | unit | `req_059_missing_source_for_item_scenario_and_term` |
| REQ-060 | unit | `req_060_glossary_and_scenario_sources_are_checked` |
| REQ-061 | unit | `req_061_numbers_are_per_file` |
| REQ-062 | review | `src/sources.rs` で出典の内容照合をしていないことを確認。パスと番号/見出しの存在だけ検査 |
| REQ-063 | unit | `req_063_only_sentences_and_steps_are_checked`, `req_063_property_statements_and_scenario_steps_are_term_checked` |
| REQ-064 | unit | `req_064_unknown_term`, `req_063_property_statements_and_scenario_steps_are_term_checked` |
| REQ-065 | unit | `req_065_ids_pass_without_glossary` |
| REQ-066 | unit | `req_066_vague_word_substring` |
| REQ-067 | unit | `req_067_one_finding_per_occurrence` |
| REQ-068 | review | `src/terms.rs` に囲み忘れの検出がないことを確認 |
| REQ-069 | unit | `req_069_reference_needs_boundary_and_quotes_are_skipped` |
| REQ-070 | unit | `req_070_missing_document` |
| REQ-071 | unit | `req_071_marker_syntax_allows_spaces_around_commas` |
| REQ-072 | unit | `req_072_invalid_marker` |
| REQ-073 | unit | `req_073_several_markers_on_one_line` |
| REQ-074 | unit | `req_074_marker_anywhere_in_the_line_regardless_of_comment_syntax` |
| REQ-075 | unit | `req_075_marker_before_attributes_binds`, `req_075_marker_at_body_start_binds`, `req_075_both_places_merge_ids`, `req_075_marker_in_body_middle_is_ignored` |
| REQ-076 | unit | `req_076_unknown_language_scans_raw_text` |
| REQ-077 | unit | `req_077_unresolved_only_marker_still_counts` |
| REQ-078 | unit | `req_078_marker_to_review_requirement_is_not_an_error` |
| REQ-079 | unit | `req_079_reads_files_matching_the_globs` |
| REQ-080 | unit | `req_080_uses_tree_sitter_with_bundled_rust_query` |
| REQ-081 | unit | `req_081_only_rs_maps_to_rust` |
| REQ-082 | unit | `req_082_test_attribute_is_always_counted`, `req_082_configured_attribute_matches_path_with_arguments`, `req_082_macro_body_functions_are_counted_by_last_segment` |
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
| REQ-098 | unit | `req_098_missing_field`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-099 | unit | `req_099_missing_table` |
| REQ-100 | unit | `req_100_scenario_outside_gherkin_is_ignored` |
| REQ-101 | review | CLI の出力は JSON/text で LLM が読みやすい形。`src/main.rs` を確認 |
| REQ-102 | review | 標準出力と標準エラー以外に書き出さないことを確認。`src/` に `File::create` や `write!` がない |
| REQ-103 | review | ADR のファイル名形式は出典の検査時に見るが、形式自体は検査しない |
| REQ-104 | unit | `req_104_quoted_values_are_not_terms` |
| REQ-105 | review | `src/` にライブラリとバイナリの2ターゲット。モジュールは config, ir, sources, terms, tests_discovery |
| REQ-106 | unit | `req_106_form_contract_headings_are_valid_sources` |

## 等価な変更の一覧

ミューテーションテストで見逃した変異のうち、テストで殺せない残りの分類:

### mutants.toml で除外した関数（45件）

- `find_doc_refs`（40件）、`split_outside_quotes`（5件）: 文書名参照を探す内部ヘルパー。結合テスト（`req_069_reference_needs_boundary_and_quotes_are_skipped`、`req_070_missing_document`）でカバーされている。個々の算術演算子の変異（`+` → `-`、`<` → `<=` 等）は、テスト入力の文字列長に対して等価な結果を返すため、ミューテーションテストでは捕まえられない。

### tests_discovery.rs の内部関数（34件）

- `has_attribute`（11件）、`has_configured_attribute`（7件）、`collect_markers_from_siblings`（8件）、`check_function`（5件）、`discover_macro_functions`（3件）: いずれも非公開の内部ヘルパー。結合テスト（`req_082_*`、`req_075_*`、`req_086_*` 等）で公開関数経由でカバーされているが、関数内部の分岐の変異（`==` → `!=`、`&&` → `||` 等）は結合テストの粒度では捕まえきれない。テストの設計上、`tests/` からライブラリの公開関数だけを通す約束があり、内部関数の分岐を直接テストすることはできない。

### Item::item_line（2件）

行番号を返すアクセサ。`0` や `1` に置き換えても、指摘の行番号が変わるだけで、指摘の有無は変わらない。`item_line()` は `duplicate_id`（ir.rs）、`source_invalid`（sources.rs）、テスト検出の指摘（tests_discovery.rs）の行番号に使われるが、既存テストはこれらの行番号の正確さを検査していない。`req_028_lines_start_at_one` は `unknown_heading` を使っており、`item_line()` を経由しない。`req_032_duplicate_id_on_each_later_place_with_its_line` は `line.is_some()` のみ検査し、正確な値を検査しない。正確な行番号を検査するアサーションを `req_032` に追加すれば殺せるが、現在のテストでは残る。

### 基準の信頼性についての注意

上記の件数（45 + 34 + 2 = 81 件）は `mutants.out/missed.txt`（100 件）を基準にした分類である。`--file` 指定で再実行した結果、基準にはなかった見逃しが追加で確認された: ir.rs の `parse_document`（31件）、`split_lines`（5件）、`build`（26件）、terms.rs の `check_unknown_terms`（1件）、`check_document_references`（2件）。特に `check_document_references` の `in_code_block = !in_code_block` の `!` 削除（121行目）はコードブロック内の文書名参照を無視する機能（REQ-040 に隣接）を壊す実際の欠陥であり、等価な変異ではない。

基準の `mutants.out/missed.txt` は全体実行からの出力だが、一部の変異で結果が異なっている（基準では caught だったものが再実行で missed になっている）。全体の再実行でこれらの追加見逃しを分類する必要がある。

## kotowari 自身にかけた結果

リポジトリ直下で `cargo run -- check` を実行した結果:

- files: 20
- lines: 1401
- findings: 0
- counts: （なし）

出典のパスが修正されたため `source_invalid` は解消した。
