# query の対応表

元: `docs/ir/core/query.md`（日本語は `query.ja.md`）。書き直した文書: `query.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-156 | C-core-query-1, C-core-query-2, C-core-query-3, C-core-query-4 | query は ID を1つ受け、その ID の決定か確認することの行を、list の1件に本文を足して出す | 読み替え。IR の項目とシナリオを、決定と確認することの行に。指摘を出さないこと、終了コード0、誤りがあっても読めたものを出すこと、同じ ID を全部出すことはそのまま（query-status の A1、A4、A6、A19、review6-gaps の A1。利用者） |
| REQ-core-157 | C-core-query-5, C-core-query-6, C-core-query-7 | 位置引数がちょうど1つの ID でないか、その ID が無ければ、引数の誤りで停止する | 読み替え。「ID の形」を、確認することの ID の形か、決定を指す ID の形に（今のコード）。"unknown id: " の詳細はそのまま（A5、A6。利用者） |
| REQ-core-158 | C-core-query-8, C-core-query-9, C-core-query-10, C-core-query-13 | query は check と同じ条件で停止し、ガイドと面は読まない | そのまま。全体像の元データを読まないこと（whole-picture の A73、LLM）は、安全に関わる細部として決定に戻した（C-13） |
| REQ-core-159 | C-core-query-1, C-core-query-2, C-core-query-14, C-core-query-15 | query は ID を1つ受け、その ID の決定か確認することの行を、list の1件に本文を足して出す | 読み替え。中身は TBL-core-027 の行を参照 |
| REQ-core-160 | C-core-query-16 | 決定の逆引きには、その決定の見出しへリンクしている決定を入れる | 読み替え。"referenced_by" の並び（query-status の A15。利用者）を、見出しへリンクしている決定の並びとして残した（提案）。"items" の並びが list と同じこと（A20 の (1)、主セッション）は落とした細部 |
| REQ-core-161 | C-core-query-11, C-core-query-12 | 出力は json と text の2つにし、text は list の行に本文の行を続ける | 読み替え。"text" の最後の "  <- ID via パス:行" の行は、"via" が IR の行と文にしか意味を持たないので、逆引きを "text" にどう出すかは実装する側が決める細部にした |
| TBL-core-027 | C-core-query-1, C-core-query-2, C-core-query-14, C-core-query-15 | query は ID を1つ受け、その ID の決定か確認することの行を、list の1件に本文を足して出す | 読み替え（一部）。"body" は残す（A2。利用者）。確認することの行の本文はその行の文字（今のコード）。決定の本文は決定の全体（利用者の答え。C-14、提案）。"referenced_by" は見出しへリンクしている決定（利用者の答え。C-15、提案）。IR の項目の本文の範囲（見出しの次の行から次の同じ深さの見出しの前まで、先頭と末尾の空行を落とす、コードブロックの中の見出しの形の行で切らない、シナリオはタグの行から）と、"via" の値（"definition"、"relations"、"about"、"text"）は消える（IR の項目の形） |
| EX-core-287 | - | - | 消える。"via" の "text"（文の中のバッククォートで囲んだ ID）は IR の文にしかない。ID を文に書いてほかの項目を指す形は、decision-documents.md の「ほかの決定を指すときは、見出しへリンクを張る」に置き換わった。逆引きを読み替えるなら、その決定の中で数え方を決める |
| EX-core-250 | C-core-query-1, C-core-query-15 | query は ID を1つ受け、その ID の決定か確認することの行を、list の1件に本文を足して出す | 読み替え。本文が付くことは残る。逆引きがあることは C-15（提案）に読み替える。逆引きの "about"（シナリオの "@about"）は消える |
| EX-core-251 | C-core-query-7 | 位置引数がちょうど1つの ID でないか、その ID が無ければ、引数の誤りで停止する | そのまま |
| EX-core-252 | C-core-query-5 | 位置引数がちょうど1つの ID でないか、その ID が無ければ、引数の誤りで停止する | そのまま |
| EX-core-253 | C-core-query-3 | query は ID を1つ受け、その ID の決定か確認することの行を、list の1件に本文を足して出す | 読み替え。同じ ID の要求を、同じ ID の確認することの行に |
| EX-core-254 | C-core-query-12 | 出力は json と text の2つにし、text は list の行に本文の行を続ける | 読み替え。最後の "<-" の行の形は実装する側が決める細部 |
| EX-core-255 | - | - | 消える。シナリオの本文（"@id" のタグの行から）。場面は、decision-documents.md の「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」で確認することの行に置き換わった |
| EX-core-256 | C-core-query-8 | query は check と同じ条件で停止し、ガイドと面は読まない | そのまま |
| EX-core-257 | - | - | 消える。"definition" と "text" の逆引きは IR の行と文にしかない。決定表は、decision-documents.md の「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」で決めたことの中の表に、ID での参照は同じ文書の「ほかの決定を指すときは、見出しへリンクを張る」に置き換わった |
| EX-core-277 | - | 決定を引いたときの本文は、決定の全体にする | 消える。IR の項目の本文の範囲。IR の項目は、decision-documents.md の「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」の決定に置き換わった。決定の本文の範囲は C-14（提案）で決めた |
| EX-core-280 | - | 決定を引いたときの本文は、決定の全体にする | 消える。IR の項目の本文の範囲（コードブロックの中の見出し）。置き換え先は EX-core-277 と同じ。今のコードは決定の文書を Markdown の構文木で読むので、同じ問題は起きない |

元の文書に PROP は無い。

## テストごとの表

「IR で準備」と書いたテストは、IR の項目で準備して、形に依らない振る舞いを確かめている。古い形のコードを消すときに、準備を決定の文書に書き直す（下の「テストの無い行」）。決定の文書で確かめている step23 のテストは、次の「マーカーに足すテスト」に分けた。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step11_query.rs | tbl_027_an_item_citing_an_id_twice_is_one_reference | TBL-core-027, EX-core-287 | - | "via" の "text" の逆引きだけを確かめる。古い形のコードと一緒に消す |
| tests/step11_query.rs | req_156_item_has_body_and_referenced_by | REQ-core-156, REQ-core-159, TBL-core-027, EX-core-250 | C-core-query-1, C-core-query-15 | IR で準備。1件の鍵が list の1件に "body" と "referenced_by" を足したものであることを確かめる。鍵の並びは決定と確認することの行の鍵に直す。"referenced_by" の中身は、見出しへリンクしている決定（C-15、提案）に直す。実装のときに直す |
| tests/step11_query.rs | req_156_duplicate_ids_are_all_listed | REQ-core-156, EX-core-253 | C-core-query-3 | IR で準備 |
| tests/step11_query.rs | req_156_items_are_listed_despite_ir_errors_and_exit_zero | REQ-core-156 | C-core-query-4 | IR で準備（"- verification:" の行の無い要求） |
| tests/step11_query.rs | req_157_unknown_id_stops | REQ-core-157, TBL-core-020, EX-core-251 | C-core-query-7 | この話題の分だけ。IR で準備。TBL-core-020 は CLI の環境の話題 |
| tests/step11_query.rs | tbl_027_scenario_body_starts_at_the_tag_line | TBL-core-027, EX-core-255 | - | シナリオの本文だけを確かめる。古い形のコードと一緒に消す |
| tests/step11_query.rs | tbl_027_definition_and_text_references_are_listed | TBL-core-027, REQ-core-054, EX-core-257 | - | IR の逆引きだけを確かめる。古い形のコードと一緒に消す（REQ-core-054 はほかの話題） |
| tests/step11_query.rs | tbl_027_text_reference_is_only_the_backticked_id | TBL-core-027, REQ-core-054 | - | IR の逆引きだけを確かめる。古い形のコードと一緒に消す |
| tests/step11_query.rs | tbl_027_body_stops_before_the_next_heading | TBL-core-027 | - | IR の項目の本文の範囲だけを確かめる。古い形のコードと一緒に消す |
| tests/step11_query.rs | req_160_referenced_by_is_ordered_by_path_then_line | REQ-core-160 | C-core-query-16 | "referenced_by" の並びを確かめる。今は IR の項目からの逆引きで準備している。逆引きを、見出しへリンクしている決定に読み替えた（C-16、提案）ので、実装のときに直す |
| tests/step11_query.rs | req_161_json_top_level_has_only_items | REQ-core-161 | C-core-query-11 | IR で準備 |
| tests/step11_query.rs | req_161_text_prints_body_and_referenced_by_lines | REQ-core-161, REQ-core-155 | C-core-query-12 | この話題の分（REQ-core-161）だけ。IR で準備。最後の "<-" の行は、逆引きを "text" にどう出すか（実装する側が決める細部）に合わせて直す。REQ-core-155 は list の対応表 |
| tests/step11_query.rs | ex_core_277_body_ends_before_the_next_heading_and_drops_trailing_blank_lines | EX-core-277 | - | IR の項目の本文の範囲だけを確かめる。古い形のコードと一緒に消す |
| tests/step11_query.rs | ex_core_280_a_heading_shaped_line_inside_a_code_block_does_not_end_the_body | EX-core-280 | - | IR の項目の本文の範囲だけを確かめる。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | tbl_027_query_carries_deferred_in_json_and_on_the_first_text_line | TBL-core-027, REQ-core-161 | - | 後回しだけを確かめる。古い形のコードと一緒に消す |
| tests/step15_guides.rs | req_152_list_and_query_do_not_read_guides_and_do_not_stop_on_their_overlap | REQ-core-152, REQ-core-158 | C-core-query-9 | この話題の分（REQ-core-158）だけ。IR で準備（"REQ-001" を引く）。REQ-core-152 は list の対応表 |
| tests/step18_overview.rs | req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it | REQ-core-152, REQ-core-158 | C-core-query-9, C-core-query-13 | この話題の分だけ。全体像の元データとガイドの置き場が同じ glob で重なり、そこのファイルが UTF-8 でなくても query が終了コード0で終わる。全体像の元データを読まないこと（C-13）と、ガイドを読まないこと（C-9）の両方を確かめている。IR で準備（"REQ-001" を引く） |
| tests/step17_surface.rs | req_229_list_and_query_do_not_read_the_surface_list_or_files | REQ-core-229, REQ-core-152, REQ-core-158 | C-core-query-10 | この話題の分だけ。IR で準備 |
| tests/step1_cli.rs | req_157_two_positional_arguments_stop | REQ-core-157, EX-core-252 | C-core-query-5 | そのまま |
| tests/step1_cli.rs | req_157_query_with_no_positional_stops | REQ-core-157 | C-core-query-5 | そのまま |
| tests/step1_cli.rs | req_157_positional_that_is_not_an_id_stops | REQ-core-157, REQ-core-124 | C-core-query-6 | この話題の分だけ。"REQ-01" は新しい形でも ID の形でない。REQ-core-124 は IR の参照の話題 |
| tests/step1_cli.rs | req_158_unreadable_config_stops_like_check | REQ-core-158, EX-core-256 | C-core-query-8 | そのまま |
| tests/step1_config.rs | req_003_list_query_and_status_read_the_config_given_by_config | REQ-core-003, REQ-core-152, REQ-core-158, REQ-core-163 | C-core-query-1 | この話題の分（REQ-core-158）だけ。"--config" の設定の置き場から1件を読むことを確かめる。IR で準備 |

## マーカーに足すテスト

この話題の ID を今は指していないが、付け替えたあとの確認することの行を決定の文書で確かめているテスト。今のマーカー（別の話題の ID）は残したまま、この話題の ID を足す。同じテストがほかの話題の対応表にも載っているときは、それぞれの表の ID を合わせて足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 足す ID | 理由 |
|---|---|---|---|---|
| tests/step23_decision_documents.rs | query_of_a_confirmation_id_returns_the_file_line_and_name_of_each_test_pointing_at_it | C-core-confirm-5 | C-core-query-1, C-core-query-11 | list の対応表の ID も足す。1件の全体と、最上位が "items" だけであることを確かめている |
| tests/step23_decision_documents.rs | query_of_a_confirmation_id_prints_its_tests_in_text | C-core-confirm-5 | C-core-query-12 | list の対応表の ID も足す |
| tests/step23_decision_documents.rs | query_of_a_confirmation_id_without_tests_returns_an_empty_list_of_tests | C-core-confirm-5 | C-core-query-1 | テストの無い行の "tests" が空の並びで返る |
| tests/step23_decision_documents.rs | query_of_a_confirmation_id_in_no_document_stops_as_an_unknown_id | C-core-confirm-5 | C-core-query-7 | どの文書にも無い確認することの ID で、"unknown id: " の停止になる |
| tests/step23_decision_documents.rs | query_of_a_wrapped_confirmation_row_joins_its_name_and_keeps_its_lines_as_the_body | C-core-confirm-5 | C-core-query-1 | 折り返した行の本文を確かめている |
| tests/step23_decision_documents.rs | a_row_of_a_decision_with_a_wrong_state_line_has_no_state | C-core-confirm-5 | C-core-query-4 | 状態の行の誤り（check の誤り）があっても、終了コード0で1件が返る |
| tests/step23_decision_documents.rs | query_of_a_decision_returns_the_fingerprint_a_guide_mark_takes | C-core-lang-5 | C-core-query-2, C-core-query-11, C-core-query-14 | list と guides の対応表の ID も足す。"body" が指紋を取る行だけであることも確かめているので、決定の全体を返す（C-14、提案）ときに直す。実装のときに直す |

## 落とした細部

### 安全に関わるもの

無し（あった1件は、下の「戻した細部」のとおり決定に戻した）。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-158: "kotowari query" は全体像の元データを読まず、その置き場と読み込みによる停止（REQ-core-278、REQ-core-280）をしない。記録では LLM（whole-picture の A73。承認の時に利用者に示す） | C-core-query-13（決定「query は check と同じ条件で停止し、ガイドと面は読まない」） | テスト tests/step18_overview.rs の req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it（"overview.files" と "guides.files" を同じ glob にし、そこに UTF-8 でないファイルを置いて、query が終了コード0で終わることを確かめる。IR の "REQ-001" を引くので、準備の書き直しが要る。下の「テストの無い行」） |

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-160: "items" の並びを list と同じにすること | 主セッション（query-status の A20 の (1)。承認時に提示）。今のコードはそう並べる |
| REQ-core-161、TBL-core-027: 固定の語の英語に当たる値（"<-"、"via" の値） | 利用者（query-status の A13）。"via" の値は IR の行・タグ・文の区別で、新しい形では消える。逆引きを "text" にどう出すかは「実装する側が決める細部」に挙げた |
| REQ-core-156、REQ-core-158、REQ-core-161: "--help" か "--version" があればほかの引数を見ない（records.md の A136） | 記録に decided_by が無い。落としたのではなく、使い方の表示の話題の決定へのリンクにした |
| TBL-core-027: 同じ項目が同じ ID を同じ "via" で何度指しても1件にすること | review6-gaps の A2。decided_by は「利用者（推奨を採用）」だが、記録の Context に「ほぼ答えが決まっているものは推奨で決めてよい」とした範囲で決めたとあるので、agent の細部とした。同じ決定から何度もリンクされたときの数え方は「実装する側が決める細部」に挙げた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| TBL-core-027 | 読み替えて残る（一部）。"body" は残る。決定の本文は決定の全体に（C-14、提案）。"referenced_by" は、見出しへリンクしている決定に読み替えて残る（C-15、提案）。IR の項目とシナリオの本文の範囲と、"via" の値（"definition"、"relations"、"about"、"text"）は、IR の行・タグ・文を前提にしているので消える |
| REQ-core-159 | TBL-core-027 と同じ |
| REQ-core-160 | 読み替えて残る。"referenced_by" の並びを、見出しへリンクしている決定の並びに（C-16、提案） |
| REQ-core-157 | 読み替えて残る。ID の形を、IR の ID（REQ-core-124）から、確認することの ID の形か決定を指す ID の形に |
| EX-core-255、EX-core-257、EX-core-277、EX-core-280、EX-core-287 | 消える |

## テストの無い行

| 行 | 要るテスト |
|---|---|
| C-core-query-3 | 同じ ID の確認することの行が2つの決定の文書にあるとき、"kotowari query" がその2件を返し、終了コードが0であること（今は req_156_duplicate_ids_are_all_listed が IR で確かめている） |
| C-core-query-9 | 決定の文書で準備した設定で、ガイドのファイルが UTF-8 でないときと、ガイドの置き場がテストのファイルの置き場と重なるときに、query が停止しないこと（今のテストは IR の "REQ-001" を引く） |
| C-core-query-10 | 決定の文書で準備した面の設定で、query が面の一覧と面のファイルを読まないこと（今のテストは IR で準備） |
| C-core-query-13 | 決定の文書で準備した設定で、"overview.files" と "guides.files" を同じ glob にし、そこに UTF-8 でないファイルを置いても、決定か確認することの ID を引く query が終了コード0で終わること（今の req_core_152_list_and_query_neither_read_overview_data_nor_stop_on_it は IR の "REQ-001" を引く。古い形のコードを消すと unknown id で止まる） |

## 誰が決めたか見分けられなかったもの

- records.md の A136（"--help" と "--version" がほかの引数に勝つ）: decided_by の行が無く、「敵対的レビューの指摘」とだけある。この文書では書かず、使い方の表示の話題の決定へリンクした。

## 理由や見送った案が記録に無かった決定

- 「位置引数がちょうど1つの ID でないか、その ID が無ければ、引数の誤りで停止する」: 見送った案が記録に無い。
- 「query は check と同じ条件で停止し、ガイドと面は読まない」: 見送った案が記録に無い。ガイドと面を要らない理由は、doc-marks の A22 と surface-check の A14 の why にあり、指紋がガイドから決まらないことと、面を読むのが check と status であることだけを書いた。全体像の元データを読まない理由は記録に無い（A73 の why は敵対的レビューの指摘から導いたとだけある）ので、「理由は記録に残っていない」と書いた。
- 「出力は json と text の2つにし、text は list の行に本文の行を続ける」: 見送った案が記録に無い。
- 「決定を引いたときの本文は、決定の全体にする」: 決定そのものは利用者の答え（確定の段）。理由は query-status の A2 の why（LLM が1件を読むときに文書を開き直さなくて済む）を当てた。見送った案は無い。
- 「決定の逆引きには、その決定の見出しへリンクしている決定を入れる」: 決定そのものは利用者の答え（確定の段）。理由は query-status の A3 の why と read-commands の A14 の why（片方向で十分。逆引きは query で行う）を当て、並びは A15（利用者）を残した。見送った案は無い。

## 分けた所、まとめた所

- REQ-core-156、REQ-core-159、REQ-core-160、TBL-core-027 のうち実装済みの部分を、1つの決定「query は ID を1つ受け、その ID の決定か確認することの行を、list の1件に本文を足して出す」にまとめた。
- 利用者の答えで今のコードと違うことになった2つを、実装済みの決定から分けて「提案」の決定にした: 決定の本文の範囲（「決定を引いたときの本文は、決定の全体にする」）と、逆引き（「決定の逆引きには、その決定の見出しへリンクしている決定を入れる」）。理由と確認することがそれぞれ違うため。
- 固定の語を英語にすること（query-status の A13）は、query と status の両方の出力の決定に書いた。どちらにも、その出力の語として出てくるため。
- 確認することの ID を渡すとテストが返ることは confirmations.md の C-core-confirm-5 にあるので、ここでは書かず、冒頭の段落からリンクした。

## あとでリンクにする所

- 確定の段で、confirmations.md の「check は、確認することの行ごとに、それを指すテストがあるかを数える」（冒頭の段落）と、decision-documents.md の「ほかの決定を指すときは、見出しへリンクを張る」（逆引きの決定の問題）を、`*.ja.md` の見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した
- 残り: 無し

## 実装する側が決める細部

前の3つは今のコードにもうある。あとの5つは、提案の決定を作るときに決める。

- 確認することの行の "text" の最後の "  decision <状態> <見出し>" の行（今のコード。状態が無ければ "-"）
- 決定を指す ID の書き方（list の対応表と同じ）
- 確認することの行の本文が、折り返した行の字下げを含めて生の行であること（今のコード）
- 決定の本文に、状態の行と見出しの行を含めるか。ガイドの指紋を取る行を別に見せるなら、その鍵の名前（今のコードの "body" は指紋を取る行だけ）
- "referenced_by" の1件の形（今のコードは "id"、"kind"、"path"、"line"、"via" を持つ。"via" は IR の区別なので、残すか、残すなら値をどうするか）
- 同じ決定が、引いた決定の見出しへ2回以上リンクしているときに、1件にするか
- 確認することの行の "referenced_by"（利用者の答えは決定の逆引きだけを決めた。今のコードは空の並び）
- "text" で逆引きをどう出すか（元の "  <- ID via パス:行" の行に当たるもの）

## 利用者に聞くこと

無し（下書きの2つの問いには答えが出た。下の「迷った判断」）。

## 「提案」で書いた決定の一覧

- 「決定を引いたときの本文は、決定の全体にする」: `crates/kotowari-core/src/decisions.rs` の `query_items` が決定の1件の `body` に `decision.fingerprinted` を入れているので、決定の全体の行（`Decision` に範囲を持たせる）を入れる。指紋の計算（`fingerprinted`）は変えない。
- 「決定の逆引きには、その決定の見出しへリンクしている決定を入れる」: 同じ `query_items` が `referenced_by` にいつも空の並びを入れているので、決定の文書の中のリンク（`decisions.rs` がリンク切れの検査のために読んでいる行き先）から、引いた決定の行き先を指すものを集めて並べる。`crates/kotowari-core/src/query.rs` の `Reference` の形と "text" の出し方も合わせて直す。

## 迷った判断と、どう決めたか

- 利用者の答え（逆引き）: 逆引きには、その決定の見出しへリンクしている決定を入れる。今のコードと違うので「提案」にした（C-15、C-16）。
- 利用者の答え（決定の本文）: 決定の全体（問題、決めたこと、見送った案、確認すること）を返す。今のコードと違うので「提案」にした（C-14）。guides の「古い節は、マーカーに書いた指紋を今の決定の指紋と照らして見つける」は、query が「指紋を取る行」を返すと書いていたので、guides の下書きから本文の範囲の文を外した（guides の対応表）。
- "referenced_by"、"decision"、"state" を決定の文に書いても書かなくても、面の検査には当たらない。このリポジトリの面の規則（`.kotowari/rules/surface.yml`）が取り出すのはサブコマンドとフラグだけで、JSON の鍵は面ではない。文に出てくるフラグ（"--format"、"--config"、"--help"、"--version"、"--tool"）は、どれも引用した。
- 全体像の元データを読まないこと（A73、LLM）は、下書きでは落としたが、確定の段で「安全に関わる細部は、今のテストか skill の文が確かめているなら決定に戻す」という利用者の決定に従って戻した（C-13）。今のテストは IR で準備しているので、「テストの無い行」にも挙げた。list の側も同じに扱った。
- REQ-core-160 の「"items" の並びは list と同じ」は query-status の A20 (1)（主セッション）なので、status の A20 の (2)、(4)、(5) と同じく落とした。A15（利用者）が決めた逆引きの並びは、逆引きの決定に残した。
- 同じ ID の行を全部出すことは query-status の A6（利用者）の決定で、理由には A6 の why（重複は check の指摘で、query が隠す理由は無い）と、review6-gaps の A1 の why（重複を直す人が全部を見られる）を書いた。review6-gaps の A1 と A2 は、どちらも細部を任せた承認の範囲で決めたもの（agent の細部）なので、振る舞いとしては A2（同じ via の重なりを1件にする）を落とし、A1 は A6 が決めた振る舞いの理由の補足としてだけ使った。

## 数

- 元の文書: 要求6、場面11、決定表1、性質0（ID は合わせて18）
- 読んだ記録のファイル: 9（2026-09-20-query-status、records.md、2026-09-19-read-commands、2026-09-24-doc-marks、2026-09-27-surface-check、2026-10-02-whole-picture、2026-09-23-ir-engine-gaps、2026-09-23-ir-english-tokens、2026-09-24-review6-gaps）
- 出来上がり: 決定6（実装済み4、提案2）、確認することの行16（読んで確認0）
- テスト: この話題の ID を今指しているもの23本、マーカーに足す step23 のもの7本（「マーカーに足すテスト」の表）
