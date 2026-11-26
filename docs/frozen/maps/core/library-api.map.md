# library-api の対応表

元: `docs/ir/core/library-api.md`（日本語は `library-api.ja.md`）。書き直した文書: `library-api.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-310 | C-core-libapi-1, C-core-libapi-2, C-core-libapi-3, C-core-libapi-13 | 別の Rust のアプリは、kotowari の操作を Project のメソッドとして型付きの入力で呼ぶ | そのまま。定義の TBL-core-041 と一緒に、決めたことの表にした |
| REQ-core-311 | C-core-libpaths-3, C-core-libpaths-11, C-core-libpaths-13 | ファイルを読む入口は作業開始位置を受け取り、既存の CLI と同じ規則でそこから解く | そのまま。作業開始位置と設定ファイルの指定は、REQ-core-311 自身が位置の契約を REQ-core-323 に任せているので、library-paths の決定にまとめた。`Project::new` と `ProjectOptions` の名前は、この話題の最初の決定にも残し、そこからリンクした。絶対パスであること（C-11）とカレントディレクトリを変えないこと（C-13）は library-paths の決定に戻した。CLI が現在の位置を渡すことは落とした細部 |
| REQ-core-312 | C-core-libapi-4, C-core-libapi-5, C-core-libapi-6, C-core-libapi-7, C-core-libapi-8, C-core-libapi-9, C-core-libapi-10 | 検査で見つけた指摘と、検査を終えられなかった失敗を分けて返す | そのまま。4つの記録の決定を束ねていたので、3つの決定に分けた（下の「分けた所」）。指摘と失敗の区別を C-4〜C-6、設定ファイルがディレクトリのときを C-7、C-8、未知の ID を C-9、C-10 にした |
| REQ-core-313 | C-core-libapi-11, C-core-libapi-12 | CLI は、ライブラリの結果を既存の出力と停止の理由に写すだけにする | 読み替え。守る約束の並びのうち「IR」は形に依存するので書かず、「未実装の overview は別計画で追加する」は経緯なので落とした（下の「形に依存する要求」と「落とした細部」） |
| TBL-core-041 | C-core-libapi-1, C-core-libapi-2, C-core-libapi-3, C-core-libapi-13, C-core-libinputs-12 | 別の Rust のアプリは、kotowari の操作を Project のメソッドとして型付きの入力で呼ぶ | 読み替え。決めたことの表にした。overview_prepare が何も書かないことを C-13 にした。plan と mutants が仕様の文書を読まないことは library-inputs の「操作は、その操作が使う入力だけを読む」に置き、表からはリンクで指した |
| EX-core-483 | C-core-libapi-1, C-core-libapi-4, C-core-libapi-6, C-core-libpaths-13 | 検査で見つけた指摘と、検査を終えられなかった失敗を分けて返す | 読み替え。指摘が型で見分けられる完了した結果を C-4、標準出力に書かないことを C-6（読んで確認）にした。カレントディレクトリを変えないことは library-paths の C-13 |
| EX-core-484 | C-core-libapi-5 | 検査で見つけた指摘と、検査を終えられなかった失敗を分けて返す | そのまま。読めないファイルのほか、設定の誤りと全体像の書き出しの失敗も、例として同じ行に入れた |
| EX-core-485 | C-core-libapi-11 | CLI は、ライブラリの結果を既存の出力と停止の理由に写すだけにする | そのまま |
| EX-core-486 | C-core-libinputs-12 | 操作は、その操作が使う入力だけを読む | 読み替え。操作ごとに読む範囲の決定（library-inputs）にまとめた |
| EX-core-554 | C-core-libapi-7 | 渡した設定ファイルがディレクトリなら入力の誤り、既定の場所がディレクトリなら読めない失敗にする | そのまま |
| EX-core-555 | C-core-libapi-8 | 渡した設定ファイルがディレクトリなら入力の誤り、既定の場所がディレクトリなら読めない失敗にする | そのまま |
| EX-core-556 | C-core-libapi-9 | 未知の ID の問い合わせの失敗から、その ID を取り出せる | 読み替え。「項目もシナリオも無い ID」は、形によらない「どこにも無い ID」に言い換えた |
| EX-core-557 | C-core-libapi-10 | 未知の ID の問い合わせの失敗から、その ID を取り出せる | そのまま |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari/tests/project.rs | project_operations_reuse_owned_results_and_explicit_reads_observe_changes | REQ-core-310, REQ-core-311, REQ-core-312, REQ-core-316, REQ-core-317, EX-core-483, EX-core-484, EX-core-490, EX-core-491 | C-core-libapi-1, C-core-libapi-4, C-core-libpaths-13 | Project の read、inspect、query、list、check、status が型付きの結果を返し、check が Err でなく指摘を持つことを確かめている。カレントディレクトリが変わらないことの確認（REQ-core-311、EX-core-483）は library-paths の C-13。REQ-core-316、REQ-core-317 の分は library-inputs の表に書いた |
| crates/kotowari/tests/project.rs | project_rejects_relative_roots_and_plan_does_not_require_ir | REQ-core-310, REQ-core-312, REQ-core-323, EX-core-484, EX-core-486, EX-core-500 | C-core-libapi-1, C-core-libapi-5, C-core-libinputs-12 | plan が IR の置き場なしで型付きの結果を返し、無い計画書には Err を返すことを確かめている。相対の作業開始位置を拒む部分（REQ-core-323）は library-paths の C-11 で、その表に書いた |
| crates/kotowari/tests/project.rs | mutants_have_typed_inputs_and_failures_without_ir_loading | REQ-core-310, REQ-core-312, EX-core-484, EX-core-486 | C-core-libapi-1, C-core-libapi-5, C-core-libinputs-12 | mutants が型付きの入力で、IR の置き場なしに結果を返し、無い結果のファイルには Err を返す |
| crates/kotowari/tests/project.rs | absolute_resolution_bases_survive_a_child_process_directory_change | REQ-core-311, REQ-core-323, EX-core-483, EX-core-501 | C-core-libpaths-3 | 相対パスで渡した設定ファイルが作業開始位置から解かれることを確かめている（あとでカレントディレクトリを変えても）。REQ-core-311 の内容は library-paths の決定に移したので、付け替え先もそちら |
| crates/kotowari/tests/project.rs | public_admission_errors_keep_distinct_facade_classifications_and_details | REQ-core-312, REQ-core-315, REQ-core-322, EX-core-484, EX-core-488, EX-core-498 | C-core-libapi-5 | 下位の失敗が kotowari::Error に移っても InvalidInput、InputMissing、ConfigError の種類で見分けられ、Display に詳細が残ることを確かめている。REQ-core-315、REQ-core-322 の分はそれぞれの表に書いた |
| crates/kotowari/tests/project.rs | ex_core_554_a_configuration_file_that_is_a_directory_is_invalid_input_like_a_missing_one | REQ-core-312, EX-core-554 | C-core-libapi-7 | そのまま |
| crates/kotowari/tests/project.rs | ex_core_555_a_directory_at_the_default_location_is_a_read_failure | REQ-core-312, EX-core-555 | C-core-libapi-8 | そのまま |
| crates/kotowari/tests/project.rs | ex_core_556_the_unknown_id_can_be_taken_from_a_failed_query | REQ-core-312, EX-core-556 | C-core-libapi-9 | そのまま |
| crates/kotowari/tests/project.rs | ex_core_557_a_failure_of_another_kind_provides_no_unknown_id | REQ-core-312, EX-core-557 | C-core-libapi-10, C-core-libapi-5 | 読めない置き場で ReadFailure になることも確かめているので C-5 も指す |
| crates/kotowari/tests/project.rs | project_check_status_and_inspect_include_the_overview_group | REQ-core-290, REQ-core-310, TBL-core-041 | C-core-libapi-2 | check、status、inspect に全体像の指摘と "overview" の群が入ることを確かめている。REQ-core-290 の分は全体像の元データの話題で付け替える |
| crates/kotowari/tests/project.rs | overview_prepare_writes_nothing_and_overview_build_writes_the_cache | REQ-core-310, TBL-core-041, REQ-core-293, REQ-core-294 | C-core-libapi-3, C-core-libapi-13 | overview_build が書いたファイルと書かなかった数を返し、変わらないファイルは書かなかった数に入ることを C-3、overview_prepare のあとで置き場が無く、build で書かれることを C-13 で確かめている（準備の結果の write は build のあとに呼ばれ、書かなかった数だけを確かめるので、行には入れなかった）。REQ-core-293、REQ-core-294 の分は全体像のコマンドの話題で付け替える |
| crates/kotowari/tests/project.rs | overview_operations_fail_with_kinds_for_bad_data_and_a_missing_key | REQ-core-294, REQ-core-312, REQ-core-279 | C-core-libapi-5 | 元データの誤りと鍵の無さが、種類を見分けられる Err になる。REQ-core-294、REQ-core-279 の分はそれぞれの話題で付け替える |
| crates/kotowari/tests/project.rs | overview_build_stops_with_a_cache_error_when_the_place_is_not_a_directory | REQ-core-324, REQ-core-312 | C-core-libapi-5 | 書けない置き場が CacheFailure の種類の Err になる。詳細の文と、置き場のファイルを上書きしないことは REQ-core-324 の分 |
| crates/kotowari/tests/project.rs | overview_build_stops_with_a_cache_error_and_the_os_error_when_a_write_fails | REQ-core-324, REQ-core-312 | C-core-libapi-5 | 同上 |
| crates/kotowari/tests/async.rs | overview_prepare_and_build_return_what_the_sync_operations_return | REQ-core-310, TBL-core-041, REQ-core-318 | C-core-libapi-13 | この話題の分は、AsyncProject の overview_prepare のあとで置き場が作られていないこと（C-13）。同期と同じ結果を返すことは library-async の C-core-libasync-1 で、その表に書いた |
| crates/kotowari-core/tests/inputs.rs | read_models_list_and_query_without_check_only_inputs | REQ-core-317, EX-core-491, REQ-core-312 | - | REQ-core-312 の分としては、未知の ID の query が Err になることしか見ておらず、種類も見ていない。このテストは library-inputs の C-core-libinputs-11 を指すので、マーカーは無くならない |
| crates/kotowari-core/tests/inputs.rs | invalid_typed_configuration_is_an_execution_failure_not_a_completed_inspection | REQ-core-312, EX-core-484 | C-core-libapi-5 | 正しくない設定が、完了した検査ではなく ConfigError の失敗になる |
| crates/kotowari-source-analysis/tests/analyzer.rs | malformed_public_rules_explain_the_analysis_failure | REQ-core-312, EX-core-484 | C-core-libapi-5 | 壊れた規則が結果でなく失敗になり、Display にファイルの名前が出る |
| crates/kotowari-core/tests/admission.rs | native_display_collisions_do_not_merge_original_files | REQ-core-315, REQ-core-313, EX-core-490, EX-core-485 | - | この話題の分（REQ-core-313、EX-core-485）は無い。CLI の出力は見ていない。core の取り込みで表示が同じ別々のファイルを1つにしないことは、library-inputs の表で C-core-libpaths-9 を指す |
| crates/kotowari-core/tests/admission.rs | native_read_preparation_retains_the_configured_ir_and_disabled_tests | REQ-core-311, REQ-core-315, REQ-core-317, EX-core-483, EX-core-491 | - | 古い形のコードと一緒に消す（設定した IR の置き場を、内部の読み込みの準備が保つことを確かめている。library-inputs の表と同じ扱い） |
| tests/step15_guides.rs | overlap_precedes_guide_only_invalid_utf8_and_missing_surface_rules | REQ-core-199, REQ-core-313, EX-core-485 | C-core-libapi-11 | CLI の契約テスト。REQ-core-199 の分はガイドの話題で付け替える |
| tests/native_paths.rs | byte_named_files_without_legacy_read_aliases_keep_the_unreadable_stop | REQ-core-313, REQ-core-018, REQ-core-198, REQ-core-224, EX-core-485 | C-core-libapi-11 | CLI の契約テスト。ほかの ID の分はそれぞれの話題で付け替える |
| tests/native_paths.rs | distinct_byte_named_entries_keep_the_legacy_alias_text_and_multiplicity | REQ-core-313, REQ-core-198, REQ-core-206, REQ-core-224, EX-core-485 | C-core-libapi-11 | 同上 |
| tests/native_paths.rs | ir_and_decision_source_walkers_still_read_original_byte_named_paths | REQ-core-313, REQ-core-092, EX-core-485 | C-core-libapi-11 | CLI の契約テスト。ただし中身は IR と判断の記録の置き場の走査で、切り替えのときにテストごと書き直すか消すことになる |
| tests/native_paths.rs | distinct_native_files_with_equal_display_paths_keep_both_counts_and_findings | REQ-core-313, EX-core-485 | C-core-libapi-11 | CLI の契約テスト |
| tests/native_paths.rs | acquired_diagnostic_paths_do_not_collapse_filename_components | REQ-core-313, EX-core-485 | C-core-libapi-11 | CLI の契約テスト |
| tests/native_paths.rs | configured_test_rules_match_the_original_acquired_path | REQ-core-313, EX-core-485 | C-core-libapi-11 | CLI の契約テスト |
| tests/step3_sources_terms.rs | literal_backslash_filenames_do_not_become_source_directories | REQ-core-313, EX-core-485 | C-core-libapi-11 | CLI の契約テスト。中身は出典の検査（source_invalid）で形に依存し、切り替えのときに書き直すか消すことになる |
| crates/kotowari-mds/tests/cli.rs | directory_check_reports_dot_prefixed_markdown_and_skips_hidden_directories | REQ-schema-044, REQ-schema-073, REQ-core-313 | C-core-libapi-11 | mds の CLI の契約テスト。REQ-schema-044、REQ-schema-073 の分はスキーマの話題で付け替える（library-io の表では C-schema-io-2） |

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-311、EX-core-483: Project はプロセスのカレントディレクトリを変えない | C-core-libpaths-13 | テスト project_operations_reuse_owned_results_and_explicit_reads_observe_changes（最後にカレントディレクトリが変わっていないことを確かめる）。library-paths の「ファイルを読む入口は作業開始位置を受け取り、…」に、SchemaLoader の分と一緒に書いた。この文書の最初の決定からはリンクと1文で指す |
| REQ-core-311: 作業開始位置は絶対パスで受ける | C-core-libpaths-11 | テスト project_rejects_relative_roots_and_plan_does_not_require_ir。library-paths の同じ決定に書いた |
| TBL-core-041: overview_prepare は検査と描画をメモリで済ませ、何も書かない。書くのは overview_build と準備の結果の write だけ | C-core-libapi-13 | テスト overview_prepare_writes_nothing_and_overview_build_writes_the_cache（同期）、overview_prepare_and_build_return_what_the_sync_operations_return（非同期）。問題の節には、記録（2026-10-04-overview-on-public-api.md の A7）の why にある、ポートが使えないのに書いてしまうことを避ける理由を書いた |

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-311: CLI が現在の位置を取得して渡す | 設計担当（A36） |
| REQ-core-312: 失敗の種類の並び（入力不足、設定・入力形式不正、未知の問い合わせ ID、読込失敗、内部対応不能）と、enum を non_exhaustive にすること | 設計担当（A40）。名前 InvalidInput、ReadFailure、UnknownQuery、InputMissing は残し、並びは「失敗の種類の例として」と例の書き方で書いた |
| REQ-core-313: 「未実装の overview は別計画で追加する」 | 利用者（A33）だが、今は全体像が実装されていて経緯にあたる。決めたことには書かない |
| TBL-core-041: 各操作の結果の中身（query の逆参照、mutants の変異の集計など）が既存の CLI の仕様の意味を保つこと | 設計担当（A55）。表の「返すもの」は元の表のまま残し、意味の正は各コマンドの話題に任せた |
| TBL-core-041: overview_build が書く場所 ".kotowari/cache/overview/" | 元の文書は REQ-core-293 を指している。「全体像を作るコマンドと見せるコマンド」の話題に任せた |
| next-release-bundle の A2: 設定ファイルがディレクトリのときの文 "config file is a directory: <path>" と CLI の標準エラー | 利用者（推奨を採用）。元の文書には無く、CLI の停止の詳細の話題（cli-environment）の分 |
| next-release-bundle の A4: CLI の出力 "argument error: unknown id: <ID>" | 利用者（推奨を採用）。元の文書には無く、query の話題の分。決めたことには「CLI の出力は変わらない」とだけ書いた |
| REQ-core-312: 失敗の型の公開の欄は読み取り専用で、CLI の JSON の形は表示の側で保つ | 設計担当（A40） |

## 形に依存する要求

| 元の ID | 消える／読み替えて残る |
|---|---|
| REQ-core-313 | 読み替えて残る。守る約束の並びの「IR」は、新しい版で決定の文書に置き換わる。文書には「設定とスキーマの受け付け方、検査の結果の意味」だけを書いた |
| TBL-core-041 | 読み替えて残る。plan と mutants の「IR の読込は要求しない」は「仕様の文書を読まない」に、list の「読み取れた項目」は「仕様の文書から読み取れたもの」に読み替えた |
| EX-core-483 | 読み替えて残る。「指摘の出る IR」を「指摘の出る仕様の文書」にした |
| EX-core-486 | 読み替えて残る。「IR の置き場はない」を「仕様の文書の置き場が無い」にした |
| EX-core-556 | 読み替えて残る。「ID を持つ項目もシナリオも無い」を「どこにも無い ID」にした。新しい形では、問い合わせる ID は確認することの ID になる |

テストの無い行: 無い（C-core-libapi-6 と C-core-libapi-12 は「（読んで確認）」）。

## 誰が決めたか見分けられなかったもの

- `Project::new`、`ProjectOptions`、メソッドの名前、`overview_prepare` と `overview_build` の分け方は、設計担当（A36、D1）と LLM（overview-on-public-api の A7）が決めた名前である。操作をすべて型付きで出すこと（A10）、全体像の build を Project に足すこと（overview-on-public-api の A3）は利用者の決定なので、名前を残し、利用者の決定が裏付ける振る舞いだけを書いた。
- C-core-libapi-6（標準出力に書かない）の元は EX-core-483 の文で、それを決めた行は設計担当の A40 だが、利用者の A7（表示の仕方は呼び出し側が決める）の言い換えとして残した。

## 理由や見送った案が記録に無かった決定

- 「検査で見つけた指摘と、検査を終えられなかった失敗を分けて返す」: 見送った案が記録に無い。
- 「渡した設定ファイルがディレクトリなら入力の誤り、…」: 見送った案が記録に無い。
- 「未知の ID の問い合わせの失敗から、その ID を取り出せる」: 見送った案が記録に無い。
- 「CLI は、ライブラリの結果を既存の出力と停止の理由に写すだけにする」: 見送った案が記録に無い。
- 「別の Rust のアプリは、kotowari の操作を …」: 見送った案は overview-on-public-api の A3 の rejected だけ。「全体像の操作を公開しない」に採らなかった理由は記録に無く、理由を付けずに書いた。

## 分けた所、まとめた所

- REQ-core-312 を3つの決定に分けた: 指摘と失敗の区別（public-crate-api の A7、A8）、設定ファイルがディレクトリのとき（next-release-bundle の A2、A15）、未知の ID（next-release-bundle の A4、A7）。それぞれ別の記録で理由を持つため。
- REQ-core-311 は、library-paths の「ファイルを読む入口は作業開始位置を受け取り、…」にまとめた。REQ-core-323 と同じ決定（作業開始位置）で、振る舞いが見えるテストもそちらに当たる。
- REQ-core-310 と TBL-core-041 は1つの決定にまとめた。
- EX-core-486（plan と mutants は IR を読まない）は、list と query がガイドを読まないこと（library-inputs の REQ-core-317）と同じ利用者の決定（public-crate-api の A19）から来るので、library-inputs の「操作は、その操作が使う入力だけを読む」にまとめた。

## あとでリンクにする所

- 無い。前の下書きで言葉で指していた3つ（ライブラリのクレート境界、任意の Tokio 対応、全体像を作るコマンドと見せるコマンド）は、どれもこのフォルダに下書きがあるので、`library-crates.ja.md#用途ごとにパッケージを分け利用者は用途でパッケージを選ぶ`、`library-async.ja.md#高水準の操作を既定で無効な-tokio-feature-で-tokio-から待てるようにする`、`overview-commands.ja.md#overview-build-は毎回すべてを描き中身の違うファイルだけを書き作られなくなったファイルを消す` へのリンクに直した（冒頭の段落は文書へのリンク）。

## 迷った判断と、どう決めたか

- 安全に関わる細部の「overview_prepare は何も書かない」は、記録では LLM の決定（承認の時に利用者に示す）だが、テストが確かめているので決定に戻した。全体像のコマンドの話題（overview-commands）は CLI の build と serve を扱い、overview_prepare を書いていないので、Project のメソッドを決めるこの話題に置いた。
- カレントディレクトリと絶対パスの2件は、library-paths にも同じ細部が挙がっていたので、library-paths に1つだけ書き、この文書の最初の決定から1文とリンクで指した。
- 失敗の種類の並びの「など」は、全部を挙げると設計担当が決めた種類（InternalMapping、CacheFailure、OverviewData、RuntimeUnavailable、TaskFailure を含む10個）まで決めたことになるので、「失敗の種類の例として」と例の書き方にした。C-core-libapi-5 の「など」も、指しているテストが読めないファイル、設定の誤り、全体像の元データの誤り、全体像の置き場の失敗にまたがるので、例の書き方にした。

- public-crate-api の A36〜A56 は、どれも設計担当が D1（具体的な型名・関数名・モジュール配置・検証方法の委任）で決めたもので、「細部は LLM に任せる」承認に当たるとして落とした。名前は残す決まりに従って残し、振る舞いは利用者の A6〜A10、A18、A19、A29 と、overview-on-public-api の A2、A3、next-release-bundle の A2、A4、A7、A15 で裏付けられる分だけを書いた。スキーマの側（library-io の表）と同じ扱いにした。
- next-release-bundle の A2、A4、A7、A15 は「推奨を採用」「仮決めの推奨をそのまま送った」の利用者の決定で、委任ではないので、利用者の決定として書いた。A7 はメソッドの署名まで決めているので、`pub fn unknown_id(&self) -> Option<&str>` をそのまま書いた。
- REQ-core-313 の「再構成は…を維持する」は作り直しのときの約束だが、CLI がライブラリの上の薄い層であることは今の振る舞いなので、実装済みの決定として残した。根拠は A9（利用者）と、overview-on-public-api の A3 の why（CLI は Project の上の薄い層）、A7（CLI が整形する）。
- 非同期の入口に同じ操作があることは、この話題の決めたことに一言だけ書き、確認することは「任意の Tokio 対応」の話題に任せた。見送った案「build を同期だけにする」は、全体像の操作をどこに置くかの決定の代わりとしてここに書いた。
- `inspect` は元の TBL-core-041 に無いが、overview-on-public-api の A3（利用者）が check と status と並べて名前を挙げているので、全体像の群を含めることと一緒に書いた。

## 数

- 元: 要求 4、場面 8、決定表 1、性質 0
- 読んだ記録のファイル: 5（2026-10-03-public-crate-api.md、2026-10-04-overview-on-public-api.md、2026-10-06-changes-rethink.md、2026-10-23-next-release-bundle.md、2026-10-24-next-release-bundle-consistency.md）
- 出来上がり: 決定 5、確認することの行 13（うち「（読んで確認）」2）
- 戻した細部: 3（1件はこの文書、2件は library-paths の決定）
- テスト: 29
