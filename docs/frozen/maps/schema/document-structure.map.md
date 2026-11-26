# document-structure の対応表

元: `docs/ir/schema/document-structure.md`（日本語は `document-structure.ja.md`）。書き直した文書: `document-structure.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-022 | C-schema-doc-3, C-schema-doc-4, C-schema-doc-5, C-schema-doc-6, C-schema-doc-7 | スキーマが題名を宣言したときだけ、題名をちょうど1つ求める | 1つの要求に、出典の違う4つの決定が束ねられていたので分けた。題名を求める条件を C-3・C-4、2つ目以降の題名ごとの指摘を「2つ目以降の題名は、題名ごとに1件の指摘にする」の C-5、1つ目の照合を「題名が2つ以上あっても、1つ目の題名を正規表現と照合する」の C-6、後の題名が節と項目を終えることを「2つ目以降の題名は、それまでの節と項目を終える」の C-7 にした |
| REQ-schema-023 | C-schema-doc-8, C-schema-doc-9 | 前置部には、その内側のノードに規則と抽出を宣言する | 内側のノードに宣言できることを C-8、前置部そのものの抽出の停止を C-9 にした |
| REQ-schema-024 | C-schema-doc-10, C-schema-doc-11 | 節は、見出しの文字で見分ける | 名前で見分けることを C-10、打ち消し線の中の文字を C-11 にした |
| REQ-schema-025 | C-schema-doc-12 | 項目の見出しは、":" の前を ID、後を名前として読む | 定義を TBL-schema-006 に持つ要求。表を「決めたこと」に移した |
| REQ-schema-026 | C-schema-doc-1 | 見出しの骨格は、題名・節・項目の3段で閉じる | PROP-schema-005 と同じ決定（A5）なので1つにまとめた |
| REQ-schema-027 | C-schema-doc-13 | 項目を宣言していない節の中の、深さ3の見出しは指摘にする | そのまま |
| REQ-schema-061 | C-schema-doc-14, C-schema-doc-15, C-schema-doc-16, C-schema-doc-17 | スキーマは、節を挟まない文書の直下の項目を "document.item" に宣言できる | 宣言したときの読み方（A15、A23）を C-14、宣言しないときの扱い（A41、A45、todo-zero の A6）を別の決定「文書の直下の項目を宣言しないスキーマでは、…」の C-15〜C-17 に分けた |
| REQ-schema-056 | C-schema-doc-18 | 項目が足りないときは、出現回数を書いたかを問わず、下限を割った指摘にする | そのまま |
| REQ-schema-057 | C-schema-cli-13 | 出現回数の指摘に、数えたノードの規則種別を持たせる | cli の同じ決定（ir-engine の A86）に一本化した。決定の見出しは cli.md のもの。この文書からは冒頭の段落でリンクする。以前の C-schema-doc-19 は欠番 |
| TBL-schema-006 | C-schema-doc-12 | 項目の見出しは、":" の前を ID、後を名前として読む | 表を「決めたこと」に移し、順2に「先頭から ":" の直前までを ID、その後を名前」（A37）を足した |
| PROP-schema-005 | C-schema-doc-2 | 見出しの骨格は、題名・節・項目の3段で閉じる | スキーマの言葉に手段が無いことなので、読んで確かめる行にした |
| EX-schema-079 | C-schema-doc-11 | 節は、見出しの文字で見分ける | そのまま |
| EX-schema-080 | C-schema-doc-7 | 2つ目以降の題名は、それまでの節と項目を終える | そのまま |
| EX-schema-009 | C-schema-doc-12 | 項目の見出しは、":" の前を ID、後を名前として読む | コロンの無い見出しの場合（item_heading_without_colon_is_invalid_id）と1行にまとめた |
| EX-schema-072 | C-schema-doc-6 | 題名が2つ以上あっても、1つ目の題名を正規表現と照合する | そのまま |
| EX-schema-074 | C-schema-doc-4 | スキーマが題名を宣言したときだけ、題名をちょうど1つ求める | そのまま |
| EX-schema-075 | C-schema-doc-18 | 項目が足りないときは、出現回数を書いたかを問わず、下限を割った指摘にする | そのまま |
| EX-schema-010 | C-schema-doc-5 | 2つ目以降の題名は、題名ごとに1件の指摘にする | 題名が3つの場面（EX-schema-042）と1行にまとめた |
| EX-schema-042 | C-schema-doc-5 | 2つ目以降の題名は、題名ごとに1件の指摘にする | 同上 |
| EX-schema-043 | C-schema-doc-14 | スキーマは、節を挟まない文書の直下の項目を "document.item" に宣言できる | そのまま |
| EX-schema-044 | C-schema-doc-15 | 文書の直下の項目を宣言しないスキーマでは、最初の節より前の深さ3の見出しを、前置部を宣言したか閉じた世界のときだけ指摘にする | そのまま |
| EX-schema-053 | C-schema-doc-16 | 文書の直下の項目を宣言しないスキーマでは、最初の節より前の深さ3の見出しを、前置部を宣言したか閉じた世界のときだけ指摘にする | そのまま |
| EX-schema-094 | C-schema-doc-17 | 文書の直下の項目を宣言しないスキーマでは、最初の節より前の深さ3の見出しを、前置部を宣言したか閉じた世界のときだけ指摘にする | そのまま |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/src/schema/tests.rs | invalid_item_id_regex_is_an_error | REQ-schema-016, REQ-schema-025 | - | 項目の ID の正規表現が書き誤りのとき停止することを確かめる。それは正規表現の検査（REQ-schema-016、「スキーマ言語の骨格」の話題）で、この話題の見出しの読み方は確かめていない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | multiple_titles_is_found | REQ-schema-022, EX-schema-010 | C-schema-doc-5 | 題名が2つで multiple_titles が出る |
| crates/kotowari-markdown-schema/src/validate/tests.rs | title_pattern_mismatch_is_found | REQ-schema-022 | C-schema-doc-6 | 題名が正規表現に合わないと title_pattern_mismatch が出る |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_heading_is_found | REQ-schema-001, REQ-schema-024 | C-schema-doc-10 | 宣言していない名前の節が宣言していない見出しの指摘になる。REQ-schema-001 の分は「閉じた世界と開いた世界」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_section_is_found | REQ-schema-004, REQ-schema-024 | C-schema-doc-10 | 名前で見分けて、宣言した節が無いとき欠落の指摘が出る。必須の欠落（REQ-schema-004）の分は「閉じた世界と開いた世界」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | heading_level_mismatch_is_found | REQ-schema-026 | C-schema-doc-1 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | invalid_item_id_is_found | REQ-schema-025, EX-schema-009 | C-schema-doc-12 | ID が正規表現に合わない見出しが invalid_id になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | title_without_rule_is_undeclared_heading_in_closed_world | REQ-schema-001, REQ-schema-022 | C-schema-doc-4 | 題名を宣言しないスキーマで、題名が宣言していない見出しになる。REQ-schema-001 の分は別の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_preamble_lines_are_flagged_in_closed_world | REQ-schema-001, REQ-schema-023 | - | 前置部を宣言しないスキーマで、前置部の行が閉じた世界の指摘になることを確かめる。前置部の内側のノードへの宣言は確かめていない。「閉じた世界と開いた世界」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_preamble_lines_are_allowed_when_open | REQ-schema-002, REQ-schema-023 | - | 同上。開いた世界で許すことだけを確かめる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_item_inside_declared_section_is_flagged_even_when_open | REQ-schema-003, REQ-schema-027 | C-schema-doc-13 | そのまま。REQ-schema-003 の分は別の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | stray_heading_in_declared_preamble_is_undeclared_even_when_open | REQ-schema-003, REQ-schema-023 | C-schema-doc-15 | 文書の直下の項目を宣言せず前置部を宣言したスキーマで、開いた世界でも前置部の深さ3の見出しが指摘になる。REQ-schema-003 の分は別の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | stray_heading_lines_in_declared_preamble_are_undeclared_even_when_open | REQ-schema-003, REQ-schema-023 | C-schema-doc-15 | 同上。その内側の行が指摘になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_item_is_reported_as_repeat_min_not_met | REQ-schema-056 | C-schema-doc-18 | 出現回数を書いた項目の欠落が repeat_min_not_met になり、missing_required_section は出ない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | item_heading_without_colon_is_invalid_id | REQ-schema-025, EX-schema-009 | C-schema-doc-12 | ":" の無い見出しが invalid_id になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_declared_on_the_preamble_is_not_undeclared | REQ-schema-023, REQ-schema-033 | C-schema-doc-8 | 前置部の内側に宣言した表が指摘にならない。REQ-schema-033 の分は見出しの下の行の規則の対応表（「-」） |
| crates/kotowari-markdown-schema/src/validate/tests.rs | code_block_declared_on_the_preamble_is_not_undeclared | REQ-schema-023, REQ-schema-034 | C-schema-doc-8 | 前置部の内側に宣言したコードブロックが指摘にならない。REQ-schema-034 の分は見出しの下の行の規則の対応表（「-」） |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_title_has_neither_a_line_nor_a_node_name | REQ-schema-022, REQ-schema-008 | C-schema-doc-3 | missing_title が1件だけ出て、行番号を持たない。ノードの名前と生の行を持たないこと（REQ-schema-008）の分は「CLI と結果の出し方」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | repeat_min_not_met_carries_the_counted_rule_kind | REQ-schema-057 | C-schema-cli-13 | 節の文の下限を割った repeat_min_not_met が規則種別として文を持つ。決定を cli に一本化したので、cli の C-13 を付け替え先にした |
| crates/kotowari-markdown-schema/src/extract/tests.rs | preamble_table_is_extracted_as_row_objects | REQ-schema-023, REQ-schema-033 | C-schema-doc-8 | 前置部の表に宣言した抽出が、ヘッダの名前を鍵にした行ごとの値を返す。REQ-schema-033 の分は見出しの下の行の規則の対応表（「-」） |
| crates/kotowari-mds/tests/cli.rs | check_missing_title_has_no_line_number | REQ-schema-008, REQ-schema-022 | C-schema-doc-3 | missing_title が行番号を持たずに出る。REQ-schema-008 の分は「CLI と結果の出し方」の話題 |
| crates/kotowari-mds/tests/cli.rs | ex_schema_053_open_world_without_a_declared_preamble_allows_a_level_three_heading_before_sections | EX-schema-053 | C-schema-doc-16 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_043_document_item_reads_items_before_the_first_section | EX-schema-043 | C-schema-doc-14 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_044_without_document_item_a_level_three_heading_before_sections_is_reported | EX-schema-044 | C-schema-doc-15 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_072_first_title_is_matched_even_when_there_are_more | EX-schema-072 | C-schema-doc-6 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_074_a_schema_without_a_title_rule_does_not_ask_for_a_title | EX-schema-074 | C-schema-doc-4 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_075_a_missing_item_without_repeat_is_below_its_minimum | EX-schema-075 | C-schema-doc-18 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_079_strikethrough_text_in_a_heading_is_part_of_its_name | REQ-schema-024, EX-schema-079 | C-schema-doc-11 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_080_a_later_title_ends_the_section_and_item_before_it | REQ-schema-022, REQ-schema-062, EX-schema-080 | C-schema-doc-7 | 後の題名の後の行が項目の文に入らない。要素の範囲（REQ-schema-062）の分は「抽出と素の構文木」の話題で付け替える |
| crates/kotowari-mds/tests/cli.rs | ex_schema_042_three_titles_give_two_multiple_titles_findings | EX-schema-042 | C-schema-doc-5 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_094_a_level_three_heading_in_a_document_without_a_title_is_outside_the_declared_preamble | EX-schema-094, REQ-schema-061 | C-schema-doc-17 | そのまま |

## 落とした細部

### 安全に関わるもの

なし。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-022 の出典 A17 の後半: kotowari が multiple_titles の detail を題名の文字（"# " を除き、前後の空白を除いたもの）にし、行を null にする | 利用者（ir-engine-gaps の A17、A40。推奨を採用）。エンジンでなく kotowari の写し分けの決定なので、この話題には書かなかった。core の指摘の写し方の話題に属する |
| REQ-schema-061 の出典 A31、A32: flags.yaml が直下の項目と節の下の項目を別の配置パスに抽出し、"end" を抽出する | 利用者（推奨を採用）。下の「形に依存する要求」 |
| REQ-schema-057: 規則種別を指摘に持たせる場所（JSON の rule_kind の鍵） | 記録に無い。ir-engine の A86 は「持たせる」までで、鍵の名前は無い。決定は cli に一本化した（cli の対応表の「落とした細部」にも、指摘の JSON の鍵の名前として rule_kind がある） |

## 形に依存する要求

元の要求・表・性質・場面の本文には、古い仕様の形そのものについての振る舞いは無い。出典の記録のうち、kotowari 自身の IR のスキーマについて決めた部分は消える。

| 元の ID | 扱い |
|---|---|
| REQ-schema-061（出典 ir-engine-gaps の A15 の後半、A31、A32） | 消える。「flags.yaml は節の下と文書の直下の両方を宣言する」「flags と flags_in_section に抽出する」「end を抽出する」は、問題の記録（FLAGS）の形が無くなるので書かなかった。エンジンが "document.item" を受けることは残した。問題の節の「kotowari の問題の記録がそうだった」は、決めた理由として残した経緯 |

## テストの無い行

- C-schema-doc-9（前置部そのものに抽出を書いたスキーマは停止）: 指すテストが無い。今あるのは、知らない鍵を停止にする一般のテスト（`crates/kotowari-markdown-schema/src/schema/tests.rs` の `unknown_key_is_an_error`、「スキーマ言語の骨格」の話題）だけ。前置部に `extract` を書いた YAML で `Schema::parse` が誤りを返す（"kotowari-mds check" の終了コードが 2 になる）ことを確かめるテストが要る。

## 誰が決めたか見分けられなかったもの

- REQ-schema-027 の「開いた世界でも同じである」: 出典の A2 と A5 には、開いた世界の扱いが書かれていない。宣言済みの節の中は開いた世界でも許さないことは、「閉じた世界と開いた世界」の話題の要求（REQ-schema-003）と、ir-engine-gaps の A45 の why にある（どちらも利用者）。決めたことに残し、確認することにも入れた。
- 「前置部には、その内側のノードに規則と抽出を宣言する」の、前置部に置けるもの（フィールド行、文、箇条書き、表、コードブロック）: mds-spec の A38（利用者が批准）。前置部そのものに抽出の鍵が無いこと（ir-engine の A68）は「利用者（訂正案を採用）」で、実装の実物に合わせた訂正だった。

## 理由や見送った案が記録に無かった決定

- 「項目の見出しは、":" の前を ID、後を名前として読む」: 理由も見送った案も記録に無い（A37 の why は not recorded）。問題の節に「理由は残っていない」と書いた。
- 「項目を宣言していない節の中の、深さ3の見出しは指摘にする」: この決定そのものの理由は記録に無い。閉じた世界の理由（A2）だけを書き、「閉じた世界の理由のほかは残っていない」と書いた。見送った案は無い。
- 「節は、見出しの文字で見分ける」: 見送った案が記録に無い。理由は A2 と review7-gaps の A2 から書いた。
- 「見出しの骨格は、題名・節・項目の3段で閉じる」: 見送った案が記録に無い（A5 は rejected を持たない）。
- 「2つ目以降の題名は、題名ごとに1件の指摘にする」: 見送った案が記録に無い。
- 「前置部には、その内側のノードに規則と抽出を宣言する」: 見送った案が記録に無い。
- 「スキーマは、節を挟まない文書の直下の項目を "document.item" に宣言できる」: 見送った案が記録に無い。
- 「文書の直下の項目を宣言しないスキーマでは、…」: 見送った案が記録に無い。
- 「項目が足りないときは、…」: 見送った案が記録に無い。

## 分けた所、まとめた所

- REQ-schema-022 を4つの決定に分けた。題名を求める条件（A5、review4-gaps の A1。見送った案あり）、2つ目以降の題名ごとの指摘（ir-engine-gaps の A7、A17）、1つ目の照合（review2-gaps の A1。見送った案あり）、後の題名が節と項目を終えること（review7-gaps の A3。見送った案あり）。
- REQ-schema-061 を2つの決定に分けた。"document.item" を宣言したときの読み方（A15、A23）と、宣言しないときの扱い（A41、A45、todo-zero の A6）。
- REQ-schema-026 と PROP-schema-005 を1つの決定「見出しの骨格は、題名・節・項目の3段で閉じる」にまとめた。どちらも A5。
- REQ-schema-025 と TBL-schema-006 を1つの決定にした。
- 値違いの場面を1行にまとめた: EX-schema-010 と EX-schema-042（C-5）、EX-schema-009 とコロンの無い見出し（C-12）。前置部を宣言した開いた世界の2本のテストと EX-schema-044 は C-15 の1行にした。

## あとでリンクにする所

無し。前の版で挙げたものは、どれも見出しへのリンクにした。

- 「閉じた世界と開いた世界」の話題: 冒頭の段落、「スキーマが題名を宣言したときだけ、…」の閉じた世界の宣言していない見出し、「文書の直下の項目を宣言しないスキーマでは、…」の問題の節の開いた世界の決まり、「項目を宣言していない節の中の、…」の開いた世界でも同じ
- 「スキーマ言語の骨格」の話題: 「項目が足りないときは、…」の既定の出現回数、「スキーマは、節を挟まない文書の直下の項目を…」の "document.item" を根のノードとして受けること
- 「抽出と素の構文木」の話題: 「2つ目以降の題名は、それまでの節と項目を終える」の問題の節の要素の範囲
- 「CLI と結果の出し方」の話題: 出現回数の指摘の規則種別（冒頭の段落）

## 迷った判断と、どう決めたか

- 「前置部には、…」の前置部そのものに抽出の鍵が無いこと（REQ-schema-023 の中身）は、それを確かめるテストが無い。この話題の ID を指す7本のテストは、前置部の内側のノードの宣言か、閉じた世界・開いた世界の扱いを確かめている。決めたことの本体なので落とさず、内側への宣言を C-8、停止を C-9 に分け、C-9 をテストの無い行に挙げた。
- stray_heading_in_declared_preamble_is_undeclared_even_when_open と stray_heading_lines_in_declared_preamble_are_undeclared_even_when_open は REQ-schema-023 のマーカーを持つが、確かめているのは「前置部を宣言したスキーマでは、開いた世界でも、最初の節より前の深さ3の見出しを指摘にする」（REQ-schema-061 の中身）なので、C-15 に付けた。
- 「スキーマは、節を挟まない文書の直下の項目を…」の問題に「kotowari の問題の記録がそうだった」と書いた。A15 の why は FLAGS.md の項目を読むためで、問題の記録の形は新しい版で無くなるが、なぜ決めたかの経緯として残した。決定そのものは汎用のエンジンの振る舞いで、形に依存しない。
- 「出現回数の指摘に、数えた規則種別を持たせる」（前の版の C-schema-doc-19）は、cli の「出現回数の指摘に、数えたノードの規則種別を持たせる」（C-schema-cli-13）と同じ決定（ir-engine の A86）だった。どちらの行にもテストがあり、「テストのある行の側に残す」では決まらないので、振る舞いが見える側として、指摘の形のほかの決定と JSON の rule_kind を確かめる CLI のテストがある cli に残した。この文書の決定は消し、冒頭の段落からリンクした。
- 「2つ目以降の題名は、題名ごとに1件の指摘にする」には、kotowari 側の detail と行の決定（A17 の後半、A40）を書かず、落とした細部に挙げた。指摘を写す kotowari の側の話題に属するため。
- REQ-schema-024 の「インラインの記法を除き」の範囲（強調、太字、リンク、コード）は、review7-gaps の A2 の why にあるので問題の節に書いた。
- 誰が決めたかの但し書き: 出典の decided_by はどれも利用者だが、次のものは利用者の個別の判断ではない。ir-engine-gaps の A45（C-schema-doc-15〜17 の開いた世界の扱い）は「不具合を直す指示。直す先は実装でなく仕様の文と主セッションが判断した」。review2-gaps、review4-gaps、review7-gaps の決定（C-schema-doc-3〜7、C-schema-doc-11、C-schema-doc-18）は、利用者が「ほぼ答えが決まっているものは推奨で決めてよい」とした範囲で、推奨を採用したもの。どれも中身が具体的な振る舞いで、細部の委任だけの承認ではないので、利用者の決定として残した。覆すなら利用者が判断する。
- 状態を変える前提で、テストで確かめる行には、C-schema-doc-9 を除いて、それを確かめる既存のテストがあることを、テストの関数名で確かめた（上の表）。
- 「決めたこと」と「確認すること」に「など」は無かった。

## mds のスキーマの言葉を広げる決定（提案）

出どころ: mds の形の追加の設計案（リポジトリには置いていない） の A1 と A4 と、利用者の答え3と5。元の ID は無い。確認することの番号は C-schema-doc-20 から（C-schema-doc-19 は欠番のまま使わない）。

### 置き換える決定

実装する側は、新しい決定を作ったときに、左の列の文と行を消すか直し、新しい決定の状態を「実装済み」にする。左の列の決定は、それまで実装済みのまま残す。

| 古い決定の見出しと、置き換わる部分 | 置き換える決定の見出しと行 | 理由 |
|---|---|---|
| 「見出しの骨格は、題名・節・項目の3段で閉じる」の決めたことの2段目（深さ4以上の見出しは指摘にする）と C-schema-doc-1 | 「深さ4以上の見出しは、それを含む前置部・節・項目の行として読み、"subheading" を宣言しなければ指摘にする」（C-schema-doc-20〜C-schema-doc-22） | A4。設計の REQ-schema-026 の変更。1段目と C-schema-doc-2（見出しのノードを宣言する手段が無い）は残る。設計も PROP-schema-005 は変えないとする |
| 「項目の見出しは、":" の前を ID、後を名前として読む」の全体（C-schema-doc-12） | 「名前も正規表現も持たない項目の規則が選んだ見出しだけを、":" の前の ID と後の名前に分けて読む」（C-schema-doc-23、C-schema-doc-24） | A1。REQ-schema-025 と TBL-schema-006 を ID の規則に限る。表は新しい決定に移した |
| 「スキーマは、節を挟まない文書の直下の項目を "document.item" に宣言できる」の全体（C-schema-doc-14） | 「スキーマは、節を挟まない文書の直下の項目を "document.items" に宣言できる」（C-schema-doc-25） | 答え3。REQ-schema-061 の変更。問題の節の経緯の文は新しい決定に移した |
| 「節は、見出しの文字で見分ける」の決めたことの1文目と C-schema-doc-10 | schema-language の「節と項目の規則は、見出しを名前の一覧か正規表現で選ぶ」（C-schema-lang-14〜C-schema-lang-17） | A1。schema-language の対応表にも同じ行がある |

テストの付け替え: 古い行 C-schema-doc-1、C-schema-doc-12、C-schema-doc-14 を指すテスト（上の「テストごとの表」の heading_level_mismatch_is_found、invalid_item_id_is_found、item_heading_without_colon_is_invalid_id、ex_schema_043_document_item_reads_items_before_the_first_section）は、それぞれ C-schema-doc-20、C-schema-doc-23、C-schema-doc-25 を指すように直す。テストの中のスキーマの "item" は "items" に書き換える。

### 「提案」で書いた決定の一覧

| 決定 | 変えることになりそうなコード（設計の §1 から） |
|---|---|
| 深さ4以上の見出しは、それを含む前置部・節・項目の行として読み、"subheading" を宣言しなければ指摘にする | `crates/kotowari-markdown-schema/src/document.rs` の深さ4以上の見出しを `stray_headings` に入れる所、`validate.rs` の深さ4以上の見出しの指摘、`line_reading.rs` の見出しの扱い |
| 名前も正規表現も持たない項目の規則が選んだ見出しだけを、":" の前の ID と後の名前に分けて読む | `validate.rs` の `validate_items` |
| スキーマは、節を挟まない文書の直下の項目を "document.items" に宣言できる | `schema.rs` の `Document.item`、`document.rs` の `preamble_items`、`extract.rs` の文書の直下の項目の抽出 |

### 実装する側が決める細部

- 深さ4以上の見出しの行を、節と項目の本文の値（抽出）に入れるかどうか。設計は「本文を描くときは新しいブロックを飛ばす」と書くが、抽出の値については決めていない。
- 箇条書きの中（一覧の項目の中）に書いた深さ4以上の見出しの持ち主。
- "subheading" の正規表現に合わないときの指摘の種類。

### 利用者に聞くこと

無い。

### 迷った判断

- 答え5（宣言していない深さ3の見出しの下の内容はブロックごとに指摘）は、この話題の「項目を宣言していない節の中の、深さ3の見出しは指摘にする」が今決めているとおりで、変えない。ID の規則の無い項目の一覧のときに同じ扱いになることは、schema-language の選び方の決定に書いた。
- C-schema-doc-2（スキーマの言葉に、項目の下に見出しのノードを宣言する手段が無い）は、"subheading" が下の行を持つノードでなく行を数える規則なので、成り立つと読んで残した。
- "document.items" の決定は、古い決定の中身を "items" に合わせて書き直したもので、振る舞いは変えていない。

## 数

- 足した提案: 決定 3、確認することの行 6（C-schema-doc-20〜C-schema-doc-25。うち「（読んで確認）」0）
- 元: 要求 9、場面 12、決定表 1、性質 1
- 読んだ記録のファイル: 7（2026-09-21-mds-spec.md、2026-09-22-ir-engine.md、2026-09-23-ir-engine-gaps.md、2026-09-24-review2-gaps.md、2026-09-24-review4-gaps.md、2026-09-24-review7-gaps.md、2026-10-06-todo-zero.md）
- 出来上がり: 決定 12、確認することの行 18（C-1〜C-18。C-19 は欠番。うち「（読んで確認）」1）
- テスト: 31
