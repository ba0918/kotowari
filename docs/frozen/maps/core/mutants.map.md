# mutants の対応表

元: `docs/ir/core/mutants.md`（日本語は `mutants.ja.md`）。書き直した文書: `mutants.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-139 | C-core-mutants-3, C-core-mutants-4, C-core-mutants-6 | 見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない | 見逃しの指摘の形を C-3、捕まえた変異とビルド不能の変異に何も出さないことを C-4、畳まないことを C-6 にした。等価の一覧に一致しないとき、の条件は equivalents の C-core-equiv-5 が確かめる |
| REQ-core-140 | C-core-mutants-5 | 見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない | そのまま |
| REQ-core-145 | C-core-mutants-7, C-core-mutants-8 | 集計を必ず出し、等価として外した件数を毎回見せる | 定義の TBL-core-025 を C-7、PROP-core-005 を C-8 にした |
| REQ-core-146 | C-core-mutants-9 | 集計を必ず出し、等価として外した件数を毎回見せる | そのまま |
| REQ-core-147 | C-core-mutants-1, C-core-mutants-2 | 変異テストは kotowari の外で走らせ、kotowari mutants はその結果のファイルを読んで指摘にする | 形に依存する部分を読み替えた（下の「形に依存する要求」）。置き場の指す先が無くても停止しないことを C-1、check の検査をしないことを C-2 にした |
| REQ-core-150 | C-core-mutants-11 | 指摘の種類と detail に、変異テストの道具に固有の語を使わない | 検証が review なので「（読んで確認）」。how_to_verify の rg の対象（crates/kotowari-core/src/mutants.rs と finding.rs、cargo_mutants.rs は対象外）は、行の文では「組み立てるコード」「写す部品」と書いた |
| TBL-core-025 | C-core-mutants-7 | 集計を必ず出し、等価として外した件数を毎回見せる | 表は「決めたこと」に Markdown の表で書いた。findings の鍵と並べ方の参照先（TBL-core-006、TBL-core-007）は「check と同じ」と書いた |
| PROP-core-005 | C-core-mutants-8 | 集計を必ず出し、等価として外した件数を毎回見せる | 性質を確認することの行にした |
| EX-core-204 | C-core-mutants-3 | 見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない | @about に REQ-core-138 と TBL-core-024 も持つ。その分は mutants-input の C-core-mutin-1 |
| EX-core-205 | C-core-mutants-4 | 見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない | caught と unviable の数は C-7 の鍵でもある |
| EX-core-206 | C-core-mutants-5 | 見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない | そのまま |
| EX-core-209 | C-core-mutants-9, C-core-mutants-10 | 集計を必ず出し、等価として外した件数を毎回見せる | 0件でも1行が出ることが C-9、0件で終了コード0・すべて0が C-10 |
| EX-core-210 | C-core-mutants-1 | 変異テストは kotowari の外で走らせ、kotowari mutants はその結果のファイルを読んで指摘にする | 「IR の置き場」を「check が読む置き場」に読み替えた |
| EX-core-229 | C-core-mutants-6, C-core-mutants-8 | 見逃しは誤り、時間切れは注意にし、捕まえた変異とビルド不能の変異には何も出さない | survived が2になることは C-8 |
| EX-core-230 | C-core-mutants-9 | 集計を必ず出し、等価として外した件数を毎回見せる | 指摘の行の文字の形（"src/a.rs:3 [error] mutant_survived ..."）は「出力」の話題の決まり |
| EX-core-231 | C-core-mutants-2, C-core-mutants-7 | 変異テストは kotowari の外で走らせ、kotowari mutants はその結果のファイルを読んで指摘にする | マーカーの無いテストに指摘を出さないことが C-2、最上位の3つの鍵が C-7 |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step9_mutants.rs | req_139_survived_mutant_is_an_error | REQ-core-138, REQ-core-139, TBL-core-024, EX-core-204 | C-core-mutants-3 | mutant_survived の severity、path、line、detail と終了コード1を確かめている。REQ-core-138 と TBL-core-024 の分（C-core-mutin-1、C-core-mutin-2）は mutants-input の対応表に書いた |
| tests/step9_mutants.rs | req_139_caught_and_unviable_mutants_yield_nothing | REQ-core-139, REQ-core-145, EX-core-205 | C-core-mutants-4 | findings が空、caught と unviable が1、終了コード0。"summary" の写し方の分は mutants-input の対応表に書いた |
| tests/step9_mutants.rs | req_140_timeout_is_a_notice_even_when_listed | REQ-core-140, EX-core-206 | C-core-mutants-5, C-core-mutants-8 | 一覧に合う1件があっても mutant_timeout の notice が出て終了コード0。timeout が1で equivalent が0なので C-8 も確かめている。equivalent_stale が出ないことは equivalents の対応表に書いた |
| tests/step9_mutants.rs | req_139_duplicate_mutants_yield_one_finding_each | REQ-core-139, PROP-core-005, EX-core-229 | C-core-mutants-6, C-core-mutants-8 | 指摘が2件、counts の mutant_survived が2、survived が2 |
| tests/step9_mutants.rs | req_145_no_mutants_exits_zero_with_all_zero_counts | REQ-core-145, REQ-core-146, PROP-core-005, EX-core-209 | C-core-mutants-9, C-core-mutants-10 | 標準出力が集計の1行だけで終了コード0。0件でも1行が出る |
| tests/step9_mutants.rs | req_146_summary_is_the_last_line_after_findings | REQ-core-146, EX-core-230 | C-core-mutants-9 | 1行目が指摘、最後の行が集計 |
| tests/step9_mutants.rs | req_147_missing_ir_directory_does_not_stop_mutants | REQ-core-147, EX-core-210 | C-core-mutants-1 | 設定の "ir" の指す先が無くても終了コード0。切り替えたあとは、設定の鍵を決定の文書の置き場の鍵に書き換える必要がある（下の「形に依存する要求」） |
| tests/step9_mutants.rs | req_147_unmarked_test_is_not_reported_and_json_has_three_keys | REQ-core-147, TBL-core-025, EX-core-231 | C-core-mutants-2, C-core-mutants-7 | findings が空と、最上位の鍵が3つだけ |
| tests/step9_mutants.rs | req_141_listed_survivor_is_counted_as_equivalent | REQ-core-139, REQ-core-141, REQ-core-145, EX-core-211 | C-core-mutants-8 | survived 0、equivalent 1。この話題の分は合計の性質だけ。一致の分は equivalents の対応表に書いた |
| tests/step9_mutants.rs | req_144_line_one_is_read | REQ-core-144, REQ-core-139 | C-core-mutants-3 | 1行目の見逃しが line 1 の mutant_survived になり終了コード1。REQ-core-144 の分は mutants-input の対応表に書いた |

REQ-core-150 を指すテストは無い（検証が review）。

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-core-025: findings の鍵と並べ方を TBL-core-006、TBL-core-007 で決めること | 利用者（2026-09-17-mutation-tests.md の A35、A40）。中身は「出力」の話題と「指摘の並べ方」の話題の決定なので、「check と同じ」とだけ書いた |
| REQ-core-150 の how_to_verify: 確かめるファイルの名前と rg のコマンド | 記録に無い（IR の確認の方法の行）。確かめ方はテストやレビューの側に任せ、行の文には対象の範囲だけを書いた |

## 形に依存する要求

| 元の ID | 扱い |
|---|---|
| REQ-core-147 | 新しい形で読み替えて残る。「IR とテストのファイルと判断の記録と ADR を読まない」は「仕様の文書とテストのファイルを読まない」に、「"ir"、"decisions.records"、"decisions.adr" の指す先が無くても停止しない」は「check が読む置き場の指す先が無くても停止しない」に読み替えた。切り替えたあとは、決定の文書の置き場の設定が指す先が無くても停止しない、になる |
| EX-core-210 | 同上。テスト req_147_missing_ir_directory_does_not_stop_mutants は設定の "ir" を使っているので、切り替えのときに決定の文書の置き場の鍵へ書き換える |

テストの無い行: 無い（C-core-mutants-11 は「（読んで確認）」）。

## 誰が決めたか見分けられなかったもの

無い。読んだ決定はすべて decided_by が利用者（推奨を採用、または利用者が足した）。

## 理由や見送った案が記録に無かった決定

- 「見逃しは誤り、時間切れは注意にし…」: 見逃しを誤りにする理由は記録に直接は無い（A11 の why は時間切れの理由）。問題の節には、見逃しの意味（用語）と A2 の why だけを書いた。
- 「指摘の種類と detail に、変異テストの道具に固有の語を使わない」: 見送った案が記録に無い。節を置かなかった。

## 分けた所、まとめた所

- REQ-core-139 と REQ-core-140 を1つの決定にまとめた。どちらも A11（見逃しは誤り、時間切れは注意、ほかは何も出さない）が本体で、指摘の形（A40）も共通のため。
- REQ-core-145、REQ-core-146、TBL-core-025、PROP-core-005 を1つの決定「集計を必ず出し…」にまとめた。どれも A28 の1段目、A35、A38 から来ている。
- REQ-core-147 には、話題の土台の決定（A1 の実行を外に置くこと、A8 の別のコマンドにすること、A27 の0件の強制を呼ぶ側に置くこと）を入れて、1つの決定にした。A1 と A8 は REQ-core-138、REQ-core-147 の出典にある。
- 等価として外した見逃しを survived に数えないことは、集計の決定に書き、一致したときの振る舞いは等価の一覧の決定にも書いた（equivalents の C-core-equiv-5 が確かめる）。

## あとでリンクにする所

確定の段で、冒頭の段落を次の見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- cli の「kotowari mutants は道具の名前を --tool で必ず受け、結果のファイルをちょうど1つ受ける」（"--tool" と結果のファイルの引数。元の REQ-core-149）
- cli-environment の「停止の理由は、英語の固定の文言で示す」（停止の文言。元の TBL-core-001）
- output の「指摘は kind、severity、path、line、detail の5つの鍵で出す」（指摘の鍵。元の TBL-core-006）
- finding-order の「指摘は path、line、kind、detail の順に並べる」（並べ方。元の TBL-core-007）

残り: 「指摘」の話題（findings）の、4つの指摘の種類の一覧（元の TBL-core-008、TBL-core-009）。種類の名前と意味の決定はこの話題に置いたので、向こうからこちらを指す。この文書から指す所は無い


## 迷った判断と、どう決めたか

- 「見逃し0件」の強制（A27）は REQ の出典に直接は無いが、A1 の superseded_by が A27 を指し、A1 は REQ-core-138 の出典にある。製品の振る舞いではないが、kotowari mutants が0件を求めない理由として要るので、決めたことに1文だけ書いた。強制する関門が何か（pre-push か CI か）は別の記録（2026-10-04-mutants-in-ci.md）の話で、ここでは「呼ぶ側の関門」とだけ書いた。
- 等価の申告への歯止めの2段目と3段目（A28）は手順の決まりで、この話題の振る舞いではない。集計の理由として問題の節にだけ書いた。
- PROP-core-005 を指す既存のテストは、性質のテスト（どんな入力でも成り立つことを確かめるもの）ではなく、具体的な入力のテストである。行は性質の文のまま残し、具体的なテストを付け替え先にした。性質のテストを要求するかは、check の数え方（性質の行を何が指すか）次第で、形の決まりで足りなかった所。
- 指摘の種類の4つの名前（A31）は、equivalent_stale と equivalent_invalid も含めて、道具に固有の語を使わない決定の中に書いた。この2つの振る舞いは equivalents の決定にある。
- C-core-mutants-8（性質の行）は、確定の段でも性質の文のまま残し、付け替え先は具体的な入力のテストのままにした（性質のテストは無い）。

## 数

- 元: 要求 6、場面 8、決定表 1、性質 1
- 読んだ記録のファイル: 1（2026-09-17-mutation-tests.md）
- 出来上がり: 決定 4、確認することの行 11（うち「（読んで確認）」1）
- テスト: 10
