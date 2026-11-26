# test-queries の対応表

元: `docs/ir/core/test-queries.md`（日本語は `test-queries.ja.md`）。書き直した文書: `test-queries.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-core-182 | C-core-queries-1, C-core-queries-2, C-core-queries-3 | 問い合わせを同梱するのは、利用者が使う4つの言語の系統だけにする | 同梱の言語を 1、Tsx を 2、JavaScript の拡張子を 3 にした |
| REQ-core-082 | C-core-queries-4, C-core-queries-5, C-core-queries-6, C-core-queries-7 | Rust では、属性のパスの末尾が test の関数を必ず数え、属性とマクロを設定で足す | 定義は TBL-core-017 |
| REQ-core-183 | C-core-queries-8, C-core-queries-9, C-core-queries-10 | TypeScript と JavaScript では、it と test の呼び出しを数える | 定義は TBL-core-032 |
| REQ-core-184 | C-core-queries-11, C-core-queries-12 | Python では、名前が test で始まる関数とメソッドを数える | 定義は TBL-core-033 |
| REQ-core-185 | C-core-queries-13, C-core-queries-14, C-core-queries-15 | Php では、メソッドと test の呼び出しを数える | 定義は TBL-core-034 |
| TBL-core-017 | C-core-queries-4, C-core-queries-5, C-core-queries-6, C-core-queries-7 | Rust では、属性のパスの末尾が test の関数を必ず数え、属性とマクロを設定で足す | 表は決めたことに写した。マクロの「最上位の関数」に "mod" と "impl" の中を含むことは落とした。マクロの中のマーカーの結び付けと invalid_marker はマーカーの話題に移した |
| TBL-core-032 | C-core-queries-8, C-core-queries-9, C-core-queries-10 | TypeScript と JavaScript では、it と test の呼び出しを数える | タグ付きテンプレートの行は落とした。describe の名前を付けないことは test-discovery の名前の決定に移した |
| TBL-core-033 | C-core-queries-11, C-core-queries-12 | Python では、名前が test で始まる関数とメソッドを数える | 関数の中のクラスのメソッドも数えることと、デコレータが節の外にあることは落とした（後者は指摘の行とマーカーの話題） |
| TBL-core-034 | C-core-queries-13, C-core-queries-14, C-core-queries-15 | Php では、メソッドと test の呼び出しを数える | 属性が節の中にあることは、指摘の行の話題に属するので書かなかった |
| EX-core-017 | C-core-queries-5 | Rust では、属性のパスの末尾が test の関数を必ず数え、属性とマクロを設定で足す | そのまま |
| EX-core-018 | C-core-queries-7 | Rust では、属性のパスの末尾が test の関数を必ず数え、属性とマクロを設定で足す | そのまま |
| EX-core-298 | C-core-queries-8 | TypeScript と JavaScript では、it と test の呼び出しを数える | そのまま |
| EX-core-299 | C-core-queries-9 | TypeScript と JavaScript では、it と test の呼び出しを数える | そのまま |
| EX-core-300 | C-core-queries-10 | TypeScript と JavaScript では、it と test の呼び出しを数える | そのまま |
| EX-core-301 | C-core-queries-2 | 問い合わせを同梱するのは、利用者が使う4つの言語の系統だけにする | そのまま |
| EX-core-302 | C-core-queries-11 | Python では、名前が test で始まる関数とメソッドを数える | そのまま |
| EX-core-303 | C-core-queries-13 | Php では、メソッドと test の呼び出しを数える | そのまま |
| EX-core-304 | C-core-queries-14 | Php では、メソッドと test の呼び出しを数える | そのまま |
| EX-core-305 | C-core-queries-15 | Php では、メソッドと test の呼び出しを数える | そのまま |
| EX-core-317 | C-core-queries-9 | TypeScript と JavaScript では、it と test の呼び出しを数える | 値違いの場面として C-9 にまとめた |
| EX-core-326 | C-core-queries-6 | Rust では、属性のパスの末尾が test の関数を必ず数え、属性とマクロを設定で足す | そのまま |
| EX-core-329 | - | Python では、名前が test で始まる関数とメソッドを数える | 関数の中のクラスのメソッドを数えることは落とした細部 |
| EX-core-330 | - | Rust では、属性のパスの末尾が test の関数を必ず数え、属性とマクロを設定で足す | マクロの中の "mod" の中の関数を数えることは落とした細部 |
| EX-core-331 | - | TypeScript と JavaScript では、it と test の呼び出しを数える | タグ付きテンプレートの扱いは落とした細部 |

元の文書に PROP は無い。

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_macro_reparse_byte_offset_reflects_delimiter_position | REQ-core-082 | C-core-queries-7 | マクロの中の関数が1つ数えられることを確かめる。行番号の確かめは指摘の行の話題の細部 |
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_macro_function_and_marker_lines_use_additive_offset | REQ-core-082, REQ-core-118, REQ-core-072 | C-core-queries-7 | この話題の分（マクロの中の関数が数えられる）だけ。REQ-core-118、REQ-core-072 はマーカーの話題 |
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_plain_test_function_line_is_one_indexed | REQ-core-082 | C-core-queries-4 | "#[test]" の関数が1つ数えられる。行番号は細部 |
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_function_with_unrelated_attribute_is_not_counted | REQ-core-082 | C-core-queries-4 | 関係の無い属性だけの関数を数えない |
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_has_attribute_skips_block_comment_to_find_test_attribute | REQ-core-082 | C-core-queries-4 | 属性と関数の間にコメントがあっても数える |
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_has_configured_attribute_skips_line_comment | REQ-core-082 | C-core-queries-5 | 設定の属性と関数の間に行コメントがあっても数える |
| crates/kotowari-source-analysis/tests/discovery.rs | req_082_has_configured_attribute_skips_block_comment | REQ-core-082 | C-core-queries-5 | 同上、ブロックコメント |
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_017_macro_not_in_the_configuration_is_not_reread | REQ-core-082, TBL-core-017 | C-core-queries-7 | 設定に無いマクロの中身を読み直さない |
| crates/kotowari-source-analysis/tests/discovery.rs | tbl_017_macro_with_parentheses_or_brackets_is_reread | REQ-core-082, TBL-core-017 | C-core-queries-7 | 区切りの記号によらずマクロの中身を読み直す |
| tests/step4_test_discovery.rs | req_082_test_attribute_is_always_counted | REQ-core-082, TBL-core-017 | C-core-queries-4 | そのまま |
| tests/step4_test_discovery.rs | req_082_configured_attribute_matches_path_with_arguments | REQ-core-082, TBL-core-017, EX-core-017 | C-core-queries-5 | そのまま |
| tests/step4_test_discovery.rs | req_082_macro_body_functions_are_counted_by_last_segment | REQ-core-082, TBL-core-017 | C-core-queries-7 | 名前の末尾の要素で当てる |
| tests/step4_test_discovery.rs | req_082_two_functions_in_a_path_macro_are_both_counted | REQ-core-082, EX-core-018 | C-core-queries-7 | そのまま |
| tests/step4_test_discovery.rs | ex_core_326_test_inside_a_function_body_is_counted | REQ-core-082, TBL-core-017, EX-core-326 | C-core-queries-6 | そのまま |
| tests/step4_test_discovery.rs | ex_core_330_function_in_a_mod_inside_a_macro_is_counted | REQ-core-082, TBL-core-017, EX-core-330 | C-core-queries-7 | 落とした細部のテスト。いちばん近い行を指す（マクロの中の "mod" の中の関数を数えることは落とした細部。マクロの中身の関数を数える C-7 がいちばん近い） |
| tests/step4_test_discovery.rs | tbl_016_marks_bind_only_to_the_first_macro_function_on_the_line | REQ-core-075, TBL-core-016, TBL-core-017 | C-core-queries-7 | この話題の分（マクロの中の2つの関数がどちらも数えられる）だけ。REQ-core-075、TBL-core-016 はマーカーの話題 |
| tests/step4_test_discovery.rs | tbl_016_macro_function_before_a_normal_test_on_the_line_takes_the_marks | REQ-core-075, TBL-core-016, TBL-core-017 | C-core-queries-7 | 同上 |
| tests/step4_test_discovery.rs | req_082_test_attribute_after_comment_is_recognized | REQ-core-082, TBL-core-017 | C-core-queries-4 | 属性と関数の間にコメントがあっても数える |
| tests/step4_test_discovery.rs | req_082_function_without_configured_attribute_not_counted | REQ-core-082, TBL-core-017 | C-core-queries-5 | 設定の属性の無い関数を数えない |
| tests/step4_test_discovery.rs | tbl_017_attribute_path_ending_in_test_is_counted | TBL-core-017 | C-core-queries-4 | "#[ test ]" の形 |
| tests/step4_test_discovery.rs | tbl_017_nested_function_in_macro_is_not_counted | TBL-core-017 | C-core-queries-7 | マクロの中の関数の中の関数を数えない |
| tests/step4_test_discovery.rs | tbl_016_macro_function_body_marker_does_not_bind | TBL-core-016, TBL-core-017 | - | 本体の先頭のマーカーが結び付かないことを確かめるだけで、この話題の行は確かめていない。TBL-core-016 の分は test-markers の対応表で付け替えるので、マーカーは空にならない |
| tests/step13_test_languages.rs | req_180_only_one_pair_of_the_same_quotes_is_stripped | REQ-core-180, TBL-core-032 | C-core-queries-8 | この話題の分（名前は最初の引数）だけ。REQ-core-180 の分は test-discovery の対応表に書いた |
| tests/step13_test_languages.rs | ex_core_298_skipped_test_is_counted | REQ-core-183, TBL-core-032, EX-core-298 | C-core-queries-8 | そのまま |
| tests/step13_test_languages.rs | ex_core_299_each_table_test_is_one_test | REQ-core-183, TBL-core-032, EX-core-299 | C-core-queries-9 | そのまま |
| tests/step13_test_languages.rs | ex_core_317_chained_each_table_test_is_one_test | REQ-core-183, TBL-core-032, EX-core-317 | C-core-queries-9 | そのまま |
| tests/step13_test_languages.rs | tbl_032_test_whose_result_is_called_again_is_counted | REQ-core-183, TBL-core-032 | C-core-queries-8 | 結果をさらに呼ぶ "it(...)" も数える |
| tests/step13_test_languages.rs | tbl_032_dotted_form_with_a_unicode_name_or_a_comment_is_counted | REQ-core-183, TBL-core-032 | C-core-queries-8 | "." の後の名前が ASCII でなくても、間にコメントがあっても数える |
| tests/step13_test_languages.rs | ex_core_331_tagged_template_alone_is_not_counted | REQ-core-183, TBL-core-032, EX-core-331 | C-core-queries-9 | 落とした細部のテスト。いちばん近い行を指す（タグ付きテンプレート "test.each`表`" の扱いは落とした細部。"each" の形を外側の呼び出しの1つだけ数える C-9 がいちばん近い） |
| tests/step13_test_languages.rs | ex_core_300_describe_and_other_callers_are_not_counted | REQ-core-183, TBL-core-032, EX-core-300 | C-core-queries-10 | そのまま |
| tests/step13_test_languages.rs | tbl_032_dotted_forms_are_counted_with_the_first_argument | REQ-core-183, TBL-core-032 | C-core-queries-8, C-core-queries-9, C-core-queries-10 | "." の形、"test.concurrent.each"、"describe.each" を数えないことの3つを確かめる |
| tests/step13_test_languages.rs | ex_core_301_test_in_a_tsx_file_is_counted | REQ-core-182, REQ-core-183, EX-core-301 | C-core-queries-2 | そのまま |
| tests/step13_test_languages.rs | req_183_javascript_files_are_counted | REQ-core-182, REQ-core-183 | C-core-queries-3 | そのまま |
| tests/step13_test_languages.rs | tbl_021_extensions_of_the_bundled_languages_have_a_query | TBL-core-021, TBL-core-031, REQ-core-182 | C-core-queries-1 | この話題の分だけ。TBL-core-031 の分は test-discovery の対応表に書いた。TBL-core-021 は出力の話題 |
| tests/step13_test_languages.rs | ex_core_302_nested_function_is_not_counted | REQ-core-184, TBL-core-033, EX-core-302 | C-core-queries-11 | そのまま |
| tests/step13_test_languages.rs | tbl_033_decorated_test_is_counted_at_its_def_line | REQ-core-184, TBL-core-033, TBL-core-019 | C-core-queries-12 | この話題の分だけ。行が "def" の行であること（TBL-core-019）は指摘の行の話題 |
| tests/step13_test_languages.rs | ex_core_329_method_of_a_class_inside_a_function_is_counted | REQ-core-184, TBL-core-033, EX-core-329 | C-core-queries-11 | 最上位の関数 "test_outer" を数えることだけがこの行に当たる。関数の中のクラスのメソッドを数えることは落とした細部 |
| tests/step13_test_languages.rs | tbl_034_test_methods_and_attributes | REQ-core-185, TBL-core-034, TBL-core-019 | C-core-queries-13 | この話題の分だけ。行が "#[Test]" の行であること（TBL-core-019）は指摘の行の話題 |
| tests/step13_test_languages.rs | ex_core_303_method_with_a_qualified_test_attribute_is_counted | REQ-core-185, TBL-core-034, EX-core-303 | C-core-queries-13 | そのまま |
| tests/step13_test_languages.rs | ex_core_304_method_with_a_test_docblock_is_counted | REQ-core-185, TBL-core-034, EX-core-304 | C-core-queries-14 | そのまま |
| tests/step13_test_languages.rs | ex_core_305_pest_test_call_is_counted | REQ-core-185, TBL-core-034, EX-core-305 | C-core-queries-15 | そのまま |
| tests/step13_test_languages.rs | tbl_034_mark_before_a_docblock_test_binds | REQ-core-075, TBL-core-034 | C-core-queries-14 | この話題の分（複数行の "/**" の docblock の "@test" で数える）だけ。REQ-core-075 はマーカーの話題 |

## 落とした細部

### 安全に関わるもの

無い。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| TBL-core-017: マクロの中で数える「最上位の関数」は、ほかの関数の本体の中にない関数とし、"mod" や "impl" の中の関数を含む | review10-gaps の A4。細部を任せた承認の範囲で決めた（下の「迷った判断」） |
| TBL-core-033: クラスの中のメソッドは、そのクラスが関数の中にあっても数える | review10-gaps の A3。同上 |
| TBL-core-032: タグ付きテンプレート（"test.each`表`"）はそれだけでは数えず、結果を呼ぶ形は外側の呼び出しを数える | review11-gaps の A1。同上 |

次は落としたのではなく、ほかの話題に移した（決めたのは利用者）。

| 要求のどの部分 | 移した先 |
|---|---|
| TBL-core-017: マクロの中の関数のマーカーの結び付けと invalid_marker（records.md の A121、multi-language-tests の A26） | テストのマーカーの話題。決めたことには言葉で指した |
| TBL-core-033: デコレータがテストの節の外にあること、TBL-core-034: 属性がテストの節の中にあること（multi-language-tests の A17、A32） | マーカーの結び付けと、test_without_id の "line" の話題 |
| TBL-core-032: "describe" の名前を付けないこと（multi-language-tests の A25） | [テストの名前は、問い合わせが捕まえた文字から取る](test-discovery.md#テストの名前は問い合わせが捕まえた文字から取る) |

## 形に依存する要求

無い。

## テストの無い行

無い。

## 誰が決めたか見分けられなかったもの

- records.md の A39、A47（第6ラウンド）、A49（第7ラウンド）、A121、A122（第9ラウンド）には decided_by の行が無い。ラウンドの見出しの利用者の答えから、利用者の決定として扱った（test-discovery の対応表と同じ扱い）。

## 理由や見送った案が記録に無かった決定

- どの決定も、見送った案が記録に無い。節を置かなかった（multi-language-tests の Rejected の A33〜A35 は、この話題の決定の代わりの案ではない）。
- 「Python では、名前が test で始まる関数とメソッドを数える」: 関数の中の関数を数えない理由が記録に無い（A19 の why は、主な2つの書き方の両方を拾えることだけ）。問題の節には A19 の why だけを書いた。

## 分けた所、まとめた所

- REQ-core-082 と TBL-core-017、REQ-core-183 と TBL-core-032、REQ-core-184 と TBL-core-033、REQ-core-185 と TBL-core-034 を、それぞれ1つの決定にした。
- REQ-core-182 は独立の決定にした。
- TBL-core-017 の「置き場所によらず数える」の行は、関数とマクロの両方に当たるが、確認することは関数の場合（C-6）だけにした。関数の中のマクロを確かめるテストが無いため。
- 値違いの場面（EX-core-299 と EX-core-317）は C-9 の1行にまとめた。

## あとでリンクにする所

確定の段で、次を見出しへのリンクにした。下書きどうしのリンクも `<file>.ja.md#<anchor>` に直した。

- テストのマーカーの下書き（test-markers）の「マーカーは、テストの直前のコメントの塊にあるものだけをそのテストに結び付ける」: 冒頭の段落と、Rust の決定の「マクロの中の関数のマーカーは、マクロの外と同じ規則で結び付ける」
- 指摘の並べ方の下書き（finding-order）の「line は1始まりで数え、文書全体にかかる指摘は null にする」: 冒頭の段落（指摘の行）。その決定が「行を持つ指摘がどの行を指すかは、その指摘を決めた話題で決める」としている

残り: 設定の話題の "tests.rust.attributes" と "tests.rust.macros" の既定。この文書の文では既定を指していないので、リンクにする所は無い

## 迷った判断と、どう決めたか

- review10-gaps と review11-gaps の決定は、decided_by が「利用者（推奨を採用）」だが、記録の Context に、利用者の「ほぼ答えが決まっているものは推奨で決めてよい」という任せた承認の範囲で決めたとあるので、agent の細部として落としたままにした（安全に関わる落とした細部の一覧には無い）。そのテストのうち、この話題の ID だけを指す `ex_core_330_...` と `ex_core_331_...` は、マーカーが空にならないよう、いちばん近い行（C-7、C-9）を指す。`ex_core_329_...` はもとから C-11 を指す。
- review10、review11 で不具合として直したもの（"it(...)" の結果をさらに呼ぶ形、"." の後の名前が ASCII でない形、"()" と "[]" のマクロ）は、元の表の文どおりの振る舞いなので、その表の行の確認することを指すテストとした。
- 記録の why と Context にあるテストのライブラリの列挙とその版の番号は、外部の名前なので、文書にも対応表にも書かなかった。元の表に出てくる "PHPUnit"（属性のパス）と Pest は、数え方そのものなので残した。

## 数

- 元: 要求 5、場面 16、決定表 4、性質 0
- 読んだ記録のファイル: 4（records.md、2026-09-24-multi-language-tests.md、2026-09-24-review10-gaps.md、2026-09-24-review11-gaps.md）
- 出来上がり: 決定 5、確認することの行 15（うち「（読んで確認）」0）
- テスト: 42
