# sources の対応表

元: `docs/ir/core/sources.md`（日本語は `sources.ja.md`）。出典の行の書式と、出典の先が判断の記録と ADR にあるかの検査を扱う。出典の行を求めなくなる（C-core-docs-2）。

この話題は新しい形で役目が無くなるので、書き直した文書は作らない（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」）。「新しい ID」の列に ID があるのは、`docs/decisions/core/` にすでにある確認することの行が同じ振る舞いを持つときだけ。「決定の見出し」の列は、ガイドのマーカーを付け替えるときの、いちばん当たる置き換え先の決定（`docs/decisions/core/` の見出し）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-057 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の行を求めない（C-core-docs-2）。missing_source と source_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-058 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の行を求めない（C-core-docs-2）。missing_source と source_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-059 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の行を求めない（C-core-docs-2）。missing_source と source_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-060 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の行を求めない（C-core-docs-2）。missing_source と source_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-061 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の行を求めない（C-core-docs-2）。missing_source と source_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-062 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。照合を持たない。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-106 | - | 決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す | 消える。形の契約は判断の記録と一緒に凍結した木へ移り、check は読まない（C-core-docs-14）。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-115 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の行を求めない（C-core-docs-2）。missing_source と source_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| TBL-core-012 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の行を求めない（C-core-docs-2）。missing_source と source_invalid は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-285 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-011 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-012 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-120 | - | 仕様は、話題ごとの、今有効な決定の文書だけで持つ | 消える。出典の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/ir.rs | req_053_tag_with_empty_value_is_treated_as_missing | REQ-core-053, REQ-core-059 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-053（ir-references。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | tbl_008_missing_source_scenario_detail_is_raw_scenario_line | REQ-core-059, TBL-core-008 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-008（findings。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_059_source_tag_present_but_only_commas_does_not_report_missing_source | REQ-core-059 | - | 古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_390_a_declaration_whose_decision_is_missing_is_a_source_invalid_on_its_line | REQ-core-210, REQ-core-115, REQ-core-046, TBL-core-011, EX-core-390 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-210、EX-core-390（deferred。同じ依頼で対応表を書いた）、REQ-core-046、TBL-core-011（ir-items。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_057_source_splits_at_first_hash_and_allows_commas | REQ-core-057 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_number_anchor_looks_for_decision_line_and_other_anchor_for_heading | REQ-core-058, TBL-core-012, EX-core-011, EX-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_source_outside_places_is_invalid | REQ-core-058, TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_source_path_equal_to_a_place_itself_is_invalid_without_crashing | REQ-core-058, TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_059_missing_source_for_item_scenario_and_term | REQ-core-059, REQ-core-098 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-098（ir-missing。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_059_empty_source_value_produces_missing_source | REQ-core-059 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_060_glossary_trailing_comma_does_not_create_empty_source | REQ-core-057, REQ-core-060 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_060_glossary_and_scenario_sources_are_checked | REQ-core-060 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_061_numbers_are_per_file | REQ-core-061 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_106_form_contract_headings_are_valid_sources | REQ-core-106 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_decision_line_with_and_without_trailing_text | REQ-core-058, TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_indented_decision_line_counts | REQ-core-058, TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_number_inside_code_block_is_not_a_source_target | REQ-core-058, TBL-core-012, EX-core-118 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: EX-core-118（record-form。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | tbl_012_decision_heading_only_inside_code_block_makes_the_file_not_a_record | REQ-core-058, TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_heading_inside_code_block_of_a_non_record_file_is_not_a_source_target | REQ-core-058, TBL-core-012, EX-core-120 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_hidden_directory_under_records_is_not_a_source_target | REQ-core-058 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_non_regular_entry_named_md_under_records_is_silently_skipped | REQ-core-058 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_non_regular_entry_named_md_under_adr_is_silently_skipped | REQ-core-058 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_a_decision_number_is_an_uppercase_letter_followed_by_digits | REQ-core-058, TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_a_source_outside_the_records_and_adr_places_is_rejected | REQ-core-058, TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_057_a_source_needs_both_a_path_and_an_anchor | REQ-core-057 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_absolute_path_source_is_rejected_even_if_it_would_otherwise_resolve | REQ-core-058 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_tie_break_prefers_the_longer_place_when_a_path_matches_both | REQ-core-058 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_boundary_violating_prefix_does_not_count_as_under_a_place | REQ-core-058 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_file_with_decision_sections_is_a_records_file | TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_010_lone_cr_ends_a_line_of_records_and_adr | TBL-core-010, TBL-core-012 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-010（ir-document。まだ書き直していない）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | tbl_012_a_file_under_both_places_belongs_to_the_deeper_one | TBL-core-012, EX-core-285 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_file_without_decision_sections_matches_headings | TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_two_letter_prefix_is_not_a_decision_number | TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_061_subheading_does_not_end_a_section | REQ-core-061 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_115_source_invalid_line_is_the_source_line | REQ-core-115 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_115_decision_table_source_invalid_line_is_the_source_line_not_another_field | REQ-core-115 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_115_property_source_invalid_line_is_the_source_line_not_another_field | REQ-core-115 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_115_flag_entry_source_invalid_line_is_the_source_line_not_another_field | REQ-core-115 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_012_records_file_headings_are_not_sources | TBL-core-012 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_058_records_place_dot_resolves_a_source_at_the_base_root | REQ-core-058, REQ-core-110 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-110（base-directory。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_110_a_parent_step_in_a_place_is_folded | REQ-core-058, REQ-core-110, EX-core-289 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-110、EX-core-289（base-directory。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_062_source_content_is_not_matched_against_the_item | REQ-core-062 | - | 古い形のコードと一緒に消す |

## ガイドのマーカー

次のガイドのマーカーが、この話題の ID を指している。付け替え先は元の ID の表の「決定の見出し」の列。見出しの列が「-」の ID や、消える指摘を説明する節は、ガイドの節ごと書き直すか消すことになる。

- `docs/guides/deferred.md` の 162 行: REQ-core-115
- `docs/guides/deferred.ja.md` の 162 行: REQ-core-115
- `docs/guides/findings.ja.md` の 177 行: REQ-core-059, REQ-core-058, TBL-core-012
- `docs/guides/commands/check.md` の 335 行: REQ-core-058
- `docs/guides/findings.md` の 177 行: REQ-core-059, REQ-core-058, TBL-core-012
- `docs/guides/commands/check.ja.md` の 333 行: REQ-core-058

## 消えない振る舞い（利用者に聞くこと）

無い。この話題の振る舞いで、決定の文書にも意味があり、`docs/decisions/core/` のどこにも決まっていないものは見つからなかった。
