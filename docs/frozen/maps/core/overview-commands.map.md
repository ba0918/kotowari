# overview-commands の対応表

元: `docs/ir/core/overview-commands.md`（日本語は `overview-commands.ja.md`）。書き直した文書: `overview-commands.md`。確認することの ID の短い名前は `ovcmd`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-291 | C-core-ovcmd-13, C-core-ovcmd-14, C-core-ovcmd-29 | 参照の表には、参照ごとに参照先の本文を入れる | 読み替え（提案）。参照の表を作る相手が、IR と判断の記録から決定の文書に変わる。今のコードは IR の項目と出典の規則で解く（`crates/kotowari-overview/src/references.rs` の `Resolver`）ので提案にした。参照ごとに1件を作ることを C-13、本文に参照先の中身が入ることを C-14 にした。表示名、本文、状態の中身は TBL-core-039 の行ごと消える（下の行）。参照の状態は、利用者の答え（BRIEF-C の2）で、提案の決定への参照を後回しにし、置き換え済みを出さないことにした（C-29、提案）。言語ごとのページの本文は全体像の言語の話題（REQ-core-354） |
| REQ-core-292 | C-core-ovcmd-15 | マーカーが古い節は、古い節として描画に渡す | そのまま。マーカーの照らし方は共有の `GuideReader` で、もう決定を指すマーカーを読むので実装済みにした。最初の "## " より前のマーカーがどの節にも属さないこと、view に渡す見出しから HTML のコメントを除くことは落とした |
| REQ-core-293 | C-core-ovcmd-1, C-core-ovcmd-2, C-core-ovcmd-3, C-core-ovcmd-25 | overview build は、毎回すべてを描き、中身の違うファイルだけを書き、作られなくなったファイルを消す | そのまま。置き場の下にあって今回返されなかったファイルは、元データから作ったものでなくても消す、という範囲は、安全に関わる細部として C-25 に戻した。先頭の言語でない言語のページを "<言語タグ>/" の下に書くことは全体像の言語の話題（REQ-core-353）。ページの名前を先頭の言語の側の元データから取ることも同じ話題に任せた |
| REQ-core-294 | C-core-ovcmd-8, C-core-ovcmd-9, C-core-ovcmd-26, C-core-ovcmd-27, C-core-ovcmd-28 | 元データか目次に誤りがあれば、何も書かずに overview error で停止する | そのまま（一部は消える）。元データの誤りで止めることを C-8、serve も止めることを C-9 にした。目次の誤りでも止めることは[全体像の目次](overview-toc.md)の C-core-ovtoc-11 にあるので、行を足さずにリンクした。元データと目次の対に translation_missing か translation_structure_mismatch の誤りがあれば止めることは、安全に関わる細部として C-26 に戻した。そのうち IR の対の部分は、IR が無くなるので消える。serve の検査、ポートの確保、書き込みの順（REQ-core-297 と共通）は C-27、C-28 に戻した |
| REQ-core-295 | C-core-ovcmd-11, C-core-ovcmd-12 | overview build は、書いたファイル、消したファイル、書かなかったファイルの数を出す | そのまま。JSON の鍵が3つだけであること、パスのバイト順、text で written の行を先に出すこと、text の行の形（"written <パス>"、"removed <パス>"）は落とした |
| REQ-core-296 | C-core-ovcmd-4 | 全体像のコマンドは、.kotowari/cache/overview の下のほかに書かず、消さない | そのまま。例外を作ったこと自体は[使い手と作り](cli-scope.md)の決定にあり、この話題では書く先の範囲を確かめる |
| REQ-core-297 | C-core-ovcmd-16, C-core-ovcmd-17, C-core-ovcmd-18, C-core-ovcmd-27, C-core-ovcmd-28 | overview serve は、起動時に build と同じく書き、127.0.0.1 の1つのポートで置き場のファイルを配る | そのまま。検査、ポートの確保、書き込みの順は C-27、C-28 に戻した（決めたことは「元データか目次に誤りがあれば、…」の決定に置いた）。割り込みで終了コード0、拡張子で決める Content-Type、置き場に無いファイルに 404 を返すことは落とした |
| REQ-core-298 | C-core-ovcmd-19, C-core-ovcmd-20 | overview serve のポートが使えなければ、ほかのポートを試さずに port error で停止する | そのまま。停止の理由が読めないファイルでも引数の誤りでもないこと、"port error" の文言は[使い方の表示と対象の環境](cli-environment.md)の停止の理由の表にある |
| REQ-core-299 | C-core-ovcmd-21, C-core-ovcmd-22 | overview serve は、置き場の外のファイルとディレクトリの一覧を返さない | そのまま。道が "/" のときに "index.html" を返すことは落とした（C-core-ovcmd-16 のテストの中に残る） |
| REQ-core-356 | C-core-ovcmd-23 | overview serve は、手元の Host のリクエストにだけ答える | そのまま |
| REQ-core-305 | C-core-ovcmd-24 | overview build は、毎回すべてを描き、中身の違うファイルだけを書き、作られなくなったファイルを消す | そのまま。振る舞いも名前も記録では LLM が決めた細部だが、指摘の種類の名前は利用者から見える名前なので残し、ページの名前を元データの名前から取る利用者の決定と同じ決定に置いた（下の「迷った判断」）。先頭以外の言語の側を別の元データとして数えないことは[対の読み方と一致の記録](translation-pairs.md)の「全体像の元データのほかの言語の側は、別のページとして数えない」。パスのバイト順で2つ目以降に出すこと、"line" を null、detail を名前にすることは落とした |
| REQ-core-324 | C-core-ovcmd-5, C-core-ovcmd-6, C-core-ovcmd-7 | 置き場かその上がシンボリックリンクかディレクトリでなければ、何も書かずに cache error で停止する | そのまま |
| TBL-core-039 | - | 参照の表には、参照ごとに参照先の本文を入れる | 消える。表の行はすべて古い形（IR の要求、決定表、性質、シナリオ、判断の記録の決定、問題の記録の項目、ADR などの見出し）で、表示名と本文の決まりは行ごと無くなる。決定の見出しと確認することの ID の表示名と本文は「実装する側が決める細部」に挙げ、状態は利用者の答えで C-core-ovcmd-29 にした |
| EX-core-474 | - | 参照の表には、参照ごとに参照先の本文を入れる | 消える。置き換えられた判断の記録の決定と後回しの要求の状態は、どちらも古い形にしか無い。新しい形での状態は C-core-ovcmd-29（提案） |
| EX-core-475 | C-core-ovcmd-2, C-core-ovcmd-11 | overview build は、毎回すべてを描き、中身の違うファイルだけを書き、作られなくなったファイルを消す | そのまま。"unchanged" の数は C-11 |
| EX-core-476 | C-core-ovcmd-3 | overview build は、毎回すべてを描き、中身の違うファイルだけを書き、作られなくなったファイルを消す | そのまま |
| EX-core-477 | C-core-ovcmd-8 | 元データか目次に誤りがあれば、何も書かずに overview error で停止する | そのまま |
| EX-core-478 | C-core-ovcmd-16, C-core-ovcmd-21 | overview serve は、起動時に build と同じく書き、127.0.0.1 の1つのポートで置き場のファイルを配る | そのまま。配ることを C-16、外を指す道に 404 を C-21 にした |
| EX-core-543 | C-core-ovcmd-23 | overview serve は、手元の Host のリクエストにだけ答える | そのまま |
| EX-core-479 | C-core-ovcmd-19 | overview serve のポートが使えなければ、ほかのポートを試さずに port error で停止する | そのまま |
| EX-core-504 | C-core-ovcmd-5 | 置き場かその上がシンボリックリンクかディレクトリでなければ、何も書かずに cache error で停止する | そのまま |
| EX-core-546 | C-core-ovcmd-7 | 置き場かその上がシンボリックリンクかディレクトリでなければ、何も書かずに cache error で停止する | そのまま |

元の文書に PROP は無い。

ガイドのマーカー: `docs/guides/cli.md`（と `.ja.md`）の REQ-core-297 は、上の「決定の見出し」（overview serve は、起動時に build と同じく書き、127.0.0.1 の1つのポートで置き場のファイルを配る）に付け替える。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-overview/src/references.rs | decision_labels_drop_the_leading_date_and_the_extension | TBL-core-039 | - | 判断の記録の決定の表示名。古い形のコードと一緒に消す |
| crates/kotowari/tests/project.rs | overview_prepare_writes_nothing_and_overview_build_writes_the_cache | REQ-core-310, TBL-core-041, REQ-core-293, REQ-core-294 | C-core-ovcmd-1, C-core-ovcmd-2 | この話題の分は、build が置き場に3つのファイルを書き、2回目は書かないこと。prepare が書かないことは REQ-core-310 の話題の分。REQ-core-294 の分は当たる行が無い |
| crates/kotowari/tests/project.rs | overview_operations_fail_with_kinds_for_bad_data_and_a_missing_key | REQ-core-294, REQ-core-312, REQ-core-279 | C-core-ovcmd-8 | この話題の分は、誤りのある元データでライブラリの build の操作が元データの誤りで失敗すること（標準エラーの文言は CLI のテスト）。REQ-core-279 の分は全体像の元データの話題 |
| crates/kotowari/tests/project.rs | overview_build_stops_with_a_cache_error_when_the_place_is_not_a_directory | REQ-core-324, REQ-core-312 | C-core-ovcmd-6 | REQ-core-312 は別の話題 |
| crates/kotowari/tests/project.rs | overview_build_stops_with_a_cache_error_and_the_os_error_when_a_write_fails | REQ-core-324, REQ-core-312 | C-core-ovcmd-7 | 同上 |
| crates/kotowari/tests/project.rs | overview_write_names_the_place_it_could_not_inspect | REQ-core-324, TBL-core-020 | C-core-ovcmd-7 | TBL-core-020 は別の話題 |
| crates/kotowari-overview/tests/overview.rs | ex_core_474_superseded_decisions_and_deferred_requirements_carry_their_states | EX-core-474, REQ-core-291, TBL-core-039 | - | 古い形の状態だけを確かめる。古い形のコードと一緒に消す。新しい形の状態（C-core-ovcmd-29、提案）は、実装のときにテストを書く |
| crates/kotowari-overview/tests/overview.rs | tbl_core_039_requirements_show_their_statements_or_their_definition | REQ-core-291, TBL-core-039 | - | 古い形のコードと一緒に消す |
| crates/kotowari-overview/tests/overview.rs | tbl_core_039_tables_show_their_name_and_properties_their_statements | REQ-core-291, TBL-core-039 | - | 同上 |
| crates/kotowari-overview/tests/overview.rs | tbl_core_039_scenarios_show_the_scenario_line_and_steps_and_their_deferral | REQ-core-291, TBL-core-039 | - | 同上 |
| crates/kotowari-overview/tests/overview.rs | tbl_core_039_decisions_show_the_text_after_the_number | REQ-core-291, TBL-core-039 | - | 同上 |
| crates/kotowari-overview/tests/overview.rs | tbl_core_039_flag_items_show_their_body_lines | REQ-core-291, TBL-core-039 | - | 同上 |
| crates/kotowari-overview/tests/overview.rs | tbl_core_039_headings_of_other_markdown_files_show_their_section | REQ-core-291, TBL-core-039 | - | 同上 |
| crates/kotowari-overview/tests/overview.rs | req_core_291_one_entry_per_distinct_reference_across_all_overviews | REQ-core-291 | C-core-ovcmd-13 | 同じ参照が1件になる。提案の行で、準備の参照が要求の ID なので、実装のときに直す |
| crates/kotowari-overview/tests/overview.rs | req_core_292_a_section_is_stale_only_when_a_mark_in_it_is_stale | REQ-core-292 | C-core-ovcmd-15 | 準備のマーカーが "REQ-core-001:..." の古い形なので、付け替えのときに決定を指すマーカーに直す必要がある |
| crates/kotowari-overview/tests/overview.rs | req_core_292_only_html_comments_are_removed_from_heading_text | REQ-core-292 | C-core-ovcmd-15 | 落とした細部のテスト。いちばん近い行を指す。見出しから HTML のコメントだけを除くことは、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | valid_data_renders_into_the_pages_the_view_returns_for_its_input | REQ-core-291, REQ-core-292, C-view-rendering-1 | - | この話題の分は無い。C-view-rendering-1 はそのまま残すので、マーカーは無くならない。この話題の行に当たる確かめ（参照の本文、古い節）は無い |
| crates/kotowari-overview/tests/overview.rs | data_with_errors_gives_no_pages_and_the_error_count | REQ-core-294 | C-core-ovcmd-8 | 誤りがあればページを作らず誤りの件数を返す計算の層のテスト |
| crates/kotowari-overview/tests/overview.rs | req_core_294_toc_errors_count_as_errors_and_block_the_pages | REQ-core-294, REQ-core-328 | C-core-ovtoc-11 | 目次の誤りでページを作らない。その行は目次の話題にある。REQ-core-328 も目次の話題 |
| src/cli/serve.rs | a_failure_to_accept_while_serving_stops_with_a_port_error | REQ-core-298 | C-core-ovcmd-20 | そのまま |
| src/cli/serve.rs | an_interrupt_ends_serving_with_exit_code_zero | REQ-core-297 | C-core-ovcmd-18 | 終了コード0は落とした細部で、テストの中に残る |
| src/cli/serve.rs | req_core_299_paths_that_leave_the_cache_do_not_resolve | REQ-core-299 | C-core-ovcmd-22 | 外を指す道が置き場のファイルに解けないこと |
| crates/kotowari-core/tests/inputs.rs | a_source_resolves_to_the_decision_line_or_the_heading_section_it_points_at | REQ-core-291, TBL-core-039 | - | 出典の規則で判断の記録の決定と見出しの節に解く。古い形のコードと一緒に消す |
| tests/step19_overview_build.rs | req_core_293_the_first_build_writes_every_page_under_the_cache | REQ-core-293, REQ-core-295, REQ-core-001 | C-core-ovcmd-1, C-core-ovcmd-11 | REQ-core-001 は別の話題 |
| tests/step19_overview_build.rs | ex_core_475_a_second_build_writes_only_the_changed_page | EX-core-475, REQ-core-293, REQ-core-295 | C-core-ovcmd-2, C-core-ovcmd-11 | そのまま |
| tests/step19_overview_build.rs | ex_core_476_the_page_of_removed_overview_data_is_removed | EX-core-476, REQ-core-293 | C-core-ovcmd-3 | そのまま |
| tests/step19_overview_build.rs | req_core_293_any_other_file_under_the_cache_is_removed | REQ-core-293 | C-core-ovcmd-25 | 元データから作ったものでない "stray.txt" を消し、"removed" に出す |
| tests/step19_overview_build.rs | req_core_295_the_text_format_writes_written_lines_before_removed_lines | REQ-core-295 | C-core-ovcmd-12 | 行の並びは落とした細部で、テストの中に残る |
| tests/step19_overview_build.rs | ex_core_477_errors_in_overview_data_stop_the_build_without_writing | EX-core-477, REQ-core-294, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-ovcmd-8 | TBL の3つは別の話題 |
| tests/step19_overview_build.rs | req_core_296_build_writes_nothing_outside_the_cache_place | REQ-core-296, REQ-core-102 | C-core-ovcmd-4 | REQ-core-102 の分は[使い手と作り](cli-scope.md)の対応表で「-」 |
| tests/step19_overview_build.rs | req_core_296_a_symbolic_link_in_the_cache_is_replaced_not_written_through | REQ-core-296 | C-core-ovcmd-4 | 置き場の中のシンボリックリンクを辿って外に書かない |
| tests/step19_overview_build.rs | ex_core_504_a_linked_cache_place_stops_without_writing_outside | EX-core-504, REQ-core-324, REQ-core-296, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-ovcmd-5 | 外に書かないことも確かめるが、C-5 の文に入っている |
| tests/step19_overview_build.rs | req_core_324_a_link_above_the_cache_place_also_stops | REQ-core-324, REQ-core-296 | C-core-ovcmd-5 | そのまま |
| tests/step19_overview_build.rs | ex_core_508_build_on_toc_errors_writes_nothing | EX-core-508, REQ-core-294, TBL-core-001 | C-core-ovtoc-11 | 目次の誤りで build が書かずに止まる。その行は目次の話題にある |
| tests/step19_overview_build.rs | ex_core_546_a_cache_place_that_cannot_be_enumerated_stops_with_a_cache_error | EX-core-546, REQ-core-324 | C-core-ovcmd-7 | そのまま |
| tests/step20_overview_serve.rs | ex_core_478_serve_gives_only_the_files_in_the_cache_and_ends_with_0_on_interrupt | EX-core-478, REQ-core-297, REQ-core-299, REQ-core-294, REQ-core-296 | C-core-ovcmd-16, C-core-ovcmd-18, C-core-ovcmd-21 | 割り込みで終わることも確かめる。REQ-core-294 と REQ-core-296 の分は、このテストの確かめに当たらない |
| tests/step20_overview_serve.rs | req_core_299_links_out_of_the_cache_directories_and_absolute_paths_are_404 | REQ-core-299 | C-core-ovcmd-22 | そのまま |
| tests/step20_overview_serve.rs | req_core_299_a_percent_encoded_page_name_is_decoded_to_the_file_in_the_cache | REQ-core-299, REQ-core-297 | C-core-ovcmd-16 | 置き場のファイルを配る。道の百分率の符号を解くことは落とした細部で、テストの中に残る |
| tests/step20_overview_serve.rs | ex_core_479_a_port_in_use_stops_with_a_port_error | EX-core-479, REQ-core-298, TBL-core-001, TBL-core-018, TBL-core-020 | C-core-ovcmd-19, C-core-ovcmd-28 | ポートの誤りで止まり、".kotowari/cache" を作らない。TBL の3つは別の話題 |
| tests/step20_overview_serve.rs | req_core_294_serve_on_data_with_errors_stops_before_binding | REQ-core-294, REQ-core-297 | C-core-ovcmd-9, C-core-ovcmd-27 | 使用中のポートでも、元データの誤りを先に overview error で出す |
| tests/step20_overview_serve.rs | req_core_294_serve_on_toc_errors_stops_before_binding | REQ-core-294, REQ-core-328 | C-core-ovtoc-11 | 目次の誤りで serve が止まる。その行は目次の話題にある |
| tests/step20_overview_serve.rs | ex_core_543_serve_answers_only_local_hosts | EX-core-543, REQ-core-356 | C-core-ovcmd-23 | そのまま |
| src/cli/tests.rs | req_core_297_serve_uses_port_4590_unless_given | REQ-core-297 | C-core-ovcmd-17 | そのまま |
| tests/step21_translations.rs | three_languages_keep_their_own_html_language_and_contents | REQ-core-288, REQ-core-293, REQ-core-353, REQ-core-355 | - | この話題の分は無い（言語ごとのページは全体像の言語の話題で、その対応表が C-core-ovlang-2、C-core-ovlang-4 を付けるので、マーカーは無くならない）。REQ-core-288 の分は申告の話題 |
| tests/step21_translations.rs | a_linked_language_directory_stops_before_any_page_is_written | REQ-core-296, REQ-core-324, REQ-core-353 | C-core-ovcmd-5 | 言語のディレクトリがシンボリックリンクでも止まる。REQ-core-353 は言語の話題 |
| tests/step21_translations.rs | an_inaccessible_language_cache_reports_a_cache_error | REQ-core-324, REQ-core-353 | C-core-ovcmd-7 | 同上 |
| tests/step21_translations.rs | req_core_294_a_missing_side_stops_the_build_without_writing | REQ-core-294 | C-core-ovcmd-26 | 後半の、目次の対の骨組みの食い違いで build が止まり置き場を作らない部分が C-26 に当たる。前半は IR の対の側を消す準備で、古い形のコードと一緒に直す（元データの対の側を消す準備に替える） |
| tests/step21_translations.rs | req_core_294_overview_links_to_the_matching_guide_sides_do_not_stop_the_build | REQ-core-294, TBL-core-044 | - | この話題の分は無い（同じ言語のガイドを指すリンクで止まらない境目）。対の話題の対応表が C-core-pairstruct-5 を付けるので、マーカーは無くならない |
| crates/kotowari-overview/tests/overview.rs | req_core_305_duplicated_names_and_index_and_style_are_errors | REQ-core-305, REQ-core-027 | C-core-ovcmd-24 | REQ-core-027 は別の話題 |

この話題の ID を指すテストは 49。

## 落とした細部

### 安全に関わるもの

無い。BRIEF-C の1で、ここにあった3件をすべて決定に戻した（下の「戻した細部」）。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-293: ".kotowari/cache/overview/" の下のファイルはすべて build が作ったものとして扱い、今回返さなかったものは、元データから作ったものでなくても消す | C-core-ovcmd-25（決定「overview build は、毎回すべてを描き、…」） | テスト req_core_293_any_other_file_under_the_cache_is_removed |
| REQ-core-294: 元データと目次の対に translation_missing か translation_structure_mismatch の誤りがあれば、build と serve は書かずに止まる（IR の対の部分は消える） | C-core-ovcmd-26（決定「元データか目次に誤りがあれば、…」） | テスト req_core_294_a_missing_side_stops_the_build_without_writing の後半（目次の対の骨組みの食い違い）。前半は IR の対の側を消す準備で、付け替えのときに直す。skill の文 `agent/skills/kotowari/references/translations.md` の最後の行（"`kotowari overview build` and `serve` write nothing while an IR, overview data or table of contents pair has translation_missing or translation_structure_mismatch."）も、元データと目次の対の欠けた側を含めて書いている |
| REQ-core-297、REQ-core-294: serve は、元データの検査、ポートの確保、書き込みの順に行い、誤りがあればポートを確保する前に止まる | C-core-ovcmd-27、C-core-ovcmd-28（同じ決定） | テスト req_core_294_serve_on_data_with_errors_stops_before_binding（検査がポートより先）、ex_core_479_a_port_in_use_stops_with_a_port_error（ポートが書き込みより先） |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-291、TBL-core-039: 表示名（ID、日付と ".md" を除いた記録のファイル名と決定の番号、".md" を除いたファイル名と見出し）、本文（要求の文の並び、algorithm なら名前と定義、決定の番号より後の文字など）、状態の値の綴り "current"、"superseded"、"deferred" | LLM（A72、A81、A85、A86）。参照の表に表示名、本文、状態を持たせること（A25、A65、利用者）と、状態の3つ（A67、LLM が導き利用者が第21ラウンドで確認）は利用者の決定。行はどれも古い形で消える |
| REQ-core-292: 最初の "## " より前のマーカーはどの節にも属さない。view に渡す見出しの文字から HTML のコメントを除く | LLM（A73） |
| REQ-core-295: JSON の鍵が3つだけ、パスのバイト順、text で written の行を先に出す、text の行の形 "written <パス>" と "removed <パス>" | LLM（A73、A78） |
| REQ-core-297: 割り込みで終了コード0、".html" と ".css" の Content-Type、置き場に無いファイルに 404 | LLM（A73、A79） |
| REQ-core-298: 停止の詳細を "127.0.0.1:<ポート>" と ": " と OS の誤りの文で区切る | LLM（A85）。詳細に "127.0.0.1:<ポート>" と OS の誤りの文を置くこと（2026-10-04-overview-on-public-api.md の A14、利用者）は残した |
| REQ-core-299: 道が "/" のときは "index.html" を返す | LLM（A72） |
| REQ-core-305: overview_name_conflict をパスのバイト順で2つ目以降の元データに出し、"line" を null、detail を名前にする | LLM（A74） |
| REQ-core-294: 一致の記録の誤りと古い側では止めない | LLM（2026-10-05-localization.md の D2）。止めない側の境目なので安全の細部に数えず、確かめるテストも無い |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-291 | 新しい形で読み替えて残る。参照の表を作る相手が、IR の項目と判断の記録の決定から、決定の文書の決定と確認することの行に変わる。今のコードはまだ古い形を読むので提案 |
| TBL-core-039、EX-core-474 | 消える。どの行も IR の項目、判断の記録、問題の記録、ADR の形にしか意味が無い。参照が何を指せるかは `docs/decisions/core/languages-and-guides.md` の「全体像の元データは、決定の文書を指す」に置き換わった |
| REQ-core-292 | 新しい形で読み替えて残る。古い節を決めるマーカーの古さが、IR の項目の指紋から決定の「決めたこと」と「確認すること」の指紋に変わる（C-core-lang-9）。節とマーカーの結び付きは変わらない |
| REQ-core-294 | 一部が消える。IR の対の誤りで止める部分は IR と一緒に消える。元データと目次の対の部分は C-core-ovcmd-26 に戻した |

## テストの無い行

無い。実装済みの行は、どれも上の表のテストが1本以上指す。提案の C-core-ovcmd-14（本文に参照先の中身が入る）と C-core-ovcmd-29（参照の状態）には今テストが無く、実装のときに、決定の見出しを指す参照と確認することの ID を指す参照の両方で、本文に中身が入ることを確かめるテストと、提案と実装済みの決定を指す参照の状態を確かめるテストが要る。C-core-ovcmd-26 のテストの前半は IR の対の側を消す準備なので、元データの対の側を消す準備に直す。C-core-ovcmd-15 のテストは準備が古い形のマーカーなので、古い形を消すときに直す必要がある。

## 誰が決めたか見分けられなかったもの

- 無い。

## 理由や見送った案が記録に無かった決定

- 「全体像のコマンドは、.kotowari/cache/overview の下のほかに書かず、消さない」: 見送った案が記録に無い。
- 「置き場かその上がシンボリックリンクかディレクトリでなければ、…」: 見送った案（既存の「読めないファイル」で止める）は[使い方の表示と対象の環境](cli-environment.md)の決定にあるので、この文書では節を置かずにリンクした。
- 「元データか目次に誤りがあれば、何も書かずに overview error で停止する」: 見送った案が記録に無い（目次の誤りの見送った案「目次に無いページを未分類の群に置いて見せる」は、目次の話題の決定の代わり）。
- 「overview build は、書いたファイル、消したファイル、書かなかったファイルの数を出す」: 見送った案が記録に無い。
- 「参照の表には、参照ごとに参照先の本文を入れる」: 見送った案（リポジトリの URL へリンクする、表示名だけを出す）は `docs/decisions/view/rendering.md` の「参照はページの外へリンクせず、選ぶとその場で本文を開く」にあるので書かなかった。
- 「マーカーが古い節は、古い節として描画に渡す」: 見送った案が記録に無い。
- 「overview serve は、起動時に build と同じく書き、…」: 見送った案（"kotowari render"、"kotowari generate"）は[コマンドと終了](cli.md)の決定にあるので書かなかった。

## 分けた所、まとめた所

- REQ-core-297 と REQ-core-298 と REQ-core-299 と REQ-core-356 を、配ること、ポートの停止、置き場の外を返さないこと、Host の4つの決定にした。それぞれ別の記録か別の理由と見送った案を持つため。
- REQ-core-324 の「列挙の失敗」（別の記録の決定）は、置き場の誤りの決定の中の段落にした。同じ停止の理由に揃える決定だったため。
- REQ-core-293 に、HTML をコミットしないこと（出典の記録の決定）を入れた。ほかの話題に書かれていないため。
- REQ-core-305（overview_name_conflict）を、ページの名前の決定（REQ-core-293）と同じ決定に入れた。
- REQ-core-294 の目次の誤りで止めることは、目次の話題の C-core-ovtoc-11 がもう持つので、この文書では、止まるときの理由と詳細がこの決定のとおりであることだけを書き、行は足さなかった。
- serve の HTTP のサーバを自作しないこと（記録の依存の決定の一部）を、「overview serve は、置き場の外のファイルとディレクトリの一覧を返さない」の決めたことと見送った案に入れた。理由が道の検査の安全だったため。フォントを外から読み込まないことは view の決定にある。

## あとでリンクにする所

- languages-and-guides の「全体像の元データは、決定の文書を指す」と view の rendering の「参照はページの外へリンクせず、選ぶとその場で本文を開く」は、見出しへのリンクにした。このフォルダの下書きどうしは `<file>.ja.md#<anchor>` のリンクにした。
- [指摘の種類](findings.md)の overview_name_conflict の欄は、もうこの話題の「overview build は、毎回すべてを描き、…」へリンクしている（確かめた）

## 迷った判断と、どう決めたか

- overview_name_conflict（REQ-core-305）は、記録では振る舞いも名前も LLM が決めた細部（A73、A74）で、「利用者が決めていない細部は落とす」に当たる。一方で指摘の種類の名前は利用者から見える名前で、「必ず残す」に当たる。名前を残すほうを採り、利用者の決定（ページの名前を元データの名前から取り、一覧とスタイルを "index.html" と "style.css" にする）と同じ決定に置き、問題の節に「名前の重なる元データを誤りにする理由は残っていない」と書いた。[指摘の種類](findings.md)は元データの話題を指しているが、元の ID がこの話題にあり、ページの名前の決定と結び付くので、この話題に置いた。残してよいかの問いは、利用者の答え（残す。問いはもう無い）で消した。
- 参照の状態（A67）の問いは、利用者の答え（「後回し」は「提案」の決定への参照に付け、「置き換え済み」は出さない）で消し、「参照の表には、…」の提案の決定に段落と C-29 を足した。答えの「提案の決定への参照」に、その決定の確認することの行を指す参照も含めた。確認することの ID を指す参照の先は、その行の決定の状態で決まるため。全体像の元データの話題（overview-data）の依頼にも挙がっていたが、状態を持つのは参照の表なので、この話題に置いた。
- 「マーカーが古い節は、古い節として描画に渡す」は、今のコードが決定を指すマーカーをもう読むので実装済みにした。どの変更で古くなるかは C-core-lang-9（提案）で、この決定の行と重ねなかった。
- serve の Host の決定の問題に、「外のサイトの名前で届いたリクエストを通さない」を書いた。記録の why が、これを目的として書いているため。それより先の脅威の説明は記録に無いので書かなかった。
- build と serve が check と同じ置き場で止まること（REQ-core-278）は全体像の元データの話題の C-core-ovdata-24 に、対の誤りで止まること（REQ-core-294）はこの話題の C-26 に戻した。C-26 の問題の理由は、D2 の why（ほかの言語のページの本文をその言語の側から必ず取れるようにする）を、元データと目次の対に当てて書いた。
- C-26 の行は、テストが確かめる目次の対の骨組みの食い違いで書いた。元データの対の欠けた側で止まることは決めたことに書き、今のテストは IR の対の側を消す準備なので、準備を直すまで行の文には入れなかった。
- 第2ラウンドの records.md の A20（終了コード）と、第9ラウンドの A104（停止の理由の固定の文言）は、BRIEF-C の4のとおり、推奨をまとめて採った利用者の決定に数えた。終了コード2と "port error" などの文言を決めたことに残したのは、この読み方による。前の下書きと同じ読み方で、直す所は無かった。
- 今のコードは、ガイドと決定の文書の対の誤りでは build を止めない（`crates/kotowari-core/src/translations/pair.rs` の `overview_blockers`）。決定の文書の対の誤りで止めるかは記録に無いので書かず、「実装する側が決める細部」に挙げた。

## 実装する側が決める細部

- 決定の見出しを指す参照と、確認することの ID を指す参照の、参照の表の表示名と本文（見出しの文か ID か、本文を「決めたこと」だけにするか、確認することの行の文にするか）。今のコードには無い。ほかの言語のページで本文をその言語の側から取ることは[言語ごとの全体像と UI の文字](overview-languages.md)の提案の決定にある
- 決定の文書の対に translation_missing か translation_structure_mismatch の誤りがあるとき、build と serve を止めるか。今のコードは止めない。参照の本文をほかの言語の側から取る（overview-languages の提案）ようになれば、元データと目次の対と同じ理由が当たる


## 利用者に聞くこと

- 無い。参照の状態の問いと overview_name_conflict の問いは、BRIEF-C の2と依頼の答えで消した。

## 「提案」で書いた決定の一覧

- 「参照の表には、参照ごとに参照先の本文を入れる」: `crates/kotowari-overview/src/references.rs` の `Resolver::resolve` が IR の ID と出典の形で解いているのを、決定の文書の決定（見出しの行き先）と確認することの ID で解くように変え、表示名、本文、状態を決定の文書から作る。状態は、参照先の決定の「提案」か「実装済み」から後回しか今のものにする（C-29）。overview_ref_unresolved の判定（C-core-lang-8）も同じ所。

## 数

- 元: 要求 12、場面 9、決定表 1、性質 0
- 読んだ記録のファイル: 7（2026-10-02-whole-picture.md、2026-10-04-overview-on-public-api.md、2026-10-05-localization.md、2026-10-05-overview-index.md、2026-10-05-serve-host.md、2026-10-06-todo-zero.md、records.md の A20）。EX-core-546 の出典の ir-form.md#出力 は読んでいない
- 出来上がり: 決定 11（実装済み 10、提案 1）、確認することの行 28（うち「（読んで確認）」0）
- 戻した安全の細部: 3件（行 C-core-ovcmd-25〜28）
- テスト: この話題の ID を指すテスト 49
