# finding-map の対応表

元: `docs/ir/core/finding-map.md`（日本語は `finding-map.ja.md`）。書き直した文書: `finding-map.md`。

書き直した文書には決定が1つも無い。この話題の要求・決定表・場面はどれも、エンジンが古い形の文書（IR の話題ごとの文書、用語集、問題の記録）について返した指摘を写すためのもので、形に依存する要求である。`crates/kotowari-core/src/finding_map.rs` の対応を使うのは `crates/kotowari-core/src/ir.rs`（IR の読み取り）だけで、計画書の検査は対応表を通らずに invalid_plan を出す（元の REQ-core-193）。形の切り替えの決まり（役目が無くなる話題は書き直さずに凍結した木へ移す）に当たる話題なので、書き直した文書は題名と冒頭の段落だけにした。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-171 | - | - | 対応表そのもの。写す元の文書（IR の文書と用語集）が無くなるので消える |
| REQ-core-172 | - | - | 写し先の無い指摘で停止し、黙って捨てない。対応表が無くなるので消える。エンジンの指摘を捨てないことは、計画書では plan の話題の「形の指摘は invalid_plan の1種類にまとめ、スキーマの側の種類と詳細を detail に入れる」（スキーマの側の指摘を1件ごとに invalid_plan にし、捨てない）に残る |
| TBL-core-029 | - | - | 対応表の列。消える |
| TBL-core-030 | - | - | 対応表の行。消える |
| EX-core-265 | - | - | 写し先の無い指摘で停止する場面。消える |
| EX-core-278 | C-core-forder-4, C-core-document-5 | 決定の文書の題名は1つにし、2つ目からの題名は multiple_titles の誤りにする | 題名が3つある IR の文書で、2つ目以降の題名ごとに multiple_titles が出て line が null になる場面。2つ目以降の題名ごとに1件出すことは ir-document の C-core-document-5（実装済み）、line が null の部分は finding-order の「line は1始まりで数え、文書全体にかかる指摘は null にする」（C-core-forder-4）に当たる |
| EX-core-279 | - | - | 用語集の表にならなかった表を捨てる場面。用語集が無くなるので消える |

元の文書に PROP は無い。ガイドにこの話題の ID のマーカーは無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/src/finding_map.rs | missing_title_becomes_missing_title_with_the_document_name | REQ-core-171 | - | 対応表の1行の写し方。形に依存する要求で、IR の読み取りと一緒に消える |
| crates/kotowari-core/src/finding_map.rs | multiple_titles_becomes_multiple_titles_with_that_title | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | title_pattern_mismatch_in_the_glossary_becomes_glossary_title_invalid | REQ-core-171 | - | 同上（用語集） |
| crates/kotowari-core/src/finding_map.rs | heading_findings_become_unknown_heading_with_the_raw_line | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | undeclared_list_lines_become_unknown_field_with_the_raw_line | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | undeclared_statements_tables_and_code_blocks_become_unknown_line | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | a_missing_field_line_splits_by_the_name_of_the_field | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | a_field_value_outside_the_allowed_list_is_repointed_at_the_item_heading | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | a_glossary_data_row_with_the_wrong_columns_becomes_invalid_glossary_row | REQ-core-171 | - | 同上（用語集） |
| crates/kotowari-core/src/finding_map.rs | a_glossary_table_header_mismatch_at_the_table_start_stops | REQ-core-171 | - | 同上（用語集） |
| crates/kotowari-core/src/finding_map.rs | an_undeclared_table_in_the_glossary_is_dropped_and_elsewhere_is_unknown_line | REQ-core-172 | - | 「出さない」の行で捨てること。同上 |
| crates/kotowari-core/src/finding_map.rs | a_code_block_with_the_wrong_language_becomes_unknown_code_block | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | a_statement_below_its_minimum_splits_by_whether_the_finding_has_a_line | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | a_table_below_its_minimum_splits_by_whether_the_finding_has_a_line | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | a_field_line_above_its_maximum_becomes_duplicate_field_with_the_field_name | REQ-core-171 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | a_kind_whose_row_says_it_cannot_happen_stops | REQ-core-172, EX-core-265 | - | 「発生しない」の行で停止すること。同上 |
| crates/kotowari-core/src/finding_map.rs | a_kind_whose_row_is_chosen_by_a_name_it_does_not_have_stops | REQ-core-172, EX-core-265 | - | 行の無い種類で停止すること。同上 |
| crates/kotowari-core/src/finding_map.rs | a_table_header_mismatch_outside_the_glossary_stops_even_on_a_glossary_row_line | REQ-core-172 | - | 同上 |
| crates/kotowari-core/src/finding_map.rs | the_item_of_a_finding_is_the_last_one_that_starts_at_or_above_its_line | REQ-core-171 | - | 指摘の行から IR の項目を引くこと。同上 |
| tests/step2_ir.rs | ex_core_278_three_titles_give_one_multiple_titles_per_extra_title | EX-core-278 | C-core-forder-4, C-core-document-5 | 2つ目以降の題名ごとに multiple_titles が1件ずつ出て、line がどれも null であることを確かめている。入力は古い形の IR の文書なので、切り替えのときに入力を決定の文書に直す（決定の文書では step23 の each_title_after_the_first_is_an_error_without_a_line がすでに確かめている） |
| tests/step3_sources_terms.rs | ex_core_279_a_table_that_did_not_become_the_glossary_table_is_dropped | EX-core-279 | - | 用語集の表。形に依存する要求で、古い形のコードと一緒に消す |

`finding_map.rs` の `a_value_that_cannot_be_mapped_stops_with_the_mapping_error_wording` は REQ-core-175（別の話題）だけを指すので載せていない。

## 落とした細部

### 安全に関わるもの

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-172: エンジンから写し先の無い種類の指摘か、「発生しない」と書いた種類の指摘を受けたら停止し、黙って捨てない。「出さない」で捨ててよいのは除外に列挙した入力だけ | 利用者（2026-09-22-ir-engine.md の A28、2026-09-23-ir-engine-gaps.md の A27・A36。推奨を採用）。理由は「黙って捨てると、将来スキーマに宣言を足したときに検査が静かに緩む」。落としたのは細部だからではなく、写す元の文書が無くなるため（形に依存する要求）。エンジンの指摘を捨てないことは、計画書では plan の話題の invalid_plan の決定（C-core-plan-19）に残る。今後、決定の文書などをエンジンで読み、その指摘を kotowari の種類へ写すことにするなら、この決まりを新しい決定として立て直す必要がある |

### そのほか

無い。この話題の中身は細部として落としたのではなく、すべて形に依存する要求として消える（下の表）。

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-171 | 消える。IR の文書と用語集をエンジンで読むことが無くなる |
| REQ-core-172 | 消える。写す対応表が無くなる。捨てない決まりは計画書の側で読み替えて残る（上の「安全に関わるもの」） |
| TBL-core-029 | 消える |
| TBL-core-030 | 消える |
| EX-core-265 | 消える |
| EX-core-278 | 新しい形で読み替えて残る。決定の文書の2つ目からの題名の multiple_titles は ir-document の C-core-document-5（実装済み、539b7c1）、line が null の部分は finding-order の C-core-forder-4 |
| EX-core-279 | 消える |

テストの無い行: 無い（確認することの行が無い）。

## 誰が決めたか見分けられなかったもの

無い。ir-engine と ir-engine-gaps の読んだ決定は decided_by が利用者（推奨を採用）で、records.md の A92 は実験者の裁定、A100・A101 は第9ラウンドの本体で、書き直しの決まりの読み方では利用者の決定（推奨をまとめて採用）に数える。どれも形に依存して消えるので、誰が決めたかで扱いは変わらない。A100（黙って見逃さない）は、cli-environment の「読めない入力と壊れた入力は、停止か誤りの指摘にし、黙って読み飛ばさない」に残っている。

## 理由や見送った案が記録に無かった決定

無い（決定が無い）。

## 分けた所、まとめた所

無い。

## あとでリンクにする所

- 書き直した文書から plan の話題へはリンクにした（同じ依頼の話題）。
- 済み: 対応表の理由の中の「指摘の種類の話題」は、2つ目以降の題名ごとに1件出すことが ir-document の「決定の文書の題名は1つにし…」に決まったので、そちらを指すように直した。

## 迷った判断と、どう決めたか

- 決定が1つも無い文書を書くかどうか。依頼どおり2つのファイルを書き、文書は題名と、なぜ決定が無いかを書いた冒頭の段落だけにした。決まりの「形に依存する要求」は文書に書かず対応表に書くので、この話題は全部が対応表に行く。形の切り替えの決まりでは、役目が無くなる話題は書き直さずに凍結した木へ移すので、この文書は置かずに凍結する扱いが合っている、と考える。どちらにするかは依頼した側で決めてほしい。
- エンジンが今も使われている所（計画書の検査、全体像の元データの検査）で、この話題の決まりが残るかを確かめた。計画書は REQ-core-193 が「TBL-core-030 で写さない」と決めていて、全体像の元データの指摘は overview_form_invalid などの全体像の種類で出す（全体像の話題）。どちらもこの対応表を使わないので、残る部分は無いと判断した。コードでも、対応表を使うのは IR の読み取り（`ir.rs`）だけである。
- REQ-core-172 の「黙って捨てない」は安全に関わる決まりなので、形に依存して消えるものでも、安全に関わるものの表に載せ、計画書の側のどこに残るかを書いた。
- 記録の why にある「mds」と配布した skill の `findings.md` は、対応表の中では触れなかった（決定が無いので文書にも出ない）。

## 数

- 元: 要求 2、場面 3、決定表 2、性質 0
- 読んだ記録のファイル: 4（2026-09-22-ir-engine.md（A2、A10、A28、A40）、2026-09-23-ir-engine-gaps.md（A7、A17、A25、A27、A36）、records.md（A92、A100、A101）、2026-09-24-plan-schema.md（A11））。TBL-core-029 と TBL-core-030 の出典のうち、行ごとの細部だけを支える決定（ir-engine の A3、A24、A29、A31、A71〜A87 ほか、2026-09-23-ir-english-tokens.md の A2、ir-form.md の「検査の種類」）は、どれも形に依存して消えるので読んでいない
- 出来上がり: 決定 0、確認することの行 0
- テスト: 21。新しい ID を持つもの 1（finding-order の C-core-forder-4 と ir-document の C-core-document-5）、「-」20
