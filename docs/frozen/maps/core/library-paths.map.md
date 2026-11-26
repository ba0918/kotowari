# library-paths の対応表

元: `docs/ir/core/library-paths.md`（日本語は `library-paths.ja.md`）。書き直した文書: `library-paths.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-322 | C-core-libpaths-1, C-core-libpaths-2, C-core-libpaths-5, C-core-libpaths-6, C-core-libpaths-7, C-core-libpaths-8 | メモリの入力は、内容と論理パスの組で受け取り、ファイルを探さない | 読み替え（IR の置き場と出典照合の部分。下の「形に依存する要求」）。残りはそのまま。論理パスと内容の組で受け、ファイルを探さないことを C-1、親ディレクトリを指す論理パスを保つことを C-2、空と絶対パスを拒むことを C-5 にした。重複と内容の食い違い（C-6〜C-8）は別の決定「同じ論理パスの入力が重なったら、…」に分けた。表示と出典照合のパスは落とした細部 |
| REQ-core-323 | C-core-libpaths-3, C-core-libpaths-4, C-core-libpaths-11, C-core-libpaths-12, C-core-libpaths-13 | ファイルを読む入口は作業開始位置を受け取り、既存の CLI と同じ規則でそこから解く | そのまま。library-api の REQ-core-311 をここにまとめた。絶対パスで受けることを C-11、カレントディレクトリが変わっても基準が変わらないことを C-12、カレントディレクトリを変えないことを C-13 にした |
| EX-core-498 | C-core-libpaths-5, C-core-libpaths-6 | 同じ論理パスの入力が重なったら、同じファイルとして扱えるときだけ受け付ける | そのまま。正規化したあとの重複を InvalidInput にする場面 |
| EX-core-499 | C-core-libpaths-1 | メモリの入力は、内容と論理パスの組で受け取り、ファイルを探さない | そのまま |
| EX-core-500 | C-core-libpaths-11 | ファイルを読む入口は作業開始位置を受け取り、既存の CLI と同じ規則でそこから解く | そのまま。相対の作業開始位置を拒む場面 |
| EX-core-501 | C-core-libpaths-3, C-core-libpaths-4, C-core-libpaths-12 | ファイルを読む入口は作業開始位置を受け取り、既存の CLI と同じ規則でそこから解く | そのまま |
| EX-core-502 | C-core-libpaths-2 | メモリの入力は、内容と論理パスの組で受け取り、ファイルを探さない | 読み替え。IR の置き場 "../spec/ir" の例は形に依存するので、".." を含む論理パス一般の行にした |
| EX-core-503 | C-core-libpaths-7, C-core-libpaths-8 | 同じ論理パスの入力が重なったら、同じファイルとして扱えるときだけ受け付ける | そのまま。群の間で同じパスの本文が食い違うと InvalidInput にする場面 |

元の文書に TBL と PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-core/tests/paths.rs | logical_paths_are_normalized_without_accessing_the_filesystem | REQ-core-322, EX-core-498, EX-core-503 | C-core-libpaths-1, C-core-libpaths-2, C-core-libpaths-5 | ファイルの無い論理パスで SourceText を作り、区切りが揃って内容を持つこと、"../outside.md" が残ること、絶対パスと "./"（揃えると空）を拒むことを確かめている |
| crates/kotowari-core/tests/paths.rs | relative_paths_with_a_colon_remain_valid_on_unix | REQ-core-322, EX-core-498 | C-core-libpaths-5 | 落とした細部のテスト。いちばん近い行を指す（コロンを含む相対パスを絶対パスとして拒まないこと） |
| crates/kotowari-core/tests/paths.rs | duplicate_normalized_paths_and_cross_group_source_mismatches_are_rejected | REQ-core-322, EX-core-499, EX-core-502 | C-core-libpaths-6, C-core-libpaths-7 | 揃えたあとの重複が InvalidInput、群の間の内容の食い違いが InvalidInput、同じ内容なら受け付けることを確かめている |
| crates/kotowari-core/tests/paths.rs | inspection_compares_original_test_and_surface_source_texts | REQ-core-322, EX-core-502 | C-core-libpaths-8 | テスト情報と面の解析結果の本文の食い違いが InvalidInput になる |
| crates/kotowari/tests/project.rs | project_rejects_relative_roots_and_plan_does_not_require_ir | REQ-core-310, REQ-core-312, REQ-core-323, EX-core-484, EX-core-486, EX-core-500 | C-core-libpaths-11 | この話題の分は、相対の作業開始位置で Project::new が失敗すること（種類は見ていない）。library-api の分はその表に書いた |
| crates/kotowari/tests/project.rs | absolute_resolution_bases_survive_a_child_process_directory_change | REQ-core-311, REQ-core-323, EX-core-483, EX-core-501 | C-core-libpaths-3, C-core-libpaths-12 | 相対パスで渡した設定ファイルを、カレントディレクトリを変えたあとでも作業開始位置から解いて query が通る |
| crates/kotowari/tests/project.rs | project_operations_reuse_owned_results_and_explicit_reads_observe_changes | REQ-core-310, REQ-core-311, REQ-core-312, REQ-core-316, REQ-core-317, EX-core-483, EX-core-484, EX-core-490, EX-core-491 | C-core-libpaths-13 | この話題の ID は指していないが、REQ-core-311 と EX-core-483 の分（Project の操作のあとでカレントディレクトリが変わっていないこと）をこの話題に移したので、ここに書く。library-api と library-inputs の分はそれぞれの表に書いた |
| crates/kotowari/tests/project.rs | facade_names_are_the_same_lower_result_types | REQ-core-322, EX-core-499 | C-core-libcrates-10 | kotowari が再公開した型が下位の型と同じであることを確かめている。パスとは関係が無く、library-crates の行に当たる |
| crates/kotowari/tests/project.rs | public_admission_errors_keep_distinct_facade_classifications_and_details | REQ-core-312, REQ-core-315, REQ-core-322, EX-core-484, EX-core-488, EX-core-498 | C-core-libpaths-5 | この話題の分は、絶対パスの SourceText が InvalidInput の種類になること。ほかの話題の分はそれぞれの表に書いた |
| crates/kotowari-core/tests/inputs.rs | nested_ir_paths_preserve_the_document_kind_of_the_filename | REQ-core-314, REQ-core-322, EX-core-487, EX-core-498 | - | 古い形のコードと一緒に消す（IR の文書の種類をファイル名で決める。library-inputs の表と同じ扱い） |
| crates/kotowari-core/tests/inputs.rs | parsed_documents_retain_nested_paths_and_definition_and_statement_references | REQ-core-314, REQ-core-317, REQ-core-322, REQ-core-054, EX-core-487, EX-core-498 | - | 古い形のコードと一緒に消す（IR の項目の definition と本文の参照。library-inputs の表と同じ扱い） |
| crates/kotowari-core/tests/inputs.rs | same_text_shared_between_guides_and_tests_uses_the_overlap_rule | REQ-core-315, REQ-core-322, EX-core-488, EX-core-489, EX-core-499 | C-core-libpaths-7 | この話題の分は、同じ内容の SourceText を群の間で共有しても InvalidInput にならず、既存の重なりの規則の誤り（ConfigError の "matched by both"）で止まること。library-inputs の分はその表に書いた |
| crates/kotowari-source-analysis/tests/analyzer.rs | cached_analysis_of_another_revision_is_rejected_at_native_admission | REQ-core-315, REQ-core-322, EX-core-496, EX-core-502 | C-core-libpaths-10 | 解析した本文と取り込んだ本文が食い違うと拒み、同じなら受け付ける |
| crates/kotowari-markdown-schema-io/tests/loader.rs | relative_start_and_cache_bases_are_rejected | REQ-core-323, EX-core-500 | C-core-libpaths-11 | 相対の作業開始位置と、相対のキャッシュ基準で SchemaLoader::new が失敗する（種類は見ていない） |
| crates/kotowari-markdown-schema-io/tests/loader.rs | later_current_directory_changes_do_not_change_the_load_base | REQ-core-323, EX-core-501, REQ-schema-073 | C-core-libpaths-4, C-core-libpaths-12, C-core-libpaths-13 | カレントディレクトリを変えたあとでも、相対パスの文書を作業開始位置から読み、読み込みのあとでカレントディレクトリが変えた先のままであることを確かめている。REQ-schema-073 の分は schema の library-io の表 |

library-inputs の表にある crates/kotowari-core/tests/admission.rs の4本（repeated_native_source_is_rejected_within_a_group、native_source_without_an_identity_is_rejected、native_original_text_is_retained_across_read_and_check、repeated_original_test_identity_cannot_be_evaded_by_display_spelling）と native_display_collisions_do_not_merge_original_files も、付け替え先はこの話題の C-core-libpaths-9、C-core-libpaths-10 である（マーカーは REQ-core-315 だけなので、表は library-inputs の側に置いた）。

## 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-323、EX-core-500: 作業開始位置と明示するキャッシュ基準は絶対パスで受け、相対パスなら InvalidInput にする（library-api の REQ-core-311 の「作業開始位置は絶対パス」も同じ） | C-core-libpaths-11 | テスト project_rejects_relative_roots_and_plan_does_not_require_ir、relative_start_and_cache_bases_are_rejected（どちらも is_err だけを見るので、行には種類を書かず、InvalidInput は決めたことの文だけに書いた。コードは crates/kotowari/src/lib.rs と crates/kotowari-markdown-schema-io/src/lib.rs で InvalidInput を返す） |
| REQ-core-323、EX-core-501、library-api の REQ-core-311: あとでカレントディレクトリが変わっても解く基準を変えない。Project と SchemaLoader はカレントディレクトリを変えない | C-core-libpaths-12, C-core-libpaths-13 | テスト absolute_resolution_bases_survive_a_child_process_directory_change、later_current_directory_changes_do_not_change_the_load_base、project_operations_reuse_owned_results_and_explicit_reads_observe_changes |
| REQ-core-322、EX-core-498: 論理パスが空か絶対パスなら拒む。同じ群の中で正規化したあとに重複するパスは InvalidInput | C-core-libpaths-5, C-core-libpaths-6 | テスト logical_paths_are_normalized_without_accessing_the_filesystem、public_admission_errors_keep_distinct_facade_classifications_and_details、duplicate_normalized_paths_and_cross_group_source_mismatches_are_rejected |
| REQ-core-322、EX-core-503: 別の群の同じパスの本文が異なれば InvalidInput。テストと面の解析結果は SourceText の全文を持ち、本文で比べる | C-core-libpaths-7, C-core-libpaths-8, C-core-libpaths-10 | テスト duplicate_normalized_paths_and_cross_group_source_mismatches_are_rejected、inspection_compares_original_test_and_surface_source_texts、cached_analysis_of_another_revision_is_rejected_at_native_admission |
| library-inputs の REQ-core-315: 同じ群の中の重複したファイルと、内部の取り込みでの元のファイルの食い違いを拒む | C-core-libpaths-9, C-core-libpaths-10 | テスト repeated_native_source_is_rejected_within_a_group、repeated_original_test_identity_cannot_be_evaded_by_display_spelling、native_original_text_is_retained_across_read_and_check。論理パスの重複と同じ理由（同じファイルを別物として扱わない）なので、library-inputs ではなくこの話題の決定に置いた |
| REQ-core-322: ファイルシステムで正規化しない（シンボリックリンクの部分は下の「テストの無い行」） | C-core-libpaths-1, C-core-libpaths-2 | テスト logical_paths_are_normalized_without_accessing_the_filesystem（ファイルの無いパスを揃え、".." を解かずに残す）。決めたことに「ファイルシステムで正規化しない」の文を足した |

## 落とした細部

### 安全に関わるもの

無い（シンボリックリンクを解かないことは、テストが無いので下の「テストの無い行」に置いた）。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-322: 論理パスの表示と出典照合は同じプロジェクトの相対パスを使う | 設計担当（A51）。出典照合は形に依存する |
| REQ-core-322: 照合記録と Git の比較の対象に固有のプロジェクトの中のパスの制約は、その境界で当てる | 設計担当（A54）。照合記録と changes は利用者の決定（2026-10-06-changes-rethink.md の A10）で製品から外れている |

区切り文字、先頭の "./"、途中の "/./"、重なった区切り、末尾の区切りの正規化（A51）は、重複の決定の文に要るので、決めたことに書いた。

## 形に依存する要求

| 元の ID | 消える／読み替えて残る |
|---|---|
| REQ-core-322 | 読み替えて残る。「IrOptions の IR の置き場」と「出典照合」の部分は形に依存する。親ディレクトリの置き場を壊さないことは、新しい形では決定の文書の置き場（設定で変えられる）について成り立つ。文書には ".." を含む論理パス一般として書いた |
| EX-core-502 | 読み替えて残る。IR の置き場 "../spec/ir" の例を、".." を含む論理パスの行にした |

## テストの無い行

| 行（案） | 要るテスト |
|---|---|
| C-core-libpaths-（次の番号） シンボリックリンクを含むパスを論理パスに渡しても、`SourceText` はリンクを解かず、渡した綴りを揃えた論理パスを持つ | 一時ディレクトリにディレクトリへのシンボリックリンクを作り、そのリンクを通るパスで SourceText を作って、path() がリンク先ではなく渡したパスのままであることを確かめるテスト。今は無い（REQ-core-322 の「シンボリックリンクも解かない」。設計担当の A51） |

## 誰が決めたか見分けられなかったもの

- 「親ディレクトリを指す論理パスを拒まずに保つ」は設計担当の A54 だが、why が「既存 CLI が受け付ける親ディレクトリの IR 設定を壊さない」で、利用者の A9（既存の設定の受け付け方を変えない）の当てはめなので残した。
- 「設定ファイルの相対パスは作業開始位置から解く」「キャッシュ基準を省いたときは作業開始位置から既存の探し方で探す」は設計担当の A52 だが、どちらも「既存どおり」と書かれていて、利用者の A9 と A17（既存 CLI の探索とキャッシュの処理を移す）の当てはめなので残した。キャッシュ基準を省いたときの行は、確かめるテストがこの話題の ID を指していないので、確認することは置かなかった。

## 理由や見送った案が記録に無かった決定

- 「メモリの入力は、内容と論理パスの組で受け取り、ファイルを探さない」: 見送った案が記録に無い。空と絶対パスを拒む理由も記録に無い（問題の節に「理由は記録に残っていない」と書いた）。
- 「同じ論理パスの入力が重なったら、…」: 見送った案が記録に無い。理由は A51 の why（同じファイルを別物として扱うことを防ぐ）と A53 の why（内容の一致を追加の読み込みなしで判定できる入力を揃える）。
- 「ファイルを読む入口は作業開始位置を受け取り、…」: 見送った案が記録に無い。絶対パスとカレントディレクトリの理由は A52 の why（明示した作業位置を、プロセスの共有状態から独立させる）。

## 分けた所、まとめた所

- library-api の REQ-core-311（呼出開始位置を明示する、Project はカレントディレクトリを変えない）を、REQ-core-323 と一緒に「ファイルを読む入口は作業開始位置を受け取り、…」にまとめた。REQ-core-311 自身が位置の契約を REQ-core-323 に任せていて、どちらも A36、A52 から来る。library-api からはこの見出しへリンクしている。
- `LoaderOptions`（スキーマの側）の作業開始位置とカレントディレクトリは、スキーマの library-io の文書が「ライブラリ入力のパス」の話題に任せているので、ここに書いた。schema の library-io の対応表も「I/O のクレートはカレントディレクトリを変更しない」（REQ-schema-073、EX-schema-090、安全に関わる細部）を同じテスト later_current_directory_changes_do_not_change_the_load_base で戻す対象にしている。同じ決定が2つに書かれないよう、schema の側はこの文書の「ファイルを読む入口は作業開始位置を受け取り、既存の CLI と同じ規則でそこから解く」（`../core/library-paths.ja.md#ファイルを読む入口は作業開始位置を受け取り既存の-cli-と同じ規則でそこから解く`）へリンクし、C-core-libpaths-13 を指すのがよい（schema の側の文書は schema の書き直し役が持つので、ここでは直していない）。
- 重複と内容の食い違い（REQ-core-322 の後半、EX-core-498、EX-core-503、library-inputs の REQ-core-315 の内部の取り込み）を、新しい決定「同じ論理パスの入力が重なったら、同じファイルとして扱えるときだけ受け付ける」にまとめた。同じ理由（A51）から来る。

## あとでリンクにする所

- 無い。library-api、library-inputs、schema の library-io へのリンクは `.ja.md` の見出しへのリンクにした。

## 迷った判断と、どう決めたか

- 戻した行のうち、テストが is_err だけを見るもの（相対の作業開始位置、"./" の空のパス、内部の取り込みの重複）は、行に InvalidInput を書かず「失敗を返す」「拒む」と書いた。種類は決めたことの文にだけ書いた。
- 「作業開始位置は絶対パス」は library-api（安全の一覧の20）と library-paths（25）の両方に挙がっていたので、この話題に1つだけ書き、library-api からリンクした。カレントディレクトリ（19 と 26）も同じ。
- C-core-libpaths-3 と C-core-libpaths-12 は同じテストが確かめる。C-3 は設定ファイルを作業開始位置から解くこと、C-12 はカレントディレクトリが変わっても変わらないことで、確かめてほしい点が違うので行を分けた。
- inputs.rs の IR の解析のテスト2本は、library-inputs の表と同じく「古い形のコードと一緒に消す」にした。

## 数

- 元: 要求 2、場面 6、決定表 0、性質 0
- 読んだ記録のファイル: 2（2026-10-03-public-crate-api.md、2026-10-06-changes-rethink.md）
- 出来上がり: 決定 3、確認することの行 13（うち「（読んで確認）」0）
- テスト: 15（この表）。ほかに library-inputs の表の admission.rs の5本がこの話題の行を指す
- 戻した細部: この話題の安全の一覧の5件のうち、25〜28 の4件と、29 の「ファイルシステムで正規化しない」の部分。29 の「シンボリックリンクを解かない」は戻せなかった（要るテストは「テストの無い行」）。ほかに library-api の 19、20 と library-inputs の 24 も、この文書の決定に書いた
