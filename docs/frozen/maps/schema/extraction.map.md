# extraction の対応表

元: `docs/ir/schema/extraction.md`（日本語は `extraction.ja.md`）。書き直した文書: `extraction.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-035 | C-schema-extract-10, C-schema-extract-11, C-schema-extract-12, C-schema-extract-13, C-schema-extract-14, C-schema-extract-15, C-schema-extract-16 | value か導かれる値を宣言したときだけ、要素ごとのオブジェクトを組み立てる | 抽出の書式は TBL-schema-008 を定義に持つので、表と一緒に1つの決定にした。ノードごとの値を行に分けた |
| REQ-schema-036 | C-schema-extract-2, C-schema-cli-22 | 値は、配置パスが指す場所に、ドットで区切った名前の入れ子で置く | 入れ子を C-2 にした。空の名前の停止（review4-gaps の A4）は cli の「配置パスの、ドットで区切った名前が空なら停止する」（C-schema-cli-22）と同じ決定なので cli に一本化し、この決定からリンクする（以前の C-schema-extract-3 は欠番） |
| REQ-schema-037 | C-schema-extract-1 | 抽出した値は文字列のまま出し、型を変換しない | 位置情報を対象外にすることは、導かれる値の決定（C-21）に書いた |
| REQ-schema-038 | C-schema-extract-4 | 抽出の対象が文書に無いときは、その配置パスのキーを出さない | そのまま |
| REQ-schema-039 | C-schema-extract-19, C-schema-extract-20 | 項目自身が抽出を宣言しないまま、内側にだけ抽出を宣言したスキーマは停止する | 内側のノードを C-19、箇条書きの子のフィールド行（mds-spec の A68）を C-20 にした |
| REQ-schema-047 | C-schema-extract-17, C-schema-extract-18 | 項目の内側に抽出があれば、項目は内側の配置パスを鍵にしたオブジェクトにする | オブジェクトになる場合を C-17、文字列のままの場合を C-18 にした。`value` か導かれる値を宣言したときのオブジェクトは C-11 |
| REQ-schema-048 | C-schema-extract-5, C-schema-extract-6, C-schema-extract-7, C-schema-extract-8, C-schema-extract-9, C-schema-extract-11, C-schema-extract-21, C-schema-extract-22, C-schema-extract-23, C-schema-extract-24, C-schema-extract-25, C-schema-extract-26 | 導かれる値は5つの語にし、語ごとに宣言できるノードを限る | 宣言の形（C-5〜C-9）、要素ごとのオブジェクト（C-11）、導かれる値の語（C-21〜C-26）の3つの決定に分けた。見出しは要求の名前に当たる導かれる値の決定にした |
| REQ-schema-062 | C-schema-extract-27 | 要素の最後の行は、次の同じ深さか浅い見出しの手前の行にする | そのまま |
| REQ-schema-040 | C-schema-extract-44 | ast は、検査とは別に、文書の素の構文木を JSON で出す | 利用者が批准したのは CLI が素の構文木の `ast` を受けることだけ（mds-spec の A26）。mdast への準拠、インライン要素、位置情報を含めないことは実装者の決定（A19）で落とした。cli の C-schema-cli-2（同じ振る舞い）は欠番にし、`ast` の出力はこの話題に一本化した |
| REQ-schema-045 | C-schema-extract-34, C-schema-extract-36 | 区切り文字を宣言したフィールド行は、出現回数に関わらず配列にする | 配列と配列の配列を C-34、`value` の鍵の下に置くことを C-36 にした |
| REQ-schema-046 | C-schema-extract-35 | 区切り文字を宣言したフィールド行は、出現回数に関わらず配列にする | REQ-schema-045 と同じ決定にまとめた。末尾の要素へのつなぎ方（extract-original-lines の A7）は落とした |
| REQ-schema-063 | C-schema-extract-37, C-schema-extract-38, C-schema-extract-39, C-schema-extract-40 | 値の行は、文書の元の行のまま使う | 字下げと空白を C-37、始まる位置を C-38、つなぎ方を C-39、マーカーだけの行を C-40 にした |
| REQ-schema-064 | C-schema-extract-41, C-schema-extract-42 | 見出しの名前と表のセルの値は、インラインの記法の記号を外した文字で読む | 抽出の値を C-41、照合を C-42 にした |
| TBL-schema-008 | C-schema-extract-10, C-schema-extract-11, C-schema-extract-12, C-schema-extract-13, C-schema-extract-14, C-schema-extract-15, C-schema-extract-16, C-schema-extract-25 | value か導かれる値を宣言したときだけ、要素ごとのオブジェクトを組み立てる | 決定表として「決めたこと」の中に置いた。表の行の鍵と繰り返す表の段は、記録が別にあり見送った案も持つので、別の決定にした |
| PROP-schema-007 | C-schema-extract-43 | 抽出の結果は、閉じた世界と開いた世界のどちらで検査しても変わらない | 性質の行にした。テストが無い（下の「テストの無い行」） |
| EX-schema-013 | C-schema-extract-2 | 値は、配置パスが指す場所に、ドットで区切った名前の入れ子で置く | そのまま |
| EX-schema-014 | C-schema-extract-19 | 項目自身が抽出を宣言しないまま、内側にだけ抽出を宣言したスキーマは停止する | 内側の表、フィールド行、コードブロックの場合を1行にまとめた |
| EX-schema-018 | C-schema-extract-30 | 表の行の鍵は、header の宣言か列の位置から取り、文書のヘッダ行の文字を使わない | そのまま |
| EX-schema-019 | C-schema-extract-32 | 要素ごとに宣言した表が繰り返すときは表ごとの段を作り、それ以外は1つの配列につなぐ | そのまま |
| EX-schema-020 | C-schema-extract-28, C-schema-extract-29 | 文に導かれる値を宣言すると行ごとの要素にし、要素の値は前後の空白を取り除く | 要素の数と継続段落を C-28、値と生の行を C-29 にした |
| EX-schema-021 | C-schema-extract-10 | value か導かれる値を宣言したときだけ、要素ごとのオブジェクトを組み立てる | そのまま |
| EX-schema-022 | C-schema-extract-5 | 抽出は、path・value・of を持つ1つの入れ子で宣言する | 書式の並びで書いた宣言の停止と1行にまとめた |
| EX-schema-023 | C-schema-extract-6 | 抽出は、path・value・of を持つ1つの入れ子で宣言する | そのまま |
| EX-schema-024 | C-schema-extract-22 | 導かれる値は5つの語にし、語ごとに宣言できるノードを限る | そのまま |
| EX-schema-025 | C-schema-extract-25, C-schema-extract-26 | 導かれる値は5つの語にし、語ごとに宣言できるノードを限る | 指す行を C-25、書いたとおりに出すことを C-26 にした |
| EX-schema-045 | C-schema-extract-27 | 要素の最後の行は、次の同じ深さか浅い見出しの手前の行にする | そのまま |
| EX-schema-046 | C-schema-extract-24 | 導かれる値は5つの語にし、語ごとに宣言できるノードを限る | そのまま |
| EX-schema-054 | C-schema-extract-37, C-schema-extract-40 | 値の行は、文書の元の行のまま使う | 字下げを C-37、マーカーだけの行を C-40 にした |
| EX-schema-055 | C-schema-extract-38, C-schema-extract-35 | 値の行は、文書の元の行のまま使う | 値の始まりを C-38、区切った末尾の要素に継続段落が付くことを C-35 にした |
| EX-schema-056 | C-schema-extract-39 | 値の行は、文書の元の行のまま使う | そのまま |
| EX-schema-057 | C-schema-extract-39 | 値の行は、文書の元の行のまま使う | 読み方が行のときを C-39 の後半にまとめた |
| EX-schema-058 | C-schema-extract-41 | 見出しの名前と表のセルの値は、インラインの記法の記号を外した文字で読む | そのまま |
| EX-schema-059 | C-schema-extract-42 | 見出しの名前と表のセルの値は、インラインの記法の記号を外した文字で読む | そのまま |
| EX-schema-060 | C-schema-extract-15 | value か導かれる値を宣言したときだけ、要素ごとのオブジェクトを組み立てる | EX-schema-061 と1行にまとめた |
| EX-schema-061 | C-schema-extract-15 | value か導かれる値を宣言したときだけ、要素ごとのオブジェクトを組み立てる | EX-schema-060 と1行にまとめた |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/tests/properties.rs | extracted_leaves_are_always_strings | REQ-schema-037 | C-schema-extract-1 | 数や日付に見える値の文書で、抽出の葉がすべて文字列であることを性質のテストで確かめている |
| crates/kotowari-markdown-schema/src/ast.rs | ast_json_is_mdast_shape_with_inline_and_no_position | REQ-schema-040 | C-schema-extract-44 | 見出し・段落・一覧の木が出ることを確かめている。インライン要素と位置情報の無さは落とした細部で、テストの中に残る |
| crates/kotowari-markdown-schema/src/ast.rs | ast_json_excludes_frontmatter_node | REQ-schema-040 | C-schema-extract-44 | 木の子が見出しだけになることを確かめている。frontmatter を木に入れないことは記録に無い細部で、テストの中に残る |
| crates/kotowari-markdown-schema/src/schema/tests.rs | extract_accepts_the_shorthand_and_the_named_group | REQ-schema-035 | C-schema-extract-7 | 略記が path だけの宣言として読まれることを確かめている |
| crates/kotowari-markdown-schema/src/schema/tests.rs | capture_extract_outside_title_is_schema_invalid | REQ-schema-035 | C-schema-extract-9 | 題名の外の7つのノードの group が停止することを確かめている |
| crates/kotowari-markdown-schema/src/schema/tests.rs | an_empty_name_in_the_value_or_an_of_key_of_an_element_object_is_schema_invalid | REQ-schema-036, TBL-schema-009 | - | 空の名前の停止は cli の「配置パスの、ドットで区切った名前が空なら停止する」に一本化した。付け替え先（C-schema-cli-22）は cli の対応表にある |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_internal_extract_without_item_extract_is_schema_invalid | REQ-schema-039 | C-schema-extract-19 | 内側のフィールド行だけの抽出で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_child_field_extract_without_item_extract_is_schema_invalid | REQ-schema-039 | C-schema-extract-20 | 箇条書きの子のフィールド行だけの抽出で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_deep_child_field_extract_without_item_extract_is_schema_invalid | REQ-schema-039 | C-schema-extract-20 | 深い入れ子の子のフィールド行だけの抽出で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | capture_extract_without_pattern_is_schema_invalid | REQ-schema-035 | C-schema-extract-9 | 正規表現の無い題名の group で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | capture_extract_with_pattern_missing_the_group_is_schema_invalid | REQ-schema-035 | C-schema-extract-9 | 名前付きキャプチャを含まない正規表現で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | children_bullets_with_extract_is_schema_invalid | REQ-schema-035 | C-schema-block-22 | 箇条書きの子の箇条書きの抽出の停止（mds-spec の A67）。block-rules の「箇条書きの子の一覧は、宣言した子の規則に照らす」の行を付け替え先にした |
| crates/kotowari-markdown-schema/src/schema/tests.rs | deep_children_bullets_with_extract_is_schema_invalid | REQ-schema-035 | C-schema-block-22 | 上と同じく、深い入れ子の子の箇条書きの抽出の停止 |
| crates/kotowari-markdown-schema/src/schema/tests.rs | child_field_can_have_extract | REQ-schema-035, REQ-schema-036 | C-schema-block-21 | 落とした細部のテスト。いちばん近い行を指す。子のフィールド行に抽出を書いたスキーマが読めることだけを確かめ、値の置き場は確かめていない。子の規則にフィールド行を宣言できる block-rules の行を指す |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_table_extract_without_item_extract_is_rejected | REQ-schema-039, EX-schema-014 | C-schema-extract-19 | 内側の表だけの抽出で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_codeblock_extract_without_item_extract_is_rejected | REQ-schema-039 | C-schema-extract-19 | 内側のコードブロックだけの抽出で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | of_id_outside_an_item_is_rejected | REQ-schema-035, REQ-schema-048 | C-schema-extract-23 | フィールド行の id で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | unknown_of_value_is_rejected | REQ-schema-035, REQ-schema-048 | C-schema-extract-22 | 知らない語で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_internal_extract_with_item_extract_loads | REQ-schema-039, REQ-schema-047 | C-schema-extract-19 | 項目が抽出を持てば内側の宣言で停止しないこと（C-19 の後半）を確かめている |
| crates/kotowari-markdown-schema/src/schema/tests.rs | nested_extract_declaration_loads | REQ-schema-035, REQ-schema-048 | C-schema-extract-5 | 落とした細部のテスト。いちばん近い行を指す。path・value・of を持つ1つの入れ子の宣言が読めること（C-5 は、その形でない宣言の停止） |
| crates/kotowari-markdown-schema/src/schema/tests.rs | legacy_sequence_extract_is_schema_invalid | REQ-schema-035 | C-schema-extract-5 | 書式の並びで停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | extract_without_a_path_is_schema_invalid | REQ-schema-035 | C-schema-extract-5 | path の無い宣言で停止する |
| crates/kotowari-markdown-schema/src/schema/tests.rs | duplicate_key_between_an_inner_node_and_a_derived_value_is_schema_invalid | REQ-schema-042, REQ-schema-048 | - | 要素のオブジェクトの中の鍵の重複（TBL-schema-009）は「CLI と結果の出し方」の話題の分 |
| crates/kotowari-markdown-schema/src/schema/tests.rs | duplicate_key_between_value_and_a_derived_value_is_schema_invalid | REQ-schema-042, REQ-schema-048 | - | 上と同じ |
| crates/kotowari-markdown-schema/src/schema/tests.rs | a_parent_key_is_a_duplicate_but_a_sibling_key_is_not | REQ-schema-042, REQ-schema-048 | - | 上と同じ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | adr_values_are_nested_by_path | REQ-schema-036 | C-schema-extract-2 | ドットを含む配置パスが入れ子になることを確かめている |
| crates/kotowari-markdown-schema/src/extract/tests.rs | undeclared_field_name_line_is_extracted_as_a_bullet | REQ-schema-028, REQ-schema-035 | C-schema-extract-12 | 箇条書きの値が元の行であることを確かめている。宣言していない名前の行を箇条書きと読むことは「見出しの下の行の規則」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_undeclared_field_name_line_as_a_bullet | REQ-schema-028, REQ-schema-035 | C-schema-extract-13 | 節の本文に箇条書きが入ることを確かめている。読み分けは「見出しの下の行の規則」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | a_child_line_with_an_undeclared_name_stays_in_the_parent_bullet_value | REQ-schema-031, REQ-schema-063 | C-schema-extract-12 | 子の箇条書きの行が字下げのまま親の値に入る。子の行の読み分けは「見出しの下の行の規則」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | undeclared_field_name_line_extract_preserves_original_marker | REQ-schema-035 | C-schema-extract-12 | 元のマーカーを保つ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_undeclared_field_name_line_with_original_marker | REQ-schema-035 | C-schema-extract-13 | 節の本文の箇条書きが元のマーカーを保つ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | title_named_group_capture_is_extracted | REQ-schema-035 | C-schema-extract-8 | 名前付きキャプチャが値になる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_node_is_an_array_when_present_and_omits_key_when_absent | REQ-schema-019, REQ-schema-038 | C-schema-extract-4 | 無いときにキーを出さないことを確かめている。範囲で配列になることは「スキーマ言語の骨格」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | non_repeated_node_is_single_and_omits_key_when_missing | REQ-schema-019, REQ-schema-038 | C-schema-extract-4 | 上と同じく、この話題の分はキーを出さないこと |
| crates/kotowari-markdown-schema/src/extract/tests.rs | non_repeated_bullets_omit_key_when_missing | REQ-schema-038 | C-schema-extract-4 | 箇条書きが無いときにキーを出さない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_bullets_omit_key_when_missing | REQ-schema-038 | C-schema-extract-4 | 上と同じ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | separator_trims_elements_and_keeps_empty_ones | REQ-schema-029, REQ-schema-035, REQ-schema-037, REQ-schema-045 | C-schema-extract-34 | 区切った値が配列になることを確かめている。前後の空白を取り除くことと空の要素を残すことは「見出しの下の行の規則」の話題（区切った要素の照合）と、落とした細部 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_field_with_separator_extracts_array_of_arrays | REQ-schema-019, REQ-schema-035, REQ-schema-045 | C-schema-extract-34 | 配列の配列になる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | separator_split_excludes_continuation_paragraphs | REQ-schema-030, REQ-schema-035, REQ-schema-046, REQ-schema-063 | C-schema-extract-35 | 継続段落が区切られずに末尾の要素に付く |
| crates/kotowari-markdown-schema/src/extract/tests.rs | a_crlf_document_gives_the_same_values_as_the_lf_document | REQ-schema-063 | C-schema-extract-37 | 落とした細部のテスト。いちばん近い行を指す。改行の形（CRLF）に依らず、値の行が元の行のまま同じ値になる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | wrapped_line_is_part_of_value_and_subject_to_separator_split | REQ-schema-029, REQ-schema-035 | C-schema-extract-35 | 空行を挟まずに続く行が区切る対象に入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | paragraph_after_code_block_lead_is_extracted_as_a_statement | REQ-schema-032, REQ-schema-035 | - | どの段落を文と読むかを確かめていて、「見出しの下の行の規則」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_statements_and_bullets_only | REQ-schema-035, REQ-schema-063 | C-schema-extract-13, C-schema-extract-39 | 表とコードブロックを抜くことを C-13、部分の間の空行を C-39 で確かめている |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_continuation_paragraphs_and_nested_bullets | REQ-schema-030, REQ-schema-035 | C-schema-extract-13 | 節の本文に箇条書きの継続段落と子の行が入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_preserves_original_markers | REQ-schema-028, REQ-schema-035 | C-schema-extract-12 | 3種のマーカーを保つ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_preserves_marker_with_continuation | REQ-schema-030, REQ-schema-035 | C-schema-extract-12 | 継続段落を付けてもマーカーを保つ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_preserves_whitespace_after_marker_with_continuation | REQ-schema-030, REQ-schema-035 | C-schema-extract-12 | マーカーの後の空白を保つ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_includes_lead_paragraph_on_a_separate_line_from_the_marker | REQ-schema-035, REQ-schema-063 | C-schema-extract-40 | マーカーだけの行の後の lead 段落を欠かさない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_keeps_trailing_whitespace_of_the_marker_line | REQ-schema-035 | C-schema-extract-37 | 途中の行の末尾の空白を残す |
| crates/kotowari-markdown-schema/src/extract/tests.rs | undeclared_field_name_line_extract_preserves_whitespace_after_marker | REQ-schema-035 | C-schema-extract-12 | マーカーの後の空白を保つ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_includes_marker_and_continuation | REQ-schema-030, REQ-schema-035, REQ-schema-063 | C-schema-extract-12, C-schema-extract-39 | 継続段落を含むことを C-12、元の空行でつなぐことを C-39 で確かめている |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_with_multiple_continuations_keeps_the_blank_lines_between_them | REQ-schema-030, REQ-schema-035, REQ-schema-063 | C-schema-extract-39 | 継続段落どうしの間の空行が残る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | ordered_list_is_not_extracted_as_a_bullet | REQ-schema-028, REQ-schema-035 | - | 順序付きリストを箇条書きと読まないことは「見出しの下の行の規則」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | field_extract_includes_continuation_paragraph | REQ-schema-030, REQ-schema-035, REQ-schema-046, REQ-schema-063 | C-schema-extract-39 | フィールド行の lead 段落と継続段落が元の空行でつながる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_body_field_line_includes_continuation_paragraph | REQ-schema-030, REQ-schema-035, REQ-schema-063 | C-schema-extract-15 | 項目の本文に、宣言したフィールド行の継続段落が入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_extract_is_heading_plus_body | REQ-schema-035, REQ-schema-063 | C-schema-extract-14 | 見出しと本文を改行でつなぐ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_extract_concatenates_multiple_tables_in_document_order | REQ-schema-033, REQ-schema-035 | C-schema-extract-33 | 2つの表の行が現れた順に1つの配列になる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_table_extracts_flat_row_objects_without_nesting | REQ-schema-033, REQ-schema-035 | C-schema-extract-33 | 略記の表は範囲を宣言しても段を作らない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | statement_with_a_derived_value_extracts_one_object_per_line | REQ-schema-035, REQ-schema-048 | C-schema-extract-28, C-schema-extract-29 | 行ごとの要素と、前後の空白を取り除いた値 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | statement_with_a_derived_value_lists_paragraphs_separated_by_a_blank_line | REQ-schema-035, REQ-schema-048 | C-schema-extract-28 | 空行を挟んで続く段落も同じ並びに入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | statement_element_value_is_trimmed_and_raw_keeps_the_original_line | REQ-schema-035, REQ-schema-048 | C-schema-extract-29 | 値と生の行が別物になる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | a_continuation_paragraph_is_not_a_statement_element | REQ-schema-035, REQ-schema-048 | C-schema-extract-28 | 一覧の行の継続段落は文の要素に入らない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_with_a_derived_value_puts_the_row_line_into_each_row_object | REQ-schema-033, REQ-schema-048 | C-schema-extract-11, C-schema-extract-25 | データ行ごとのオブジェクトと、データ行の行番号 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_table_with_a_derived_value_nests_rows_per_table | REQ-schema-033, REQ-schema-048 | C-schema-extract-32 | 表ごとの段ができる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_without_a_declared_header_gives_each_row_as_an_array | REQ-schema-033, REQ-schema-035 | C-schema-extract-30 | header を宣言しない表の行は列の位置の配列 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_with_empty_or_repeated_header_cells_keeps_every_column | REQ-schema-033, REQ-schema-035 | C-schema-extract-30 | 空のヘッダと重なった列の名前で値が失われない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | declared_header_decides_the_row_keys_when_the_column_count_differs | REQ-schema-033, REQ-schema-035 | C-schema-extract-31 | 列の数が違うときの鍵 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_omits_key_when_no_tables | REQ-schema-038 | C-schema-extract-4 | 表が無いときにキーを出さない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_section_extracts_array_of_bodies | REQ-schema-019, REQ-schema-035 | - | 出現回数の範囲で配列になることを確かめていて、「スキーマ言語の骨格」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | typed_ast_includes_type_when_schema_is_named | REQ-schema-036 | C-schema-cli-3 | `ast --schema` の型付きの値に "name" が "type" として入り、題名の値も入る。cli の「ast --schema は、…」の行を付け替え先にした |
| crates/kotowari-markdown-schema/src/extract/tests.rs | typed_ast_omits_type_when_schema_is_unnamed | REQ-schema-036 | C-schema-cli-3 | "name" を宣言しないスキーマでは "type" の鍵が無い。上と同じ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | parent_bullet_element_includes_child_bullet_lines_with_indentation | REQ-schema-031, REQ-schema-035 | C-schema-extract-12 | 子の箇条書きの行を字下げのまま含む |
| crates/kotowari-markdown-schema/src/extract/tests.rs | declared_child_field_is_excluded_from_the_parent_element | REQ-schema-031, REQ-schema-035 | - | 宣言した子のフィールド行を親の値から外すことを確かめている。子のフィールド行の扱いは「見出しの下の行の規則」の話題の分 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | child_field_with_own_extract_is_placed_at_its_path | REQ-schema-036 | C-schema-extract-2 | 子のフィールド行の値が自分の配置パスに置かれる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | recursive_nesting_is_included_in_the_parent_element | REQ-schema-031, REQ-schema-035 | C-schema-extract-12 | 孫の箇条書きの行まで含む |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_child_bullet_lines | REQ-schema-031, REQ-schema-035 | C-schema-extract-13 | 節の本文に子の箇条書きの行が入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_body_includes_child_bullet_lines | REQ-schema-031, REQ-schema-035 | C-schema-extract-14 | 項目の本文に箇条書きが子の行ごと入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | deep_child_field_extract_is_placed_at_its_path | REQ-schema-036 | C-schema-extract-2 | 深い子のフィールド行の値が自分の配置パスに置かれる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_body_excludes_table_and_code_block | REQ-schema-035 | C-schema-extract-14 | 項目の本文に表とコードブロックを含めない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | of_line_places_the_line_number_as_a_number | REQ-schema-048 | C-schema-extract-6, C-schema-extract-21 | of だけの要素と、行番号が数値であること |
| crates/kotowari-markdown-schema/src/extract/tests.rs | a_path_only_item_extract_becomes_an_object_when_internals_declare_extract | REQ-schema-047 | C-schema-extract-17 | path だけの項目の抽出でもオブジェクトになる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_extract_becomes_an_object_when_internals_declare_extract | REQ-schema-047, REQ-schema-048 | C-schema-extract-17 | 内側の配置パスを鍵にしたオブジェクトになる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_extract_stays_a_string_without_internal_extract | REQ-schema-047 | C-schema-extract-18 | 内側に抽出が無ければ文字列のまま |
| crates/kotowari-markdown-schema/src/extract/tests.rs | codeblock_with_a_derived_value_gives_one_object_per_block | REQ-schema-035, REQ-schema-048 | C-schema-extract-11, C-schema-extract-16, C-schema-extract-25 | ブロックごとのオブジェクト、中身全体の値、フェンスの開始行 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullets_with_a_derived_value_give_one_object_per_line | REQ-schema-035, REQ-schema-048 | C-schema-extract-11 | 行ごとのオブジェクト |
| crates/kotowari-markdown-schema/src/extract/tests.rs | title_with_a_raw_derived_value_gives_the_heading_line | REQ-schema-035, REQ-schema-048 | C-schema-extract-26 | 生の行が見出しの行を書いたとおりに出す |
| crates/kotowari-markdown-schema/src/extract/tests.rs | field_with_a_value_and_a_derived_value_nests_under_the_path | REQ-schema-035, REQ-schema-048 | C-schema-extract-11 | 分けないフィールド行が配置パスの下に1つのオブジェクトになる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_without_a_value_keeps_the_inner_placement_paths | REQ-schema-047, REQ-schema-048 | C-schema-extract-6 | value を省いた要素が導かれる値と内側の配置パスだけを持つ |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_object_keys_come_from_the_schema_not_the_document | REQ-schema-047 | C-schema-extract-17 | 文書の語が違っても鍵が同じ |
| crates/kotowari-markdown-schema/src/document/tests.rs | statement_keeps_each_raw_line_with_its_line_number | REQ-schema-035 | C-schema-extract-29 | 読み込んだ文書の内部の構造で、文の行ごとに字下げを含む元の行と行番号を持つことを確かめている。出力は確かめていない |
| crates/kotowari-markdown-schema/src/document/tests.rs | table_keeps_the_line_number_of_each_data_row | REQ-schema-048 | C-schema-extract-25 | ヘッダの行と区切りの行を数えず、データ行の行番号を持つ（内部の構造で確かめている） |
| crates/kotowari-markdown-schema/src/document/tests.rs | end_line_does_not_count_a_heading_shaped_line_inside_a_code_block | REQ-schema-062 | C-schema-extract-27 | コードブロックの中の見出しの形の行で終わらないことと、文書の最後の行 |
| crates/kotowari-markdown-schema/src/document/tests.rs | end_line_is_the_last_line_when_the_document_does_not_end_with_a_break | REQ-schema-062 | C-schema-extract-27 | 最後の行に改行が無くても最後の行になる |
| crates/kotowari-markdown-schema/src/document/tests.rs | a_lone_cr_breaks_a_line_the_same_way_as_lf_and_crlf | REQ-schema-035 | C-schema-extract-29 | 落とした細部のテスト。いちばん近い行を指す。単独の CR と CRLF の文書でも、文の行ごとの生の行と行番号が LF の文書と同じになる（内部の構造で確かめている） |
| crates/kotowari-mds/tests/cli.rs | ast_outputs_mdast_json_for_the_adr_fixture | REQ-schema-005, REQ-schema-040 | C-schema-extract-44 | `kotowari-mds ast` が見出し・一覧・段落の木を出す。REQ-schema-005 は「CLI と結果の出し方」の話題の分 |
| crates/kotowari-mds/tests/cli.rs | ast_schema_outputs_typed_tree | REQ-schema-036, REQ-schema-067 | C-schema-extract-2 | `ast --schema` の値が配置パスの入れ子になる。REQ-schema-067 は「CLI と結果の出し方」の話題の分 |
| crates/kotowari-mds/tests/cli.rs | check_item_child_field_extract_stops_with_schema_invalid | REQ-schema-039 | C-schema-extract-20 | `check` が `schema_invalid` で終了コード 2 になる |
| crates/kotowari-mds/tests/cli.rs | values_json_is_nested_by_path | REQ-schema-036, EX-schema-013 | C-schema-extract-2 | `values --format json` の入れ子 |
| crates/kotowari-mds/tests/cli.rs | ir_fixture_passes_check_and_extracts_values | REQ-schema-010, REQ-schema-035 | - | 見本の文書を端から端まで通すだけで、この話題のどの行の振る舞いも区別して確かめていない。REQ-schema-010 は別の話題の分 |
| crates/kotowari-mds/tests/cli.rs | decision_record_fixture_passes_check_and_extracts_values | REQ-schema-035, REQ-schema-031 | C-schema-extract-12 | 箇条書きの値が元の行の文字列になることを確かめている。REQ-schema-031 は「見出しの下の行の規則」の話題の分 |
| crates/kotowari-mds/tests/cli.rs | values_json_keys_table_rows_by_declared_header_or_by_column_position | EX-schema-018 | C-schema-extract-30 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_nests_repeated_table_rows_per_table_with_the_data_row_lines | EX-schema-019 | C-schema-extract-32 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_splits_a_statement_with_derived_values_into_one_element_per_line | EX-schema-020 | C-schema-extract-28, C-schema-extract-29 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_keeps_shorthand_extractions_unsplit | EX-schema-021 | C-schema-extract-10 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_without_a_path_in_the_extract_declaration_stops | EX-schema-022 | C-schema-extract-5 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_omits_the_value_key_when_the_extract_declares_only_derived_values | EX-schema-023 | C-schema-extract-6 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_with_an_unaccepted_derived_word_stops | EX-schema-024 | C-schema-extract-22 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_gives_the_raw_line_for_the_title_and_for_table_rows | EX-schema-025 | C-schema-extract-25, C-schema-extract-26 | 指す行と、書いたとおりに出すこと |
| crates/kotowari-mds/tests/cli.rs | ex_schema_045_end_is_the_line_before_the_next_heading_as_deep_or_shallower | EX-schema-045 | C-schema-extract-27, C-schema-extract-21 | end の位置と、数値で出ること |
| crates/kotowari-mds/tests/cli.rs | ex_schema_046_end_outside_items_and_sections_stops | EX-schema-046 | C-schema-extract-24 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_077_a_placement_path_with_an_empty_name_stops | REQ-schema-036, EX-schema-077 | - | 空の名前の停止は cli に一本化した（以前の C-schema-extract-3 は欠番）。付け替え先（C-schema-cli-22）は cli の対応表にある |
| crates/kotowari-mds/tests/cli.rs | ex_schema_080_a_later_title_ends_the_section_and_item_before_it | REQ-schema-022, REQ-schema-062, EX-schema-080 | C-schema-cli-27 | この話題の分（`end`）は確かめていない。2つ目の題名で違反（multiple_titles）のある文書に `values` が終了コード 0 で値を出すので、cli の C-27 の付け替え先にした。題名の扱いは document-structure の C-schema-doc-7 |
| crates/kotowari-mds/tests/cli.rs | values_json_keeps_bullet_lines_and_blank_lines_as_written | EX-schema-054 | C-schema-extract-37, C-schema-extract-40 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_starts_a_field_value_after_the_name_and_keeps_its_continuation_apart | EX-schema-055 | C-schema-extract-38, C-schema-extract-35 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_drops_excluded_blocks_from_a_section_body_and_merges_blank_lines | EX-schema-056 | C-schema-extract-39 | そのまま |
| crates/kotowari-mds/tests/cli.rs | values_json_adds_no_blank_line_between_statement_lines_read_by_line | EX-schema-057 | C-schema-extract-39 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_058_heading_names_and_cells_drop_inline_markup_and_keep_code | EX-schema-058, REQ-schema-064 | C-schema-extract-41 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_059_a_heading_whose_text_without_markup_differs_is_undeclared | EX-schema-059 | C-schema-extract-42 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_060_an_item_body_leaves_out_the_child_bullets_of_a_declared_field | EX-schema-060, TBL-schema-008 | C-schema-extract-15 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_061_an_item_body_keeps_the_child_bullets_of_an_undeclared_line | EX-schema-061, TBL-schema-008 | C-schema-extract-15 | そのまま |

## 落とした細部

### 安全に関わるもの

なし。この話題の停止（path の無い宣言、知らない語、置き場の無い内側の抽出）は、どれも利用者の決定として残した。空の名前の停止は cli の C-schema-cli-22 に一本化して残した。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-040: 素の構文木を mdast に沿った形で出すこと、インライン要素まで含めること、位置情報を含めないこと | 実装者（2026-09-21-mds-spec.md の A19。「利用者の明示の判断ではない」） |
| REQ-schema-040 のテスト: frontmatter を木に入れないこと | 記録に無い（テストの中だけ） |
| REQ-schema-046: 継続段落を、区切った末尾の要素へ A3 のつなぎ方（元の空行を残す）で付けること | 主セッション（2026-09-23-extract-original-lines.md の A7）。改められた元の A50（mds-spec、利用者）は「改行を挟んで付ける」だった。C-35 には「末尾の要素に付く」だけを書いた |
| REQ-schema-045 のテスト: 区切った要素の前後の空白を取り除き、空の要素も残すこと | 主セッション（extract-original-lines の A7「今のまま取り除く」）。照合の側は「見出しの下の行の規則」の話題（mds-spec の A29、利用者） |
| テスト: CRLF と単独の CR の文書が LF と同じ値になること | この話題の記録に無い。単独の CR の数え方の決定（mutants-gaps の A1）は kotowari 側の話題 |

## 形に依存する要求

なし。この話題の要求は、どれも mds の抽出の振る舞いで、IR や判断の記録の形に依らない。記録の中の、用語集に語を足す決定（ir-engine の A62）と、`.mds/schemas/` の宣言を書き換える決定（A51）は、要求に写されておらず、この文書にも書かない。

## テストの無い行

- C-schema-extract-36（区切り文字と `value` を宣言したフィールド行では、区切った配列が `value` の鍵の下に入る）: 区切り文字と `{ path, value }` を宣言したフィールド行を抽出し、配置パスの下のオブジェクトの `value` の鍵に配列が入ることを確かめるテストが要る。
- C-schema-extract-43（抽出の結果は、閉じた世界と開いた世界のどちらで検査しても変わらない）: 元の PROP-schema-007 を指すテストが1本も無かった。同じ文書とスキーマで、スキーマの "open" を真にしたものと偽にしたものの抽出の結果が等しいことを確かめる性質のテストが要る。

## 誰が決めたか見分けられなかったもの

- REQ-schema-064 の、宣言した見出しの名前との照合も記号を外した文字で行うこと（C-42）: mutants-gaps の A4（利用者）は「見出しの名前を記号を外した文字で読む」とだけ書く。照合がその読み方に従うことは、閉じた世界の A2（利用者）との組み合わせから EX-schema-059 が導いている。A4 の当てはめとして残した。
- TBL-schema-008 の文の要素の単位（「導かれる値を宣言したときだけ行」）: ir-engine の A63（利用者）は「要素に分けるのは導かれる値を宣言したときだけ」、A69（利用者）は「`value` か導かれる値を宣言したとき」と書き、`value` だけを宣言した文がどうなるかが記録で揃っていない。元の表のとおり A63 の側で書いた。

## 理由や見送った案が記録に無かった決定

- 「値は、配置パスが指す場所に、…」: 入れ子に置くこと（mds-spec の A46）の理由が not recorded。問題の節に「理由は残っていない」と書いた。空の名前の停止の理由（review4-gaps の A4）はある。見送った案は無い。
- 「抽出の対象が文書に無いときは、…」: 理由（A47）も見送った案も無い。
- 「抽出は、path・value・of を持つ1つの入れ子で宣言する」: group を題名にだけ宣言できること（A64）の理由が not recorded。
- 「value か導かれる値を宣言したときだけ、…」: 見送った案が無い。
- 「項目自身が抽出を宣言しないまま、…」: A48 の理由は not recorded、A68 の why は実装の事実を述べるだけで理由ではない。見送った案も無い。
- 「導かれる値は5つの語にし、…」: id と name を項目に限ること（mds-spec の A54）の理由が not recorded。
- 「要素の最後の行は、…」: 見送った案が無い。
- 「要素ごとに宣言した表が繰り返すときは、…」: 1つの配列につなぐこと（A55、A66）の理由と、見送った案が無い。
- 「区切り文字を宣言したフィールド行は、…」: 理由（A50）も見送った案も無い。
- 「見出しの名前と表のセルの値は、…」: 見送った案が無い。理由は「今の振る舞いの追認」と、記号でなく文字を使うという A4 の why だけ。
- 「抽出の結果は、閉じた世界と開いた世界の…」: 理由（A59）も見送った案も無い。
- 「ast は、検査とは別に、…」: 利用者の決定（A26）の理由が not recorded。A19 の why（スキーマで表せない調べ方の逃げ道）は実装者の決定のものなので書かなかった。

見送った案のうち、この話題の決定の代わりではないものは書かなかった: ir-engine の R6・R7（鍵の重複と列の名前の衝突の停止。「CLI と結果の出し方」の話題）、review4-gaps の A3・A5 の rejected（同じ名前の2度の宣言、同じ列の名前。別の話題）、extract-original-lines の A1〜A4 以外の記録の rejected。

## 分けた所、まとめた所

- REQ-schema-048 を3つの決定に分けた: 宣言の形（「抽出は、path・value・of を持つ1つの入れ子で宣言する」。ir-engine の A45、A52、A53、A55、A60）、要素ごとのオブジェクト（TBL-schema-008 と同じ決定）、導かれる値の語（A23、A49、A54）。
- REQ-schema-035 と TBL-schema-008 を1つの決定にし、表を「決めたこと」の中に置いた。表の行の鍵（A43、A44、A48）と繰り返す表の段（A16、A59、A66）は、記録が別で見送った案や理由を持つので、別の決定にした。
- 文の行ごとの要素（ir-engine の A37、A57、extract-original-lines の A5、ir-engine の A66）は、表の1行から独立の決定にした。
- REQ-schema-045 と REQ-schema-046 を1つの決定にまとめた。
- REQ-schema-036 に入れていた空の名前の停止（review4-gaps の A4）は、cli の C-schema-cli-22 に一本化した（C-3 は欠番）。
- 値違いの場面をまとめた: C-4（ノードの種類と繰り返し）、C-9（group の停止の3通り）、C-19（内側のノードの種類）、C-30（空のヘッダと重なった名前）、C-15（EX-schema-060 と EX-schema-061）。

## あとでリンクにする所

無し。前の版で挙げたものは、どれも見出しへのリンクにした。

- 「スキーマ言語の骨格」の話題: 冒頭の段落の、出現回数の範囲で値が配列になること（PROP-schema-004 は schema-language に置いたまま。この話題では言い直さず、その決定へリンクした）
- 「見出しの下の行の規則」の話題: 冒頭の段落の、行の読み分け。「value か導かれる値を宣言したときだけ、…」の、ヘッダ行より多いセルの扱い
- 「CLI と結果の出し方」の話題: 冒頭の段落の、配置パスの衝突と停止の理由。「値は、配置パスが指す場所に、…」の空の名前の停止と `ast --schema` の出力。「ast は、検査とは別に、…」の CLI が受けるコマンド
- 「閉じた世界と開いた世界」の話題: 「抽出の結果は、閉じた世界と開いた世界の…」のスキーマの "open" と "--open"
- 「ライブラリの検証と値の抽出」の話題: 冒頭の段落

## 迷った判断と、どう決めたか

- REQ-schema-040（素の構文木）: 中身（mdast、インライン要素、位置情報を含めない）は実装者の A19 で、利用者の決定ではない。利用者が批准したのは、CLI が素の構文木の `ast` を受けること（A26）だけである。A26 を根拠に、`ast` が素の構文木を JSON で出すことだけを決定に残し、中身は落とした。cli の C-schema-cli-2 と二重になっていたので、依頼の指定どおりこの話題に残し、cli からはこの見出しへリンクする。CLI のコマンドの一覧は cli に置いたまま。
- 配置パスの空の名前の停止（前の版の C-schema-extract-3）は、cli の C-schema-cli-22 と同じ決定・同じテストだった。停止の条件の決定が並ぶ cli に一本化し、この話題の決定からはリンクにした。
- 付け替え先が「-」で、ほかの話題も付け替え先を持たなかったテスト（typed_ast_* の2本、ex_schema_080、children_bullets_with_extract_* の2本、child_field_can_have_extract、a_crlf_*、a_lone_cr_*、nested_extract_declaration_loads）は、確かめていることがいちばん近い行を付け替え先にした（ほかの話題の行を含む）。
- 読み方に依らないこと（extract-original-lines の A6）: 主セッションの導出で、利用者の決定ではない。ただし `reading: line` という設定のキーが元の場面（EX-schema-057）にあり、名前を残す決まりがある。A3（利用者）のつなぎ方は元の文書の行と空行だけで決まるので、その当てはめとして C-39 の後半に入れた。
- C-36 と C-43 は、状態が「実装済み」なのにテストが無い。C-36 は記録の Revisions（mds-spec が ir-engine の A52 を指す）にある利用者の決定、C-43 は利用者が追認した A59 なので、行を残し、上の「テストの無い行」に要るテストを書いた。
- 宣言した子のフィールド行を箇条書きの値から外すテスト（declared_child_field_is_excluded_from_the_parent_element）は、この話題の箇条書きの値の行（C-12）に入れるか迷った。外すこと自体は元の表に書かれていない（表は「子の箇条書きの行を含む」とだけ書く）ので、「見出しの下の行の規則」の話題の分として「-」にした。
- ex_schema_080 のテストで、`value` だけを宣言した文が `[{ "text": "in item" }]` になっている。1行しか無いので、行ごとに分けたのか段落ごとなのかは、このテストからは読めない。上の「誰が決めたか見分けられなかったもの」の文の要素の単位と関係するが、確かめていない。
- ir-engine の A37 の理由は、kotowari の指摘の種類（unknown_term など）を名指ししていた。この話題は mds の決定なので、kotowari が文の行ごとに指摘を出すのに要る、という中身に言い換えた。

## mds のスキーマの言葉を広げる決定（提案）

出どころ: mds の形の追加の設計案（リポジトリには置いていない） の A2（`of` の語）。元の ID は無い。

### 置き換える決定

実装する側は、新しい決定を作ったときに、左の列の文と行を消すか直し、新しい決定の状態を「実装済み」にする。左の列の決定は、それまで実装済みのまま残す。

| 古い決定の見出しと、置き換わる部分 | 置き換える決定の見出しと行 | 理由 |
|---|---|---|
| 「導かれる値は5つの語にし、語ごとに宣言できるノードを限る」の決めたことの3段目（どのノードにどの語を宣言できるか）と、C-schema-extract-23、C-schema-extract-24 | 「導かれる値の name は節にも、end は箇条書きにも宣言できる」（C-schema-extract-45〜C-schema-extract-47） | A2。設計の REQ-schema-048 の変更。5つの語、数値で出すこと、行番号と生の行が指す行（C-21、C-22、C-25、C-26）は残る。3段目を消したら、古い見出しは「導かれる値は5つの語にする」のように中身に合わせて直す |
| 「抽出は、path・value・of を持つ1つの入れ子で宣言する」の決めたことの3段目（`group` を `path` と並べ、題名にだけ）、問題の節の最後の2文、C-schema-extract-8、C-schema-extract-9 | 「名前付きキャプチャは、題名と箇条書きの of の鍵の値に group で書く」（C-schema-extract-48〜C-schema-extract-52） | A2。`extract.group` を除く。設計はリポジトリの中に使っているスキーマが無いと確かめている |
| 「value か導かれる値を宣言したときだけ、要素ごとのオブジェクトを組み立てる」の表の題名の行の「値」の列の「`group` を宣言したときは名前付きキャプチャが捕まえた文字」 | 同上（C-schema-extract-51） | 題名の値は見出しの文字列だけになる |
| 「要素の最後の行は、次の同じ深さか浅い見出しの手前の行にする」 | 置き換えない | 箇条書きの `end` は新しい決定に書いた。この決定は項目と節の `end` のまま残る |

テストの付け替え: title_named_group_capture_is_extracted（C-8）と extract_accepts_the_shorthand_and_the_named_group の `group` の部分、capture_extract_* の3本（C-9）は、新しい書き方に直して C-schema-extract-51、C-schema-extract-52 を指す。of_id_outside_an_item_is_rejected（C-23）と ex_schema_046_end_outside_items_and_sections_stops（C-24）は C-schema-extract-47 を指す。

### 「提案」で書いた決定の一覧

| 決定 | 変えることになりそうなコード（設計の §1 から） |
|---|---|
| 導かれる値の name は節にも、end は箇条書きにも宣言できる | `crates/kotowari-markdown-schema/src/schema/check.rs` の語の一覧（`SECTION_OF` と箇条書きの語）、`Block::Bullet` に最後の行を持たせる所、`extract.rs` の `Derived` と `place_derived` |
| 名前付きキャプチャは、題名と箇条書きの of の鍵の値に group で書く | `OfKind` に `Group` を足す、`check.rs` の題名だけの `group` の検査を `of` の中の `group` の検査に広げる |

### 実装する側が決める細部

- フィールド行の `end`（設計のコードの所には `Block::Field` に最後の行を足すとあるが、承認した語には無い）。宣言できるようにするなら、新しい決定の表を直す提案として利用者に出す。
- 名前付きキャプチャの値に、行の中のインラインの記法を残すか（設計の §7）。
- `{ group: <名前> }` を書いた鍵と、ほかの鍵や内側の配置パスとの重なりの停止は、cli の「配置パスが衝突するスキーマは、文書を読まずに停止する」のとおりと読んだ。

### 利用者に聞くこと

無い。

### 迷った判断

- 語ごとの置ける場所を、古い決定の3段目だけを置き換える新しい決定にし、5つの語の決定の残りは残した。新しい決定を、5つの語の決定ごとの書き直しにしなかったのは、変わるのが置ける場所の表だけだから。
- 確認することの行の例の正規表現の名前は、導かれる値の語（`name`、`id`）と紛れないように `code`、`text`、`flag` にした。

## 数

- 足した提案: 決定 2、確認することの行 8（C-schema-extract-45〜C-schema-extract-52。うち「（読んで確認）」0）
- 元: 要求 13、場面 20、決定表 1、性質 1
- 読んだ記録のファイル: 6（2026-09-21-mds-spec.md、2026-09-22-ir-engine.md、2026-09-23-ir-engine-gaps.md、2026-09-23-extract-original-lines.md、2026-09-23-mutants-gaps.md、2026-09-24-review4-gaps.md）
- 出来上がり: 決定 17、確認することの行 43（C-1、C-2、C-4〜C-44。C-3 は欠番。うち「（読んで確認）」0、テストの無い行 2）
- テスト: 120（うち付け替え先が「-」のもの 10。どれも、ほかの話題の対応表が付け替え先を持つ）
