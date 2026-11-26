# ir-input の対応表

元: `docs/ir/core/ir-input.md`（日本語は `ir-input.ja.md`）。書き直した文書: `ir-input.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-111 | C-core-input-1, C-core-input-2 | 先頭の UTF-8 の BOM は読み飛ばし、UTF-8 でないファイルとして停止しない | そのまま。records.md の A107（第9ラウンド、利用者が推奨をまとめて採用）。決定の文書でも、今のコードは UTF-8 として読んだあとに BOM を除く（`crates/kotowari/src/acquisition.rs` の `read_utf8_file`）。step23 と step24 のテストが確かめる |
| REQ-core-112 | C-core-input-3, C-core-input-4 | 閉じないコードブロックは unclosed_code_block の誤りにし、開始から文書の終わりまでを読まない | 読み替え（実装済み）。records.md の A108、A140（第9ラウンド）、2026-09-22-ir-engine.md の A23（利用者）。決定の文書を読むコードは d422662、79ba0e8 で閉じないコードブロックを unclosed_code_block にした（`crates/kotowari-core/src/decisions.rs` の `unclosed_code_blocks`） |

元の文書に場面、決定表、性質は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/ir.rs | req_111_bom_is_skipped_in_ir_config_records_adr_and_tests | REQ-core-111 | - | IR の読み取りに BOM 付きの文書を渡すだけのテスト。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/translations.rs | req_core_346_a_switcher_after_a_byte_order_mark_does_not_count_as_document_scope | REQ-core-346, REQ-core-111 | - | この話題の分は IR の文書の BOM で、古い形のコードと一緒に消える。切り替えの行の分は対の話題（REQ-core-346）が付け替える |
| tests/step3_sources_terms.rs | req_111_bom_in_config_records_adr_and_tests_is_skipped_end_to_end | REQ-core-111 | C-core-input-1 | 設定ファイルとテストのファイルの BOM を確かめる。判断の記録、ADR、IR の分は消えるので、付け替えのときに準備から外す |
| tests/step9_mutants.rs | req_111_source_with_a_bom_still_matches_on_its_first_line | REQ-core-111, REQ-core-141 | C-core-input-1 | この話題の分だけ。変異の結果のソースの BOM。REQ-core-141 は等価の一覧の話題 |
| tests/step23_decision_documents.rs | a_leading_byte_order_mark_is_skipped | C-core-docs-1 | C-core-docs-1, C-core-input-2 | 先頭に BOM のある決定の文書で、題名の誤りが出ず、決定が1つ数えられる |
| tests/step24_decision_pairs.rs | a_leading_byte_order_mark_on_either_side_is_skipped | C-core-docs-1 | C-core-docs-1, C-core-input-2 | 対の両側の先頭の BOM を読み飛ばし、指摘が出ない |
| tests/step23_decision_documents.rs | an_unclosed_code_block_is_an_error_at_its_opening_line_and_check_fails | C-core-docs-1 | C-core-docs-1, C-core-input-3, C-core-input-4 | 文書の直下の閉じないコードブロックに、開始の行と行の文字の unclosed_code_block が出て終了コード1。後ろの決定の確認することの行に confirmation_without_test が出ない（数えない） |
| tests/step23_decision_documents.rs | a_closed_code_block_is_no_error_and_the_decision_after_it_is_read | C-core-docs-1 | C-core-docs-1, C-core-input-3 | 閉じたコードブロックには誤りが出ない（C-3 の裏側） |
| tests/step23_decision_documents.rs | an_unclosed_code_block_under_a_confirmation_row_is_an_error_and_the_next_rows_are_still_read | C-core-docs-1 | C-core-docs-1, C-core-input-3, C-core-input-4 | 一覧の項目の中の閉じないコードブロックにも誤りが出て、detail は字下げを含む行の文字。開始より前の行が数えられる（C-4 の前半）。項目の外の行が読まれることは実装する側が決める細部（下） |
| tests/step23_decision_documents.rs | closed_and_indented_code_blocks_are_no_error | C-core-docs-1 | C-core-docs-1, C-core-input-3 | 閉じたコードブロックと字下げのコードブロックには誤りが出ない（C-3 の裏側） |
| tests/step23_decision_documents.rs | a_fence_on_the_last_line_of_a_document_is_unclosed | C-core-docs-1 | C-core-docs-1, C-core-input-3 | 最後の行の開始の行も閉じないコードブロックになる |
| tests/step24_decision_pairs.rs | an_unclosed_code_block_on_the_japanese_side_is_an_error_on_that_side | C-core-docs-1 | C-core-docs-1, C-core-input-3 | 対のほかの言語の側の閉じないコードブロックにも、その側のパスで誤りが出る |
| crates/kotowari-core/tests/ir.rs | tbl_core_010_bare_cr_keeps_items_before_an_unclosed_fence | TBL-core-010, REQ-core-112 | - | IR の読み取りのテスト。決定の文書では step23 のテストが確かめるので、古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_112_unclosed_code_block_is_an_error | REQ-core-112 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_112_unclosed_gherkin_block_is_not_checked | REQ-core-112 | - | gherkin のブロックと場面は消える。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_112_unclosed_gherkin_block_with_multiple_scenarios_excludes_items | REQ-core-112, REQ-core-040 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_112_findings_and_items_before_the_unclosed_fence_are_retained | REQ-core-112 | - | IR の読み取りのテスト。古い形のコードと一緒に消す |
| crates/kotowari-core/tests/ir.rs | req_112_the_opening_line_of_an_unclosed_fence_is_excluded_too | REQ-core-112 | - | 同上 |
| crates/kotowari-core/tests/ir.rs | req_112_a_fence_closes_only_with_at_least_as_many_marks | REQ-core-112 | - | 同上。閉じる行の長さの規則は CommonMark の読み取りの側にあり、step23 の closed_and_indented_code_blocks_are_no_error が "`````" で閉じる場合を確かめる |

## 戻した細部

無い。安全に関わる落とした細部はこの話題に無い。

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-112: コードブロックの境界を、行頭の3つ以上の "`" か "~" で始まる行とし、同じ文字で同じ数以上の行で閉じる | 利用者（records.md の A108、第9ラウンド）。決定の文書は Markdown の読み取り（CommonMark）がコードブロックを決めるので、この規則は読み取りの側にある。決定の文には書かなかった |
| REQ-core-112: 閉じない gherkin のブロックも一律に対象から外し、その中の "@id" を定義に数えない | 利用者（records.md の A140）。gherkin のブロックと場面が消えるので、形の細部として消える |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-112 | 新しい形で読み替えて残る（実装済み）。gherkin のブロックの部分と、その中の項目の定義の扱いは消える。開始から文書の終わりまでを読まないことは、決定の文書では決定と確認することの行を読まないことと読む |

### テストの無い行

無い。C-core-input-2 は step23 と step24 の BOM のテストが確かめるようになった。

## 誰が決めたか見分けられなかったもの

- records.md の第9ラウンドの本体（A107、A108、A140）は、決定の行ごとの decided_by が無く、見出しに「すべて推奨をそのまま採った（利用者の答えは「進めて」）」とある。BRIEF-C の読み方どおり、利用者の決定に数えた。

## 理由や見送った案が記録に無かった決定

- 「先頭の UTF-8 の BOM は読み飛ばし…」: 理由も見送った案も記録に無い。問題の節に「理由は残っていない」と書いた。
- 「閉じないコードブロックは unclosed_code_block の誤りにし…」: 見送った案が記録に無い。理由は、A23 の why（Markdown の読み取りは閉じないコードブロックを検出せず、文書の終わりまで伸ばす）と、黙って読み飛ばすと決定が数えられなくなること（A100 の「黙って見逃さない」）を書いた。

## 分けた所、まとめた所

- REQ-core-111 を、設定ファイル・テストのファイル・変異の結果のソース（C-1）と、決定の文書（C-2）の2行に分けた。
- REQ-core-112 は、誤りを出すこと（C-3）と、範囲を読まないこと（C-4）の2行にした。

## あとでリンクにする所

- ほかの文書からこの文書へ: equivalents.map.md が「IR の入力」の話題として言葉で指している、変異の結果のソースの BOM の読み飛ばし（元の REQ-core-111）は、この文書の「先頭の UTF-8 の BOM は読み飛ばし…」へのリンクにできる（equivalents は別の作業者の話題なので、ここでは直していない）。

## 迷った判断と、どう決めたか

- 閉じないコードブロックの決定を「提案」から「実装済み」にした。d422662 と 79ba0e8 で、決定の文書の閉じないコードブロックに unclosed_code_block が出るようになり、step23 と step24 のテストが確かめている（コードで確かめた: `crates/kotowari-core/src/decisions.rs` の `unclosed_code_blocks`）。問題の節の「今のコードは何も出さない（実測）」は事実でなくなったので、何も出さなければ何が起きるかの文に直した。
- 一覧の項目の中で閉じないコードブロックは、今のコードでは項目の終わりまでを呑み、項目の外の行は読まれる（step23 の an_unclosed_code_block_under_a_confirmation_row_… が確かめる）。A108 が決めたのは「開始から文書の終わりまで」で、項目の中の場合は Markdown の読み取りの決め方なので、読まない範囲の文は「文書の直下のコードブロック」に絞り、項目の中の場合は実装する側が決める細部に置いた。誤りを出すことは、どちらの場合にも当てた。
- detail の「開始の行の文字をそのまま（字下げを含む）」は、A108 の「detail は開始の行の文字」と A111 の「行の文字はそのまま、字下げを含む」（第9ラウンド、利用者）から書いた。
- 終了コードが1になることを C-3 に入れた。誤りであることから導ける（[指摘は誤りと注意に分け、注意にするのは表の種類だけにする](findings.ja.md#指摘は誤りと注意に分け注意にするのは表の種類だけにする)）が、新しいテストが確かめているので行に書いた。

## 実装する側が決める細部

- 閉じないコードブロックの見つけ方。今のコードは、読み取りの結果のコードブロックの元の文字の最後の行が、開始と同じ文字の開始以上の長さの並びかで見分ける。
- 一覧の項目や引用の中にある閉じないコードブロックが、どこまでを呑むか（CommonMark が決める）。今のコードは項目の終わりまでを呑み、その外の行を読む。

## 利用者に聞くこと

無い。

## 「提案」で書いた決定の一覧

無い。

## 数

- 元: 要求 2、場面 0、決定表 0、性質 0
- 読んだ記録のファイル: 2（records.md、2026-09-22-ir-engine.md）
- 出来上がり: 決定 2（実装済み 2、提案 0）、確認することの行 4（うち「（読んで確認）」0）
- テスト: 19（付け替え先のあるもの 10、「-」9）。このうち、この話題の ID を今は指さず C-core-docs-1 だけを指すもの 8
- 利用者に聞くこと: 0
- 戻した安全の細部: 0
