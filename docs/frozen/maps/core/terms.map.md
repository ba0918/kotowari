# terms の対応表

元: `docs/ir/core/terms.md`（日本語は `terms.ja.md`）。短い名前は terms（確認することの ID を付けるなら `C-core-terms-<n>`）。

書き直した文書（`terms.md`）は作らなかった。この話題で新しい版に残る決定は、どれもほかの文書にすでに書いてある。

- 曖昧語の検査を、決定の「決めたこと」と「確認すること」に当て、「問題」と「見送った案」には当てない: `docs/decisions/core/decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」（C-core-docs-18、C-core-docs-19。提案）
- 曖昧語の一覧は設定の "vague_words" で、既定は4語: `config.md` の「設定のキーは表のものだけにし、書かなかったキーは表の既定の値にする」
- 一覧を言語ごとに持たず、対のどの側にもかける: `translation-pairs.md` の「決定の文書の対は、数えることと出力を先頭の言語の側だけで行う」（C-core-pairs-17。提案）
- 指摘の種類の名前 vague_word と、それが誤りであること: `findings.md` の「指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める」の2つ目の表
- 無い文書を指す参照を誤りにする: `docs/decisions/core/decision-documents.md` の「ほかの決定を指すときは、見出しへリンクを張る」（C-core-docs-11。実装済み。種類は link_target_missing）

残りの細部（部分一致、出現ごとに1件、最長一致）は、records.md の第6、第7、第9ラウンドで利用者が推奨をまとめて採ったもので、書き直しの決まりの読み方では利用者の決定になる。書く先は `decision-documents.ja.md` の「用語集は人が読む文書として残し、用語の検査はやめる」（提案）だが、リポジトリに既にある core の文書でこの依頼の外なので、下の「書く先が依頼の外にある利用者の決定」に挙げた。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-063 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 読み替え（提案）。曖昧語の検査の対象は、決定の「決めたこと」と「確認すること」になる（`decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」）。用語の検査の対象であることは消える |
| REQ-core-064 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語の検査をやめた（`decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」） |
| REQ-core-065 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語の検査をやめた（`decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」） |
| REQ-core-066 | C-core-docs-18 | 用語集は人が読む文書として残し、用語の検査はやめる | 読み替え（提案）。曖昧語の一覧は設定の "vague_words"（config.md の「設定のキーは表のものだけにし、書かなかったキーは表の既定の値にする」）、種類の名前 vague_word と誤りであることは findings.md の「指摘の detail は、その指摘を特定する短い文字列にし、種類ごとに表で決める」の表にある。部分一致は利用者の決定で、書く先は依頼の外（下の節）。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-067 | C-core-docs-18 | 用語集は人が読む文書として残し、用語の検査はやめる | 読み替え（提案）。出現ごとに1件、最長一致で重ならないように数えることは利用者の決定（A53、A117、A142）で、書く先は依頼の外（下の節）。unknown_term の部分は消える。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-068 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。決定の文書は用語を囲まない。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-069 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文の中の裸の文書名を参照として読むのは IR の形。決定の文書はほかの決定をリンクで指し、無い先は C-core-docs-11 が誤りにする（`decision-documents.md` の「ほかの決定を指すときは、見出しへリンクを張る」） |
| REQ-core-070 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。missing_document は無くなり、無い文書を指すリンクの link_target_missing（C-core-docs-11）に置き換わる。今のコードの `check_links` がもうそう動く。見出しは `docs/decisions/core/decision-documents.md` のもの |
| TBL-core-013 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 読み替え（提案）。表の行（要求・性質の文、Gherkin の手順、用語集の意味の列、問題の記録の本文）は消え、対象は「決めたこと」と「確認すること」、対象外は「問題」と「見送った案」になる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| TBL-core-014 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文の中の裸の文書名を参照として読むのは IR の形。決定の文書はほかの決定をリンクで指し、無い先は C-core-docs-11 が誤りにする（`decision-documents.md` の「ほかの決定を指すときは、見出しへリンクを張る」） |
| EX-core-284 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語の検査の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-013 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語の検査の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-014 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-022 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-023 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-024 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-025 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-031 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-033 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-034 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語の検査と文書名の参照の両方の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-032 | - | ほかの決定を指すときは、見出しへリンクを張る | 消える。文書名の参照の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-026 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語の検査の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/document_references.rs | req_069_doc_ref_line_number_is_correct | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | req_069_doc_ref_at_line_end | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | req_069_doc_ref_at_line_start | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | req_069_doc_ref_after_punctuation | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | req_069_mdx_extension_not_matched_but_md_after_it_is | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_md_followed_by_letter_is_not_a_reference | TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_unclosed_quote_hides_the_rest_of_the_line | TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_text_between_the_second_and_third_quote_is_still_scanned | TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_long_digit_run_before_mdx_does_not_produce_a_spurious_reference | TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_md_followed_by_hyphen_is_not_a_reference | TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_bare_dot_md_with_nothing_before_it_is_not_a_reference | TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_md_followed_by_hash_or_slash_is_not_a_reference | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/document_references.rs | tbl_014_slash_separated_path_is_a_reference | REQ-core-069, REQ-core-070, TBL-core-014 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_063_only_sentences_and_steps_are_checked | REQ-core-063, TBL-core-013 | - | 古い形のコードと一緒に消す。unknown_term だけを確かめている |
| tests/step3_sources_terms.rs | req_064_unknown_term | REQ-core-064 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_064_a_quote_inside_a_backtick_pair_does_not_split_the_pair | REQ-core-064, EX-core-284 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_065_ids_pass_without_glossary | REQ-core-065 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_066_vague_word_substring | REQ-core-066 | C-core-docs-18 | 実装のときに直す。要求の文の曖昧語に vague_word が出ることを確かめる。準備を決定の「決めたこと」に直す。部分一致は利用者の決定で、C-core-docs-18 の決定に書き足す（下の節） |
| tests/step3_sources_terms.rs | req_067_one_finding_per_occurrence | REQ-core-067 | C-core-docs-18 | 実装のときに直す。出現ごとに1件は利用者の決定で、C-core-docs-18 の決定に書き足す（下の節）。準備を決定の文書に直す |
| tests/step3_sources_terms.rs | req_069_reference_needs_boundary_and_quotes_are_skipped | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_069_quoted_text_ending_in_a_multibyte_character_is_split_at_the_quote | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_069_dot_md_at_the_start_of_a_line_is_skipped_and_scanning_continues | REQ-core-069, TBL-core-014 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_070_missing_document | REQ-core-070 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_063_property_statements_and_scenario_steps_are_term_checked | REQ-core-063, REQ-core-064 | - | 古い形のコードと一緒に消す。unknown_term だけを確かめている |
| tests/step3_sources_terms.rs | req_064_backtick_content_is_trimmed | REQ-core-064 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_064_empty_backticks_are_unknown_term | REQ-core-064 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_067_overlapping_vague_words_longest_match_once | REQ-core-067 | C-core-docs-18 | 実装のときに直す。最長一致で1件は利用者の決定で、C-core-docs-18 の決定に書き足す（下の節）。準備を決定の文書に直す |
| tests/step3_sources_terms.rs | req_064_term_from_a_sibling_glossary_is_unknown | REQ-core-064 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_064_term_from_a_parent_glossary_is_visible | REQ-core-064 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_065_document_with_no_glossary_in_its_chain_flags_every_backtick | REQ-core-065, EX-core-026 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_070_bare_name_resolves_in_the_same_directory_only | REQ-core-070, TBL-core-014, EX-core-023 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_070_slash_path_resolves_from_the_ir_root | REQ-core-070, TBL-core-014, EX-core-024 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_070_dot_and_dotdot_elements_never_resolve | REQ-core-070, TBL-core-014, EX-core-025 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_070_detail_is_the_whole_reference | REQ-core-069, REQ-core-070, TBL-core-014, TBL-core-008, EX-core-014 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-008（findings。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_070_document_under_a_directory_symlink_is_missing | REQ-core-033, REQ-core-070, TBL-core-014, EX-core-032 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-033（ir-document。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | tbl_014_reference_after_a_japanese_character_is_recognized | REQ-core-069, TBL-core-014, EX-core-033 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | tbl_014_backticked_path_is_a_term_not_a_reference | REQ-core-064, REQ-core-069, TBL-core-014, EX-core-034 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_117_invalid_glossary_hides_only_its_own_terms | REQ-core-117, REQ-core-064 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-117（terms-form。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_068_unquoted_term_gets_no_finding_on_its_line | REQ-core-068 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_064_a_backticked_path_outside_the_glossary_is_an_unknown_term | REQ-core-064, EX-core-013 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_069_a_source_shaped_path_in_the_scope_line_is_not_a_reference | REQ-core-069, EX-core-022 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_069_md_followed_by_a_slash_is_not_a_reference | REQ-core-069, EX-core-031 | - | 古い形のコードと一緒に消す |

## ガイドのマーカー

次のガイドのマーカーが、この話題の ID を指している。付け替え先は元の ID の表の「決定の見出し」の列。見出しの列が「-」の ID や、消える指摘を説明する節は、ガイドの節ごと書き直すか消すことになる。

- `docs/guides/config.md` の 365 行: REQ-core-066
- `docs/guides/findings.ja.md` の 195 行: REQ-core-064, REQ-core-065, REQ-core-066, REQ-core-070
- `docs/guides/commands/check.md` の 335 行: REQ-core-064, REQ-core-066
- `docs/guides/findings.md` の 195 行: REQ-core-064, REQ-core-065, REQ-core-066, REQ-core-070
- `docs/guides/config.ja.md` の 358 行: REQ-core-066
- `docs/guides/commands/check.ja.md` の 333 行: REQ-core-064, REQ-core-066

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

無い（前の版でここにあった3件は、利用者の決定に数え直して下の節に移した）。

## 書く先が依頼の外にある利用者の決定

次の3つは、決定の文書の曖昧語の検査（`decision-documents.ja.md` の「用語集は人が読む文書として残し、用語の検査はやめる」、提案。C-core-docs-18）の細部として、その決定の「決めたこと」に書く必要がある。core の文書はこの依頼の外なので、ここに挙げて呼び出し元に戻す。今のコード（`crates/kotowari-core/src/terms.rs` の `check_vague_words`）はどれもこのとおりに動く。

| 細部 | 誰が決めたと記録にあるか | 今それを確かめているテスト |
|---|---|---|
| REQ-core-066: 曖昧語を、文の中の部分一致で見つける | 利用者（records.md の A41。第6ラウンド、答えは「推奨で」） | tests/step3_sources_terms.rs の req_066_vague_word_substring |
| REQ-core-067: 指摘を出現ごとに1件出す | 利用者（records.md の A53。第7ラウンド、答えは「基本的には細部の調整といったところで、推奨で問題無いと思う」） | tests/step3_sources_terms.rs の req_067_one_finding_per_occurrence |
| REQ-core-067: 重なる曖昧語は、行の左から最長一致で1件とし、同じ文字を2つの語に数えない | 利用者（records.md の A117、A142。第9ラウンド、答えは「進めて」） | tests/step3_sources_terms.rs の req_067_overlapping_vague_words_longest_match_once |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-063、TBL-core-013 | 新しい形で読み替えて残る。対象の行（要求と性質の文、Gherkin の手順）は、決定の「決めたこと」と「確認すること」になる（C-core-docs-18、C-core-docs-19）。用語の検査の対象であることと、用語集の意味の列・問題の記録の本文の行は消える |
| REQ-core-066 | 新しい形で読み替えて残る（C-core-docs-18）。検査する文が IR の文から決定の文に変わる |
| REQ-core-067 | 曖昧語の部分は新しい形で読み替えて残る（上の「書く先が依頼の外にある利用者の決定」）。unknown_term の部分は消える |
| REQ-core-064、REQ-core-065、REQ-core-068 | 消える。用語の検査をやめた |
| REQ-core-069、REQ-core-070、TBL-core-014 | 消える。文の中の裸の文書名を参照として読むのは IR の形で、決定の文書はほかの決定をリンクで指す。無い先は C-core-docs-11 の link_target_missing が見る。`translation-pairs.map.md` の REQ-core-342 の行も、文書名の参照の検査は消えると扱っている |

テストの無い行: 確認することの行を書いていないので無い。

## 誰が決めたか見分けられなかったもの

無い。

## 理由や見送った案が記録に無かった決定

書いた決定が無いので無い。

## 分けた所、まとめた所

無い。

## 実装する側が決める細部

C-core-docs-18 を作るときに決める。決定の文には書かない。

- vague_word の detail と line。今のコード（`crates/kotowari-core/src/terms.rs` の `check_vague_words`）は、見つけた語を detail に、語のある行を line にしている。`findings.md` の表は vague_word の detail を決める所を `decision-documents.md` としているが、そこに detail は書かれていない。
- 二重引用符の中とコードスパンの中の語を見るか。古い形の用語集の「除外」は、曖昧語の検査で二重引用符の中を見ないとしていた。見るなら、`config.md` の既定の4語を挙げた行に、その決定自身への指摘が出る。

## 利用者に聞くこと

無い。答えの出た問いは「迷った判断」に書いた。

## 「提案」で書いた決定の一覧

無い。曖昧語の検査を決定の文書に当てる作業は、`decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」（提案）の作業になる。`crates/kotowari-core/src/decisions.rs` で決定の「決めたこと」と「確認すること」の文を集め、`crates/kotowari-core/src/terms.rs` の `check_vague_words` にかけることになりそう。

## あとでリンクにする所

書き直した文書が無いので無い。

## 迷った判断と、どう決めたか

- 文書を作るかどうか。依頼は、曖昧語の部分を提案の決定として残し、C-core-docs-18、C-core-docs-19 で決まっていないこと（一覧の出どころ、出現ごとに1件）だけを足すと示していた。一覧の出どころは `config.md` と `translation-pairs.md` に、種類の名前と誤りであることは `findings.md` にすでにある。出現ごとに1件（A53）は、前の版では細部として落としたが、書き直しの決まりの読み方で利用者の決定に数え直した。それでも書く先は既にある `decision-documents.ja.md` の曖昧語の決定で、この話題に独立した決定は残らないので、確定の版でも文書を作らなかった（書く先が依頼の外にある3件は上の節に挙げた）。
- 文書名の参照（REQ-core-069、REQ-core-070）を利用者に聞くかどうか。決定の文書がほかの決定をリンクで指すと利用者が決めている（C-core-docs-11）ので、裸の文書名を読む検査は消えると決めた。リンクの検査は今のコードの `check_links` がもう行っている。
- REQ-core-063 の2つのテストは、名前に対象の行とあるが unknown_term だけを確かめているので、「-」にした。
- `finding-order.map.md` は TBL-core-019 の vague_word を「消える」としていたが、曖昧語の検査は C-core-docs-18 で残るので、確定の版で「読み替えて残る」に直した（finding-order の C-core-forder-8、提案）。
- 前の版の問い（既定の曖昧語の「など」が、承認済みの文書と書き直した文書に30か所ある）は、利用者が「決めたこと」と「確認すること」の「など」を無くすと決めたので消した。kotowari 自身の設定の "vague_words" は変えない。
- 部分一致、出現ごとに1件、最長一致（A41、A53、A117、A142）を、前の版は細部として落としていた。書き直しの決まりの読み方（第4〜9ラウンドの「推奨で」「進めて」は利用者の決定）で利用者の決定に数え直した。

## 数

- 元: 要求 8、場面 12、決定表 2、性質 0
- 読んだ記録のファイル: 4（records.md、ir-form.md、2026-09-23-ir-english-tokens.md、2026-10-05-localization.md）。消える部分だけの出典（2026-09-16-ir-tree.md、2026-09-17-check-reach.md、2026-09-22-ir-engine.md、2026-09-24-review5-gaps.md）は読んでいない
- 出来上がり: 決定 0、確認することの行 0
- テスト: 42
