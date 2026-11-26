# guides の対応表

元: `docs/ir/core/guides.md`（日本語は `guides.ja.md`）。書き直した文書: `guides.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-198 | C-core-guides-5, C-core-guides-6, C-core-guides-7, C-core-guides-8 | ガイドは guides.files の glob で選び、check と status だけが読む | 読み替え。鍵が無いときを C-5、読み方の失敗と停止を C-6、glob が仕様の置き場に当たるときを C-7（「IR や判断の記録の置き場」を「決定の文書の置き場」に読み替えた）、list と query が読まないことを C-8 にした。走査と除外を "tests.files" と同じにすること（A29）は落とした細部 |
| REQ-core-199 | C-core-guides-9, C-core-guides-10, C-core-guides-23, C-core-guides-24 | ガイドとテストの置き場が重なると、設定の誤りで停止する | そのまま。停止と詳細の文言を C-9、バイト順で最初の1つを C-10 にした。対の側を重なりに数える部分（2026-10-23-guide-overlap-sides.md の A1、整合のフェーズ）は、安全に関わる細部として決定に戻した（C-23、C-24） |
| REQ-core-200 | C-core-guides-11, C-core-guides-12, C-core-guides-13 | ガイドのマーカーは、HTML のコメントの中だけから読む | そのまま。1つのコメントと1行のマーカーをすべて拾うこと（A30）は落とした細部で、C-11 のテストの中に残る |
| REQ-core-201 | C-core-guides-14, C-core-guides-15, C-core-guides-17 | ガイドのマーカーの1件は、決定への行き先と指紋をコロンでつなぐ | 読み替え。定義の TBL-core-036 を、"ID:指紋" から "<行き先>:<指紋>" に読み替えて表にした |
| REQ-core-202 | C-core-guides-15, C-core-guides-16 | ガイドのマーカーの1件は、決定への行き先と指紋をコロンでつなぐ | 読み替え。「":" の前が ID の形でない」を「":" の前が行き先の形でない」に読み替えた。1件でも形が誤ればどの1件も照らさないこと（A31）は落とした細部（下書きでは C-15 の文に残していたが、確定の段で外した。下の「迷った判断」）。detail と line の形（A31、A32）は落とした |
| REQ-core-203 | C-core-guides-1, C-core-guides-2, C-core-guides-3, C-core-guides-4 | 古い節は、マーカーに書いた指紋を今の決定の指紋と照らして見つける | 読み替え。query が指紋を取る行を本文に返すことは、利用者の答え（query で決定を引くと決定の全体を返す。query の話題の提案の決定）に合わせて、この話題の文と C-1 から外した。SHA-256 の先頭8文字（A7）と行の終わりの文字を入れないこと（A26）を残した。指紋を取る行（項目の本文から出典の行を除く、シナリオのステップの行、後回しの宣言の行）は消える。指紋を取る行は `docs/decisions/core/languages-and-guides.md` の「ガイドは、決めたことと確認することが変わったら古いとする」に置き換わった |
| REQ-core-204 | C-core-guides-18, C-core-guides-19, C-core-guides-20 | 古いマーカーは guide_stale の注意にし、終了コードと complete を変えない | 読み替え。「ID の項目かシナリオが IR に無い」を「行き先の決定が無い」に読み替えた。同じ ID が2か所にあるときの扱い（A20、A34）は消える（行き先は文書のパスと見出しの行き先の名前で1つに決まる） |
| REQ-core-205 | C-core-guides-21, C-core-guides-22 | ガイドを書く場面と見直す場面を、kotowari の skill に置く | 読み替え。検証が review なので「（読んで確認）」。how_to_verify の「項目を kotowari query で確かめる」を「決定を kotowari query で確かめる」に読み替えた |
| TBL-core-036 | C-core-guides-14, C-core-guides-15, C-core-guides-16, C-core-guides-17 | ガイドのマーカーの1件は、決定への行き先と指紋をコロンでつなぐ | 読み替え。表は決めたことに置いた。中身の "ID:指紋" を "<行き先>:<指紋>" にし、行き先の行を足した。行き先の形の細かい制限（".md" で終わる、空白と2つ目の "#" を含まない）は落とした細部 |
| EX-core-362 | C-core-guides-3 | 古い節は、マーカーに書いた指紋を今の決定の指紋と照らして見つける | 読み替え。"guides" の "files" と "marks" の数は出力の話題の分 |
| EX-core-363 | C-core-guides-18, C-core-guides-20 | 古いマーカーは guide_stale の注意にし、終了コードと complete を変えない | 読み替え。「本文を変える」を「決めたことを変える」と読んだ。「問題の記録が無ければ」は「未決が無ければ」に当たる |
| EX-core-364 | - | - | 消える。見出しの名前と出典の行を変えても古くならないこと。出典の行は無くなり、決定の見出しを変えると行き先が変わる。決定のどこが変わったら古いとするかは `docs/decisions/core/languages-and-guides.md` の「ガイドは、決めたことと確認することが変わったら古いとする」に置き換わった |
| EX-core-365 | C-core-guides-19 | 古いマーカーは guide_stale の注意にし、終了コードと complete を変えない | 読み替え。「IR から消した項目」を「無い決定」と読んだ |
| EX-core-366 | C-core-guides-12 | ガイドのマーカーは、HTML のコメントの中だけから読む | そのまま |
| EX-core-367 | C-core-guides-15 | ガイドのマーカーの1件は、決定への行き先と指紋をコロンでつなぐ | 読み替え。終了コード1と text の行の形は出力の話題の分 |
| EX-core-368 | C-core-guides-9 | ガイドとテストの置き場が重なると、設定の誤りで停止する | そのまま |
| EX-core-369 | C-core-guides-5 | ガイドは guides.files の glob で選び、check と status だけが読む | そのまま。"guides" の数は出力の話題の分 |
| EX-core-370 | - | - | 消える。同じ ID が2か所にあるときの一致の判定（A20）。行き先は1つの決定に決まる |
| EX-core-371 | C-core-guides-17 | ガイドのマーカーの1件は、決定への行き先と指紋をコロンでつなぐ | 読み替え。"marks" が2になることは出力の話題の分 |
| EX-core-372 | C-core-guides-2 | 古い節は、マーカーに書いた指紋を今の決定の指紋と照らして見つける | 読み替え。要求の1件を決定の1件に読み替えた |
| EX-core-373 | C-core-guides-15 | ガイドのマーカーの1件は、決定への行き先と指紋をコロンでつなぐ | 読み替え |
| EX-core-374 | C-core-guides-13, C-core-guides-14 | ガイドのマーカーは、HTML のコメントの中だけから読む | そのまま。空白を許すことが C-14（見出しは「ガイドのマーカーの1件は…」）、コメントの後ろを読まないことが C-13 |
| EX-core-375 | - | - | 消える。シナリオの名前とタグを変えても古くならないこと。シナリオは無くなる |

元の文書に PROP は無い。

ガイドのマーカー: `docs/guides/writing-guides.md`（と `.ja.md`）、`docs/guides/config.md`、`docs/guides/findings.md`、`docs/guides/deferred.md`、`docs/guides/commands/list.md`、`docs/guides/commands/query.md`、`docs/guides/commands/status.md`（それぞれ `.ja.md` も）にある、この話題の ID のマーカーは、上の「決定の見出し」で付け替える。EX-core-364、EX-core-370、EX-core-375 と、REQ-core-203 の指紋を取る行の部分（`docs/guides/deferred.md` の後回しの宣言と指紋の節）は、`docs/decisions/core/languages-and-guides.md` の「ガイドは、決めたことと確認することが変わったら古いとする」へ付け替えるか、古い形の説明として節ごと消す。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step15_guides.rs | overlap_precedes_guide_only_invalid_utf8_and_missing_surface_rules | REQ-core-199, REQ-core-313, EX-core-485 | C-core-guides-9 | 重なりの停止が、ほかの停止より先に出ることを確かめる。この話題の分は重なりの停止と文言だけ。REQ-core-313 と EX-core-485 は別の話題 |
| tests/step15_guides.rs | ex_372_list_shows_the_fingerprint_of_a_requirement | REQ-core-203, TBL-core-026, EX-core-372 | - | 要求の指紋を確かめる。古い形のコードと一緒に消す。決定の1件の分は step23 の list_includes_each_decision_with_its_fingerprint が確かめる |
| tests/step15_guides.rs | req_203_a_table_keeps_its_leading_blank_line_after_the_source_line_is_dropped | REQ-core-203, TBL-core-026 | - | 出典の行を除いた本文の指紋。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_203_a_scenario_fingerprint_is_its_step_lines_only | REQ-core-203, TBL-core-026 | - | シナリオの指紋。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_203_heading_name_and_source_do_not_change_the_fingerprint | REQ-core-203 | - | 出典の行と見出しの名前。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_203_scenario_name_and_tags_do_not_change_the_fingerprint | REQ-core-203 | - | シナリオのタグ。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_203_crlf_line_endings_do_not_change_the_fingerprint | REQ-core-203 | C-core-guides-4 | "\r\n" の文書と "\n" の文書で指紋が同じ。準備が IR の要求とシナリオなので、付け替えのときに、決定の文書を "\r\n" で書く準備に直す必要がある |
| tests/step15_guides.rs | req_203_a_document_level_declaration_leads_the_fingerprint_of_its_requirements_only | REQ-core-203, REQ-core-209 | - | 後回しの宣言の行。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_203_a_requirement_level_declaration_is_part_of_the_body_it_fingerprints | REQ-core-203, REQ-core-208 | - | 同上 |
| tests/step15_guides.rs | ex_368_check_stops_when_a_guide_is_also_a_test_file | REQ-core-199, TBL-core-001, TBL-core-020, EX-core-368 | C-core-guides-9 | check が終了コード2と詳細の文言で止まる。config error の前置きは「使い方の表示と対象の環境」の話題の分（その対応表に書いてある） |
| tests/step15_guides.rs | ex_368_status_stops_when_a_guide_is_also_a_test_file | REQ-core-163, REQ-core-199, EX-core-368 | C-core-guides-9 | status が同じく止まる。REQ-core-163 は status の話題 |
| tests/step15_guides.rs | req_199_only_the_first_overlapping_file_in_byte_order_is_reported | REQ-core-199, TBL-core-020 | C-core-guides-10 | 2つ重なるとき、バイト順で先の1つだけが出る |
| tests/step15_guides.rs | ex_369_without_the_guides_key_no_guide_is_read | REQ-core-198, REQ-core-206, TBL-core-005, EX-core-369 | C-core-guides-5 | 鍵が無いと guide_stale が出ない。準備のマーカーが "REQ-001:00000000" の古い形なので、付け替えのときに決定を指すマーカーに直す必要がある（古い形の1件の扱いによっては、今のままだと invalid_marker が出る）。"guides" の数は出力の話題の分 |
| tests/step15_guides.rs | req_198_a_guide_that_is_not_utf8_stops | REQ-core-198, TBL-core-001 | C-core-guides-6 | UTF-8 でないガイドで停止する。停止の文言は「使い方の表示と対象の環境」の話題の分 |
| tests/step15_guides.rs | req_198_a_guides_glob_may_hit_the_ir_documents | REQ-core-198, REQ-core-206 | C-core-guides-7 | glob が IR の文書に当たっても止まらず両方として読む。付け替えのときに、glob が決定の文書の置き場に当たる準備に直し、"decisions" の "documents" と "guides" の "files" が両方1になることを確かめる必要がある |
| tests/step15_guides.rs | req_200_the_ex_362_project_without_a_guide_mark_has_no_findings | REQ-core-198, REQ-core-200, REQ-core-206 | - | マーカーの無いガイドに指摘が無く "files" が1、"marks" が0。この話題の行には当たらない（数え方は出力の話題）。古い形のコードと一緒に消す |
| tests/step15_guides.rs | ex_366_marks_outside_comments_and_inside_code_are_not_read | REQ-core-200, EX-core-366 | C-core-guides-12 | コードブロックの2つの形、コードスパン、コメントの外。準備の IR を決定の文書に直す必要がある |
| tests/step15_guides.rs | ex_367_a_mark_without_fingerprint_and_an_uppercase_fingerprint_are_invalid | REQ-core-201, REQ-core-202, TBL-core-036, TBL-core-008, TBL-core-019, EX-core-367 | C-core-guides-15 | ":" の無い1件と大文字の指紋。準備のマーカーの ID を決定への行き先に直す必要がある。TBL-core-008、TBL-core-019 は別の話題（指摘の並べ方の対応表では「-」） |
| tests/step15_guides.rs | ex_373_one_malformed_entry_leaves_the_whole_mark_unmatched | REQ-core-201, REQ-core-202, TBL-core-036, EX-core-373 | C-core-guides-15 | 落とした細部のテスト。いちばん近い行を指す（1件の形の誤りでマーカーの全部を照らさないこと（A31）は落とした細部。形の誤った1件に invalid_marker を出す C-15 がいちばん近い）。準備のマーカーを決定への行き先に直す必要がある |
| tests/step15_guides.rs | ex_374_spaces_inside_the_brackets_are_allowed_and_text_after_the_comment_is_not_read | REQ-core-200, REQ-core-201, TBL-core-036, EX-core-374 | C-core-guides-13, C-core-guides-14 | 空白を許すことが C-14、コメントの後ろを読まないことが C-13。準備を決定の文書に直す必要がある |
| tests/step15_guides.rs | req_200_inline_comments_and_several_marks_in_one_comment_are_all_read | REQ-core-200, REQ-core-206 | C-core-guides-11 | 段落の中のコメントと行をまたぐコメントを読む。1つのコメントの2つのマーカー（A30）は落とした細部でテストの中に残る。準備を決定の文書に直す必要がある |
| tests/step15_guides.rs | req_200_each_comment_of_one_html_block_is_read_and_the_text_between_them_is_not | REQ-core-200, REQ-core-204 | C-core-guides-13 | 1つの HTML ブロックのコメントの間の文字を読まない。準備を決定の文書に直す必要がある |
| tests/step15_guides.rs | req_202_an_empty_mark_and_a_mark_without_its_closing_bracket_are_invalid | REQ-core-202, TBL-core-008 | C-core-guides-16 | 区切りだけのマーカーと閉じ括弧の無いマーカー。準備を決定の文書に直す必要がある |
| tests/step15_guides.rs | ex_362_a_mark_with_the_current_fingerprint_yields_nothing | REQ-core-198, REQ-core-200, REQ-core-203, REQ-core-204, REQ-core-206, EX-core-362 | - | 要求の指紋で古くないこと。決定の分は step23 の a_guide_mark_with_the_current_fingerprint_of_a_decision_is_not_stale が確かめる。古い形のコードと一緒に消す |
| tests/step15_guides.rs | ex_363_a_changed_body_makes_the_mark_a_stale_notice | REQ-core-204, REQ-core-031, TBL-core-009, TBL-core-019, EX-core-363 | C-core-guides-18, C-core-guides-20 | guide_stale の注意、終了コード0、status の complete が true。C-20 を確かめるテストはこれだけなので、付け替えのときに準備を決定の文書に直す必要がある。REQ-core-031、TBL-core-009、TBL-core-019 は別の話題 |
| tests/step15_guides.rs | ex_364_renaming_the_heading_and_adding_a_source_keeps_the_mark_fresh | REQ-core-203, REQ-core-204, EX-core-364 | - | 消える要求（出典の行）だけを確かめる。古い形のコードと一緒に消す |
| tests/step15_guides.rs | ex_365_a_mark_on_an_id_missing_from_the_ir_is_a_stale_notice | REQ-core-204, EX-core-365, TBL-core-006 | - | IR に無い ID。決定の分は step23 の a_guide_mark_pointing_at_no_decision_is_stale_with_no_current_fingerprint が確かめる。古い形のコードと一緒に消す |
| tests/step15_guides.rs | ex_370_matching_any_item_of_a_duplicated_id_keeps_the_mark_fresh | REQ-core-204, EX-core-370 | - | 消える要求（同じ ID が2か所）。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_204_a_duplicated_id_matching_neither_item_reports_the_first_fingerprint | REQ-core-204, REQ-core-032 | - | 同上 |
| tests/step15_guides.rs | ex_371_two_entries_in_one_mark_are_counted_and_matched_one_by_one | REQ-core-200, REQ-core-201, REQ-core-204, REQ-core-206, TBL-core-036, EX-core-371 | C-core-guides-17 | 2件を1件ずつ照らす。準備を決定2つの文書に直す必要がある。"marks" の数は出力の話題の分 |
| tests/step15_guides.rs | ex_375_renaming_a_scenario_and_changing_its_source_tag_keeps_the_mark_fresh | REQ-core-203, REQ-core-204, EX-core-375 | - | 消える要求（シナリオ）。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_204_the_line_of_guide_stale_is_where_the_mark_starts | REQ-core-204, TBL-core-019 | C-core-guides-18 | 行をまたぐコメントの中のマーカーの行に出る。準備を決定の文書に直す必要がある |
| tests/step18_overview.rs | req_core_280_the_guides_and_tests_overlap_is_judged_first_and_guides_win_when_all_three_match | REQ-core-280, REQ-core-199 | C-core-guides-9 | この話題の分は、ガイドとテストの重なりの停止と文言。REQ-core-280 は全体像の元データの話題 |
| tests/step19_overview_build.rs | req_core_280_build_judges_the_guides_and_tests_overlap_first | REQ-core-280, REQ-core-199 | C-core-guides-9 | 同上 |
| tests/step21_translations.rs | req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap | REQ-core-280, REQ-core-199, REQ-core-338 | C-core-guides-9, C-core-guides-24 | この話題の分だけ。言語を [ja, en] にし、先頭の言語の側が無い "shared/a.en.md" が3つの glob に当たるとき、check、status、overview build がガイドとテストの重なりの詳細で止まる。C-24 の戻した細部を確かめている。REQ-core-280、REQ-core-338 はほかの話題 |
| tests/step21_translations.rs | req_core_199_a_side_found_by_name_counts_as_a_guide_in_the_overlap | REQ-core-199, REQ-core-337 | C-core-guides-9, C-core-guides-23 | "guides.files" が "guides/a.md" だけで、対の側 "guides/a.en.md" が "tests.files" に当たると、check がその側の詳細で止まる。C-23 の戻した細部を確かめている。REQ-core-337 はほかの話題 |
| crates/kotowari-core/tests/admission.rs | a_guide_path_that_is_also_a_test_file_stops_the_native_inspection | REQ-core-199, REQ-core-315 | C-core-guides-9 | ライブラリの経路で、テストのファイルでもあるガイドのパスが config error になる。REQ-core-315 は別の話題 |
| tests/native_paths.rs | byte_named_files_without_legacy_read_aliases_keep_the_unreadable_stop | REQ-core-313, REQ-core-018, REQ-core-198, REQ-core-224, EX-core-485 | C-core-guides-6 | この話題の分は、ガイドの読めないファイルで停止すること。ほかの ID は別の話題 |
| tests/native_paths.rs | distinct_byte_named_entries_keep_the_legacy_alias_text_and_multiplicity | REQ-core-313, REQ-core-198, REQ-core-206, REQ-core-224, EX-core-485 | - | バイト列の名前のファイルの扱いと数え方を確かめる。この話題の行には当たらない（REQ-core-313 などの別の話題の分だけになる） |

この話題の ID を指すテストは 39。ほかに、この話題の ID を指していないが付け替えを勧めるものが 10（req_152 と step23 の9つ）。

## マーカーに足すテスト

この話題の ID を今は指していないが、付け替えたあとの確認することの行を確かめているテスト。今のマーカー（別の話題の ID）は残したまま、この話題の ID を足す。同じテストがほかの話題の対応表にも載っているときは、それぞれの表の ID を合わせて足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 足す ID | 理由 |
|---|---|---|---|---|
| tests/step15_guides.rs | req_152_list_and_query_do_not_read_guides_and_do_not_stop_on_their_overlap | REQ-core-152, REQ-core-158 | C-core-guides-8 | この話題の ID を今は指していないが、list と query が UTF-8 でないガイドで止まらないことを確かめている。REQ-core-152 と REQ-core-158 は list と query の話題の分で、そちらの対応表の付け替え先に置き換える。準備が IR の要求なので、付け替えのときに決定の文書の準備に直す必要がある |
| tests/step23_decision_documents.rs | a_guide_mark_with_the_current_fingerprint_of_a_decision_is_not_stale | C-core-lang-5 | C-core-guides-3 | 決定の今の指紋を書いたマーカーに指摘が出ない。C-core-guides-3 を足すと、新しい形で C-3 を確かめるテストになる |
| tests/step23_decision_documents.rs | changing_what_was_decided_makes_the_guide_mark_stale | C-core-lang-5 | C-core-guides-18 | guide_stale が1件で、detail が行き先、書かれた指紋、今の指紋 |
| tests/step23_decision_documents.rs | a_guide_mark_pointing_at_no_decision_is_stale_with_no_current_fingerprint | C-core-lang-5 | C-core-guides-19 | 無い見出し、決定でない見出し、無い文書で "-"、unresolved_reference が出ない |
| tests/step23_decision_documents.rs | a_repeated_heading_is_pointed_at_with_the_numbered_anchor | C-core-lang-5 | C-core-guides-3 | 同じ見出しの2つ目を "-1" の行き先で指し、今の指紋なら指摘が出ない |
| tests/step23_decision_documents.rs | a_guide_mark_holds_marks_of_the_old_form_and_of_decisions_together | C-core-lang-5 | - | 古い形の1件と決定の1件を1つのマーカーに並べられることを確かめる。古い形の1件を受けることは切り替えで無くなるので、この話題の行は足さない。古い形の1件の扱いを決めるときに、このテストを直すか消す |
| tests/step23_decision_documents.rs | a_decision_entry_without_an_anchor_or_with_a_bad_fingerprint_is_an_invalid_mark | C-core-lang-5 | C-core-guides-15 | 見出しの行き先の名前が無い、パスが無い、大文字の指紋で invalid_marker |
| tests/step23_decision_documents.rs | query_of_a_decision_returns_the_fingerprint_a_guide_mark_takes | C-core-lang-5 | C-core-guides-1 | query の1件の kind と fingerprint。body も確かめているが、C-1 からは外した（query の話題の提案で決定の全体に変わる） |
| tests/step23_decision_documents.rs | list_includes_each_decision_with_its_fingerprint | C-core-lang-5 | C-core-guides-2 | list が決定の行き先を id にした1件を出す |
| tests/step23_decision_documents.rs | a_decision_entry_outside_a_markdown_path_or_with_a_second_hash_or_a_space_is_an_invalid_mark | C-core-lang-5 | C-core-guides-15 | 行き先の形の誤りで invalid_marker。".md"、2つ目の "#"、空白の制限は落とした細部で、テストの中に残る |

## 落とした細部

### 安全に関わるもの

無し（あった1件は、下の「戻した細部」のとおり決定に戻した）。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-199: 重なりの判定に、"guides.files" の glob に当たったファイルに加えて、同じ対にあるためにガイドとして読む側（glob に当たらなくても）と、先頭の言語の側が無い側を数える。記録では整合のフェーズ（2026-10-23-guide-overlap-sides.md の A1。根拠は実測） | C-core-guides-23、C-core-guides-24（決定「ガイドとテストの置き場が重なると、設定の誤りで停止する」） | テスト tests/step21_translations.rs の req_core_199_a_side_found_by_name_counts_as_a_guide_in_the_overlap（C-23）と req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap（C-24） |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-198: ガイドの走査と除外（隠しディレクトリ、ディレクトリのシンボリックリンク、ディレクトリでも通常のファイルでもないもの）を "tests.files" と同じにする | IR を書く段で推奨を採用（2026-09-24-doc-marks.md の A29。「承認で利用者が確かめる」）。読み方の失敗を同じにすること（A16、利用者）は残した |
| REQ-core-200: 1つの HTML のコメントの中と1行の中のマーカーをすべて拾う | IR を書く段で推奨を採用（A30） |
| REQ-core-202: 形の誤ったマーカーは1つにつき invalid_marker を1件出し、detail はその行の文字にする。1件でも形が誤れば、同じマーカーのどの1件も照らさない | IR を書く段で推奨を採用（A31） |
| REQ-core-202、REQ-core-204: 指摘の path はガイドの基準のディレクトリからの相対パス、line はマーカーの始まりの行 | IR を書く段で推奨を採用（A32）。マーカーの行を指すこと（A12、利用者）は残した |
| REQ-core-203: SHA-256 は UTF-8 のバイト列から取る | IR を書く段で推奨を採用（A33） |
| REQ-core-204: 同じ ID が2か所にあるときの今の指紋（1つ目の項目の指紋） | IR を書く段で推奨を採用（A34）。この部分は消える |
| "guides" の "marks" と "files" の数え方 | IR を書く段で推奨を採用（A35）。出力の話題に属する |
| TBL-core-036 の読み替えで足した行き先の形の制限（パスが ".md" で終わる、空白と2つ目の "#" を含まない） | 記録に無い。決定の文書の形へ切り替える実装（コミット 15c0590 の前後）で、実装する側が決めた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-198 | 新しい形で読み替えて残る。「glob が IR や判断の記録の置き場のファイルに当たっても停止せず、ガイドとしても読む」を「決定の文書の置き場のファイルに当たっても」と読んだ。今のコードはガイドを glob だけで集め、決定の文書を設定の置き場から別に読み、重なりを見るのはテストと全体像の元データとだけなので、もうそう動いている（コードを読んで確かめた。テストは無い。下の「テストの無い行」） |
| REQ-core-201、REQ-core-202、TBL-core-036 | 新しい形で読み替えて残る。1件の形を "ID:指紋" から "<行き先>:<指紋>" に読み替えた。今のコードは両方の形を受ける（`crates/kotowari-core/src/guides.rs` の `parse_entries`） |
| REQ-core-203 | 一部が消える。指紋を取る行（項目の本文から "- source:" の行を除く、シナリオのステップの行、文書単位と要求の "- deferred:" の行）は、IR の項目、出典の行、後回しの宣言と一緒に消え、`docs/decisions/core/languages-and-guides.md` の「ガイドは、決めたことと確認することが変わったら古いとする」に置き換わった。アルゴリズムと行の終わりの文字の扱いは残る |
| REQ-core-204 | 新しい形で読み替えて残る。「ID の項目かシナリオが IR に無い」を「行き先の決定が無い」と読んだ。同じ ID が2か所にあるときの部分は、ID の重複が IR の形のものなので消える |
| EX-core-364、EX-core-375 | 消える。出典の行とシナリオのタグと名前は、新しい形に無い |
| EX-core-370 | 消える。同じ ID の項目が2か所にあることは、行き先では起きない |
| EX-core-363 | 新しい形で読み替えて残る。「問題の記録が無ければ complete は true」を「未決が無ければ」と読んだ（`docs/decisions/core/decision-documents.md` の「人に戻す問題は、話題の文書の「未決」の節に書く」） |

### テストの無い行

- C-core-guides-7: 今あるテストは IR の置き場に glob が当たる準備で書かれている。"guides.files" の glob を決定の文書の置き場に当てた準備で、check が停止せず、"decisions" の "documents" と "guides" の "files" がどちらも1になることを確かめるテストが要る（今の req_198_a_guides_glob_may_hit_the_ir_documents の準備を直せばよい）。

このほか、準備が古い形のまま付け替え先を書いたテスト（上の表の「準備を直す必要がある」のもの）は、切り替えで IR の置き場を読まなくなると、準備の作り方を直すまで、その行を確かめられない。

## 誰が決めたか見分けられなかったもの

無し。2026-09-24-doc-marks.md の A1〜A28、A36、A37、R1 は「利用者（推奨を採用）」か「利用者」、A29〜A35 は「IR を書く段で推奨を採用（承認で利用者が確かめる）」と書いてある。後者は、描画の話題の対応表が「LLM が決め、承認のときに示す」とある決定を落とした扱いに合わせて、agent の細部とした。2026-10-23-guide-overlap-sides.md の A1 は「整合のフェーズ」で、agent の細部とした。

## 理由や見送った案が記録に無かった決定

- 「ガイドは guides.files の glob で選び、check と status だけが読む」: 見送った案が記録に無い。節を置かなかった。
- 「ガイドとテストの置き場が重なると、設定の誤りで停止する」: 見送った案の節は置かなかった。2026-10-23-guide-overlap-sides.md の Rejected（A2、A3）は戻した対の側の数え方の代わりの案だが、整合のフェーズが退けたもので利用者の決定ではないので、見送った案には書かず、A3 の why（glob の重なりが対の側の無い誤りの陰に隠れる）を問題の節に使った。
- 「ガイドのマーカーは、HTML のコメントの中だけから読む」: 見送った案が記録に無い。
- 「ガイドのマーカーの1件は、決定への行き先と指紋をコロンでつなぐ」: 見送った案が記録に無い。行き先の形そのもの（"<文書のパス>#<見出しの行き先の名前>"）を選んだ理由は記録に無い。ガイドのマーカーが決定の見出しを指すことは `docs/decisions/core/languages-and-guides.md` の決定で、その文書には理由がある。
- 「ガイドを書く場面と見直す場面を、kotowari の skill に置く」: 見送った案が記録に無い。

見送った案のうち、この話題の決定の代わりでないものは書かなかった: R1（判断の記録の番号にもマーカーを付ける。判断の記録は新しい形で無くなる）。A3（P1 の render と trace に当たらない）は判断の記録の禁止の行についての決定で、新しい形に無いので書かなかった。

## 分けた所、まとめた所

- REQ-core-203 の残る部分（A2、A7、A14、A26）を、話題の土台の「古い節は、マーカーに書いた指紋を…」にまとめた。指紋を list と query に出すこと（A14）と書き写すこと（A2）は、指紋の取り方と同じ決定の中に置いた。
- REQ-core-201、REQ-core-202、TBL-core-036 を1つの決定にまとめた。どれもマーカーの1件の形と、その誤りを決めている。
- REQ-core-204 と A8、A11 を1つの決定にまとめた。A8 と A11 の見送った案を、その決定の見送った案に置いた。
- REQ-core-205 と A12、A13、A37 を1つの決定にまとめた。kotowari が検査しないこと（A12、A13）を、skill に置く理由として同じ決定に書いた。
- REQ-core-198 と A22（list と query がガイドを読まない）を1つの決定にした。

## あとでリンクにする所

確定の段で、次をすべて見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- `languages-and-guides.ja.md` の「ガイドは、決めたことと確認することが変わったら古いとする」（冒頭の段落）
- `decision-documents.ja.md` の「ほかの決定を指すときは、見出しへリンクを張る」（「ガイドのマーカーの1件は…」）
- 出力の下書き `output.ja.md` の「check の JSON の最上位には、指摘と種類ごとの数と、読んだものの数を出す」と、status の下書き `status.ja.md` の「集計は、テストのマーカー、ガイド、全体像、面、指摘の数を群ごとに出す」（冒頭の段落の "guides" の群）
- 対の下書き `translation-pairs.ja.md` の「対の側は、ファイル名と設定だけで見分ける」（戻した対の側の数え方）
- もとからリンクだった `test-discovery`、`cli-environment`、`test-markers` の見出し

残り: 無し。

## 実装する側が決める細部

- 切り替えたあとに、ガイドのマーカーに IR の ID の形の1件（"REQ-core-001:3fa9c2e1" など）が残っていたときの扱い。今のコードは受けて、IR に無ければ guide_stale にする。古い形を読む仕組みは作らない（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」）ので受け続けることはないが、invalid_marker の形の誤りにするか、行き先の無い guide_stale にするかは、記録にも決定にも無い。マーカーの形の誤りを invalid_marker にする決まり（A10、A23。":" の前が ID の形でなければ誤り）をそのまま読めば invalid_marker になる。kotowari 自身のガイドは、切り替えの版で決定の見出しへ付け替えるので、残らない。

## 利用者に聞くこと

無し。

## 「提案」で書いた決定の一覧

無し。書いた決定はどれも、今のコードがもう新しい形で動いているので「実装済み」にした。切り替えで要る作業は、古い形の1件を受ける部分（`crates/kotowari-core/src/guides.rs` の `parse_entries` の `is_valid_id`、`fingerprints_by_id` の IR の項目の部分）を消すことと、上の「準備を直す必要がある」テストの準備の書き直しと、kotowari の skill（`agent/skills/kotowari/references/guides.md`）から IR の項目とシナリオの説明を消すことである。

## 迷った判断と、どう決めたか

- 指紋を取る行の範囲は `docs/decisions/core/languages-and-guides.md` の決定にあるので、書き直さなかった。この話題には、アルゴリズム（A7）と行の終わりの文字の扱い（A26）だけを残した。A26 は IR の項目について決めたものだが、理由（checkout の改行の形で値が変わる）は決定の文書にも同じく当たり、今のコードも決定の行を行の終わりの文字なしで取る（`decisions.rs` が `str::lines` で行に分ける）ので、読み替えとして残した。
- A31 の「形の誤ったマーカーのどの1件も照らさない」は agent の細部で、下書きでは C-15 の文に残していたが、利用者の確認の記録が無く、安全に関わる落とした細部の一覧にも無いので、確定の段で C-15 の文から外した（書き直しの決まりの「承認のときに示す」「推奨をまとめて採用」の扱い）。それを確かめる ex_373_… は、いちばん近い C-15 を指す。
- 行き先の形（"<文書のパス>#<見出しの行き先の名前>"）は、記録に無い新しい細部だが、追加の決まりにしたがって今のコードの形をそのまま書いた。".md" で終わる、空白と2つ目の "#" を含まない、の制限は書かず、落とした細部にした。
- C-15 の「など」を「（例: パスか見出しの行き先の名前が無いもの）」に直した。step23 のテストが確かめる例である。
- REQ-core-199 の、対の側を重なりに数えること（2026-10-23 の A1）は、記録では整合のフェーズが実測を根拠に決めたもので、下書きでは落としたが、確定の段で「安全に関わる細部は、今のテストか skill の文が確かめているなら決定に戻す」という利用者の決定に従って戻した（C-23、C-24）。理由は A1 の why にあるので、問題の節に書いた。
- query で決定を引いたときの本文を決定の全体にする、という利用者の答え（query の話題の提案の決定）に合わせて、「古い節は…」の決めたことの「指紋を取る行が返る」と、C-1 の "body" の部分を外した。今のテスト query_of_a_decision_returns_the_fingerprint_a_guide_mark_takes は "body" が指紋を取る行だけであることも確かめているが、C-1 に残る部分（kind と "fingerprint"）は変わらない。
- EX-core-363 の C-20（終了コードと complete）は、今のテストが古い形の準備で書かれたものだけなので、そのテストを付け替え先にして準備の書き直しを書いた。step23 の新しい形のテストは check の JSON だけを見ていて、終了コードと status を見ていない。
- "guides" の群の数（files、marks）は元の要求の多くの場面に出てくるが、REQ-core-206 の話題の分なので、この話題の行にしなかった。

## 数

- 元: 要求 8、場面 14、決定表 1、性質 0
- 読んだ記録のファイル: 3（2026-09-24-doc-marks.md、2026-10-23-guide-overlap-sides.md、2026-09-25-deferred-items.md の A6、A20、A23）
- 出来上がり: 決定 7、確認することの行 24（うち「（読んで確認）」2）
- テスト: 39（この話題の ID を指すもの）。ほかに付け替えを勧めるもの 10
