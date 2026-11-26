# test-side-findings の対応表

元: `docs/ir/core/test-side-findings.md`（日本語は `test-side-findings.ja.md`）。書き直した文書: `test-side-findings.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-357 | C-core-testside-1, C-core-testside-2 | check に --allow-test-findings を付けると、テスト側の指摘を終了コードに数えない | 終了コードを C-1、出力が変わらないことを C-2 にした |
| REQ-core-358 | C-core-testside-9 | フックの組み方を、スキルと check のガイドで案内する | 元が verification: review なので「（読んで確認）」の行にした。how_to_verify の3か所をそのまま行に書いた |
| TBL-core-047 | C-core-testside-4, C-core-testside-5, C-core-testside-6, C-core-testside-7, C-core-testside-8 | テスト側の指摘は、テストのファイルが原因の誤りに限り、範囲を表で定める | 表は決めたことに置いた。requirement_without_test と scenario_without_test の行を confirmation_without_test の1行に読み替えた（形に依存する要求） |
| EX-core-547 | C-core-testside-1, C-core-testside-2 | check に --allow-test-findings を付けると、テスト側の指摘を終了コードに数えない | オプション付きで0になることを C-1、指摘が severity "error" のまま出ることを C-2 にした |
| EX-core-548 | C-core-testside-1 | check に --allow-test-findings を付けると、テスト側の指摘を終了コードに数えない | 値違いの場面として C-1 の1行にまとめた |
| EX-core-549 | C-core-testside-6 | テスト側の指摘は、テストのファイルが原因の誤りに限り、範囲を表で定める | 「IR の誤り」を「表に無い種類の誤り」と読んだ |
| EX-core-550 | C-core-testside-8 | テスト側の指摘は、テストのファイルが原因の誤りに限り、範囲を表で定める | そのまま |
| EX-core-551 | C-core-testside-7 | テスト側の指摘は、テストのファイルが原因の誤りに限り、範囲を表で定める | そのまま |
| EX-core-552 | C-core-testside-3 | check に --allow-test-findings を付けると、テスト側の指摘を終了コードに数えない | 停止することを残した。停止の理由が引数の誤りであることは、主セッションの決定（A12）なので落とした |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step22_test_side_findings.rs | ex_core_547_a_specification_ahead_of_its_tests_passes_with_the_option | EX-core-547 | C-core-testside-1, C-core-testside-2, C-core-testside-4 | オプション付きで終了コード0、テストの無いことの指摘が severity "error" のまま出る。準備が IR の要求と requirement_without_test なので、付け替えのときに、実装済みの決定と confirmation_without_test の準備に直す必要がある |
| tests/step22_test_side_findings.rs | ex_core_548_the_same_state_stops_without_the_option | EX-core-548 | C-core-testside-1 | オプション無しで終了コード1。準備は EX-core-547 と同じく直す必要がある |
| tests/step22_test_side_findings.rs | ex_core_549_an_error_in_the_ir_stops_even_with_the_option | EX-core-549 | C-core-testside-6 | 表に無い種類（unknown_term）の誤りで止まる。用語の検査は新しい形で無くなるので、付け替えのときに、表に無いほかの種類の誤りを準備に使う必要がある |
| tests/step22_test_side_findings.rs | ex_core_550_an_unreadable_test_file_that_is_also_a_surface_file_stops | EX-core-550 | C-core-testside-8 | 両方に当たるファイルの unparsable_file で止まる |
| tests/step22_test_side_findings.rs | tbl_core_047_an_unreadable_test_file_matched_by_surface_files_in_no_rule_language_stops | TBL-core-047 | C-core-testside-8 | 面の規則の言語でなくても、"surface.files" に当たるテストのファイルは両方に当たるファイルとして止まる |
| tests/step22_test_side_findings.rs | tbl_core_047_errors_only_in_test_files_pass_with_the_option | TBL-core-047 | C-core-testside-5 | テストのファイルの3種類の誤りだけなら終了コード0 |
| tests/step22_test_side_findings.rs | ex_core_551_a_malformed_guide_mark_stops_even_with_the_option | EX-core-551 | C-core-testside-7 | ガイドのマーカーの形の誤りで止まる |
| tests/step22_test_side_findings.rs | req_core_357_the_option_leaves_the_output_unchanged | REQ-core-357 | C-core-testside-2 | json と text の両方で、オプションの有無で出力が同じ |
| crates/kotowari-core/tests/test_side_findings.rs | tbl_core_047_each_row_decides_whether_an_error_is_a_test_side_finding | TBL-core-047 | C-core-testside-4, C-core-testside-5, C-core-testside-6, C-core-testside-7, C-core-testside-8 | 表の各行の判定を確かめる。今の場合の並びは requirement_without_test と scenario_without_test を持ち、confirmation_without_test を持たないので、付け替えのときに confirmation_without_test の場合を足し、2つの古い種類を外す必要がある |
| tests/step1_cli.rs | ex_core_552_allow_test_findings_on_status_is_an_argument_error | REQ-core-004, EX-core-552 | C-core-testside-3 | この話題の分だけ。REQ-core-004 は CLI の話題の ID。停止の理由が引数の誤りであることも確かめるが、それは落とした細部 |

## 戻した細部

無い。安全に関わる落とした細部はこの話題に無い。

## 落とした細部

### 安全に関わるもの

無し。止める範囲を決める部分（表の各行、両方に当たるファイル、ガイドと面のファイルを含めないこと）は、どれも利用者の決定（A2、A3、A9）として残した。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| EX-core-552: check でないコマンドに付けたときの停止の理由が「引数の誤り」であること | 主セッション（2026-10-06-spec-first-commit.md の A12。「A1 と A5 から導いた。承認で確かめる」） |
| A12: `--allow-test-findings` は値を取らない | 主セッション（A12） |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| TBL-core-047 の行 requirement_without_test、scenario_without_test | 新しい形で読み替えて残る。要求と場面が確認することの行に置き換わり、2つの種類は confirmation_without_test の1つになる（`docs/decisions/core/confirmations.md` の「check は、確認することの行ごとに、それを指すテストがあるかを数える」）。今のコードは4つの種類のどれも「常に」で扱っている |
| EX-core-547、EX-core-548 | 新しい形で読み替えて残る。「verification: unit の要求があり、その ID を含む印のテストが無い」を「実装済みの決定の確認することの行を指すテストが無い」と読む |
| EX-core-549 | 新しい形で読み替えて残る。「IR の文書に用語集に無い語を囲んだ行」を「表に無い種類の誤り」と読む。用語集と用語の検査は新しい形で無くなる |
| 後回しに関する指摘 | 元の表に行は無い。後回しの宣言と、それに関する注意は新しい形で無くなる（`docs/decisions/core/confirmations.md` の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」）ので、表に足す行も無い |

テストの無い行は無い。ただし C-core-testside-1 と C-core-testside-4 の confirmation_without_test の部分は、この話題の ID を指すテストでは、古い形の準備（要求と requirement_without_test）でしか確かめていない。新しい形で確かめているテストは、`tests/step23_decision_documents.rs` の `a_confirmation_without_a_test_does_not_stop_check_with_allow_test_findings`（今のマーカーは C-core-confirm-1 で、この話題の ID を指していない）にある。confirmation_without_test だけがあるときに、オプション付きで終了コード0、無しで1になることを確かめている。このテストのマーカーに C-core-testside-1 と C-core-testside-4 を足すか、表の各行のテストに confirmation_without_test の場合を足す必要がある。

## 誰が決めたか見分けられなかったもの

無し。2026-10-06-spec-first-commit.md の A1〜A11 は「利用者（推奨を採用）」、A12 は「主セッション」と書いてある。

## 理由や見送った案が記録に無かった決定

- 「フックの組み方を、スキルと check のガイドで案内する」: 見送った案が記録に無い（A7 に rejected が無い）。

見送った案のうち、この話題の決定の代わりでないものは書かなかった: R1（途中から導入するプロジェクトのための検査の厳しさの段階。テスト側の指摘の代わりではなく、導入のやり方の案）。

## 分けた所、まとめた所

- REQ-core-357 を、終了コード（C-1）と出力が変わらないこと（C-2）に分けた。check だけが受けること（A5）は同じ決定の中に置き、C-3 にした。
- TBL-core-047 と、その範囲の理由（A2、A3、A9）を、オプションの決定とは別の決定にした。範囲の決め方には独自の理由と見送った案（A9）があるため。
- EX-core-547 と EX-core-548 を、値違いの場面として C-1 の1行にまとめた。
- C-8 に、両方に当たるファイルの場合（A3）と、面のファイルだけの場合（A2）をまとめた。

## あとでリンクにする所

- 済み: 冒頭の段落の、オプションの受け口と終了コードの一覧は、cli の見出しへのリンクにした。`confirmations.ja.md` と `workflow.ja.md` へのリンクも `.ja.md` に直した。

## 迷った判断と、どう決めたか

- `docs/decisions/core/confirmations.md` の「状態は実装する側が変え、まだ作らない決定は「提案」のまま置く」は、問題の節で、このオプションを「仕様を先にコミットしたときに止まるのを通すために足していた」ものとして書いている。一方 `docs/decisions/core/workflow.md` の「既存のコードを取り込むときは…」は、このオプションを残すと書いている。どちらも状態は「提案」である。今の振る舞い（オプションはあり、表には confirmation_without_test が入っている）を「実装済み」で書き、問題の節で両方にリンクした。2つの文書のどちらが正かは、この書き直しの中では決めていない。
- EX-core-552 の「停止の理由は引数の誤り」は A12（主セッション）なので落としたが、check だけが受けること（A5、利用者）から、status に付けると停止することは残した（C-3）。受けないオプションを停止にする一般の決まりは「CLI」の話題に属する。
- 用語「誤り」の定義の改め（A10）と、用語「テスト側の指摘」の追加（A11）は、用語集の形に依存するので決定にしなかった。中身（オプションを付けてもテスト側の指摘は誤りのまま、終了コードに数えないだけ）は、オプションの決定の文に入れた。
- A19 の見送った案（フックを飛ばす、新しい要求を一旦 review で入れる）は、テスト側の指摘を pre-commit で止めないと決めたときのものなので、オプションの決定の見送った案に入れた。「review で入れる」は古い形の語なので、「テストで確かめない形で仕様に入れる」と書いた。
- 問題の節の、フックの設定に種類を直書きしていたことは、記録ではツールの名前と設定のファイルの名前で書かれているが、名前を出さずに中身だけを書いた。
- kotowari 自身のフックの設定をオプションに切り替えたこと（A8）は、記録に「利用者から見える製品の振る舞いではない」とあり、元の要求も引いていないので書かなかった。

## 数

- 元: 要求 2、場面 6、決定表 1、性質 0
- 読んだ記録のファイル: 3（2026-10-06-spec-first-commit.md、2026-09-17-decision-log.md の A19、records.md の A20 と A60）
- 出来上がり: 決定 3、確認することの行 9（うち「（読んで確認）」1）
- テスト: 10（この話題の ID を指すもの）。ほかに、この話題の ID を指していないがマーカーを足すことを勧めるもの 1（「形に依存する要求」の節の後ろに書いた）
