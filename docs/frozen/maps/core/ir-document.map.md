# ir-document の対応表

元: `docs/ir/core/ir-document.md`（日本語は `ir-document.ja.md`）。書き直した文書: `ir-document.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-033 | C-core-document-1, C-core-document-2, C-core-document-3, C-core-document-4, C-core-document-13 | 決定の文書の置き場は深さに制限なく辿り、小文字の .md のファイルだけを読む | 読み替え（IR の置き場を決定の文書の置き場と読む）。決定の文書を読むコードは IR と同じ走査の関数を使っているので、もうそう動いている（実装済み）。深さに制限を付けないことは 2026-09-16-ir-tree.md の A1（利用者）、除外がどの深さでも同じに働くことは同じ記録の A13（提示して異論なし）、".MD" と隠しディレクトリとディレクトリのシンボリックリンクを読まずファイルのシンボリックリンクを読むこと、種類を取れない要素で停止することは records.md の A102（第9ラウンド、利用者が推奨をまとめて採用）。ソケット、名前付きパイプ、デバイスを読まないこと（A165、実験者の裁定）は安全に関わる細部として決定に戻した（C-13）。"CONTEXT.md" が用語集であることは `decision-documents.ja.md` の「用語集は人が読む文書として残し、用語の検査はやめる」（提案）に置き換わる。"FLAGS.md" は消える（`decision-documents.ja.md` の「人に戻す問題は、話題の文書の「未決」の節に書く」）。対の側の "CONTEXT.<言語タグ>.md" は translation-pairs の話題 |
| REQ-core-034 | - | 決定の文書の題名は1つにし、2つ目からの題名は multiple_titles の誤りにする | 読み替え済み。題名の無い文書への誤りは `decision-documents.ja.md` の C-core-docs-8 と findings の「題名の無い決定の文書の missing_title は、detail を文書名にする」にある。この文書の決定からはリンクで指した |
| REQ-core-035 | C-core-document-5 | 決定の文書の題名は1つにし、2つ目からの題名は multiple_titles の誤りにする | 読み替え（実装済み）。records.md の A42（第6ラウンド、利用者の答え「推奨で」）と 2026-09-23-ir-engine-gaps.md の A7（利用者）。決定の文書を読むコードは 539b7c1 で2つ目からの題名を multiple_titles にした（`crates/kotowari-core/src/decisions.rs` の `extra_titles`） |
| REQ-core-036 | - | 題名のあとの段落は、あるかだけを見て、中身と行数と責務の分離は見ない | 消える。範囲の行の検査は `decision-documents.ja.md` の「1つの決定は、問題・決めたこと・見送った案・確認することの文章で書く」の、題名のあとの段落の検査（topic_lead_missing、C-core-docs-8）に置き換わる |
| REQ-core-037 | C-core-document-7, C-core-document-8, C-core-document-9, C-core-document-10, C-core-document-14 | kotowari は、"\n"、"\r\n"、単独の "\r" を、それぞれ1つの行の終わりに数える | そのまま（テストのファイル、変異の結果のソース）と読み替え（IR の文書を決定の文書と読む）。決定の文書の確認することの本文と指紋と状態の行は、d6ecb63 で単独の "\r" でも分けて読むようになったので、その決定（C-10、C-14）も実装済みにした |
| REQ-core-038 | C-core-document-11 | 行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす | 消える（利用者の答え: 行数の上限の注意は決定の文書ではやめる）。古い形のコードがまだ too_many_lines を出すので、「無くす」決定を「提案」で書いた。records.md の A17、A29、A41 の上限の注意は、この答えで取り下げ |
| REQ-core-039 | C-core-document-11 | 行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす | 消える（利用者の答え: too_many_requirements と "limits.requirements" も無くす）。同じ「無くす」決定に入れた |
| REQ-core-040 | C-core-document-12, C-core-document-15 | 決定の文書のコードブロックの中は、決定の形としても引用としても読まない | 読み替え。records.md の A52（第7ラウンド、利用者の答え「推奨で問題無いと思う」）の「コードブロックの中は検査の対象外」を残した。確認することの行の下のコードブロックを行の名前に入れないこと（4688af9）は、この読み替えとして C-15 にした。gherkin のブロックを読む例外と、文書名の参照の扱い（A88、実験者の裁定）は消える。面の引用に数えない分は surface の C-core-surface-14 |
| REQ-core-041 | C-core-document-6 | 題名のあとの段落は、あるかだけを見て、中身と行数と責務の分離は見ない | 読み替え（範囲の行を、題名のあとの段落と読む）。records.md の A17（利用者）、A30。A17 の「行数が設定の値を超えたら警告」は、行数の上限の注意をやめる利用者の答えで取り下げ |
| TBL-core-010 | C-core-document-7, C-core-document-8, C-core-document-9, C-core-document-10 | kotowari は、"\n"、"\r\n"、単独の "\r" を、それぞれ1つの行の終わりに数える | そのまま。2026-09-23-mutants-gaps.md の A1、2026-09-24-review3-gaps.md の A3（利用者）。判断の記録と ADR の分は消える |
| EX-core-006 | - | 決定の文書の置き場は深さに制限なく辿り、小文字の .md のファイルだけを読む | 消える。範囲の行が無くなる。用語集を話題の文書として読まないことは `decision-documents.ja.md` の C-core-docs-20（提案） |
| EX-core-007 | C-core-document-7 | kotowari は、"\n"、"\r\n"、単独の "\r" を、それぞれ1つの行の終わりに数える | そのまま |
| EX-core-281 | C-core-document-7 | kotowari は、"\n"、"\r\n"、単独の "\r" を、それぞれ1つの行の終わりに数える | そのまま |
| EX-core-282 | C-core-document-9 | kotowari は、"\n"、"\r\n"、単独の "\r" を、それぞれ1つの行の終わりに数える | 読み替え。unknown_heading は消える種類なので、「決定の文書への指摘」と読む |
| EX-core-020 | C-core-document-1 | 決定の文書の置き場は深さに制限なく辿り、小文字の .md のファイルだけを読む | 読み替え（requirement_without_test を、決定の文書への指摘と読む） |
| EX-core-030 | C-core-document-3 | 決定の文書の置き場は深さに制限なく辿り、小文字の .md のファイルだけを読む | 読み替え（IR の置き場を決定の文書の置き場と読む） |
| EX-core-021 | - | 決定の文書の置き場は深さに制限なく辿り、小文字の .md のファイルだけを読む | 消える。"FLAGS.md" と範囲の行が無くなる。サブディレクトリの "CONTEXT.md" は C-core-docs-20（提案）が扱う |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_008_invalid_marker_detail_of_a_crlf_test_file_has_no_carriage_return | REQ-core-072, TBL-core-008, TBL-core-010 | C-core-document-8 | この話題の分だけ。"\r\n" で区切ったテストのファイルのマーカーの detail に "\r" が残らない |
| tests/step4_test_discovery.rs | tbl_010_lone_cr_ends_a_line_of_a_test_file | REQ-core-118, TBL-core-010 | C-core-document-7, C-core-document-8 | この話題の分だけ。単独の "\r" を行の終わりに数えることを、テストのファイルで確かめる。設定に "ir" と "decisions.records" を書いているので、付け替えのときに設定を直す |
| tests/step4_test_discovery.rs | req_033_requirement_without_test_path_carries_the_subdirectory | REQ-core-033, REQ-core-085, REQ-core-110, TBL-core-006 | C-core-document-1 | この話題の分だけ。準備の IR の要求を、サブディレクトリの決定の文書の確認することの行に直し、confirmation_without_test の path を見る |
| crates/kotowari/src/ir_tests.rs | req_033_subdirectories_are_read_at_any_depth | REQ-core-033, REQ-core-037, TBL-core-005 | C-core-document-1 | IR の置き場で文書の数を見る。準備を決定の文書の置き場に直す |
| crates/kotowari/src/ir_tests.rs | ex_core_282_finding_after_bare_cr_has_the_split_line_number | EX-core-282 | - | unknown_heading の行番号を見る IR の読み取りのテスト。決定の文書の行番号は step23 と step24 の単独の "\r" のテストが確かめるので、古い形のコードと一緒に消す |
| crates/kotowari/src/ir_tests.rs | req_033_hidden_dir_and_dir_symlink_are_not_followed_at_any_depth | REQ-core-033 | C-core-document-2, C-core-document-3, C-core-document-4, C-core-document-13 | ".MD"、隠しディレクトリ、ディレクトリのシンボリックリンク、ファイルのシンボリックリンク、".md" で終わる名前のソケットを置き、文書が4つだけ読まれて指摘が無いことを確かめる。準備を決定の文書の置き場に直す |
| crates/kotowari/src/ir_tests.rs | req_033_broken_symlink_in_a_subdirectory_stops | REQ-core-033, REQ-core-018, TBL-core-020 | C-core-document-4 | この話題の分だけ。準備を決定の文書の置き場に直す |
| crates/kotowari/src/ir_tests.rs | req_033_context_and_flags_in_a_subdirectory_are_glossary_and_flags | REQ-core-033, REQ-core-036, REQ-core-117 | - | 用語集と問題の記録の種類を確かめる。古い形のコードと一緒に消す。C-core-docs-20 のテストは別に要る |
| crates/kotowari/src/ir_tests.rs | tbl_008_whole_document_detail_is_the_bare_filename_in_a_subdirectory | TBL-core-008, REQ-core-034, REQ-core-036, REQ-core-117 | - | この話題の分（missing_title、missing_scope）は、この文書の行に当たらない。missing_title の detail は findings の C-core-findings-10 の分で、そちらの対応表が付け替える |
| crates/kotowari-core/tests/ir.rs | req_034_missing_title | REQ-core-034 | - | IR の読み取りのテスト。古い形のコードと一緒に消す。決定の文書の題名が無いときは、step23 の a_document_without_a_title_is_an_error が C-core-docs-8 を指して確かめる |
| crates/kotowari-core/tests/ir.rs | req_035_multiple_titles | REQ-core-035 | - | IR の読み取りのテスト。決定の文書の multiple_titles は step23 と step24 のテストが確かめるので、古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_036_missing_scope | REQ-core-036 | - | 範囲の行は消える。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_036_glossary_and_flags_need_no_scope | REQ-core-036 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_037_crlf_counts_as_one_line | REQ-core-037, TBL-core-010 | - | IR の文書の line_count を見る。古い形のコードと一緒に消す。C-7 は tbl_010_split_lines_strips_cr_and_does_not_panic_on_bare_lf と tbl_010_lone_cr_ends_a_line_of_a_test_file が確かめる |
| crates/kotowari-core/tests/ir.rs | ex_core_281_bare_cr_ends_a_line | EX-core-281 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | tbl_core_010_bare_cr_keeps_items_before_an_unclosed_fence | TBL-core-010, REQ-core-112 | - | IR の読み取りのテスト。古い形のコードと一緒に消す（ir-input の対応表も同じ扱い） |
| crates/kotowari-core/tests/ir.rs | req_038_too_many_lines_is_a_notice | REQ-core-038 | - | 行数の上限の注意は無くす。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_039_too_many_requirements_skips_glossary_and_flags | REQ-core-039 | - | 要求の数の上限の注意は無くす。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_039_unknown_heading_does_not_inflate_requirement_count | REQ-core-039 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_040_code_blocks_are_skipped_except_gherkin | REQ-core-040 | C-core-document-12 | IR の読み取りのテスト。決定の文書のコードブロックの中の見出しと一覧の行を数えないことを確かめるテストに書き直す（それまで C-12 はテストの無い行） |
| crates/kotowari-core/tests/ir.rs | req_037_crlf_title_and_requirement_line_numbers | REQ-core-037, TBL-core-010 | - | IR の要求の行番号を見る。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_038_exactly_at_limit_no_notice_one_over_notices | REQ-core-038 | - | 行数の上限の注意は無くす。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_040_tilde_fence_is_a_code_block | REQ-core-040 | - | 何をコードブロックとするかは、Markdown の読み取りが CommonMark のとおりに持つ。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_040_longer_fence_needs_same_or_longer_close | REQ-core-040 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_112_unclosed_gherkin_block_with_multiple_scenarios_excludes_items | REQ-core-112, REQ-core-040 | - | gherkin のブロックと場面は消える。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | tbl_010_empty_document_has_zero_lines_and_missing_title | TBL-core-010, REQ-core-037 | - | IR の文書の line_count を見る。古い形のコードと一緒に消す。空の決定の文書の行数は status の側で確かめる |
| crates/kotowari-core/tests/ir.rs | req_034_lines_before_title_are_ignored | REQ-core-034 | - | 題名より前の行を読まないことは records.md の A156（実験者の裁定）で、落とした。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | tbl_010_split_lines_strips_cr_and_does_not_panic_on_bare_lf | TBL-core-010 | C-core-document-7 | 行の区切りの関数そのもので、"\r\n" と先頭の "\n" を確かめる（単独の "\r" は確かめていない）。テストのファイルと決定の文書の読み取りも同じ関数を使う |
| crates/kotowari-core/tests/ir.rs | req_037_two_lines_separated_by_crlf_count_as_two | REQ-core-037, EX-core-007 | - | IR の文書の line_count を見る。古い形のコードと一緒に消す |
| tests/step2_ir.rs | req_033_uppercase_md_is_not_read | REQ-core-033 | C-core-document-2 | 準備を決定の文書の置き場に直す |
| tests/step2_ir.rs | req_033_file_symlink_is_read | REQ-core-033 | C-core-document-4 | 準備を決定の文書の置き場に直す |
| tests/step2_ir.rs | req_033_broken_symlink_in_ir_dir_stops | REQ-core-033 | C-core-document-4 | 準備を決定の文書の置き場に直す |
| tests/step2_ir.rs | req_036_glossary_with_only_a_title_and_a_table_has_no_missing_scope | REQ-core-036, EX-core-006 | - | 範囲の行は消える。古い形のコードと一緒に消す |
| tests/step2_ir.rs | req_033_context_and_flags_under_a_subdirectory_have_no_missing_scope | REQ-core-033, EX-core-021 | - | 同上。サブディレクトリの "CONTEXT.md" は C-core-docs-20 のテストが要る |
| tests/step2_ir.rs | req_033_a_document_deep_in_the_tree_is_read_and_an_empty_directory_is_silent | REQ-core-033, EX-core-020 | C-core-document-1 | 準備の IR の要求を、深いディレクトリの決定の文書に直す |
| tests/step2_ir.rs | req_033_hidden_directory_and_directory_symlink_deep_in_the_tree_are_not_read | REQ-core-033, EX-core-030 | C-core-document-3 | 準備を決定の文書の置き場に直す |
| tests/step3_sources_terms.rs | req_040_gherkin_code_block_doc_ref_is_not_checked | REQ-core-040 | - | gherkin のブロックと文書名の参照は消える。古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_010_lone_cr_ends_a_line_of_records_and_adr | TBL-core-010, TBL-core-012 | - | 判断の記録と ADR は消える。古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_033_file_symlink_in_records_dir_is_read | REQ-core-033, REQ-core-018 | - | 判断の記録の置き場は消える。古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_070_document_under_a_directory_symlink_is_missing | REQ-core-033, REQ-core-070, TBL-core-014, EX-core-032 | - | 文書名の参照（missing_document）は消える。この話題の分も、IR の文書の参照の向こうの文書としての確かめなので一緒に消す |
| tests/step9_mutants.rs | req_141_lone_cr_ends_a_line_of_the_source | REQ-core-141, TBL-core-010 | - | 変異の結果のソースの行の区切りは equivalents の C-core-equiv-8 が持ち、そちらの対応表が付け替える。この話題の行には当てない |
| tests/step23_decision_documents.rs | every_line_ending_gives_the_same_body_fingerprint_and_lines | C-core-docs-1 | C-core-docs-1, C-core-document-9, C-core-document-10 | "\n"、"\r\n"、単独の "\r" で区切った同じ文書で、指摘（行番号を含む）、決定の本文と指紋、確認することの行が同じになる |
| tests/step23_decision_documents.rs | a_state_line_wrapped_by_a_lone_carriage_return_is_read | C-core-docs-1 | C-core-docs-1, C-core-document-14 | 単独の "\r" で区切った文書で、日本語と英語の状態の行が読まれ、指摘が出ない |
| tests/step24_decision_pairs.rs | a_pair_whose_lines_end_in_a_lone_carriage_return_reads_like_one_ending_in_line_feeds | C-core-docs-1 | C-core-docs-1, C-core-document-9, C-core-document-10 | 対の両側を単独の "\r" で区切っても、指摘の行番号と決定の中身が "\n" のときと同じ |
| tests/step23_decision_documents.rs | each_title_after_the_first_is_an_error_without_a_line | C-core-docs-1 | C-core-docs-1, C-core-document-5 | 題名が3つの文書に、2つ目と3つ目の題名ごとに multiple_titles が1件ずつ出る（日本語と英語の形の語の両方で）。detail は findings の C-core-findings-13、line が null なのは finding-order の C-core-forder-4 の分 |
| tests/step24_decision_pairs.rs | a_second_title_on_both_sides_is_an_error_on_each_side | C-core-docs-1 | C-core-docs-1, C-core-document-5 | 対の両側の2つ目の題名に、それぞれの側で multiple_titles が出る |
| tests/step23_decision_documents.rs | a_code_block_under_a_confirmation_row_stays_out_of_its_name_and_in_its_body | C-core-docs-1 | C-core-docs-1, C-core-document-15 | 確認することの行の下のコードブロックが、query の "name" に入らず、"body" には残る |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-033: ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）は、名前が ".md" でも読まない（読むと固まりうる） | C-core-document-13（「決定の文書の置き場は深さに制限なく辿り…」の決めたことに文を足した） | `crates/kotowari/src/ir_tests.rs` の req_033_hidden_dir_and_dir_symlink_are_not_followed_at_any_depth（"socket.md" という名前の Unix ドメインソケットを置き、読まれる文書が4つで指摘が無いことを確かめる。決定の文書の置き場も同じ走査の関数を通る）。テストが置くのはソケットだけで、名前付きパイプとデバイスは決めたことの文にだけある |

## 落とした細部

### 安全に関わるもの

無い（ソケットの細部は上で戻した）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-034（読み替え済みの所）: 題名より前にある空でない行は読まない | 実験者（records.md の A156、第9ラウンドの続きの裁定） |
| REQ-core-038: "limits.lines" の既定の値 | 利用者（records.md の A41、第6ラウンド「推奨で」）。行数の上限の注意を無くすので、値も消える |
| REQ-core-040: 閉じた gherkin のブロックの中を、場面のタグと用語と曖昧語の検査の対象にし、文書名の参照の検査では対象にしない | 消える形の細部（A52、A88） |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-033 | 新しい形で読み替えて残る。IR の置き場を決定の文書の置き場と読む。"CONTEXT.md" を用語集とすることは `decision-documents.ja.md` の C-core-docs-20（提案）に置き換わり、"FLAGS.md" は消える |
| REQ-core-034 | 新しい形で読み替えて、ほかの文書に残る（C-core-docs-8、findings） |
| REQ-core-035 | 新しい形で読み替えて残る（実装済み、C-core-document-5） |
| REQ-core-036、EX-core-006、EX-core-021 | 消える。範囲の行は `decision-documents.ja.md` の題名のあとの段落の検査に置き換わり、用語集と問題の記録の例外は要らなくなる |
| REQ-core-038、REQ-core-039 | 消える（利用者の答え）。古い形のコードがまだ持つので、「無くす」決定を提案で書いた（C-core-document-11） |
| REQ-core-040 | 新しい形で読み替えて残る。gherkin のブロックの例外は消える |
| REQ-core-041 | 新しい形で読み替えて残る。範囲の行を題名のあとの段落と読む |
| TBL-core-010 | 判断の記録と ADR の行は消え、IR の文書を決定の文書と読む |

### テストの無い行

- C-core-document-12: コードブロックの中の "## " の見出しと確認することの形の行を数えないこと。今のコードはそう動く（Markdown の読み取りの結果から決定と行を取るので、コードブロックの中は見出しにも一覧にもならない）が、決定の文書で確かめるテストが無い。step23 の閉じたコードブロックのテストは、中身が空か、見出しも "- C-…" の行も持たない。決定の「決めたこと」と「確認すること」に閉じたコードブロックを置き、その中に "## " の見出しと "- C-…" の行を書いて、list に決定も確認することの行も増えないことを確かめるテストが要る（行の文はこのまま）。
- C-core-document-1〜4、C-core-document-8、C-core-document-13: テストはあるが、どれも準備が IR の置き場か古い形の設定なので、付け替えのときに準備を直す必要がある。

## 誰が決めたか見分けられなかったもの

- records.md の第4、第6、第7ラウンドは、決定の行ごとの decided_by が無く、ラウンドの見出しに利用者の答え（「推奨で」「基本的には細部の調整といったところで、推奨で問題無いと思う」）がある。書き直しの決まりの読み方どおり、利用者が推奨をまとめて採ったものとして利用者の決定に数えた。
- 2026-09-16-ir-tree.md の A13 は「A1〜A8 から導いたこと（第2ラウンドで提示し異論なし）」である。提示して異論が無かったものとして、利用者の決定に数えた。

## 理由や見送った案が記録に無かった決定

- 「決定の文書の置き場は深さに制限なく辿り…」: 見送った案が記録に無い。
- 「決定の文書の題名は1つにし…」: 2つ以上の題名を誤りにする理由は記録に無い（A42 は形を採っただけ）。問題の節には、黙って通すと何が起きるかと、2つ目以降に出す理由（A7 の why）を書いた。見送った案も無い。
- 「題名のあとの段落は…」: 見送った案が記録に無い。
- 「kotowari は、"\n"、"\r\n"、単独の "\r" を…」: 見送った案が記録に無い。
- 「決定の文書の確認することの本文と決定の指紋も…」: 新しい形で見つかった食い違いを直したもので、見送った案は無い。
- 「行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす」: 理由も見送った案も記録に無い（利用者の答えだけ）。問題の節に「理由は残っていない」と書いた。
- 「決定の文書のコードブロックの中は…」: 理由も見送った案も記録に無い（A52 は形の契約の細部の採用）。問題の節に「理由は残っていない」と書いた。

## 分けた所、まとめた所

- REQ-core-037 と TBL-core-010 を、行の数え方（C-7〜C-9）と、決定の文書の本文・指紋・状態の行の取り方（C-10、C-14）の2つの決定に分けた。後者は d6ecb63 で直った食い違いで、決定の文書の読み取りの中の分け方の決まりなので、別の決定のまま実装済みにした。
- REQ-core-033 の部分（深さ、拡張子、隠しディレクトリとリンク、ファイルのリンクと停止、ソケット）を、行に分けて1つの決定にまとめた。
- REQ-core-041 と REQ-core-036 の残る部分（題名のあとの段落は、あるかだけを見る）を1つの決定にした。
- REQ-core-038 と REQ-core-039 を、1つの「無くす」決定にまとめた。設定のキーを無くすことは config の話題の決定にあるので、リンクで指した。
- REQ-core-040 のうち、面の引用に数えない分は surface の C-core-surface-14 に置き（振る舞いが見える側）、この文書には決定の形として読まない分と、確認することの行の名前に入れない分（C-15）を置いた。閉じないコードブロックは ir-input に置いた。

## あとでリンクにする所

- 済み: `decision-documents.ja.md`、`config.ja.md`、`ir-input.ja.md`、`cli-environment.ja.md`、`findings.ja.md`、`equivalents.ja.md`、`status.ja.md`、`surface.ja.md` への言葉の指し方は、見出しへのリンクにした。
- ほかの文書からこの文書へ: config の「行数と要求の数の上限のキー limits.lines と limits.requirements は無くす」の問題の節は、この文書の「行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす」へのリンクにできる（config は別の作業者の話題）。equivalents の「kotowari がほかのファイルの行を数えるときと同じ」も、この文書の「kotowari は、"\n"、"\r\n"、単独の "\r" を…」へのリンクにできる。

## 迷った判断と、どう決めたか

- multiple_titles の detail と line（2026-09-23-ir-engine-gaps.md の A17）は、decided_by が利用者（推奨を採用）で、決定の文書でもそう動くので、落とさずに findings の detail の表（C-core-findings-13）と finding-order の null の種類の列挙（C-core-forder-4）に置いた。前の版の「落とした細部」と「実装する側が決める細部」から外した。

- 行数と要求の数の上限（利用者の答え）: too_many_lines は決定の文書ではやめ、too_many_requirements と "limits.requirements" も無くす。提案で書いていた「行数が limits.lines を超えたら too_many_lines の注意を出す」を、「無くす」決定に書き換えた。古い形のコードがまだ持つので状態は提案。見出しも変えたので、ほかの話題から旧い見出しを指すリンクがあれば直す必要がある（この依頼の中では findings と finding-order を直した）。
- multiple_titles の決定を「提案」から「実装済み」にした。539b7c1 で、決定の文書の2つ目からの題名に multiple_titles が出るようになり、step23 と step24 のテストが確かめている（コードで確かめた: `extra_titles`）。
- 本文と指紋の決定を「提案」から「実装済み」にした。d6ecb63 で、決定の文書の読み取りが `crate::ir::split_lines`（単独の "\r" も区切る）で行を分けるようになった。状態の行も同じ分け方で読むので、C-14 を足した（d6ecb63 のテスト a_state_line_wrapped_by_a_lone_carriage_return_is_read）。
- 決定の文書のコードブロックの中の扱いで、確認することの行の下のコードブロックが行の名前につながって入る食い違いは、4688af9 で直った。A52 の「コードブロックの中は検査の対象外」の読み替えとして、行の名前に入れないことを C-15 にした。
- 題名が無い文書（REQ-core-034）は、もう `decision-documents.ja.md` と findings にあるので、この文書には決定を立てずリンクで指した。
- ソケットの細部を戻す確認することの行は、テストが置くソケットの場合だけにした。名前付きパイプとデバイスは決めたことの文に残した。

## 実装する側が決める細部

- 曖昧語の検査（`decision-documents.ja.md` の C-core-docs-18、提案）を、コードブロックの中に当てないこと。C-core-document-12 の決めたことは「検査の対象から外す」なので、作るときに揃える。
- too_many_lines と too_many_requirements を無くすときに、`FindingKind` からも外すか（skill の references の指摘の種類の表は本体のコードの種類と突き合わせるので、表も一緒に直る）。

## 利用者に聞くこと

無い。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えることになりそうか |
|---|---|
| 行数と要求の数の上限の注意 too_many_lines と too_many_requirements は無くす | `crates/kotowari-core/src/finding.rs` の `TooManyLines` と `TooManyRequirements`、それを出す古い形の読み取り（`crates/kotowari-core/src/ir/` の下）と `crates/kotowari-core/src/test_side.rs` の並び。設定のキーは config の話題の提案の決定で外す |

## 数

- 元: 要求 9、場面 7、決定表 1、性質 0
- 読んだ記録のファイル: 7（records.md、2026-09-16-ir-tree.md、2026-10-05-localization.md、2026-09-23-ir-engine-gaps.md、2026-09-23-mutants-gaps.md、2026-09-16-notice.md、2026-09-24-review3-gaps.md）。ほかに 2026-09-24-kotowari-dir.md（REQ-core-041 の how_to_verify の出典。中身はスキーマの置き場で、この話題には効かない）と、d6ecb63、539b7c1、4688af9 のコミット
- 出来上がり: 決定 7（実装済み 6、提案 1）、確認することの行 15（うち「（読んで確認）」1）
- テスト: 47（付け替え先のあるもの 19、「-」28）。このうち、この話題の ID を今は指さず C-core-docs-1 だけを指すもの 6
- 利用者に聞くこと: 0
- 戻した安全の細部: 1（C-core-document-13）
