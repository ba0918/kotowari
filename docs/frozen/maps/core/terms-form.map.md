# terms-form の対応表

元: `docs/ir/core/terms-form.md`（日本語は `terms-form.ja.md`）。短い名前は termsform（確認することの ID を付けるなら `C-core-termsform-<n>`）。

書き直した文書（`terms-form.md`）は作らなかった。この話題の要求はどれも、用語をバッククォートで囲む決まりと、用語集の表の形の検査についてのもので、`docs/decisions/core/decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」で役目が無くなった。決定の文書は用語を囲まず、check は用語集 `CONTEXT.md` の中身を検査しない（C-core-docs-20）。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-104 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。値を二重引用符で書き、バッククォートを用語と ID に限る決まりは、用語の検査のためのもの。決定の文書は用語を囲まない。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-116 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。unclosed_backtick は用語と ID の囲みの検査のためのもの。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-117 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。check は用語集 `CONTEXT.md` の中身を検査しない（C-core-docs-20）。glossary_invalid、invalid_glossary_row、duplicate_term は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-122 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。check は用語集 `CONTEXT.md` の中身を検査しない（C-core-docs-20）。glossary_invalid、invalid_glossary_row、duplicate_term は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| REQ-core-123 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。check は用語集 `CONTEXT.md` の中身を検査しない（C-core-docs-20）。glossary_invalid、invalid_glossary_row、duplicate_term は無くなる。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-027 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語集の表の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-270 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語集の表の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-271 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語集の表の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |
| EX-core-272 | - | 用語集は人が読む文書として残し、用語の検査はやめる | 消える。用語集の表の場面。見出しは `docs/decisions/core/decision-documents.md` のもの |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/ir.rs | req_054_backtick_oddness_counted_outside_quotes_only | REQ-core-054, REQ-core-116 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-054（ir-references。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari-core/tests/ir.rs | req_122_glossary_row_with_missing_column_is_invalid | REQ-core-122 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_122_glossary_row_with_empty_term_cell_is_invalid | REQ-core-122 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_123_duplicate_term_reported_for_second_row_onward | REQ-core-123 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_122_two_cell_row_with_trailing_pipe_is_invalid | REQ-core-122 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_122_three_cells_without_trailing_pipe_is_a_term | REQ-core-122 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_123_duplicate_row_is_not_a_term | REQ-core-123 | - | 古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_117_non_dash_row_after_header_is_not_a_valid_separator | REQ-core-117 | - | 古い形のコードと一緒に消す |
| crates/kotowari/src/ir_tests.rs | req_033_context_and_flags_in_a_subdirectory_are_glossary_and_flags | REQ-core-033, REQ-core-036, REQ-core-117 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-033、REQ-core-036（ir-document。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| crates/kotowari/src/ir_tests.rs | tbl_008_whole_document_detail_is_the_bare_filename_in_a_subdirectory | TBL-core-008, REQ-core-034, REQ-core-036, REQ-core-117 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-008（findings。対応表あり）、REQ-core-034、REQ-core-036（ir-document。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_104_quoted_values_are_not_terms | REQ-core-104 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_116_odd_backticks_skip_terms_but_check_vague_words | REQ-core-116 | - | 古い形のコードと一緒に消す。閉じないバッククォートの行でも曖昧語を見ることは、囲みの検査が無くなると意味を持たない |
| tests/step3_sources_terms.rs | tbl_008_unclosed_backtick_detail_keeps_leading_indentation | REQ-core-116, TBL-core-008 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-008（findings。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_117_second_table_is_not_glossary | REQ-core-117 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_117_glossary_without_proper_table_is_invalid | REQ-core-117 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_117_glossary_header_without_rows_is_valid | REQ-core-117 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | req_123_duplicate_across_the_chain_is_reported_on_the_deeper_row | REQ-core-123, TBL-core-019 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: TBL-core-019（finding-order。対応表あり）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_117_invalid_glossary_hides_only_its_own_terms | REQ-core-117, REQ-core-064 | - | この話題の分は消える（マーカーから外す）。ほかの話題の ID も指す: REQ-core-064（terms。同じ依頼で対応表を書いた）。テストを残すかどうかはその話題の対応表で決まる |
| tests/step3_sources_terms.rs | req_123_same_term_above_and_below_the_chain_is_reported_on_the_lower_row | REQ-core-123, EX-core-027 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | ex_core_270_the_first_table_with_the_header_is_the_glossary_table | EX-core-270 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | ex_core_271_a_glossary_without_a_table_with_the_header_is_invalid | EX-core-271 | - | 古い形のコードと一緒に消す |
| tests/step3_sources_terms.rs | ex_core_272_a_row_with_four_cells_becomes_a_term_from_its_first_three | EX-core-272 | - | 古い形のコードと一緒に消す |

## ガイドのマーカー

次のガイドのマーカーが、この話題の ID を指している。付け替え先は元の ID の表の「決定の見出し」の列。見出しの列が「-」の ID や、消える指摘を説明する節は、ガイドの節ごと書き直すか消すことになる。

- `docs/guides/findings.ja.md` の 195 行: REQ-core-116
- `docs/guides/findings.ja.md` の 216 行: REQ-core-117, REQ-core-122, REQ-core-123
- `docs/guides/findings.md` の 195 行: REQ-core-116
- `docs/guides/findings.md` の 216 行: REQ-core-117, REQ-core-122, REQ-core-123

## 落とした細部

無い。どの要求も消える。

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-104 | 消える。値を二重引用符で書き、バッククォートを用語と ID に限る決まりは、用語の検査を成り立たせるためのもの |
| REQ-core-116 | 消える。unclosed_backtick は無くなる。閉じないバッククォートの行でも曖昧語を見ることは、囲みの検査が無いので意味を持たない |
| REQ-core-117、REQ-core-122、REQ-core-123 | 消える。glossary_invalid、invalid_glossary_row、duplicate_term は無くなる。用語集の置き場は `docs/decisions/<まとまり>/CONTEXT.md` になる |

テストの無い行: 確認することの行を書いていないので無い。

## 消えない振る舞い

無い。用語集の対の骨組みに何を比べるか（古い形では表の行の数と出典の列）は、`translation-pairs.map.md` の「利用者に聞くこと」ですでに問いになっているので、ここでは足さない。

## 数

- 元: 要求 5、場面 4、決定表 0、性質 0
- 読んだ記録のファイル: 0（どの要求も消え、置き換え先は `docs/decisions/core/decision-documents.md` で決まっている）
- 出来上がり: 決定 0、確認することの行 0
- テスト: 22
