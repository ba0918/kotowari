# finding-order の対応表

元: `docs/ir/core/finding-order.md`（日本語は `finding-order.ja.md`）。書き直した文書: `finding-order.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-024 | C-core-forder-1, C-core-forder-2 | 指摘は path、line、kind、detail の順に並べる | そのまま。並べ方を C-1、性質を C-2 にした |
| REQ-core-027 | C-core-forder-4 | line は1始まりで数え、文書全体にかかる指摘は null にする | 読み替え。原則を C-4 にした。どの種類が当たるかは、種類の名前を文に残して、それぞれの話題に任せた。too_many_lines、too_many_requirements（利用者の答えで無くす）、missing_scope、glossary_invalid（records.md の A112、第9ラウンド。用語集の検査が無くなる）は消える。ガイド `docs/guides/findings*.md` と `docs/guides/commands/check*.md` の REQ-core-027 のマーカーはこの見出しへ |
| REQ-core-028 | C-core-forder-3 | line は1始まりで数え、文書全体にかかる指摘は null にする | そのまま。ガイド `docs/guides/cli*.md` の REQ-core-028 のマーカーはこの見出しへ |
| TBL-core-007 | C-core-forder-1 | 指摘は path、line、kind、detail の順に並べる | そのまま。表をそのまま決めたことに書いた。ガイド `docs/guides/cli*.md` と `docs/guides/commands/check*.md` の TBL-core-007 のマーカーはこの見出しへ |
| TBL-core-019 | C-core-forder-5, C-core-forder-6, C-core-forder-7 | 行を持つ指摘の line は、種類ごとに1か所の表で決める | 読み替え。records.md の A144（第9ラウンド、利用者が推奨をまとめて採用）が、line の決め方を1か所の表にまとめた。残る種類の行のうち、A144（第9ラウンド）と 2026-09-24-multi-language-tests.md の A32（利用者）が決めたものを表に書いた。マーカーの行（A121）は test-markers の決定に、duplicate_id の行は findings の決定に、unclosed_code_block の行は ir-input の決定にあるので、表の中からリンクで指した。requirement_without_test の「項目の見出しの行」は、confirmation_without_test の「確認することの行」と読む。vague_word の行は、決定の文書への曖昧語の検査が提案なので、別の提案の決定にした（C-7）。古い形の種類の行は消える。ガイド `docs/guides/findings*.md`、`docs/guides/deferred*.md`、`docs/guides/commands/check*.md` の TBL-core-019 のマーカーはこの見出しへ |
| PROP-core-003 | C-core-forder-2 | 指摘は path、line、kind、detail の順に並べる | そのまま。性質を確認することの行にした。ガイド `docs/guides/cli*.md` の PROP-core-003 のマーカーはこの見出しへ |

元の文書に場面（EX）は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step5_findings.rs | req_024_sorted_by_path_line_kind_detail | REQ-core-024, TBL-core-007 | C-core-forder-1 | 2つの文書の指摘が4つの鍵の順に並ぶことを確かめている。入力は古い形の IR の文書なので、切り替えのときに入力を決定の文書とテストのファイルに直す必要がある |
| tests/step5_findings.rs | prop_003_findings_are_sorted | PROP-core-003 | C-core-forder-2 | 文書の数と項目の数の6通りをすべて試す性質のテスト。入力は古い形の IR の文書と判断の記録 |
| tests/step5_findings.rs | req_024_a_finding_kind_equals_only_its_own_name | REQ-core-024 | C-core-forder-1 | FindingKind と文字列の等しさを確かめていて、並べ方そのものは確かめていない。落とした細部のテスト。いちばん近い行を指す（kind で並べるときの名前の文字） |
| tests/step5_findings.rs | req_028_lines_start_at_one | REQ-core-028 | C-core-forder-3 | 7行目の見出しへの指摘の line が 7 になることを確かめている。入力は古い形の IR の文書 |
| tests/step5_findings.rs | req_027_document_wide_findings_have_null_line | REQ-core-027 | C-core-forder-4 | 題名の無い文書の missing_title の line が null であることを確かめている。入力は古い形の IR の文書 |
| tests/step5_findings.rs | req_027_glossary_invalid_has_null_line | REQ-core-027 | - | glossary_invalid（用語集）は消える。古い形のコードと一緒に消す |
| tests/step5_findings.rs | tbl_019_unclosed_code_block_line_is_the_opening_line | TBL-core-019 | - | IR の文書の unclosed_code_block の行。決定の文書の開始の行は step23 のテスト（ir-input の対応表で C-core-input-3）が確かめるので、古い形のコードと一緒に消す |
| tests/step5_findings.rs | tbl_019_invalid_gherkin_line_and_invalid_id_lines | TBL-core-019 | - | 場面の invalid_gherkin_line と invalid_id の行。古い形のコードと一緒に消す |
| tests/step5_findings.rs | tbl_019_unclosed_backtick_line | TBL-core-019 | - | IR の文書の unclosed_backtick の行。古い形のコードと一緒に消す |
| tests/step5_findings.rs | tbl_019_marker_findings_line_is_the_marker_line | TBL-core-019 | C-core-markers-7 | テストのファイルの空のマーカーへの invalid_marker の line が、マーカーのある1行目になる。マーカーの行は test-markers の「形の誤ったマーカーには invalid_marker の誤りを出し、マーカーの行を指す」が決めて確かめる（この文書の表からはリンクで指した）ので、その行を指す。この話題の行には当てない
| tests/step5_findings.rs | tbl_019_source_invalid_line_is_the_source_line | TBL-core-019 | - | source_invalid（出典）の行。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | tbl_019_unresolved_definition_reference_line_is_the_definition_fields_own_line | TBL-core-019 | - | IR の定義の行の参照。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | tbl_019_unresolved_relation_reference_line_is_the_relation_fields_own_line | TBL-core-019 | - | 問題の記録の関係の行の参照。古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_123_duplicate_across_the_chain_is_reported_on_the_deeper_row | REQ-core-123, TBL-core-019 | - | 用語集の duplicate_term の行。古い形のコードと一緒に消す |
| tests/step8_record_form.rs | req_131_unknown_name_on_two_lines_yields_two_findings | REQ-core-131, REQ-core-133, TBL-core-019 | - | 判断の記録の record_field_unknown の行。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_210_an_empty_declaration_is_a_missing_source_on_its_line | REQ-core-210, TBL-core-008, TBL-core-019 | - | 後回しの宣言の行。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_392_a_mark_on_a_deferred_requirement_is_a_notice_on_its_heading | REQ-core-211, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-392 | - | deferred_with_test の行。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_393_a_mark_on_a_deferred_scenario_is_a_notice_on_its_tag_line | REQ-core-211, TBL-core-019, EX-core-393 | - | 同上 |
| tests/step16_deferred.rs | ex_394_a_statement_of_a_requirement_that_is_not_deferred_pointing_at_one_is_a_notice | REQ-core-212, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-394 | - | depends_on_deferred の行。古い形のコードと一緒に消す |
| tests/step13_test_languages.rs | tbl_033_decorated_test_is_counted_at_its_def_line | REQ-core-184, TBL-core-033, TBL-core-019 | C-core-forder-6 | この話題の分（TBL-core-019）だけ。Python のデコレータの付いたテストの test_without_id の line が "def" の行。REQ-core-184 と TBL-core-033 はテストを見つける話題 |
| tests/step13_test_languages.rs | tbl_034_test_methods_and_attributes | REQ-core-185, TBL-core-034, TBL-core-019 | C-core-forder-6 | この話題の分だけ。PHP の "#[Test]" の付いたメソッドの line が "#[Test]" の行。REQ-core-185 と TBL-core-034 はテストを見つける話題 |
| tests/step15_guides.rs | ex_367_a_mark_without_fingerprint_and_an_uppercase_fingerprint_are_invalid | REQ-core-201, REQ-core-202, TBL-core-036, TBL-core-008, TBL-core-019, EX-core-367 | - | ガイドのマーカーの invalid_marker の行。ガイドの話題に属し、そちらの ID が付け替えられる |
| tests/step15_guides.rs | ex_363_a_changed_body_makes_the_mark_a_stale_notice | REQ-core-204, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-363 | - | guide_stale の行。ガイドの話題に属する |
| tests/step15_guides.rs | req_204_the_line_of_guide_stale_is_where_the_mark_starts | REQ-core-204, TBL-core-019 | - | 同上 |
| tests/step17_surface.rs | ex_core_408_a_surface_absent_from_the_ir_is_an_error_at_its_first_place | EX-core-408, EX-core-409, REQ-core-227, TBL-core-019, TBL-core-006, TBL-core-008 | - | surface_without_spec の行。面の話題に属する |
| tests/step17_surface.rs | ex_core_421_an_entry_whose_reason_is_blank_is_invalid_and_removes_nothing | EX-core-421, REQ-core-233, REQ-core-027, TBL-core-006, TBL-core-008 | C-core-forder-4 | 未記載の面の一覧への surface_unspecified_invalid の line が null であることを確かめている。この話題の分だけで、ほかの ID は面の話題の分 |
| tests/step17_surface.rs | ex_core_422_an_entry_for_a_surface_gone_from_the_code_is_a_notice | EX-core-422, REQ-core-234, REQ-core-027, TBL-core-009 | C-core-forder-4 | surface_unspecified_stale の line が null であること。同上 |
| crates/kotowari-core/tests/translations.rs | req_core_027_lines_of_the_pair_findings | REQ-core-027, TBL-core-019 | C-core-forder-4 | translation_missing と translation_record_invalid の line が null であることを確かめている。行を持つ対の指摘の行は対の話題の分 |
| crates/kotowari-overview/tests/overview.rs | ex_core_466_an_unknown_part_and_a_part_with_an_unknown_key_are_errors | EX-core-466, REQ-core-282, REQ-core-027, TBL-core-019 | - | 部品のフェンスの開始の行を確かめている。全体像の話題に属する |
| crates/kotowari-overview/tests/overview.rs | req_core_282_places_name_missing_keys_the_whole_value_and_unreadable_yaml_once_each | REQ-core-282, REQ-core-027 | - | detail だけを確かめていて、line を確かめていない。全体像の話題（REQ-core-282）が付け替える |
| crates/kotowari-overview/tests/overview.rs | ex_core_467_a_first_part_that_is_not_lead_is_an_error_without_a_line | EX-core-467, REQ-core-283, REQ-core-027 | C-core-forder-4 | overview_lead_missing の line が null であること。この話題の分だけで、ほかの ID は全体像の話題の分 |
| crates/kotowari-overview/tests/overview.rs | ex_core_468_a_missing_ir_document_and_one_shared_by_two_overviews_are_errors | EX-core-468, REQ-core-284, REQ-core-027 | C-core-forder-4 | overview_ir_missing と overview_ir_shared の line が null であること。同上 |
| crates/kotowari-overview/tests/overview.rs | ex_core_469_unresolved_references_are_errors_one_per_reference | EX-core-469, REQ-core-285, REQ-core-027 | - | overview_ref_unresolved が部品のフェンスの開始の行を指すこと。全体像の話題に属する |
| crates/kotowari-overview/tests/overview.rs | tbl_core_038_frontmatter_must_hold_only_a_nonempty_ir_list_of_strings | REQ-core-281, TBL-core-038, REQ-core-027 | C-core-forder-4 | frontmatter への overview_form_invalid の line が null であること。同上 |
| crates/kotowari-overview/tests/overview.rs | tbl_core_038_the_title_is_exactly_one_heading_of_depth_one | REQ-core-281, TBL-core-038, REQ-core-027 | - | 2つ目の題名の行を指すことを確かめている。全体像の話題に属する |
| crates/kotowari-overview/tests/overview.rs | req_core_305_duplicated_names_and_index_and_style_are_errors | REQ-core-305, REQ-core-027 | C-core-forder-4 | overview_name_conflict の line が null であること。同上 |
| crates/kotowari-overview/tests/overview.rs | ex_core_507_an_unknown_key_in_a_group_is_a_form_error_and_stops_the_comparison | EX-core-507, REQ-core-327, REQ-core-331, TBL-core-043, REQ-core-027 | C-core-forder-4 | 目次の指摘の line が null であること（補助関数 toc_findings が確かめる）。同上 |
| crates/kotowari-overview/tests/overview.rs | tbl_core_043_each_form_violation_names_its_place | REQ-core-327, TBL-core-043, REQ-core-027 | C-core-forder-4 | 同上 |
| crates/kotowari-overview/tests/overview.rs | ex_core_508_missing_unknown_and_repeated_names_are_errors | EX-core-508, REQ-core-328, REQ-core-329, REQ-core-027 | C-core-forder-4 | 同上 |
| crates/kotowari-overview/tests/overview.rs | ex_core_509_a_group_without_items_is_an_error | EX-core-509, REQ-core-330, REQ-core-027 | C-core-forder-4 | 同上 |
| tests/step18_overview.rs | ex_core_472_errors_in_overview_data_join_findings_counts_and_the_exit_code | EX-core-472, REQ-core-288, REQ-core-290, REQ-core-027 | C-core-forder-4 | check の出力で overview_lead_missing の line が null であること。同上 |
| tests/step18_overview.rs | req_core_290_overview_findings_are_sorted_with_the_other_findings | REQ-core-290, REQ-core-024 | C-core-forder-1 | 全体像の元データの指摘が、ほかの指摘と一緒に path の順に並ぶことを確かめている（path の鍵の分だけ）。REQ-core-290 は全体像の話題の分 |
| tests/step23_decision_documents.rs | an_unclosed_code_block_under_a_confirmation_row_is_an_error_and_the_next_rows_are_still_read | C-core-docs-1（この話題の ID を指していない） | C-core-docs-1, C-core-forder-5 | この話題の ID を今は指していないが、決定の文書の形で、3つの確認することの行の confirmation_without_test の line が、それぞれの行（19、21、22）になることを確かめている。マーカーに C-core-forder-5 を足す（ir-input の対応表も C-core-input-3、C-core-input-4 を足す） |
| tests/step23_decision_documents.rs | each_title_after_the_first_is_an_error_without_a_line | C-core-docs-1（この話題の ID を指していない） | C-core-docs-1, C-core-forder-4 | 決定の文書の multiple_titles の line が null になる。マーカーに C-core-forder-4 を足す（ir-document の対応表も C-core-document-5 を足す） |
| tests/step23_decision_documents.rs | a_document_without_a_title_is_an_error | C-core-docs-8（この話題の ID を指していない） | C-core-docs-8, C-core-forder-4 | 決定の文書の missing_title の line が null になる。マーカーに C-core-forder-4 を足す（findings の対応表も C-core-findings-10 を足す） |

## 戻した細部

無い。安全に関わる落とした細部はこの話題に無い。

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-027: unparsable_file の line を null にする | 実験者の裁定（records.md の A83。A75〜A97 の裁定の1つ。読めないファイルに行は無い、から導いた）。名前は「それぞれの話題で決める」側に残した |
| REQ-core-027: overview_lead_missing、overview_ir_missing、overview_ir_shared、overview_name_conflict の line を null にする | LLM（2026-10-02-whole-picture.md の A74。承認の時に利用者に示す）。名前は残し、行の決め方は全体像の話題に任せた |
| REQ-core-027、TBL-core-019: 対の指摘（translation_*）の line | 細部を LLM に任せた承認（2026-10-05-localization.md の D1、D2）。名前は残し、対の話題に任せた |
| TBL-core-019: 表に書かなかった残る種類（mutant_survived、mutant_timeout、guide_stale、ガイドのマーカーの invalid_marker、surface_without_spec、overview_part_unknown、overview_part_invalid、overview_ref_unresolved、overview_form_invalid、translation_structure_mismatch、translation_switcher_invalid、link_language_mismatch）の行 | 落としたのではなく、それぞれの種類を決めた話題へ移す。どの行も A144 より後の、それぞれの話題の記録で決まっている |
| TBL-core-019: invalid_id の行（タグの行） | A144（利用者）。場面のタグの行なので消える。確認することの行の invalid_id の行は記録に無い |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-027 の missing_scope | 消える。文書が扱う範囲の行は、決定の文書では題名のあとの段落になる（題名のあとに段落が無い文書への誤りは、`decision-documents.ja.md` の C-core-docs-8 が決める） |
| REQ-core-027 の missing_title、multiple_titles | 新しい形で読み替えて残る。IR の話題ごとの文書ではなく、決定の文書についての指摘として。決定の文書を読むコードは、どちらも line を null で出す（multiple_titles は ir-document の C-core-document-5、実装済み） |
| REQ-core-027 の too_many_lines、too_many_requirements | 消える（利用者の答え。ir-document の「行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす」、提案） |
| REQ-core-027 の glossary_invalid | 消える。用語集の表の検査が無くなる |
| TBL-core-019 の missing_source、source_invalid、record_field_missing、record_field_unknown、revision_link_invalid | 消える。出典と判断の記録が無くなる |
| TBL-core-019 の link_to_record | 新しい形で読み替えて残る（利用者の答え。提案）。決定の文書とその対の側から凍結した木の下へのリンクの誤りになり、決定は translation-structure の話題に置く。行（リンクの行）は対の話題が決める |
| TBL-core-019 の missing_tag、unknown_tag、invalid_id（場面のタグ）、scenario_without_test、invalid_gherkin_line | 消える。場面が無くなる |
| TBL-core-019 の vague_word | 新しい形で読み替えて残る（C-core-forder-7、提案）。決定の文書の曖昧語の検査（`decision-documents.ja.md` の C-core-docs-18）がまだ作られていない |
| TBL-core-019 の unknown_term、unclosed_backtick、missing_document、invalid_glossary_row、duplicate_term、glossary_title_invalid | 消える。用語集と用語の検査と文書名の参照が無くなる |
| TBL-core-019 の unclosed_code_block | 新しい形で読み替えて残る（実装済み）。決定の文書の閉じないコードブロックの開始の行（ir-input の C-core-input-3）。表の行からリンクで指した |
| TBL-core-019 の id_domain_mismatch | 新しい形で読み替えて残る（利用者の答え。findings の「確認することの ID のまとまりの名前が…」、提案）。line は実装する側が決める |
| TBL-core-019 の unknown_field、duplicate_field、unknown_heading、unknown_line、unknown_code_block、missing_field、missing_statement、missing_table、verification_missing、verification_invalid、unknown_kind、algorithm_without_definition | 消える。IR の項目の形の検査が無くなる |
| TBL-core-019 の deferred_with_test、depends_on_deferred | 消える。後回しの宣言が無くなる |
| TBL-core-019 の duplicate_id | 新しい形で読み替えて残る。同じ ID の確認することの行への誤り。行は findings の C-core-findings-8 が確かめ、この文書の表からリンクで指した |
| TBL-core-019 の requirement_without_test | 新しい形で読み替えて残る。テストの無い確認することの行への confirmation_without_test（C-core-forder-5） |
| TBL-core-019 の unresolved_reference | 定義・関係・文・"@about" の中の参照は消える。テストのマーカーが無い ID を指す誤りは残る。行は test-markers の C-core-markers-7 |

テストの無い行（切り替えのあとに、古い形でない入力で確かめるテストが無い行）:

- C-core-forder-2: 付け替えるテストは `prop_003_findings_are_sorted` だけで、入力は古い形の IR の文書と判断の記録である。切り替えで check が IR を読まなくなると指摘が出ず、確かめる中身が無くなる。決定の文書とテストのファイルのような残る入力で、2つ以上のファイルに行を持つ指摘と行を持たない指摘が混ざって出る組を作り、並びを確かめる性質のテストが要る（行の文はこのまま）。
- C-core-forder-1 と C-core-forder-3 も、古い形の入力のテスト（`req_024_sorted_by_path_line_kind_detail`、`req_028_lines_start_at_one`）があるが、ほかに残る入力のテストがある（C-1 は `tests/step18_overview.rs` の `req_core_290_overview_findings_are_sorted_with_the_other_findings`、C-3 は plan の `req_core_193_the_line_is_the_line_the_schema_side_reported`。plan の対応表に書いた）。古い形の入力のテストは、入力を直すか消す。
- C-core-forder-7 は提案の行で、テストは実装のときに作る。

## 誰が決めたか見分けられなかったもの

- surface_unspecified_invalid と surface_unspecified_stale の line を null にすること。2026-09-27-surface-check.md の A6（利用者）は「等価の一覧と同じ扱い」と決め、行の扱いを書いた A22 は「IR を書く段で決めた細部をまとめて採用」である。名前だけ残し、行の決め方は面の話題に任せた。

## 理由や見送った案が記録に無かった決定

- 「指摘は path、line、kind、detail の順に並べる」: この並べ方を選んだ理由が記録に無い（records.md の A40、A61、A70 に why が無い）。mutants の指摘を同じ並べ方にした理由（2026-09-17-mutation-tests.md の A40）だけを問題に書いた。見送った案も無い。
- 「line は1始まりで数え、文書全体にかかる指摘は null にする」: 理由も見送った案も記録に無い。問題の節に「理由は残っていない」と書いた。
- 「行を持つ指摘の line は、種類ごとに1か所の表で決める」: 理由も見送った案も記録に無い（A144 は「敵対的レビューの指摘」とあるだけ。A121 と multi-language-tests の A32 の why は、それぞれマーカーとテストの行の細部の理由）。問題の節に「理由は残っていない」と書いた。
- 「曖昧語の指摘 vague_word の line は、その語のある行にする」: 理由も見送った案も記録に無い。

## 分けた所、まとめた所

- REQ-core-024、TBL-core-007、PROP-core-003 を1つの決定にまとめた。並べ方の表は決定表として決めたことに書き、性質は C-2 にした。
- REQ-core-027、REQ-core-028 を1つの決定にまとめた。
- TBL-core-019 を、残る種類の行の決定（C-5、C-6、実装済み）と、vague_word の行の決定（C-7、提案）の2つに分けた。今のコードの振る舞いと、まだ作っていない振る舞いが混ざるため。duplicate_id、unclosed_code_block、テストのマーカーの invalid_marker と unresolved_reference の行は、それを確かめる行がほかの話題（findings、ir-input、test-markers）にあるので、表の中からリンクで指し、ここに確認することの行を重ねなかった。
- mutants の指摘も同じ並べ方にすること（2026-09-17-mutation-tests.md の A40、利用者）は、元の文書に無く、mutants の話題の側にあるので、言葉で指すだけにした。その理由（受け取る側が1つの読み方で済む）は、並べ方を揃える理由として問題の節にも書いた。

## あとでリンクにする所

- 済み: plan、findings、ir-input、mutants、`decision-documents.ja.md` の見出しへのリンクにした。
- 済み: 「指摘は path、line、kind、detail の順に並べる」の mutants の指摘は、mutants の「集計を必ず出し、等価として外した件数を毎回見せる」（JSON の最上位の findings の並べ方を check と同じにする表がある）へのリンクにした。
- 残り: 「行を持つ指摘の line は…」の表の外の種類を決める話題（mutants、guides、surface、overview-data、overview-toc、translation-structure）は、言葉で指した。どの見出しが行を決めるかが各話題で1つに定まっていないため。

## 迷った判断と、どう決めたか

- C-core-forder-4 の「（目次のファイルへの overview_toc_invalid、など）」を、例だと分かる「（例: …）」の書き方に直し、決定の文書の missing_title の例を足した。意味は変えていない。

- records.md の第9ラウンド（A98〜A144、利用者の答え「進めて」）を、BRIEF-C の読み方どおり利用者の決定に数え直した。前の版は細部の側に置いていた。これで A144 の「line の決め方は1か所の表にまとめる」と、その表の残る種類の行が利用者の決定になり、決定「行を持つ指摘の line は、種類ごとに1か所の表で決める」と C-5〜C-7 を足した。A112（用語集の glossary_invalid の line）は用語集の検査と一緒に消える。A83（unparsable_file の line、A75〜A97 の裁定）と A156 以降は、前のとおり agent の細部として落とした。
- 第6〜8ラウンド（A40、A61、A70）の利用者の答えは「推奨で」「基本的には細部の調整といったところで、推奨で問題無いと思う」「推奨で。細かい実装の判断なんだよな」である。前の版と同じく、利用者の決定として残した。
- test_without_id の行は、A144 の「関数の宣言の行」を、2026-09-24-multi-language-tests.md の A32（利用者）が「テストの節の最初の行」に改めているので、後者で書いた。
- REQ-027 の種類の列挙は、決めたことには原則だけを書き、種類の名前は「どの種類が当たるかは、それぞれの指摘を決めた話題で決める」の文の中に残した。too_many_lines は、利用者の答えで無くすので列挙から外した。overview_ir_missing と overview_ir_shared は、全体像の話題で改名するが、新しい名前は実装する側が決めるので、今の名前に「改名する」と添えた。
- C-core-forder-4 は原則の行にし、全体像・面・対のテストのうち line が null であることを確かめているものを、この話題の分として付け替えた。決定の文書の missing_title と multiple_titles のテストも、line が null であることを確かめているので、マーカーを足す側に挙げた。
- 計画書の `req_core_193_a_finding_without_a_line_has_a_null_line`（plan の話題の ID だけを指す）も C-core-forder-4 に当たる。plan の対応表に書いた。finding-map の EX-core-278 のテストも同じで、finding-map の対応表に書いた。
- `req_024_a_finding_kind_equals_only_its_own_name` は並べ方を確かめないが、この話題の ID だけを持つので、「-」のままだとマーカーの無いテストになる。BRIEF-C のとおり、いちばん近い C-core-forder-1 を指した。

## 数

- 元: 要求 3、場面 0、決定表 2、性質 1
- 読んだ記録のファイル: 8（records.md（A20、A40、A61、A70、A83、A104、A108、A112、A121、A144 と各ラウンドの見出し）、2026-09-17-mutation-tests.md、2026-09-27-surface-check.md、2026-10-02-whole-picture.md、2026-10-05-overview-index.md、2026-10-05-localization.md、2026-09-24-guide-gaps.md、2026-09-24-multi-language-tests.md の A32）。TBL-core-019 の出典のうち、古い形の種類とほかの話題の種類の行だけを支える記録（record-form、deferred-items、doc-marks、id-namespace、ir-tree、scenario-tests、ir-engine の一部、ir-form.md）は読んでいない
- 出来上がり: 決定 4（実装済み 3、提案 1）、確認することの行 7（うち「（読んで確認）」0）
- テスト: 42（この話題の ID を指すもの）。新しい ID を持つもの 21（C-1 に 3、C-2 に 1、C-3 に 1、C-4 に 13、C-6 に 2、test-markers の C-core-markers-7 に 1）、「-」21。ほかに、この話題の ID を指していないがマーカーを足すことを勧めるもの 3
- 利用者に聞くこと: 0
- 戻した安全の細部: 0
