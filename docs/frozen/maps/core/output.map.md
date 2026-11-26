# output の対応表

元: `docs/ir/core/output.md`（日本語は `output.ja.md`）。書き直した文書: `output.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-021 | C-core-output-1, C-core-output-2 | "--format" は json と text の2つを受け、既定を json にする | そのまま |
| REQ-core-022 | C-core-output-1, C-core-output-2 | "--format" は json と text の2つを受け、既定を json にする | そのまま |
| REQ-core-023 | C-core-output-3, C-core-output-4, C-core-output-6 | check の JSON の最上位には、指摘と種類ごとの数と、読んだものの数を出す | 読み替え。JSON の中身の表（TBL-core-005、TBL-core-006）と性質（PROP-core-002、PROP-core-004）を指す要求。IR の文書の数と行数（files、lines）は、読んだ決定の文書の数（decisions の documents）に読み替えた |
| REQ-core-025 | C-core-output-8, C-core-output-10 | --format text は、1つの指摘を1行で出す | そのまま |
| REQ-core-026 | C-core-output-9 | --format text は、1つの指摘を1行で出す | そのまま |
| REQ-core-128 | C-core-output-3, C-core-output-11, C-core-output-12 | check の JSON の "tests" は、読んだテストのファイルを拡張子ごとに数える | そのまま。text に出さないことは C-11 |
| REQ-core-206 | C-core-output-3, C-core-output-5 | check の JSON の最上位には、指摘と種類ごとの数と、読んだものの数を出す | そのまま。"files" の数え方と、text に出さないことは、誰が決めたか見分けられないので落とした |
| TBL-core-005 | C-core-output-3 | check の JSON の最上位には、指摘と種類ごとの数と、読んだものの数を出す | 読み替え。files と lines の行を、decisions の行に読み替えた。mutants と plan の最上位は、それぞれの話題の文書へのリンクにした |
| TBL-core-006 | C-core-output-6, C-core-output-7 | 指摘は kind、severity、path、line、detail の5つの鍵で出す | 読み替え。path の行の「正規化した置き場と、置き場からの文書の相対パスをつなぐ」を、IR の置き場から決定の文書の置き場に読み替えた。mutants、plan、ガイド、面のファイル、一覧のファイルの path は、それぞれの話題で決めるとした |
| TBL-core-021 | C-core-output-12, C-core-output-13, C-core-output-14, C-core-output-15 | check の JSON の "tests" は、読んだテストのファイルを拡張子ごとに数える | そのまま。query が true になる拡張子の一覧は書かず、問い合わせのある言語の定義へのリンクにした |
| PROP-core-002 | C-core-output-4 | check の JSON の最上位には、指摘と種類ごとの数と、読んだものの数を出す | そのまま。性質を確認することの行にした |
| PROP-core-004 | C-core-output-16 | check の JSON の "tests" は、読んだテストのファイルを拡張子ごとに数える | そのまま。性質を確認することの行にした |
| EX-core-286 | C-core-output-10 | --format text は、1つの指摘を1行で出す | そのまま。準備の「題名の無い IR の文書」は、決定の文書に読み替える |
| EX-core-004 | C-core-output-8, C-core-output-9 | --format text は、1つの指摘を1行で出す | そのまま。期待する行の detail "a.md" は、`findings.md` の「題名の無い決定の文書の missing_title は、detail を文書名にする」（提案）に依存する |
| EX-core-035 | C-core-output-12 | check の JSON の "tests" は、読んだテストのファイルを拡張子ごとに数える | そのまま |
| EX-core-038 | C-core-output-14 | check の JSON の "tests" は、読んだテストのファイルを拡張子ごとに数える | そのまま |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari/src/ir_tests.rs | req_033_subdirectories_are_read_at_any_depth | REQ-core-033, REQ-core-037, TBL-core-005 | - | この話題の分（TBL-core-005）は、IR の文書の数と行数を確かめている。files と lines をどうするかは実装する側が決める細部なので、当たる行が無い。REQ-core-033 と REQ-core-037 は「文書の読み方と文書全体の検査」の話題 |
| tests/step18_overview.rs | ex_core_463_without_the_overview_key_check_reads_no_overview_data | EX-core-463, REQ-core-279, REQ-core-288, TBL-core-005 | C-core-output-3 | この話題の分（TBL-core-005）だけ。"overview" の鍵がいつも出る（元データが無ければ files 0、marks 0） |
| tests/native_paths.rs | distinct_byte_named_entries_keep_the_legacy_alias_text_and_multiplicity | REQ-core-313, REQ-core-198, REQ-core-206, REQ-core-224, EX-core-485 | C-core-output-5 | この話題の分（REQ-core-206）だけ。名前の違う2つのガイドのファイルが guides の files 2 になる |
| tests/step4_test_discovery.rs | req_033_requirement_without_test_path_carries_the_subdirectory | REQ-core-033, REQ-core-085, REQ-core-110, TBL-core-006 | C-core-output-7 | この話題の分（TBL-core-006）だけ。path が置き場とサブディレクトリをつないだものになる。準備が IR の文書と requirement_without_test なので、付け替えのときに決定の文書と confirmation_without_test に直す |
| tests/step14_plan.rs | req_core_025_plan_text_output_has_one_line_per_finding | REQ-core-025, REQ-core-193 | C-core-output-8 | この話題の分（REQ-core-025）だけ。plan の text も "パス:行 [error] 種類 " で始まる1行 |
| tests/step14_plan.rs | req_core_025_plan_text_output_shows_a_missing_line_as_a_dash | REQ-core-025, REQ-core-193 | C-core-output-9 | この話題の分だけ。line の無い指摘が "パス:- [error] ..." になる |
| tests/step14_plan.rs | ex_core_346_plan_json_has_only_findings_and_counts | REQ-core-194, TBL-core-005, EX-core-346 | - | plan の JSON の最上位は `plan.md` の決定（C-core-plan-22）が持つ。この話題の行には当たらない |
| tests/step14_plan.rs | req_core_194_plan_finding_keys_are_those_of_check | REQ-core-194, TBL-core-006 | C-core-output-6 | この話題の分（TBL-core-006）だけ。指摘の鍵が5つ |
| tests/step14_plan.rs | ex_core_360_a_plan_outside_the_base_has_a_parent_path | REQ-core-193, TBL-core-006, EX-core-360 | - | 計画書の path は `plan.md` の決定（C-core-plan-21）が持つ。この話題の行には当たらない |
| tests/step14_plan.rs | req_core_193_the_path_is_normalised_from_the_base | REQ-core-193, TBL-core-006 | - | 同上 |
| tests/step1_cli.rs | req_021_default_format_is_json | REQ-core-021 | C-core-output-1 | "--format" を付けない check が JSON を出す。v["files"] が 0 であることも見ているので、files を消すなら付け替えのときに直す |
| tests/step15_guides.rs | ex_369_without_the_guides_key_no_guide_is_read | REQ-core-198, REQ-core-206, TBL-core-005, EX-core-369 | C-core-output-5 | この話題の分だけ。ガイドが0件なら guides が files 0、marks 0。準備の IR の文書は、付け替えのときに決定の文書に直す |
| tests/step15_guides.rs | req_198_a_guides_glob_may_hit_the_ir_documents | REQ-core-198, REQ-core-206 | C-core-output-5 | この話題の分（REQ-core-206）だけ。guides の files が 1。IR の文書の files も見ているので、付け替えのときに、決定の文書の置き場に当たる glob と decisions の documents に直す |
| tests/step15_guides.rs | req_200_the_ex_362_project_without_a_guide_mark_has_no_findings | REQ-core-198, REQ-core-200, REQ-core-206 | C-core-output-5 | この話題の分だけ。マーカーの無いガイドで files 1、marks 0 |
| tests/step15_guides.rs | req_200_inline_comments_and_several_marks_in_one_comment_are_all_read | REQ-core-200, REQ-core-206 | C-core-output-5 | この話題の分だけ。marks が 4 |
| tests/step15_guides.rs | ex_362_a_mark_with_the_current_fingerprint_yields_nothing | REQ-core-198, REQ-core-200, REQ-core-203, REQ-core-204, REQ-core-206, EX-core-362 | C-core-output-5 | この話題の分だけ。files 1、marks 1。準備の IR の要求を指すマーカーは、付け替えのときに決定を指すマーカーに直す |
| tests/step15_guides.rs | ex_365_a_mark_on_an_id_missing_from_the_ir_is_a_stale_notice | REQ-core-204, EX-core-365, TBL-core-006 | - | この話題の分（TBL-core-006）は、ガイドへの指摘の path がガイドの基準のディレクトリからの相対であること。ほかのファイルへの指摘の path はその話題で決めるとしたので、当たる行が無い。「ガイドの印」の話題で付け替える（detail の分は `findings.map.md` に無い。このテストは findings の ID を指していない） |
| tests/step15_guides.rs | ex_371_two_entries_in_one_mark_are_counted_and_matched_one_by_one | REQ-core-200, REQ-core-201, REQ-core-204, REQ-core-206, TBL-core-036, EX-core-371 | C-core-output-5 | この話題の分（REQ-core-206）だけ。1つのマーカーに2件あると marks が 2 |
| tests/step1_config.rs | req_110_path_carries_the_subdirectory | REQ-core-110, TBL-core-006 | C-core-output-7 | 置き場を "./docs//ir/./" と書いても、path が "docs/ir/sub/deep/a.md" に正規化される。準備が IR の置き場（設定の "ir"）なので、付け替えのときに決定の文書の置き場の鍵に直し、files を見ている所も直す |
| tests/step17_surface.rs | ex_core_408_a_surface_absent_from_the_ir_is_an_error_at_its_first_place | EX-core-408, EX-core-409, REQ-core-227, TBL-core-019, TBL-core-006, TBL-core-008 | - | この話題の分（TBL-core-006）は、面のファイルへの指摘の path。その話題（「面の検査」）で決めるとしたので、当たる行が無い |
| tests/step17_surface.rs | ex_core_421_an_entry_whose_reason_is_blank_is_invalid_and_removes_nothing | EX-core-421, REQ-core-233, REQ-core-027, TBL-core-006, TBL-core-008 | - | この話題の分（TBL-core-006）は、一覧のファイルへの指摘の path。`surface-unspecified.md` の話題に属する |
| tests/step17_surface.rs | ex_core_407_a_surface_quoted_in_a_requirement_is_not_an_error | EX-core-407, REQ-core-223, REQ-core-226, REQ-core-228, TBL-core-005 | C-core-output-3 | この話題の分（TBL-core-005）だけ。規則があるとき surface が {"unspecified": 0} で出る |
| tests/step17_surface.rs | req_228_the_count_line_comes_after_the_findings | REQ-core-228, TBL-core-005 | - | text の最後の "surface: unspecified=1" の行で、「面の検査」の話題が持つ。この話題の行には当たらない |
| tests/step6_output.rs | req_021_format_values_are_json_and_text | REQ-core-021, REQ-core-022 | C-core-output-2 | json で1つの JSON の値、text で JSON でない |
| tests/step6_output.rs | req_023_files_counts_all_docs_and_lines_sums_them | REQ-core-023, TBL-core-005, TBL-core-006 | C-core-output-3 | findings が配列、counts がオブジェクト。files（用語集を含む）と lines を見ている所は、実装する側の決め方に合わせて付け替えのときに直す |
| tests/step6_output.rs | req_023_finding_keys_match_the_table | REQ-core-023, TBL-core-006 | C-core-output-6 | 5つの鍵の型。準備の「題名の無い IR の文書」は、付け替えのときに決定の文書に直す |
| tests/step6_output.rs | prop_002_counts_match_findings | PROP-core-002 | C-core-output-4 | counts と findings の一致。準備は上と同じく直す |
| tests/step6_output.rs | req_025_text_has_one_line_per_finding_with_bracketed_severity | REQ-core-025 | C-core-output-8 | どの行も "[error]" か "[notice]" を持つ。準備は上と同じく直す |
| tests/step6_output.rs | req_025_a_line_break_in_a_file_name_stays_on_one_text_line | REQ-core-025, EX-core-286 | C-core-output-10 | 名前に改行を含む文書のパスが "\\n" で1行に出る。準備は決定の文書の置き場に直す |
| tests/step6_output.rs | req_025_a_carriage_return_in_a_file_name_is_written_as_backslash_r | REQ-core-025 | C-core-output-10 | パスと detail の "\r" が "\\r" になる。detail の側は missing_title の detail が文書名であること（`findings.md` の提案の決定）に頼っているので、その実装のときに直す |
| tests/step6_output.rs | req_025_text_line_for_a_document_without_a_title | REQ-core-025, REQ-core-026, EX-core-004 | C-core-output-8, C-core-output-9 | "docs/ir/a.md:- [error] missing_title a.md" の行。detail は上と同じく提案の決定に頼るので、その実装のときに直す |
| tests/step6_output.rs | req_128_tests_key_lists_files_per_extension_with_query_flag | REQ-core-128, TBL-core-021, EX-core-035 | C-core-output-12 | そのまま |
| tests/step6_output.rs | req_128_tests_is_an_empty_object_without_test_files | REQ-core-128, TBL-core-021 | C-core-output-15 | そのまま |
| tests/step6_output.rs | req_128_text_format_does_not_print_tests | REQ-core-128 | C-core-output-11 | 指摘の無い text は空。準備の IR の文書は、付け替えのときに決定の文書に直す |
| tests/step6_output.rs | tbl_021_extension_is_after_the_last_dot_and_dotless_names_share_the_empty_key | TBL-core-021 | C-core-output-13 | そのまま |
| tests/step6_output.rs | tbl_021_uppercase_extension_is_a_separate_key_without_query | TBL-core-021 | C-core-output-13 | そのまま |
| tests/step6_output.rs | tbl_021_unparsable_file_is_counted | TBL-core-021, EX-core-038 | C-core-output-14 | そのまま |
| tests/step6_output.rs | tbl_021_same_path_counts_once_and_symlink_counts_apart | TBL-core-021 | C-core-output-14 | そのまま |
| tests/step6_output.rs | tbl_021_keys_are_in_byte_order | TBL-core-021 | C-core-output-13 | そのまま |
| tests/step6_output.rs | tbl_021_excluded_entries_are_not_counted | TBL-core-021 | C-core-output-14 | 落とした細部のテスト。いちばん近い行を指す。glob に当たっても読まないもの（ソケット）を数えないことを確かめている。その決定は誰が決めたか見分けられず（2026-09-17-check-reach.md の A26）落とした。files の数え方の C-14 に当てた |
| tests/step6_output.rs | prop_004_files_sum_equals_the_number_of_read_test_files | PROP-core-004 | C-core-output-16 | そのまま |
| tests/step13_test_languages.rs | tbl_021_extensions_of_the_bundled_languages_have_a_query | TBL-core-021, TBL-core-031, REQ-core-182 | C-core-output-12 | この話題の分（TBL-core-021）だけ。同梱のルールのある言語の拡張子で query が true、".go" で false |
| tests/step12_status.rs | tbl_028_documents_and_test_files_match_check | TBL-core-028, TBL-core-021 | - | この話題の分（TBL-core-021）は、status の tests が check の tests と同じであること。「揃っているかの集計」の話題が持つ |
| tests/step5_findings.rs | req_025_finding_kind_displays_as_its_own_kind_word | REQ-core-025 | C-core-output-8 | text に書く種類の語を作る部分の単体のテスト。例の SourceInvalid は消える種類なので、付け替えのときに残る種類に直す |
| tests/step23_decision_documents.rs | check_counts_the_documents_decisions_and_confirmations_it_read | C-core-docs-1（この話題の ID を指していない） | C-core-docs-1, C-core-output-3 | この話題の ID を今は指していないが、決定の文書の形で、check の JSON の decisions の documents が読んだ決定の文書の数になることを確かめている。マーカーに C-core-output-3 を足す |

## 戻した細部

無い。安全に関わる落とした細部はこの話題に無い。

## 落とした細部

### 安全に関わるもの

無し。この話題が決めるのは出力の形で、落とした細部はどれも数え方と書き方である。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-206: "guides" の "files" の数え方（パスごとに1回、複数の glob に当たっても1回、シンボリックリンクと実体は別のパス）と、"marks" を形の正しいマーカーの1件だけにすること、"--format text" の check に出さないこと | 「IR を書く段で推奨を採用（照合レビューの指摘から。承認で利用者が確かめる）」（2026-09-24-doc-marks.md の A35）。誰が決めたか見分けられないので落とした。"files" と "marks" を出すこと自体は利用者（A17）なので残した |
| TBL-core-005: "overview" を "--format text" に出さないこと | LLM（2026-10-02-whole-picture.md の A73。「承認の時に利用者に示す」）。元の文書にも無い |
| TBL-core-021: glob に当たっても読まないもの（除外）を数えないこと | 誰が決めたか記録に無い（2026-09-17-check-reach.md の A26）。除外の列挙は実験者の裁定（records.md の A165） |
| TBL-core-021: query が true になる拡張子の一覧（"rs"、"ts"、"mts" など15個） | 利用者（2026-09-24-multi-language-tests.md の A47）。落としたのではなく、拡張子と言語の対応を持つ `test-discovery.md` の決定に任せ、ここには問い合わせのある言語の定義へのリンクだけを置いた |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| TBL-core-005 の files と lines | 新しい形で読み替えて残る。IR の文書（用語集と問題の記録を含む）の数と行数を数えていた。用語集は検査せず、問題の記録は無くなる（`docs/decisions/core/decision-documents.md` の「用語集は人が読む文書として残し、用語の検査はやめる」と「人に戻す問題は、話題の文書の「未決」の節に書く」）。読んだ仕様の文書の数を出すことは、今のコードの decisions の documents に読み替えた。files と lines を残すかは、実装する側が決める |
| TBL-core-006 の path の「正規化した置き場と、置き場からの文書の相対パス」 | 新しい形で読み替えて残る。置き場は決定の文書の置き場になる（`docs/decisions/core/decision-documents.md` の「決定の文書は docs/decisions に置き、古い形の文書は凍結した木へ移す」）。今のコードがもうそう動いている |
| EX-core-286、EX-core-004 の「題名の無い IR の文書」 | 新しい形で読み替えて残る。題名の無い決定の文書と読む |

テストの無い行は無い。C-core-output-3 の decisions の部分は、上のテストごとの表のとおり、step23 のテストにマーカーを足すと新しい形の準備で確かめるテストになる。

## 実装する側が決める細部

- check の JSON の最上位の "files" と "lines" を、切り替えたあとに残すか（消す、決定の文書の数と行数にする、など）。残すなら、"decisions" の "documents" との関係も決める。
- "decisions" の鍵の名前と、その中の "decisions"（決定の数）と "confirmations"（確認することの行の数）。今のコードはこの3つの数を出していて、決定の文書の数のほかは利用者の決定に無い。
- "guides" の "files" の数え方と、"marks" に形の誤ったマーカーを数えるか、"--format text" に "guides" と "overview" を出すか。今のコードは、パスごとに1回数え、形の正しい1件だけを数え、text には出さない。
- 置き場を設定に書いたときだけ "decisions" を出す今の振る舞いは、古い形と新しい形が並ぶ間のもので、切り替えたあとの扱いは置き場の既定と一緒に決める。

## 利用者に聞くこと

無し。

## 「提案」で書いた決定の一覧

無し。EX-core-004 と、その detail に頼る2つのテストは、`findings.md` の「題名の無い決定の文書の missing_title は、detail を文書名にする」（提案）の実装で直る。

## 誰が決めたか見分けられなかったもの

- 2026-09-24-doc-marks.md の A35（"guides" の数え方と text の扱い）: 「IR を書く段で推奨を採用（照合レビューの指摘から。承認で利用者が確かめる）」。落とした。
- 2026-09-17-check-reach.md の A26（除外は数えない）: 誰が決めたかが書かれていない。落とした。
- 2026-09-17-check-reach.md の A8、A14、A15、A19、A20 は、decided_by の行が無く、決定の文の末尾に「（推奨を採用）」とある。利用者が推奨を採ったものとして、利用者の決定に数えた。
- records.md の A40（第6ラウンド、利用者の答え「推奨で」）、A47 の (6)（同）、A50（第7ラウンド、「推奨で問題無いと思う」）は、利用者が推奨を採ったものとして、利用者の決定に数えた（`finding-order.map.md` と同じ扱い）。

## 理由や見送った案が記録に無かった決定

- 「指摘は kind、severity、path、line、detail の5つの鍵で出す」: 理由も見送った案も記録に無い（records.md の A40、2026-09-16-notice.md の A1）。問題の節に「理由は残っていない」と書いた。
- 「"--format" は json と text の2つを受け、既定を json にする」: 見送った案が記録に無い。読みやすさを狙う形を作らないことは、未決に残したもので（records.md の A18 の U25）、退けた案ではないので書かなかった。
- 「check の JSON の最上位には、指摘と種類ごとの数と、読んだものの数を出す」: 見送った案が記録に無い。findings と counts の理由は記録に無く、数を出す理由はガイド（2026-09-24-doc-marks.md の A17）と全体像（2026-10-02-whole-picture.md の A64）の why から書いた。
- 「--format text は、1つの指摘を1行で出す」: 見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-core-021 と REQ-core-022 を1つの決定にまとめた。JSON を1つ出すことは、"--format" の値の決定の一部として読める。
- REQ-core-023 と TBL-core-005、PROP-core-002 を、最上位の鍵の決定にまとめ、TBL-core-006 を指摘の鍵の決定に分けた。
- REQ-core-128 の "tests" の中身（TBL-core-021、PROP-core-004）を、最上位の決定から分けた。理由（問い合わせの無い言語を出力から読めるようにする）と見送った案（2026-09-17-check-reach.md の R1）が、"tests" にだけあるため。
- REQ-core-128 と REQ-core-206 の「text には出さない」を、text の決定に置いた。
- REQ-core-025 と REQ-core-026 を、text の1つの決定にまとめた。

## あとでリンクにする所

- 済み: 「--format text は、1つの指摘を1行で出す」の、未記載の面の数の行を最後に出すことは、surface の見出しへのリンクにした。下書きどうしのリンクは `.ja.md` に直した。

## 迷った判断と、どう決めたか

- 「path」の行の「テストのファイル、ガイド、面のファイル、一覧のファイルなど」の「など」を、「決定の文書でないファイル」と言い、例だと分かる書き方（「例: …」）に直した。意味は変えていない。
- `tbl_021_excluded_entries_are_not_counted` は、この話題の ID だけを持つので、「-」のままだとマーカーの無いテストになる。いちばん近い C-14 を指した。

- 最上位の表の "decisions" の行には、読んだ決定の文書の数（"documents"）だけを書いた。利用者が決めたのは「読んだ仕様の文書の数を出す」（records.md の A40 の files）までで、決定の数と確認することの行の数は、今のコードが足したものである。
- "files" と "lines" を表から外した。中身が IR の文書の数と行数で、切り替えたあとに何を数えるかは決まっていないため。BRIEF-B の決まりどおり、JSON の形は実装する側が決める細部に挙げた。表は「次の鍵を出す」と書き、鍵がこれだけだとは書いていない。
- check の JSON の "files" と "lines" は、元の文書にある利用者から見える名前だが、文書から外した。BRIEF の「名前を残す」と BRIEF-B の「JSON の形は実装する側が決める」がぶつかり、BRIEF-B を優先した。面の規則は CLI のサブコマンドとフラグだけを取り出すので、JSON の鍵が文書から消えても surface_without_spec は出ない。
- C-core-output-3 は、決定の文書の置き場を読んだ check に絞った。置き場を設定に書かないと "decisions" を出さない今の振る舞い（step23 の check_without_the_place_in_the_configuration_reads_no_decision_documents）と食い違わないようにするためである。
- "overview" と "surface" の行は、出す条件を決める話題（全体像の元データの申告、面の検査）が別にあるが、最上位の表から行を外すと最上位の鍵の一覧が欠けるので、中身だけを書いて残した。
- C-core-output-7 の例の置き場は、IR の置き場の例（"./docs//ir/./"）を決定の文書の置き場に読み替えて書いた。
- 「"warning" は使わない」は、2026-09-16-notice.md の A1（利用者）の決定なので、severity の行に残した。
- 問題の節の「人が読む文字の出力は、中身を詰めて作らないと読みづらい」は、記録では利用者が別の道具で経験したこととして書かれているが、その道具の名前は書かなかった。

## 数

- 元: 要求 7、場面 4、決定表 3、性質 2
- 読んだ記録のファイル: 12（records.md、ir-form.md、2026-09-16-ir-tree.md、2026-09-16-notice.md、2026-09-17-check-reach.md、2026-09-17-mutation-tests.md、2026-09-24-doc-marks.md、2026-09-24-multi-language-tests.md、2026-09-24-plan-schema.md、2026-09-24-review5-gaps.md、2026-09-27-surface-check.md、2026-10-02-whole-picture.md のうち、出典に引かれた決定の行）
- 出来上がり: 決定 5（すべて実装済み）、確認することの行 16（うち「（読んで確認）」0）
- テスト: 44（この話題の ID を指すもの。付け替え先のあるもの 35、「-」9）。ほかに、この話題の ID を指していないが付け替えを勧めるもの 1
