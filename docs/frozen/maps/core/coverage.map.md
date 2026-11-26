# coverage の対応表

元: `docs/ir/core/coverage.md`（日本語は `coverage.ja.md`）。書き直した文書: `coverage.md`。確認することの ID の短い名前は `cover`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-085 | - | - | 読み替え。要求ごとのテストの有無は、確認することの行ごとのテストの有無に置き換わり、`docs/decisions/core/confirmations.md` の「check は、確認することの行ごとに、それを指すテストがあるかを数える」（C-core-confirm-1、C-core-confirm-2）にもう書いてある。同じことを書かないので、この文書には決定を置かない。検証が "review" の要求を除くことは「テストで確かめない行は、頭に「（読んで確認）」と書く」に、後回しの要求を除くことは「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」に、"- verification:" の行が無い要求の扱いと、シナリオのマーカーが要求の分も満たすことは IR の形と一緒に消える |
| REQ-core-086 | C-core-cover-1, C-core-cover-2 | マーカーの無いテストには test_without_id の誤りを出す | そのまま。誤りを出すことを C-1、名前が null のときの detail を C-2 にした |
| REQ-core-087 | C-core-cover-3, C-core-cover-4 | 問い合わせの無い言語のファイルのマーカーも、確認することの行を満たす側に数える | 読み替え。「requirement_without_test と scenario_without_test を消す側に数える」を「確認することの行を満たす側に数える」（confirmation_without_test を消す側）に読み替えた。今のコードは、問い合わせの無い言語のファイルのマーカーも同じ一覧に積み、確認することの行の検査に渡している（`crates/kotowari-core/src/tests_discovery.rs` と `inputs/read.rs`。コードを読んで確かめた） |
| REQ-core-088 | C-core-cover-5 | 決定の文書が1つも無くても、テストの検査は行う | 読み替え。「IR に文書が無い」を「決定の文書の置き場に文書が無い」に、「IR の検査の指摘を0件」を「決定の文書の検査の指摘を0件」に読み替えた |
| REQ-core-137 | - | - | 消える。場面（シナリオ）ごとのテストの有無は、シナリオと一緒に無くなる。確認することの行ごとに数えることに置き換わった（`docs/decisions/core/confirmations.md` の「check は、確認することの行ごとに、それを指すテストがあるかを数える」の見送った案に、以前の決定として書いてある） |
| EX-core-019 | C-core-cover-5 | 決定の文書が1つも無くても、テストの検査は行う | 読み替え。requirement_without_test が出ないことを、confirmation_without_test が出ないことと読んだ |
| EX-core-121 | - | - | 消える（scenario_without_test） |
| EX-core-122 | - | - | 消える（シナリオのマーカーが要求の分も満たすこと） |
| EX-core-123 | - | - | 消える（review の要求だけのシナリオ）。テストで確かめない行の扱いは `docs/decisions/core/confirmations.md` の「テストで確かめない行は、頭に「（読んで確認）」と書く」 |
| EX-core-124 | C-core-cover-3 | 問い合わせの無い言語のファイルのマーカーも、確認することの行を満たす側に数える | 読み替え。シナリオの ID を確認することの ID と読んだ |
| EX-core-125 | - | - | 消える（要求を挙げないシナリオ） |
| EX-core-310 | C-core-cover-2 | マーカーの無いテストには test_without_id の誤りを出す | そのまま |

元の文書に TBL と PROP は無い。

ガイドのマーカー: `docs/guides/marks.md`、`docs/guides/findings.md`、`docs/guides/deferred.md`、`docs/guides/commands/check.md`（それぞれ `.ja.md` も）にある、この話題の ID のマーカーは、上の「決定の見出し」で付け替える。REQ-core-085 と REQ-core-137 と EX-core-121、EX-core-122、EX-core-123 を指すマーカーは、決定の見出しが「-」なので、`docs/decisions/core/confirmations.md` の「check は、確認することの行ごとに、それを指すテストがあるかを数える」（EX-core-123 は「テストで確かめない行は、頭に「（読んで確認）」と書く」）へ付け替える。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step4_test_discovery.rs | req_085_fixture_has_no_uncovered_requirement | REQ-core-085 | - | requirement_without_test が出ないことを確かめる。古い形のコードと一緒に消す。確認することの行の分は step23 のテストが C-core-confirm-2 を確かめる |
| tests/step4_test_discovery.rs | req_086_fixture_has_no_unmarked_test_and_one_after_removal | REQ-core-086 | C-core-cover-1 | マーカーがあると出ず、外すと test_without_id が1件 |
| tests/step4_test_discovery.rs | req_087_unknown_language_only_feeds_coverage | REQ-core-087 | C-core-cover-4 | ".go" のファイルから test_without_id が出ない。準備に IR の要求を置いているが、確かめていることは IR に依らないので、設定の "ir" の鍵を外す程度で残せる |
| tests/step4_test_discovery.rs | req_087_marker_in_a_file_without_query_feeds_scenario_coverage | REQ-core-087, EX-core-124 | C-core-cover-3 | ".go" のマーカーがシナリオを満たす。付け替えのときに、準備を実装済みの決定の確認することの行と、それを指す ".go" のマーカーに直し、confirmation_without_test が出ないことを確かめる形にする必要がある |
| tests/step4_test_discovery.rs | req_088_empty_ir_still_checks_tests | REQ-core-088, EX-core-019 | C-core-cover-5 | 仕様の文書が無くても test_without_id が1件出て終了コード1。付け替えのときに、空の決定の文書の置き場を設定し、confirmation_without_test が出ないことを確かめる準備に直す必要がある |
| tests/step4_test_discovery.rs | req_085_requirement_without_verification_line_gets_no_coverage_finding | REQ-core-085 | - | "- verification:" の行の無い要求。古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_033_requirement_without_test_path_carries_the_subdirectory | REQ-core-033, REQ-core-085, REQ-core-110, TBL-core-006 | - | この話題の分は requirement_without_test だけで、消える。REQ-core-033、REQ-core-110、TBL-core-006 の分はそれぞれの話題の対応表で扱う |
| tests/step4_test_discovery.rs | req_137_scenario_without_marker_is_an_error | REQ-core-137, EX-core-121 | - | 消える要求だけを確かめる。古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_137_review_only_scenario_is_not_required | REQ-core-137, EX-core-123 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_scenario_about_a_table_only_is_not_required | REQ-core-137, EX-core-125 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_requirement_without_verification_line_does_not_count | REQ-core-137, REQ-core-048 | - | 同上。REQ-core-048 も IR の形の要求 |
| tests/step4_test_discovery.rs | req_137_requirement_with_invalid_verification_value_does_not_count | REQ-core-137, REQ-core-049 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_duplicate_scenario_uses_the_first_about | REQ-core-137, REQ-core-032 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_duplicate_scenario_reports_once_on_the_first | REQ-core-137, REQ-core-032 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_scenario_without_id_is_not_reported | REQ-core-137, REQ-core-053 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_scenario_with_invalid_id_is_not_reported | REQ-core-137, REQ-core-114 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_scenario_about_an_unknown_id_is_not_required | REQ-core-137, REQ-core-054 | - | 同上 |
| tests/step4_test_discovery.rs | req_137_one_marker_may_name_several_scenarios | REQ-core-137, REQ-core-071 | - | この話題の分は消える。REQ-core-071 の分（1つのマーカーに ID を並べる）は test-markers の対応表で C-core-markers-1 に当てた |
| tests/step4_test_discovery.rs | req_085_scenario_marker_covers_its_requirement | REQ-core-085, EX-core-122 | - | 消える要求だけを確かめる。古い形のコードと一緒に消す |
| tests/step4_test_discovery.rs | req_085_duplicate_scenario_marker_uses_the_first_about | REQ-core-085, REQ-core-032 | - | 同上 |
| tests/step13_test_languages.rs | ex_core_310_test_without_a_name_uses_its_first_line | REQ-core-086, EX-core-310 | C-core-cover-2 | 名前を捕まえないルールのテストの detail が最初の行の文字 |
| tests/step16_deferred.rs | ex_384_a_requirement_level_declaration_removes_the_missing_test | REQ-core-208, REQ-core-210, REQ-core-085, EX-core-384 | - | 後回しの宣言と requirement_without_test。どれも消える。古い形のコードと一緒に消す |
| tests/step16_deferred.rs | ex_385_a_document_level_declaration_defers_every_requirement_of_the_document | REQ-core-208, REQ-core-209, REQ-core-137, EX-core-385 | - | 同上 |
| tests/step16_deferred.rs | req_137_a_scenario_about_only_deferred_and_review_requirements_needs_no_test | REQ-core-137 | - | 同上 |
| tests/step16_deferred.rs | req_137_a_requirement_without_verification_does_not_keep_a_scenario_from_being_deferred | REQ-core-137 | - | 同上 |
| tests/step16_deferred.rs | req_137_the_first_of_two_scenarios_with_one_id_decides_whether_it_is_deferred | REQ-core-137 | - | 同上 |
| tests/step16_deferred.rs | req_208_the_first_of_two_requirements_with_one_id_decides_whether_it_is_deferred | REQ-core-208, REQ-core-085 | - | 同上 |
| tests/step16_deferred.rs | ex_395_a_scenario_about_deferred_and_other_requirements_is_a_notice_and_needs_a_test | REQ-core-212, REQ-core-137, EX-core-395 | - | 同上 |
| tests/step12_status.rs | tbl_028_requirement_covered_through_a_scenario_counts_as_with_tests | TBL-core-028, REQ-core-085, EX-core-263 | - | この話題の分（シナリオを通して要求を満たす）は消える。TBL-core-028 と EX-core-263 は status の話題の分 |

## 落とした細部

### 安全に関わるもの

無し。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-086: 名前が null のテストの detail で除く「前後の空白」が Unicode の空白であること | 利用者（2026-09-24-review10-gaps.md の A1 の why に、名前の無いテストの detail は Unicode の空白を除いている、とある）。決めたことには「前後の空白を除く」とだけ書き、範囲はテストに任せた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-085 | 新しい形で読み替えて残る。要求を単位に数えることは、確認することの行を単位に数えることに置き換わった（`docs/decisions/core/confirmations.md` の「check は、確認することの行ごとに、それを指すテストがあるかを数える」）。検証の値（review）は「（読んで確認）」の行に、後回しは「提案」の状態に置き換わった。"- verification:" の行が無い要求に verification_missing だけを出すことと、シナリオのマーカーが要求の分も満たすことは、IR の形と一緒に消える |
| REQ-core-137、EX-core-121、EX-core-122、EX-core-123、EX-core-125 | 消える。シナリオが無くなる |
| REQ-core-087、EX-core-124 | 新しい形で読み替えて残る（上の表） |
| REQ-core-088、EX-core-019 | 新しい形で読み替えて残る（上の表） |

テストの無い行: 無い。C-core-cover-3 と C-core-cover-5 は、今は古い形の準備のテストしか無いので、付け替えのときに準備を直す（上のテストごとの表）。

## 誰が決めたか見分けられなかったもの

無し。records.md の A21（第2ラウンド、推奨をそのまま採ったもの）、A24（第3ラウンド、利用者が自分の言葉で決めた方向）、A39 と A41（第6ラウンド、利用者の答え「推奨で」）、A51（第7ラウンド、「推奨で問題無いと思う」）は、test-discovery の対応表と同じく、利用者の決定として扱った。2026-09-24-multi-language-tests.md の A31、A42 と、2026-09-17-scenario-tests.md の A10 は「利用者（推奨を採用）」。

## 理由や見送った案が記録に無かった決定

- 「マーカーの無いテストには test_without_id の誤りを出す」: マーカーの無いテストを誤りにする理由が記録に無い（records.md の A21 は検査の項目を並べただけ）。問題の節に「理由は残っていない」と書いた。見送った案も無い。
- 「問い合わせの無い言語のファイルのマーカーも…」: 見送った案が記録に無い。理由は、scenario-tests の A10 の「言語ごとの扱いを2つにしない」だけを書いた。test_without_id を出さない理由は記録に無い（問い合わせの無い言語ではテストを見つけない、という事実だけを書いた）。
- 「決定の文書が1つも無くても、テストの検査は行う」: 理由も見送った案も記録に無い（records.md の A41、A51 に why が無い）。

## 分けた所、まとめた所

- REQ-core-086 と、名前が null のときの detail（2026-09-24-multi-language-tests.md の A31、A42）を1つの決定にした。
- REQ-core-087 の「test_without_id を出さない」と「数える」を1つの決定にし、行を C-3 と C-4 に分けた。
- REQ-core-085 と REQ-core-137 は決定を置かず、core の文書を言葉で指した。

## あとでリンクにする所

確定の段で、confirmations.md の「check は、確認することの行ごとに、それを指すテストがあるかを数える」（冒頭の段落）を `confirmations.ja.md` の見出しへのリンクにした。test-markers へのリンクも `test-markers.ja.md#<anchor>` に直した。

残り: 無し。

## 実装する側が決める細部

無し。

## 利用者に聞くこと

無し。

## 「提案」で書いた決定の一覧

無し。3つの決定とも、今のコードがもう新しい形で動いている。切り替えで要る作業は、`crates/kotowari-core/src/tests_discovery.rs` の `check_missing_tests` から requirement_without_test と scenario_without_test の検査を消すことと、上の「準備を直す」テストの準備の書き直しである。

## 迷った判断と、どう決めたか

- この話題の中心だった REQ-core-085 は、core の文書にもう書いてあるので、決定を置かなかった。元の ID の表の「決定の見出し」は「-」にし、理由に core の見出しを書いた。ガイドのマーカーの付け替え先が、書き直した話題の外（core の文書）になる。元の ID の表の決定の見出しの列は「書き直した文書の決定の見出し」なので、core の文書の見出しを書く場所が無い。形の決まりで足りなかった所。
- C-core-cover-5 の文は、元の場面の「requirement_without_test は出ない」を「confirmation_without_test は出ない」と読み、終了コード1を入れた。
- 名前が null のテストの detail の理由のうち、「ほかの種類でも行の文字そのままを detail にしている」（A31）と「line も行で決めている」（A42）を、決めたことの文に書いた。A31 の why が引く records.md の A150 は、IR の指摘の detail の決まりで、中身だけを書いた。

## 数

- 元: 要求 5、場面 7、決定表 0、性質 0
- 読んだ記録のファイル: 4（records.md の A21、A24、A39、A41、A51、A120、2026-09-17-scenario-tests.md、2026-09-24-multi-language-tests.md の A31、A42、2026-09-25-deferred-items.md の A3、A4）。ir-form.md の「検査の種類」と ADR の 0002 は、凍結される古い形の文書で、この話題の残る決定の理由を持たないので読まなかった
- 出来上がり: 決定 3、確認することの行 5（うち「（読んで確認）」0）
- テスト: 29（この話題の ID を指すもの）
