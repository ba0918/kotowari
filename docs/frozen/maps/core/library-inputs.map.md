# library-inputs の対応表

元: `docs/ir/core/library-inputs.md`（日本語は `library-inputs.ja.md`）。書き直した文書: `library-inputs.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-314 | C-core-libinputs-14, C-core-libinputs-15 | 決定の文書の文字列を解析して、取れた決定と確認することを読み取り専用で返す公開の入口を置く | 読み替え。IR の部分的な解析（`ir::parse`、`IrOptions`、`IrDocument`）を、決定の文書の文字列を解析して決定と確認することを返す公開の入口に読み替えた（利用者の答え）。今のコードに無いので「提案」。名前と型は実装する側が決める |
| REQ-core-315 | C-core-libinputs-1, C-core-libinputs-2, C-core-libinputs-3, C-core-libinputs-4, C-core-libinputs-5, C-core-libinputs-6, C-core-libinputs-13, C-core-libpaths-9, C-core-libpaths-10 | 必須の入力の群が渡されなければ InputMissing を返し、空の群は通常の規則で検査する | そのまま。未提供と空の区別を最初の決定（C-1〜C-4、群の中のファイルの漏れを見つけないことを C-13）に、内部の取り込みでの重複と食い違いを library-paths の C-9、C-10 に、ファイルを読み足さないことと既存の規則の再現を2つ目の決定「メモリの入力の検査は、ファイルを読み足さず、…」（C-5、C-6）に分けた |
| REQ-core-316 | C-core-libinputs-9, C-core-libinputs-10 | 読み込んだ結果は使い回せ、明示して読み直すまで変わらない | そのまま。保存や自動監視をしないこと、同時点性を保証しないことは落とした細部 |
| REQ-core-317 | C-core-libinputs-11, C-core-libinputs-14 | 操作は、その操作が使う入力だけを読む | そのまま（ほとんど）。「IR だけの解析は他の入力群を要求しない」の部分は読み替えて、提案の決定「決定の文書の文字列を解析して、…」の C-14（文書の文字列だけで返る）に入れた。plan と mutants（library-api の EX-core-486）を同じ決定にまとめた |
| TBL-core-042 | C-core-libinputs-1, C-core-libinputs-2, C-core-libinputs-3, C-core-libinputs-7, C-core-libinputs-8 | 必須の入力の群が渡されなければ InputMissing を返し、空の群は通常の規則で検査する | 読み替え。決めたことの表にした。IR、判断の記録の置き場、ADR の3行は「仕様の文書」の1行に読み替えた。追加の指摘の群の行は、別の決定「名前を付けた追加の指摘の群を受け取り、…」（C-7、C-8）にした |
| EX-core-487 | C-core-libinputs-14, C-core-libinputs-15 | 決定の文書の文字列を解析して、取れた決定と確認することを読み取り専用で返す公開の入口を置く | 読み替え。IR の部分的な解析の場面を、決定の文書の入口が、ほかの入力なしに、取れたものと元の行と指摘を返す行に読み替えた。「ID を補わない、一覧の掲載条件で除かない」は落とした（設計担当の A38） |
| EX-core-488 | C-core-libinputs-1 | 必須の入力の群が渡されなければ InputMissing を返し、空の群は通常の規則で検査する | そのまま |
| EX-core-489 | C-core-libinputs-2 | 必須の入力の群が渡されなければ InputMissing を返し、空の群は通常の規則で検査する | そのまま |
| EX-core-490 | C-core-libinputs-9, C-core-libinputs-10 | 読み込んだ結果は使い回せ、明示して読み直すまで変わらない | そのまま。読んだときの内容を返すことを C-9、読み直しだけが変更を反映することを C-10 にした |
| EX-core-491 | C-core-libinputs-11 | 操作は、その操作が使う入力だけを読む | そのまま。「ガイドが読めない」は、メモリの入力では「ガイドを渡さない」に当たる。それを確かめるテストに合わせて行を書いた |
| EX-core-496 | C-core-libinputs-4 | 必須の入力の群が渡されなければ InputMissing を返し、空の群は通常の規則で検査する | そのまま |
| EX-core-497 | C-core-libinputs-3 | 必須の入力の群が渡されなければ InputMissing を返し、空の群は通常の規則で検査する | そのまま。表の条件に当たらない群の一般の行にまとめた |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari/tests/project.rs | project_operations_reuse_owned_results_and_explicit_reads_observe_changes | REQ-core-310, REQ-core-311, REQ-core-312, REQ-core-316, REQ-core-317, EX-core-483, EX-core-484, EX-core-490, EX-core-491 | C-core-libinputs-9, C-core-libinputs-10 | ファイルを変えたあとも ReadModel と Inspection が読んだときの内容を返し、読み直しと Project の操作だけが変更を返すことを確かめている。REQ-core-317、EX-core-491 の分（ガイドを読まない）は確かめていない。library-api の分はその表に書いた |
| crates/kotowari/tests/project.rs | public_admission_errors_keep_distinct_facade_classifications_and_details | REQ-core-312, REQ-core-315, REQ-core-322, EX-core-484, EX-core-488, EX-core-498 | C-core-libinputs-1 | 既定の ReadInputs から ReadModel::build を呼ぶと InputMissing になる。ほかの話題の分はそれぞれの表に書いた |
| crates/kotowari-core/tests/inputs.rs | partial_parsing_retains_a_nameless_requirement_and_its_heading_diagnostic | REQ-core-314, REQ-core-043, EX-core-290 | C-core-libinputs-15 | 実装のときに直す。名前の欠けた項目と見出しの指摘が取れることを確かめている。決定の文書の入口で、形に誤りのある文書から取れたものと指摘を返すテストに書き直す。REQ-core-043 と EX-core-290 の分はそれぞれの話題 |
| crates/kotowari-core/tests/inputs.rs | required_groups_are_distinct_from_explicit_empty_groups | REQ-core-315, REQ-core-317, EX-core-488, EX-core-489 | C-core-libinputs-1, C-core-libinputs-2, C-core-libinputs-3 | 必須の群が無ければ InputMissing、空で渡せば通る、tests.files が空ならテスト情報を求めない、を確かめている。REQ-core-317 の分は確かめていない |
| crates/kotowari-core/tests/inputs.rs | read_models_list_and_query_without_check_only_inputs | REQ-core-317, EX-core-491, REQ-core-312 | C-core-libinputs-11 | guides.files を設定してもガイドを渡さずに ReadModel を作れ、list と query が返る。REQ-core-312 の分は library-api の表に書いた |
| crates/kotowari-core/tests/inputs.rs | reading_takes_ir_sources_under_the_configured_place_only | REQ-core-314 | - | 古い形のコードと一緒に消す（IR の置き場の外の IR を拒む） |
| crates/kotowari-core/tests/inputs.rs | enabled_check_groups_must_be_provided_even_when_empty | REQ-core-315, EX-core-488, EX-core-489 | C-core-libinputs-1, C-core-libinputs-2 | ガイド、面、未記載の面の一覧が、条件が真なら1つずつ必須で、空で渡せば通る |
| crates/kotowari-core/tests/inputs.rs | supplied_analysis_preserves_diagnostics_counts_and_duplicate_suppression | REQ-core-308, REQ-core-315, EX-core-481, EX-core-496, EX-core-497 | C-core-libinputs-4, C-core-libinputs-6 | 解析の失敗で空のテスト情報の指摘が結果に残り、テストと面の両方にある同じ指摘が1つになり、数が数えられる。REQ-core-308 の分はクレート境界の話題で付け替える |
| crates/kotowari-core/tests/inputs.rs | disabled_groups_are_ignored_including_their_inconsistent_sources | REQ-core-315, EX-core-488, EX-core-489 | C-core-libinputs-3 | 条件が偽のテスト情報とガイドを渡しても結果に入らない |
| crates/kotowari-core/tests/inputs.rs | enabled_guide_and_surface_inputs_are_evaluated_without_acquisition | REQ-core-315, EX-core-488, EX-core-489, EX-core-497 | C-core-libinputs-5 | メモリで渡したガイドと面から、ファイルを読まずにガイドの古さと仕様の無い面の指摘が出る |
| crates/kotowari-core/tests/inputs.rs | partial_ir_retains_invalid_and_missing_ids_at_the_original_lines | REQ-core-314, REQ-core-317, EX-core-487 | C-core-libinputs-14, C-core-libinputs-15 | 実装のときに直す。ほかの入力なしに ir::parse を呼び、ID の不正な項目と欠けた項目が元の行と一緒に取れ、指摘が返ることを確かめている。決定の文書の入口のテストに書き直す |
| crates/kotowari-core/tests/inputs.rs | partial_parsing_reads_the_ir_place_from_the_options | REQ-core-314 | - | 古い形のコードと一緒に消す（IrOptions の IR の置き場と、置き場からの文書の種類） |
| crates/kotowari-core/tests/inputs.rs | nested_ir_paths_preserve_the_document_kind_of_the_filename | REQ-core-314, REQ-core-322, EX-core-487, EX-core-498 | - | 古い形のコードと一緒に消す（IR の文書の種類をファイル名で決める） |
| crates/kotowari-core/tests/inputs.rs | parsed_documents_retain_nested_paths_and_definition_and_statement_references | REQ-core-314, REQ-core-317, REQ-core-322, REQ-core-054, EX-core-487, EX-core-498 | - | 古い形のコードと一緒に消す（IR の項目の definition と本文の参照） |
| crates/kotowari-core/tests/inputs.rs | non_query_marker_facts_are_consumed_without_source_discovery | REQ-core-308, REQ-core-315, EX-core-481, EX-core-496 | C-core-libinputs-5 | メモリで渡したテスト情報のマーカーが、ソースを探さずに判定とマーカーの数に使われる。REQ-core-308 の分はクレート境界の話題 |
| crates/kotowari-core/tests/inputs.rs | each_enabled_group_is_required_independently_and_empty_is_provided | REQ-core-315, EX-core-488, EX-core-489 | C-core-libinputs-1, C-core-libinputs-2, C-core-libinputs-13 | 7つの群のどれを欠いても InputMissing、全部を空で渡せば通る。guides.files と surface.files を指定したまま空で渡して通ることが C-13 |
| crates/kotowari-core/tests/inputs.rs | same_text_shared_between_guides_and_tests_uses_the_overlap_rule | REQ-core-315, REQ-core-322, EX-core-488, EX-core-489, EX-core-499 | C-core-libinputs-6 | ガイドとテストの両方に当たるファイルが、ファイルから読むときと同じ重なりの誤りで止まる。REQ-core-322 の分は library-paths の表に書いた |
| crates/kotowari-core/tests/inputs.rs | an_additional_group_is_sorted_counted_and_reported_with_its_numbers | REQ-core-315, TBL-core-042, REQ-core-290 | C-core-libinputs-7 | 追加の群の指摘が並び、counts と status の数と complete に入り、群の名前と数が check と status に出る。REQ-core-290 の分は全体像の元データの話題 |
| crates/kotowari-core/tests/inputs.rs | an_additional_group_is_optional_and_only_added_when_given | REQ-core-315, TBL-core-042 | C-core-libinputs-8 | 追加の群を渡さなければ、群が出ない |
| crates/kotowari-core/tests/inputs.rs | two_additional_groups_with_one_name_are_invalid_input | REQ-core-315, TBL-core-042 | C-core-libinputs-7 | 落とした細部のテスト。いちばん近い行を指す（同じ名前の追加の群を2つ渡すと InvalidInput になる。元の文書にも記録にも無い細部） |
| crates/kotowari-core/tests/admission.rs | repeated_native_source_is_rejected_within_a_group | REQ-core-315, EX-core-490 | C-core-libpaths-9 | 同じ群に同じ元のファイルが2回あると拒む。library-paths の決定に戻した |
| crates/kotowari-core/tests/admission.rs | native_source_without_an_identity_is_rejected | REQ-core-315 | C-core-libpaths-9 | 落とした細部のテスト。いちばん近い行を指す（内部の取り込みで、元のファイルの識別が空なら拒む。元の文書にも記録にも無い細部） |
| crates/kotowari-core/tests/admission.rs | native_original_text_is_retained_across_read_and_check | REQ-core-315, EX-core-490 | C-core-libpaths-10 | 読むときと検査のときで、同じ元のファイルの内容が食い違うと拒む。library-paths の決定に戻した |
| crates/kotowari-core/tests/admission.rs | native_disabled_tests_do_not_contribute_stale_facts | REQ-core-315, EX-core-488, EX-core-489 | C-core-libinputs-3 | tests.files が空のときに渡したテスト情報が結果に入らない |
| crates/kotowari-core/tests/admission.rs | native_disabled_unspecified_list_does_not_hide_a_surface | REQ-core-315, EX-core-488, EX-core-489 | C-core-libinputs-3 | surface.unspecified が無いときに渡した未記載の面の一覧が判定に使われない |
| crates/kotowari-core/tests/admission.rs | native_display_collisions_do_not_merge_original_files | REQ-core-315, REQ-core-313, EX-core-490, EX-core-485 | C-core-libpaths-9 | 落とした細部のテスト。いちばん近い行を指す（表示が同じ別々の元のファイルを1つにしない。元の文書にも記録にも無い細部） |
| crates/kotowari-core/tests/admission.rs | native_read_preparation_retains_the_configured_ir_and_disabled_tests | REQ-core-311, REQ-core-315, REQ-core-317, EX-core-483, EX-core-491 | - | 古い形のコードと一緒に消す（設定した IR の置き場を保つ内部の読み込みの準備） |
| crates/kotowari-core/tests/admission.rs | native_inspection_cannot_finish_without_its_required_guides | REQ-core-315, EX-core-488, EX-core-489 | C-core-libinputs-1, C-core-libinputs-2 | 必須のガイドを渡さなければ終えられず、空で渡せば終えられる |
| crates/kotowari-core/tests/admission.rs | repeated_original_test_identity_cannot_be_evaded_by_display_spelling | REQ-core-315, EX-core-490 | C-core-libpaths-9 | 表示の綴りを変えても同じ元のファイルの重複は拒む。library-paths の決定に戻した |
| crates/kotowari-core/tests/admission.rs | a_guide_path_that_is_also_a_test_file_stops_the_native_inspection | REQ-core-199, REQ-core-315 | C-core-libinputs-6 | テストのファイルでもあるガイドのパスが、ファイルから読むときと同じ重なりの誤りで止まる。REQ-core-199 の分はガイドの話題 |
| crates/kotowari-source-analysis/tests/analyzer.rs | source_analysis_retains_source_facts_and_diagnostics_without_acquisition | REQ-core-308, REQ-core-315, EX-core-481, EX-core-496, EX-core-497 | - | Analyzer がファイルを探さずに文字列からテストと解析の失敗を返すことを確かめている。Inspection には渡しておらず、この話題の行には当たらない。library-crates の表で C-core-libcrates-6 を指すので、マーカーは無くならない |
| crates/kotowari-source-analysis/tests/analyzer.rs | cached_analysis_of_another_revision_is_rejected_at_native_admission | REQ-core-315, REQ-core-322, EX-core-496, EX-core-502 | C-core-libpaths-10 | 解析した本文と取り込んだ本文が食い違うと拒む。library-paths の決定に戻した（library-paths の表にも同じ行がある） |
| crates/kotowari-source-analysis/tests/analyzer.rs | public_surface_inspection_reports_uncovered_analyzer_facts | REQ-core-315, REQ-core-228, EX-core-497 | - | 面の判定の関数を直接呼んでいて、入力の群を通らない。この話題の行には当たらない。面の話題（REQ-core-228）の行を指すので、マーカーは無くならない |
| crates/kotowari-overview/tests/overview.rs | the_findings_and_numbers_become_the_overview_group | REQ-core-290, TBL-core-042 | - | kotowari-overview が全体像の指摘と数から "overview" の群を作ることを確かめている。この話題の行には当たらない。overview-output の対応表で C-core-ovout-1 を指すので、マーカーは無くならない |
| crates/kotowari-overview/tests/overview.rs | req_core_288_the_number_of_files_counts_every_overview_data_file | REQ-core-288, TBL-core-042 | - | 全体像の元データのファイルの数え方。この話題の行には当たらない。overview-output の対応表で C-core-ovout-1 を指すので、マーカーは無くならない |

C-core-libinputs-12 は、この話題の ID を指すテストではなく、library-api の EX-core-486 を指すテスト（project.rs の project_rejects_relative_roots_and_plan_does_not_require_ir と mutants_have_typed_inputs_and_failures_without_ir_loading）が確かめる。付け替えは library-api の表に書いた。

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-315: InputMissing が見つけるのは渡していない群で、群の中の個々のファイルの漏れは見つけない。どのファイルを渡すかの網羅は呼び出し側が持つ | C-core-libinputs-13 | テスト each_enabled_group_is_required_independently_and_empty_is_provided（guides.files と surface.files を指定したまま、群を空で渡して InputMissing にならない）。最初の決定の文と問題の節に足した |
| REQ-core-315: 同じ群の中の重複したファイルと、内部の取り込みでの元のファイルの食い違いを拒む | C-core-libpaths-9, C-core-libpaths-10 | テスト repeated_native_source_is_rejected_within_a_group、repeated_original_test_identity_cannot_be_evaded_by_display_spelling、native_original_text_is_retained_across_read_and_check、cached_analysis_of_another_revision_is_rejected_at_native_admission。論理パスの重複と同じ理由なので、library-paths の新しい決定「同じ論理パスの入力が重なったら、同じファイルとして扱えるときだけ受け付ける」に書き、最初の決定からリンクした |

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-316: ReadModel と Inspection は不変の値で、所有値は利用者が破棄するまで持ち、保存や自動監視はしない | 設計担当（A37）。利用者の A18 は「暗黙に読み直さず、反映には明示の読み直し」までで、それは書いた |
| REQ-core-316: ファイルの群を読み込む間に、同じ時点の内容であることは保証しない | 設計担当（A37） |
| TBL-core-042: 群ごとの中身の正確な並び（テスト情報の SourceText、問い合わせの有無、発見済みのテストとマーカー、解析のときの指摘など） | 設計担当（A49）と、言語の欄を外した整合のフェーズ（2026-10-24-next-release-bundle-consistency.md の A1）。名前と、表の形は残した（下の「迷った判断」） |
| TBL-core-042: 必須の条件（tests.files が空でない、guides.files が空でない、surface.rules が空でない、surface.unspecified が指定されている） | 設計担当（A50）。設定のキーは利用者から見える名前なので残した（下の「迷った判断」） |
| TBL-core-042: 判断の記録の置き場の文書には、判断の記録だけでなく出典の対象の通常の Markdown を含め、ADR とともに参照の有無によらず必須 | 設計担当（A49、A50）。形に依存する |
| REQ-core-314: 項目の行と終端の行、ID の欠けた項目を除かないこと、list と query の掲載の条件を変えないこと | 設計担当（A38）。形に依存する |

## 形に依存する要求

| 元の ID | 消える／読み替えて残る |
|---|---|
| REQ-core-314 | 読み替えて残る。利用者の決定（public-crate-api の A12「IR の項目・参照・位置を読み取り専用で取る API を出す。構文違反のある文書でも、取れた項目と指摘を参照できる」、目的は A3 のエディタ連携や独自ツール、A29「IR 解析だけの API は IR だけで利用できる」）を、決定の文書を解析する公開の入口に読み替えた（利用者の答え）。提案の決定「決定の文書の文字列を解析して、…」。IR の置き場（`IrOptions`）と、IR の項目の種類と参照は消える |
| REQ-core-317 | 「IR だけの解析は他の入力群を要求しない」の部分は、REQ-core-314 と一緒に読み替えて C-14 に入れた。残りは「操作は、その操作が使う入力だけを読む」に書いた |
| TBL-core-042 | IR、判断の記録の置き場、ADR の3行は、決定の文書の群に読み替えて残る（常に必須）。文書には「仕様の文書」の1行として書いた |
| EX-core-487 | REQ-core-314 と一緒に読み替えて、C-14、C-15 にした |

## テストの無い行

無い。C-core-libinputs-14、C-core-libinputs-15 は「提案」の決定の行で、テストを求めない（今の ir::parse のテスト2本を、実装のときに書き直してこの行へ付け替える）。

## 実装する側が決める細部

- 決定の文書を解析する公開の入口の名前、受け取る型と返す型、置くモジュール（今の `ir::parse`、`IrOptions`、`IrDocument`、`ParsedItem` に当たるもの）。利用者の答えで「名前と型は実装する側が決める」とされた。
- 返すものに、決定から別の決定の見出しへのリンク（元の A12 の「参照関係」に当たるもの）と、決定の状態を含めるか。A12 は IR の項目の参照関係を挙げていたが、決定の文書で何を返すかは決まっていない。
- 形の誤りの指摘の種類。今の check が決定の文書に出している指摘の名前を使う。

## 利用者に聞くこと

無い。前の下書きで残した REQ-core-314 の問い（決定の文書を読み取る API として残すか）は、利用者が「読み替えて残す」と答えた。

## 「提案」で書いた決定の一覧

| 決定 | 今のコードのどこを変えるか |
|---|---|
| 決定の文書の文字列を解析して、取れた決定と確認することを読み取り専用で返す公開の入口を置く | `crates/kotowari-core/src/decisions.rs` の解析を公開の入口として出し、`crates/kotowari-core/src/ir/partial.rs` の `ir::parse` と、`crates/kotowari-core/src/lib.rs` の `IrOptions`、`IrDocument`、`ParsedItem` の再公開、`crates/kotowari/src/lib.rs` の `kotowari::IrDocument` の再公開を置き換える。テストは `crates/kotowari-core/tests/inputs.rs` の ir::parse のテストを書き直す |

## 誰が決めたか見分けられなかったもの

- 「テスト情報と面の解析結果が持つ解析のときの指摘は、検査の結果に引き継ぐ」は設計担当の A49 の文だが、利用者の A29 の why（必要な情報の欠落を、検査の成功として扱わない）が裏付けるとして残した。EX-core-496 の確認することも同じ根拠。
- 「メモリの入力からファイルを自動取得しない」は設計担当の A39 だが、利用者の A6 の why（ファイルを保存せずに検査する）が裏付けるとして残した。
- 「同じ規則で指摘の重複の抑制、数え方、置き場の重なりを検査する」は設計担当の A49 だが、利用者の A9（検査の結果の意味を変えない）が裏付けるとして残した。

## 理由や見送った案が記録に無かった決定

- 「必須の入力の群が渡されなければ InputMissing を返し、…」: 見送った案が記録に無い。
- 「メモリの入力の検査は、ファイルを読み足さず、…」: 見送った案が記録に無い。
- 「読み込んだ結果は使い回せ、明示して読み直すまで変わらない」: 見送った案が記録に無い。
- 「操作は、その操作が使う入力だけを読む」: 見送った案が記録に無い。
- 「必須の入力の群が渡されなければ…」に足した「InputMissing は群の中のファイルの漏れを見つけない」: 理由が記録に無い（A50 の why は不足の判定を一意に決めることだけ）。問題の節に「理由は記録に残っていない」と書いた。
- 「決定の文書の文字列を解析して、…」（提案）: 見送った案が記録に無い。理由は A12 と A3 の why（エディタ連携や独自ツール）と、A29。

## 分けた所、まとめた所

- REQ-core-315 を2つの決定に分けた: 未提供と空の区別（A29）と、ファイルを読み足さず既存の規則で検査すること（A6、A9）。
- TBL-core-042 の「追加の指摘の群」の行を、別の決定にした。別の記録（2026-10-04-overview-on-public-api.md の A2）で、理由と見送った案を持つため。
- library-api の EX-core-486（plan と mutants は IR を読まない）を、REQ-core-317 と同じ決定「操作は、その操作が使う入力だけを読む」にまとめた。どちらも利用者の A19 から来る。
- EX-core-488、EX-core-489、EX-core-497 と、条件が偽の群のテストを、C-1〜C-3 の3行にまとめた。

## あとでリンクにする所

- 無い。前の下書きで言葉で指していた「ライブラリのクレート境界」の「core の型をそのまま再公開する決まり」は、`library-crates.ja.md#公開にするかは外の利用者が頼るかで決めkotowari-は入口から届く下位の型だけを同じ型のまま再公開する` へのリンクに直した。提案の決定の問題の節から、`decision-documents.ja.md#仕様は話題ごとの今有効な決定の文書だけで持つ`（リポジトリの core の文書）へリンクした。

## 迷った判断と、どう決めたか

- REQ-core-314（IR の部分的な解析）は、利用者の答え（IR を読む公開の入口は、決定の文書の文字列を解析して決定と確認することを返す公開の入口に読み替えて残す。名前と型は実装する側が決める。提案）に従い、提案の決定にした。決めたことには、A12（読み取り専用、位置、構文違反があっても取れたものと指摘）と A29（解析だけの入口はほかの入力を求めない）が裏付ける振る舞いだけを書き、名前と型は書かなかった。置き換えられる名前として `ir::parse` だけを残した。決定の文書を読むエンジン（mds を使うか）には触れていない。
- 戻した「InputMissing は群の中のファイルの漏れを見つけない」は、専用のテストが無い。each_enabled_group_is_required_independently_and_empty_is_provided が、対象のファイルの範囲（guides.files、surface.files）を設定したまま群を空で渡して通ることを確かめているので、これを付け替え先にした。C-core-libinputs-2（空で渡せば通常の規則で検査する）と同じテストだが、確かめてほしい点（漏れを見つけない、網羅は呼び出し側）が違うので、行を分けた。
- 内部の取り込みの重複と食い違い（安全の一覧の24）は、library-paths の論理パスの重複（27、28）と同じ理由の振る舞いなので、library-paths の新しい決定に1つにまとめ、この文書からはリンクした。
- ir::parse のテストは、取れたものと指摘を返すことを見る2本を「実装のときに直す」で C-14、C-15 に付け替え、IR の置き場、文書の種類、IR の参照を見る4本を「古い形のコードと一緒に消す」にした。native_read_preparation_retains_the_configured_ir_and_disabled_tests も IR の置き場を保つ準備なので「消す」にした。
- TBL-core-042 の必須の条件は設計担当（A50）が決めたものだが、設定のキー（tests.files、guides.files、surface.rules、surface.unspecified）と型の名前（ReadInputs、CheckInputs、SourceText）は利用者から見える名前なので、表の形のまま残した。振る舞いとしては、利用者の A29（必要な入力が無ければ不足を返す）と A19（list と query にガイドを求めない）が、「条件が真のときだけ必須」を裏付ける。
- 「仕様の文書」の行は、今の形では IR、判断の記録の置き場、ADR の3つの群だが、形によらない言い方で1行にした。実装済みの決定に新しい形の読み替えを書いたことになる。新しい版でも「仕様の文書の群は常に必須」が成り立つと見て、そうした。
- EX-core-491 の「ガイドが読めない」を確かめる Project の水準のテストは無い。メモリの入力でガイドを渡さない場合を確かめるテストがあるので、行をそちらに合わせた。

## 数

- 元: 要求 4、場面 7、決定表 1、性質 0
- 読んだ記録のファイル: 5（2026-10-03-public-crate-api.md、2026-10-04-overview-on-public-api.md、2026-10-06-changes-rethink.md、2026-10-23-next-release-bundle.md、2026-10-24-next-release-bundle-consistency.md）
- 出来上がり: 決定 6（実装済み 5、提案 1）、確認することの行 15（うち「（読んで確認）」0、提案の行 2）
- テスト: 35
- 戻した細部: 2（1件はこの文書の C-13、1件は library-paths の C-9、C-10）
