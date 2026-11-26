# plan の対応表

元: `docs/ir/core/plan.md`（日本語は `plan.ja.md`）。書き直した文書: `plan.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-190 | C-core-plan-3, C-core-plan-4 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | 位置引数の数を C-3、`--config` を C-4 にした。パスをカレントディレクトリからの相対で読むことは決めたことに書き、C-core-plan-21 のテストが確かめる。ガイド `docs/guides/commands/plan*.md` の REQ-core-190 のマーカーはこの見出しへ |
| REQ-core-196 | C-core-plan-5 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | 設定ファイルを読まないことだけを C-5 にした（A15）。IR と判断の記録を読まないことは形に依存する要求、テストのファイルを読まないことは落とした細部（A32、主セッション）。ガイドのマーカーはこの見出しへ |
| REQ-core-197 | C-core-plan-6 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | 読めないファイルと UTF-8 でないファイルを1行にまとめた |
| REQ-core-191 | C-core-plan-1, C-core-plan-2 | 計画書の形は、本体に同梱したスキーマで kotowari plan が検査する | 同梱のスキーマで読み frontmatter の宣言を使わないことを C-1、壊れた frontmatter を飛ばすことを C-2 にした。ガイドのマーカーはこの見出しへ |
| REQ-core-192 | C-core-plan-7, C-core-plan-8, C-core-plan-9, C-core-plan-10, C-core-plan-11, C-core-plan-12, C-core-plan-13, C-core-plan-14, C-core-plan-15, C-core-plan-16, C-core-plan-17, C-core-plan-18 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | 3つの決定に分けた（下の「分けた所、まとめた所」）。題名がちょうど1つで列挙のほかの節を置けないこと（A31）と、最初のステップより前に行を置けないこと（A22）は落とした細部。ガイドのマーカーは、表とコードブロックと番号付きの一覧を扱う所（plan.md の 78〜79 行目、333〜334 行目、349〜350 行目）は「ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける」へ、並び順を扱う所（105〜106 行目）は「節の並び順、ステップの番号、ステップの見出しの名前は検査しない」へ、ほかはこの見出しへ |
| REQ-core-193 | C-core-plan-19, C-core-plan-20, C-core-plan-21 | 形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる | 種類・重さ・detail を C-19、line を C-20、path を C-21 にした。「TBL-core-030 で写さない」は形に依存する要求で、「kotowari のほかの種類へは写さない」と読み替えた。行が無いときに null にすることは落とした細部（A33）で、行の数え方の原則は finding-order の C-core-forder-4 に当たる。ガイド `docs/guides/findings*.md` と `docs/guides/commands/plan*.md` の REQ-core-193 のマーカーはこの見出しへ |
| REQ-core-194 | C-core-plan-22 | kotowari plan の出力は check と同じ指摘の鍵を使い、JSON は findings と counts だけにする | "counts" の中身を check と同じにすることは落とした細部（A33） |
| REQ-core-207 | C-core-plan-23 | kotowari plan の出力は check と同じ指摘の鍵を使い、JSON は findings と counts だけにする | そのまま |
| EX-core-332 | C-core-plan-7 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | そのまま |
| EX-core-333 | C-core-plan-11, C-core-plan-19 | 形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる | 欄の欠けが誤りになることを C-11、指摘の形を C-19 にした |
| EX-core-334 | C-core-plan-12 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | EX-core-335 と1行にまとめた |
| EX-core-335 | C-core-plan-12 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | EX-core-334 と1行にまとめた |
| EX-core-336 | C-core-plan-9 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | そのまま |
| EX-core-337 | C-core-plan-8 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | 必須の節の欠けと重なり、Test command の0個と2個を1行にまとめた |
| EX-core-338 | C-core-plan-8 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | 同上 |
| EX-core-339 | C-core-plan-15 | ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける | 表、コードブロック、子の一覧を1行にまとめた |
| EX-core-340 | C-core-plan-14 | ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける | そのまま |
| EX-core-341 | C-core-plan-17 | 節の並び順、ステップの番号、ステップの見出しの名前は検査しない | そのまま |
| EX-core-342 | C-core-plan-3 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | EX-core-357 と1行にまとめた |
| EX-core-343 | C-core-plan-4 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | そのまま |
| EX-core-344 | C-core-plan-6 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | EX-core-358 と1行にまとめた |
| EX-core-345 | C-core-plan-1 | 計画書の形は、本体に同梱したスキーマで kotowari plan が検査する | そのまま |
| EX-core-346 | C-core-plan-22 | kotowari plan の出力は check と同じ指摘の鍵を使い、JSON は findings と counts だけにする | "counts" の "invalid_plan" が指摘の数に等しいことは落とした細部（A33）で、テストの中に残る |
| EX-core-347 | C-core-plan-8 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | 同上（EX-core-337） |
| EX-core-348 | - | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | 知らない節を誤りにすることは A31（主セッションが導いた細部）だけが支えるので落とした |
| EX-core-349 | C-core-plan-11 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | 欄の欠け・重なり・順の違いを1行にまとめた |
| EX-core-350 | C-core-plan-11 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | 同上 |
| EX-core-351 | - | ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける | 最初のステップより前の行を誤りにすることは A22（主セッションが導いた細部）だけが支えるので落とした |
| EX-core-352 | C-core-plan-10 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | そのまま |
| EX-core-353 | C-core-plan-16 | ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける | 番号付きの一覧、深い見出し、題名と最初の節の間の行を1行にまとめた |
| EX-core-354 | C-core-plan-15 | ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける | 同上（EX-core-339） |
| EX-core-355 | C-core-plan-16 | ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける | 同上（EX-core-353） |
| EX-core-356 | C-core-plan-13 | 計画書は、決まった計画全体の節と、8つの欄を持つステップで書く | "*" と "+" を1行にまとめた |
| EX-core-357 | C-core-plan-3 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | 同上（EX-core-342） |
| EX-core-358 | C-core-plan-6 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | 同上（EX-core-344） |
| EX-core-359 | C-core-plan-5 | kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える | そのまま |
| EX-core-360 | C-core-plan-21 | 形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる | そのまま |
| EX-core-361 | C-core-plan-2 | 計画書の形は、本体に同梱したスキーマで kotowari plan が検査する | そのまま |
| EX-core-381 | C-core-plan-20, C-core-plan-23 | 形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる | 見出しの行を指すことを C-20、text の形を C-23 にした |
| EX-core-382 | C-core-plan-23 | kotowari plan の出力は check と同じ指摘の鍵を使い、JSON は findings と counts だけにする | そのまま |

元の文書に TBL と PROP は無い。

## テストごとの表

ファイルはどれも `tests/step14_plan.rs`（最後の1本だけ `crates/kotowari-core/src/schema.rs`）。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step14_plan.rs | ex_core_357_plan_without_a_plan_file_stops | REQ-core-190, EX-core-357 | C-core-plan-3 | そのまま |
| tests/step14_plan.rs | ex_core_342_two_plan_files_stop | REQ-core-190, EX-core-342 | C-core-plan-3 | そのまま |
| tests/step14_plan.rs | ex_core_343_config_on_plan_stops | REQ-core-190, EX-core-343 | C-core-plan-4 | そのまま |
| tests/step14_plan.rs | req_core_190_config_pointing_to_a_directory_on_plan_stops | REQ-core-190 | C-core-plan-4 | `--config` がディレクトリを指す場合 |
| tests/step14_plan.rs | ex_core_333_a_step_missing_a_field_is_an_invalid_plan_error | REQ-core-192, REQ-core-193, EX-core-333 | C-core-plan-11, C-core-plan-19 | 欄の欠けで終了コード1になることと、指摘の種類・重さ・detail の形を確かめている |
| tests/step14_plan.rs | req_core_192_a_valid_plan_has_no_findings | REQ-core-192, REQ-core-193 | C-core-plan-7 | そのまま |
| tests/step14_plan.rs | req_core_193_a_finding_without_a_line_has_a_null_line | REQ-core-193 | C-core-forder-4 | 題名の無い計画書の invalid_plan の line が null であることを確かめている。plan の側では落とした細部（A33）で、文書全体にかかる指摘を null にする原則（finding-order の話題）に当たる |
| tests/step14_plan.rs | req_core_193_the_line_is_the_line_the_schema_side_reported | REQ-core-193 | C-core-plan-20, C-core-forder-3 | 知らない節の見出しの行（1始まりの行の番号）を指すことを確かめている。知らない節が誤りになること自体は落とした細部だが、この行はスキーマの側の行をそのまま使うことを確かめる入力として使っている。行が1始まりであることは finding-order の C-core-forder-3 に当たる |
| tests/step14_plan.rs | req_core_025_plan_text_output_has_one_line_per_finding | REQ-core-025, REQ-core-193 | C-core-plan-23 | REQ-core-025（text の形。CLI と出力の話題）の分は、その話題の書き直しで付け替える |
| tests/step14_plan.rs | ex_core_381_a_step_missing_a_field_points_at_its_heading | REQ-core-193, REQ-core-207, EX-core-381 | C-core-plan-20, C-core-plan-23 | 見出しの行と、text が指摘の行だけであることを確かめている |
| tests/step14_plan.rs | ex_core_382_the_text_of_a_plan_without_findings_is_empty | REQ-core-207, EX-core-382 | C-core-plan-23 | そのまま |
| tests/step14_plan.rs | req_core_025_plan_text_output_shows_a_missing_line_as_a_dash | REQ-core-025, REQ-core-193 | - | 確かめているのは、行の無い指摘を text で "-" と出すこと（REQ-core-025、CLI と出力の話題）。この話題の分（行が無いときの null）は落とした細部。REQ-core-025 の分を出力の話題の対応表で付け替えるので、マーカーは空にならない |
| tests/step14_plan.rs | ex_core_344_a_missing_plan_stops_as_an_unreadable_file | REQ-core-197, EX-core-344 | C-core-plan-6 | そのまま |
| tests/step14_plan.rs | req_core_197_a_directory_given_as_the_plan_stops_as_an_unreadable_file | REQ-core-197 | C-core-plan-6 | そのまま |
| tests/step14_plan.rs | ex_core_358_a_non_utf8_plan_stops | REQ-core-197, EX-core-358 | C-core-plan-6 | そのまま |
| tests/step14_plan.rs | ex_core_345_a_schema_named_in_the_frontmatter_is_not_used | REQ-core-191, EX-core-345 | C-core-plan-1 | そのまま |
| tests/step14_plan.rs | ex_core_361_a_broken_frontmatter_is_skipped_unread | REQ-core-191, EX-core-361 | C-core-plan-2 | そのまま |
| tests/step14_plan.rs | ex_core_346_plan_json_has_only_findings_and_counts | REQ-core-194, TBL-core-005, EX-core-346 | C-core-plan-22 | 最上位の鍵を確かめている。"counts" の中身は落とした細部で、テストの中に残る。TBL-core-005（CLI と出力の話題）の分は、その話題の書き直しで付け替える |
| tests/step14_plan.rs | req_core_194_plan_finding_keys_are_those_of_check | REQ-core-194, TBL-core-006 | C-core-plan-22 | TBL-core-006（CLI と出力の話題）の分は、その話題の書き直しで付け替える |
| tests/step14_plan.rs | ex_core_359_a_broken_config_does_not_stop_plan | REQ-core-196, EX-core-359 | C-core-plan-5 | そのまま |
| tests/step14_plan.rs | ex_core_360_a_plan_outside_the_base_has_a_parent_path | REQ-core-193, TBL-core-006, EX-core-360 | C-core-plan-21 | TBL-core-006 の分は CLI と出力の話題で付け替える |
| tests/step14_plan.rs | req_core_193_the_path_is_normalised_from_the_base | REQ-core-193, TBL-core-006 | C-core-plan-21 | カレントディレクトリが基準の下で、"./" と ".." を含むパスを渡す。同上 |
| tests/step14_plan.rs | ex_core_332_a_well_formed_plan_has_no_findings | REQ-core-192, REQ-core-193, EX-core-332 | C-core-plan-7 | そのまま |
| tests/step14_plan.rs | ex_core_334_shown_by_not_starting_with_one_of_the_four_words_is_an_error | REQ-core-192, EX-core-334 | C-core-plan-12 | そのまま |
| tests/step14_plan.rs | ex_core_335_shown_by_word_followed_by_more_letters_is_an_error | REQ-core-192, EX-core-335 | C-core-plan-12 | そのまま |
| tests/step14_plan.rs | req_core_192_each_of_the_four_words_may_start_shown_by_alone_or_before_a_space | REQ-core-192 | C-core-plan-12 | そのまま |
| tests/step14_plan.rs | ex_core_336_a_plan_without_a_step_is_an_error | REQ-core-192, EX-core-336 | C-core-plan-9 | そのまま |
| tests/step14_plan.rs | ex_core_337_the_test_command_section_may_be_absent | REQ-core-192, EX-core-337 | C-core-plan-8 | そのまま |
| tests/step14_plan.rs | req_core_192_two_test_command_sections_are_an_error | REQ-core-192 | C-core-plan-8 | そのまま |
| tests/step14_plan.rs | ex_core_338_two_goal_sections_are_an_error | REQ-core-192, EX-core-338 | C-core-plan-8 | そのまま |
| tests/step14_plan.rs | ex_core_339_a_plan_wide_section_may_hold_a_table | REQ-core-192, EX-core-339 | C-core-plan-15 | そのまま |
| tests/step14_plan.rs | req_core_192_a_plan_wide_section_may_hold_a_code_block | REQ-core-192 | C-core-plan-15 | そのまま |
| tests/step14_plan.rs | ex_core_340_prose_right_after_a_field_line_is_an_error | REQ-core-192, EX-core-340 | C-core-plan-14 | そのまま |
| tests/step14_plan.rs | req_core_192_an_indented_line_after_a_blank_line_under_a_step_is_an_error | REQ-core-192 | C-core-plan-14 | そのまま |
| tests/step14_plan.rs | ex_core_341_section_order_and_gaps_in_step_numbers_are_not_checked | REQ-core-192, EX-core-341 | C-core-plan-17 | そのまま |
| tests/step14_plan.rs | req_core_192_two_steps_with_the_same_number_are_not_checked | REQ-core-192 | C-core-plan-18 | そのまま |
| tests/step14_plan.rs | req_core_192_the_name_after_the_colon_of_a_step_heading_is_not_checked | REQ-core-192 | C-core-plan-18 | そのまま |
| tests/step14_plan.rs | req_core_192_a_step_heading_without_a_colon_is_an_error | REQ-core-192 | C-core-plan-10 | そのまま |
| tests/step14_plan.rs | ex_core_347_a_missing_required_section_is_an_error | REQ-core-192, EX-core-347 | C-core-plan-8 | そのまま |
| tests/step14_plan.rs | req_core_192_every_required_section_is_required | REQ-core-192 | C-core-plan-8 | 10個の必須の節のどれが欠けても誤りになることを確かめている。指摘の line が null で detail に節の名前が入ることは、テストの中に残る細部 |
| tests/step14_plan.rs | req_core_192_two_titles_are_an_error | REQ-core-192 | C-core-plan-8 | 落とした細部のテスト。いちばん近い行を指す（題名がちょうど1つであることは落とした細部（A31）。節の数を縛る C-8 がいちばん近い） |
| tests/step14_plan.rs | ex_core_348_an_unknown_section_is_an_error | REQ-core-192, EX-core-348 | C-core-plan-8 | 落とした細部のテスト。いちばん近い行を指す（知らない節を誤りにすることは落とした細部（A31）。決まった節の列挙と数を縛る C-8 がいちばん近い） |
| tests/step14_plan.rs | ex_core_349_fields_out_of_order_are_an_error | REQ-core-192, EX-core-349 | C-core-plan-11 | そのまま |
| tests/step14_plan.rs | ex_core_350_a_field_twice_is_an_error | REQ-core-192, EX-core-350 | C-core-plan-11 | そのまま |
| tests/step14_plan.rs | req_core_192_every_step_field_is_required | REQ-core-192 | C-core-plan-11 | そのまま |
| tests/step14_plan.rs | req_core_192_an_unknown_field_under_a_step_is_an_error | REQ-core-192 | C-core-plan-14 | ステップの下に8つの欄のほかの行を置けないこと（A19）に当たる |
| tests/step14_plan.rs | ex_core_351_a_line_before_the_first_step_is_an_error | REQ-core-192, EX-core-351 | C-core-plan-14 | 落とした細部のテスト。いちばん近い行を指す（最初のステップより前の行を誤りにすることは落とした細部（A22）。ステップの下に欄の行のほかを置くと誤りになる C-14 がいちばん近い） |
| tests/step14_plan.rs | req_core_192_a_list_before_the_first_step_is_an_error | REQ-core-192 | C-core-plan-14 | 落とした細部のテスト。いちばん近い行を指す（同上） |
| tests/step14_plan.rs | ex_core_352_a_step_heading_not_of_s_and_a_number_is_an_error | REQ-core-192, EX-core-352 | C-core-plan-10 | そのまま |
| tests/step14_plan.rs | ex_core_353_a_numbered_list_in_a_plan_wide_section_is_an_error | REQ-core-192, EX-core-353 | C-core-plan-16 | そのまま |
| tests/step14_plan.rs | req_core_192_a_deeper_heading_in_a_plan_wide_section_is_an_error | REQ-core-192 | C-core-plan-16 | そのまま |
| tests/step14_plan.rs | ex_core_354_a_plan_wide_section_may_hold_a_child_list | REQ-core-192, EX-core-354 | C-core-plan-15 | そのまま |
| tests/step14_plan.rs | ex_core_355_a_line_between_the_title_and_the_first_section_is_an_error | REQ-core-192, EX-core-355 | C-core-plan-16 | そのまま |
| tests/step14_plan.rs | req_core_192_a_line_before_the_title_is_not_checked | REQ-core-192 | C-core-plan-16 | 落とした細部のテスト。いちばん近い行を指す（題名より前の行を検査しないこと。誰が決めたかを記録から見分けられなかった（下の一覧）。題名の前後の行の扱いを決める C-16 がいちばん近い） |
| tests/step14_plan.rs | req_core_192_a_section_before_the_title_does_not_count_as_a_second_one | REQ-core-192 | C-core-plan-8 | 落とした細部のテスト。いちばん近い行を指す（題名より前の節を数えないこと。見分けられなかった細部。節の数を縛る C-8 がいちばん近い） |
| tests/step14_plan.rs | req_core_192_a_section_before_the_title_does_not_fill_a_missing_one | REQ-core-192 | C-core-plan-8 | 落とした細部のテスト。いちばん近い行を指す（同上） |
| tests/step14_plan.rs | req_core_192_a_line_before_the_title_does_not_hide_a_line_after_the_title | REQ-core-192 | C-core-plan-16 | 題名と最初の節の間の地の文が誤りになり、その行（5行目）を指すことを確かめている。題名より前に行があることは、見分けられなかった細部の入力 |
| tests/step14_plan.rs | ex_core_356_field_lines_may_use_an_asterisk | REQ-core-192, EX-core-356 | C-core-plan-13 | そのまま |
| tests/step14_plan.rs | req_core_192_field_lines_may_use_a_plus | REQ-core-192 | C-core-plan-13 | そのまま |
| crates/kotowari-core/src/schema.rs | req_core_191_the_embedded_plan_schema_parses | REQ-core-191 | C-core-plan-1 | 本体に取り込んだ計画書のスキーマが読めることを確かめている。C-1 の「同梱のスキーマで検査する」の前提 |

`tests/step14_plan.rs` の `req_core_004_tool_on_plan_stops` はこの話題の ID を指していない（REQ-core-004 だけ）ので載せていない。

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-192: 題名がちょうど1つで、列挙した節のほかの "## " の節を置けない | 主セッション（2026-09-24-plan-schema.md の A31。A7 と書式の案から導いた。承認のときに利用者へ示す） |
| REQ-core-192: "## Steps" の最初のステップより前に、空でない行もほかの見出しも置けない | 主セッション（A22。A7 と A19 から導いた。承認のときに利用者へ示す） |
| REQ-core-193: スキーマの側の指摘が行を持たなければ line を null にする | 主セッション（A33）。文書全体にかかる指摘を null にする原則は finding-order の決定に残る |
| REQ-core-194: "counts" の中身を check と同じく種類ごとの数にする | 主セッション（A33） |
| REQ-core-196: テストのファイルを読まない | 主セッション（A32。A9、A10、A15 から導いた） |
| REQ-core-192: スキーマの読み方の名前（"reading: line"） | 利用者（A23）だが、スキーマの書き方の内部の名前なので、「1行ずつ読む」という振る舞いだけを書いた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-196 | IR と判断の記録を読まない部分は消える。その2つの文書が新しい版で無くなる |
| REQ-core-193 | 「スキーマの側の指摘を TBL-core-030 で写さない」は、TBL-core-030（IR の文書のための対応表）が無くなるので、「kotowari のほかの種類へは写さない」と読み替えて残る（A11） |
| REQ-core-192 の元（A3） | 「要求の ID が実在するかは kotowari query で確かめる」は、「計画書が指す仕様の ID」と読み替えて残る。新しい形では確認することの ID になる |

テストの無い行: 無い。

## 誰が決めたか見分けられなかったもの

- REQ-core-192 のテスト（`req_core_192_a_line_before_the_title_is_not_checked` ほか2本）が確かめている「題名より前の行は検査せず、題名より前の節は数えない」。元の要求の文にも、plan-schema の記録にも無い。A27 の why が IR の除外の「題名より前の空でない行」に触れているだけである。決めたことには書かず、テストに任せた。

## 理由や見送った案が記録に無かった決定

- 「kotowari plan は設定を読まず、引数と読めないファイルの扱いを mutants に揃える」: 見送った案が記録に無い。節を置かなかった。
- 「ステップの下には欄の行だけを置き、計画全体の節には文・一覧・表・コードブロックを置ける」: 見送った案が記録に無い。
- 「節の並び順、ステップの番号、ステップの見出しの名前は検査しない」: 見送った案が記録に無い。
- 「形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる」: 見送った案が記録に無い。
- 「kotowari plan の出力は check と同じ指摘の鍵を使い、JSON は findings と counts だけにする」: 見送った案が記録に無い。text の形を check と同じにする理由は、guide-gaps の A4 の why の「check の text と同じ形になる」だけである。

## 分けた所、まとめた所

- REQ-core-192 を3つの決定に分けた: 節とステップの欄の形（A7、A20、A28）、ステップの下と計画全体の節に置けるもの（A19、A23、A25）、検査しないもの（A7 の並び順、A21、A24、A29）。見送った案と理由がそれぞれ違うため。
- REQ-core-190、REQ-core-196、REQ-core-197 を1つの決定にまとめた。どれも A15 と A16（mutants に揃える）が支える。
- REQ-core-191 と、記録の A9（同梱）・A10（別のコマンド）を1つの決定にまとめた。
- REQ-core-194 と REQ-core-207 を出力の1つの決定にまとめた。
- 値違いの場面を1行にまとめた: EX-core-342 と EX-core-357（C-3）、EX-core-344 と EX-core-358（C-6）、EX-core-334 と EX-core-335（C-12）、EX-core-337・EX-core-338・EX-core-347（C-8）、EX-core-339 と EX-core-354（C-15）、EX-core-353 と EX-core-355（C-16）。

## あとでリンクにする所

確定の段で、次を見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- 冒頭の段落の「停止の理由の文言、終了コード、text の形、JSON の指摘の鍵」: cli-environment の「停止の理由は、英語の固定の文言で示す」、cli の「終了コードは、誤りが無ければ0、誤りがあれば1、停止したら2にする」、output の「--format text は、1つの指摘を1行で出す」と「指摘は kind、severity、path、line、detail の5つの鍵で出す」
- 「計画書の形は…」の問題の、ファイルを1つ受けて読むコマンドの前例: mutants の「変異テストは kotowari の外で走らせ、kotowari mutants はその結果のファイルを読んで指摘にする」
- 「計画書は、決まった…」の問題の `kotowari query`: query の「位置引数がちょうど1つの ID でないか、その ID が無ければ、引数の誤りで停止する」

残り: 「kotowari plan は設定を読まず…」の受けるオプション（REQ-core-002）と停止の理由（"argument error"、"unreadable file"、"non-UTF-8 file"）、「形の指摘は…」の path の書き方（TBL-core-006、設定ファイルの停止の詳細）、指摘の種類の話題（findings）の invalid_plan の種類と重さ、「計画書の形は…」の見送った案の status の完了の判定。どれもこの文書の決めたことの中で値を書いていて、別の決定を指す文が無いか、見送った案の中なので、リンクにしなかった

## 迷った判断と、どう決めたか

- 記録の decided_by が「主セッション（…から導いた。承認のときに利用者へ示す）」の決定（A22、A31、A32、A33）は、描画の話題の書き直しで LLM の細部として落とした前例に合わせて落とした。A31（知らない節と2つ目の題名を誤りにする）は、利用者が決めた A7 の節の列挙から閉じた形を導いたもので、残す案もあったが、決まりどおりにした。そのため EX-core-348 と EX-core-351 は新しい ID を持たない。
- REQ-core-196 は A32（主セッション）が出典だが、設定を読まないことは A15（利用者）の why にあるので、その部分だけを残した。
- 計画を書く工程の skill は、のちの記録（2026-11-24-unattended-merge.md の A36）で例の計画書の要求ごと消えている。問題の節は、検査が無かった当時のこととして過去形で書き、工程の skill の手順（A4 の lefthook に載せない、A8 の指摘を直して回す、A13 の例の計画書）は書かなかった。
- スキーマの置き場は、A14 の `.kotowari/schemas/plan.yaml` と今のコード（`crates/kotowari-core/schemas/plan.yaml` を取り込む）が食い違うので、決めたことには置き場を書かなかった。見送った案の `.kotowari/schemas/plan.yaml` は A6 の案そのものなので残した。
- A9 の why のうち、IR・用語集・問題の記録のスキーマと判断の記録の形の検査に触れる部分は、新しい形で無くなるものとの比較なので書かなかった。
- 記録の「mds」は、エンジンの呼び名として「Markdown の形をスキーマで検査するエンジン」「エンジン」と書いた。
- 停止の理由の文言（records.md の A104、第9ラウンド）は CLI の話題の決定なので、この話題では、テストで見える標準エラーの1行目の文字として確認することの行に残すだけにした。
- `req_core_193_a_finding_without_a_line_has_a_null_line` は、plan の側では落とした細部を確かめるが、文書全体にかかる指摘を null にする finding-order の原則の実例なので、C-core-forder-4 に付け替える。

- 主セッションが「承認のときに利用者へ示す」として決めた細部（A22、A31、A32、A33）は、確定の段でも落としたままにした（安全に関わる落とした細部の一覧には無い）。そのテストのうちこの話題の ID だけを指す7本は、マーカーが空にならないよう、いちばん近い行（C-8、C-14、C-16）を指す。
- 「など」「のような」を直した: frontmatter の「形の宣言（`$schema` など）」を「形の宣言としては使わず（`$schema` の鍵があっても使わない）」に、C-10 の「（"### Step 1: …"、":" が無い、など）」を「（例: …）」に、path の「"./a.md" のような形」を「"./" や ".." を含む形」にした。
## 数

- 元: 要求 8、場面 32、決定表 0、性質 0
- 読んだ記録のファイル: 4（2026-09-24-plan-schema.md、2026-09-24-guide-gaps.md、records.md（A20、A40、A104）、2026-11-24-unattended-merge.md（A36））
- 出来上がり: 決定 7、確認することの行 23（うち「（読んで確認）」0）
- テスト: 60（`tests/step14_plan.rs` 59、`crates/kotowari-core/src/schema.rs` 1）。新しい ID を持つもの 59（うち finding-order の行だけへ 1、plan の行と finding-order の行の両方へ 1、落とした細部のテストでいちばん近い行を指すもの 7）、「-」1（ほかの話題の ID が残る req_core_025_plan_text_output_shows_a_missing_line_as_a_dash）
