# cli の対応表

元: `docs/ir/core/cli.md`（日本語は `cli.ja.md`）。書き直した文書: `cli.md`。REQ-core-003 は同時に書き直した `config.md` の決定に入れた。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-001 | C-core-cli-1 | kotowari のコマンドは7つで、check の1つで仕様の検査とテストとの対応の検査を両方行う | 読み替え。「IR の検査」を「仕様の文書の検査」と書いた。check は設定の "decisions.documents" があれば決定の文書を読んで検査し、確認することの行とテストの対応を数えるので、今のコードで実装済み |
| REQ-core-002 | C-core-cli-2, C-core-cli-3 | コマンドごとに受けるオプションを決め、オプションと位置引数はどの順に書いてもよい | そのまま。オプションの一覧を表にした。どの順でも書けることを C-2、check の "--allow-test-findings" を C-3 にした。"--help" と "--version" を受けることは cli-environment の C-core-clienv-1 が確かめる |
| REQ-core-003 | C-core-config-2 | 設定ファイルは基準のディレクトリの .kotowari/config.yaml に置き、--config で別のファイルを指せる | そのまま。設定ファイルの場所の決定と一緒に読むほうが分かるので、config.md の決定に入れた |
| REQ-core-004 | C-core-cli-4, C-core-cli-5, C-core-cli-6, C-core-cli-7, C-core-cli-8, C-core-cli-9, C-core-cli-10, C-core-cli-11 | 受けないオプション、知らないコマンド、余計な位置引数、値の誤ったオプションは、引数の誤りで停止する | そのまま。条件を表にし、確認することを条件の種類ごとに分けた。"--help" か "--version" があれば勝つことは cli-environment の C-core-clienv-2 |
| REQ-core-149 | C-core-cli-12, C-core-cli-13 | kotowari mutants は道具の名前を --tool で必ず受け、結果のファイルをちょうど1つ受ける | そのまま。引数の誤りの場面を独立の決定にした。"--tool" を必須にする理由と見送った案（2026-09-17-mutation-tests.md の A14）は mutants-input の決定にあるので、リンクにした |
| REQ-core-304 | C-core-cli-11 | 受けないオプション、知らないコマンド、余計な位置引数、値の誤ったオプションは、引数の誤りで停止する | そのまま。"--port" の値の範囲（1〜65535 の10進）は落とした細部 |
| REQ-core-005 | C-core-cli-14 | 停止するときは、標準出力に何も出さず、標準エラーの1行目に理由と詳細を出す | そのまま。文言の表（TBL-core-018）、詳細の表（TBL-core-020）、パスを基準のディレクトリからの相対にすること、英語で書くことは cli-environment の決定にあるので、リンクだけにした |
| REQ-core-006 | C-core-cli-14 | 停止するときは、標準出力に何も出さず、標準エラーの1行目に理由と詳細を出す | 読み替え。TBL-core-001 を定義に持つだけの要求。下の TBL-core-001 と同じ |
| REQ-core-007 | C-core-cli-15, C-core-cli-16 | 終了コードは、誤りが無ければ0、誤りがあれば1、停止したら2にする | そのまま |
| REQ-core-008 | C-core-cli-17 | 仕様の文書から構成と中身を写した、人向けの文書は生成しない | 読み替え。「IR から」を「仕様の文書から」にした。禁止の意図（機械的に写すと読みにくく二重に保つことになる）は形によらない。生成するコマンドが無いことは今のコードのとおりなので実装済み |
| TBL-core-001 | C-core-cli-14 | 停止するときは、標準出力に何も出さず、標準エラーの1行目に理由と詳細を出す | 読み替え。表は、ほかの話題の場面がどの停止の理由になるかの索引だった。新しい形では場面ごとの理由はその場面の話題の決定に書き、索引は持たない。読めないファイルと UTF-8 でないファイルの一般の規則は cli-environment の「読めない入力と壊れた入力は…」にある。表の "ir"、"decisions.records"、"decisions.adr" の置き場と、判断の記録と ADR の UTF-8 は消える（下の「形に依存する要求」） |
| TBL-core-002 | C-core-cli-15, C-core-cli-16 | 終了コードは、誤りが無ければ0、誤りがあれば1、停止したら2にする | そのまま。表は決めたことに置いた。"--help" と "--version" の0は cli-environment の C-core-clienv-1 が、"--allow-test-findings" の0と1は test-side-findings の C-core-testside-1 が確かめる |
| EX-core-001 | C-core-cli-4, C-core-cli-14, C-core-cli-16 | 受けないオプション、知らないコマンド、余計な位置引数、値の誤ったオプションは、引数の誤りで停止する | 停止すること、標準出力が空なこと、終了コード2を分けた |
| EX-core-218 | C-core-cli-12 | kotowari mutants は道具の名前を --tool で必ず受け、結果のファイルをちょうど1つ受ける | そのまま |
| EX-core-219 | C-core-cli-6 | 受けないオプション、知らないコマンド、余計な位置引数、値の誤ったオプションは、引数の誤りで停止する | 7つのコマンド名を挙げる文言は cli-environment の C-core-clienv-10 |
| EX-core-240 | C-core-cli-12 | kotowari mutants は道具の名前を --tool で必ず受け、結果のファイルをちょうど1つ受ける | 値違いの場面として C-12 の1行にまとめた |
| EX-core-242 | C-core-cli-13 | kotowari mutants は道具の名前を --tool で必ず受け、結果のファイルをちょうど1つ受ける | そのまま |
| EX-core-241 | C-core-cli-6 | 受けないオプション、知らないコマンド、余計な位置引数、値の誤ったオプションは、引数の誤りで停止する | EX-core-219 と値違いの場面として1行にまとめた |
| EX-core-244 | C-core-cli-2 | コマンドごとに受けるオプションを決め、オプションと位置引数はどの順に書いてもよい | そのまま |
| EX-core-380 | C-core-cli-2 | コマンドごとに受けるオプションを決め、オプションと位置引数はどの順に書いてもよい | 「IR に REQ-001 がある」の準備は、新しい形では確認することの ID を query に渡す準備になる。行そのものは形によらない |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step1_cli.rs | req_001_the_commands_run_and_other_words_are_not_commands | REQ-core-001 | C-core-cli-1, C-core-cli-5 | 6つのコマンドが動き、"mutate" で終了コード2。準備の IR の要求（"REQ-001"）は、切り替えのときに決定の文書と確認することの ID に直す |
| tests/step19_overview_build.rs | req_core_293_the_first_build_writes_every_page_under_the_cache | REQ-core-293, REQ-core-295, REQ-core-001 | C-core-cli-1 | この話題の分だけ。"overview build" がコマンドとして動くことを確かめる。ほかは全体像のコマンドの話題 |
| tests/step1_cli.rs | req_004_unknown_option_stops | REQ-core-004, REQ-core-005, REQ-core-007, REQ-core-008, EX-core-001, TBL-core-018, TBL-core-020 | C-core-cli-4, C-core-cli-14, C-core-cli-16 | 終了コード2、標準出力が空、1行目が "argument error: unknown option: --verbose"。REQ-core-008 の印は、受けるオプションの集合を固定する意図で付けた（2026-09-17-check-reach.md の A23）が、新しい行 C-17 は "render" の停止だけなので当たらない。TBL-core-018 と TBL-core-020 の分は cli-environment の対応表 |
| tests/step1_cli.rs | req_004_positional_argument_stops | REQ-core-004, REQ-core-005 | C-core-cli-10, C-core-cli-14 | "check extra" と "check render" で終了コード2、標準出力が空 |
| tests/step1_cli.rs | req_004_unknown_format_value_stops | REQ-core-004 | C-core-cli-8 | "--format xml" で停止 |
| tests/step1_cli.rs | req_004_missing_config_file_stops | REQ-core-004 | C-core-cli-9 | 無いファイルを "--config" で指して停止 |
| tests/step1_cli.rs | req_004_unknown_command_before_check_has_the_unknown_command_wording | REQ-core-004 | C-core-cli-5 | 1つ目の位置引数 "foo" で停止。1行目の "unknown command: foo" は cli-environment の C-core-clienv-11 も確かめる |
| tests/step1_cli.rs | req_004_no_arguments_names_all_seven_commands | REQ-core-004, REQ-core-005, TBL-core-020, EX-core-219 | C-core-cli-6, C-core-cli-14 | 引数が無いときに終了コード2、標準出力が空。7つのコマンド名の文言は cli-environment の C-core-clienv-10 |
| tests/step1_cli.rs | req_004_options_without_a_command_names_all_seven_commands | REQ-core-004, TBL-core-020, EX-core-241 | C-core-cli-6 | "--format text" だけで停止。文言は C-core-clienv-10 |
| tests/step1_cli.rs | req_107_help_and_version_exit_zero_without_check | REQ-core-107, REQ-core-002, TBL-core-002 | C-core-clienv-1 | この話題の分（"--help" と "--version" を受けること、その終了コード0）は cli-environment の C-core-clienv-1 が持つ |
| tests/step1_cli.rs | req_107_help_wins_over_argument_errors | REQ-core-107, REQ-core-004 | C-core-clienv-2 | この話題の分（"--help" も "--version" も無いとき、という条件）は cli-environment の C-core-clienv-2 が持つ |
| tests/step1_cli.rs | req_004_option_without_value_stops | REQ-core-004, REQ-core-005 | C-core-cli-8, C-core-cli-14 | "--format" の値が無くて停止、標準出力が空 |
| tests/step1_cli.rs | req_004_repeated_option_stops | REQ-core-004, REQ-core-005 | C-core-cli-8, C-core-cli-14 | "--format" が2回で停止、標準出力が空 |
| tests/step1_cli.rs | req_004_config_pointing_to_directory_stops | REQ-core-004, REQ-core-005 | C-core-cli-9, C-core-cli-14 | "--config ." で停止、標準出力が空 |
| tests/step1_cli.rs | req_002_options_before_or_after_check | REQ-core-002 | C-core-cli-2 | そのまま |
| tests/step1_cli.rs | req_005_config_error_detail_path_is_relative_to_base_not_to_cwd | REQ-core-005, TBL-core-020 | C-core-cli-14 | 1行目が "config error: <パス>:" の形。パスが基準のディレクトリからの相対であることは cli-environment の C-core-clienv-8。準備は IR の置き場の設定なので、切り替えのときに直す |
| tests/step1_cli.rs | req_005_config_outside_the_base_is_shown_relative_with_parent_segments | REQ-core-005, TBL-core-020 | C-core-cli-14 | 1行目の形。"../" を含む相対パスで示すことは記録では裁定役の細部（records.md の A164）で、cli-environment の C-core-clienv-8 の範囲 |
| tests/step1_cli.rs | req_002_mutants_options_can_come_before_the_command_and_after_the_path | REQ-core-002, EX-core-244 | C-core-cli-2 | そのまま |
| tests/step1_cli.rs | req_149_mutants_without_tool_is_an_argument_error | REQ-core-149, EX-core-218 | C-core-cli-12 | そのまま |
| tests/step1_cli.rs | req_149_unknown_tool_is_an_argument_error | REQ-core-149, EX-core-240 | C-core-cli-12 | そのまま |
| tests/step1_cli.rs | req_149_two_result_paths_is_an_argument_error | REQ-core-149, EX-core-242 | C-core-cli-13 | そのまま |
| tests/step1_cli.rs | req_149_mutants_without_a_result_path_is_an_argument_error | REQ-core-149 | C-core-cli-13 | 位置引数が0個 |
| tests/step1_cli.rs | req_004_tool_on_check_is_an_argument_error | REQ-core-004 | C-core-cli-7 | そのまま |
| tests/step1_cli.rs | req_152_tool_on_list_is_an_argument_error | REQ-core-152, REQ-core-004 | C-core-cli-7 | この話題の分だけ。REQ-core-152 は list の話題 |
| tests/step1_cli.rs | req_152_positional_after_list_is_an_argument_error | REQ-core-152, REQ-core-004 | C-core-cli-10 | この話題の分だけ |
| tests/step1_cli.rs | req_002_list_options_can_come_before_or_after_the_command | REQ-core-002 | C-core-cli-2 | そのまま |
| tests/step1_cli.rs | req_008_render_is_an_argument_error | REQ-core-008 | C-core-cli-17 | そのまま |
| tests/step1_cli.rs | req_004_positional_after_status_stops | REQ-core-004 | C-core-cli-10 | そのまま |
| tests/step1_cli.rs | req_004_tool_on_query_or_status_stops | REQ-core-004 | C-core-cli-7 | そのまま |
| tests/step1_cli.rs | req_002_query_and_status_options_before_or_after_the_command | REQ-core-002 | C-core-cli-2 | 準備の IR の要求は、切り替えのときに直す |
| tests/step1_cli.rs | req_002_check_runs_with_allow_test_findings | REQ-core-002 | C-core-cli-3 | そのまま |
| tests/step1_cli.rs | ex_core_552_allow_test_findings_on_status_is_an_argument_error | REQ-core-004, EX-core-552 | C-core-cli-7 | この話題の分だけ。EX-core-552 の分（C-core-testside-3）は test-side-findings の対応表 |
| tests/step1_cli.rs | ex_core_380_query_takes_an_option_after_the_id | REQ-core-002, EX-core-380 | C-core-cli-2 | 準備の IR の要求は、切り替えのときに確認することの ID に直す |
| src/cli/tests.rs | req_core_002_plan_takes_the_format_before_or_after_the_path | REQ-core-002 | C-core-cli-2 | そのまま |
| src/cli/tests.rs | req_core_304_the_lowest_port_is_1 | REQ-core-304 | C-core-cli-11 | ポート番号として読める値が通ることを確かめる側。範囲の下限は落とした細部で、テストの中に残る |
| src/cli/tests.rs | req_core_002_check_takes_allow_test_findings_without_a_value | REQ-core-002 | C-core-cli-3 | 前後どちらにも書けることを確かめる。値を取らないことは落とした細部（2026-10-06-spec-first-commit.md の A12、主セッション）で、テストの中に残る |
| src/cli/tests.rs | req_core_004_allow_test_findings_on_a_command_other_than_check_stops | REQ-core-004 | C-core-cli-7 | status の分は C-core-testside-3 も確かめる |
| src/cli/tests.rs | req_core_004_allow_test_findings_twice_on_check_stops | REQ-core-004 | C-core-cli-8 | 同じオプションの2回目 |
| tests/step14_plan.rs | req_core_004_tool_on_plan_stops | REQ-core-004 | C-core-cli-7 | そのまま |
| tests/step19_overview_build.rs | req_core_304_overview_takes_exactly_build_or_serve | REQ-core-304, TBL-core-001 | C-core-cli-11 | TBL-core-001 の分（引数の誤りの文言で止まること）は cli-environment の C-core-clienv-7 |
| tests/step19_overview_build.rs | req_core_304_the_port_is_a_decimal_integer_from_1_to_65535 | REQ-core-304 | C-core-cli-11 | 範囲と10進の細部はテストの中に残る |
| tests/step19_overview_build.rs | req_core_004_port_belongs_to_serve_and_format_does_not | REQ-core-004, REQ-core-002 | C-core-cli-7 | build の "--port" と serve の "--format" で停止 |
| tests/step19_overview_build.rs | req_core_002_build_takes_format_and_config_before_or_after_the_command | REQ-core-002, REQ-core-003 | C-core-cli-2 | REQ-core-003 の分は config の対応表（C-core-config-2） |
| tests/step6_output.rs | req_007_exit_code_one_on_error_and_zero_on_notice_only | REQ-core-007, TBL-core-002 | C-core-cli-15 | 準備が IR の要求の誤りと注意なので、切り替えのときに決定の文書の誤りと注意に直す |
| tests/step1_config.rs | req_006_non_utf8_config_stops | REQ-core-006, TBL-core-001 | C-core-clienv-7 | UTF-8 でない設定ファイルで "non-UTF-8 file" の文言で止まることを確かめる。当たる行は cli-environment にある |
| tests/step1_config.rs | req_018_missing_ir_dir_stops_with_its_relative_path_and_the_os_error | REQ-core-005, REQ-core-018, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-cli-14 | この話題の分だけ（標準出力が空で、1行目が「文言: 詳細」）。REQ-core-018 の分は config の対応表 |
| tests/step4_test_discovery.rs | tbl_001_unreadable_test_file_stops | REQ-core-006, TBL-core-001 | C-core-clienv-7 | 読めないテストのファイルで "unreadable file" の文言で止まる。当たる行は cli-environment |
| tests/step4_test_discovery.rs | tbl_001_non_utf8_test_file_stops | REQ-core-006, TBL-core-001 | C-core-clienv-7 | 同上（"non-UTF-8 file"） |
| tests/step4_test_discovery.rs | req_018_unreadable_directory_under_tests_stops | REQ-core-018, TBL-core-001 | C-core-clienv-7 | この話題の分は停止の理由の文言。REQ-core-018 の分は config の対応表 |
| tests/step3_sources_terms.rs | tbl_001_non_utf8_records_or_adr_stops | REQ-core-006, TBL-core-001 | - | 判断の記録と ADR だけを確かめる。古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_018_broken_symlink_in_records_dir_stops | REQ-core-018, TBL-core-001 | - | 判断の記録の置き場だけを確かめる。古い形のコードと一緒に消す |
| tests/step1_config.rs | req_018_missing_records_dir_stops | REQ-core-018, TBL-core-001 | - | 同上（判断の記録の置き場が無い） |
| tests/step1_config.rs | req_018_missing_adr_dir_stops | REQ-core-018, TBL-core-001 | - | 同上（ADR の置き場が無い） |
| tests/step15_guides.rs | ex_368_check_stops_when_a_guide_is_also_a_test_file | REQ-core-199, TBL-core-001, TBL-core-020, EX-core-368 | C-core-clienv-7 | この話題の分は停止の理由の文言だけ。場面はガイドのマーカーの話題 |
| tests/step15_guides.rs | req_198_a_guide_that_is_not_utf8_stops | REQ-core-198, TBL-core-001 | C-core-clienv-7 | 同上 |
| tests/step17_surface.rs | req_018_a_dangling_symlink_in_the_surface_walk_stops | REQ-core-018, TBL-core-001 | C-core-clienv-7 | この話題の分は停止の理由の文言。REQ-core-018 の分は config の対応表 |
| tests/step17_surface.rs | ex_core_424_a_missing_list_file_stops_as_unreadable | EX-core-424, REQ-core-231, TBL-core-001 | C-core-clienv-7 | この話題の分は停止の理由の文言。場面は未記載の面の一覧の話題 |
| tests/step17_surface.rs | req_231_a_list_that_is_not_yaml_or_not_utf8_stops_check_and_status | REQ-core-231, TBL-core-001, TBL-core-020 | C-core-clienv-7 | 同上 |
| tests/step18_overview.rs | req_core_278_overview_data_that_is_not_utf8_stops | REQ-core-278, TBL-core-001 | C-core-clienv-7 | この話題の分は停止の理由の文言。場面は全体像の元データの話題 |
| tests/step18_overview.rs | req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops | REQ-core-018, REQ-core-278, TBL-core-001 | C-core-clienv-7 | 同上。REQ-core-018 の分は config の対応表 |
| tests/step18_overview.rs | req_core_325_a_toc_that_is_not_utf8_stops | REQ-core-325, TBL-core-001 | C-core-clienv-7 | 同上 |
| tests/step18_overview.rs | ex_core_506_a_toc_matched_by_the_overview_glob_is_a_config_error | EX-core-506, REQ-core-326, TBL-core-001 | C-core-clienv-7 | 同上 |
| tests/step19_overview_build.rs | ex_core_477_errors_in_overview_data_stop_the_build_without_writing | EX-core-477, REQ-core-294, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7 | この話題の分は停止の理由の文言。場面は全体像のコマンドの話題 |
| tests/step19_overview_build.rs | ex_core_504_a_linked_cache_place_stops_without_writing_outside | EX-core-504, REQ-core-324, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7 | 同上 |
| tests/step19_overview_build.rs | ex_core_505_build_stops_without_the_toc_key_or_the_toc_file | EX-core-505, REQ-core-013, REQ-core-325, TBL-core-001 | C-core-clienv-7 | この話題の分だけ。REQ-core-013 の分は config の対応表 |
| tests/step19_overview_build.rs | ex_core_508_build_on_toc_errors_writes_nothing | EX-core-508, REQ-core-294, TBL-core-001 | C-core-clienv-7 | 同上 |
| tests/step20_overview_serve.rs | ex_core_479_a_port_in_use_stops_with_a_port_error | EX-core-479, REQ-core-298, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-clienv-7 | 同上 |
| tests/step1_config.rs | req_018_unreadable_ir_subdirectory_stops | REQ-core-018, TBL-core-001, TBL-core-020 | C-core-clienv-7 | この話題の分は停止の理由の文言。REQ-core-018 の分は config の対応表 |
| tests/step4_test_discovery.rs | tbl_016_macro_function_boundary_breaks_marker_binding | REQ-core-075, TBL-core-016（本体の最初のコメントに REQ-core-001 の文字） | - | REQ-core-001 は、準備を説明するコメントの中の文字で、コマンドを確かめていない。本体の最初のコメントなのでマーカーとして読まれうる。付け替えのときに、このコメントから "@kotowari[" の形を外す |
| crates/kotowari-overview/tests/overview.rs | req_core_286_a_malformed_guide_mark_in_overview_data_is_an_invalid_marker | （準備の文字列の中に REQ-core-001） | - | テストの中の文字列（全体像の元データの準備）で、テストのマーカーではない。全体像の元データの参照を決定の見出しに変えるときに、全体像の話題の側で直す |
| crates/kotowari-overview/tests/overview.rs | ex_core_471_an_unknown_frontmatter_key_is_a_form_error | （準備の文字列の中に REQ-core-001） | - | 同上 |
| crates/kotowari-overview/tests/overview.rs | req_core_292_only_html_comments_are_removed_from_heading_text | （準備の文字列の中に REQ-core-001） | - | 同上 |
| crates/kotowari-overview/tests/overview.rs | req_core_283_a_second_title_before_the_lead_is_the_first_block_after_the_title | （準備の文字列の中に REQ-core-001 と REQ-core-010） | - | 同上 |
| crates/kotowari-markdown-view/tests/rendering.rs | req_view_007_text_next_to_a_comment_is_not_drawn_as_code | （準備の文字列の中に REQ-core-001） | - | テストの中の文字列（コメントの描き方の準備）で、テストのマーカーではない。付け替えない |

## 落とした細部

### 安全に関わるもの

この話題の行は `safety-dropped.md` に無い。停止の条件は、どれも利用者の決定として残した（引数の誤りの表、"--config" の指す先が無いかディレクトリのとき）。下の「そのほか」の TBL-core-001 の「カレントディレクトリを取得できないとき」は安全に関わる細部で、cli-environment の対応表（`safety-dropped.md` の7）で扱う。確かめるテストが無いので戻しておらず、cli-environment の対応表の「テストの無い行」に要るテストを書いた。

## 戻した細部

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-304: "--port" の値は1から65535までの10進の整数 | LLM（2026-10-02-whole-picture.md の A72。「承認の時に利用者に示す」）。決めたことには「ポート番号として読めないと停止」とだけ書いた。"--port" の名前は利用者の決定（A57） |
| REQ-core-002: "--allow-test-findings" は値を取らない | 主セッション（2026-10-06-spec-first-commit.md の A12）。test-side-findings の対応表でも落としている |
| REQ-core-008: 全体像の参照が指す項目の本文をその場で開いて見せることは、この禁止に当たらない | LLM（2026-10-02-whole-picture.md の A73）。本文をその場で開くこと自体は view の決定（「参照はページの外へリンクせず、選ぶとその場で本文を開く」）にある |
| REQ-core-005: パスを基準のディレクトリからの相対にし、英語で書く | 利用者（records.md の A104）。cli-environment の決定にあるので、ここではリンクだけにした（落としたのではなく、書く場所を寄せた） |
| TBL-core-001: "kotowari check" か "kotowari status" で "surface.unspecified" の指す先が無いとき、"overview.toc" の指す先が無いとき、のような場面ごとの理由の索引 | 利用者の決定が多いが、それぞれの場面の話題の決定にある。索引としては持たない |
| TBL-core-001: カレントディレクトリを取得できないときは読めないファイルで停止する | 裁定役（records.md の A160。「実験者が A74 に従って裁定した」）。安全に関わる。cli-environment の対応表の「テストの無い行」で扱う |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-001 | 読み替えて残る。「IR の検査とテストとの対応の検査」を「仕様の文書の検査とテストとの対応の検査」にした。今のコードは設定の "decisions.documents" があれば決定の文書を検査し、確認することの行とテストの対応を数える |
| REQ-core-006、TBL-core-001 の「読めないファイル」の "ir"、"decisions.records"、"decisions.adr" の指すディレクトリと、その下のディレクトリ | 読み替えて残る（決定の文書の置き場）。config.md の「決定の文書の置き場が無いか、ディレクトリでないか、読めなければ、読めないファイルで停止する」に書いた。判断の記録と ADR の置き場は消える（`docs/decisions/core/form-switch.md` の「新しい版で一度に切り替え、古い形を読む仕組みは作らない」と「ADR はやめ、今あるものは凍結する」） |
| TBL-core-001 の「UTF-8 でないファイル」の IR の文書、判断の記録、ADR | IR の文書は決定の文書に読み替えて残る（cli-environment の「読めない入力と壊れた入力は…」の停止に含まれる。今のコードも決定の文書を UTF-8 として読み、読めなければ停止する）。判断の記録と ADR は消える |
| REQ-core-008 | 読み替えて残る。「IR から写した」を「仕様の文書から写した」にした |
| EX-core-380 | 準備の「IR に REQ-001 がある」は、確認することの ID に読み替える。行（オプションを ID の後ろに書ける）は形によらない |

テストの無い行: 無い。ただし、決定の文書が UTF-8 でないときに停止することを確かめるテストは無い（cli-environment の行の範囲。そちらの対応表で扱う）。

## 利用者に聞くこと

無い。

## 誰が決めたか見分けられなかったもの

無い。この話題の出典の決定は、decided_by か、ラウンドの見出し（records.md の第1〜第9ラウンドは「推奨を採用」、第9ラウンドの続きは裁定役）で見分けられた。

## 理由や見送った案が記録に無かった決定

- 「受けないオプション、知らないコマンド…」: 引数の誤りのそれぞれを停止にした理由は記録に無い（records.md の A60、A103、A136 に why が無い）。問題の節には、黙って通さないという一般の決定と、"kotowari plan" の "--config" の理由（2026-09-24-plan-schema.md の A15）だけを書いた。見送った案は無い。
- 「停止するときは、標準出力に何も出さず…」: 理由が記録に無い（records.md の A40、A104）。見送った案も無い。
- 「終了コードは…」: 理由が記録に無い（records.md の A20、A29）。見送った案も無い。
- 「コマンドごとに受けるオプションを決め…」: 見送った案が記録に無い。
- 「kotowari のコマンドは7つで…」: 見送った案の "kotowari render" と "kotowari generate" に、採らなかった理由が記録に無い（2026-10-02-whole-picture.md の A27）。

見送った案のうち、この話題の決定の代わりではないものは書かなかった: 2026-09-20-query-status.md の A1 の rejected（query の名前を show にする。理由が既存の記録の呼び名なので、記録が無くなる新しい形では意味が無い）、2026-10-02-whole-picture.md の A17 の rejected（全体像の元データの置き場。全体像の元データの話題）。

## 分けた所、まとめた所

- REQ-core-004 と REQ-core-304 を1つの決定にまとめた。どちらも「受けないもの・読めないものは引数の誤り」の同じ規則の場面で、"overview" の位置引数と "--port" の誤りは表の1行にした。
- REQ-core-149 は独立の決定にした。"mutants" の引数の誤りは位置引数の数も含み、"--tool" の理由と見送った案は mutants-input の決定へのリンクにした。
- REQ-core-005、REQ-core-006、TBL-core-001 を1つの決定にまとめた。REQ-core-006 と TBL-core-001 は場面ごとの理由の索引で、新しい形で独自に決めることは、停止の出力の形と、場面ごとの理由をその場面の話題で決めることだけだった。
- REQ-core-007 と TBL-core-002 を1つの決定にした。
- REQ-core-003 は config.md の、設定ファイルの場所の決定に入れた。
- EX-core-219 と EX-core-241、EX-core-218 と EX-core-240 を、それぞれ値違いの場面として1行にまとめた。

## あとでリンクにする所

残っていない。前に挙げた所は、どれもリンクにした（下書きどうしも `.ja.md` で書いた）。

- query.ja.md の「位置引数がちょうど1つの ID でないか、その ID が無ければ、引数の誤りで停止する」と、plan.ja.md の「kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える」: 「受けないオプション、知らないコマンド…」の位置引数の誤り
- overview-commands.ja.md（話題の文書へ）: 「kotowari のコマンドは7つで…」の build と serve
- overview-data.ja.md（話題の文書へ）と decision-documents.ja.md の「仕様は、話題ごとの、今有効な決定の文書だけで持つ」: 「仕様の文書から構成と中身を写した…」
- mutants-input.ja.md の「変異の結果は道具を知らない形で持ち…」と「形の合わない結果のファイルでは、結果の誤りで停止する」: 「kotowari mutants は道具の名前を --tool で必ず受け…」
- config.ja.md と cli-environment.ja.md（冒頭の段落と、停止の出力の決定）、test-side-findings.ja.md

## 迷った判断と、どう決めたか

- 「kotowari mutants は道具の名前を --tool で必ず受け…」から、"--tool" を必須にする理由と見送った案（既定の道具を置く、中身から判別する）を外し、mutants-input.ja.md の見出しへのリンクにした。同じ理由と見送った案を mutants-input が書いていて、"--tool" を必須にする振る舞いの中身はそちらで見える。この話題には、受けられないときの引数の誤り（C-core-cli-12、C-core-cli-13）だけを残した。結果の誤り（results error）の文言も mutants-input の決定にリンクし、ここには書かない。見出しは、ほかの話題からのリンクを壊さないため変えなかった。

- records.md の第9ラウンド（A98〜A144）は「問いは実験者が推奨で決め、利用者の答えは「進めて」」と見出しにある。細部を任せた承認ではなく、示された推奨をそのまま採ったものとして、利用者の決定に数えた（cli-environment の書き直しと同じ扱い）。第9ラウンドの続き（A145 以降）は「実験者が A74 に従って裁定した」とあるので、裁定役の決定として落とした。
- TBL-core-001 の索引は、新しい形では持たないことにした。場面ごとの理由はその場面の話題の決定にあり、索引を別に保つと食い違う。決定の見出しの列には、停止の出力の決定を書いた（ガイドのマーカーの付け替え先として、いちばん近いため）。
- 停止の理由の文言で止まることを確かめているテスト（TBL-core-001 を指すもの）は、ほかの話題の行 C-core-clienv-7 を付け替え先にした。この話題の行には、文言を確かめる行が無い。
- REQ-core-008 は「形に依存する」とも読めるが、禁止の意図（写すと読みにくく二重に保つことになる）は形によらないので、読み替えて実装済みで書いた。新しい形で、決定の文書は人がそのまま読める文章になるので、写した文書を作る理由はさらに無い。
- "--allow-test-findings" を表に入れた。振る舞いは test-side-findings の決定にあり、ここは受けるオプションの一覧だけを書いた（test-side-findings の対応表の「あとでリンクにする所」がこの話題を挙げている）。
- REQ-core-002 の "overview build" の受けるオプションは、記録では LLM が導いた細部（2026-10-02-whole-picture.md の A78）だが、"--format" に従うこと（A62）は利用者の決定で、"--config"、"--help"、"--version" はどのコマンドにも共通なので、表に残した。"overview build" が "--port" と "--tool" を受けないことの追認（2026-10-06-changes-rethink.md の A30）も LLM で、表に無いものは受けない一般の規則から出る。

## 数

- 元: 要求 10（REQ-core-001〜008、149、304）、場面 8、決定表 2、性質 0
- 読んだ記録のファイル: 20（records.md、2026-09-16-ir-tree.md、2026-09-16-notice.md、2026-09-17-check-reach.md、2026-09-17-mutation-tests.md、2026-09-19-read-commands.md、2026-09-20-query-status.md、2026-09-22-ir-engine.md、2026-09-24-doc-marks.md、2026-09-24-guide-gaps.md、2026-09-24-multi-language-tests.md、2026-09-24-plan-schema.md、2026-09-27-surface-check.md、2026-10-02-whole-picture.md、2026-10-04-overview-on-public-api.md、2026-10-05-localization.md、2026-10-05-overview-index.md、2026-10-06-changes-rethink.md、2026-10-06-spec-first-commit.md、2026-10-06-todo-zero.md）。config と合わせて読んだ数。ほかに、形の契約の ir-form.md の「出力」の節
- 出来上がり: 決定 7（実装済み 7、提案 0）、確認することの行 17（うち「（読んで確認）」0）
- 確定で: 戻した細部 0、戻せなかった細部 0（この話題の行は `safety-dropped.md` に無い）、直した「など」0
- テスト: 74（この話題の ID を指すもの 68、準備の文字列やコメントの中にこの話題の ID の文字があるもの 6）。REQ-core-003 だけを指す2本は config の対応表に載せた
