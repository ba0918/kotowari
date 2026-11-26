# block-rules の対応表

元: `docs/ir/schema/block-rules.md`（日本語は `block-rules.ja.md`）。書き直した文書: `block-rules.md`。

## 元の ID の表

| 元の ID | 新しい ID | 決定の見出し | 理由 |
|---|---|---|---|
| REQ-schema-028 | - | 一覧の行は、宣言した名前と一致すればフィールド行、一致しなければ箇条書きとして読む | 定義を TBL-schema-007 に持つだけの要求なので、行は TBL-schema-007 と PROP-schema-006 の側に置いた。ガイド `docs/guides/marks*.md` の該当は、性質のテストを例に示したコードの中のマーカーで、節のマーカーではない（例の ID は C-schema-block-2 に書き換えればよい） |
| REQ-schema-029 | C-schema-block-4, C-schema-block-5 | フィールド行の値に、正規表現と許可リストを課せる | 正規表現と許可リストを C-4、区切った要素ごとの照合を C-5 にした |
| REQ-schema-054 | C-schema-block-6, C-schema-block-7 | 文と箇条書きの行に正規表現を課し、文には許可リストも課せる | 文を C-6、箇条書きを C-7 にした |
| REQ-schema-030 | C-schema-block-15 | 段落で読むとき、一覧の行に続く行と継続段落は、その一覧の行の一部にする | 「行で読む間は継続段落を作らない」は C-schema-block-9 の側で確かめる |
| REQ-schema-031 | C-schema-block-20, C-schema-block-21, C-schema-block-22 | 箇条書きの子の一覧は、宣言した子の規則に照らす | 宣言の無い子の行の指摘を C-20、宣言できるものと読み分けを C-21、子の箇条書きの抽出の停止を C-22 にした |
| REQ-schema-032 | C-schema-block-16 | 段落で読むとき、空行で区切った段落を1つの文として数える | 行で読むときの数え方は表と C-9〜C-12 の側 |
| REQ-schema-058 | - | 見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む | 定義を TBL-schema-011 に持つだけの要求なので、行は TBL-schema-011 の側に置いた |
| REQ-schema-033 | C-schema-block-23, C-schema-block-24, C-schema-block-25, C-schema-block-26 | 表のヘッダは、宣言したときだけ照合する | ヘッダの宣言の有無を C-23・C-24 に、列数の基準と余ったセルを別の決定「表の列数は文書のヘッダ行に合わせ、余ったセルは捨てる」の C-25・C-26 に分けた |
| REQ-schema-059 | C-schema-block-27, C-schema-block-28 | 表の規則は、"select: first" でヘッダの合う最初の表だけを選べる | 選び方を C-27、停止を C-28 にした。"select" を書かない規則の振る舞いは C-schema-block-24 で確かめる |
| REQ-schema-034 | C-schema-block-29, C-schema-block-30 | コードブロックの言語は宣言したときだけ照合し、行の照合は行頭の空白を除いて行う | 言語を C-29、行を C-30 にした |
| REQ-schema-041 | C-schema-block-8 | フィールド行の並び順は、強制を宣言したときだけ照合する | そのまま |
| TBL-schema-007 | C-schema-block-1, C-schema-block-3 | 一覧の行は、宣言した名前と一致すればフィールド行、一致しなければ箇条書きとして読む | 表を「決めたこと」に移した。順3（数字で始まる行）は、")" の区切りに見送った案があるので別の決定「数字で始まる一覧の行は、箇条書きにしない」にし、C-3 を置いた |
| TBL-schema-011 | C-schema-block-9, C-schema-block-10, C-schema-block-11, C-schema-block-12, C-schema-block-13, C-schema-block-14 | 見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む | 表を「決めたこと」に移した。見出しと GFM の表の行は「見出しと表は、どちらの読み方でも CommonMark と GFM のとおりに読む」（C-17、C-18）、字下げしたフェンスは「行で読むとき、フェンスの開始の行は字下げの深さを問わない」（C-19）で確かめる |
| PROP-schema-006 | C-schema-block-2 | 一覧の行は、宣言した名前と一致すればフィールド行、一致しなければ箇条書きとして読む | 性質の行にした |
| EX-schema-011 | C-schema-block-1 | 一覧の行は、宣言した名前と一致すればフィールド行、一致しなければ箇条書きとして読む | そのまま |
| EX-schema-012 | C-schema-block-23 | 表のヘッダは、宣言したときだけ照合する | そのまま |
| EX-schema-031 | C-schema-block-9 | 見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む | そのまま |
| EX-schema-032 | C-schema-block-9, C-schema-block-10 | 見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む | 空行の後の字下げした行が C-9、引用・HTML・画像の行が C-10 |
| EX-schema-033 | C-schema-block-11 | 見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む | そのまま |
| EX-schema-034 | C-schema-block-12 | 見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む | そのまま |
| EX-schema-071 | C-schema-block-19 | 行で読むとき、フェンスの開始の行は字下げの深さを問わない | そのまま |
| EX-schema-035 | C-schema-block-15, C-schema-block-16 | 段落で読むとき、一覧の行に続く行と継続段落は、その一覧の行の一部にする | 続く行と継続段落が C-15、引用の行が C-16 |
| EX-schema-036 | C-schema-block-27 | 表の規則は、"select: first" でヘッダの合う最初の表だけを選べる | そのまま |
| EX-schema-037 | C-schema-block-24, C-schema-block-28 | 表の規則は、"select: first" でヘッダの合う最初の表だけを選べる | "select" を書かない規則の指摘が C-24、"header" なしの "select" の停止が C-28 |
| EX-schema-050 | C-schema-block-28 | 表の規則は、"select: first" でヘッダの合う最初の表だけを選べる | "first" 以外の値の停止を、"header" なしの停止と1行にまとめた |
| EX-schema-038 | C-schema-block-25 | 表の列数は文書のヘッダ行に合わせ、余ったセルは捨てる | そのまま |
| EX-schema-039 | C-schema-block-13 | 見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む | そのまま |
| EX-schema-051 | C-schema-block-17 | 見出しと表は、どちらの読み方でも CommonMark と GFM のとおりに読む | そのまま |
| EX-schema-052 | C-schema-block-18 | 見出しと表は、どちらの読み方でも CommonMark と GFM のとおりに読む | そのまま |

## テストごとの表

| テストのファイル | テストの関数名 | 今のマーカーの ID | 付け替えたあとの ID | 理由 |
|---|---|---|---|---|
| crates/kotowari-markdown-schema/tests/properties.rs | markers_do_not_change_how_list_lines_are_read | REQ-schema-028 | C-schema-block-2 | マーカーを "-" から "*"、"+" に変えても指摘が変わらないことを生成した入力で確かめる性質のテスト |
| crates/kotowari-markdown-schema/tests/properties.rs | mixed_markers_read_the_same_as_hyphens | REQ-schema-028 | C-schema-block-2 | 行ごとにマーカーを混ぜても同じ指摘になることを確かめる性質のテスト |
| crates/kotowari-markdown-schema/src/schema/tests.rs | csv_is_a_shorthand_for_comma_separator | REQ-schema-029 | C-schema-block-5 | 落とした細部のテスト。いちばん近い行を指す。csv の略記が区切り文字 "," になることだけを確かめる。略記は記録に無い細部 |
| crates/kotowari-markdown-schema/src/schema/tests.rs | bullets_with_children_loads | REQ-schema-031 | C-schema-block-21 | 子の規則にフィールド行と箇条書きを宣言でき、子の箇条書きがさらに子の規則を持てることを確かめる |
| crates/kotowari-markdown-schema/src/schema/tests.rs | item_bullets_can_have_children | REQ-schema-031 | C-schema-block-21 | 項目の箇条書きにも子の規則を宣言できることを確かめる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | continuation_paragraph_is_part_of_the_bullet_not_undeclared | REQ-schema-030 | C-schema-block-15 | 継続段落が閉じた世界で指摘にならない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | continuation_paragraph_does_not_count_as_statement | REQ-schema-030, REQ-schema-032 | C-schema-block-15 | 継続段落が文に数えられない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | bullet_pattern_is_matched_only_on_the_marker_line_not_on_a_lazy_continuation_line | REQ-schema-054 | C-schema-block-7 | 照合がマーカーの行だけで、続く行に当たらないことを両方の読み方で確かめる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | bullet_pattern_is_not_applied_to_continuation_paragraphs | REQ-schema-030 | C-schema-block-7 | 継続段落に箇条書きの正規表現が当たらない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | code_block_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-block-20 | 子の規則の無い箇条書きの下のコードブロックが undeclared_line になる。REQ-schema-001 の分は「閉じた世界と開いた世界」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | code_block_as_first_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | - | 一覧の項目の先頭がコードブロックのときの読み方は、記録に無い細部（落とした細部）。この話題の行に当たらない。REQ-schema-001 の分は「閉じた世界と開いた世界」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | a_fence_opened_on_a_list_marker_line_closes_at_its_closing_line_in_both_readings | TBL-schema-011 | C-schema-block-14 | マーカーの行で開いたフェンスが、どちらの読み方でも同じコードブロックとして閉じる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_as_first_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | - | 一覧の項目の先頭が表のときの読み方は、記録に無い細部（落とした細部）。REQ-schema-001 の分は「閉じた世界と開いた世界」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_child_of_list_item_is_undeclared_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-block-20 | 子の規則の無い箇条書きの下の表が undeclared_line になる。REQ-schema-001 の分は別の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | paragraph_after_code_block_lead_is_undeclared_line_in_closed_world | REQ-schema-001, REQ-schema-032 | - | 先頭がコードブロックの一覧の項目の後の段落の扱い（落とした細部）と、閉じた世界の指摘を確かめる。この話題の行に当たらない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | paragraph_after_table_lead_is_undeclared_line_in_closed_world | REQ-schema-001, REQ-schema-032 | - | 先頭が表の一覧の項目の後の段落の扱い（落とした細部）と、閉じた世界の指摘を確かめる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | paragraph_after_code_block_lead_counts_as_a_statement | REQ-schema-032 | C-schema-block-16 | 段落が1つの文として数えられることを、先頭がコードブロックの一覧の項目の後の段落で確かめる。その場面そのものは落とした細部 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | paragraph_after_code_block_lead_is_subject_to_statement_pattern | REQ-schema-032 | C-schema-block-6 | 文の正規表現が当たり statement_pattern_mismatch が出る。場面そのものは落とした細部 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | image_only_paragraph_after_code_block_lead_is_not_a_statement | REQ-schema-032 | C-schema-block-16 | 画像だけの段落が文に数えられず、指摘にもならない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | nested_list_items_are_not_counted_as_top_level_bullets | REQ-schema-031 | C-schema-block-21 | 子の行が親の数に入らない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | declared_child_field_passes | REQ-schema-031 | C-schema-block-21 | 子の規則に宣言したフィールド行が通る |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_child_field_name_is_a_bullet_and_undeclared_without_rule | REQ-schema-028, REQ-schema-031 | C-schema-block-20, C-schema-block-21 | 宣言していない名前の子の行が子の箇条書きとして読まれ（C-21）、子の箇条書きの規則が無いので指摘になる（C-20） |
| crates/kotowari-markdown-schema/src/validate/tests.rs | child_list_without_children_rule_is_undeclared_line | REQ-schema-031 | C-schema-block-20 | 子の規則の無い箇条書きの子の一覧が undeclared_line になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | child_list_under_declared_field_is_undeclared_line | REQ-schema-031 | C-schema-block-20 | フィールド行の子の一覧が undeclared_line になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | child_list_under_declared_child_field_is_undeclared_line | REQ-schema-031 | C-schema-block-20 | 子のフィールド行の子の一覧が undeclared_line になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | a_child_line_with_an_undeclared_name_is_checked_as_a_child_bullet | REQ-schema-031, REQ-schema-028 | C-schema-block-21 | 宣言していない名前の子の行が子の箇条書きとして照合される |
| crates/kotowari-markdown-schema/src/validate/tests.rs | children_are_validated_when_parent_bullets_when_is_true | REQ-schema-020, REQ-schema-031 | C-schema-block-20 | 子の規則に無い子の行が指摘になる。条件付き規則（REQ-schema-020）の分は「スキーマ言語の骨格」の話題で付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | children_are_validated_when_parent_bullets_when_is_false | REQ-schema-020, REQ-schema-031 | C-schema-block-20 | 同上。親の条件が偽でも子の照合が行われる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | recursive_children_bullets_match | REQ-schema-031 | C-schema-block-21 | 子の箇条書きの子の規則に照らされる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | recursive_children_bullets_pattern_is_enforced | REQ-schema-031 | C-schema-block-21 | 孫の箇条書きにも子の規則の正規表現が当たる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | child_bullets_are_counted_by_children_rule_not_parent_repeat | REQ-schema-019, REQ-schema-031 | C-schema-block-21 | 子の行が親の数に入らず、子の規則の数で数えられる。出現回数（REQ-schema-019）の分は別の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_child_is_undeclared_even_when_open | REQ-schema-003, REQ-schema-031 | C-schema-block-20 | 子の規則に無い子の行が指摘になる。開いた世界でも指摘になる分（REQ-schema-003）は「閉じた世界と開いた世界」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | declared_child_field_pattern_is_enforced | REQ-schema-029, REQ-schema-031 | C-schema-block-4, C-schema-block-21 | 子の規則に宣言したフィールド行（C-21）の値に正規表現が当たる（C-4） |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_required_child_field_is_found | REQ-schema-004, REQ-schema-031 | C-schema-block-21 | 子の規則のフィールド行が子として数えられる。必須の欠落（REQ-schema-004）の分は「閉じた世界と開いた世界」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | nested_list_items_are_each_reported_in_closed_world | REQ-schema-001, REQ-schema-031 | C-schema-block-20 | 親と子の行がそれぞれ undeclared_line になる。REQ-schema-001 の分は別の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | ordered_list_is_not_a_bullet_and_is_undeclared_in_closed_world | REQ-schema-028 | C-schema-block-3 | 数字で始まる行に箇条書きの正規表現が当たらず、undeclared_line になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | ordered_list_does_not_satisfy_required_bullets | REQ-schema-028 | C-schema-block-3 | 数字で始まる行が箇条書きの数に入らない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | field_continuation_is_part_of_the_field_not_undeclared_or_statement | REQ-schema-030 | C-schema-block-15 | フィールド行の継続段落が文に数えられず、指摘にもならない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | declared_field_line_passes | REQ-schema-028 | C-schema-block-1 | 宣言した名前の行がフィールド行として読まれる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_field_name_line_counts_as_a_bullet | REQ-schema-028, EX-schema-011 | C-schema-block-1 | 宣言していない名前の行が箇条書きの数に入る |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_field_name_line_is_subject_to_bullet_pattern | REQ-schema-028 | C-schema-block-1 | 宣言していない名前の行に箇条書きの正規表現が当たる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_field_name_line_matches_bullet_pattern_on_original_text | REQ-schema-028 | C-schema-block-1, C-schema-block-7 | 箇条書きとして読まれ（C-1）、照合がマーカーを除いた元の行の文字に当たる（C-7） |
| crates/kotowari-markdown-schema/src/validate/tests.rs | undeclared_field_name_line_is_undeclared_when_no_bullets_declared | REQ-schema-001, REQ-schema-028 | C-schema-block-1 | 箇条書きとして読まれ、箇条書きを宣言していないので指摘になる。REQ-schema-001 の分は別の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | field_pattern_mismatch_is_found | REQ-schema-029 | C-schema-block-4 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | field_enum_invalid_is_found | REQ-schema-029 | C-schema-block-4 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | separator_splits_value_before_enum_check | REQ-schema-029 | C-schema-block-5 | 区切った要素ごとに許可リストが当たる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | separator_elements_are_trimmed_before_enum_check | REQ-schema-029 | C-schema-block-5 | 要素の前後の空白を除いて照合する |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_statement_is_found | REQ-schema-004, REQ-schema-032 | C-schema-lang-7 | 出現回数を書かない文が無いときの欠落の指摘（missing_statement）だけを確かめ、文の数え方は確かめていない。この話題の行に当たらないので、schema-language の C-schema-lang-7 を付け替え先にした（closed-world の対応表では「-」） |
| crates/kotowari-markdown-schema/src/validate/tests.rs | statement_pattern_mismatch_is_found | REQ-schema-054 | C-schema-block-6 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | statement_enum_invalid_is_found | REQ-schema-054 | C-schema-block-6 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | bullet_pattern_mismatch_is_found | REQ-schema-054 | C-schema-block-7 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_table_is_found | REQ-schema-033 | C-schema-lang-7 | "header" を宣言し出現回数を書かない表が無いときの欠落の指摘（missing_table）だけを確かめる。この話題の行に当たらないので、schema-language の C-schema-lang-7 を付け替え先にした。"header" を宣言しない表の欠落は table_without_declared_header_is_still_required が C-23 で確かめる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_header_mismatch_is_found | REQ-schema-033 | C-schema-block-24 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | missing_codeblock_is_found | REQ-schema-034 | C-schema-lang-7 | 出現回数を書かないコードブロックが無いときの欠落の指摘（missing_codeblock）だけを確かめる。この話題の行に当たらないので、schema-language の C-schema-lang-7 を付け替え先にした |
| crates/kotowari-markdown-schema/src/validate/tests.rs | codeblock_lang_mismatch_is_found | REQ-schema-034 | C-schema-block-29 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | codeblock_line_mismatch_is_found | REQ-schema-034 | C-schema-block-30 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | codeblock_line_matching_keeps_trailing_whitespace | REQ-schema-034 | C-schema-block-30 | 除くのが行頭の空白だけで、末尾の空白は残ることを確かめる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | codeblock_line_matching_trims_leading_whitespace_and_skips_blank_lines | REQ-schema-034 | C-schema-block-30 | 行頭の空白を除き、空の行を照合しない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | blockquote_and_thematic_break_lines_are_ignored_in_closed_world | REQ-schema-032 | C-schema-block-16 | 段落で読むと引用と水平線が指摘にならない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | image_only_line_is_not_counted_as_statement | REQ-schema-032 | C-schema-block-16 | 画像だけの行が文に数えられず、指摘にもならない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | a_repeated_field_in_a_row_is_still_in_declared_order | REQ-schema-041 | C-schema-block-8 | 同じフィールド行が続いても書いた順に反しない（細部はテストの中） |
| crates/kotowari-markdown-schema/src/validate/tests.rs | fields_in_declared_order_pass | REQ-schema-041 | C-schema-block-8 | 書いた順に並んだフィールド行に指摘が出ない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_declared_on_an_item_is_not_undeclared | REQ-schema-033 | C-schema-block-31 | 項目に宣言した表が指摘にならない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_header_mismatch_on_an_item_is_found | REQ-schema-033 | C-schema-block-31 | 項目の表のヘッダの食い違いが table_header_mismatch になる |
| crates/kotowari-markdown-schema/src/validate/tests.rs | code_block_declared_on_an_item_is_not_undeclared | REQ-schema-034 | C-schema-block-31 | 項目に宣言したコードブロックが指摘にならない |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_without_declared_header_accepts_any_header | REQ-schema-033, EX-schema-012 | C-schema-block-23 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_without_declared_header_is_still_required | REQ-schema-033 | C-schema-block-23 | "header" を宣言しない表が無いときに missing_table が出る |
| crates/kotowari-markdown-schema/src/validate/tests.rs | table_declared_on_the_preamble_is_not_undeclared | REQ-schema-023, REQ-schema-033 | - | 表の検査の分は無い。REQ-schema-023 の分は文書の骨格の対応表で C-schema-doc-8 に付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | code_block_declared_on_the_preamble_is_not_undeclared | REQ-schema-023, REQ-schema-034 | - | コードブロックの検査の分は無い。REQ-schema-023 の分は文書の骨格の対応表で C-schema-doc-8 に付け替える |
| crates/kotowari-markdown-schema/src/validate/tests.rs | a_table_row_with_the_wrong_column_count_points_at_that_row | REQ-schema-008, REQ-schema-033 | C-schema-block-25 | セルの少ない行が、その行を指す指摘になる。指摘の行の位置（REQ-schema-008）の分は「CLI と結果の出し方」の話題 |
| crates/kotowari-markdown-schema/src/validate/tests.rs | rows_are_counted_against_the_documents_header_not_the_declared_one | REQ-schema-033 | C-schema-block-26 | そのまま |
| crates/kotowari-markdown-schema/src/validate/tests.rs | field_order_mismatch_carries_the_name_of_the_field_out_of_order | REQ-schema-041, REQ-schema-008 | C-schema-block-8 | 順に反したフィールド行が field_order_mismatch になる。指摘が持つ名前と行（REQ-schema-008）の分は「CLI と結果の出し方」の話題 |
| crates/kotowari-markdown-schema/src/document/tests.rs | nested_list_items_become_children_of_the_parent_bullet | REQ-schema-031 | C-schema-block-21 | 子と孫の行が親の子として読まれる |
| crates/kotowari-markdown-schema/src/document/tests.rs | nested_list_children_keep_their_indentation | REQ-schema-031 | C-schema-block-21 | 子として読まれた行が元の字下げを保つ（字下げを保つことは抽出の話題の細部） |
| crates/kotowari-markdown-schema/src/document/tests.rs | continuation_paragraph_attaches_to_the_parent_bullet | REQ-schema-030 | C-schema-block-15 | 継続段落が直前の一覧の行に付く |
| crates/kotowari-markdown-schema/src/document/tests.rs | code_block_child_of_bullet_remains_a_block | REQ-schema-031, REQ-schema-034 | C-schema-block-20 | 箇条書きの下に字下げして続くコードブロックが子として読まれる |
| crates/kotowari-markdown-schema/src/document/tests.rs | table_child_of_bullet_remains_a_block | REQ-schema-031, REQ-schema-033 | C-schema-block-20 | 箇条書きの下に字下げして続く表が子として読まれる |
| crates/kotowari-markdown-schema/src/document/tests.rs | statement_after_code_block_lead_is_a_statement | REQ-schema-032 | C-schema-block-16 | 段落が1つの文として読まれる。場面（先頭がコードブロックの一覧の項目）は落とした細部 |
| crates/kotowari-markdown-schema/src/document/tests.rs | field_line_with_children_keeps_children | REQ-schema-031 | C-schema-block-20 | フィールド行の下の子の一覧が子として読まれる |
| crates/kotowari-markdown-schema/src/document/tests.rs | ordered_list_item_children_are_not_attached | REQ-schema-028 | C-schema-block-3 | 数字で始まる行が箇条書きにならない。その子が外に残ることは落とした細部 |
| crates/kotowari-markdown-schema/src/document/tests.rs | a_continuation_paragraph_of_an_ordered_item_belongs_to_that_item | REQ-schema-030 | C-schema-block-15 | 数字で始まる一覧の行の継続段落も、その行に付いて文にならない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | undeclared_field_name_line_is_extracted_as_a_bullet | REQ-schema-028, REQ-schema-035 | C-schema-block-1 | 宣言していない名前の行が箇条書きとして抽出される。抽出の値の形（REQ-schema-035）の分は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_undeclared_field_name_line_as_a_bullet | REQ-schema-028, REQ-schema-035 | C-schema-block-1 | 同上。節の本文に箇条書きとして入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | a_child_line_with_an_undeclared_name_stays_in_the_parent_bullet_value | REQ-schema-031, REQ-schema-063 | C-schema-block-21 | 宣言していない名前の子の行が子の箇条書きとして読まれる。値のつなぎ方（REQ-schema-063）の分は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | separator_trims_elements_and_keeps_empty_ones | REQ-schema-029, REQ-schema-035, REQ-schema-037, REQ-schema-045 | C-schema-block-5 | 要素の前後の空白を除き、空の要素を残す。ほかの ID の分は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | separator_split_excludes_continuation_paragraphs | REQ-schema-030, REQ-schema-035, REQ-schema-046, REQ-schema-063 | C-schema-block-15 | 継続段落がフィールド行の値に付く。分割の順序とつなぎ方は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | wrapped_line_is_part_of_value_and_subject_to_separator_split | REQ-schema-029, REQ-schema-035 | C-schema-block-15 | 空行なしで続く行がフィールド行の値に入る。分割の分は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | paragraph_after_code_block_lead_is_extracted_as_a_statement | REQ-schema-032, REQ-schema-035 | C-schema-block-16 | 段落が1つの文として抽出される。場面は落とした細部 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_continuation_paragraphs_and_nested_bullets | REQ-schema-030, REQ-schema-035 | C-schema-block-15 | 継続段落が一覧の行の一部として節の本文に入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_preserves_original_markers | REQ-schema-028, REQ-schema-035 | C-schema-block-2 | "*"、"+"、"-" のどの行も箇条書きとして読まれる。元のマーカーを保つ値の形は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_preserves_marker_with_continuation | REQ-schema-030, REQ-schema-035 | C-schema-block-15 | 継続段落が箇条書きに付く。値の形は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_preserves_whitespace_after_marker_with_continuation | REQ-schema-030, REQ-schema-035 | C-schema-block-15 | 同上 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_extract_includes_marker_and_continuation | REQ-schema-030, REQ-schema-035, REQ-schema-063 | C-schema-block-15 | 同上 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | bullet_with_multiple_continuations_keeps_the_blank_lines_between_them | REQ-schema-030, REQ-schema-035, REQ-schema-063 | C-schema-block-15 | 複数の継続段落が同じ箇条書きに付く。つなぎ方は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | ordered_list_is_not_extracted_as_a_bullet | REQ-schema-028, REQ-schema-035 | C-schema-block-3 | 数字で始まる行が箇条書きの抽出に入らない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | field_extract_includes_continuation_paragraph | REQ-schema-030, REQ-schema-035, REQ-schema-046, REQ-schema-063 | C-schema-block-15 | 継続段落がフィールド行の値に入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_body_field_line_includes_continuation_paragraph | REQ-schema-030, REQ-schema-035, REQ-schema-063 | C-schema-block-15 | 項目の中のフィールド行でも、継続段落がその行の一部になる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_extract_concatenates_multiple_tables_in_document_order | REQ-schema-033, REQ-schema-035 | - | 表の抽出の値の形だけを確かめ、表の検査（この話題）の分は無い。「抽出と素の構文木」の話題で付け替える |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_table_extracts_flat_row_objects_without_nesting | REQ-schema-033, REQ-schema-035 | - | 同上 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_with_a_derived_value_puts_the_row_line_into_each_row_object | REQ-schema-033, REQ-schema-048 | - | 同上（導かれる値の形） |
| crates/kotowari-markdown-schema/src/extract/tests.rs | repeated_table_with_a_derived_value_nests_rows_per_table | REQ-schema-033, REQ-schema-048 | - | 同上 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_without_a_declared_header_gives_each_row_as_an_array | REQ-schema-033, REQ-schema-035 | - | 同上。"header" を宣言しない表の値の形 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | table_with_empty_or_repeated_header_cells_keeps_every_column | REQ-schema-033, REQ-schema-035 | - | 同上 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | declared_header_decides_the_row_keys_when_the_column_count_differs | REQ-schema-033, REQ-schema-035 | C-schema-block-25 | 文書の列が多いとき余ったセルが捨てられる。鍵の決め方は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | parent_bullet_element_includes_child_bullet_lines_with_indentation | REQ-schema-031, REQ-schema-035 | C-schema-block-21 | 子の行が親の子として読まれ、親の値に入る。値の形は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | declared_child_field_is_excluded_from_the_parent_element | REQ-schema-031, REQ-schema-035 | C-schema-block-21 | 子の規則に宣言したフィールド行が子のフィールド行として読まれる。値の形は「抽出と素の構文木」の話題 |
| crates/kotowari-markdown-schema/src/extract/tests.rs | child_of_declared_field_does_not_shadow_declared_child_field | REQ-schema-031 | C-schema-block-20 | 子の規則の無いフィールド行の下の子は、宣言した子のフィールド行として読まれない |
| crates/kotowari-markdown-schema/src/extract/tests.rs | recursive_nesting_is_included_in_the_parent_element | REQ-schema-031, REQ-schema-035 | C-schema-block-21 | 孫までの入れ子が親の子として読まれる |
| crates/kotowari-markdown-schema/src/extract/tests.rs | section_body_includes_child_bullet_lines | REQ-schema-031, REQ-schema-035 | C-schema-block-21 | 同上。節の本文に子の行が入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | item_body_includes_child_bullet_lines | REQ-schema-031, REQ-schema-035 | C-schema-block-21 | 同上。項目の本文に子の行が入る |
| crates/kotowari-markdown-schema/src/extract/tests.rs | preamble_table_is_extracted_as_row_objects | REQ-schema-023, REQ-schema-033 | - | 表の検査の分は無い。REQ-schema-023 の分は文書の骨格の対応表で C-schema-doc-8 に付け替える |
| crates/kotowari-mds/tests/cli.rs | check_undeclared_child_under_declared_field_exits_one | REQ-schema-003, REQ-schema-031 | C-schema-block-20 | フィールド行の下の子の一覧が undeclared_line になる。REQ-schema-003 の分は別の話題 |
| crates/kotowari-mds/tests/cli.rs | decision_record_fixture_passes_check_and_extracts_values | REQ-schema-035, REQ-schema-031 | C-schema-block-21 | 子の規則に宣言したフィールド行を持つ文書が端から端まで通る。抽出の分は「抽出と素の構文木」の話題 |
| crates/kotowari-mds/tests/cli.rs | list_nesting_and_fences_read_the_same_in_both_readings | REQ-schema-058, TBL-schema-011, REQ-schema-031 | C-schema-block-13, C-schema-block-14 | 入れ子（C-13）とフェンス（C-14）が両方の読み方で同じ指摘と値になる。タブや囲みの記号の細部はテストの中 |
| crates/kotowari-mds/tests/cli.rs | line_reading_takes_up_to_three_leading_spaces_for_a_heading | REQ-schema-058, TBL-schema-011 | C-schema-block-17 | 行で読むとき、行頭の空白が3つまでの "##" が見出しで、4つは文 |
| crates/kotowari-mds/tests/cli.rs | line_reading_reads_thematic_breaks_as_statements_and_numbered_lines_as_ordered_lists | REQ-schema-058, TBL-schema-011, TBL-schema-007 | C-schema-block-10, C-schema-block-3 | 行で読むと水平線が文になり（C-10）、"1." と "1)" の行が順序付きの行として undeclared_line になる（C-3） |
| crates/kotowari-mds/tests/cli.rs | ex_schema_051_headings_are_commonmark_atx_headings_in_both_readings | EX-schema-051 | C-schema-block-17 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_052_a_gfm_table_without_leading_pipes_is_a_table_when_reading_by_line | EX-schema-052 | C-schema-block-18 | そのまま |
| crates/kotowari-mds/tests/cli.rs | tbl_schema_011_tables_missing_a_pipe_in_the_header_or_delimiter_row_are_tables_in_both_readings | TBL-schema-011 | C-schema-block-18 | 縦棒の欠けた GFM の表が、どちらの読み方でも表として抽出される |
| crates/kotowari-mds/tests/cli.rs | ex_schema_035_paragraph_reading_absorbs_following_lines_and_ignores_quotes | EX-schema-035 | C-schema-block-15, C-schema-block-16 | 続く行と継続段落（C-15）、引用の行（C-16） |
| crates/kotowari-mds/tests/cli.rs | ex_schema_031_line_reading_keeps_the_line_after_a_field_line_as_a_statement | EX-schema-031 | C-schema-block-9 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_032_line_reading_counts_indented_quote_html_and_image_lines_as_statements | EX-schema-032 | C-schema-block-9, C-schema-block-10 | 字下げした行（C-9）と、引用・HTML・画像の行（C-10） |
| crates/kotowari-mds/tests/cli.rs | ex_schema_033_line_reading_makes_no_setext_heading_or_indented_code_block | EX-schema-033 | C-schema-block-11 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_071_line_reading_opens_a_fence_at_any_indent | EX-schema-071 | C-schema-block-19 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_034_pipe_line_without_delimiter_is_a_statement_in_both_readings | EX-schema-034 | C-schema-block-12 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_039_line_reading_keeps_an_indented_list_line_a_list_line | EX-schema-039 | C-schema-block-13 | そのまま |
| crates/kotowari-mds/tests/cli.rs | tbl_schema_011_line_reading_reads_hash_word_and_equals_lines_as_statements | TBL-schema-011 | C-schema-block-11, C-schema-block-17 | "===" の行が見出しを作らない（C-11）、"#foo" が文になる（C-17） |
| crates/kotowari-mds/tests/cli.rs | tbl_schema_011_a_hash_only_line_is_a_heading_in_both_readings | TBL-schema-011 | C-schema-block-17 | "#" だけの行が、どちらの読み方でも見出しになる |
| crates/kotowari-mds/tests/cli.rs | ex_schema_036_select_first_uses_only_the_first_table_whose_header_matches | EX-schema-036 | C-schema-block-27 | そのまま |
| crates/kotowari-mds/tests/cli.rs | req_schema_059_select_first_picks_one_table_per_occurrence_of_a_repeated_section_in_both_check_and_values | REQ-schema-059 | C-schema-block-27 | 繰り返し現れる節で、出現ごとに1つの表が選ばれる |
| crates/kotowari-mds/tests/cli.rs | ex_schema_037_table_rule_without_select_reports_a_mismatched_header_and_select_needs_header | EX-schema-037, TBL-schema-009 | C-schema-block-24, C-schema-block-28 | "select" を書かない規則のヘッダの指摘（C-24）と、"header" なしの "select" の停止（C-28）。停止の理由の表（TBL-schema-009）の分は「CLI と結果の出し方」の話題 |
| crates/kotowari-mds/tests/cli.rs | ex_schema_038_extra_cells_are_dropped_and_missing_cells_are_reported | EX-schema-038 | C-schema-block-25 | そのまま |
| crates/kotowari-mds/tests/cli.rs | ex_schema_050_select_other_than_first_stops | EX-schema-050 | C-schema-block-28 | そのまま |

## 落とした細部

### 安全に関わるもの

なし。停止の条件（"header" なしの "select"、"first" 以外の "select"、子の箇条書きの抽出）は、どれも決定に残した。

### そのほか

| 要求のどの部分 | 誰が決めたと記録にあるか |
|---|---|
| REQ-schema-029: 区切り文字の略記 csv（テスト csv_is_a_shorthand_for_comma_separator だけにある） | 記録に無い |
| REQ-schema-031、REQ-schema-032: 一覧の項目の先頭がコードブロックか表のときに、その後の段落を文として読み、閉じた世界で指摘にすること（テストだけにある） | 記録に無い |
| REQ-schema-028: 数字で始まる一覧の行の子の行を、その行に付けずに外に残すこと（テスト ordered_list_item_children_are_not_attached だけにある） | 記録に無い |
| REQ-schema-041: 同じフィールド行が続いても書いた順に反しないこと | 記録に無い（テストの中だけ） |
| REQ-schema-034: 行の照合で末尾の空白を残すこと | 記録に無い（A35 の「行頭の空白を取り除いてから」から読めるが、末尾は書かれていない）。テストの中に残る |
| TBL-schema-011: 一覧の入れ子の字下げの数え方（タブ）と、フェンスの囲みの記号と長さの細部（テスト list_nesting_and_fences_read_the_same_in_both_readings だけにある） | 記録に無い。CommonMark の定めのとおりで、テストの中に残る |
| 記録の中で、ir、ir-context、ir-flags の各スキーマが "reading: line" や "select: first" を書くと決めた部分（ir-engine-gaps の A13、A20、A27、A28） | 利用者（推奨を採用）。下の「形に依存する要求」 |

## 形に依存する要求

元の要求・表・性質・場面の本文には、古い仕様の形そのものについての振る舞いは無い。出典の記録のうち、kotowari 自身の IR のスキーマについて決めた部分は消える。

| 元の ID | 扱い |
|---|---|
| REQ-schema-058、TBL-schema-011（出典 ir-engine-gaps の A13、A20 の後半） | 消える。「IR の3つのスキーマ（ir、ir-context、ir-flags）は "reading: line" を書く」は、IR の形が無くなるので書かなかった。エンジンの読み方の決定は残した |
| REQ-schema-059（出典 ir-engine-gaps の A27、A28 の後半） | 消える。「ir-context が "select: first" を書き、kotowari が用語集で宣言の外になった表を捨てる」は、用語集の形の検査が無くなるので書かなかった。"select: first" の選び方は、汎用の表の書式の例として問題の節に残した |

## テストの無い行

無し。前の版で挙げた C-schema-block-22（子の箇条書きに抽出を宣言したスキーマは停止）は、`crates/kotowari-markdown-schema/src/schema/tests.rs` の `children_bullets_with_extract_is_schema_invalid` と `deep_children_bullets_with_extract_is_schema_invalid`（REQ-schema-035 のマーカー）が確かめているので、extraction の対応表のテストごとの表で、この2本の付け替え先を C-schema-block-22 にした。

## 誰が決めたか見分けられなかったもの

- なし。出典の決定はどれも decided_by が利用者（「推奨を採用」「散文の仕様から起こし批准」を含む）だった。ただし A29〜A36 は、散文の仕様から起こして利用者がまとめて批准したもので、why が not recorded である。利用者の個別の判断でないものは「迷った判断」に挙げた。

## 理由や見送った案が記録に無かった決定

- 「フィールド行の値に、正規表現と許可リストを課せる」: 理由も見送った案も記録に無い（A29 の why は not recorded）。問題の節に「理由は残っていない」と書いた。
- 「文と箇条書きの行に正規表現を課し、文には許可リストも課せる」: 同上（A31、A32）。
- 「フィールド行の並び順は、強制を宣言したときだけ照合する」: 同上（A36）。
- 「コードブロックの言語は宣言したときだけ照合し、…」: 同上（A35）。A12 の why は項目に置けることの理由で、照合の理由ではない。
- 「数字で始まる一覧の行は、箇条書きにしない」: 箇条書きにしないことの理由は記録に無い（A33 は not recorded）。")" を認めることの理由と見送った案だけを書いた。
- 「段落で読むとき、空行で区切った段落を1つの文として数える」: A30 の why は not recorded。既定を CommonMark のまま残す理由（A12）を問題に書き、中身の理由は残っていないと書いた。見送った案は無い。
- 「段落で読むとき、一覧の行に続く行と継続段落は、その一覧の行の一部にする」: 見送った案が記録に無い。理由は A12 から書いた。
- 「箇条書きの子の一覧は、宣言した子の規則に照らす」: A34、A67 の why は not recorded。閉じた世界の理由（A2）だけを書いた。見送った案は無い。
- 「一覧の行は、宣言した名前と一致すれば…」: 見送った案が記録に無い。
- 「表のヘッダは、宣言したときだけ照合する」: 見送った案が記録に無い。
- 「表の規則は、"select: first" で…」: 見送った案が記録に無い。
- 「見出しの下の行は、スキーマが選ぶ読み方…」: 見送った案は A13 の rejected だけ。
- 「表とコードブロックの規則は、項目にも節と同じに宣言できる」: 見送った案が記録に無い。

## 分けた所、まとめた所

- TBL-schema-007 の順3（数字で始まる行）を、独立の決定「数字で始まる一覧の行は、箇条書きにしない」にした。")" の区切りを決めた review3-gaps の A2 が見送った案を持つため。
- TBL-schema-011 の見出しの行と GFM の表の行を、独立の決定「見出しと表は、どちらの読み方でも CommonMark と GFM のとおりに読む」にした。A43 が見送った案を持ち、A44 が同じ方針の決定のため。表には、その決定へのリンクを残した。
- TBL-schema-011 のフェンスの字下げを、独立の決定「行で読むとき、フェンスの開始の行は字下げの深さを問わない」にした。indented-fence の A1 が見送った案を持つため。
- REQ-schema-033 を2つの決定に分けた。ヘッダの宣言の有無（A13）と、列数の基準と余ったセル（A16、A46。A46 が見送った案を持つ）。
- REQ-schema-034 の出典の A12（項目にも表とコードブロックを宣言できる）を、独立の決定「表とコードブロックの規則は、項目にも節と同じに宣言できる」にした。元の文書には要求の文として無いが、REQ-schema-033 と REQ-schema-034 を指すテストのうち3本が確かめている。
- REQ-schema-030 と REQ-schema-032 は、段落で読むときの決定として別々に置いた。行で読むときの部分は、表の決定にまとめた。
- 値違いの場面（EX-schema-037 の後半と EX-schema-050）は C-schema-block-28 の1行にまとめた。

## あとでリンクにする所

無し。前の版で挙げたものは、どれも見出しへのリンクにした。

- 「スキーマ言語の骨格」の話題: 「見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む」の "reading" が受ける値、「表とコードブロックの規則は、項目にも節と同じに宣言できる」の規則種別と置ける場所の表
- 「閉じた世界と開いた世界」の話題: 「数字で始まる一覧の行は、箇条書きにしない」と「箇条書きの子の一覧は、宣言した子の規則に照らす」の閉じた世界
- 「抽出と素の構文木」の話題: 「箇条書きの子の一覧は、宣言した子の規則に照らす」の子のフィールド行の抽出の値の置き場（停止の決定はこの話題に置いたまま）

## 迷った判断と、どう決めたか

- REQ-schema-031 の最後の文（子の箇条書きに抽出を宣言したスキーマは停止）は、出典の A67 を指しているのがこの話題だけで、確かめているテストは「抽出と素の構文木」の REQ-schema-035 のマーカーを持つ。停止の条件なので落とさず、この話題に行を置き、テストの無い行に挙げた。
- `ordered`、`separator`、`csv`、`lines`、`children` のうち、元の文書に出てくる綴りは無い。`children` は利用者が批准した A34 にあるので決めたことに書いた。ほかは記録にも元の文書にも無く、テストの中だけにあるので、元の文書と同じく言葉（「並び順の強制」「区切り文字」「行ごとの正規表現」）で書いた。形の決まりで足りなかった所（下）。
- 一覧の項目の先頭がコードブロックか表のときの、後の段落の扱いを確かめるテストが7本ある。場面そのものは記録に無い細部として落とした。そのうち、文の数え方や文の照合を確かめている4本は C-schema-block-16 か C-schema-block-6 に付け、閉じた世界の指摘だけを確かめる3本は「-」にした。
- C-schema-block-17 は、EX-schema-051 の場面に、"#" だけの行（tbl_schema_011_a_hash_only_line_is_a_heading_in_both_readings）と、行で読むときの4つの字下げ（line_reading_takes_up_to_three_leading_spaces_for_a_heading）を足した。4つの字下げは段落で読むと字下げのコードブロックになるので、「出力が一致する」の外に書いた。
- 「見出しの下の行は、スキーマが選ぶ読み方…」の問題に書いた「kotowari の仕様の文書が求める文は1行ずつ」は、A12 の why の「IR の文の定義は行単位」を、IR の名前を出さずに書いたもの。
- 「表の規則は、"select: first" で…」の問題の例（題名の後に用語の表を置く）は、A27 の why の用語集の場面を、形に依存しない書式の例に言い換えた。
- 誰が決めたかの但し書き: ir-engine-gaps の A44（C-schema-block-18、行で読むときの GFM の表）は「CommonMark / GFM に寄せる指示を、同じ種類の食い違いに当てた」もので、表そのものを利用者が個別に決めた記録ではない。review3-gaps の A2（")" の区切り、C-schema-block-3）は、利用者が「ほぼ答えが決まっているものは推奨で決めてよい」とした範囲で推奨を採用したもの。どれも中身が具体的な振る舞いで、細部の委任だけの承認ではないので、利用者の決定として残した。覆すなら利用者が判断する。
- 状態を変える前提で、テストで確かめる行には、それを確かめる既存のテストがあることを、テストの関数名で確かめた（上の表と、C-22 は extraction の対応表）。
- 欠落の指摘だけを確かめるテスト（missing_statement_is_found、missing_table_is_found、missing_codeblock_is_found）は、この話題の行に当たらず、ほかの ID も付け替え先を持たないので、確かめていることがそのまま当たる schema-language の C-schema-lang-7（出現回数を書かないノードが無ければ欠落の指摘）を付け替え先にした。
- 「決めたこと」と「確認すること」の「など」は無かった。「〜のような」「〜のように」は、どれも範囲を決めた文の中の具体例（"| a | b |"、"a | b"、"select: last"）なので直さなかった。

## 形の決まりで足りなかった所

- 利用者から見える鍵の名前（`ordered`、`separator`、`csv`、`lines`）が、元の文書にも記録にも無く、テストの中にだけある。決まりは「元の文書に出てくる綴りを残す」なので、残す義務は無いと読んで書かなかった。check が「名前が仕様に引用されているか」を見るなら、これらは引用の無い名前として出る。確定の依頼で、このまま（決定の文に書かない）にすると決まった。

## mds のスキーマの言葉を広げる決定（提案）

出どころ: mds の形の追加の設計案（リポジトリには置いていない） の A2（箇条書きの照合）と A5 と、利用者の答え2と4。元の ID は無い。

### 置き換える決定

実装する側は、新しい決定を作ったときに、左の列の文と行を消すか直し、新しい決定の状態を「実装済み」にする。左の列の決定は、それまで実装済みのまま残す。

| 古い決定の見出しと、置き換わる部分 | 置き換える決定の見出しと行 | 理由 |
|---|---|---|
| 「文と箇条書きの行に正規表現を課し、文には許可リストも課せる」の決めたことの2段目の「照合は、元のマーカーの行の…照合しない」の2文と、C-schema-block-7 の「照合はマーカーの行の…照合されない」の部分 | 「箇条書きの正規表現は、一覧の項目の先頭の段落を1行につないだ文字と照合する」（C-schema-block-32、C-schema-block-33） | 答え4。設計の REQ-schema-054 の変更。文の正規表現と許可リスト、箇条書きの正規表現を課せること、bullet_pattern_mismatch は残る |
| 「段落で読むとき、空行で区切った段落を1つの文として数える」の決めたことの2文目のうち引用・水平線・HTML の部分と、C-schema-block-16 の同じ部分 | 「段落で読むとき、"open: true" を宣言しないノードの中の引用、HTML のブロック、水平線は指摘にする」（C-schema-block-34〜C-schema-block-37） | 答え2。設計の REQ-schema-032 の変更。段落を1つの文として数えることと、画像だけの行の扱いは残る |
| 「見出しの下の行は、スキーマが選ぶ読み方（段落か行）で読む」の表の「引用、水平線、HTML、画像の行」の行の "paragraph" の列 | 同上 | 答え2。"paragraph" の列を「引用、水平線、HTML は、"open: true" を宣言しないノードの中では undeclared_line。画像の行は文に数えず、指摘にもしない」に直す。"line" の列と C-schema-block-10 は残る |

テストの付け替え: bullet_pattern_is_matched_only_on_the_marker_line_not_on_a_lazy_continuation_line は、段落で読むときの期待が変わる（続く行もつないで照合する）。実装のときに直し、C-schema-block-32 を指す。blockquote_and_thematic_break_lines_are_ignored_in_closed_world は期待が逆になる（指摘が出る）ので、直して C-schema-block-34 を指す。ex_schema_035_paragraph_reading_absorbs_following_lines_and_ignores_quotes の引用の部分も同じ。image_only_line_is_not_counted_as_statement は C-schema-block-16 のまま。

### 「提案」で書いた決定の一覧

| 決定 | 変えることになりそうなコード（設計の §1 から） |
|---|---|
| 箇条書きの正規表現は、一覧の項目の先頭の段落を1行につないだ文字と照合する | `crates/kotowari-markdown-schema/src/validate.rs` の `marker_line_part` |
| 段落で読むとき、"open: true" を宣言しないノードの中の引用、HTML のブロック、水平線は指摘にする | `document.rs` の段落の読み取りで捨てている引用・HTML・水平線のブロック、`validate.rs` の `undeclared_line_for_block` |
| 閉じないフェンスは、どのスキーマでも unclosed_codeblock の指摘にする | 指摘の種類の追加（`finding.rs`）、構文木の全体を辿る検査、`line_reading.rs` の `Fence::read`。kotowari の `crates/kotowari-core/src/finding_map.rs` に写し先を同じ変更で足す（足さないと写しの食い違いで止まる） |

### 実装する側が決める細部

- 引用・HTML のブロック・水平線の undeclared_line が持つ「その行をどう読んだか」の種別の値（closed-world の「宣言していない行の指摘に、その行をどう読んだかの種別を持たせる」の表に無い）。決まったら、closed-world の決めたことの列挙を事実の書き直しで足す。
- unclosed_codeblock の深刻度と、cli の「指摘は、種類、深刻度、文書のパス、行番号、詳細を持つ」の分類の表のどこに入れるか（5つの分類のどれにも書かれていない。設計は「TBL-schema-002 に行を足す」とだけ書く）。決まったら、その表を事実の書き直しで直す。
- unclosed_codeblock の詳細の文言。引用の中で開いて閉じないフェンスを指摘にするか（設計は「引用の中も辿る」と書くが、CommonMark では引用が終わるとフェンスも終わる）。行で読むときの、閉じないフェンスの中身の扱い。
- 箇条書きの照合の文字に、行の中のインラインの記法を残すか（設計の §7: 行の名前にインラインの記法が残る）。

### 利用者に聞くこと

- 問い: 全体像の元データのスキーマ（`crates/kotowari-overview/schemas/overview.yaml`）は、スキーマの最上位を "open: true" にし、前置部を宣言して、その中に HTML のコメントを置けるとしている（スキーマの頭の注記）。答え2のとおりに作ると、前置部は "open: true" を宣言しないノードなので、前置部の HTML のコメントが undeclared_line になる。設計の「全体像は開いているので影響しない」は、最上位の "open" が宣言済みの前置部の中を許さないこと（closed-world の「開いた世界は、…」の2段目）を見落としている。前置部に HTML のコメントを置けることは、古い仕様の TBL-core-038（元データの形）の表にあり、core の overview-data の下書きでは「題名の後と節の下に置けるものの細部」として落とされていて、実装済みの決定には書かれていない。スキーマの注記と今の振る舞いにだけ残る。節の見出しの隣のマーカー（HTML のコメント）は、全体像のスキーマが節を宣言しないので、開いた世界の宣言していない構造の内側として許され、影響を受けない。
  - 推奨: HTML のコメントだけのブロックは、リンクの参照定義と同じく、どのノードの中でも指摘にしない。表示に何も出ないものなので、黙って通っても書式が崩れない。全体像の元データの前置部の形も変わらない。
  - 変わる条件: HTML のコメントも書式の外として指摘にしたいなら、全体像のスキーマの前置部に "open: true" を書く（部品のコードブロックの規則は保たれるが、前置部に段落も書けるようになる）。

### 迷った判断

- 答え2の「"open" でない入れ物」を、ノードの "open: true" だけで読み、スキーマの最上位の "open" では緩めないとした（C-schema-block-37）。開いた世界は宣言済みのノードの中を許さない（closed-world の決定）ので、その続きとして読んだ。開いた世界で許す宣言していない構造の内側は、ほかの行と同じく許す。
- A5 の閉じないフェンスの決定は、コードブロックの読み方を決めるこの話題に置いた。cli の分類の表は、どの分類かが決まっていないので触らなかった（上の「実装する側が決める細部」）。
- 設計の見送った案「マーカーの行の照合を残し、kotowari が折り返した名前をつなぐ」は、照合の決定に置いた。mds の決定なので「mds を使う側（kotowari）」と書いた。
- 答え4の照合の文字は、行で読むときにはマーカーの行だけになる（行で読むと、続く行は文になる。C-schema-block-9）。決めたことには「先頭の段落」とだけ書き、読み方ごとの言い直しはしなかった。

## 数

- 足した提案: 決定 3、確認することの行 9（C-schema-block-32〜C-schema-block-40。うち「（読んで確認）」0）
- 元: 要求 11、場面 15、決定表 2、性質 1
- 読んだ記録のファイル: 4（2026-09-21-mds-spec.md、2026-09-23-ir-engine-gaps.md、2026-09-24-review3-gaps.md、2026-09-23-indented-fence.md）
- 出来上がり: 決定 16、確認することの行 31（うち「（読んで確認）」0）
- テスト: 133
