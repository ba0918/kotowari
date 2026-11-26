# 安全に関わる落とした細部の一覧

schema と core の対応表（`*.map.md`）の「落とした細部」にある、安全に関わるものを1つの表にまとめた。記録の語は対応表のまま写した。確かめているものの列は、対応表に書いてあることだけを写した。

「（落としていないと map が書く）」を付けた行は、安全に関わるものの表には載っているが、map 自身が決定に残した、または別の問いに回したと書いている。

## 一覧

| # | まとまり/話題 | 落とした細部（何をする・しない振る舞いか） | 誰が決めたと記録にあるか | 今それを確かめているもの |
|---|---|---|---|---|
| 1 | schema / cli | REQ-schema-044: ディレクトリの検査で、シンボリックリンクを辿らない（同じ文書を二重に読む経路と、置き場の外へ出る経路を作らない） | 実装者（2026-09-21-mds-spec.md の A21。「利用者の明示の判断ではない」） | テスト check_directory_does_not_follow_symlinks |
| 2 | schema / cli | REQ-schema-043（テストだけ）: 停止の説明に、"$schema" が指した参照先のファイルの中身を出さない | 元の文書の外（テストのコメントだけ）。記録に決定が無い | テスト stop_does_not_carry_the_referenced_file_contents |
| 3 | schema / library-extraction | REQ-schema-068: Schema は不変の型で、外からの欄の変更や、検証していない直接の Deserialize で、意味の検査を迂回できない | 設計担当（2026-10-03-public-crate-api.md の A42。D1 の型設計の委任）。利用者の決定は見つからない | 無し（map に「これを直接確かめるテストは無い」とある） |
| 4 | schema / library-io | REQ-schema-073、EX-schema-090: I/O のクレートはカレントディレクトリを変更しない | 設計担当（2026-10-03-public-crate-api.md の A44。D1 の API 設計の委任） | テスト later_current_directory_changes_do_not_change_the_load_base |
| 5 | schema / schema-resolution | 相対パスの ".." を重ねたとき、前の ".." を打ち消さない（打ち消すと、文書が意図しないスキーマで黙って検証される） | 記録に無い（元の文書の外。テストのコメントだけ） | テスト consecutive_parent_refs_are_not_collapsed |
| 6 | core / adoption（落としていないと map が書く。「利用者に聞くこと」の問い5に回した） | TBL-core-037 の「既存の要求」の行: 実装と食い違う要求、確信が持てない要求を書き換えない。作る予定の無い要求を消さない | 利用者（2026-09-26-adoption.md の A8、A30。推奨を採用） | map に記載なし |
| 7 | core / cli-environment | TBL-core-020: カレントディレクトリを取得できないときは、読めないファイルを理由に停止し、詳細を "current directory: " と OS の誤りの文にする（停止の条件） | 実験者の裁定（records.md の A160。利用者には聞いていない） | map に記載なし |
| 8 | core / cli-scope | REQ-core-102 の例外の中身: ".kotowari/cache/overview/" の下のファイルはすべて build が作ったものとして扱い、今回返さなかったものを消す（消すものの範囲） | LLM（2026-10-02-whole-picture.md の A80。承認の時に利用者に示す）。利用者が決めたのは「その下の自分が作ったファイルだけを消す」（A28） | map に記載なし（テスト req_core_296_build_writes_nothing_outside_the_cache_place は置き場の外に書かないことを確かめるとあり、消す範囲は載っていない） |
| 9 | core / config | REQ-core-014: "vague_words" に空の文字列の要素があれば、設定の誤りで停止する | 裁定役（records.md の A93。「裁定。導いた元: A41 の型の検査」） | テスト req_014_empty_vague_word_stops（map に「今のコードとテストには残る」とある） |
| 10 | core / config | REQ-core-014: 設定の値が絶対パスかどうかを先頭の "/" だけで見分け、Windows のドライブ文字は見ない | 裁定役（records.md の A161） | テスト req_014_windows_drive_letter_like_path_is_not_absolute |
| 11 | core / config（落としていないと map が書く） | REQ-core-018: 置き場が無いときだけでなく、読めないときも停止する | 裁定役（records.md の A95） | テスト req_018_unreadable_dir_stops、req_018_missing_ir_dir_stops_with_its_relative_path_and_the_os_error、req_018_place_that_is_a_file_stops（C-core-config-10） |
| 12 | core / config（落としていないと map が書く） | REQ-core-018: "tests.files" の走査で読めないディレクトリに出会ったら停止する。走査で先の無いシンボリックリンクに出会ったら停止する | 裁定役（records.md の A96、A146） | テスト req_018_unreadable_directory_under_tests_stops、req_079_broken_symlink_in_tests_stops、req_079_broken_symlink_outside_glob_stops（C-core-config-11）。ほかに req_018_a_dangling_symlink_in_the_surface_walk_stops、req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops も C-core-config-11 |
| 13 | core / config（落としていないと map が書く。cli-environment の C-core-clienv-5 にある） | REQ-core-018: 置き場の下のディレクトリが読めないときの停止 | 裁定役（records.md の A96）と、2026-09-16-ir-tree.md の A16（誰が決めたか書いていない） | テスト req_018_unreadable_ir_subdirectory_stops、req_033_broken_symlink_in_a_subdirectory_stops（C-core-clienv-5） |
| 14 | core / consistency-phase | REQ-core-361: 仕様の誤りを示すのは、仕様どうしの矛盾か実現できないことの再現だけで、テストが通ることは根拠にしない | LLM（2026-10-06-changes-rethink.md の A29。「承認の時に利用者に示す」） | 今の reference に "Passing tests are not grounds" として残る（map の記述） |
| 15 | core / consistency-phase | REQ-core-361（A29）: フェーズは実装役と別の文脈の LLM が走らせる（実装役が自分の実装を追認しない） | LLM（A29）。利用者が決めたのは「別系統のモデルが望ましいが必須にしない」（A13）だけ | 今の cycle の skill に、実装役と別の文脈で、実装役や修正役には任せないと書いてある（map の記述） |
| 16 | core / finding-map | REQ-core-172: エンジンから写し先の無い種類の指摘か、「発生しない」と書いた種類の指摘を受けたら停止し、黙って捨てない。「出さない」で捨ててよいのは除外に列挙した入力だけ | 利用者（2026-09-22-ir-engine.md の A28、2026-09-23-ir-engine-gaps.md の A27・A36。推奨を採用） | テスト a_kind_whose_row_says_it_cannot_happen_stops、a_kind_whose_row_is_chosen_by_a_name_it_does_not_have_stops、a_table_header_mismatch_outside_the_glossary_stops_even_on_a_glossary_row_line（形に依存して消える）。計画書の側は plan の話題の invalid_plan の決定 C-core-plan-19 に残る（map の記述） |
| 17 | core / guides | REQ-core-199: 重なりの判定に、"guides.files" の glob に当たったファイルに加えて、同じ対にあるためにガイドとして読む側と、先頭の言語の側が無い側を数える | 整合のフェーズ（2026-10-23-guide-overlap-sides.md の A1。根拠は実測） | テスト step21 の2つ: req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap、req_core_199_a_side_found_by_name_counts_as_a_guide_in_the_overlap |
| 18 | core / ir-document | REQ-core-033: ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）は、名前が ".md" でも読まない（読むと固まりうる） | 実験者（records.md の A165。A74 に従って実験者が裁定した） | map に記載なし（今のコードが決定の文書の置き場でも共通の走査の関数で守る、とある。テスト req_033_hidden_dir_and_dir_symlink_are_not_followed_at_any_depth には「ソケットの分は落とした細部」とある） |
| 19 | core / library-api | REQ-core-311、EX-core-483: Project はプロセスのカレントディレクトリを変えない | 設計担当（2026-10-03-public-crate-api.md の A36。D1 の API 設計の委任） | テスト project_operations_reuse_owned_results_and_explicit_reads_observe_changes |
| 20 | core / library-api | REQ-core-311: 作業開始位置は絶対パスで受ける | 設計担当（A36、A52） | map に記載なし（library-paths の表の安全に関わる細部に同じものがある、とだけある。そこではテスト project_rejects_relative_roots_and_plan_does_not_require_ir） |
| 21 | core / library-api | TBL-core-041: overview_prepare は検査と描画をメモリで済ませ、何も書かない。書くのは overview_build（と準備の結果の書く操作）だけ | LLM（2026-10-04-overview-on-public-api.md の A7。計画の敵対的レビューで見つけた穴。「承認の時に利用者に示す」） | テスト overview_prepare_writes_nothing_and_overview_build_writes_the_cache |
| 22 | core / library-async | REQ-core-321、EX-core-495: Tokio のランタイムの外で待ったときと、タスクの実行に失敗したときに、panic せず型付きの失敗（RuntimeUnavailable、TaskFailure）を返す | 設計担当（2026-10-03-public-crate-api.md の A45。D1 の API 設計の委任） | テスト every_operation_is_send_and_reuses_sync_results_and_failure_kinds、loader_futures_and_results_are_send_and_match_sync_operations、runtime_shutdown_of_queued_blocking_work_is_a_typed_task_failure |
| 23 | core / library-inputs | REQ-core-315: InputMissing が見つけるのは渡していない群で、群の中の個々のファイルの漏れは見つけない。どのファイルを渡すかの網羅は呼び出し側が持つ | 設計担当（2026-10-03-public-crate-api.md の A50。D1 の型設計の委任） | map に記載なし |
| 24 | core / library-inputs | REQ-core-315: 同じ群の中の重複したファイルと、内部の取り込みでの元のファイルの食い違いを拒む | 設計担当（A51、A53） | テスト repeated_native_source_is_rejected_within_a_group、native_original_text_is_retained_across_read_and_check、repeated_original_test_identity_cannot_be_evaded_by_display_spelling |
| 25 | core / library-paths | REQ-core-323、EX-core-500: `ProjectOptions` と `LoaderOptions` の作業開始位置と、明示するキャッシュ基準は絶対パスで受け、相対パスなら InvalidInput にする | 設計担当（2026-10-03-public-crate-api.md の A52。D1 の型設計の委任。仕様レビュー R3 への対応） | テスト project_rejects_relative_roots_and_plan_does_not_require_ir、relative_start_and_cache_bases_are_rejected |
| 26 | core / library-paths | REQ-core-323、EX-core-501、library-api の REQ-core-311: あとでプロセスのカレントディレクトリが変わっても解く基準を変えない。Project と SchemaLoader はカレントディレクトリを変えない | 設計担当（A36、A44、A52） | テスト absolute_resolution_bases_survive_a_child_process_directory_change、later_current_directory_changes_do_not_change_the_load_base |
| 27 | core / library-paths | REQ-core-322、EX-core-498: 論理パスが空か絶対パスなら InvalidInput。同じ群の中で正規化したあとに重複するパスは InvalidInput | 設計担当（A51。why は「同じファイルを別物として扱うことを防ぐ」） | テスト logical_paths_are_normalized_without_accessing_the_filesystem（正規化と、絶対パス・空のパスを拒むことは落とした細部で、テストの中に残る）、duplicate_normalized_paths_and_cross_group_source_mismatches_are_rejected、public_admission_errors_keep_distinct_facade_classifications_and_details（絶対パスの SourceText を拒む） |
| 28 | core / library-paths | REQ-core-322、EX-core-503: 別の群の同じパスの元の本文が異なれば InvalidInput。テストと面の解析結果には解析に使った SourceText の全文を持たせ、本文で比べる | 設計担当（A51、A53） | テスト duplicate_normalized_paths_and_cross_group_source_mismatches_are_rejected、inspection_compares_original_test_and_surface_source_texts、cached_analysis_of_another_revision_is_rejected_at_native_admission |
| 29 | core / library-paths | REQ-core-322: ファイルシステムで正規化せず、シンボリックリンクも解かない | 設計担当（A51） | map に記載なし |
| 30 | core / list | REQ-core-152: 言語が1つのとき、"kotowari list" は全体像の元データを読まず、その置き場と読み込みによる停止（REQ-core-278、REQ-core-280）をしない | LLM（whole-picture の A73。敵対的レビューの指摘から導いた細部、承認の時に利用者に示す） | テスト req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it（map に「コードと step18 のテストに残る」とある） |
| 31 | core / overview-commands | REQ-core-293: ".kotowari/cache/overview/" の下のファイルはすべて build が作ったものとして扱い、今回返さなかったものは、元データから作ったものでなくても消す | LLM（2026-10-02-whole-picture.md の A80。承認の時に利用者に示す）。利用者が決めたのは「その下の自分が作ったファイルだけを消す」 | テスト req_core_293_any_other_file_under_the_cache_is_removed（テストの中に残る） |
| 32 | core / overview-commands | REQ-core-294: IR、元データ、目次の対に translation_missing か translation_structure_mismatch の誤りがあれば、build と serve は書かずに止まる（一致の記録の誤りと古い側では止めない） | LLM が推奨で決めた細部（2026-10-05-localization.md の D2） | テスト req_core_294_a_missing_side_stops_the_build_without_writing（テストの中に残る） |
| 33 | core / overview-commands | REQ-core-297、REQ-core-294: serve は、元データの検査、ポートの確保、書き込みの順に行い、誤りがあればポートを確保する前に止まる | LLM（A73） | テスト req_core_294_serve_on_data_with_errors_stops_before_binding |
| 34 | core / overview-data | REQ-core-278: build と serve も、参照を解くために check と同じ設定と置き場から仕様の文書を読み、その置き場が無いか読めなければ check と同じく停止する | LLM（2026-10-02-whole-picture.md の A73、A76）。ADR の置き場も読むことは照合で追認した LLM（2026-10-06-changes-rethink.md の A30） | テスト req_core_278_build_reads_the_ir_places_and_stops_like_check |
| 35 | core / overview-data | REQ-core-278: 元データのファイルが読めない、UTF-8 でない、走査で先の無いシンボリックリンクに出会うときの停止 | LLM（A76） | テスト req_core_278_overview_data_that_is_not_utf8_stops。走査の停止は config の C-core-config-11（req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops）、読めないファイルと UTF-8 でないファイルの停止は cli-environment の決まり（map の記述） |
| 36 | core / overview-data | REQ-core-280: ガイドとテストの重なりを先に判定し、3つすべてに当たるファイルではガイドとテストの重なりで止まる。停止の詳細は、パスのバイト順で最初の1つの相対パスに ": matched by both overview.files and guides.files" か ": matched by both overview.files and tests.files" を続ける | LLM（A73、A75）と整合のフェーズ（2026-10-23-overview-test-contents-consistency.md の A1） | テスト ex_core_465_overview_data_matched_by_guides_is_a_config_error、req_core_280_the_first_overlap_in_byte_order_is_reported_and_tests_overlap_too、req_core_280_the_guides_and_tests_overlap_is_judged_first_and_guides_win_when_all_three_match、req_core_280_build_judges_the_guides_and_tests_overlap_first |
| 37 | core / overview-data | REQ-core-280: ガイドとの重なりに、"guides.files" に当たる側と同じ対にあるためにガイドとして読む側、先頭の言語の側の無い側を数える | 整合のフェーズ（2026-10-23-guide-overlap-sides.md の A1。根拠は実測） | テスト req_core_280_a_side_without_its_first_side_matched_by_overview_and_guides_stops、req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap |
| 38 | core / overview-languages | REQ-core-352: 設定の "labels" に、言語の一覧に無い言語タグの鍵があれば、設定の誤りで停止する | LLM（2026-10-05-localization.md の D3） | map に記載なし |
| 39 | core / overview-languages | REQ-core-352: "labels.en" に表に無い鍵があれば、設定の誤りで停止する | LLM（D3） | テスト req_core_352_unknown_keys_and_a_doubled_number_placeholder_stop が "labels.ja" の分を確かめ、"labels.en" の分は「テストの中に残る」とある |
| 40 | core / overview-toc | REQ-core-326: 重なりは glob の文字列でなく、各置き場の走査で実際に読むファイルに目次のファイルが入るかで決める。目次を読む前に判定する | LLM（2026-10-05-overview-index.md の A28） | テスト req_core_326_the_toc_is_compared_with_the_files_each_walk_reads（前半の細部はテストの中に残る） |
| 41 | core / overview-toc | REQ-core-325: 目次のファイルが UTF-8 でないときの停止の理由を、読めないファイルでなく UTF-8 でないファイル（"non-UTF-8 file"）にする | LLM（A32。利用者の A17 は「元データと同じく読めないファイルを理由に止める」） | テスト req_core_325_a_toc_that_is_not_utf8_stops（テストの中に残る） |
| 42 | core / query | REQ-core-158: "kotowari query" は全体像の元データを読まず、その置き場と読み込みによる停止（REQ-core-278、REQ-core-280）をしない | LLM（whole-picture の A73。承認の時に利用者に示す） | テスト req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it（map に「コードと step18 のテストに残る」とある） |
| 43 | core / surface | REQ-core-224: "severity" が "off" の面の規則も当てる（規則を止める口を作らず、検査する面が黙って減らない） | 利用者（2026-09-27-surface-check.md の A20。「IR を書く段で決めた細部をまとめて採用」） | テスト req_224_rule_files_and_ignores_are_applied_and_severity_off_still_applies |
| 44 | core / surface-unspecified | REQ-core-232: 一致は "kind" と "name" の完全な一致で、前後の空白を除かない（外す面を広げない） | 利用者（2026-09-27-surface-check.md の A22。「IR を書く段で決めた細部をまとめて採用」） | テスト req_232_names_are_compared_without_trimming |
| 45 | core / surface-unspecified | REQ-core-233: 形の誤りとする条件のうち、1件が鍵と値の組でないことと、"why" を空とみなすときに除く文字（前後の半角空白とタブ） | 利用者（A22 と A6。A6 は「形の壊れた件は誤り」まで。条件の列挙は A22 のまとめての採用） | map に記載なし（C-core-unspec-3 に当たるテストとして ex_core_421_an_entry_whose_reason_is_blank_is_invalid_and_removes_nothing、req_233_each_malformed_entry_is_invalid_with_the_written_kind_and_name が載っている） |
| 46 | core / surface-unspecified | REQ-core-231: 停止の理由の種類（無いか読めない→読めないファイル、UTF-8 でない→UTF-8 でないファイル、YAML でないか最上位が並びでない→設定の誤り） | 利用者（A22、まとめて採用） | テスト ex_core_424_a_missing_list_file_stops_as_unreadable、ex_core_425_a_list_that_is_not_a_sequence_stops_naming_the_list、req_231_a_list_that_is_not_yaml_or_not_utf8_stops_check_and_status（C-core-unspec-5） |
| 47 | core / test-discovery | REQ-core-079: ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）は、glob に当たっても読まない（読むと固まりうる） | 実験者の裁定（records.md の A165。A74 に従い利用者に聞いていない） | map に記載なし |
| 48 | core / test-discovery | REQ-core-079: 走査は基準のディレクトリ全体（隠しディレクトリを除く）を歩いてから glob で選ぶので、glob に当たらない場所でも、読めないディレクトリと先の無いシンボリックリンクで停止する | 実験者の裁定（records.md の A159、読めないディレクトリでの停止は A96） | テスト req_079_broken_symlink_outside_glob_stops |
| 49 | core / test-discovery | REQ-core-079: 先の無いシンボリックリンクでは、読めないファイルを理由に停止する | 実験者の裁定（records.md の A146） | テスト req_079_broken_symlink_in_tests_stops |
| 50 | core / unattended-merge | REQ-core-366: マージへ進むのは、収束の終わり方に着いた回だけ。往復の上限、進まない、差し戻しで終わった回は人に報告し、マージを頼まない | 主セッション（2026-11-24-unattended-merge.md の A41。A3 と A17 から導いた） | 今の skill（kotowari-cycle、その references/merge.md、kotowari-brainstorm）に書いてある（表の下の文。この行だけの文は map に記載なし） |
| 51 | core / unattended-merge | REQ-core-365、366: 枝へコミットを足すたびに線を調べ直す。頼んだあとは、足す前に頼みを取り消し、足したあとの先頭で、収束、線に当たらない、人の確認が要る場合に当たらない、を確かめてから頼み直す。条件が崩れたときと人に渡すときも取り消す | 主セッション（A39） | 同上（skill の文） |
| 52 | core / unattended-merge | REQ-core-366: 頼みを取り消すかは、この回が頼んだかでなく、ホスティングのサービスがプルリクエストに頼みを示しているかで決め、コミットを足す委譲の前と push の前に読む。再開、さらに回す、iterate の枝の使い回しでも同じ | 整合のフェーズ（2026-11-24-unattended-merge-consistency.md の A7） | 同上（skill の文） |
| 53 | core / unattended-merge | REQ-core-366: 頼みが残っているかを読めないときは push せず、止まって人に伝える。プルリクエストを作れないホスティングだけは push へ進む | 整合のフェーズ（2026-11-24-merge-flow-fixes-consistency.md の A8） | 同上（skill の文） |
| 54 | core / unattended-merge | REQ-core-366: 自動のマージを頼む条件に、マージ先が承認のレビューを必須にしていないことを足す | 主セッション（2026-11-24-merge-flow-fixes.md の A10 の細部） | 同上（skill の文） |
| 55 | core / unattended-merge | REQ-core-366: 承認のレビューを必須にしていないと確かめるのは、枝の保護、ルールセット、プルリクエスト自身の reviewDecision の3つの読みがそろったときだけで、ほかの出力は確かめられなかったものとする。枝の保護は GraphQL の requiresApprovingReviews で読む | 整合のフェーズ（fixes-consistency の A1、A2） | 同上（skill の文） |
| 56 | core / unattended-merge | REQ-core-366: プルリクエストが cycle の受け取ったマージ先へ向いていることを頼む条件にし、再開で別の枝へ向いていたら頼みを取り消して人に渡す | 整合のフェーズ（fixes-consistency の A10） | 同上（skill の文） |
| 57 | core / unattended-merge | REQ-core-366: 頼む条件を確かめられなければ、プルリクエストで止まる | 主セッション（A35 の「確かめられなければ止まる」） | 同上（skill の文） |
| 58 | core / unattended-merge | REQ-core-366: 必須の検査の読みが1つでも失敗したら、マージ先が検査を必須にしているかは確かめられなかったものとし、見えている検査を見届けてから、失敗を付けてプルリクエストを人に渡す | 整合のフェーズ（fixes-consistency の A7） | 同上（skill の文） |
| 59 | core / unattended-merge | REQ-core-365: `.kotowari/confirm-paths` 自身は、一覧に書かれていなくても、手順の側で常に当たりとする | 主セッション（A19 の最後の2文。A16 から導いた） | 同上（skill の文） |
| 60 | core / unattended-merge | REQ-core-365: パスの指定は、マージ先の先頭にある一覧と枝の先頭にある一覧の両方から読み、どちらかで当たれば当たり。1行ずつ別々に照らし、パスを除く行がほかの行の当たりを消さない。手順のコマンドが失敗したら、当たったパスの無い当たりとし、失敗として報告する | 整合のフェーズ（unattended-merge-consistency の A5） | 同上（skill の文） |
| 61 | core / unattended-merge | REQ-core-365: 移したファイルは、元のパスと先のパスの両方で数える | 主セッション（A20 の細部） | 同上（skill の文） |
| 62 | core / unattended-merge | REQ-core-364: 枝の名前は依頼が終わったあと使い回さない。新しい依頼でマージ済みか閉じたプルリクエストがあれば止まって新しい枝の名前を求め、開いたプルリクエストの先頭が手元の枝に無ければ止まって伝える。後者はどの開始でも再開でも確かめる | 主セッション（merge-flow-fixes の A7。利用者が代決で承認した「先頭のコミットで見分ける」を取り替えた）、整合のフェーズ（fixes-consistency の A4） | 同上（skill の文） |
| 63 | core / unattended-merge | REQ-core-364: マージ先を決めておらず既定の枝も読めないときは、推測せずに人に聞く。開始のときにホスティングのサービスを読めなければ、上の2つの確かめを省いて進み、push の前に確かめ、できなかった確かめを最後の報告に書く | 整合のフェーズ（fixes-consistency の A11） | 同上（skill の文） |
| 64 | core / unattended-merge | REQ-core-364: 人がコミットしてよいと言ったときは壁打ちがコミットし、承認したファイルが新しい枝にコミットされていないあいだは主の作業ツリーを戻さず、どのファイルかを伝えて止まる | 整合のフェーズ（fixes-consistency の A12） | 同上（skill の文） |
| 65 | core / unattended-merge | REQ-core-364: 再開は依頼を持たない開始とし、指摘のファイルがある枝へ依頼を持ってきた開始は同じ枝へのあとの依頼として、仕様の承認を確かめ、実装からやり直し、マージ先との分かれ目から読む。開始は、プルリクエストを読む、仕様を確かめる、指摘のファイルを作るか書き換える、の順 | 整合のフェーズ（fixes-consistency の A9） | 同上（skill の文） |
| 66 | core / unattended-merge | REQ-core-364: プルリクエストを作ったあとの再開は、プルリクエストがあるか、頼んであるか、検査の状態、マージ済みかをホスティングのサービスから読む | 主セッション（A33 の細部） | 同上（skill の文） |
| 67 | core / unattended-merge | REQ-core-367: 人の判断を待つ指摘は、出どころ（整合のフェーズ、レビュー）を問わず、開いていればマージを頼まない。決定には、利用者が決めた整合のフェーズの指摘だけを書いた | 主セッション（A32 の細部。出どころを問わない形に広げた） | 同上（skill の文） |
| 68 | core / unattended-merge | REQ-core-368: セキュリティの指摘の扱いを、提案された扱いが直すことであるものに限る。人の判断を求めるセキュリティの指摘は、人の判断待ちとして開いたまま残り、人が確認する場合に数える | 主セッション（merge-flow-fixes の A5 の細部） | 同上（skill の文） |
| 69 | core / unattended-merge | REQ-core-369: 指摘のファイルは、片付けで消す作業用のディレクトリの外に置き、片付けの前にそこにあることを確かめる | 主セッション（A40） | 同上（skill の文） |
| 70 | core / unattended-merge | REQ-core-369: 片付けるのは、cycle が自分でマージを見たときだけ。作業用のディレクトリに実装役の作業メモのほかにコミットしていない変更があれば、消さずに報告する。人があとでマージした回は片付けない | 主セッション（A43） | 同上（skill の文） |
| 71 | core / unattended-merge | REQ-core-369: マージで終わった回の指摘のファイルは、あとで同じ名前の枝で始めた回の再開としては読まない | 主セッション（A45） | 同上（skill の文） |

## 件数

| まとまり | 話題 | 件数 |
|---|---|---|
| schema | cli | 2 |
| schema | library-extraction | 1 |
| schema | library-io | 1 |
| schema | schema-resolution | 1 |
| schema | 小計 | 5 |
| core | adoption | 1 |
| core | cli-environment | 1 |
| core | cli-scope | 1 |
| core | config | 5 |
| core | consistency-phase | 2 |
| core | finding-map | 1 |
| core | guides | 1 |
| core | ir-document | 1 |
| core | library-api | 3 |
| core | library-async | 1 |
| core | library-inputs | 2 |
| core | library-paths | 5 |
| core | list | 1 |
| core | overview-commands | 3 |
| core | overview-data | 4 |
| core | overview-languages | 2 |
| core | overview-toc | 2 |
| core | query | 1 |
| core | surface | 1 |
| core | surface-unspecified | 3 |
| core | test-discovery | 3 |
| core | unattended-merge | 22 |
| core | 小計 | 66 |
| 合計 | | 71 |

「落としていないと map が書く」行は #6、#11、#12、#13 の4件で、合計に含めている。

安全に関わるものが「なし」「無い」「無し」と書いてある話題（表に行が無い）:
- schema: block-rules、closed-world、document-structure、extraction、library、schema-language
- core: adoption-scope、base-directory、cli、coverage、equivalents、finding-order、findings、form-contract、ir-input、library-crates、mutants、mutants-input、output、overview-output、overview-workflow、plan、query-rules、skill-references、status、terms、test-markers、test-queries、test-side-findings、translation-pairs、translation-structure
- core の terms-form は「無い。どの要求も消える」とある。
- core の decision-records、deferred、ir-items、ir-missing、ir-references、record-form、revision-link、sources は「落とした細部」の節が無い（「消えない振る舞い（利用者に聞くこと）」の節だけ。ir-references の1件は安全の話ではなく ID の取り違えの検査）。

## 落とした細部ではなく、map が安全に関わる変化として書いているもの

- core/unattended-merge.map.md: REQ-core-367 の3つ目の場合（整合のフェーズが承認済みの要求を、矛盾か再現で誤りだと示して書き改めたら、人が確認する。A46、利用者の決定）が、あとの利用者の決定で置き換わって消える。map は「止まる条件が減るので、形に依存する要求の表で安全に関わる変化として書いた」と書く。
- core/finding-map.map.md: REQ-core-172（写し先の無い指摘で停止し、黙って捨てない）は、対応表が無くなるので形に依存して消える。map は「安全に関わる決まりなので」安全の表にも載せ、計画書の側の invalid_plan の決定（C-core-plan-19）に残ると書く（表の #16 と同じ項目）。
- core/adoption.map.md: 安全に関わる振る舞い（TBL-core-037 の既存の要求を書き換えない）は落としたのではなく、利用者の決定が要るため文書に書かず「利用者に聞くこと」の問い5に回した（表の #6 と同じ項目）。
- core/ir-input.map.md: 閉じないコードブロック（REQ-core-112）を、消える要求にせず提案の読み替えにした。理由は、決定の文書でも閉じないコードブロックのあとの決定と確認することの行が黙って消え、check が終了コード0で通ったため（map の記述）。
