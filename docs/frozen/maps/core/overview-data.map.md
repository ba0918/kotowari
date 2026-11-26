# overview-data の対応表

元: `docs/ir/core/overview-data.md`（日本語は `overview-data.ja.md`）。書き直した文書: `overview-data.md`。確認することの ID の短い名前は `ovdata`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-278 | C-core-ovdata-1, C-core-ovdata-16, C-core-ovdata-18, C-core-ovdata-19, C-core-ovdata-24 | 全体像の元データは、リポジトリにコミットして保守し、overview.files の glob で選ぶ | 読み替え（一部は落とした細部）。glob で選び、check、status、build、serve が読むことを C-1 にした。UTF-8 でない元データと、走査の先の無いシンボリックリンクでの停止を、安全に関わる細部として戻した（C-18、C-19）。build と serve がテストのファイルのパスだけを集めることは別の決定「overview build と overview serve は、テストのファイルのパスだけを集め、中身を読まない」（C-16）にした。build と serve が参照を解くために check と同じ置き場から仕様の文書を読み、置き場の停止も同じにする部分は、相手を決定の文書の置き場に読み替え、安全に関わる細部として別の決定「overview build と overview serve も、check と同じ置き場から決定の文書を読み、同じ条件で停止する」（C-24）に戻した。拡張子が小文字の ".md" だけを読むことは落とした |
| REQ-core-279 | C-core-ovdata-2, C-core-ovdata-17 | overview の鍵が無ければ、overview build と overview serve は設定の誤りで停止する | そのまま。build と serve の停止を C-2、check が読まないことを C-17 にした。全体像を "overview" の鍵で有効にすることは[設定](config.md)の決定にあるので、問題の節からリンクした。停止の詳細の文言（"overview is not configured"）は落とした |
| REQ-core-280 | C-core-ovdata-3, C-core-ovdata-4, C-core-ovdata-20, C-core-ovdata-21, C-core-ovdata-22 | 全体像の元データがガイドかテストの置き場と重なると、設定の誤りで停止する | そのまま。ガイドとの重なりを C-3、テストとの重なりを C-4 にした。安全に関わる細部として、停止の詳細の文言とバイト順で最初の1つ（C-20）、ガイドとテストの重なりを先に判定すること（C-21）、対の側を重なりに数えること（C-22）を戻した |
| REQ-core-281 | C-core-ovdata-5, C-core-ovdata-6 | 全体像の元データは、frontmatter と Markdown の本文で書き、部品は view の付いたフェンスの中に YAML で書く | そのまま。定義の TBL-core-038 と一緒に1つの決定にした。"ir" の一覧が何を指すかは languages-and-guides の「全体像の元データは、決定の文書を指す」に置き換わった。鍵 "ir" を改名することは別の提案の決定（C-23） |
| REQ-core-282 | C-core-ovdata-7, C-core-ovdata-8, C-core-ovdata-9, C-core-ovdata-10 | 部品の中身は、kotowari の側が view の部品のスキーマで検査する | そのまま。部品として読むことを C-7、知らない種類と合わない中身を C-8、detail の形を C-9、YAML の誤りの行を C-10 にした。値の全体の "(root)"、知らない鍵と欠けた鍵の名前を JSON Pointer に "/" で足すこと、同じ行と detail の誤りを1件にまとめることは落とした |
| REQ-core-283 | C-core-ovdata-11 | 題名の直後の最初の部品は、lead にする | そのまま。HTML のコメントと空行を飛ばすこと、"line" を null、detail を文書名にすることは落とした（"line" が null なのは[指摘の並べ方と行](finding-order.md)の C-core-forder-4 が確かめる） |
| REQ-core-284 | C-core-ovdata-12, C-core-ovdata-23 | 1つの決定の文書は、多くても1つの全体像の元データの一覧に入れる | 読み替え（提案）。「IR の話題ごとの文書」を「決定の文書」に読み替えた。今のコードは "ir" を IR の話題ごとの文書と照らす（`crates/kotowari-overview/src/lib.rs` の `Inspection::new` の `topics`）ので提案にした。無い文書を指す1件（overview_ir_missing）は languages-and-guides の C-core-lang-7 に置き換わった。overview_ir_missing と overview_ir_shared の改名は C-23（提案）。2つ目以降の元データに出すことと detail の形は落とした |
| REQ-core-285 | C-core-ovdata-13 | 部品の中の参照は、refs と ref の欄に書く | 読み替え。"refs" と "ref" の欄を参照として読むことは、形に関係ないので実装済みの C-13 にした。参照の形（IR の ID の項目かシナリオ、出典の規則で判断の記録）は消え、languages-and-guides の「全体像の元データは、決定の文書を指す」（C-core-lang-8）に置き換わった。入れ子の深さを問わないこと、スキーマに合う部品の中だけから集めることは落とした |
| REQ-core-286 | C-core-ovdata-14, C-core-ovdata-15 | 全体像の元データのマーカーは、ガイドのマーカーと同じ規則で読む | 読み替え。照らす相手が IR の項目から決定に変わる。今のコードは、元データのマーカーを、ガイドと同じ `GuideReader`（`crates/kotowari-core/src/guides.rs`）で読み、それはもう決定を指すマーカーを読むので実装済みにした。マーカーを見出しの次の行に、それだけの行として置くことは落とした（決めたことには、利用者が決めた「節の見出しの隣」を書いた） |
| REQ-core-287 | - | - | そのまま。kotowari-overview が計算だけを持ち、core が全体像を知らず、読み書きを kotowari のライブラリが、serve を kotowari-cli が行うことは、[ライブラリのクレート境界](library-crates.md)の「用途ごとにパッケージを分け、利用者は用途でパッケージを選ぶ」と「計算のクレートは環境を操作せず、Tokio と CLI の依存を持たない」に書いてある。check へ統合することは[メモリの入力と読み込んだ結果](library-inputs.md)の「名前を付けた追加の指摘の群を受け取り、ほかの指摘と一緒に並べて数える」と[全体像の元データの申告](overview-output.md)にある。もう一度は書かなかった |
| TBL-core-038 | C-core-ovdata-5, C-core-ovdata-6, C-core-ovdata-7 | 全体像の元データは、frontmatter と Markdown の本文で書き、部品は view の付いたフェンスの中に YAML で書く | そのまま。表の行は決めたことの文にした。スキーマのファイルの置き場とコンパイル時に取り込むこと、題名の後と節の下に置けるものの細部、frontmatter の無い・読めない・空の扱いは落とした。frontmatter の違反の detail を "frontmatter" にすることは[指摘の種類](findings.md)の決定にあるのでリンクした |
| EX-core-463 | C-core-ovdata-2, C-core-ovdata-17 | overview の鍵が無ければ、overview build と overview serve は設定の誤りで停止する | そのまま。check の "overview" の "files" が 0 で出ることは、出力の話題の落とした細部 |
| EX-core-464 | C-core-ovdata-1 | 全体像の元データは、リポジトリにコミットして保守し、overview.files の glob で選ぶ | そのまま。名指しした隠しディレクトリに入ることは[設定](config.md)の C-core-config-24 |
| EX-core-553 | C-core-ovdata-16 | overview build と overview serve は、テストのファイルのパスだけを集め、中身を読まない | そのまま |
| EX-core-465 | C-core-ovdata-3, C-core-ovdata-20 | 全体像の元データがガイドかテストの置き場と重なると、設定の誤りで停止する | そのまま。標準エラーの文言を C-20 に戻した |
| EX-core-466 | C-core-ovdata-8, C-core-ovdata-9 | 部品の中身は、kotowari の側が view の部品のスキーマで検査する | そのまま |
| EX-core-511 | C-core-ovdata-10 | 部品の中身は、kotowari の側が view の部品のスキーマで検査する | そのまま |
| EX-core-467 | C-core-ovdata-11 | 題名の直後の最初の部品は、lead にする | そのまま |
| EX-core-468 | C-core-ovdata-12 | 1つの決定の文書は、多くても1つの全体像の元データの一覧に入れる | 読み替え（提案）。IR の文書を決定の文書に読み替えた。前半の無い文書は C-core-lang-7 に当たる |
| EX-core-469 | C-core-ovdata-13 | 部品の中の参照は、refs と ref の欄に書く | 読み替え。例の参照（要求の ID、判断の記録の出典）は消える形なので、C-13 は解決できない参照に誤りが出ることだけにした。何を解決できないとするかは C-core-lang-8 |
| EX-core-470 | C-core-ovdata-14 | 全体像の元データのマーカーは、ガイドのマーカーと同じ規則で読む | 読み替え。指紋の違うマーカーを、決定への行き先のマーカーと読んだ |
| EX-core-471 | C-core-ovdata-5 | 全体像の元データは、frontmatter と Markdown の本文で書き、部品は view の付いたフェンスの中に YAML で書く | そのまま |

元の文書に PROP は無い。

ガイドのマーカー: `docs/guides/config.md`（と `.ja.md`）の REQ-core-280 は、上の「決定の見出し」（全体像の元データがガイドかテストの置き場と重なると、設定の誤りで停止する）に付け替える。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-overview/tests/overview.rs | valid_overview_data_has_no_findings_and_counts_its_files | REQ-core-281, TBL-core-038, REQ-core-278 | C-core-ovdata-1 | 正しい元データを読み、指摘が無く、1つと数える。準備の "ir" が IR の話題の文書を指すので、付け替えのときに決定の文書を指す準備に直す必要がある |
| crates/kotowari-overview/tests/overview.rs | ex_core_466_an_unknown_part_and_a_part_with_an_unknown_key_are_errors | EX-core-466, REQ-core-282, REQ-core-027, TBL-core-019 | C-core-ovdata-8, C-core-ovdata-9 | 知らない種類と知らない鍵。REQ-core-027 と TBL-core-019 は別の話題（指摘の並べ方の対応表では「-」） |
| crates/kotowari-overview/tests/overview.rs | req_core_282_places_name_missing_keys_the_whole_value_and_unreadable_yaml_once_each | REQ-core-282, REQ-core-027 | C-core-ovdata-9 | detail の場所の形。"(root)" と1件にまとめることは落とした細部で、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | req_core_282_each_unknown_key_and_each_value_that_does_not_fit_has_its_own_place | REQ-core-282 | C-core-ovdata-9 | 合わなかった場所ごとの detail |
| crates/kotowari-overview/tests/overview.rs | ex_core_467_a_first_part_that_is_not_lead_is_an_error_without_a_line | EX-core-467, REQ-core-283, REQ-core-027 | C-core-ovdata-11 | line が null であることは C-core-forder-4 の分（その対応表に書いてある） |
| crates/kotowari-overview/tests/overview.rs | req_core_283_comments_and_blank_lines_before_the_lead_are_skipped | REQ-core-283 | C-core-ovdata-11 | 落とした細部のテスト。いちばん近い行を指す。HTML のコメントと空行を飛ばすことは、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | ex_core_468_a_missing_ir_document_and_one_shared_by_two_overviews_are_errors | EX-core-468, REQ-core-284, REQ-core-027 | C-core-ovdata-12, C-core-lang-7 | 2つの元データに属する文書を C-12、無い文書を C-core-lang-7 に当てた。どちらも提案で、準備が IR の文書なので実装のときに直す |
| crates/kotowari-overview/tests/overview.rs | req_core_284_a_valid_ir_list_is_checked_beside_other_frontmatter_violations | REQ-core-284, REQ-core-281 | C-core-lang-7 | ほかの frontmatter の違反と並んでも一覧の1件を照らす。提案の行なので実装のときに直す |
| crates/kotowari-overview/tests/overview.rs | req_core_284_only_topic_documents_count_as_ir_documents | REQ-core-284 | C-core-lang-7 | 話題の文書だけを一覧の相手に数える。決定の文書で何を数えるかの細部は実装する側が決める。実装のときに直す |
| crates/kotowari-overview/tests/overview.rs | ex_core_469_unresolved_references_are_errors_one_per_reference | EX-core-469, REQ-core-285, REQ-core-027 | C-core-ovdata-13, C-core-lang-8 | 準備の参照が要求の ID と判断の記録の出典なので、付け替えのときに決定の見出しと確認することの ID に直す必要がある。C-core-lang-8 は提案なので、実装のときに直す |
| crates/kotowari-overview/tests/overview.rs | req_core_285_ref_fields_at_any_depth_are_references_and_invalid_parts_are_not_read | REQ-core-285 | C-core-ovdata-13 | "refs" と "ref" の欄を読む。入れ子の深さとスキーマに合わない部品を読まないことは落とした細部 |
| crates/kotowari-overview/tests/overview.rs | ex_core_470_a_stale_guide_mark_in_overview_data_is_a_guide_stale_notice | EX-core-470, REQ-core-286 | C-core-ovdata-14 | 準備のマーカーが "REQ-core-001:..." の古い形なので、付け替えのときに決定を指すマーカーに直す必要がある |
| crates/kotowari-overview/tests/overview.rs | req_core_286_a_malformed_guide_mark_in_overview_data_is_an_invalid_marker | REQ-core-286 | C-core-ovdata-15 | 同上 |
| crates/kotowari-overview/tests/overview.rs | ex_core_471_an_unknown_frontmatter_key_is_a_form_error | EX-core-471, REQ-core-281, TBL-core-038 | C-core-ovdata-5 | そのまま |
| crates/kotowari-overview/tests/overview.rs | tbl_core_038_frontmatter_must_hold_only_a_nonempty_ir_list_of_strings | REQ-core-281, TBL-core-038, REQ-core-027 | C-core-ovdata-5 | 知らない鍵を含む違反。無い・空・文字列でないの扱いと detail は落とした細部で、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | tbl_core_038_the_title_is_exactly_one_heading_of_depth_one | REQ-core-281, TBL-core-038, REQ-core-027 | C-core-ovdata-6 | 題名の無い元データ。2つ目の題名の扱いは落とした細部 |
| crates/kotowari-overview/tests/overview.rs | tbl_core_038_before_the_first_section_only_parts_and_comments_are_allowed | REQ-core-281, TBL-core-038 | C-core-ovdata-6 | 落とした細部のテスト。いちばん近い行を指す（形の崩れが overview_form_invalid になる）。最初の節より前に置けるものは、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | tbl_core_038_a_section_takes_text_lists_tables_code_comments_and_subheadings | REQ-core-281, TBL-core-038 | C-core-ovdata-6 | 落とした細部のテスト。いちばん近い行を指す。節の下に置けるものは、テストの中に残る |
| crates/kotowari-overview/tests/overview.rs | tbl_core_038_a_part_is_a_fence_whose_info_string_is_view_and_a_kind | REQ-core-281, TBL-core-038, REQ-core-282 | C-core-ovdata-7, C-core-ovdata-8 | "view" のフェンスを部品として読み、知らない種類を誤りにする |
| crates/kotowari-overview/tests/overview.rs | parts_after_the_lead_and_before_the_first_section_reach_the_page_in_order | TBL-core-038, C-view-rendering-10 | - | この話題の分は、最初の節より前に部品を置けること（落とした細部）。C-view-rendering-10 はそのまま残すので、マーカーは無くならない |
| crates/kotowari-overview/tests/overview.rs | req_core_283_a_second_title_before_the_lead_is_the_first_block_after_the_title | REQ-core-283, REQ-core-281 | C-core-ovdata-11 | 題名の直後が lead でないときの誤り |
| crates/kotowari-overview/tests/overview.rs | req_core_284_one_file_listing_a_document_twice_does_not_share_it | REQ-core-284 | C-core-ovdata-12 | 1つの元データに2回書いても overview_ir_shared にならない。提案の行なので実装のときに直す |
| crates/kotowari-core/tests/inputs.rs | guide_marks_of_one_text_are_read_with_lines_and_staleness_without_the_overlap_stop | REQ-core-286 | C-core-ovdata-14 | 落とした細部のテスト。いちばん近い行を指す。1つの文字列のマーカーを行と古さ付きで読む core の `GuideReader` の層のテストで、元データの "path" は確かめない。準備が古い形のマーカーなので、決定を指すマーカーの準備に直す |
| crates/kotowari-core/tests/inputs.rs | the_read_documents_expose_their_place_relative_paths_and_kinds | REQ-core-284 | - | 古い形のコードと一緒に消す。IR の文書の置き場からの相対パスと種類を公開する層のテストで、一覧の照合の相手が決定の文書に変わる |
| crates/kotowari-overview/src/form.rs | the_embedded_overview_schema_parses | TBL-core-038 | C-core-ovdata-5 | 落とした細部のテスト。いちばん近い行を指す（形のスキーマ）。スキーマの置き場は落とした細部 |
| crates/kotowari/tests/project.rs | overview_operations_fail_with_kinds_for_bad_data_and_a_missing_key | REQ-core-294, REQ-core-312, REQ-core-279 | C-core-ovdata-2 | この話題の分は、鍵が無いと build の操作が設定の誤りで失敗すること。REQ-core-294 は全体像のコマンドの話題、REQ-core-312 は別の話題 |
| tests/step19_overview_build.rs | req_core_280_build_stops_on_an_overlap_with_the_test_files | REQ-core-280, REQ-core-278 | C-core-ovdata-4 | build がテストとの重なりで止まる |
| tests/step19_overview_build.rs | req_core_280_build_judges_the_guides_and_tests_overlap_first | REQ-core-280, REQ-core-199 | C-core-ovdata-21 | 3つすべてに当たるとき build がガイドとテストの重なりで止まる。REQ-core-199 の分は C-core-guides-9（ガイドの対応表に書いてある） |
| tests/step19_overview_build.rs | ex_core_553_a_test_file_that_is_not_utf8_does_not_stop_the_build | EX-core-553, REQ-core-278 | C-core-ovdata-16 | そのまま |
| tests/step19_overview_build.rs | req_core_278_build_reads_the_ir_places_and_stops_like_check | REQ-core-278, REQ-core-018 | C-core-ovdata-24 | build が置き場を check と同じに読んで止まる。準備が ADR の置き場を消しているので、付け替えのときに決定の文書の置き場を消す準備に直す（今のコードは決定の文書の置き場が無くても check と同じに止まることを、使い捨ての場で確かめた） |
| tests/step19_overview_build.rs | ex_core_463_build_without_the_overview_key_is_a_config_error | EX-core-463, REQ-core-279, TBL-core-020 | C-core-ovdata-2 | TBL-core-020 は別の話題 |
| tests/step20_overview_serve.rs | req_core_279_serve_without_the_overview_key_is_a_config_error | REQ-core-279 | C-core-ovdata-2 | そのまま |
| tests/step20_overview_serve.rs | req_core_280_serve_stops_on_an_overlap_with_the_guides | REQ-core-280 | C-core-ovdata-3 | そのまま |
| tests/step18_overview.rs | ex_core_464_overview_data_in_a_named_hidden_directory_is_read | EX-core-464, REQ-core-278, REQ-core-019, REQ-core-288 | C-core-ovdata-1 | この話題の分は、glob に当たる元データを読むこと。REQ-core-019 は設定の話題、REQ-core-288 は申告の話題 |
| tests/step18_overview.rs | req_core_019_a_broad_glob_does_not_enter_hidden_directories_it_does_not_name | REQ-core-019, REQ-core-278 | - | この話題の分は無い。隠しディレクトリの決まりは設定の対応表が C-core-config-24 を付けるので、マーカーは無くならない |
| tests/step18_overview.rs | req_core_278_only_files_with_a_lowercase_md_extension_are_read | REQ-core-278 | C-core-ovdata-1 | glob に当たる元データを読み数える。拡張子が小文字の ".md" だけを読むことは落とした細部で、テストの中に残る |
| tests/step18_overview.rs | req_core_278_overview_data_that_is_not_utf8_stops | REQ-core-278, TBL-core-001 | C-core-ovdata-18 | UTF-8 でない元データで止まる。TBL-core-001 の分は別の話題 |
| tests/step18_overview.rs | req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops | REQ-core-018, REQ-core-278, TBL-core-001 | C-core-ovdata-19 | この話題の分は、"overview.files" の走査の先の無いシンボリックリンクで止まること。設定の対応表は同じテストを C-core-config-11 にも当てている |
| tests/step18_overview.rs | ex_core_465_overview_data_matched_by_guides_is_a_config_error | EX-core-465, REQ-core-280, TBL-core-020 | C-core-ovdata-3, C-core-ovdata-20 | 詳細の文言も確かめる |
| tests/step18_overview.rs | req_core_280_the_first_overlap_in_byte_order_is_reported_and_tests_overlap_too | REQ-core-280, TBL-core-020 | C-core-ovdata-4, C-core-ovdata-20 | テストとの重なりで status が止まり、バイト順で最初の "tests/a.md" が詳細に出る |
| tests/step18_overview.rs | req_core_280_the_guides_and_tests_overlap_is_judged_first_and_guides_win_when_all_three_match | REQ-core-280, REQ-core-199 | C-core-ovdata-21 | 3つすべてに当たるとき check がガイドとテストの重なりで止まる |
| tests/step18_overview.rs | ex_core_463_without_the_overview_key_check_reads_no_overview_data | EX-core-463, REQ-core-279, REQ-core-288, TBL-core-005 | C-core-ovdata-17 | 鍵が無いと形の崩れたファイルを指す指摘が無い。"files" が 0 は申告の話題の落とした細部 |
| tests/step18_overview.rs | req_core_326_the_overview_data_overlap_is_judged_before_the_toc_overlap | REQ-core-326, REQ-core-280 | - | この話題の分は、目次との重なりより先に判定する順（落とした細部）。目次の対応表が C-core-ovtoc-6 を付けるので、マーカーは無くならない |
| tests/step18_overview.rs | ex_core_511_an_unreadable_yaml_part_points_at_the_line_of_the_yaml_error | EX-core-511 | C-core-ovdata-10 | そのまま |
| tests/step21_translations.rs | translated_overview_files_cannot_also_be_test_files | REQ-core-280, REQ-core-337 | C-core-ovdata-4 | 対のある元データでもテストとの重なりで止まる。REQ-core-337 は対の話題 |
| tests/step21_translations.rs | req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap | REQ-core-280, REQ-core-199, REQ-core-338 | C-core-ovdata-21 | 先頭の言語の側の無い側が3つすべてに当たると、ガイドとテストの重なりで止まる。REQ-core-199 の分は C-core-guides-9 |
| tests/step21_translations.rs | req_core_280_a_side_without_its_first_side_matched_by_overview_and_guides_stops | REQ-core-280, REQ-core-338 | C-core-ovdata-3, C-core-ovdata-22 | 先頭の言語の側の無い側を、ガイドとの重なりに数えて check、status、build が止まる |

この話題の ID を指すテストは 47。どの行にも当たらず「-」にしたものを含む。「-」の4本のうち3本はほかの話題の対応表が ID を付け、1本は古い形のコードと一緒に消す。

## 落とした細部

### 安全に関わるもの

無い。書き直しの決まり（安全に関わる落とした細部は決定に戻す）で、ここにあった4件をすべて決定に戻した（下の「戻した細部」）。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-278: build と serve も、参照を解くために check と同じ設定と置き場から仕様の文書を読み、置き場が無いか読めなければ check と同じく停止する。相手を決定の文書の置き場に読み替えた | C-core-ovdata-24（決定「overview build と overview serve も、check と同じ置き場から決定の文書を読み、同じ条件で停止する」） | テスト req_core_278_build_reads_the_ir_places_and_stops_like_check（準備は ADR の置き場を消す古い形で、付け替えのときに直す）。今のコードが決定の文書の置き場で同じに止まることは、使い捨ての場で "decisions.documents" を無い置き場にして "kotowari overview build" と check を走らせ、どちらも "unreadable file: docs/nothere: No such file or directory (os error 2)" で止まることを確かめた |
| REQ-core-278: 元データのファイルが UTF-8 でないときの停止 | C-core-ovdata-18 | テスト req_core_278_overview_data_that_is_not_utf8_stops |
| REQ-core-278: "overview.files" の走査で先の無いシンボリックリンクに出会うときの停止 | C-core-ovdata-19 | テスト req_core_018_a_dangling_symbolic_link_in_the_overview_walk_stops |
| REQ-core-280: 停止の詳細の文言と、パスのバイト順で最初の1つ | C-core-ovdata-20 | テスト ex_core_465_overview_data_matched_by_guides_is_a_config_error、req_core_280_the_first_overlap_in_byte_order_is_reported_and_tests_overlap_too |
| REQ-core-280: ガイドとテストの重なりを先に判定し、3つすべてに当たるファイルではガイドとテストの重なりで止まる | C-core-ovdata-21 | テスト req_core_280_the_guides_and_tests_overlap_is_judged_first_and_guides_win_when_all_three_match、req_core_280_build_judges_the_guides_and_tests_overlap_first、req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap |
| REQ-core-280: ガイドとの重なりに、"guides.files" に当たる先頭の言語の側の無い側と、同じ対にあるためにガイドとして読む側を数える | C-core-ovdata-22（先頭の言語の側の無い側の分） | テスト req_core_280_a_side_without_its_first_side_matched_by_overview_and_guides_stops。同じ対にあるためにガイドとして読む側の分は、元データとの重なりのテストが無い（下の「テストの無い行」） |

### 戻せなかった細部

| 細部 | 戻すために要るテスト |
|---|---|
| REQ-core-278: 元データのファイルが読めないとき（権限で開けない）の停止 | 下の「テストの無い行」 |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-278: 拡張子が小文字の ".md" のファイルだけを読む | LLM（2026-10-02-whole-picture.md の A73） |
| REQ-core-278: list と query は元データを読まない | LLM（A73）。決めたことには、利用者の決定の check、status、build、serve が読むことだけを書いた |
| REQ-core-279: "overview" の鍵が無いときの設定の誤りの詳細（設定ファイルの相対パスと "overview is not configured"、のちにパスを付けない） | LLM（A72、A73） |
| REQ-core-281、TBL-core-038: 形のスキーマを kotowari-overview のパッケージの中の "crates/kotowari-overview/schemas/overview.yaml" に置き、コンパイル時に取り込む | LLM（2026-10-04-overview-on-public-api.md の A5） |
| TBL-core-038: frontmatter が無い、読めない、"ir" が無いか空か文字列の一覧でない、を形の違反にする。題名の "# " がちょうど1つ | LLM（A77）。知らない鍵を誤りにすること（A55、利用者）は残した |
| TBL-core-038: 題名の後で最初の "## " より前に部品と HTML のコメントだけを置ける。節の下に置けるもの（文、箇条書き、番号付きの一覧、表、コードブロック、HTML のコメント、"### " の見出し） | LLM（A71、A73） |
| TBL-core-038: 形の違反の detail を kotowari-markdown-schema の指摘の種類の名前にし、frontmatter の違反だけ "frontmatter" にする | detail の種類の名前は LLM（A71）、"frontmatter" は利用者（2026-10-04-overview-on-public-api.md の A11）。[指摘の種類](findings.md)の決定にあるので、この文書ではリンクだけにした |
| REQ-core-282: 合わなかった場所で、知らない鍵と欠けた鍵の名前を JSON Pointer に "/" で足す。値の全体を "(root)" と書く。同じ "line" と detail の誤りを1件にまとめる | LLM（A73、A86） |
| REQ-core-282、REQ-core-285: 部品の誤りの "line" をフェンスの開始の行にする | LLM（A74）。YAML の誤りの行にすること（2026-10-05-overview-page-reading.md の A10、利用者）は残した |
| REQ-core-283: lead の判定で HTML のコメントと空行を飛ばす。"line" を null、detail を文書名にする | LLM（A74、A77） |
| REQ-core-284: overview_ir_shared をパスのバイト順で2つ目以降の元データに出し、detail を重なった文書のパスにする | LLM（A74） |
| REQ-core-285: 参照を、スキーマに合う部品の中だけから、入れ子の深さを問わず集める。参照の1件ごとに1件の誤りを出す | LLM（A74、A77） |
| REQ-core-286: 節のマーカーを "## " の見出しの次の行に、それだけの行として置く | LLM（A73、A77）。利用者の決定は「節の見出しの隣に HTML のコメントで書く」（A54） |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-278 | 新しい形で読み替えて残る。build と serve が参照を解くために読む置き場は、IR、判断の記録、ADR から、決定の文書の置き場になる（C-core-ovdata-24） |
| REQ-core-281、TBL-core-038 | 新しい形で読み替えて残る。"ir" の一覧が指す相手は、languages-and-guides の「全体像の元データは、決定の文書を指す」で決定の文書になった。鍵の名前 "ir" は改名する（C-core-ovdata-23、提案） |
| REQ-core-284、EX-core-468 | 新しい形で読み替えて残る。無い文書を指す1件は C-core-lang-7 に置き換わり、2つの元データに属する文書は C-core-ovdata-12（提案）にした |
| REQ-core-285、EX-core-469 | 一部が消える。参照の形（IR の ID の項目かシナリオ、出典の規則で判断の記録の決定）は消え、C-core-lang-8 に置き換わった。"refs" と "ref" の欄は残る |
| REQ-core-286、EX-core-470 | 新しい形で読み替えて残る。マーカーが照らす相手が IR の項目から決定になる。古さの見方は C-core-lang-9 |

## テストの無い行

実装済みの行は、どれも上の表のテストが1本以上指す。C-core-ovdata-1、C-core-ovdata-13、C-core-ovdata-14、C-core-ovdata-15、C-core-ovdata-24 のテストは、準備が古い形（IR の話題の文書、要求の ID、"REQ-core-001:..." のマーカー、ADR の置き場）なので、古い形を消すときに、決定の文書を準備に使う形へ直す必要がある。

戻せなかった細部と、決定の文に書いたがテストが一部にしか無い所:

| 細部 | 要るテスト | 行の文の案 |
|---|---|---|
| 元データのファイルが読めないときの停止（REQ-core-278） | "overview.files" に当たる元データの権限を外して check を走らせ、終了コード 2 と "unreadable file: " とそのパスを確かめる | 読めない元データがあると、check は終了コード 2 で止まり、標準エラーの1行目が "unreadable file: " とそのパスで始まる |
| glob に当たらないが同じ対にあるためにガイドとして読む側が、元データとの重なりに数えられること（REQ-core-280。決めたことに書いた） | 言語が2つ以上で、"guides.files" が先頭の言語の側だけに当たり、"overview.files" がほかの言語の側に当たる場で、check が ": matched by both overview.files and guides.files" で止まることを確かめる | 同じ対にあるためにガイドとして読むほかの言語の側が "overview.files" に当たると、check は設定の誤りで停止する |

## 誰が決めたか見分けられなかったもの

- 無い。

## 理由や見送った案が記録に無かった決定

- 「overview の鍵が無ければ、overview build と overview serve は設定の誤りで停止する」: 見送った案は、有効にするかどうかの決定（[設定](config.md)にある）の分しか無いので、節を置かなかった。止める理由は、有効にする決定と同じ「黙って既定で有効にしない」だけで、問題の節にそう書いた。
- 「全体像の元データがガイドかテストの置き場と重なると、設定の誤りで停止する」: 理由はガイドとテストの重なりと同じ扱いにする、だけ。戻した細部のうち、対の側の数え方の理由は 2026-10-23-guide-overlap-sides.md の A1 の why（黙って二重に数えるのを許さない）を書いた。判定の順と詳細の文言の理由は記録に無く（A73、A75 の why は「仕様に書いた値や文言が記録に無かった」）、「記録に残っていない」と書いた。見送った案は無い。
- 「全体像の元データは、リポジトリにコミットして保守し、…」: 戻した停止の条件（A76）の理由は記録に無く、「記録に残っていない」と書いた。
- 「全体像の元データは、frontmatter と Markdown の本文で書き、…」: 見送った案が記録に無い。
- 「題名の直後の最初の部品は、lead にする」: 見送った案が記録に無い。
- 「1つの決定の文書は、多くても1つの全体像の元データの一覧に入れる」: 見送った案が記録に無い。
- 「全体像の元データの一覧の鍵 ir と、…は改名する」: 理由が記録に無い（利用者の答えは「改名する」だけ）。問題の節には、IR が無くなるという事実だけを書いた。見送った案も無い。
- 「部品の中の参照は、refs と ref の欄に書く」: 見送った案が記録に無い。
- 「全体像の元データのマーカーは、ガイドのマーカーと同じ規則で読む」: 見送った案が記録に無い。
- 「overview build と overview serve は、テストのファイルのパスだけを集め、中身を読まない」: 見送った案が記録に無い。
- 「overview build と overview serve も、check と同じ置き場から決定の文書を読み、同じ条件で停止する」: 理由が記録に無い（A73、A76 は LLM の細部で、why は「仕様に書いた値や文言が記録に無かった」）。見送った案も無い。

見送った案のうち、view の決定の代わりのもの（部品の中身の検査を view に持たせる、view の型への読み込みで検査する、JSON Type Definition、スキーマを ".kotowari/schemas/overview/" に置く）は、view の parts の「部品の形の正は、view が埋め込んで公開する部品のスキーマにする」にあるので書かなかった。

## 分けた所、まとめた所

- REQ-core-278 を3つの決定に分けた: 「全体像の元データは、リポジトリにコミットして保守し、overview.files の glob で選ぶ」と「overview build と overview serve は、テストのファイルのパスだけを集め、中身を読まない」と「overview build と overview serve も、check と同じ置き場から決定の文書を読み、同じ条件で停止する」。2つ目は別の記録の別の決定で、理由を持つため。3つ目は、戻した安全の細部が、元データの読み方ではなく build と serve が読む仕様の置き場の振る舞いで、独立しているため。
- REQ-core-279 は、check の側（有効にしない）が設定の話題にあるので、build と serve の停止を中心にした決定にし、check の側は C-17 として同じ決定に置いた。
- REQ-core-281 と TBL-core-038 を1つの決定にまとめた。鍵の改名（提案）は、実装済みの決定と状態が違うので、独立の決定にした。
- REQ-core-282 の YAML の誤りの行（別の記録の決定）は、同じ決定の中の段落にした。決定を覆すものではなく、行の決まりの書き直しだったため。
- 決定「全体像の元データは、リポジトリにコミットして保守し、…」に、REQ-core-278 に無い利用者の決定（元データを保守してコミットする、置き場を ".kotowari/overview/" にする）を入れた。出典の行が指していた記録の決定で、ほかの話題に書かれていないため。戻した停止の条件（UTF-8 でないとき、先の無いシンボリックリンク）も、元データの読み方の決定に入れた。
- REQ-core-280 の戻した細部（文言とバイト順、判定の順、対の側の数え方）は、重なりの決定の中の段落にした。

## あとでリンクにする所

- 無い。languages-and-guides と view の parts は見出しへのリンクにし、このフォルダの下書きどうしは `<file>.ja.md#<anchor>` のリンクにした。

## 迷った判断と、どう決めたか

- 全体像の名前の問いへの利用者の答え（書き直しの依頼で受けたもの）: 鍵 "ir" と overview_ir_missing、overview_ir_shared は改名する。新しい名前は実装する側が決める。これを独立の提案の決定にし、実装済みの決定の中では今の名前のまま書いて改名の決定へリンクした。
- 参照の状態の答え（「後回し」のマーカーは「提案」の決定への参照に付け、「置き換え済み」は出さない）: 参照の状態を持つのは参照の表で、その決定と問いは[全体像を作るコマンドと見せるコマンド](overview-commands.md)の「参照の表には、参照ごとに参照先の本文を入れる」にあるので、そちらに書いた。この文書の「部品の中の参照は、…」は参照の欄の読み方だけを持つ。
- 部品の指摘の種類の名前（overview_part_unknown、overview_part_invalid、overview_lead_missing、overview_ref_unresolved）と「detail に検査のライブラリの文を入れない」は、記録では LLM が決めた細部で、decided_by に「利用者が第21ラウンドで確認した」とある（2026-10-02-whole-picture.md の A69）。書き直しの決まりは「見直しのラウンドで利用者に示した」を agent の細部に数えるが、指摘の種類の名前は利用者から見える名前なので、どちらに数えても残る。detail の決まりは、view の rendering の対応表と同じく利用者が確かめたものとして残した。
- YAML の誤りの detail の "(yaml)" は、もとは LLM の細部（A73）だが、利用者の決定（2026-10-05-overview-page-reading.md の A10）の文に「今までどおり種類の名前と "(yaml)"」とあるので残した。
- 停止の詳細の文言（": matched by both overview.files and guides.files"）は、書き直しの決まり（安全に関わる落とした細部は決定に戻す）で安全に関わる細部として戻した（C-20）。
- 戻した C-24 のテストは、準備が古い形（ADR の置き場を消す）だが、今のコードは決定の文書の置き場でも check と同じに止まる（使い捨ての場で確かめた）ので、実装済みにした。
- C-22 の行は、テストのある先頭の言語の側の無い側の分だけにした。同じ対にあるためにガイドとして読む側を数えることは、決めたことに書き、元データとの重なりのテストが無いので「テストの無い行」に挙げた。コードはガイドとテストの重なりと同じ集合で判定する（2026-10-23-guide-overlap-sides.md の A1 の根拠）。
- マーカーの読み方（REQ-core-286）は、今のコードが決定を指すマーカーをもう読むので実装済みにした。ただし、それを指すテストの準備は古い形のマーカーなので、古い形を消すときに直す必要がある（テストごとの表）。
- 「1つの決定の文書は、…」の確認することの行は、2つ目以降の元データに出すこと（LLM の細部）を書かず、誤りが出ることだけにした。

## 実装する側が決める細部

- 鍵 "ir" と指摘 overview_ir_missing、overview_ir_shared の新しい名前。案: 鍵を "documents"、指摘を overview_document_missing、overview_document_shared。
- "ir" の一覧の1件が「決定の文書」として数えられる範囲（決定の文書の置き場の下の話題の文書だけか、置き場の CONTEXT.md や凍結した木を除くか、パスの正規化）。今のコードは IR の話題ごとの文書だけを数える。
- 参照の文字列が、決定への行き先の形でも確認することの ID の形でもないときの扱い。今のコードは、どちらの形（IR の ID、"#" を含む出典）でもない参照を overview_ref_unresolved にする。

## 利用者に聞くこと

- 無い。鍵と指摘の名前の問いは、書き直しの依頼への利用者の答え（改名する）で消した。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えることになりそうか |
|---|---|
| 1つの決定の文書は、多くても1つの全体像の元データの一覧に入れる | `crates/kotowari-overview/src/lib.rs` の `Inspection::new` が `topics` を IR の話題ごとの文書から作っているのを、決定の文書の一覧から作るように変える。overview_ir_missing の判定（C-core-lang-7）も同じ所 |
| 全体像の元データの一覧の鍵 ir と、指摘 overview_ir_missing、overview_ir_shared は改名する | `crates/kotowari-overview/schemas/overview.yaml` の frontmatter の鍵、`crates/kotowari-core/src/finding.rs` の指摘の種類、`agent/skills/kotowari/references/overview.md` の書き方 |

## 数

- 元: 要求 10、場面 12、決定表 1、性質 0
- 読んだ記録のファイル: 8（2026-10-02-whole-picture.md、2026-10-03-public-crate-api.md、2026-10-04-overview-on-public-api.md、2026-10-05-overview-page-reading.md、2026-10-06-changes-rethink.md、2026-10-23-overview-test-contents.md、2026-10-23-overview-test-contents-consistency.md、2026-10-23-guide-overlap-sides.md）
- 出来上がり: 決定 12（実装済み 10、提案 2）、確認することの行 24（うち「（読んで確認）」0）
- 戻した安全の細部: 4件（行 C-core-ovdata-18〜22、24 の6行）。戻せなかった細部: 1件（読めない元データの停止）
- テスト: この話題の ID を指すテスト 47
