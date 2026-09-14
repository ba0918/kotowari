# 要求とテストの対応の表

この表は、`docs/ir` の要求と、それを確かめるテスト（`tests/*.rs` の `// @kotowari[ID]` の印で結び付いた関数名）の対応を記す。unit と property の行はテストの印から機械的に作り、review の行は確認の手順を手で書く。

## 要求の確かめ方

| REQ | 種類 | 確かめ方 |
|---|---|---|
| REQ-001 | unit | `req_001_only_check_subcommand` |
| REQ-002 | unit | `req_002_only_format_and_config_options`, `req_107_help_and_version_exit_zero_without_check`, `req_002_options_before_or_after_check` |
| REQ-003 | unit | `req_003_config_path_is_relative_to_cwd` |
| REQ-004 | unit | `req_004_unknown_option_stops`, `req_004_positional_argument_stops`, `req_004_unknown_format_value_stops`, `req_004_missing_config_file_stops`, `req_107_help_wins_over_argument_errors`, `req_004_no_arguments_stops`, `req_004_option_without_value_stops`, `req_004_repeated_option_stops`, `req_004_config_pointing_to_directory_stops` |
| REQ-005 | unit | `req_004_unknown_option_stops`, `req_004_positional_argument_stops`, `req_005_stop_writes_nothing_to_stdout_and_reason_to_stderr`, `req_004_no_arguments_stops`, `req_004_option_without_value_stops`, `req_004_repeated_option_stops`, `req_004_config_pointing_to_directory_stops`, `req_005_stderr_first_line_has_the_reason_wording`, `req_005_config_error_detail_path_is_relative_to_base_not_to_cwd`, `req_005_stderr_detail_path_is_relative`, `req_005_config_outside_the_base_is_shown_relative_with_parent_segments`, `req_005_stderr_carries_the_stop_reason_text` |
| REQ-006 | unit | `req_006_non_utf8_config_stops`, `tbl_001_non_utf8_records_or_adr_stops`, `tbl_001_unreadable_test_file_stops`, `tbl_001_non_utf8_test_file_stops` |
| REQ-007 | unit | `req_004_unknown_option_stops`, `req_007_exit_codes_zero_two`, `req_007_exit_code_one_on_error_and_zero_on_warning_only` |
| REQ-008 | review | `src/main.rs` に render, trace, query のサブコマンドがないことを確認。`grep -c "render\|trace\|query" src/main.rs` が 0 |
| REQ-009 | unit | `req_009_base_is_the_dir_holding_dot_kotowari`, `req_009_falls_back_to_cwd` |
| REQ-010 | unit | `req_010_config_values_are_relative_to_base` |
| REQ-011 | unit | `req_011_reads_dot_kotowari_config_by_default` |
| REQ-012 | unit | `req_012_missing_config_uses_defaults`, `req_012_empty_config_uses_defaults`, `req_012_comment_only_config_uses_defaults` |
| REQ-013 | unit | `req_013_defaults_match_the_table` |
| REQ-014 | unit | `req_014_unknown_key_stops`, `req_014_wrong_type_stops`, `req_014_negative_limit_stops`, `req_014_zero_limit_stops`, `req_014_empty_vague_word_stops`, `req_014_unparsable_yaml_stops`, `req_014_duplicate_key_stops`, `req_014_null_value_stops`, `req_014_null_tests_files_stops`, `req_014_null_tests_rust_attributes_stops`, `req_014_null_tests_rust_macros_stops`, `req_014_null_vague_words_key_stops`, `req_014_null_decisions_stops`, `req_014_null_tests_stops`, `req_014_null_tests_rust_stops`, `req_014_null_limits_stops`, `req_014_windows_drive_letter_like_path_is_not_absolute`, `req_014_absolute_path_stops`, `req_014_duplicate_vague_word_stops`, `req_014_invalid_glob_stops`, `req_014_empty_vague_word_stops_with_config_error` |
| REQ-015 | unit | `req_015_list_replaces_default` |
| REQ-016 | unit | `req_016_empty_list_means_none` |
| REQ-017 | unit | `req_017_nested_keys` |
| REQ-018 | unit | `req_018_unreadable_dir_stops`, `req_018_unreadable_records_dir_stops`, `req_018_unreadable_adr_dir_stops`, `req_018_missing_ir_dir_stops`, `req_005_stderr_carries_the_stop_reason_text`, `req_018_missing_records_dir_stops`, `req_018_missing_adr_dir_stops`, `req_018_place_that_is_a_file_stops`, `req_018_broken_symlink_in_records_dir_stops`, `req_033_file_symlink_in_records_dir_is_read`, `req_018_unreadable_directory_under_tests_stops`, `req_079_broken_symlink_in_tests_stops`, `req_079_broken_symlink_outside_glob_stops` |
| REQ-019 | unit | `req_019_glob_is_recursive_and_skips_hidden_dirs`, `req_019_hidden_directory_is_excluded_and_subdirectory_is_included`, `req_019_hidden_file_matched_by_glob_is_read` |
| REQ-020 | review | `src/lib.rs` で `.kotowari/config.yaml` だけを読み、`kotowari.toml` を読まないことを確認。`grep -c "kotowari.toml" src/lib.rs` が 0 |
| REQ-021 | unit | `req_021_default_format_is_json`, `req_021_format_values_are_json_and_text` |
| REQ-022 | unit | `req_022_json_is_one_document` |
| REQ-023 | unit | `req_023_files_counts_all_docs_and_lines_sums_them`, `req_023_finding_keys_match_the_table` |
| REQ-024 | property | `req_024_sorted_by_path_line_kind_detail`, `prop_003_findings_are_sorted`（PROP-003 の印） |
| REQ-025 | unit | `req_025_text_has_one_line_per_finding_with_bracketed_severity` |
| REQ-026 | unit | `req_026_null_line_prints_dash` |
| REQ-027 | unit | `req_027_document_wide_findings_have_null_line`, `req_027_glossary_invalid_has_null_line` |
| REQ-028 | unit | `req_028_lines_start_at_one` |
| REQ-029 | unit | `req_029_every_error_kind_has_the_detail_of_the_table` |
| REQ-030 | unit | `req_030_warning_kinds_have_the_detail_of_the_table` |
| REQ-031 | unit | `req_031_only_two_kinds_are_warnings` |
| REQ-032 | unit | `req_032_duplicate_id_on_each_later_place_with_its_line`, `req_032_first_occurrence_is_bytewise_first_path` |
| REQ-033 | unit | `req_033_only_direct_children`, `req_033_uppercase_md_is_not_read`, `req_033_file_symlink_is_read`, `req_033_broken_symlink_in_ir_dir_stops`, `req_033_file_symlink_in_records_dir_is_read` |
| REQ-034 | unit | `req_034_missing_title`, `req_034_lines_before_title_are_ignored` |
| REQ-035 | unit | `req_035_multiple_titles` |
| REQ-036 | unit | `req_036_missing_scope`, `req_036_glossary_and_flags_need_no_scope` |
| REQ-037 | unit | `req_037_crlf_counts_as_one_line`, `req_037_crlf_title_and_requirement_line_numbers`, `req_037_empty_content_has_zero_lines` |
| REQ-038 | unit | `req_038_too_many_lines_is_a_warning`, `req_038_exactly_at_limit_no_warning_one_over_warns` |
| REQ-039 | unit | `req_039_too_many_requirements_skips_glossary_and_flags`, `req_039_unknown_heading_does_not_inflate_requirement_count` |
| REQ-040 | unit | `req_040_code_blocks_are_skipped_except_gherkin`, `req_040_tilde_fence_is_a_code_block`, `req_040_longer_fence_needs_same_or_longer_close`, `req_112_unclosed_gherkin_block_with_multiple_scenarios_excludes_items`, `req_040_gherkin_code_block_doc_ref_is_not_checked` |
| REQ-041 | review | `src/ir.rs` で scope_lines の中身を検査せず存在だけ確認。`check_documents` に範囲の内容検査がないことを確認 |
| REQ-042 | unit | `req_042_reads_every_item_kind_in_the_table`, `req_053_consecutive_scenarios_without_tags_each_get_missing_tag`, `req_042_gherkin_all_step_keywords_recognized`, `req_042_glossary_table_parses_terms`, `req_042_flag_relation_and_source_fields_read`, `req_042_scenario_source_tag_parsed_into_sources` |
| REQ-043 | unit | `req_043_unknown_heading`, `req_043_unknown_heading_detail_is_full_heading_text`, `req_043_unknown_heading_invalid_id_detail_is_full_heading_text`, `req_043_heading_without_colon_is_unknown`, `req_043_deeper_heading_is_unknown_heading`, `req_043_lines_under_unknown_heading_are_not_an_item`, `req_043_valid_prefix_invalid_digits_is_not_an_item` |
| REQ-044 | unit | `req_044_unknown_field`, `req_044_unknown_field_without_colon_has_line_text_as_detail`, `req_044_property_definition_field_is_unknown`, `req_044_prop_unknown_field_does_not_produce_unresolved_reference`, `req_044_star_plus_numbered_and_bare_dash_lines_are_unknown_fields`, `req_044_detail_is_the_raw_line`, `req_044_unknown_field_detail_is_raw_line_not_reconstructed` |
| REQ-045 | unit | `req_045_duplicate_field`, `req_045_third_known_line_gives_two_duplicates`, `req_045_unknown_line_repeated_gives_only_unknown_field` |
| REQ-046 | unit | `req_046_fields_in_any_order_with_blank_lines_and_commas` |
| REQ-047 | unit | `req_047_missing_statement`, `req_098_required_lines_are_told_apart_from_empty_values`, `req_047_requirement_without_kind_line_needs_statement` |
| REQ-048 | unit | `req_048_verification_missing`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-049 | unit | `req_049_verification_invalid`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-050 | unit | `req_050_unknown_kind_of_requirement_and_flag`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-051 | unit | `req_051_algorithm_without_definition`, `req_051_algorithm_definition_must_point_to_tbl_or_prop`, `req_098_required_lines_are_told_apart_from_empty_values` |
| REQ-052 | unit | `req_052_unknown_tag`, `req_052_bare_tag_without_equals_is_unknown`, `req_052_unbound_tag_line_is_still_checked`, `req_052_word_without_at_in_tag_line_is_unknown_tag`, `req_052_word_with_equals_not_starting_with_at_keeps_full_word_as_detail` |
| REQ-053 | unit | `req_053_consecutive_scenarios_without_tags_each_get_missing_tag`, `req_053_missing_tag`, `req_053_tag_with_empty_value_is_treated_as_missing`, `gherkin_tags_cleared_after_block_without_scenario` |
| REQ-054 | unit | `req_054_unresolved_reference_in_definition_about_relation_and_sentence`, `req_054_flag_relation_to_unknown_id_produces_unresolved_reference`, `req_054_backtick_id_known_no_finding_unknown_produces_unresolved`, `req_054_backtick_id_at_line_start_detected`, `req_054_two_backtick_ids_on_one_line_both_reported`, `req_054_backtick_non_id_not_reported_as_unresolved`, `req_054_property_statement_backtick_id_produces_unresolved`, `req_054_non_id_definition_value_is_unresolved`, `req_054_backtick_id_in_step_is_checked`, `req_054_backtick_id_inside_double_quotes_is_not_checked`, `req_054_backtick_oddness_counted_outside_quotes_only`, `req_054_marker_to_known_id_is_not_unresolved`, `req_054_marker_to_unknown_id_is_unresolved`, `req_072_non_query_language_checks_invalid_marker`, `req_054_non_query_language_checks_unresolved_reference`, `req_054_duplicate_marker_id_in_same_test_reports_per_occurrence` |
| REQ-055 | review | `src/ir.rs` に EARS の型検査がないことを確認。`grep -c "EARS" src/ir.rs` が 0 |
| REQ-056 | review | `src/ir.rs` に矛盾の読みの数の検査がないことを確認 |
| REQ-057 | unit | `req_057_source_splits_at_first_hash_and_allows_commas`, `req_060_glossary_trailing_comma_does_not_create_empty_source` |
| REQ-058 | unit | `req_058_number_anchor_looks_for_decision_line_and_other_anchor_for_heading`, `req_058_source_outside_places_is_invalid`, `req_058_source_path_equal_to_a_place_itself_is_invalid_without_crashing`, `tbl_012_decision_line_with_and_without_trailing_text`, `tbl_012_indented_decision_line_counts`, `req_058_hidden_directory_under_records_is_not_a_source_target`, `tbl_012_is_decision_number_rejects_invalid_forms`, `tbl_012_check_source_outside_records_and_adr_returns_err`, `req_058_records_place_dot_resolves_a_source_at_the_base_root` |
| REQ-059 | unit | `req_053_tag_with_empty_value_is_treated_as_missing`, `req_059_scenario_without_id_missing_source_detail_is_scenario_text`, `tbl_008_missing_source_scenario_detail_is_raw_scenario_line`, `req_059_missing_source_for_item_scenario_and_term`, `req_059_empty_source_value_produces_missing_source` |
| REQ-060 | unit | `req_060_glossary_trailing_comma_does_not_create_empty_source`, `req_060_glossary_and_scenario_sources_are_checked` |
| REQ-061 | unit | `req_061_numbers_are_per_file`, `req_061_subheading_does_not_end_a_section` |
| REQ-062 | review | `src/sources.rs` で出典の内容照合をしていないことを確認。パスと番号/見出しの存在だけ検査 |
| REQ-063 | unit | `req_063_only_sentences_and_steps_are_checked`, `req_063_property_statements_and_scenario_steps_are_term_checked` |
| REQ-064 | unit | `req_064_unknown_term`, `req_063_property_statements_and_scenario_steps_are_term_checked`, `req_064_backtick_content_is_trimmed`, `req_064_empty_backticks_are_unknown_term` |
| REQ-065 | unit | `req_065_ids_pass_without_glossary` |
| REQ-066 | unit | `req_066_vague_word_substring` |
| REQ-067 | unit | `req_067_one_finding_per_occurrence`, `req_067_overlapping_vague_words_longest_match_once` |
| REQ-068 | review | `src/terms.rs` に囲み忘れの検出がないことを確認 |
| REQ-069 | unit | `req_069_reference_needs_boundary_and_quotes_are_skipped`, `req_069_quoted_text_ending_in_a_multibyte_character_is_split_at_the_quote`, `req_069_dot_md_at_the_start_of_a_line_is_skipped_and_scanning_continues`, `req_069_doc_ref_line_number_is_correct`, `req_069_doc_ref_at_line_end`, `req_069_doc_ref_at_line_start`, `req_069_doc_ref_after_punctuation`, `req_069_mdx_extension_not_matched_but_md_after_it_is` |
| REQ-070 | unit | `req_070_missing_document` |
| REQ-071 | unit | `req_071_marker_syntax_allows_spaces_around_commas`, `req_071_marker_line_number_is_reported` |
| REQ-072 | unit | `req_072_invalid_marker`, `req_072_invalid_marker_detail_is_line_text`, `req_072_invalid_marker_outside_test_is_ignored`, `req_072_invalid_marker_second_on_line_is_detected`, `req_072_non_query_language_checks_invalid_marker`, `req_072_marker_spanning_lines_is_invalid`, `req_072_detail_is_the_raw_line` |
| REQ-073 | unit | `req_072_invalid_marker_second_on_line_is_detected`, `req_073_several_markers_on_one_line` |
| REQ-074 | unit | `req_074_marker_anywhere_in_the_line_regardless_of_comment_syntax` |
| REQ-075 | unit | `req_075_marker_before_attributes_binds`, `req_075_marker_at_body_start_binds`, `req_075_both_places_merge_ids`, `req_075_marker_in_body_middle_is_ignored`, `tbl_016_blank_line_between_marker_and_test_breaks_binding`, `tbl_016_macro_function_boundary_breaks_marker_binding` |
| REQ-076 | unit | `req_076_unknown_language_scans_raw_text` |
| REQ-077 | unit | `req_077_unresolved_only_marker_still_counts`, `req_054_marker_to_known_id_is_not_unresolved`, `req_077_malformed_id_in_marker_is_unresolved_reference_rs`, `req_077_malformed_id_in_marker_is_unresolved_reference_non_rs` |
| REQ-078 | unit | `req_078_marker_to_review_requirement_is_not_an_error` |
| REQ-079 | unit | `req_079_reads_files_matching_the_globs`, `req_079_file_symlink_is_read`, `req_079_broken_symlink_in_tests_stops`, `req_079_broken_symlink_outside_glob_stops` |
| REQ-080 | unit | `req_080_uses_tree_sitter_with_bundled_rust_query` |
| REQ-081 | unit | `req_081_only_rs_maps_to_rust`, `req_081_uppercase_extension_has_no_query` |
| REQ-082 | unit | `req_082_test_attribute_is_always_counted`, `req_082_configured_attribute_matches_path_with_arguments`, `req_082_macro_body_functions_are_counted_by_last_segment`, `req_082_test_attribute_after_comment_is_recognized`, `req_082_function_without_configured_attribute_not_counted` |
| REQ-083 | unit | `req_083_unparsable_file_is_skipped` |
| REQ-084 | review | `src/tests_discovery.rs` に正規表現によるテスト検出がないことを確認。tree-sitter のみ使用 |
| REQ-085 | unit | `req_085_fixture_has_no_uncovered_requirement`, `req_085_requirement_without_verification_line_gets_no_coverage_finding` |
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
| REQ-098 | unit | `req_098_missing_field`, `req_098_required_lines_are_told_apart_from_empty_values`, `req_098_flag_without_relation_line_produces_missing_field`, `req_098_tbl_with_valid_source_no_missing_source`, `req_098_empty_verification_value_is_missing_not_invalid`, `req_098_empty_kind_value_is_missing_field`, `req_098_empty_definition_value_on_algorithm_is_without_definition`, `req_059_missing_source_for_item_scenario_and_term` |
| REQ-099 | unit | `req_099_missing_table` |
| REQ-100 | unit | `req_100_scenario_outside_gherkin_is_ignored`, `req_100_scenario_line_under_heading_is_excluded_from_statement` |
| REQ-101 | review | CLI の出力は JSON/text で LLM が読みやすい形。`src/main.rs` を確認 |
| REQ-102 | review | 標準出力と標準エラー以外に書き出さないことを確認。`src/` に `File::create`、`fs::write`、`OpenOptions` が無い。`write!` は `StopReason` の表示（`fmt::Formatter` 向け）にだけ使われ、ファイルには書かない |
| REQ-103 | review | ADR のファイル名形式は出典の検査時に見るが、形式自体は検査しない |
| REQ-104 | unit | `req_104_quoted_values_are_not_terms` |
| REQ-105 | review | `src/` にライブラリとバイナリの2ターゲット。モジュールは config, ir, sources, terms, tests_discovery |
| REQ-106 | unit | `req_106_form_contract_headings_are_valid_sources` |
| REQ-107 | unit | `req_107_help_and_version_exit_zero_without_check`, `req_107_help_wins_over_argument_errors` |
| REQ-108 | review | `src/lib.rs` の `StopReason::fmt` で TBL-018 の文言を使っていることを確認。`src/main.rs` の `stop` で標準エラーの1行目の形を組んでいることを確認 |
| REQ-109 | review | `src/lib.rs` の `run_check` で置き場の存在を検査し、`StopReason` で停止していることを確認。黙って飛ばす経路が無いことを `rg 'filter_map|if let Ok' src/` で確認 |
| REQ-110 | unit | `req_110_trailing_slash_in_config_is_normalized_in_path`, `req_110_dot_segments_are_folded`, `req_110_dot_alone_normalizes_to_empty_place`, `req_110_ir_dot_produces_bare_filename_path`, `req_058_records_place_dot_resolves_a_source_at_the_base_root` |
| REQ-111 | unit | `req_111_bom_is_skipped_in_ir_config_records_adr_and_tests`, `req_111_bom_in_config_records_adr_and_tests_is_skipped_end_to_end` |
| REQ-112 | unit | `req_112_unclosed_code_block_is_an_error`, `req_112_unclosed_gherkin_block_is_not_checked`, `req_112_unclosed_gherkin_block_with_multiple_scenarios_excludes_items` |
| REQ-113 | unit | `req_113_indented_steps_are_recognized`, `req_113_feature_and_examples_lines_are_invalid`, `req_113_tag_line_binds_only_when_immediately_before_scenario`, `req_113_step_without_preceding_scenario_is_invalid`, `req_113_tag_line_not_immediately_before_scenario_is_invalid`, `req_113_tags_do_not_leak_into_the_next_untagged_scenario`, `req_113_consecutive_steps_without_scenario_report_only_the_first`, `req_113_orphan_step_after_a_blank_line_is_reported_again`, `req_113_step_right_after_tag_line_reports_both_lines`, `req_113_tag_line_right_before_closing_fence_is_invalid` |
| REQ-114 | unit | `req_114_malformed_id_tag_is_invalid_id_and_not_defined`, `req_114_malformed_id_scenario_missing_source_detail_is_scenario_line`, `req_114_malformed_heading_is_not_defined`, `req_114_malformed_id_still_reports_missing_about` |
| REQ-115 | unit | `req_115_source_invalid_line_is_the_source_line` |
| REQ-116 | unit | `req_054_backtick_oddness_counted_outside_quotes_only`, `req_116_odd_backticks_skip_terms_but_check_vague_words`, `tbl_008_unclosed_backtick_detail_keeps_leading_indentation` |
| REQ-117 | unit | `req_117_second_table_is_not_glossary`, `req_117_glossary_without_proper_table_is_invalid`, `req_117_glossary_header_without_rows_is_valid` |
| REQ-118 | unit | `req_118_unresolved_reference_line_is_the_marker_line` |
| REQ-120 | review | `src/ir.rs` と `src/lib.rs` で、仕様に列挙されていない振る舞いを黙って決めていないことを確認。`parse_document` の gherkin 解析で有効な行の種類以外を invalid_gherkin_line にし、`read_utf8_file` で読めないファイルを停止にし、`check_documents` で形に合わない見出しの下を読まないことを確認 |
| REQ-121 | review | `src/config.rs` と `src/tests_discovery.rs` で、設定ファイルから問い合わせ（tree-sitter の文法）を足す経路が無いことを確認。`grep -c "grammar\|Language::new" src/config.rs` が 0 |
| REQ-122 | unit | `req_122_glossary_row_with_missing_column_is_invalid`, `req_122_glossary_row_with_empty_term_cell_is_invalid`, `req_122_two_cell_row_with_trailing_pipe_is_invalid`, `req_122_three_cells_without_trailing_pipe_is_a_term` |
| REQ-123 | unit | `req_123_duplicate_term_reported_for_second_row_onward`, `req_123_duplicate_row_is_not_a_term` |

## ミューテーションテスト

走らせる形は `CARGO_UNSTABLE_CHECKSUM_FRESHNESS=true CARGO_BUILD_JOBS=4 cargo +nightly mutants -j 1 --no-config`（メモリの上限つきのサービスの中で。変異で無限ループになった子プロセスが孤児として残るので、60秒以上生きているものを殺す見張りを付ける）。安定版の cargo はファイルの更新時刻で再ビルドの要否を決めるため、変異を書き込んでも再コンパイルされず「見逃し」と記録される変異が混ざる。nightly の内容ハッシュによる判定で、すべての変異が再コンパイルされることをログ（`Compiling kotowari`）で確かめている。`.cargo/mutants.toml` は使わない（`--no-config`）。

| 時点 | 変異 | 殺した | 見逃し | ビルド不能 | 打ち切り |
|---|---|---|---|---|---|
| 前の段の最終（`46adc55`、`kotowari-check`） | 502 | 413 | 59 | 16 | 14 |
| cycle の1周目と A145〜A149 の反映の後（`b8a8f96`、`mutants-g1/`） | 708 | 459 | 137 | 99 | 13 |
| 再調査の21件の反映の後（`3dbcc22`、`mutants-g2/`） | 677 | 508 | 131 | 25 | 13 |
| 差分のレビューの所見の反映の後（`094d262`、`mutants-g3/`） | 675 | 510 | 127 | 25 | 13 |
| 読まれないコードの除去の後（`9c5161e`、最終、`mutants-g4/`） | 669 | 508 | 123 | 25 | 13 |

打ち切りは、変異で無限ループになったものをテストの打ち切り（20秒）で止めた数。殺した側に数える。ビルド不能が `mutants-g1/` で多いのは、指摘の種類を列挙型にした直後で、その表示と比較の関数への変異が型の制約で通らなかったため。

最終（`9c5161e`）の見逃し123件は、1件ずつ差分を読んで次の3つに分けた（分類の記録は `experiments/003-cli/mutation/missed_classification_general_final.json`。分類は `094d262` の見逃し127件（差分は同じ場所の `missed_bundle_general.md`）に対して Sonnet 5 の調査役2体が実装とテストを読んで書き、実験者が3件を訂正し、差分の本体で最終の実行へ引き継いだ。読まれないコードの除去で4件の変異が消えた）。

| 分類 | 件数 |
|---|---|
| 等価（どんな入力でも振る舞いが変わらない） | 26 |
| 未検査（振る舞いが変わる入力があるが、テストに無い） | 97 |
| 元のコードの欠陥の疑い | 0 |

### 等価な変更（26件）

根拠（その値や分岐がどこで使われ、なぜ結果に影響しないか）は分類の記録にある。

| 関数 | 件数 | 変異 |
|---|---|---|
| `split_outside_quotes` | 4 | M06 `lib.rs:385` replace == with !=; M09 `lib.rs:402` replace + with *; M10 `lib.rs:406` replace && with ||; M11 `lib.rs:406` replace < with <= |
| `Config::parse` | 1 | M15 `config.rs:175` replace || with && |
| `parse_document (#### heading detection)` | 1 | M28 `ir.rs:531` replace > with >= |
| `parse_document (unclosed code block retain)` | 2 | M35 `ir.rs:585` replace < with <=; M37 `ir.rs:588` replace < with <= |
| `check_gherkin_tags_findings` | 2 | M47 `ir.rs:897` replace > with >=; M48 `ir.rs:904` replace > with >= |
| `build_scenario` | 1 | M49 `ir.rs:924` replace match guard !tag_value.is_empty() with true |
| `check_documents` | 1 | M51 `ir.rs:1036` replace > with >= |
| `check_item` | 3 | M52 `ir.rs:1085` replace && with ||; M53 `ir.rs:1093` replace && with ||; M57 `ir.rs:1249` replace && with || |
| `SourceContext::check_source` | 1 | M70 `sources.rs:177` replace == with != |
| `check_vague_words` | 2 | M77 `terms.rs:82` replace < with <=; M78 `terms.rs:93` replace > with >= |
| `find_doc_refs` | 1 | M79 `terms.rs:163` replace < with <= |
| `parse_markers_in_line` | 2 | M85 `tests_discovery.rs:47` replace + with -; M86 `tests_discovery.rs:47` replace + with * |
| `has_attribute` | 3 | M101 `tests_discovery.rs:382` replace || with &&; M102 `tests_discovery.rs:382` replace == with !=; M103 `tests_discovery.rs:382` replace == with != |
| `collect_markers_from_siblings` | 1 | M111 `tests_discovery.rs:456` replace < with <= |
| `collect_body_start_markers` | 1 | M122 `tests_discovery.rs:527` replace < with <= |

### 未検査の変更（97件）

振る舞いが変わる入力は存在するが、今のテストにその入力が無いもの。「殺す手間」は、low が既存テストに断言を1つ足す程度、medium が新しい入力のテストを1本書く程度。入力の例は分類の記録にある。

| 関数 | 件数 | 殺す手間 | 変異 |
|---|---|---|---|
| `Display::fmt for FindingKind` | 1 | low 1 | M01 `lib.rs:112` replace <impl std::fmt::Display for FindingKind>::fmt -> std::fmt::Result with Ok(Default::default()) |
| `PartialEq<str>::eq for FindingKind` | 3 | low 3 | M02 `lib.rs:124` replace <impl PartialEq<str> for FindingKind>::eq -> bool with true; M03 `lib.rs:124` replace <impl PartialEq<str> for FindingKind>::eq -> bool with false; M04 `lib.rs:124` replace == with != in <impl PartialEq<str> for FindingKind>::eq |
| `parse_args` | 1 | low 1 | M05 `lib.rs:272` delete ! |
| `split_outside_quotes` | 2 | medium 2 | M07 `lib.rs:393` replace && with ||; M08 `lib.rs:393` replace == with != |
| `deserialize_nullable_nonzero` | 3 | low 3 | M12 `config.rs:134` replace deserialize_nullable_nonzero -> Result<Option<Option<NonZeroU64>>, D::Error> with Ok(None); M13 `config.rs:134` replace deserialize_nullable_nonzero -> Result<Option<Option<NonZeroU64>>, D::Error> with Ok(Some(None)); M14 `config.rs:134` replace deserialize_nullable_nonzero -> Result<Option<Option<NonZeroU64>>, D::Error> with Ok(Some(Some(1.try_into().unwrap()))) |
| `split_lines` | 6 | medium 6 | M16 `ir.rs:177` replace > with ==; M17 `ir.rs:177` replace > with <; M18 `ir.rs:177` replace > with >=; M19 `ir.rs:177` replace - with /; M20 `ir.rs:178` replace - with +; M21 `ir.rs:178` replace - with / |
| `parse_document (glossary separator detection)` | 1 | medium 1 | M22 `ir.rs:437` replace && with || |
| `parse_document (#### heading detection)` | 9 | low 9 | M23 `ir.rs:531` replace || with &&; M24 `ir.rs:531` replace && with ||; M25 `ir.rs:531` replace && with ||; M26 `ir.rs:531` replace > with ==; M27 `ir.rs:531` replace > with <; M29 `ir.rs:533` replace == with !=; M30 `ir.rs:534` replace && with ||; M31 `ir.rs:534` replace >= with <; M32 `ir.rs:534` replace == with != |
| `parse_document (unclosed code block retain)` | 3 | medium 3 | M33 `ir.rs:585` replace < with ==; M34 `ir.rs:585` replace < with >; M36 `ir.rs:588` replace < with == |
| `is_blankable_field` | 1 | low 1 | M38 `ir.rs:615` replace is_blankable_field -> bool with true |
| `ItemBuilder::add_line` | 2 | low 2 | M39 `ir.rs:660` replace && with ||; M40 `ir.rs:660` replace > with >= |
| `ItemBuilder::build` | 4 | low 4 | M41 `ir.rs:753` replace match guard is_valid_id(&id) with true; M42 `ir.rs:775` replace match guard is_valid_id(&id) with true; M43 `ir.rs:797` replace match guard is_valid_id(&id) with true; M44 `ir.rs:779` replace == with != |
| `check_gherkin_tags_findings` | 2 | low 2 | M45 `ir.rs:897` replace > with ==; M46 `ir.rs:897` replace > with < |
| `build_scenario` | 1 | medium 1 | M50 `ir.rs:926` replace && with || |
| `check_item` | 6 | medium 4, low 2 | M54 `ir.rs:1208` replace && with ||; M55 `ir.rs:1208` replace && with ||; M56 `ir.rs:1224` replace && with ||; M58 `ir.rs:1249` delete !; M59 `ir.rs:1249` replace == with !=; M60 `ir.rs:1252` replace && with || |
| `check_references` | 3 | low 3 | M61 `ir.rs:1337` replace == with !=; M62 `ir.rs:1341` delete !; M63 `ir.rs:1377` replace == with != |
| `extract_backtick_contents` | 1 | low 1 | M66 `ir.rs:1452` replace + with * |
| `split_source` | 1 | low 1 | M67 `sources.rs:113` replace || with && |
| `SourceContext::check_source` | 2 | low 1, medium 1 | M68 `sources.rs:141` replace || with &&; M69 `sources.rs:152` replace >= with < |
| `is_under_place` | 1 | medium 1 | M71 `sources.rs:220` replace && with || |
| `load_all_md` | 1 | medium 1 | M72 `sources.rs:301` replace && with || |
| `load_all_md_as_other` | 1 | medium 1 | M73 `sources.rs:366` replace && with || |
| `check_sources` | 3 | low 3 | M74 `sources.rs:398` replace == with !=; M75 `sources.rs:405` replace == with !=; M76 `sources.rs:412` replace == with != |
| `find_doc_refs` | 3 | medium 1, low 2 | M80 `terms.rs:169` replace + with *; M81 `terms.rs:172` replace || with &&; M82 `terms.rs:189` replace < with <= |
| `discover_tests_in_node` | 2 | medium 2 | M87 `tests_discovery.rs:223` replace + with -; M88 `tests_discovery.rs:223` replace + with * |
| `discover_macro_functions` | 9 | low 9 | M89 `tests_discovery.rs:274` replace + with -; M90 `tests_discovery.rs:274` replace + with *; M91 `tests_discovery.rs:274` replace + with *; M92 `tests_discovery.rs:280` replace + with *; M93 `tests_discovery.rs:286` replace + with *; M94 `tests_discovery.rs:288` replace + with -; M95 `tests_discovery.rs:288` replace + with *; M96 `tests_discovery.rs:291` replace + with -; M97 `tests_discovery.rs:291` replace + with * |
| `check_function` | 2 | low 2 | M98 `tests_discovery.rs:339` replace + with -; M99 `tests_discovery.rs:339` replace + with * |
| `attr_path_ends_with_test` | 1 | low 1 | M100 `tests_discovery.rs:361` replace attr_path_ends_with_test -> bool with true |
| `has_attribute` | 1 | low 1 | M104 `tests_discovery.rs:398` replace != with == |
| `has_configured_attribute` | 3 | low 3 | M105 `tests_discovery.rs:428` replace && with ||; M106 `tests_discovery.rs:428` replace != with ==; M107 `tests_discovery.rs:428` replace != with == |
| `collect_markers_from_siblings` | 7 | medium 4, low 3 | M108 `tests_discovery.rs:455` replace + with -; M109 `tests_discovery.rs:456` replace < with ==; M110 `tests_discovery.rs:456` replace < with >; M112 `tests_discovery.rs:456` replace + with -; M113 `tests_discovery.rs:456` replace + with *; M114 `tests_discovery.rs:457` replace + with -; M115 `tests_discovery.rs:457` replace + with * |
| `collect_body_start_markers` | 10 | medium 5, low 5 | M116 `tests_discovery.rs:526` replace + with -; M117 `tests_discovery.rs:526` replace + with *; M118 `tests_discovery.rs:526` replace + with -; M119 `tests_discovery.rs:526` replace + with *; M120 `tests_discovery.rs:527` replace < with ==; M121 `tests_discovery.rs:527` replace < with >; M123 `tests_discovery.rs:527` replace + with -; M124 `tests_discovery.rs:527` replace + with *; M125 `tests_discovery.rs:528` replace + with -; M126 `tests_discovery.rs:528` replace + with * |
| `discover_and_check` | 1 | low 1 | M127 `tests_discovery.rs:601` replace + with * |

## kotowari 自身にかけた結果

リポジトリ直下（`docs/ir` とこのリポジトリの `tests/`）で `cargo run -- check` を実行した結果（`9c5161e`）:

- files: 23
- lines: 1600
- findings: 0
- 終了コード: 0
