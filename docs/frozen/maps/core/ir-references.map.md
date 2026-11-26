# ir-references の対応表

元: `docs/ir/core/ir-references.md`（日本語は `ir-references.ja.md`）。シナリオのタグ、ID の参照切れ、ID の形を扱う。場面とタグは書かなくなり、ID は確認することの ID に置き換わる。マーカーが無い ID を指すと誤りになることは `docs/decisions/core/confirmations.md` の C-core-confirm-4 に、ID の形の誤りは `findings.md` の C-core-findings-4（invalid_id）に残る。

この話題は新しい形で役目が無くなるので、書き直した文書は作らない（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」）。「新しい ID」の列に ID があるのは、`docs/decisions/core/` にすでにある確認することの行か、下書きの findings の行が同じ振る舞いを持つときだけ。「決定の見出し」の列は、ガイドのマーカーを付け替えるときの、いちばん当たる置き換え先の決定（`docs/decisions/core/` の見出し）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-052 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。場面とそのタグと gherkin のブロックは書かない（C-core-docs-5〜8 の形）。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-053 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。場面とそのタグと gherkin のブロックは書かない（C-core-docs-5〜8 の形）。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-054 | - | check は、確認することの行ごとに、それを指すテストがあるかを数える | 読み替え（実装済み）。マーカーが無い ID を指すと誤りになることは C-core-confirm-4（テストごとの表で、マーカーを確かめる4本だけを付け替える）。"- definition:"、"@about"、"- related:"、文の中の囲んだ ID の参照切れは消え、ほかの決定はリンクで指す（C-core-docs-11）。見出しは `docs/decisions/core/confirmations.md` のもの |
| REQ-core-055 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。決まった文型の要求文を書かない。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-056 | - | 人に戻す問題は、話題の文書の「未決」の節に書く | 消える。問題の記録を持たない。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-113 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。場面とそのタグと gherkin のブロックは書かない（C-core-docs-5〜8 の形）。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-114 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。"@id" のタグが無くなる。確認することの ID の形の誤りは findings.md の C-core-findings-4（invalid_id）。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-124 | - | check は、確認することの行ごとに、それを指すテストがあるかを数える | 読み替え。IR の ID の形（REQ-、TBL-、PROP-、EX-、FLAG-）は消え、確認することの ID の形 `C-<まとまりと話題の短い名前>-<連番>` が置き換わる（`confirmations.md` の「check は、確認することの行ごとに、それを指すテストがあるかを数える」 の決めたこと）。形の誤りの invalid_id は findings.md の C-core-findings-4 |
| REQ-core-167 | C-core-findings-11, C-core-findings-12 | 確認することの ID のまとまりの名前が、置いてあるまとまりのフォルダと合わなければ、id_domain_mismatch の誤りにする | 読み替え（利用者の答え。提案）。確認することの ID のまとまりの名前と、置き場のまとまりのフォルダの一致を見る。置き場の直下の文書は見ない。今の決定の文書のコードは見ていないので、findings に提案の決定として置いた。見出しは findings のもの |
| EX-core-283 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。gherkin とタグの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-009 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。gherkin とタグの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-010 | - | 1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く | 消える。"@about" の参照切れの場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-028 | - | check は、確認することの行ごとに、それを指すテストがあるかを数える | 消える。IR の ID の形の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-029 | - | check は、確認することの行ごとに、それを指すテストがあるかを数える | 消える。IR の ID の形の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-043 | - | check は、確認することの行ごとに、それを指すテストがあるかを数える | 消える。IR の ID の形の場面。見出しは `docs/decisions/core/confirmations.md` のもの |
| EX-core-044 | C-core-findings-11 | 確認することの ID のまとまりの名前が、置いてあるまとまりのフォルダと合わなければ、id_domain_mismatch の誤りにする | 読み替え（利用者の答え。提案）。名前の一致しない ID の場面を、まとまりのフォルダと合わない確認することの ID と読む。見出しは findings のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/inputs.rs | parsed_documents_retain_nested_paths_and_definition_and_statement_references | REQ-core-314, REQ-core-317, REQ-core-322, REQ-core-054, EX-core-487, EX-core-498 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-314、REQ-core-317、EX-core-487（library-inputs。対応表あり）、REQ-core-322、EX-core-498（library-paths。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_053_consecutive_scenarios_without_tags_each_get_missing_tag | REQ-core-042, REQ-core-053 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-042（ir-items。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_052_unknown_tag | REQ-core-052 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_052_bare_tag_without_equals_is_unknown | REQ-core-052 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_053_missing_tag | REQ-core-053 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_053_tag_with_empty_value_is_treated_as_missing | REQ-core-053, REQ-core-059 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-059（sources。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_054_unresolved_reference_in_definition_about_relation_and_sentence | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | gherkin_tags_cleared_after_block_without_scenario | REQ-core-053 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_flag_relation_to_unknown_id_produces_unresolved_reference | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_backtick_id_known_no_finding_unknown_produces_unresolved | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_backtick_id_at_line_start_detected | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_two_backtick_ids_on_one_line_both_reported | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_backtick_non_id_not_reported_as_unresolved | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_property_statement_backtick_id_produces_unresolved | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_indented_steps_are_recognized | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_blank_lines_and_comments_inside_a_scenario_keep_its_steps | REQ-core-113, EX-core-283 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_feature_and_examples_lines_are_invalid | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_tag_line_binds_only_when_immediately_before_scenario | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_052_unbound_tag_line_is_still_checked | REQ-core-052 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_052_word_without_at_in_tag_line_is_unknown_tag | REQ-core-052 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_114_malformed_id_tag_is_invalid_id_and_not_defined | REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_114_malformed_id_scenario_missing_source_detail_is_scenario_line | REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_114_malformed_heading_is_not_defined | REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_non_id_definition_value_is_unresolved | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_backtick_id_in_step_is_checked | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_052_word_with_equals_not_starting_with_at_keeps_full_word_as_detail | REQ-core-052 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_backtick_id_inside_double_quotes_is_not_checked | REQ-core-054 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_backtick_oddness_counted_outside_quotes_only | REQ-core-054, REQ-core-116 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-116（terms-form。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_113_tag_line_not_immediately_before_scenario_is_invalid | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_114_malformed_id_still_reports_missing_about | REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_tags_do_not_leak_into_the_next_untagged_scenario | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_consecutive_steps_without_scenario_report_only_the_first | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_orphan_step_after_a_blank_line_is_reported_again | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_step_right_after_tag_line_reports_both_lines | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_113_tag_line_right_before_closing_fence_is_invalid | REQ-core-113 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_124_an_id_name_takes_lowercase_letters_digits_and_hyphens_only | REQ-core-124, REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_052_dangling_bare_word_in_unbound_tag_line_keeps_its_line_number | REQ-core-052 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_114_well_formed_non_ex_id_is_not_used_as_scenario_id | REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_114_malformed_id_value_is_the_actual_malformed_tag_not_a_bare_at_id | REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_114_malformed_id_value_is_not_stolen_from_an_unrelated_about_tag | REQ-core-114 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_054_valid_and_known_definition_id_produces_no_unresolved_reference | REQ-core-054 | - | 古い形のコードと一緒に消す |
| tests/step11_query.rs | tbl_027_definition_and_text_references_are_listed | TBL-core-027, REQ-core-054, EX-core-257 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-027、EX-core-257（query。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step11_query.rs | tbl_027_text_reference_is_only_the_backticked_id | TBL-core-027, REQ-core-054 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-027（query。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step1_cli.rs | req_157_positional_that_is_not_an_id_stops | REQ-core-157, REQ-core-124 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-157（query。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step2_ir.rs | req_055_non_ears_statement_gets_no_finding_on_its_line | REQ-core-055 | - | 古い形のコードと一緒に消す |
| tests/step2_ir.rs | req_056_contradiction_flag_with_one_reading_gets_no_finding | REQ-core-056 | - | 古い形のコードと一緒に消す |
| tests/step2_ir.rs | req_052_retired_tag_on_a_scenario_is_unknown_tag | REQ-core-052, EX-core-009 | - | 古い形のコードと一緒に消す |
| tests/step2_ir.rs | req_054_scenario_about_an_unknown_requirement_is_unresolved | REQ-core-054, EX-core-010 | - | 古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_054_marker_to_known_id_is_not_unresolved | REQ-core-054, REQ-core-077 | C-core-confirm-4 | 実装のときに直す。準備が IR の要求なので、決定の文書の確認することの行に直すか、step23 の C-core-confirm-4 のテストと重なるなら消す。ほかの話題の ID も指す: REQ-core-077（test-markers。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step4_test_discovery.rs | req_054_marker_to_unknown_id_is_unresolved | REQ-core-054 | C-core-confirm-4 | 実装のときに直す。準備が IR の要求なので、決定の文書の確認することの行に直すか、step23 の C-core-confirm-4 のテストと重なるなら消す |
| tests/step4_test_discovery.rs | req_072_non_query_language_checks_invalid_marker | REQ-core-072, REQ-core-054 | - | この話題の分は無い（invalid_marker を確かめるテスト）。ほかの話題の ID も指す: REQ-core-072（test-markers。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step4_test_discovery.rs | req_054_non_query_language_checks_unresolved_reference | REQ-core-054 | C-core-confirm-4 | 実装のときに直す。問い合わせの無い言語のファイルのマーカーでも出ることを確かめる。準備を決定の文書に直す |
| tests/step4_test_discovery.rs | req_054_duplicate_marker_id_in_same_test_reports_per_occurrence | REQ-core-054 | C-core-confirm-4 | 実装のときに直す。出現ごとに1件は落とした細部で、テストの中に残る |
| tests/step4_test_discovery.rs | req_124_four_digit_id_is_valid_in_heading_tag_and_marker | REQ-core-124, REQ-core-114, REQ-core-043 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-043（ir-items。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step4_test_discovery.rs | req_124_leading_zero_and_short_ids_are_rejected | REQ-core-124, REQ-core-043, REQ-core-114 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-043（ir-items。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step4_test_discovery.rs | req_137_scenario_without_id_is_not_reported | REQ-core-137, REQ-core-053 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-137（coverage。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step4_test_discovery.rs | req_137_scenario_with_invalid_id_is_not_reported | REQ-core-137, REQ-core-114 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-137（coverage。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step4_test_discovery.rs | req_137_scenario_about_an_unknown_id_is_not_required | REQ-core-137, REQ-core-054 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-137（coverage。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step4_test_discovery.rs | req_124_four_digit_heading_is_read_and_the_other_two_are_unknown_headings | REQ-core-124, EX-core-028 | - | 古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_124_four_digit_id_in_a_tag_and_a_marker_is_read_the_same_way | REQ-core-124, EX-core-029 | - | 古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_124_named_and_unnamed_ids_are_both_read | REQ-core-124, EX-core-043 | - | 古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_167_id_name_must_match_the_first_directory_segment | REQ-core-167, EX-core-044 | C-core-findings-11 | 提案の行。実装のときに直す。準備を、まとまりのフォルダの決定の文書と確認することの ID に直す |
| tests/step4_test_discovery.rs | req_167_scenario_id_name_is_reported_on_the_tag_line | REQ-core-167 | - | 古い形のコードと一緒に消す（"@id" のタグの行） |
| tests/step5_findings.rs | ex_core_376_lines_in_a_non_gherkin_block_are_not_read_as_gherkin | REQ-core-174, REQ-core-113, EX-core-376 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-174、EX-core-376（findings。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step5_findings.rs | req_174_a_gherkin_block_beside_a_non_gherkin_block_is_still_read_as_gherkin | REQ-core-174, REQ-core-113 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-174（findings。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |

## ガイドのマーカー

次のガイドのマーカーが、この話題の ID を指している。付け替え先は元の ID の表の「決定の見出し」の列。見出しの列が「-」の ID や、消える指摘を説明する節は、ガイドの節ごと書き直すか消すことになる。

- `docs/guides/findings.ja.md` の 234 行: REQ-core-054, REQ-core-114, REQ-core-167
- `docs/guides/findings.ja.md` の 254 行: REQ-core-053, REQ-core-052, REQ-core-113
- `docs/guides/findings.ja.md` の 518 行: REQ-core-054
- `docs/guides/findings.md` の 234 行: REQ-core-054, REQ-core-114, REQ-core-167
- `docs/guides/findings.md` の 254 行: REQ-core-053, REQ-core-052, REQ-core-113
- `docs/guides/findings.md` の 518 行: REQ-core-054

## 消えない振る舞い

- 確認することの ID のまとまりの名前と、置き場のまとまりのフォルダの一致（元の REQ-core-167、EX-core-044）は、利用者の答えで残す。findings の「確認することの ID のまとまりの名前が、置いてあるまとまりのフォルダと合わなければ、id_domain_mismatch の誤りにする」（提案、C-core-findings-11、C-core-findings-12）に置いた。理由と見送った案は 2026-09-22-id-namespace.md の A3（利用者）のもの。置き場の直下の文書は見ない（古い A5 の「直下の文書の名前付きの ID を誤りにする」は当てない）。

## 利用者に聞くこと

無い。

## 迷った判断と、どう決めたか

- 前の版で利用者に聞いていた id_domain_mismatch の扱いは、利用者の答え（残す。置き場の直下の文書は見ない）が出たので、問いを消し、findings の提案の決定に付け替えた。
