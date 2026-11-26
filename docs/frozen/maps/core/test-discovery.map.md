# test-discovery の対応表

元: `docs/ir/core/test-discovery.md`（日本語は `test-discovery.ja.md`）。書き直した文書: `test-discovery.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-079 | C-core-discover-1, C-core-discover-2, C-core-discover-3, C-core-discover-14, C-core-discover-15, C-core-discover-16, C-core-discover-17 | テストのファイルは tests.files の glob で選ぶ | そのまま。glob で選ぶことを 1、ファイルのシンボリックリンクを読むことを 2、ディレクトリのシンボリックリンクを辿らないことを 3 にした。先の無いシンボリックリンクでの停止（14）、走査全体での停止（15）、特殊なファイルを読まないこと（16）、読めないディレクトリでの停止（17）は、安全に関わる細部として決定に戻した |
| REQ-core-080 | C-core-discover-4, C-core-discover-6 | テストは構文木に問い合わせを当てて見つけ、行の正規表現では探さない | 構文木で見つけることを 4、問い合わせの無い言語は構文木を読まないことを 6（別の決定）にした |
| REQ-core-081 | C-core-discover-7 | 拡張子と言語の対応は ast-grep と同じにし、大文字小文字を区別する | 定義は TBL-core-031 で、決定表として同じ決定に入れた |
| REQ-core-083 | C-core-discover-8 | 構文の誤りがあるテストのファイルは、unparsable_file を出して飛ばす | マクロの中身の読み直しの誤りの扱いは落とした |
| REQ-core-084 | C-core-discover-5 | テストは構文木に問い合わせを当てて見つけ、行の正規表現では探さない | 確認の方法が review だったので「（読んで確認）」にした |
| REQ-core-180 | C-core-discover-9, C-core-discover-10, C-core-discover-11 | テストの名前は、問い合わせが捕まえた文字から取る | 引用符を 9、null を 11 にした。describe の名前を付けないこと（元は TBL-core-032 の行と記録の A25）を 10 として同じ決定に入れた |
| REQ-core-181 | C-core-discover-12, C-core-discover-13 | 同じ節に重なって当たっても1つと数え、テストの中のテストは別に数える | 重なりを 12、入れ子を 13 にした。重なったときの名前の選び方は落とした |
| TBL-core-031 | C-core-discover-7 | 拡張子と言語の対応は ast-grep と同じにし、大文字小文字を区別する | 表は決めたことの中にそのまま写した |
| EX-core-293 | C-core-discover-6 | 問い合わせが1つも無い言語のファイルは構文木を読まない | そのまま |
| EX-core-294 | C-core-discover-8 | 構文の誤りがあるテストのファイルは、unparsable_file を出して飛ばす | そのまま |
| EX-core-295 | C-core-discover-7 | 拡張子と言語の対応は ast-grep と同じにし、大文字小文字を区別する | 値違いの場面として C-7 にまとめた |
| EX-core-296 | C-core-discover-9, C-core-discover-10 | テストの名前は、問い合わせが捕まえた文字から取る | 引用符を外すことと、describe の名前が付かないことの両方を確かめる場面 |
| EX-core-297 | C-core-discover-13 | 同じ節に重なって当たっても1つと数え、テストの中のテストは別に数える | そのまま |
| EX-core-328 | - | 同じ節に重なって当たっても1つと数え、テストの中のテストは別に数える | 消える（落とした細部）。名前の選び方は記録の review10-gaps A2 で、細部を任せた承認の範囲で決めたもの |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| tests/step4_test_discovery.rs | req_079_reads_files_matching_the_globs | REQ-core-079 | C-core-discover-1 | glob に当たるファイルの印が要求を満たす |
| tests/step4_test_discovery.rs | req_080_uses_tree_sitter_with_bundled_rust_query | REQ-core-080 | C-core-discover-4 | そのまま |
| tests/step4_test_discovery.rs | req_081_extension_decides_the_language_case_sensitively | REQ-core-081, TBL-core-031 | C-core-discover-7 | ".go" は言語が決まっても問い合わせが無いことも見ていて、C-6 の前提でもあるが、構文木を読まないことは見ていないので C-7 だけ |
| tests/step4_test_discovery.rs | ex_core_293_file_of_a_language_without_a_query_is_not_parsed | REQ-core-080, EX-core-293 | C-core-discover-6 | そのまま |
| tests/step4_test_discovery.rs | ex_core_295_upper_case_extension_decides_no_language | REQ-core-081, EX-core-295 | C-core-discover-7 | そのまま |
| tests/step4_test_discovery.rs | req_083_unparsable_file_is_skipped | REQ-core-083 | C-core-discover-8 | そのまま |
| tests/step4_test_discovery.rs | req_079_file_symlink_is_read | REQ-core-079 | C-core-discover-2, C-core-discover-3 | ファイルのシンボリックリンクの先を読むこと（C-2）に加え、後半で "tests/linked_dir" のディレクトリのシンボリックリンクの先のテストに test_without_id が出ないこと（C-3）も確かめている（下書きでは C-3 を「テストの無い行」にしていたが、確定の段でテストを読み直して直した） |
| tests/step4_test_discovery.rs | req_081_uppercase_extension_has_no_query | REQ-core-081 | C-core-discover-7 | ".RS" の場合 |
| tests/step4_test_discovery.rs | req_079_broken_symlink_in_tests_stops | REQ-core-079, REQ-core-018 | C-core-discover-14 | この話題の分だけ。"tests/" の下の先の無いシンボリックリンクで、check が終了コード2、標準出力が空。戻した細部。REQ-core-018 の分は停止の話題の新しい ID を指す |
| tests/step4_test_discovery.rs | req_079_broken_symlink_outside_glob_stops | REQ-core-079, REQ-core-018 | C-core-discover-15 | この話題の分だけ。既定の glob に当たらない基準のディレクトリ直下の先の無いシンボリックリンクで、check が終了コード2、標準出力が空。戻した細部。REQ-core-018 の分は停止の話題の新しい ID を指す |
| tests/step13_test_languages.rs | ex_core_294_syntax_error_in_a_file_of_a_language_with_a_query | REQ-core-083, EX-core-294 | C-core-discover-8 | そのまま |
| tests/step13_test_languages.rs | ex_core_296_string_name_loses_its_quotes | REQ-core-180, EX-core-296 | C-core-discover-9, C-core-discover-10 | 名前が "does x" で、describe の名前が付かないことを確かめる。マーカーの行の確かめはマーカーの話題の分 |
| tests/step13_test_languages.rs | req_180_only_one_pair_of_the_same_quotes_is_stripped | REQ-core-180, TBL-core-032 | C-core-discover-9 | この話題の分だけ。TBL-core-032 の分（名前は最初の引数）は test-queries の対応表に書いた |
| tests/step13_test_languages.rs | ex_core_297_test_inside_a_test_is_counted_apart | REQ-core-181, EX-core-297 | C-core-discover-13 | そのまま |
| tests/step13_test_languages.rs | ex_core_312_rule_hitting_the_bundled_node_counts_once | REQ-core-121, REQ-core-181, EX-core-312 | C-core-discover-12 | この話題の分だけ。REQ-core-121 の分は query-rules の対応表に書いた |
| tests/step13_test_languages.rs | req_180_test_without_a_name_is_listed_with_a_null_name | REQ-core-180, TBL-core-026 | C-core-discover-11 | この話題の分だけ。TBL-core-026 は list の話題 |
| tests/step13_test_languages.rs | ex_core_328_name_comes_from_the_first_rule_that_captures_it | REQ-core-181, EX-core-328 | C-core-discover-12 | 落とした細部のテスト。いちばん近い行を指す（重なったときの名前の選び方を確かめる。同梱と設定の問い合わせが同じ節に当たる場面なので、重なりを1つと数える C-12 がいちばん近い） |
| tests/step13_test_languages.rs | req_181_rule_with_files_keeps_its_place_in_the_order | REQ-core-181, REQ-core-187 | C-core-discover-12 | この話題の分（名前の選び方）は落とした細部のテスト。いちばん近い行を指す（C-12）。REQ-core-187 の分は query-rules の対応表に書いた |
| tests/step13_test_languages.rs | tbl_021_extensions_of_the_bundled_languages_have_a_query | TBL-core-021, TBL-core-031, REQ-core-182 | C-core-discover-7 | この話題の分（".py3"、".bzl" などが表の言語になる）だけ。REQ-core-182 の分は test-queries の対応表に書いた。TBL-core-021 は出力の話題 |
| crates/kotowari-source-analysis/tests/discovery.rs | req_181_test_inside_a_test_is_counted_apart | REQ-core-181 | C-core-discover-13 | 内側のテストへのマーカーの結び付けも見ているが、それはマーカーの話題 |
| crates/kotowari-source-analysis/tests/discovery.rs | req_181_one_node_hit_by_two_queries_is_one_test | REQ-core-181, REQ-core-180 | C-core-discover-12 | REQ-core-180 の分は、名前が関数の名前になることを見るだけで、引用符や null は見ないので付けない |

## マーカーに足すテスト

この話題の ID を今は指していないが、戻した細部を確かめているテスト。今のマーカー（別の話題の ID）は残したまま、この話題の ID を足す。

| テストのファイル | テストの関数名 | 今のマーカーの ID | 足す ID | 理由 |
|---|---|---|---|---|
| tests/step4_test_discovery.rs | req_018_unreadable_directory_under_tests_stops | REQ-core-018, TBL-core-001 | C-core-discover-17 | "tests/sub" の読み取りの権限を外し、check が終了コード2で止まり、標準出力が空であることを確かめている。REQ-core-018 と TBL-core-001 は停止の話題 |
| tests/step6_output.rs | tbl_021_excluded_entries_are_not_counted | TBL-core-021 | C-core-discover-16 | "tests.files" の glob（"lib/**/*.rs"）に当たるソケット "lib/socket.rs" を置き、check の "tests" の "rs" の "files" が1のままであることを確かめている。TBL-core-021 は出力の話題 |

## 落とした細部

### 安全に関わるもの

無し（あった3件は、下の「戻した細部」のとおり決定に戻した）。

### 戻した細部

| 細部 | 新しい ID | 確かめているもの |
|---|---|---|
| REQ-core-079: 先の無いシンボリックリンクでは、読めないファイルを理由に停止する（records.md の A146、実験者の裁定） | C-core-discover-14 | テスト tests/step4_test_discovery.rs の req_079_broken_symlink_in_tests_stops（終了コード2と、標準出力が空であることを確かめる。停止の理由の文言は確かめていないので、行には終了コードと標準出力だけを書いた） |
| REQ-core-079: 走査は基準のディレクトリ全体（隠しディレクトリを除く）を歩いてから glob で選ぶので、glob に当たらない場所の先の無いシンボリックリンクでも停止する（records.md の A159、実験者の裁定） | C-core-discover-15 | テスト tests/step4_test_discovery.rs の req_079_broken_symlink_outside_glob_stops |
| REQ-core-079: ディレクトリでも通常のファイルでもないもの（ソケット、名前付きパイプ、デバイス）は、glob に当たっても読まない（records.md の A165、実験者の裁定） | C-core-discover-16 | テスト tests/step6_output.rs の tbl_021_excluded_entries_are_not_counted（ソケットだけを確かめる。名前付きパイプとデバイスのテストは無いので、行はソケットに限った） |
| REQ-core-079: 走査で読めないディレクトリに当たると、読めないファイルを理由に停止する（records.md の A96、A159、実験者の裁定） | C-core-discover-17 | テスト tests/step4_test_discovery.rs の req_018_unreadable_directory_under_tests_stops（"tests.files" の glob の下のディレクトリだけを確かめる。glob に当たらない場所の読めないディレクトリのテストは無いので、行は glob の下に限った） |

4つとも決定「テストのファイルは tests.files の glob で選ぶ」に戻した。問題の節には、黙って読み飛ばさない方針（停止の話題の決定へのリンク）と、A165 の why（読むと固まりうる）を書いた。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-core-083: "tests.rust.macros" のマクロの中身を読み直したときの構文の誤りは unparsable_file に含めず、読めた最上位の関数だけを数える | 実験者の裁定（records.md の A149） |
| REQ-core-181: 同じ節に当たった問い合わせの名前は、"$NAME" を捕まえたもののうち、同梱、"tests.rules" の並びの順で最初のものにする | review10-gaps の A2。decided_by は「利用者（推奨を採用）」だが、その記録の Context に「利用者が『ほぼ答えが決まっているものは推奨で決めてよい』とした範囲で決める」とあり、細部を任せた承認だけなので agent の細部とした |

## 形に依存する要求

- REQ-core-081（記録の multi-language-tests A47 の「IR の表に写す」）: 新しい形で読み替えて残る。IR の決定表ではなく、決定の文書の「決めたこと」の中の表として持つ。

## テストの無い行

- 無し。下書きでは C-core-discover-3 を挙げていたが、req_079_file_symlink_is_read の後半が確かめていた。
- 戻した細部の行は、テストが確かめる範囲に限った。広げるなら次のテストが要る: glob に当たらない場所の読めないディレクトリで check が止まるテスト（C-17 を「どの glob にも当たらない場所でも」に広げる）、名前付きパイプを glob に当てても読まないテスト（C-16 を広げる）、先の無いシンボリックリンクと読めないディレクトリで標準エラーの1行目が "unreadable file: " で始まるテスト（停止の理由を行に書く）。

## 誰が決めたか見分けられなかったもの

- records.md の第6〜第9ラウンドの決定（A39、A47、A49、A58、A102、A120〜A123）には decided_by の行が無く、ラウンドの見出しにある利用者の答え（「推奨で」「細部の調整といったところで、推奨で問題無いと思う」「進めて」）だけが手がかりになる。どれも利用者が推奨の一つ一つを見て答えたものなので、利用者の決定として扱った。A74 で裁定を実験者に任せたあとの「実験者の裁定」（A75〜A97、A145〜A165）は agent の細部とした。

## 理由や見送った案が記録に無かった決定

- 「テストのファイルは tests.files の glob で選ぶ」: シンボリックリンクの扱い（A102）の理由が無い。決めたことの中に「理由は残っていない」と書いた。見送った案の節は置かなかった。
- 「問い合わせが1つも無い言語のファイルは構文木を読まない」: 見送った案が無い。
- 「拡張子と言語の対応は ast-grep と同じにし、大文字小文字を区別する」: 大文字小文字を区別する理由（A123。「方針として書く」だけ）が無い。
- 「構文の誤りがあるテストのファイルは、unparsable_file を出して飛ばす」: 止めずに飛ばす理由（A58、A120）が無い。問題の節に「理由は残っていない」と書いた。見送った案が無い。
- 「テストの名前は、問い合わせが捕まえた文字から取る」: 見送った案が無い。
- 「同じ節に重なって当たっても1つと数え、テストの中のテストは別に数える」: 見送った案が無い。

## 分けた所、まとめた所

- REQ-core-080 を2つの決定に分けた: 構文木で見つけること（REQ-core-084 と合わせた）と、問い合わせの無い言語は構文木を読まないこと（記録の A7 が理由を持つため）。
- REQ-core-084 は、同じ理由（正規表現は誤検出する）を持つので、REQ-core-080 と同じ決定にまとめた。
- REQ-core-081 と TBL-core-031 を1つの決定にした。
- REQ-core-180 の決定に、TBL-core-032 の「describe の名前を付けない」（記録の A25）を移した。名前の決まりなので名前の決定に置き、test-queries からは見出しへのリンクで指す。
- 値違いの場面（EX-core-295 と、".RS"、".Rs" のテスト）は C-7 の1行にまとめた。

## あとでリンクにする所

確定の段で、次を見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- テストのマーカーの下書き（test-markers）: 冒頭の段落（「マーカーは、テストの直前のコメントの塊にあるものだけをそのテストに結び付ける」）、「問い合わせが1つも無い言語のファイルは構文木を読まない」（「問い合わせの無い言語のファイルでは、ファイルの中のマーカーをすべて拾う」）
- 設定の下書き（config）: 「テストのファイルは tests.files の glob で選ぶ」の既定の glob（「設定のキーは表のものだけにし、書かなかったキーは表の既定の値にする」）
- list の下書き: 「テストの名前は、問い合わせが捕まえた文字から取る」の、list の "name" が null を取れること（「確認することの行にはそれを指すテストを、決定には指紋を付ける」）
- 停止と黙って見逃さない方針の話題（cli-environment）: 確定の段で、戻した細部の問題の節から見出しへのリンクにした

## 迷った判断と、どう決めたか

- review10-gaps と review11-gaps の決定は decided_by が「利用者（推奨を採用）」だが、記録の Context が、利用者の「ほぼ答えが決まっているものは推奨で決めてよい」という任せた承認の範囲で決めたと書いているので、agent の細部として落とした。逆に読むなら、C-core-discover-12 の決定に名前の選び方を足し、`ex_core_328_...` と `req_181_rule_with_files_...` を指せばよい。
- 安全に関わる REQ-core-079 の細部（特殊なファイル、先の無いシンボリックリンク、走査全体での停止）は、記録では実験者の裁定で、下書きでは落としたが、確定の段で「安全に関わる細部は、今のテストか skill の文が確かめているなら決定に戻す」という利用者の決定に従い、テストのあるものを戻した（C-14、C-15、C-16、C-17）。行の文は、テストが確かめる範囲（終了コードと標準出力、ソケット）に限った。読めないディレクトリでの停止は、停止の話題の ID だけを指すテスト req_018_unreadable_directory_under_tests_stops が確かめていたので、それも戻した（C-17。「マーカーに足すテスト」）。
- review10-gaps の決定（名前の選び方）は落としたままにした。そのテスト2本（ex_core_328_…、req_181_rule_with_files_…）は、マーカーが空にならないよう、いちばん近い C-12 を指す。
- 「など」を直した: 名前の引用符の決めたことの「Python の三重引用符や "f" の接頭辞の付いた文字列など」を「例:」の書き方に、「区切り文字などの決めごと」を「つなぐ区切り文字の決めごと」に、C-9 の「片方だけの引用符や式など」を「例:」の書き方にした。
- ast-grep、ast-grep-language、tree-sitter、Pest、"ast-grep scan" の名前は、使う人が書くルールの形（記録の A10。利用者の決定）と、元の IR の文に出てくる振る舞いの一部なので残した。記録の why と Context にだけ出てくる名前（外部のレビューの名前、テストのライブラリの列挙、ビルドの大きさの測定の詳細）は書かなかった。
- 「文法を絞る」案は、記録の A6 の why が、5言語、16個、全27個の大きさを比べて全部を選んでいるので、見送った案として書いた。
- JSON 出力の "tests" の "query" の鍵（記録の A24）は出力の話題に属するので書かず、テストがそれで確かめていることだけを使った。
- 「テスト」「問い合わせ」「問い合わせのある言語」の語の意味は、用語集のとおりに文の中で使った。

## 数

- 元: 要求 7、場面 6、決定表 1、性質 0
- 読んだ記録のファイル: 4（records.md、2026-09-24-multi-language-tests.md、2026-09-24-review10-gaps.md、2026-09-17-check-reach.md）
- 出来上がり: 決定 7、確認することの行 17（うち「（読んで確認）」1、テストの無い行 0）
- テスト: 21（この話題の ID を指すもの）。ほかにマーカーに足すもの 2
