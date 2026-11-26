# test-markers の対応表

元: `docs/ir/core/test-markers.md`（日本語は `test-markers.ja.md`）。書き直した文書: `test-markers.md`。確認することの ID の短い名前は `markers`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-071 | C-core-markers-1 | マーカーは ID をコンマで並べた形で、行のどこにあってもよい | そのまま。定義の TBL-core-015 を決めたことの表にした |
| REQ-core-072 | C-core-markers-4, C-core-markers-5, C-core-markers-6, C-core-markers-20 | 形の誤ったマーカーには invalid_marker の誤りを出し、マーカーの行を指す | そのまま。空か区切りだけを C-4、閉じ括弧の無いものを C-5、detail を C-6 にした。問い合わせの無い言語の分は C-20（見出しは「問い合わせの無い言語のファイルでは…」） |
| REQ-core-073 | C-core-markers-2 | マーカーは ID をコンマで並べた形で、行のどこにあってもよい | そのまま |
| REQ-core-074 | C-core-markers-3 | マーカーは ID をコンマで並べた形で、行のどこにあってもよい | そのまま |
| REQ-core-075 | C-core-markers-8, C-core-markers-9, C-core-markers-10, C-core-markers-11, C-core-markers-12, C-core-markers-13, C-core-markers-14, C-core-markers-15, C-core-markers-16, C-core-markers-17, C-core-markers-18 | マーカーは、テストの直前のコメントの塊にあるものだけをそのテストに結び付ける | そのまま。定義の TBL-core-016 をこの決定の表に、TBL-core-035 を「直前のコメントの塊は…」の決定にした |
| REQ-core-076 | C-core-markers-19 | 問い合わせの無い言語のファイルでは、ファイルの中のマーカーをすべて拾う | そのまま |
| REQ-core-077 | C-core-markers-21 | どの文書にも無い ID だけを指すマーカーでも、テストはマーカーのあるものと数える | 読み替え。「存在しない ID」を「どの文書にも無い ID」と読んだ。ID の形でない要素を unresolved_reference にする部分（A89）は落とした細部 |
| REQ-core-078 | C-core-markers-22 | 読んで確かめる行を指すマーカーは、誤りにしない | 読み替え。「検証の値 "review" の要求」を「頭に「（読んで確認）」と書いた確認することの行」と読んだ（`docs/decisions/core/confirmations.md` の「テストで確かめない行は、頭に「（読んで確認）」と書く」） |
| REQ-core-118 | C-core-markers-7 | 形の誤ったマーカーには invalid_marker の誤りを出し、マーカーの行を指す | そのまま |
| TBL-core-015 | C-core-markers-1 | マーカーは ID をコンマで並べた形で、行のどこにあってもよい | そのまま。表を決めたことに置いた |
| TBL-core-016 | C-core-markers-8, C-core-markers-9, C-core-markers-10, C-core-markers-11, C-core-markers-12 | マーカーは、テストの直前のコメントの塊にあるものだけをそのテストに結び付ける | そのまま。表を決めたことに置いた |
| TBL-core-035 | C-core-markers-13, C-core-markers-14, C-core-markers-15, C-core-markers-16, C-core-markers-17, C-core-markers-18 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま。導入の文と表を決めたことに置いた |
| EX-core-015 | C-core-markers-2 | マーカーは ID をコンマで並べた形で、行のどこにあってもよい | そのまま |
| EX-core-016 | C-core-markers-13 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |
| EX-core-306 | C-core-markers-8 | マーカーは、テストの直前のコメントの塊にあるものだけをそのテストに結び付ける | そのまま |
| EX-core-307 | C-core-markers-14 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |
| EX-core-308 | C-core-markers-13 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま。クラスの中のメソッドもテストとして見つかることは、同梱の問い合わせの話題の分 |
| EX-core-309 | C-core-markers-16 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |
| EX-core-318 | C-core-markers-14 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |
| EX-core-319 | C-core-markers-10 | マーカーは、テストの直前のコメントの塊にあるものだけをそのテストに結び付ける | そのまま |
| EX-core-320 | C-core-markers-16 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |
| EX-core-323 | C-core-markers-15 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |
| EX-core-324 | C-core-markers-18 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |
| EX-core-325 | C-core-markers-11 | マーカーは、テストの直前のコメントの塊にあるものだけをそのテストに結び付ける | そのまま |
| EX-core-327 | C-core-markers-17 | 直前のコメントの塊は、空行までのコメントの行と、言語ごとの挟んでよい行でできる | そのまま |

元の文書に PROP は無い。

ガイドのマーカー: `docs/guides/marks.md`、`docs/guides/findings.md`（それぞれ `.ja.md` も）にある、この話題の ID のマーカーは、上の「決定の見出し」で付け替える。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/test_facts.rs | req_071_marker_syntax_allows_spaces_around_commas | REQ-core-071, TBL-core-015 | C-core-markers-1 | コンマの前後の空白を許して ID を読む |
| crates/kotowari-core/tests/test_facts.rs | req_073_several_markers_on_one_line | REQ-core-073 | C-core-markers-2 | 1行の複数のマーカー |
| crates/kotowari-core/tests/test_facts.rs | req_074_marker_anywhere_in_the_line_regardless_of_comment_syntax | REQ-core-074 | C-core-markers-3 | コメント記号を見ない |
| crates/kotowari-core/tests/test_facts.rs | req_071_marker_line_number_is_reported | REQ-core-071, TBL-core-015 | C-core-markers-1 | マーカーの行を記録する |
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_macro_function_and_marker_lines_use_additive_offset | REQ-core-082, REQ-core-118, REQ-core-072 | C-core-markers-7 | マクロの中のマーカーの行。REQ-core-082 はテストの見つけ方の話題の分 |
| crates/kotowari-source-analysis/tests/discovery.rs | req_072_invalid_marker_on_second_line_of_multiline_comment_before_test | REQ-core-072 | C-core-markers-4, C-core-markers-7 | 複数行のコメントの2行目の形の誤ったマーカーと、その行 |
| crates/kotowari-source-analysis/tests/discovery.rs | req_072_indented_invalid_marker_before_test_keeps_indentation | REQ-core-072 | C-core-markers-6 | detail が字下げを含む |
| crates/kotowari-source-analysis/tests/discovery.rs | req_072_invalid_marker_line_index_stays_additive_at_boundary | REQ-core-072, TBL-core-035 | C-core-markers-7 | 塊の境目でのマーカーの行 |
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_016_invalid_marker_at_body_start_is_ignored | REQ-core-072, TBL-core-016 | C-core-markers-9 | 本体の先頭の形の誤ったマーカーに指摘が出ない |
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_035_comments_and_multi_line_attributes_form_one_block | REQ-core-075, TBL-core-035 | C-core-markers-14 | コメントと複数行の属性が1つの塊になる |
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_035_blank_line_inside_a_comment_or_an_attribute_does_not_cut_the_block | REQ-core-075, TBL-core-035 | C-core-markers-15 | 途中の空白だけの行で切れない |
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_008_invalid_marker_detail_of_a_crlf_test_file_has_no_carriage_return | REQ-core-072, TBL-core-008, TBL-core-010 | C-core-markers-6 | この話題の分は detail が行の文字であること。"\r" を含めないことは TBL-core-010（行の終わり）の話題の分 |
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_035_comment_after_code_on_the_same_line_breaks_the_block | REQ-core-075, TBL-core-035 | C-core-markers-16 | コードと同じ行のコメントで切れる |
| tests/step4_test_discovery.rs | tbl_010_lone_cr_ends_a_line_of_a_test_file | REQ-core-118, TBL-core-010 | C-core-markers-7 | この話題の分はマーカーの行。単独の "\r" を行の終わりに数えることは TBL-core-010 の話題の分 |
| tests/step4_test_discovery.rs | req_072_invalid_marker | REQ-core-072 | C-core-markers-4 | 空のマーカー |
| tests/step4_test_discovery.rs | req_072_invalid_marker_detail_is_line_text | REQ-core-072, TBL-core-008 | C-core-markers-6 | detail が行の文字。TBL-core-008 は指摘の話題 |
| tests/step4_test_discovery.rs | req_072_invalid_marker_outside_test_is_ignored | REQ-core-072, TBL-core-016 | C-core-markers-9 | テストの外の形の誤ったマーカーに指摘が出ない |
| tests/step4_test_discovery.rs | req_072_invalid_marker_second_on_line_is_detected | REQ-core-072, REQ-core-073 | C-core-markers-4 | 同じ行の2つ目のマーカーの形の誤り |
| tests/step4_test_discovery.rs | req_073_both_markers_on_one_line_bind_to_the_test | REQ-core-073, EX-core-015 | C-core-markers-2 | 両方の ID に結び付く |
| tests/step4_test_discovery.rs | req_075_marker_before_attributes_binds | REQ-core-075, TBL-core-016 | C-core-markers-14 | 属性を挟んで結び付く |
| tests/step4_test_discovery.rs | ex_core_306_marker_at_body_start_does_not_bind | REQ-core-075, TBL-core-016, EX-core-306 | C-core-markers-8 | 本体の先頭で結び付かず test_without_id |
| tests/step4_test_discovery.rs | req_075_body_start_marker_does_not_add_to_the_marks_before | REQ-core-075, TBL-core-016 | C-core-markers-8 | 直前のマーカーに本体の先頭の ID が足されない |
| tests/step4_test_discovery.rs | req_075_marker_in_body_middle_is_ignored | REQ-core-075, TBL-core-016 | C-core-markers-9 | 本体の途中のマーカーを無視する |
| tests/step4_test_discovery.rs | ex_core_324_mark_in_an_attribute_body_is_not_read | REQ-core-075, TBL-core-035, EX-core-324 | C-core-markers-18 | 属性の本体の文字列のマーカーを読まない |
| tests/step4_test_discovery.rs | ex_core_325_marks_bind_only_to_the_first_test_on_the_line | REQ-core-075, TBL-core-016, EX-core-325 | C-core-markers-11 | 1行の2つのテストの最初だけに結び付く |
| tests/step4_test_discovery.rs | tbl_016_marks_bind_only_to_the_first_macro_function_on_the_line | REQ-core-075, TBL-core-016, TBL-core-017 | C-core-markers-11, C-core-markers-12 | マクロの中の関数でも最初だけ。TBL-core-017 は同梱の問い合わせの話題 |
| tests/step4_test_discovery.rs | tbl_016_macro_function_before_a_normal_test_on_the_line_takes_the_marks | REQ-core-075, TBL-core-016, TBL-core-017 | C-core-markers-11, C-core-markers-12 | 同上 |
| tests/step4_test_discovery.rs | req_076_unknown_language_scans_raw_text | REQ-core-076 | C-core-markers-19 | コメントでない行のマーカーも拾う |
| tests/step4_test_discovery.rs | req_076_unknown_language_marker_line_is_one_indexed_from_its_own_line | REQ-core-076 | C-core-markers-19 | 拾ったマーカーの行 |
| tests/step4_test_discovery.rs | req_077_unresolved_only_marker_still_counts | REQ-core-077 | C-core-markers-21 | 無い ID だけを指すマーカーのテストに test_without_id が出ない。準備に IR を使わないので、そのまま残せる |
| tests/step4_test_discovery.rs | req_054_marker_to_known_id_is_not_unresolved | REQ-core-054, REQ-core-077 | - | 既にある ID を指すマーカーに unresolved_reference が出ないことを確かめる。この話題の行（REQ-core-077 の分）には当たらない。REQ-core-054 の分は ID の参照の話題の対応表が C-core-confirm-4 に付け替えるので、マーカーは空にならない |
| tests/step4_test_discovery.rs | req_078_marker_to_review_requirement_is_not_an_error | REQ-core-078 | - | review の要求に requirement_without_test が出ないことだけを確かめ、マーカーを置いていない。消える振る舞いなので、古い形のコードと一緒に消す。C-core-markers-22 のテストは無い（下の「テストの無い行」） |
| tests/step4_test_discovery.rs | req_072_non_query_language_checks_invalid_marker | REQ-core-072, REQ-core-054 | C-core-markers-20 | 問い合わせの無い言語のファイルの形の誤ったマーカー。REQ-core-054 の分は ID の参照の話題 |
| tests/step4_test_discovery.rs | req_077_malformed_id_in_marker_is_unresolved_reference_rs | REQ-core-077 | C-core-confirm-4 | ID の形でない要素（"REQ001"）に unresolved_reference が出る。この部分は落とした細部（A89）で、この話題の行には無い。どの文書にも無い ID を指すマーカーの誤り（`docs/decisions/core/confirmations.md` の C-core-confirm-4）にいちばん近いので、そこへ付け替え、細部はテストの中に残す |
| tests/step4_test_discovery.rs | req_077_malformed_id_in_marker_is_unresolved_reference_non_rs | REQ-core-077 | C-core-confirm-4 | 同上（問い合わせの無い言語） |
| tests/step4_test_discovery.rs | tbl_016_blank_line_between_marker_and_test_breaks_binding | REQ-core-075, TBL-core-016, EX-core-016 | C-core-markers-13 | 空行を挟むと結び付かない |
| tests/step4_test_discovery.rs | tbl_016_blank_line_between_marker_and_function_breaks_binding | REQ-core-075, TBL-core-016 | C-core-markers-13 | 同上 |
| tests/step4_test_discovery.rs | tbl_016_macro_function_boundary_breaks_marker_binding | REQ-core-075, TBL-core-016 | C-core-markers-12 | マクロの中の関数の境目 |
| tests/step4_test_discovery.rs | req_118_unresolved_reference_line_is_the_marker_line | REQ-core-118 | C-core-markers-7 | unresolved_reference の行がマーカーの行 |
| tests/step4_test_discovery.rs | tbl_016_macro_function_body_marker_does_not_bind | TBL-core-016, TBL-core-017 | C-core-markers-8, C-core-markers-12 | マクロの中の関数の本体のマーカーが結び付かない |
| tests/step4_test_discovery.rs | tbl_016_macro_function_block_comment_marker_binds | TBL-core-016 | C-core-markers-12 | マクロの中の関数の直前のブロックコメント |
| tests/step4_test_discovery.rs | req_072_marker_spanning_lines_is_invalid | REQ-core-072 | C-core-markers-5 | 行をまたぐマーカー |
| tests/step4_test_discovery.rs | req_072_detail_is_the_raw_line | REQ-core-072 | C-core-markers-6 | detail が行の文字そのまま |
| tests/step4_test_discovery.rs | req_137_one_marker_may_name_several_scenarios | REQ-core-137, REQ-core-071 | C-core-markers-1 | この話題の分は、1つのマーカーに複数の ID を並べられること。REQ-core-137 の分は消える（coverage の対応表）。準備がシナリオなので、付け替えのときに確認することの行を2つ指すマーカーに直す必要がある |
| tests/step13_test_languages.rs | ex_core_309_code_line_between_breaks_the_block | REQ-core-075, EX-core-309 | C-core-markers-16 | コードの行で切れる |
| tests/step13_test_languages.rs | ex_core_319_mark_before_a_nested_test_binds_to_it | REQ-core-075, EX-core-319 | C-core-markers-10 | 内側のテストに結び付く |
| tests/step13_test_languages.rs | ex_core_327_comment_after_a_full_width_space_joins_the_block | REQ-core-075, TBL-core-035, EX-core-327 | C-core-markers-17 | 全角空白の後のコメント |
| tests/step13_test_languages.rs | ex_core_320_comment_after_code_on_the_same_line_breaks_the_block | REQ-core-075, EX-core-320 | C-core-markers-16 | コードと同じ行のコメントで切れる |
| tests/step13_test_languages.rs | ex_core_307_mark_binds_across_a_decorator | REQ-core-075, TBL-core-035, EX-core-307 | C-core-markers-14 | デコレータを挟んで結び付く |
| tests/step13_test_languages.rs | ex_core_308_mark_before_a_method_binds | REQ-core-075, TBL-core-035, EX-core-308 | C-core-markers-13 | クラスの中のメソッドの直前のコメント |
| tests/step13_test_languages.rs | ex_core_318_mark_binds_across_a_multi_line_decorator_and_a_comment | REQ-core-075, TBL-core-035, EX-core-318 | C-core-markers-14 | 複数行のデコレータとコメントを挟んで結び付く |
| tests/step13_test_languages.rs | ex_core_323_blank_line_inside_a_multi_line_decorator_does_not_cut_the_block | REQ-core-075, TBL-core-035, EX-core-323 | C-core-markers-15 | デコレータの途中の空白だけの行で切れない |
| tests/step13_test_languages.rs | tbl_034_mark_before_a_docblock_test_binds | REQ-core-075, TBL-core-034 | C-core-markers-13 | PHP のドキュメントコメントの直前のマーカー。TBL-core-034 は同梱の問い合わせの話題 |

## 落とした細部

### 安全に関わるもの

無し。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-077: マーカーの角括弧の中の ID の形でない要素（"REQ001" など）は、存在しない ID を指したものとして unresolved_reference にする | 実験者の裁定（records.md の A89。cycle の判断待ちを A74 に従って裁定した）。テストは C-core-confirm-4 に付け替えて残す |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-077 | 新しい形で読み替えて残る。「存在しない ID」を「どの文書にも無い ID」と読んだ。今のコードは、確認することの ID も IR の ID と同じ一覧に入れてマーカーを照らす（`crates/kotowari-core/src/inputs/read.rs`） |
| REQ-core-078 | 新しい形で読み替えて残る。「検証の値 "review" の要求」を「（読んで確認）の行」と読んだ。今のコードは、確認することの行の ID をどれも、マーカーが指してよい ID にしている（`crates/kotowari-core/src/decisions.rs` の `known_ids`）ので、もうそう動いている（コードを読んで確かめた） |

### テストの無い行

- C-core-markers-22: 実装済みの決定に「（読んで確認）」の行を置き、その ID を指すマーカーを持つテストを置いて、check が unresolved_reference などの誤りを出さないことを確かめるテストが要る。今の req_078_marker_to_review_requirement_is_not_an_error は、マーカーを置いておらず、この行を確かめていない。

## 誰が決めたか見分けられなかったもの

無し。records.md の決定は、test-discovery の対応表と同じく、第1〜第3ラウンドで利用者が決めたものと推奨をそのまま採ったもの（A5、A14、A24）、第6〜第9ラウンドで利用者が答えたもの（A39、A57、A67、A111、A121）を利用者の決定とし、A74 による実験者の裁定（A89）を agent の細部とした。2026-09-24-multi-language-tests.md の A15〜A17、A26、A39〜A41、A49〜A52 と、2026-09-24-review10-gaps.md の A1 は「利用者（推奨を採用）」。review10-gaps は、利用者が「ほぼ答えが決まっているものは推奨で決めてよい」とした範囲で決めたと書いてあるが、決定ごとに「利用者（推奨を採用）」とあるので、利用者の決定とした。

## 理由や見送った案が記録に無かった決定

- 「マーカーは ID をコンマで並べた形で…」: 見送った案が記録に無い。
- 「形の誤ったマーカーには invalid_marker の誤りを出し…」: 理由も見送った案も記録に無い（records.md の A57、A111、A121 に why が無い）。問題の節に「理由は残っていない」と書いた。
- 「直前のコメントの塊は…」: 見送った案が記録に無い。
- 「問い合わせの無い言語のファイルでは…」: 理由も見送った案も記録に無い（A39、A57）。
- 「どの文書にも無い ID だけを指すマーカーでも…」: 理由も見送った案も記録に無い（A57）。
- 「読んで確かめる行を指すマーカーは、誤りにしない」: 理由も見送った案も記録に無い（A39）。

見送った案のうち、この話題の決定の代わりでないものは書かなかった: 2026-09-24-multi-language-tests.md の A33（Rust だけ手で辿る形を残す）と A34（同梱のルールだけにする）は、テストの見つけ方と問い合わせの話題の案。records.md の R3（設定の正規表現でテストの定義の行を見つける）と、A16 の「印の後ろ5行以内の定義の行に結び付ける」（A24 で改めた）は、テストの見つけ方の話題の以前の決定。

## 分けた所、まとめた所

- REQ-core-071、REQ-core-073、REQ-core-074、TBL-core-015 を1つの決定にまとめた。どれもマーカーの形と拾い方を決めている。
- REQ-core-072 と REQ-core-118 を1つの決定にまとめた。どれも形の誤ったマーカーの指摘と、その行を決めている。REQ-core-072 の「問い合わせのある言語でどのテストの直前のコメントの塊にも無いものを除く」は、結び付けの決定の表（無視し、invalid_marker も出さない）と、この決定の最後の段落の両方に置いた。
- REQ-core-075 の定義の2つの表を、2つの決定に分けた。TBL-core-016（どの位置のマーカーをどう扱うか）は本体の先頭をやめた理由（A15）と見送った案を持ち、TBL-core-035（塊の範囲）は別の理由（A16、A17、A40、A50、A39、A51、review10 の A1）を持つため。
- 値違いの場面（EX-core-309 と EX-core-320、EX-core-307 と EX-core-318）は、それぞれ1行にまとめた。

## あとでリンクにする所

確定の段で、confirmations.md の「check は、確認することの行ごとに、それを指すテストがあるかを数える」（冒頭の段落と「どの文書にも無い ID だけを指すマーカーでも…」の、無い ID を指すマーカーの誤り。C-core-confirm-4）を `confirmations.ja.md` の見出しへのリンクにした。下書きどうしのリンク（test-discovery、coverage）も `<file>.ja.md#<anchor>` に直した。test-discovery の下書きの、この話題を言葉で指していた所（冒頭の段落と「問い合わせが1つも無い言語のファイルは構文木を読まない」）は、test-discovery の確定でこの話題の見出しへのリンクになった。

残り: 「読んで確かめる行を指すマーカーは、誤りにしない」の「（読んで確認）」の行は、文の中で別の決定を指していないので、リンクにする所は無い。

## 実装する側が決める細部

無し。

## 利用者に聞くこと

無し。

## 「提案」で書いた決定の一覧

無し。7つの決定とも、今のコードがもう新しい形で動いている。切り替えで要る作業は、マーカーが指してよい ID から IR の ID を外すこと（`crates/kotowari-core/src/inputs/read.rs` の `collect_known_ids` の部分）と、上の「準備を直す」テストの準備の書き直しである。

## 迷った判断と、どう決めたか

- A89（ID の形でない要素を unresolved_reference にする）は実験者の裁定なので落とした。それを確かめる2つのテストは、どの文書にも無い ID を指すマーカーの誤り（C-core-confirm-4）に付け替えた。テストにマーカーが無いと test_without_id になるため、近い行へ付け替えて細部をテストの中に残す、描画の話題の対応表と同じ扱いにした。
- REQ-core-078 の読み替えは、「（読んで確認）」の行が検証の値 "review" の役目を引き継ぐ（`docs/decisions/core/confirmations.md` の「テストで確かめない行は、頭に「（読んで確認）」と書く」の見送った案に以前の決定として書いてある）ことを根拠にした。
- 本体の先頭の書き方の理由の中にあった、PHP のコーディング規約の名前は、決まりどおり中身（関数の次の行に "{" を置く書き方）だけを書いた。
- C-core-markers-22（「（読んで確認）」の行を指すマーカーを誤りにしない）は、確かめるテストが無いまま「テストの無い行」に残した。
- C-core-markers-9 は、TBL-core-016 の2つの行（テストの節の中、どのテストの直前にも無い）が同じ扱い（無視し、指摘を出さない）なので1行にまとめた。

## 数

- 元: 要求 9、場面 14、決定表 3、性質 0
- 読んだ記録のファイル: 3（records.md の A5、A14、A24、A26、A34、A39、A57、A67、A89、A111、A121、2026-09-24-multi-language-tests.md、2026-09-24-review10-gaps.md）
- 出来上がり: 決定 7、確認することの行 22（うち「（読んで確認）」0）
- テスト: 53（この話題の ID を指すもの。テストごとの表の行と、今のマーカーの grep が合う）
