# translation-structure の対応表

元: `docs/ir/core/translation-structure.md`（日本語は `translation-structure.ja.md`）。書き直した文書: `translation-structure.md`。短い名前は `pairstruct`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-344 | C-core-pairstruct-1, C-core-pairstruct-2, C-core-pairstruct-3, C-core-pairstruct-4, C-core-pairstruct-5 | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | そのまま（ガイド、全体像の元データ、目次）。A19、A26、A30（利用者）。IR の話題ごとの文書と問題の記録の骨組みは消え、決定の文書の骨組みは `languages-and-guides.md` の C-core-lang-4 にある |
| REQ-core-345 | C-core-pairstruct-2 | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | そのまま。A19、A27（利用者）。detail を部分の名前にすること、"line" の決め方、表の行の順に比べることは D1、D2、D3 なので落とした |
| REQ-core-346 | C-core-pairstruct-6, C-core-pairstruct-7, C-core-pairstruct-8, C-core-pairstruct-9, C-core-pairstruct-10, C-core-pairstruct-16 | ガイドの対の各側は、題名の直後に切り替えの行を持つ | そのまま（ガイド、全体像の元データ、目次。A33、A36、A40）と、読み替え（「IR の対」を決定の文書の対に。「決定の文書の対の各側も、題名の直後に切り替えの行を持つ」、実装済み）。切り替えの行を文書が扱う範囲の行に数えないこと（D3）は、決定の文書では、話題の段落の検査が切り替えの行を飛ばすことに読み替えて C-16 にした（作られたもののとおり）。区切りの " \| " と "[名前](ファイル名)" の形、"line" と detail、題名が無いときの扱い（D2、D3）は落とした |
| REQ-core-347 | C-core-pairstruct-11, C-core-pairstruct-12, C-core-pairstruct-13 | ガイドと全体像の元データの中のリンクは、同じ言語の側を指す | そのまま（ガイドと全体像の元データ。A14、A30）と、読み替え（IR を決定の文書に。「決定の文書の中のリンクも、同じ言語の側を指す」、実装済み）。画像と参照の定義を含めること、"?" より後を比べないこと、スキームの決め方（D3）は落とした |
| REQ-core-348 | C-core-pairstruct-11, C-core-pairstruct-13, C-core-pairstruct-14 | ガイドと全体像の元データの中のリンクは、同じ言語の側を指す | そのまま（A14、A27）と読み替え（決定の文書の対の側へのリンク。実装済み）。"path"、"line"、detail の値（D1、D3）は落とした |
| REQ-core-349 | C-core-pairstruct-17 | 決定の文書とその対の側から、凍結した木へのリンクは誤りにする | 読み替え（提案）。判断の記録と ADR の置き場へのリンクを誤りにすること（link_to_record）を、利用者の答え（BRIEF-C の2）で、決定の文書とその対の側から凍結した木（"docs/frozen/"）の下へのリンクを誤りにすることに読み替えた。判断の記録と ADR は凍結した木へ移る（decision-documents の「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」、form-switch の「ADR はやめ、今あるものは凍結する」）。ガイドと全体像の元データからのリンクは、答えに入っていないので含めない |
| REQ-core-350 | C-core-pairstruct-15 | 対を揃える手順は、kotowari の skill が持つ | そのまま。A25、A35、A40（利用者） |
| TBL-core-044 | C-core-pairstruct-1, C-core-pairstruct-2, C-core-pairstruct-3, C-core-pairstruct-4, C-core-pairstruct-5, C-core-pairstruct-9 | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | そのまま（ガイド、全体像の元データ、目次の行のうち A19、A26 が挙げたもの）。話題ごとの文書の行（heading、field、table、gherkin、code）と問題の記録の行は消える。用語集の行（表の行の数と出典の列）は、出典の列が消え、決定の文書の置き場の用語集では骨組みを比べない（作られたもの。translation-pairs の C-core-pairs-19）。全体像の元データの frontmatter・見出し・表・リンク、目次の "note" の有無、コードブロックの情報文字列は D3 なので落とした。部分の名前（heading、mark、code、table、link、part、toc など）は detail の値で、落とした |
| TBL-core-045 | C-core-pairstruct-3 | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | そのまま。部品の種類ごとに文の欄を決めることは A19（利用者）だが、表の中身は A19 が「仕様で表で決める」と任せたもので、落とした |
| EX-core-526 | C-core-pairstruct-1 | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | 読み替え。準備が IR の話題ごとの文書（要求の欄とシナリオのタグ）なので、ガイドで同じことを確かめる行に当てた |
| EX-core-527 | - | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | 消える。"- source:" の欄の食い違いで、出典の行は無くなる |
| EX-core-528 | C-core-pairstruct-3 | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | そのまま |
| EX-core-529 | C-core-pairstruct-4 | 対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない | そのまま。"note" の文字が文であることは D3 の細部だが、場面の題名の文として残した |
| EX-core-530 | C-core-pairstruct-6 | ガイドの対の各側は、題名の直後に切り替えの行を持つ | そのまま |
| EX-core-531 | C-core-pairstruct-7 | ガイドの対の各側は、題名の直後に切り替えの行を持つ | そのまま。detail にあるべき行を出すことは落とした細部（D3） |
| EX-core-532 | C-core-pairstruct-8 | ガイドの対の各側は、題名の直後に切り替えの行を持つ | そのまま |
| EX-core-533 | C-core-pairstruct-14 | 決定の文書の中のリンクも、同じ言語の側を指す | 読み替え（実装済み）。リンク先の「IR の日本語の側」を決定の文書の日本語の側に読み替えた |
| EX-core-534 | C-core-pairstruct-12 | ガイドと全体像の元データの中のリンクは、同じ言語の側を指す | そのまま。準備のリンク先が IR の側なので、ガイドか決定の文書の側に読み替える |
| EX-core-535 | C-core-pairstruct-17 | 決定の文書とその対の側から、凍結した木へのリンクは誤りにする | 読み替え（提案）。ガイドから判断の記録へのリンクの場面を、決定の文書から凍結した木へのリンクに読み替える |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/translations.rs | tbl_core_044_table_cells_preserve_escaped_pipes_and_source_boundaries | TBL-core-044 | C-core-pairstruct-2 | ガイドと全体像の元データで、列の数が違えば table で食い違うことを確かめる。セルの区切りの読み方は落とした細部。`Kind::Topic` と用語集の出典の部分は、古い形のコードと一緒に消す |
| crates/kotowari-core/tests/translations.rs | req_core_345_absent_headings_have_no_line_but_changed_headings_have_their_line | REQ-core-345, TBL-core-044 | C-core-pairstruct-2 | ガイドの見出しの深さと並びの食い違いを確かめる。"line" の値は落とした細部 |
| crates/kotowari-core/tests/translations.rs | tbl_core_045_overview_sentences_translate_but_keys_values_and_lengths_remain | TBL-core-044, TBL-core-045 | C-core-pairstruct-3 | 部品の文の欄だけが違えば一致し、文でない欄が違えば食い違う |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_topic_documents_compare_headings_fields_tables_gherkin_and_code | TBL-core-044 | - | 古い形のコードと一緒に消す（IR の話題ごとの文書の骨組み） |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_glossaries_compare_the_rows_and_their_sources | TBL-core-044 | - | 古い形のコードと一緒に消す（用語集の出典の列）。決定の文書の置き場の用語集の骨組みを比べないことは、translation-pairs の C-core-pairs-19 のテストが確かめる |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_flag_records_compare_ids_and_fields | TBL-core-044 | - | 古い形のコードと一緒に消す（問題の記録） |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_guides_compare_headings_marks_code_and_tables | TBL-core-044 | C-core-pairstruct-1, C-core-pairstruct-2 | 文だけの違いは一致し、見出し・マーカー・コード・表の違いは食い違う |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_overview_data_compares_frontmatter_headings_parts_marks_and_tables | TBL-core-044, TBL-core-045 | C-core-pairstruct-3 | 部品とマーカーの分。frontmatter・見出し・表の分は落とした細部（D3）で、テストの中に残る |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_contents_compare_nesting_names_and_whether_a_note_is_present | TBL-core-044 | C-core-pairstruct-4 | 入れ子と名前の分。"note" の有無は落とした細部 |
| crates/kotowari-core/tests/translations.rs | reference_links_preserve_the_order_of_their_resolved_destinations | REQ-core-344, REQ-core-345, REQ-core-347, TBL-core-044 | C-core-pairstruct-5 | ガイドのリンクの行き先の並びが違えば食い違う。参照リンクの解き方は落とした細部 |
| crates/kotowari-core/tests/translations.rs | reference_links_and_images_are_checked_at_the_usage_and_definition_lines | REQ-core-347, REQ-core-348 | C-core-pairstruct-11 | 英語のガイドの側からほかの対の日本語の側へのリンクが誤りになる。出す行と、画像と参照の定義を含めることは落とした細部 |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_link_parts_compare_destinations_read_as_the_first_language_side | TBL-core-044 | C-core-pairstruct-5 | そのまま |
| crates/kotowari-core/tests/translations.rs | tbl_core_044_the_switcher_line_is_not_in_the_skeleton | TBL-core-044 | C-core-pairstruct-9 | そのまま |
| crates/kotowari-core/tests/translations.rs | req_core_346_a_switcher_at_end_of_file_needs_no_final_newline | REQ-core-346, TBL-core-044 | C-core-pairstruct-6 | 正しい切り替えの行で何も出ないことを確かめる。最後の改行が要らないことは落とした細部 |
| crates/kotowari-core/tests/translations.rs | req_core_346_three_language_switchers_do_not_count_as_document_scope | REQ-core-346 | - | IR の文書が扱う範囲（missing_scope）を確かめる。古い形のコードと一緒に消す。決定の文書で題名のあとの段落をどう読むかは C-core-pairstruct-16 で、step24 のテストが確かめる |
| crates/kotowari-core/tests/translations.rs | req_core_346_a_switcher_after_a_byte_order_mark_does_not_count_as_document_scope | REQ-core-346, REQ-core-111 | - | この話題の分だけ。同上（IR の文書が扱う範囲）。REQ-core-111 はほかの話題の ID |
| crates/kotowari-core/tests/translations.rs | req_core_347_inline_images_have_checked_and_compared_destinations | REQ-core-347, REQ-core-348, REQ-core-349, TBL-core-044 | C-core-pairstruct-5, C-core-pairstruct-11, C-core-pairstruct-17 | 画像の行き先の言語の誤りと骨組みの食い違いの分が C-5、C-11。link_to_record を確かめる部分は提案の C-17 を指し、実装のときに凍結した木へのリンクの誤りに直す |
| crates/kotowari-core/tests/translations.rs | req_core_347_toc_titles_are_not_scanned_as_markdown_links | REQ-core-347 | - | link_to_record が出ないことを確かめる。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/translations.rs | req_core_347_a_colon_in_a_relative_link_query_is_not_a_uri_scheme | REQ-core-347, REQ-core-349 | C-core-pairstruct-17 | link_to_record で確かめている。提案の C-17 を指し、実装のときに凍結した木へのリンクの誤りで直す。スキームの読み方は落とした細部で、テストの中に残る |
| tests/step21_translations.rs | ex_core_526_sides_that_differ_only_in_sentences_match | REQ-core-344, EX-core-526 | - | 準備が IR の話題ごとの文書（要求の欄とシナリオのタグ）で、古い形のコードと一緒に消す。C-core-pairstruct-1 はガイドのテストが指す |
| tests/step21_translations.rs | ex_core_527_a_different_source_is_reported_on_its_line | REQ-core-345, EX-core-527 | - | 古い形のコードと一緒に消す（出典の欄） |
| tests/step21_translations.rs | ex_core_528_a_part_field_that_is_not_a_sentence_must_match | TBL-core-045, EX-core-528 | C-core-pairstruct-3 | そのまま |
| tests/step21_translations.rs | ex_core_529_contents_that_differ_only_in_titles_and_notes_match | EX-core-529 | C-core-pairstruct-4 | そのまま |
| tests/step21_translations.rs | ex_core_530_correct_switcher_lines_raise_nothing | REQ-core-346, EX-core-530 | C-core-pairstruct-6 | そのまま |
| tests/step21_translations.rs | ex_core_531_a_missing_switcher_line_reports_the_line_it_should_be | EX-core-531 | C-core-pairstruct-7 | そのまま。detail の値は落とした細部 |
| tests/step21_translations.rs | ex_core_532_overview_data_has_no_switcher_line | EX-core-532 | C-core-pairstruct-8 | そのまま |
| tests/step21_translations.rs | req_core_346_a_switcher_line_is_not_the_scope_of_an_ir_document | REQ-core-346 | - | 古い形のコードと一緒に消す（IR の文書が扱う範囲） |
| tests/step21_translations.rs | ex_core_533_an_english_guide_linking_the_japanese_ir_is_an_error | REQ-core-347, REQ-core-348, EX-core-533 | C-core-pairstruct-14 | 準備のリンク先が IR の日本語の側なので、付け替えのときに決定の文書の日本語の側に直す（今のコードがそれを誤りにすることは、使い捨ての場で確かめた） |
| tests/step21_translations.rs | ex_core_534_links_to_the_same_language_outside_or_inside_the_page_are_not_errors | EX-core-534 | C-core-pairstruct-12 | 準備のリンク先が IR の側なので、付け替えのときにガイドの側に直す |
| tests/step21_translations.rs | ex_core_535_a_link_to_a_decision_record_is_an_error | REQ-core-349, EX-core-535 | C-core-pairstruct-17 | 提案の C-17 を指す。実装のときに、準備をガイドから決定の文書に、リンク先を判断の記録から凍結した木に直す |
| tests/step21_translations.rs | req_core_294_overview_links_to_the_matching_guide_sides_do_not_stop_the_build | REQ-core-294, TBL-core-044 | C-core-pairstruct-5 | この話題の分だけ（各側が同じ言語のガイドを指すリンクは食い違わない）。REQ-core-294 は全体像の書き出しの話題の ID |

### 今のマーカーに足すテスト

この話題の元の ID を指していないが、提案から実装済みにした行を確かめているテスト。今のマーカー（ほかの話題の ID）はそのまま残し、この話題の ID を足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step24_decision_pairs.rs | a_decision_document_without_its_switcher_line_is_an_error | C-core-lang-1 | C-core-lang-1, C-core-pairstruct-10 | 英語の側の切り替えの行を除くと、その側に translation_switcher_invalid が出る |
| tests/step24_decision_pairs.rs | the_switcher_line_is_not_the_topic_paragraph | C-core-docs-8 | C-core-docs-8, C-core-pairstruct-16 | 両方の側の話題の段落を除くと、切り替えの行があっても topic_lead_missing が出る |
| tests/step24_decision_pairs.rs | links_in_the_japanese_side_are_checked_against_the_japanese_headings | C-core-docs-11 | C-core-docs-11, C-core-pairstruct-13 | 3つ目の場合で、日本語の側から英語の側 "greet.md#greet" へのリンクに link_language_mismatch が出る |

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-344、TBL-core-044: 全体像の元データの frontmatter、見出し、表、リンクの行き先を骨組みに入れる | LLM（2026-10-05-localization.md の D3） |
| TBL-core-044: 目次の "note" の有無を骨組みに入れ、"note" の文字を文とする | LLM（D3） |
| TBL-core-044: ガイドのコードブロックの情報文字列を比べる | LLM（D3。A19 は「コードブロックの中身」） |
| TBL-core-045: 部品の種類ごとの文の欄の一覧（lead、flow、steps、cards、status、compare、decisions、quiz の各欄）と、文でない欄の一覧 | 利用者が A19 で「仕様で部品の種類ごとに表で決める」と任せた細部 |
| REQ-core-345: detail を部分の名前にし、表の行の順で最初に一致しない部分を出す。"line" の決め方。行の番号そのものは比べない | LLM（D1、D2、D3） |
| REQ-core-346: 区切りの " \| " と "[名前](ファイル名)" の形、言語の一覧の順に並べること | LLM（D3） |
| REQ-core-346: 題名が無ければファイルの最初の空でない行を見る。あるべき行と同じ行だけを切り替えの行とする | LLM（D2） |
| REQ-core-346: "path"、"line"、detail（あるべき行） | LLM（D1、D3） |
| REQ-core-346: 切り替えの行を文書が扱う範囲の行に数えず、用語、曖昧語、文書名の参照の検査を受けない | LLM（D3） |
| REQ-core-347: 画像の行き先とリンクの参照の定義の行き先も検査する。"?" より後を比べない。スキームを「英字で始まり ":" が続く並び」とする | LLM（D3、D1） |
| REQ-core-347、REQ-core-348: "path"、"line"、detail の値 | LLM（D1、D3） |
| REQ-core-346: UI の文字の鍵の名前 "language_name" | LLM（D1）。利用者から見える設定の鍵なので、名前は決めたことに残し、言語の名前を UI の文字の鍵から取ること（A32、A33、利用者）だけを書いた |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-344、TBL-core-044 | 一部が消える。話題ごとの文書の行（"- kind:"、"- source:" などの欄、gherkin、要求の ID の見出し）と問題の記録の行は消える。決定の文書の骨組みは languages-and-guides の C-core-lang-4 に置き換わった。用語集の行は、出典の列が消え、決定の文書の置き場の用語集では骨組みを比べない（translation-pairs の C-core-pairs-19） |
| REQ-core-346 | 読み替えて残る。「IR とガイドの対」を「決定の文書とガイドの対」に読む（実装済み）。「文書が扱う範囲の行に数えない」は、決定の文書では話題の段落の検査が切り替えの行を飛ばすことになる（C-core-pairstruct-16） |
| REQ-core-347、REQ-core-348 | 読み替えて残る。検査するリンクの置き場の「IR」を決定の文書に読む（実装済み） |
| REQ-core-349、EX-core-535 | 読み替えて残る（提案）。判断の記録と ADR の置き場へのリンクの誤りを、決定の文書とその対の側から凍結した木へのリンクの誤りに読む（C-core-pairstruct-17）。判断の記録と ADR の置き場そのものは凍結した木へ移り、check は読まない |
| EX-core-526、EX-core-527 | EX-core-526 はガイドでの同じ確かめに読み替え、EX-core-527 は出典の欄の場面なので消える |
| EX-core-533、EX-core-534 | リンク先の「IR の側」を決定の文書かガイドの側に読み替えて残る |

### テストの無い行

実装済みの行 C-core-pairstruct-12 と C-core-pairstruct-14 を指すテストは、準備のリンク先が IR の側なので、付け替えのときに準備を直す必要がある。

| 行 | 要るテスト | 行の文の案 |
|---|---|---|
| C-core-pairstruct-14 | 今のテスト ex_core_533_an_english_guide_linking_the_japanese_ir_is_an_error は、IR の日本語の側へのリンクを確かめる。決定の文書の対を置いた場で、英語のガイドの側から決定の文書の日本語の側へのリンクに link_language_mismatch が出ることを確かめるテストが要る（使い捨ての場で、"guides/g.md" から "../docs/decisions/greet/greet.ja.md#挨拶する" へのリンクに link_language_mismatch が出ることを確かめた） | 行の文は今のままでよい |

## 誰が決めたか見分けられなかったもの

- TBL-core-045 の中身。A19（利用者）は「全体像の部品のどの欄が文かは、仕様で部品の種類ごとに表で決める」と書き、表の中身は IR を書く LLM が書いた。D1〜D3 は TBL-core-045 を名指ししていない。仕様の全体は利用者が承認した（Position の行）が、「細部は LLM に任せる」という承認に当たるとみて、agent の細部として落とした。

## 理由や見送った案が記録に無かった決定

- 「対の側どうしは、自然言語の文のほかの骨組みが一致しなければならない」: 見送った案が記録に無い。
- 「ガイドの対の各側は、題名の直後に切り替えの行を持つ」: 見送った案は A40 の rejected（言語の名前だけを "languages" の側に持つ）だけを書いた。
- 「決定の文書の対の各側も、題名の直後に切り替えの行を持つ」: 見送った案が記録に無い。理由は A33 の why を書いた。話題の段落との関係は、作られたものに合わせて書き、理由は decision-documents の形の決定（題名と話題の段落で始める）から書いた。
- 「決定の文書とその対の側から、凍結した木へのリンクは誤りにする」: 理由が記録に無い（利用者の答えは決まりだけ）。問題の節には、凍結した木の扱いと、これまでの link_to_record の事実を書いた。見送った案も無い。
- 「ガイドと全体像の元データの中のリンクは、同じ言語の側を指す」: 見送った案が記録に無い。
- 「決定の文書の中のリンクも、同じ言語の側を指す」: 見送った案が記録に無い。
- 「対を揃える手順は、kotowari の skill が持つ」: 見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-core-344、REQ-core-345、TBL-core-044、TBL-core-045 を、1つの決定と、文書の種類ごとの決定表にまとめた。
- REQ-core-346 を、ガイドの対と決定の文書の対の2つの決定に分けた（どちらも実装済み）。全体像の元データと目次が切り替えの行を持たないことは、ガイドの決定に入れた。
- REQ-core-347 と REQ-core-348 を、ガイドと全体像の元データと、決定の文書の2つの決定に分けた（どちらも実装済み）。ガイドから決定の文書の側へのリンク（EX-core-533 の読み替え）は、決定の文書の側に入れた。
- REQ-core-349 の読み替え（凍結した木へのリンク）は、状態が提案なので独立の決定にした。

## あとでリンクにする所

- 無い。languages-and-guides、decision-documents、view の parts の決定と、このフォルダの translation-pairs、overview-languages は、見出しか文書へのリンクにした。

## 迷った判断と、どう決めたか

- 決定の文書に切り替えの行を求めることは、A33 の理由（GitHub で開かれるのは先頭の言語の側）がそのまま当たる。前の下書きは提案にし、切り替えの行と話題の段落の関係を利用者に聞いていた。対が作られ（98c3fe1 から 4c2dd9b のコミット）、切り替えの行は題名と話題の段落の間に置かれ、話題の段落の検査は切り替えの行を飛ばす（tests/step24_decision_pairs.rs の the_switcher_line_is_not_the_topic_paragraph）ので、実装済みにして問いを消した。
- 「決定の文書の中のリンクも、…」は、`crates/kotowari-core/src/translations/check.rs` の `has_links` が決定の文書の対（用語集を除く）のリンクを検査し、テスト links_in_the_japanese_side_are_checked_against_the_japanese_headings が日本語の側から英語の側へのリンクの誤りを確かめるので、実装済みにした。C-13 の文は、テストが確かめる形（同じ対のほかの言語の側へのリンク）に直した。C-14 は使い捨ての場で確かめたが、決定の文書を相手にしたテストが無いので「テストの無い行」に挙げた。
- TBL-core-045 の中身を落とした。A19 は「仕様で部品の種類ごとに表で決める」と書いたが、書き直した文書はその表を持たず、どの欄が文かの正はコードとテストになる。表の中身を利用者が1つずつ決めた記録が無いため。
- 凍結した木へのリンク: 前の下書きは、link_to_record が指摘の種類ごと無くなり、新しい検査は作らないとしていた。利用者の答え（BRIEF-C の2）で、決定の文書とその対の側から "docs/frozen/" の下へのリンクを誤りにする提案の決定（C-17）にした。今のコードは誤りにしない（使い捨ての場で、決定の文書の両方の側とガイドから "docs/frozen/x.md" へリンクしても、check は何も出さなかった。link_to_record は "decisions.records" と "decisions.adr" の置き場だけを見る）。答えが決定の文書とその対の側だけを挙げているので、ガイドと全体像の元データからのリンクは含めなかった。
- 決定の文書の骨組みに表、コード、リンクを入れるかの問いは、作られたもの（骨組みは見出し、状態の行、確認することの ID。tests/step24_decision_pairs.rs の different_words_in_the_same_skeleton_are_no_mismatch が、日本語の側にだけ表を足しても食い違わないことを確かめる）で答えが出たので消した。languages-and-guides の C-core-lang-4 のとおりで、この話題の決定表は決定の文書の骨組みをそこへリンクするだけにした。
- 決定の文書の形の語を日本語でない側でどう書くかの問いは、作られたもの（形の語に英語の綴りがあり、check はどちらの綴りでも読む。98c3fe1 のコミットと skill の文）で答えが出たので消した。形の語は decision-documents の話題で、この話題には決定を足さなかった。
- EX-core-529 の「"note" の文字だけが違えば一致する」は、"note" の文字を文とすることが D3 の細部だが、目次の題名の文が訳してよいこと（A26）を確かめる例として、確認することの行は群の題名と目次の題名で書いた。

## 実装する側が決める細部

- ガイドのマーカーは骨組みに入るので、ガイドのほかの言語の側のマーカーも、先頭の言語の側の決定の見出し（の行き先の名前）を指す。新しい名前は要らない。
- 凍結した木へのリンクの誤りの指摘の名前。今のコードの link_to_record を、相手を凍結した木に読み替えて使うか、改名するか。

## 利用者に聞くこと

- 無い。前の下書きの3つの問い（決定の文書の骨組みに入れるもの、形の語を日本語でない側でどう書くか、題名と話題の段落の間の切り替えの行）は、作られたもので答えが出たので消した（「迷った判断」）。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えることになりそうか |
|---|---|
| 決定の文書とその対の側から、凍結した木へのリンクは誤りにする | `crates/kotowari-core/src/translations/check.rs` の `side_text` が判断の記録と ADR の置き場で link_to_record を出す所を、決定の文書の対の側からの "docs/frozen/" の下への行き先で出すように変える。言語が1つのときも当てるなら、`crates/kotowari-core/src/decisions.rs` の `check_links` の側に置く |

## 数

- 元: 要求 7、場面 10、決定表 2、性質 0
- 読んだ記録のファイル: 1（2026-10-05-localization.md）
- 出来上がり: 決定 7（実装済み 6、提案 1）、確認することの行 17（うち「（読んで確認）」1）
- テスト: この話題の ID を指す 31（付け替え先のあるもの 22、「-」9。「-」はどれも古い形のコードと一緒に消す）と、今のマーカーに足す3本
- 利用者に聞くこと: 0（答えの出た3つを消した）
