# findings の対応表

元: `docs/ir/core/findings.md`（日本語は `findings.ja.md`）。書き直した文書: `findings.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-029 | C-core-findings-1, C-core-findings-3, C-core-findings-4, C-core-findings-5, C-core-findings-6 | 指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める | 読み替え。誤りの種類の表（TBL-core-008）を指す要求。新しい版にある種類だけを残した。誤りであることは C-1、detail は C-3〜C-6 |
| REQ-core-030 | C-core-findings-1, C-core-findings-7 | 指摘は誤りと注意に分け、注意にするのは表の種類だけにする | 読み替え。注意の種類の表（TBL-core-009）を指す要求。後回しの2種類は消える。too_many_lines と too_many_requirements は利用者の答えで無くす（ir-document の「行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす」、提案） |
| REQ-core-031 | C-core-findings-1, C-core-findings-2 | 指摘は誤りと注意に分け、注意にするのは表の種類だけにする | 読み替え。「8つだけ」を、数を持たない表と「表に無い種類はすべて誤り」の文にした。後回しの2種類と、too_many_lines・too_many_requirements は消える。注意が終了コードを変えないこと（用語「注意」の定義）は C-2 にした |
| REQ-core-032 | C-core-findings-8, C-core-findings-9 | 同じ ID が何か所にもあれば、2つ目以降の場所ごとに duplicate_id を出す | 読み替え。IR の項目の見出しと場面の ID が、確認することの ID になる。場面の重複を "Scenario:" の行に出す部分（records.md の A72）は消える。重複を誤りにすること自体は `confirmations.ja.md` の C-core-confirm-3 にある |
| REQ-core-174 | - | - | 消える。宣言の外の行（unknown_line）、gherkin でないコードブロック（unknown_code_block）、用語集の題名（glossary_title_invalid）は IR の形にしか意味が無い。`decision-documents.ja.md` の「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」と「用語集は人が読む文書として残し、用語の検査はやめる」に置き換わる |
| TBL-core-008 | C-core-findings-1, C-core-findings-3, C-core-findings-4, C-core-findings-5, C-core-findings-6, C-core-findings-10, C-core-findings-11, C-core-findings-12, C-core-findings-13 | 指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める | 読み替え。行ごとの扱いは「形に依存する要求」の表。新しい版にある種類のうち、detail を利用者が決めた種類を1つ目の表、ほかの話題で決める種類と detail を利用者が決めていない種類を2つ目の表に置いた。missing_title は「提案」の決定に分けた（C-10）。id_domain_mismatch は利用者の答えで残し、「提案」の決定にした（C-11、C-12）。multiple_titles の detail（2026-09-23-ir-engine-gaps.md の A17、利用者）は、決定の文書でもそう動くので1つ目の表に置いた（C-13） |
| TBL-core-009 | C-core-findings-1, C-core-findings-7 | 指摘は誤りと注意に分け、注意にするのは表の種類だけにする | 読み替え。注意の表は D1、guide_stale の detail は D2 の表に置いた。too_many_lines と too_many_requirements の行は消える（利用者の答え）。mutant_timeout、equivalent_stale、surface_unspecified_stale の detail はそれぞれの話題の文書にある |
| EX-core-005 | C-core-findings-8 | 同じ ID が何か所にもあれば、2つ目以降の場所ごとに duplicate_id を出す | 読み替え。"REQ-001" の見出し3つを、同じ ID の確認することの行3つと読む |
| EX-core-266 | - | - | 消える（REQ-core-174 と同じ） |
| EX-core-376 | - | - | 消える。gherkin のブロックと、その中の行の検査は無くなる |
| EX-core-377 | - | - | 消える。用語集の題名は検査しない |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_008_invalid_marker_detail_of_a_crlf_test_file_has_no_carriage_return | REQ-core-072, TBL-core-008, TBL-core-010 | C-core-findings-3 | この話題の分（TBL-core-008）だけ。CRLF の行でも invalid_marker の detail が行の文字になる。REQ-core-072 と TBL-core-010 は「テストの印」と「文書の読み方」の話題 |
| crates/kotowari/src/ir_tests.rs | tbl_008_whole_document_detail_is_the_bare_filename_in_a_subdirectory | TBL-core-008, REQ-core-034, REQ-core-036, REQ-core-117 | C-core-findings-10 | この話題の分だけ。サブディレクトリの題名の無い文書の missing_title の detail がファイル名になる。「提案」の行なので実装のときに直す。準備を決定の文書にし、消える missing_scope と glossary_invalid の場合を外す |
| crates/kotowari/src/ir_tests.rs | req_032_first_occurrence_is_bytewise_first_relative_path | REQ-core-032 | C-core-findings-9 | "a.md" と "a/b.md" の同じ ID で、後の "a/b.md" に1件出る。準備が IR の要求なので、付け替えのときに確認することの行に直す |
| crates/kotowari-core/tests/ir.rs | req_043_unknown_heading_detail_is_full_heading_text | REQ-core-043, TBL-core-008 | - | unknown_heading は消える。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_043_unknown_heading_invalid_id_detail_is_full_heading_text | REQ-core-043, TBL-core-008 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_043_heading_without_colon_is_unknown | REQ-core-043, TBL-core-008 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_044_unknown_field_without_colon_has_line_text_as_detail | REQ-core-044, TBL-core-008 | - | unknown_field は消える。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_098_review_requirement_without_how_to_verify_is_a_missing_field | REQ-core-098, TBL-core-008, EX-core-259 | - | missing_field は消える。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_032_duplicate_id_on_each_later_place_with_its_line | REQ-core-032 | C-core-findings-8 | 3か所で2件、どれも line を持つ。IR の読み取りを直接呼ぶので、付け替えのときに決定の文書の読み取りで書き直す |
| crates/kotowari-core/tests/ir.rs | req_032_first_occurrence_is_bytewise_first_path | REQ-core-032 | C-core-findings-9 | "a.md" と "b.md" で "b.md" に出る。上と同じく書き直す |
| crates/kotowari-core/tests/ir.rs | req_044_unknown_field_detail_is_raw_line_not_reconstructed | REQ-core-044, TBL-core-008 | - | unknown_field は消える。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | tbl_008_missing_source_scenario_detail_is_raw_scenario_line | REQ-core-059, TBL-core-008 | - | missing_source は消える。古い形のコードと一緒に消す |
| tests/step2_ir.rs | req_032_three_places_yield_two_duplicates_and_none_on_the_first | REQ-core-032, EX-core-005 | C-core-findings-8 | 3か所で2件、1つ目の行には出ない。準備が IR の要求なので、付け替えのときに確認することの行に直す |
| tests/step4_test_discovery.rs | req_072_invalid_marker_detail_is_line_text | REQ-core-072, TBL-core-008 | C-core-findings-3 | この話題の分だけ。空のマーカーの detail が行の文字そのまま |
| tests/step4_test_discovery.rs | req_137_duplicate_scenario_uses_the_first_about | REQ-core-137, REQ-core-032 | - | 場面の ID の重複と "@about" は消える。古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_137_duplicate_scenario_reports_once_on_the_first | REQ-core-137, REQ-core-032 | - | 同上 |
| tests/step4_test_discovery.rs | req_085_duplicate_scenario_marker_uses_the_first_about | REQ-core-085, REQ-core-032 | - | 同上 |
| tests/step3_sources_terms.rs | tbl_008_unclosed_backtick_detail_keeps_leading_indentation | REQ-core-116, TBL-core-008 | - | unclosed_backtick は消える。古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_070_detail_is_the_whole_reference | REQ-core-069, REQ-core-070, TBL-core-014, TBL-core-008, EX-core-014 | - | missing_document は消える。古い形のコードと一緒に消す |
| tests/step15_guides.rs | ex_367_a_mark_without_fingerprint_and_an_uppercase_fingerprint_are_invalid | REQ-core-201, REQ-core-202, TBL-core-036, TBL-core-008, TBL-core-019, EX-core-367 | C-core-findings-3 | この話題の分（TBL-core-008）だけ。ガイドのマーカーの invalid_marker の detail が行の文字。準備の IR の文書とマーカーの行き先は、付け替えのときに決定の文書に直す |
| tests/step15_guides.rs | req_202_an_empty_mark_and_a_mark_without_its_closing_bracket_are_invalid | REQ-core-202, TBL-core-008 | C-core-findings-3 | この話題の分だけ。字下げを含む行の文字が detail になる |
| tests/step15_guides.rs | ex_363_a_changed_body_makes_the_mark_a_stale_notice | REQ-core-204, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-363 | C-core-findings-1, C-core-findings-2, C-core-findings-7 | この話題の分だけ。guide_stale が "[notice]" で出て終了コード0、detail が「行き先 書かれた指紋 今の指紋」。準備が IR の要求の本文の書き換えなので、付け替えのときに決定の「決めたこと」の書き換えに直す |
| tests/step15_guides.rs | req_204_a_duplicated_id_matching_neither_item_reports_the_first_fingerprint | REQ-core-204, REQ-core-032 | - | 2つの IR の文書に同じ ID があるときの今の指紋の選び方。決定の行き先は文書のパスと見出しで1つに決まるので、新しい形では起きない。古い形のコードと一緒に消す |
| tests/step17_surface.rs | ex_core_408_a_surface_absent_from_the_ir_is_an_error_at_its_first_place | EX-core-408, EX-core-409, REQ-core-227, TBL-core-019, TBL-core-006, TBL-core-008 | C-core-findings-6 | この話題の分（TBL-core-008）だけ。detail "flag --verbose" と severity "error"。準備が IR の要求なので、付け替えのときに決定の文書に直す |
| tests/step17_surface.rs | ex_core_421_an_entry_whose_reason_is_blank_is_invalid_and_removes_nothing | EX-core-421, REQ-core-233, REQ-core-027, TBL-core-006, TBL-core-008 | - | この話題の分（TBL-core-008）は surface_unspecified_invalid の detail で、surface-unspecified の決定（C-core-unspec-3）が持ち、そちらの対応表が付け替える |
| tests/step17_surface.rs | ex_core_422_an_entry_for_a_surface_gone_from_the_code_is_a_notice | EX-core-422, REQ-core-234, REQ-core-027, TBL-core-009 | C-core-findings-1, C-core-findings-2 | この話題の分だけ。surface_unspecified_stale が severity "notice" で、終了コード0 |
| tests/step16_deferred.rs | ex_387_a_second_document_level_declaration_is_a_duplicate_field | REQ-core-209, TBL-core-008, EX-core-387 | - | 後回しの宣言は消える。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | req_210_an_empty_declaration_is_a_missing_source_on_its_line | REQ-core-210, TBL-core-008, TBL-core-019 | - | 同上 |
| tests/step16_deferred.rs | ex_392_a_mark_on_a_deferred_requirement_is_a_notice_on_its_heading | REQ-core-211, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-392 | - | deferred_with_test は消える。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_394_a_statement_of_a_requirement_that_is_not_deferred_pointing_at_one_is_a_notice | REQ-core-212, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-394 | - | depends_on_deferred は消える。古い形のコードと一緒に消す |
| tests/step5_findings.rs | req_029_every_error_kind_has_the_detail_of_the_table | REQ-core-029, TBL-core-008 | C-core-findings-10 | 題名の無い文書の missing_title が detail "a.md"、severity "error"。「提案」の行なので実装のときに直す（準備を決定の文書にする） |
| tests/step5_findings.rs | req_030_notice_kinds_have_the_detail_of_the_table | REQ-core-030, TBL-core-009 | - | too_many_lines が severity "notice" で detail が行数であることを確かめる。too_many_lines は無くすので、古い形のコードと一緒に消す |
| tests/step5_findings.rs | req_031_only_eight_kinds_are_notices | REQ-core-031 | C-core-findings-1 | FindingKind::ALL の重さを見る。期待する集合に deferred_with_test、depends_on_deferred、too_many_lines、too_many_requirements が入っているので、古い種類を消すときに集合から外す |
| tests/step5_findings.rs | req_031_check_reports_notices_only_for_the_notice_kinds | REQ-core-031 | C-core-findings-1 | 出た指摘の重さが種類の表のとおり。準備が IR の要求の数と行数の超過なので、付け替えのときに、残る注意の種類（例: guide_stale）を出す準備に直す |
| tests/step5_findings.rs | req_174_a_line_outside_the_declaration_below_a_section_heading_is_unknown_line | REQ-core-174 | - | 消える。古い形のコードと一緒に消す |
| tests/step5_findings.rs | req_174_an_undeclared_table_or_code_block_is_also_unknown_line | REQ-core-174 | - | 同上 |
| tests/step5_findings.rs | req_174_a_non_gherkin_code_block_under_examples_is_unknown_code_block | REQ-core-174 | - | 同上 |
| tests/step5_findings.rs | req_174_a_glossary_title_outside_the_declared_form_is_glossary_title_invalid | REQ-core-174 | - | 同上 |
| tests/step5_findings.rs | ex_core_376_lines_in_a_non_gherkin_block_are_not_read_as_gherkin | REQ-core-174, REQ-core-113, EX-core-376 | - | 同上 |
| tests/step5_findings.rs | req_174_a_gherkin_block_beside_a_non_gherkin_block_is_still_read_as_gherkin | REQ-core-174, REQ-core-113 | - | 同上 |
| tests/step5_findings.rs | ex_core_377_a_glossary_titled_other_than_glossary_is_glossary_title_invalid | REQ-core-174, EX-core-377 | - | 同上 |
| tests/step5_findings.rs | ex_core_266_the_three_places_outside_the_declaration_each_become_an_error | EX-core-266 | - | 同上 |
| tests/step23_decision_documents.rs | two_confirmations_with_the_same_id_are_an_error | C-core-confirm-3（この話題の ID を指していない） | C-core-confirm-3, C-core-findings-9 | この話題の ID を今は指していないが、決定の文書の形で、2つの文書の同じ ID の行のうち、パスのバイト順で後の "wave.md" の行に duplicate_id が出ることを確かめている。マーカーに C-core-findings-9 を足すと、新しい形の準備で C-9 を確かめるテストになる |
| tests/step23_decision_documents.rs | a_confirmation_row_without_a_well_formed_id_is_an_error | C-core-confirm-3（この話題の ID を指していない） | C-core-confirm-3, C-core-findings-4 | 形の誤った ID の行に、書かれた値を detail にした invalid_id が出る。マーカーに C-core-findings-4 を足す |
| tests/step23_decision_documents.rs | a_mark_pointing_at_an_id_in_no_document_is_an_error | C-core-confirm-4（この話題の ID を指していない） | C-core-confirm-4, C-core-findings-4 | 無い ID を指すマーカーに、その ID を detail にした unresolved_reference が出る。マーカーに C-core-findings-4 を足す |
| tests/step23_decision_documents.rs | a_guide_mark_pointing_at_no_decision_is_stale_with_no_current_fingerprint | C-core-lang-5（この話題の ID を指していない） | C-core-lang-5, C-core-findings-7 | 決定を指す行き先で、無い決定の guide_stale の今の指紋が "-" になる。マーカーに C-core-findings-7 を足す |
| tests/step23_decision_documents.rs | a_document_without_a_title_is_an_error | C-core-docs-8（この話題の ID を指していない） | C-core-docs-8, C-core-findings-10 | 題名の無い決定の文書の missing_title を確かめている。今は detail に空の文字列を期待しているので、「提案」の行の実装のときに "greet.md" に直す |
| tests/step23_decision_documents.rs | each_title_after_the_first_is_an_error_without_a_line | C-core-docs-1（この話題の ID を指していない） | C-core-docs-1, C-core-findings-13 | 2つ目と3つ目の題名（"#  二つ目 "、"# 三つ目"）への multiple_titles の detail が、"#" と前後の空白を除いた題名の文字になる。マーカーに C-core-findings-13 を足す |
| tests/step13_test_languages.rs | ex_core_310_test_without_a_name_uses_its_first_line | REQ-core-086, EX-core-310（この話題の ID を指していない） | C-core-findings-5 | この話題の ID を今は指していないが、名前の取れないテストの test_without_id の detail が節の最初の行の全体の文字になることを確かめている。マーカーに C-core-findings-5 を足す。REQ-core-086 と EX-core-310 は「要求とテストの対応」の話題 |

## 戻した細部

無い。安全に関わる落とした細部はこの話題に無い。

## 落とした細部

### 安全に関わるもの

無し。この話題が決めるのは指摘の重さと detail の文字で、落とした細部はどれも detail の書き方である。注意と誤りの分け方（終了コードを変えるか）は、利用者の決定（records.md の A29、2026-09-16-notice.md の A2、2026-09-24-doc-marks.md の A8、2026-09-27-surface-check.md の A6）として残した。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-core-008: overview_part_unknown、overview_part_invalid（"(root)"、"(yaml)" を含む）、overview_lead_missing、overview_ir_missing、overview_ir_shared、overview_ref_unresolved、overview_name_conflict の detail | LLM（2026-10-02-whole-picture.md の A73、A74。「承認の時に利用者に示す」）。種類の名前は A69（LLM が導き、利用者が第21ラウンドで確認）なので、2つ目の表に名前と意味だけを残した |
| TBL-core-008: overview_form_invalid の、frontmatter でない違反の detail（形の検査の側の指摘の種類の名前） | LLM（2026-10-02-whole-picture.md の A71。「承認の時に利用者に示す」）。frontmatter の違反の "frontmatter" は利用者（2026-10-04-overview-on-public-api.md の A11）なので残した |
| TBL-core-008: overview_toc_invalid と overview_toc_group_empty の "(root)"、知らない鍵と欠けた鍵での鍵の名前の足し方 | LLM（2026-10-05-overview-index.md の A34） |
| TBL-core-008: translation_missing、translation_record_invalid（"missing"、"yaml"、"keys"、"value"）、translation_stale、translation_structure_mismatch、translation_switcher_invalid、link_language_mismatch の detail | LLM（2026-10-05-localization.md の D3。D1 で利用者が細部を LLM に任せた）。種類の名前は利用者（A27、A33）なので、2つ目の表に名前と意味だけを残した |
| TBL-core-008: id_domain_mismatch の detail | 記録に無い（2026-09-22-id-namespace.md の A4 は「新しい種類にし、invalid_id を使い回さない」まで）。実装する側が決める細部に挙げた |
| TBL-core-009: guide_stale の、IR の同じ ID が2つの文書にあるときに1つ目の指紋を使うこと | テスト（req_204_a_duplicated_id_matching_neither_item_reports_the_first_fingerprint）にだけある。新しい形では起きないので落とした |
| TBL-core-008 の注記: detail が「見出しの文字」「Scenario: の行の文字」の種類も行の文字そのままにすること | 実験者の裁定（records.md の A150、第9ラウンドの続き）。対象の種類はどれも消える |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-174、EX-core-266、EX-core-376、EX-core-377 | 消える。unknown_line、unknown_code_block、glossary_title_invalid は IR と用語集の形の検査。`decision-documents.ja.md` の「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」と「用語集は人が読む文書として残し、用語の検査はやめる」に置き換わる |
| REQ-core-032 のうち、場面の重複を "Scenario:" の行に出すこと | 消える。場面は無くなる |
| TBL-core-008 の missing_scope | 消える。`decision-documents.ja.md` の「1つの決定は…」の、題名のあとの段落の検査（topic_lead_missing、C-core-docs-8）に置き換わる |
| TBL-core-008 の multiple_titles | 新しい形で読み替えて残る（実装済み）。出る条件は ir-document の「決定の文書の題名は1つにし、2つ目からの題名は multiple_titles の誤りにする」（C-core-document-5）、detail はこの文書の1つ目の表（C-core-findings-13） |
| TBL-core-008 の unclosed_code_block | 新しい形で読み替えて残る（実装済み）。ir-input の「閉じないコードブロックは unclosed_code_block の誤りにし、開始から文書の終わりまでを読まない」（C-core-input-3、C-core-input-4）。2つ目の表に名前とリンクを置いた |
| TBL-core-008 の unknown_heading、unknown_field、missing_field、missing_table、duplicate_field、missing_statement、verification_missing、verification_invalid、unknown_kind、algorithm_without_definition | 消える。IR の項目の形の検査。`decision-documents.ja.md` の「1つの決定は…」（decision_state_invalid、decision_section_missing）と、`confirmations.ja.md` の「テストで確かめない行は、頭に「（読んで確認）」と書く」に置き換わる |
| TBL-core-008 の missing_tag、unknown_tag、invalid_gherkin_line | 消える。場面の gherkin のブロックの検査。決定の文書は Markdown として読み、場面を書かない |
| TBL-core-008 の link_to_record | 新しい形で読み替えて残る（利用者の答え。提案）。判断の記録へのリンクの誤りを、決定の文書とその対の側から凍結した木（"docs/frozen/"）の下へのリンクの誤りに読み替える。名前を link_to_record のまま読み替えるか改名するかは実装する側が決める。この指摘を出すコードは対の検査（`crates/kotowari-core/src/translations/check.rs`）にあるので、決定は translation-structure の「決定の文書とその対の側から、凍結した木へのリンクは誤りにする」（C-core-pairstruct-17）にあり、2つ目の表からその見出しを指した |
| TBL-core-008 の missing_source、source_invalid、record_field_missing、record_field_unknown、revision_link_invalid | 消える。出典の行と判断の記録の形。`decision-documents.ja.md` の「仕様は、話題ごとの、今有効な決定の文書だけで持つ」と「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」に置き換わる |
| TBL-core-008 の unknown_term、unclosed_backtick、glossary_invalid、invalid_glossary_row、duplicate_term | 消える。用語の検査と用語集の形。`decision-documents.ja.md` の「用語集は人が読む文書として残し、用語の検査はやめる」に置き換わる |
| TBL-core-008 の missing_document | 消える。文書名の参照。`decision-documents.ja.md` の「ほかの決定を指すときは、見出しへリンクを張る」（link_target_missing）に置き換わる |
| TBL-core-008 の requirement_without_test、scenario_without_test | 新しい形で読み替えて残る。confirmation_without_test の1つになる（`confirmations.ja.md` の「check は、確認することの行ごとに、それを指すテストがあるかを数える」）。2つ目の表にその名前を置いた |
| TBL-core-008 の vague_word | 新しい形で読み替えて残る。検査する文が IR の要求の文から、決定の「決めたこと」と「確認すること」になる（`decision-documents.ja.md` の「用語集は…」。状態は提案）。2つ目の表に名前と意味だけを置いた |
| TBL-core-008 の id_domain_mismatch | 新しい形で読み替えて残る（利用者の答え。提案、C-core-findings-11、C-core-findings-12）。IR の項目の ID の名前を、確認することの ID のまとまりの名前と読み、置き場の直下の文書は見ない |
| TBL-core-008 の overview_ir_missing、overview_ir_shared | 新しい形で読み替えて残る。一覧が指すものが IR の文書から決定の文書になる（`languages-and-guides.ja.md` の「全体像の元データは、決定の文書を指す」。状態は提案）。利用者の答えで改名する。2つ目の表には今の名前を「改名する」と書いて置いた |
| TBL-core-008 の duplicate_id、invalid_id、unresolved_reference | 新しい形で読み替えて残る。ID が IR の項目の ID から確認することの ID になる。決定の文書を読む今のコードはこの3つを同じ detail で出している |
| TBL-core-008 の missing_title | 新しい形で読み替えて残るが、決定の文書を読む今のコードは detail を空の文字列にしている。「提案」の決定に分けた |
| TBL-core-009 の too_many_requirements、too_many_lines | 消える（利用者の答え）。ir-document の「行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす」（提案）。古い形のコードがまだ持つ |
| TBL-core-009 の deferred_with_test、depends_on_deferred | 消える。`confirmations.ja.md` の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」に置き換わる |
| TBL-core-009 の guide_stale | 新しい形で読み替えて残る。行き先が IR の ID から決定の行き先（文書のパスと見出し）になる。決定の文書を読む今のコードがもうそう動いている（`languages-and-guides.ja.md` の「ガイドは、決めたことと確認することが変わったら古いとする」） |

テストの無い行は無い。C-core-findings-4、C-core-findings-5、C-core-findings-7、C-core-findings-9、C-core-findings-10、C-core-findings-13 は、上のテストごとの表のとおり、今この話題の ID を指していないテストにマーカーを足すと、新しい形の準備で確かめるテストになる。C-core-findings-11 と C-core-findings-12 は提案の行で、テストは実装のときに作る（今の古い形のテスト req_167_id_name_must_match_the_first_directory_segment は、ir-references の対応表が C-core-findings-11 に付け替える）。

## 実装する側が決める細部

- 2つ目の表の種類（全体像の元データ、対、決定の文書の形の指摘）の detail。決定の文書の形の指摘は、今のコードでは decision_state_invalid が書かれた状態の値（状態の行が無ければ空の文字列）、decision_section_missing が無い節の名前、topic_lead_missing が題名の文字、confirmation_without_test が確認することの ID、link_target_missing が書かれた行き先、unclosed_code_block が開始の行の文字である。
- 決定の文書が Markdown として読めないときの unparsable_file の detail。今のコードは読み取りの誤りの文を入れているが、表は「ファイルのパス」である（迷った判断を参照）。
- overview_ir_missing と overview_ir_shared の新しい名前（案: overview_document_missing、overview_document_shared）。元データのキー "ir" の新しい名前（案: documents）も一緒に決める。
- id_domain_mismatch の detail と line。古い形では line が見出しの行（"@id" の値ならタグの行）だったので、確認することの行に読み替える案がある。
- 凍結した木へのリンクの指摘の名前を、link_to_record のまま読み替えるか、改名するか。

## 利用者に聞くこと

無い。答えの出た問いは「迷った判断」に書いた。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えることになりそうか |
|---|---|
| 題名の無い決定の文書の missing_title は、detail を文書名にする | `crates/kotowari-core/src/decisions.rs` の `parse` の、題名が無いときの `FindingKind::MissingTitle` の detail を、パスからディレクトリを除いた名前にする |
| 確認することの ID のまとまりの名前が、置いてあるまとまりのフォルダと合わなければ、id_domain_mismatch の誤りにする | `crates/kotowari-core/src/decisions.rs` の `check_ids` で、置き場からの相対パスの最初のディレクトリと ID のまとまりの名前を比べ、`FindingKind::IdDomainMismatch` を積む（古い形の `crates/kotowari-core/src/ir/check.rs` の比べ方を移す） |

## 誰が決めたか見分けられなかったもの

- 2026-09-24-doc-marks.md の A31（ガイドのマーカーの形の誤りの detail を、その行の文字にする）は「IR を書く段で推奨を採用（照合レビューの指摘から。承認で利用者が確かめる）」である。テストのマーカーの detail を行の文字にすること（records.md の A111、第9ラウンド）と同じ形なので、invalid_marker の行の detail は1つの行にまとめ、ガイドのマーカーにも当てた（C-core-findings-3）。
- records.md の第9ラウンド（A98〜A144 のラウンドの本体）は、決定の行ごとの decided_by が無く、見出しに「すべて推奨をそのまま採った（利用者の答えは「進めて」）」とある。書き直しの決まりの読み方どおり、利用者の決定に数えた（A111 の行の文字、A113 の duplicate_id の1つ目）。同じ見出しの「続き」（A145 以降。実験者が A74 に従って裁定）は、実験者の決定として落とした（A150、A153、A154）。finding-order の対応表も、この読み方に揃えた。
- 2026-10-02-whole-picture.md の A69（全体像の元データの指摘の種類の名前）は「LLM（決定から導いた細部。利用者が第21ラウンドで確認した）」である。利用者が確認したものとして、名前を残した。

## 理由や見送った案が記録に無かった決定

- 「同じ ID が何か所にもあれば、2つ目以降の場所ごとに duplicate_id を出す」: 理由も見送った案も記録に無い（records.md の A47 の (5)、A61、A113）。問題の節に「理由は残っていない」と書いた。
- 「題名の無い決定の文書の missing_title は、detail を文書名にする」: 理由も見送った案も記録に無い（2026-09-16-ir-tree.md の A19 は「区別は path が付ける」だけ）。
- 「指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める」: 見送った案は frontmatter の detail の1つだけが記録にある（2026-10-04-overview-on-public-api.md の A11）。
- 「確認することの ID のまとまりの名前が…」: 理由と見送った案は 2026-09-22-id-namespace.md の A3 にあるものを書いた。置き場の直下の文書を見ないことは書き直しの依頼への利用者の答えで、理由は記録に無い。

## 分けた所、まとめた所

- REQ-core-029、REQ-core-030、REQ-core-031、TBL-core-008、TBL-core-009 を、重さの決定（D1）と detail の決定（D2）の2つにまとめ直した。重さは注意の表と「ほかはすべて誤り」の文で決まり、detail は種類ごとの表で決まるので、確かめる中身が分かれる。
- TBL-core-008 を、この文書で detail を決める種類の表と、ほかの話題で決める種類と detail を利用者が決めていない種類の表の2つに分けた。後者は、名前を残すためと、どこで決めるかを示すために置いた。
- missing_title を、決定の文書を読む今のコードと合わないので、「提案」の決定に分けた（D4）。
- REQ-core-032 を、`confirmations.ja.md` にある「重複は誤り」とは別に、どこに何件出すかの決定（D3）にした。
- id_domain_mismatch を、指摘の種類のこの文書に「提案」の決定として置いた（D5）。ID の形を決めているのは core の `confirmations.ja.md` だが、既にリポジトリにある文書でこの依頼の外なので、指摘が見えるこちらに置いた。
- mutant_timeout、mutant_survived、equivalent_stale、equivalent_invalid、invalid_plan、surface_unspecified_stale、surface_unspecified_invalid の detail は、書き直し済みの話題の文書が持っているので、表では名前と行き先だけにした。

## あとでリンクにする所

- 済み: `confirmations.ja.md`、`decision-documents.ja.md`、`languages-and-guides.ja.md`、cli、ir-document、ir-input、overview-data、overview-commands、translation-pairs、translation-structure の見出しへのリンクにした。
- 済み: link_to_record の行は、translation-structure の「決定の文書とその対の側から、凍結した木へのリンクは誤りにする」へのリンクにした。

## 迷った判断と、どう決めたか

- multiple_titles の detail（題名の文字、"#" を除く）は、前の版では ir-document の対応表が IR のエンジンの細部として落としていた。記録（2026-09-23-ir-engine-gaps.md の A17）の decided_by は利用者（推奨を採用）で、539b7c1 で決定の文書でも同じ detail を出すようになったので、1つ目の表に置き、C-13 にした。

- 問い1（行数と要求の数の上限の注意）の答え: too_many_lines は決定の文書ではやめ、too_many_requirements と "limits.requirements"、"limits.lines" も無くす。決定は ir-document に「無くす」で置き、注意の表には足さなかった。
- 問い2（id_domain_mismatch）の答え: 残す。置き場の直下の文書は見ない。今のコードは決定の文書に当てていないので「提案」の決定 D5 にした。
- 問い3（凍結した木へのリンク）の答え: 決定の文書とその対の側から凍結した木の下へのリンクは誤りにする（提案）。指摘の名前は実装する側が決める。振る舞いが見えるのは対の検査とリンクの話題なので、決定は translation-structure の側にあり（C-core-pairstruct-17）、ここでは2つ目の表に今の名前とその見出しへのリンクを置いた。
- overview_name_conflict の決める所を、全体像の元データの話題でなく、それを決めている overview-commands の「overview build は、毎回すべてを描き…」に直した。ほかの全体像と対の種類も、それぞれを決めている見出しへのリンクにした。
- 2つ目の表に、出る条件をこの文書で決めない種類まで並べた。依頼の「新しい版にある種類だけを表に残す」と、利用者から見える名前を文の中に残す決まりの両方を満たすためである。detail を利用者が決めていない種類は、detail を書かず、意味と決める所だけにした。
- 決定の文書を Markdown として読めないときの unparsable_file の detail は、今のコードでは読み取りの誤りの文で、表の「ファイルのパス」と合わない。Markdown の読み取りは今の設定では誤りを返さないので、実際には出ないと見て、表は今のまま「ファイルのパス」とし、実装する側が決める細部に挙げた。
- 「注意は8つだけ」の数は、新しい版で変わるので書かなかった。
- 注意が終了コードを変えないことは、用語集の「注意」の定義（2026-09-16-notice.md の A2）で、用語集の形には依存しない中身なので、決定の文に入れ、C-core-findings-2 にした。
- 記録の why にあるツールの名前とプロジェクトの名前は、文書に書かなかった（frontmatter の detail の見送った案は「形の検査の側」と書いた）。

## 数

- 元: 要求 5、場面 4、決定表 2、性質 0
- 読んだ記録のファイル: 22（records.md、ir-form.md、2026-09-16-ir-tree.md、2026-09-16-notice.md、2026-09-17-mutation-tests.md、2026-09-17-record-form.md、2026-09-17-scenario-tests.md、2026-09-22-id-namespace.md、2026-09-22-ir-engine.md、2026-09-23-ir-engine-gaps.md、2026-09-23-ir-english-tokens.md、2026-09-24-doc-marks.md、2026-09-24-guide-gaps.md、2026-09-24-multi-language-tests.md、2026-09-24-plan-schema.md、2026-09-25-deferred-items.md、2026-09-27-surface-check.md、2026-10-02-whole-picture.md、2026-10-04-overview-on-public-api.md、2026-10-05-overview-index.md、2026-10-05-localization.md、2026-10-06-todo-zero.md のうち、出典に引かれた決定の行）
- 出来上がり: 決定 5（実装済み 3、提案 2）、確認することの行 13（うち「（読んで確認）」0）
- テスト: 42（この話題の ID を指すもの。付け替え先のあるもの 15、「-」27）。ほかに、この話題の ID を指していないが付け替えを勧めるもの 7
- 利用者に聞くこと: 0
- 戻した安全の細部: 0
