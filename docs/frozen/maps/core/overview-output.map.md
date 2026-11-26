# overview-output の対応表

元: `docs/ir/core/overview-output.md`（日本語は `overview-output.ja.md`）。書き直した文書: `overview-output.md`。確認することの ID の短い名前は `ovout`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-288 | C-core-ovout-1 | check の JSON に、ガイドと同じ形の overview の群を出す | そのまま。数え方を "guides" と同じにすること、"overview" の鍵が無いときも 0 で出すこと、"--format text" では出さないことは落とした |
| REQ-core-289 | - | check の JSON に、ガイドと同じ形の overview の群を出す | そのまま。status が同じ群を出すことは決めたことに1文で書き、確認することは[揃っているかの集計](status.md)の C-core-status-7 に任せた（同じ決定が2つの話題に出るので、群の中身が見える status の側に行を置いた） |
| REQ-core-290 | C-core-ovout-2, C-core-ovout-3, C-core-ovout-4 | 元データの指摘は、ほかの指摘と同じく並べて数え、終了コードと complete に効かせる | そのまま。findings、counts、終了コードを C-2、並びを C-3、status の complete を C-4 にした。kotowari のライブラリが core に追加の指摘の群として渡すことは[メモリの入力と読み込んだ結果](library-inputs.md)の「名前を付けた追加の指摘の群を受け取り、ほかの指摘と一緒に並べて数える」にあるので、冒頭の段落からリンクした |
| EX-core-472 | C-core-ovout-1, C-core-ovout-2 | 元データの指摘は、ほかの指摘と同じく並べて数え、終了コードと complete に効かせる | そのまま。"files" が 2 なのは C-1 |
| EX-core-473 | C-core-ovout-4 | 元データの指摘は、ほかの指摘と同じく並べて数え、終了コードと complete に効かせる | そのまま。"overview" の群が status に出ることは C-core-status-7 |

元の文書に TBL と PROP は無い。

ガイドのマーカー: `docs/guides/commands/check.md`（と `.ja.md`）の REQ-core-288 は、上の「決定の見出し」（check の JSON に、ガイドと同じ形の overview の群を出す）に付け替える。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari/tests/project.rs | project_check_status_and_inspect_include_the_overview_group | REQ-core-290, REQ-core-310, TBL-core-041 | C-core-ovout-2 | この話題の分は、ライブラリの check と status の結果に元データの指摘が入ること。REQ-core-310 と TBL-core-041 は「プロジェクトを使う Rust API」の話題 |
| crates/kotowari-overview/tests/overview.rs | the_findings_and_numbers_become_the_overview_group | REQ-core-290, TBL-core-042 | C-core-ovout-1 | 元データの数とマーカーの数が "overview" の群の "files" と "marks" になる。TBL-core-042 は別の話題 |
| crates/kotowari-overview/tests/overview.rs | req_core_288_the_number_of_files_counts_every_overview_data_file | REQ-core-288, TBL-core-042 | C-core-ovout-1 | 読んだ元データの数 |
| crates/kotowari-core/tests/inputs.rs | an_additional_group_is_sorted_counted_and_reported_with_its_numbers | REQ-core-315, TBL-core-042, REQ-core-290 | - | 名前を付けた追加の群一般のテストで、C-core-libinputs-7 に当たる（その対応表が付けるので、マーカーは無くならない）。元データの指摘の振る舞いは CLI のテストが確かめる |
| tests/step18_overview.rs | ex_core_464_overview_data_in_a_named_hidden_directory_is_read | EX-core-464, REQ-core-278, REQ-core-019, REQ-core-288 | C-core-ovout-1 | この話題の分は "overview" の群の "files" と "marks" |
| tests/step18_overview.rs | ex_core_472_errors_in_overview_data_join_findings_counts_and_the_exit_code | EX-core-472, REQ-core-288, REQ-core-290, REQ-core-027 | C-core-ovout-1, C-core-ovout-2 | REQ-core-027 は別の話題 |
| tests/step18_overview.rs | req_core_290_overview_findings_are_sorted_with_the_other_findings | REQ-core-290, REQ-core-024 | C-core-ovout-3 | 準備で、元データの指摘の間に並ぶほかの指摘が IR の文書の誤りなので、付け替えのときに決定の文書の誤りに直す必要がある。REQ-core-024 は指摘の並べ方の話題 |
| tests/step18_overview.rs | req_core_288_the_text_format_does_not_show_the_overview_group | REQ-core-288 | C-core-ovout-1 | 落とした細部のテスト。いちばん近い行を指す。"--format text" で群を出さないことは、テストの中に残る |
| tests/step18_overview.rs | ex_core_463_without_the_overview_key_check_reads_no_overview_data | EX-core-463, REQ-core-279, REQ-core-288, TBL-core-005 | - | この話題の分は、鍵が無いときも群を 0 で出すこと（落とした細部）。鍵が無いと読まないことは全体像の元データの話題の C-core-ovdata-17 で、その対応表が付ける |
| tests/step18_overview.rs | ex_core_473_an_error_in_overview_data_keeps_status_from_complete | EX-core-473, REQ-core-289, REQ-core-290, REQ-core-162, TBL-core-028 | C-core-ovout-4, C-core-status-7 | complete が false になることが C-4、status に "overview" の群が出ることが C-core-status-7。REQ-core-162 と TBL-core-028 は status の話題 |
| tests/step18_overview.rs | req_core_289_status_text_shows_the_overview_group_after_guides | REQ-core-289, REQ-core-166, TBL-core-028 | C-core-status-7 | status の text に "overview" の群が出る。行の位置と形は status の話題 |
| tests/step18_overview.rs | req_core_325_check_reads_the_toc_and_reports_its_errors_on_the_toc_file | REQ-core-325, REQ-core-328, REQ-core-329, REQ-core-288 | - | 目次の話題のテストで、その対応表が C-core-ovtoc-1 を付ける。"overview" の群の数は C-core-ovout-1 のほかのテストで足りる |
| tests/step21_translations.rs | three_languages_keep_their_own_html_language_and_contents | REQ-core-288, REQ-core-293, REQ-core-353, REQ-core-355 | - | この話題の分は、対のある元データの数え方（言語の話題の細部）。全体像の言語の話題の対応表が C-core-ovlang-2、C-core-ovlang-4 を付ける |

この話題の ID を指すテストは 13。

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-288: "marks" の数え方を "guides" と同じにする | 記録の決定は「guides と同じ形の群」（2026-10-02-whole-picture.md の A64、利用者）で、数え方を同じにする文は出典の要求の文だけにある。形が同じなら数え方も同じと読めるが、決めたことには書かなかった |
| REQ-core-288: "overview" の鍵が無いときも、"files" と "marks" を両方 0 で出す | LLM（A78） |
| REQ-core-288: "--format" が "text" のときは群を出さない | LLM（A73） |

## 形に依存する要求

無い。

## テストの無い行

無い。

## 誰が決めたか見分けられなかったもの

- 無い。

## 理由や見送った案が記録に無かった決定

- 「check の JSON に、ガイドと同じ形の overview の群を出す」: 見送った案が記録に無い。理由は「ガイドと同じ扱いにし、新しい合否の決まりを作らない」だけ。
- 「元データの指摘は、ほかの指摘と同じく並べて数え、終了コードと complete に効かせる」: 見送った案が記録に無い。

## 分けた所、まとめた所

- 記録の1つの決定（A64）を、群の決定と、指摘の数え方の決定の2つに分けた。並び順の決定（A50）と、check に誤りとして出す決定（A33）を後者に入れた。
- REQ-core-289 は、確認することを status の話題の C-core-status-7 に任せ、この文書では1文にした。

## あとでリンクにする所

無い。リンク先の話題は、どれもこのフォルダにあり、`<file>.ja.md#<anchor>` のリンクにした。

## 迷った判断と、どう決めたか

- status の "overview" の群（REQ-core-289）は、[揃っているかの集計](status.md)がもう群の表と確認することの行（C-core-status-7）を持つので、同じ行を足さなかった。
- 問題の節には、記録の理由（ガイドと同じ扱い、check に入れればコミットの前の検査でも捕まる、出力の順番の決まりを2つに分けない）だけを書いた。

## 実装する側が決める細部

無い。

## 利用者に聞くこと

無い。

## 「提案」で書いた決定の一覧

無い。

## 数

- 元: 要求 3、場面 2、決定表 0、性質 0
- 読んだ記録のファイル: 3（2026-10-02-whole-picture.md、2026-10-03-public-crate-api.md、2026-10-04-overview-on-public-api.md）
- 出来上がり: 決定 2（実装済み 2）、確認することの行 4（うち「（読んで確認）」0）
- テスト: この話題の ID を指すテスト 13
