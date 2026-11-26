# status の対応表

元: `docs/ir/core/status.md`（日本語は `status.ja.md`）。書き直した文書: `status.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-162 | C-core-status-1, C-core-status-2 | status は check と同じものを読んで同じ検査をし、指摘の代わりに数と complete を出す | 読み替え。IR の文書を決定の文書に。読むもの、check と同じ検査、指摘を出さないことはそのまま（query-status の A8、A19、doc-marks の A22、surface-check の A14、whole-picture の A64。利用者） |
| REQ-core-163 | C-core-status-3, C-core-status-4 | status は check と同じものを読んで同じ検査をし、指摘の代わりに数と complete を出す | そのまま（A19。利用者）。読むことの決定の中にまとめた |
| REQ-core-164 | C-core-status-6, C-core-status-7, C-core-status-8, C-core-status-9, C-core-status-10, C-core-status-13 | 集計は、テストのマーカー、ガイド、全体像、面、指摘の数を群ごとに出す | 読み替え。中身は TBL-core-028 の行を参照 |
| REQ-core-165 | C-core-status-5 | 終了コードは、complete なら 0、そうでなければ 1 にする | 読み替え（一部）。終了コードはそのまま（A11。利用者）。complete の条件のうち「check の誤りが0件」は残り、「問題の記録の項目が0件」は消えて、decision-documents.md の「人に戻す問題は、話題の文書の「未決」の節に書く」（C-core-docs-12、C-core-docs-13）の「未決」の項目に置き換わった。complete の条件は、そちらに書いてあるので書き直さなかった |
| REQ-core-166 | C-core-status-11, C-core-status-12 | 出力は json と text の2つにし、text は群ごとに1行にする | そのまま（A13、A14。利用者）。群の順と "complete" を最後に置くこと（A20 の (2)、主セッション）は落とした |
| TBL-core-028 | C-core-status-6, C-core-status-7, C-core-status-8, C-core-status-9, C-core-status-10, C-core-status-13 | 集計は、テストのマーカー、ガイド、全体像、面、指摘の数を群ごとに出す | 読み替え（一部）。"tests"、"guides"、"overview"、"surface"、"findings"、"complete" の群は残る（A8、doc-marks の A17、whole-picture の A64、surface-check の A10。利用者）。"surface" の "specified" は「IR にある面」を「決定の文書に書かれた面」に読み替えた。"documents" の群は決定の文書を数えるように読み替え、今のコードがまだ IR の文書を数えるので、別の決定「documents の群は、読んだ決定の文書の数と行数を数える」（提案）にした。"items"、"requirements"、"scenarios" の群は消える（IR の項目の種類、検証の値、how_to_verify、具体例、後回し）。代わりは confirmations.md の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」の、決定の状態ごとの数（C-core-confirm-9）と、利用者の答えで足した確認することの行の数（別の決定「確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す」、提案。C-13）。"requirements" の "with_tests"、"without_tests"、"review_with_how_to_verify"、"review_without_how_to_verify" は、この決定に読み替えた |
| EX-core-258 | C-core-status-5, C-core-status-6, C-core-status-9 | 終了コードは、complete なら 0、そうでなければ 1 にする | 読み替え。complete で終了コード0、マーカーの数、誤りの数は残る。要求とシナリオの数は消える |
| EX-core-259 | - | - | 消える。review の要求の how_to_verify は、confirmations.md の「テストで確かめない行は、頭に「（読んで確認）」と書く」に置き換わった。「（読んで確認）」の行には確かめ方の行を求めないので、how_to_verify の無い要求に当たるものは新しい形に無い |
| EX-core-260 | - | 終了コードは、complete なら 0、そうでなければ 1 にする | 消える。問題の記録は、decision-documents.md の「人に戻す問題は、話題の文書の「未決」の節に書く」の「未決」の項目に置き換わった（C-core-docs-12 が同じ場面） |
| EX-core-261 | C-core-status-11 | 出力は json と text の2つにし、text は群ごとに1行にする | 読み替え。"items"、"requirements"、"scenarios" の行は消える |
| EX-core-262 | C-core-status-3 | status は check と同じものを読んで同じ検査をし、指摘の代わりに数と complete を出す | そのまま |
| EX-core-263 | - | 確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す | 消える。シナリオの "@about" を経てテストのある要求を数えることは IR の形にしかない。テストを数える単位は、confirmations.md の「check は、確認することの行ごとに、それを指すテストがあるかを数える」で確認することの行に置き換わった |
| EX-core-398 | - | 集計は、テストのマーカー、ガイド、全体像、面、指摘の数を群ごとに出す | 消える。後回しは、confirmations.md の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」の「提案」に置き換わった |

元の文書に PROP は無い。

## テストごとの表

「IR で準備」と書いたテストは、IR の項目で準備して、形に依らない振る舞いを確かめている。古い形のコードを消すときに、準備を決定の文書に書き直す（下の「テストの無い行」）。決定の文書で確かめている step23 のテストは、次の「マーカーに足すテスト」に分けた。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step12_status.rs | req_162_complete_project_counts_and_exit_zero | REQ-core-162, REQ-core-165, TBL-core-028, EX-core-258 | C-core-status-5, C-core-status-6, C-core-status-9 | IR で準備。complete で終了コード0、"tests" の "marks"、"findings" の数を確かめる。要求とシナリオの数の確かめは消す |
| tests/step12_status.rs | req_165_review_requirement_without_how_to_verify_is_not_complete | REQ-core-165, REQ-core-098, EX-core-259 | - | how_to_verify の無い review の要求だけを確かめる。古い形のコードと一緒に消す |
| tests/step12_status.rs | req_165_flag_makes_it_not_complete | REQ-core-165, EX-core-260 | - | 問題の記録だけを確かめる。古い形のコードと一緒に消す（「未決」の場面は step23 の an_item_in_the_open_section_makes_status_incomplete が C-core-docs-12 で確かめる） |
| tests/step12_status.rs | tbl_028_requirement_covered_through_a_scenario_counts_as_with_tests | TBL-core-028, REQ-core-085, EX-core-263 | - | シナリオを経たテストの数え方だけを確かめる。古い形のコードと一緒に消す |
| tests/step12_status.rs | tbl_028_review_requirements_are_not_in_with_or_without_tests | TBL-core-028 | - | 要求の数だけを確かめる。古い形のコードと一緒に消す |
| tests/step12_status.rs | tbl_028_requirement_without_a_verification_line_is_counted_by_marks_only | TBL-core-028 | - | 要求の数だけを確かめる。古い形のコードと一緒に消す |
| tests/step12_status.rs | tbl_028_items_are_counted_by_kind | TBL-core-028 | - | IR の項目の種類ごとの数だけを確かめる。古い形のコードと一緒に消す |
| tests/step12_status.rs | tbl_028_requirements_are_counted_by_verification_value | TBL-core-028 | - | 検証の値ごとの数だけを確かめる。古い形のコードと一緒に消す |
| tests/step12_status.rs | tbl_028_notices_are_counted_apart_from_errors | TBL-core-028, REQ-core-165 | C-core-status-5, C-core-status-9 | IR で準備（行数の上限の注意）。注意だけなら complete で終了コード0、誤りと注意を分けて数えることを確かめる。行数の上限の注意は無くなるので、実装のときに別の注意（guide_stale）で準備し直す |
| tests/step12_status.rs | tbl_028_documents_and_test_files_match_check | TBL-core-028, TBL-core-021 | C-core-status-6, C-core-status-10 | この話題の分だけ。"tests" の "files" は C-6。"documents" は決定の文書を数える提案（C-10）なので実装のときに直す。check と同じ値であることは落とした細部。TBL-core-021 はほかの話題 |
| tests/step12_status.rs | req_166_json_top_level_has_only_the_groups | REQ-core-164, REQ-core-166, TBL-core-028 | C-core-status-12 | IR で準備。群の名前の並びから "items"、"requirements"、"scenarios" を外し、群の順の確かめ（落とした細部）を外す。実装のときに直す |
| tests/step12_status.rs | req_162_status_writes_no_finding | REQ-core-162 | C-core-status-1 | IR で準備（"- verification:" の行の無い要求で誤りを出す） |
| tests/step12_status.rs | req_166_text_prints_one_line_per_group | REQ-core-166, TBL-core-028, EX-core-261 | C-core-status-6, C-core-status-11 | IR で準備。行の数と "items"、"requirements"、"scenarios" の行は直す。実装のときに直す |
| tests/step15_guides.rs | ex_368_status_stops_when_a_guide_is_also_a_test_file | REQ-core-163, REQ-core-199, EX-core-368 | C-core-status-4 | この話題の分（REQ-core-163）だけ。形に依らない（IR の文書を置かない）。REQ-core-199 と EX-core-368 はガイドの話題 |
| tests/step15_guides.rs | req_162_status_carries_the_guides_group | REQ-core-162, TBL-core-028 | C-core-status-7, C-core-status-11 | IR で準備（ガイドのマーカーが IR の要求を指す）。"guides" の群と、"text" の "guides" の行を確かめる。"text" の行の順の確かめは落とした細部 |
| tests/step16_deferred.rs | ex_398_deferred_items_are_counted_and_leave_complete_true | TBL-core-028, REQ-core-165, EX-core-398 | - | 後回しだけを確かめる。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | tbl_028_a_deferred_requirement_without_examples_is_counted_in_without_examples | TBL-core-028 | - | 後回しと具体例だけを確かめる。古い形のコードと一緒に消す |
| tests/step18_overview.rs | ex_core_473_an_error_in_overview_data_keeps_status_from_complete | EX-core-473, REQ-core-289, REQ-core-290, REQ-core-162, TBL-core-028 | C-core-status-2, C-core-status-5, C-core-status-7 | この話題の分（REQ-core-162、TBL-core-028）だけ。全体像の元データの誤りで complete が false、終了コード1、"overview" の群を確かめる。元データの中が IR の文書を指しているので、全体像の元データが決定の文書を指すようになったら直す。ほかの ID は全体像の話題 |
| tests/step18_overview.rs | req_core_289_status_text_shows_the_overview_group_after_guides | REQ-core-289, REQ-core-166, TBL-core-028 | C-core-status-7, C-core-status-11 | この話題の分だけ。"text" の "overview" の行を確かめる。"guides" の行の次という順は落とした細部。REQ-core-289 は全体像の話題 |
| tests/step17_surface.rs | req_229_status_counts_surfaces_and_a_surface_error_makes_it_incomplete | REQ-core-229, REQ-core-162, TBL-core-028 | C-core-status-2, C-core-status-5, C-core-status-8, C-core-status-11 | この話題の分だけ。面の誤りで complete が false、終了コード1、"surface" の群の3つの数と "text" の行を確かめる。IR で準備（面が IR にある）。REQ-core-229 は面の話題 |
| tests/step17_surface.rs | tbl_028_status_surface_counts_are_zero_without_surface_rules | TBL-core-028, REQ-core-162 | C-core-status-8 | 面の規則が無ければ面が見つからないので、3つの数が0になる。群をいつも出すこと（surface-check の A17、まとめて採用）は落とした細部で、テストの中に残る。IR で準備 |
| tests/step17_surface.rs | req_226_a_name_quoted_in_a_decision_is_counted_as_specified_by_status | REQ-core-226, TBL-core-028 | C-core-status-8 | この話題の分（TBL-core-028）だけ。決定の文書に引用された面が "specified" に数えられることを、決定の文書で確かめる。REQ-core-226 は面の話題 |
| tests/step1_cli.rs | req_163_unreadable_config_stops_like_check | REQ-core-163, EX-core-262 | C-core-status-3 | そのまま |
| tests/step1_config.rs | req_003_list_query_and_status_read_the_config_given_by_config | REQ-core-003, REQ-core-152, REQ-core-158, REQ-core-163 | C-core-status-10 | この話題の分（REQ-core-163）だけ。"--config" の設定の置き場の文書を "documents" の "files" に数えることを確かめる。IR で準備。"documents" は提案なので実装のときに直す |
| crates/kotowari-core/tests/ir.rs | req_098_review_requirement_without_how_to_verify_is_a_missing_field | REQ-core-098, TBL-core-008, EX-core-259 | - | この話題の分（EX-core-259）は消える。古い形のコードと一緒に消す。ほかの ID は IR の欠けた行の話題 |

## マーカーに足すテスト

この話題の ID を今は指していないが、付け替えたあとの確認することの行を決定の文書で確かめているテスト。今のマーカー（別の話題の ID）は残したまま、この話題の ID を足す。同じテストがほかの話題の対応表にも載っているときは、それぞれの表の ID を合わせて足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 足す ID | 理由 |
|---|---|---|---|---|
| tests/step23_decision_documents.rs | no_errors_and_no_open_items_make_status_complete | C-core-docs-13 | C-core-status-2 | 状態の行の誤り（check の誤り）のある文書を足すと complete が false になることを確かめている |
| tests/step23_decision_documents.rs | status_counts_the_proposed_and_the_implemented_decisions_apart | C-core-confirm-9 | C-core-status-11 | "text" の "decisions proposed=3 implemented=1" の行を確かめている |

## 落とした細部

### 安全に関わるもの

- 無し。停止の条件（REQ-core-163）と、"surface.rules" が空なら面を読まないこと（REQ-core-162）は、どちらも決定に残した。安全に関わる落とした細部の一覧に、この話題の行は無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-166: JSON の群の順を表の順にし、"complete" を最後に置くこと。"text" の行も表の順に出すこと | 主セッション（query-status の A20 の (2)。承認時に提示） |
| TBL-core-028: "tests" の "marks" を、1つのマーカーに ID が複数あれば ID ごとに1つと数えること | 主セッション（A20 の (4)） |
| TBL-core-028: "documents" の "files" と "lines"、"tests" の "files" が、check の同じ鍵と同じ値であること | 主セッション（A20 の (5)） |
| TBL-core-028: "surface" の群をいつも出し、"surface.rules" が空なら3つとも0にすること。一覧に載っていて仕様にもある面を "specified" に数えること | 利用者（surface-check の A17。IR を書く段で決めた細部をまとめて採用）。surface-unspecified の対応表が A22 を扱ったのと同じく、決定の文には面の数の意味（A10。利用者）だけを書いた |
| TBL-core-028: "text" の "tests" の行の "files" を "拡張子=ファイルの数" と書くこと | 表にだけある書き方。A8（利用者）は拡張子ごとの数を出すことだけを決め、A14（利用者）が "鍵=値" の形を決めている。決定の文には拡張子ごとの数とだけ書いた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| TBL-core-028 | 読み替えて残る（一部）。"items"、"requirements"、"scenarios" の群は消える。"documents" は決定の文書を数えるように読み替える（提案）。"surface" の "specified" は「決定の文書に書かれた面」に読み替える |
| REQ-core-164 | TBL-core-028 と同じ |
| REQ-core-165 | 読み替えて残る（一部）。問題の記録の項目を、決定の文書の「未決」の項目に（decision-documents.md にすでにある） |
| REQ-core-162 | 読み替えて残る。IR の文書を決定の文書に |
| EX-core-259、EX-core-260、EX-core-263、EX-core-398 | 消える |

## テストの無い行

| 行 | 要るテスト |
|---|---|
| C-core-status-1 | 決定の文書に check の誤りがあるとき、"kotowari status" の出力に指摘の1件ずつ（"kind"、"detail"、"severity"）が無く、"findings" が "error" と "notice" だけであること（今は IR の誤りで確かめている） |
| C-core-status-6 | 確認することの ID を指すマーカーで、"tests" の "marks" と拡張子ごとの数が出ること（今は IR の ID のマーカーで確かめている） |
| C-core-status-9 | 決定の文書だけの置き場で、注意だけが出るときに "notice" に数えられ、complete が true で終了コード0であること（今は IR の文書の行数の上限の注意 too_many_lines で確かめているが、行数の上限の注意は利用者の答えで無くなるので、別の注意で準備する。例: 古いガイドのマーカーの guide_stale） |
| C-core-status-12 | 決定の文書だけの置き場で、JSON の最上位の鍵が残る群の名前と "complete" だけであること（今のテストは消える群の名前を並べている） |

## 誰が決めたか見分けられなかったもの

- 無し。

## 理由や見送った案が記録に無かった決定

- 「status は check と同じものを読んで同じ検査をし、指摘の代わりに数と complete を出す」: 見送った案が記録に無い。
- 「集計は、テストのマーカー、ガイド、全体像、面、指摘の数を群ごとに出す」: 見送った案が記録に無い。"tests" の群の理由は記録に無い（A8 は見本で合意したとだけある）。
- 「確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す」: 決定そのものは利用者の答え（確定の段）。理由は query-status の A8（利用者。見本を見て合意し、数の追加と削除は使いながら判断する）と、確認することの行を単位にする confirmations.md の決定を当てた。見送った案は無い。
- 「documents の群は、読んだ決定の文書の数と行数を数える」: この決定そのものは記録に無い（読み替えで足した提案）。文書の数と行数を出すことの理由は記録に無く、A8 で見本に合意したことだけがある。問題の節には読み替えの理由だけを書いた。
- 「出力は json と text の2つにし、text は群ごとに1行にする」: 見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-core-162 と REQ-core-163 を1つの決定「status は check と同じものを読んで同じ検査をし、指摘の代わりに数と complete を出す」にまとめた。読むことと停止が同じ理由（check と同じ作り）だから。
- REQ-core-165 を分けた。終了コードは「終了コードは、complete なら 0、そうでなければ 1 にする」に、complete の条件は decision-documents.md にすでにあるので言葉で指した。status 独自の判定を持たないこと（A9）は、読むことの決定に入れた。
- REQ-core-164 と TBL-core-028 を2つに分けた。実装済みの群（「集計は、テストのマーカー、ガイド、全体像、面、指摘の数を群ごとに出す」）と、提案の "documents"（「documents の群は、読んだ決定の文書の数と行数を数える」）。
- 利用者の答えで足した確認することの行の数は、実装済みの群の決定に混ぜず、別の「提案」の決定にした（「確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す」）。
- 固定の語を英語にすること（query-status の A13）は、query と status の両方の出力の決定に書いた。

## あとでリンクにする所

- 確定の段で、decision-documents.md の「人に戻す問題は、話題の文書の「未決」の節に書く」（冒頭の段落と complete の決め方）と、confirmations.md の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」（冒頭の段落と「集計は…」の決めたこと）、「check は、確認することの行ごとに、それを指すテストがあるかを数える」と「テストで確かめない行は、頭に「（読んで確認）」と書く」（確認することの行の数の決定の問題）を、`*.ja.md` の見出しへのリンクにした
- 残り: 面の検査の話題（surface.md）。"surface" の群の、面が決定の文書に書かれているかの判定。今は文の中で指していないので、リンクにする所は無い

## ガイドのマーカーの行き先

- `docs/guides/commands/status.md` と `status.ja.md` の204行目のマーカー（`REQ-core-098` と `EX-core-259`）は、節「`review_without_how_to_verify` が 0 でない」に付いている。この話題の ID は EX-core-259 だけで、当たる決定は無い（how_to_verify の場面が消える）。REQ-core-098 も IR の欠けた行の要求で、古い形と一緒に消える。「（読んで確認）」の行には確かめ方の行を求めないので、この節が説明する症状は新しい版で起きない。**この節はマーカーごと消す。** status の新しい数（確認することの行のうちテストの無い行の数）を説明する節をガイドに足すなら、そのマーカーは status.md の決定「確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す」の見出しを指す（その決定が実装済みになってから）
- 同じガイドの EX-core-260、EX-core-398、EX-core-263 を指すマーカーは、同じ行に REQ-core-165 か TBL-core-028 も持つので、そちらの決定の見出しに付け替えられる。ただし EX-core-263 の節「`without_tests` が 0 でない」は "requirements" の群を説明しているので、その節は「確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す」の見出しへ付け替えて書き直すことになる

## 実装する側が決める細部

- まだ決まっていないもの: 「documents の群は、読んだ決定の文書の数と行数を数える」で、ほかの言語の側も数えるか、check の "files" と "lines" と同じ値にするか
- まだ決まっていないもの: 「確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す」の群の名前と鍵の名前（今のコードに当たる名前は無い。古い "requirements" の群の "with_tests"、"without_tests" が案になる）、"text" の行の置き場所、提案の決定の行と実装済みの決定の行を分けて数えるか、決定の文書の置き場が設定に無いときに群を出すか
- 今のコードにあるもの: 決定の状態ごとの数の群（"decisions"）を、設定に決定の文書の置き場があるときだけ出すことと、JSON と "text" の中の置き場所

## 利用者に聞くこと

無し（下書きの問いには答えが出た。下の「迷った判断」）。

## 「提案」で書いた決定の一覧

- 「確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す」: `crates/kotowari-core/src/status.rs` の `StatusResult` に確認することの行の数の群を足し、`crates/kotowari-core/src/inputs/inspect.rs` の `finish` で、決定の文書の確認することの行（`crates/kotowari-core/src/decisions.rs`）とテストのマーカーから数を入れる。"requirements" と "scenarios" の群は古い形のコードと一緒に消す。
- 「documents の群は、読んだ決定の文書の数と行数を数える」: `crates/kotowari-core/src/inputs/inspect.rs` の `finish` で、`status.documents` に IR の文書と側の代わりに、決定の文書（`self.read.decisions`）の数と行数を入れる。`crates/kotowari-core/src/decisions.rs` の `Document` に行数を持たせることになりそう。check の "files" と "lines"（出力の話題）も同じく直すかは、その話題の書き直しで決める。

## 迷った判断と、どう決めたか

- 利用者の答え（確認することの行の数）: status に、確認することの行の数を、テストのあるもの、無いもの、読んで確認のものに分けて出す。今のコードに無いので「提案」の決定にし、鍵の名前は「実装する側が決める細部」にした。
- complete の条件は decision-documents.md にすでに「提案」として書いてあり、今のコードはもう「未決」の項目で complete を false にする（`crates/kotowari-core/src/inputs/inspect.rs`）。同じことを二度書かないので、status の文書には終了コードと「status 独自の判定を持たない」ことだけを書いた。
- "surface" の群の鍵の名前は surface-check の A17（まとめて採用）で決まったものだが、利用者から見える名前なので表に残し、意味は A10（利用者。全部、仕様にある数、一覧に載っている数）が裏付ける範囲だけを書いた。
- "tests" の "files" が check の "tests" と同じ値であることは主セッションの細部なので落とし、拡張子ごとのファイルの数とだけ書いた。
- "documents" の群を残すかどうかは、A8 で文書の数と行数を出すと利用者が決めていて、読み替え先がはっきりしている（決定の文書）ので、利用者に聞かずに提案の決定にした。
- 「など」を直した: complete の決め方の「check が誤りにしないこと（注意など）」を「check の注意は、誤りではないので」にした（指摘の重さは誤りと注意の2つだけなので、挙げ切れる）。

## 数

- 元の文書: 要求5、場面7、決定表1、性質0（ID は合わせて13）
- 読んだ記録のファイル: 8（2026-09-20-query-status、2026-09-24-doc-marks、2026-09-27-surface-check、2026-10-02-whole-picture、2026-09-19-read-commands、2026-09-17-scenario-tests、2026-09-23-ir-english-tokens、2026-09-25-deferred-items）
- 出来上がり: 決定6（実装済み4、提案2）、確認することの行13（読んで確認0）
- テスト: この話題の ID を今指しているもの25本、マーカーに足す step23 のもの2本（「マーカーに足すテスト」の表）
