# library-crates の対応表

元: `docs/ir/core/library-crates.md`（日本語は `library-crates.ja.md`）。書き直した文書: `library-crates.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-306 | C-core-libcrates-1, C-core-libcrates-2 | 用途ごとにパッケージを分け、利用者は用途でパッケージを選ぶ | そのまま。本文は TBL-core-040 の定義だけ。how_to_verify の「パッケージ一覧・通常依存グラフ」を C-1、「各公開入口の外部利用例」を C-2 にした。どちらも元が review なので読んで確認。ガイド `docs/guides/public-crate-api*.md` の「クレートを選ぶ」のマーカーはこの見出しへ |
| REQ-core-307 | C-core-libcrates-3, C-core-libcrates-4 | 計算のクレートは環境を操作せず、Tokio と CLI の依存を持たない | 読み替え。環境を操作しないことを C-3、依存を持たないことを C-4 にした。標準出力と標準エラーへ書かないことは落とした細部。core のモジュールの並びは「形に依存する要求」 |
| REQ-core-308 | C-core-libcrates-5, C-core-libcrates-6 | テストとの対応の判定は見つけたテストとマーカーを受け取り、ソースを解析しない | 読み替え。判定がソースの解析を呼ばないことを C-5、Analyzer が文字列から見つけることを C-6 にした。ガイド `docs/guides/public-crate-api*.md` の「クレートを選ぶ」のマーカーの REQ-core-308 はこの見出しへ |
| REQ-core-309 | C-core-libcrates-7, C-core-libcrates-8, C-core-libcrates-9, C-core-libcrates-10 | 公開にするかは外の利用者が頼るかで決め、kotowari は入口から届く下位の型だけを同じ型のまま再公開する | そのまま。1段落目の内部の型を C-7（別の決定「公開の署名に内部の実装の型を出さない」）、2段落目の公開の基準を C-8、再公開を C-9 と C-10 にした。ガイド `docs/guides/package-validation*.md` の「ローカルの検証」のマーカーの REQ-core-309 はこの見出しへ（パッケージの検証で再公開した型の同一性を確かめる段落なので） |
| TBL-core-040 | C-core-libcrates-1 | 用途ごとにパッケージを分け、利用者は用途でパッケージを選ぶ | 読み替え。表は「決めたこと」の表（パッケージと使う目的）と、依存の向きの箇条にした。ガイドの2か所（`public-crate-api*.md` の「クレートを選ぶ」、`package-validation*.md` の「将来の公開順」）のマーカーはこの見出しへ |
| EX-core-480 | C-core-libcrates-4, C-core-libcrates-7 | 計算のクレートは環境を操作せず、Tokio と CLI の依存を持たない | そのまま。ast-grep、Tokio、CLI と HTTP の依存が要らないことが C-4、内部の構文木の型を扱わずに済むことが C-7 |
| EX-core-481 | C-core-libcrates-5 | テストとの対応の判定は見つけたテストとマーカーを受け取り、ソースを解析しない | そのまま |
| EX-core-482 | C-core-libcrates-3, C-core-libcrates-7 | 計算のクレートは環境を操作せず、Tokio と CLI の依存を持たない | そのまま。「core が Git を実行する候補は通らない」が C-3、「公開署名が検索エンジンの内部型を返す候補は通らない」が C-7 |
| EX-core-558 | C-core-libcrates-8, C-core-libcrates-9 | 公開にするかは外の利用者が頼るかで決め、kotowari は入口から届く下位の型だけを同じ型のまま再公開する | そのまま。結果型を作る関数を公開する候補が C-8、届かない下位の型の再公開が C-9 |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-source-analysis/tests/analyzer.rs | source_analysis_retains_source_facts_and_diagnostics_without_acquisition | REQ-core-308, REQ-core-315, EX-core-481, EX-core-496, EX-core-497 | C-core-libcrates-6 | 存在しないパスの文字列から、テスト、マーカー、壊れたソースの指摘、問い合わせの無い言語の行のマーカーが返ることを確かめている。元の文字列とパスを保つことは REQ-core-315（「メモリ入力と読み込み済みの結果」の話題）の分で、その話題で付け替える |
| crates/kotowari-core/tests/inputs.rs | supplied_analysis_preserves_diagnostics_counts_and_duplicate_suppression | REQ-core-308, REQ-core-315, EX-core-481, EX-core-496, EX-core-497 | C-core-libcrates-5 | 渡した解析の結果（テストの情報と面）だけで検査を組み立て、ソースのファイルを読まずに結果を返すことを確かめている。指摘の重複の抑えと拡張子ごとの数は REQ-core-315 の分 |
| crates/kotowari-core/tests/inputs.rs | non_query_marker_facts_are_consumed_without_source_discovery | REQ-core-308, REQ-core-315, EX-core-481, EX-core-496 | C-core-libcrates-5 | 渡したマーカーの情報だけで、要求にテストがあると判定することを確かめている。REQ-core-315 の分はその話題で付け替える |
| crates/kotowari/tests/project.rs | facade_names_are_the_same_lower_result_types | REQ-core-322, EX-core-499 | C-core-libcrates-10 | この話題の ID は指していない（今のマーカーは library-paths の話題）。kotowari の再公開の名前（QueryItem、RequirementItem、GuideTally、MutantCounts、GuideReader、IrDocument）を受け取り、下位のクレートの型として返す関数がコンパイルできることで、同じ型であることを確かめている。library-paths の表でも同じ付け替え先にした |

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-core-040: kotowari-core から kotowari-markdown-schema への依存、kotowari-source-analysis から kotowari-core への依存、kotowari-mds から kotowari-markdown-schema への直接の依存 | 設計担当（2026-10-03-public-crate-api.md の A35。D1 の配置の委任）。決めたことの依存の向きには、利用者の決定（A11、A17、A33、2026-10-04-overview-on-public-api.md の A4）が裏付ける依存だけを書いた |
| TBL-core-040 と A34: CLI をルートの kotowari-cli と crates/kotowari-mds に置き、ほかのライブラリを同じ名前で crates/ の下に置くこと | 設計担当（A34。D1 の命名・配置の委任）。パッケージの名前は利用者から見えるので残した |
| REQ-core-307: 標準出力と標準エラーへ書かない | A35（設計担当）の「表示を置かない」。利用者の A7（表示は呼び出し側が決める）はこの要求の出典に引かれていない。schema の「スキーマと文書を読み込むライブラリ」の書き直しでも同じ扱いで落としている |
| REQ-core-307: core の中に、設定の読み込み、ファイルの探索、Git の取得、表示を置かない、というモジュールの並び | A35（設計担当）。決めたことには、利用者の A25 の「責務ごとのモジュールに分ける」だけを書いた |
| REQ-core-308: Analyzer が「既存の組込み規則と、文字列で渡す追加の規則」を読むこと、規則と未対応言語の既存の意味を保つこと | 設計担当（A41）。既存の意味を保つこと自体は利用者の A9 の向きと同じだが、A9 はこの要求の出典に無い |
| REQ-core-309: 失敗の型、指摘と結果の欄を読み取り専用にすること、non_exhaustive | 設計担当（A40）。要求の文には出ていない。プロジェクトを使う操作の結果と失敗は「プロジェクトを使う Rust API」の話題 |
| REQ-core-309 の出典の next-release-bundle の A8: 公開の基準を当てた範囲（保留にしていた候補と、同じ場所で同じ理由が当たるものまで。terms の関数4つ、fence の項目3つ、GuideReader の再公開） | 利用者（推奨を採用）だが、1回のリリースの作業の範囲で、今の製品の振る舞いの決定ではない。具体の項目はコードと変更履歴にある |
| REQ-core-309 の出典の next-release-bundle の A12: 再公開をやめた6つの型の名前（CheckInputs、ReadInputs、IrOptions、SourceText、TestAnalysis、SurfaceAnalysis） | 利用者（仮決めの推奨をそのまま送った）。決めたことには「メモリ上で検査する利用者は、そのための入力型を kotowari-core から取る」と書き、型の名前は落とした |
| 2026-10-24-next-release-bundle-consistency.md の A2（ParsedItem の再公開をやめる）と、-consistency-2.md の A1（kotowari::IrDocument の再公開先を kotowari_core::ir::IrDocument にする） | 整合のフェーズ（LLM）。REQ-core-309 を当てた結果の細部 |

## 形に依存する要求

| 元の ID | どうなるか |
|---|---|
| REQ-core-307 の一部（core のモジュールの並び「IR 解析・出典・テスト対応・変異結果・計画書・読み取り結果」） | 新しい形で読み替えて残る。並びは書かず、「責務ごとのモジュールに分ける」だけを書いた。「出典」の検査は新しい形で消え、「IR 解析」は決定の文書の解析に置き換わる |
| REQ-core-308、EX-core-481 の「IR と発見済みのテスト・印」 | 新しい形で読み替えて残る。テストとの対応の判定の相手が、要求と場面から確認することの行に変わる。決めたことには形に依存しない「テストとの対応の判定」とだけ書いた |
| TBL-core-040 の kotowari-core の目的「メモリ上の IR 解析・検査・結果の組立て」 | 新しい形で読み替えて残る。「メモリ上の仕様の解析と検査」と書いた |
| 元の冒頭の段落「公開作業と版管理の手順は判断の記録に置く」 | 消える。判断の記録を持たなくなる。版の系列（A15、利用者）はこの話題の要求に無いので書かなかった |

## テストの無い行

無い。前の下書きで挙げていた C-core-libcrates-10 は、テスト facade_names_are_the_same_lower_result_types を付け替え先にした（上のテストごとの表）。

## 誰が決めたか見分けられなかったもの

- kotowari が kotowari-core と kotowari-source-analysis に依存すること。具体の依存の辺は A35（設計担当）だが、kotowari がすべての操作を提供し（A10、利用者）、計算を2つのクレートに分ける（A25、利用者）ことから、kotowari がその2つの上に載ることは導ける。決めたことに書いた。
- パッケージの名前のうち、kotowari、kotowari-cli（A14）、kotowari-core、kotowari-source-analysis（A25）、kotowari-overview、kotowari-markdown-view（overview の A4）は利用者の決定に名前が出てくる。kotowari-markdown-schema-io と kotowari-mds の名前は A34（設計担当）。どれも利用者から見える名前なので残した。
- 「同じ型のまま再公開する」は A36（設計担当）の文。利用者の決定では、overview の A2 が「kotowari の側で core の結果を包む別の結果型を作る」案を、この決まりとぶつかるとして退けていて、next-release-bundle の A12 も A36 の「入口で使う型に限る」を理由にしている。利用者がこの決まりを前提に決めているので、決めたことに書いた。

## 理由や見送った案が記録に無かった決定

- 「テストとの対応の判定は見つけたテストとマーカーを受け取り、ソースを解析しない」: 見送った案が記録に無い。
- 「公開の署名に内部の実装の型を出さない」: 見送った案が記録に無い。
- 「計算のクレートは環境を操作せず…」の見送った案は、overview の A1 の rejected（A49 のまま kotowari-overview に読み書きと HTTP サーバまで置く）で、採らなかった理由は A1 の why から書いた。

## 分けた所、まとめた所

- REQ-core-309 を2つの決定に分けた: 「公開の署名に内部の実装の型を出さない」（A13、A16。2026-10-03 の記録）と「公開にするかは外の利用者が頼るかで決め…」（next-release-bundle の A6、A12 と、A36）。別の記録で、理由と見送った案が別だから。
- REQ-core-306 と TBL-core-040 は1つの決定にまとめた。REQ-core-306 の本文は表の定義だけである。
- REQ-core-307 の「Tokio と CLI 用の依存を持たない」と、EX-core-480 の「ast-grep を必要としない」は、C-4 の1行にまとめた。
- 2つの場面（EX-core-482、EX-core-558）は「境界の検証に通らない候補」の形なので、それぞれの決定の確認することの行に分けて当てた。

## あとでリンクにする所

- 無い。前の下書きで言葉で指していた「プロジェクトを使う Rust API」と「メモリ入力と読み込み済みの結果」は、このフォルダに下書きがあるので、冒頭の段落から `library-api.ja.md`、`library-inputs.ja.md` へのリンクにした。schema へのリンクは `../schema/library.ja.md#…` にした。

## 迷った判断と、どう決めたか

- 依存の向きの表（TBL-core-040）の辺は、A35（設計担当）が具体にしたものと、利用者が決めたもの（A11、A17、A33、overview の A4）が混ざっている。表の「使う目的」の列は名前と目的なので残し、依存は利用者の決定が裏付ける辺だけを箇条にした。表の3列目をそのまま残すと、設計担当の辺まで承認したことになるため。形の決まりで足りなかった所（表の一部の列だけが agent の細部のとき、どう書くか）。
- REQ-core-306、307、309 は元の確認の方法が review で、それを指すテストが無い。確認することの行はほとんど「（読んで確認）」になった。C-10 だけは既存のテスト（facade_names_are_the_same_lower_result_types）で確かめられるので、テストで確かめる行にし、そのテストを付け替え先にした。
- 「計算のクレートは環境を操作せず…」の決定では、kotowari-markdown-schema と kotowari-markdown-view も純粋な計算のクレートだが、元の要求が名指すのは core、source-analysis、overview の3つだけなので、その3つに限った。前の2つは schema と view の話題で決めている。
- 「Git」を残した。changes を外した（changes-rethink の A10）あとも、利用者の A11 は Git の操作を計算から分けるとしていて、REQ-core-307 も Git を挙げている。

- facade_names_are_the_same_lower_result_types は、kotowari::IrDocument の再公開も確かめている。IR を読む公開の入口を決定の文書の入口に置き換える（library-inputs の提案の決定）ときに、このテストのその1行も直すことになる。

## 数

- 元: 要求 4、場面 4、決定表 1、性質 0
- 読んだ記録のファイル: 6（2026-10-03-public-crate-api.md、2026-10-04-overview-on-public-api.md、2026-10-06-changes-rethink.md、2026-10-23-next-release-bundle.md、2026-10-24-next-release-bundle-consistency.md、2026-10-24-next-release-bundle-consistency-2.md）
- 出来上がり: 決定 5、確認することの行 10（うち「（読んで確認）」8）
- テスト: 4（facade_names_are_the_same_lower_result_types を足した）
- 戻した細部: 0（安全の一覧にこの話題の行は無い）
