# surface の対応表

元: `docs/ir/core/surface.md`（日本語は `surface.ja.md`）。書き直した文書: `surface.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-223 | C-core-surface-1, C-core-surface-2, C-core-surface-3 | 面は、設定の surface.files と surface.rules で、ast-grep の規則に当たった節としてコードから取り出す | そのまま。A1、A8、A29（利用者）。"language" の大文字小文字と ast-grep の別名、外す引用符の種類（一重・二重・バッククォート）、"$NAME" を捕まえない当たりを面にしないことは A19、A20（まとめて採用）なので落とした。同梱しないこと（A5）は決めたことの文に書いた |
| REQ-core-224 | C-core-surface-5, C-core-surface-8, C-core-surface-26 | 面の規則と面のファイルは、tests.rules と tests.files と同じ作りで読む | そのまま。A1（利用者）の「tests.rules と同じ作り」を残した。規則の "files" と "ignores" を当てること、規則のほかの欄（"fix"、"message"、"severity"、"note"、"metadata"）を使わないことは A20（まとめて採用）なので落とした。"severity: off" も当てることは、安全に関わる細部として決定に戻した（C-26）。規則の言語のファイルが読めないときの停止は A29 の why が前提にしているので、「規則の言語の面のファイルだけを…」の決定に C-8 として置いた |
| REQ-core-236 | C-core-surface-6, C-core-surface-7 | 規則の言語の面のファイルだけを読んで構文木にし、ほかの面のファイルは読まない | そのまま。A29（利用者）。同じパスにテストのファイルとして出した unparsable_file を重ねて出さないことは A20（まとめて採用）なので落とした |
| REQ-core-225 | C-core-surface-9, C-core-surface-10 | 面の設定の鍵が揃っていないときは、設定を読むどのコマンドも停止する | そのまま。A12、A24（利用者）。規則のファイルの誤りの条件は "tests.rules" の話題（query-rules.md）を指した |
| REQ-core-226 | C-core-surface-11, C-core-surface-12, C-core-surface-13, C-core-surface-14 | 面の名前が、決定の「決めたこと」か「確認すること」に引用されていれば、その面は仕様にある | 読み替え。A2 の「要求の文、決定表のセル、シナリオのステップ」を、決定の「決めたこと」（表はセルごと）と「確認すること」に、A13 の「後回しの要求の文も数える」を「提案の決定も数える」に読み替えた。今のコードはもうそう動いている（9ef3516、c8993a9）。二重引用符を左から対にすること、バッククォートを二重引用符の外で対にすることは A18（まとめて採用）なので落とした。性質の文、範囲の行、項目の "- " の行、問題の記録、用語集は消える |
| REQ-core-227 | C-core-surface-15, C-core-surface-16, C-core-surface-17 | 仕様に無い面は、コードで最初に見つかった場所に surface_without_spec の誤りを1件出す | そのまま（「IR に無い」を「仕様に無い」と読む）。A3、A7（利用者）。最初の1か所をパスのバイト順、次に行で決めることは A19（まとめて採用）なので落とした。指摘の種類の名前は A15 だが、利用者から見える名前として残した |
| REQ-core-228 | C-core-surface-18, C-core-surface-19, C-core-surface-20 | 未記載の面の一覧で外した面の数を、check の出力の最後に必ず出す | そのまま。出すことは A4、規則が無ければ出さないことは A3（利用者）。JSON の鍵と文字の行の形は A16（まとめて採用）だが、利用者から見える名前として残した。数える単位（種類と名前の組）は落とした |
| REQ-core-229 | C-core-surface-21 | 面を読むのは check と status だけにする | そのまま。A14、A10、A24（利用者）。list と query の行は list.md と query.md に、status の数は status.md にあるので、言葉とリンクで指した |
| REQ-core-230 | C-core-surface-22, C-core-surface-23, C-core-surface-24 | 面の検査の書き方と直し方は skill の reference に置き、未記載の面の一覧に足すのは壁打ちと導入の skill だけにする | 読み替え（「IR に無い面」を「仕様に無い面」と読む）。A11、A26（利用者）。例のライブラリの名前は書かなかった。C-core-surface-25 は同じ決定に、A11 と A28 の導入の skill の分を足した行 |
| EX-core-407 | C-core-surface-11, C-core-surface-18 | 面の名前が、決定の「決めたこと」か「確認すること」に引用されていれば、その面は仕様にある | 読み替え（要求の文を決定の「決めたこと」と読む） |
| EX-core-408 | C-core-surface-15 | 仕様に無い面は、コードで最初に見つかった場所に surface_without_spec の誤りを1件出す | 読み替え（「IR のどこにも」を「仕様のどこにも」と読む） |
| EX-core-409 | C-core-surface-16 | 仕様に無い面は、コードで最初に見つかった場所に surface_without_spec の誤りを1件出す | そのまま |
| EX-core-410 | C-core-surface-12 | 面の名前が、決定の「決めたこと」か「確認すること」に引用されていれば、その面は仕様にある | 読み替え（要求の文を決定と読む） |
| EX-core-411 | C-core-surface-11 | 面の名前が、決定の「決めたこと」か「確認すること」に引用されていれば、その面は仕様にある | 読み替え。決定表のセルを「決めたこと」の表のセルに、後回しの要求を提案の決定に読む。シナリオのステップは消え、「確認すること」の行が近い |
| EX-core-412 | C-core-surface-11 | 面の名前が、決定の「決めたこと」か「確認すること」に引用されていれば、その面は仕様にある | 一部が消える。用語集の用語と unknown_term は消える（`decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」）。バッククォートで囲んだ名前を数えることは C-11 に残る |
| EX-core-413 | C-core-surface-17, C-core-surface-20 | 仕様に無い面は、コードで最初に見つかった場所に surface_without_spec の誤りを1件出す | そのまま |
| EX-core-414 | C-core-surface-18, C-core-surface-19 | 未記載の面の一覧で外した面の数を、check の出力の最後に必ず出す | そのまま |
| EX-core-415 | C-core-surface-9 | 面の設定の鍵が揃っていないときは、設定を読むどのコマンドも停止する | そのまま。標準エラーの "config error: " は停止の文言の話題（cli-environment.md）の分 |
| EX-core-416 | C-core-surface-9 | 面の設定の鍵が揃っていないときは、設定を読むどのコマンドも停止する | そのまま |
| EX-core-417 | C-core-surface-10 | 面の設定の鍵が揃っていないときは、設定を読むどのコマンドも停止する | そのまま。list が停止しない分は list.md の C-core-list-16 |
| EX-core-418 | C-core-surface-3 | 面は、設定の surface.files と surface.rules で、ast-grep の規則に当たった節としてコードから取り出す | そのまま |
| EX-core-426 | C-core-surface-7 | 規則の言語の面のファイルだけを読んで構文木にし、ほかの面のファイルは読まない | そのまま |
| EX-core-427 | C-core-surface-6 | 規則の言語の面のファイルだけを読んで構文木にし、ほかの面のファイルは読まない | そのまま |
| EX-core-428 | C-core-surface-9 | 面の設定の鍵が揃っていないときは、設定を読むどのコマンドも停止する | そのまま |
| EX-core-429 | - | 面の名前が、決定の「決めたこと」か「確認すること」に引用されていれば、その面は仕様にある | 消える。性質の文と "- how_to_verify:" の行は決定の文書に無い（`decision-documents.md` の「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」） |

元の文書に TBL と PROP は無い（TBL-core-031 は拡張子と言語の表で、テストの見つけ方の話題の ID）。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-source-analysis/tests/surface.rs | req_223_a_matched_node_is_a_surface_named_by_name_with_one_quote_pair_removed | REQ-core-223 | C-core-surface-1, C-core-surface-2 | 当たった節が面になり、種類と名前が規則の "id" と "$NAME" から決まり、引用符が1組外れる |
| crates/kotowari-source-analysis/tests/surface.rs | req_223_only_one_pair_of_quotes_is_removed_and_other_forms_stay | REQ-core-223 | C-core-surface-2 | そのまま |
| crates/kotowari-source-analysis/tests/surface.rs | req_223_a_match_without_name_is_not_a_surface | REQ-core-223 | C-core-surface-1 | "$NAME" を捕まえない当たりを面にしないことは落とした細部（A19）だが、名前が "$NAME" から来ることの裏側として C-1 に当たる |
| crates/kotowari-source-analysis/tests/surface.rs | req_223_language_is_matched_without_case_and_accepts_aliases | REQ-core-223 | C-core-surface-3 | 大文字小文字と別名は落とした細部。規則の言語のファイルにだけ当てることを確かめている |
| crates/kotowari-source-analysis/tests/surface.rs | ex_core_418_a_rule_is_not_applied_to_a_file_of_another_language | EX-core-418, REQ-core-223 | C-core-surface-3 | そのまま |
| crates/kotowari-source-analysis/tests/surface.rs | req_224_rule_files_and_ignores_are_applied_and_severity_off_still_applies | REQ-core-224 | C-core-surface-5, C-core-surface-26 | "severity: off" と書いた規則が当たって面を取り出す（C-26）。"files" と "ignores" は落とした細部（A20）で、"tests.rules" と同じ規則として当たることの一部として C-5 に当てた |
| crates/kotowari-source-analysis/tests/surface.rs | ex_core_426_a_file_not_in_a_rule_language_is_not_parsed | EX-core-426, REQ-core-236 | C-core-surface-7 | そのまま |
| crates/kotowari-source-analysis/tests/surface.rs | ex_core_427_a_syntax_error_in_a_rule_language_file_is_unparsable | EX-core-427, REQ-core-236 | C-core-surface-6 | そのまま |
| crates/kotowari-source-analysis/tests/analyzer.rs | public_surface_inspection_reports_uncovered_analyzer_facts | REQ-core-315, REQ-core-228, EX-core-497 | C-core-surface-15 | この話題の分だけ。仕様に無い面に surface_without_spec が出ることを、公開の関数で確かめる。関数に IR の文書を渡しているので、付け替えのときに決定の文書を渡すように直す。REQ-core-315、EX-core-497 はライブラリの話題の ID |
| crates/kotowari-core/tests/surface.rs | tbl_004_surface_keys_are_read_and_their_paths_normalized | TBL-core-004, REQ-core-224 | - | この話題の行には当たらない。鍵の読み込みとパスの正規化は設定の話題（TBL-core-004）の分 |
| tests/native_paths.rs | byte_named_files_without_legacy_read_aliases_keep_the_unreadable_stop | REQ-core-313, REQ-core-018, REQ-core-198, REQ-core-224, EX-core-485 | C-core-surface-8 | この話題の分だけ。規則の言語の面のファイルが読めないと停止する。ほかの ID はそれぞれの話題 |
| tests/native_paths.rs | distinct_byte_named_entries_keep_the_legacy_alias_text_and_multiplicity | REQ-core-313, REQ-core-198, REQ-core-206, REQ-core-224, EX-core-485 | - | 名前のバイトが違う2つのファイルを別々に読むことを確かめている。この話題の行には当たらない（ライブラリの話題） |
| tests/step17_surface.rs | ex_core_415_rules_without_files_stop_with_a_config_error | EX-core-415, REQ-core-225 | C-core-surface-9 | そのまま |
| tests/step17_surface.rs | ex_core_416_files_without_rules_stop | EX-core-416, REQ-core-225 | C-core-surface-9 | そのまま |
| tests/step17_surface.rs | ex_core_428_a_list_key_without_rules_stops_even_list | EX-core-428, REQ-core-225 | C-core-surface-9 | そのまま |
| tests/step17_surface.rs | req_225_a_key_combination_error_stops_every_command_that_reads_the_config | REQ-core-225 | C-core-surface-9 | list、query、status、mutants で確かめる。query に渡す "REQ-core-001" は古い形の ID なので、付け替えのときに確認することの ID に直す（停止は引数より先なので結果は変わらない見込み） |
| tests/step17_surface.rs | req_224_surface_rules_are_not_added_to_the_test_queries | REQ-core-224 | C-core-surface-5 | そのまま |
| tests/step17_surface.rs | req_224_a_non_utf8_surface_file_in_a_rule_language_stops_as_a_test_file_does | REQ-core-224 | C-core-surface-8 | そのまま |
| tests/step17_surface.rs | req_236_a_non_utf8_surface_file_not_in_a_rule_language_is_not_read | REQ-core-236, REQ-core-224 | C-core-surface-7 | そのまま |
| tests/step17_surface.rs | ex_core_417_a_missing_rule_file_stops_check_but_not_list | EX-core-417, REQ-core-225, REQ-core-229 | C-core-surface-10 | この話題の分は check が停止すること。list が停止しないことは list.md の C-core-list-16 の分 |
| tests/step17_surface.rs | req_225_a_broken_rule_file_stops_check_and_status_naming_the_rule_file | REQ-core-225, TBL-core-020 | C-core-surface-10 | この話題の分だけ。詳細にルールのファイルのパスが出ることは cli-environment.md の C-core-clienv-9 |
| tests/step17_surface.rs | req_225_the_same_rule_path_twice_stops | REQ-core-225 | C-core-surface-10 | そのまま |
| tests/step17_surface.rs | req_236_a_file_already_unparsable_as_a_test_file_is_not_reported_twice | REQ-core-236 | C-core-surface-6 | 重ねて出さないことは落とした細部（A20）。構文の誤りが unparsable_file になることの一部として C-6 に当てた |
| tests/step17_surface.rs | ex_core_408_a_surface_absent_from_the_ir_is_an_error_at_its_first_place | EX-core-408, EX-core-409, REQ-core-227, TBL-core-019, TBL-core-006, TBL-core-008 | C-core-surface-15, C-core-surface-16 | この話題の分だけ。準備の "--format" を IR の要求の文に書いているので、付け替えのときに決定の文書に書くように直す（直さないと "--format" も誤りになる） |
| tests/step17_surface.rs | req_227_the_first_place_is_by_path_bytes_then_line | REQ-core-227 | C-core-surface-16 | パスのバイト順は落とした細部（A19）。最初の1か所に1件だけ出ることを確かめる。準備の IR の要求の文を決定の文書に直す |
| tests/step17_surface.rs | ex_core_410_a_name_that_is_only_part_of_the_quoted_content_is_not_in_the_ir | EX-core-410, REQ-core-226 | C-core-surface-12 | 準備の IR の要求の文を決定の「決めたこと」に直す |
| tests/step17_surface.rs | req_226_double_quotes_pair_from_the_left | REQ-core-226 | C-core-surface-11 | 落とした細部のテスト。いちばん近い行を指す。二重引用符を左から対にすることは落とした細部（A18）で、二重引用符で囲んだ名前を数える C-11 に当てた。準備の IR の要求の文を決定の「決めたこと」に直す |
| tests/step17_surface.rs | ex_core_411_table_cells_deferred_statements_and_steps_count | EX-core-411, REQ-core-226 | - | 決定表、後回しの要求、シナリオのステップは消える。古い形のコードと一緒に消す。決定の表のセルと提案の決定は、C-11 のほかのテストが確かめる |
| tests/step17_surface.rs | ex_core_412_a_glossary_term_in_backticks_counts | EX-core-412, REQ-core-226 | - | 用語集と unknown_term が消える。古い形のコードと一緒に消す。バッククォートの引用は req_226_names_quoted_in_what_a_decision_decided_and_its_rows_count が確かめる |
| tests/step17_surface.rs | ex_core_429_property_statements_and_how_to_verify_lines_do_not_count | EX-core-429, REQ-core-226 | - | 性質と how_to_verify は消える。古い形のコードと一緒に消す |
| tests/step17_surface.rs | req_226_scope_lines_flags_and_the_glossary_do_not_count | REQ-core-226 | - | 範囲の行、問題の記録、用語集の形は消える。古い形のコードと一緒に消す |
| tests/step17_surface.rs | req_227_no_surface_is_extracted_when_surface_rules_is_empty | REQ-core-227 | C-core-surface-17 | そのまま（準備の IR の文書は面に関わらない） |
| tests/step17_surface.rs | req_229_list_and_query_do_not_read_the_surface_list_or_files | REQ-core-229, REQ-core-152, REQ-core-158 | - | この話題の分は list.md の C-core-list-16 と query.md の C-core-query-10 にある |
| tests/step17_surface.rs | req_229_mutants_does_not_read_the_surface_list_rules_or_files | REQ-core-229 | C-core-surface-21 | 準備の IR の要求の文を決定の文書に直す（"mutants" の結果は面に関わらないので、直さなくても通る見込み） |
| tests/step17_surface.rs | ex_core_407_a_surface_quoted_in_a_requirement_is_not_an_error | EX-core-407, REQ-core-223, REQ-core-226, REQ-core-228, TBL-core-005 | C-core-surface-1, C-core-surface-11, C-core-surface-18 | この話題の分だけ。準備の IR の要求の文を決定の「決めたこと」に直す。TBL-core-005 は出力の話題の ID |
| tests/step17_surface.rs | ex_core_414_the_count_line_is_printed_even_with_no_findings | EX-core-414, REQ-core-228 | C-core-surface-18, C-core-surface-19 | 準備の IR の要求の文を決定の文書に直す |
| tests/step17_surface.rs | ex_core_413_a_project_without_surface_rules_sees_nothing | EX-core-413, REQ-core-227, REQ-core-228 | C-core-surface-17, C-core-surface-20 | そのまま |
| tests/step17_surface.rs | ex_core_419_a_listed_surface_is_not_an_error_and_is_counted | EX-core-419, REQ-core-232, REQ-core-228 | C-core-surface-18 | この話題の分は外した数が1になること。REQ-core-232 は未記載の面の一覧の話題。準備の IR の要求の文を決定の文書に直す |
| tests/step17_surface.rs | ex_core_423_a_listed_surface_now_in_the_ir_is_not_counted | EX-core-423, REQ-core-228 | C-core-surface-18 | 仕様に書かれた面は外した数に数えない。準備の IR の要求の文を決定の文書に直す |
| tests/step17_surface.rs | req_228_the_count_line_comes_after_the_findings | REQ-core-228, TBL-core-005 | C-core-surface-19 | この話題の分だけ。準備の IR と用語集を決定の文書に直す |
| tests/step17_surface.rs | req_229_status_counts_surfaces_and_a_surface_error_makes_it_incomplete | REQ-core-229, REQ-core-162, TBL-core-028 | - | status の数と complete は status.md の C-core-status-2、C-core-status-8 などの分。この話題の行は無い |
| tests/step17_surface.rs | req_227_the_first_place_is_the_smallest_line_even_when_rules_share_an_id | REQ-core-227 | C-core-surface-16 | そのまま（準備の IR の文書は面に関わらない） |
| tests/step17_surface.rs | ex_core_427_check_reports_the_unparsable_surface_file_beside_a_test_file_one | EX-core-427, REQ-core-236 | C-core-surface-6 | そのまま |
| tests/step17_surface.rs | req_226_a_backticked_term_in_a_decision_table_cell_counts | REQ-core-226 | - | IR の決定表と用語集が消える。古い形のコードと一緒に消す |
| tests/step17_surface.rs | req_226_names_quoted_in_what_a_decision_decided_and_its_rows_count | REQ-core-226 | C-core-surface-11 | 提案の決定の「決めたこと」の二重引用符と、「確認すること」のバッククォートを確かめる |
| tests/step17_surface.rs | req_226_names_quoted_only_in_the_problem_rejected_options_or_open_items_do_not_count | REQ-core-226 | C-core-surface-13 | そのまま |
| tests/step17_surface.rs | req_226_a_decision_quotes_a_name_only_as_the_whole_content_of_a_cell_outside_code | REQ-core-226 | C-core-surface-12, C-core-surface-14 | セルをまたぐ引用符と、セルの中身の一部、コードブロックの中を確かめる |
| tests/step17_surface.rs | req_226_a_name_quoted_in_a_decision_is_counted_as_specified_by_status | REQ-core-226, TBL-core-028 | C-core-surface-11 | この話題の分は、決定に引用した面を仕様にあるとすること。status の specified は C-core-status-8、一覧の1件の注意は C-core-unspec-4 の分 |
| tests/step17_surface.rs | req_226_a_name_in_a_decision_cell_counts_even_after_a_stray_quote_in_another_cell | REQ-core-226 | C-core-surface-11 | 表をセルごとに見ること |
| tests/step17_surface.rs | req_226_a_code_block_inside_a_row_or_a_quote_of_a_decision_is_not_read | REQ-core-226 | C-core-surface-11, C-core-surface-14 | 引用（>）の中の名前を数え、一覧の項目と引用の中のコードブロックの名前を数えない |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-224: "severity" が "off" の面の規則も当てる（規則を止める口を作らず、検査する面が黙って減らない） | C-core-surface-26（「面の規則と面のファイルは、tests.rules と tests.files と同じ作りで読む」の決めたことに文を足した） | `crates/kotowari-source-analysis/tests/surface.rs` の req_224_rule_files_and_ignores_are_applied_and_severity_off_still_applies（"severity: off" の規則で "--format" の面が取り出される） |

## 落とした細部

### 安全に関わるもの

無い（"severity: off" の細部は下の「戻した細部」で決定に戻した）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-223: "language" を大文字小文字を区別せずに突き合わせ、ast-grep の別名（"ts"、"py"）も受ける | 利用者（A20、まとめて採用。REQ-core-189 に揃える） |
| REQ-core-223: 外す引用符の種類（一重引用符、二重引用符、バッククォート） | 利用者（A19、まとめて採用）。1組だけ外すことは A8 なので残した |
| REQ-core-223: "$NAME" を捕まえない当たりは面にしない | 利用者（A19、まとめて採用） |
| REQ-core-224: 規則の "files" と "ignores" を面のファイルの相対パスに当て、規則のほかの欄（"fix"、"message"、"severity"、"note"、"metadata"）を使わない | 利用者（A20、まとめて採用） |
| REQ-core-224: "surface.rules" のパスを基準のディレクトリからの相対パスとして読む | 記録に無い（基準のディレクトリの話題の一般の決まり） |
| REQ-core-236: 同じパスにテストのファイルとして unparsable_file を出したときは重ねて出さない | 利用者（A20、まとめて採用） |
| REQ-core-226: 二重引用符を行の左から対にし、バッククォートは二重引用符の外のものを対にする。決定表の見出しの行のセルも数える | 利用者（A18、まとめて採用） |
| REQ-core-226（決定の文書への読み替え）: "####" より深い見出しの下も「決めたこと」と「確認すること」の中に数える。一覧の項目と引用の中へも降りて読む | 実装する側（9ef3516、c8993a9 のコミットの本文に「自分で決めた細部」とある） |
| REQ-core-227: 最初の1か所は、面のファイルのパスのバイト順、次に行の小さい順で決める。"line" は節の最初の行 | 利用者（A19、まとめて採用） |
| REQ-core-227: detail は種類と名前を1つの半角空白で区切る | 利用者（A7 は「種類 名前」まで。区切りの文字は A15〜A19 のまとめての採用） |
| REQ-core-228: 数える単位は種類と名前の組 | 利用者（A16、まとめて採用） |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-226 | 新しい形で読み替えて残る。「要求の文、決定表のセル、シナリオのステップ」を「決定の「決めたこと」（表はセルごと）と「確認すること」」に、「後回しの要求の文」を「提案の決定」に読む。性質の文、範囲の行、項目の "- " の行（how_to_verify を含む）、問題の記録の本文、用語集の除外は、その形が無くなるので消える。新しい形で数えないのは「問題」、「見送った案」、「未決」 |
| REQ-core-227、REQ-core-228、REQ-core-230 | 新しい形で読み替えて残る。「IR にない」「IR に無い面」を「仕様に無い」「仕様に無い面」と読む |
| EX-core-407、EX-core-408、EX-core-410、EX-core-411、EX-core-419、EX-core-423 | 新しい形で読み替えて残る。準備の「要求の文に書いた」を「決定の「決めたこと」に書いた」と読む。テストの準備も IR の文書から決定の文書に直す |
| EX-core-412 | 一部が消える。用語集の用語であることと unknown_term は、用語の検査をやめるので消える（`decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」） |
| EX-core-429 | 消える。性質と how_to_verify の行は決定の文書に無い |
| A25（用語集で面を別の呼び方にしない） | 消える。用語集の中身を check が検査しない（同上） |
| A28（FLAG になった面を一覧に残す） | 新しい形で読み替えて残る。FLAG を「未決に残った面」と読む（`decision-documents.md` の「人に戻す問題は、話題の文書の「未決」の節に書く」） |

テストの無い行は無い。ただし C-core-surface-15、C-core-surface-18 を指すテストは、どれも付け替えのときに準備を IR の文書から決定の文書に直す必要がある（上の表）。

## 誰が決めたか見分けられなかったもの

- 2026-09-27-surface-check.md の A15〜A22 は decided_by が「利用者（IR を書く段で決めた細部をまとめて採用）」である。未記載の面の一覧の対応表（surface-unspecified.map.md）と同じく、agent の細部として落とし、指摘の種類の名前（A15）と JSON の鍵と文字の行の形（A16）は利用者から見える名前として残した。
- 決定の文書の「決めたこと」と「確認すること」だけを数えることは、9ef3516 のコミットの本文では実装する側が決めた細部とされている。ただし A2 が挙げた3つの場所（要求の文、決定表のセル、シナリオのステップ）を新しい形に移すと、そのまま「決めたこと」（決定表もここに書く）と「確認すること」に当たるので、A2 の読み替えとして決定に書いた。コードブロックの中を数えないことは、文書の読み方の話題の「決定の文書のコードブロックの中は、決定の形としても引用としても読まない」（records.md の A52 の読み替え）に支えさせた。

## 理由や見送った案が記録に無かった決定

- 「面は、設定の surface.files と surface.rules で…」: 見送った案が記録に無い。
- 「面の規則と面のファイルは、tests.rules と tests.files と同じ作りで読む」: 見送った案が記録に無い。理由は A1 と A20 の why（書き方を1つ覚えれば済む）だけで、テストを見つける問い合わせに加えない理由は無い。問題の節に「理由は残っていない」と書いた。
- 「規則の言語の面のファイルだけを読んで…」: 見送った案が記録に無い。
- 「面の設定の鍵が揃っていないときは…」: 見送った案が記録に無い。
- 「仕様に無い面は、コードで最初に見つかった場所に…」: 見送った案が記録に無い。
- 「未記載の面の一覧で外した面の数を…」: 見送った案が記録に無い（A16 の why の「ガイドの集計はいつも出すが、それには倣わない」は、まとめての採用の細部なので書かなかった）。
- 「面を読むのは check と status だけにする」: 見送った案が記録に無い。
- 「面の検査の書き方と直し方は skill の reference に置き…」: 見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-core-224 と REQ-core-236 を、規則とファイルの読み方の作り（D2）と、規則の言語のファイルだけを読むこと（D3）の2つに分けた。規則の言語のファイルが読めないときの停止（REQ-core-224 の後半）は D3 に入れた。
- REQ-core-223 に、A5 と A27 の kotowari 自身の設定（C-core-surface-4）を足した。元の要求には無いが、A5 は REQ-core-223 の出典で、自分の CLI で動かすことを決めている。A9 は A27 で改められているので、A27 の中身で書いた。
- REQ-core-229 の list と query の分は list.md と query.md に、status の分は status.md に既にあるので、この文書には mutants の行（C-core-surface-21）だけを置き、残りはリンクで指した。plan は行を置かず、決めたことの文にだけ書いた（テストが無い）。
- REQ-core-230 に、A11 と A28 の kotowari-adopt の分（C-core-surface-25）を足した。

## あとでリンクにする所

- `docs/decisions/core/decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」と「人に戻す問題は、話題の文書の「未決」の節に書く」: この対応表の形に依存する要求の節で言葉で指した（文書の本文からは指していない）。
- ほかの文書からこの文書へ: surface-unspecified と output が「面の検査」の話題として言葉で指していた所は、この確定でそれぞれの見出しへのリンクに直した（どちらも同じ依頼の話題）。

## 迷った判断と、どう決めたか

- C-core-surface-12 の「（"--verbose true" の中の "--verbose" のように）」を、例だと分かる「（例: …）」に直した。意味は変えていない。

- 安全に関わる細部（"severity: off" の規則も当てる）は、A20（まとめて採用）の細部だが、今のテストが確かめているので、BRIEF-C の利用者の決定どおり決定に戻した。問題の節には、落とすと検査する面が黙って減ることと、理由が記録に残っていないことを書いた。
- `req_226_double_quotes_pair_from_the_left` は、この話題の ID だけを持つので、「-」のままだとマーカーの無いテストになる。いちばん近い C-11 を指した。

- A15〜A22 の扱いは、兄弟の対応表（surface-unspecified.map.md）に揃えて落とした。そのため、引用符の対の取り方、最初の1か所の決め方、数の単位は、テストの中にだけ残る。
- 決定の文書に引用した面を数える範囲（「決めたこと」と「確認すること」、提案の決定も数える）は、依頼で「すでに仕様にあると数える」と示され、今のコードもそう動いているので、実装済みの読み替えにした。
- IR を準備にしている CLI のテストは、付け替えのときに準備を決定の文書に直すものとして、行に当てた。IR にしか無い形（決定表、後回し、シナリオ、性質、how_to_verify、問題の記録、用語集）を確かめるテストは「-」にした。
- C-core-surface-9 の「どのコマンドでも」は、テストが確かめている check、status、list、query、mutants を行に書いた。plan は設定を読むが、テストが無いので行に挙げなかった。
- 面の規則の例は、元の文書ではライブラリの名前で指していたが、外部の名前を書かない決まりに合わせ、「CLI のフラグとサブコマンドを取り出す規則の例」と書いた。

## 実装する側が決める細部

無い。読み替えに要る名前（surface_without_spec、"surface"、"unspecified"）は今のコードのまま使う。

## 利用者に聞くこと

無い。

## 「提案」で書いた決定の一覧

無い。どの決定も、今のコードがもうそう動いている。

## 数

- 元: 要求 9、場面 16、決定表 0、性質 0
- 読んだ記録のファイル: 1（2026-09-27-surface-check.md）。ほかに、決定の文書への読み替えのコミット 9ef3516 と c8993a9 の本文
- 出来上がり: 決定 9（実装済み 9、提案 0）、確認することの行 26（うち「（読んで確認）」5）
- テスト: 50（付け替え先のあるもの 41、「-」9）
- 戻した安全の細部: 1（C-core-surface-26）
- 利用者に聞くこと: 0
