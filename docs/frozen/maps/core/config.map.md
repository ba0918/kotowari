# config の対応表

元: `docs/ir/core/config.md`（日本語は `config.ja.md`）。書き直した文書: `config.md`。cli の REQ-core-003 もこの文書の決定に入れた（cli の対応表）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-011 | C-core-config-1 | 設定ファイルは基準のディレクトリの .kotowari/config.yaml に置き、--config で別のファイルを指せる | そのまま |
| REQ-core-012 | C-core-config-4, C-core-config-5 | 設定ファイルが無いか空なら、既定の値で動く | そのまま。無いときを C-4、空のときを C-5 にした |
| REQ-core-013 | C-core-config-6, C-core-config-7 | 設定のキーは表のものだけにし、書かなかったキーは表の既定の値にする | TBL-core-004 を定義に持つだけの要求。下の TBL-core-004 と同じ |
| REQ-core-014 | C-core-config-12, C-core-config-13, C-core-config-14, C-core-config-15, C-core-config-16, C-core-config-17, C-core-config-18, C-core-config-25, C-core-config-26 | 設定ファイルが YAML として読めないか、キーと値が決まりに合わなければ、設定の誤りで停止する | そのまま（一部は消える）。条件を表にし、確認することを条件の種類ごとに分けた。null の例の "ir:" は、キーが無くなるので "tests:" にした。負の数と0は "limits.lines" と "limits.requirements" にしか当たらず、キーと一緒に消える（「行数と要求の数の上限のキー…」）。"vague_words" の空の文字列の要素（C-25）と、絶対パスを先頭の "/" だけで見分けること（C-26）は、安全に関わる細部として戻した |
| REQ-core-015 | C-core-config-19 | 一覧のキーは一覧だけを受け、書いた一覧で既定を置き換え、空の一覧は要素の無い一覧にする | そのまま |
| REQ-core-016 | C-core-config-20 | 一覧のキーは一覧だけを受け、書いた一覧で既定を置き換え、空の一覧は要素の無い一覧にする | そのまま |
| REQ-core-017 | C-core-config-21 | 設定のキーは入れ子の形で読む | そのまま。例の "decisions:" の下の "records:" は、キーが無くなるので "tests:" の下の "rust:" の下の "attributes:" にした |
| REQ-core-018 | C-core-config-10, C-core-config-11 | 決定の文書の置き場が無いか、ディレクトリでないか、読めなければ、読めないファイルで停止する | 読み替え。"ir" の置き場を決定の文書の置き場に読み替えた。今のコードは "decisions.documents" の置き場が無いかディレクトリでなければ unreadable file で停止するので実装済み。"decisions.records" と "decisions.adr" の置き場は消える（下の「形に依存する要求」）。走査の停止は C-11 |
| REQ-core-019 | C-core-config-22, C-core-config-23, C-core-config-24 | glob の "**" は再帰で読み、隠しディレクトリにはパスの成分で名指ししたときだけ入る | そのまま |
| REQ-core-020 | C-core-config-3 | 設定ファイルは基準のディレクトリの .kotowari/config.yaml に置き、--config で別のファイルを指せる | そのまま。設定ファイルの場所の決定に入れた |
| TBL-core-004 | C-core-config-6, C-core-config-7, C-core-config-8, C-core-config-9, C-core-config-27 | 設定のキーは表のものだけにし、書かなかったキーは表の既定の値にする | 一部が読み替え、一部が消える。残るキーは実装済みの決定の表に置いた。"ir"、"decisions.records"、"decisions.adr" の行は消え、決定の文書の置き場の "decisions.documents" を提案の決定（C-8、C-9）に書いた。"limits.lines" と "limits.requirements" の行は消え、無くすことを提案の決定（C-27）に書いた（利用者の答え） |
| EX-core-003 | C-core-config-12 | 設定ファイルが YAML として読めないか、キーと値が決まりに合わなければ、設定の誤りで停止する | そのまま |
| EX-core-383 | C-core-config-13 | 設定ファイルが YAML として読めないか、キーと値が決まりに合わなければ、設定の誤りで停止する | そのまま。準備の "ir: docs/ir" の2回は、切り替えのときにほかのキーの2回に直す |
| EX-core-544 | C-core-config-15 | 設定ファイルが YAML として読めないか、キーと値が決まりに合わなければ、設定の誤りで停止する | そのまま |
| EX-core-545 | C-core-config-24 | glob の "**" は再帰で読み、隠しディレクトリにはパスの成分で名指ししたときだけ入る | そのまま |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step1_config.rs | req_003_config_path_is_relative_to_cwd | REQ-core-003 | C-core-config-2 | cli の ID。この話題の決定に入れたので、ここに書いた |
| tests/step1_config.rs | req_003_list_query_and_status_read_the_config_given_by_config | REQ-core-003, REQ-core-152, REQ-core-158, REQ-core-163 | C-core-config-2 | この話題の分だけ。準備は "ir" のキーと IR の要求なので、切り替えのときに "decisions.documents" と決定の文書に直す |
| tests/step19_overview_build.rs | req_core_002_build_takes_format_and_config_before_or_after_the_command | REQ-core-002, REQ-core-003 | C-core-config-2 | この話題の分（overview build が "--config" の設定を使う）だけ。REQ-core-002 の分は cli の対応表 |
| tests/step1_config.rs | req_011_reads_dot_kotowari_config_by_default | REQ-core-011 | C-core-config-1 | そのまま |
| tests/step1_config.rs | req_020_kotowari_toml_beside_the_base_is_not_read | REQ-core-020 | C-core-config-3 | そのまま |
| tests/step1_config.rs | req_012_missing_config_uses_defaults | REQ-core-012 | C-core-config-4 | 準備に IR と判断の記録と ADR の置き場を作っている。切り替えのときに決定の文書の置き場に直す |
| tests/step1_config.rs | req_012_empty_config_uses_defaults | REQ-core-012 | C-core-config-5 | "ir" の既定を見ているので、切り替えのときにほかのキーの既定を見るように直す |
| tests/step1_config.rs | req_012_comment_only_config_uses_defaults | REQ-core-012 | C-core-config-5 | 同上 |
| tests/step1_config.rs | req_013_defaults_match_the_table | REQ-core-013, TBL-core-004 | C-core-config-6 | "ir"、"decisions.records"、"decisions.adr"、"limits" の既定も見ている。実装のときにその分を外し、"decisions.documents" の既定（C-8）を足す |
| crates/kotowari-core/tests/guides.rs | tbl_004_guides_files_defaults_to_an_empty_list | TBL-core-004 | C-core-config-6 | そのまま |
| crates/kotowari-core/tests/guides.rs | tbl_004_guides_files_reads_a_list_of_globs | TBL-core-004 | C-core-config-7 | そのまま |
| crates/kotowari-core/tests/surface.rs | tbl_004_surface_keys_default_to_empty_lists_and_no_list_file | TBL-core-004 | C-core-config-6 | そのまま |
| crates/kotowari-core/tests/surface.rs | tbl_004_surface_keys_are_read_and_their_paths_normalized | TBL-core-004, REQ-core-224 | C-core-config-7 | この話題の分だけ。正規化は base-directory の行 |
| tests/step1_config.rs | tbl_004_tests_rules_is_read_and_defaults_to_empty | TBL-core-004, REQ-core-186 | C-core-config-6, C-core-config-7 | この話題の分だけ |
| tests/step1_config.rs | tbl_004_mutants_equivalents_is_read_and_has_no_default | TBL-core-004, REQ-core-013 | C-core-config-6, C-core-config-7 | そのまま |
| tests/step1_config.rs | req_014_overview_files_is_read_as_a_glob_list | REQ-core-014, TBL-core-004 | C-core-config-7 | 書いた glob の一覧がそのまま読まれる |
| tests/step1_config.rs | tbl_core_004_overview_toc_is_read_as_one_path | REQ-core-013, TBL-core-004 | C-core-config-7 | そのまま |
| tests/step1_config.rs | tbl_core_004_without_the_overview_key_no_overview_data_is_configured | REQ-core-013, TBL-core-004 | C-core-config-6 | "overview" が無ければ元データも目次も読まない |
| tests/step1_config.rs | req_013_explicit_limits_values_are_used_as_given | REQ-core-013, REQ-core-014 | C-core-config-27 | "limits" の値が読まれることを確かめている。"limits" を無くす提案の行を指し、実装のときに直す（"limits" を書くと設定の誤りで止まることを確かめるテストに書き換える） |
| tests/step1_config.rs | req_014_unknown_key_stops | REQ-core-014, EX-core-003 | C-core-config-12 | そのまま |
| tests/step1_config.rs | req_014_unparsable_yaml_stops | REQ-core-014 | C-core-config-12 | そのまま |
| tests/step1_config.rs | req_014_only_the_eleven_config_keys_are_accepted | REQ-core-014, TBL-core-004 | C-core-config-12, C-core-config-9, C-core-config-27 | 表に無いキーがどの階層でも止まることを確かめる。今は "ir"、"decisions.records"、"decisions.adr"、"limits" を受けるキーとして書いている。C-9 と C-27 は提案なので、実装のときに直す（それらのキーを、止まる側に移す） |
| tests/step1_config.rs | req_014_duplicate_key_stops | REQ-core-014 | C-core-config-13 | 準備の重複するキーが "ir" なら、切り替えのときにほかのキーに直す |
| tests/step1_config.rs | ex_core_383_a_duplicate_key_stops_with_one_line_naming_the_key | REQ-core-014, EX-core-383 | C-core-config-13 | 同上 |
| tests/step1_config.rs | req_014_wrong_type_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_null_value_stops | REQ-core-014 | C-core-config-14 | 準備が "ir:" なら、切り替えのときにほかのキーに直す |
| tests/step1_config.rs | req_014_null_tests_files_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_null_tests_rust_attributes_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_null_tests_rust_macros_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_null_vague_words_key_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_null_decisions_stops | REQ-core-014 | C-core-config-14 | "decisions" のキーは "decisions.documents" のために残る |
| tests/step1_config.rs | req_014_null_tests_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_null_tests_rust_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_null_limits_stops | REQ-core-014 | - | "limits" のキーの null だけを確かめる。"limits" が無くなるので、古い形のコードと一緒に消す（null の停止は C-core-config-14 のほかのテストが確かめる） |
| tests/step1_config.rs | req_014_null_tests_rules_stops | REQ-core-014 | C-core-config-14 | そのまま |
| tests/step1_config.rs | req_014_mutants_equivalents_that_is_not_a_string_stops | REQ-core-014 | C-core-config-14 | 型の違う値 |
| crates/kotowari-core/tests/surface.rs | req_014_null_or_absolute_surface_values_stop | REQ-core-014 | C-core-config-14, C-core-config-15 | そのまま |
| tests/step1_config.rs | req_014_a_null_absolute_or_non_string_overview_toc_stops | REQ-core-014 | C-core-config-14, C-core-config-15 | そのまま |
| tests/step1_config.rs | req_014_absolute_path_stops | REQ-core-014 | C-core-config-15 | 準備が "ir" の値なら、切り替えのときにほかのキーに直す |
| tests/step1_config.rs | req_014_absolute_tests_rules_stops | REQ-core-014 | C-core-config-15 | そのまま |
| tests/step1_config.rs | req_014_absolute_mutants_equivalents_stops | REQ-core-014 | C-core-config-15 | そのまま |
| tests/step1_config.rs | ex_core_544_a_glob_element_starting_with_a_slash_stops_as_a_config_error | EX-core-544, REQ-core-014 | C-core-config-15 | そのまま |
| tests/step1_config.rs | req_014_every_files_key_rejects_a_glob_element_starting_with_a_slash | REQ-core-014 | C-core-config-15 | そのまま |
| tests/step1_config.rs | req_014_windows_drive_letter_like_path_is_not_absolute | REQ-core-014 | C-core-config-26 | 戻した細部。"a:b" が絶対パスとして拒まれないことを確かめる。準備の "ir" のキーは無くなるので、実装のときにほかのパスのキーに直す |
| tests/step1_config.rs | req_014_invalid_glob_stops | REQ-core-014 | C-core-config-16 | そのまま |
| tests/step1_config.rs | req_014_invalid_overview_glob_stops | REQ-core-014 | C-core-config-16 | そのまま |
| tests/step1_config.rs | req_014_an_invalid_overview_glob_stops_check_with_a_config_error | REQ-core-014 | C-core-config-16 | そのまま |
| crates/kotowari-core/tests/surface.rs | req_014_an_invalid_surface_files_glob_stops | REQ-core-014 | C-core-config-16 | そのまま |
| tests/step15_guides.rs | req_014_an_unreadable_guides_glob_is_a_config_error | REQ-core-014 | C-core-config-16 | そのまま |
| tests/step1_config.rs | req_014_duplicate_vague_word_stops | REQ-core-014 | C-core-config-17 | そのまま |
| tests/step1_config.rs | req_014_empty_vague_word_stops | REQ-core-014 | C-core-config-25 | 戻した細部。終了コード2と空の標準出力を確かめる。準備の "ir"、"decisions.records"、"decisions.adr" のキーは、実装のときに外す |
| tests/step1_config.rs | req_014_negative_limit_stops | REQ-core-014 | - | "limits" の負の数。"limits" が無くなるので、古い形のコードと一緒に消す |
| tests/step1_config.rs | req_014_zero_limit_stops | REQ-core-014 | - | 同上（0）。古い形のコードと一緒に消す |
| tests/step1_config.rs | req_014_overview_without_toc_stops_with_a_config_error | REQ-core-014, TBL-core-004 | C-core-config-18 | そのまま |
| tests/step1_config.rs | req_014_overview_without_files_or_with_null_stops | REQ-core-014, TBL-core-004 | C-core-config-18, C-core-config-14 | そのまま |
| tests/step19_overview_build.rs | ex_core_505_build_stops_without_the_toc_key_or_the_toc_file | EX-core-505, REQ-core-013, REQ-core-325, TBL-core-001 | C-core-config-18 | この話題の分（"overview.toc" が無いと設定の誤り）だけ。TBL-core-001 の分は cli の対応表 |
| tests/step1_config.rs | req_015_list_replaces_default | REQ-core-015 | C-core-config-19 | そのまま |
| tests/step1_config.rs | req_016_empty_list_means_none | REQ-core-016 | C-core-config-20 | そのまま |
| tests/step1_config.rs | req_017_nested_keys | REQ-core-017 | C-core-config-21 | "decisions.records" と "decisions.adr" で確かめている。切り替えのときにほかの入れ子のキーに直す |
| tests/step1_config.rs | req_018_unreadable_dir_stops | REQ-core-018 | C-core-config-10 | 読み替え。"ir" の置き場が読めないときの停止。切り替えのときに準備を決定の文書の置き場に直す |
| tests/step1_config.rs | req_018_missing_ir_dir_stops_with_its_relative_path_and_the_os_error | REQ-core-005, REQ-core-018, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-config-10 | この話題の分だけ。同上 |
| tests/step1_config.rs | req_018_place_that_is_a_file_stops | REQ-core-018 | C-core-config-10 | 同上（置き場がファイル） |
| tests/step23_decision_documents.rs | check_stops_when_the_configured_place_does_not_exist | C-core-docs-15（この話題の ID を指していない） | C-core-docs-15, C-core-config-10 | この話題の ID を今は指していないが、"decisions.documents" の置き場が無いと停止することを確かめている。マーカーに C-core-config-10 を足すと、新しい形で C-10 を確かめるテストになる |
| tests/step1_config.rs | req_018_unreadable_records_dir_stops | REQ-core-018 | - | 判断の記録の置き場だけを確かめる。古い形のコードと一緒に消す |
| tests/step1_config.rs | req_018_unreadable_adr_dir_stops | REQ-core-018 | - | ADR の置き場だけを確かめる。古い形のコードと一緒に消す |
| tests/step1_config.rs | req_018_missing_records_dir_stops | REQ-core-018, TBL-core-001 | - | 同上（判断の記録） |
| tests/step1_config.rs | req_018_missing_adr_dir_stops | REQ-core-018, TBL-core-001 | - | 同上（ADR） |
| tests/step3_sources_terms.rs | req_018_broken_symlink_in_records_dir_stops | REQ-core-018, TBL-core-001 | - | 同上（判断の記録の置き場の下） |
| tests/step3_sources_terms.rs | req_033_file_symlink_in_records_dir_is_read | REQ-core-033, REQ-core-018 | - | この話題の分は判断の記録の置き場だけ。古い形のコードと一緒に消す |
| tests/step1_config.rs | req_018_unreadable_ir_subdirectory_stops | REQ-core-018, TBL-core-001, TBL-core-020 | C-core-clienv-5 | 置き場の下のディレクトリが読めないときの停止は cli-environment の行にある。準備を決定の文書の置き場に直す |
| crates/kotowari/src/ir_tests.rs | req_033_broken_symlink_in_a_subdirectory_stops | REQ-core-033, REQ-core-018, TBL-core-020 | C-core-clienv-5 | 置き場の下の先の無いシンボリックリンク。同上 |
| tests/native_paths.rs | byte_named_files_without_legacy_read_aliases_keep_the_unreadable_stop | REQ-core-313, REQ-core-018, REQ-core-198, REQ-core-224, EX-core-485 | C-core-clienv-5 | この話題の分だけ。走査で読めないファイル（名前のバイトが UTF-8 でない）で停止することは、cli-environment の行にある |
| tests/step4_test_discovery.rs | req_018_unreadable_directory_under_tests_stops | REQ-core-018, TBL-core-001 | C-core-config-11 | "tests.files" の走査の読めないディレクトリ |
| tests/step4_test_discovery.rs | req_079_broken_symlink_in_tests_stops | REQ-core-079, REQ-core-018 | C-core-config-11 | この話題の分だけ |
| tests/step4_test_discovery.rs | req_079_broken_symlink_outside_glob_stops | REQ-core-079, REQ-core-018 | C-core-config-11 | 同上 |
| tests/step17_surface.rs | req_018_a_dangling_symlink_in_the_surface_walk_stops | REQ-core-018, TBL-core-001 | C-core-config-11 | この話題の分だけ |
| tests/step18_overview.rs | req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops | REQ-core-018, REQ-core-278, TBL-core-001 | C-core-config-11 | 同上 |
| tests/step19_overview_build.rs | req_core_278_build_reads_the_ir_places_and_stops_like_check | REQ-core-278, REQ-core-018 | C-core-config-10 | 落とした細部のテスト。いちばん近い行を指す。この話題の分（build も置き場が無ければ check と同じく止まる）は、記録では LLM の細部（2026-10-02-whole-picture.md の A76）で、この話題の行は check に限る。準備の IR の置き場は、実装のときに決定の文書の置き場に直す。REQ-core-278 の分は全体像の元データの話題 |
| tests/step4_test_discovery.rs | req_019_hidden_directory_is_excluded_and_subdirectory_is_included | REQ-core-019 | C-core-config-22 | そのまま |
| tests/step4_test_discovery.rs | req_019_hidden_file_matched_by_glob_is_read | REQ-core-019 | C-core-config-23 | そのまま |
| tests/step18_overview.rs | ex_core_464_overview_data_in_a_named_hidden_directory_is_read | EX-core-464, REQ-core-278, REQ-core-019, REQ-core-288 | C-core-config-24 | この話題の分だけ |
| tests/step18_overview.rs | req_core_019_a_broad_glob_does_not_enter_hidden_directories_it_does_not_name | REQ-core-019, REQ-core-278 | C-core-config-24 | この話題の分だけ |
| tests/step18_overview.rs | req_core_019_a_brace_across_a_slash_in_overview_files_does_not_stop_the_check | REQ-core-019 | C-core-config-24 | 波括弧の中の "/" を含む glob でも止まらずに読む。波括弧の読み方の細部はテストの中に残る |
| tests/step18_overview.rs | ex_core_545_a_hidden_directory_named_only_inside_braces_is_not_entered | EX-core-545, REQ-core-019 | C-core-config-24 | そのまま |

## 落とした細部

### 安全に関わるもの

安全に関わる落とした細部は、もう無い。一覧（`safety-dropped.md`）の5件のうち2件は下の「戻した細部」で決定に戻し、3件は落としていないことを確かめた。

| 要求のどの部分 | 誰が決めたと記録にあるか | 今の下書き |
|---|---|---|
| REQ-core-018: 置き場が無いときだけでなく、読めないときも停止する | 裁定役（records.md の A95）。読めない入力は停止するという利用者の決定（cli-environment の「読めない入力と壊れた入力は…」、records.md の A100）に含まれる | 落としていない。「決定の文書の置き場が無いか…」の決めたことと C-core-config-10 にある（テスト req_018_unreadable_dir_stops、req_018_missing_ir_dir_stops_with_its_relative_path_and_the_os_error、req_018_place_that_is_a_file_stops、check_stops_when_the_configured_place_does_not_exist） |
| REQ-core-018: "tests.files" の走査で読めないディレクトリに出会ったら停止する。走査で先の無いシンボリックリンクに出会ったら停止する | 裁定役（records.md の A96、A146）。ガイドの置き場の走査を "tests.files" と同じに扱うと利用者が決めている（2026-09-24-doc-marks.md の A16） | 落としていない。同じ決定の決めたことと C-core-config-11 にある（テスト req_018_unreadable_directory_under_tests_stops、req_079_broken_symlink_in_tests_stops、req_079_broken_symlink_outside_glob_stops、req_018_a_dangling_symlink_in_the_surface_walk_stops、req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops） |
| REQ-core-018: 置き場の下のディレクトリが読めないときの停止 | 裁定役（records.md の A96）と、2026-09-16-ir-tree.md の A16（誰が決めたか書いていない） | 落としていない。cli-environment の C-core-clienv-5 にある（テスト req_018_unreadable_ir_subdirectory_stops、req_033_broken_symlink_in_a_subdirectory_stops） |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-014: "vague_words" に空の文字列の要素があれば、設定の誤りで停止する（裁定役、records.md の A93） | C-core-config-25（「設定ファイルが YAML として読めないか…」の決めたことの表の行を「空の文字列の要素と、同じ語の2回目」に広げた） | テスト req_014_empty_vague_word_stops（終了コード2と空の標準出力。設定の誤りの文言までは見ていないので、行は終了コード2の停止として書いた。コードは `crates/kotowari-core/src/config.rs` で "vague_words contains an empty string" の設定の誤りにしている） |
| REQ-core-014: 設定の値が絶対パスかどうかを先頭の "/" だけで見分け、Windows のドライブ文字は見ない（裁定役、records.md の A161。理由は「対象の OS に合わせる」） | C-core-config-26（同じ決定の表の「絶対パスの値」の補足と、問題の節） | テスト req_014_windows_drive_letter_like_path_is_not_absolute |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-core-004: "overview.files" のキーの名前と、"overview" を書くときは必須であること | LLM（2026-10-02-whole-picture.md の A71）。名前は利用者から見えるので残した。必須であることは、"overview" のキーで有効にして元データを指す glob を書くという利用者の決定（A32）が裏付ける |
| REQ-core-014: "overview" に "overview.toc" が無いときは、"overview.files" が無いときと同じく設定の誤り | LLM（2026-10-05-overview-index.md の A32）。"overview" があれば "overview.toc" も必須とする利用者の決定（A11）が裏付けるので、決めたことに残した |
| TBL-core-004: "decisions.records" の下に、判断の記録でない Markdown（形の契約、補足の文書）も置ける | 利用者（records.md の A69）。判断の記録が無くなるので消える |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| TBL-core-004 の行 "ir" | 新しい形で読み替えて残る。決定の文書の置き場の "decisions.documents" に置き換わる（`docs/decisions/core/decision-documents.md` の「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」）。キーの名前は今のコードの名前を使った。既定の "docs/decisions" はまだコードに無いので「提案」 |
| TBL-core-004 の行 "decisions.records"、"decisions.adr" | 消える（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」と「ADR はやめ、今あるものは凍結する」）。書かれていれば知らないキーとして止める、を提案の決定に書いた |
| TBL-core-004 の行 "limits.requirements"、"limits.lines" | 消える（利用者の答え: 行数の上限の注意は決定の文書ではやめ、要求の数の注意も無くす）。今のコードはまだキーを持つので、無くすことを提案の決定「行数と要求の数の上限のキー limits.lines と limits.requirements は無くす」に書いた |
| REQ-core-014 の「負の数、0」 | "limits" のキーにしか当たらないので、キーと一緒に消える |
| REQ-core-018 | "ir" の置き場は決定の文書の置き場に読み替えて残る。今のコードで実装済み。"decisions.records" と "decisions.adr" の置き場は消える |
| REQ-core-017 の例、REQ-core-014 の例 | 消えるキーを例にしていたので、残るキーの例に書き換えた |

テストの無い行: 提案の C-core-config-8、C-core-config-9、C-core-config-27 は、提案なのでテストを求めない（実装のときに足す。C-27 には req_013_explicit_limits_values_are_used_as_given を書き換えて当てる）。実装済みの行には、どれも付け替えたあとに指すテストがある。C-core-config-10 の「ディレクトリでない」と「読めない」を決定の文書の置き場で確かめるテストは今は無く、"ir" の置き場で確かめているテスト（req_018_place_that_is_a_file_stops、req_018_unreadable_dir_stops）の準備を、切り替えのときに決定の文書の置き場に直す。

決めたことの "guides.files" の走査の停止は、C-core-config-11 に入れていない。"guides.files" の走査で読めないディレクトリか先の無いシンボリックリンクに出会うことを確かめるテストが無いため（ガイドのファイルが読めないときの停止は guides の C-core-guides-6）。戻すなら、要るテストは「"guides.files" の glob の下に先の無いシンボリックリンクを置くと、check が unreadable file で停止する」で、C-core-config-11 の列挙に "guides.files" を足す。

## 実装する側が決める細部

- 古い形の置き場のキー（"ir"、"decisions.records"、"decisions.adr"）を書いたときの config error の説明の文。今は表に無いキーの説明と同じになる。
- "decisions" のキーの下が "documents" だけになったときの、設定ファイルの書き方の案内（kotowari のスキルの設定の参照と、ガイド）。

## 利用者に聞くこと

無い（行数の上限の問いは答えが出た。下の「迷った判断」）。

## 「提案」で書いた決定の一覧

- 「決定の文書の置き場は decisions.documents で指して既定を docs/decisions にし、古い形の置き場のキーは無くす」: `crates/kotowari-core/src/config.rs` の `DecisionsConfig`（`documents` を `Option` から既定 "docs/decisions" の文字列に、`records` と `adr` と最上位の `ir` を `RawConfig` と `RawDecisions` から外す）と、それを読む `crates/kotowari/src/decisions.rs` の `push`（キーが無いときに読まない分岐を外す）。
- 「行数と要求の数の上限のキー limits.lines と limits.requirements は無くす」: `crates/kotowari-core/src/config.rs` の `limits` のキー（`RawConfig` の欄と、負の数と0の検査）を外し、それを読む行数と要求の数の注意（too_many_lines、too_many_requirements）を古い形のコードと一緒に消す。

## 誰が決めたか見分けられなかったもの

- 2026-09-16-ir-tree.md の A16（IR の置き場の下のディレクトリが読めないときは停止する）: decided_by が無く、「敵対的レビューの指摘で足した」とだけある。cli-environment の決定に含まれるので、この話題には書かなかった。

## 理由や見送った案が記録に無かった決定

- 「設定ファイルは基準のディレクトリの .kotowari/config.yaml に置き…」: "--config" をカレントディレクトリからの相対で読む理由が記録に無い（records.md の A60）。
- 「設定ファイルが無いか空なら、既定の値で動く」: 理由が記録に無い（records.md の A12、A105、A135）。見送った案も無い。
- 「設定のキーは表のものだけにし…」: キーの表をまとめて決めた理由は無く、キーごとの理由はそれぞれの話題の記録にある。見送った案は、全体像を自動で有効にする案（2026-10-02-whole-picture.md の A32）だけを書いた。
- 「決定の文書の置き場は decisions.documents で…」: この決定そのものの記録は無い。決定の文書と切り替えの決定（`docs/decisions/core/`）から導いた。見送った案は無い。
- 「決定の文書の置き場が無いか…」: 置き場が無いときを読めないファイルの停止にした理由が記録に無い（records.md の A41、A66）。見送った案も無い。
- 「設定ファイルが YAML として読めないか…」: 見送った案が記録に無い。知らないキーで止める理由も記録に無い（A12）。
- 「一覧のキーは一覧だけを受け…」: 理由が記録に無い（records.md の A41、A59）。見送った案も無い。
- 「設定のキーは入れ子の形で読む」: 理由が記録に無い（records.md の A59）。見送った案も無い。
- 「glob の "**" は再帰で読み…」: 隠しディレクトリを除く理由が記録に無い（records.md の A59、A102）。見送った案も無い。

見送った案のうち、この話題の決定の代わりではないものは書かなかった: 2026-09-17-mutation-tests.md の A16 の rejected（等価の一覧を判断の記録に混ぜる。等価の一覧の話題で、しかも判断の記録が無くなる）、2026-10-02-whole-picture.md の A17 の rejected（全体像の元データの置き場。全体像の元データの話題）、2026-10-05-localization.md の A16 と A40 の rejected（言語の一覧と UI の文字の置き場。言語の話題）。

## 分けた所、まとめた所

- REQ-core-011、REQ-core-020 と、cli の REQ-core-003 を、設定ファイルの場所の1つの決定にまとめた。どれも「どのファイルを設定として読むか」で、R6（直下の kotowari.toml を退けた）が見送った案になる。
- TBL-core-004 を、残るキーの実装済みの決定と、決定の文書の置き場の提案の決定に分けた。1つの決定の中で実装済みと提案が混ざるため。
- REQ-core-015 と REQ-core-016 を1つの決定にまとめた。どちらも一覧のキーの読み方で、同じ決定（records.md の A41、A59）から来ている。
- REQ-core-018 のうち、置き場の下のファイルとディレクトリの停止は cli-environment の決定に寄せ、ここには置き場そのものと走査の停止だけを書いた。
- 値違いの場面（null の各キー、絶対パスの各キー、glob の各キー）は、それぞれ1行にまとめた。

## あとでリンクにする所

- `docs/decisions/core/decision-documents.md` の「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」と、`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」: 「決定の文書の置き場は decisions.documents で…」の問題の節
- 「文書の読み方と文書全体の検査」の話題（`ir-document.md`）: 「行数と要求の数の上限のキー…」の問題の節の too_many_lines。行数の上限の注意をやめる決定は、その話題の書き直しが今まさに見出しを変えているので、言葉で書いた。見出しが決まったらリンクにする

ここで済ませたもの（見出しへのリンクにした）: `decision-documents.ja.md` と `form-switch.ja.md`（提案の決定の問題の節）、guides.ja.md、surface.ja.md、overview-data.ja.md、overview-toc.ja.md、translation-pairs.ja.md、overview-languages.ja.md（キーの表）、`decision-documents.ja.md` の「用語集は人が読む文書として残し、用語の検査はやめる」（"vague_words"）、cli-environment.ja.md の対象の OS（C-26 の問題）。リンクは下書きどうしも `.ja.md` で書いた。

同時に依頼された cli と、同じフォルダにある base-directory、cli-environment、test-discovery、test-queries、query-rules、equivalents、surface-unspecified へも、見出しへのリンク（`.ja.md`）にした。

## 迷った判断と、どう決めたか

- 古い形の置き場のキーを、書かれていれば設定の誤りで止める、と提案の決定に書いた。利用者がこの形で決めた記録は無い。ただ、知らないキーで止めることは利用者の決定（records.md の A12）で、古い形を読む仕組みを作らないことも利用者の決定（`docs/decisions/core/form-switch.md`）なので、キーを無くせば止まる、はその帰結として書いた。黙って受けて読まない案は、受けて黙って無視すると付けた人が効いていると誤解する、という利用者の決定の理由（2026-09-24-plan-schema.md の A15）に反する。
- 行数の上限（利用者の答え）: 行数の上限の注意は決定の文書ではやめ、"limits.lines"、"limits.requirements"、too_many_requirements も無くす。今のコードがまだキーを持つので、無くす決定を「提案」で書いた。
- 戻した2件（C-core-config-25、C-core-config-26）は、新しい決定にせず、設定の誤りの決定の表の行と補足に足した。どちらも設定の誤りで止める条件の細部で、独立の振る舞いではないため。C-25 の行は、テストが確かめる範囲（終了コード2と空の標準出力）で書いた。
- 決定の文書の置き場が無いときの停止を、状態「実装済み」で書いた。今のコードは "decisions.documents" を書いたときだけ読むが、書いたときの停止はもう動いている。既定の値で読むことは提案の決定に分けた。
- 置き場が読めないとき（A95）と走査の停止（A96、A146）は裁定役の決定だが、落とさずに書いた。利用者の一般の決定（A100）と、ガイドの置き場を "tests.files" と同じに扱うという利用者の決定（doc-marks の A16）が、同じ振る舞いを裏付けるため。安全に関わる表に、そう書いた。
- records.md の第6〜第8ラウンドは、見出しに利用者の答え「推奨で」「推奨で問題無いと思う」「推奨で。細かい実装の判断なんだよな」がある。示された推奨を採ったものとして利用者の決定に数えた。第8ラウンドの中でも「裁定」と書いた A87 以降（A93、A95、A96）は、裁定役の決定として扱った。
- "vague_words" は、曖昧語の検査が決定の文書でも続くので、そのまま残した。既定の4語は決めたことの中に二重引用符で書いた（引用符の中は曖昧語の検査から外れる）。
- 全体像は "overview" のキーを書いたときだけ有効にする、という一文をキーの表の決定に入れた。キーの既定の値の意味を決める決定で、見送った案（自動で有効にする）を持つため。

## 数

- 元: 要求 10（REQ-core-011〜020）、場面 4、決定表 1、性質 0
- 読んだ記録のファイル: cli と合わせて 20（cli の対応表の「数」のとおり）
- 出来上がり: 決定 10（実装済み 8、提案 2）、確認することの行 27（うち「（読んで確認）」0、提案の決定の行 3）
- テスト: 84（この話題の ID か、この決定に入れた REQ-core-003 を指すもの 83、この話題の ID を指していないが付け替えを勧めるもの 1）
- 確定で: 戻した細部 2、落としていないと確かめたもの 3、直した「など」0
