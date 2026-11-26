# list の対応表

元: `docs/ir/core/list.md`（日本語は `list.ja.md`）。書き直した文書: `list.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-151 | C-core-list-1, C-core-list-2 | list は check と同じものを読み、指摘を出さず、読めた決定と確認することの行をすべて出す | 読み替え。IR の項目とシナリオを、決定と形の正しい ID を持つ確認することの行に読み替えた。指摘を出さないこと、終了コード0、誤りがあっても読めたものを出すことはそのまま（read-commands の A1、A2、A3、A9。利用者） |
| REQ-core-152 | C-core-list-11, C-core-list-13, C-core-list-14, C-core-list-15, C-core-list-16, C-core-list-17 | list は check と同じ条件で停止し、ガイドと面は読まない | そのまま。言語が2つ以上のときに置き場を辿る部分（localization の A37）は「言語が2つ以上なら、対ごとの blob hash も出す」に分けた（C-11）。言語が1つのときに全体像の元データを読まないこと（whole-picture の A73、LLM）は、安全に関わる細部として決定に戻した（C-17） |
| REQ-core-153 | C-core-list-3, C-core-list-4, C-core-list-5, C-core-list-6 | 確認することの行にはそれを指すテストを、決定には指紋を付ける | 読み替え。中身は TBL-core-026 の行を参照 |
| REQ-core-154 | C-core-list-7 | 並びはパスの順、同じパスの中は行の順にする | そのまま（read-commands の A12。利用者） |
| REQ-core-155 | C-core-list-8, C-core-list-9, C-core-list-10 | 出力は json と text の2つにし、絞り込みのオプションは持たない | 読み替え。"text" の "検証" の欄を、「（読んで確認）」の行で "reading"、ほかで "-" にした（今のコードの綴り）。"--format"、既定、絞り込みを持たないこと、字下げした "tests" の行はそのまま（A7、A8、A19。利用者）。"translations" は「言語が2つ以上なら、対ごとの blob hash も出す」に分けた（C-10）。行の末尾の " deferred" は消える（後回しの宣言は confirmations.md の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」で「提案」に置き換わった） |
| TBL-core-026 | C-core-list-3, C-core-list-4, C-core-list-5, C-core-list-6 | 確認することの行にはそれを指すテストを、決定には指紋を付ける | 読み替え（一部）。"id"、"kind"、"name"、"path"、"line"、"tests"、"fingerprint" は残す。"kind" の値は "decision" と "confirmation" に（今のコード）。"verification" は「（読んで確認）」の行かどうか（"by_reading"）に読み替えた。"type"、"verification"、"definition"、"examples"、"how_to_verify"、"relations"、"sources"、"deferred" は消える（IR の項目の形、出典の行、問題の記録、後回しの宣言。decision-documents.md の「仕様は、話題ごとの、今有効な決定の文書だけで持つ」「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」、confirmations.md の「テストで確かめない行は、頭に「（読んで確認）」と書く」「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」に置き換わった）。"fingerprint" は決定だけが持つ（doc-marks の A14 の読み替え） |
| EX-core-288 | - | - | 消える。シナリオの重複と "examples" は IR の形にしかない。確認することの ID の重複は confirmations.md の C-core-confirm-3 で check が誤りにする |
| EX-core-245 | C-core-list-3 | 確認することの行にはそれを指すテストを、決定には指紋を付ける | 読み替え。要求を確認することの行に。"verification" が "unit" であることは消える |
| EX-core-246 | C-core-list-2 | list は check と同じものを読み、指摘を出さず、読めた決定と確認することの行をすべて出す | 読み替え。IR の誤り（"- verification:" の行が無い）を、決定の文書の check の誤りに |
| EX-core-247 | C-core-list-4 | 確認することの行にはそれを指すテストを、決定には指紋を付ける | そのまま |
| EX-core-248 | C-core-list-9 | 出力は json と text の2つにし、絞り込みのオプションは持たない | 読み替え。"検証" の欄の "unit" を、確認することの行の "-" か "reading" に |
| EX-core-249 | C-core-list-13 | list は check と同じ条件で停止し、ガイドと面は読まない | そのまま |
| EX-core-399 | - | 出力は json と text の2つにし、絞り込みのオプションは持たない | 消える。後回しの宣言は「提案」の状態に置き換わった（confirmations.md の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」） |

元の文書に PROP は無い。

## テストごとの表

「IR で準備」と書いたテストは、IR の項目で準備して、形に依らない振る舞いを確かめている。古い形のコードを消すときに、準備を決定の文書に書き直す（下の「テストの無い行」）。決定の文書で確かめている step23 のテストは、次の「マーカーに足すテスト」に分けた。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step10_list.rs | req_151_requirement_with_a_marked_test_is_listed | REQ-core-151, REQ-core-153, TBL-core-026, EX-core-245 | C-core-list-3 | IR で準備。マーカーのあるテストのファイル・行・名前が "tests" に入ることを確かめる |
| tests/step10_list.rs | req_151_items_are_listed_despite_ir_errors_and_exit_zero | REQ-core-151, EX-core-246 | C-core-list-2 | IR で準備（"- verification:" の行の無い要求）。終了コード0と、出力に "findings" が無いことを確かめる |
| tests/step10_list.rs | req_153_test_in_a_language_without_a_query_has_a_null_name | REQ-core-153, TBL-core-026, EX-core-247 | C-core-list-4 | IR で準備 |
| tests/step10_list.rs | req_153_table_property_scenario_and_flag_carry_their_keys | REQ-core-153, TBL-core-026 | - | 決定表、性質、シナリオ、問題の記録の鍵だけを確かめる。古い形のコードと一緒に消す |
| tests/step10_list.rs | tbl_026_only_the_first_scenario_of_a_duplicated_id_counts_as_an_example | TBL-core-026, EX-core-288 | - | "examples" だけを確かめる。古い形のコードと一緒に消す |
| tests/step10_list.rs | req_153_examples_are_the_scenarios_about_the_item | REQ-core-153, TBL-core-026 | - | "examples" だけを確かめる。古い形のコードと一緒に消す |
| tests/step10_list.rs | req_153_how_to_verify_is_the_value_of_the_line_or_null | REQ-core-153, TBL-core-026, TBL-core-011 | - | "how_to_verify" だけを確かめる。古い形のコードと一緒に消す（TBL-core-011 はほかの話題） |
| tests/step10_list.rs | req_153_tests_have_one_entry_per_marker_occurrence | REQ-core-153, TBL-core-026 | C-core-list-3 | IR で準備。同じテストの2つのマーカーが2件になることを確かめる |
| tests/step10_list.rs | req_154_items_and_tests_are_ordered_by_path_then_line | REQ-core-154 | C-core-list-7 | IR で準備。シナリオを使った並べ替えの場面は、決定の文書では決定と行の並びで書き直す |
| tests/step10_list.rs | req_155_json_top_level_has_only_items | REQ-core-155 | C-core-list-8 | IR で準備 |
| tests/step10_list.rs | req_155_text_prints_one_line_per_item_and_indented_test_lines | REQ-core-155, EX-core-248 | C-core-list-9 | IR で準備 |
| tests/step10_list.rs | req_155_text_writes_dash_for_a_null_test_name | REQ-core-155 | C-core-list-9 | IR で準備 |
| tests/step10_list.rs | req_155_text_writes_dash_for_the_verification_of_a_non_requirement | REQ-core-155 | C-core-list-9 | IR で準備。要求以外の "-" を、「（読んで確認）」でない行の "-" に読み替える |
| tests/step10_list.rs | req_155_text_writes_dash_for_a_requirement_without_a_verification_line | REQ-core-155 | - | "- verification:" の行の無い要求だけを確かめる。古い形のコードと一緒に消す |
| tests/step10_list.rs | req_155_translations_list_every_pair_with_the_blob_hash_of_each_side | REQ-core-152, REQ-core-155 | C-core-list-10, C-core-list-12 | ガイド、全体像の元データ、目次の対は C-10。IR の対の部分は、決定の文書の対（C-12、提案）に直す。実装のときに直す |
| tests/step10_list.rs | req_152_with_two_languages_list_stops_on_the_guide_places_as_check_does | REQ-core-152 | C-core-list-11, C-core-list-15 | 形に依らない（IR の文書を置かない）。言語が1つのときに重なりで停止しないことも C-15 に当たる |
| tests/step11_query.rs | req_161_text_prints_body_and_referenced_by_lines | REQ-core-161, REQ-core-155 | C-core-list-9 | この話題の分（REQ-core-155）だけ。1行目とテストの行が list の "text" と同じことを確かめる。IR で準備。REQ-core-161 は query の対応表 |
| tests/step15_guides.rs | ex_372_list_shows_the_fingerprint_of_a_requirement | REQ-core-203, TBL-core-026, EX-core-372 | - | この話題の分（TBL-core-026）は要求の指紋で、古い形のコードと一緒に消す。決定の指紋は step23 の query_of_a_decision_returns_the_fingerprint_a_guide_mark_takes が確かめる。REQ-core-203 と EX-core-372 はガイドの話題 |
| tests/step15_guides.rs | req_203_a_table_keeps_its_leading_blank_line_after_the_source_line_is_dropped | REQ-core-203, TBL-core-026 | - | この話題の分は決定表の指紋で、古い形のコードと一緒に消す。REQ-core-203 はガイドの話題 |
| tests/step15_guides.rs | req_203_a_scenario_fingerprint_is_its_step_lines_only | REQ-core-203, TBL-core-026 | - | この話題の分はシナリオの指紋で、古い形のコードと一緒に消す。REQ-core-203 はガイドの話題 |
| tests/step15_guides.rs | req_152_list_and_query_do_not_read_guides_and_do_not_stop_on_their_overlap | REQ-core-152, REQ-core-158 | C-core-list-15 | この話題の分（REQ-core-152）だけ。IR で準備。REQ-core-158 は query の対応表 |
| tests/step16_deferred.rs | ex_399_list_marks_deferred_requirements_and_scenarios | TBL-core-026, REQ-core-155, EX-core-399 | - | 後回しだけを確かめる。古い形のコードと一緒に消す |
| tests/step18_overview.rs | req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it | REQ-core-152, REQ-core-158 | C-core-list-15, C-core-list-17 | この話題の分だけ。言語が1つの設定で、全体像の元データとガイドの置き場が同じ glob で重なり、そこのファイルが UTF-8 でなくても list が終了コード0で終わる。全体像の元データを読まないこと（C-17）と、ガイドを読まないこと（C-15）の両方を確かめている。REQ-core-158 は query の対応表 |
| tests/step1_cli.rs | req_152_tool_on_list_is_an_argument_error | REQ-core-152, REQ-core-004 | C-core-list-14 | この話題の分だけ。REQ-core-004 は CLI の話題 |
| tests/step1_cli.rs | req_152_positional_after_list_is_an_argument_error | REQ-core-152, REQ-core-004 | C-core-list-14 | この話題の分だけ。REQ-core-004 は CLI の話題 |
| tests/step1_cli.rs | req_152_unreadable_config_stops_like_check | REQ-core-152, EX-core-249 | C-core-list-13 | そのまま |
| tests/step1_config.rs | req_003_list_query_and_status_read_the_config_given_by_config | REQ-core-003, REQ-core-152, REQ-core-158, REQ-core-163 | C-core-list-1 | この話題の分（REQ-core-152）だけ。"--config" の設定の置き場から読むことを確かめる。IR で準備。REQ-core-003 は設定の話題、REQ-core-158 と REQ-core-163 は query と status の対応表 |
| tests/step17_surface.rs | req_229_list_and_query_do_not_read_the_surface_list_or_files | REQ-core-229, REQ-core-152, REQ-core-158 | C-core-list-16 | この話題の分だけ。IR で準備（query は "REQ-001" を引く）。REQ-core-229 は面の話題 |
| tests/step13_test_languages.rs | req_180_test_without_a_name_is_listed_with_a_null_name | REQ-core-180, TBL-core-026 | C-core-list-4 | この話題の分（TBL-core-026）だけ。IR の ID のマーカーで準備。REQ-core-180 はテストの見つけ方の話題 |

## マーカーに足すテスト

この話題の ID を今は指していないが、付け替えたあとの確認することの行を決定の文書で確かめているテスト。今のマーカー（別の話題の ID）は残したまま、この話題の ID を足す。同じテストがほかの話題の対応表にも載っているときは、それぞれの表の ID を合わせて足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 足す ID | 理由 |
|---|---|---|---|---|
| tests/step23_decision_documents.rs | list_includes_the_confirmation_rows_in_path_and_line_order | C-core-confirm-5 | C-core-list-1, C-core-list-3, C-core-list-7, C-core-list-9 | 決定と行が出ること、テストの1件、行の順、"text" の形を決定の文書で確かめている |
| tests/step23_decision_documents.rs | list_sorts_the_confirmation_rows_with_the_ir_items_by_path | C-core-confirm-5 | C-core-list-7 | IR の要求と混ぜた並びを確かめる。IR の文書は古い形のコードと一緒に外す |
| tests/step23_decision_documents.rs | list_leaves_out_a_row_without_a_well_formed_id | C-core-confirm-5 | C-core-list-1, C-core-list-2 | 形の誤った ID の行（check の誤り）があっても、ほかの決定と行が出る。終了コードと指摘の無いことは確かめていない（下の「テストの無い行」） |
| tests/step23_decision_documents.rs | list_includes_each_decision_with_its_fingerprint | C-core-lang-5 | C-core-list-1, C-core-list-9 | "text" だけを見ていて、"fingerprint" の値は確かめていない |
| tests/step23_decision_documents.rs | query_of_a_confirmation_id_returns_the_file_line_and_name_of_each_test_pointing_at_it | C-core-confirm-5 | C-core-list-3, C-core-list-5 | query の対応表の ID も足す。query の1件は list の1件と同じ鍵なので、"tests" の中身と "by_reading" が false であることを確かめている |
| tests/step23_decision_documents.rs | query_of_a_confirmation_id_prints_its_tests_in_text | C-core-confirm-5 | C-core-list-9 | query の対応表の ID も足す。「（読んで確認）」の行の検証の欄が "reading" であることを確かめている |
| tests/step23_decision_documents.rs | query_of_a_decision_returns_the_fingerprint_a_guide_mark_takes | C-core-lang-5 | C-core-list-6 | query の対応表の ID も足す。決定の1件の "fingerprint" を確かめている |

## 落とした細部

### 安全に関わるもの

無し（あった1件は、下の「戻した細部」のとおり決定に戻した）。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-152: 言語が1つのとき、"kotowari list" は全体像の元データを読まず、その置き場と読み込みによる停止（REQ-core-278、REQ-core-280）をしない。記録では LLM（whole-picture の A73。承認の時に利用者に示す） | C-core-list-17（決定「list は check と同じ条件で停止し、ガイドと面は読まない」） | テスト tests/step18_overview.rs の req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it（"overview.files" と "guides.files" を同じ glob にし、そこに UTF-8 でないファイルを置いて、list が終了コード0で終わることを確かめる） |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-155: "translations" の1件に、先頭の言語の側の無い対も1件出すこと、無い側の "blob" を null にし "text" で "言語タグ=-" と書くこと、1件を "path" の昇順に、"sides" を言語の一覧の順に並べること | LLM（localization の D2、D3。細部を LLM に任せた承認 D1 の範囲）。"sides" の名前は利用者から見えるので決定の文に残した。"path"、"language"、"blob" は A29（利用者）にある |
| REQ-core-155: "text" の "translations" の行の正確な形（区切りの空白） | 細部を LLM に任せた承認（localization の D1） |
| TBL-core-026: 1件の "path" が基準のディレクトリからの相対であること | 利用者（read-commands の A18）。落としたのではなく、基準のディレクトリの話題の決定へのリンクにした |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| TBL-core-026 | 読み替えて残る（一部）。IR の項目の5種類（"requirement"、"table"、"property"、"scenario"、"flag"）の1件は消え、決定と確認することの行の1件に置き換わる。"type"、"verification"、"definition"、"examples"、"how_to_verify"、"relations"、"sources"、"deferred" の鍵は消える。"verification" は "by_reading" に読み替えて残る |
| REQ-core-153 | 読み替えて残る。TBL-core-026 と同じ |
| REQ-core-155 | 読み替えて残る。"検証" の欄を「（読んで確認）」の行かどうかに。" deferred" の印は消える |
| EX-core-288 | 消える（シナリオの重複と "examples"） |
| EX-core-399 | 消える（後回しの宣言） |
| REQ-core-151、EX-core-245、EX-core-246 | 読み替えて残る。IR の文書を決定の文書に、項目とシナリオを決定と確認することの行に |

## テストの無い行

古い形のコードを消すと、IR で準備したテストは準備を書き直すまで当たらなくなる。決定の文書だけで確かめるテストが無い行と、要るテストは次のとおり。

| 行 | 要るテスト |
|---|---|
| C-core-list-2 | 決定の文書に check の誤り（たとえば状態の誤り）があるとき、"kotowari list" の終了コードが0で、出力に "findings" が無く、標準エラーが空であること。今の list_leaves_out_a_row_without_a_well_formed_id は出力の中身だけを見ている |
| C-core-list-3 | 同じテストに同じ確認することの ID のマーカーが2つあるとき、"tests" が2件になること（今は req_153_tests_have_one_entry_per_marker_occurrence が IR で確かめている） |
| C-core-list-4 | 問い合わせの無い言語のテストと、名前を取れないテストのマーカーが確認することの ID を指すとき、"name" が null になること（今は IR で準備した2本） |
| C-core-list-5 | 「（読んで確認）」の行の JSON の "by_reading" が true であること（今は "text" の "reading" と、false の側だけを確かめている） |
| C-core-list-6 | list の JSON の決定の1件の "fingerprint" が、ガイドのマーカーに書く値と同じであること（今は query の1件で確かめている） |
| C-core-list-8 | 言語が1つのとき、決定の文書だけの置き場で list の最上位の鍵が "items" だけであること |
| C-core-list-10 | 決定の文書を置いた言語2つの設定で、ガイド、全体像の元データ、目次の対が "translations" に出ること（今のテストは IR の対を含む） |
| C-core-list-16 | 決定の文書で準備した面の設定で、list が面の一覧と面のファイルを読まないこと（今のテストは IR で準備） |

## 誰が決めたか見分けられなかったもの

- 無し。read-commands の A17（主セッション、事実の制約）と A21（用語の書き分け、主セッションが選んだ）は、どちらも IR の形と用語集の話で、新しい版では消える。

## 理由や見送った案が記録に無かった決定

- 「確認することの行にはそれを指すテストを、決定には指紋を付ける」: 見送った案が記録に無い。決定の指紋を list が持つ理由は、doc-marks の A14 の why に、ガイドの注意の detail の話としてしか無い。問題の節には、ガイドのマーカーが指紋を書くという事実を書いた。
- 「並びはパスの順、同じパスの中は行の順にする」: 見送った案が記録に無い。
- 「言語が2つ以上なら、対ごとの blob hash も出す」: 見送った案が記録に無い。
- 「決定の文書の対も translations に出す」: この決定そのものは記録に無い（読み替えで足した提案）。理由は localization の A29 の why をそのまま当てた。見送った案は無い。
- 「list は check と同じ条件で停止し、ガイドと面は読まない」: 見送った案が記録に無い。面を読まない理由は、surface-check の A14 の why（status の complete に面の誤りが入る）が status の側の理由なので、問題には「面を読んで数を出すのは check と status」とだけ書いた。

## 分けた所、まとめた所

- REQ-core-153 と TBL-core-026 を1つの決定「確認することの行にはそれを指すテストを、決定には指紋を付ける」にまとめた。
- REQ-core-155 を3つに分けた: 「出力は json と text の2つにし、絞り込みのオプションは持たない」（"--format"、"items"、"text"）、「言語が2つ以上なら、対ごとの blob hash も出す」（"translations"。実装済み）、「決定の文書の対も translations に出す」（提案）。実装済みと提案が混ざるため。
- REQ-core-152 の、言語が2つ以上のときに置き場を辿る部分（localization の A37）は、"translations" の決定の中に移した。
- 1件の "path" が基準のディレクトリからの相対であることは、[設定ファイルの値のパスと、出力の "path" は、基準のディレクトリからの相対にする](base-directory.md#設定ファイルの値のパスと出力の-path-は基準のディレクトリからの相対にする)へのリンクにした。

## あとでリンクにする所

- 確定の段で、languages-and-guides.md の「ガイドは、決めたことと確認することが変わったら古いとする」（「確認することの行にはそれを指すテストを、決定には指紋を付ける」の問題）と、「決定の文書も、多言語の対で持てる」（「決定の文書の対も translations に出す」の問題）を、`languages-and-guides.ja.md` の見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した
- 残り: confirmations.md の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」: 対応表の EX-core-399 と REQ-core-155（後回しの置き換え先）。文書の中では指していない
- 1件の読み取り（query.md）と揃っているかの集計（status.md）は、一緒に書き直したので見出しへのリンクにした

## 実装する側が決める細部

決定の文には書かなかった。前の6つは今のコードにもうあり、作業は残っていない。最後の1つは、提案の決定を作るときに決める。

- 確認することの行の1件が持つ、属する決定の見出し（"decision"）と、その決定の状態（"state"。"proposed" か "implemented"、状態の行が誤りなら null）
- 決定の1件が持つ状態（"state"）
- 決定の1件の ID の書き方（"<文書のパス>#<見出しの行き先の名前>"。ガイドのマーカーが決定を指す書き方と同じ）
- 確認することの行は指紋を持たないこと
- "text" の決定の行で、"検証" の欄が "-"、"tests=0" であること
- 「決定の文書の対も translations に出す」で、決定の文書の対をどう集めるか（ほかの置き場と同じく先頭の言語の側の同じディレクトリで名前で探す、など）

## 利用者に聞くこと

- 無し

## 「提案」で書いた決定の一覧

- 「決定の文書の対も translations に出す」: `crates/kotowari/src/decisions.rs` の `push` が決定の文書を `translations::Assembly` の `place` に通していないので、ガイドや IR と同じく対に入れる。languages-and-guides.md の「決定の文書も、多言語の対で持てる」（C-core-lang-1〜4）の実装と一緒に行うことになる。

## 迷った判断と、どう決めたか

- 全体像の元データを読まないこと（whole-picture の A73、LLM）は、下書きでは落としたが、確定の段で「安全に関わる細部は、今のテストか skill の文が確かめているなら決定に戻す」という利用者の決定に従って戻した（C-17）。理由は記録に無い（A73 の why は敵対的レビューの指摘から導いたとだけある）ので、問題の節には落とすと何が起きるかと「理由は記録に残っていない」だけを書いた。言語が2つ以上のときは置き場を辿る（C-10、C-11）ので、文は「言語が1つのとき」に限った。
- 「（読んで確認）」の行かどうかを、元の "verification" の読み替えとして1件に残した。read-commands の A6（利用者）が要求の検証の値を1件に持たせたのは、1回の list でテストの要否が分かるためで、新しい形でそれに当たるのは「（読んで確認）」かどうかである。鍵の名前 "by_reading" と "text" の "reading" は今のコードの綴り。
- 確認することの行が属する決定の見出しと状態（"decision"、"state"）は、利用者の決定に当たるものが無いので、決定の文に書かず「実装する側が決める細部」にした。read-commands の A3（1回の list で揃える）で裏付けることもできたが、どの値を足すかまでは決めていない。
- 形の誤った ID の行を出さないことは、read-commands の A6 の「ID を持つ項目を出す」の読み替えとして決定に書いた（今のコードとテストもそう）。
- "translations" の鍵の名前のうち "sides" は D3（LLM）の綴りだが、利用者から見える名前なので文に残し、振る舞いは A29（利用者）が決めた範囲だけを書いた。
- step18 の list と query のテストは、全体像の元データの検査を落としたが、同じファイルがガイドの置き場にも当たるので、ガイドを読まないこと（C-15）に当てた。

## 数

- 元の文書: 要求5、場面7、決定表1、性質0（ID は合わせて13）
- 読んだ記録のファイル: 9（2026-09-19-read-commands、2026-09-24-doc-marks、2026-09-27-surface-check、2026-10-02-whole-picture、2026-10-05-localization、2026-09-23-ir-english-tokens、2026-09-25-deferred-items、2026-09-24-review6-gaps、2026-09-24-multi-language-tests）
- 出来上がり: 決定7（実装済み6、提案1）、確認することの行17（読んで確認0）
- テスト: この話題の ID を今指しているもの29本、マーカーに足す step23 のもの7本（「マーカーに足すテスト」の表）
