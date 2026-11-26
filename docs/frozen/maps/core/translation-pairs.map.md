# translation-pairs の対応表

元: `docs/ir/core/translation-pairs.md`（日本語は `translation-pairs.ja.md`）。書き直した文書: `translation-pairs.md`。短い名前は `pairs`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-334 | C-core-pairs-1, C-core-pairs-2 | 言語の一覧は、設定の languages で決める | そのまま。A16、A21（利用者）。言語が1つなら出さない指摘の並びから link_to_record を除いた（判断の記録と ADR の置き場へのリンクの誤りとしては消える。判断の記録は凍結した木にあり、check は読まない。決定の文書から凍結した木へのリンクの誤りに読み替えることは translation-structure の C-core-pairstruct-17（提案）で、言語が1つのときに出すかは、その指摘の名前と一緒に実装する側が決める）。"languages" が空の一覧なら英語とすることは D3（LLM）なので落とした |
| REQ-core-335 | C-core-pairs-3 | 言語タグの形が違うか重なっていれば、停止する | そのまま。A21（利用者） |
| REQ-core-336 | C-core-pairs-4, C-core-pairs-5, C-core-pairs-14 | ガイド、全体像の元データ、目次は、言語の一覧のすべての言語の側を持つ | 読み替え。ガイド、全体像の元データ、目次はそのまま（実装済み）。「IR の置き場の文書」は決定の文書の置き場の文書に読み替え、決定の文書は languages-and-guides の「決定の文書も、多言語の対で持てる」（実装済み）、用語集は「用語集も対にする」（実装済み。C-14、C-19）へ分けた。問題の記録は消える（`decision-documents.md` の「人に戻す問題は、話題の文書の「未決」の節に書く」）。"decisions.records" と "decisions.adr" を対にしないことは消える（凍結した木。`form-switch.md` の「ADR はやめ、今あるものは凍結する」） |
| REQ-core-337 | C-core-pairs-6 | 対の側は、ファイル名と設定だけで見分ける | そのまま。A15、A31、A39（利用者）。"<幹>.i18n.yaml" を側として読まないこと、2つの置き場から読まれる対の指摘を1回だけ出すこと（D2）、ほかの言語の側を glob を問わず名前で探すこと（D3）は落とした |
| REQ-core-338 | C-core-pairs-4, C-core-pairs-5 | ガイド、全体像の元データ、目次は、言語の一覧のすべての言語の側を持つ | そのまま。A17、A27（利用者）。"path"、"line"、detail の値（D1、D3）と、先頭の言語の側の無い側をほかの検査で読まないこと（D3）は落とした |
| REQ-core-339 | C-core-pairs-7, C-core-pairs-8 | 一致の記録は、最後に同じ中身だと確かめたときの各側の blob hash を持つ | そのまま。A15、A18、A27、A28（利用者）。detail の値と、2つ以上に当たるときの順、値を小文字に限ること（D3）は落とした |
| REQ-core-340 | C-core-pairs-9, C-core-pairs-10 | 一致の記録は、最後に同じ中身だと確かめたときの各側の blob hash を持つ | そのまま。A18（利用者）。hash の求め方のバイト列の並びは "git hash-object" と同じ値の言い換えなので、行は「同じ値」で書いた。kotowari が記録を書き込まないこと（A18）を C-10 にした |
| REQ-core-341 | C-core-pairs-11 | 側を直して一致の記録を直していなければ、その側を古いとする | そのまま。A18、A27（利用者）。detail の値（D3）、一致の記録に誤りがあるときに出さないこと（D3）、無い側に出さないこと（D2）は落とした |
| REQ-core-342 | C-core-pairs-15, C-core-pairs-16, C-core-pairs-17, C-core-pairs-18 | 決定の文書の対は、数えることと出力を先頭の言語の側だけで行う | 読み替え。ID の重なり、面の判定、list と query の出力は実装済み（C-15、C-16、C-18）。曖昧語をどの側にもかけることは、今のコードが決定の文書に曖昧語の検査をしないので、別の提案の決定「曖昧語の検査は、決定の文書のどの側の「決めたこと」と「確認すること」にも行う」（C-17）に分けた。A22、A23、A24（利用者）。「項目とシナリオ」を確認することの行に、テストのマーカーとの照合を `languages-and-guides.md` の C-core-lang-3 に読み替えた。出典の検査、用語の検査と言語ごとの用語集の連鎖、閉じないバッククォート、用語集の形の検査は消える（`decision-documents.md` の「仕様は、話題ごとの、今有効な決定の文書だけで持つ」と「用語集は人が読む文書として残し、用語の検査はやめる」）。文書名の参照の検査は IR の検査で、消える。"files" と "lines" にすべての側を数えること（D2）と、指紋を先頭の言語の側で取ること（D3）は落とした |
| REQ-core-343 | C-core-pairs-12, C-core-pairs-13 | ガイドのマーカーの古さは、ガイドの対のどの側にも出す | そのまま（ガイドの部分、A22）と、そのまま（全体像の名前の重なりと目次との照合を先頭の言語の側だけで行う部分、A31・A39 の Revisions の行）。全体像の元データの形・部品・lead・参照・マーカーの検査をどの側にも行うこと、目次の形の検査をどの側にも行うことは D3 なので落とした。扱う IR の文書の検査（REQ-core-284）は、決定の文書を指す一覧に読み替えて `languages-and-guides.md` の「全体像の元データは、決定の文書を指す」（C-core-lang-7）にある |
| EX-core-512 | C-core-pairs-1 | 言語の一覧は、設定の languages で決める | そのまま。準備の「IR の置き場」は、ガイドか決定の文書の置き場に読み替える |
| EX-core-513 | C-core-pairs-3 | 言語タグの形が違うか重なっていれば、停止する | そのまま |
| EX-core-514 | C-core-pairs-4 | ガイド、全体像の元データ、目次は、言語の一覧のすべての言語の側を持つ | そのまま。準備の「IR の置き場」は読み替える。"path" と detail の値は落とした細部 |
| EX-core-515 | C-core-pairs-5 | ガイド、全体像の元データ、目次は、言語の一覧のすべての言語の側を持つ | そのまま。準備は読み替える |
| EX-core-516 | C-core-pairs-6 | 対の側は、ファイル名と設定だけで見分ける | そのまま。準備は読み替える |
| EX-core-517 | - | - | 消える。判断の記録は凍結した木にあり、check は読まない（`decision-documents.md` の C-core-docs-14） |
| EX-core-518 | C-core-pairs-7 | 一致の記録は、最後に同じ中身だと確かめたときの各側の blob hash を持つ | そのまま |
| EX-core-519 | C-core-pairs-11 | 側を直して一致の記録を直していなければ、その側を古いとする | そのまま。detail の2つの hash は落とした細部 |
| EX-core-520 | C-core-pairs-8 | 一致の記録は、最後に同じ中身だと確かめたときの各側の blob hash を持つ | そのまま。記録に誤りがあるときに translation_stale を出さないことは落とした細部（D3） |
| EX-core-521 | C-core-pairs-8 | 一致の記録は、最後に同じ中身だと確かめたときの各側の blob hash を持つ | そのまま |
| EX-core-522 | C-core-pairs-15, C-core-pairs-16 | 決定の文書の対は、数えることと出力を先頭の言語の側だけで行う | 読み替え（実装済み）。要求の重なりを確認することの ID の重なりに、list の "items" を決定と確認することの行に読み替えた。英語の用語集から用語を引く部分は消える |
| EX-core-523 | - | 決定の文書の対は、数えることと出力を先頭の言語の側だけで行う | 消える。用語の検査をやめた（`decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」） |
| EX-core-524 | C-core-pairs-12 | ガイドのマーカーの古さは、ガイドの対のどの側にも出す | そのまま。準備のマーカーが指す要求は、決定の見出しに読み替える |
| EX-core-525 | C-core-pairs-13 | 全体像の元データのほかの言語の側は、別のページとして数えない | そのまま（overview_name_conflict と overview_toc_page_missing）。overview_ir_shared は扱う IR の文書の検査で、決定の文書に読み替えて全体像の話題に属する |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/translations.rs | req_core_334_the_language_list_is_the_languages_key_and_english_when_absent_or_empty | REQ-core-334 | C-core-pairs-2 | 鍵があれば書いた順、無ければ英語になることを確かめる。空の一覧で英語になることは落とした細部で、テストの中に残る |
| crates/kotowari-core/tests/translations.rs | req_core_335_an_empty_malformed_or_repeated_tag_stops | REQ-core-335 | C-core-pairs-3 | そのまま |
| crates/kotowari-core/tests/translations.rs | req_core_335_invalid_tags_stop_even_with_complete_labels | REQ-core-335, REQ-core-352 | C-core-pairs-3 | この話題の分だけ。REQ-core-352 は UI の文字の話題（overview-languages）の ID |
| crates/kotowari-core/tests/translations.rs | req_core_340_the_blob_hash_is_the_git_blob_hash | REQ-core-340 | C-core-pairs-9 | そのまま |
| tests/step1_config.rs | ex_core_513_a_language_tag_with_an_upper_case_letter_stops | EX-core-513 | C-core-pairs-3 | そのまま |
| tests/step21_translations.rs | ex_core_512_with_one_language_a_suffixed_file_is_a_document_of_its_own | REQ-core-334, EX-core-512 | C-core-pairs-1 | 準備が IR の置き場と IR の要求なので、付け替えのときにガイドか決定の文書の置き場の準備に直す。list の "translations" が出ないことは list の話題の分 |
| tests/step21_translations.rs | ex_core_514_a_missing_english_side_is_an_error_on_the_first_side | REQ-core-336, REQ-core-337, REQ-core-338, EX-core-514 | C-core-pairs-4 | 準備が IR の置き場なので、付け替えのときにガイドの置き場に直す。"line" が null であることは落とした細部 |
| tests/step21_translations.rs | ex_core_515_a_missing_first_side_is_an_error_on_the_side_that_exists | REQ-core-338, EX-core-515 | C-core-pairs-5 | 準備が IR の置き場なので直す。その側をほかの検査で読まないことは落とした細部（D3） |
| tests/step21_translations.rs | ex_core_516_a_suffix_of_the_first_language_or_an_unlisted_one_is_part_of_the_name | REQ-core-337, EX-core-516 | C-core-pairs-6 | 準備が IR の置き場なので直す |
| tests/step21_translations.rs | ex_core_517_decision_records_are_not_paired | REQ-core-336, EX-core-517 | - | 古い形のコードと一緒に消す |
| tests/step21_translations.rs | ex_core_518_a_record_matching_the_files_raises_nothing | REQ-core-339, REQ-core-340, REQ-core-341, EX-core-518 | C-core-pairs-7 | そのまま（ガイドの対） |
| tests/step21_translations.rs | ex_core_519_changing_one_side_makes_it_stale | REQ-core-341, EX-core-519 | C-core-pairs-11 | そのまま。detail の値は落とした細部 |
| tests/step21_translations.rs | ex_core_520_record_keys_other_than_the_side_names_are_invalid | REQ-core-339, EX-core-520 | C-core-pairs-8 | そのまま |
| tests/step21_translations.rs | ex_core_521_a_missing_record_is_invalid | REQ-core-339, EX-core-521 | C-core-pairs-8 | そのまま |
| tests/step21_translations.rs | req_core_339_an_unreadable_record_or_a_malformed_value_is_named_in_order | REQ-core-339 | C-core-pairs-8 | YAML でない記録、対応表でない記録、40文字の16進でない値が誤りになることを確かめる。detail の値と大文字の扱いは落とした細部 |
| tests/step21_translations.rs | req_core_337_a_pair_read_from_two_places_is_reported_once_and_sides_are_found_by_name | REQ-core-337 | C-core-pairs-6 | 落とした細部のテスト。いちばん近い行を指す。2つの置き場から読まれる対の指摘を1回だけ出すこと（D2）と、glob に当たらない側を名前で探すこと（D3）は、テストの中に残る。準備が IR の置き場なので、ガイドと決定の文書の置き場で書き直す |
| tests/step21_translations.rs | ex_core_522_the_english_side_is_not_counted_and_takes_terms_from_the_english_glossary | REQ-core-342, EX-core-522 | C-core-pairs-15, C-core-pairs-16 | 準備が IR の対なので、付け替えのときに、ID の重なりと list の出力を決定の文書の対で確かめる形に直す。用語の部分と "files"・"lines" の部分は消す |
| tests/step21_translations.rs | ex_core_523_a_term_only_in_the_japanese_glossary_is_unknown_on_the_english_side | EX-core-523 | - | 古い形のコードと一緒に消す（用語の検査） |
| tests/step21_translations.rs | req_core_342_text_and_glossary_checks_run_on_the_english_side_but_item_checks_do_not | REQ-core-342 | C-core-pairs-17 | 実装のときに直す。曖昧語がほかの言語の側に出ることを、決定の文書の「決めたこと」か「確認すること」で確かめる形にする。閉じないバッククォートと用語集の形の部分は消す |
| tests/step21_translations.rs | ex_core_524_a_stale_guide_mark_is_reported_on_every_side | REQ-core-343, EX-core-524 | C-core-pairs-12 | 準備のマーカーが IR の要求を指しているので、付け替えのときに決定の見出しを指す形に直す |
| tests/step21_translations.rs | translated_overview_files_cannot_also_be_test_files | REQ-core-280, REQ-core-337 | - | この話題の分だけ。置き場の重なりの停止を確かめるテストで、全体像の元データの話題の対応表が C-core-ovdata-4 を付けるので、マーカーは無くならない |
| tests/step21_translations.rs | req_core_280_a_side_without_its_first_side_matched_by_all_three_stops_on_the_guides_and_tests_overlap | REQ-core-280, REQ-core-199, REQ-core-338 | - | この話題の分だけ。重なりの判定に数える側の決定（2026-10-23-guide-overlap-sides.md の A1、整合のフェーズ）で、全体像の元データの話題の対応表が C-core-ovdata-21 を付ける |
| tests/step21_translations.rs | req_core_280_a_side_without_its_first_side_matched_by_overview_and_guides_stops | REQ-core-280, REQ-core-338 | - | 同上（C-core-ovdata-3、C-core-ovdata-22 が付く） |
| tests/step21_translations.rs | req_core_199_a_side_found_by_name_counts_as_a_guide_in_the_overlap | REQ-core-199, REQ-core-337 | - | 同上（ガイドの対応表が C-core-guides-9 を付ける） |
| tests/step21_translations.rs | ex_core_525_the_english_overview_side_is_not_another_page | EX-core-525 | C-core-pairs-13 | overview_name_conflict と overview_toc_page_missing の分。overview_ir_shared の分は全体像の話題の新しい ID を指す |
| tests/step21_translations.rs | req_core_343_overview_and_contents_forms_are_checked_on_every_side | REQ-core-343 | C-core-pairs-13 | 落とした細部のテスト。いちばん近い行を指す。全体像の元データと目次の形をどの側にも検査すること（D3）は、テストの中に残る |
| tests/step21_translations.rs | req_core_336_a_guide_glob_over_the_record_places_does_not_pair_the_records | REQ-core-336, EX-core-517 | - | 古い形のコードと一緒に消す |

### 今のマーカーに足すテスト

この話題の元の ID を指していないが、提案から実装済みにした行を確かめているテスト。今のマーカー（ほかの話題の ID）はそのまま残し、この話題の ID を足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step24_decision_pairs.rs | a_pair_with_both_sides_and_a_matching_record_raises_nothing | C-core-lang-1 | C-core-lang-1, C-core-pairs-15 | 両方の側に同じ C-greet-1 があっても指摘が無く、"confirmations" が 1 になる |
| tests/step24_decision_pairs.rs | ids_and_marks_are_matched_against_the_first_language_side_only | C-core-lang-3 | C-core-lang-3, C-core-pairs-16 | "kotowari query" が先頭の言語の側の1件だけを返す（list の分は確かめない） |
| tests/step24_decision_pairs.rs | a_glossary_is_paired_but_its_content_is_not_compared | C-core-docs-20 | C-core-docs-20, C-core-pairs-14, C-core-pairs-19 | 前半で日本語の側と一致の記録が無いと translation_missing と translation_record_invalid が出ることを、後半で（両方の側に切り替えの行を置いたうえで）見出しと表の違う側と無い文書へのリンクでも指摘が出ないことを確かめる |

## 落とした細部

### 安全に関わるもの

無い。停止の条件（言語タグの誤り）は C-core-pairs-3 に残した。safety-dropped.md にこの話題の行は無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-334: "languages" が空の一覧なら英語だけとみなす | LLM（2026-10-05-localization.md の D3） |
| REQ-core-337: "<幹>.i18n.yaml" を側として読まない | LLM（D2） |
| REQ-core-337: 1つの対が2つ以上の置き場から読まれても、対の指摘を1回だけ出す | LLM（D2） |
| REQ-core-337: ほかの言語の側を、"guides.files" と "overview.files" の glob に当たるかを問わず、先頭の言語の側と同じディレクトリで名前で探す | LLM（D3） |
| REQ-core-338: "path"、"line"、detail の値（無い側のパス） | LLM（D1、D3） |
| REQ-core-338: 先頭の言語の側の無い側を、ほかの検査で読まない | LLM（D3） |
| REQ-core-339: "path"、"line"、detail の値（"missing"、"yaml"、"keys"、"value"）と、2つ以上に当たるときの順 | LLM（D3） |
| REQ-core-339: 値を16進の小文字に限る | LLM（D3） |
| REQ-core-341: detail を2つの hash を半角空白で区切った文字列にする | LLM（D3） |
| REQ-core-341: 一致の記録に translation_record_invalid があるときは translation_stale を出さない | LLM（D3） |
| REQ-core-341: 無い側には translation_stale を出さない | LLM（D2） |
| REQ-core-342: check と status の "files" と "lines" に、すべての側を数える | LLM（D2） |
| REQ-core-342: 指紋を先頭の言語の側だけで取る | LLM（D3） |
| REQ-core-343: 全体像の元データの形、部品、lead、参照、マーカーの検査をどの側にも行い、目次の形の検査を目次のどの側にも行う | LLM（D3） |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-334 | 新しい形で読み替えて残る。言語が1つのときに出さない指摘の並びから link_to_record を外す（凍結した木へのリンクの誤りへの読み替えは translation-structure の C-core-pairstruct-17、提案） |
| REQ-core-336 | 一部が読み替えて残る。IR の置き場の文書を決定の文書の置き場の文書（決定の文書と用語集）に読み替える。問題の記録は消える。判断の記録と ADR の置き場を対にしないことは消える（凍結した木にあり、check が読まない） |
| REQ-core-342 | 一部が読み替えて残る（「決定の文書の対は、数えることと出力を先頭の言語の側だけで行う」は実装済み、「曖昧語の検査は、…」は提案）。項目とシナリオを確認することの行に、テストのマーカーとの照合を `languages-and-guides.md` の C-core-lang-3 に読み替える。出典と ID の参照の検査、用語の検査、用語集の連鎖、閉じないバッククォート、用語集の形、文書名の参照の検査は消える |
| REQ-core-343 | 扱う IR の文書の検査（REQ-core-284 を指す部分）は、決定の文書の一覧に読み替えて `languages-and-guides.md` の C-core-lang-7 にある |
| EX-core-512、EX-core-514、EX-core-515、EX-core-516 | 準備の「IR の置き場」をガイドか決定の文書の置き場に読み替えて残る |
| EX-core-517 | 消える |
| EX-core-522 | 読み替えて残る（実装済み。準備を決定の文書の対に直す）。用語の部分は消える |
| EX-core-523 | 消える |
| EX-core-524 | 準備のマーカーの指す先を、要求から決定の見出しに読み替えて残る |

### テストの無い行

実装済みの行のうち C-core-pairs-1、C-core-pairs-4、C-core-pairs-5、C-core-pairs-6、C-core-pairs-12、C-core-pairs-15、C-core-pairs-16 を指すテストの一部は、準備が IR の置き場か IR の要求なので、付け替えのときに準備を直す必要がある（テストごとの表に書いた）。

テストが無いか、一部にしか無い所:

| 行か細部 | 要るテスト | 行の文の案 |
|---|---|---|
| C-core-pairs-16 の "kotowari list" の分 | 決定の文書の対で "kotowari list" を走らせ、"items" に先頭の言語の側の決定と確認することの行だけが出て、日本語の側のパスの行が無いことを確かめる（使い捨ての場で、"items" に "docs/decisions/greet/greet.md" の2件だけが出ることを確かめた） | 行の文は今のままでよい |
| C-core-pairs-18 | 面のファイルを持つ場で、ほかの言語の側の「決めたこと」にだけ面の名前を書き、check が surface_without_spec を出すことを確かめる。今のコードは面の引用を先頭の言語の側の決定からだけ集める（`crates/kotowari-core/src/inputs/read.rs` の `parse_decisions` がほかの言語の側を行き先としてだけ読み、`crates/kotowari-core/src/surface.rs` の `quoted_in_ir` が先頭の言語の側の決定だけを受ける） | 行の文は今のままでよい |
| 用語集の対の切り替えの行（決めたことに書いた） | 用語集の対の一方の側から切り替えの行を除き、check がその側に translation_switcher_invalid を出すことを確かめる（使い捨ての場で、切り替えの行の無い用語集の両方の側に translation_switcher_invalid が出ることを確かめた） | 用語集の対の側の題名の次の行が切り替えの行でなければ、check がその側に translation_switcher_invalid の誤りを出す |

## 誰が決めたか見分けられなかったもの

- 「全体像の元データのほかの言語の側は、別のページとして数えない」（C-core-pairs-13）: 名前の重なりと目次との照らし合わせを先頭の言語の側だけで行うことは、Revisions の行が A31 と A39（利用者）の結果として書き、D3（LLM）も「扱う IR の文書と名前の重なりと目次との照合は先頭の言語の側だけで行う」と書いている。利用者の決定（A31、A39）から生まれた振る舞いとして、Revisions の行を根拠に決定に残した。D3 にしか無い部分（扱う IR の文書の検査を先頭の言語の側だけで行うこと、形の検査をどの側にも行うこと）は落とした。
- ほかには無い。2026-10-05-localization.md の A 行は、どれも decided_by が「利用者（推奨を採用）」か「利用者」で、D1〜D3 は LLM が決めた細部（D1 は利用者が「細部は LLM に任せる」と承認したもの）。2026-10-23-guide-overlap-sides.md の A1 は decided_by が整合のフェーズで、ガイドと全体像の元データの話題に属するので、この話題には書かなかった。

## 理由や見送った案が記録に無かった決定

- 「言語タグの形が違うか重なっていれば、停止する」: 理由が記録に無い（A21 の why は UI の文字を言語ごとに用意する理由）。問題の節に「理由は残っていない」と書いた。見送った案も無い。
- 「対の側は、ファイル名と設定だけで見分ける」: 見送った案が記録に無い。
- 「側を直して一致の記録を直していなければ、その側を古いとする」: 見送った案が記録に無い。理由は A27 の why（揃っていないことは原語に落ちることに直結する）を書いた。
- 「ガイドのマーカーの古さは、ガイドの対のどの側にも出す」: どの側にも出す理由が記録に無い（A22 の why は、骨組みの検査を片側で、文の検査を側ごとに行う理由で、マーカーの古さには当たらない）。「理由は残っていない」と書いた。見送った案も無い。
- 「全体像の元データのほかの言語の側は、別のページとして数えない」: 理由が記録に無い（Revisions の行は何を変えたかだけ）。A22 の「1つの文書の言語ごとの姿」を問題に書き、「理由は、ほかには残っていない」と書いた。見送った案も無い。
- 「用語集も対にする」: 見送った案が記録に無い。理由は A11 の why を書いた。骨組みとリンクを比べないことは、決定の文書の形の決定（decision-documents の「用語集は人が読む文書として残し、用語の検査はやめる」、C-core-docs-20）から来るので、問題の節からリンクした。
- 「曖昧語の検査は、決定の文書のどの側の…」: 見送った案が記録に無い。理由は A22、A23 の why を書いた。
- 「決定の文書の対は、数えることと出力を先頭の言語の側だけで行う」: 見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-core-336 を、ガイド・全体像の元データ・目次と、用語集の2つの決定に分けた（どちらも実装済み）。決定の文書は `languages-and-guides.md` に既にあるので書かなかった。
- REQ-core-336 と REQ-core-338 を「ガイド、全体像の元データ、目次は、…すべての言語の側を持つ」の1つの決定にまとめた。どの文書を対にするかは、欠けた側の誤りで見えるため。
- REQ-core-339 と REQ-core-340 を「一致の記録は…」の1つの決定にまとめた。どちらも A18 の決定で、blob hash は一致の記録の値としてだけ使われる。
- REQ-core-343 を、ガイドのマーカー（A22）と全体像の名前の数え方（A31、A39）の2つの決定に分けた。決めた記録の行が違う。
- REQ-core-342 は、決定の文書の対に読み替えた分を、実装済みの「決定の文書の対は、数えることと出力を先頭の言語の側だけで行う」と、提案の「曖昧語の検査は、…」の2つに分けた（状態が違うため）。マーカーとの照合は languages-and-guides の C-core-lang-3 にあるので書かなかった。

## あとでリンクにする所

- 無い。languages-and-guides と decision-documents の決定、このフォルダの translation-structure、overview-languages、list、overview-commands、overview-toc、surface の決定は、見出しか文書へのリンクにした。

## 迷った判断と、どう決めたか

- 対の仕組みそのもの（側の見分け方、欠けた側、一致の記録、古い側）は、ガイド、全体像の元データ、目次で今動いているので、置き場によらない「実装済み」の決定として書いた。決定の文書への当て方は languages-and-guides の「決定の文書も、多言語の対で持てる」（実装済み）が持つので、繰り返さなかった。
- 用語集の対を独立の決定にした。用語集の置き場は decision-documents の決定で変わったが、対にすることはそこに書かれていないため。前の下書きは提案にし、骨組みと切り替えの行を利用者に聞いていた。対が作られた（98c3fe1 から 4c2dd9b のコミット、テスト tests/step24_decision_pairs.rs の a_glossary_is_paired_but_its_content_is_not_compared）ので、実装済みにして問いを消した。
- 用語集の対で何を検査するかは、作られたものに合わせた。依頼の要約は「側があることと一致の記録だけ」だったが、使い捨ての場で、切り替えの行の無い用語集の対の両方の側に translation_switcher_invalid が出た（`crates/kotowari-core/src/translations/check.rs` の `has_switcher` が決定の文書の置き場の対すべてに切り替えの行を求め、用語集だけを外すのは骨組み（`skeleton_kind` の DecisionGlossary）とリンク（`has_links`）である）。skill の translations.md の骨組みの表も「nothing; only the sides and the record are checked」で、骨組みのことを言っている。そこで、骨組みとリンクを比べないことを行（C-19）にし、切り替えの行を求めることは決めたことに書いて、テストが無いので「テストの無い行」に挙げた。
- 「決定の文書の対は、…」を、作られたものと照らして実装済みにした。ID の重なり（C-15）は、ほかの言語の側を決定として読まない（`parse_decisions` が行き先としてだけ読む）ので出ない。list と query（C-16）は、使い捨ての場で "kotowari list" の "items" が先頭の言語の側の2件だけで、query はテストが確かめる。面の判定（C-18）はコードを読んで確かめた。曖昧語（C-17）は、使い捨ての場で両方の側の「決めたこと」に「適切に」を書いても check が何も出さず、今のコードは決定の文書に曖昧語の検査をしない（decision-documents の C-core-docs-18 も提案）ので、提案の決定に分けた。
- 曖昧語の検査をほかの言語の側にも行うことは、A22（文の検査は側ごと）と A23（1つの一覧）の読み替えで意味が通るので、聞かずに提案の決定に書いた。
- 面の判定を先頭の言語の側だけで行う（C-core-pairs-18）は、A22 が「面の判定」を挙げているので残した。
- REQ-core-337 の "guides.files" と "overview.files" の名前は、落とした D3 の細部（glob を問わず名前で探す）の中にだけ出てくるので、文書に残さなかった。どちらの鍵も、ガイドと全体像の元データの話題の文書に出てくる。

## 実装する側が決める細部

- ほかの言語の側の曖昧語の指摘の名前と、出す行の位置は、decision-documents の C-core-docs-18 を作るときに決める。今の決定の文書のコードには曖昧語の検査が無い。

## 利用者に聞くこと

- 無い。用語集の対で何を骨組みとして比べ、切り替えの行を求めるかの問いは、作られたもの（骨組みとリンクは比べず、側と一致の記録と古い側を見る。切り替えの行は求める）で答えが出たので消した。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えることになりそうか |
|---|---|
| 曖昧語の検査は、決定の文書のどの側の「決めたこと」と「確認すること」にも行う | decision-documents の C-core-docs-18 の曖昧語の検査を `crates/kotowari-core/src/decisions.rs` に作るとき、`crates/kotowari-core/src/inputs/read.rs` の `parse_decisions` がほかの言語の側を行き先としてだけ読む所でも、「決めたこと」と「確認すること」の文にかける |

## 数

- 元: 要求 10、場面 14、決定表 0、性質 0
- 読んだ記録のファイル: 2（2026-10-05-localization.md、2026-10-23-guide-overlap-sides.md）
- 出来上がり: 決定 11（実装済み 10、提案 1）、確認することの行 19（うち「（読んで確認）」1）
- テスト: この話題の ID を指す 27（付け替え先のあるもの 20、「-」7。「-」のうち3本は古い形のコードと一緒に消し、4本はほかの話題の対応表が ID を付ける）と、今のマーカーに足す3本
- 利用者に聞くこと: 0（答えの出た1つを消した）
